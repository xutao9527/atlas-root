mod handler;

use tracing::info;
use atlas_core::net::router::auth::AuthMethod;
use atlas_core::net::router::Router;
use atlas_core::net::server::AtlasNetServer;
use crate::handler::auth_handler::login;

pub async fn serve_auth(bind_addr: String, bind_port: String) -> anyhow::Result<()> {

    let mut router = Router::new();

    router.register(AuthMethod::SignIn, login);

    let serve_addr = format!("{}:{}", bind_addr, bind_port);
    let server = AtlasNetServer::new(serve_addr.as_str(), router);

    info!("auth server listening on {}", serve_addr);
    server.run().await
}