use crate::net::rpc::packet_header::AtlasWireHeader;
use crate::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};
use crate::net::rpc::router::AtlasModuleId;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

/// ================== 注册节点 ==================
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AtlasRegNodeId {
    GateNode(u16),
    AuthNode(u16),
    HoldemNode(u16),
}

/// ================== 通知目标 ==================
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

/// ================== 通知载体 ==================
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasNotify<T> {
    /// 通知目标
    pub targets: Vec<AtlasNotifyTarget>,
    /// 通知类型 ID（由 scheme 定义）
    pub notify_type_id: u32,
    /// 通知数据
    pub data: T,
}

/// ================== 核心 Notifier（对象安全） ==================
pub trait Notifier: Send + Sync {
    fn notify_raw(
        &self,
        reg_node_id: &AtlasRegNodeId,
        msg: AtlasRawMessage,
    ) -> bool;
}

/// ================== 泛型扩展（你真正用的） ==================
pub trait NotifierExt: Notifier {
    fn notify<T>(
        &self,
        reg_node_id: &AtlasRegNodeId,
        msg: AtlasWireMessage<AtlasNotify<T>>,
    ) -> bool
    where
        T: Serialize + DeserializeOwned + Send + 'static,
    {
        let raw = match msg.into_raw() {
            Ok(r) => r,
            Err(_) => return false,
        };

        self.notify_raw(reg_node_id, raw)
    }
}

// ⭐ 所有 Notifier 自动获得 notify<T>
impl<T: Notifier + ?Sized> NotifierExt for T {}


/// ================== Notify Spec（scheme 用） ==================
pub trait AtlasNotifySpec: 'static {
    const MODULE_ID: AtlasModuleId;
    const NOTIFY_TYPE_ID: u16;
    const WIRE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::NOTIFY_TYPE_ID as u32);
}

/// ================== Notify Builder（真正的 glue） ==================
pub trait AtlasNotifyBuildExt: AtlasNotifySpec + Sized {
    fn build_notify(
        self,
        targets: Vec<AtlasNotifyTarget>,
    ) -> AtlasWireMessage<AtlasNotify<Self>> {
        AtlasWireMessage {
            header: AtlasWireHeader::build_notify(),
            payload: AtlasNotify {
                targets,
                notify_type_id: Self::WIRE,
                data: self,
            },
        }
    }
}
/// 自动给所有满足条件的类型实现
impl<T> AtlasNotifyBuildExt for T
where
    T: AtlasNotifySpec,
{}