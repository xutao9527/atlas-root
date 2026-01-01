use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::sea_orm::{EnumIter, Iterable};


#[derive(DeriveIden)]
pub enum AtlasUser {
    Table,
    Id,
    UserType,
    Account,
    Password,
    Name,
    Balance,
    Avatar,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum UserType{
    Normal,
    Test,
    Bot
}

#[derive(DeriveMigrationName)]
pub struct AtlasUserMigration;

#[async_trait::async_trait]
impl MigrationTrait for AtlasUserMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut create_statement = Table::create();
        create_statement
            .table(AtlasUser::Table)
            .if_not_exists()
            .col(char_len(AtlasUser::Id,26))
            .col(enumeration_null(AtlasUser::UserType,"UserType",UserType::iter()))
            .col(string_len(AtlasUser::Account,64))
            .col(string_len(AtlasUser::Password,128))
            .col(string_len(AtlasUser::Name,64))
            .col(decimal_len(AtlasUser::Balance,10,2).default(0))
            .col(string_len_null(AtlasUser::Avatar,256))
            .col(timestamp(AtlasUser::CreatedAt))
            .col(timestamp_null(AtlasUser::UpdatedAt))
            .primary_key(Index::create().col(AtlasUser::Id))
            .index(Index::create().unique().col(AtlasUser::Account));
        manager.create_table(create_statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AtlasUser::Table).to_owned())
            .await
    }
}
