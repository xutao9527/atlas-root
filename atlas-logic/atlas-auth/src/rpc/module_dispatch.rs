use crate::rpc::auth_handler;
use atlas_core::atlas_rpc_dispatch;
use atlas_scheme::module_method::auth_method;

atlas_rpc_dispatch! {
    module auth_bind {
        auth_method::RegisterRpc => auth_handler::register,
        auth_method::LoginRpc => auth_handler::login,
    }
}