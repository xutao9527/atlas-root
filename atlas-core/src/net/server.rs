use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use anyhow::Result;

pub struct AtlasNetServer {
    addr: String,
}

impl AtlasNetServer {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", self.addr);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("Accepted connection from {}", addr);

            tokio::spawn(async move {
                let mut buf = vec![0u8; 1024];
                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => break, // closed
                        Ok(n) => {
                            println!("Server received: {:?}", &buf[..n]);
                            // Echo back
                            if socket.write_all(&buf[..n]).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                println!("Connection {} closed", addr);
            });
        }
    }
}