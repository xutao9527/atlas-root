use atlas_core::atlas_rpc_module;
use crate::dto::auth_model::*;

atlas_rpc_module! {
    module auth_method {
        ModuleId = AtlasModuleId::Auth;
        RegisterRpc = (1, RegisterReq, RegisterResp),
        BasicAuthRpc = (2, BasicAuthReq, AuthResp),
        TokenAuthRpc = (3, TokenAuthReq, AuthResp),
    }
}

