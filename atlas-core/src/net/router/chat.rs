use crate::net::router::{Module, RouterMethod};

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum ChatMethod {
    SendMessage = 1,
    GetHistory = 2,
}

impl RouterMethod for ChatMethod {
    const MODULE: Module = Module::Chat;
    fn id(self) -> u16 {
        self as u16
    }
}