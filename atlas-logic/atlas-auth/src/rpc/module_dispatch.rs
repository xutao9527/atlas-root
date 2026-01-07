use crate::rpc::handler::account_handler;
use atlas_core::atlas_rpc_dispatch;
use atlas_scheme::module_method::auth_method;

atlas_rpc_dispatch! {
    module auth_bind {
        auth_method::RegisterRpc => account_handler::register,
        auth_method::LoginRpc => account_handler::login,
    }
}
