use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct FrameWireCodec;

impl Default for FrameWireCodec {
    fn default() -> Self {
        Self {}
    }
}

impl Encoder<Bytes> for FrameWireCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = item.len() as u32;
        dst.put_u32(len);
        dst.extend_from_slice(&item);
        Ok(())
    }
}

impl Decoder for FrameWireCodec {
    type Item = Bytes;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }
        let len = (&src[..4]).get_u32() as usize;
        if src.len() < 4 + len {
            return Ok(None);
        }
        src.advance(4);
        let frame = src.split_to(len);
        Ok(Some(frame.freeze()))
    }
}