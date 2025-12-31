use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;
use atlas_auth::rpc::auth_model::{LoginReq, LoginResp};

use atlas_auth::rpc::method::AuthMethod;
use atlas_core::net::rpc::client::client::AtlasRpcClient;

use atlas_core::net::rpc::packet::{AtlasRequest, AtlasResponse};
use atlas_core::net::rpc::router_spec::AtlasRouterMethod;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let mut client = AtlasRpcClient::new("127.0.0.1:5566".into(), 4);
    client.connect().await?;

    let req = AtlasRequest {
        id: 0,
        slot_index: 0 as usize,
        method: AuthMethod::Login.wire(),
        payload: LoginReq{
            account: "111".to_string(),
            password: "2222".to_string(),
        },
    };

    client.call_cb(req.into_raw().unwrap(),|resp| {
        let resp = AtlasResponse::<LoginResp>::from_raw(resp);
        info!("callback {:?}", resp);
    }).await;
    loop{
        sleep(Duration::from_secs(3)).await;
    }
    //Ok(())
}