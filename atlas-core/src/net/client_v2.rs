use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_util::codec::Framed;
use crate::net::codec_rmp::MsgPackCodec;
use crate::net::packet::{Packet, Request};
use crate::net::router::{AuthMethod, RouterMethod};

pub struct AtlasNetClientV2 {
    addr: String,
    pub writer_tx: mpsc::Sender<Packet>,
    reader_rx: Option<mpsc::Receiver<Packet>>, // 用 Option 包裹
}

impl AtlasNetClientV2 {
    pub fn new(addr: &str) -> Self {
        let (writer_tx, reader_rx) = mpsc::channel::<Packet>(4096);
        Self {
            addr: addr.to_string(),
            writer_tx,
            reader_rx: Some(reader_rx), // 放到 Some 里
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let stream = TcpStream::connect(self.addr.clone()).await?;
        let framed = Framed::new(stream, MsgPackCodec::<Packet>::default());
        let (mut writer, mut reader) = framed.split();
        // ===== 写任务 =====
        if let Some(mut reader) = self.reader_rx.take() {
            tokio::spawn(async move {
                while let Some(packet) = reader.recv().await {
                    if writer.send(packet).await.is_err() {
                        break;
                    }
                }
            });
        }
        // ===== 读任务 =====
        tokio::spawn(async move {
            while let Some(result) = reader.next().await {
                match result {
                    Ok(Packet::Response(resp)) => {
                        println!("Received Response: {:?}", resp);
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });
        Ok(())
    }

    pub async fn send(&mut self)  {
        let packet = Packet::Request(Request {
            id: 1,
            method: AuthMethod::SignIn.wire(),
            payload:vec![],
        });
        let _ = self.writer_tx.send(packet).await;
    }
}
