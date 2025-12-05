<!-- @format -->

# Dynamic REST API

Ferrisbase automatically generates REST API endpoints for any database table you create, similar to Firebase and Supabase.

## Overview

When you create a table in your project, you immediately get full CRUD (Create, Read, Update, Delete) API endpoints without writing any backend code.

## API Endpoints

All dynamic API endpoints follow this pattern:

```
/api/data/{project_slug}/{table_name}
```

### List All Rows

```http
GET /api/data/{project_slug}/{table_name}?limit=100&offset=0
```

**Query Parameters:**

- `limit` (optional): Number of rows to return (default: 100, max: 1000)
- `offset` (optional): Number of rows to skip (default: 0)

**Response:** `200 OK`

```json
[
  {
    "id": "uuid",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z",
    "column1": "value1",
    "column2": "value2"
  }
]
```

### Get Single Row

```http
GET /api/data/{project_slug}/{table_name}/{row_id}
```

**Response:** `200 OK`

```json
{
  "id": "uuid",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "column1": "value1",
  "column2": "value2"
}
```

**Error:** `404 Not Found` if row doesn't exist

### Create Row

```http
POST /api/data/{project_slug}/{table_name}
Content-Type: application/json

{
  "column1": "value1",
  "column2": "value2"
}
```

**Response:** `201 Created`

```json
{
  "id": "uuid",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "column1": "value1",
  "column2": "value2"
}
```

**Notes:**

- `id`, `created_at`, and `updated_at` are automatically generated
- Only include columns you want to set in the request body

### Update Row (Full Replace)

```http
PUT /api/data/{project_slug}/{table_name}/{row_id}
Content-Type: application/json

{
  "column1": "new_value1",
  "column2": "new_value2"
}
```

**Response:** `200 OK`

```json
{
  "id": "uuid",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z",
  "column1": "new_value1",
  "column2": "new_value2"
}
```

**Error:** `404 Not Found` if row doesn't exist

### Update Row (Partial)

```http
PATCH /api/data/{project_slug}/{table_name}/{row_id}
Content-Type: application/json

{
  "column1": "new_value1"
}
```

**Response:** `200 OK`

```json
{
  "id": "uuid",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z",
  "column1": "new_value1",
  "column2": "value2"
}
```

**Notes:**

- Only updates columns included in request body
- Other columns remain unchanged
- `updated_at` is always updated

### Delete Row

```http
DELETE /api/data/{project_slug}/{table_name}/{row_id}
```

**Response:** `204 No Content`

**Error:** `404 Not Found` if row doesn't exist

## Authentication

All dynamic API endpoints require authentication via JWT token:

```http
Authorization: Bearer <your_jwt_token>
```

The API automatically:

- Verifies the JWT token
- Checks that the user owns the project
- Ensures the table exists in the project
- Validates all column names against the table schema

## Security

- **Project Ownership**: You can only access tables in projects you own
- **Table Validation**: Table names are verified against `project_tables` metadata
- **Column Validation**: Only columns defined in `project_columns` can be accessed
- **SQL Injection Protection**: All inputs are validated and properly escaped
- **UUID Validation**: Row IDs must be valid UUIDs

## Example Usage

### JavaScript/TypeScript

```javascript
const API_BASE = "https://your-domain.com/api/data";
const PROJECT_SLUG = "my-project";
const TABLE_NAME = "users";
const TOKEN = "your_jwt_token";

// List all users
const response = await fetch(`${API_BASE}/${PROJECT_SLUG}/${TABLE_NAME}`, {
  headers: {
    Authorization: `Bearer ${TOKEN}`,
  },
});
const users = await response.json();

// Create a user
const newUser = await fetch(`${API_BASE}/${PROJECT_SLUG}/${TABLE_NAME}`, {
  method: "POST",
  headers: {
    Authorization: `Bearer ${TOKEN}`,
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    name: "John Doe",
    email: "john@example.com",
  }),
});

// Update a user
await fetch(`${API_BASE}/${PROJECT_SLUG}/${TABLE_NAME}/${userId}`, {
  method: "PATCH",
  headers: {
    Authorization: `Bearer ${TOKEN}`,
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    name: "Jane Doe",
  }),
});

// Delete a user
await fetch(`${API_BASE}/${PROJECT_SLUG}/${TABLE_NAME}/${userId}`, {
  method: "DELETE",
  headers: {
    Authorization: `Bearer ${TOKEN}`,
  },
});
```

### cURL

```bash
# List all rows
curl -H "Authorization: Bearer $TOKEN" \
  "https://your-domain.com/api/data/my-project/users"

# Create row
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}' \
  "https://your-domain.com/api/data/my-project/users"

# Update row
curl -X PATCH \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe"}' \
  "https://your-domain.com/api/data/my-project/users/uuid-here"

# Delete row
curl -X DELETE \
  -H "Authorization: Bearer $TOKEN" \
  "https://your-domain.com/api/data/my-project/users/uuid-here"
```

## Implementation Details

The dynamic API is implemented in:

- `backend/src/services/dynamic_api_service.rs` - Business logic for CRUD operations
- `backend/src/handlers/dynamic_api.rs` - HTTP request handlers
- `backend/src/routes.rs` - Route configuration

The system uses PostgreSQL's `row_to_json()` function to efficiently convert database rows to JSON without manual serialization.

## Limitations

1. **Auto-generated Columns**: You cannot manually set `id`, `created_at`, or `updated_at` - these are managed automatically
2. **Rate Limiting**: Not yet implemented (coming soon)
3. **Filtering**: Advanced query filtering beyond pagination is not yet supported
4. **Batch Operations**: No batch insert/update/delete endpoints yet

## Future Enhancements

- [ ] Advanced filtering and searching
- [ ] Sorting by any column
- [ ] Batch operations
- [ ] Rate limiting
- [ ] Webhooks for data changes
- [ ] Real-time subscriptions
- [ ] Field-level permissions
- [ ] Custom validation rules
