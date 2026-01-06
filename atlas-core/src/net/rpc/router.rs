use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AtlasModuleId {
    Auth = 1,
    Chat = 2,
}

impl AtlasModuleId {
    #[inline]
    pub fn from_wire(wire: u32) -> Option<Self> {
        match (wire >> 16) as u16 {
            1 => Some(Self::Auth),
            2 => Some(Self::Chat),
            _ => None,
        }
    }
}

pub trait AtlasMethodSpec: Copy + 'static {
    const MODULE_ID: AtlasModuleId;
    const METHOD_ID: u16;
    const WIRE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::METHOD_ID as u32);
    type Request: Serialize + DeserializeOwned + Send + 'static;
    type Response: Serialize + DeserializeOwned + Send + 'static;
}

pub async fn handle<M, Fut>(
    raw: AtlasRawMessage,
    f: fn(AtlasWireMessage<M::Request>) -> Fut,
) -> AtlasRawMessage
where
    M: AtlasMethodSpec,
    Fut: Future<Output=AtlasWireMessage<M::Response>>,
{
    let req_msg = match AtlasWireMessage::<M::Request>::from_raw(raw.clone()) {
        Ok(r) => r,
        Err(_e) => return AtlasRawMessage {
            header: raw.header,
            payload: Bytes::new(),
        },
    };
    let resp_msg = f(req_msg).await;
    match resp_msg.into_raw() {
        Ok(resp_msg) => resp_msg,
        Err(_) => AtlasRawMessage {
            header: raw.header,
            payload: Bytes::new(),
        },
    }
}
