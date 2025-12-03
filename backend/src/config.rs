use std::sync::Arc;
use sea_orm::DatabaseConnection;

/// Application configuration loaded from environment variables
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: usize,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set".to_string())?,
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            jwt_expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .map_err(|_| "JWT_EXPIRATION_HOURS must be a valid number".to_string())?,
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| "SERVER_PORT must be a valid number".to_string())?,
        })
    }
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: Config) -> Self {
        Self {
            db: Arc::new(db),
            config: Arc::new(config),
        }
    }
}
