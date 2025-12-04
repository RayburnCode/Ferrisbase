pub use sea_orm_migration::prelude::*;

// Import your migration files
mod m002_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m002_create_users_table::Migration),
        ]
    }
}  