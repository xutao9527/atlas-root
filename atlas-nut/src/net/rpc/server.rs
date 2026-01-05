use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
use crate::net::rpc::codec::FrameWireCodec;
use crate::net::rpc::packet_message::AtlasRawMessage;

pub struct AtlasNetServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawMessage) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawMessage> + Send + 'static,
{
    addr: String,
    dispatch_fn: DispatchFn,
}

impl<DispatchFn, Fut> AtlasNetServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawMessage) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawMessage> + Send + 'static,

{
    pub fn new(addr: String,dispatch_fn: DispatchFn) -> Self {
        Self { addr, dispatch_fn }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        debug!("AtlasNetServer listening on {}", self.addr);
        loop {
            let (stream, addr) = listener.accept().await?;
            debug!("AtlasNetServer accepted connection from {}", addr);
            let dispatch_fn = self.dispatch_fn;
            tokio::spawn(async move {
                let mut framed = Framed::new(stream, FrameWireCodec::default());
                while let Some(result) = framed.next().await {
                    match result {
                        Ok(req) =>{
                            if let Ok(req_raw_msg) = AtlasRawMessage::from_wire_bytes(req){
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
            });
        }
    }
}