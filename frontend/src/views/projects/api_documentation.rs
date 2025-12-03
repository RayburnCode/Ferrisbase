use dioxus::prelude::*;

/// The API Documentation page - showcasing auto-generated REST API endpoints
#[component]
pub fn APIDocs(id: String) -> Element {
    let mut selected_endpoint = use_signal(|| "get-users".to_string());
    let mut search_query = use_signal(|| String::new());
    
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-6",
                div { class: "bg-gradient-to-r from-blue-600 to-indigo-600 rounded-lg p-8 text-white mb-6",
                    div { class: "flex items-start justify-between",
                        div {
                            h1 { class: "text-4xl font-bold mb-2", "Auto-Generated REST API" }
                            p { class: "text-blue-100 text-lg mb-4", "Project: {id}" }
                            p { class: "text-blue-50 max-w-2xl",
                                "Your database schema automatically generates REST endpoints. No backend code required - just configure your tables and start making requests."
                            }
                        }
                        span { class: "text-6xl", "⚡" }
                    }
                    div { class: "mt-6 flex gap-3",
                        div { class: "bg-white/20 rounded-lg px-4 py-2",
                            p { class: "text-xs text-blue-100", "Base URL" }
                            p { class: "text-sm font-mono font-semibold",
                                "https://api.ferrisbase.io/v1/{id}"
                            }
                        }
                        div { class: "bg-white/20 rounded-lg px-4 py-2",
                            p { class: "text-xs text-blue-100", "Total Endpoints" }
                            p { class: "text-sm font-semibold", "24 endpoints" }
                        }
                        div { class: "bg-white/20 rounded-lg px-4 py-2",
                            p { class: "text-xs text-blue-100", "API Version" }
                            p { class: "text-sm font-semibold", "v1.0" }
                        }
                    }
                }
                // Quick Actions
                div { class: "flex flex-wrap gap-3 mb-6",
                    button { class: "px-4 py-2 bg-white text-gray-900 rounded-md shadow-sm hover:shadow-md font-medium transition",
                        "📥 Download OpenAPI Spec"
                    }
                    button { class: "px-4 py-2 bg-white text-gray-900 rounded-md shadow-sm hover:shadow-md font-medium transition",
                        "🔑 Manage API Keys"
                    }
                    button { class: "px-4 py-2 bg-white text-gray-900 rounded-md shadow-sm hover:shadow-md font-medium transition",
                        "🧪 Test in Playground"
                    }
                }
                // Search
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    input {
                        r#type: "text",
                        placeholder: "Search endpoints... (e.g., users, GET, /api/posts)",
                        class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                        value: "{search_query}",
                        oninput: move |evt| search_query.set(evt.value()),
                    }
                }
            }
            // Main Content Grid
            div { class: "max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Sidebar - Endpoints List
                div { class: "lg:col-span-1",
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden sticky top-6",
                        div { class: "bg-gray-50 border-b border-gray-200 px-4 py-3",
                            h2 { class: "text-lg font-semibold text-gray-900", "Endpoints" }
                        }
                        div { class: "divide-y divide-gray-200 max-h-[600px] overflow-y-auto",
                            // Users Endpoints
                            div { class: "p-3",
                                p { class: "text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2",
                                    "Users"
                                }
                                div { class: "space-y-1",
                                    button {
                                        class: if selected_endpoint() == "get-users" { "w-full text-left px-3 py-2 rounded-md bg-blue-50 border border-blue-200" } else { "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50" },
                                        onclick: move |_| selected_endpoint.set("get-users".to_string()),
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                                "GET"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/users"
                                            }
                                        }
                                    }
                                    button {
                                        class: if selected_endpoint() == "get-user-by-id" { "w-full text-left px-3 py-2 rounded-md bg-blue-50 border border-blue-200" } else { "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50" },
                                        onclick: move |_| selected_endpoint.set("get-user-by-id".to_string()),
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                                "GET"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/users/:id"
                                            }
                                        }
                                    }
                                    button {
                                        class: if selected_endpoint() == "create-user" { "w-full text-left px-3 py-2 rounded-md bg-blue-50 border border-blue-200" } else { "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50" },
                                        onclick: move |_| selected_endpoint.set("create-user".to_string()),
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                "POST"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/users"
                                            }
                                        }
                                    }
                                    button {
                                        class: if selected_endpoint() == "update-user" { "w-full text-left px-3 py-2 rounded-md bg-blue-50 border border-blue-200" } else { "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50" },
                                        onclick: move |_| selected_endpoint.set("update-user".to_string()),
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-orange-100 text-orange-800 text-xs font-semibold rounded",
                                                "PUT"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/users/:id"
                                            }
                                        }
                                    }
                                    button {
                                        class: if selected_endpoint() == "delete-user" { "w-full text-left px-3 py-2 rounded-md bg-blue-50 border border-blue-200" } else { "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50" },
                                        onclick: move |_| selected_endpoint.set("delete-user".to_string()),
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-red-100 text-red-800 text-xs font-semibold rounded",
                                                "DELETE"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/users/:id"
                                            }
                                        }
                                    }
                                }
                            }
                            // Posts Endpoints
                            div { class: "p-3",
                                p { class: "text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2",
                                    "Posts"
                                }
                                div { class: "space-y-1",
                                    button { class: "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50",
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                                "GET"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/posts"
                                            }
                                        }
                                    }
                                    button { class: "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50",
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                "POST"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/posts"
                                            }
                                        }
                                    }
                                }
                            }
                            // Auth Endpoints
                            div { class: "p-3",
                                p { class: "text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2",
                                    "Authentication"
                                }
                                div { class: "space-y-1",
                                    button { class: "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50",
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                "POST"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/auth/login"
                                            }
                                        }
                                    }
                                    button { class: "w-full text-left px-3 py-2 rounded-md hover:bg-gray-50",
                                        div { class: "flex items-center gap-2",
                                            span { class: "px-2 py-0.5 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                                "POST"
                                            }
                                            span { class: "text-sm text-gray-700 font-mono",
                                                "/auth/register"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Main Content - Endpoint Details
                div { class: "lg:col-span-2",
                    // Endpoint Header
                    if selected_endpoint() == "get-users" {
                        div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6 mb-6",
                            div { class: "flex items-start justify-between mb-4",
                                div {
                                    div { class: "flex items-center gap-3 mb-2",
                                        span { class: "px-3 py-1 bg-green-100 text-green-800 text-sm font-bold rounded",
                                            "GET"
                                        }
                                        h2 { class: "text-2xl font-bold text-gray-900",
                                            "Get All Users"
                                        }
                                    }
                                    p { class: "text-gray-600",
                                        "Retrieve a paginated list of all users in the database"
                                    }
                                }
                                button { class: "px-4 py-2 bg-blue-600 text-white rounded-md font-medium hover:bg-blue-700 transition",
                                    "Try It Out"
                                }
                            }
                            // Endpoint URL
                            div { class: "bg-gray-50 rounded-lg p-4 mb-4",
                                p { class: "text-xs font-semibold text-gray-500 uppercase mb-2",
                                    "Endpoint"
                                }
                                div { class: "flex items-center gap-2",
                                    code { class: "flex-1 bg-gray-900 text-green-400 px-4 py-2 rounded font-mono text-sm",
                                        "GET https://api.ferrisbase.io/v1/{id}/users"
                                    }
                                    button { class: "px-3 py-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-medium",
                                        "📋 Copy"
                                    }
                                }
                            }
                            // Query Parameters
                            div { class: "mb-4",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-3",
                                    "Query Parameters"
                                }
                                div { class: "border border-gray-200 rounded-lg overflow-hidden",
                                    table { class: "min-w-full divide-y divide-gray-200",
                                        thead { class: "bg-gray-50",
                                            tr {
                                                th { class: "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                    "Parameter"
                                                }
                                                th { class: "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                    "Type"
                                                }
                                                th { class: "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                    "Required"
                                                }
                                                th { class: "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase",
                                                    "Description"
                                                }
                                            }
                                        }
                                        tbody { class: "bg-white divide-y divide-gray-200",
                                            tr {
                                                td { class: "px-4 py-2 text-sm font-mono text-gray-900",
                                                    "page"
                                                }
                                                td { class: "px-4 py-2 text-sm text-gray-600",
                                                    "integer"
                                                }
                                                td { class: "px-4 py-2 text-sm",
                                                    span { class: "text-gray-500", "Optional" }
                                                }
                                                td { class: "px-4 py-2 text-sm text-gray-600",
                                                    "Page number (default: 1)"
                                                }
                                            }
                                            tr {
                                                td { class: "px-4 py-2 text-sm font-mono text-gray-900",
                                                    "limit"
                                                }
                                                td { class: "px-4 py-2 text-sm text-gray-600",
                                                    "integer"
                                                }
                                                td { class: "px-4 py-2 text-sm",
                                                    span { class: "text-gray-500", "Optional" }
                                                }
                                                td { class: "px-4 py-2 text-sm text-gray-600",
                                                    "Results per page (default: 10, max: 100)"
                                                }
                                            }
                                            tr {
                                                td { class: "px-4 py-2 text-sm font-mono text-gray-900",
                                                    "sort"
                                                }
                                                td { class: "px-4 py-2 text-sm text-gray-600",
                                                    "string"
                                                }
                                                td { class: "px-4 py-2 text-sm",
                                                    span { class: "text-gray-500", "Optional" }
                                                }
                                                td { class: "px-4 py-2 text-sm text-gray-600",
                                                    "Sort field (e.g., created_at, name)"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // Example Request
                            div { class: "mb-4",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-3",
                                    "Example Request"
                                }
                                div { class: "bg-gray-900 rounded-lg p-4 overflow-x-auto",
                                    pre { class: "text-sm text-gray-100 font-mono",
                                        "curl -X GET \\\\\n  'https://api.ferrisbase.io/v1/{id}/users?page=1&limit=10' \\\\\n  -H 'Authorization: Bearer YOUR_API_KEY' \\\\\n  -H 'Content-Type: application/json'"
                                    }
                                }
                            }
                            // Response
                            div {
                                h3 { class: "text-lg font-semibold text-gray-900 mb-3",
                                    "Response (200 OK)"
                                }
                                div { class: "bg-gray-900 rounded-lg p-4 overflow-x-auto",
                                    pre { class: "text-sm text-gray-100 font-mono",
                                        "{{\n  \"data\": [\n    {{\n      \"id\": 1,\n      \"name\": \"John Doe\",\n      \"email\": \"john@example.com\",\n      \"created_at\": \"2025-12-01T10:30:00Z\",\n      \"updated_at\": \"2025-12-01T10:30:00Z\"\n    }},\n    {{\n      \"id\": 2,\n      \"name\": \"Jane Smith\",\n      \"email\": \"jane@example.com\",\n      \"created_at\": \"2025-12-02T14:20:00Z\",\n      \"updated_at\": \"2025-12-02T14:20:00Z\"\n    }}\n  ],\n  \"meta\": {{\n    \"page\": 1,\n    \"limit\": 10,\n    \"total\": 42,\n    \"total_pages\": 5\n  }}\n}}"
                                    }
                                }
                            }
                        }
                    }
                    // Response Codes
                    div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                        h3 { class: "text-lg font-semibold text-gray-900 mb-3", "Response Codes" }
                        div { class: "space-y-2",
                            div { class: "flex items-start gap-3 p-3 bg-green-50 rounded-md",
                                span { class: "px-2 py-1 bg-green-600 text-white text-xs font-bold rounded",
                                    "200"
                                }
                                div {
                                    p { class: "font-medium text-gray-900", "Success" }
                                    p { class: "text-sm text-gray-600",
                                        "Request completed successfully"
                                    }
                                }
                            }
                            div { class: "flex items-start gap-3 p-3 bg-yellow-50 rounded-md",
                                span { class: "px-2 py-1 bg-yellow-600 text-white text-xs font-bold rounded",
                                    "400"
                                }
                                div {
                                    p { class: "font-medium text-gray-900", "Bad Request" }
                                    p { class: "text-sm text-gray-600",
                                        "Invalid parameters or malformed request"
                                    }
                                }
                            }
                            div { class: "flex items-start gap-3 p-3 bg-red-50 rounded-md",
                                span { class: "px-2 py-1 bg-red-600 text-white text-xs font-bold rounded",
                                    "401"
                                }
                                div {
                                    p { class: "font-medium text-gray-900", "Unauthorized" }
                                    p { class: "text-sm text-gray-600", "Missing or invalid API key" }
                                }
                            }
                            div { class: "flex items-start gap-3 p-3 bg-orange-50 rounded-md",
                                span { class: "px-2 py-1 bg-orange-600 text-white text-xs font-bold rounded",
                                    "429"
                                }
                                div {
                                    p { class: "font-medium text-gray-900", "Too Many Requests" }
                                    p { class: "text-sm text-gray-600", "Rate limit exceeded" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
