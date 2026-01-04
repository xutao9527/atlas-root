use std::io::Cursor;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::net::rpc::packet_request::AtlasRawRequest;
use crate::net::rpc::packet_response::AtlasRawResponse;

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

pub fn decode_wire_header_only(buf: &Bytes) -> Result<AtlasWireHeader, String> {
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


#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use serde::{Serialize, Deserialize};
    use crate::net::rpc::packet_request::AtlasWireRequest;

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
            },
        };

        let buf = Bytes::from(rmp_serde::to_vec(&req).unwrap());
        let wire_header_only = decode_wire_header_only(&buf);
        println!("wire_header_only: {:?}", wire_header_only);
      
    }
}