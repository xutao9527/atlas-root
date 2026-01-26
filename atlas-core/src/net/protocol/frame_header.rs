use bytes::{Buf, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use crate::net::protocol::frame_kind::AtlasFrameKind;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AtlasFrameHeader {
    pub id: u64,
    pub slot_index: u32,
    pub op_code: u32,
    pub kind: AtlasFrameKind,
    pub uid: [u8; 16],
}

impl AtlasFrameHeader {
    pub const FRAME_LEN: usize = 33;

    /// 变更消息种类
    #[inline]
    pub fn with_kind(self, kind: AtlasFrameKind) -> Self {
        Self {
            kind,
            ..self
        }
    }

    /// 构造请求
    #[inline]
    pub fn build_request(op_code: u32) -> Self {
        Self {
            id: 0,
            slot_index: 0,
            op_code,
            kind: AtlasFrameKind::Request,
            uid: [0u8; 16],
        }
    }

    /// 构造通知
    #[inline]
    pub fn build_notify(op_code: u32) -> Self {
        Self {
            id: 0,
            slot_index: 0,
            op_code,
            kind: AtlasFrameKind::Notify,
            uid: [0u8; 16],
        }
    }

    /// 读取请求头中字段
    pub fn read_wire_header(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::FRAME_LEN {
            return Err(format!(
                "buffer too small: need {}, got {}",
                Self::FRAME_LEN,
                buf.len()
            ));
        }
        let mut p = &buf[..Self::FRAME_LEN];
        let id = p.get_u64();
        let slot_index = p.get_u32();
        let op_code = p.get_u32();
        let kind_u8 = p.get_u8();
        let kind = AtlasFrameKind::try_from(kind_u8)?;
        let mut uid = [0u8; 16];
        p.copy_to_slice(&mut uid);
        Ok(AtlasFrameHeader {
            id,
            slot_index,
            op_code,
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