use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameActReq {
    pub table_id: String,
    // pub act: GameAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameActResp {
    pub ok: bool,
    pub message: Option<String>,
}
