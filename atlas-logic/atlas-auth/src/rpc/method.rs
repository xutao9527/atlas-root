use atlas_core::net::rpc::router_spec::{AtlasModuleId, AtlasRouterMethod};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AuthModule;


#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AuthMethod {
    Register = 1,
    Login = 2,
}

impl AtlasRouterMethod for AuthMethod {
    const MODULE: AtlasModuleId = AtlasModuleId::Auth;

    fn id(self) -> u16 {
        self as u16
    }
}