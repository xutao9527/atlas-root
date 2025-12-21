use crate::net::client::pending::PendingTable;
use crate::net::codec_rmp::MsgPackCodec as Codec;
use crate::net::packet::{Packet, Response};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify, mpsc};
use tokio::time::sleep;
use tokio_util::codec::Framed;

pub struct AtlasConnection {
    addr: String,
    channel_reader: Mutex<Option<mpsc::Receiver<Packet>>>,
    channel_writer: mpsc::Sender<Packet>,
    pending: Arc<PendingTable>,
    notify_connected: Arc<Notify>,
    notify_disconnected: Arc<Notify>,
    connected: AtomicBool,
}

impl AtlasConnection {
    pub async fn new(addr: String) -> anyhow::Result<Self> {
        let pending = Arc::new(PendingTable::new(100 * 1024));
        let (channel_writer, channel_reader) = mpsc::channel::<Packet>(100 * 1024);
        Ok(Self {
            addr: addr.to_string(),
            channel_reader: Mutex::new(Some(channel_reader)),
            channel_writer,
            pending,
            notify_connected: Arc::new(Notify::new()),
            notify_disconnected: Arc::new(Notify::new()),
            connected: AtomicBool::new(false),
        })
    }

    pub async fn connect(self: Arc<Self>) {
        let this = self.clone();
        {
            let this = this.clone(); // 给这个 spawn 单独 clone 一份
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    let pending_count = this.pending.len();
                    //println!("当前 pending 请求数量: {}", pending_count);
                }
            });
        }

        tokio::spawn(async move {
            let mut attempt = 0u32;
            loop {
                match this.establish_connection().await{
                    Ok(()) => {
                        attempt = 0;
                        this.notify_connected.notify_waiters();             // 通知连接成功

                        eprintln!("等待断开连接通知! => connect_loop");
                        this.notify_disconnected.notified().await;          // 等待通知断线
                        eprintln!("收到断开连接通知! => connect_loop");

                        this.pending.drain(|slot| {
                            let resp = Response {
                                id: slot.request_id,
                                slot_index: usize::MAX,
                                payload: Vec::new(),
                                error: Some("connection closed".into()),
                            };
                            (slot.callback)(Packet::Response(resp));
                        });
                    }
                    Err(e) => {
                        attempt += 1;
                        let delay = Duration::from_secs(2u64.pow(attempt.min(3))); // 2,4,8,16,32,64 秒
                        //let delay = Duration::from_secs(3);
                        eprintln!("❌ 连接失败: {:?}, 重连间隔 {:?}", e.to_string(), delay);
                        sleep(delay).await;
                    }
                }
            }
        });
        eprintln!("等待连接成功通知! => connect_with_timeout");
        self.notify_connected.notified().await;
        eprintln!("收到连接成功通知! => connect_with_timeout");
    }

    pub async fn establish_connection(&self) -> anyhow::Result<()> {
        let stream = TcpStream::connect(&self.addr).await?;
        let framed = Framed::new(stream, Codec::<Packet>::default());
        let (mut socket_writer, mut socket_reader) = framed.split();
        let mut channel_rx = {
            let mut guard = self.channel_reader.lock().await;
            guard.take().expect("establish_connection called twice")
        };
        // ===== 写 socket =====
        let notify_disconnected = self.notify_disconnected.clone();
        tokio::spawn(async move {
            while let Some(packet) = channel_rx.recv().await {
                if socket_writer.send(packet).await.is_err() {
                    break;
                }
            }
            notify_disconnected.notify_waiters();          // 通知连接断线
        });
        // ===== 读 socket =====
        let notify_disconnected = self.notify_disconnected.clone();
        let pending = self.pending.clone();
        tokio::spawn(async move {
            while let Some(result) = socket_reader.next().await {
                match result {
                    Ok(Packet::Response(resp)) => {
                        if let Some(slot) = pending.remove(resp.slot_index) {
                            if resp.id == slot.request_id {
                                (slot.callback)(Packet::Response(resp));
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            notify_disconnected.notify_waiters();      // 通知连接断线
        });
        Ok(())
    }

    #[inline]
    pub async fn send<F: FnOnce(Packet) + Send + 'static>(
        &self,
        mut packet: Packet,
        callback: F,
    ) {
        if let Packet::Request(ref mut req) = packet {
            if !self.connected.load(Ordering::Acquire) {
                let resp = Response {
                    id: req.id,
                    slot_index: usize::MAX,
                    payload: Vec::new(),
                    error: Some("connection closed".into()),
                };
                callback(Packet::Response(resp));
                return
            }
            self.pending.insert(req, Box::new(callback));
            let _ = self.channel_writer.send(packet).await;
        }

    }
}
