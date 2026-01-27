use crate::core::{load_tera, rust_type_to_ts};
use crate::model::{TsEnumNumberVariant, TsEnumVariant, TsFieldCtx, TypeRegistry};
use std::fs;
use std::fs::create_dir_all;
use syn::__private::quote;
use syn::{Fields, ItemEnum, ItemStruct};
use tera::Context;

pub fn generate_ts_enum(type_registry: &TypeRegistry, name: &str, e: &ItemEnum) {
    // ========= 1️⃣ 分类判断 =========
    let has_payload = e.variants.iter().any(|v| !matches!(v.fields, Fields::Unit));

    let all_have_discriminant = e.variants.iter().all(|v| v.discriminant.is_some());

    // ===== 2️⃣ 选择模板 & 填充数据 =====
    let tera = load_tera();

    let mut ctx = Context::new();
    ctx.insert("name", name);

    let rendered = if has_payload {
        // ===== Discriminated union =====
        let variants: Vec<TsEnumVariant> = e
            .variants
            .iter()
            .map(|v| {
                let payload = match &v.fields {
                    Fields::Unnamed(fields) => {
                        let ty = &fields.unnamed.first().unwrap().ty;
                        let inner = rust_type_to_ts(ty, type_registry);
                        Some(inner.ts_type)
                    }
                    _ => None,
                };

                TsEnumVariant {
                    name: v.ident.to_string(),
                    payload,
                }
            })
            .collect();

        ctx.insert("variants", &variants);
        tera.render("enum_discriminated.ts.tera", &ctx)
    } else if all_have_discriminant {
        // ===== number enum =====
        let variants: Vec<TsEnumNumberVariant> = e
            .variants
            .iter()
            .map(|v| {
                let (_, expr) = v.discriminant.as_ref().unwrap();
                TsEnumNumberVariant {
                    name: v.ident.to_string(),
                    value: quote::quote!(#expr).to_string(),
                }
            })
            .collect();

        ctx.insert("variants", &variants);
        tera.render("enum_number.ts.tera", &ctx)
    } else {
        // ===== string union =====
        let variants: Vec<String> = e.variants.iter().map(|v| v.ident.to_string()).collect();

        ctx.insert("variants", &variants);
        tera.render("enum_string_union.ts.tera", &ctx)
    }
    .expect("render enum template failed");


    let _ = create_dir_all(type_registry.out_dir.join("type"));
    fs::write(
        type_registry
            .out_dir
            .join("type")
            .join(format!("{}.ts", name)),
        rendered,
    )
    .unwrap();
}

pub fn generate_ts_struct(type_registry: &TypeRegistry,name: &str, s: &ItemStruct) {
    let tera = load_tera();

    let mut ctx = Context::new();
    ctx.insert("name", name);

    let mut fields: Vec<TsFieldCtx> = Vec::new();
    let mut imports: Vec<String> = Vec::new();

    if let Fields::Named(fields_named) = &s.fields {
        for f in &fields_named.named {
            let field_name = f.ident.as_ref().unwrap().to_string();

            let ts_type_info = rust_type_to_ts(&f.ty, type_registry);

            // 收集 imports（去重）
            for imp in ts_type_info.imports {
                if !imports.contains(&imp) {
                    imports.push(imp);
                }
            }

            fields.push(TsFieldCtx {
                name: field_name,
                ts_type: ts_type_info.ts_type,
            });
        }
    }

    ctx.insert("fields", &fields);
    ctx.insert("imports", &imports);

    let rendered = tera
        .render("struct.ts.tera", &ctx)
        .expect("render struct template failed");

    let _ = create_dir_all(type_registry.out_dir.join("type"));
    fs::write(
        type_registry
            .out_dir
            .join("type")
            .join(format!("{}.ts", name)),
        rendered,
    )
        .unwrap();
}
