use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AtlasPacket {
    AtlasRequest(AtlasRequest),
    AtlasResponse(AtlasResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtlasRequest {
    pub id: u64,
    pub slot_index: usize,
    pub method: u32,
    pub payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtlasResponse {
    pub id: u64,
    pub slot_index: usize,
    pub payload: Vec<u8>,
    pub error: Option<String>,
}