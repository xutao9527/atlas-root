use atlas_core::net::rpc::client::client::AtlasRpcClient;
use atlas_core::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_core::net::rpc::router::AtlasRpcSpec;
use atlas_scheme::dto::auth_model::LoginReq;
use atlas_scheme::module_method::auth_method::LoginRpc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::sleep;

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

    let request = AtlasWireMessage {
        header: AtlasWireHeader {
            id: 0,
            slot_index: 0,
            method: LoginRpc::WIRE,
            kind: AtlasWireKind::Request,
        },
        payload: LoginReq {
            account: "val1".into(),
            password: "val2".into(),
        },
    };

    let req_bytes = request.into_raw().unwrap().into_wire_bytes();

    let total_requests = 10_0000_0000; // 总共发多少次

    let mut client = AtlasRpcClient::new("127.0.0.1:5566".into(), 4);
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
                    match AtlasWireHeader::read_wire_header(&resp) {
                        Ok(header) => {
                            if header.kind == AtlasWireKind::ResponseOk {
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
                    // let resp_msg = AtlasWireMessage::<LoginResp>::from_raw(raw_msg.unwrap());
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
