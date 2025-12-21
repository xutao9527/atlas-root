use std::sync::Arc;
use crate::net::client::connection::AtlasConnection;
use crate::net::packet::{Packet, Request};
use crate::net::router::auth::AuthMethod;
use crate::net::router::RouterMethod;
use std::sync::atomic::{AtomicU64, Ordering};


pub struct AtlasRpcClient {
    addr: String,
    next_req_id: AtomicU64,
    connections: Vec<Arc<AtlasConnection>>, // 多连接
}

impl AtlasRpcClient {
    pub fn new(addr: &str, conn_num: usize) -> Self {
        Self {
            addr: addr.to_string(),
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

    pub async fn call_cb<F: FnOnce(Packet) + Send + 'static>(&mut self, callback: F) {
        let req_id = self.next_req_id.fetch_add(1, Ordering::Relaxed);
        let req = Request {
            id: req_id,
            slot_index: req_id as usize,
            method: AuthMethod::SignIn.wire(),
            payload: vec![],
        };
        //self.pending.insert(&mut req, Box::new(callback)).await;
        let packet = Packet::Request(req);
        let idx = (req_id as usize) % self.connections.len();
        self.connections[idx].send(packet,callback).await;
    }
}
