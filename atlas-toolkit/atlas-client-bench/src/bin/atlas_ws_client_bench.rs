use atlas_client_bench::ws_client::WsClient;
use atlas_core::AtlasMethodSpec;
use atlas_core::net::rpc::packet::AtlasPacket;
use atlas_core::net::rpc::packet_request::AtlasWireRequest;
use atlas_core::net::rpc::packet_response::AtlasWireResponse;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp};
use atlas_scheme::module_method::auth_method;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // 每秒统计 QPS
    let success_counter = Arc::new(AtomicUsize::new(0));
    // 总发送 / 总收到
    let sent_total = Arc::new(AtomicUsize::new(0));
    let recv_total = Arc::new(AtomicUsize::new(0));
    {
        let success = success_counter.clone();

        let sent = sent_total.clone();
        let recv = recv_total.clone();
        tokio::spawn(async move {
            loop {
                let _s = success.swap(0, Ordering::Relaxed);
                let _sent_val = sent.load(Ordering::Relaxed);
                let _recv_val = recv.load(Ordering::Relaxed);
                println!(
                    "QPS: {}, Sent Total: {}, Recv Total: {}",
                    _s, _sent_val, _recv_val
                );
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    let success = success_counter.clone();
    let recv = recv_total.clone();
    let mut ws_client = WsClient::new("ws://127.0.0.1:8080/ws".to_string(), move |resp| {
        success.fetch_add(1, Ordering::Relaxed);
        recv.fetch_add(1, Ordering::Relaxed);
        let _resp = AtlasWireResponse::<LoginResp>::from_raw(resp);
        //println!("ws client Received : {:?}", resp);
    })
    .await;

    ws_client.run().await;
    let total_requests = 200_0000usize;
    for _i in 0..total_requests {
        let req = AtlasWireRequest {
            id: 0,
            slot_index: 0 as usize,
            method: auth_method::Login::WIRE,
            payload: LoginReq {
                account: "test".to_string(),
                password: "test".to_string(),
            },
        };
        let raw_req = req.into_raw().unwrap();
        let packet = AtlasPacket::AtlasRequest(raw_req);
        let buf = rmp_serde::to_vec(&packet).unwrap();
        ws_client.send_byte(buf).await;
        sent_total.fetch_add(1, Ordering::Relaxed);
    }
    loop{
        sleep(Duration::from_secs(3)).await;
    }
}
