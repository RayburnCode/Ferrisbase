# Justfile - Common development commands for Ferrisbase

# List all available commands
default:
    @just --list

# Install all dependencies
install:
    cargo install sea-orm-cli
    cargo build

# Setup database (create DB, run migrations, generate entities)
setup-db:
    ./setup-db.sh

# Run database migrations
migrate:
    cd database/migration && cargo run

# Generate SeaORM entities from database
generate-entities:
    sea-orm-cli generate entity \
        -u "${DATABASE_URL:-postgresql://localhost/ferrisbase}" \
        -o database/src/entities \
        --with-serde both

# Create a new migration
create-migration name:
    cd database/migration && sea-orm-cli migrate generate {{name}}

# Run the backend server
run-backend:
    cargo run --bin backend

# Run backend with auto-reload (requires cargo-watch)
watch-backend:
    cargo watch -x 'run --bin backend'

# Run the frontend
run-frontend:
    cd frontend && dx serve --hot-reload

# Run both backend and frontend (requires cargo-watch and dx)
dev:
    @echo "Starting backend and frontend in development mode..."
    @just watch-backend & just run-frontend

# Build everything
build:
    cargo build --workspace

# Build for production
build-release:
    cargo build --workspace --release

# Run all tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# Check code without building
check:
    cargo check --workspace

# Format code
fmt:
    cargo fmt --all

# Lint code
lint:
    cargo clippy --workspace -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Reset database (WARNING: Deletes all data!)
reset-db:
    @echo "⚠️  WARNING: This will delete all data!"
    @read -p "Are you sure? (y/N): " confirm && [ "$$confirm" = "y" ] || exit 1
    dropdb ferrisbase || true
    createdb ferrisbase
    just migrate
    just generate-entities
    @echo "✅ Database reset complete"

# Test authentication endpoints
test-auth:
    @echo "Testing registration..."
    curl -X POST http://localhost:8080/auth/register \
        -H "Content-Type: application/json" \
        -d '{"email":"test@example.com","password":"password123","name":"Test User"}' \
        | jq .
    @echo ""
    @echo "Testing login..."
    curl -X POST http://localhost:8080/auth/login \
        -H "Content-Type: application/json" \
        -d '{"email":"test@example.com","password":"password123"}' \
        | jq .

# Health check
health:
    curl http://localhost:8080/health

# View database schema
db-schema:
    psql "${DATABASE_URL:-postgresql://localhost/ferrisbase}" -c "\dt"

# Open database CLI
db-cli:
    psql "${DATABASE_URL:-postgresql://localhost/ferrisbase}"

# Install development tools
install-tools:
    cargo install cargo-watch
    cargo install just
    npm install -g dx-cli
