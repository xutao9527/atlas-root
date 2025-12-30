use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;
use atlas_core::net::client::client_rpc::AtlasRpcClient;
use atlas_core::net::packet::Packet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let mut client = AtlasRpcClient::new("127.0.0.1:5566".into(), 1);
    client.connect().await?;
    client.call_cb(|res| {
        match res {
            Packet::Response(_resp) => {
                info!("callback {:?}", _resp);
            }
            _ => {

            }
        }
    }).await;
    sleep(Duration::from_secs(3)).await;
    Ok(())
}