pub use sea_orm_migration::prelude::*;

mod m20240503_013152_create_user_roles;
mod m20240503_025221_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240503_013152_create_user_roles::Migration),
            Box::new(m20240503_025221_create_user::Migration),
        ]
    }
}
