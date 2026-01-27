#[macro_export]
macro_rules! atlas_notify_specs {
    (
        module $mod_name:ident {
            $(
                $ty:ident = ($module:expr, $notify_id:expr)
            ),+ $(,)?
        }
    ) => {
        pub mod $mod_name {
            use super::*;
            use atlas_core::net::client::NotifyHandler;
            use atlas_core::net::core::{AtlasModuleId, AtlasNotifySpec, handle_notify};
            use atlas_core::net::protocol::AtlasFrame;
            // ===== 1️⃣ impl AtlasNotifySpec =====
            $(
                impl AtlasNotifySpec for $ty {
                    const MODULE_ID: AtlasModuleId = $module;
                    const NOTIFY_ID: u16 = $notify_id;
                }
            )+
            // ===== 2️⃣ 编译期校验: OP_CODE 不能重复 =====
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            enum AtlasNotifyOpCodeCheck {
                $(
                    $ty = <$ty as AtlasNotifySpec>::OP_CODE,
                )+
            }
            // ===== 2️⃣ notify_dispatcher（固定生成） =====
            pub async fn notify_dispatcher(
                notify_frame: bytes::Bytes,
                notify_handler: NotifyHandler,
            ) {
                if let Ok(notify_raw_frame) = AtlasFrame::from_bytes(notify_frame.clone()) {
                    match notify_raw_frame.header.op_code {
                        $(
                            <$ty as AtlasNotifySpec>::OP_CODE => handle_notify::<$ty>(notify_raw_frame,notify_handler.clone()).await,
                        )+
                        _ => {}
                    }
                }
            }
        }
    };
}
