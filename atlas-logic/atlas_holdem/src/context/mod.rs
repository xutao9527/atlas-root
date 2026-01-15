use crate::context::table_manager::TableManager;
use std::sync::{Arc, OnceLock};

pub mod table_manager;

static TABLE_MANAGER: OnceLock<Arc<TableManager>> = OnceLock::new();

/// 获取全局 TOKEN_MAP
pub fn table_manager() -> Arc<TableManager> {
    TABLE_MANAGER.get_or_init(|| {
        let manager = TableManager::new();
        manager.init_tables(10);
        Arc::new(manager)
    }).clone()
}