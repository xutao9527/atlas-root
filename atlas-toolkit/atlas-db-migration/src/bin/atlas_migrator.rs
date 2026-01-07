use atlas_db_migration::migrator::Migrator;

pub const DATABASE_URL: &str  = "mysql://root:root@localhost:3306/atlas";

#[tokio::main]
async fn main() {
    Migrator::with_db(DATABASE_URL).await;
    Migrator::with_entity(DATABASE_URL,"D:/codebase/rProjects/atlas-root/atlas-scheme/src/model").await;
}