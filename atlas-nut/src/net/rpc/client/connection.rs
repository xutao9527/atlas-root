use std::pin::Pin;
use crate::net::rpc::client::pending::{ PendingTable};
use crate::net::rpc::codec::FrameWireCodec;
use crate::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use crate::net::rpc::packet_message::AtlasWireMessage;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify, mpsc};
use tokio::time::sleep;
use tokio_util::codec::Framed;
use tracing::{debug, info, warn};

pub type AsyncCallback = Box<dyn FnOnce(Bytes) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub struct AtlasConnection {
    addr: String,
    pending: Arc<PendingTable<AsyncCallback>>,
    channel_writer: Mutex<mpsc::Sender<Bytes>>,
    notify_connected: Arc<Notify>,
    notify_disconnected: Arc<Notify>,
    connected: Arc<AtomicBool>,
}

impl AtlasConnection {
    pub fn new(addr: String) -> Self {
        let pending = Arc::new(PendingTable::new(100 * 1024));
        let (channel_writer, _) = mpsc::channel::<Bytes>(100 * 1024);
        Self {
            addr,
            pending,
            channel_writer: Mutex::new(channel_writer),
            notify_connected: Arc::new(Notify::new()),
            notify_disconnected: Arc::new(Notify::new()),
            connected: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn connect(self: Arc<Self>) {
        let this = self.clone();
        tokio::spawn(async move {
            let mut attempt = 0u32;
            loop {
                match this.establish_connection().await {
                    Ok(_) => {
                        attempt = 0;
                        info!("✅ 连接成功: {}", this.addr);
                        if this.connected.load(Ordering::SeqCst) {
                            debug!("[2]等待断开连接通知! => connect_loop");
                            this.notify_disconnected.notified().await; // 等待连接断线通知
                            debug!("[2]收到断开连接通知! => connect_loop");
                        }
                        let slots = this.pending.drain();
                        for slot in slots {
                            let resp_msg = AtlasWireMessage {
                                header: AtlasWireHeader {
                                    id: slot.request_id,
                                    slot_index: u32::MAX,
                                    method: u32::MAX,
                                    kind: AtlasWireKind::ResponseErr,
                                },
                                payload: Bytes::new(),
                            };
                            let resp_bytes = resp_msg.into_raw().unwrap().into_wire_bytes();
                            (slot.body)(resp_bytes).await;
                        }
                    }
                    Err(e) => {
                        attempt += 1;
                        let delay = Duration::from_secs(2u64.pow(attempt.min(3))); // 2,4,8,16,32,64 秒
                        warn!("❌ 连接失败: {:?}, 重连间隔 {:?}", e.to_string(), delay);
                        sleep(delay).await;
                    }
                }
            }
        });
        if !self.connected.load(Ordering::SeqCst) {
            self.notify_connected.notified().await; // 等待连接成功通知!
        }
    }

    pub async fn establish_connection(&self) -> anyhow::Result<()> {
        let stream = TcpStream::connect(&self.addr).await?;
        let framed = Framed::new(stream, FrameWireCodec::default());
        let (mut socket_writer, mut socket_reader) = framed.split();
        let (channel_writer, mut channel_reader) = mpsc::channel::<Bytes>(100 * 1024);
        // 替换成新的 channel_writer
        {
            let mut guard = self.channel_writer.lock().await;
            *guard = channel_writer.clone();
        }

        self.connected.store(true, Ordering::SeqCst); // 标记为已连接
        self.notify_connected.notify_waiters(); // 通知连接成功

        // ===== 写入 socket 数据 =====
        tokio::spawn(async move {
            while let Some(packet) = channel_reader.recv().await {
                if socket_writer.send(packet).await.is_err() {
                    break;
                }
            }
        });

        // ===== 读取 socket 数据 =====
        let pending = self.pending.clone();
        let notify_disconnected = self.notify_disconnected.clone();
        let connected = self.connected.clone();
        tokio::spawn(async move {
            while let Some(result) = socket_reader.next().await {
                match result {
                    Ok(packet) => {
                        if let Ok(header) = AtlasWireHeader::read_wire_header(&packet) {
                            if let Some(slot) = pending.remove(header.slot_index) {
                                if header.id == slot.request_id {
                                    (slot.body)(packet).await;
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            if connected.swap(false, Ordering::SeqCst) {
                // 标记为未连接
                notify_disconnected.notify_waiters(); // 通知连接断线
            }
        });
        Ok(())
    }

    pub async fn send<F, Fut>(&self, req_id: u64, req: Bytes, callback: F)
    where
        F: FnOnce(Bytes) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        if let Ok(header) = AtlasWireHeader::read_wire_header(&req) {
            if !self.connected.load(Ordering::Acquire) {
                let resp_msg = AtlasWireMessage {
                    header: AtlasWireHeader {
                        id: header.id,
                        slot_index: u32::MAX,
                        method: u32::MAX,
                        kind: AtlasWireKind::ResponseErr,
                    },
                    payload: Bytes::new(),
                };
                let resp_bytes = resp_msg.into_raw().unwrap().into_wire_bytes();
                callback(resp_bytes);
                return;
            }
            let slot_index = self
                .pending
                .insert(req_id, Box::new(move |resp| Box::pin(callback(resp))));
            let req_msg = AtlasWireHeader::overwrite_wire_header(req, req_id, slot_index);
            let channel_writer = {
                let guard = self.channel_writer.lock().await;
                guard.clone()
            };
            let _ = channel_writer.send(Bytes::from(req_msg)).await;
        }
    }
}
