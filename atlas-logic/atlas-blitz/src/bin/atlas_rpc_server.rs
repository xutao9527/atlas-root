use atlas_nut::net::rpc::server::AtlasNetServer;
use tracing_subscriber::fmt::time::LocalTime;

use crate::auth_mod::dispatch;

pub mod auth_mod {
    use atlas_blitz::dto::{LoginReq, LoginResp, RegisterReq, RegisterResp};
    use atlas_blitz::handler::{login, register};
    use atlas_nut::net::rpc::packet_message::{AtlasRawMessage};
    use atlas_nut::net::rpc::router::{AtlasMethodSpec, AtlasModuleId, handle};
    use bytes::Bytes;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Register;

    impl AtlasMethodSpec for Register {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 1;
        type Request = RegisterReq;
        type Response = RegisterResp;
    }
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Login;

    impl AtlasMethodSpec for Login {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 2;
        type Request = LoginReq;
        type Response = LoginResp;
    }

    pub async fn dispatch(raw: AtlasRawMessage) -> AtlasRawMessage {
        match raw.header.method {
            <Register>::WIRE => handle::<Register, _>(raw, register).await,
            <Login>::WIRE => handle::<Login, _>(raw, login).await,
            _ => AtlasRawMessage {
                header: raw.header,
                payload: Bytes::new(),

            },
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();
    let serve_addr = format!("{}:{}", "0.0.0.0", "5566");
    let server = AtlasNetServer::new(serve_addr, dispatch);
    server.run().await?;
    Ok(())
}
