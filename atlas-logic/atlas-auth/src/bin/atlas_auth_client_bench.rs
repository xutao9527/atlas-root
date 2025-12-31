use std::time::Duration;
use tokio::time::sleep;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use atlas_auth::rpc::method::AuthMethod;
use atlas_core::net::rpc::client::client_rpc::AtlasRpcClient;
use atlas_core::net::rpc::packet::AtlasRequest;
use atlas_core::net::rpc::router_spec::AtlasRouterMethod;

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

    let total_requests = 20000_0000; // 总共发多少次
    let success = success_counter.clone();
    let fail = fail_counter.clone();
    let sent = sent_total.clone();
    let recv = recv_total.clone();
    let mut client = AtlasRpcClient::new("127.0.0.1:5566".into(), 4);
    let _batch_size = 100;
    if let Ok(_) = client.connect().await {
        for _i in 0..total_requests {
            let _success = success.clone();
            let _fail = fail.clone();
            let _recv = recv.clone();
            let req = AtlasRequest {
                id: 0,
                slot_index: 0 as usize,
                method: AuthMethod::Login.wire(),
                payload: vec![],
            };
            //let packet = AtlasPacket::AtlasRequest(req);
            client.call_cb(req, move |_resp| {
                _success.fetch_add(1, Ordering::Relaxed);
                _recv.fetch_add(1, Ordering::Relaxed);
                //println!("callback {:?}", resp);
            }).await;
            sent.fetch_add(1, Ordering::Relaxed);
            // 每 _batch_size 个请求暂停 1 秒
            // if (i + 1) % batch_size == 0 {
            //     tokio::time::sleep(Duration::from_millis(10)).await;
            // }
        }
    }
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
