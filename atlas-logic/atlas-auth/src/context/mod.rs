pub mod token_manager;

use sea_orm::{Database, DatabaseConnection};
use std::sync::{Arc, OnceLock};

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
