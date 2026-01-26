#[macro_export]
macro_rules! atlas_notify_specs {
    (
        $(
            $ty:ty => ($module:expr, $notify_id:expr)
        ),+ $(,)?
    ) => {
        pub mod notify {
            use super::*;
            use atlas_core::net::client::NotifyHandler;
            use atlas_core::net::core::{handle_notify, AtlasNotifySpec};
            use atlas_core::net::protocol::AtlasFrame;

            // ===== 1️⃣ impl AtlasNotifySpec =====
            $(
                impl AtlasNotifySpec for $ty {
                    const MODULE_ID: AtlasModuleId = $module;
                    const NOTIFY_ID: u16 = $notify_id;
                }
            )+


            // ===== 2️⃣ notify_dispatcher（固定生成） =====
            pub async fn notify_dispatcher(
                notify_frame: bytes::Bytes,
                notify_handler: NotifyHandler,
            ) {
                if let Ok(notify_raw_frame) = AtlasFrame::from_bytes(notify_frame.clone()) {
                    match notify_raw_frame.header.op_code {
                        $(
                            <$ty as AtlasNotifySpec>::OP_CODE => {
                                handle_notify::<$ty>(
                                    notify_raw_frame,
                                    notify_handler.clone(),
                                )
                                .await
                            }
                        )+
                        _ => {}
                    }
                }
            }
        }
    };
}
