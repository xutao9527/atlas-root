use std::sync::{Arc, OnceLock};
use dashmap::DashMap;
use tokio::sync::RwLock;
use crate::ws::ws_session::WsSession;

pub type SessionRef = Arc<RwLock<WsSession>>;

static SESSION_MAP: OnceLock<DashMap<String, SessionRef>> = OnceLock::new();

pub fn session_map() -> &'static DashMap<String, SessionRef> {
    SESSION_MAP.get_or_init(|| DashMap::new())
}