use atlas_core::net::core::{handle_notify, AtlasNotifySpec};
use atlas_core::net::protocol::frame::AtlasFrame;
use bytes::Bytes;
use atlas_scheme::proto::auth::notify::user_update_notify::UserUpdateNotify;


pub async fn dispatch_notify(notify_msg: Bytes) {
    if let Ok(notify_raw_frame) = AtlasFrame::from_bytes(notify_msg.clone()) {
        match notify_raw_frame.header.op_code {
            UserUpdateNotify::OP_CODE => {
                handle_notify::<UserUpdateNotify>(notify_raw_frame).await;
            }
            _ => {

            }
        }
    }

    // if let Ok(notify_raw_message) = AtlasFrame::from_bytes(notify_msg.clone()) {
    //     match AtlasFrame::<AtlasNotifyInternal<UserUpdateNotify>>::from_raw(notify_raw_message) {
    //         Ok(notify_internal_frame) => {
    //             println!("notify_handler: \n{:?}", notify_internal_frame);
    //             let notify_internal = notify_internal_frame.body;
    //             let _targets = notify_internal.targets.clone();
    //             let _notify_public: AtlasNotifyPublic<UserUpdateNotify> = notify_internal.into();
    //         }
    //         Err(e) => {
    //             println!("notify_handler err: \n{:?}", e);
    //         }
    //     }
    // }


    // for entry in session_map().iter() {
    //     let session = entry.value().clone();
    //     let msg = notify_msg.clone(); // Bytes 必须 clone
    //
    //     let ws = session.read().await;
    //     if let Err(e) = ws.msg_tx.send(Message::Binary(msg)).await {
    //         debug!("send notify failed: {:?}", e);
    //     }
    // }
}