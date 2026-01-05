use tracing_subscriber::fmt::time::LocalTime;
use atlas_core::net::rpc::server::AtlasNetServer;


pub mod auth_bind {
    use bytes::Bytes;
    use atlas_auth::rpc::auth_handler;
    use atlas_core::AtlasMethodSpec;
    use atlas_core::net::rpc::packet_request::AtlasRawRequest;
    use atlas_core::net::rpc::packet_response::AtlasRawResponse;
    use atlas_core::net::rpc::router::handle;
    use atlas_scheme::module_method::auth_method;

    pub async fn dispatch(raw: AtlasRawRequest) -> AtlasRawResponse {
        match raw.method {
            <auth_method::Register>::WIRE => handle::<auth_method::Register, _>(raw, auth_handler::register).await,
            <auth_method::Login>::WIRE => handle::<auth_method::Login, _>(raw, auth_handler::login).await,
            _ => AtlasRawResponse {
                id: raw.id,
                slot_index: raw.slot_index,
                method: raw.method,
                payload: Bytes::new(),
                error: Some("method not found".into()),
            },
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    let serve_addr = format!("{}:{}", "0.0.0.0", "5566");
    let server = AtlasNetServer::new(serve_addr, auth_bind::dispatch);
    server.run().await?;
    Ok(())
}