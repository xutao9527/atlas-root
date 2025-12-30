mod ws;
mod http;

use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tracing::{info};
use crate::http::http_index;
use crate::ws::ws_handler;

pub async fn serve_gateway(bind_addr: String, bind_port: String) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(http_index))
        .route("/ws", get(ws_handler));
    let addr = format!("{}:{}", bind_addr, bind_port);
    let listener = TcpListener::bind(addr.clone()).await.unwrap();
    info!("Gateway listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}




