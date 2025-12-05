pub use sea_orm_migration::prelude::*;

// Import your migration files
mod m002_create_users_table;
mod m003_create_projects_table;
mod m004_create_project_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m002_create_users_table::Migration),
            Box::new(m003_create_projects_table::Migration),
            Box::new(m004_create_project_tables::Migration),
        ]
    }
}  