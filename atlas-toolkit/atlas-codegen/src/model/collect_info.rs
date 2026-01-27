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



// 转 ModuleId 字符串到 u16
pub fn module_id_to_u16(s: &str) -> u16 {
    match s {
        "AtlasModuleId::Auth" => 1,
        "AtlasModuleId::Chat" => 2,
        "AtlasModuleId::Holdem" => 3,
        _ => 0,
    }
}