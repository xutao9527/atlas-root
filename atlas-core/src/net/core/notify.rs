use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::net::core::module_id::AtlasModuleId;
use crate::net::protocol::frame::AtlasFrame;
use crate::net::protocol::frame_body_notify::{AtlasNotifyInternal, AtlasNotifyTarget};
use crate::net::protocol::frame_header::AtlasFrameHeader;

/// ================== Notify Spec（scheme 用） ==================
pub trait AtlasNotifySpec: 'static {
    const MODULE_ID: AtlasModuleId;
    const NOTIFY_TYPE_ID: u16;
    const WIRE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::NOTIFY_TYPE_ID as u32);
}

/// ================== Notify Builder（真正的 glue） ==================
pub trait AtlasNotifyBuildExt: AtlasNotifySpec + Sized {
    fn build_notify(self, targets: Vec<AtlasNotifyTarget>) -> AtlasFrame<AtlasNotifyInternal<Self>> {
        AtlasFrame {
            header: AtlasFrameHeader::build_notify(),
            body: AtlasNotifyInternal {
                targets,
                data: self,
            },
        }
    }
}

/// 自动给所有满足条件的类型实现
impl<T> AtlasNotifyBuildExt for T where T: AtlasNotifySpec {}


pub async fn handle_notify<T>(notify_frame: Bytes)
where
    T: Serialize + DeserializeOwned + AtlasNotifySpec,
{

}