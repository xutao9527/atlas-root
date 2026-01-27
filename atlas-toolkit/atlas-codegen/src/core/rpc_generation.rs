use crate::core::rust_type_to_ts;
use crate::entity::RpcInfo;
use std::fs;
use std::io::Write;
use std::path::Path;
use syn::{Fields, File, Item, ItemStruct};

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
    // 解析字段
    let mut fields_vec = Vec::new();
    if let Fields::Named(fields_named) = &s.fields {
        for f in fields_named.named.iter() {
            let name = f.ident.as_ref().unwrap().to_string();
            let ty_ts = rust_type_to_ts(&f.ty);
            fields_vec.push((name, ty_ts));
        }
    }

    // 生成 TS 文件
    let ts_file_path = out_dir.join(format!("{}.ts", struct_name));
    let mut ts_file = fs::File::create(ts_file_path).unwrap();

    let mut code = String::new();
    code.push_str("// @ts-ignore\n");
    code.push_str("import {WirePayload} from \"db://assets/scripts/wire/base/Message\";\n\n");

    // interface
    code.push_str(&format!("export interface {}Props {{\n", struct_name));
    for (name, ty) in &fields_vec {
        code.push_str(&format!("    {}: {};\n", name, ty));
    }
    code.push_str("}\n\n");

    // class
    code.push_str(&format!("export class {} extends WirePayload {{\n", struct_name));
    code.push_str(&format!(
        "    static readonly METHOD = {} << 16 | {};\n\n",
        rpc_info.module_id,
        rpc_info.method_id
    ));

    // 字段声明
    for (name, ty) in &fields_vec {
        code.push_str(&format!("    {}: {};\n", name, ty));
    }
    code.push('\n');

    // constructor
    code.push_str(&format!(
        "    constructor(props: {}Props) {{\n",
        struct_name
    ));
    code.push_str("        super();\n");
    for (name, _) in &fields_vec {
        code.push_str(&format!("        this.{0} = props.{0};\n", name));
    }
    code.push_str("    }\n");
    code.push_str("}\n");

    ts_file.write_all(code.as_bytes()).unwrap();
    println!("Generated TS: {}", struct_name);
}
