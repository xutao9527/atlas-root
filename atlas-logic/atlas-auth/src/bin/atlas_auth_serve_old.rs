use tracing_subscriber::fmt::time::LocalTime;
use atlas_auth::module_dispatch::auth_bind::dispatch;
use atlas_core::net::rpc::server::AtlasNetServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    let serve_addr = format!("{}:{}", "0.0.0.0", "5566");
    let server = AtlasNetServer::new(serve_addr, dispatch);
    server.run().await?;
    Ok(())
}