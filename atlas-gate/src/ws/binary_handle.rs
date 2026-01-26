use crate::context::session_map;
use crate::ws::ws_session::WsSession;

use atlas_scheme::module_rpc::auth_method::{BasicAuthRpc, TokenAuthRpc};
use atlas_scheme::proto::auth::rpc::AuthResp;
use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;
use ulid::Ulid;
use atlas_core::net::client::client_registry::RpcClientRegistry;
use atlas_core::net::core::{AtlasModuleId, AtlasRpcSpec};
use atlas_core::net::protocol::frame::{AtlasFrame, AtlasRawFrame};
use atlas_core::net::protocol::frame_header::AtlasFrameHeader;
use atlas_core::net::protocol::frame_kind::AtlasFrameKind;

pub async fn handle_binary_message(
    mut bin: Bytes,
    ws_session: Arc<RwLock<WsSession>>,
    client_registry: Arc<RpcClientRegistry>,
    // resp_tx: tokio::sync::mpsc::Sender<Message>,
    inflight: Arc<tokio::sync::Semaphore>,
) {
    let header = match AtlasFrameHeader::read_wire_header(&bin) {
        Ok(h) => h,
        Err(_) => return,
    };
    let module = match AtlasModuleId::from_wire(header.op_code) {
        Some(m) => m,
        None => return,
    };
    // 非 Auth 必须已认证
    if module != AtlasModuleId::Auth {
        let session_guard = ws_session.read().await;
        let authed = session_guard.is_authed();


        if !authed {
            return;
        }
        if let Some(uid) = session_guard.uid.as_ref(){
            if let Ok(ulid) = Ulid::from_string(uid) {
                bin = AtlasFrameHeader::overwrite_uid(bin,ulid.to_bytes());
            }
        }
    }
    // === 获取 inflight permit（背压在这里）===
    let permit = match inflight.acquire_owned().await {
        Ok(p) => p,
        Err(_) => return,
    };

    let is_auth_rpc = header.op_code == BasicAuthRpc::OP_CODE || header.op_code == TokenAuthRpc::OP_CODE;

    if let Some(client) = client_registry.get(module).await {
        let _ = client
            .call_cb(bin, move |resp| {
                async move {
                    // 回包进入有界队列（不丢）
                    if is_auth_rpc {
                        process_auth_resp(resp.clone(), ws_session.clone()).await;
                    }
                    let guard = ws_session.read().await;
                    guard.send_binary(resp).await;
                    // let _ = resp_tx.send(Message::Binary(resp)).await;
                    drop(permit); // 释放 inflight
                }
            })
            .await;
    }
}

pub async fn process_auth_resp(resp: Bytes, ws_session: Arc<RwLock<WsSession>>) {
    let header = match AtlasFrameHeader::read_wire_header(&resp) {
        Ok(h) => h,
        Err(_) => return,
    };
    // 只处理成功响应
    if header.kind != AtlasFrameKind::ResponseOk {
        return;
    }

    let Ok(data) = AtlasRawFrame::from_bytes(resp) else {
        return;
    };

    let Ok(auth_msg) = AtlasFrame::<AuthResp>::from_raw(data) else {
        return;
    };

    if let Some(uid) = auth_msg.body.uid.clone() {
        // === 1️⃣ 更新 ws_session 自身 ===
        {
            let mut guard = ws_session.write().await;
            guard.uid = auth_msg.body.uid;
            guard.token = auth_msg.body.token;
            guard.expire_at = auth_msg.body.expire_at;

        }
        // === 2️⃣ 注册到全局 session_map ===
        session_map().insert(uid.clone(), ws_session);
        debug!("session map insert:\n {:?}", uid)
    }
}
