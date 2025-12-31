use atlas_core::atlas_methods;
use atlas_core::net::rpc::router_spec::AtlasModuleId;


atlas_methods! {
    module AuthMethod = AtlasModuleId::Auth {
        Register = 1,
        Login = 2,
    }
}