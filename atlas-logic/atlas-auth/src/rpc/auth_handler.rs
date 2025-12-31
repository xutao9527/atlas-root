use atlas_core::net::rpc::packet::{AtlasRequest, AtlasResponse};
use crate::rpc::entity::{LoginReq, LoginResp};

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


// AtlasResponse {
// id: request.id,
// slot_index: request.slot_index,
// payload: b"SignIn Handler".to_vec(),
// error: None,
// }
