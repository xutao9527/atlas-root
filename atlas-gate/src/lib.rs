mod http;
mod ws;

use crate::http::http_index;
use crate::ws::ws_handler;
use atlas_core::net::rpc::client::client::AtlasRpcClient;
use axum::Router;
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub async fn serve_gateway(bind_addr: String, bind_port: String) -> anyhow::Result<()> {
    // 1️⃣ 创建并连接 RPC Client（只做一次）
    let mut auth_client = AtlasRpcClient::new("127.0.0.1:5566".into(), 1);
    auth_client.connect().await?;
    let auth_client = Arc::new(auth_client); // 用 Arc 包裹

    let app = Router::new().route("/", get(http_index)).route(
        "/ws",
        get({
            let auth_client = auth_client.clone();
            move |ws: WebSocketUpgrade| ws_handler(ws, auth_client.clone())
        }),
    );
    let serve_addr = format!("{}:{}", bind_addr, bind_port);
    let listener = TcpListener::bind(serve_addr.clone()).await.unwrap();
    info!("Gateway listening on {}", serve_addr);
    Ok(axum::serve(listener, app).await?)
}
