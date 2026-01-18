use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SitTableReq {
    pub table_id: String,
    pub seat_index: u8,
    pub buy_in: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  SitTableResp{
    pub ok: bool,
    pub table_id: String,
    pub message: Option<String>,
}
