use bytes::Bytes;
use crate::net::protocol::frame::{AtlasFrame, AtlasRawFrame};
use crate::net::protocol::frame_header::AtlasFrameHeader;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::net::core::module_id::AtlasModuleId;
use crate::net::protocol::frame_body_rpc::{AtlasRpcBody, AtlasRpcFrame};
use crate::net::protocol::frame_kind::AtlasFrameKind;


pub trait AtlasRpcSpec: Copy + 'static {
    const MODULE_ID: AtlasModuleId;
    const METHOD_ID: u16;
    const OP_CODE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::METHOD_ID as u32);
    type Request: Serialize + DeserializeOwned + Send + 'static;
    type Response: Serialize + DeserializeOwned + Send + 'static;

    /// ⚡ 生成 RawMessage 的辅助方法
    fn build_request(req: Self::Request) -> Result<AtlasRawFrame, String> {
        AtlasFrame {
            header: AtlasFrameHeader::build_request(Self::OP_CODE),
            body: req,
        }
        .into_raw()
        .map_err(|_| "build raw request failed".to_string())
    }
}


pub async fn handle_rpc<M, Fut>(
    raw: AtlasRawFrame,
    f: fn(AtlasFrame<M::Request>) -> Fut,
) -> AtlasRawFrame
where
    M: AtlasRpcSpec,
    AtlasRpcFrame<M::Response>: Serialize + DeserializeOwned,
    M::Response: Serialize + DeserializeOwned,
    Fut: Future<Output=AtlasRpcBody<M::Response>>,
{
    let req_msg = match AtlasFrame::<M::Request>::from_raw(raw.clone()) {
        Ok(r) => r,
        Err(_e) => {
            return AtlasRawFrame {
                header: raw.header,
                body: Bytes::new(),
            };
        }
    };

    let header = req_msg.header.clone();

    let payload = f(req_msg).await;

    let kind = match payload {
        AtlasRpcBody::Ok(_) => AtlasFrameKind::ResponseOk,
        AtlasRpcBody::Err(_) => AtlasFrameKind::ResponseErr,
    };

    AtlasFrame {
        header: header.with_kind(kind),
        body: payload,
    }.into_raw().unwrap_or_else(|_| AtlasRawFrame {
        header: raw.header,
        body: Bytes::new(),
    })
}
