use sea_orm::*;
use database::entities::{project_tables, project_columns, projects};
use shared::models::{CreateTableRequest, TableResponse, TableSummary, ColumnResponse};
use crate::error::{AppError, AppResult};
use uuid::Uuid;

/// List all tables for a project
pub async fn list_project_tables(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
) -> AppResult<Vec<TableSummary>> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Get project and verify ownership
    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(project_slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Get all tables for this project
    let tables = project_tables::Entity::find()
        .filter(project_tables::Column::ProjectId.eq(project.id))
        .order_by_asc(project_tables::Column::TableName)
        .all(db)
        .await?;

    Ok(tables
        .into_iter()
        .map(|t| TableSummary {
            id: t.id.to_string(),
            table_name: t.table_name,
            display_name: t.display_name,
            description: t.description,
            row_count: t.row_count.unwrap_or(0),
            created_at: t.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        })
        .collect())
}

/// Get a specific table with its columns
pub async fn get_project_table(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    table_name: &str,
) -> AppResult<TableResponse> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Get project and verify ownership
    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(project_slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Get table
    let table = project_tables::Entity::find()
        .filter(project_tables::Column::ProjectId.eq(project.id))
        .filter(project_tables::Column::TableName.eq(table_name))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Table not found".to_string()))?;

    // Get columns for this table
    let columns = project_columns::Entity::find()
        .filter(project_columns::Column::ProjectTableId.eq(table.id))
        .order_by_asc(project_columns::Column::ColumnOrder)
        .all(db)
        .await?;

    let column_responses: Vec<ColumnResponse> = columns
        .into_iter()
        .map(|c| ColumnResponse {
            id: c.id.to_string(),
            name: c.column_name,
            display_name: c.display_name,
            data_type: c.data_type,
            is_nullable: c.is_nullable.unwrap_or(true),
            is_primary_key: c.is_primary_key.unwrap_or(false),
            is_unique: c.is_unique.unwrap_or(false),
            default_value: c.default_value,
            column_order: c.column_order,
        })
        .collect();

    Ok(TableResponse {
        id: table.id.to_string(),
        table_name: table.table_name,
        display_name: table.display_name,
        description: table.description,
        row_count: table.row_count.unwrap_or(0),
        columns: column_responses,
        created_at: table.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        updated_at: table.updated_at.map(|dt| dt.to_string()),
    })
}

