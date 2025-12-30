use axum::response::IntoResponse;

pub async fn http_index() -> impl IntoResponse {
    "Hello"
}
