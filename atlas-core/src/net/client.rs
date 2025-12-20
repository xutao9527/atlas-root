use crate::net::packet::{Packet, Request};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[cfg(debug_assertions)]
use crate::net::codec_json::JsonCodec as Codec;

#[cfg(not(debug_assertions))]
use crate::net::codec_rmp::MsgPackCodec as Codec;
use crate::net::router::{AuthMethod, RouterMethod};

pub struct AtlasNetClient {
    addr: String,
}

impl AtlasNetClient {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let stream = TcpStream::connect(&self.addr).await?;
        println!("Client connected to {}", self.addr);

        let mut framed = Framed::new(stream, Codec::<Packet>::default());

        let msg = Packet::Request(Request {
            id: 1,
            method: AuthMethod::SignIn.wire(),
            payload: b"hello server".to_vec(),
        });

        framed.send(msg).await?;
        println!("Client sent message");

        if let Some(res) = framed.next().await {
            match res {
                Ok(pkt) => println!("Client received: {:?}", pkt),
                Err(e) => eprintln!("Decode error: {:?}", e),
            }
        }

        Ok(())
    }
}
