

use bytes::Bytes;
use atlas_core::net::protocol::frame::AtlasFrame;
use atlas_core::net::protocol::frame_body_notify::AtlasNotifyBody;
use atlas_scheme::proto::auth::notify::user_update_notify::UserUpdateNotify;

pub async fn notify_handler(notify_msg: Bytes) {

    if let Ok(notify_raw_message) = AtlasFrame::from_bytes(notify_msg.clone()) {
        match AtlasFrame::<AtlasNotifyBody<UserUpdateNotify>>::from_raw(notify_raw_message) {
            Ok(notify_wire_message) => {
                println!("notify_handler: \n{:?}", notify_wire_message);
            }
            Err(e) => {
                println!("notify_handler err: \n{:?}", e);
            }
        }
    }


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