use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::net::rpc::packet::{AtlasRawRequest, AtlasRawResponse, AtlasRequest, AtlasResponse};
use crate::net::rpc::router_spec::AtlasRouterMethod;


pub trait AsyncHandler: Send + Sync + 'static {
    fn call(&self, req: AtlasRawRequest) -> Pin<Box<dyn Future<Output=AtlasRawResponse> + Send>>;
}

impl<F, Fut> AsyncHandler for F
where
    F: Fn(AtlasRawRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=AtlasRawResponse> + Send + 'static,
{
    fn call(&self, req: AtlasRawRequest) -> Pin<Box<dyn Future<Output=AtlasRawResponse> + Send>> {
        Box::pin(self(req))
    }
}

pub fn adapter_handler<Req, Resp, F, Fut>(f: F) -> impl Fn(AtlasRawRequest) -> Pin<Box<dyn Future<Output=AtlasRawResponse> + Send>> + Send + Sync + 'static
where
    Req: Serialize + DeserializeOwned + Send + 'static,
    Resp: Serialize + DeserializeOwned + Send + 'static,
    F: Fn(AtlasRequest<Req>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=AtlasResponse<Resp>> + Send + 'static,
{
    let f = Arc::new(f);
    move |raw: AtlasRawRequest| {
        let f = Arc::clone(&f);
        Box::pin(async move {
            let req = match AtlasRequest::<Req>::from_raw(raw.clone()) {
                Ok(r) => r,
                Err(e) => {
                    return AtlasRawResponse {
                        id: raw.id,
                        slot_index: raw.slot_index,
                        payload: Vec::new(),
                        error: Some(e),
                    };
                }
            };
            let resp = f(req).await;
            resp.into_raw()
        })
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
        M: AtlasRouterMethod,
        H: AsyncHandler,
    {
        self.routes.insert(method.wire(), Arc::new(handler));
    }

    /// 分发（异步）
    pub async fn dispatch(&self, req: AtlasRawRequest) -> AtlasRawResponse {
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
