use crate::{Route, AuthState};
use dioxus::prelude::*;


#[component]
pub fn Navbar(children: Element) -> Element {
    const LOGO: Asset = asset!("/assets/Original_Ferris.svg");

    let current_route = use_route::<Route>();
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut show_user_menu = use_signal(|| false);
    let mut auth_state = use_context::<Signal<AuthState>>();
    
    let is_authenticated = auth_state.read().is_authenticated();
    let user = auth_state.read().user.clone();

    rsx! {
        nav { class: "sticky top-0 z-50 w-full bg-white backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    // Logo/Brand
                    div { class: "flex items-center gap-8",
                        // Logo
                        Link {
                            to: Route::Home {},
                            class: "flex items-center gap-2 text-2xl font-bold",
                            img { class: "text-blue-600 w-8 h-8", src: "{LOGO}" }
                            span { class: "text-gray-900", "Ferrisbase" }
                        }
                        // Desktop navigation
                        div { class: "hidden md:flex items-center space-x-6",
                            Link {
                                to: Route::Home {},
                                class: if matches!(current_route, Route::Home {}) { "text-blue-600 font-semibold px-3 py-2 text-sm transition-colors" } else { "text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors" },
                                "Home"
                            }
                            if is_authenticated {
                                Link {
                                    to: Route::Projects {},
                                    class: if matches!(current_route, Route::Projects {}) { "text-blue-600 font-semibold px-3 py-2 text-sm transition-colors" } else { "text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors" },
                                    "Projects"
                                }
                            }
                            Link {
                                to: Route::FAQ {},
                                class: if matches!(current_route, Route::FAQ {}) { "text-blue-600 font-semibold px-3 py-2 text-sm transition-colors" } else { "text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors" },
                                "FAQ"
                            }
                        }
                    }

                    // Right side - Auth buttons or User menu
                    div { class: "flex items-center gap-4",
                        if is_authenticated {
                            // User menu (desktop)
                            div { class: "hidden md:flex items-center gap-3",
                                // New Project button

                                // User dropdown
                                div { class: "relative",
                                    button {
                                        class: "flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-gray-100 transition-colors",
                                        onclick: move |_| show_user_menu.set(!show_user_menu()),
                                        // Avatar
                                        div { class: "w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-full flex items-center justify-center text-white font-semibold text-sm",
                                            {
                                                if let Some(ref u) = user {
                                                    u.name
                                                        .as_ref()
                                                        .and_then(|n| n.chars().next())
                                                        .map(|c| c.to_string())
                                                        .unwrap_or_else(|| u.email.chars().next().unwrap_or('U').to_string())
                                                } else {
                                                    "U".to_string()
                                                }
                                            }
                                        }
                                        // Name
                                        span { class: "text-sm font-medium text-gray-700",
                                            {
                                                if let Some(ref u) = user {
                                                    u.name
                                                        .clone()
                                                        .unwrap_or_else(|| {
                                                            u.email.split('@').next().unwrap_or("User").to_string()
                                                        })
                                                } else {
                                                    "User".to_string()
                                                }
                                            }
                                        }
                                        // Dropdown icon
                                        svg {
                                            class: "w-4 h-4 text-gray-500",
                                            fill: "none",
                                            "viewBox": "0 0 24 24",
                                            stroke: "currentColor",
                                            path {
                                                "stroke-linecap": "round",
                                                "stroke-linejoin": "round",
                                                "stroke-width": "2",
                                                d: "M19 9l-7 7-7-7",
                                            }
                                        }
                                    }
                                    // Dropdown menu
                                    if show_user_menu() {
                                        div { class: "absolute right-0 mt-2 w-56 bg-white rounded-lg shadow-lg border border-gray-200 py-2 z-50",
                                            // User info
                                            div { class: "px-4 py-3 border-b border-gray-200",
                                                if let Some(ref u) = user {
                                                    div {
                                                        p { class: "text-sm font-medium text-gray-900",
                                                            {u.name.clone().unwrap_or_else(|| "User".to_string())}
                                                        }
                                                        p { class: "text-xs text-gray-500 truncate",
                                                            {u.email.clone()}
                                                        }
                                                    }
                                                }
                                            }
                                            // Menu items
                                            Link {
                                                to: Route::Projects {},
                                                onclick: move |_| show_user_menu.set(false),
                                                class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                "My Projects"
                                            }
                                            button {
                                                class: "w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                onclick: move |_| show_user_menu.set(false),
                                                "Settings"
                                            }
                                            div { class: "border-t border-gray-200 mt-2 pt-2",
                                                button {
                                                    class: "w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50",
                                                    onclick: move |_| {
                                                        auth_state.write().logout();
                                                        show_user_menu.set(false);
                                                        navigator().push(Route::Home {});
                                                    },
                                                    "Sign Out"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Login/Signup buttons (desktop)
                            div { class: "hidden md:flex items-center gap-3",
                                Link {
                                    to: Route::LoginForm {},
                                    class: "px-4 py-2 text-sm font-medium text-gray-700 hover:text-gray-900 transition-colors",
                                    "Log in"
                                }
                                Link {
                                    to: Route::RegisterForm {},
                                    class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition-colors shadow-sm",
                                    "Register"
                                }
                            }
                        }
                        // Mobile menu button
                        button {
                            class: "md:hidden inline-flex items-center justify-center p-2 rounded-lg text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors",
                            onclick: move |_| is_mobile_menu_open.set(!is_mobile_menu_open()),
                            if !is_mobile_menu_open() {
                                svg {
                                    class: "h-6 w-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M4 6h16M4 12h16M4 18h16",
                                    }
                                }
                            } else {
                                svg {
                                    class: "h-6 w-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Mobile menu
            if is_mobile_menu_open() {
                div { class: "md:hidden border-t border-gray-200 bg-white",
                    div { class: "px-4 py-3 space-y-2",
                        Link {
                            to: Route::Home {},
                            class: if matches!(current_route, Route::Home {}) { "block px-3 py-2 text-sm font-semibold text-blue-600 bg-blue-50 rounded-lg" } else { "block px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg" },
                            onclick: move |_| is_mobile_menu_open.set(false),
                            "Home"
                        }
                        if is_authenticated {
                            Link {
                                to: Route::Projects {},
                                class: if matches!(current_route, Route::Projects {}) { "block px-3 py-2 text-sm font-semibold text-blue-600 bg-blue-50 rounded-lg" } else { "block px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg" },
                                onclick: move |_| is_mobile_menu_open.set(false),
                                "Projects"
                            }
                        }
                        Link {
                            to: Route::FAQ {},
                            class: if matches!(current_route, Route::FAQ {}) { "block px-3 py-2 text-sm font-semibold text-blue-600 bg-blue-50 rounded-lg" } else { "block px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg" },
                            onclick: move |_| is_mobile_menu_open.set(false),
                            "FAQ"
                        }
                    }
                    // Mobile auth section
                    div { class: "px-4 py-3 border-t border-gray-200",
                        if is_authenticated {
                            // User info and actions
                            div { class: "space-y-2",
                                // User info
                                div { class: "px-3 py-2 bg-gray-50 rounded-lg",
                                    if let Some(ref u) = user {
                                        div {
                                            p { class: "text-sm font-medium text-gray-900",
                                                {u.name.clone().unwrap_or_else(|| "User".to_string())}
                                            }
                                            p { class: "text-xs text-gray-500", {u.email.clone()} }
                                        }
                                    }
                                }
                                Link {
                                    to: Route::CreateNewProject {},
                                    class: "block px-3 py-2 text-sm font-medium text-center bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                                    onclick: move |_| is_mobile_menu_open.set(false),
                                    "+ New Project"
                                }
                                button {
                                    class: "w-full text-left px-3 py-2 text-sm font-medium text-red-600 hover:bg-red-50 rounded-lg",
                                    onclick: move |_| {
                                        auth_state.write().logout();
                                        is_mobile_menu_open.set(false);
                                        navigator().push(Route::Home {});
                                    },
                                    "Sign Out"
                                }
                            }
                        } else {
                            // Login/Signup
                            div { class: "space-y-2",
                                Link {
                                    to: Route::LoginForm {},
                                    class: "block text-center w-full px-3 py-2 text-sm font-medium text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50",
                                    onclick: move |_| {
                                        is_mobile_menu_open.set(false);
                                    },
                                    "Log in"
                                }
                                Link {
                                    to: Route::RegisterForm {},
                                    class: "block text-center w-full px-3 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700",
                                    onclick: move |_| {
                                        is_mobile_menu_open.set(false);
                                    },
                                    "Register"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}