use crate::ws::ws_session::WsSession;
use atlas_core::net::rpc::client::client::AtlasRpcClient;
use atlas_core::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_core::net::rpc::router::{AtlasModuleId, AtlasRpcSpec};
use atlas_scheme::dto::auth_model::AuthResp;
use atlas_scheme::module_method::auth_method::{BasicAuthRpc, TokenAuthRpc};
use bytes::Bytes;
use std::sync::Arc;

pub async fn handle_binary_message(
    bin: Bytes,
    ws_session: Arc<tokio::sync::RwLock<WsSession>>,
    auth_client: Arc<AtlasRpcClient>,
    resp_tx: tokio::sync::mpsc::Sender<Bytes>,
    inflight: Arc<tokio::sync::Semaphore>,
) {
    let header = match AtlasWireHeader::read_wire_header(&bin) {
        Ok(h) => h,
        Err(_) => return,
    };
    let module = match AtlasModuleId::from_wire(header.method) {
        Some(m) => m,
        None => return,
    };
    // 非 Auth 必须已认证
    if module != AtlasModuleId::Auth {
        let authed = ws_session.read().await.is_authed();
        if !authed {
            return;
        }
    }
    // === 获取 inflight permit（背压在这里）===
    let permit = match inflight.acquire_owned().await {
        Ok(p) => p,
        Err(_) => return,
    };

    let is_auth_rpc = header.method == BasicAuthRpc::WIRE || header.method == TokenAuthRpc::WIRE;
    let _ = auth_client
        .call_cb(bin, move |resp| {
            async move {
                // 回包进入有界队列（不丢）
                if is_auth_rpc {
                    process_auth_resp(resp.clone(), ws_session).await;
                }
                let _ = resp_tx.send(resp).await;
                drop(permit); // 释放 inflight
            }
        })
        .await;
}

pub async fn process_auth_resp(resp: Bytes, ws_session: Arc<tokio::sync::RwLock<WsSession>>) {
    let header = match AtlasWireHeader::read_wire_header(&resp) {
        Ok(h) => h,
        Err(_) => return,
    };
    // 只处理成功响应
    if header.kind != AtlasWireKind::ResponseOk {
        return;
    }
    if let Ok(data) = AtlasWireMessage::from_wire_bytes(resp) {
        if let Ok(auth_resp) = AtlasWireMessage::<AuthResp>::from_raw(data) {
            let mut guard = ws_session.write().await;
            guard.uid = auth_resp.payload.uid;
            guard.token = auth_resp.payload.token;
            guard.expire_at = auth_resp.payload.expire_at;
        }
    }
}
