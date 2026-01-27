use crate::core::{load_tera, rust_type_to_ts};
use crate::model::{RpcInfo, TsFieldCtx, TypeRegistry};
use std::fs;
use std::fs::create_dir_all;
use syn::{Fields, ItemStruct};
use tera::Context;


pub fn generate_rpc(type_registry: &TypeRegistry) {
    for rpc_info in type_registry.rpc_infos.iter() {
        if let Some(request_struct) = type_registry.structs.get(&rpc_info.request) {
            generate_rpc_struct(type_registry, &rpc_info.request, request_struct, rpc_info);
        }
        if let Some(response_struct) = type_registry.structs.get(&rpc_info.response) {
            generate_rpc_struct(type_registry, &rpc_info.response, response_struct, rpc_info);
        }
    }
}

fn generate_rpc_struct(
    type_registry: &TypeRegistry,
    struct_name: &str,
    s: &ItemStruct,
    rpc_info: &RpcInfo,
) {
    // ===== 3️⃣ 构建上下文 =====
    let mut ctx = Context::new();
    ctx.insert("name", struct_name);
    ctx.insert("module_id", &rpc_info.module_id);
    ctx.insert("rpc_id", &rpc_info.rpc_id);

    // ===== 1️⃣ 解析字段 =====
    let mut fields = Vec::new();
    let mut imports: Vec<String> = Vec::new();

    if let Fields::Named(fields_named) = &s.fields {
        for f in &fields_named.named {
            let name = f.ident.as_ref().unwrap().to_string();
            let ts_type_info = rust_type_to_ts(&f.ty, type_registry);

            // 收集 imports
            for imp in &ts_type_info.imports {
                if !imports.contains(imp) {
                    imports.push(imp.clone());
                }
            }

            fields.push(TsFieldCtx { name, ts_type:ts_type_info.ts_type });
        }
    }
    let tera = load_tera();
    ctx.insert("fields", &fields);
    ctx.insert("imports", &imports);

    // ===== 4️⃣ 渲染 =====
    let code = tera.render("proto.ts.tera", &ctx).unwrap();

    // ===== 5️⃣ 写文件 =====
    let _ = create_dir_all(type_registry.out_dir.join("rpc"));
    let ts_file_path = type_registry.out_dir.join("rpc").join(format!("{}.ts", struct_name));
    fs::write(ts_file_path, code).unwrap();

    // println!("Generated TS: {}", struct_name);
}