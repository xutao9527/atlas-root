use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;
use atlas_auth::rpc::method::AuthMethod;
use atlas_core::net::rpc::client::client_rpc::AtlasRpcClient;
use atlas_core::net::rpc::packet::AtlasRequest;
use atlas_core::net::rpc::router_spec::AtlasRouterMethod;

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

    let req = AtlasRequest {
        id: 0,
        slot_index: 0 as usize,
        method: AuthMethod::Login.wire(),
        payload: vec![],
    };


    client.call_cb(req,|resp| {
        info!("callback {:?}", resp);
    }).await;
    sleep(Duration::from_secs(3)).await;
    Ok(())
}