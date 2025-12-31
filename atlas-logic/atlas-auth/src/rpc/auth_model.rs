use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginReq  {
    pub account: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResp  {
    pub ok: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}