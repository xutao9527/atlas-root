pub mod auth;
pub mod chat;
pub mod router_handler;



/// RPC Module
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RouterModule {
    Auth = 1,
    Chat = 2,
}

/// RPC Method
pub trait RouterMethod: Copy {
    /// 所属模块
    const MODULE: RouterModule;
    /// 方法号（低 16 位）
    fn id(self) -> u16;
    #[inline(always)]
    fn wire(self) -> u32 {
        ((Self::MODULE as u32) << 16) | self.id() as u32
    }
}

