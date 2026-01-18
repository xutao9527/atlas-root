use serde::{Deserialize, Serialize};
use crate::proto::holdem::types::TableView;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableListReq {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetTableListResp {
    pub tables: Vec<TableView>,
}

