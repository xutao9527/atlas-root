use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tracing::{info, warn};

pub async fn serve_gateway(bind_addr: String, bind_port: String) -> anyhow::Result<()> {

    let app = Router::new()
        .route("/", get(http_index))
        .route("/ws", get(ws_handler));

    let addr = format!("{}:{}", bind_addr, bind_port);

    let listener = TcpListener::bind(addr.clone()).await.unwrap();
    info!("Gateway listening on {}", addr);
    Ok(axum::serve(listener, app).await?)

}

async fn http_index() -> impl IntoResponse {
    "Hello"
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
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