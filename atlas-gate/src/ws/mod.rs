use atlas_nut::net::rpc::client::client::AtlasRpcClient;
use atlas_nut::net::rpc::packet_header::AtlasWireHeader;
use atlas_nut::net::rpc::router::AtlasModuleId;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn};
use atlas_nut::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};
use atlas_scheme::dto::auth_model::LoginResp;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    auth_client: Arc<AtlasRpcClient>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, auth_client.clone()))
}

async fn handle_ws(socket: WebSocket, auth_client: Arc<AtlasRpcClient>) {
    info!("WS connected");

    // 有界队列：限制内存 + 允许背压
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (out_tx, mut out_rx) = mpsc::channel::<Message>(100 * 1024);

    // ===== writer task（唯一写 socket 的地方）=====
    let writer = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    // ===== reader loop（高 QPS 核心）=====
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let _ = out_tx.send(Message::Text(text)).await;
            }
            Ok(Message::Binary(bin)) => {
                if let Ok(header) = AtlasWireHeader::read_wire_header(&bin) {
                    match AtlasModuleId::from_wire(header.method) {
                        Some(module_id) => match module_id {
                            AtlasModuleId::Auth => {
                                let sender = out_tx.clone();
                                let client = auth_client.clone();
                                let _ = client.call_cb(bin, |resp| async move {
                                    // let raw_msg = AtlasRawMessage::from_wire_bytes(resp.clone());
                                    // let resp_msg = AtlasWireMessage::<LoginResp>::from_raw(raw_msg.unwrap());
                                    // println!("{:?}", resp_msg);
                                    let _ = sender.send(Message::binary(resp)).await;
                                }).await;
                            }
                            _ => {}
                        }
                        None => {
                            warn!("unknown module wire: {}", header.method);
                            continue;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            _ => {}
        }
    }

    drop(out_tx); // 通知 writer 退出
    let _ = writer.await;
    info!("WS disconnected");
}
