use crate::core::{load_tera, rust_type_to_ts, type_collector};
use std::fs;
use std::path::Path;
use syn::{Fields, Item, ItemEnum, ItemStruct};
use syn::__private::quote;
use tera::Context;
use crate::model::{TsEnumNumberVariant, TsEnumVariant};

pub fn generate_type_info(rs_file_vec: &[std::path::PathBuf], out_dir: &Path) {
    let guard = type_collector().lock().unwrap();

    for rs_file in rs_file_vec {
        let src = match fs::read_to_string(rs_file) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let syntax = match syn::parse_file(&src) {
            Ok(f) => f,
            Err(_) => continue,
        };

        for item in syntax.items {
            match item {
                Item::Struct(item_struct) => {
                    let name = item_struct.ident.to_string();
                    if guard.composites.contains(&name) {
                        generate_ts_struct(&name, &item_struct, out_dir);
                        println!("generate struct type: {}", name);
                    }
                }
                Item::Enum(item_enum) => {
                    let name = item_enum.ident.to_string();
                    if guard.composites.contains(&name) {
                        generate_ts_enum(&name, &item_enum, out_dir);
                        println!("generate enum type: {}", name);
                    }
                }
                _ => {}
            }
        }
    }

    for ty in guard.composites.iter() {
        println!("generate composite type: {}", ty);
    }
}

fn generate_ts_struct(_name: &str, _s: &ItemStruct, _out_dir: &Path) {
    // println!("  -> emit TS struct {}", name);

}

fn generate_ts_enum(name: &str, e: &ItemEnum, out_dir: &Path) {
    // println!("  -> emit TS enum {}", name);

    // ========= 1️⃣ 分类判断 =========
    let has_payload = e.variants.iter().any(|v| {
        !matches!(v.fields, Fields::Unit)
    });

    let all_have_discriminant =
        e.variants.iter().all(|v| v.discriminant.is_some());


    // ===== 2️⃣ 选择模板 & 填充数据 =====
    let tera = load_tera();

    let mut ctx = Context::new();
    ctx.insert("name", name);


    let rendered = if has_payload {
        // ===== Discriminated union =====
        let variants: Vec<TsEnumVariant> = e.variants.iter().map(|v| {
            let payload = match &v.fields {
                Fields::Unnamed(fields) => {
                    let ty = &fields.unnamed.first().unwrap().ty;
                    Some(rust_type_to_ts(ty))
                }
                _ => None,
            };

            TsEnumVariant {
                name: v.ident.to_string(),
                payload,
            }
        }).collect();

        ctx.insert("variants", &variants);
        tera.render("enum_discriminated.ts.tera", &ctx)

    } else if all_have_discriminant {
        // ===== number enum =====
        let variants: Vec<TsEnumNumberVariant> = e.variants.iter().map(|v| {
            let (_, expr) = v.discriminant.as_ref().unwrap();
            TsEnumNumberVariant {
                name: v.ident.to_string(),
                value: quote::quote!(#expr).to_string(),
            }
        }).collect();

        ctx.insert("variants", &variants);
        tera.render("enum_number.ts.tera", &ctx)

    } else {
        // ===== string union =====
        let variants: Vec<String> = e.variants
            .iter()
            .map(|v| v.ident.to_string())
            .collect();

        ctx.insert("variants", &variants);
        tera.render("enum_string_union.ts.tera", &ctx)
    }.expect("render enum template failed");

    fs::create_dir_all(out_dir).unwrap();
    fs::write(out_dir.join(format!("{}.ts", name)), rendered).unwrap();
}
