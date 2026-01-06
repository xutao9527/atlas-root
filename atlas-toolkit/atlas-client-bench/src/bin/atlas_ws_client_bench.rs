use atlas_client_bench::ws_client::WsClient;
use atlas_nut::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use atlas_nut::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};
use atlas_nut::net::rpc::router::AtlasMethodSpec;
use atlas_scheme::dto::auth_model::{LoginReq, LoginResp};
use atlas_scheme::module_method::auth_method::Login;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
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

    let request = AtlasWireMessage {
        header: AtlasWireHeader {
            id: 0,
            slot_index: 0,
            method: Login::WIRE,
            kind: AtlasWireKind::Request,
        },
        payload: LoginReq {
            account: "val".into(),
            password: "val".into(),
        },
    };
    let req_bytes = request.into_raw().unwrap().into_wire_bytes();
    // ================== 启动多个连接 ==================
    for conn_id in 0..connections {
        let sent = sent_total.clone();
        let recv = recv_total.clone();
        let qps = qps_counter.clone();
        let req_clone1 = req_bytes.clone();
        tokio::spawn(async move {
            let mut ws_client = WsClient::new("ws://127.0.0.1:8080/ws".to_string(),
                move |_resp| {
                    qps.fetch_add(1, Ordering::Relaxed);
                    recv.fetch_add(1, Ordering::Relaxed);
                    // let raw_msg = AtlasRawMessage::from_wire_bytes(_resp);
                    // let resp_msg = AtlasWireMessage::<LoginResp>::from_raw(raw_msg.unwrap());
                    // println!("{:?}", resp_msg);
                },
            ).await;
            ws_client.run().await;
            for _ in 0..per_conn {
                let req_clone2 = req_clone1.clone();
                ws_client.send_byte(req_clone2).await;
                sent.fetch_add(1, Ordering::Relaxed);
            }
            println!("connection {} finished", conn_id);
        });

    }
    // ================== 不让 main 退出 ==================
    loop {
        sleep(Duration::from_secs(10)).await;
    }

}
