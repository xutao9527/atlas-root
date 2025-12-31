use bytes::{Buf, BufMut, BytesMut};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;
use tokio_util::codec::{Decoder, Encoder};

pub struct MsgPackCodec<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for MsgPackCodec<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Encoder<T> for MsgPackCodec<T>
where
    T: Serialize,
{
    type Error = anyhow::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let body = rmp_serde::to_vec(&item)?;
        let len = body.len() as u32;

        dst.put_u32(len);
        dst.extend_from_slice(&body);
        Ok(())
    }
}

impl<T> Decoder for MsgPackCodec<T>
where
    T: DeserializeOwned,
{
    type Item = T;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        let mut len_buf = &src[..4];
        let len = len_buf.get_u32() as usize;

        if src.len() < 4 + len {
            return Ok(None);
        }

        src.advance(4);
        let body = src.split_to(len);
        let msg = rmp_serde::from_slice(&body)?;
        Ok(Some(msg))
    }
}
