use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub type AtlasRawRequest = AtlasWireRequest<Bytes>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AtlasWireRequest<T> {
    pub id: u64,
    pub slot_index: u64,
    pub method: u32,
    pub payload: T,
}

impl<T> AtlasWireRequest<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn from_raw(raw: AtlasRawRequest) -> Result<Self, String> {
        let payload = rmp_serde::from_slice(&raw.payload.as_ref()).map_err(|e| e.to_string())?;

        Ok(Self {
            id: raw.id,
            slot_index: raw.slot_index,
            method: raw.method,
            payload,
        })
    }

    pub fn into_raw(self) -> Result<AtlasRawRequest, String> {
        let payload = rmp_serde::to_vec(&self.payload).map_err(|e| e.to_string())?;

        Ok(AtlasRawRequest {
            id: self.id,
            slot_index: self.slot_index,
            method: self.method,
            payload: Bytes::from(payload),
        })
    }
}
