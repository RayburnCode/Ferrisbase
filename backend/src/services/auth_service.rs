use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use database::entities::prelude::*;
use database::entities::users;
use shared::models::{AuthResponse, Claims, LoginRequest, RegisterRequest, UserResponse};

/// Hash a plain text password
pub fn hash_password(password: &str) -> AppResult<String> {
    hash(password, DEFAULT_COST).map_err(AppError::from)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    verify(password, hash).map_err(AppError::from)
}

/// Generate a JWT token for a user
pub fn generate_token(user_id: &str, email: &str, role: &str, jwt_secret: &str, expiration_hours: usize) -> AppResult<String> {
    let now = Utc::now();
    let exp = (now + chrono::Duration::hours(expiration_hours as i64)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(AppError::from)
}

/// Verify and decode a JWT token
pub fn verify_token(token: &str, jwt_secret: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(AppError::from)
}

/// Register a new user
pub async fn register_user(
    db: &DatabaseConnection,
    req: RegisterRequest,
    jwt_secret: &str,
    expiration_hours: usize,
) -> AppResult<AuthResponse> {
    // Validate email format
    if !req.email.contains('@') {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }

    // Validate password strength (minimum 8 characters)
    if req.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    // Check if user already exists
    let existing_user = Users::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::BadRequest("User with this email already exists".to_string()));
    }

    // Hash the password
    let password_hash = hash_password(&req.password)?;

    // Create new user
    let user_id = Uuid::new_v4();
    let new_user = users::ActiveModel {
        id: Set(user_id),
        email: Set(req.email.clone()),
        password_hash: Set(password_hash),
        name: Set(req.name.clone()),
        role: Set("user".to_string()),
        email_verified: Set(false),
        is_active: Set(true),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    // Generate JWT token
    let token = generate_token(
        &user.id.to_string(),
        &user.email,
        &user.role,
        jwt_secret,
        expiration_hours,
    )?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            role: user.role,
            email_verified: user.email_verified,
            created_at: user.created_at.map(|t| t.to_string()).unwrap_or_default(),
        },
    })
}

/// Login a user
pub async fn login_user(
    db: &DatabaseConnection,
    req: LoginRequest,
    jwt_secret: &str,
    expiration_hours: usize,
) -> AppResult<AuthResponse> {
    // Find user by email
    let user = Users::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    // Check if user is active
    if !user.is_active {
        return Err(AppError::Unauthorized("Account is disabled".to_string()));
    }

    // Verify password
    let password_valid = verify_password(&req.password, &user.password_hash)?;
    if !password_valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    // Generate JWT token
    let token = generate_token(
        &user.id.to_string(),
        &user.email,
        &user.role,
        jwt_secret,
        expiration_hours,
    )?;

    Ok(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            role: user.role,
            email_verified: user.email_verified,
            created_at: user.created_at.map(|t| t.to_string()).unwrap_or_default(),
        },
    })
}

/// Get user by ID
pub async fn get_user_by_id(db: &DatabaseConnection, user_id: &str) -> AppResult<UserResponse> {
    let user_uuid = Uuid::parse_str(user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID format".to_string()))?;

    let user = Users::find_by_id(user_uuid)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        role: user.role,
        email_verified: user.email_verified,
        created_at: user.created_at.map(|t| t.to_string()).unwrap_or_default(),
    })
}
