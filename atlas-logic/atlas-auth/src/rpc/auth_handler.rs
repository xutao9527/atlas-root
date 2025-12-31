use atlas_core::net::rpc::packet::{AtlasRequest, AtlasResponse};
use crate::rpc::auth_model::{LoginReq, LoginResp};

pub async fn login(request: AtlasRequest<LoginReq>) -> AtlasResponse<LoginResp> {

    let token = format!("{}|{}",request.payload.account,request.payload.password);
    
    AtlasResponse {
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