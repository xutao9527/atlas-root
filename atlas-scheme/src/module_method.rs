use atlas_nut::atlas_method;
use crate::dto::auth_model::*;

atlas_method! {
    module auth_method {
        module_id = AtlasModuleId::Auth;
        Register = (1, RegisterReq, RegisterResp),
        Login = (2, LoginReq, LoginResp),
    }
}