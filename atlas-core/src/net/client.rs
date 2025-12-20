use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;

pub struct AtlasNetClient {
    addr: String,
}

impl AtlasNetClient {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }

    pub async fn run(&self) -> Result<()> {
        let mut stream = TcpStream::connect(&self.addr).await?;
        println!("Client connected to {}", self.addr);

        let msg = b"hello server";
        stream.write_all(msg).await?;
        println!("Client sent: {:?}", msg);

        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        println!("Client received: {:?}", &buf[..n]);

        Ok(())
    }
}
