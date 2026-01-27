use std::fs::create_dir_all;

use atlas_codegen::utils::{collect_rs_files, get_work_path};

fn main() {
    let (src_dir, ts_out_dir) = get_work_path().unwrap();
    create_dir_all(&ts_out_dir).unwrap();
    println!("TS output dir: {}", ts_out_dir.display());

    let mut rs_file_vec = Vec::new();
    collect_rs_files(&src_dir, &mut rs_file_vec);



    // generate_rpc_info(&rs_file_vec, &rpc_info_vec, &ts_out_dir.join("rpc"));
    // generate_type_info(&rs_file_vec, &ts_out_dir.join("type"))
}
