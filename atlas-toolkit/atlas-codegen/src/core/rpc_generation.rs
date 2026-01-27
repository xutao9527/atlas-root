use crate::core::rust_type_to_ts;
use std::fs;
use std::path::Path;
use syn::{Fields, File, Item, ItemStruct};
use tera::{Context, Tera};
use crate::model::{RpcInfo, TsFieldCtx};

/// 生成 TS 文件
pub fn rpc_info_generate(rs_file_vec: &[std::path::PathBuf], rpc_info_vec: &[RpcInfo], out_dir: &Path) {
    fs::create_dir_all(out_dir).unwrap();
    for file in rs_file_vec {
        let src = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let syntax: File = match syn::parse_file(&src) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for item in syntax.items {
            if let Item::Struct(s) = item {
                let struct_name = s.ident.to_string();
                // 只处理 RPC 相关结构体
                let rpc_info_opt = rpc_info_vec.iter().find(|r| r.request == struct_name || r.response == struct_name);
                match rpc_info_opt {
                    None => {}
                    Some(rpc_info) => {
                        generate_rpc_ts_struct(
                            &struct_name,
                            &s,
                            rpc_info,
                            out_dir,
                        );
                    }
                }
            }
        }
    }
}

fn generate_rpc_ts_struct(
    struct_name: &str,
    s: &ItemStruct,
    rpc_info: &RpcInfo,
    out_dir: &Path,
) {
    // ===== 1️⃣ 解析字段 =====
    let mut fields = Vec::new();

    if let Fields::Named(fields_named) = &s.fields {
        for f in &fields_named.named {
            let name = f.ident.as_ref().unwrap().to_string();
            let ts_type = rust_type_to_ts(&f.ty);
            fields.push(TsFieldCtx { name, ts_type });
        }
    }
    let tera = load_tera();

    // ===== 3️⃣ 构建上下文 =====
    let mut ctx = Context::new();
    ctx.insert("name", struct_name);
    ctx.insert("fields", &fields);
    ctx.insert("module_id", &rpc_info.module_id);
    ctx.insert("method_id", &rpc_info.method_id);

    // ===== 4️⃣ 渲染 =====
    let code = tera.render("rpc_struct.ts.tera", &ctx).unwrap();

    // ===== 5️⃣ 写文件 =====
    let ts_file_path = out_dir.join(format!("{}.ts", struct_name));
    fs::write(ts_file_path, code).unwrap();

    println!("Generated TS: {}", struct_name);
}

fn load_tera() -> Tera {
    let glob = format!(
        "{}/templates/**/*",
        env!("CARGO_MANIFEST_DIR")
    );
    Tera::new(&glob).expect("load tera templates failed")
}