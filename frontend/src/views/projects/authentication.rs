use dioxus::prelude::*;

/// The Authentication page component for managing users and auth settings
#[component]
pub fn Authentication(id: String) -> Element {
    let mut active_tab = use_signal(|| "users".to_string());
    let mut search_query = use_signal(|| String::new());
    
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-6",
                div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Authentication & Users" }
                        p { class: "text-gray-600 mt-1", "Project: {id}" }
                    }
                    button { class: "inline-flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold px-6 py-3 rounded-md shadow-sm transition duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                        span { class: "text-lg", "+" }
                        "Add User"
                    }
                }
                // Tabs Navigation
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-2",
                    div { class: "flex gap-2",
                        button {
                            class: if active_tab() == "users" { "flex-1 px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "flex-1 px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("users".to_string()),
                            "Users"
                        }
                        button {
                            class: if active_tab() == "strategies" { "flex-1 px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "flex-1 px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("strategies".to_string()),
                            "Auth Strategies"
                        }
                        button {
                            class: if active_tab() == "security" { "flex-1 px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "flex-1 px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("security".to_string()),
                            "Security Settings"
                        }
                    }
                }
            }
            // Users Tab Content
            if active_tab() == "users" {
                div { class: "max-w-7xl mx-auto",
                    // User Stats
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6",
                        div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                            div { class: "flex items-center justify-between mb-2",
                                span { class: "text-sm font-medium text-gray-600", "Total Users" }
                                span { class: "text-2xl", "👥" }
                            }
                            p { class: "text-3xl font-bold text-gray-900", "3,847" }
                            p { class: "text-sm text-green-600 mt-1", "↑ 284 this week" }
                        }
                        div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                            div { class: "flex items-center justify-between mb-2",
                                span { class: "text-sm font-medium text-gray-600", "Active Today" }
                                span { class: "text-2xl", "✅" }
                            }
                            p { class: "text-3xl font-bold text-gray-900", "1,234" }
                            p { class: "text-sm text-gray-600 mt-1", "32% of total" }
                        }
                        div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                            div { class: "flex items-center justify-between mb-2",
                                span { class: "text-sm font-medium text-gray-600", "New Signups" }
                                span { class: "text-2xl", "🆕" }
                            }
                            p { class: "text-3xl font-bold text-gray-900", "284" }
                            p { class: "text-sm text-green-600 mt-1", "Last 7 days" }
                        }
                        div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                            div { class: "flex items-center justify-between mb-2",
                                span { class: "text-sm font-medium text-gray-600", "Failed Logins" }
                                span { class: "text-2xl", "🚫" }
                            }
                            p { class: "text-3xl font-bold text-gray-900", "127" }
                            p { class: "text-sm text-red-600 mt-1", "Last 24 hours" }
                        }
                    }
                    // Search and Filters
                    div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4 mb-6",
                        div { class: "flex flex-col sm:flex-row gap-4",
                            input {
                                r#type: "text",
                                placeholder: "Search users by name, email, or ID...",
                                class: "flex-1 px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                                value: "{search_query}",
                                oninput: move |evt| search_query.set(evt.value()),
                            }
                            select { class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                                option { "All Users" }
                                option { "Active" }
                                option { "Inactive" }
                                option { "Banned" }
                            }
                            select { class: "px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                                option { "Sort: Newest" }
                                option { "Sort: Oldest" }
                                option { "Sort: Name" }
                                option { "Sort: Last Active" }
                            }
                        }
                    }
                    // Users Table
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden",
                        table { class: "min-w-full divide-y divide-gray-200",
                            thead { class: "bg-gray-50",
                                tr {
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "User"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Status"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Auth Method"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Last Login"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Actions"
                                    }
                                }
                            }
                            tbody { class: "bg-white divide-y divide-gray-200",
                                tr { class: "hover:bg-gray-50 transition",
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "h-10 w-10 flex-shrink-0 bg-blue-100 rounded-full flex items-center justify-center text-blue-600 font-semibold",
                                                "JD"
                                            }
                                            div { class: "ml-4",
                                                div { class: "text-sm font-medium text-gray-900",
                                                    "John Doe"
                                                }
                                                div { class: "text-sm text-gray-500",
                                                    "john.doe@example.com"
                                                }
                                            }
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span { class: "px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
                                            "Active"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "Email/Password"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "2 hours ago"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                        button { class: "text-blue-600 hover:text-blue-900 mr-3",
                                            "Edit"
                                        }
                                        button { class: "text-red-600 hover:text-red-900",
                                            "Disable"
                                        }
                                    }
                                }
                                tr { class: "hover:bg-gray-50 transition",
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "h-10 w-10 flex-shrink-0 bg-purple-100 rounded-full flex items-center justify-center text-purple-600 font-semibold",
                                                "JS"
                                            }
                                            div { class: "ml-4",
                                                div { class: "text-sm font-medium text-gray-900",
                                                    "Jane Smith"
                                                }
                                                div { class: "text-sm text-gray-500",
                                                    "jane.smith@example.com"
                                                }
                                            }
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span { class: "px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
                                            "Active"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "OAuth (Google)"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "1 day ago"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                        button { class: "text-blue-600 hover:text-blue-900 mr-3",
                                            "Edit"
                                        }
                                        button { class: "text-red-600 hover:text-red-900",
                                            "Disable"
                                        }
                                    }
                                }
                                tr { class: "hover:bg-gray-50 transition",
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "h-10 w-10 flex-shrink-0 bg-orange-100 rounded-full flex items-center justify-center text-orange-600 font-semibold",
                                                "BJ"
                                            }
                                            div { class: "ml-4",
                                                div { class: "text-sm font-medium text-gray-900",
                                                    "Bob Johnson"
                                                }
                                                div { class: "text-sm text-gray-500",
                                                    "bob.j@example.com"
                                                }
                                            }
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        span { class: "px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800",
                                            "Inactive"
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "Email/Password"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "30 days ago"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                        button { class: "text-blue-600 hover:text-blue-900 mr-3",
                                            "Edit"
                                        }
                                        button { class: "text-red-600 hover:text-red-900",
                                            "Disable"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Auth Strategies Tab Content
            if active_tab() == "strategies" {
                div { class: "max-w-7xl mx-auto",
                    // Current Strategy Overview
                    div { class: "bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg border border-blue-200 p-6 mb-6",
                        div { class: "flex items-start justify-between",
                            div {
                                h2 { class: "text-xl font-semibold text-gray-900 mb-2",
                                    "Active Authentication Strategies"
                                }
                                p { class: "text-sm text-gray-600",
                                    "Configure how users authenticate to your application"
                                }
                            }
                            span { class: "text-4xl", "🔐" }
                        }
                    }
                    // Auth Strategies Grid
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                        // Email/Password Strategy
                        div { class: "bg-white rounded-lg shadow-md border-2 border-blue-500 p-6",
                            div { class: "flex items-start justify-between mb-4",
                                div { class: "flex items-center gap-3",
                                    div { class: "w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center text-2xl",
                                        "✉️"
                                    }
                                    div {
                                        h3 { class: "text-lg font-semibold text-gray-900",
                                            "Email & Password"
                                        }
                                        span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                            "Active"
                                        }
                                    }
                                }
                                button { class: "text-blue-600 hover:text-blue-700 font-medium text-sm",
                                    "Configure"
                                }
                            }
                            p { class: "text-sm text-gray-600 mb-4",
                                "Traditional email and password authentication with password hashing and verification."
                            }
                            div { class: "space-y-2 mb-4",
                                div { class: "flex items-center text-sm",
                                    span { class: "text-green-600 mr-2", "✓" }
                                    span { class: "text-gray-700", "Password requirements enforced" }
                                }
                                div { class: "flex items-center text-sm",
                                    span { class: "text-green-600 mr-2", "✓" }
                                    span { class: "text-gray-700", "Email verification enabled" }
                                }
                                div { class: "flex items-center text-sm",
                                    span { class: "text-green-600 mr-2", "✓" }
                                    span { class: "text-gray-700", "Password reset flow active" }
                                }
                            }
                            div { class: "flex gap-2",
                                button { class: "flex-1 px-4 py-2 bg-blue-600 text-white rounded-md font-medium hover:bg-blue-700 transition",
                                    "Settings"
                                }
                                button { class: "px-4 py-2 bg-red-600 text-white rounded-md font-medium hover:bg-red-700 transition",
                                    "Disable"
                                }
                            }
                        }
                        // OAuth Strategy
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                            div { class: "flex items-start justify-between mb-4",
                                div { class: "flex items-center gap-3",
                                    div { class: "w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center text-2xl",
                                        "🔗"
                                    }
                                    div {
                                        h3 { class: "text-lg font-semibold text-gray-900",
                                            "OAuth Providers"
                                        }
                                        span { class: "px-2 py-1 bg-gray-100 text-gray-800 text-xs font-semibold rounded",
                                            "Available"
                                        }
                                    }
                                }
                                button { class: "text-blue-600 hover:text-blue-700 font-medium text-sm",
                                    "Enable"
                                }
                            }
                            p { class: "text-sm text-gray-600 mb-4",
                                "Allow users to sign in with third-party providers like Google, GitHub, or Facebook."
                            }
                            div { class: "space-y-2 mb-4",
                                div { class: "flex items-center justify-between p-2 border border-gray-200 rounded",
                                    div { class: "flex items-center gap-2",
                                        span { "🔴" }
                                        span { class: "text-sm font-medium text-gray-700",
                                            "Google"
                                        }
                                    }
                                    button { class: "text-xs text-blue-600 hover:text-blue-700",
                                        "Configure"
                                    }
                                }
                                div { class: "flex items-center justify-between p-2 border border-gray-200 rounded",
                                    div { class: "flex items-center gap-2",
                                        span { "⚫" }
                                        span { class: "text-sm font-medium text-gray-700",
                                            "GitHub"
                                        }
                                    }
                                    button { class: "text-xs text-blue-600 hover:text-blue-700",
                                        "Configure"
                                    }
                                }
                                div { class: "flex items-center justify-between p-2 border border-gray-200 rounded",
                                    div { class: "flex items-center gap-2",
                                        span { "🔵" }
                                        span { class: "text-sm font-medium text-gray-700",
                                            "Facebook"
                                        }
                                    }
                                    button { class: "text-xs text-blue-600 hover:text-blue-700",
                                        "Configure"
                                    }
                                }
                            }
                            button { class: "w-full px-4 py-2 bg-gray-200 text-gray-700 rounded-md font-medium hover:bg-gray-300 transition",
                                "Enable OAuth"
                            }
                        }
                        // Magic Link Strategy
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                            div { class: "flex items-start justify-between mb-4",
                                div { class: "flex items-center gap-3",
                                    div { class: "w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center text-2xl",
                                        "🪄"
                                    }
                                    div {
                                        h3 { class: "text-lg font-semibold text-gray-900",
                                            "Magic Link"
                                        }
                                        span { class: "px-2 py-1 bg-gray-100 text-gray-800 text-xs font-semibold rounded",
                                            "Available"
                                        }
                                    }
                                }
                                button { class: "text-blue-600 hover:text-blue-700 font-medium text-sm",
                                    "Enable"
                                }
                            }
                            p { class: "text-sm text-gray-600 mb-4",
                                "Passwordless authentication via email. Users receive a secure link to sign in."
                            }
                            div { class: "space-y-2 mb-4",
                                div { class: "flex items-center text-sm",
                                    span { class: "text-gray-400 mr-2", "○" }
                                    span { class: "text-gray-500", "No password required" }
                                }
                                div { class: "flex items-center text-sm",
                                    span { class: "text-gray-400 mr-2", "○" }
                                    span { class: "text-gray-500", "Time-limited links (15 min)" }
                                }
                                div { class: "flex items-center text-sm",
                                    span { class: "text-gray-400 mr-2", "○" }
                                    span { class: "text-gray-500", "Email service required" }
                                }
                            }
                            button { class: "w-full px-4 py-2 bg-gray-200 text-gray-700 rounded-md font-medium hover:bg-gray-300 transition",
                                "Enable Magic Link"
                            }
                        }
                        // Two-Factor Auth
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                            div { class: "flex items-start justify-between mb-4",
                                div { class: "flex items-center gap-3",
                                    div { class: "w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center text-2xl",
                                        "🔢"
                                    }
                                    div {
                                        h3 { class: "text-lg font-semibold text-gray-900",
                                            "Two-Factor Auth (2FA)"
                                        }
                                        span { class: "px-2 py-1 bg-yellow-100 text-yellow-800 text-xs font-semibold rounded",
                                            "Recommended"
                                        }
                                    }
                                }
                                button { class: "text-blue-600 hover:text-blue-700 font-medium text-sm",
                                    "Enable"
                                }
                            }
                            p { class: "text-sm text-gray-600 mb-4",
                                "Add an extra layer of security with TOTP codes or SMS verification."
                            }
                            div { class: "space-y-2 mb-4",
                                div { class: "flex items-center text-sm",
                                    span { class: "text-gray-400 mr-2", "○" }
                                    span { class: "text-gray-500", "TOTP apps (Google Authenticator)" }
                                }
                                div { class: "flex items-center text-sm",
                                    span { class: "text-gray-400 mr-2", "○" }
                                    span { class: "text-gray-500", "SMS verification codes" }
                                }
                                div { class: "flex items-center text-sm",
                                    span { class: "text-gray-400 mr-2", "○" }
                                    span { class: "text-gray-500", "Backup codes available" }
                                }
                            }
                            button { class: "w-full px-4 py-2 bg-gray-200 text-gray-700 rounded-md font-medium hover:bg-gray-300 transition",
                                "Enable 2FA"
                            }
                        }
                    }
                }
            }
            // Security Settings Tab Content
            if active_tab() == "security" {
                div { class: "max-w-7xl mx-auto",
                    // Security Score
                    div { class: "bg-gradient-to-r from-green-50 to-emerald-50 rounded-lg border border-green-200 p-6 mb-6",
                        div { class: "flex items-center justify-between",
                            div {
                                h2 { class: "text-2xl font-bold text-gray-900 mb-2",
                                    "Security Score: 85/100"
                                }
                                p { class: "text-sm text-gray-600",
                                    "Your application security is strong. Consider enabling 2FA for better protection."
                                }
                            }
                            div { class: "text-6xl", "🛡️" }
                        }
                    }
                    // Security Settings Grid
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                        // Password Policy
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                            h3 { class: "text-lg font-semibold text-gray-900 mb-4",
                                "Password Policy"
                            }
                            div { class: "space-y-4",
                                div {
                                    label { class: "flex items-center justify-between cursor-pointer",
                                        span { class: "text-sm font-medium text-gray-700",
                                            "Minimum password length"
                                        }
                                        input {
                                            r#type: "number",
                                            value: "8",
                                            class: "w-20 px-3 py-1 border border-gray-300 rounded-md text-sm",
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700",
                                            "Require uppercase letters"
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700", "Require numbers" }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700",
                                            "Require special characters"
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700",
                                            "Password expiration (90 days)"
                                        }
                                    }
                                }
                            }
                            button { class: "mt-4 w-full px-4 py-2 bg-blue-600 text-white rounded-md font-medium hover:bg-blue-700 transition",
                                "Save Changes"
                            }
                        }
                        // Session Management
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                            h3 { class: "text-lg font-semibold text-gray-900 mb-4",
                                "Session Management"
                            }
                            div { class: "space-y-4",
                                div {
                                    label { class: "flex items-center justify-between",
                                        span { class: "text-sm font-medium text-gray-700",
                                            "Session timeout (minutes)"
                                        }
                                        input {
                                            r#type: "number",
                                            value: "30",
                                            class: "w-20 px-3 py-1 border border-gray-300 rounded-md text-sm",
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700", "Remember me option" }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700",
                                            "Logout on password change"
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700", "Single session per user" }
                                    }
                                }
                            }
                            button { class: "mt-4 w-full px-4 py-2 bg-blue-600 text-white rounded-md font-medium hover:bg-blue-700 transition",
                                "Save Changes"
                            }
                        }
                        // Account Security
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                            h3 { class: "text-lg font-semibold text-gray-900 mb-4",
                                "Account Security"
                            }
                            div { class: "space-y-4",
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700",
                                            "Email verification required"
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700",
                                            "Account lockout after failed attempts"
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center justify-between",
                                        span { class: "text-sm font-medium text-gray-700",
                                            "Max failed login attempts"
                                        }
                                        input {
                                            r#type: "number",
                                            value: "5",
                                            class: "w-20 px-3 py-1 border border-gray-300 rounded-md text-sm",
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700",
                                            "CAPTCHA for suspicious activity"
                                        }
                                    }
                                }
                            }
                            button { class: "mt-4 w-full px-4 py-2 bg-blue-600 text-white rounded-md font-medium hover:bg-blue-700 transition",
                                "Save Changes"
                            }
                        }
                        // IP & Rate Limiting
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                            h3 { class: "text-lg font-semibold text-gray-900 mb-4",
                                "IP & Rate Limiting"
                            }
                            div { class: "space-y-4",
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: true,
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700", "Enable rate limiting" }
                                    }
                                }
                                div {
                                    label { class: "flex items-center justify-between",
                                        span { class: "text-sm font-medium text-gray-700",
                                            "Max requests per minute"
                                        }
                                        input {
                                            r#type: "number",
                                            value: "100",
                                            class: "w-20 px-3 py-1 border border-gray-300 rounded-md text-sm",
                                        }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700", "IP whitelist enabled" }
                                    }
                                }
                                div {
                                    label { class: "flex items-center cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            class: "mr-2 h-4 w-4 text-blue-600 rounded",
                                        }
                                        span { class: "text-sm text-gray-700", "Geo-blocking enabled" }
                                    }
                                }
                            }
                            button { class: "mt-4 w-full px-4 py-2 bg-blue-600 text-white rounded-md font-medium hover:bg-blue-700 transition",
                                "Save Changes"
                            }
                        }
                    }
                }
            }
        }
    }
}
