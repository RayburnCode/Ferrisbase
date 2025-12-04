use axum::{extract::{State, Path}, Extension, Json};
use crate::config::AppState;
use crate::error::AppResult;
use crate::services;
use shared::models::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest, Claims};

/// POST /api/projects - Create a new project
pub async fn create_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(create_req): Json<CreateProjectRequest>,
) -> AppResult<Json<ProjectResponse>> {
    let project = services::create_project(&state.db, &claims.sub, create_req).await?;
    Ok(Json(project))
}

/// GET /api/projects - Get all projects for current user
pub async fn list_projects(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<Json<Vec<ProjectResponse>>> {
    let projects = services::list_user_projects(&state.db, &claims.sub).await?;
    Ok(Json(projects))
}

/// GET /api/projects/:slug - Get a single project by slug
pub async fn get_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
) -> AppResult<Json<ProjectResponse>> {
    let project = services::get_project_by_slug(&state.db, &claims.sub, &slug).await?;
    Ok(Json(project))
}

/// PUT /api/projects/:slug - Update a project
pub async fn update_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
    Json(update_req): Json<UpdateProjectRequest>,
) -> AppResult<Json<ProjectResponse>> {
    let project = services::update_project(&state.db, &claims.sub, &slug, update_req).await?;
    Ok(Json(project))
}

/// DELETE /api/projects/:slug - Delete a project
pub async fn delete_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    services::delete_project(&state.db, &claims.sub, &slug).await?;
    
    Ok(Json(serde_json::json!({
        "message": "Project deleted successfully"
    })))
}
