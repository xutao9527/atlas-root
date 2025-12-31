use atlas_core::net::rpc::packet::{AtlasRequest, AtlasResponse};
use crate::rpc::auth_model::{LoginReq, LoginResp};

pub async fn login(request: AtlasRequest<LoginReq>) -> AtlasResponse<LoginResp> {
    AtlasResponse {
        id: request.id,
        slot_index: request.slot_index,
        payload: LoginResp {
            ok: true,
            token: Some("abc123".into()),
            error: None,
        },
        error: None,
    }
}