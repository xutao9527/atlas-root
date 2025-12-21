use crate::net::client::connection::AtlasConnection;
use crate::net::client::pending::PendingTable;
use crate::net::packet::{Packet, Request};
use crate::net::router::RouterMethod;
use crate::net::router::auth::AuthMethod;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};


pub struct AtlasRpcClient {
    addr: String,
    next_req_id: AtomicU64,
    pending: Arc<PendingTable>,
    connections: Vec<AtlasConnection>, // 多连接
}

impl AtlasRpcClient {
    pub fn new(addr: &str, conn_num: usize) -> Self {
        Self {
            addr: addr.to_string(),
            next_req_id: AtomicU64::new(1),
            pending: Arc::new(PendingTable::new(100 * 1024)),
            connections: Vec::with_capacity(conn_num),
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        for _ in 0..self.connections.capacity() {
            let conn = AtlasConnection::connect(self.addr.clone(), self.pending.clone()).await?;
            self.connections.push(conn);
        }
        Ok(())
    }

    pub async fn send<F: FnOnce(Packet) + Send + 'static>(&mut self, callback: F) {
        let req_id = self.next_req_id.fetch_add(1, Ordering::Relaxed);
        let mut req = Request {
            id: req_id,
            slot_index: req_id as usize,
            method: AuthMethod::SignIn.wire(),
            payload: vec![],
        };
        self.pending.insert(&mut req, Box::new(callback)).await;
        let packet = Packet::Request(req);
        let idx = (req_id as usize) % self.connections.len();
        self.connections[idx].send(packet).await;
    }
}
