use dioxus::prelude::*;
use futures::stream::StreamExt;
use shared::models::{ExecuteSqlRequest, ExecuteSqlResponse};
use crate::config::endpoints;

/// Hook to execute SQL queries against a project
pub fn use_execute_sql(
    project_slug: String,
) -> Coroutine<(String, Signal<Option<Result<ExecuteSqlResponse, String>>>)> {
    let auth_state = use_context::<Signal<crate::AuthState>>();
    
    use_coroutine(move |mut rx: UnboundedReceiver<(String, Signal<Option<Result<ExecuteSqlResponse, String>>>)>| {
        let project_slug = project_slug.clone();
        let auth_state = auth_state.clone();
        
        async move {
            while let Some((query, mut result_signal)) = rx.next().await {
                // Set loading state
                result_signal.set(None);
                
                let token = match auth_state.read().token.clone() {
                    Some(t) => t,
                    None => {
                        result_signal.set(Some(Err("Not authenticated".to_string())));
                        continue;
                    }
                };
                
                let url = endpoints::execute_sql(&project_slug);
                let request = ExecuteSqlRequest { query };
                
                let response = reqwest::Client::new()
                    .post(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&request)
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            match resp.json::<ExecuteSqlResponse>().await {
                                Ok(data) => result_signal.set(Some(Ok(data))),
                                Err(e) => result_signal.set(Some(Err(format!("Failed to parse response: {}", e)))),
                            }
                        } else {
                            let status = resp.status();
                            let error_text = resp.text().await.unwrap_or_default();
                            result_signal.set(Some(Err(format!("HTTP {}: {}", status, error_text))));
                        }
                    }
                    Err(e) => {
                        result_signal.set(Some(Err(format!("Network error: {}", e))));
                    }
                }
            }
        }
    })
}
