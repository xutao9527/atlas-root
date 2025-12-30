use crate::net::client::connection::AtlasConnection;
use crate::net::packet::AtlasPacket;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;


pub struct AtlasRpcClient {
    addr: String,
    next_req_id: AtomicU64,
    connections: Vec<Arc<AtlasConnection>>, // 多连接
}

impl AtlasRpcClient {
    pub fn new(addr: String, conn_num: usize) -> Self {
        Self {
            addr,
            next_req_id: AtomicU64::new(1),
            connections: Vec::with_capacity(conn_num),
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        for _ in 0..self.connections.capacity() {
            //let connection = AtlasConnection::new(self.addr.clone()).await?;
            let connection = Arc::new(
                AtlasConnection::new(self.addr.clone()).await?
            );
            connection.clone().connect().await;
            self.connections.push(connection);
        }
        Ok(())
    }

    pub async fn call_cb<F: FnOnce(AtlasPacket) + Send + 'static>(&mut self, mut packet: AtlasPacket, callback: F) {
        let req_id = self.next_req_id.fetch_add(1, Ordering::Relaxed);
        if let AtlasPacket::AtlasRequest(ref mut req) = packet{
            req.id = req_id;
            let idx = (req_id as usize) % self.connections.len();
            self.connections[idx].send(packet, callback).await;
        }
    }
}
