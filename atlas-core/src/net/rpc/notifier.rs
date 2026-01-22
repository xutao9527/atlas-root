use serde::{Deserialize, Serialize};
use crate::net::rpc::packet_message::AtlasRawMessage;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AtlasRegNodeId {
    AuthNode(u16),
    HoldemNode(u16),
}


pub trait Notifier: Send + Sync {
    fn notify(&self, logical_id: &AtlasRegNodeId, msg: AtlasRawMessage) -> bool;
}

