use atlas_core::net::rpc::client::client::AtlasRpcClient;
use atlas_core::net::rpc::packet_header::AtlasWireHeader;
use atlas_core::net::rpc::router::AtlasModuleId;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use bytes::Bytes;
use tracing::{info};

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

    let (mut ws_tx, mut ws_rx) = socket.split();

    // === 有界响应队列（不丢）===
    let (resp_tx, mut resp_rx) = tokio::sync::mpsc::channel::<Bytes>(RESP_QUEUE);

    // === inflight 限制（核心）===
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
        let msg = match msg {
            Ok(Message::Binary(b)) => b,
            Ok(Message::Close(_)) => break,
            _ => continue,
        };

        // 校验 header
        let header = match AtlasWireHeader::read_wire_header(&msg) {
            Ok(h) => h,
            Err(_) => continue,
        };

        if AtlasModuleId::from_wire(header.method) != Some(AtlasModuleId::Auth) {
            continue;
        }

        // === 获取 inflight permit（背压在这里）===
        let permit = match inflight.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => break,
        };

        let resp_tx = resp_tx.clone();
        let client = auth_client.clone();

        // ⚠ 不 await WS，不 spawn 炸弹
        let _ = client
            .call_cb(msg, move |resp| {
                async move {
                    // 回包进入有界队列（不丢）
                    let _ = resp_tx.send(resp).await;
                    drop(permit); // 释放 inflight
                }
            })
            .await;
    }

    // === 清理 ===
    drop(resp_tx); // 通知 writer 退出
    let _ = writer.await;

    info!("WS disconnected");
}
