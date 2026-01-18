use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  LeaveTableReq{
    pub table_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  LeaveTableResp{
    pub ok: bool,
    pub message: Option<String>,
}
