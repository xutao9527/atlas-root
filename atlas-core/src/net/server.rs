use crate::net::codec_rmp::MsgPackCodec as Codec;
use crate::net::packet::Packet;

use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
use crate::net::router::AtlasRouter;

pub struct AtlasNetServer {
    addr: String,
    router: Arc<AtlasRouter>,
}

impl AtlasNetServer {
    pub fn new(addr: &str, router: AtlasRouter) -> Self {
        Self {
            addr: addr.to_string(),
            router: Arc::new(router),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        debug!("AtlasNetServer listening on {}", self.addr);
        loop {
            let (stream, addr) = listener.accept().await?;
            debug!("AtlasNetServer accepted connection from {}", addr);
            let router = self.router.clone(); // Arc Router
            tokio::spawn(async move {
                let mut framed = Framed::new(stream, Codec::<Packet>::default());
                while let Some(result) = framed.next().await {
                    match result {
                        Ok(Packet::Request(req)) => {
                            //println!("Server received: {:?}", req);
                            let resp = router.dispatch(req).await;
                            if framed.send(Packet::Response(resp)).await.is_err() {
                                break;
                            }
                        }
                        Ok(_) => {}
                        Err(e) => {
                            //eprintln!("Connection {} closed: {}", addr, e);
                            warn!("decode error: {:?}", e);
                            break;
                        }
                    }
                }
                warn!("connection {} closed", addr);
            });
        }
    }
}
