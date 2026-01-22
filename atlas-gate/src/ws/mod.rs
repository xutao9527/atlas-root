mod binary_handle;
pub mod ws_session;
use crate::ws::binary_handle::{handle_binary_message, process_auth_resp};
use crate::ws::ws_session::WsSession;
use atlas_core::net::rpc::client::client::AtlasRpcClient;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::select;
use tokio::sync::RwLock;
use tokio::sync::mpsc::channel;
use tracing::{info};
use atlas_core::net::rpc::client_registry::RpcClientRegistry;
use atlas_core::net::rpc::router::{AtlasModuleId, AtlasRpcSpec};
use atlas_scheme::proto::auth::rpc::{TokenAuthReq};
use atlas_scheme::module_method::auth_method::TokenAuthRpc;
use crate::context::session_map;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    client_registry: Arc<RpcClientRegistry>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, client_registry.clone()))
}

const MAX_INFLIGHT: usize = 8192; // 每 WS 连接最大 RPC 并发
const RESP_QUEUE: usize = 8192; // 回包队列
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);//心跳定时器

async fn handle_ws(socket: WebSocket, client_registry: Arc<RpcClientRegistry>) {
    info!("WS connected");
    // ws socket
    let (mut ws_tx, mut ws_rx) = socket.split();
    // === 有界响应队列===
    let (resp_tx, mut resp_rx) = channel::<Message>(RESP_QUEUE);
    // === inflight ===
    let inflight = Arc::new(tokio::sync::Semaphore::new(MAX_INFLIGHT));
    // 独立会话
    let ws_session = Arc::new(RwLock::new(WsSession::new(resp_tx)));
    // 心跳
    let mut heartbeat = tokio::time::interval(HEARTBEAT_INTERVAL);
    // ===== writer：唯一 socket IO =====
    let writer = tokio::spawn(async move {
        while let Some(resp) = resp_rx.recv().await {
            if ws_tx.send(resp).await.is_err() {
                break;
            }
        }
    });

    loop {
        select! {
            ws_msg = ws_rx.next() => {
                match ws_msg {
                    Some(msg) => {
                        match msg {
                            Ok(Message::Binary(bin)) => {
                                handle_binary_message(
                                    bin,
                                    ws_session.clone(),
                                    client_registry.clone(),
                                    inflight.clone(),
                                )
                                .await;
                            }
                            Ok(Message::Text(txt)) => {
                                info!("WS received text message: {}", txt);
                                // let _ = resp_tx.send(Message::Text(txt)).await;
                                let guard = ws_session.read().await;
                                guard.send_text_bytes(txt).await;
                            }
                            Ok(Message::Ping(_)) => {
                                info!("WS received ping message: {:?}", msg);
                            }
                            Ok(Message::Pong(_)) => {
                                info!("WS received pong message");
                            }
                            Ok(Message::Close(_)) => {
                                info!("WS received close message");
                                break
                            }
                            Err(_) => { break }
                        }
                    }
                    None => {},
                }
            }
            _ = heartbeat.tick() => {
                let  guard = ws_session.read().await;
                if let (Some(token), Some(expire_at_unix)) = (&guard.token, guard.expire_at) {
                    let now_unix = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if expire_at_unix.saturating_sub(now_unix) < 3600 {
                        let token = token.clone();
                        if let Some(client) = client_registry.get(AtlasModuleId::Auth).await {
                            let session = ws_session.clone();
                            tokio::spawn(async move {
                                 refresh_token_if_needed(token.as_str(), client, session).await;
                            });
                        }
                    }
                    // info!("now_unix => expire_at_unix: {:?} - {:?} = {:?}s", expire_at_unix ,now_unix ,expire_at_unix - now_unix);
                }
            }
        }

    }
    // === 断线清理 session_map ===
    {
        let guard = ws_session.read().await;
        if let Some(uid) = guard.uid.as_ref() {
            session_map().remove(uid);
        }
    }
    drop(ws_session);
    let _ = writer.await;
    info!("WS disconnected");
}

// 续签 token
async fn refresh_token_if_needed(
    token: &str,
    auth_client: Arc<AtlasRpcClient>,
    ws_session: Arc<RwLock<WsSession>>,
) {
    let req = TokenAuthRpc::build_request(TokenAuthReq {
        token: token.to_string(),
    }).unwrap();
    let bytes = req.into_wire_bytes();

    auth_client.call_cb(bytes, move |resp| {
        let ws_session = ws_session.clone();
        async move {
            process_auth_resp(resp, ws_session).await;
        }
    }).await;

}