use crate::net::client::pending::PendingTable;
use crate::net::codec_rmp::MsgPackCodec as Codec;
use crate::net::packet::Packet;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_util::codec::Framed;

pub struct AtlasConnection {
    writer_tx: mpsc::Sender<Packet>,
    pending: Arc<PendingTable>,
}

impl AtlasConnection {
    pub async fn connect(addr: String ) -> anyhow::Result<Self> {
        let pending = Arc::new(PendingTable::new(100 * 1024));
        let (writer_tx, mut writer_rx) = mpsc::channel::<Packet>(100 * 1024);
        let stream = TcpStream::connect(addr).await?;
        let framed = Framed::new(stream, Codec::<Packet>::default());
        let (mut writer, mut reader) = framed.split();
        // ===== 写任务 =====
        tokio::spawn(async move {
            while let Some(packet) = writer_rx.recv().await {
                if writer.send(packet).await.is_err() {
                    break;
                }
            }
            // TODO: 自动重连 hook
        });
        // ===== 读任务 =====
        let pending_clone = pending.clone();
        tokio::spawn(async move {
            while let Some(result) = reader.next().await {
                match result {
                    Ok(Packet::Response(resp)) => {
                        if let Some(slot) = pending_clone.remove(resp.slot_index).await {
                            if resp.id == slot.request_id {
                                (slot.callback)(Packet::Response(resp));
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            // TODO: 自动重连 hook
        });
        Ok(Self {
            writer_tx,
            pending,
        })
    }

    #[inline]
    pub async fn send<F: FnOnce(Packet) + Send + 'static>(&mut self, mut packet: Packet,callback: F)
   {
       if let Packet::Request(ref mut req) = packet {
           self.pending.insert(req, Box::new(callback)).await;
       }
        let _ = self.writer_tx.send(packet).await;
    }
}
