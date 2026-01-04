use atlas_core::net::rpc::packet_request::AtlasWireRequest;
use atlas_core::net::rpc::packet_response::AtlasWireResponse;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp, RegisterReq, RegisterResp};

pub async fn login(request: AtlasWireRequest<LoginReq>) -> AtlasWireResponse<LoginResp> {
    let token = format!("{}|{}",request.payload.account,request.payload.password);
    AtlasWireResponse {
        id: request.id,
        slot_index: request.slot_index,
        method: request.method,
        payload: LoginResp {
            ok: true,
            token: Some(token),
            error: None,
        },
        error: None,
    }
}


pub async fn register(req: AtlasWireRequest<RegisterReq>) -> AtlasWireResponse<RegisterResp> {
    AtlasWireResponse {
        id: req.id,
        slot_index: req.slot_index,
        method: req.method,
        payload: RegisterResp { ok: true, error: None },
        error: None,
    }
}