<!-- @format -->

# Backend Authentication - Quick Reference

## What Was Created

✅ **Database Layer**

- `m002_create_users_table.rs` - Users table migration with indexes
- Users entity (will be auto-generated)

✅ **Shared Models** (`shared/src/models/auth.rs`)

- `RegisterRequest` - User registration payload
- `LoginRequest` - User login payload
- `AuthResponse` - JWT token + user info
- `UserResponse` - User data (no password)
- `Claims` - JWT claims structure

✅ **Backend Services** (`backend/src/services/auth_service.rs`)

- `hash_password()` - Bcrypt password hashing
- `verify_password()` - Password verification
- `generate_token()` - JWT token generation
- `verify_token()` - JWT token validation
- `register_user()` - Complete registration flow
- `login_user()` - Complete login flow
- `get_user_by_id()` - Fetch user by ID

✅ **Backend Handlers** (`backend/src/handlers/auth.rs`)

- `POST /auth/register` - Register endpoint
- `POST /auth/login` - Login endpoint
- `GET /auth/me` - Get current user (protected)
- `POST /auth/logout` - Logout endpoint

✅ **Middleware** (`backend/src/middleware/auth.rs`)

- `AuthUser` extractor - Validates JWT from headers
- `require_auth` - Middleware for protected routes
- `require_admin` - Middleware for admin-only routes

✅ **Infrastructure**

- `config.rs` - Config & AppState
- `error.rs` - Unified error handling
- `routes.rs` - Router setup with CORS
- `main.rs` - Server entry point

✅ **Developer Tools**

- `.env.example` - Environment template
- `setup-db.sh` - Database setup script
- `Justfile` - Common commands
- `AUTHENTICATION_SETUP.md` - Full documentation

## Quick Start Commands

```bash
# 1. Setup environment
cp .env.example .env
# Edit .env with your PostgreSQL credentials

# 2. Setup database
./setup-db.sh

# 3. Run server
cargo run --bin backend

# 4. Test it
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123","name":"Test"}'
```

## Architecture

```
┌─────────────┐
│   Frontend  │
│   (Dioxus)  │
└──────┬──────┘
       │ HTTP + JWT
       ▼
┌─────────────────────────────┐
│   Backend (Axum)            │
│                             │
│  ┌──────────────────────┐  │
│  │ Routes               │  │
│  │ /auth/register       │  │
│  │ /auth/login          │  │
│  │ /auth/me (protected) │  │
│  └──────────┬───────────┘  │
│             │              │
│  ┌──────────▼───────────┐  │
│  │ Handlers             │  │
│  │ - Extract request    │  │
│  │ - Call services      │  │
│  │ - Return response    │  │
│  └──────────┬───────────┘  │
│             │              │
│  ┌──────────▼───────────┐  │
│  │ Services             │  │
│  │ - Business logic     │  │
│  │ - Password hashing   │  │
│  │ - JWT generation     │  │
│  │ - DB operations      │  │
│  └──────────┬───────────┘  │
│             │              │
└─────────────┼──────────────┘
              │
              ▼
      ┌──────────────┐
      │  PostgreSQL  │
      │  + SeaORM    │
      └──────────────┘
```

## Security Features

🔐 **Password Security**

- Bcrypt hashing (cost 12)
- Minimum 8 character requirement
- No plaintext storage

🔑 **JWT Tokens**

- HS256 algorithm
- Configurable expiration
- Contains user ID, email, role
- Stateless authentication

✅ **Validation**

- Email format validation
- Password strength check
- Active user verification
- Token expiration check

📊 **Database**

- Indexed email lookups
- UUID primary keys
- Timestamps for audit
- Unique email constraint

## Next Steps for Production

1. **Add Email Verification**
2. **Implement Password Reset**
3. **Add Refresh Tokens**
4. **Enable Rate Limiting**
5. **Add 2FA Support**
6. **Setup Session Management**
7. **Implement OAuth (Google, GitHub)**
8. **Add Audit Logging**
9. **Setup Token Blacklisting**
10. **Configure HTTPS/TLS**

## Integration with Frontend

```rust
// In your Dioxus frontend

// 1. Register
async fn register(email: String, password: String) -> Result<AuthResponse> {
    let response = reqwest::Client::new()
        .post("http://localhost:8080/auth/register")
        .json(&RegisterRequest { email, password, name: None })
        .send()
        .await?;
    response.json().await
}

// 2. Store token
fn store_token(token: String) {
    // Use web-sys localStorage or similar
    localStorage.set("jwt_token", token);
}

// 3. Make authenticated requests
async fn get_current_user(token: String) -> Result<UserResponse> {
    let response = reqwest::Client::new()
        .get("http://localhost:8080/auth/me")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    response.json().await
}
```

## Common Issues & Solutions

**"Database connection failed"**
→ Check PostgreSQL is running: `pg_isready`

**"JWT verification failed"**
→ Ensure JWT_SECRET matches between registration and validation

**"User not found"**
→ Run migrations: `cd database/migration && cargo run`

**"Password too short"**
→ Minimum 8 characters required

**"Email already exists"**
→ Each email can only be registered once

## Files to Review

📖 **Full Documentation**: `AUTHENTICATION_SETUP.md`
🔧 **Environment Setup**: `.env.example`
📝 **Quick Commands**: `Justfile`
🗄️ **Database Migration**: `database/migration/src/m002_create_users_table.rs`
🎯 **Service Logic**: `backend/src/services/auth_service.rs`
🛣️ **Routes**: `backend/src/routes.rs`
