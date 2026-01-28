// 转 ModuleId 字符串到 u16
pub fn module_id_to_u16(s: &str) -> u16 {
    match s {
        "Auth" => 1,
        "Holdem" => 2,
        "Chat" => 3,
        _ => 0,
    }
}