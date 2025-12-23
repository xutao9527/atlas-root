use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:8080/ws";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to WebSocket server");
    let (mut write, mut read) = ws_stream.split();

    // 发送一条文本消息
    write.send(Message::Text("aa".into())).await.expect("send text failed");

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