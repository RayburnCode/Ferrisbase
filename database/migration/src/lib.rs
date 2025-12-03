pub use sea_orm_migration::prelude::*;

// Import your migration files
mod m001_create_contact_form;
mod m002_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_create_contact_form::Migration),
            Box::new(m002_create_users_table::Migration),
        ]
    }
}  