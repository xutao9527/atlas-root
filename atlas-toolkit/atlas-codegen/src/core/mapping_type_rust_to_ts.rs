use syn::{PathArguments, Type, TypePath};
use crate::core::generate_ts_enum;
use crate::model::TypeRegistry;

/// 将 Rust 类型转换为 TS 类型
pub fn rust_type_to_ts(ty: &Type,type_registry: &TypeRegistry) -> (bool, String) {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let seg = path.segments.last().unwrap();
            let ident = seg.ident.to_string();
            // println!("{}", ident.as_str());
            match ident.as_str() {
                // ===== 原始类型 =====
                "String" | "str" => (false, "string".to_string()),
                "u8" | "u16" | "u32" | "u64" |
                "i8" | "i16" | "i32" | "i64" |
                "usize" |
                "f32" | "f64" => (false, "number".to_string()),
                "bool" => (false, "boolean".to_string()),
                // ===== Option<T> =====
                "Option" => {
                    // 递归解析 Option<T>
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let (is_composite,type_value) = rust_type_to_ts(inner_ty,type_registry);
                            return (is_composite, format!("{}[]", type_value));
                        }
                    }
                    (false, "any | null".to_string())
                },
                // ===== Vec<T> =====
                "Vec" => {
                    // 递归解析 Vec<T>
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let (is_composite,type_value) = rust_type_to_ts(inner_ty,type_registry);
                            return (is_composite, format!("{}[]", type_value));
                        }
                    }
                    (false, "any[]".to_string())
                },
                // ===== 👇 关键：自定义复合类型 =====
                _ => {
                    println!("{}", ident);
                    if let Some(e) = type_registry.enums.get(ident.as_str()){
                        generate_ts_enum(type_registry, ident.as_str(), e);
                        (true, ident.to_string())
                    }else if let Some(_e) = type_registry.structs.get(ident.as_str()) {
                        (true, ident.to_string())
                    }else{
                        (false, "any".to_string())

                    }
                }
            }
        }
        _ => (false,"any".to_string()),
    }
}