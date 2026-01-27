use crate::model::{module_id_to_u16, NotifyInfo};
use std::fs;
use std::path::Path;

pub fn notify_info_collect(path: &Path) -> Vec<NotifyInfo> {
    let mut notify_vec = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return notify_vec,
    };

    let mut offset = 0;
    while let Some(start) = content[offset..].find("atlas_notify_specs!") {
        let start = offset + start;

        // 找 '{'
        let open_brace = match content[start..].find('{') {
            Some(v) => start + v,
            None => break,
        };

        // ⚠️ 这里简单版只找第一个 '}'（你的宏结构目前是安全的）
        let close_brace = match content[open_brace..].find('}') {
            Some(v) => open_brace + v,
            None => break,
        };

        let body = &content[open_brace + 1..close_brace];
        // println!("{}", body);

        for line in body.lines().map(|l| l.trim()) {
            if line.contains('=') && line.contains('(') && line.contains(')') {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    continue;
                }

                let notify = parts[0].trim();
                let tuple = parts[1].trim().trim_end_matches(',');
                if tuple.starts_with('(') && tuple.ends_with(')') {
                    let inner = &tuple[1..tuple.len() - 1];
                    let elems: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
                    if elems.len() == 2 {
                        let module_id = elems[0].to_string();
                        let module_id_val = module_id_to_u16(&module_id);
                        let notify_id = elems[1].parse().unwrap_or(0);
                        notify_vec.push(NotifyInfo {
                            module_id: module_id_val,
                            notify_id,
                            notify: notify.to_string(),
                        });
                    }
                }
            }
        }

        // ⏭️ 移动 offset，继续找下一个宏
        offset = close_brace + 1;
    }
    notify_vec
}