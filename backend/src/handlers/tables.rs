use axum::{extract::{State, Path}, Extension, Json};
use crate::config::AppState;
use crate::error::AppResult;
use crate::services;
use shared::models::{Claims, CreateTableRequest, TableResponse, TableSummary};

/// GET /api/projects/:slug/tables - List all tables in a project
pub async fn list_tables(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
) -> AppResult<Json<Vec<TableSummary>>> {
    let tables = services::list_project_tables(&state.db, &claims.sub, &slug).await?;
    Ok(Json(tables))
}

/// GET /api/projects/:slug/tables/:table_name - Get a specific table
pub async fn get_table(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((slug, table_name)): Path<(String, String)>,
) -> AppResult<Json<TableResponse>> {
    let table = services::get_project_table(&state.db, &claims.sub, &slug, &table_name).await?;
    Ok(Json(table))
}

/// POST /api/projects/:slug/tables - Create a new table
pub async fn create_table(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
    Json(req): Json<CreateTableRequest>,
) -> AppResult<Json<TableResponse>> {
    let table = services::create_project_table(&state.db, &claims.sub, &slug, req).await?;
    Ok(Json(table))
}

/// DELETE /api/projects/:slug/tables/:table_name - Delete a table
pub async fn delete_table(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((slug, table_name)): Path<(String, String)>,
) -> AppResult<Json<serde_json::Value>> {
    services::delete_project_table(&state.db, &claims.sub, &slug, &table_name).await?;
    
    Ok(Json(serde_json::json!({
        "message": "Table deleted successfully"
    })))
}
