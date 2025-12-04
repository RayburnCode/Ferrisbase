<!-- @format -->

# Navbar with Authentication

## Overview

The new navbar implementation includes:

### ✅ **Authentication State Management**

- `AuthState` context provider in `main.rs`
- Persistent login state using localStorage
- User information stored (id, email, name, role)
- JWT token management

### ✅ **Different UI for Logged In vs Logged Out**

**When NOT logged in:**

- "Log in" button
- "Sign up" button
- Home, FAQ links only

**When logged in:**

- User avatar with dropdown
- User name/email display
- "Projects" link visible
- "+ New Project" button
- Logout option
- Settings (coming soon)

### ✅ **Modern Design**

- Clean white background with subtle shadow
- Ferrisbase logo with 🦀 emoji
- Responsive mobile menu
- Gradient user avatars
- Smooth transitions and hover states

## How to Test

### Simulate Login (Temporary - until backend is connected)

Add this to your home page or any component:

```rust
use crate::{AuthState, User};

let mut auth_state = use_context::<Signal<AuthState>>();

// Test login button
button {
    class: "px-4 py-2 bg-green-600 text-white rounded",
    onclick: move |_| {
        let test_user = User {
            id: "test-123".to_string(),
            email: "test@ferrisbase.com".to_string(),
            name: Some("Test User".to_string()),
            role: "user".to_string(),
        };
        auth_state.write().login(test_user, "fake-jwt-token".to_string());
    },
    "Test Login"
}

// Test logout button
button {
    class: "px-4 py-2 bg-red-600 text-white rounded",
    onclick: move |_| {
        auth_state.write().logout();
    },
    "Test Logout"
}
```

## Features

### Desktop Navigation

- Logo on left
- Nav links in center
- Auth buttons/user menu on right
- User dropdown with:
  - User info display
  - My Projects link
  - Settings link
  - Sign Out button

### Mobile Navigation

- Hamburger menu
- Collapsible menu with all links
- User info section at bottom
- Login/signup buttons when logged out
- User profile and logout when logged in

### State Persistence

- User data saved to localStorage on login
- Token saved to localStorage
- Automatically loads on page refresh
- Cleared on logout

## Next Steps

1. **Connect to Backend API**

   - Wire up login button to `/auth/login`
   - Wire up signup button to `/auth/register`
   - Store JWT token from response
   - Make authenticated requests with token

2. **Create Auth Modals**

   - Login modal component
   - Signup modal component
   - Password reset flow

3. **Protected Routes**

   - Add route guards for `/projects/*`
   - Redirect to login if not authenticated
   - Show loading state during auth check

4. **Token Refresh**
   - Implement token refresh logic
   - Handle token expiration
   - Auto-refresh before expiration

## File Structure

```
frontend/src/
├── auth.rs                    # NEW: Auth state & User types
├── main.rs                    # Updated: Auth context provider
└── components/layout/
    └── navbar.rs              # Completely rewritten
```

## Usage in Other Components

```rust
use crate::AuthState;

#[component]
pub fn MyComponent() -> Element {
    let auth_state = use_context::<Signal<AuthState>>();

    let is_logged_in = auth_state.read().is_authenticated();
    let user = auth_state.read().user.clone();

    rsx! {
        if is_logged_in {
            p { "Welcome, {user.name.unwrap_or(\"User\".to_string())}" }
        } else {
            p { "Please log in" }
        }
    }
}
```
