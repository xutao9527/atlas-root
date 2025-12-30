pub mod auth;
pub mod chat;

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use crate::net::packet::{Request, Response};


/// RPC Module
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RouterModule {
    Auth = 1,
    Chat = 2,
}

/// RPC Method
pub trait RouterMethod: Copy {
    /// 所属模块
    const MODULE: RouterModule;
    /// 方法号（低 16 位）
    fn id(self) -> u16;
    #[inline(always)]
    fn wire(self) -> u32 {
        ((Self::MODULE as u32) << 16) | self.id() as u32
    }
}


#[derive(Default)]
pub struct AtlasRouter {
    routes: HashMap<u32, Arc<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>>,
}

impl AtlasRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<M: RouterMethod, F, Fut>(&mut self, method: M, handler: F)
    where
        M: RouterMethod,
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output=Response> + Send + 'static,
    {
        let handler: Arc<dyn Fn(Request) -> Pin<Box<dyn Future<Output=Response> + Send>> + Send + Sync> = Arc::new(move |req: Request| {
            Box::pin(handler(req))
        });
        self.routes.insert(method.wire(), handler);
    }

    /// 分发（异步）
    pub async fn dispatch(&self, req: Request) -> Response {
        match self.routes.get(&req.method) {
            Some(handler) => handler(req).await,
            None => Response {
                id: req.id,
                slot_index: req.slot_index,
                payload: Vec::new(),
                error: Some("method not found".into()),
            },
        }
    }
}
