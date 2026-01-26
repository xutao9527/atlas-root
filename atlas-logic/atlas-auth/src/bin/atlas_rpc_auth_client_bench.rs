use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::sleep;
use atlas_core::net::client::client::AtlasNetClient;
use atlas_core::net::core::reg_node::AtlasRegNodeId;
use atlas_core::net::core::rpc::AtlasRpcSpec;
use atlas_core::net::protocol::frame::AtlasFrame;
use atlas_core::net::protocol::frame_header::AtlasFrameHeader;
use atlas_core::net::protocol::frame_kind::AtlasFrameKind;
use atlas_scheme::proto::auth::rpc::BasicAuthReq;
use atlas_scheme::proto::rpc_def::auth_method::BasicAuthRpc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 每秒统计 QPS
    let success_counter = Arc::new(AtomicUsize::new(0));
    let fail_counter = Arc::new(AtomicUsize::new(0));

    // 总发送 / 总收到
    let sent_total = Arc::new(AtomicUsize::new(0));
    let recv_total = Arc::new(AtomicUsize::new(0));
    {
        let success = success_counter.clone();
        let fail = fail_counter.clone();
        let sent = sent_total.clone();
        let recv = recv_total.clone();
        tokio::spawn(async move {
            loop {
                let _s = success.swap(0, Ordering::Relaxed);
                let _f = fail.swap(0, Ordering::Relaxed);
                let _sent_val = sent.load(Ordering::Relaxed);
                let _recv_val = recv.load(Ordering::Relaxed);
                println!(
                    "QPS: {}, Failures: {}, Sent Total: {}, Recv Total: {}",
                    _s, _f, _sent_val, _recv_val
                );
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    let request = AtlasFrame {
        header: AtlasFrameHeader {
            id: 0,
            slot_index: 0,
            op_code: BasicAuthRpc::OP_CODE,
            kind: AtlasFrameKind::Request,
            uid: [0; 16],
        },
        body: BasicAuthReq {
            account: "111".into(),
            password: "123123".into(),
        },
    };

    let req_bytes = request.into_raw().unwrap().into_bytes();

    let total_requests = 10_0000_0000; // 总共发多少次

    let mut client = AtlasNetClient::new("127.0.0.1:5566".into(), AtlasRegNodeId::AuthNode(2),4);
    if let Ok(_) = client.connect().await {
        for _i in 0..total_requests {
            let success = success_counter.clone();
            let fail = fail_counter.clone();
            let sent = sent_total.clone();
            let recv = recv_total.clone();

            let req_clone = req_bytes.clone();
            client
                .call_cb(req_clone, |resp| async move {
                    recv.fetch_add(1, Ordering::Relaxed);
                    match AtlasFrameHeader::read_wire_header(&resp) {
                        Ok(header) => {
                            if header.kind == AtlasFrameKind::ResponseOk {
                                success.fetch_add(1, Ordering::Relaxed);
                            } else {
                                fail.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                        Err(_) => {
                            fail.fetch_add(1, Ordering::Relaxed);
                        }
                    };
                    // let raw_msg = AtlasRawMessage::from_wire_bytes(resp);
                    // let resp_msg = AtlasFrame::<LoginResp>::from_raw(raw_msg.unwrap());
                    // println!("{:?}", resp_msg);
                })
                .await;
            sent.fetch_add(1, Ordering::Relaxed);
            // sleep(Duration::from_secs(1)).await;
            // if (i + 1) % batch_size == 0 {
            //     sleep(Duration::from_secs(1)).await;
            // }
        }
    }
    loop {
        sleep(Duration::from_secs(3)).await;
    }
}
