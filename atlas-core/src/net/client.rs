use crate::net::packet::{Packet, Request};
use crate::net::router::{AuthMethod, RouterMethod};
use futures::SinkExt;
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio_util::codec::Framed;

#[cfg(debug_assertions)]
use crate::net::codec_json::JsonCodec as Codec;
#[cfg(not(debug_assertions))]
use crate::net::codec_rmp::MsgPackCodec as Codec;


pub struct AtlasNetClient {
    writer_tx: mpsc::Sender<(Packet, oneshot::Sender<Packet>)>,
    next_id: AtomicU64,
}

impl AtlasNetClient {
    pub async fn connect(addr: &str) -> anyhow::Result<Arc<Self>> {
        let stream = TcpStream::connect(addr).await?;
        let framed = Framed::new(stream, Codec::<Packet>::default());
        let (mut writer, mut reader) = framed.split();

        let (tx, mut rx) = mpsc::channel::<(Packet, oneshot::Sender<Packet>)>(1024);
        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Packet>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // 写任务
        let pending_write = pending.clone();
        tokio::spawn(async move {
            while let Some((packet, responder)) = rx.recv().await {
                if let Packet::Request(req) = &packet {
                    pending_write.lock().await.insert(req.id, responder);
                }
                if writer.send(packet).await.is_err() {
                    break;
                }
            }
        });

        // 读任务
        let pending_read = pending.clone();
        tokio::spawn(async move {
            while let Some(res) = reader.next().await {
                match res {
                    Ok(Packet::Response(resp)) => {
                        let mut map = pending_read.lock().await;
                        if let Some(tx) = map.remove(&resp.id) {
                            let _ = tx.send(Packet::Response(resp));
                        }
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        Ok(Arc::new(AtlasNetClient {
            writer_tx: tx,
            next_id: AtomicU64::new(1),
        }))
    }

    pub async fn call(&self, payload: Vec<u8>) -> anyhow::Result<Packet> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let request = Packet::Request(Request {
            id,
            method: AuthMethod::SignIn.wire(),
            payload,
        });

        let (resp_tx, resp_rx) = oneshot::channel();
        self.writer_tx.send((request, resp_tx)).await?;

        let packet = resp_rx.await?;
        Ok(packet)
    }
}
