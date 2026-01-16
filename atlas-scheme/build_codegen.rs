use std::fs;
use std::io::Write;
use std::path::Path;
use syn::{Fields, File, Item, PathArguments, Type, TypePath};

#[derive(Debug)]
pub struct RpcInfo {
    pub module_id: u16,
    pub method_id: u16,
    pub _rpc_name: String,
    pub request: String,
    pub response: String,
}

// 转 ModuleId 字符串到 u16
pub fn module_id_to_u16(s: &str) -> u16 {
    match s {
        "AtlasModuleId::Auth" => 1,
        "AtlasModuleId::Chat" => 2,
        "AtlasModuleId::Holdem" => 3,
        _ => 0,
    }
}

// 递归收集 src 下的 .rs 文件
pub fn visit_rs_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_rs_files(&path, files);
            } else if path.extension().map(|s| s == "rs").unwrap_or(false) {
                files.push(path);
            }
        }
    }
}

pub fn collect_rpcs_from_file(path: &Path) -> Vec<RpcInfo> {
    let mut rpcs = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return rpcs,
    };

    let mut offset = 0;

    // 🔁 不断查找 atlas_rpc_module!
    while let Some(start) = content[offset..].find("atlas_rpc_module!") {
        let start = offset + start;

        // 找 '{'
        let open_brace = match content[start..].find('{') {
            Some(v) => start + v,
            None => break,
        };

        // ⚠️ 这里简单版只找第一个 '}'（你的宏结构目前是安全的）
        let close_brace = match content[open_brace..].find('}') {
            Some(v) => open_brace + v,
            None => break,
        };

        let body = &content[open_brace + 1..close_brace];

        // ===== 解析当前 module =====
        let mut module_id_val: u16 = 0;

        for line in body.lines().map(|l| l.trim()) {
            // ModuleId = AtlasModuleId::Auth;
            if line.starts_with("ModuleId") {
                if let Some(eq_idx) = line.find('=') {
                    let val = line[eq_idx + 1..].trim().trim_end_matches(';');
                    module_id_val = module_id_to_u16(val);
                }
                continue;
            }

            // RegisterRpc = (1, RegisterReq, RegisterResp),
            if line.contains('=') && line.contains('(') && line.contains(')') {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    continue;
                }

                let rpc_name = parts[0].trim();
                let tuple = parts[1].trim().trim_end_matches(',');

                if tuple.starts_with('(') && tuple.ends_with(')') {
                    let inner = &tuple[1..tuple.len() - 1];
                    let elems: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

                    if elems.len() == 3 {
                        let method_id: u16 = elems[0].parse().unwrap_or(0);
                        let request = elems[1].to_string();
                        let response = elems[2].to_string();

                        rpcs.push(RpcInfo {
                            module_id: module_id_val,
                            _rpc_name: rpc_name.to_string(),
                            method_id,
                            request,
                            response,
                        });
                    }
                }
            }
        }

        // ⏭️ 移动 offset，继续找下一个宏
        offset = close_brace + 1;
    }

    rpcs
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
                //println!("cargo:warning=Generated TS: {}", struct_name);
            }
        }
    }
}

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

