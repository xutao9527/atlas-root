use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterReq {
    pub account: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterResp {
    pub ok: bool,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicAuthReq {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAuthReq {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResp {
    pub ok: bool,
    pub uid: Option<String>,
    pub token: Option<String>,
    pub expire_at: Option<u64>,
    pub error: Option<String>,
}
