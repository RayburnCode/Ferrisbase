use axum::{
    routing::{get, post},
    Router,
};

use crate::config::AppState;
use crate::handlers;

/// Create the main application router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Auth routes (public)
        .route("/auth/register", post(handlers::register))
        .route("/auth/login", post(handlers::login))
        .route("/auth/logout", post(handlers::logout))
        // Protected routes (require authentication)
        .route("/auth/me", get(handlers::get_current_user))
        // Add more routes here as needed
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}
