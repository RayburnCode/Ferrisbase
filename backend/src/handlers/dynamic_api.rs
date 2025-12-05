use axum::{
    extract::{Path, Query, State, Extension},
    Json,
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use crate::config::AppState;
use crate::error::AppResult;
use crate::services;
use shared::models::Claims;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

/// GET /api/data/:project_slug/:table_name
/// List all rows from a user-defined table
pub async fn list_table_rows(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((project_slug, table_name)): Path<(String, String)>,
    Query(params): Query<QueryParams>,
) -> AppResult<Json<Vec<JsonValue>>> {
    let rows = services::query_table(
        &state.db,
        &claims.sub,
        &project_slug,
        &table_name,
        None,
        params.limit,
        params.offset,
    )
    .await?;

    Ok(Json(rows))
}

/// GET /api/data/:project_slug/:table_name/:id
/// Get a single row by ID from a user-defined table
pub async fn get_table_row(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((project_slug, table_name, row_id)): Path<(String, String, String)>,
) -> AppResult<Json<JsonValue>> {
    let row = services::get_table_row(
        &state.db,
        &claims.sub,
        &project_slug,
        &table_name,
        &row_id,
    )
    .await?;

    Ok(Json(row))
}

/// POST /api/data/:project_slug/:table_name
/// Create a new row in a user-defined table
pub async fn create_table_row(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((project_slug, table_name)): Path<(String, String)>,
    Json(data): Json<JsonValue>,
) -> AppResult<(StatusCode, Json<JsonValue>)> {
    let row = services::insert_table_row(
        &state.db,
        &claims.sub,
        &project_slug,
        &table_name,
        data,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(row)))
}

/// PUT /api/data/:project_slug/:table_name/:id
/// Update a row in a user-defined table
pub async fn update_table_row(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((project_slug, table_name, row_id)): Path<(String, String, String)>,
    Json(data): Json<JsonValue>,
) -> AppResult<Json<JsonValue>> {
    let row = services::update_table_row(
        &state.db,
        &claims.sub,
        &project_slug,
        &table_name,
        &row_id,
        data,
    )
    .await?;

    Ok(Json(row))
}

/// PATCH /api/data/:project_slug/:table_name/:id
/// Partially update a row in a user-defined table
pub async fn patch_table_row(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((project_slug, table_name, row_id)): Path<(String, String, String)>,
    Json(data): Json<JsonValue>,
) -> AppResult<Json<JsonValue>> {
    // PATCH works the same as PUT in this case since we only update provided fields
    let row = services::update_table_row(
        &state.db,
        &claims.sub,
        &project_slug,
        &table_name,
        &row_id,
        data,
    )
    .await?;

    Ok(Json(row))
}

/// DELETE /api/data/:project_slug/:table_name/:id
/// Delete a row from a user-defined table
pub async fn delete_table_row(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((project_slug, table_name, row_id)): Path<(String, String, String)>,
) -> AppResult<StatusCode> {
    services::delete_table_row(
        &state.db,
        &claims.sub,
        &project_slug,
        &table_name,
        &row_id,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
