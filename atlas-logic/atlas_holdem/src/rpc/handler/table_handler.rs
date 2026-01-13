use atlas_core::net::rpc::packet_header::AtlasWireKind::ResponseOk;
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_scheme::dto::holdem_model::{GetTableReq, GetTableResp};

pub async fn get_table(request: AtlasWireMessage<GetTableReq>) -> AtlasWireMessage<GetTableResp> {
    AtlasWireMessage {
        header: request.header.with_kind(ResponseOk),
        payload: GetTableResp {},
    }
}
