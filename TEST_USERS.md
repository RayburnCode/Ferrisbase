<!-- @format -->

# Test User Credentials

This file contains test user credentials for development and testing purposes.

## Test Users

### Regular User

- **Email:** `test@ferrisbase.com`
- **Password:** `test123`
- **Name:** Test User
- **Role:** user
- **Email Verified:** Yes
- **Active:** Yes

### Admin User

- **Email:** `admin@ferrisbase.com`
- **Password:** `admin123`
- **Name:** Admin User
- **Role:** admin
- **Email Verified:** Yes
- **Active:** Yes

## How to Log In

1. **Start the Backend Server:**

   ```bash
   cd backend
   cargo run
   ```

2. **Start the Frontend:**

   ```bash
   cd frontend
   dx serve
   ```

3. **Access the Application:**

   - Open your browser to `http://localhost:8080`
   - Click "Login" in the navbar
   - Use either test account above

4. **API Testing (Optional):**
   You can also test the API directly:

   ```bash
   # Login to get JWT token
   curl -X POST http://localhost:3000/api/auth/login \
     -H "Content-Type: application/json" \
     -d '{"email":"test@ferrisbase.com","password":"test123"}'

   # Use the token in subsequent requests
   curl http://localhost:3000/api/projects \
     -H "Authorization: Bearer YOUR_TOKEN_HERE"
   ```

## Recreating Test Users

If you need to reset or recreate the test users:

```bash
cd backend
cargo run --bin create_test_user
```

This script uses `ON CONFLICT` so it will update existing users with the same email.

## Security Note

⚠️ **These credentials are for DEVELOPMENT ONLY!**

- Never use these credentials in production
- Always use strong, unique passwords in production
- Consider implementing proper user registration flow for production

## Testing Different Scenarios

### Test Regular User Permissions

1. Log in as `test@ferrisbase.com`
2. Try creating projects
3. Try accessing project settings
4. Verify you can only see your own projects

### Test Admin User Permissions

1. Log in as `admin@ferrisbase.com`
2. Test admin-only features (when implemented)
3. Verify elevated permissions work correctly

### Test Authentication Flows

1. **Login:** Use credentials above
2. **Logout:** Click logout button (clears localStorage)
3. **Token Persistence:** Refresh page and verify you stay logged in
4. **Protected Routes:** Try accessing `/projects` without logging in
