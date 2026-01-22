use axum::extract::ws::Message;
use bytes::Bytes;
use tracing::debug;
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use crate::context::session_map;

pub async fn notify_handler(notify_msg: Bytes) {
    let bytes = AtlasWireMessage::from_wire_bytes(notify_msg.clone());
    println!("notify_handler: {:?}", bytes);
    for entry in session_map().iter() {
        let session = entry.value().clone();
        let msg = notify_msg.clone(); // Bytes 必须 clone
        let ws = session.read().await;
        if let Err(e) = ws.msg_tx.send(Message::Binary(msg)).await {
            debug!("send notify failed: {:?}", e);
        }
    }
}