
use serde::{Deserialize, Serialize};
use crate::net::rpc::packet_request::AtlasRawRequest;
use crate::net::rpc::packet_response::AtlasRawResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AtlasPacket {
    AtlasRequest(AtlasRawRequest),
    AtlasResponse(AtlasRawResponse),
}