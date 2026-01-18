use atlas_core::atlas_rpc_module;
use crate::proto::auth::auth_model::*;
use crate::proto::holdem::rpc::*;


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
