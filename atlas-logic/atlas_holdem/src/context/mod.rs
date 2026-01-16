use crate::context::table_manager::TableManager;
use std::sync::{Arc, OnceLock};
use sea_orm::{Database, DatabaseConnection};

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





static DB: OnceLock<Arc<DatabaseConnection>> = OnceLock::new();

pub async fn init_db(db_url: &str) {
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    DB.set(Arc::from(conn)).expect("DB already initialized");
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("DB not initialized")
}
