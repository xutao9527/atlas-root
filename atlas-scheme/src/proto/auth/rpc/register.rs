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