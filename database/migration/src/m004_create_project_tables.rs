use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create project_tables table to track user-created tables
        manager
            .create_table(
                Table::create()
                    .table(ProjectTables::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectTables::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                    )
                    .col(ColumnDef::new(ProjectTables::ProjectId).uuid().not_null())
                    .col(ColumnDef::new(ProjectTables::TableName).string().not_null())
                    .col(ColumnDef::new(ProjectTables::DisplayName).string().not_null())
                    .col(ColumnDef::new(ProjectTables::Description).text())
                    .col(ColumnDef::new(ProjectTables::RowCount).integer().default(0))
                    .col(ColumnDef::new(ProjectTables::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(ProjectTables::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_tables_project")
                            .from(ProjectTables::Table, ProjectTables::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create project_columns table to track columns in user-created tables
        manager
            .create_table(
                Table::create()
                    .table(ProjectColumns::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectColumns::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                    )
                    .col(ColumnDef::new(ProjectColumns::ProjectTableId).uuid().not_null())
                    .col(ColumnDef::new(ProjectColumns::ColumnName).string().not_null())
                    .col(ColumnDef::new(ProjectColumns::DisplayName).string().not_null())
                    .col(ColumnDef::new(ProjectColumns::DataType).string().not_null())
                    .col(ColumnDef::new(ProjectColumns::IsNullable).boolean().default(true))
                    .col(ColumnDef::new(ProjectColumns::IsPrimaryKey).boolean().default(false))
                    .col(ColumnDef::new(ProjectColumns::IsUnique).boolean().default(false))
                    .col(ColumnDef::new(ProjectColumns::DefaultValue).string())
                    .col(ColumnDef::new(ProjectColumns::ColumnOrder).integer().not_null())
                    .col(ColumnDef::new(ProjectColumns::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_columns_table")
                            .from(ProjectColumns::Table, ProjectColumns::ProjectTableId)
                            .to(ProjectTables::Table, ProjectTables::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint on project_id + table_name
        manager
            .create_index(
                Index::create()
                    .name("idx_project_tables_unique")
                    .table(ProjectTables::Table)
                    .col(ProjectTables::ProjectId)
                    .col(ProjectTables::TableName)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create index for faster lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_project_tables_project_id")
                    .table(ProjectTables::Table)
                    .col(ProjectTables::ProjectId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectColumns::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(ProjectTables::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProjectTables {
    Table,
    Id,
    ProjectId,
    TableName,
    DisplayName,
    Description,
    RowCount,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ProjectColumns {
    Table,
    Id,
    ProjectTableId,
    ColumnName,
    DisplayName,
    DataType,
    IsNullable,
    IsPrimaryKey,
    IsUnique,
    DefaultValue,
    ColumnOrder,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}
