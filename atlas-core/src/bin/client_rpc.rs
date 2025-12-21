use std::time::Duration;
use tokio::time::sleep;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use atlas_core::net::client::client_rpc::AtlasRpcClient;
use atlas_core::net::packet::Packet;

const SERVER_ADDR: &str = "127.0.0.1:9001";

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
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
                let s = success.swap(0, Ordering::Relaxed);
                let f = fail.swap(0, Ordering::Relaxed);
                let sent_val = sent.load(Ordering::Relaxed);
                let recv_val = recv.load(Ordering::Relaxed);
                // println!(
                //     "QPS: {}, Failures: {}, Sent Total: {}, Recv Total: {}",
                //     s, f, sent_val, recv_val
                // );
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    let total_requests = 10_0000_0000; // 总共发多少次
    let success = success_counter.clone();
    let fail = fail_counter.clone();
    let sent = sent_total.clone();
    let recv = recv_total.clone();
    let mut client = AtlasRpcClient::new(SERVER_ADDR, 1);
    let batch_size = 100;
    if let Ok(_) = client.connect().await {
        for i in 0..total_requests {
            let success = success.clone();
            let fail = fail.clone();
            let recv = recv.clone();
            client
                .send(move |res| {
                    match res {
                        Packet::Response(_resp) => {
                            success.fetch_add(1, Ordering::Relaxed);
                            recv.fetch_add(1, Ordering::Relaxed);
                            //println!("callback {:?}", resp);
                        }
                        _ => {
                            fail.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                })
                .await;
            sent.fetch_add(1, Ordering::Relaxed);
            // 每 batch_size 个请求暂停 1 秒
            if (i + 1) % batch_size == 0 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    }
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
