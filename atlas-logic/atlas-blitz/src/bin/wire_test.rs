use atlas_blitz::dto::RegisterReq;
use atlas_nut::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use atlas_nut::net::rpc::packet_message::AtlasWireMessage;
use bytes::{Buf, BufMut, Bytes, BytesMut};

fn main() {
    // 1️⃣ 构造请求
    let request = AtlasWireMessage {
        header: AtlasWireHeader {
            id: 1,
            slot_index: 2,
            method: 3,
            kind: AtlasWireKind::Request,
        },
        payload: RegisterReq {
            account: "val".into(),
            password: "val".into(),
        },
    };

    let payload_bytes = rmp_serde::to_vec(&request.payload).expect("payload encode failed");

    // 3️⃣ 分配 wire buffer（header 固定 17 字节）
    let mut wire = BytesMut::with_capacity(21 + payload_bytes.len());

    // 4️⃣ 手动写 AtlasWireHeader（固定布局）
    wire.put_u64(request.header.id); // 8
    wire.put_u32(request.header.slot_index); // 4
    wire.put_u32(request.header.method); // 4
    wire.put_u8(request.header.kind as u8); // 1

    // 2️⃣ payload → msgpack bytes（只序列化 payload）
    let payload_bytes = rmp_serde::to_vec(&request.payload).expect("payload encode failed");

    // 3️⃣ 分配 wire buffer（header 固定 21 字节）
    let mut wire = BytesMut::with_capacity(21 + payload_bytes.len());

    // 4️⃣ 手动写 AtlasWireHeader（固定布局）
    wire.put_u64(request.header.id); // 8
    wire.put_u32(request.header.slot_index); // 8
    wire.put_u32(request.header.method); // 4
    wire.put_u8(request.header.kind as u8); // 1
    // total = 21 bytes

    // 5️⃣ 拼 payload（msgpack）
    wire.extend_from_slice(&payload_bytes);

    // 6️⃣ freeze 成 Bytes（这就是 wire bytes）
    let wire_bytes = Bytes::from(wire);

    // 7️⃣ 打印验证
    println!("wire len = {}", wire_bytes.len());
    println!("wire bytes = {:02x?}", wire_bytes);

    // ======== 修改AtlasWireHeader ========
    let mut wire_mut = BytesMut::from(&wire_bytes[..]);

    // 修改 id
    {
        let mut p = &mut wire_mut[0..8];
        p.put_u64(10086);
    }

    // 修改 slot_index
    {
        let mut p = &mut wire_mut[8..12];
        p.put_u32(42);
    }

    // 修改 method
    {
        let mut p = &mut wire_mut[12..16];
        p.put_u32(999);
    }

    // 修改 kind
    wire_mut[16] = AtlasWireKind::ResponseOk as u8;

    // ======== 验证反向解析 ========
    let wire_bytes = wire_mut.freeze();
    let mut buf = wire_bytes.clone();

    let id = buf.get_u64();
    let slot_index = buf.get_u32();
    let method = buf.get_u32();
    let kind = buf.get_u8();

    println!(
        "decoded header => id={}, slot_index={}, method={}, kind={}",
        id, slot_index, method, kind
    );

    let decoded_payload: RegisterReq = rmp_serde::from_slice(&buf).expect("payload decode failed");

    println!("decoded payload = {:?}", decoded_payload);
}
