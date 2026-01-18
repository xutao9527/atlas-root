use serde::{Deserialize, Serialize};
use crate::proto::holdem::types::PlayerActionKind;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameActReq {
    pub table_id: String,
    pub act: PlayerActionKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameActResp {
    pub ok: bool,
    pub message: Option<String>,
}