/// Create a new table in PostgreSQL and track it in project_tables
pub async fn create_project_table(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    req: CreateTableRequest,
) -> AppResult<TableResponse> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Validate table name (alphanumeric + underscores only)
    if !req.table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(AppError::BadRequest(
            "Table name can only contain letters, numbers, and underscores".to_string(),
        ));
    }

    if req.columns.is_empty() {
        return Err(AppError::BadRequest("Table must have at least one column".to_string()));
    }

    // Get project and verify ownership
    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(project_slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Check if table already exists
    let existing = project_tables::Entity::find()
        .filter(project_tables::Column::ProjectId.eq(project.id))
        .filter(project_tables::Column::TableName.eq(&req.table_name))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Table with this name already exists".to_string()));
    }

    // Build the actual PostgreSQL table name (prefixed with project ID to avoid conflicts)
    let pg_table_name = format!("project_{}_{}", project.id.simple(), req.table_name);

    // Build CREATE TABLE SQL
    let mut column_defs: Vec<String> = Vec::new();
    let mut has_primary_key = false;

    for (_idx, col) in req.columns.iter().enumerate() {
        let mut col_def = format!("\"{}\" {}", col.name, col.data_type.to_postgres_type());

        if col.is_primary_key {
            col_def.push_str(" PRIMARY KEY");
            has_primary_key = true;
        }

        if !col.is_nullable && !col.is_primary_key {
            col_def.push_str(" NOT NULL");
        }

        if col.is_unique && !col.is_primary_key {
            col_def.push_str(" UNIQUE");
        }

        if let Some(ref default_val) = col.default_value {
            col_def.push_str(&format!(" DEFAULT {}", default_val));
        }

        column_defs.push(col_def);
    }

    // Add auto-increment ID if no primary key specified
    if !has_primary_key {
        column_defs.insert(
            0,
            "\"id\" UUID PRIMARY KEY DEFAULT gen_random_uuid()".to_string(),
        );
    }

    // Add timestamps
    column_defs.push("\"created_at\" TIMESTAMP DEFAULT CURRENT_TIMESTAMP".to_string());
    column_defs.push("\"updated_at\" TIMESTAMP DEFAULT CURRENT_TIMESTAMP".to_string());

    let create_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
        pg_table_name,
        column_defs.join(", ")
    );

    // Execute the CREATE TABLE statement
    db.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        create_table_sql,
    ))
    .await?;

    // Create record in project_tables
    let table_id = Uuid::new_v4();
    let project_table = project_tables::ActiveModel {
        id: Set(table_id),
        project_id: Set(project.id),
        table_name: Set(req.table_name.clone()),
        display_name: Set(req.display_name.clone()),
        description: Set(req.description.clone()),
        row_count: Set(Some(0)),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
        updated_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    let table_result = project_table.insert(db).await?;

    // Create column records
    let mut column_responses = Vec::new();
    let mut order = 0;

    // Add auto ID column if no PK was specified
    if !has_primary_key {
        let col_id = Uuid::new_v4();
        let col = project_columns::ActiveModel {
            id: Set(col_id),
            project_table_id: Set(table_id),
            column_name: Set("id".to_string()),
            display_name: Set("ID".to_string()),
            data_type: Set("uuid".to_string()),
            is_nullable: Set(Some(false)),
            is_primary_key: Set(Some(true)),
            is_unique: Set(Some(true)),
            default_value: Set(Some("gen_random_uuid()".to_string())),
            column_order: Set(order),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
        };
        let col_result = col.insert(db).await?;
        column_responses.push(ColumnResponse {
            id: col_result.id.to_string(),
            name: col_result.column_name,
            display_name: col_result.display_name,
            data_type: col_result.data_type,
            is_nullable: col_result.is_nullable.unwrap_or(false),
            is_primary_key: col_result.is_primary_key.unwrap_or(false),
            is_unique: col_result.is_unique.unwrap_or(false),
            default_value: col_result.default_value,
            column_order: col_result.column_order,
        });
        order += 1;
    }

    for col_def in req.columns {
        let col_id = Uuid::new_v4();
        let col = project_columns::ActiveModel {
            id: Set(col_id),
            project_table_id: Set(table_id),
            column_name: Set(col_def.name.clone()),
            display_name: Set(col_def.display_name.clone()),
            data_type: Set(format!("{:?}", col_def.data_type).to_lowercase()),
            is_nullable: Set(Some(col_def.is_nullable)),
            is_primary_key: Set(Some(col_def.is_primary_key)),
            is_unique: Set(Some(col_def.is_unique)),
            default_value: Set(col_def.default_value.clone()),
            column_order: Set(order),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
        };

        let col_result = col.insert(db).await?;
        column_responses.push(ColumnResponse {
            id: col_result.id.to_string(),
            name: col_result.column_name,
            display_name: col_result.display_name,
            data_type: col_result.data_type,
            is_nullable: col_result.is_nullable.unwrap_or(true),
            is_primary_key: col_result.is_primary_key.unwrap_or(false),
            is_unique: col_result.is_unique.unwrap_or(false),
            default_value: col_result.default_value,
            column_order: col_result.column_order,
        });
        order += 1;
    }

    // Add timestamp columns
    for (name, display) in [("created_at", "Created At"), ("updated_at", "Updated At")] {
        let col_id = Uuid::new_v4();
        let col = project_columns::ActiveModel {
            id: Set(col_id),
            project_table_id: Set(table_id),
            column_name: Set(name.to_string()),
            display_name: Set(display.to_string()),
            data_type: Set("timestamp".to_string()),
            is_nullable: Set(Some(true)),
            is_primary_key: Set(Some(false)),
            is_unique: Set(Some(false)),
            default_value: Set(Some("CURRENT_TIMESTAMP".to_string())),
            column_order: Set(order),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
        };
        let col_result = col.insert(db).await?;
        column_responses.push(ColumnResponse {
            id: col_result.id.to_string(),
            name: col_result.column_name,
            display_name: col_result.display_name,
            data_type: col_result.data_type,
            is_nullable: col_result.is_nullable.unwrap_or(true),
            is_primary_key: col_result.is_primary_key.unwrap_or(false),
            is_unique: col_result.is_unique.unwrap_or(false),
            default_value: col_result.default_value,
            column_order: col_result.column_order,
        });
        order += 1;
    }

    Ok(TableResponse {
        id: table_result.id.to_string(),
        table_name: table_result.table_name,
        display_name: table_result.display_name,
        description: table_result.description,
        row_count: 0,
        columns: column_responses,
        created_at: table_result.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        updated_at: table_result.updated_at.map(|dt| dt.to_string()),
    })
}

/// Delete a table
pub async fn delete_project_table(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    table_name: &str,
) -> AppResult<()> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Get project and verify ownership
    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(project_slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Get table
    let table = project_tables::Entity::find()
        .filter(project_tables::Column::ProjectId.eq(project.id))
        .filter(project_tables::Column::TableName.eq(table_name))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Table not found".to_string()))?;

    // Drop the actual PostgreSQL table
    let pg_table_name = format!("project_{}_{}", project.id.simple(), table_name);
    let drop_table_sql = format!("DROP TABLE IF EXISTS \"{}\" CASCADE", pg_table_name);

    db.execute(Statement::from_string(
        DatabaseBackend::Postgres,
        drop_table_sql,
    ))
    .await?;

    // Delete the table record (columns will cascade delete)
    table.delete(db).await?;

    Ok(())
}
