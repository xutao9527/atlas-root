use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use crate::net::packet::{AtlasRequest, AtlasResponse};
use crate::net::router::RouterMethod;

pub trait AsyncHandler: Send + Sync + 'static {
    fn call(&self, req: AtlasRequest) -> Pin<Box<dyn Future<Output=AtlasResponse> + Send>>;
}

impl<F, Fut> AsyncHandler for F
where
    F: Fn(AtlasRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=AtlasResponse> + Send + 'static,
{
    fn call(&self, req: AtlasRequest) -> Pin<Box<dyn Future<Output=AtlasResponse> + Send>> {
        Box::pin(self(req))
    }
}

#[derive(Default)]
pub struct AtlasRouter {
    routes: HashMap<u32, Arc<dyn AsyncHandler>>,
}

impl AtlasRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<M, H>(&mut self, method: M, handler: H)
    where
        M: RouterMethod,
        H: AsyncHandler,
    {
        self.routes.insert(method.wire(), Arc::new(handler));
    }

    /// 分发（异步）
    pub async fn dispatch(&self, req: AtlasRequest) -> AtlasResponse {
        match self.routes.get(&req.method) {
            Some(handler) => handler.call(req).await,
            None => AtlasResponse {
                id: req.id,
                slot_index: req.slot_index,
                payload: Vec::new(),
                error: Some("method not found".into()),
            },
        }
    }
}
