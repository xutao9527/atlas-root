use bytes::{Buf, BufMut, BytesMut};
use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use tokio_util::codec::{Decoder, Encoder};

use std::io;
use std::marker::PhantomData;

pub struct JsonCodec<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for JsonCodec<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> JsonCodec<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Encoder<T> for JsonCodec<T>
where
    T: Serialize,
{
    type Error = io::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let vec = serde_json::to_vec(&item).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        dst.put_u32(vec.len() as u32); // 先写长度
        dst.extend_from_slice(&vec);
        Ok(())
    }
}

impl<T> Decoder for JsonCodec<T>
where
    T: DeserializeOwned,
{
    type Item = T;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<T>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }
        let len = u32::from_be_bytes(src[..4].try_into().unwrap()) as usize;
        if src.len() < 4 + len {
            return Ok(None);
        }
        let data = src[4..4 + len].to_vec();
        src.advance(4 + len);
        let item =
            serde_json::from_slice(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Some(item))
    }
}
