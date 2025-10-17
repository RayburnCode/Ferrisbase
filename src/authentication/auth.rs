// Example user model
#[derive(sqlx::FromRow, Serialize, Deserialize)]
struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

// Auth middleware
struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

async fn auth_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Extract and validate JWT token
    // Add user to request extensions
    Ok(next.run(request).await)
}