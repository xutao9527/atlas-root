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
    // ================== 参数 ==================
    let connections: usize = 4;            // WS 连接数
    let total_requests: usize = 2_0000_0000; // 总请求数
    let per_conn = total_requests / connections;

    // ================== 统计 ==================
    let sent_total = Arc::new(AtomicUsize::new(0));
    let recv_total = Arc::new(AtomicUsize::new(0));
    let qps_counter = Arc::new(AtomicUsize::new(0));

    // ================== QPS 打印 ==================
    {
        let sent = sent_total.clone();
        let recv = recv_total.clone();
        let qps = qps_counter.clone();

        tokio::spawn(async move {
            loop {
                let qps_val = qps.swap(0, Ordering::Relaxed);
                let sent_val = sent.load(Ordering::Relaxed);
                let recv_val = recv.load(Ordering::Relaxed);

                println!(
                    "QPS: {:>7}, Sent: {:>10}, Recv: {:>10}",
                    qps_val, sent_val, recv_val
                );

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // ================== 启动多个连接 ==================
    for conn_id in 0..connections {
        let sent = sent_total.clone();
        let recv = recv_total.clone();
        let qps = qps_counter.clone();

        tokio::spawn(async move {
            let mut ws_client = WsClient::new(
                "ws://127.0.0.1:8080/ws".to_string(),
                move |resp| {
                    qps.fetch_add(1, Ordering::Relaxed);
                    recv.fetch_add(1, Ordering::Relaxed);
                    let _ =
                        AtlasWireResponse::<LoginResp>::from_raw(resp);
                },
            )
                .await;

            ws_client.run().await;

            // for _ in 0..per_conn {
            //     let req = AtlasWireRequest {
            //         id: 0,
            //         slot_index: 0,
            //         method: auth_method::Login::WIRE,
            //         payload: LoginReq {
            //             account: "test".to_string(),
            //             password: "test".to_string(),
            //         },
            //     };
            //
            //     let raw = req.into_raw().unwrap();
            //     let packet = AtlasPacket::AtlasRequest(raw);
            //     let buf = rmp_serde::to_vec(&packet).unwrap();
            //
            //     ws_client.send_byte(buf).await;
            //     sent.fetch_add(1, Ordering::Relaxed);
            // }

            println!("connection {} finished", conn_id);
        });

    }
    // ================== 不让 main 退出 ==================
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}
