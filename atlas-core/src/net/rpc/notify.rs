use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::net::rpc::notify_body::AtlasNotify;
use crate::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};

/// ================== 注册节点 ==================
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AtlasRegNodeId {
    GateNode(u16),
    AuthNode(u16),
    HoldemNode(u16),
}

/// ================== 核心 Notifier（对象安全） ==================
pub trait Notifier: Send + Sync {
    fn notify_raw(
        &self,
        reg_node_id: &AtlasRegNodeId,
        raw_notify_msg: AtlasRawMessage,
    ) -> bool;
}

/// ================== 泛型扩展（你真正用的） ==================
pub trait NotifierExt: Notifier {
    fn notify<T>(
        &self,
        reg_node_id: &AtlasRegNodeId,
        wire_notify_msg: AtlasWireMessage<AtlasNotify<T>>,
    ) -> bool
    where
        T: Serialize + DeserializeOwned + Send + 'static,
    {
        let raw = match wire_notify_msg.into_raw() {
            Ok(r) => r,
            Err(_) => return false,
        };

        self.notify_raw(reg_node_id, raw)
    }
}

/// 所有 Notifier 自动获得 notify<T>
impl<T: Notifier + ?Sized> NotifierExt for T {}


