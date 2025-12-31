#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AtlasModuleId {
    Auth = 1,
    Chat = 2,

}

pub trait AtlasRouterModule: Copy + Eq + 'static {
    /// 模块号（高 16 位）
    const ID: AtlasModuleId;
}

pub trait AtlasRouterMethod: Copy + 'static {
    type Module: AtlasRouterModule;

    fn id(self) -> u16;

    #[inline(always)]
    fn wire(self) -> u32 {
        ((Self::Module::ID as u32) << 16) | self.id() as u32
    }
}