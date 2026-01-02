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
    const MODULE_ID: AtlasModuleId;
    const METHOD_ID: u16;
    const WIRE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::METHOD_ID as u32);
}

