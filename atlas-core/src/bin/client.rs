use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use atlas_core::net::client::AtlasNetClient;

const SERVER_ADDR: &str = "127.0.0.1:9001";
const CONNECTIONS: usize = 5;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> anyhow::Result<()> {
    let success_counter = Arc::new(AtomicUsize::new(0));
    let fail_counter = Arc::new(AtomicUsize::new(0));

    // 每秒统计 QPS
    {
        let success = success_counter.clone();
        let fail = fail_counter.clone();
        tokio::spawn(async move {
            loop {
                let s = success.swap(0, Ordering::Relaxed);
                let f = fail.swap(0, Ordering::Relaxed);
                println!("QPS: {}, Failures: {}", s, f);
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    for _ in 0..CONNECTIONS {
        let success = success_counter.clone();
        let fail = fail_counter.clone();

        tokio::spawn(async move {
            if let Ok(client) = AtlasNetClient::connect(SERVER_ADDR).await {
                loop {
                    let payload = b"hello".to_vec();
                    let success = success.clone();
                    let fail = fail.clone();
                    // 基于回调发送请求
                    match client.call(payload).await {
                        Ok(_) => {
                            success.fetch_add(1, Ordering::Relaxed);
                        },
                        Err(_) => {
                            fail.fetch_add(1, Ordering::Relaxed);
                            break;
                        }
                    }
                }
            } else {
                fail.fetch_add(1, Ordering::Relaxed);
            }
        });
    }

    loop { sleep(Duration::from_secs(60)).await; }
}
