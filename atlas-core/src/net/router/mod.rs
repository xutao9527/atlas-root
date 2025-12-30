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


pub trait AsyncHandler: Send + Sync + 'static {
    fn call(&self, req: Request) -> Pin<Box<dyn Future<Output=Response> + Send>>;
}

impl<F, Fut> AsyncHandler for F
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Response> + Send + 'static,
{
    fn call(&self, req: Request) -> Pin<Box<dyn Future<Output=Response> + Send>> {
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
    pub async fn dispatch(&self, req: Request) -> Response {
        match self.routes.get(&req.method) {
            Some(handler) => handler.call(req).await,
            None => Response {
                id: req.id,
                slot_index: req.slot_index,
                payload: Vec::new(),
                error: Some("method not found".into()),
            },
        }
    }
}
