use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, Mutex};
use tracing::{info, warn};
use atlas_core::AtlasModuleId;
use atlas_core::net::rpc::client::client::AtlasRpcClient;
use atlas_core::net::rpc::packet::{AtlasPacket};

pub async fn ws_handler(ws: WebSocketUpgrade,auth_client: Arc<AtlasRpcClient>) -> impl IntoResponse {

    ws.on_upgrade(move |socket| handle_ws(socket, auth_client.clone()))
}

async fn handle_ws(socket: WebSocket,auth_client: Arc<AtlasRpcClient>) {
    info!("WS connected");

    let (mut ws_tx, mut ws_rx) = socket.split();

    // 1️⃣ WS 写通道（唯一）
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<Message>();

    // 2️⃣ writer task（唯一写 socket 的地方）
    let writer = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    // 3️⃣ reader / dispatcher
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Binary(bin)) => {
                let packet: AtlasPacket = match rmp_serde::from_slice(&bin) {
                    Ok(p) => p,
                    Err(e) => {
                        warn!("decode packet failed: {}", e);
                        continue;
                    }
                };
                match packet {
                    AtlasPacket::AtlasRequest(req) => {
                        let module = match AtlasModuleId::from_wire(req.method) {
                            Some(m) => m,
                            None => {
                                warn!("unknown module wire: {}", req.method);
                                continue;
                            }
                        };
                        match module {
                            AtlasModuleId::Auth => {
                                let out = out_tx.clone();
                                let client = auth_client.clone();
                                let _ = client.call_cb(req, move |resp| {
                                    let packet = AtlasPacket::AtlasResponse(resp);
                                    info!("gateway resp {:?}", packet);
                                    let buf = rmp_serde::to_vec(&packet).unwrap();
                                    let _ = out.send(Message::binary(buf));
                                }).await;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

            }
            _ => {}
        }
    }

    drop(out_tx); // 通知 writer 退出
    let _ = writer.await;
    info!("WS disconnected");
}