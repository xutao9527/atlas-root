use std::time::Duration;
use tokio::time::sleep;
use atlas_core::net::client_v2::AtlasNetClientV2;

const SERVER_ADDR: &str = "127.0.0.1:9001";
// const CONNECTIONS: usize = 5;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> anyhow::Result<()> {
    let mut client_v2 = AtlasNetClientV2::new(SERVER_ADDR);
    client_v2.connect().await?;
    client_v2.send().await;
    sleep(Duration::from_secs(2)).await;
    Ok(())
}
