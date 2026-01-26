use crate::net::client::connection::AtlasConnection;
use crate::net::core::reg_node::AtlasRegNodeId;
use crate::net::protocol::{AtlasNotifyTarget, AtlasRawFrame};
use bytes::Bytes;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Mutex;

pub type NotifyHandler = Arc<
    dyn Fn(Vec<AtlasNotifyTarget>, AtlasRawFrame) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

pub type NotifyDispatch =
    Arc<dyn Fn(Bytes, NotifyHandler) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct AtlasNetClient {
    addr: String,
    logical_id: AtlasRegNodeId,
    next_req_id: AtomicU64,
    connections: Vec<Arc<AtlasConnection>>,
    notify_handler: Arc<Mutex<Option<NotifyHandler>>>,
    notify_dispatcher: Arc<Mutex<Option<NotifyDispatch>>>,
}

impl AtlasNetClient {
    pub fn new(addr: String, logical_id: AtlasRegNodeId, con_num: usize) -> Self {
        Self {
            addr,
            logical_id,
            next_req_id: AtomicU64::new(1),
            connections: Vec::with_capacity(con_num),
            notify_handler: Arc::new(Mutex::new(None)),
            notify_dispatcher: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        for _ in 0..self.connections.capacity() {
            let connection = Arc::new(AtlasConnection::new(
                self.addr.clone(),
                self.logical_id.clone(),
                self.notify_handler.clone(),
                self.notify_dispatcher.clone(),
            ));
            connection.clone().connect().await;
            self.connections.push(connection);
        }
        Ok(())
    }

    pub async fn call_cb<F, Fut>(&self, req_buf: Bytes, callback: F)
    where
        F: FnOnce(Bytes) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let req_id = self.next_req_id.fetch_add(1, Ordering::Relaxed);
        let idx = (req_id as usize) % self.connections.len();
        self.connections[idx]
            .send(req_id, req_buf.into(), callback)
            .await;
    }

    pub async fn set_notify_handler<F1, Fut1, F2, Fut2>(
        &self,
        handler: Option<F1>,
        dispatcher: Option<F2>,
    ) where
        F1: Fn(Vec<AtlasNotifyTarget>, AtlasRawFrame) -> Fut1 + Send + Sync + 'static,
        Fut1: Future<Output = ()> + Send + 'static,
        F2: Fn(Bytes, NotifyHandler) -> Fut2 + Send + Sync + 'static,
        Fut2: Future<Output = ()> + Send + 'static,
    {
        // notify_handler
        if let Some(f) = handler {
            let mut guard = self.notify_handler.lock().await;
            *guard = Some(Arc::new(move |targets, raw| Box::pin(f(targets, raw))));
        }

        // notify_dispatcher
        if let Some(f) = dispatcher {
            let mut guard = self.notify_dispatcher.lock().await;
            *guard = Some(Arc::new(move |bytes, notify_handler| {
                Box::pin(f(bytes, notify_handler))
            }));
        }
    }
}
