use crate::net::packet::Packet;
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;

#[cfg(debug_assertions)]
use crate::net::codec_json::JsonCodec as Codec;

#[cfg(not(debug_assertions))]
use crate::net::codec_rmp::MsgPackCodec as Codec;

pub struct AtlasNetServer {
    addr: String,
}

impl AtlasNetServer {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", self.addr);

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("Accepted connection from {}", addr);

            tokio::spawn(async move {
                let mut framed = Framed::new(stream, Codec::<Packet>::default());

                while let Some(result) = framed.next().await {
                    match result {
                        Ok(pkt) => {
                            println!("Server received: {:?}", pkt);
                            // Echo back
                            if framed.send(pkt).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Decode error: {:?}", e);
                            break;
                        }
                    }
                }
                println!("Connection {} closed", addr);
            });
        }
    }
}
