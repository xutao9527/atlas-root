use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;
use atlas_core::AtlasMethodSpec;
use atlas_core::net::rpc::client::client::AtlasRpcClient;
use atlas_core::net::rpc::packet_request::AtlasWireRequest;
use atlas_core::net::rpc::packet_response::AtlasWireResponse;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp};
use atlas_scheme::module_method::auth_method;

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

    let req = AtlasWireRequest {
        id: 0,
        slot_index: 0u64,
        method: auth_method::Login::WIRE,
        payload: LoginReq{
            account: "1111".to_string(),
            password: "2222".to_string(),
        },
    };

    client.call_cb(req.into_raw().unwrap(),|resp| {
        let resp = AtlasWireResponse::<LoginResp>::from_raw(resp);
        info!("callback {:?}", resp);
    }).await;
    // loop{
    sleep(Duration::from_secs(3)).await;
    // }
    Ok(())
}