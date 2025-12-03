#!/bin/bash

# Ferrisbase Database Setup Script

set -e

echo "🦀 Ferrisbase Database Setup"
echo "=============================="

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "⚠️  DATABASE_URL not set. Using default..."
    DB_URL="postgresql://localhost/ferrisbase"
else
    DB_URL="$DATABASE_URL"
fi

# Extract database name from URL
DB_NAME=$(echo "$DB_URL" | sed -n 's|.*/\([^?]*\).*|\1|p')
DB_HOST=$(echo "$DB_URL" | sed -n 's|.*://\([^/]*\)/.*|\1|p')

echo "📍 Database: $DB_NAME"
echo "🖥️  Host: $DB_HOST"

# Create database if it doesn't exist
echo ""
echo "Creating database if it doesn't exist..."
psql -h localhost -d postgres -c "CREATE DATABASE $DB_NAME;" 2>/dev/null || echo "Database $DB_NAME already exists"

# Run migrations
echo ""
echo "Running migrations..."
cd "$(dirname "$0")"
cargo run --manifest-path database/migration/Cargo.toml

# Generate entities
echo ""
echo "Generating SeaORM entities..."
sea-orm-cli generate entity -u "$DB_URL" -o database/src/entities --with-serde both

echo ""
echo "✅ Database setup complete!"
echo ""
echo "Next steps:"
echo "1. Copy .env.example to .env and update with your values"
echo "2. Run 'cargo run --bin backend' to start the server"
