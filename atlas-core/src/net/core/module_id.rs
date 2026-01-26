#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AtlasModuleId {
    Auth = 1,
    Holdem = 2,
    Chat = 3,
}

impl AtlasModuleId {
    #[inline]
    pub fn from_wire(wire: u32) -> Option<Self> {
        match (wire >> 16) as u16 {
            1 => Some(Self::Auth),
            2 => Some(Self::Holdem),
            3 => Some(Self::Chat),
            _ => None,
        }
    }
}