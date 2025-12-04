use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create projects table
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                    )
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::Description).text())
                    .col(ColumnDef::new(Projects::Slug).string().not_null().unique_key())
                    .col(ColumnDef::new(Projects::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Projects::DatabaseStatus).string().default("pending"))
                    .col(ColumnDef::new(Projects::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Projects::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_projects_owner")
                            .from(Projects::Table, Projects::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on slug for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_projects_slug")
                    .table(Projects::Table)
                    .col(Projects::Slug)
                    .to_owned(),
            )
            .await?;

        // Create index on owner_id for faster queries
        manager
            .create_index(
                Index::create()
                    .name("idx_projects_owner_id")
                    .table(Projects::Table)
                    .col(Projects::OwnerId)
                    .to_owned(),
            )
            .await?;

        // Create index on database_status for filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_projects_database_status")
                    .table(Projects::Table)
                    .col(Projects::DatabaseStatus)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
    Name,
    Description,
    Slug,
    OwnerId,
    DatabaseStatus,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
