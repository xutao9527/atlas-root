use atlas_core::net::server::AtlasNetServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = AtlasNetServer::new("127.0.0.1:9001");
    server.run().await
}