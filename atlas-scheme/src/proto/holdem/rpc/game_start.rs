use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameStartReq {
    pub table_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameStartResp {
    pub ok: bool,
    pub message: Option<String>,
}

