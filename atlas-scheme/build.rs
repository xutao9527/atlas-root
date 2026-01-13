mod build_codegen;

use std::fs::create_dir_all;
use crate::build_codegen::{collect_rpcs_from_file, generate_ts_from_structs, visit_rs_files};
use std::path::{Path, PathBuf};

fn main() {
    // 只有 src 或 build.rs 改动才 rerun
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");

    let src_dir = Path::new("src");

    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = crate_dir.parent().unwrap(); // atlas-root
    let profile = std::env::var("PROFILE").unwrap();
    let ts_out_dir = workspace_root.join("target").join(&profile).join("ts_generated");
    create_dir_all(&ts_out_dir).unwrap();
    //println!("cargo:warning=TS output dir: {}", ts_out_dir.display());

    let mut rs_files = Vec::new();
    visit_rs_files(src_dir, &mut rs_files);

    // println!("cargo:warning=Found Rust files:");
    // for file in &rs_files {
    //     println!("cargo:warning={}", file.display());
    // }

    // 扫描所有文件收集 RPC 信息
    let mut all_rpcs = Vec::new();
    for file in &rs_files {
        let rpcs = collect_rpcs_from_file(file);
        all_rpcs.extend(rpcs);
    }

    println!("cargo:warning=Collected RPCs:");
    for rpc in &all_rpcs {
        println!(
            "cargo:warning=module_id: {}, method_id: {}, rpc_name: {}, request: {}, response: {}",
            rpc.module_id, rpc.method_id, rpc._rpc_name, rpc.request, rpc.response
        );
    }

    // 正式生成 TS
    generate_ts_from_structs(&rs_files, &all_rpcs, &ts_out_dir);
}



