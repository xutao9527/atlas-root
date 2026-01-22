use std::sync::{Arc, OnceLock};
use dashmap::{DashMap, Entry};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
use crate::net::rpc::codec::FrameWireCodec;
use crate::net::rpc::notifier::{AtlasNotifyRaw, AtlasRegNodeId, Notifier};
use crate::net::rpc::packet_header::AtlasWireKind;
use crate::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};

type NotifyTx = mpsc::UnboundedSender<AtlasRawMessage>;

/// ⭐ 模块级 notifier（关键）
static GLOBAL_NOTIFIER: OnceLock<Arc<dyn Notifier>> = OnceLock::new();

pub fn set_global_notifier(n: Arc<dyn Notifier>) {
    let _ = GLOBAL_NOTIFIER.set(n);
}

pub fn global_notifier() -> Option<&'static Arc<dyn Notifier>> {
    GLOBAL_NOTIFIER.get()
}

pub struct AtlasRpcServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawMessage) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawMessage> + Send + 'static,
{
    addr: String,
    dispatch_fn: DispatchFn,
    /// 保存所有连接，用来发通知
    registry_node: Arc<DashMap<AtlasRegNodeId, NotifyTx>>,
}

impl<DispatchFn, Fut> AtlasRpcServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawMessage) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawMessage> + Send + 'static,

{
    pub fn new(addr: String, dispatch_fn: DispatchFn) -> Arc<Self> {
        let server = Arc::new(Self {
            addr,
            dispatch_fn,
            registry_node: Arc::new(DashMap::new()),
        });

        // ⭐ 注册全局 notifier（只做一次）
        set_global_notifier(server.clone() as Arc<dyn Notifier>);

        server
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        debug!("AtlasNetServer listening on {}", self.addr);
        loop {
            let (stream, addr) = listener.accept().await?;
            debug!("AtlasNetServer accepted connection from {}", addr);

            let dispatch_fn = self.dispatch_fn;
            let registry_node = self.registry_node.clone();
            let mut logical_id: Option<AtlasRegNodeId> = None;
            let mut is_owner = false;

            tokio::spawn(async move {
                let mut framed = Framed::new(stream, FrameWireCodec::default());
                // === 每个连接一个 notify channel ===
                let (notify_tx, mut notify_rx) = mpsc::unbounded_channel::<AtlasRawMessage>();
                loop {
                    select! {
                        // ===== 客户端 RPC 请求 =====
                        result = framed.next() => {
                            match result {
                                Some(Ok(req)) => {
                                    if let Ok(msg) = AtlasRawMessage::from_wire_bytes(req) {
                                        // ===== RegistryNode =====
                                        if msg.header.kind == AtlasWireKind::RegistryNode {
                                            match AtlasWireMessage::<AtlasRegNodeId>::from_raw(msg) {
                                                Ok(msg) => {
                                                    let reg_node_id = msg.payload; // ⭐ 强类型 NodeId

                                                    match registry_node.entry(reg_node_id) {
                                                        Entry::Vacant(e) => {
                                                            e.insert(notify_tx.clone());
                                                            logical_id = Some(reg_node_id);
                                                            is_owner = true;
                                                            debug!("registry_node registered: {:?}", reg_node_id);
                                                        }
                                                        Entry::Occupied(_) => {
                                                            debug!(
                                                                "registry_node already exists, ignore: {:?}",
                                                                reg_node_id
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    warn!("invalid RegistryNode payload: {:?}", e);
                                                }
                                            }
                                            continue;
                                        }
                                        // ===== RPC =====
                                        if msg.header.kind == AtlasWireKind::Request {
                                            let resp = dispatch_fn(msg).await;
                                            if framed.send(resp.into_wire_bytes()).await.is_err() {
                                                break;
                                            }
                                        }

                                    }
                                }
                                Some(Err(e)) => {
                                    warn!("decode error: {:?}", e);
                                    break;
                                }
                                None => {
                                    // ⭐ 这里就是 TCP 断开
                                    debug!("AtlasNetServer connection closed");
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
                if is_owner {
                    if let Some(reg_node_id) = logical_id {
                        registry_node.remove(&reg_node_id);
                        debug!("registry_node removed: {:?}", reg_node_id);
                    }
                }
            });
        }
    }
}

impl<DispatchFn, Fut> Notifier for AtlasRpcServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawMessage) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawMessage> + Send + 'static,
{
    fn notify(&self, reg_node_id: &AtlasRegNodeId, msg: AtlasWireMessage<AtlasNotifyRaw>) -> bool {
        if let Some(notify_tx) = self.registry_node.get(reg_node_id) {
            match msg.into_raw() {
                Ok(notify) => {
                    notify_tx.send(notify).is_ok()
                }
                Err(_) => {
                    false
                }
            }

        } else {
            false
        }
    }
}