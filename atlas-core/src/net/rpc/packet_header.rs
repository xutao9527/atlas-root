use bytes::{Buf, Bytes, BytesMut};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasWireKind {
    Request = 0b0000_0001,
    ResponseOk = 0b0000_0010,
    ResponseErr = 0b0000_0100,
    Notify = 0b0000_1000,
}

#[derive(Debug, Clone, Copy)]
pub struct AtlasWireHeader {
    pub id: u64,
    pub slot_index: u32,
    pub method: u32,
    pub kind: AtlasWireKind,
}

impl AtlasWireHeader {
    pub const WIRE_LEN: usize = 17;
    /// 变更消息种类
    #[inline]
    pub fn with_kind(self, kind: AtlasWireKind) -> Self {
        Self {
            kind,
            ..self
        }
    }

    /// 构造请求头
    pub fn build_request(method: u32) -> Self {
        Self {
            id: 0,
            slot_index: 0,
            method,
            kind: AtlasWireKind::Request,
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

        let kind = match kind_u8 {
            x if x == AtlasWireKind::Request as u8 => AtlasWireKind::Request,
            x if x == AtlasWireKind::ResponseOk as u8 => AtlasWireKind::ResponseOk,
            x if x == AtlasWireKind::ResponseErr as u8 => AtlasWireKind::ResponseErr,
            x if x == AtlasWireKind::Notify as u8 => AtlasWireKind::Notify,
            other => {
                return Err(format!(
                    "invalid AtlasWireKind: {:#010b}",
                    other
                ))
            }
        };

        Ok(AtlasWireHeader {
            id,
            slot_index,
            method,
            kind,
        })
    }

    /// 覆盖请求头中字段
    pub fn overwrite_wire_header(
        wire: Bytes,
        new_id: u64,
        new_slot_index: u32,
    ) -> Bytes {
        // 如果 Bytes 是共享的，这里才会发生一次 copy（仅 17 字节）
        let mut buf = BytesMut::from(wire.as_ref());
        // id
        buf[0..8].copy_from_slice(&new_id.to_be_bytes());
        // slot_index
        buf[8..12].copy_from_slice(&new_slot_index.to_be_bytes());

        buf.freeze()
    }
}
