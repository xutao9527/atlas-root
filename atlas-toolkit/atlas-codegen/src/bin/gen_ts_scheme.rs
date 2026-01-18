use std::fs::create_dir_all;
use std::path::PathBuf;
use atlas_codegen::ts_code_gen::generate_ts_from_structs;
use atlas_codegen::utils::{collect_rpcs_from_file, visit_rs_files};

fn main() {
    let (src_dir, ts_out_dir) =  get_ts_scheme_path().unwrap();
    create_dir_all(&ts_out_dir).unwrap();
    println!("TS output dir: {}", ts_out_dir.display());

    let mut rs_files = Vec::new();
    visit_rs_files(&src_dir, &mut rs_files);

    println!("Found Rust files:");
    for file in &rs_files {
        println!("{}", file.display());
    }

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

    generate_ts_from_structs(&rs_files, &all_rpcs, &ts_out_dir);
}

fn get_ts_scheme_path() ->  Result<(PathBuf, PathBuf), String>  {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "CARGO_MANIFEST_DIR not set".to_string())?;

    let crate_dir = PathBuf::from(crate_dir);

    let workspace_root = crate_dir
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| "failed to get workspace root".to_string())?;

    let ts_out_dir = workspace_root
        .join("atlas-toolkit")
        .join("atlas-codegen")
        .join("ts_generated");

    let src_dir = workspace_root
        .join("atlas-scheme")
        .join("src");

    Ok((src_dir, ts_out_dir))
}