<!-- @format -->

# Backend Authentication Setup Guide

## 🎯 Overview

Complete authentication system for Ferrisbase with JWT tokens, password hashing, and SeaORM integration.

## 📁 Project Structure

```
backend/
├── src/
│   ├── main.rs              # Server entry point
│   ├── lib.rs               # Module exports
│   ├── config.rs            # Config & AppState
│   ├── error.rs             # Error handling
│   ├── routes.rs            # Route definitions
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── auth.rs          # Auth endpoints
│   ├── services/
│   │   ├── mod.rs
│   │   └── auth_service.rs  # Business logic
│   └── middleware/
│       ├── mod.rs
│       └── auth.rs          # JWT validation
│
database/
├── src/
│   ├── lib.rs
│   └── entities/            # Generated SeaORM entities
│       ├── mod.rs
│       ├── users.rs
│       └── prelude.rs
└── migration/
    └── src/
        ├── m001_create_contact_form.rs
        └── m002_create_users_table.rs  # NEW: Users table
│
shared/
└── src/
    └── models/
        └── auth.rs          # Shared types (requests, responses)
```

## 🚀 Quick Start

### 1. Prerequisites

- PostgreSQL installed and running
- Rust 1.75+ installed
- `sea-orm-cli` installed (run `cargo install sea-orm-cli`)

### 2. Setup Environment

```bash
# Copy example env file
cp .env.example .env

# Edit .env with your PostgreSQL credentials
# DATABASE_URL=postgresql://username:password@localhost:5432/ferrisbase
```

### 3. Setup Database

```bash
# Option A: Use the setup script
./setup-db.sh

# Option B: Manual setup
# Create database
createdb ferrisbase

# Run migrations
cd database/migration
cargo run

# Generate entities
sea-orm-cli generate entity \
  -u postgresql://localhost/ferrisbase \
  -o ../src/entities \
  --with-serde both
```

### 4. Run the Server

```bash
cargo run --bin backend
```

Server will start on `http://127.0.0.1:8080`

## 📡 API Endpoints

### Public Endpoints (No Auth Required)

#### Register User

```bash
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure-password",
  "name": "John Doe"  # optional
}

# Response
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "user",
    "email_verified": false,
    "created_at": "2025-12-03T..."
  }
}
```

#### Login

```bash
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure-password"
}

# Response: Same as register
```

#### Logout

```bash
POST /auth/logout

# Response
{
  "message": "Logged out successfully"
}
```

### Protected Endpoints (Require JWT)

#### Get Current User

```bash
GET /auth/me
Authorization: Bearer <jwt-token>

# Response
{
  "id": "uuid",
  "email": "user@example.com",
  "name": "John Doe",
  "role": "user",
  "email_verified": false,
  "created_at": "2025-12-03T..."
}
```

## 🔑 Authentication Flow

### Frontend Integration

```typescript
// Register
const response = await fetch("http://localhost:8080/auth/register", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    email: "user@example.com",
    password: "password123",
    name: "John Doe",
  }),
});

const { token, user } = await response.json();
localStorage.setItem("jwt_token", token);

// Make authenticated requests
const userResponse = await fetch("http://localhost:8080/auth/me", {
  headers: {
    Authorization: `Bearer ${localStorage.getItem("jwt_token")}`,
  },
});
```

## 🗄️ Database Schema

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    name VARCHAR,
    role VARCHAR DEFAULT 'user',
    email_verified BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
```

## 🔐 Security Features

- **Password Hashing**: Bcrypt with cost factor 12
- **JWT Tokens**: HS256 algorithm with configurable expiration
- **Email Validation**: Basic format checking
- **Password Requirements**: Minimum 8 characters
- **Active User Check**: Inactive users cannot login
- **Index on Email**: Fast user lookups

## 🛠️ Key Components

### AppState

Shared application state containing:

- Database connection pool
- Configuration (JWT secret, server settings)

### AuthUser Extractor

Axum extractor that:

- Validates JWT tokens
- Extracts user claims
- Rejects invalid/expired tokens

### Error Handling

Unified error handling with:

- Typed errors (`AppError` enum)
- HTTP status code mapping
- JSON error responses

## 🧪 Testing with curl

```bash
# Register
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123","name":"Test User"}'

# Login
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'

# Get current user (replace TOKEN with actual JWT)
curl http://localhost:8080/auth/me \
  -H "Authorization: Bearer TOKEN"

# Health check
curl http://localhost:8080/health
```

## 📝 Next Steps

### Recommended Additions

1. **Email Verification**

   - Send verification emails
   - Verify email endpoint
   - Resend verification

2. **Password Reset**

   - Request reset token
   - Verify reset token
   - Update password

3. **Refresh Tokens**

   - Long-lived refresh tokens
   - Rotate access tokens
   - Revocation mechanism

4. **OAuth Integration**

   - Google OAuth
   - GitHub OAuth
   - OAuth callback handlers

5. **Rate Limiting**

   - Login attempt limits
   - Request rate limits
   - IP-based throttling

6. **2FA Support**

   - TOTP generation
   - QR code generation
   - Backup codes

7. **Session Management**
   - Active sessions tracking
   - Device fingerprinting
   - Logout all devices

### Adding Protected Routes

```rust
use crate::middleware::AuthUser;

// In your handler
pub async fn protected_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,  // Automatically validated
) -> AppResult<Json<YourResponse>> {
    // Access user ID via claims.sub
    // Access user role via claims.role
    Ok(Json(response))
}
```

### Admin-Only Routes

```rust
use axum::middleware;
use crate::middleware::require_admin;

Router::new()
    .route("/admin/users", get(list_all_users))
    .layer(middleware::from_fn_with_state(state.clone(), require_admin))
```

## 🐛 Troubleshooting

### "Database not found"

```bash
createdb ferrisbase
```

### "sea-orm-cli not found"

```bash
cargo install sea-orm-cli
```

### "JWT secret not set"

Add to `.env`:

```
JWT_SECRET=your-super-secret-key
```

### "Connection refused"

Check PostgreSQL is running:

```bash
pg_isready
```

## 📚 Resources

- [Axum Documentation](https://docs.rs/axum/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [JWT.io](https://jwt.io/)
- [Bcrypt](https://en.wikipedia.org/wiki/Bcrypt)

## 🤝 Contributing

When adding new features:

1. Create migration in `database/migration/src/`
2. Run migration and regenerate entities
3. Add service logic in `backend/src/services/`
4. Create handlers in `backend/src/handlers/`
5. Wire up routes in `backend/src/routes.rs`
