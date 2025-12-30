use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let total = Arc::new(AtomicU64::new(0));

    // QPS 统计
    {
        let total = total.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                println!("QPS: {}", total.swap(0, Ordering::Relaxed));
            }
        });
    }

    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(2000)
        //.http2_prior_knowledge()
        .tcp_nodelay(true)
        .build()
        .unwrap();

    let workers = 16;

    for _ in 0..workers {
        let client = client.clone();
        let total = total.clone();

        tokio::spawn(async move {
            loop {
                match client.get("http://127.0.0.1:8080/")
                    .send()
                    .await
                {
                    Ok(_) => {
                        total.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(_) => {
                        // ignore
                    }
                }
            }
        });
    }

    // 防止 main 退出
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
