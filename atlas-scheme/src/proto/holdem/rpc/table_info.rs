use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableInfoReq{
    pub table_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableInfoResp {
    pub table_id: String,
}
