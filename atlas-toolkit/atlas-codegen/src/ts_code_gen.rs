use std::fs;
use std::io::Write;
use std::path::Path;
use syn::{Fields, File, Item, PathArguments, Type, TypePath};
use crate::utils::RpcInfo;

/// 将 Rust 类型转换为 TS 类型
fn rust_type_to_ts(ty: &Type) -> String {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let seg = path.segments.last().unwrap();
            let ident = seg.ident.to_string();
            match ident.as_str() {
                "String" | "str" => "string".to_string(),
                "u8" | "u16" | "u32" | "u64" |
                "i8" | "i16" | "i32" | "i64" |
                "f32" | "f64" => "number".to_string(),
                "bool" => "boolean".to_string(),
                "Option" => {
                    // 递归解析 Option<T>
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            return format!("{} | null", rust_type_to_ts(inner_ty));
                        }
                    }
                    "any | null".to_string()
                },
                "Vec" => {
                    // 递归解析 Vec<T>
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            return format!("{}[]", rust_type_to_ts(inner_ty));
                        }
                    }
                    "any[]".to_string()
                },
                _ => "any".to_string()
            }
        }
        _ => "any".to_string(),
    }
}

/// 生成 TS 文件
pub fn generate_ts_from_structs(rs_files: &[std::path::PathBuf], all_rpcs: &[RpcInfo], out_dir: &Path) {
    fs::create_dir_all(out_dir).unwrap();

    for file in rs_files {
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
                let rpc_info_opt = all_rpcs.iter().find(|r| r.request == struct_name || r.response == struct_name);
                if rpc_info_opt.is_none() { continue; }
                let rpc_info = rpc_info_opt.unwrap();

                // 解析字段
                let mut fields_vec = Vec::new();
                if let Fields::Named(fields_named) = s.fields {
                    for f in fields_named.named.iter() {
                        let name = f.ident.as_ref().unwrap().to_string();
                        let ty_ts = rust_type_to_ts(&f.ty);

                        // 输出字段日志，方便检查
                        // println!(
                        //     "cargo:warning=Struct {} Field {} -> Rust type: {:?}, TS type: {}",
                        //     struct_name,
                        //     name,
                        //     f.ty,
                        //     ty_ts
                        // );

                        fields_vec.push((name, ty_ts));
                    }
                }

                // 生成 TS 文件
                let ts_file_path = out_dir.join(format!("{}.ts", struct_name));
                let mut ts_file = fs::File::create(ts_file_path).unwrap();

                let mut code = String::new();
                code.push_str("import {WirePayload} from \"db://assets/scripts/wire/base/Message\";\n\n");

                // interface
                code.push_str(&format!("export interface {}Props {{\n", struct_name));
                for (name, ty) in &fields_vec {
                    code.push_str(&format!("    {}: {};\n", name, ty));
                }
                code.push_str("}\n\n");

                // class
                code.push_str(&format!("export class {} extends WirePayload{{\n", struct_name));
                code.push_str(&format!("    static readonly METHOD = {} << 16 | {};\n\n", rpc_info.module_id, rpc_info.method_id));

                // 字段声明
                for (name, ty) in &fields_vec {
                    code.push_str(&format!("    {}: {};\n", name, ty));
                }
                code.push('\n');

                // constructor
                code.push_str(&format!("    constructor(\n        props: {}Props\n    ) {{\n", struct_name));
                code.push_str("        super();\n");
                for (name, _) in &fields_vec {
                    code.push_str(&format!("        this.{0} = props.{0};\n", name));
                }
                code.push_str("    }\n");
                code.push_str("}\n");

                ts_file.write_all(code.as_bytes()).unwrap();
                println!("Generated TS: {}", struct_name);
            }
        }
    }
}