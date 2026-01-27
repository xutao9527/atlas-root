use crate::model::TypeRegistry;
use std::fs;
use syn::{File, Item};

pub fn collect_type(rs_file_vec: &[std::path::PathBuf]) -> TypeRegistry {
    let mut registry = TypeRegistry::default();

    for file in rs_file_vec {
        let source_content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let syntax: File = match syn::parse_file(&source_content) {
            Ok(s) => s,
            Err(_) => continue,
        };

        for item in syntax.items {
            match item {
                Item::Enum(e) => {
                    let name = e.ident.to_string();
                    registry.enums.insert(name, e);
                }
                Item::Struct(s) => {
                    let name = s.ident.to_string();
                    registry.structs.insert(name, s);
                }
                Item::Macro(m) => {
                    registry.macros.push(m);
                }
                _ => {}
            }
        }
    }
    registry.parse_macro();
    registry
}
