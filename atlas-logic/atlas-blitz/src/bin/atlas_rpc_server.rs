use atlas_blitz::auth_mod;
use atlas_nut::net::rpc::server::AtlasRpcServer;
use tracing_subscriber::fmt::time::LocalTime;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    let serve_addr = format!("{}:{}", "0.0.0.0", "5566");
    let server = AtlasRpcServer::new(serve_addr, auth_mod::dispatch);
    server.run().await?;
    Ok(())
}
