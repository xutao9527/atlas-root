use atlas_core::net::core::{AtlasNotifySpec, handle_notify};
use atlas_core::net::protocol::frame::AtlasFrame;
use atlas_core::net::protocol::{AtlasNotifyTarget, AtlasRawFrame};
use atlas_scheme::proto::auth::notify::user_update_notify::UserUpdateNotify;
use bytes::Bytes;
use atlas_core::net::client::NotifyHandler;

pub async fn notify_handler(target: Vec<AtlasNotifyTarget>, notify: AtlasRawFrame) {
    println!("notify_handler target: {:?}", target);
    println!("notify_handler notify: {:?}", notify);
}

pub async fn notify_dispatcher(notify_frame: Bytes, notify_handler: NotifyHandler) {
    if let Ok(notify_raw_frame) = AtlasFrame::from_bytes(notify_frame.clone()) {
        match notify_raw_frame.header.op_code {
            UserUpdateNotify::OP_CODE => {
                if let Ok((targets,public_notify_frame)) = handle_notify::<UserUpdateNotify>(notify_raw_frame).await {
                    notify_handler(targets,public_notify_frame).await;
                }
            }
            _ => {}
        }
    }
}
