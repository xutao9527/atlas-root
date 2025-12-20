use atlas_core::net::client::AtlasNetClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = AtlasNetClient::new("127.0.0.1:9001");
    client.run().await
}