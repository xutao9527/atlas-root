use bytes::Bytes;

use crate::net::rpc::packet_request::{AtlasRawRequest, AtlasWireRequest};
use crate::net::rpc::packet_response::{AtlasRawResponse, AtlasWireResponse};
use crate::net::rpc::router_spec::AtlasMethodSpec;

pub async fn handle<M, Fut>(
    raw: AtlasRawRequest,
    f: fn(AtlasWireRequest<M::Request>) -> Fut,
) -> AtlasRawResponse
where
    M: AtlasMethodSpec,
    Fut: Future<Output=AtlasWireResponse<M::Response>>,
{
    let req = match AtlasWireRequest::<M::Request>::from_raw(raw.clone()) {
        Ok(r) => r,
        Err(e) => return AtlasRawResponse {
            id: raw.id,
            slot_index: raw.slot_index,
            payload: Bytes::new(),
            error: Some(e),
        },
    };
    let resp = f(req).await;
    resp.into_raw()
}


