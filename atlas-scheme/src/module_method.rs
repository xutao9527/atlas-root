use atlas_core::atlas_rpc_module;
use crate::proto::auth::auth_model::*;
use crate::proto::holdem::holdem_model::*;

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
        GetTableRpc = (1, GetTableReq, GetTableResp),
        SitTableRpc = (2, SitTableReq, SitTableResp),
        LeaveTableRpc = (3, LeaveTableReq, LeaveTableResp),
        GameActRpc = (4, GameActReq, GameActResp),
    }
}
