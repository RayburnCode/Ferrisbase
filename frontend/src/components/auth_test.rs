use dioxus::prelude::*;
use crate::{AuthState, User};

/// A temporary component to test authentication functionality
/// Add this to your home page to test login/logout
#[component]
pub fn AuthTestButtons() -> Element {
    let mut auth_state = use_context::<Signal<AuthState>>();
    
    rsx! {
        div {
            class: "fixed bottom-4 right-4 bg-white p-4 rounded-lg shadow-lg border border-gray-200 space-y-2 z-50",
            
            p {
                class: "text-sm font-semibold text-gray-700 mb-2",
                "🧪 Auth Test Controls"
            }
            
            // Test Login
            button {
                class: "w-full px-4 py-2 bg-green-600 hover:bg-green-700 text-white text-sm font-medium rounded transition",
                onclick: move |_| {
                    let test_user = User {
                        id: "test-user-123".to_string(),
                        email: "demo@ferrisbase.com".to_string(),
                        name: Some("Demo User".to_string()),
                        role: "user".to_string(),
                    };
                    auth_state.write().login(test_user, "fake-jwt-token-12345".to_string());
                },
                "✅ Test Login"
            }
            
            // Test Logout
            button {
                class: "w-full px-4 py-2 bg-red-600 hover:bg-red-700 text-white text-sm font-medium rounded transition",
                onclick: move |_| {
                    auth_state.write().logout();
                },
                "❌ Test Logout"
            }
            
            // Status
            div {
                class: "pt-2 border-t border-gray-200",
                p {
                    class: "text-xs text-gray-600",
                    if auth_state.read().is_authenticated() {
                        "Status: Logged In ✓"
                    } else {
                        "Status: Logged Out"
                    }
                }
            }
        }
    }
}
