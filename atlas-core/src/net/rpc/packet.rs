use crate::net::rpc::packet_request::AtlasRawRequest;
use crate::net::rpc::packet_response::AtlasRawResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AtlasPacket {
    AtlasRequest(AtlasRawRequest),
    AtlasResponse(AtlasRawResponse),
}
