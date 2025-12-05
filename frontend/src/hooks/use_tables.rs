use dioxus::prelude::*;
use futures::stream::StreamExt;
use shared::models::{CreateTableRequest, TableResponse, TableSummary};
use crate::config::API_BASE_URL;

#[derive(Clone, Debug, PartialEq)]
pub enum TableState {
    Loading,
    Loaded(Vec<TableSummary>),
    Error(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableDetailState {
    Loading,
    Loaded(TableResponse),
    Error(String),
}

/// Hook to list all tables for a project
pub fn use_list_tables(project_slug: String) -> Resource<Result<Vec<TableSummary>, String>> {
    let auth_state = use_context::<Signal<crate::AuthState>>();
    
    use_resource(move || {
        let project_slug = project_slug.clone();
        let auth_state = auth_state.clone();
        
        async move {
            let token = auth_state.read().token.clone().ok_or("Not authenticated")?;
            
            let url = format!("{}/api/projects/{}/tables", API_BASE_URL, project_slug);
            let response = reqwest::Client::new()
                .get(&url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .map_err(|e| format!("Network error: {}", e))?;
            
            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(format!("HTTP {}: {}", status, error_text));
            }
            
            response
                .json::<Vec<TableSummary>>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        }
    })
}

/// Hook to get details for a specific table
pub fn use_get_table(project_slug: String, table_name: String) -> Resource<Result<TableResponse, String>> {
    let auth_state = use_context::<Signal<crate::AuthState>>();
    
    use_resource(move || {
        let project_slug = project_slug.clone();
        let table_name = table_name.clone();
        let auth_state = auth_state.clone();
        
        async move {
            let token = auth_state.read().token.clone().ok_or("Not authenticated")?;
            
            let url = format!("{}/api/projects/{}/tables/{}", API_BASE_URL, project_slug, table_name);
            let response = reqwest::Client::new()
                .get(&url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
                .map_err(|e| format!("Network error: {}", e))?;
            
            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                return Err(format!("HTTP {}: {}", status, error_text));
            }
            
            response
                .json::<TableResponse>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        }
    })
}

/// Action to create a new table
pub fn use_create_table(
    project_slug: String,
) -> Coroutine<CreateTableRequest> {
    let auth_state = use_context::<Signal<crate::AuthState>>();
    
    use_coroutine(move |mut rx: UnboundedReceiver<CreateTableRequest>| {
        let project_slug = project_slug.clone();
        let auth_state = auth_state.clone();
        
        async move {
            while let Some(request) = rx.next().await {
                let token = match auth_state.read().token.clone() {
                    Some(t) => t,
                    None => continue,
                };
                
                let url = format!("{}/api/projects/{}/tables", API_BASE_URL, project_slug);
                let _ = reqwest::Client::new()
                    .post(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await;
            }
        }
    })
}

/// Action to delete a table
pub fn use_delete_table(
    project_slug: String,
) -> Coroutine<String> {
    let auth_state = use_context::<Signal<crate::AuthState>>();
    
    use_coroutine(move |mut rx: UnboundedReceiver<String>| {
        let project_slug = project_slug.clone();
        let auth_state = auth_state.clone();
        
        async move {
            while let Some(table_name) = rx.next().await {
                let token = match auth_state.read().token.clone() {
                    Some(t) => t,
                    None => continue,
                };
                
                let url = format!("{}/api/projects/{}/tables/{}", API_BASE_URL, project_slug, table_name);
                let _ = reqwest::Client::new()
                    .delete(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await;
            }
        }
    })
}
