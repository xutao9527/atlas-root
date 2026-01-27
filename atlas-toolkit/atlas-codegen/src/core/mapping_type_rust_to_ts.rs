use syn::{PathArguments, Type, TypePath};
use crate::core::{generate_ts_enum, generate_ts_struct};
use crate::model::{TsTypeInfo, TypeRegistry};

/// 将 Rust 类型转换为 TS 类型
pub fn rust_type_to_ts(ty: &Type,type_registry: &TypeRegistry) -> TsTypeInfo {
    match ty {
        Type::Array(arr) => {
            let inner = rust_type_to_ts(&arr.elem, type_registry);
            let ts = if inner.ts_type.contains('|') {
                format!("({})[]", inner.ts_type)
            } else {
                format!("{}[]", inner.ts_type)
            };
            TsTypeInfo {
                ts_type: ts,
                is_composite: inner.is_composite,
                imports: inner.imports,
            }
        }
        Type::Path(TypePath { path, .. }) => {
            let seg = path.segments.last().unwrap();
            let ident = seg.ident.to_string();
            // println!("{}", ident.as_str());
            match ident.as_str() {
                // ===== 原始类型 =====
                "String" | "str" => TsTypeInfo {
                    ts_type: "string".to_string(),
                    is_composite: false,
                    imports: vec![],
                },
                "u8" | "u16" | "u32" | "u64" |
                "i8" | "i16" | "i32" | "i64" |
                "usize" |
                "f32" | "f64" => TsTypeInfo {
                    ts_type: "number".to_string(),
                    is_composite: false,
                    imports: vec![],
                },
                "bool" => TsTypeInfo {
                    ts_type: "boolean".to_string(),
                    is_composite: false,
                    imports: vec![],
                },
                // ===== Option<T> =====
                "Option" => {
                    // 递归解析 Option<T>
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner  = rust_type_to_ts(inner_ty,type_registry);
                            return TsTypeInfo {
                                ts_type: format!("{} | null", inner.ts_type),
                                is_composite: inner.is_composite,
                                imports: inner.imports,
                            };
                        }
                    }
                    TsTypeInfo {
                        ts_type: "any | null".into(),
                        is_composite: false,
                        imports: vec![],
                    }
                },
                // ===== Vec<T> =====
                "Vec" => {
                    // 递归解析 Vec<T>
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner  = rust_type_to_ts(inner_ty,type_registry);

                            let ts = if inner.ts_type.contains('|') {
                                format!("({})[]", inner.ts_type)
                            } else {
                                format!("{}[]", inner.ts_type)
                            };

                            return TsTypeInfo {
                                ts_type: ts,
                                is_composite: inner.is_composite,
                                imports: inner.imports,
                            };
                        }
                    }
                    TsTypeInfo {
                        ts_type: "any[]".into(),
                        is_composite: false,
                        imports: vec![],
                    }
                },
                // ===== 👇 关键：自定义复合类型 =====
                _ => {
                    // println!("{}", ident);
                    if let Some(e) = type_registry.enums.get(ident.as_str()){
                        generate_ts_enum(type_registry, ident.as_str(), e);
                        TsTypeInfo {
                            ts_type: ident.clone(),
                            is_composite: true,
                            imports: vec![ident],
                        }
                    }else if let Some(s) = type_registry.structs.get(ident.as_str()) {
                        generate_ts_struct(type_registry, ident.as_str(), s);
                        TsTypeInfo {
                            ts_type: ident.clone(),
                            is_composite: true,
                            imports: vec![ident],
                        }
                    }else{
                        TsTypeInfo {
                            ts_type: "any".into(),
                            is_composite: false,
                            imports: vec![],
                        }
                    }
                }
            }
        }
        _ => TsTypeInfo {
            ts_type: "any".into(),
            is_composite: false,
            imports: vec![],
        },
    }
}