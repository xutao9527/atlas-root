pub mod ws_session;
mod binary_handle;
use atlas_core::net::rpc::client::client::AtlasRpcClient;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use bytes::Bytes;
use tokio::sync::mpsc::channel;
use tokio::sync::RwLock;
use tracing::{info};
use crate::ws::binary_handle::handle_binary_message;
use crate::ws::ws_session::{ WsSession};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    auth_client: Arc<AtlasRpcClient>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, auth_client.clone()))
}

const MAX_INFLIGHT: usize = 8192;   // 每 WS 连接最大 RPC 并发
const RESP_QUEUE: usize = 8192;     // 回包队列

async fn handle_ws(socket: WebSocket, auth_client: Arc<AtlasRpcClient>) {
    info!("WS connected");
    // 独立会话
    let ws_session = Arc::new(RwLock::new(WsSession::new()));
    // ws socket
    let (mut ws_tx, mut ws_rx) = socket.split();
    // === 有界响应队列===
    let (resp_tx, mut resp_rx) = channel::<Bytes>(RESP_QUEUE);
    // === inflight ===
    let inflight = Arc::new(tokio::sync::Semaphore::new(MAX_INFLIGHT));
    // ===== writer：唯一 socket IO =====
    let writer = tokio::spawn(async move {
        while let Some(resp) = resp_rx.recv().await {
            if ws_tx.send(Message::Binary(resp)).await.is_err() {
                break;
            }
        }
    });
    // ===== reader：解析 + RPC =====
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Binary(bin)) => {
                handle_binary_message(bin, ws_session.clone(), auth_client.clone(), resp_tx.clone(), inflight.clone()).await;
            }
            Ok(Message::Text(txt)) => {
                info!("WS received text message: {}", txt);
            },
            Ok(Message::Ping(msg)) => {
                info!("WS received ping message: {:?}", msg);
            }
            Ok(Message::Pong(_)) => {
                info!("WS received pong message");
            }
            Ok(Message::Close(_)) => {
                info!("WS received close message");
            }
            Err(_) => {}
        }
    }
    drop(resp_tx);
    let _ = writer.await;
    info!("WS disconnected");
}
