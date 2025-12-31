use crate::net::rpc::client::pending::PendingTable;
use crate::net::rpc::codec_rmp::MsgPackCodec as Codec;
use crate::net::rpc::packet::{AtlasPacket, AtlasRawRequest, AtlasRawResponse, AtlasResponse};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use bytes::Bytes;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify, mpsc};
use tokio::time::sleep;
use tokio_util::codec::Framed;
use tracing::{debug, info, warn};

pub struct AtlasConnection {
    addr: String,
    channel_writer: Mutex<mpsc::Sender<AtlasPacket>>,
    pending: Arc<PendingTable>,
    notify_connected: Arc<Notify>,
    notify_disconnected: Arc<Notify>,
    connected: Arc<AtomicBool>,
}

impl AtlasConnection {
    pub async fn new(addr: String) -> anyhow::Result<Self> {
        let pending = Arc::new(PendingTable::new(100 * 1024));
        let (channel_writer, _channel_reader) = mpsc::channel::<AtlasPacket>(100 * 1024);
        Ok(Self {
            addr: addr.to_string(),
            channel_writer: Mutex::new(channel_writer),
            pending,
            notify_connected: Arc::new(Notify::new()),
            notify_disconnected: Arc::new(Notify::new()),
            connected: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn connect(self: Arc<Self>) {
        let this = self.clone();
        tokio::spawn(async move {
            let mut attempt = 0u32;
            loop {
                match this.establish_connection().await{
                    Ok(()) => {
                        attempt = 0;
                        info!("✅ 连接成功: {}", this.addr);
                        if this.connected.load(Ordering::SeqCst) {
                            debug!("[2]等待断开连接通知! => connect_loop");
                            this.notify_disconnected.notified().await;          // 等待通知断线
                            debug!("[2]收到断开连接通知! => connect_loop");
                        }
                        this.pending.drain(|slot| {
                            let resp = AtlasResponse {
                                id: slot.request_id,
                                slot_index: usize::MAX,
                                payload: Bytes::new(),
                                error: Some("connection closed".into()),
                            };
                            (slot.callback)(resp);
                        });
                    }
                    Err(e) => {
                        attempt += 1;
                        let delay = Duration::from_secs(2u64.pow(attempt.min(3))); // 2,4,8,16,32,64 秒
                        //let delay = Duration::from_secs(3);
                        warn!("❌ 连接失败: {:?}, 重连间隔 {:?}", e.to_string(), delay);
                        sleep(delay).await;
                    }
                }
            }
        });
        if !self.connected.load(Ordering::SeqCst) {
            //eprintln!("[1]等待连接成功通知! => connect");
            self.notify_connected.notified().await;
            //eprintln!("[1]收到连接成功通知! => connect");
        }
    }

    pub async fn establish_connection(&self) -> anyhow::Result<()> {
        let stream = TcpStream::connect(&self.addr).await?;
        let framed = Framed::new(stream, Codec::<AtlasPacket>::default());
        let (mut socket_writer, mut socket_reader) = framed.split();

        let (channel_writer, mut channel_reader) = mpsc::channel::<AtlasPacket>(100 * 1024);
        {
            let mut guard = self.channel_writer.lock().await;
            *guard = channel_writer.clone(); // 替换成新的 channel
        }

        self.connected.store(true, Ordering::SeqCst);                       // 标记为已连接
        self.notify_connected.notify_waiters();                                 // 通知连接成功

        // let mut channel_rx = {
        //     let mut guard = self.channel_reader.lock().await;
        //     guard.take().expect("establish_connection called twice")
        // };
        // ===== 写 socket =====
        let notify_disconnected = self.notify_disconnected.clone();
        let connected = self.connected.clone();
        tokio::spawn(async move {
            while let Some(packet) = channel_reader.recv().await {
                if socket_writer.send(packet).await.is_err() {
                    break;
                }
            }
            connected.store(false, Ordering::SeqCst);                    // 标记为未连接
            notify_disconnected.notify_waiters();                        // 通知连接断线
        });
        // ===== 读 socket =====
        let pending = self.pending.clone();
        let notify_disconnected = self.notify_disconnected.clone();
        let connected = self.connected.clone();
        tokio::spawn(async move {
            while let Some(result) = socket_reader.next().await {
                match result {
                    Ok(AtlasPacket::AtlasResponse(resp)) => {
                        if let Some(slot) = pending.remove(resp.slot_index) {
                            if resp.id == slot.request_id {
                                (slot.callback)(resp);
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            connected.store(false, Ordering::SeqCst);           // 标记为未连接
            notify_disconnected.notify_waiters();                   // 通知连接断线
        });
        Ok(())
    }

    #[inline]
    pub async fn send<F: FnOnce(AtlasRawResponse) + Send + 'static>(
        &self,
        mut req: AtlasRawRequest,
        callback: F,
    ) {
        if !self.connected.load(Ordering::Acquire) {
            let resp = AtlasResponse {
                id: req.id,
                slot_index: usize::MAX,
                payload: Bytes::new(),
                error: Some("connection closed".into()),
            };
            callback(resp);
            return
        }
        self.pending.insert(&mut req, Box::new(callback));
        let channel_writer = {
            let guard = self.channel_writer.lock().await;
            guard.clone()
        };
        let _ = channel_writer.send(AtlasPacket::AtlasRequest(req)).await;
    }
}
