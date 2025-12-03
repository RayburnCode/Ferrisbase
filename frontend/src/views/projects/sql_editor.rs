use dioxus::prelude::*;

/// The SQL Editor page component that will be rendered when the current route is `[Route::SQLEditor]`
#[component]
pub fn SQLEditor(id: String) -> Element {
    let mut query = use_signal(|| "SELECT * FROM users LIMIT 10;".to_string());
    let mut results = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    
    rsx! {
        div { class: "min-h-screen bg-gray-50 p-6",
            // Header Section
            div { class: "max-w-7xl mx-auto mb-6",
                div { class: "flex items-center justify-between mb-4",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", "SQL Editor" }
                        p { class: "text-gray-600 mt-1", "Project: {id}" }
                    }
                    div { class: "flex gap-3",
                        button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition duration-200",
                            "History"
                        }
                        button { class: "px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium rounded-md transition duration-200",
                            "Saved Queries"
                        }
                    }
                }
            }
            // Main Editor Container
            div { class: "max-w-7xl mx-auto grid grid-cols-1 gap-6",
                // Query Input Section
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                    div { class: "flex items-center justify-between mb-4",
                        h2 { class: "text-lg font-semibold text-gray-900", "Query Editor" }
                        div { class: "flex gap-2",
                            button { class: "px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition",
                                "Format"
                            }
                            button { class: "px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition",
                                "Clear"
                            }
                        }
                    }
                    // SQL Textarea
                    textarea {
                        class: "w-full h-64 px-4 py-3 font-mono text-sm bg-gray-50 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none resize-y",
                        placeholder: "Enter your SQL query here...",
                        value: "{query}",
                        oninput: move |evt| query.set(evt.value()),
                    }
                    // Action Buttons
                    div { class: "flex items-center justify-between mt-4",
                        div { class: "flex gap-2 text-sm text-gray-600",
                            span { class: "px-2 py-1 bg-blue-50 text-blue-700 rounded",
                                "PostgreSQL"
                            }
                            span { "Lines: {query().lines().count()}" }
                        }
                        div { class: "flex gap-3",
                            button {
                                class: "px-4 py-2 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-md transition duration-200 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed",
                                disabled: is_loading(),
                                onclick: move |_| {
                                    is_loading.set(true);
                                    results
                                        .set(
                                            "Query executed successfully!\n\nid | name | email\n1 | John Doe | john@example.com\n2 | Jane Smith | jane@example.com\n3 | Bob Johnson | bob@example.com"
                                                .to_string(),
                                        );
                                    is_loading.set(false);
                                },
                                if is_loading() {
                                    "Running..."
                                } else {
                                    "▶ Run Query"
                                }
                            }
                            button { class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition duration-200",
                                "Save Query"
                            }
                        }
                    }
                }
                // Query Results Section
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                    div { class: "flex items-center justify-between mb-4",
                        h2 { class: "text-lg font-semibold text-gray-900", "Query Results" }
                        if !results().is_empty() {
                            div { class: "flex gap-2",
                                button { class: "px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition",
                                    "Export CSV"
                                }
                                button { class: "px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition",
                                    "Export JSON"
                                }
                            }
                        }
                    }
                    // Results Display
                    if results().is_empty() {
                        div { class: "flex flex-col items-center justify-center h-64 text-gray-400",
                            span { class: "text-5xl mb-4", "📊" }
                            p { class: "text-sm", "No results yet. Run a query to see results here." }
                        }
                    } else {
                        div { class: "bg-gray-50 rounded-md p-4 overflow-x-auto",
                            pre { class: "font-mono text-sm text-gray-800 whitespace-pre",
                                "{results}"
                            }
                        }
                        div { class: "mt-4 flex items-center justify-between text-sm text-gray-600",
                            span { "3 rows returned" }
                            span { "Executed in 42ms" }
                        }
                    }
                }
                // Query Templates Section
                div { class: "bg-white rounded-lg shadow-md border border-gray-200 p-6",
                    h2 { class: "text-lg font-semibold text-gray-900 mb-4", "Quick Templates" }
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3",
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| query.set("SELECT * FROM table_name LIMIT 10;".to_string()),
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Select All" }
                            div { class: "text-xs text-gray-600 font-mono", "SELECT * FROM ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| {
                                query
                                    .set(
                                        "INSERT INTO table_name (column1, column2) VALUES (value1, value2);"
                                            .to_string(),
                                    )
                            },
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Insert Row" }
                            div { class: "text-xs text-gray-600 font-mono", "INSERT INTO ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| {
                                query.set("UPDATE table_name SET column1 = value1 WHERE condition;".to_string())
                            },
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Update Row" }
                            div { class: "text-xs text-gray-600 font-mono", "UPDATE ... SET ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| query.set("DELETE FROM table_name WHERE condition;".to_string()),
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Delete Row" }
                            div { class: "text-xs text-gray-600 font-mono", "DELETE FROM ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| {
                                query
                                    .set(
                                        "CREATE TABLE table_name (\n  id SERIAL PRIMARY KEY,\n  column1 VARCHAR(255),\n  created_at TIMESTAMP DEFAULT NOW()\n);"
                                            .to_string(),
                                    )
                            },
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Create Table" }
                            div { class: "text-xs text-gray-600 font-mono", "CREATE TABLE ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| {
                                query
                                    .set(
                                        "SELECT column1, COUNT(*) as count\nFROM table_name\nGROUP BY column1\nORDER BY count DESC;"
                                            .to_string(),
                                    )
                            },
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Group & Count" }
                            div { class: "text-xs text-gray-600 font-mono", "GROUP BY ... COUNT" }
                        }
                    }
                }
            }
        }
    }
}
