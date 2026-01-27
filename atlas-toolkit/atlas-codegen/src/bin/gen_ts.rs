use atlas_codegen::core::{collect_type, generate_rpc};
use atlas_codegen::utils::{collect_rs_files, get_work_path};
use std::fs::create_dir_all;

fn main() {
    let (src_dir, out_dir) = get_work_path().unwrap();
    create_dir_all(&out_dir).unwrap();

    let mut rs_file_vec = Vec::new();
    collect_rs_files(&src_dir, &mut rs_file_vec);

    let mut type_registry = collect_type(&rs_file_vec);
    type_registry.src_dir = src_dir;
    type_registry.out_dir = out_dir;

    generate_rpc(&type_registry);
    // generate_type_info(&rs_file_vec, &ts_out_dir.join("type"))
}






