use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;


pub type AtlasRawResponse = AtlasWireResponse<Bytes>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtlasWireResponse<T> {
    pub id: u64,
    pub slot_index: u64,
    pub method: u32,
    pub payload: T,
    pub error: Option<String>,
}

impl<T> AtlasWireResponse<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn from_raw(raw: AtlasRawResponse) -> Result<Self, String> {
        let payload = rmp_serde::from_slice(&raw.payload.as_ref()).map_err(|e| e.to_string())?;
        Ok(Self {
            id: raw.id,
            slot_index: raw.slot_index,
            method: raw.method,
            payload,
            error:raw.error,
        })
    }

    pub fn into_raw(self) -> AtlasRawResponse {
        let payload = rmp_serde::to_vec(&self.payload).unwrap_or_default();
        AtlasRawResponse {
            id: self.id,
            slot_index: self.slot_index,
            method: self.method,
            payload: Bytes::from(payload), // 👈 只拷一次
            error: self.error,
        }
    }
}