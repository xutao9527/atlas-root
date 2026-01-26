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


/// ================== 内部通知载体（服务器用） ==================
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: Deserialize<'de>"
))]
pub struct AtlasNotifyInternal<T> {
    /// 通知目标
    pub targets: Vec<AtlasNotifyTarget>,
    /// 通知数据
    pub data: T,
}

impl<T> From<AtlasNotifyInternal<T>> for AtlasNotifyPublic<T> {
    fn from(internal: AtlasNotifyInternal<T>) -> Self {
        AtlasNotifyPublic {
            data: internal.data,
        }
    }
}

/// ================== 对外通知载体（前端用） ==================
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: Deserialize<'de>"
))]
pub struct AtlasNotifyPublic<T> {
    /// 通知数据
    pub data: T,
}