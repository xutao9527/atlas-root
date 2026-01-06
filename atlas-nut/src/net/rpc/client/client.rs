use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use bytes::Bytes;
use crate::net::rpc::client::connection::AtlasConnection;

pub struct AtlasRpcClient {
    addr: String,
    next_req_id: AtomicU64,
    connections: Vec<Arc<AtlasConnection>>,
}

impl AtlasRpcClient {
    pub fn new(addr: String, con_num: usize) -> Self {
        Self {
            addr,
            next_req_id: AtomicU64::new(1),
            connections: Vec::with_capacity(con_num),
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        for _ in 0..self.connections.capacity() {
            let connection = Arc::new(
                AtlasConnection::new(self.addr.clone())
            );
            connection.clone().connect().await;
            self.connections.push(connection);
        }
        Ok(())
    }

    pub async fn call_cb<F, Fut>(& self, req_buf: Bytes, callback: F)
    where
        F: FnOnce(Bytes) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let req_id = self.next_req_id.fetch_add(1, Ordering::Relaxed);
        let idx = (req_id as usize) % self.connections.len();
        self.connections[idx].send(req_id, req_buf.into(), callback).await;
    }
}