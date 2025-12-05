use crate::{Route, AuthState};
use dioxus::prelude::*;

//Logo / Breadcrumbs / Nav links / User menu

#[component]
pub fn SignedInNavbar(children: Element) -> Element {
    const LOGO: Asset = asset!("/assets/Original_Ferris.svg");

    let route = use_route::<Route>();
    let route_clone = route.clone();
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut show_user_menu = use_signal(|| false);
    let mut auth_state = use_context::<Signal<AuthState>>();
    
    let is_authenticated = auth_state.read().is_authenticated();
    let user = auth_state.read().user.clone();

    // Generate breadcrumb items based on current route
    let breadcrumbs = use_memo(move || {
        let mut items: Vec<(String, Option<Route>)> = vec![];
        
        match route_clone {
            Route::Projects {} => {
                items.push(("Projects".to_string(), None));
            }
            Route::CreateNewProject {} => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push(("New Project".to_string(), None));
            }
            Route::UserSettings {} => {
                items.push(("Settings".to_string(), None));
            }
            Route::ProjectById { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), None));
            }
            Route::TableEditor { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("Table Editor".to_string(), None));
            }
            Route::SQLEditor { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("SQL Editor".to_string(), None));
            }
            Route::Database { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("Database".to_string(), None));
            }
            Route::Authentication { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("Authentication".to_string(), None));
            }
            Route::Reports { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("Reports".to_string(), None));
            }
            Route::Logs { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("Logs".to_string(), None));
            }
            Route::APIDocs { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("API Docs".to_string(), None));
            }
            Route::ProjectSettings { ref id } => {
                items.push(("Projects".to_string(), Some(Route::Projects {})));
                items.push((id.clone(), Some(Route::ProjectById { id: id.clone() })));
                items.push(("Settings".to_string(), None));
            }
            _ => {}
        }
        
        items
    });

    rsx! {
        nav { class: "sticky top-0 z-50 w-full bg-white backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    // Logo and Breadcrumbs (Desktop)
                    div { class: "flex items-center gap-6 flex-1 min-w-0",
                        // Logo
                        Link {
                            to: Route::Projects {},
                            class: "flex items-center gap-2 text-xl font-bold shrink-0",
                            img { class: "w-8 h-8", src: "{LOGO}" }
                            span { class: "hidden sm:inline text-gray-900", "Ferrisbase" }
                        }

                        // Breadcrumbs (Desktop only)
                        nav {
                            aria_label: "Breadcrumb",
                            class: "hidden md:flex items-center min-w-0 flex-1",
                            ol { class: "inline-flex items-center space-x-1",
                                // Home/Dashboard breadcrumb
                                li { class: "inline-flex items-center",
                                    Link {
                                        to: Route::Projects {},
                                        class: "inline-flex items-center text-sm font-medium text-gray-600 hover:text-blue-600 transition-colors",
                                        svg {
                                            class: "w-4 h-4",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            path { d: "M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z" }
                                        }
                                    }
                                }
                                // Dynamic breadcrumb items
                                for (index , (label , route)) in breadcrumbs().iter().enumerate() {
                                    li { key: "{index}",
                                        div { class: "flex items-center",
                                            // Separator
                                            svg {
                                                class: "w-5 h-5 text-gray-400 mx-1",
                                                fill: "currentColor",
                                                view_box: "0 0 20 20",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                path {
                                                    fill_rule: "evenodd",
                                                    d: "M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z",
                                                    clip_rule: "evenodd",
                                                }
                                            }
                                            // Breadcrumb link or text
                                            {
                                                if let Some(r) = route {
                                                    rsx! {
                                                        Link {
                                                            to: r.clone(),
                                                            class: "text-sm font-medium text-gray-600 hover:text-blue-600 transition-colors truncate max-w-[150px]",
                                                            "{label}"
                                                        }
                                                    }
                                                } else {
                                                    rsx! {
                                                        span { class: "text-sm font-medium text-gray-900 truncate max-w-[150px]", "{label}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
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

                                            Link {
                                                to: Route::UserSettings {},
                                                onclick: move |_| show_user_menu.set(false),
                                                class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
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
                            to: Route::Projects {},
                            class: if matches!(route, Route::Projects {}) { "block px-3 py-2 text-sm font-semibold text-blue-600 bg-blue-50 rounded-lg" } else { "block px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg" },
                            onclick: move |_| is_mobile_menu_open.set(false),
                            "Projects"
                        }
                        if is_authenticated {
                            Link {
                                to: Route::UserSettings {},
                                class: if matches!(route, Route::UserSettings {}) { "block px-3 py-2 text-sm font-semibold text-blue-600 bg-blue-50 rounded-lg" } else { "block px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg" },
                                onclick: move |_| is_mobile_menu_open.set(false),
                                "Settings"
                            }
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
