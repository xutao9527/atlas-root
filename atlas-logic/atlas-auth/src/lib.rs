pub mod rpc;

use atlas_core::net::rpc::packet::{AtlasRequest, AtlasResponse};

use rpc::auth_handler::login;
use crate::rpc::method::AuthMethod;
use atlas_core::net::rpc::router::AtlasRouter;
use atlas_core::net::rpc::server::AtlasNetServer;
use tracing::info;

pub async fn serve_auth(bind_addr: String, bind_port: String) -> anyhow::Result<()> {
    let mut router = AtlasRouter::new();

    router.register(AuthMethod::Login, login);
    router.register(AuthMethod::Register, |req: AtlasRequest| async move {
        AtlasResponse {
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
