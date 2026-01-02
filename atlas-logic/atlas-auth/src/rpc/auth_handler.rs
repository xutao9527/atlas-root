use atlas_core::net::rpc::packet_request::AtlasWireRequest;
use atlas_core::net::rpc::packet_response::AtlasWireResponse;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp};

pub async fn login(request: AtlasWireRequest<LoginReq>) -> AtlasWireResponse<LoginResp> {
    let token = format!("{}|{}",request.payload.account,request.payload.password);
    AtlasWireResponse {
        id: request.id,
        slot_index: request.slot_index,
        payload: LoginResp {
            ok: true,
            token: Some(token),
            error: None,
        },
        error: None,
    }
}