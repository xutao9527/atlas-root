use crate::net::rpc::packet_message::{AtlasRawMessage, AtlasWireMessage};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::net::rpc::packet_header::AtlasWireHeader;
use crate::net::rpc::router::AtlasModuleId;

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

pub trait Notifier: Send + Sync {
    fn notify(&self, reg_node_id: &AtlasRegNodeId, msg: AtlasRawMessage) -> bool;
}


/// ================== Notify Spec（scheme 用） ==================
pub trait AtlasNotifySpec: 'static {
    const MODULE_ID: AtlasModuleId;
    const NOTIFY_TYPE_ID: u16;
    const WIRE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::NOTIFY_TYPE_ID as u32);
}

/// ================== Notify Builder（真正的 glue） ==================
pub trait AtlasNotifySpecExt: AtlasNotifySpec + Serialize + DeserializeOwned + Send + 'static
{
    fn build_notify(
        self,
        targets: Vec<AtlasNotifyTarget>,
    ) -> Result<AtlasRawMessage, String> {
        let notify = AtlasNotify {
            targets,
            notify_type_id: Self::WIRE,
            data: self,
        };

        AtlasWireMessage {
            header: AtlasWireHeader::build_notify(),
            payload: notify,
        }
            .into_raw()
            .map_err(|_| "build notify wire failed".to_string())
    }
}

/// 自动给所有满足条件的类型实现
impl<T> AtlasNotifySpecExt for T
where
    T: AtlasNotifySpec + Serialize + DeserializeOwned + Send + 'static,
{
}