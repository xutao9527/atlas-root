use atlas_core::net::server::server::AtlasNetServer;
use crate::context::{init_db, token_manager};
use crate::rpc::rpc_dispatch_def::auth_bind::dispatch;


pub mod rpc;
pub mod context;

pub async fn serve_auth(bind_addr: String, bind_port: String) -> anyhow::Result<()> {
    // 初始化数据库
    init_db("mysql://root:root@localhost:3306/atlas").await;
    // 令牌管理
    token_manager::start_token_cleaner();
    // 运行Rpc服务
    let serve_addr = format!("{}:{}", bind_addr,bind_port);

    let server = AtlasNetServer::new(serve_addr, dispatch);

    server.run().await?;
    Ok(())
}
