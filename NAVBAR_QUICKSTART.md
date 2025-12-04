<!-- @format -->

# Navbar Authentication - Quick Start

## ✅ What Was Created

1. **`frontend/src/auth.rs`** - Authentication state management

   - `AuthState` - Holds user info and JWT token
   - `User` - User data structure
   - localStorage persistence
   - Login/logout methods

2. **`frontend/src/main.rs`** - Updated with auth context

   - Provides `AuthState` globally via context
   - Loads auth from localStorage on startup

3. **`frontend/src/components/layout/navbar.rs`** - Completely redesigned

   - Modern Ferrisbase branding
   - Different UI for logged in vs logged out
   - User dropdown menu
   - Mobile responsive
   - Logout functionality

4. **`frontend/src/components/auth_test.rs`** - Test component
   - Quick test buttons for login/logout
   - Shows current auth status

## 🚀 How to Test Right Now

### Option 1: Add Test Buttons to Home Page

Edit `frontend/src/views/home.rs`:

```rust
use crate::components::AuthTestButtons;

// In your Home component, add:
AuthTestButtons {}
```

This will show a floating widget in the bottom-right with:

- ✅ Test Login button
- ❌ Test Logout button
- Status indicator

### Option 2: Use Browser Console

```javascript
// Login
localStorage.setItem(
  "user",
  JSON.stringify({
    id: "test-123",
    email: "test@ferrisbase.com",
    name: "Test User",
    role: "user",
  })
);
localStorage.setItem("token", "fake-jwt-token");
// Refresh page

// Logout
localStorage.clear();
// Refresh page
```

## 📋 What Changes When Logged In

### Navbar Shows:

- ✅ Your avatar (first letter of name/email)
- ✅ Your name
- ✅ "Projects" link
- ✅ "+ New Project" button
- ✅ User dropdown menu with:
  - My Projects
  - Settings
  - Sign Out

### Navbar Hides:

- ❌ "Log in" button
- ❌ "Sign up" button

## 🎨 Design Features

- **Clean & Modern**: White background, subtle shadows
- **Ferrisbase Branding**: 🦀 logo + brand name
- **Gradient Avatars**: Beautiful color gradients for user icons
- **Smooth Animations**: All interactions have smooth transitions
- **Fully Responsive**: Works perfectly on mobile
- **Persistent State**: Survives page refreshes

## 🔌 Next: Connect to Backend

When ready to connect to your backend API:

```rust
// In navbar.rs, replace the TODO comments:

// Login button onclick:
onclick: move |_| {
    spawn(async move {
        // Call your /auth/login endpoint
        let response = reqwest::Client::new()
            .post("http://localhost:8080/auth/login")
            .json(&LoginRequest { email, password })
            .send()
            .await?;

        let auth_response: AuthResponse = response.json().await?;

        // Store in state
        auth_state.write().login(
            User {
                id: auth_response.user.id,
                email: auth_response.user.email,
                name: auth_response.user.name,
                role: auth_response.user.role,
            },
            auth_response.token
        );
    });
},
```

## 📱 Mobile Features

- Hamburger menu (≡) on small screens
- Full-height slide-out menu
- User section at bottom
- All desktop features available
- Tap outside to close

## 💾 Data Persistence

Your auth state is automatically saved to browser localStorage:

- User info → `localStorage.user`
- JWT token → `localStorage.token`
- Loaded on page refresh
- Cleared on logout

## 🎯 Quick Test Steps

1. Run your frontend: `dx serve`
2. Add `<AuthTestButtons />` to home page (optional)
3. Click "✅ Test Login"
4. Watch navbar transform!
5. Click user dropdown to see menu
6. Try "Sign Out"
7. Watch navbar revert to logged-out state

The navbar will now show different content based on authentication! 🎉
