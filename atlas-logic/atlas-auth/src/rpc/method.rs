use atlas_core::net::rpc::router_spec::{AtlasModuleId, AtlasRouterMethod, AtlasRouterModule};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AuthModule;

impl AtlasRouterModule for AuthModule {
    const ID: AtlasModuleId = AtlasModuleId::Auth;
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AuthMethod {
    Register = 1,
    Login = 2,
}

impl AtlasRouterMethod for AuthMethod {
    type Module = AuthModule;

    fn id(self) -> u16 {
        self as u16
    }
}