use atlas_core::net::protocol::{AtlasNotifyTarget, AtlasRawFrame};

pub async fn notify_handler(target: Vec<AtlasNotifyTarget>, notify: AtlasRawFrame) {
    println!("notify_handler target: {:?}", target);
    println!("notify_handler notify: {:?}", notify);
}

