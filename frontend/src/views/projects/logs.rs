use dioxus::prelude::*;

/// The Logs page component that displays project activity logs
#[component]
pub fn Logs(id: String) -> Element {
    let mut log_level = use_signal(|| "all".to_string());
    let mut log_source = use_signal(|| "all".to_string());
    let mut search_query = use_signal(|| String::new());
    
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-6",
                div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Activity Logs" }
                        p { class: "text-gray-600 mt-1", "Project: {id}" }
                    }
                    div { class: "flex gap-3",
                        button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                            "🔄 Refresh"
                        }
                        button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-md transition",
                            "📥 Export Logs"
                        }
                    }
                }
                // Filters and Search Section
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    div { class: "flex flex-col lg:flex-row gap-4",
                        // Search Input
                        div { class: "flex-1",
                            input {
                                r#type: "text",
                                placeholder: "Search logs... (e.g., error, user@example.com, /api/users)",
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition",
                                value: "{search_query}",
                                oninput: move |evt| search_query.set(evt.value()),
                            }
                        }
                        // Log Level Filter
                        div { class: "w-full lg:w-48",
                            select {
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                                value: "{log_level}",
                                onchange: move |evt| log_level.set(evt.value()),
                                option { value: "all", "All Levels" }
                                option { value: "info", "Info" }
                                option { value: "warning", "Warning" }
                                option { value: "error", "Error" }
                                option { value: "critical", "Critical" }
                            }
                        }
                        // Source Filter
                        div { class: "w-full lg:w-48",
                            select {
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                                value: "{log_source}",
                                onchange: move |evt| log_source.set(evt.value()),
                                option { value: "all", "All Sources" }
                                option { value: "database", "Database" }
                                option { value: "api", "API" }
                                option { value: "auth", "Authentication" }
                                option { value: "system", "System" }
                            }
                        }
                    }
                }
            }
            // Stats Overview
            div { class: "max-w-7xl mx-auto mb-6 grid grid-cols-2 lg:grid-cols-4 gap-4",
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    div { class: "flex items-center justify-between",
                        span { class: "text-sm font-medium text-gray-600", "Total Logs" }
                        span { class: "text-xl", "📝" }
                    }
                    p { class: "text-2xl font-bold text-gray-900 mt-2", "1,247" }
                }
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    div { class: "flex items-center justify-between",
                        span { class: "text-sm font-medium text-gray-600", "Errors" }
                        span { class: "text-xl", "❌" }
                    }
                    p { class: "text-2xl font-bold text-red-600 mt-2", "23" }
                }
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    div { class: "flex items-center justify-between",
                        span { class: "text-sm font-medium text-gray-600", "Warnings" }
                        span { class: "text-xl", "⚠️" }
                    }
                    p { class: "text-2xl font-bold text-orange-600 mt-2", "87" }
                }
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    div { class: "flex items-center justify-between",
                        span { class: "text-sm font-medium text-gray-600", "Last Updated" }
                        span { class: "text-xl", "🕐" }
                    }
                    p { class: "text-sm font-semibold text-gray-900 mt-2", "2 seconds ago" }
                }
            }
            // Logs List
            div { class: "max-w-7xl mx-auto",
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden",
                    // Logs Header
                    div { class: "bg-gray-50 border-b border-gray-200 px-6 py-3",
                        div { class: "flex items-center justify-between",
                            h2 { class: "text-lg font-semibold text-gray-900", "Recent Activity" }
                            span { class: "text-sm text-gray-500", "Showing last 50 entries" }
                        }
                    }
                    // Logs Entries
                    div { class: "divide-y divide-gray-200",
                        // Error Log Entry
                        div { class: "px-6 py-4 hover:bg-gray-50 transition",
                            div { class: "flex items-start gap-4",
                                // Timestamp
                                div { class: "w-32 flex-shrink-0",
                                    p { class: "text-xs font-mono text-gray-500",
                                        "2025-12-03"
                                    }
                                    p { class: "text-xs font-mono text-gray-500",
                                        "14:32:18"
                                    }
                                }
                                // Level Badge
                                div { class: "flex-shrink-0",
                                    span { class: "px-2 py-1 bg-red-100 text-red-800 text-xs font-semibold rounded uppercase",
                                        "Error"
                                    }
                                }
                                // Message and Details
                                div { class: "flex-1 min-w-0",
                                    p { class: "text-sm font-medium text-gray-900",
                                        "Database connection timeout"
                                    }
                                    p { class: "text-xs text-gray-600 mt-1",
                                        "Source: Database • User: system@ferrisbase.io • IP: 192.168.1.100"
                                    }
                                    details { class: "mt-2",
                                        summary { class: "text-xs text-blue-600 hover:text-blue-700 cursor-pointer",
                                            "View Details"
                                        }
                                        div { class: "mt-2 p-3 bg-gray-50 rounded text-xs font-mono text-gray-700",
                                            "Error: Connection to database failed after 30s timeout\\nStack trace: /app/database/connection.rs:42"
                                        }
                                    }
                                }
                            }
                        }
                        // Warning Log Entry
                        div { class: "px-6 py-4 hover:bg-gray-50 transition",
                            div { class: "flex items-start gap-4",
                                div { class: "w-32 flex-shrink-0",
                                    p { class: "text-xs font-mono text-gray-500",
                                        "2025-12-03"
                                    }
                                    p { class: "text-xs font-mono text-gray-500",
                                        "14:31:45"
                                    }
                                }
                                div { class: "flex-shrink-0",
                                    span { class: "px-2 py-1 bg-orange-100 text-orange-800 text-xs font-semibold rounded uppercase",
                                        "Warning"
                                    }
                                }
                                div { class: "flex-1 min-w-0",
                                    p { class: "text-sm font-medium text-gray-900",
                                        "High memory usage detected"
                                    }
                                    p { class: "text-xs text-gray-600 mt-1",
                                        "Source: System • Memory: 87% • Threshold: 80%"
                                    }
                                }
                            }
                        }
                        // Info Log Entry
                        div { class: "px-6 py-4 hover:bg-gray-50 transition",
                            div { class: "flex items-start gap-4",
                                div { class: "w-32 flex-shrink-0",
                                    p { class: "text-xs font-mono text-gray-500",
                                        "2025-12-03"
                                    }
                                    p { class: "text-xs font-mono text-gray-500",
                                        "14:30:22"
                                    }
                                }
                                div { class: "flex-shrink-0",
                                    span { class: "px-2 py-1 bg-blue-100 text-blue-800 text-xs font-semibold rounded uppercase",
                                        "Info"
                                    }
                                }
                                div { class: "flex-1 min-w-0",
                                    p { class: "text-sm font-medium text-gray-900",
                                        "User login successful"
                                    }
                                    p { class: "text-xs text-gray-600 mt-1",
                                        "Source: Authentication • User: john.doe@example.com • IP: 203.0.113.42"
                                    }
                                }
                            }
                        }
                        // API Request Log
                        div { class: "px-6 py-4 hover:bg-gray-50 transition",
                            div { class: "flex items-start gap-4",
                                div { class: "w-32 flex-shrink-0",
                                    p { class: "text-xs font-mono text-gray-500",
                                        "2025-12-03"
                                    }
                                    p { class: "text-xs font-mono text-gray-500",
                                        "14:29:58"
                                    }
                                }
                                div { class: "flex-shrink-0",
                                    span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded uppercase",
                                        "Info"
                                    }
                                }
                                div { class: "flex-1 min-w-0",
                                    p { class: "text-sm font-medium text-gray-900",
                                        "GET /api/users - 200 OK"
                                    }
                                    p { class: "text-xs text-gray-600 mt-1",
                                        "Source: API • Duration: 124ms • Size: 2.3 KB"
                                    }
                                }
                            }
                        }
                        // Database Query Log
                        div { class: "px-6 py-4 hover:bg-gray-50 transition",
                            div { class: "flex items-start gap-4",
                                div { class: "w-32 flex-shrink-0",
                                    p { class: "text-xs font-mono text-gray-500",
                                        "2025-12-03"
                                    }
                                    p { class: "text-xs font-mono text-gray-500",
                                        "14:29:15"
                                    }
                                }
                                div { class: "flex-shrink-0",
                                    span { class: "px-2 py-1 bg-purple-100 text-purple-800 text-xs font-semibold rounded uppercase",
                                        "Info"
                                    }
                                }
                                div { class: "flex-1 min-w-0",
                                    p { class: "text-sm font-medium text-gray-900",
                                        "Database query executed"
                                    }
                                    p { class: "text-xs text-gray-600 mt-1",
                                        "Source: Database • Query: SELECT • Duration: 42ms • Rows: 150"
                                    }
                                    details { class: "mt-2",
                                        summary { class: "text-xs text-blue-600 hover:text-blue-700 cursor-pointer",
                                            "View Query"
                                        }
                                        div { class: "mt-2 p-3 bg-gray-50 rounded text-xs font-mono text-gray-700",
                                            "SELECT * FROM users WHERE created_at > '2025-12-01' LIMIT 150"
                                        }
                                    }
                                }
                            }
                        }
                        // System Event Log
                        div { class: "px-6 py-4 hover:bg-gray-50 transition",
                            div { class: "flex items-start gap-4",
                                div { class: "w-32 flex-shrink-0",
                                    p { class: "text-xs font-mono text-gray-500",
                                        "2025-12-03"
                                    }
                                    p { class: "text-xs font-mono text-gray-500",
                                        "14:28:30"
                                    }
                                }
                                div { class: "flex-shrink-0",
                                    span { class: "px-2 py-1 bg-gray-100 text-gray-800 text-xs font-semibold rounded uppercase",
                                        "Info"
                                    }
                                }
                                div { class: "flex-1 min-w-0",
                                    p { class: "text-sm font-medium text-gray-900",
                                        "Backup completed successfully"
                                    }
                                    p { class: "text-xs text-gray-600 mt-1",
                                        "Source: System • Type: Automated • Size: 1.2 GB • Duration: 3m 42s"
                                    }
                                }
                            }
                        }
                    }
                    // Pagination
                    div { class: "bg-gray-50 border-t border-gray-200 px-6 py-4",
                        div { class: "flex items-center justify-between",
                            p { class: "text-sm text-gray-700", "Showing 1 to 6 of 1,247 entries" }
                            div { class: "flex gap-2",
                                button {
                                    class: "px-3 py-1 border border-gray-300 rounded-md bg-white hover:bg-gray-50 text-sm font-medium text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed",
                                    disabled: true,
                                    "Previous"
                                }
                                button { class: "px-3 py-1 bg-blue-600 text-white rounded-md text-sm font-medium",
                                    "1"
                                }
                                button { class: "px-3 py-1 border border-gray-300 rounded-md bg-white hover:bg-gray-50 text-sm font-medium text-gray-700",
                                    "2"
                                }
                                button { class: "px-3 py-1 border border-gray-300 rounded-md bg-white hover:bg-gray-50 text-sm font-medium text-gray-700",
                                    "3"
                                }
                                button { class: "px-3 py-1 border border-gray-300 rounded-md bg-white hover:bg-gray-50 text-sm font-medium text-gray-700",
                                    "Next"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
