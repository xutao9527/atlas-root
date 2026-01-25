use serde::{Deserialize, Serialize};
use crate::net::core::rpc::AtlasModuleId;
use crate::net::protocol::frame::AtlasFrame;
use crate::net::protocol::frame_header::AtlasFrameHeader;

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

/// ================== 通知载体 ==================
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasNotifyBody<T> {
    /// 通知目标
    pub targets: Vec<AtlasNotifyTarget>,
    /// 通知数据
    pub data: T,
}

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
    ) -> AtlasFrame<AtlasNotifyBody<Self>> {
        AtlasFrame {
            header: AtlasFrameHeader::build_notify(),
            body: AtlasNotifyBody {
                targets,
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
