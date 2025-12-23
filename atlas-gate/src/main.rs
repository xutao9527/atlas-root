use std::net::SocketAddr;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
// use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::fmt::time::LocalTime;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        //.with_max_level(tracing::Level::DEBUG)
        .init();
    let app = Router::new()
        .route("/", get(http_index))
        .route("/ws", get(ws_handler));
        //.layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Gateway listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
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