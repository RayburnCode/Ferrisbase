use dioxus::prelude::*;
use crate::Route;
use crate::components::{ErrorNotification, SuccessNotification};
const LOGO: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn ResetPasswordForm() -> Element {
    let mut email = use_signal(|| String::new());
    let mut email_error = use_signal(|| String::new());
    let mut success_message = use_signal(|| String::new());
    let mut is_submitted = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    
    let handle_submit = move |_| {
        // Clear previous messages
        email_error.set("".to_string());
        success_message.set("".to_string());
        
        // Validation
        if email().is_empty() {
            email_error.set("Email is required".to_string());
            return;
        }
        
        if !email().contains('@') || !email().contains('.') {
            email_error.set("Please enter a valid email address".to_string());
            return;
        }
        
        is_loading.set(true);
        
        // Simulate API call
        let email_val = email();
        spawn(async move {
            // For demo purposes, simulate success after a brief moment
            // In real implementation, this would be an API call
            is_loading.set(false);
            success_message.set(format!("Password reset link sent to {}", email_val));
            is_submitted.set(true);
        });
    };

    // Render the component
    rsx! {
        div { class: " bg-gray-900 min-h-screen flex items-center justify-center px-4 py-8",
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
                        if !is_submitted() {
                            h1 { class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl",
                                "Reset your password"
                            }
                            p { class: "text-sm text-gray-600 mb-6",
                                "Enter your email address and we'll send you a link to reset your password."
                            }
                            form {
                                class: "space-y-4 md:space-y-6",
                                onsubmit: handle_submit,
                                div {
                                    label {
                                        r#for: "email",
                                        class: "block mb-2 text-sm font-medium text-gray-900",
                                        "Your email"
                                    }
                                    input {
                                        r#type: "email",
                                        name: "email",
                                        id: "email",
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value()),
                                        class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-600 focus:border-blue-600 block w-full p-2.5",
                                        placeholder: "name@company.com",
                                        required: true,
                                    }
                                    if !email_error().is_empty() {
                                        ErrorNotification {
                                            message: email_error(),
                                            class: Some("mt-2 mb-0".to_string()),
                                        }
                                    }
                                    if !success_message().is_empty() {
                                        SuccessNotification {
                                            message: success_message(),
                                            class: Some("mt-2 mb-0".to_string()),
                                        }
                                    }
                                }
                                button {
                                    r#type: "submit",
                                    disabled: is_loading(),
                                    class: "w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center disabled:opacity-75",
                                    if is_loading() {
                                        div { class: "flex items-center justify-center",
                                            svg {
                                                class: "animate-spin -ml-1 mr-3 h-5 w-5 text-white",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                view_box: "0 0 24 24",
                                                circle {
                                                    class: "opacity-25",
                                                    cx: "12",
                                                    cy: "12",
                                                    r: "10",
                                                    stroke: "currentColor",
                                                    stroke_width: "4",
                                                }
                                                path {
                                                    class: "opacity-75",
                                                    fill: "currentColor",
                                                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                                                }
                                            }
                                            "Processing..."
                                        }
                                    } else {
                                        div { class: "cursor-pointer", "Reset password" }
                                    }
                                }
                                p { class: "text-sm font-light text-gray-500 text-center",
                                    "Remember your password? "

                                    Link {
                                        to: Route::LoginForm {},
                                        class: "font-medium text-blue-600 hover:underline",
                                        "Back to login"
                                    }
                                }
                            }
                        } else {
                            div { class: "text-center",
                                div { class: "mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-green-100",
                                    svg {
                                        class: "h-6 w-6 text-green-600",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke_width: "1.5",
                                        stroke: "currentColor",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                        }
                                    }
                                }
                                h2 { class: "mt-4 text-xl font-bold text-gray-900",
                                    "Check your email"
                                }
                                p { class: "mt-2 text-sm text-gray-600",
                                    "We've sent a password reset link to "
                                    span { class: "font-medium text-blue-600", "{email}" }
                                }
                            }
                            div { class: "mt-6",
                                button {
                                    r#type: "button",
                                    onclick: move |_| {
                                        is_submitted.set(false);
                                        email.set("".to_string());
                                    },
                                    class: "w-full text-white bg-blue-600 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center",
                                    "Resend email"
                                }
                            }
                            div { class: "text-center mt-4",
                                Link {
                                    to: Route::LoginForm {},
                                    class: "text-sm font-medium text-blue-600 hover:underline",
                                    "Back to login"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}