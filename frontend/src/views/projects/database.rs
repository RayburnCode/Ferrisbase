use dioxus::prelude::*;

/// The Database page for managing tables, schemas, and database operations
#[component]
pub fn Database(id: String) -> Element {
    let mut active_tab = use_signal(|| "tables".to_string());
    let mut selected_table = use_signal(|| "users".to_string());
    let mut search_query = use_signal(|| String::new());
    
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-6",
                div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Database Management" }
                        p { class: "text-gray-600 mt-1", "Project: {id} • PostgreSQL 15.3" }
                    }
                    div { class: "flex gap-3",
                        button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                            "🔄 Sync Schema"
                        }
                        button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-md transition",
                            "+ Create Table"
                        }
                    }
                }
                // Tabs Navigation
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-2 mb-6",
                    div { class: "flex flex-wrap gap-2",
                        button {
                            class: if active_tab() == "tables" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("tables".to_string()),
                            "Tables"
                        }
                        button {
                            class: if active_tab() == "schema" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("schema".to_string()),
                            "Schema"
                        }
                        button {
                            class: if active_tab() == "migrations" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("migrations".to_string()),
                            "Migrations"
                        }
                        button {
                            class: if active_tab() == "backups" { "px-4 py-2 bg-blue-600 text-white rounded-md font-medium transition" } else { "px-4 py-2 bg-transparent text-gray-700 hover:bg-gray-100 rounded-md font-medium transition" },
                            onclick: move |_| active_tab.set("backups".to_string()),
                            "Backups"
                        }
                    }
                }
                // Search Bar
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    input {
                        r#type: "text",
                        placeholder: "Search tables, columns, or data...",
                        value: "{search_query}",
                        oninput: move |evt| search_query.set(evt.value()),
                        class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                    }
                }
            }
            // Tables Tab Content
            if active_tab() == "tables" {
                div { class: "max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-4 gap-6",
                    // Sidebar - Tables List
                    div { class: "lg:col-span-1",
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden sticky top-6",
                            div { class: "bg-gray-50 border-b border-gray-200 px-4 py-3",
                                h2 { class: "text-lg font-semibold text-gray-900",
                                    "Tables (8)"
                                }
                            }
                            div { class: "divide-y divide-gray-200 max-h-[600px] overflow-y-auto",
                                button {
                                    class: if selected_table() == "users" { "w-full text-left px-4 py-3 bg-blue-50 border-l-4 border-blue-600 hover:bg-blue-100 transition" } else { "w-full text-left px-4 py-3 hover:bg-gray-50 transition" },
                                    onclick: move |_| selected_table.set("users".to_string()),
                                    div {
                                        p { class: "font-medium text-gray-900", "users" }
                                        p { class: "text-xs text-gray-500",
                                            "3,847 rows • 12 columns"
                                        }
                                    }
                                }
                                button {
                                    class: if selected_table() == "posts" { "w-full text-left px-4 py-3 bg-blue-50 border-l-4 border-blue-600 hover:bg-blue-100 transition" } else { "w-full text-left px-4 py-3 hover:bg-gray-50 transition" },
                                    onclick: move |_| selected_table.set("posts".to_string()),
                                    div {
                                        p { class: "font-medium text-gray-900", "posts" }
                                        p { class: "text-xs text-gray-500",
                                            "1,234 rows • 8 columns"
                                        }
                                    }
                                }
                                button {
                                    class: if selected_table() == "comments" { "w-full text-left px-4 py-3 bg-blue-50 border-l-4 border-blue-600 hover:bg-blue-100 transition" } else { "w-full text-left px-4 py-3 hover:bg-gray-50 transition" },
                                    onclick: move |_| selected_table.set("comments".to_string()),
                                    div {
                                        p { class: "font-medium text-gray-900", "comments" }
                                        p { class: "text-xs text-gray-500",
                                            "5,621 rows • 6 columns"
                                        }
                                    }
                                }
                                button { class: "w-full text-left px-4 py-3 hover:bg-gray-50 transition",
                                    div {
                                        p { class: "font-medium text-gray-900", "sessions" }
                                        p { class: "text-xs text-gray-500", "892 rows • 5 columns" }
                                    }
                                }
                                button { class: "w-full text-left px-4 py-3 hover:bg-gray-50 transition",
                                    div {
                                        p { class: "font-medium text-gray-900", "api_keys" }
                                        p { class: "text-xs text-gray-500", "24 rows • 7 columns" }
                                    }
                                }
                            }
                        }
                    }
                    // Main Content - Table Details
                    div { class: "lg:col-span-3",
                        if selected_table() == "users" {
                            div { class: "space-y-6",
                                // Table Header
                                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                                    div { class: "flex items-start justify-between mb-4",
                                        div {
                                            h2 { class: "text-2xl font-bold text-gray-900",
                                                "users"
                                            }
                                            p { class: "text-sm text-gray-600 mt-1",
                                                "User accounts and authentication data"
                                            }
                                        }
                                        div { class: "flex gap-2",
                                            button { class: "px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded-md",
                                                "✏️ Edit"
                                            }
                                            button { class: "px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md",
                                                "+ Add Row"
                                            }
                                        }
                                    }
                                    div { class: "grid grid-cols-3 gap-4",
                                        div { class: "p-3 bg-gray-50 rounded",
                                            p { class: "text-xs text-gray-500", "Total Rows" }
                                            p { class: "text-lg font-bold text-gray-900",
                                                "3,847"
                                            }
                                        }
                                        div { class: "p-3 bg-gray-50 rounded",
                                            p { class: "text-xs text-gray-500", "Columns" }
                                            p { class: "text-lg font-bold text-gray-900",
                                                "12"
                                            }
                                        }
                                        div { class: "p-3 bg-gray-50 rounded",
                                            p { class: "text-xs text-gray-500", "Size" }
                                            p { class: "text-lg font-bold text-gray-900",
                                                "2.4 MB"
                                            }
                                        }
                                    }
                                }
                                // Columns Structure
                                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                                    div { class: "flex items-center justify-between mb-4",
                                        h3 { class: "text-lg font-semibold text-gray-900",
                                            "Table Structure"
                                        }
                                        button { class: "px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md",
                                            "+ Add Column"
                                        }
                                    }
                                    div { class: "border border-gray-200 rounded-lg overflow-hidden",
                                        table { class: "min-w-full divide-y divide-gray-200",
                                            thead { class: "bg-gray-50",
                                                tr {
                                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "Column"
                                                    }
                                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "Type"
                                                    }
                                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "Nullable"
                                                    }
                                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "Default"
                                                    }
                                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "Actions"
                                                    }
                                                }
                                            }
                                            tbody { class: "bg-white divide-y divide-gray-200",
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-4 py-3",
                                                        div { class: "flex items-center gap-2",
                                                            span { class: "text-sm font-mono font-medium text-gray-900",
                                                                "id"
                                                            }
                                                            span { class: "px-2 py-0.5 bg-purple-100 text-purple-800 text-xs font-semibold rounded",
                                                                "PK"
                                                            }
                                                        }
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600 font-mono",
                                                        "SERIAL"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600",
                                                        "No"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-500 font-mono",
                                                        "AUTO"
                                                    }
                                                    td { class: "px-4 py-3",
                                                        button { class: "text-blue-600 hover:text-blue-800 text-sm",
                                                            "Edit"
                                                        }
                                                    }
                                                }
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-4 py-3",
                                                        span { class: "text-sm font-mono font-medium text-gray-900",
                                                            "email"
                                                        }
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600 font-mono",
                                                        "VARCHAR(255)"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600",
                                                        "No"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-500",
                                                        "-"
                                                    }
                                                    td { class: "px-4 py-3",
                                                        button { class: "text-blue-600 hover:text-blue-800 text-sm",
                                                            "Edit"
                                                        }
                                                    }
                                                }
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-4 py-3",
                                                        span { class: "text-sm font-mono font-medium text-gray-900",
                                                            "password_hash"
                                                        }
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600 font-mono",
                                                        "VARCHAR(255)"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600",
                                                        "No"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-500",
                                                        "-"
                                                    }
                                                    td { class: "px-4 py-3",
                                                        button { class: "text-blue-600 hover:text-blue-800 text-sm",
                                                            "Edit"
                                                        }
                                                    }
                                                }
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-4 py-3",
                                                        span { class: "text-sm font-mono font-medium text-gray-900",
                                                            "name"
                                                        }
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600 font-mono",
                                                        "VARCHAR(100)"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600",
                                                        "Yes"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-500",
                                                        "NULL"
                                                    }
                                                    td { class: "px-4 py-3",
                                                        button { class: "text-blue-600 hover:text-blue-800 text-sm",
                                                            "Edit"
                                                        }
                                                    }
                                                }
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-4 py-3",
                                                        span { class: "text-sm font-mono font-medium text-gray-900",
                                                            "created_at"
                                                        }
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600 font-mono",
                                                        "TIMESTAMP"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-600",
                                                        "No"
                                                    }
                                                    td { class: "px-4 py-3 text-sm text-gray-500 font-mono",
                                                        "NOW()"
                                                    }
                                                    td { class: "px-4 py-3",
                                                        button { class: "text-blue-600 hover:text-blue-800 text-sm",
                                                            "Edit"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                // Indexes
                                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                                    div { class: "flex items-center justify-between mb-4",
                                        h3 { class: "text-lg font-semibold text-gray-900",
                                            "Indexes"
                                        }
                                        button { class: "px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md",
                                            "+ Add Index"
                                        }
                                    }
                                    div { class: "space-y-2",
                                        div { class: "p-3 border border-gray-200 rounded-lg",
                                            div { class: "flex items-center justify-between",
                                                div {
                                                    p { class: "font-medium text-gray-900 font-mono text-sm",
                                                        "users_pkey"
                                                    }
                                                    p { class: "text-xs text-gray-600 mt-1",
                                                        "PRIMARY KEY on (id)"
                                                    }
                                                }
                                                span { class: "px-2 py-1 bg-purple-100 text-purple-800 text-xs font-semibold rounded",
                                                    "Primary"
                                                }
                                            }
                                        }
                                        div { class: "p-3 border border-gray-200 rounded-lg",
                                            div { class: "flex items-center justify-between",
                                                div {
                                                    p { class: "font-medium text-gray-900 font-mono text-sm",
                                                        "users_email_unique"
                                                    }
                                                    p { class: "text-xs text-gray-600 mt-1",
                                                        "UNIQUE on (email)"
                                                    }
                                                }
                                                span { class: "px-2 py-1 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                    "Unique"
                                                }
                                            }
                                        }
                                    }
                                }
                                // Data Preview
                                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                                    h3 { class: "text-lg font-semibold text-gray-900 mb-4",
                                        "Data Preview (First 5 rows)"
                                    }
                                    div { class: "overflow-x-auto",
                                        table { class: "min-w-full divide-y divide-gray-200 text-sm",
                                            thead { class: "bg-gray-50",
                                                tr {
                                                    th { class: "px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "id"
                                                    }
                                                    th { class: "px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "email"
                                                    }
                                                    th { class: "px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "name"
                                                    }
                                                    th { class: "px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                        "created_at"
                                                    }
                                                }
                                            }
                                            tbody { class: "bg-white divide-y divide-gray-200",
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-3 py-2 font-mono text-gray-900",
                                                        "1"
                                                    }
                                                    td { class: "px-3 py-2 text-gray-600",
                                                        "john@example.com"
                                                    }
                                                    td { class: "px-3 py-2 text-gray-600",
                                                        "John Doe"
                                                    }
                                                    td { class: "px-3 py-2 text-gray-600 font-mono text-xs",
                                                        "2025-11-15 10:30"
                                                    }
                                                }
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-3 py-2 font-mono text-gray-900",
                                                        "2"
                                                    }
                                                    td { class: "px-3 py-2 text-gray-600",
                                                        "jane@example.com"
                                                    }
                                                    td { class: "px-3 py-2 text-gray-600",
                                                        "Jane Smith"
                                                    }
                                                    td { class: "px-3 py-2 text-gray-600 font-mono text-xs",
                                                        "2025-11-16 14:20"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Schema Tab Content
            if active_tab() == "schema" {
                div { class: "max-w-7xl mx-auto",
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6 mb-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4",
                            "Database Schema Visualization"
                        }
                        p { class: "text-gray-600 mb-4",
                            "Visual representation of your database structure and relationships"
                        }
                        // Schema Diagram (Simplified)
                        div { class: "bg-gray-50 rounded-lg p-6 border border-gray-200",
                            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                                // Users Table
                                div { class: "bg-white rounded-lg border-2 border-blue-300 shadow-sm",
                                    div { class: "bg-blue-600 text-white px-4 py-2 font-semibold",
                                        "users"
                                    }
                                    div { class: "p-3 space-y-1 text-sm",
                                        div { class: "flex items-center gap-2",
                                            span { class: "text-purple-600 font-bold",
                                                "🔑"
                                            }
                                            span { class: "font-mono text-gray-900",
                                                "id: SERIAL"
                                            }
                                        }
                                        div { class: "font-mono text-gray-700", "email: VARCHAR" }
                                        div { class: "font-mono text-gray-700", "name: VARCHAR" }
                                        div { class: "font-mono text-gray-700", "created_at: TIMESTAMP" }
                                    }
                                }
                                // Posts Table
                                div { class: "bg-white rounded-lg border-2 border-green-300 shadow-sm",
                                    div { class: "bg-green-600 text-white px-4 py-2 font-semibold",
                                        "posts"
                                    }
                                    div { class: "p-3 space-y-1 text-sm",
                                        div { class: "flex items-center gap-2",
                                            span { class: "text-purple-600 font-bold",
                                                "🔑"
                                            }
                                            span { class: "font-mono text-gray-900",
                                                "id: SERIAL"
                                            }
                                        }
                                        div { class: "flex items-center gap-2",
                                            span { class: "text-blue-600 font-bold",
                                                "🔗"
                                            }
                                            span { class: "font-mono text-gray-700",
                                                "user_id: INTEGER"
                                            }
                                        }
                                        div { class: "font-mono text-gray-700", "title: VARCHAR" }
                                        div { class: "font-mono text-gray-700", "content: TEXT" }
                                    }
                                }
                                // Comments Table
                                div { class: "bg-white rounded-lg border-2 border-orange-300 shadow-sm",
                                    div { class: "bg-orange-600 text-white px-4 py-2 font-semibold",
                                        "comments"
                                    }
                                    div { class: "p-3 space-y-1 text-sm",
                                        div { class: "flex items-center gap-2",
                                            span { class: "text-purple-600 font-bold",
                                                "🔑"
                                            }
                                            span { class: "font-mono text-gray-900",
                                                "id: SERIAL"
                                            }
                                        }
                                        div { class: "flex items-center gap-2",
                                            span { class: "text-blue-600 font-bold",
                                                "🔗"
                                            }
                                            span { class: "font-mono text-gray-700",
                                                "post_id: INTEGER"
                                            }
                                        }
                                        div { class: "flex items-center gap-2",
                                            span { class: "text-blue-600 font-bold",
                                                "🔗"
                                            }
                                            span { class: "font-mono text-gray-700",
                                                "user_id: INTEGER"
                                            }
                                        }
                                        div { class: "font-mono text-gray-700", "content: TEXT" }
                                    }
                                }
                            }
                            div { class: "mt-6 flex gap-4 text-sm",
                                div { class: "flex items-center gap-2",
                                    span { class: "text-purple-600 font-bold", "🔑" }
                                    span { class: "text-gray-600", "Primary Key" }
                                }
                                div { class: "flex items-center gap-2",
                                    span { class: "text-blue-600 font-bold", "🔗" }
                                    span { class: "text-gray-600", "Foreign Key" }
                                }
                            }
                        }
                    }
                    // Export Schema
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h3 { class: "text-lg font-semibold text-gray-900 mb-4", "Export Schema" }
                        div { class: "flex gap-3",
                            button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                "Download as SQL"
                            }
                            button { class: "px-4 py-2 bg-green-600 hover:bg-green-700 text-white font-medium rounded-md transition",
                                "Download as JSON"
                            }
                            button { class: "px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white font-medium rounded-md transition",
                                "Generate Diagram (PNG)"
                            }
                        }
                    }
                }
            }
            // Migrations Tab Content
            if active_tab() == "migrations" {
                div { class: "max-w-7xl mx-auto",
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        div { class: "flex items-center justify-between mb-6",
                            h2 { class: "text-xl font-semibold text-gray-900", "Migration History" }
                            button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                "+ Create Migration"
                            }
                        }
                        div { class: "space-y-4",
                            // Migration 1
                            div { class: "border border-gray-200 rounded-lg p-4",
                                div { class: "flex items-start justify-between mb-3",
                                    div {
                                        div { class: "flex items-center gap-2 mb-1",
                                            span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                                "Applied"
                                            }
                                            p { class: "font-medium text-gray-900",
                                                "m003_add_posts_table"
                                            }
                                        }
                                        p { class: "text-xs text-gray-500",
                                            "Applied on Dec 1, 2025 at 10:30 AM"
                                        }
                                    }
                                    button { class: "text-sm text-blue-600 hover:text-blue-800",
                                        "View SQL"
                                    }
                                }
                                p { class: "text-sm text-gray-600",
                                    "Created posts table with user_id foreign key and indexes"
                                }
                            }
                            // Migration 2
                            div { class: "border border-gray-200 rounded-lg p-4",
                                div { class: "flex items-start justify-between mb-3",
                                    div {
                                        div { class: "flex items-center gap-2 mb-1",
                                            span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                                "Applied"
                                            }
                                            p { class: "font-medium text-gray-900",
                                                "m002_add_email_verification"
                                            }
                                        }
                                        p { class: "text-xs text-gray-500",
                                            "Applied on Nov 20, 2025 at 2:15 PM"
                                        }
                                    }
                                    button { class: "text-sm text-blue-600 hover:text-blue-800",
                                        "View SQL"
                                    }
                                }
                                p { class: "text-sm text-gray-600",
                                    "Added email_verified and verification_token columns to users table"
                                }
                            }
                            // Migration 3
                            div { class: "border border-gray-200 rounded-lg p-4",
                                div { class: "flex items-start justify-between mb-3",
                                    div {
                                        div { class: "flex items-center gap-2 mb-1",
                                            span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                                "Applied"
                                            }
                                            p { class: "font-medium text-gray-900",
                                                "m001_create_users_table"
                                            }
                                        }
                                        p { class: "text-xs text-gray-500",
                                            "Applied on Nov 15, 2025 at 9:00 AM"
                                        }
                                    }
                                    button { class: "text-sm text-blue-600 hover:text-blue-800",
                                        "View SQL"
                                    }
                                }
                                p { class: "text-sm text-gray-600",
                                    "Initial migration - created users table with authentication fields"
                                }
                            }
                        }
                    }
                }
            }
            // Backups Tab Content
            if active_tab() == "backups" {
                div { class: "max-w-7xl mx-auto space-y-6",
                    // Backup Settings
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Backup Settings" }
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between p-4 bg-gray-50 rounded-lg",
                                div {
                                    p { class: "font-medium text-gray-900", "Automatic Daily Backups" }
                                    p { class: "text-sm text-gray-600",
                                        "Backups run every day at 2:00 AM UTC"
                                    }
                                }
                                button { class: "relative inline-flex h-6 w-11 items-center rounded-full bg-blue-600",
                                    span { class: "inline-block h-4 w-4 transform rounded-full bg-white transition translate-x-6" }
                                }
                            }
                            div { class: "flex gap-3",
                                button { class: "px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                    "Create Backup Now"
                                }
                                button { class: "px-6 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                                    "Restore from Backup"
                                }
                            }
                        }
                    }
                    // Backup History
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Recent Backups" }
                        div { class: "space-y-3",
                            // Backup 1
                            div { class: "flex items-center justify-between p-4 border border-gray-200 rounded-lg hover:border-gray-300 transition",
                                div {
                                    p { class: "font-medium text-gray-900",
                                        "backup_2025_12_03_02_00.sql"
                                    }
                                    p { class: "text-sm text-gray-600 mt-1",
                                        "Dec 3, 2025 at 2:00 AM • Size: 12.4 MB • Automatic"
                                    }
                                }
                                div { class: "flex gap-2",
                                    button { class: "px-3 py-1 bg-blue-100 hover:bg-blue-200 text-blue-700 text-sm font-medium rounded",
                                        "Restore"
                                    }
                                    button { class: "px-3 py-1 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-medium rounded",
                                        "Download"
                                    }
                                }
                            }
                            // Backup 2
                            div { class: "flex items-center justify-between p-4 border border-gray-200 rounded-lg hover:border-gray-300 transition",
                                div {
                                    p { class: "font-medium text-gray-900",
                                        "backup_2025_12_02_02_00.sql"
                                    }
                                    p { class: "text-sm text-gray-600 mt-1",
                                        "Dec 2, 2025 at 2:00 AM • Size: 12.1 MB • Automatic"
                                    }
                                }
                                div { class: "flex gap-2",
                                    button { class: "px-3 py-1 bg-blue-100 hover:bg-blue-200 text-blue-700 text-sm font-medium rounded",
                                        "Restore"
                                    }
                                    button { class: "px-3 py-1 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-medium rounded",
                                        "Download"
                                    }
                                }
                            }
                            // Backup 3
                            div { class: "flex items-center justify-between p-4 border border-gray-200 rounded-lg hover:border-gray-300 transition",
                                div {
                                    p { class: "font-medium text-gray-900",
                                        "manual_backup_pre_migration.sql"
                                    }
                                    p { class: "text-sm text-gray-600 mt-1",
                                        "Dec 1, 2025 at 10:15 AM • Size: 11.8 MB • Manual"
                                    }
                                }
                                div { class: "flex gap-2",
                                    button { class: "px-3 py-1 bg-blue-100 hover:bg-blue-200 text-blue-700 text-sm font-medium rounded",
                                        "Restore"
                                    }
                                    button { class: "px-3 py-1 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-medium rounded",
                                        "Download"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
