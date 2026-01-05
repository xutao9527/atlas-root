use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct ByteFrameCodec;

impl Default for ByteFrameCodec {
    fn default() -> Self {
        Self {}
    }
}

impl Encoder<Bytes> for ByteFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, src: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = src.len() as u32;
        dst.put_u32(len);
        dst.extend_from_slice(&src);
        Ok(())
    }
}

impl Decoder for ByteFrameCodec {
    type Item = Bytes;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Bytes>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        let len = u32::from_be_bytes(src[..4].try_into().unwrap()) as usize;
        if src.len() < 4 + len {
            return Ok(None);
        }

        src.advance(4);
        let body = src.split_to(len).freeze();
        Ok(Some(body))
    }
}
