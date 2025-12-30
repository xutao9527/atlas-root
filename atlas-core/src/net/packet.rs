use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Packet {
    Request(Request),
    Response(Response),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub id: u64,
    pub slot_index: usize,
    pub method: u32,
    pub payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub id: u64,
    pub slot_index: usize,
    pub payload: Vec<u8>,
    pub error: Option<String>,
}