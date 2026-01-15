use crate::net::rpc::packet_header::AtlasWireHeader;
use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub type AtlasRawMessage = AtlasWireMessage<Bytes>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: Deserialize<'de>"
))]
pub struct AtlasWireMessage<T> {
    pub header: AtlasWireHeader,
    pub payload: T,
}

impl AtlasWireMessage<Bytes> {
    pub fn from_wire_bytes(buf: Bytes) -> Result<Self, String> {
        if buf.len() < AtlasWireHeader::WIRE_LEN {
            return Err("buffer too short for AtlasWireHeader".into());
        }
        let header = AtlasWireHeader::read_wire_header(&buf[..AtlasWireHeader::WIRE_LEN])?;
        let payload = buf.slice(AtlasWireHeader::WIRE_LEN..);
        Ok(Self { header, payload })
    }

    pub fn into_wire_bytes(self) -> Bytes {
        let payload_len = self.payload.len();
        let mut buf = BytesMut::with_capacity(AtlasWireHeader::WIRE_LEN + payload_len);
        buf.put_u64(self.header.id);
        buf.put_u32(self.header.slot_index);
        buf.put_u32(self.header.method);
        buf.put_u8(self.header.kind as u8);

        buf.extend_from_slice(&self.header.uid);
        // 2️⃣ 拼 payload（零拷贝语义）
        buf.extend_from_slice(&self.payload);
        buf.freeze()
    }
}

impl<T> AtlasWireMessage<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn from_raw(raw: AtlasRawMessage) -> Result<Self, String> {
        let payload = rmp_serde::from_slice(&raw.payload.as_ref()).map_err(|e| e.to_string())?;
        Ok(Self {
            header: raw.header,
            payload,
        })
    }

    pub fn into_raw(self) -> Result<AtlasRawMessage, String> {
        let payload = rmp_serde::to_vec_named(&self.payload).map_err(|e| e.to_string())?;
        Ok(AtlasRawMessage {
            header: self.header,
            payload: Bytes::from(payload),
        })
    }
}
