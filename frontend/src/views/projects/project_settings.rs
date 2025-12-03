use dioxus::prelude::*;

/// The Project Settings page for managing project configuration
#[component]
pub fn ProjectSettings(id: String) -> Element {
    let mut active_tab = use_signal(|| "general".to_string());
    let mut project_name = use_signal(|| "My Awesome Project".to_string());
    let mut custom_domain = use_signal(|| String::new());
    
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-6",
                div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Project Settings" }
                        p { class: "text-gray-600 mt-1", "Project ID: {id}" }
                    }
                    div { class: "flex gap-3",
                        button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                            "Cancel"
                        }
                        button { class: "px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-md transition focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                            "Save Changes"
                        }
                    }
                }
                // Tabs Navigation
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-2",
                    div { class: "flex flex-wrap gap-2",
                        button {
                            class: if active_tab() == "general" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("general".to_string()),
                            "General"
                        }
                        button {
                            class: if active_tab() == "api" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("api".to_string()),
                            "API & Domains"
                        }
                        button {
                            class: if active_tab() == "team" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("team".to_string()),
                            "Team & Access"
                        }
                        button {
                            class: if active_tab() == "database" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("database".to_string()),
                            "Database"
                        }
                        button {
                            class: if active_tab() == "danger" { "px-4 py-2 bg-red-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("danger".to_string()),
                            "Danger Zone"
                        }
                    }
                }
            }
            // General Settings Tab
            if active_tab() == "general" {
                div { class: "max-w-7xl mx-auto space-y-6",
                    // Project Information
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Project Information" }
                        div { class: "space-y-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-2",
                                    "Project Name"
                                }
                                input {
                                    r#type: "text",
                                    value: "{project_name}",
                                    oninput: move |evt| project_name.set(evt.value()),
                                    class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                                }
                                p { class: "text-xs text-gray-500 mt-1",
                                    "This is the display name for your project"
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-2",
                                    "Project Description"
                                }
                                textarea {
                                    rows: "3",
                                    placeholder: "Describe your project...",
                                    class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition resize-none",
                                    "A powerful backend API for managing user data and authentication."
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-2",
                                    "Region"
                                }
                                select { class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                                    option { "US East (N. Virginia)" }
                                    option { "US West (Oregon)" }
                                    option { "EU (Ireland)" }
                                    option { "Asia Pacific (Tokyo)" }
                                }
                                p { class: "text-xs text-gray-500 mt-1",
                                    "⚠️ Changing region requires database migration"
                                }
                            }
                        }
                    }
                    // Project Status
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Project Status" }
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between p-4 bg-green-50 border border-green-200 rounded-lg",
                                div {
                                    p { class: "font-medium text-gray-900", "Current Status" }
                                    p { class: "text-sm text-gray-600",
                                        "Your project is active and accepting requests"
                                    }
                                }
                                span { class: "px-3 py-1 bg-green-600 text-white text-sm font-semibold rounded-full",
                                    "Active"
                                }
                            }
                            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                div { class: "p-4 bg-gray-50 rounded-lg",
                                    p { class: "text-xs text-gray-500 mb-1", "Created" }
                                    p { class: "text-sm font-semibold text-gray-900",
                                        "Nov 15, 2025"
                                    }
                                }
                                div { class: "p-4 bg-gray-50 rounded-lg",
                                    p { class: "text-xs text-gray-500 mb-1", "Last Updated" }
                                    p { class: "text-sm font-semibold text-gray-900",
                                        "Dec 3, 2025"
                                    }
                                }
                                div { class: "p-4 bg-gray-50 rounded-lg",
                                    p { class: "text-xs text-gray-500 mb-1", "Total Requests" }
                                    p { class: "text-sm font-semibold text-gray-900",
                                        "42,581"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // API & Domains Tab
            if active_tab() == "api" {
                div { class: "max-w-7xl mx-auto space-y-6",
                    // Project URL
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Project URL" }
                        div { class: "space-y-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-2",
                                    "Auto-Generated API Endpoint"
                                }
                                div { class: "flex items-center gap-2",
                                    input {
                                        r#type: "text",
                                        value: "https://api.ferrisbase.io/v1/{id}",
                                        readonly: true,
                                        class: "flex-1 px-4 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-600 font-mono text-sm",
                                    }
                                    button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                                        "📋 Copy"
                                    }
                                }
                                p { class: "text-xs text-gray-500 mt-1",
                                    "This is your default API endpoint with auto-generated hash"
                                }
                            }
                        }
                    }
                    // Custom Domain
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        div { class: "flex items-start justify-between mb-4",
                            div {
                                h2 { class: "text-xl font-semibold text-gray-900",
                                    "Custom Domain"
                                }
                                p { class: "text-sm text-gray-600 mt-1",
                                    "Use your own domain for API requests"
                                }
                            }
                            span { class: "px-2 py-1 bg-yellow-100 text-yellow-800 text-xs font-semibold rounded",
                                "Pro Plan"
                            }
                        }
                        div { class: "space-y-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-2",
                                    "Domain Name"
                                }
                                div { class: "flex gap-2",
                                    input {
                                        r#type: "text",
                                        placeholder: "api.yourdomain.com",
                                        value: "{custom_domain}",
                                        oninput: move |evt| custom_domain.set(evt.value()),
                                        class: "flex-1 px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                                    }
                                    button { class: "px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                        "Add Domain"
                                    }
                                }
                                p { class: "text-xs text-gray-500 mt-1",
                                    "Point your DNS to: cname.ferrisbase.io"
                                }
                            }
                        }
                    }
                    // API Keys
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        div { class: "flex items-center justify-between mb-4",
                            h2 { class: "text-xl font-semibold text-gray-900", "API Keys" }
                            button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                "+ Generate New Key"
                            }
                        }
                        p { class: "text-sm text-gray-600 mb-4",
                            "API keys are used to authenticate requests to your project"
                        }
                        div { class: "space-y-3",
                            // API Key 1
                            div { class: "p-4 border border-gray-200 rounded-lg hover:border-gray-300 transition",
                                div { class: "flex items-start justify-between mb-2",
                                    div {
                                        p { class: "font-medium text-gray-900", "Production API Key" }
                                        p { class: "text-xs text-gray-500 mt-1",
                                            "Created on Nov 15, 2025 • Last used 2 hours ago"
                                        }
                                    }
                                    span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                        "Active"
                                    }
                                }
                                div { class: "flex items-center gap-2",
                                    code { class: "flex-1 px-3 py-2 bg-gray-900 text-green-400 rounded font-mono text-sm",
                                        "frb_live_sk_a8f9d2e1b3c4..."
                                    }
                                    button { class: "px-3 py-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-medium",
                                        "👁️"
                                    }
                                    button { class: "px-3 py-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-medium",
                                        "📋"
                                    }
                                    button { class: "px-3 py-2 bg-red-100 hover:bg-red-200 text-red-700 rounded text-sm font-medium",
                                        "🗑️"
                                    }
                                }
                            }
                            // API Key 2
                            div { class: "p-4 border border-gray-200 rounded-lg hover:border-gray-300 transition",
                                div { class: "flex items-start justify-between mb-2",
                                    div {
                                        p { class: "font-medium text-gray-900", "Development API Key" }
                                        p { class: "text-xs text-gray-500 mt-1",
                                            "Created on Nov 20, 2025 • Last used 1 day ago"
                                        }
                                    }
                                    span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                        "Active"
                                    }
                                }
                                div { class: "flex items-center gap-2",
                                    code { class: "flex-1 px-3 py-2 bg-gray-900 text-green-400 rounded font-mono text-sm",
                                        "frb_test_sk_c7b2e4f3a1d5..."
                                    }
                                    button { class: "px-3 py-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-medium",
                                        "👁️"
                                    }
                                    button { class: "px-3 py-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-medium",
                                        "📋"
                                    }
                                    button { class: "px-3 py-2 bg-red-100 hover:bg-red-200 text-red-700 rounded text-sm font-medium",
                                        "🗑️"
                                    }
                                }
                            }
                        }
                    }
                    // Rate Limiting
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Rate Limiting" }
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between",
                                div {
                                    p { class: "font-medium text-gray-900", "Enable Rate Limiting" }
                                    p { class: "text-sm text-gray-600",
                                        "Protect your API from excessive requests"
                                    }
                                }
                                button { class: "relative inline-flex h-6 w-11 items-center rounded-full bg-blue-600",
                                    span { class: "inline-block h-4 w-4 transform rounded-full bg-white transition translate-x-6" }
                                }
                            }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                                        "Requests per minute"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "100",
                                        class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 outline-none",
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                                        "Requests per hour"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "5000",
                                        class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 outline-none",
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Team & Access Tab
            if active_tab() == "team" {
                div { class: "max-w-7xl mx-auto space-y-6",
                    // Team Members
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        div { class: "flex items-center justify-between mb-4",
                            h2 { class: "text-xl font-semibold text-gray-900", "Team Members" }
                            button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                "+ Invite Member"
                            }
                        }
                        div { class: "space-y-3",
                            // Team Member 1
                            div { class: "flex items-center justify-between p-4 border border-gray-200 rounded-lg",
                                div { class: "flex items-center gap-3",
                                    div { class: "h-10 w-10 bg-blue-100 rounded-full flex items-center justify-center text-blue-600 font-semibold",
                                        "YO"
                                    }
                                    div {
                                        p { class: "font-medium text-gray-900", "You (Owner)" }
                                        p { class: "text-sm text-gray-500", "your.email@example.com" }
                                    }
                                }
                                span { class: "px-3 py-1 bg-purple-100 text-purple-800 text-xs font-semibold rounded-full",
                                    "Owner"
                                }
                            }
                            // Team Member 2
                            div { class: "flex items-center justify-between p-4 border border-gray-200 rounded-lg",
                                div { class: "flex items-center gap-3",
                                    div { class: "h-10 w-10 bg-green-100 rounded-full flex items-center justify-center text-green-600 font-semibold",
                                        "JD"
                                    }
                                    div {
                                        p { class: "font-medium text-gray-900", "John Developer" }
                                        p { class: "text-sm text-gray-500", "john@example.com" }
                                    }
                                }
                                div { class: "flex items-center gap-2",
                                    select { class: "px-3 py-1 border border-gray-300 rounded text-sm",
                                        option { "Admin" }
                                        option { "Developer" }
                                        option { "Viewer" }
                                    }
                                    button { class: "px-3 py-1 text-red-600 hover:bg-red-50 rounded text-sm font-medium",
                                        "Remove"
                                    }
                                }
                            }
                            // Team Member 3
                            div { class: "flex items-center justify-between p-4 border border-gray-200 rounded-lg",
                                div { class: "flex items-center gap-3",
                                    div { class: "h-10 w-10 bg-orange-100 rounded-full flex items-center justify-center text-orange-600 font-semibold",
                                        "SA"
                                    }
                                    div {
                                        p { class: "font-medium text-gray-900", "Sarah Analyst" }
                                        p { class: "text-sm text-gray-500", "sarah@example.com" }
                                    }
                                }
                                div { class: "flex items-center gap-2",
                                    select { class: "px-3 py-1 border border-gray-300 rounded text-sm",
                                        option { "Viewer" }
                                        option { "Admin" }
                                        option { "Developer" }
                                    }
                                    button { class: "px-3 py-1 text-red-600 hover:bg-red-50 rounded text-sm font-medium",
                                        "Remove"
                                    }
                                }
                            }
                        }
                    }
                    // Roles & Permissions
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Roles & Permissions" }
                        div { class: "space-y-4",
                            // Owner Role
                            div { class: "p-4 bg-purple-50 border border-purple-200 rounded-lg",
                                div { class: "flex items-center gap-2 mb-2",
                                    span { class: "px-2 py-1 bg-purple-600 text-white text-xs font-bold rounded",
                                        "OWNER"
                                    }
                                    p { class: "font-medium text-gray-900", "Full Access" }
                                }
                                p { class: "text-sm text-gray-600",
                                    "Can manage all project settings, delete project, and manage billing"
                                }
                            }
                            // Admin Role
                            div { class: "p-4 bg-blue-50 border border-blue-200 rounded-lg",
                                div { class: "flex items-center gap-2 mb-2",
                                    span { class: "px-2 py-1 bg-blue-600 text-white text-xs font-bold rounded",
                                        "ADMIN"
                                    }
                                    p { class: "font-medium text-gray-900", "Administrative Access" }
                                }
                                p { class: "text-sm text-gray-600",
                                    "Can modify settings, manage team members, and access all features"
                                }
                            }
                            // Developer Role
                            div { class: "p-4 bg-green-50 border border-green-200 rounded-lg",
                                div { class: "flex items-center gap-2 mb-2",
                                    span { class: "px-2 py-1 bg-green-600 text-white text-xs font-bold rounded",
                                        "DEVELOPER"
                                    }
                                    p { class: "font-medium text-gray-900", "Development Access" }
                                }
                                p { class: "text-sm text-gray-600",
                                    "Can manage database, API settings, and view analytics"
                                }
                            }
                            // Viewer Role
                            div { class: "p-4 bg-gray-50 border border-gray-200 rounded-lg",
                                div { class: "flex items-center gap-2 mb-2",
                                    span { class: "px-2 py-1 bg-gray-600 text-white text-xs font-bold rounded",
                                        "VIEWER"
                                    }
                                    p { class: "font-medium text-gray-900", "Read-Only Access" }
                                }
                                p { class: "text-sm text-gray-600",
                                    "Can view project data and analytics, but cannot make changes"
                                }
                            }
                        }
                    }
                }
            }
            // Database Tab
            if active_tab() == "database" {
                div { class: "max-w-7xl mx-auto space-y-6",
                    // Database Info
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Database Information" }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div { class: "p-4 bg-gray-50 rounded-lg",
                                p { class: "text-xs text-gray-500 mb-1", "Database Type" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "PostgreSQL 15.3"
                                }
                            }
                            div { class: "p-4 bg-gray-50 rounded-lg",
                                p { class: "text-xs text-gray-500 mb-1", "Database Size" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "12.4 GB"
                                }
                            }
                            div { class: "p-4 bg-gray-50 rounded-lg",
                                p { class: "text-xs text-gray-500 mb-1", "Connection Pool" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "20 connections"
                                }
                            }
                            div { class: "p-4 bg-gray-50 rounded-lg",
                                p { class: "text-xs text-gray-500 mb-1", "Backup Schedule" }
                                p { class: "text-sm font-semibold text-gray-900",
                                    "Daily at 2:00 AM"
                                }
                            }
                        }
                    }
                    // Backup Settings
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Backup & Recovery" }
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between",
                                div {
                                    p { class: "font-medium text-gray-900", "Automatic Backups" }
                                    p { class: "text-sm text-gray-600",
                                        "Daily backups retained for 7 days"
                                    }
                                }
                                button { class: "relative inline-flex h-6 w-11 items-center rounded-full bg-blue-600",
                                    span { class: "inline-block h-4 w-4 transform rounded-full bg-white transition translate-x-6" }
                                }
                            }
                            button { class: "w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                "Create Manual Backup Now"
                            }
                        }
                    }
                    // Connection String
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Database Connection" }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-2",
                                "Connection String"
                            }
                            div { class: "flex items-center gap-2",
                                input {
                                    r#type: "password",
                                    value: "postgresql://user:pass@db.ferrisbase.io:5432/mydb",
                                    readonly: true,
                                    class: "flex-1 px-4 py-2 border border-gray-300 rounded-md bg-gray-50 font-mono text-sm",
                                }
                                button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                                    "👁️"
                                }
                                button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                                    "📋"
                                }
                            }
                            p { class: "text-xs text-red-600 mt-1",
                                "⚠️ Keep this connection string secure and never commit to version control"
                            }
                        }
                    }
                }
            }
            // Danger Zone Tab
            if active_tab() == "danger" {
                div { class: "max-w-7xl mx-auto space-y-6",
                    // Transfer Project
                    div { class: "bg-white rounded-lg shadow-md border-2 border-orange-200 p-6",
                        div { class: "flex items-start gap-3 mb-4",
                            span { class: "text-3xl", "⚠️" }
                            div {
                                h2 { class: "text-xl font-semibold text-gray-900",
                                    "Transfer Project Ownership"
                                }
                                p { class: "text-sm text-gray-600 mt-1",
                                    "Transfer this project to another user. This action cannot be undone."
                                }
                            }
                        }
                        button { class: "px-6 py-2 bg-orange-600 hover:bg-orange-700 text-white font-medium rounded-md transition",
                            "Transfer Project"
                        }
                    }
                    // Archive Project
                    div { class: "bg-white rounded-lg shadow-md border-2 border-yellow-200 p-6",
                        div { class: "flex items-start gap-3 mb-4",
                            span { class: "text-3xl", "📦" }
                            div {
                                h2 { class: "text-xl font-semibold text-gray-900",
                                    "Archive Project"
                                }
                                p { class: "text-sm text-gray-600 mt-1",
                                    "Archive this project to make it read-only. You can restore it later."
                                }
                            }
                        }
                        button { class: "px-6 py-2 bg-yellow-600 hover:bg-yellow-700 text-white font-medium rounded-md transition",
                            "Archive Project"
                        }
                    }
                    // Delete Project
                    div { class: "bg-white rounded-lg shadow-md border-2 border-red-300 p-6",
                        div { class: "flex items-start gap-3 mb-4",
                            span { class: "text-3xl", "🗑️" }
                            div {
                                h2 { class: "text-xl font-semibold text-gray-900",
                                    "Delete Project"
                                }
                                p { class: "text-sm text-gray-600 mt-1",
                                    "Permanently delete this project and all associated data. This action cannot be undone."
                                }
                            }
                        }
                        div { class: "bg-red-50 border border-red-200 rounded-lg p-4 mb-4",
                            p { class: "text-sm text-red-800 font-medium mb-2",
                                "⚠️ Warning: This will permanently delete:"
                            }
                            ul { class: "list-disc list-inside text-sm text-red-700 space-y-1",
                                li { "All database tables and data" }
                                li { "All API endpoints and configurations" }
                                li { "All team members and permissions" }
                                li { "All backups and snapshots" }
                            }
                        }
                        button { class: "px-6 py-2 bg-red-600 hover:bg-red-700 text-white font-medium rounded-md transition",
                            "Delete Project Forever"
                        }
                    }
                }
            }
        }
    }
}
