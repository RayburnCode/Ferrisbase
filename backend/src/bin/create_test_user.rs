use bcrypt::{hash, DEFAULT_COST};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/ferrisbase".to_string());
    
    // Hash the password "test123"
    let password = "test123";
    let password_hash = hash(password, DEFAULT_COST)?;
    
    // Connect to database using sqlx
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Insert test user
    let result: (uuid::Uuid, String) = sqlx::query_as(
        r#"
        INSERT INTO users (email, password_hash, name, role, email_verified, is_active)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (email) DO UPDATE 
        SET password_hash = EXCLUDED.password_hash,
            name = EXCLUDED.name,
            role = EXCLUDED.role
        RETURNING id, email
        "#,
    )
    .bind("test@ferrisbase.com")
    .bind(&password_hash)
    .bind("Test User")
    .bind("user")
    .bind(true)
    .bind(true)
    .fetch_one(&pool)
    .await?;
    
    println!("✅ Test user created successfully!");
    println!("\n📧 Email: test@ferrisbase.com");
    println!("🔑 Password: test123");
    println!("👤 Name: Test User");
    println!("🔐 Role: user");
    println!("🆔 ID: {}", result.0);
    println!("\n💡 You can now log in at http://localhost:8080/login");
    
    // Create an admin test user too
    let admin_password_hash = hash("admin123", DEFAULT_COST)?;
    let admin_result: (uuid::Uuid, String) = sqlx::query_as(
        r#"
        INSERT INTO users (email, password_hash, name, role, email_verified, is_active)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (email) DO UPDATE 
        SET password_hash = EXCLUDED.password_hash,
            name = EXCLUDED.name,
            role = EXCLUDED.role
        RETURNING id, email
        "#,
    )
    .bind("admin@ferrisbase.com")
    .bind(&admin_password_hash)
    .bind("Admin User")
    .bind("admin")
    .bind(true)
    .bind(true)
    .fetch_one(&pool)
    .await?;
    
    println!("\n✅ Admin user created successfully!");
    println!("\n📧 Email: admin@ferrisbase.com");
    println!("🔑 Password: admin123");
    println!("👤 Name: Admin User");
    println!("🔐 Role: admin");
    println!("🆔 ID: {}", admin_result.0);
    
    pool.close().await;
    
    Ok(())
}
