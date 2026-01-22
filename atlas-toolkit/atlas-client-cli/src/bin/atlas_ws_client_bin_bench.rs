use atlas_core::net::rpc::packet_header::{AtlasWireHeader, AtlasWireKind};
use atlas_core::net::rpc::packet_message::AtlasWireMessage;
use atlas_scheme::proto::auth::rpc::{BasicAuthReq};
use atlas_scheme::module_method::auth_method::{BasicAuthRpc};
use futures_util::{SinkExt, StreamExt};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use atlas_core::net::rpc::router::AtlasRpcSpec;

const CONNECTIONS: usize = 4;
const INFLIGHT_PER_CONN: usize = 1024;
const MAX_MSGS_PER_CONN: usize = 1_0000_0000;//1_0000_0000;

#[tokio::main]
async fn main() {
    let sent_total = Arc::new(AtomicUsize::new(0));
    let recv_total = Arc::new(AtomicUsize::new(0));
    let qps_counter = Arc::new(AtomicUsize::new(0));

    // ===== QPS 统计 =====
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
                    "QPS: {:>8}, Sent: {:>10}, Recv: {:>10}",
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
            method: BasicAuthRpc::WIRE,
            kind: AtlasWireKind::Request,
            uid: [0; 16],
        },
        payload: BasicAuthReq {
            account: "val".into(),
            password: "val".into(),
        },
    };
    let _req_bytes = request.into_raw().unwrap().into_wire_bytes();

    // ===== 启动连接 =====
    for conn_id in 0..CONNECTIONS  {
        let sent_total = sent_total.clone();
        let recv_total = recv_total.clone();
        let qps_counter = qps_counter.clone();
        let req_bytes_clone = _req_bytes.clone();
        tokio::spawn(async move {
            let (ws, _) = connect_async("ws://127.0.0.1:8080/ws").await.unwrap();
            let (mut tx, mut rx) = ws.split();

            let sent_conn = Arc::new(AtomicUsize::new(0));
            let recv_conn = Arc::new(AtomicUsize::new(0));

            // receiver
            let recv_conn_clone = recv_conn.clone();
            let recv_task = tokio::spawn(async move {
                while let Some(msg) = rx.next().await {
                    if msg.is_ok() {
                        recv_conn_clone.fetch_add(1, Ordering::Relaxed);
                        recv_total.fetch_add(1, Ordering::Relaxed);
                        qps_counter.fetch_add(1, Ordering::Relaxed);
                    }
                    match msg {
                        Ok(Message::Text(_text)) => {
                            // println!("Received: {}", text);
                        }
                        Ok(Message::Binary(_bin)) => {
                            // let raw_msg = AtlasRawMessage::from_wire_bytes(bin);
                            // let resp_msg = AtlasWireMessage::<LoginResp>::from_raw(raw_msg.unwrap());
                            // println!("{:?}", resp_msg);
                        }
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            });

            // ===== sender（受控 inflight）=====
            while sent_conn.load(Ordering::Relaxed) < MAX_MSGS_PER_CONN {
                for _ in 0..INFLIGHT_PER_CONN {
                    let cur = sent_conn.fetch_add(1, Ordering::Relaxed);
                    if cur >= MAX_MSGS_PER_CONN {
                        break;
                    }
                    let req_bytes_clone = req_bytes_clone.clone();
                    if tx.send(Message::Binary(req_bytes_clone)).await.is_err() {
                        break;
                    }

                    sent_total.fetch_add(1, Ordering::Relaxed);
                }

                // 给 receiver / runtime 让出执行权
                tokio::task::yield_now().await;
            }

            // ===== 等所有 echo 收齐 =====
            while recv_conn.load(Ordering::Relaxed)
                < sent_conn.load(Ordering::Relaxed)
            {
                tokio::task::yield_now().await;
            }

            // ===== 再关闭 =====
            let _ = tx.close().await;
            let _ = recv_task.await;

            println!(
                "connection {} finished: sent={}, recv={}",
                conn_id,
                sent_conn.load(Ordering::Relaxed),
                recv_conn.load(Ordering::Relaxed),
            );
        });
    }

    // 主线程不退出
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
