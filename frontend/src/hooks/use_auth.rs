use dioxus::prelude::*;
use shared::models::{AuthResponse, UserResponse};
use reqwest::Client;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;
use crate::config::endpoints;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct AuthContext { 
    pub is_authenticated: Signal<bool>,
    pub user: Signal<Option<UserResponse>>,
    pub token: Signal<Option<String>>,
    pub send_magic_link: Signal<Option<fn(String) -> Pin<Box<dyn Future<Output = Result<(), String>>>>>>,
    pub verify_magic_link: Signal<Option<fn(String) -> Pin<Box<dyn Future<Output = Result<UserResponse, String>>>>>>,
    pub logout: Signal<Option<fn() -> Pin<Box<dyn Future<Output = Result<(), String>>>>>>,
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
}

// Helper function to make authenticated requests
#[allow(dead_code)]
async fn make_authenticated_request(token: &str, url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    client.get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
}

// Helper function to make authenticated POST requests
#[allow(dead_code)]
async fn make_authenticated_post_request(token: &str, url: &str, body: serde_json::Value) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let body_string = serde_json::to_string(&body).unwrap();
    client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(body_string)
        .send()
        .await
}

#[component]
pub fn AuthProvider(children: Element) -> Element {
    let is_authenticated = use_signal(|| false);
    let user = use_signal(|| None::<UserResponse>);
    let token = use_signal(|| None::<String>);

    // Create the auth context with signals
    let auth_context = AuthContext {
        is_authenticated,
        user,
        token,
        send_magic_link: use_signal(|| None),
        verify_magic_link: use_signal(|| None),
        logout: use_signal(|| None),
    };

    use_context_provider(|| auth_context);

    // Check auth status on mount using stored token
    use_effect(move || {
        let mut auth_context_check = auth_context;
        spawn(async move {
            // Clone the token to avoid borrow checker issues
            let stored_token = auth_context_check.token.read().clone();
            
            // Only check if we have a stored token
            if let Some(token) = stored_token {
                log::debug!("🔍 Checking authentication status using stored token");
                
                match make_authenticated_request(&token, &endpoints::me()).await {
                    Ok(response) => {
                        let status_code = response.status();
                        log::debug!("📡 Auth check response status: {}", status_code);
                        
                        if response.status().is_success() {
                            match response.json::<UserResponse>().await {
                                Ok(user_data) => {
                                    log::info!("✅ Found valid session for user: {} (ID: {})", 
                                        user_data.email, user_data.id);
                                    auth_context_check.is_authenticated.set(true);
                                    auth_context_check.user.set(Some(user_data));
                                }
                                Err(e) => {
                                    log::debug!("❌ Failed to parse auth check response: {}", e);
                                    // Clear invalid token
                                    auth_context_check.token.set(None);
                                    auth_context_check.is_authenticated.set(false);
                                    auth_context_check.user.set(None);
                                }
                            }
                        } else {
                            log::debug!("❌ Auth check failed with status: {}", status_code);
                            // Clear invalid token
                            auth_context_check.token.set(None);
                            auth_context_check.is_authenticated.set(false);
                            auth_context_check.user.set(None);
                        }
                    }
                    Err(e) => {
                        log::error!("❌ Auth check request failed: {}", e);
                        // Clear invalid token on network error
                        auth_context_check.token.set(None);
                        auth_context_check.is_authenticated.set(false);
                        auth_context_check.user.set(None);
                    }
                }
            } else {
                log::debug!("🔍 No stored token found, user not authenticated");
            }
        });
    });

    rsx! {
        {children}
    }
}



