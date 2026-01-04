use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use bytes::{Bytes, BytesMut};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex, Notify};
use tokio::time::sleep;
use tokio_util::codec::Framed;
use tracing::{debug, info, warn};
use crate::net::rpc::client::pending::PendingTable;
use crate::net::rpc::codec_rmp::MsgPackCodec;
use crate::net::rpc::packet::{read_wire_header_only, write_wire_header_only};
use crate::net::rpc::packet_response::{AtlasWireResponse};

pub struct AtlasRawConnection {
    addr: String,
    channel_writer: Mutex<mpsc::Sender<Bytes>>,
    pending: Arc<PendingTable<Bytes>>,
    notify_connected: Arc<Notify>,
    notify_disconnected: Arc<Notify>,
    connected: Arc<AtomicBool>,
}

impl AtlasRawConnection {
    pub async fn new(addr: String) -> anyhow::Result<Self> {
        let pending = Arc::new(PendingTable::new(100 * 1024));
        let (channel_writer, _channel_reader) = mpsc::channel::<Bytes>(100 * 1024);
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
                            let resp = AtlasWireResponse {
                                id: slot.request_id,
                                slot_index: u64::MAX,
                                method: u32::MAX,
                                payload: Bytes::new(),
                                error: Some("connection closed".into()),
                            };
                            let buf = Bytes::from(rmp_serde::to_vec(&resp).unwrap());
                            (slot.callback)(buf);
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
        // 等待连接成功通知!
        if !self.connected.load(Ordering::SeqCst) {
            self.notify_connected.notified().await;
        }
    }

    pub async fn establish_connection(&self) -> anyhow::Result<()> {
        let stream = TcpStream::connect(&self.addr).await?;
        let framed = Framed::new(stream, MsgPackCodec::<Bytes>::default());
        let (mut socket_writer, mut socket_reader) = framed.split();

        let (channel_writer, mut channel_reader) = mpsc::channel::<Bytes>(100 * 1024);
        {
            let mut guard = self.channel_writer.lock().await;
            *guard = channel_writer.clone(); // 替换成新的 channel
        }

        self.connected.store(true, Ordering::SeqCst);                       // 标记为已连接
        self.notify_connected.notify_waiters();                                 // 通知连接成功

        // ===== 写 socket =====
        // let notify_disconnected = self.notify_disconnected.clone();
        // let connected = self.connected.clone();
        tokio::spawn(async move {
            while let Some(packet) = channel_reader.recv().await {
                if socket_writer.send(packet).await.is_err() {
                    break;
                }
            }
            // 标记为未连接 并 通知连接断线
            // if connected.swap(false, Ordering::SeqCst) {
            //     notify_disconnected.notify_waiters();
            // }
        });
        // ===== 读 socket =====
        let pending = self.pending.clone();
        let notify_disconnected = self.notify_disconnected.clone();
        let connected = self.connected.clone();
        tokio::spawn(async move {
            while let Some(result) = socket_reader.next().await {
                match result {
                    Ok(resp) => {
                        if let Ok(wire_header) = read_wire_header_only(&resp){
                            if let Some(slot) = pending.remove(wire_header.slot_index as usize) {
                                if wire_header.id == slot.request_id {
                                    (slot.callback)(resp);
                                }
                            }
                        }

                    }
                    Err(_) => break,
                }
            }
            // 标记为未连接 并 通知连接断线
            if connected.swap(false, Ordering::SeqCst) {
                notify_disconnected.notify_waiters();
            }
        });
        Ok(())
    }

    #[inline]
    pub async fn send<F: FnOnce(Bytes) + Send + 'static>(
        &self,
        req_id : u64,
        mut req: BytesMut,
        callback: F,
    ) {
        if let Ok(wire_header) = read_wire_header_only(&req) {
            if !self.connected.load(Ordering::Acquire) {
                let resp = AtlasWireResponse {
                    id: wire_header.id,
                    slot_index: u64::MAX,
                    method: wire_header.method,
                    payload: Bytes::new(),
                    error: Some("connection closed".into()),
                };
                let buf = Bytes::from(rmp_serde::to_vec(&resp).unwrap());
                callback(buf);
                return
            }
            let slot_index = self.pending.insert(req_id, Box::new(callback)) as u64;
            if let Ok(_) = write_wire_header_only(&mut req, req_id, slot_index){
                let channel_writer = {
                    let guard = self.channel_writer.lock().await;
                    guard.clone()
                };
                let _ = channel_writer.send(Bytes::from(req)).await;
            }
        }
    }
}