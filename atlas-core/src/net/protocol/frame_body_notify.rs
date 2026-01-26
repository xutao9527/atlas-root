use serde::{Deserialize, Serialize};

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
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: Deserialize<'de>"
))]
pub struct AtlasNotifyBody<T> {
    /// 通知目标
    pub targets: Vec<AtlasNotifyTarget>,
    /// 通知数据
    pub data: T,
}