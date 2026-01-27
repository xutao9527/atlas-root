use proc_macro2::{Delimiter, TokenTree};
use std::collections::BTreeMap;
use std::path::PathBuf;
use syn::{ItemEnum, ItemMacro, ItemStruct};
use crate::core::module_id_to_u16;

#[derive(Debug)]
pub struct RpcInfo {
    pub module_id: u16,
    pub rpc_id: u16,
    pub rpc_name: String,
    pub request: String,
    pub response: String,
}

#[derive(Debug)]
pub struct NotifyInfo {
    pub module_id: u16,
    pub notify_id: u16,
    pub notify: String,
}


#[derive(Default)]
pub struct TypeRegistry {
    pub structs: BTreeMap<String, ItemStruct>,
    pub enums: BTreeMap<String, ItemEnum>,
    pub macros: Vec<ItemMacro>,
    pub rpc_infos: Vec<RpcInfo>,
    pub notify_infos: Vec<NotifyInfo>,
    pub src_dir: PathBuf,
    pub out_dir: PathBuf,
}


impl TypeRegistry {
    pub fn parse_macro(&mut self)  {
        for m in &self.macros {
            if m.mac.path.is_ident("atlas_rpc_module") {
                let mut rpc_info_vec  = self.parse_atlas_rpc_macro(m);
                self.rpc_infos.append(&mut rpc_info_vec);
            }
            if m.mac.path.is_ident("atlas_notify_specs") {
                let mut notify_info_vec = self.parse_atlas_notify_macro(m);
                self.notify_infos.append(&mut notify_info_vec);
            }
        }
    }

    fn parse_atlas_notify_macro(&self,m: &ItemMacro) -> Vec<NotifyInfo>{
        let mut notify_info_vec = Vec::new();
        for tt in m.mac.tokens.clone() {
            let TokenTree::Group(group) = tt else { continue };
            if group.delimiter() != Delimiter::Brace {
                continue;
            }

            let mut iter = group.stream().into_iter().peekable();
            while let Some(tt) = iter.next() {
                match tt {
                    TokenTree::Ident(notify) => {
                        // println!("notify_name: {}", notify);
                        iter.next(); // =
                        let TokenTree::Group(args) = iter.next().unwrap() else { continue };
                        let mut args_it = args.stream().into_iter();
                        args_it.next();
                        args_it.next();
                        args_it.next();
                        let module_id = match args_it.next() {
                            Some(TokenTree::Ident(id)) => {
                                module_id_to_u16(&id.to_string())
                            }
                            _ => continue,
                        };
                        args_it.next();
                        let notify_id = match args_it.next() {
                            Some(TokenTree::Literal(l)) => l.to_string().parse::<u16>().unwrap(),
                            _ => continue,
                        };
                        notify_info_vec.push(NotifyInfo {
                            module_id,
                            notify_id,
                            notify:notify.to_string(),

                        });
                        // println!("module_id: {:?}", module_id);
                        // println!("notify_id: {:?}", notify_id);
                        // println!("notify: {:?}", notify);
                    }
                    _ => {}
                }
            }
        }
        notify_info_vec
    }

    fn parse_atlas_rpc_macro(&self,m: &ItemMacro) -> Vec<RpcInfo>{
        let mut module_id: u16 = 0;
        let mut rpc_info_vec = Vec::new();
        for tt in m.mac.tokens.clone() {
            let TokenTree::Group(group) = tt else { continue };
            if group.delimiter() != Delimiter::Brace {
                continue;
            }
            let mut iter = group.stream().into_iter().peekable();
            while let Some(tt) = iter.next() {
                match tt {
                    TokenTree::Ident(ident) if ident == "ModuleId" => {
                        let _path1 = iter.next(); // =
                        let _path2 = iter.next(); // AtlasModuleId
                        let _path3 = iter.next(); // :
                        let _path4 = iter.next(); // :
                        let path5 = iter.next(); // Auth
                        // println!("path1: {:?}, path2: {:?}, path3: {:?}, path4: {:?}, path5: {:?}", _path1, _path2, _path3, _path4, path5);
                        if let Some(TokenTree::Ident(id)) = path5 {
                            module_id = module_id_to_u16(&id.to_string());
                        }
                    }
                    TokenTree::Ident(rpc_name) =>{
                        iter.next(); // =
                        let TokenTree::Group(args) = iter.next().unwrap() else { continue };
                        let mut args_it = args.stream().into_iter();
                        let rpc_id = match args_it.next() {
                            Some(TokenTree::Literal(l)) => l.to_string().parse::<u16>().unwrap(),
                            _ => continue,
                        };
                        args_it.next(); // ,

                        let request = match args_it.next() {
                            Some(TokenTree::Ident(i)) => i.to_string(),
                            _ => continue,
                        };
                        args_it.next(); // ,

                        let response = match args_it.next() {
                            Some(TokenTree::Ident(i)) => i.to_string(),
                            _ => continue,
                        };
                        // println!("rpc_name: {}, rpc_id: {}, request: {}, response: {}", rpc_name, rpc_id, request, response);
                        // 吃掉结尾的 ,
                        let _ = iter.next();

                        rpc_info_vec.push(RpcInfo {
                            module_id,
                            rpc_id,
                            rpc_name: rpc_name.to_string(),
                            request,
                            response,
                        });
                    }
                    _ => {}
                }
            }
        }
        rpc_info_vec
    }
}


