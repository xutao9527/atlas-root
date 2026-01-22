use bytes::{Buf, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtlasWireKind {
    Request = 0b0000_0001,
    ResponseOk = 0b0000_0010,
    ResponseErr = 0b0000_0100,
    Notify = 0b0000_1000,
    RegistryNode = 0b0001_0000,
}

impl TryFrom<u8> for AtlasWireKind {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == AtlasWireKind::Request as u8      => Ok(Self::Request),
            x if x == AtlasWireKind::ResponseOk as u8   => Ok(Self::ResponseOk),
            x if x == AtlasWireKind::ResponseErr as u8  => Ok(Self::ResponseErr),
            x if x == AtlasWireKind::Notify as u8       => Ok(Self::Notify),
            x if x == AtlasWireKind::RegistryNode as u8 => Ok(Self::RegistryNode),
            other => Err(format!(
                "invalid AtlasWireKind: {:#010b}",
                other
            )),
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AtlasWireHeader {
    pub id: u64,
    pub slot_index: u32,
    pub method: u32,
    pub kind: AtlasWireKind,
    pub uid: [u8; 16],
}

impl AtlasWireHeader {
    pub const WIRE_LEN: usize = 33;
    /// 变更消息种类
    #[inline]
    pub fn with_kind(self, kind: AtlasWireKind) -> Self {
        Self {
            kind,
            ..self
        }
    }

    /// 构造请求
    pub fn build_request(method: u32) -> Self {
        Self {
            id: 0,
            slot_index: 0,
            method,
            kind: AtlasWireKind::Request,
            uid: [0u8; 16],
        }
    }

    /// 构造通知
    pub fn build_notify() -> Self {
        Self {
            id: 0,
            slot_index: 0,
            method: 0,
            kind: AtlasWireKind::Notify,
            uid: [0u8; 16],
        }
    }


    /// 读取请求头中字段
    pub fn read_wire_header(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::WIRE_LEN {
            return Err(format!(
                "buffer too small: need {}, got {}",
                Self::WIRE_LEN,
                buf.len()
            ));
        }
        let mut p = &buf[..Self::WIRE_LEN];
        let id = p.get_u64();
        let slot_index = p.get_u32();
        let method = p.get_u32();
        let kind_u8 = p.get_u8();
        let kind = AtlasWireKind::try_from(kind_u8)?;
        let mut uid = [0u8; 16];
        p.copy_to_slice(&mut uid);
        Ok(AtlasWireHeader {
            id,
            slot_index,
            method,
            kind,
            uid,
        })
    }

    /// 覆盖请求头中字段
    pub fn overwrite_request_meta(
        wire: Bytes,
        id: u64,
        slot_index: u32,

    ) -> Bytes {
        let mut buf = BytesMut::from(wire.as_ref());
        // id
        buf[0..8].copy_from_slice(&id.to_be_bytes());
        // slot_index
        buf[8..12].copy_from_slice(&slot_index.to_be_bytes());
        buf.freeze()
    }

    #[inline]
    pub fn overwrite_uid(wire: Bytes, uid: [u8; 16]) -> Bytes {
        let mut buf = BytesMut::from(wire.as_ref());
        // uid offset: 17..33
        buf[17..33].copy_from_slice(&uid);
        buf.freeze()
    }
}
