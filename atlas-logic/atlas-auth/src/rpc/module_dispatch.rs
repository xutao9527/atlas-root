use crate::rpc::auth_handler;
use atlas_core::atlas_dispatch;
use atlas_scheme::module_method::auth_method;

atlas_dispatch! {
    module auth_bind {
        auth_method::Register => auth_handler::register,
        auth_method::Login => auth_handler::login,
    }
}