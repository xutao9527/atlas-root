use tracing_subscriber::fmt::time::LocalTime;
use atlas_auth::serve_auth;
use atlas_core::net::router::Router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .init();
    Ok(serve_auth("0.0.0.0".into(), "5566".into()).await?)
}