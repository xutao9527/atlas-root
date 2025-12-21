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
}

impl AtlasConnection {
    pub async fn connect(addr: String, pending: Arc<PendingTable>) -> anyhow::Result<Self> {
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
        tokio::spawn(async move {
            while let Some(result) = reader.next().await {
                match result {
                    Ok(Packet::Response(resp)) => {
                        if let Some(slot) = pending.remove(resp.slot_index).await {
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
        Ok(Self { writer_tx })
    }

    #[inline]
    pub async fn send(&self, packet: Packet) {
        let _ = self.writer_tx.send(packet).await;
    }
}
