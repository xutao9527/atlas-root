#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AtlasModuleId {
    Auth = 1,
    Chat = 2,
}

impl AtlasModuleId {
    #[inline]
    pub fn from_wire(wire: u32) -> Option<Self> {
        match (wire >> 16) as u16 {
            1 => Some(Self::Auth),
            2 => Some(Self::Chat),
            _ => None,
        }
    }
}

pub trait AtlasRouterMethod: Copy + 'static {
    /// 所属模块（直接是 enum，不是 trait）
    const MODULE: AtlasModuleId;

    fn id(self) -> u16;

    #[inline(always)]
    fn wire(self) -> u32 {
        ((Self::MODULE as u32) << 16) | self.id() as u32
    }
}