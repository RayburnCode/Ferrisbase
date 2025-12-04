use serde::{Deserialize, Serialize};

/// User information stored in auth state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

/// Authentication state 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthState {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            user: None,
            token: None,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.is_some() && self.token.is_some()
    }

    pub fn login(&mut self, user: User, token: String) {
        // Store in localStorage
        if let Ok(user_json) = serde_json::to_string(&user) {
            let _ = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
                .map(|storage| {
                    let _ = storage.set_item("user", &user_json);
                    let _ = storage.set_item("token", &token);
                });
        }
        self.user = Some(user);
        self.token = Some(token);
    }

    pub fn logout(&mut self) {
        self.user = None;
        self.token = None;
        // Clear localStorage
        let _ = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .map(|storage| {
                let _ = storage.remove_item("user");
                let _ = storage.remove_item("token");
            });
    }

    pub fn load_from_storage() -> Self {
        let window = web_sys::window();
        if let Some(storage) = window.and_then(|w| w.local_storage().ok().flatten()) {
            let user = storage
                .get_item("user")
                .ok()
                .flatten()
                .and_then(|json| serde_json::from_str(&json).ok());
            let token = storage.get_item("token").ok().flatten();
            
            if user.is_some() && token.is_some() {
                return Self { user, token };
            } 
        }
        Self::new()
    }
}

impl Default for AuthState {
    fn default() -> Self {
        Self::new()
    }
}
