use std::fs;
use std::path::Path;
use crate::entity::{module_id_to_u16, RpcInfo};

// 递归收集 src 下的 .rs 文件
pub fn visit_rs_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_rs_files(&path, files);
            } else if path.extension().map(|s| s == "rs").unwrap_or(false) {
                files.push(path);
            }
        }
    }
}

pub fn collect_rpcs_from_file(path: &Path) -> Vec<RpcInfo> {
    let mut rpcs = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return rpcs,
    };

    let mut offset = 0;

    // 🔁 不断查找 atlas_rpc_module!
    while let Some(start) = content[offset..].find("atlas_rpc_module!") {
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

        // ===== 解析当前 module =====
        let mut module_id_val: u16 = 0;

        for line in body.lines().map(|l| l.trim()) {
            // ModuleId = AtlasModuleId::Auth;
            if line.starts_with("ModuleId") {
                if let Some(eq_idx) = line.find('=') {
                    let val = line[eq_idx + 1..].trim().trim_end_matches(';');
                    module_id_val = module_id_to_u16(val);
                }
                continue;
            }

            // RegisterRpc = (1, RegisterReq, RegisterResp),
            if line.contains('=') && line.contains('(') && line.contains(')') {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    continue;
                }

                let rpc_name = parts[0].trim();
                let tuple = parts[1].trim().trim_end_matches(',');

                if tuple.starts_with('(') && tuple.ends_with(')') {
                    let inner = &tuple[1..tuple.len() - 1];
                    let elems: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

                    if elems.len() == 3 {
                        let method_id: u16 = elems[0].parse().unwrap_or(0);
                        let request = elems[1].to_string();
                        let response = elems[2].to_string();

                        rpcs.push(RpcInfo {
                            module_id: module_id_val,
                            _rpc_name: rpc_name.to_string(),
                            method_id,
                            request,
                            response,
                        });
                    }
                }
            }
        }

        // ⏭️ 移动 offset，继续找下一个宏
        offset = close_brace + 1;
    }

    rpcs
}
