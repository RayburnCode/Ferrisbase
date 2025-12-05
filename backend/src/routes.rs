use axum::{
    routing::{get, post},
    Router,
};

use crate::config::AppState;
use crate::handlers;
use crate::middleware;

/// Create the main application router
pub fn create_router(state: AppState) -> Router {
    let protected_projects = Router::new()
        .route("/", post(handlers::create_project).get(handlers::list_projects))
        .route("/{slug}", 
            get(handlers::get_project)
                .put(handlers::update_project)
                .delete(handlers::delete_project)
        )
        .route("/{slug}/tables",
            get(handlers::list_tables)
                .post(handlers::create_table)
        )
        .route("/{slug}/tables/{table_name}",
            get(handlers::get_table)
                .delete(handlers::delete_table)
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::require_auth
        ));

    // Dynamic REST API routes for user-defined tables
    let dynamic_api = Router::new()
        .route("/{project_slug}/{table_name}",
            get(handlers::list_table_rows)
                .post(handlers::create_table_row)
        )
        .route("/{project_slug}/{table_name}/{id}",
            get(handlers::get_table_row)
                .put(handlers::update_table_row)
                .patch(handlers::patch_table_row)
                .delete(handlers::delete_table_row)
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::require_auth
        ));

    Router::new()
        // Health check
        .route("/health", get(health_check))
        // API routes
        .nest("/api", 
            Router::new()
                // Auth routes (public)
                .route("/auth/register", post(handlers::register))
                .route("/auth/login", post(handlers::login))
                .route("/auth/logout", post(handlers::logout))
                // Protected auth routes
                .route("/auth/me", get(handlers::get_current_user))
                // Project routes (protected)
                .nest("/projects", protected_projects)
                // Dynamic data API (protected)
                .nest("/data", dynamic_api)
        )
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}
