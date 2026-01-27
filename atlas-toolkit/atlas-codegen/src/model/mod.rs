mod info_model;
mod tera_model;


pub use info_model::*;
pub use tera_model::*;



// 转 ModuleId 字符串到 u16
pub fn module_id_to_u16(s: &str) -> u16 {
    match s {
        "Auth" => 1,
        "Chat" => 2,
        "Holdem" => 3,
        _ => 0,
    }
}