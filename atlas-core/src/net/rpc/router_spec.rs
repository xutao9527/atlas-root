#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AtlasModuleId {
    Auth = 1,
    Chat = 2,
}

impl AtlasModuleId {
    #[inline]
    fn from_u16(v: u16) -> Option<Self> {
        match v {
            1 => Some(Self::Auth),
            2 => Some(Self::Chat),
            _ => None,
        }
    }
    
    #[inline]
    pub fn from_wire(wire: u32) -> Option<Self> {
        Self::from_u16((wire >> 16) as u16)
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