pub fn use_password_login() -> impl Fn(String, String) -> Pin<Box<dyn Future<Output = Result<UserResponse, String>>>> {
    let auth_state = use_auth();
    
    move |email: String, password: String| {
        let mut auth_state = auth_state;
        Box::pin(async move {
            log::info!("🔐 Starting login attempt for email: {}", email);
            log::debug!("📊 Login request details - Email length: {}, Has password: {}", 
                email.len(), !password.is_empty());
            
            let client = Client::new();
            let payload = json!({ "email": email, "password": password });
            let body_string = serde_json::to_string(&payload).unwrap();
            
            log::debug!("🌐 Making POST request to: {}", endpoints::login());
            
            let response = client
                .post("endpoints::login()")
                .header("Content-Type", "application/json")
                .body(body_string)
                .send()
                .await;
            
            match response {
                Ok(response) => {
                    let status_code = response.status();
                    log::info!("📡 Received response with status: {} ({})", 
                        status_code.as_u16(), status_code.canonical_reason().unwrap_or("Unknown"));
                    log::debug!("⏱️ Login request completed");
                    
                    if response.status().is_success() {
                        log::debug!("✅ Login request successful, parsing response...");
                        
                        // Parse AuthResponse from backend
                        match response.json::<AuthResponse>().await {
                            Ok(auth_response) => {
                                log::info!("🎉 Login successful for user: {} (ID: {})", 
                                    auth_response.user.email, auth_response.user.id);
                                log::debug!("👤 User data loaded: name={:?}", 
                                    auth_response.user.name);
                                log::debug!("🔑 JWT token received and will be stored");
                                
                                // Update auth state with user and token
                                auth_state.is_authenticated.set(true);
                                auth_state.user.set(Some(auth_response.user.clone()));
                                auth_state.token.set(Some(auth_response.token.clone()));
                                
                                // Test with Authorization header instead of cookies
                                log::info!("🧪 Testing Authorization header authentication");
                                let test_client = Client::new();
                                match test_client.get(&endpoints::me())
                                    .header("Content-Type", "application/json")
                                    .header("Authorization", format!("Bearer {}", auth_response.token))
                                    .send().await {
                                    Ok(test_response) => {
                                        log::info!("🧪 Auth test response status: {}", test_response.status());
                                        if test_response.status().is_success() {
                                            log::info!("✅ Authorization header authentication is working!");
                                        } else {
                                            log::error!("❌ Authorization header authentication failed");
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("🧪 Auth test failed: {}", e);
                                    }
                                }
                                
                                Ok(auth_response.user)
                            }
                            Err(e) => {
                                log::error!("❌ Failed to parse login response: {}", e);
                                log::debug!("🔍 JSON parsing error details: {:?}", e);
                                Err(format!("Failed to parse response: {}", e))
                            }
                        }
                    } else {
                        // Enhanced error handling with detailed logging
                        log::warn!("⚠️ Login failed with HTTP status: {}", status_code);
                        
                        let error_body = match response.text().await {
                            Ok(body) => {
                                log::debug!("📝 Error response body: {}", body);
                                body
                            }
                            Err(e) => {
                                log::warn!("🔍 Could not read error response body: {}", e);
                                "Unknown error".to_string()
                            }
                        };
                        
                        let error_message = match status_code.as_u16() {
                            400 => {
                                log::warn!("🚫 Bad request - likely invalid email/password format");
                                "Invalid email or password format".to_string()
                            }
                            401 => {
                                log::warn!("🔒 Unauthorized - invalid credentials for email: {}", email);
                                "Invalid email or password".to_string()
                            }
                            403 => {
                                log::warn!("🚷 Forbidden - account access denied for email: {}", email);
                                "Account access denied".to_string()
                            }
                            404 => {
                                log::warn!("👤 Not found - account doesn't exist for email: {}", email);
                                "Account not found".to_string()
                            }
                            429 => {
                                log::warn!("🕐 Rate limited - too many attempts for email: {}", email);
                                "Too many login attempts. Please try again later".to_string()
                            }
                            500..=599 => {
                                log::error!("🔥 Server error {} during login for email: {}", status_code, email);
                                log::error!("🔍 Server error details: {}", error_body);
                                "Server error. Please try again later".to_string()
                            }
                            _ => {
                                log::error!("❓ Unexpected status code {} for email: {}", status_code, email);
                                format!("Login failed: {}", error_body)
                            }
                        };
                        
                        Err(error_message)
                    }
                }
                Err(e) => {
                    log::error!("🌐 Network error during login for email: {} - Error: {}", email, e);
                    log::debug!("🔍 Network error details: {:?}", e);
                    Err(format!("Network error: {}", e))
                }
            }
        })
    }
}

#[allow(dead_code)]
pub fn use_register_user() -> impl Fn(String, String, Option<String>) -> Pin<Box<dyn Future<Output = Result<UserResponse, String>>>> {
    let auth_state = use_auth();
    
    move |email: String, password: String, name: Option<String>| {
        let mut auth_state = auth_state;
        Box::pin(async move {
            log::info!("📝 Starting registration attempt for email: {}, name: {:?}", email, name);
            log::debug!("📊 Registration request details - Email length: {}, Name: {:?}, Has password: {}", 
                email.len(), name, !password.is_empty());
            let client = Client::new();
            let payload = json!({ "email": email, "password": password, "name": name });
            let body_string = serde_json::to_string(&payload).unwrap();
            
            log::debug!("🌐 Making POST request to: {}", endpoints::register());
            
            let response = client
                .post("endpoints::register()")
                .header("Content-Type", "application/json")
                .body(body_string)
                .send()
                .await;
            
            match response {
                Ok(response) => {
                    let status_code = response.status();
                    log::info!("📡 Received registration response with status: {} ({})", 
                        status_code.as_u16(), status_code.canonical_reason().unwrap_or("Unknown"));
                    log::debug!("⏱️ Registration request completed");
                    
                    if status_code.is_success() {
                        log::debug!("✅ Registration request successful, parsing response...");
                        
                        let response_json = response.json::<AuthResponse>().await;
                        match response_json {
                            Ok(auth_response) => {
                                log::info!("🎉 Registration successful for user: {} (ID: {})", 
                                    auth_response.user.email, auth_response.user.id);
                                log::debug!("👤 New user data: name={:?}", 
                                    auth_response.user.name);
                                
                                // Update auth state after successful registration
                                auth_state.is_authenticated.set(true);
                                auth_state.user.set(Some(auth_response.user.clone()));
                                auth_state.token.set(Some(auth_response.token.clone()));
                                
                                Ok(auth_response.user)
                            }
                            Err(e) => {
                                log::error!("❌ Failed to parse successful registration response: {}", e);
                                log::debug!("🔍 JSON parsing error details: {:?}", e);
                                Err(format!("Failed to parse response: {}", e))
                            }
                        }
                    } else {
                        log::warn!("⚠️ Registration failed with HTTP status: {}", status_code);
                        
                        let error_body = match response.text().await {
                            Ok(body) => {
                                log::debug!("📝 Registration error response body: {}", body);
                                body
                            }
                            Err(e) => {
                                log::warn!("🔍 Could not read registration error response body: {}", e);
                                "Unknown error".to_string()
                            }
                        };
                        
                        let error_message = match status_code.as_u16() {
                            400 => {
                                log::warn!("🚫 Bad request during registration - likely validation error");
                                format!("Registration failed: {}", error_body)
                            }
                            409 => {
                                log::warn!("👥 Conflict - user already exists with email: {}", email);
                                "User with this email already exists".to_string()
                            }
                            422 => {
                                log::warn!("📋 Validation error during registration");
                                format!("Validation error: {}", error_body)
                            }
                            500..=599 => {
                                log::error!("🔥 Server error {} during registration", status_code);
                                log::error!("🔍 Server error details: {}", error_body);
                                "Server error. Please try again later".to_string()
                            }
                            _ => {
                                log::error!("❓ Unexpected status code {} during registration", status_code);
                                format!("Registration failed: {}", error_body)
                            }
                        };
                        
                        Err(error_message)
                    }
                }
                Err(e) => {
                    log::error!("🌐 Network error during registration for email: {} - Error: {}", email, e);
                    log::debug!("🔍 Network error details: {:?}", e);
                    Err(format!("Network error: {}", e))
                }
            }
        })
    }
}

#[allow(dead_code)]
pub fn use_logout() -> impl Fn() -> Pin<Box<dyn Future<Output = Result<(), String>>>> {
    let auth_state = use_auth();
    
    move || {
        let mut auth_state = auth_state;
        Box::pin(async move {
            log::info!("🚪 Starting logout request");
            
            // Get the token before clearing it
            let token = auth_state.token.read().clone();
            
            let client = Client::new();
            
            let mut request_builder = client
                .post("endpoints::logout()")
                .header("Content-Type", "application/json");
            
            // Add Authorization header if token exists
            if let Some(ref token_value) = token {
                request_builder = request_builder.header("Authorization", format!("Bearer {}", token_value));
            }
            
            let response = request_builder.send().await;
            
            // Clear auth state regardless of server response
            log::debug!("🔄 Clearing local authentication state");
            auth_state.is_authenticated.set(false);
            auth_state.user.set(None);
            auth_state.token.set(None); // Also clear the token
            
            match response {
                Ok(response) => {
                    let status_code = response.status();
                    log::info!("📡 Logout response status: {} ({})", 
                        status_code.as_u16(), status_code.canonical_reason().unwrap_or("Unknown"));
                    log::debug!("⏱️ Logout request completed");
                    
                    if response.status().is_success() {
                        log::info!("✅ Logout successful - user signed out");
                        Ok(())
                    } else {
                        log::warn!("⚠️ Logout request failed with status: {} (local state still cleared)", status_code);
                        // Still return success since we cleared the local state
                        Ok(())
                    }
                }
                Err(e) => {
                    log::warn!("🌐 Network error during logout: {} (local state still cleared)", e);
                    log::debug!("🔍 Logout network error details: {:?}", e);
                    // Still return success since we cleared the local state
                    Ok(())
                }
            }
        })
    }
}