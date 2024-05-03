use entity::user::UserRole;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Schema},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);

        manager
            .create_type(schema.create_enum_from_active_enum::<UserRole>())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(Alias::new("user_roles")).to_owned())
            .await
    }
}
