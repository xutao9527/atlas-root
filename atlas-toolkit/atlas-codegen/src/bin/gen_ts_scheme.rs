use std::fs::create_dir_all;
use atlas_codegen::core::{rpc_info_collect, rpc_info_generate};
use atlas_codegen::utils::{collect_rs_files, get_work_path};

fn main() {
    let (src_dir, ts_out_dir) = get_work_path().unwrap();
    create_dir_all(&ts_out_dir).unwrap();
    println!("TS output dir: {}", ts_out_dir.display());

    let mut rs_files = Vec::new();
    collect_rs_files(&src_dir, &mut rs_files);

    println!("Found Rust files:");
    for file in &rs_files {
        println!("{}", file.display());
    }

    let mut rpc_info_vec = Vec::new();
    for file in &rs_files {
        rpc_info_vec.extend(rpc_info_collect(file));
    }

    println!("warning=Collected RPCs:");
    for rpc in &rpc_info_vec {
        println!(
            ":warning=module_id: {}, method_id: {}, rpc_name: {}, request: {}, response: {}",
            rpc.module_id, rpc.method_id, rpc._rpc_name, rpc.request, rpc.response
        );
    }

    rpc_info_generate(&rs_files, &rpc_info_vec, &ts_out_dir);
}
