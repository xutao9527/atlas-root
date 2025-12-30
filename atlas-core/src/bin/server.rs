use atlas_core::net::packet::{Request, Response};
use atlas_core::net::router::auth::AuthMethod;
use atlas_core::net::router::{AtlasRouter};
use atlas_core::net::server::AtlasNetServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut router = AtlasRouter::new();
    router.register(AuthMethod::SignIn, |req: Request| async move {
        Response {
            id: req.id,
            slot_index: req.slot_index,
            payload: b"AUTH_SIGN_IN".to_vec(),
            error: None,
        }
    });
    let server = AtlasNetServer::new("0.0.0.0:9001", router);
    server.run().await
}
