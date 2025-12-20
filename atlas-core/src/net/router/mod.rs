use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use crate::net::packet::{Request, Response};

pub type Handler = Arc<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;

/// 所有模块（高 16 位）
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Module {
    Auth = 1,
    Chat = 2,
}

/// 所有 RPC Method 的公共行为
pub trait RouterMethod: Copy {
    /// 所属模块
    const MODULE: Module;

    /// 方法号（低 16 位）
    fn id(self) -> u16;

    #[inline(always)]
    fn wire(self) -> u32 {
        ((Self::MODULE as u32) << 16) | self.id() as u32
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum AuthMethod {
    SignIn = 1,
    SignUp = 2,
}

impl RouterMethod for AuthMethod {
    const MODULE: Module = Module::Auth;
    fn id(self) -> u16 {
        self as u16
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum ChatMethod {
    SendMessage = 1,
    GetHistory = 2,
}

impl RouterMethod for ChatMethod {
    const MODULE: Module = Module::Chat;
    fn id(self) -> u16 {
        self as u16
    }
}

#[derive(Default)]
pub struct Router {
    routes: HashMap<u32, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<M: RouterMethod>(&mut self, method: M, handler: Handler) {
        self.routes.insert(method.wire(), handler);
    }

    /// 分发（异步）
    pub async fn dispatch(&self, req: Request) -> Response {
        match self.routes.get(&req.method) {
            Some(handler) => handler(req).await,
            None => Response {
                id: req.id,
                payload: Vec::new(),
                error: Some("method not found".into()),
            },
        }
    }
}
