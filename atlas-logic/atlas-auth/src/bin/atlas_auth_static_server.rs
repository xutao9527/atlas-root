use atlas_core::net::rpc::packet_request::{AtlasRawRequest, AtlasWireRequest};
use atlas_core::net::rpc::packet_response::{AtlasRawResponse, AtlasWireResponse};
use atlas_core::net::rpc::router::handle;
use atlas_core::net::rpc::server::AtlasNetServer;
use atlas_core::{AtlasMethodSpec, AtlasModuleId};
use serde::{Deserialize, Serialize};
use tokio_util::bytes::Bytes;
use tracing_subscriber::fmt::time::LocalTime;

// -------------------------- 数据结构 --------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterReq { pub account: String, pub password: String }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterResp { pub ok: bool, pub error: Option<String> }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginReq { pub account: String, pub password: String }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResp { pub ok: bool, pub token: Option<String>, pub error: Option<String> }

// -------------------------- 方法实现 --------------------------
pub async fn register(req: AtlasWireRequest<RegisterReq>) -> AtlasWireResponse<RegisterResp> {
    AtlasWireResponse {
        id: req.id,
        slot_index: req.slot_index,
        payload: RegisterResp { ok: true, error: None },
        error: None,
    }
}

pub async fn login(req: AtlasWireRequest<LoginReq>) -> AtlasWireResponse<LoginResp> {
    let token = format!("{}|{}", req.payload.account, req.payload.password);
    AtlasWireResponse {
        id: req.id,
        slot_index: req.slot_index,
        payload: LoginResp { ok: true, token: Some(token), error: None },
        error: None,
    }
}

// -------------------------- 方法类型 --------------------------
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Register;
impl AtlasMethodSpec for Register {
    const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
    const METHOD_ID: u16 = 1;
    type Request = RegisterReq;
    type Response = RegisterResp;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Login;
impl AtlasMethodSpec for Login {
    const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
    const METHOD_ID: u16 = 2;
    type Request = LoginReq;
    type Response = LoginResp;
}


// -------------------------- 静态 dispatch --------------------------
async fn dispatch(raw: AtlasRawRequest) -> AtlasRawResponse {
    match raw.method {
        Register::WIRE => handle::<Register, _>(raw, register).await,
        Login::WIRE => handle::<Login, _>(raw, login).await,
        _ => AtlasRawResponse {
            id: raw.id,
            slot_index: raw.slot_index,
            payload: Bytes::new(),
            error: Some("method not found".into()),
        },
    }
}

// -------------------------- main --------------------------
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    let serve_addr = format!("{}:{}", "0.0.0.0", "5566");
    let server = AtlasNetServer::new(serve_addr, dispatch);
    server.run().await?;
    Ok(())
}
