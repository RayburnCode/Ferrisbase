use dioxus::prelude::*;
use shared::models::{CreateProjectRequest, ProjectResponse};
use reqwest::Client;

/// Hook to create a new project
#[allow(dead_code)]
pub fn use_create_project() -> impl Fn(String, Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ProjectResponse, String>>>> {
    let auth_state = use_context::<Signal<crate::AuthState>>();
    
    move |name: String, description: Option<String>| {
        let token = auth_state.read().token.clone();
        
        Box::pin(async move {
            let token = token.ok_or_else(|| "Not authenticated".to_string())?;
            
            let client = Client::new();
            let create_req = CreateProjectRequest {
                name: name.clone(),
                description,
            };

            match client
                .post("http://127.0.0.1:8081/api/projects")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .json(&create_req)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<ProjectResponse>().await {
                            Ok(project) => Ok(project),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    } else {
                        let status_code = response.status().as_u16();
                        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        Err(match status_code {
                            400 => "Invalid project data".to_string(),
                            401 => "Unauthorized. Please log in again.".to_string(),
                            409 => "A project with this name already exists".to_string(),
                            _ => error_text,
                        })
                    }
                }
                Err(e) => Err(format!("Network error: {}", e)),
            }
        })
    }
}

/// Hook to fetch all projects for the current user
#[allow(dead_code)]
pub fn use_fetch_projects() -> impl Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<ProjectResponse>, String>>>> {
    let auth_state = use_context::<Signal<crate::AuthState>>();
    
    move || {
        let token = auth_state.read().token.clone();
        
        Box::pin(async move {
            let token = token.ok_or_else(|| "Not authenticated".to_string())?;
            
            let client = Client::new();

            match client
                .get("http://127.0.0.1:8081/api/projects")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<Vec<ProjectResponse>>().await {
                            Ok(projects) => Ok(projects),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    } else {
                        let status_code = response.status().as_u16();
                        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        Err(match status_code {
                            401 => "Unauthorized. Please log in again.".to_string(),
                            _ => error_text,
                        })
                    }
                }
                Err(e) => Err(format!("Network error: {}", e)),
            }
        })
    }
}
