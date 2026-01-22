use atlas_core::net::rpc::codec::FrameWireCodec;
use atlas_core::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use atlas_core::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};
use atlas_core::net::rpc::router::AtlasRpcSpec;
use atlas_scheme::proto::auth::rpc::{AuthResp, BasicAuthReq};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_util::codec::Framed;
use atlas_scheme::module_method::auth_method::BasicAuthRpc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:5566".to_string()).await?;
    let framed = Framed::new(stream, FrameWireCodec::default());
    let (mut socket_writer, mut socket_reader) = framed.split();

    tokio::spawn(async move {
        while let Some(result) = socket_reader.next().await {
            match result {
                Ok(resp) => {
                    let raw_msg = AtlasRawMessage::from_wire_bytes(resp);
                    let resp_msg = AtlasWireMessage::<AuthResp>::from_raw(raw_msg.unwrap());
                    println!("{:?}", resp_msg);
                }
                Err(_) => break,
            }
        }
    });

    let request = AtlasWireMessage {
        header: AtlasWireHeader {
            id: 1,
            slot_index: 1,
            method: BasicAuthRpc::WIRE,
            kind: AtlasWireKind::Request,
            uid: [0; 16],
        },
        payload: BasicAuthReq {
            account: "val".into(),
            password: "val".into(),
        },
    };

    let request_bytes = request.into_raw().unwrap().into_wire_bytes();
    socket_writer.send(request_bytes).await?;
    sleep(Duration::from_secs(3)).await;
    Ok(())
}
