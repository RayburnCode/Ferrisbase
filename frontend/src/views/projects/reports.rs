use dioxus::prelude::*;

/// The Reports page component that displays project analytics and reports
#[component]
pub fn Reports(id: String) -> Element {
    let mut date_range = use_signal(|| "last-7-days".to_string());
    let mut report_type = use_signal(|| "all".to_string());
    
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-6",
                div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "Reports & Analytics" }
                        p { class: "text-gray-600 mt-1", "Project: {id}" }
                    }
                    button { class: "inline-flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold px-6 py-3 rounded-md shadow-sm transition duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                        span { class: "text-lg", "📥" }
                        "Export Report"
                    }
                }
                // Filters Section
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4",
                    div { class: "flex flex-col sm:flex-row gap-4",
                        div { class: "flex-1",
                            label { class: "block text-sm font-medium text-gray-700 mb-2",
                                "Date Range"
                            }
                            select {
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                                value: "{date_range}",
                                onchange: move |evt| date_range.set(evt.value()),
                                option { value: "today", "Today" }
                                option { value: "last-7-days", "Last 7 Days" }
                                option { value: "last-30-days", "Last 30 Days" }
                                option { value: "last-90-days", "Last 90 Days" }
                                option { value: "custom", "Custom Range" }
                            }
                        }
                        div { class: "flex-1",
                            label { class: "block text-sm font-medium text-gray-700 mb-2",
                                "Report Type"
                            }
                            select {
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition bg-white",
                                value: "{report_type}",
                                onchange: move |evt| report_type.set(evt.value()),
                                option { value: "all", "All Reports" }
                                option { value: "database", "Database Activity" }
                                option { value: "api", "API Usage" }
                                option { value: "auth", "Authentication" }
                                option { value: "performance", "Performance" }
                            }
                        }
                        button { class: "self-end px-6 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition",
                            "Apply Filters"
                        }
                    }
                }
            }
            // Key Metrics Overview
            div { class: "max-w-7xl mx-auto mb-6",
                h2 { class: "text-xl font-semibold text-gray-900 mb-4", "Key Metrics" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
                    // Total Requests
                    div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-sm font-medium text-gray-600", "Total Requests" }
                            span { class: "text-2xl", "📊" }
                        }
                        p { class: "text-3xl font-bold text-gray-900", "42,581" }
                        p { class: "text-sm text-green-600 mt-1", "↑ 12.5% from last period" }
                    }
                    // Database Queries
                    div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-sm font-medium text-gray-600", "DB Queries" }
                            span { class: "text-2xl", "🗄️" }
                        }
                        p { class: "text-3xl font-bold text-gray-900", "28,934" }
                        p { class: "text-sm text-green-600 mt-1", "↑ 8.3% from last period" }
                    }
                    // Auth Events
                    div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-sm font-medium text-gray-600", "Auth Events" }
                            span { class: "text-2xl", "🔐" }
                        }
                        p { class: "text-3xl font-bold text-gray-900", "5,847" }
                        p { class: "text-sm text-red-600 mt-1", "↓ 3.2% from last period" }
                    }
                    // Avg Response Time
                    div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-5",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-sm font-medium text-gray-600", "Avg Response" }
                            span { class: "text-2xl", "⚡" }
                        }
                        p { class: "text-3xl font-bold text-gray-900", "142ms" }
                        p { class: "text-sm text-green-600 mt-1", "↓ 5.1% faster" }
                    }
                }
            }
            // Detailed Reports Grid
            div { class: "max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-6",
                // Database Activity Report
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                    div { class: "flex items-center justify-between mb-4",
                        h3 { class: "text-lg font-semibold text-gray-900", "Database Activity" }
                        button { class: "text-sm text-blue-600 hover:text-blue-700 font-medium",
                            "View Details →"
                        }
                    }
                    div { class: "space-y-3",
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            span { class: "text-sm text-gray-700", "SELECT queries" }
                            span { class: "text-sm font-semibold text-gray-900", "18,542" }
                        }
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            span { class: "text-sm text-gray-700", "INSERT queries" }
                            span { class: "text-sm font-semibold text-gray-900", "6,231" }
                        }
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            span { class: "text-sm text-gray-700", "UPDATE queries" }
                            span { class: "text-sm font-semibold text-gray-900", "3,847" }
                        }
                        div { class: "flex items-center justify-between py-2",
                            span { class: "text-sm text-gray-700", "DELETE queries" }
                            span { class: "text-sm font-semibold text-gray-900", "314" }
                        }
                    }
                }
                // API Endpoints Report
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                    div { class: "flex items-center justify-between mb-4",
                        h3 { class: "text-lg font-semibold text-gray-900", "Top API Endpoints" }
                        button { class: "text-sm text-blue-600 hover:text-blue-700 font-medium",
                            "View All →"
                        }
                    }
                    div { class: "space-y-3",
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            div {
                                p { class: "text-sm font-medium text-gray-900", "GET /api/users" }
                                p { class: "text-xs text-gray-500", "Avg: 124ms" }
                            }
                            span { class: "px-2 py-1 bg-green-100 text-green-800 text-xs font-semibold rounded",
                                "8,432"
                            }
                        }
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            div {
                                p { class: "text-sm font-medium text-gray-900",
                                    "POST /api/auth/login"
                                }
                                p { class: "text-xs text-gray-500", "Avg: 287ms" }
                            }
                            span { class: "px-2 py-1 bg-blue-100 text-blue-800 text-xs font-semibold rounded",
                                "5,621"
                            }
                        }
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            div {
                                p { class: "text-sm font-medium text-gray-900", "GET /api/projects" }
                                p { class: "text-xs text-gray-500", "Avg: 98ms" }
                            }
                            span { class: "px-2 py-1 bg-purple-100 text-purple-800 text-xs font-semibold rounded",
                                "4,218"
                            }
                        }
                        div { class: "flex items-center justify-between py-2",
                            div {
                                p { class: "text-sm font-medium text-gray-900",
                                    "PUT /api/projects/:id"
                                }
                                p { class: "text-xs text-gray-500", "Avg: 156ms" }
                            }
                            span { class: "px-2 py-1 bg-orange-100 text-orange-800 text-xs font-semibold rounded",
                                "2,847"
                            }
                        }
                    }
                }
                // Error Rate Report
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                    div { class: "flex items-center justify-between mb-4",
                        h3 { class: "text-lg font-semibold text-gray-900", "Error Tracking" }
                        button { class: "text-sm text-blue-600 hover:text-blue-700 font-medium",
                            "View Logs →"
                        }
                    }
                    div { class: "space-y-3",
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            div {
                                span { class: "text-sm text-gray-700", "4xx Client Errors" }
                                p { class: "text-xs text-gray-500", "Error rate: 2.3%" }
                            }
                            span { class: "text-sm font-semibold text-orange-600", "982" }
                        }
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            div {
                                span { class: "text-sm text-gray-700", "5xx Server Errors" }
                                p { class: "text-xs text-gray-500", "Error rate: 0.1%" }
                            }
                            span { class: "text-sm font-semibold text-red-600", "43" }
                        }
                        div { class: "flex items-center justify-between py-2",
                            div {
                                span { class: "text-sm text-gray-700", "Database Timeouts" }
                                p { class: "text-xs text-gray-500", "Error rate: 0.05%" }
                            }
                            span { class: "text-sm font-semibold text-red-600", "21" }
                        }
                    }
                }
                // User Activity Report
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                    div { class: "flex items-center justify-between mb-4",
                        h3 { class: "text-lg font-semibold text-gray-900", "User Activity" }
                        button { class: "text-sm text-blue-600 hover:text-blue-700 font-medium",
                            "View Users →"
                        }
                    }
                    div { class: "space-y-3",
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            span { class: "text-sm text-gray-700", "Active Users (24h)" }
                            span { class: "text-sm font-semibold text-gray-900", "3,847" }
                        }
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            span { class: "text-sm text-gray-700", "New Signups" }
                            span { class: "text-sm font-semibold text-gray-900", "284" }
                        }
                        div { class: "flex items-center justify-between py-2 border-b border-gray-100",
                            span { class: "text-sm text-gray-700", "Failed Logins" }
                            span { class: "text-sm font-semibold text-gray-900", "127" }
                        }
                        div { class: "flex items-center justify-between py-2",
                            span { class: "text-sm text-gray-700", "Session Duration (avg)" }
                            span { class: "text-sm font-semibold text-gray-900", "18m 42s" }
                        }
                    }
                }
            }
            // Performance Insights Section
            div { class: "max-w-7xl mx-auto mt-6",
                div { class: "bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg border border-blue-200 p-6",
                    h3 { class: "text-lg font-semibold text-gray-900 mb-4", "Performance Insights" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        div { class: "flex items-start gap-3",
                            span { class: "text-2xl", "✅" }
                            div {
                                p { class: "text-sm font-medium text-gray-900",
                                    "Response Time Improved"
                                }
                                p { class: "text-xs text-gray-600 mt-1",
                                    "Your average response time decreased by 5.1% this week"
                                }
                            }
                        }
                        div { class: "flex items-start gap-3",
                            span { class: "text-2xl", "⚠️" }
                            div {
                                p { class: "text-sm font-medium text-gray-900",
                                    "High Error Rate on /api/users"
                                }
                                p { class: "text-xs text-gray-600 mt-1",
                                    "Consider investigating this endpoint for issues"
                                }
                            }
                        }
                        div { class: "flex items-start gap-3",
                            span { class: "text-2xl", "📈" }
                            div {
                                p { class: "text-sm font-medium text-gray-900",
                                    "Traffic Spike Detected"
                                }
                                p { class: "text-xs text-gray-600 mt-1",
                                    "Traffic increased 12.5% compared to last period"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
