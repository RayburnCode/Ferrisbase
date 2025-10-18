// backend/src/main.rs - Manual TLS approach
use axum::http::Method;
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use backend::config::{AppState, setup_database};
use backend::routes::api;
use std::env;
use migration::{Migrator, MigratorTrait};

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Create database connection
    let db_conn: sea_orm::DatabaseConnection = setup_database().await
        .expect("Failed to connect to database");

    // Run migrations automatically on startup
    println!("🔄 Running database migrations...");
    Migrator::up(&db_conn, None).await
        .expect("Failed to run migrations");
    println!("✅ Database migrations completed");

    // Create app state
    let app_state = AppState { db_conn };

    let app = api::routes()
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "https://rayburnlp.com".parse().unwrap(),
                    "http://127.0.0.1:8082".parse().unwrap(),  // For development
                    "http://localhost:8082".parse().unwrap(),   // Alternative dev
                ])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
                .allow_headers([
                    "Content-Type".parse().unwrap(),
                    "Authorization".parse().unwrap(),
                    "User-Agent".parse().unwrap(),
                    "Accept".parse().unwrap(),
                    "Accept-Language".parse().unwrap(),
                    "Content-Language".parse().unwrap(),
                    "X-Requested-With".parse().unwrap(),
                ])
                .expose_headers([
                    "Content-Length".parse().unwrap(),
                    "X-Kuma-Revision".parse().unwrap(),
                ])
                .allow_credentials(true),
        );


    // Get host and port from environment variables or use defaults
    let host = env::var("BACKEND_HOST")
        .unwrap_or_else(|_| {
            println!("⚠️  BACKEND_HOST not set, using default: 0.0.0.0");
            "0.0.0.0".to_string()
        });
    
    let port = env::var("BACKEND_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| {
            println!("⚠️  BACKEND_PORT not set or invalid, using default: 8083");
            8083
        });

    // Validate host format
    if !host.chars().all(|c| c.is_ascii_digit() || c == '.') && host != "localhost" {
        println!("❌ Invalid BACKEND_HOST format: '{}'. Expected format: '0.0.0.0' or 'localhost'", host);
    }

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Failed to parse host:port into SocketAddr");
    
    println!("🚀 Backend server starting...");
    println!("📍 Listening on: {}:{}", host, port);
    println!("🌐 Access URL: http://{}:{}", 
        if host == "0.0.0.0" { "localhost" } else { &host }, 
        port
    );
    println!("📋 Environment: {}", env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()));
    
    // Debug environment variables (without sensitive data)
    println!("🔍 Configuration Debug:");
    println!("  - BACKEND_HOST: {}", host);
    println!("  - BACKEND_PORT: {}", port);
    println!("  - DATABASE_URL: {}", 
        if env::var("DATABASE_URL").is_ok() { "✅ Set" } else { "❌ Missing" }
    );
 
    axum_server::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}