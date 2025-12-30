use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use atlas_core::net::packet::{Packet, Request};
use atlas_core::net::router::auth::AuthMethod;
use atlas_core::net::router::RouterMethod;

#[tokio::main]
async fn main() {
    let ws_server_addr = "ws://127.0.0.1:8080/ws";

    let (ws_stream, _) = connect_async(ws_server_addr).await.expect("Failed to connect");

    println!("Connected to WebSocket server");
    let (mut write, mut read) = ws_stream.split();

    // 发送一条文本消息
    write.send(Message::Text("aa".into())).await.expect("send text failed");
    // 发送二进制消息
    let packet = Packet::Request(Request {
        id: 1,
        slot_index: 1,
        method: AuthMethod::SignIn.wire(),
        payload: vec![],
    });
    let vec  = rmp_serde::to_vec(&packet).expect("serialize failed");
    let bytes = bytes::Bytes::from(vec);
    write.send(Message::Binary(bytes)).await.expect("send binary failed");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text);
            }
            Ok(Message::Close(_)) => {
                println!("Server closed connection");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
