use atlas_core::atlas_rpc_module;
use crate::proto::auth::rpc::*;
use crate::proto::holdem::rpc::*;



pub mod auth_method1 {
    use super::*;

    use atlas_core::net::core::{AtlasModuleId, AtlasRpcSpec};


    #[allow(non_camel_case_types)]
    enum AtlasRpcMethodIdCheck {
        RegisterRpc = 1,
        BasicAuthRpc = 2,
        TokenAuthRpc = 3,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct RegisterRpc;
    impl AtlasRpcSpec for RegisterRpc {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 1;
        type Request = RegisterReq;
        type Response = RegisterResp;
    }
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct BasicAuthRpc;
    impl AtlasRpcSpec for BasicAuthRpc {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 2;
        type Request = BasicAuthReq;
        type Response = AuthResp;
    }
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct TokenAuthRpc;
    impl AtlasRpcSpec for TokenAuthRpc {
        const MODULE_ID: AtlasModuleId = AtlasModuleId::Auth;
        const METHOD_ID: u16 = 3;
        type Request = TokenAuthReq;
        type Response = AuthResp;
    }
}

atlas_rpc_module! {
    module auth_method {
        ModuleId = AtlasModuleId::Auth;
        RegisterRpc = (1, RegisterReq, RegisterResp),
        BasicAuthRpc = (2, BasicAuthReq, AuthResp),
        TokenAuthRpc = (3, TokenAuthReq, AuthResp),
    }
}

atlas_rpc_module! {
    module holdem_method {
        ModuleId = AtlasModuleId::Holdem;
        GetTableListRpc = (1, GetTableListReq, GetTableListResp),
        GetTableInfoRpc = (2, GetTableInfoReq, GetTableInfoResp),
        SitTableRpc = (3, SitTableReq, SitTableResp),
        LeaveTableRpc = (4, LeaveTableReq, LeaveTableResp),
        GameActRpc = (5, GameActReq, GameActResp),
        GameStartRpc = (6, GameStartReq, GameStartResp),
    }
}
