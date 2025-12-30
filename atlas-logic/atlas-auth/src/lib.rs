mod handler;

use atlas_core::net::router::auth::AuthMethod;
use atlas_core::net::router::Router;
use atlas_core::net::server::AtlasNetServer;
use crate::handler::auth_handler::login;

pub async fn serve_auth(bind_addr: String, bind_port: String) -> anyhow::Result<()> {

    let mut router = Router::new();

    router.register(AuthMethod::SignIn, login);

    let server = AtlasNetServer::new("0.0.0.0:9001", router);
    server.run().await
}