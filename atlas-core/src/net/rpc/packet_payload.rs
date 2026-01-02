// use bytes::Bytes;
// use serde::Serialize;
// use crate::net::rpc::packet_request::AtlasRawRequest;
// use crate::net::rpc::packet_response::AtlasWireResponse;
//
// pub trait AtlasRequestPayload: Serialize + Sized {
//     /// 该请求对应的响应类型
//     type Response: AtlasResponsePayload;
//
//     /// 协议 method
//     const METHOD: u32;
//
//     fn raw(self) -> Result<AtlasRawRequest, String> {
//         let payload = rmp_serde::to_vec(&self)
//             .map_err(|e| e.to_string())?;
//
//         Ok(AtlasRawRequest {
//             id: 0,
//             slot_index: 0,
//             method: Self::METHOD,
//             payload: Bytes::from(payload),
//         })
//     }
// }
//
// pub trait AtlasResponsePayload: Serialize + Sized {
//     fn into_wire(self, id: u64) -> AtlasWireResponse<Self> {
//         AtlasWireResponse {
//             id,
//             slot_index: 0,
//             payload: self,
//             error: None,
//         }
//     }
// }