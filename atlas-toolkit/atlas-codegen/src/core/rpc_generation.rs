use crate::core::rust_type_to_ts;
use crate::entity::RpcInfo;
use std::fs;
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

const RPC_STRUCT_TS_TEMPLATE: &str = r#"
// @ts-ignore
import { WirePayload } from "db://assets/scripts/wire/base/Message";

export interface {{struct_name}}Props {
{{fields_interface}}
}

export class {{struct_name}} extends WirePayload {
    static readonly METHOD = {{module_id}} << 16 | {{method_id}};

{{fields_decl}}
    constructor(props: {{struct_name}}Props) {
        super();
{{fields_assign}}
    }
}
"#;

fn generate_rpc_ts_struct(
    struct_name: &str,
    s: &ItemStruct,
    rpc_info: &RpcInfo,
    out_dir: &Path,
) {

    // ===== 1. 解析字段 =====
    let mut fields_interface = String::new();
    let mut fields_decl = String::new();
    let mut fields_assign = String::new();

    // 解析字段
    if let Fields::Named(fields_named) = &s.fields {
        for f in fields_named.named.iter() {
            let name = f.ident.as_ref().unwrap().to_string();
            let ty_ts = rust_type_to_ts(&f.ty);
            fields_interface.push_str(&format!("    {}: {};\n", name, ty_ts));
            fields_decl.push_str(&format!("    {}: {};\n", name, ty_ts));
            fields_assign.push_str(&format!("        this.{0} = props.{0};\n", name));
        }
    }

    // ===== 2. 渲染模板 =====
    let mut code = RPC_STRUCT_TS_TEMPLATE.to_string();
    code = code.replace("{{struct_name}}", struct_name);
    code = code.replace("{{module_id}}", &rpc_info.module_id.to_string());
    code = code.replace("{{method_id}}", &rpc_info.method_id.to_string());
    code = code.replace("{{fields_interface}}", &fields_interface);
    code = code.replace("{{fields_decl}}", &fields_decl);
    code = code.replace("{{fields_assign}}", &fields_assign);

    // ===== 3. 写文件 =====
    let ts_file_path = out_dir.join(format!("{}.ts", struct_name));
    fs::write(&ts_file_path, code).unwrap();



    println!("Generated TS: {}", struct_name);
}
