use dioxus::prelude::*;
use crate::{Route, AuthState, User};
use crate::components::ErrorNotification;
use shared::models::{AuthResponse, RegisterRequest};
use reqwest::Client;
 
const LOGO: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn RegisterForm() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let mut name = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);
    let mut auth_state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    // Check if already authenticated
    use_effect(move || {
        if auth_state.read().is_authenticated() {
            navigator.push(Route::Home {});
        }
    });

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let email_val = email.read().clone();
        let password_val = password.read().clone();
        let confirm_password_val = confirm_password.read().clone();
        let name_val = name.read().clone();

        // Validation
        if email_val.is_empty() || password_val.is_empty() {
            if email_val.is_empty() && password_val.is_empty() {
                error_message.set(Some("Email and password are required".to_string()));
            } else if email_val.is_empty() {
                error_message.set(Some("Email is required".to_string()));
            } else {
                error_message.set(Some("Password is required".to_string()));
            }
            return;
        }

        // Basic email validation
        if !email_val.contains('@') || !email_val.contains('.') {
            error_message.set(Some("Please enter a valid email address".to_string()));
            return;
        }

        // Password validation
        if password_val.len() < 8 {
            error_message.set(Some("Password must be at least 8 characters long".to_string()));
            return;
        }

        // Confirm password validation
        if password_val != confirm_password_val {
            error_message.set(Some("Passwords do not match".to_string()));
            return;
        }

        error_message.set(None);
        is_loading.set(true);

        // Register request
        spawn(async move {
            let client = Client::new();
            let register_req = RegisterRequest {
                email: email_val.clone(),
                password: password_val.clone(),
                name: if name_val.is_empty() { None } else { Some(name_val) },
            };

            match client
                .post("http://127.0.0.1:8081/api/auth/register")
                .header("Content-Type", "application/json")
                .json(&register_req)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<AuthResponse>().await {
                            Ok(auth_response) => {
                                // Convert UserResponse to User
                                let user = User {
                                    id: auth_response.user.id,
                                    email: auth_response.user.email,
                                    name: auth_response.user.name,
                                    role: auth_response.user.role,
                                };
                                
                                // Update auth state
                                auth_state.write().login(user, auth_response.token);
                                is_loading.set(false);
                                navigator.push(Route::Home {});
                            }
                            Err(e) => {
                                is_loading.set(false);
                                error_message.set(Some(format!("Failed to parse response: {}", e)));
                            }
                        }
                    } else {
                        let status_code = response.status().as_u16();
                        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        is_loading.set(false);
                        error_message.set(Some(match status_code {
                            400 => "Invalid registration data".to_string(),
                            409 => "An account with this email already exists".to_string(),
                            _ => error_text,
                        }));
                    }
                }
                Err(e) => {
                    is_loading.set(false);
                    error_message.set(Some(format!("Network error: {}", e)));
                }
            }
        });
    };

    rsx! {
        div { class: "bg-gray-900 min-h-screen flex items-center justify-center px-4 py-8",
            div { class: "w-full max-w-md",
                div { class: "bg-white rounded-xl shadow-2xl p-8",
                    div { class: "text-center pt-4 mb-8",
                        a {
                            href: "#",
                            class: "flex items-center justify-center mb-6 text-2xl font-semibold text-gray-900",
                            img {
                                class: "w-8 h-8 mr-2",
                                src: LOGO,
                                alt: "logo",
                            }
                            "Ferrisbase"
                        }
                    }
                    div { class: "space-y-6 px-4",
                        h1 { class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl mb-6",
                            "Create your account"
                        }
                        form {
                            class: "space-y-4 md:space-y-6",
                            onsubmit: handle_submit,
                            div {
                                label {
                                    r#for: "name",
                                    class: "block mb-2 text-sm font-medium text-gray-900",
                                    "Name (optional)"
                                }
                                input {
                                    r#type: "text",
                                    id: "name",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5",
                                    placeholder: "Your name",
                                    value: "{name}",
                                    oninput: move |e| name.set(e.value()),
                                }
                            }

                            div {
                                label {
                                    r#for: "email",
                                    class: "block mb-2 text-sm font-medium text-gray-900",
                                    "Email"
                                }
                                input {
                                    r#type: "email",
                                    id: "email",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5",
                                    placeholder: "name@company.com",
                                    required: true,
                                    value: "{email}",
                                    oninput: move |e| email.set(e.value()),
                                }
                            }

                            div {
                                label {
                                    r#for: "password",
                                    class: "block mb-2 text-sm font-medium text-gray-900",
                                    "Password"
                                }
                                input {
                                    r#type: "password",
                                    id: "password",
                                    placeholder: "••••••••",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5",
                                    required: true,
                                    value: "{password}",
                                    oninput: move |e| password.set(e.value()),
                                }
                                p { class: "mt-1 text-xs text-gray-500",
                                    "Must be at least 8 characters"
                                }
                            }

                            div {
                                label {
                                    r#for: "confirm_password",
                                    class: "block mb-2 text-sm font-medium text-gray-900",
                                    "Confirm Password"
                                }
                                input {
                                    r#type: "password",
                                    id: "confirm_password",
                                    placeholder: "••••••••",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5",
                                    required: true,
                                    value: "{confirm_password}",
                                    oninput: move |e| confirm_password.set(e.value()),
                                }
                            }

                            div { class: "flex items-start",
                                div { class: "flex items-center h-5",
                                    input {
                                        id: "terms",
                                        aria_describedby: "terms",
                                        r#type: "checkbox",
                                        class: "w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300",
                                        required: true,
                                    }
                                }
                                div { class: "ml-3 text-sm",
                                    label {
                                        r#for: "terms",
                                        class: "font-light text-gray-500",
                                        "I accept the "
                                        Link {
                                            to: Route::TermsOfService {},
                                            class: "font-medium text-blue-600 hover:underline",
                                            "Terms and Conditions"
                                        }
                                    }
                                }
                            }

                            button {
                                r#type: "submit",
                                class: "w-full mb-4 cursor-pointer text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center disabled:opacity-75",
                                disabled: *is_loading.read(),
                                if *is_loading.read() {
                                    "Creating account..."
                                } else {
                                    "Create account"
                                }
                            }

                            if let Some(msg) = &*error_message.read() {
                                ErrorNotification {
                                    message: msg.clone(),
                                    class: Some("mb-0".to_string()),
                                }
                            }

                            p { class: "text-sm font-light text-gray-500 text-center",
                                "Already have an account? "
                                Link {
                                    to: Route::LoginForm {},
                                    class: "font-medium text-blue-600 hover:underline",
                                    "Sign in"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
