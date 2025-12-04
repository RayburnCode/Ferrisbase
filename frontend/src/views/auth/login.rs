use dioxus::prelude::*;

use crate::hooks::{use_auth, use_password_login};
use crate::Route;
use crate::components::ErrorNotification;
 
const LOGO: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn LoginForm() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let is_loading = use_signal(|| false);
    let error_message = use_signal(|| None::<String>);

    let auth = use_auth();
    let navigator = use_navigator();


    // Check if already authenticated
    use_effect(move || {
        if *auth.is_authenticated.read() {
            navigator.push(Route::Home {});
        }
    });

    let email_signal = email.clone();
    let password_signal = password.clone();
    let error_message_signal = error_message.clone();
    let is_loading_signal = is_loading.clone();
    let navigator_signal = navigator.clone();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();

        let email = email_signal.clone();
        let password = password_signal.clone();
        let mut error_message = error_message_signal.clone();
        let mut is_loading = is_loading_signal.clone();
        let navigator = navigator_signal.clone();
        let password_login = use_password_login();

        let email_val = email.read().clone();
        let password_val = password.read().clone();

        println!("Login attempt for: {}", email_val); // Debug log

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

        error_message.set(None);
        is_loading.set(true);

        spawn(async move {
            match password_login(email_val.clone(), password_val.clone()).await {
                Ok(_user) => {
                    // Success - auth state is already updated by the hook
                    is_loading.set(false);
                    navigator.push(Route::Home {});
                }
                Err(error) => {
                    // Error - display the message from the hook
                    is_loading.set(false);
                    error_message.set(Some(error));
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
                            "Mortgage Portal"
                        }
                    }
                    div { class: "space-y-6 px-4",
                        h1 { class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl mb-6",
                            "Sign in to your account"
                        }
                        form {
                            class: "space-y-4 md:space-y-6",
                            onsubmit: handle_submit,
                            div {
                                label {
                                    r#for: "email",
                                    class: "block mb-2 text-sm font-medium text-gray-900",
                                    "Email or Username"
                                }
                                input {
                                    r#type: "text",
                                    id: "email",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5",
                                    placeholder: "Enter your email or username",
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
                            }
                            div { class: "flex items-center justify-between",
                                div { class: "flex items-start",
                                    div { class: "flex items-center h-5",
                                        input {
                                            id: "remember",
                                            aria_describedby: "remember",
                                            r#type: "checkbox",
                                            class: "w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800",
                                            required: true,
                                        }
                                    }
                                    div { class: "ml-3 text-sm",
                                        label {
                                            r#for: "remember",
                                            class: "text-gray-500",
                                            "Remember me"
                                        }
                                    }
                                }
                                Link {
                                    to: Route::ResetPasswordForm {},
                                    class: "text-sm font-medium text-blue-600 hover:underline",
                                    "Forgot password?"
                                }
                            }
                            button {
                                r#type: "submit",
                                class: "w-full mb-4 cursor-pointer text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center disabled:opacity-75",
                                disabled: *is_loading.read(),
                                if *is_loading.read() {
                                    "Signing in..."
                                } else {
                                    "Sign in"
                                }
                            }
                            if let Some(msg) = &*error_message.read() {
                                ErrorNotification {
                                    message: msg.clone(),
                                    class: Some("mb-0".to_string()),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}