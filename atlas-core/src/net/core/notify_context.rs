use std::sync::{Arc, OnceLock};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::net::core::reg_node::AtlasRegNodeId;
use crate::net::protocol::frame::{AtlasFrame, AtlasRawFrame};
use crate::net::protocol::frame_body_notify::AtlasNotifyBody;

static GLOBAL_NOTIFIER: OnceLock<Arc<dyn Notifier>> = OnceLock::new();

pub fn set_global_notifier(n: Arc<dyn Notifier>) {
    let _ = GLOBAL_NOTIFIER.set(n);
}

pub fn global_notifier() -> Option<&'static Arc<dyn Notifier>> {
    GLOBAL_NOTIFIER.get()
}

/// ================== 核心 Notifier（对象安全） ==================
pub trait Notifier: Send + Sync {
    fn notify_raw(
        &self,
        reg_node_id: &AtlasRegNodeId,
        raw_notify_msg: AtlasRawFrame,
    ) -> bool;
}

/// ================== 泛型扩展（你真正用的） ==================
pub trait NotifierExt: Notifier {
    fn notify<T>(
        &self,
        reg_node_id: &AtlasRegNodeId,
        wire_notify_msg: AtlasFrame<AtlasNotifyBody<T>>,
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