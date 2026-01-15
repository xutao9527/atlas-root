use crate::net::rpc::packet_message::AtlasWireMessage;
use serde::{Deserialize, Serialize};

pub(crate) type AtlasRpcResult<T> = AtlasWireMessage<AtlasRpcPayload<T>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: Deserialize<'de>"
))]
pub enum AtlasRpcPayload<T>
{
    Ok(T),
    Err(AtlasWireError),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtlasWireError {
    pub code: u32,
    pub message: String,
    pub data: Option<serde_value::Value>,
}