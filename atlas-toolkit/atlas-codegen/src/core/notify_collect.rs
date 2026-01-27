use std::fs;
use crate::model::NotifyInfo;
use std::path::Path;

pub fn notify_info_collect(path: &Path) -> Vec<NotifyInfo> {
    let mut rpc_vec = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return rpc_vec,
    };

    let mut offset = 0;
    while let Some(start) = content[offset..].find("atlas_notify_specs!") {

    }

    rpc_vec
}