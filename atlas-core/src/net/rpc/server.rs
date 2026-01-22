use std::sync::Arc;
use dashmap::{DashMap, Entry};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
use crate::net::rpc::codec::FrameWireCodec;
use crate::net::rpc::packet_header::AtlasWireKind;
use crate::net::rpc::packet_message::AtlasRawMessage;

type NotifyTx = mpsc::UnboundedSender<AtlasRawMessage>;

pub struct AtlasRpcServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawMessage) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawMessage> + Send + 'static,
{
    addr: String,
    dispatch_fn: DispatchFn,
    /// 保存所有连接，用来发通知
    registry_node: Arc<DashMap<String, NotifyTx>>,
}

impl<DispatchFn, Fut> AtlasRpcServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawMessage) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawMessage> + Send + 'static,

{
    pub fn new(addr: String, dispatch_fn: DispatchFn) -> Self {
        Self {
            addr,
            dispatch_fn,
            registry_node:
            Arc::new(DashMap::new()),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        debug!("AtlasNetServer listening on {}", self.addr);
        loop {
            let (stream, addr) = listener.accept().await?;
            debug!("AtlasNetServer accepted connection from {}", addr);

            let dispatch_fn = self.dispatch_fn;
            let registry_node = self.registry_node.clone();
            let mut logical_id: Option<String> = None;

            tokio::spawn(async move {
                let mut framed = Framed::new(stream, FrameWireCodec::default());
                // === 每个连接一个 notify channel ===
                let (notify_tx, mut notify_rx) = mpsc::unbounded_channel::<AtlasRawMessage>();
                loop {
                    select! {
                        // ===== 客户端 RPC 请求 =====
                        Some(result) = framed.next() => {
                            match result {
                                Ok(req) => {
                                    if let Ok(msg) = AtlasRawMessage::from_wire_bytes(req) {
                                         // ===== RegistryNode =====
                                        if msg.header.kind == AtlasWireKind::RegistryNode {
                                            if let Ok(id) = std::str::from_utf8(&msg.payload) {
                                                let id = id.to_string();
                                                match registry_node.entry(id.clone()) {
                                                    Entry::Vacant(e) => {
                                                        e.insert(notify_tx.clone());
                                                        logical_id = Some(id.clone());
                                                        debug!("registry_node registered: {}", id);
                                                    }
                                                    Entry::Occupied(_) => {
                                                        debug!("registry_node already exists, ignore: {}", id);
                                                    }
                                                }
                                            }
                                            continue;
                                        }
                                        // ===== 普通 RPC =====
                                        if msg.header.kind == AtlasWireKind::Request {
                                            let resp = dispatch_fn(msg).await;
                                            if framed.send(resp.into_wire_bytes()).await.is_err() {
                                                break;
                                            }
                                        }

                                    }
                                }
                                Err(e) => {
                                    warn!("decode error: {:?}", e);
                                    break;
                                }
                            }
                        }
                        // ===== 服务器主动通知 =====
                        Some(notify) = notify_rx.recv() => {
                            if framed.send(notify.into_wire_bytes()).await.is_err() {
                                break;
                            }
                        }
                    }
                }
                // ===== 连接断开：只有 owner 才能 remove =====
                if let Some(id) = logical_id {
                    if let Some(entry) = registry_node.get(&id) {
                        // 只有当前连接仍是 owner，才有权删除
                        if std::ptr::eq(entry.value(), &notify_tx) {
                            registry_node.remove(&id);
                            debug!("registry_node removed: {}", id);
                        }
                    }
                }

            });
        }
    }
}