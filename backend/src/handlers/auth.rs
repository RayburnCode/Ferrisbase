use axum::{extract::{State, Request}, Json};

use crate::config::AppState;
use crate::error::AppResult;
use crate::services;
use shared::models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};

/// POST /auth/register - Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let response = services::register_user(
        &state.db,
        req,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )
    .await?;

    Ok(Json(response))
}

/// POST /auth/login - Login a user
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let response = services::login_user(
        &state.db,
        req,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )
    .await?;

    Ok(Json(response))
}

/// GET /auth/me - Get current user info (requires auth middleware)
pub async fn get_current_user(
    State(state): State<AppState>,
    req: Request,
) -> AppResult<Json<UserResponse>> {
    // Get claims from request extensions (set by require_auth middleware)
    let claims = crate::middleware::get_claims(&req)
        .ok_or_else(|| crate::error::AppError::Unauthorized("No authentication found".to_string()))?;
    
    let user = services::get_user_by_id(&state.db, &claims.sub).await?;
    Ok(Json(user))
}

/// POST /auth/logout - Logout (client-side token removal)
/// This is mostly handled client-side by removing the JWT token
/// Server can optionally implement token blacklisting here
pub async fn logout() -> AppResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}
