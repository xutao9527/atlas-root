mod http;
mod ws;
mod context;

use crate::http::http_index;
use crate::ws::ws_handler;

use axum::Router;
use axum::extract::WebSocketUpgrade;
use axum::routing::get;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use atlas_core::net::rpc::client::client::AtlasRpcClient;
use atlas_core::net::rpc::client_registry::RpcClientRegistry;
use atlas_core::net::rpc::notifier::AtlasRegNodeId;
use atlas_core::net::rpc::router::AtlasModuleId;

pub async fn serve_gateway(bind_addr: String, bind_port: String) -> anyhow::Result<()> {
    // 1️⃣ 创建并连接 RPC Client（只做一次）
    let mut auth_client = AtlasRpcClient::new("127.0.0.1:5566".into(), AtlasRegNodeId::AuthNode(1),1);
    auth_client.connect().await?;
    let mut holdem_client = AtlasRpcClient::new("127.0.0.1:6677".into(), AtlasRegNodeId::HoldemNode(1),1);
    holdem_client.connect().await?;


    let registry = RpcClientRegistry::new();
    registry.register(AtlasModuleId::Auth, auth_client).await;
    registry.register(AtlasModuleId::Holdem, holdem_client).await;

    let client_registry = Arc::new(registry); // 用 Arc 包裹

    let app = Router::new()
        .route("/", get(http_index))
        .route("/ws",get(move |ws: WebSocketUpgrade| ws_handler(ws, client_registry.clone())));
    
    let serve_addr = format!("{}:{}", bind_addr, bind_port);
    let listener = TcpListener::bind(serve_addr.clone()).await.unwrap();
    info!("Gateway listening on {}", serve_addr);
    Ok(axum::serve(listener, app).await?)
}
