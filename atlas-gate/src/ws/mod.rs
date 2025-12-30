use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use tracing::{info, warn};


pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_ws)
}

async fn handle_ws(mut socket: WebSocket) {
    info!("WS connected");
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("recv text: {}", text);
                // echo
                if socket.send(Message::Text(format!("echo: {}", text).into())).await.is_err() {
                    break;
                }
            }
            Ok(Message::Binary(_)) => {
                warn!("binary message ignored");
            }
            Ok(Message::Close(_)) => {
                info!("WS closed by client");
                break;
            }
            Err(e) => {
                warn!("WS error: {}", e);
                break;
            }
            _ => {}
        }
    }
    info!("WS disconnected");
}