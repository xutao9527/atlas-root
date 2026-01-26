use crate::context::session_map;
use atlas_core::net::protocol::{AtlasNotifyTarget, AtlasRawFrame};

pub async fn notify_handler(
    targets: Vec<AtlasNotifyTarget>,
    notify: AtlasRawFrame,
) {
    // println!("notify_handler target: {:?}", targets);
    // println!("notify_handler notify: {:?}", notify);
    let session_map = session_map(); // HashMap<String, Arc<WsSession>>
    let bytes = notify.into_bytes();
    for target in targets {
        match target {
            // ===== 1️⃣ 全体广播 =====
            AtlasNotifyTarget::Broadcast => {
                for session in session_map.iter() {
                    let read_guard = session.read().await;
                    read_guard.send_binary(bytes.clone()).await;
                }
            }
            // ===== 2️⃣ 单个用户 =====
            AtlasNotifyTarget::User { uid } => {
                if let Some(session) = session_map.get(&uid) {
                    let read_guard = session.read().await;
                    read_guard.send_binary(bytes.clone()).await;
                }
            }
            // ===== 3️⃣ 多个用户 =====
            AtlasNotifyTarget::Users { uids } => {
                for uid in uids {
                    if let Some(session) = session_map.get(&uid) {
                        let read_guard = session.read().await;
                        read_guard.send_binary(bytes.clone()).await;
                    }
                }
            }
        }
    }
}


