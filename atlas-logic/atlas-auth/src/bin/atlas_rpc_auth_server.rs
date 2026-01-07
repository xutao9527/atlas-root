
use atlas_core::net::rpc::server::AtlasRpcServer;
use tracing_subscriber::fmt::time::LocalTime;
use atlas_auth::context::init_db;
use atlas_auth::rpc::module_dispatch::auth_bind::dispatch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    // 初始化数据库
    init_db("mysql://root:root@localhost:3306/atlas").await;
    // 初始化Rpc服务
    let serve_addr = format!("{}:{}", "0.0.0.0", "5566");
    let server = AtlasRpcServer::new(serve_addr, dispatch);
    server.run().await?;
    Ok(())
}
