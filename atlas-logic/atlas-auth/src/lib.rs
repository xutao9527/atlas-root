pub mod rpc;

use atlas_core::net::rpc::packet_request::AtlasWireRequest;
use atlas_core::net::rpc::packet_response::AtlasWireResponse;
use atlas_core::net::rpc::router::{AtlasRouter, adapter_handler};
use atlas_core::net::rpc::server::AtlasNetServer;

use rpc::auth_handler::login;
use tracing::info;
use atlas_scheme::module_methods::auth;

pub async fn serve_auth(bind_addr: String, bind_port: String) -> anyhow::Result<()> {
    let mut router = AtlasRouter::new();

    router.register(auth::Login, adapter_handler(login));
    router.register(
        auth::Register,
        adapter_handler(|req: AtlasWireRequest<Vec<u8>>| async move {
            AtlasWireResponse {
                id: req.id,
                slot_index: req.slot_index,
                payload: b"SignUp OK".to_vec(),
                error: None,
            }
        }),
    );

    let serve_addr = format!("{}:{}", bind_addr, bind_port);
    let server = AtlasNetServer::new(serve_addr.as_str(), router);

    info!("auth server listening on {}", serve_addr);
    server.run().await
}
