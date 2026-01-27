use syn::{PathArguments, Type, TypePath};

/// 将 Rust 类型转换为 TS 类型
pub fn rust_type_to_ts(ty: &Type) -> String {
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