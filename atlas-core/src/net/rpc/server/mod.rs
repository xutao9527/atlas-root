use crate::net::rpc::codec::FrameWireCodec;
use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
use crate::net::rpc::packet_request::AtlasRawRequest;
use crate::net::rpc::packet_response::AtlasRawResponse;

pub struct AtlasNetServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawRequest) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawResponse> + Send + 'static,
{
    addr: String,
    dispatch_fn: DispatchFn,
}

impl<DispatchFn, Fut> AtlasNetServer<DispatchFn, Fut>
where
    DispatchFn: Fn(AtlasRawRequest) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = AtlasRawResponse> + Send + 'static,

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
            tokio::spawn(async move {
                let mut framed = Framed::new(stream, FrameWireCodec::default());
                while let Some(result) = framed.next().await {
                    match result {
                        Ok(_req) =>{

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
