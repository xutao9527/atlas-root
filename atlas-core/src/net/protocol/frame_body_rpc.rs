use serde::{Deserialize, Serialize};
use crate::net::protocol::frame::AtlasFrame;

pub(crate) type AtlasRpcResult<T> = AtlasFrame<AtlasRpcPayload<T>>;

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