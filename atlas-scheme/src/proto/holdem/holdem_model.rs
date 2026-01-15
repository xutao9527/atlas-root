use serde::{Deserialize, Serialize};

// ================================================== GetTable ==================================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableReq {}

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

// ================================================== SitTable ==================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SitTableReq {
    pub table_id: String,
    pub seat_index: u8,
    pub buy_in: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  SitTableResp{
    pub ok: bool,
    pub message: Option<String>,
}


// ================================================== LeaveTable ==================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  LeaveTableReq{
    pub table_id: String,
    pub seat_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  LeaveTableResp{
    pub ok: bool,
    pub message: Option<String>,
}


// ================================================== GameAct ==================================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameAction {
    Fold,
    Call,
    Check,
    Bet(u64),
    Raise(u64),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameActReq {
    pub table_id: String,
    pub act :GameAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameActResp {
    pub ok: bool,
    pub message: Option<String>,
}

