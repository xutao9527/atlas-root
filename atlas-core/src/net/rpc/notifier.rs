use serde::{Deserialize, Serialize};
use crate::net::rpc::packet_message::AtlasRawMessage;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AtlasRegNodeId {
    GateNode(u16),
    AuthNode(u16),
    HoldemNode(u16),
}


pub trait Notifier: Send + Sync {
    fn notify(&self, reg_node_id: &AtlasRegNodeId, msg: AtlasRawMessage) -> bool;
}

