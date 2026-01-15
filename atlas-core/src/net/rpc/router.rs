use crate::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use crate::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};
use crate::net::rpc::packet_payload::{AtlasRpcPayload, AtlasRpcResult};
use bytes::Bytes;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AtlasModuleId {
    Auth = 1,
    Chat = 2,
    Holdem = 3,
}

impl AtlasModuleId {
    #[inline]
    pub fn from_wire(wire: u32) -> Option<Self> {
        match (wire >> 16) as u16 {
            1 => Some(Self::Auth),
            2 => Some(Self::Chat),
            3 => Some(Self::Holdem),
            _ => None,
        }
    }
}

pub trait AtlasRpcSpec: Copy + 'static {
    const MODULE_ID: AtlasModuleId;
    const METHOD_ID: u16;
    const WIRE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::METHOD_ID as u32);
    type Request: Serialize + DeserializeOwned + Send + 'static;
    type Response: Serialize + DeserializeOwned + Send + 'static;

    /// ⚡ 生成 RawMessage 的辅助方法
    fn build_request(req: Self::Request) -> Result<AtlasRawMessage, String> {
        AtlasWireMessage {
            header: AtlasWireHeader::build_request(Self::WIRE),
            payload: req,
        }
        .into_raw()
        .map_err(|_| "build raw request failed".to_string())
    }
}

pub async fn handle<M, Fut>(
    raw: AtlasRawMessage,
    f: fn(AtlasWireMessage<M::Request>) -> Fut,
) -> AtlasRawMessage
where
    M: AtlasRpcSpec,
    AtlasRpcResult<M::Response>: Serialize + DeserializeOwned,
    M::Response: Serialize + DeserializeOwned,
    Fut: Future<Output=AtlasRpcPayload<M::Response>>,
{
    let req_msg = match AtlasWireMessage::<M::Request>::from_raw(raw.clone()) {
        Ok(r) => r,
        Err(_e) => {
            return AtlasRawMessage {
                header: raw.header,
                payload: Bytes::new(),
            };
        }
    };

    let header = req_msg.header.clone();

    let payload = f(req_msg).await;

    let kind = match payload {
        AtlasRpcPayload::Ok(_) => AtlasWireKind::ResponseOk,
        AtlasRpcPayload::Err(_) => AtlasWireKind::ResponseErr,
    };

    AtlasWireMessage {
        header: header.with_kind(kind),
        payload,
    }.into_raw().unwrap_or_else(|_| AtlasRawMessage {
        header: raw.header,
        payload: Bytes::new(),
    })
}
