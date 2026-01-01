use clap::Parser;
use sea_orm_migration::MigratorTrait;
use sea_orm_migration::sea_orm::{Database, DatabaseConnection};

pub async fn connect(url: &str) -> DatabaseConnection {
    Database::connect(url)
        .await
        .expect("Failed to connect to database")
}

// 迁移到数据库
pub async fn migration_db<T>(db_url: &str)
where
    T: MigratorTrait,
{
    let db = connect(db_url).await;
    T::fresh(&db).await.expect("Failed to migrate");
}


// 生成实体
pub async fn generate_entity(db_url: &str,out_path: &str) {
    let cli = sea_orm_cli::cli::Cli::parse_from([
        "sea-orm-cli",
        "generate",
        "entity",
        // "-l",
        "-u",
        db_url,
        "-o",
        out_path,
        //"--expanded-format",
        // "--with-serde",
        // "both",
        "--model-extra-derives",
        "Default",
        // "-t",
        // table_name.as_str(),
    ]);
    if let sea_orm_cli::Commands::Generate { command } = cli.command {
        sea_orm_cli::run_generate_command(command, cli.verbose)
            .await
            .expect("TODO: panic message");
    }
}