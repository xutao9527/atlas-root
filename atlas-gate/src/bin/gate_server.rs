use tracing_subscriber::fmt::time::LocalTime;
use atlas_gate::serve_gateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .init();
    Ok(serve_gateway("0.0.0.0".into(), "8080".into()).await?)
}