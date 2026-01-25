use serde::{Deserialize, Serialize};

/// ================== 注册节点 ==================
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AtlasRegNodeId {
    GateNode(u16),
    AuthNode(u16),
    HoldemNode(u16),
}