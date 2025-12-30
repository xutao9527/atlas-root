use atlas_core::net::packet::{AtlasRequest, AtlasResponse};

pub async fn login(request: AtlasRequest) -> AtlasResponse {
    AtlasResponse {
        id: request.id,
        slot_index: request.slot_index,
        payload: b"SignIn Handler".to_vec(),
        error: None,
    }
}
