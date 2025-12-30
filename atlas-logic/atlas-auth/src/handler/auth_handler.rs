use atlas_core::net::packet::{Request, Response};

pub async fn login(request: Request) -> Response {
    Response {
        id: request.id,
        slot_index: request.slot_index,
        payload: b"login response".to_vec(),
        error: None,
    }
}
