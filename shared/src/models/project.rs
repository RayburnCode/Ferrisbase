use serde::{Deserialize, Serialize};

/// Request payload for creating a new project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Response payload for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub owner_id: String,
    pub database_status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

/// Request payload for updating a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
