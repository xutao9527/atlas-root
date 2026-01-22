use std::sync::Arc;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
use crate::net::rpc::codec::FrameWireCodec;
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
    connections: Arc<DashMap<String, NotifyTx>>,
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
            connections:
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
            let connections = self.connections.clone();
            let conn_id = addr.to_string();

            tokio::spawn(async move {
                let mut framed = Framed::new(stream, FrameWireCodec::default());
                // === 每个连接一个 notify channel ===
                let (notify_tx, mut notify_rx) = mpsc::unbounded_channel::<AtlasRawMessage>();
                connections.insert(conn_id.clone(), notify_tx);
                debug!("connection established: {:?}",connections);
                loop {
                    select! {
                        // ===== 客户端 RPC 请求 =====
                        Some(result) = framed.next() => {
                            match result {
                                Ok(req) => {
                                    if let Ok(req_raw_msg) = AtlasRawMessage::from_wire_bytes(req) {
                                        let resp = dispatch_fn(req_raw_msg).await;
                                        if framed.send(resp.into_wire_bytes()).await.is_err() {
                                            break;
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
                connections.remove(&conn_id);
                debug!("connection closed: {}", conn_id);
            });
        }
    }
}