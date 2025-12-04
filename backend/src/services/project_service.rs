use sea_orm::*;
use database::entities::projects;
use shared::models::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest};
use crate::error::{AppError, AppResult};
use uuid::Uuid;
use slug::slugify;

/// Create a new project
pub async fn create_project(
    db: &DatabaseConnection,
    user_id: &str,
    req: CreateProjectRequest,
) -> AppResult<ProjectResponse> {
    // Validate project name
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("Project name cannot be empty".to_string()));
    }

    if req.name.len() < 3 {
        return Err(AppError::BadRequest("Project name must be at least 3 characters".to_string()));
    }

    // Parse user_id
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Generate slug from name
    let base_slug = slugify(&req.name);
    let mut slug = base_slug.clone();
    let mut counter = 1;

    // Ensure slug is unique
    while projects::Entity::find()
        .filter(projects::Column::Slug.eq(&slug))
        .one(db)
        .await?
        .is_some()
    {
        slug = format!("{}-{}", base_slug, counter);
        counter += 1;
    }

    // Create project
    let project = projects::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(req.name.clone()),
        description: Set(req.description.clone()),
        slug: Set(slug.clone()),
        owner_id: Set(owner_uuid),
        database_status: Set(Some("pending".to_string())),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
        updated_at: Set(Some(chrono::Utc::now().naive_utc())),
    };

    let result = project.insert(db).await?;

    Ok(ProjectResponse {
        id: result.id.to_string(),
        name: result.name,
        description: result.description,
        slug: result.slug,
        owner_id: result.owner_id.to_string(),
        database_status: result.database_status.unwrap_or_else(|| "pending".to_string()),
        created_at: result.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        updated_at: result.updated_at.map(|dt| dt.to_string()),
    })
}

/// List all projects for a user
pub async fn list_user_projects(
    db: &DatabaseConnection,
    user_id: &str,
) -> AppResult<Vec<ProjectResponse>> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let projects_list = projects::Entity::find()
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .order_by_desc(projects::Column::CreatedAt)
        .all(db)
        .await?;

    Ok(projects_list
        .into_iter()
        .map(|p| ProjectResponse {
            id: p.id.to_string(),
            name: p.name,
            description: p.description,
            slug: p.slug,
            owner_id: p.owner_id.to_string(),
            database_status: p.database_status.unwrap_or_else(|| "pending".to_string()),
            created_at: p.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: p.updated_at.map(|dt| dt.to_string()),
        })
        .collect())
}

/// Get a project by slug (verify ownership)
pub async fn get_project_by_slug(
    db: &DatabaseConnection,
    user_id: &str,
    slug: &str,
) -> AppResult<ProjectResponse> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    Ok(ProjectResponse {
        id: project.id.to_string(),
        name: project.name,
        description: project.description,
        slug: project.slug,
        owner_id: project.owner_id.to_string(),
        database_status: project.database_status.unwrap_or_else(|| "pending".to_string()),
        created_at: project.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        updated_at: project.updated_at.map(|dt| dt.to_string()),
    })
}

/// Update a project
pub async fn update_project(
    db: &DatabaseConnection,
    user_id: &str,
    slug: &str,
    req: UpdateProjectRequest,
) -> AppResult<ProjectResponse> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Find the project
    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Update fields
    let mut project: projects::ActiveModel = project.into();
    
    if let Some(name) = req.name {
        if name.trim().is_empty() {
            return Err(AppError::BadRequest("Project name cannot be empty".to_string()));
        }
        project.name = Set(name);
    }
    
    if let Some(description) = req.description {
        project.description = Set(Some(description));
    }
    
    project.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

    let updated = project.update(db).await?;

    Ok(ProjectResponse {
        id: updated.id.to_string(),
        name: updated.name,
        description: updated.description,
        slug: updated.slug,
        owner_id: updated.owner_id.to_string(),
        database_status: updated.database_status.unwrap_or_else(|| "pending".to_string()),
        created_at: updated.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        updated_at: updated.updated_at.map(|dt| dt.to_string()),
    })
}

/// Delete a project
pub async fn delete_project(
    db: &DatabaseConnection,
    user_id: &str,
    slug: &str,
) -> AppResult<()> {
    let owner_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Find the project
    let project = projects::Entity::find()
        .filter(projects::Column::Slug.eq(slug))
        .filter(projects::Column::OwnerId.eq(owner_uuid))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

    // Delete the project
    project.delete(db).await?;

    Ok(())
}
