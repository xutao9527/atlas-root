use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableReq {

}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableResp {
    pub tables: Vec<TableView>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableView {
    pub id: String,
    pub seats: Vec<bool>,
    pub small_blind_amount: u64,
    pub big_blind_amount: u64,
}