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
    use atlas_core::net::core::{handle_rpc, AtlasRpcSpec};
    use atlas_core::net::protocol::frame::AtlasRawFrame;


    pub async fn dispatch(raw: AtlasRawFrame) -> AtlasRawFrame {
        match raw.header.op_code {
            <auth_method::RegisterRpc>::OP_CODE => handle_rpc::<auth_method::RegisterRpc, _>(raw, account_handler::register).await,
            <auth_method::BasicAuthRpc>::OP_CODE => handle_rpc::<auth_method::BasicAuthRpc, _>(raw, account_handler::basic_auth).await,
            <auth_method::TokenAuthRpc>::OP_CODE => handle_rpc::<auth_method::TokenAuthRpc, _>(raw, account_handler::token_auth).await,
            _ => AtlasRawFrame {
                header: raw.header,
                body: Bytes::new(),
            },
        }
    }
}