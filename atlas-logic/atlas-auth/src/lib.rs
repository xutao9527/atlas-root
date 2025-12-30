mod handler;

use crate::handler::auth_handler::login;
use atlas_core::net::packet::{Request, Response};
use atlas_core::net::router::AtlasRouter;
use atlas_core::net::router::auth::AuthMethod;
use atlas_core::net::server::AtlasNetServer;
use tracing::info;

pub async fn serve_auth(bind_addr: String, bind_port: String) -> anyhow::Result<()> {
    let mut router = AtlasRouter::new();

    router.register(AuthMethod::SignIn, login);
    router.register(AuthMethod::SignUp, |req: Request| async move {
        Response {
            id: req.id,
            slot_index: req.slot_index,
            payload: b"SignUp OK".to_vec(),
            error: None,
        }
    });

    let serve_addr = format!("{}:{}", bind_addr, bind_port);
    let server = AtlasNetServer::new(serve_addr.as_str(), router);

    info!("auth server listening on {}", serve_addr);
    server.run().await
}
