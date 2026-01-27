use atlas_codegen::core::{collect_type, generate_notify, generate_rpc};
use atlas_codegen::utils::{collect_rs_files, get_work_path};
use std::fs::create_dir_all;

fn main() {
    let (src_dir, out_dir) = get_work_path().unwrap();
    create_dir_all(&out_dir).unwrap();
    // 收集源文件
    let mut rs_file_vec = Vec::new();
    collect_rs_files(&src_dir, &mut rs_file_vec);
    // 收集数据类型
    let mut type_registry = collect_type(&rs_file_vec);
    // 设置目录
    type_registry.src_dir = src_dir;
    type_registry.out_dir = out_dir;
    // 生成代码
    generate_rpc(&type_registry);
    generate_notify(&type_registry)
}






