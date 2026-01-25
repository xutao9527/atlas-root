use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::net::protocol::frame_header::AtlasFrameHeader;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: Deserialize<'de>"
))]
pub struct AtlasFrame<T> {
    pub header: AtlasFrameHeader,
    pub body: T,
}

impl<T> AtlasFrame<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn from_raw(raw: AtlasRawFrame) -> Result<Self, String> {
        let body = rmp_serde::from_slice(&raw.body.as_ref()).map_err(|e| e.to_string())?;
        Ok(Self {
            header: raw.header,
            body,
        })
    }

    pub fn into_raw(self) -> Result<AtlasRawFrame, String> {
        let body = rmp_serde::to_vec_named(&self.body).map_err(|e| e.to_string())?;
        Ok(AtlasRawFrame {
            header: self.header,
            body: Bytes::from(body),
        })
    }
}


pub type AtlasRawFrame = AtlasFrame<Bytes>;

impl AtlasFrame<Bytes> {

    pub fn from_bytes(buf: Bytes) -> Result<Self, String> {
        if buf.len() < AtlasFrameHeader::FRAME_LEN {
            return Err("buffer too short for AtlasWireHeader".into());
        }
        let header = AtlasFrameHeader::read_wire_header(&buf[..AtlasFrameHeader::FRAME_LEN])?;
        let body = buf.slice(AtlasFrameHeader::FRAME_LEN..);
        Ok(Self { header, body })
    }

    pub fn into_bytes(self) -> Bytes {
        let payload_len = self.body.len();
        let mut buf = BytesMut::with_capacity(AtlasFrameHeader::FRAME_LEN + payload_len);
        buf.put_u64(self.header.id);
        buf.put_u32(self.header.slot_index);
        buf.put_u32(self.header.op_code);
        buf.put_u8(self.header.kind as u8);
        buf.extend_from_slice(&self.header.uid);
        // 2️⃣ 拼 body（零拷贝语义）
        buf.extend_from_slice(&self.body);
        buf.freeze()
    }
    
}

