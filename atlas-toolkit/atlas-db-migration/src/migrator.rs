use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};
use crate::model::atlas_user::AtlasUserMigration;
use crate::utils::{generate_entity, migration_db};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(AtlasUserMigration),
            // Box::new(lumi_room::Migration),
        ]
    }
}


impl Migrator {
   

    pub async fn with_db(db_url: &str) {
        migration_db::<Migrator>(db_url).await;
       
    }
    
    pub async fn with_entity(db_url: &str,out_path: &str) {
        generate_entity(db_url,out_path).await;
    }
}