use crate::net::router::{RouterModule, RouterMethod};

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum AuthMethod {
    SignIn = 1,
    SignUp = 2,
}

impl RouterMethod for AuthMethod {
    const MODULE: RouterModule = RouterModule::Auth;
    fn id(self) -> u16 {
        self as u16
    }
}