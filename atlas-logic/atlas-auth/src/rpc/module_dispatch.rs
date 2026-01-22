use crate::rpc::handler::account_handler;
use atlas_core::atlas_rpc_dispatch;
use atlas_scheme::module_method::auth_method;

atlas_rpc_dispatch! {
    module auth_bind {
        auth_method::RegisterRpc => account_handler::register,
        auth_method::BasicAuthRpc => account_handler::basic_auth,
        auth_method::TokenAuthRpc => account_handler::token_auth,
    }
}


pub mod auth_bind1 {
    use super::*;
    use bytes::Bytes;
    use atlas_core::net::rpc::packet_message::AtlasRawMessage;
    use atlas_core::net::rpc::router::{handle, AtlasRpcSpec};


    pub async fn dispatch(raw: AtlasRawMessage) -> AtlasRawMessage {
        match raw.header.method {
            <auth_method::RegisterRpc>::WIRE => handle::<auth_method::RegisterRpc, _>(raw, account_handler::register).await,
            <auth_method::BasicAuthRpc>::WIRE => handle::<auth_method::BasicAuthRpc, _>(raw, account_handler::basic_auth).await,
            <auth_method::TokenAuthRpc>::WIRE => handle::<auth_method::TokenAuthRpc, _>(raw, account_handler::token_auth).await,
            _ => AtlasRawMessage {
                header: raw.header,
                payload: Bytes::new(),
            },
        }
    }
}