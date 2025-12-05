use dioxus::prelude::*;
use shared::models::{CreateTableRequest, ColumnDefinition, ColumnDataType};
use crate::hooks::{use_list_tables, use_create_table, use_delete_table};

/// The Table Editor page - Excel-like interface for editing table structure and data
#[component]
pub fn TableEditor(id: String) -> Element {
    let mut selected_table = use_signal(|| None::<String>);
    let mut show_create_modal = use_signal(|| false);
    let mut show_api_modal = use_signal(|| false);
    let mut show_rls_modal = use_signal(|| false);
    
    // Fetch tables for this project
    let tables_resource = use_list_tables(id.clone());
    let create_table = use_create_table(id.clone());
    let delete_table = use_delete_table(id.clone());
    
    // Form state for creating a new table
    let mut table_name = use_signal(|| String::new());
    let mut display_name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut columns = use_signal(|| Vec::<ColumnDefinition>::new());
    
    rsx! {
        div { class: "flex h-screen bg-gray-50",
            // Left Sidebar - Tables List
            div { class: "w-64 bg-white border-r border-gray-200 flex flex-col",
                // Sidebar Header
                div { class: "p-4 border-b border-gray-200",
                    h2 { class: "text-lg font-semibold text-gray-900", "Tables" }
                    p { class: "text-xs text-gray-500 mt-1", "Project: {id}" }
                }
                // Search Tables
                div { class: "p-4 border-b border-gray-200",
                    input {
                        r#type: "text",
                        placeholder: "Search tables...",
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none",
                    }
                }
                // Tables List
                div { class: "flex-1 overflow-y-auto",
                    div { class: "p-2 space-y-1",
                        match &*tables_resource.read_unchecked() {
                            Some(Ok(tables)) => rsx! {
                                for table in tables {
                                    button {
                                        key: "{table.table_name}",
                                        class: if selected_table() == Some(table.table_name.clone()) { 
                                            "w-full text-left px-3 py-2 bg-blue-50 border-l-4 border-blue-600 rounded-r hover:bg-blue-100 transition group" 
                                        } else { 
                                            "w-full text-left px-3 py-2 hover:bg-gray-100 rounded transition group" 
                                        },
                                        onclick: {
                                            let table_name = table.table_name.clone();
                                            move |_| selected_table.set(Some(table_name.clone()))
                                        },
                                        div { class: "flex items-center justify-between",
                                            div {
                                                p { class: "font-medium text-gray-900 text-sm",
                                                    "📊 {table.display_name}"
                                                }
                                                p { class: "text-xs text-gray-500", 
                                                    "{table.row_count.unwrap_or(0)} rows • {table.column_count} columns" 
                                                }
                                            }
                                            span { class: "text-gray-400 group-hover:text-gray-600 text-xs",
                                                "→"
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                div { class: "p-4 text-red-600 text-sm",
                                    "Error loading tables: {e}"
                                }
                            },
                            None => rsx! {
                                div { class: "p-4 text-gray-500 text-sm",
                                    "Loading tables..."
                                }
                            }
                        }
                    }
                }
                // Add Table Button
                div { class: "p-4 border-t border-gray-200",
                    button { class: "w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition text-sm",
                        "+ New Table"
                    }
                }
            }
            // Main Content Area
            div { class: "flex-1 flex flex-col overflow-hidden",
                // Top Toolbar
                div { class: "bg-white border-b border-gray-200 px-6 py-4",
                    div { class: "flex items-center justify-between",
                        // Table Info
                        div {
                            h1 { class: "text-2xl font-bold text-gray-900", "{selected_table}" }
                            p { class: "text-sm text-gray-600 mt-1",
                                "3,847 rows • 5 columns • Last modified 2 hours ago"
                            }
                        }
                        // Action Buttons
                        div { class: "flex gap-3",
                            button {
                                class: "px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white font-medium rounded-md transition text-sm flex items-center gap-2",
                                onclick: move |_| show_rls_modal.set(true),
                                "🛡️ RLS Settings"
                            }
                            button {
                                class: "px-4 py-2 bg-green-600 hover:bg-green-700 text-white font-medium rounded-md transition text-sm flex items-center gap-2",
                                onclick: move |_| show_api_modal.set(true),
                                "🔗 API Endpoint"
                            }
                            button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition text-sm",
                                "+ Add Row"
                            }
                        }
                    }
                }
                // Excel-like Grid
                div { class: "flex-1 overflow-auto p-6 bg-gray-50",
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden",
                        // Grid Table
                        div { class: "overflow-x-auto",
                            table { class: "min-w-full border-collapse",
                                // Header Row
                                thead { class: "bg-gray-100 sticky top-0 z-10",
                                    tr {
                                        // Row Number Header
                                        th { class: "w-12 px-3 py-3 border-r border-b border-gray-300 bg-gray-200 text-center text-xs font-semibold text-gray-600",
                                            "#"
                                        }
                                        // Column Headers
                                        th { class: "px-4 py-3 border-r border-b border-gray-300 text-left min-w-[150px]",
                                            div { class: "flex items-center justify-between group",
                                                div {
                                                    p { class: "font-semibold text-gray-900 text-sm",
                                                        "id"
                                                    }
                                                    p { class: "text-xs text-gray-500 font-normal",
                                                        "SERIAL"
                                                    }
                                                }
                                                button { class: "opacity-0 group-hover:opacity-100 text-gray-400 hover:text-gray-600 transition",
                                                    "⋮"
                                                }
                                            }
                                        }
                                        th { class: "px-4 py-3 border-r border-b border-gray-300 text-left min-w-[200px]",
                                            div { class: "flex items-center justify-between group",
                                                div {
                                                    p { class: "font-semibold text-gray-900 text-sm",
                                                        "email"
                                                    }
                                                    p { class: "text-xs text-gray-500 font-normal",
                                                        "VARCHAR(255)"
                                                    }
                                                }
                                                button { class: "opacity-0 group-hover:opacity-100 text-gray-400 hover:text-gray-600 transition",
                                                    "⋮"
                                                }
                                            }
                                        }
                                        th { class: "px-4 py-3 border-r border-b border-gray-300 text-left min-w-[180px]",
                                            div { class: "flex items-center justify-between group",
                                                div {
                                                    p { class: "font-semibold text-gray-900 text-sm",
                                                        "name"
                                                    }
                                                    p { class: "text-xs text-gray-500 font-normal",
                                                        "VARCHAR(100)"
                                                    }
                                                }
                                                button { class: "opacity-0 group-hover:opacity-100 text-gray-400 hover:text-gray-600 transition",
                                                    "⋮"
                                                }
                                            }
                                        }
                                        th { class: "px-4 py-3 border-r border-b border-gray-300 text-left min-w-[160px]",
                                            div { class: "flex items-center justify-between group",
                                                div {
                                                    p { class: "font-semibold text-gray-900 text-sm",
                                                        "role"
                                                    }
                                                    p { class: "text-xs text-gray-500 font-normal",
                                                        "VARCHAR(50)"
                                                    }
                                                }
                                                button { class: "opacity-0 group-hover:opacity-100 text-gray-400 hover:text-gray-600 transition",
                                                    "⋮"
                                                }
                                            }
                                        }
                                        th { class: "px-4 py-3 border-b border-gray-300 text-left min-w-[180px]",
                                            div { class: "flex items-center justify-between group",
                                                div {
                                                    p { class: "font-semibold text-gray-900 text-sm",
                                                        "created_at"
                                                    }
                                                    p { class: "text-xs text-gray-500 font-normal",
                                                        "TIMESTAMP"
                                                    }
                                                }
                                                button { class: "opacity-0 group-hover:opacity-100 text-gray-400 hover:text-gray-600 transition",
                                                    "⋮"
                                                }
                                            }
                                        }
                                    }
                                }
                                // Data Rows
                                tbody { class: "bg-white",
                                    // Row 1
                                    tr { class: "hover:bg-blue-50 group",
                                        td { class: "px-3 py-2 border-r border-b border-gray-200 bg-gray-50 text-center text-sm font-medium text-gray-600",
                                            "1"
                                        }
                                        td {
                                            class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            onclick: move |_| editing_cell.set(Some((1, 1))),
                                            "1"
                                        }
                                        td {
                                            class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            onclick: move |_| editing_cell.set(Some((1, 2))),
                                            "john@example.com"
                                        }
                                        td {
                                            class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            onclick: move |_| editing_cell.set(Some((1, 3))),
                                            "John Doe"
                                        }
                                        td {
                                            class: "px-4 py-2 border-r border-b border-gray-200 cursor-pointer hover:bg-blue-100",
                                            onclick: move |_| editing_cell.set(Some((1, 4))),
                                            span { class: "px-2 py-1 bg-purple-100 text-purple-800 text-xs font-semibold rounded",
                                                "admin"
                                            }
                                        }
                                        td {
                                            class: "px-4 py-2 border-b border-gray-200 text-sm text-gray-600 font-mono cursor-pointer hover:bg-blue-100",
                                            onclick: move |_| editing_cell.set(Some((1, 5))),
                                            "2025-11-15 10:30"
                                        }
                                    }
                                    // Row 2
                                    tr { class: "hover:bg-blue-50 group",
                                        td { class: "px-3 py-2 border-r border-b border-gray-200 bg-gray-50 text-center text-sm font-medium text-gray-600",
                                            "2"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "2"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "jane@example.com"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "Jane Smith"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 cursor-pointer hover:bg-blue-100",
                                            span { class: "px-2 py-1 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                "user"
                                            }
                                        }
                                        td { class: "px-4 py-2 border-b border-gray-200 text-sm text-gray-600 font-mono cursor-pointer hover:bg-blue-100",
                                            "2025-11-16 14:20"
                                        }
                                    }
                                    // Row 3
                                    tr { class: "hover:bg-blue-50 group",
                                        td { class: "px-3 py-2 border-r border-b border-gray-200 bg-gray-50 text-center text-sm font-medium text-gray-600",
                                            "3"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "3"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "bob@example.com"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "Bob Wilson"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 cursor-pointer hover:bg-blue-100",
                                            span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                                "moderator"
                                            }
                                        }
                                        td { class: "px-4 py-2 border-b border-gray-200 text-sm text-gray-600 font-mono cursor-pointer hover:bg-blue-100",
                                            "2025-11-18 09:15"
                                        }
                                    }
                                    // Row 4
                                    tr { class: "hover:bg-blue-50 group",
                                        td { class: "px-3 py-2 border-r border-b border-gray-200 bg-gray-50 text-center text-sm font-medium text-gray-600",
                                            "4"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "4"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "alice@example.com"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "Alice Johnson"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 cursor-pointer hover:bg-blue-100",
                                            span { class: "px-2 py-1 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                "user"
                                            }
                                        }
                                        td { class: "px-4 py-2 border-b border-gray-200 text-sm text-gray-600 font-mono cursor-pointer hover:bg-blue-100",
                                            "2025-11-20 16:45"
                                        }
                                    }
                                    // Row 5
                                    tr { class: "hover:bg-blue-50 group",
                                        td { class: "px-3 py-2 border-r border-b border-gray-200 bg-gray-50 text-center text-sm font-medium text-gray-600",
                                            "5"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "5"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "charlie@example.com"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 text-sm text-gray-900 cursor-pointer hover:bg-blue-100",
                                            "Charlie Brown"
                                        }
                                        td { class: "px-4 py-2 border-r border-b border-gray-200 cursor-pointer hover:bg-blue-100",
                                            span { class: "px-2 py-1 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                "user"
                                            }
                                        }
                                        td { class: "px-4 py-2 border-b border-gray-200 text-sm text-gray-600 font-mono cursor-pointer hover:bg-blue-100",
                                            "2025-11-22 11:00"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Pagination
                    div { class: "mt-4 flex items-center justify-between bg-white rounded-lg shadow-sm border border-gray-200 px-4 py-3",
                        div { class: "flex items-center gap-2",
                            p { class: "text-sm text-gray-600", "Showing" }
                            select { class: "px-2 py-1 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none",
                                option { "5" }
                                option { "10" }
                                option { "25" }
                                option { "50" }
                                option { "100" }
                            }
                            p { class: "text-sm text-gray-600", "of 3,847 rows" }
                        }
                        div { class: "flex gap-2",
                            button {
                                class: "px-3 py-1 border border-gray-300 rounded hover:bg-gray-100 text-sm font-medium text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed",
                                disabled: true,
                                "← Previous"
                            }
                            button { class: "px-3 py-1 border border-gray-300 rounded hover:bg-gray-100 text-sm font-medium text-gray-700",
                                "Next →"
                            }
                        }
                    }
                }
            }
        }
        // API Endpoint Modal
        if show_api_modal() {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4",
                onclick: move |_| show_api_modal.set(false),
                div {
                    class: "bg-white rounded-lg shadow-xl max-w-3xl w-full p-6",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "flex items-center justify-between mb-4",
                        h2 { class: "text-2xl font-bold text-gray-900", "REST API Endpoints" }
                        button {
                            class: "text-gray-400 hover:text-gray-600 text-2xl",
                            onclick: move |_| show_api_modal.set(false),
                            "×"
                        }
                    }
                    div { class: "space-y-4",
                        // GET All
                        div { class: "bg-gray-50 rounded-lg p-4 border border-gray-200",
                            div { class: "flex items-center gap-3 mb-2",
                                span { class: "px-2 py-1 bg-green-100 text-green-800 font-semibold text-xs rounded",
                                    "GET"
                                }
                                p { class: "font-mono text-sm text-gray-900",
                                    "https://api.ferrisbase.com/{id}/users"
                                }
                            }
                            p { class: "text-sm text-gray-600", "Retrieve all users" }
                            button { class: "mt-2 px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded",
                                "📋 Copy"
                            }
                        }
                        // GET by ID
                        div { class: "bg-gray-50 rounded-lg p-4 border border-gray-200",
                            div { class: "flex items-center gap-3 mb-2",
                                span { class: "px-2 py-1 bg-green-100 text-green-800 font-semibold text-xs rounded",
                                    "GET"
                                }
                                p { class: "font-mono text-sm text-gray-900",
                                    "https://api.ferrisbase.com/{id}/users/:id"
                                }
                            }
                            p { class: "text-sm text-gray-600", "Retrieve a specific user by ID" }
                            button { class: "mt-2 px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded",
                                "📋 Copy"
                            }
                        }
                        // POST
                        div { class: "bg-gray-50 rounded-lg p-4 border border-gray-200",
                            div { class: "flex items-center gap-3 mb-2",
                                span { class: "px-2 py-1 bg-blue-100 text-blue-800 font-semibold text-xs rounded",
                                    "POST"
                                }
                                p { class: "font-mono text-sm text-gray-900",
                                    "https://api.ferrisbase.com/{id}/users"
                                }
                            }
                            p { class: "text-sm text-gray-600 mb-2", "Create a new user" }
                            div { class: "bg-gray-800 rounded p-3 text-xs font-mono text-green-400 overflow-x-auto",
                                pre {
                                    r#"{{
  "email": "new@example.com",
  "name": "New User",
  "role": "user"
}}"#
                                }
                            }
                            button { class: "mt-2 px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded",
                                "📋 Copy"
                            }
                        }
                        // PUT
                        div { class: "bg-gray-50 rounded-lg p-4 border border-gray-200",
                            div { class: "flex items-center gap-3 mb-2",
                                span { class: "px-2 py-1 bg-yellow-100 text-yellow-800 font-semibold text-xs rounded",
                                    "PUT"
                                }
                                p { class: "font-mono text-sm text-gray-900",
                                    "https://api.ferrisbase.com/{id}/users/:id"
                                }
                            }
                            p { class: "text-sm text-gray-600", "Update a user" }
                            button { class: "mt-2 px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded",
                                "📋 Copy"
                            }
                        }
                        // DELETE
                        div { class: "bg-gray-50 rounded-lg p-4 border border-gray-200",
                            div { class: "flex items-center gap-3 mb-2",
                                span { class: "px-2 py-1 bg-red-100 text-red-800 font-semibold text-xs rounded",
                                    "DELETE"
                                }
                                p { class: "font-mono text-sm text-gray-900",
                                    "https://api.ferrisbase.com/{id}/users/:id"
                                }
                            }
                            p { class: "text-sm text-gray-600", "Delete a user" }
                            button { class: "mt-2 px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded",
                                "📋 Copy"
                            }
                        }
                    }
                }
            }
        }
        // Row Level Security Modal
        if show_rls_modal() {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4",
                onclick: move |_| show_rls_modal.set(false),
                div {
                    class: "bg-white rounded-lg shadow-xl max-w-3xl w-full p-6",
                    onclick: move |e| e.stop_propagation(),
                    div { class: "flex items-center justify-between mb-4",
                        h2 { class: "text-2xl font-bold text-gray-900", "Row Level Security Settings" }
                        button {
                            class: "text-gray-400 hover:text-gray-600 text-2xl",
                            onclick: move |_| show_rls_modal.set(false),
                            "×"
                        }
                    }
                    p { class: "text-gray-600 mb-6",
                        "Control access to rows in the '{selected_table}' table based on user permissions and policies."
                    }
                    // Enable RLS Toggle
                    div { class: "bg-gray-50 rounded-lg p-4 border border-gray-200 mb-6",
                        div { class: "flex items-center justify-between",
                            div {
                                p { class: "font-semibold text-gray-900", "Enable Row Level Security" }
                                p { class: "text-sm text-gray-600 mt-1",
                                    "When enabled, only rows that match the policies can be accessed"
                                }
                            }
                            button { class: "relative inline-flex h-6 w-11 items-center rounded-full bg-blue-600",
                                span { class: "inline-block h-4 w-4 transform rounded-full bg-white transition translate-x-6" }
                            }
                        }
                    }
                    // Policies
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-semibold text-gray-900 mb-3", "Active Policies" }
                        // Policy 1
                        div { class: "border border-gray-200 rounded-lg p-4",
                            div { class: "flex items-start justify-between mb-2",
                                div {
                                    p { class: "font-medium text-gray-900",
                                        "Users can view their own data"
                                    }
                                    p { class: "text-xs text-gray-500 mt-1",
                                        "SELECT • Created Nov 15, 2025"
                                    }
                                }
                                span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                    "Active"
                                }
                            }
                            div { class: "mt-3 bg-gray-800 rounded p-3 text-xs font-mono text-green-400",
                                pre { "auth.uid() = user_id" }
                            }
                            div { class: "mt-3 flex gap-2",
                                button { class: "px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded",
                                    "Edit"
                                }
                                button { class: "px-3 py-1 bg-red-100 hover:bg-red-200 text-red-700 text-sm font-medium rounded",
                                    "Delete"
                                }
                            }
                        }
                        // Policy 2
                        div { class: "border border-gray-200 rounded-lg p-4",
                            div { class: "flex items-start justify-between mb-2",
                                div {
                                    p { class: "font-medium text-gray-900",
                                        "Admins can view all data"
                                    }
                                    p { class: "text-xs text-gray-500 mt-1",
                                        "SELECT • Created Nov 15, 2025"
                                    }
                                }
                                span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                    "Active"
                                }
                            }
                            div { class: "mt-3 bg-gray-800 rounded p-3 text-xs font-mono text-green-400",
                                pre { "auth.role() = 'admin'" }
                            }
                            div { class: "mt-3 flex gap-2",
                                button { class: "px-3 py-1 bg-gray-200 hover:bg-gray-300 text-gray-700 text-sm font-medium rounded",
                                    "Edit"
                                }
                                button { class: "px-3 py-1 bg-red-100 hover:bg-red-200 text-red-700 text-sm font-medium rounded",
                                    "Delete"
                                }
                            }
                        }
                        // Add Policy Button
                        button { class: "w-full px-4 py-3 border-2 border-dashed border-gray-300 rounded-lg hover:border-blue-500 hover:bg-blue-50 text-gray-600 hover:text-blue-600 font-medium transition",
                            "+ Add New Policy"
                        }
                    }
                }
            }
        }
    }
}
