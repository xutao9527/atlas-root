use bytes::Buf;

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
}
