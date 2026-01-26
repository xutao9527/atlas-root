use crate::rpc::handler::account_handler;
use atlas_core::atlas_rpc_dispatch;
use atlas_scheme::proto::rpc_def::auth_method;

atlas_rpc_dispatch! {
    module auth_bind {
        auth_method::RegisterRpc => account_handler::register,
        auth_method::BasicAuthRpc => account_handler::basic_auth,
        auth_method::TokenAuthRpc => account_handler::token_auth,
    }
}