use std::fs;
use std::path::{Path, PathBuf};

/// 获得工作目录 , 返回(源码目录,输出目录)
pub fn get_work_path() ->  Result<(PathBuf, PathBuf), String>  {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "CARGO_MANIFEST_DIR not set".to_string())?;

    let crate_dir = PathBuf::from(crate_dir);

    let workspace_root = crate_dir
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| "failed to get workspace root".to_string())?;

    let src_dir = workspace_root
        .join("atlas-scheme")
        .join("src")
        .join("proto");

    let ts_out_dir = workspace_root
        .join("atlas-toolkit")
        .join("atlas-codegen")
        .join("ts_generated");

    Ok((src_dir, ts_out_dir))
}

// 递归收集 src 下的 .rs 文件
pub fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                collect_rs_files(&path, files);
            } else if path.extension().map(|s| s == "rs").unwrap_or(false) {
                files.push(path);
            }
        }
    }
}