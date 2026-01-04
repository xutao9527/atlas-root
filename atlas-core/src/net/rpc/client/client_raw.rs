use crate::net::rpc::client::connection_raw::AtlasRawConnection;
use bytes::{Bytes, BytesMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct AtlasRpcRawClient {
    addr: String,
    next_req_id: AtomicU64,
    connections: Vec<Arc<AtlasRawConnection>>,
}

impl AtlasRpcRawClient {
    pub fn new(addr: String, conn_num: usize) -> Self {
        Self {
            addr,
            next_req_id: AtomicU64::new(1),
            connections: Vec::with_capacity(conn_num),
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        for _ in 0..self.connections.capacity() {
            let connection = Arc::new(
                AtlasRawConnection::new(self.addr.clone()).await?
            );
            connection.clone().connect().await;
            self.connections.push(connection);
        }
        Ok(())
    }

    pub async fn call_raw_cb<F: FnOnce(Bytes) + Send + 'static>(& self, req_buf: Bytes, callback: F) {
        let req_id = self.next_req_id.fetch_add(1, Ordering::Relaxed);
        let idx = (req_id as usize) % self.connections.len();
        self.connections[idx].send(req_id, req_buf.into(), callback).await;
    }
}
