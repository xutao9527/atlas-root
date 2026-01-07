use atlas_core::atlas_rpc_module;
use crate::dto::auth_model::*;

atlas_rpc_module! {
    module auth_method {
        ModuleId = AtlasModuleId::Auth;
        RegisterRpc = (1, RegisterReq, RegisterResp),
        LoginRpc = (2, LoginReq, LoginResp),
    }
}


pub mod auth_method1 {
    use super::*;
    use atlas_core::net::rpc::router::{AtlasModuleId, AtlasRpcSpec};

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Register;

    impl AtlasRpcSpec for Register {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 1;
        type Request = RegisterReq;
        type Response = RegisterResp;
    }
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Login;

    impl AtlasRpcSpec for Login {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 2;
        type Request = LoginReq;
        type Response = LoginResp;
    }
}