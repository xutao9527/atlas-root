use atlas_scheme::proto::auth::rpc::{AuthResp, BasicAuthReq};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio_util::codec::Framed;
use atlas_core::net::codec::frame_codec::FrameWireCodec;
use atlas_core::net::core::rpc::AtlasRpcSpec;
use atlas_core::net::protocol::frame::{AtlasFrame, AtlasRawFrame};
use atlas_core::net::protocol::frame_header::AtlasFrameHeader;
use atlas_core::net::protocol::frame_kind::AtlasFrameKind;
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
                    let raw_msg = AtlasRawFrame::from_bytes(resp);
                    let resp_msg = AtlasFrame::<AuthResp>::from_raw(raw_msg.unwrap());
                    println!("{:?}", resp_msg);
                }
                Err(_) => break,
            }
        }
    });

    let request = AtlasFrame {
        header: AtlasFrameHeader {
            id: 1,
            slot_index: 1,
            op_code: BasicAuthRpc::WIRE,
            kind: AtlasFrameKind::Request,
            uid: [0; 16],
        },
        body: BasicAuthReq {
            account: "val".into(),
            password: "val".into(),
        },
    };

    let request_bytes = request.into_raw().unwrap().into_bytes();
    socket_writer.send(request_bytes).await?;
    sleep(Duration::from_secs(3)).await;
    Ok(())
}
