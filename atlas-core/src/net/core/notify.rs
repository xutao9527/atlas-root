use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::net::core::module_id::AtlasModuleId;
use crate::net::protocol::{AtlasNotifyPublic, AtlasRawFrame};
use crate::net::protocol::frame::AtlasFrame;
use crate::net::protocol::frame_body_notify::{AtlasNotifyInternal, AtlasNotifyTarget};
use crate::net::protocol::frame_header::AtlasFrameHeader;

/// ================== Notify Spec（scheme 用） ==================
pub trait AtlasNotifySpec: 'static {
    const MODULE_ID: AtlasModuleId;
    const NOTIFY_ID: u16;
    const OP_CODE: u32 = ((Self::MODULE_ID as u32) << 16) | (Self::NOTIFY_ID as u32);
}

/// ================== Notify Builder（真正的 glue） ==================
pub trait AtlasNotifyBuildExt: AtlasNotifySpec + Sized {
    fn build_notify(self, targets: Vec<AtlasNotifyTarget>) -> AtlasFrame<AtlasNotifyInternal<Self>> {
        AtlasFrame {
            header: AtlasFrameHeader::build_notify(Self::OP_CODE),
            body: AtlasNotifyInternal {
                targets,
                data: self,
            },
        }
    }
}

/// 自动给所有满足条件的类型实现
impl<T> AtlasNotifyBuildExt for T where T: AtlasNotifySpec {}


pub async fn handle_notify<T>(raw: AtlasRawFrame) -> anyhow::Result<(Vec<AtlasNotifyTarget>, AtlasRawFrame)>
where
    T: Serialize + DeserializeOwned + AtlasNotifySpec + Debug,
{
    // 1️⃣ raw → AtlasFrame<AtlasNotifyInternal<T>>
    let notify_frame =
        AtlasFrame::<AtlasNotifyInternal<T>>::from_raw(raw)
            .map_err(|e| anyhow::anyhow!("decode internal notify failed: {:?}", e))?;
    // 2️⃣ 拆 targets
    let AtlasNotifyInternal { targets, data } = notify_frame.body;
    // 3️⃣ Internal<T> → Public<T>
    let notify_public = AtlasNotifyPublic { data };
    // 4️⃣ 重新组 frame（header 原样复用）
    let public_frame = AtlasFrame {
        header: notify_frame.header,
        body: notify_public,
    };
    let public_raw = public_frame
        .into_raw()
        .map_err(|e| anyhow::anyhow!("encode public notify failed: {:?}", e))?;

    // 6️⃣ 返回
    Ok((targets, public_raw))
}