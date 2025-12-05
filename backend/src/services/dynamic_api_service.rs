use sea_orm::*;
use database::entities::{project_tables, project_columns, projects};
use crate::error::{AppError, AppResult};
use uuid::Uuid;
use serde_json::{Value as JsonValue};

/// Execute a SELECT query on a user-defined table
pub async fn query_table(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    table_name: &str,
    _filters: Option<Vec<(String, String)>>,
    limit: Option<u64>,
    offset: Option<u64>,
) -> AppResult<Vec<JsonValue>> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Verify project ownership and get table
    let (project, _table) = verify_table_access(db, owner_uuid, project_slug, table_name).await?;
    
    // Build the actual PostgreSQL table name
    let pg_table_name = format!("project_{}_{}", project.id.as_simple(), table_name);
    
    // Use row_to_json to convert rows to JSON automatically
    let query = format!(
        "SELECT row_to_json(t) FROM (SELECT * FROM \"{}\" ORDER BY created_at DESC LIMIT {} OFFSET {}) t",
        pg_table_name,
        limit.unwrap_or(100).min(1000),
        offset.unwrap_or(0)
    );
    
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    let result = db.query_all(stmt).await?;
    
    // Convert rows to JSON
    let rows: Vec<JsonValue> = result
        .into_iter()
        .filter_map(|row| {
            row.try_get::<JsonValue>("", "row_to_json").ok()
        })
        .collect();
    
    Ok(rows)
}

/// Get a single row by ID from a user-defined table
pub async fn get_table_row(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    table_name: &str,
    row_id: &str,
) -> AppResult<JsonValue> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let (project, _table) = verify_table_access(db, owner_uuid, project_slug, table_name).await?;
    
    let pg_table_name = format!("project_{}_{}", project.id.as_simple(), table_name);
    
    // Parse row_id as UUID for validation
    let _row_uuid = Uuid::parse_str(row_id)
        .map_err(|_| AppError::BadRequest("Invalid row ID".to_string()))?;
    
    let query = format!(
        "SELECT row_to_json(t) FROM (SELECT * FROM \"{}\" WHERE id = '{}') t",
        pg_table_name, row_id
    );
    
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    
    let result = db.query_one(stmt).await?
        .ok_or_else(|| AppError::NotFound("Row not found".to_string()))?;
    
    let row = result.try_get::<JsonValue>("", "row_to_json")
        .map_err(|_| AppError::NotFound("Row not found".to_string()))?;
    
    Ok(row)
}

/// Insert a new row into a user-defined table
pub async fn insert_table_row(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    table_name: &str,
    data: JsonValue,
) -> AppResult<JsonValue> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let (project, table) = verify_table_access(db, owner_uuid, project_slug, table_name).await?;
    
    let pg_table_name = format!("project_{}_{}", project.id.as_simple(), table_name);
    
    // Get columns for this table
    let columns = project_columns::Entity::find()
        .filter(project_columns::Column::ProjectTableId.eq(table.id))
        .order_by_asc(project_columns::Column::ColumnOrder)
        .all(db)
        .await?;
    
    let data_obj = data.as_object()
        .ok_or_else(|| AppError::BadRequest("Request body must be a JSON object".to_string()))?;
    
    // Build INSERT statement
    let mut col_names = vec![];
    let mut values = vec![];
    
    for col in columns.iter() {
        if col.column_name == "id" || col.column_name == "created_at" || col.column_name == "updated_at" {
            continue; // Skip auto-generated columns
        }
        
        if let Some(value) = data_obj.get(&col.column_name) {
            col_names.push(format!("\"{}\"", col.column_name));
            // Properly escape JSON values for SQL
            let sql_value = match value {
                JsonValue::String(s) => format!("'{}'", s.replace("'", "''")),
                JsonValue::Number(n) => n.to_string(),
                JsonValue::Bool(b) => b.to_string(),
                JsonValue::Null => "NULL".to_string(),
                _ => format!("'{}'", value.to_string().replace("'", "''")),
            };
            values.push(sql_value);
        }
    }
    
    if col_names.is_empty() {
        return Err(AppError::BadRequest("No valid columns provided".to_string()));
    }
    
    let query = format!(
        "INSERT INTO \"{}\" ({}) VALUES ({}) RETURNING row_to_json(\"{}\".*)",
        pg_table_name,
        col_names.join(", "),
        values.join(", "),
        pg_table_name
    );
    
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    
    let result = db.query_one(stmt).await?
        .ok_or_else(|| AppError::BadRequest("Failed to insert row".to_string()))?;
    
    let row = result.try_get::<JsonValue>("", "row_to_json")
        .map_err(|_| AppError::BadRequest("Failed to parse result".to_string()))?;
    
    Ok(row)
}

