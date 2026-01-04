use std::io::Cursor;

use crate::net::rpc::packet_request::AtlasRawRequest;
use crate::net::rpc::packet_response::AtlasRawResponse;
use bytes::BytesMut;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AtlasPacket {
    AtlasRequest(AtlasRawRequest),
    AtlasResponse(AtlasRawResponse),
}

#[derive(Debug, Clone, Copy)]
pub struct AtlasWireHeader {
    pub id: u64,
    pub slot_index: u64,
    pub method: u32,
}

pub fn read_wire_header_only(buf: &[u8]) -> Result<AtlasWireHeader, String> {
    let mut cur = Cursor::new(buf.as_ref());
    rmp::decode::read_array_len(&mut cur).map_err(|e| e.to_string())?;
    let id = rmp::decode::read_int(&mut cur).map_err(|e| e.to_string())?;
    let slot_index =  rmp::decode::read_int(&mut cur).map_err(|e| e.to_string())?;
    let method =  rmp::decode::read_int(&mut cur).map_err(|e| e.to_string())?;
    Ok(AtlasWireHeader {
        id,
        slot_index,
        method,
    })
}

pub fn write_wire_header_only(buf: &mut BytesMut, req_id: u64, slot_index: u64) -> Result<(), String> {
    let mut cur = std::io::Cursor::new(&buf[..]);

    let array_len = rmp::decode::read_array_len(&mut cur).map_err(|e| e.to_string())?;
    if array_len < 4 {
        return Err(format!("invalid array length {}, expected >=4", array_len));
    }

    let _old_id: u64 = rmp::decode::read_int(&mut cur).map_err(|e| e.to_string())?;
    let _old_slot: u64 = rmp::decode::read_int(&mut cur).map_err(|e| e.to_string())?;
    let method: u32 = rmp::decode::read_int(&mut cur).map_err(|e| e.to_string())?;

    let payload_offset = cur.position() as usize;

    // payload 内存零拷贝
    let payload_bytes = buf.split_off(payload_offset);

    // 临时 Vec 写 header
    let mut header_buf = Vec::new();
    rmp::encode::write_array_len(&mut header_buf, array_len).map_err(|e| e.to_string())?;
    rmp::encode::write_uint(&mut header_buf, req_id).map_err(|e| e.to_string())?;
    rmp::encode::write_uint(&mut header_buf, slot_index).map_err(|e| e.to_string())?;
    rmp::encode::write_uint(&mut header_buf, method as u64).map_err(|e| e.to_string())?;

    buf.clear();
    buf.extend_from_slice(&header_buf);
    buf.unsplit(payload_bytes);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::net::rpc::packet_request::AtlasWireRequest;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct LoginReq {
        account: String,
        password: String,
    }

    #[test]
    fn test_decode_header_only() {
        let req = AtlasWireRequest {
            id: 42,
            slot_index: 7,
            method: 1001,
            payload: LoginReq {
                account: "test".into(),
                password: "pwd".into(),
            }
        };
        let buf = rmp_serde::to_vec(&req.into_raw().unwrap()).unwrap();
        let mut bytes = BytesMut::from(buf.as_slice());


        // 读取 header
        let header = read_wire_header_only(&bytes).unwrap();
        let slice = rmp_serde::from_slice::<AtlasRawRequest>(&bytes);
        println!("22222222222222 AtlasNetServer received request: \n{:?} header: \n{:?} ", slice , header );

        // 修改 header
        write_wire_header_only(&mut bytes, 100, 200);

        // 再读一次，确认修改成功
        let header = read_wire_header_only(&bytes).unwrap();
        let slice = rmp_serde::from_slice::<AtlasRawRequest>(&bytes);
        println!("33333333333333 AtlasNetServer received request: \n{:?} header: \n{:?} ", slice , header );
    }
}