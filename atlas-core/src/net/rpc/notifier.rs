use crate::net::rpc::packet_message::AtlasWireMessage;
use bytes::Bytes;
use serde::{Deserialize, Serialize};


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AtlasRegNodeId {
    GateNode(u16),
    AuthNode(u16),
    HoldemNode(u16),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtlasNotifyTarget {
    /// 全体广播
    Broadcast,
    /// 指定一个用户
    User {
        uid: String,
    },
    /// 指定多个用户（性能友好）
    Users {
        uids: Vec<String>,
    },
}

pub type AtlasNotifyRaw = AtlasNotify<Bytes>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasNotify<T> {
    /// 通知目标
    pub targets: Vec<AtlasNotifyTarget>,
    /// 通知类型 ID（由 scheme 定义）
    pub notify_type_id: u32,
    /// 通知数据
    pub data: T,
}

pub trait Notifier: Send + Sync {
    fn notify(&self, reg_node_id: &AtlasRegNodeId, msg: AtlasWireMessage<AtlasNotifyRaw>) -> bool;
}