/// Update a row in a user-defined table
pub async fn update_table_row(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    table_name: &str,
    row_id: &str,
    data: JsonValue,
) -> AppResult<JsonValue> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let (project, table) = verify_table_access(db, owner_uuid, project_slug, table_name).await?;
    
    let pg_table_name = format!("project_{}_{}", project.id.as_simple(), table_name);
    
    let _row_uuid = Uuid::parse_str(row_id)
        .map_err(|_| AppError::BadRequest("Invalid row ID".to_string()))?;
    
    // Get columns
    let columns = project_columns::Entity::find()
        .filter(project_columns::Column::ProjectTableId.eq(table.id))
        .all(db)
        .await?;
    
    let data_obj = data.as_object()
        .ok_or_else(|| AppError::BadRequest("Request body must be a JSON object".to_string()))?;
    
    // Build UPDATE statement
    let mut set_clauses = vec![];
    
    for col in columns.iter() {
        if col.column_name == "id" || col.column_name == "created_at" {
            continue; // Skip immutable columns
        }
        
        if let Some(value) = data_obj.get(&col.column_name) {
            let sql_value = match value {
                JsonValue::String(s) => format!("'{}'", s.replace("'", "''")),
                JsonValue::Number(n) => n.to_string(),
                JsonValue::Bool(b) => b.to_string(),
                JsonValue::Null => "NULL".to_string(),
                _ => format!("'{}'", value.to_string().replace("'", "''")),
            };
            set_clauses.push(format!("\"{}\" = {}", col.column_name, sql_value));
        }
    }
    
    if set_clauses.is_empty() {
        return Err(AppError::BadRequest("No valid columns to update".to_string()));
    }
    
    // Always update updated_at
    set_clauses.push("updated_at = NOW()".to_string());
    
    let query = format!(
        "UPDATE \"{}\" SET {} WHERE id = '{}' RETURNING row_to_json(\"{}\".*)",
        pg_table_name,
        set_clauses.join(", "),
        row_id,
        pg_table_name
    );
    
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    
    let result = db.query_one(stmt).await?
        .ok_or_else(|| AppError::NotFound("Row not found".to_string()))?;
    
    let row = result.try_get::<JsonValue>("", "row_to_json")
        .map_err(|_| AppError::BadRequest("Failed to parse result".to_string()))?;
    
    Ok(row)
}

/// Delete a row from a user-defined table
pub async fn delete_table_row(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    table_name: &str,
    row_id: &str,
) -> AppResult<()> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let (project, _table) = verify_table_access(db, owner_uuid, project_slug, table_name).await?;
    
    let pg_table_name = format!("project_{}_{}", project.id.as_simple(), table_name);
    
    let _row_uuid = Uuid::parse_str(row_id)
        .map_err(|_| AppError::BadRequest("Invalid row ID".to_string()))?;
    
    let query = format!("DELETE FROM \"{}\" WHERE id = '{}'", pg_table_name, row_id);
    let stmt = Statement::from_string(DatabaseBackend::Postgres, query);
    
    let result = db.execute(stmt).await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Row not found".to_string()));
    }
    
    Ok(())
}

/// Execute arbitrary SQL query within project context
/// This allows users to run custom SQL queries but only on their own project tables
pub async fn execute_sql(
    db: &DatabaseConnection,
    user_id: &str,
    project_slug: &str,
    sql_query: &str,
) -> AppResult<(Vec<JsonValue>, Option<u64>)> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Verify project ownership
    let _project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(project_slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Basic SQL injection prevention - reject dangerous keywords
    let sql_lower = sql_query.to_lowercase();
    let dangerous_patterns = [
        "drop database",
        "drop schema", 
        "create database",
        "create schema",
        "alter database",
        "alter schema",
    ];
    
    for pattern in &dangerous_patterns {
        if sql_lower.contains(pattern) {
            return Err(AppError::BadRequest(
                format!("Query contains forbidden pattern: {}", pattern)
            ));
        }
    }

    // Check if query accesses system tables/schemas that should be restricted
    let restricted_patterns = [
        "pg_authid",
        "pg_shadow",
        "pg_user",
    ];
    
    for pattern in &restricted_patterns {
        if sql_lower.contains(pattern) {
            return Err(AppError::BadRequest(
                format!("Access to {} is not allowed", pattern)
            ));
        }
    }

    // Determine if this is a SELECT query or a modification query
    let is_select = sql_lower.trim().starts_with("select") || 
                    sql_lower.trim().starts_with("with") ||
                    sql_lower.trim().starts_with("show") ||
                    sql_lower.trim().starts_with("explain");
    
    if is_select {
        // For SELECT queries, wrap in a subquery to convert to JSON
        let wrapped_query = format!(
            "SELECT row_to_json(t) as data FROM ({}) t",
            sql_query
        );
        
        let stmt = Statement::from_string(DatabaseBackend::Postgres, wrapped_query);
        let result = db.query_all(stmt).await
            .map_err(|e| AppError::BadRequest(format!("SQL execution error: {}", e)))?;
        
        // Convert rows to JSON and get count before moving
        let row_count = result.len() as u64;
        let rows: Vec<JsonValue> = result
            .into_iter()
            .filter_map(|row| {
                row.try_get::<JsonValue>("", "data").ok()
            })
            .collect();
        
        Ok((rows, Some(row_count)))
    } else {
        // For INSERT, UPDATE, DELETE, CREATE TABLE etc.
        let stmt = Statement::from_string(DatabaseBackend::Postgres, sql_query.to_string());
        let result = db.execute(stmt).await
            .map_err(|e| AppError::BadRequest(format!("SQL execution error: {}", e)))?;
        
        let rows_affected = result.rows_affected();
        
        // Return empty result set with row count
        Ok((vec![], Some(rows_affected)))
    }
}

/// Helper function to verify table access and ownership
async fn verify_table_access(
    db: &DatabaseConnection,
    owner_uuid: Uuid,
    project_slug: &str,
    table_name: &str,
) -> AppResult<(projects::Model, project_tables::Model)> {
    // Get project and verify ownership
    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(project_slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Get table and verify it belongs to this project
    let table = project_tables::Entity::find()
        .filter(project_tables::Column::ProjectId.eq(project.id))
        .filter(project_tables::Column::TableName.eq(table_name))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Table not found".to_string()))?;

    Ok((project, table))
}
