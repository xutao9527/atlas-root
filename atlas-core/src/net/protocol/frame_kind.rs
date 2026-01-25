use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtlasFrameKind {
    Request = 1,
    ResponseOk = 2,
    ResponseErr = 3,
    Notify = 4,
    RegNode = 5,
}

impl TryFrom<u8> for AtlasFrameKind {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == AtlasFrameKind::Request as u8 => Ok(Self::Request),
            x if x == AtlasFrameKind::ResponseOk as u8 => Ok(Self::ResponseOk),
            x if x == AtlasFrameKind::ResponseErr as u8 => Ok(Self::ResponseErr),
            x if x == AtlasFrameKind::Notify as u8 => Ok(Self::Notify),
            x if x == AtlasFrameKind::RegNode as u8 => Ok(Self::RegNode),
            other => Err(format!("invalid AtlasWireKind: {:#010b}", other)),
        }
    }
}
