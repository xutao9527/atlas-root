use atlas_core::net::packet::Response;
use atlas_core::net::router::Router;
use atlas_core::net::router::auth::AuthMethod;
use atlas_core::net::server::AtlasNetServer;
use futures::FutureExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut router = Router::new();
    router.register(
        AuthMethod::SignIn,
        Arc::new(|req| {
            async move {
                Response {
                    id: req.id,
                    slot_index: req.slot_index,
                    payload: b"AUTH_SIGN_IN".to_vec(),
                    error: None,
                }
            }
            .boxed()
        }),
    );
    let server = AtlasNetServer::new("127.0.0.1:9001", router);
    server.run().await
}
