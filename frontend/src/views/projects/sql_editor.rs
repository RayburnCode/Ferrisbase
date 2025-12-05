use dioxus::prelude::*;
use crate::hooks::use_execute_sql;
use shared::models::ExecuteSqlResponse;
use serde_json::Value as JsonValue;

/// The SQL Editor page component that will be rendered when the current route is `[Route::SQLEditor]`
#[component]
pub fn SQLEditor(id: String) -> Element {
    let mut query = use_signal(|| "SELECT * FROM users LIMIT 10;".to_string());
    let mut result = use_signal(|| None::<Result<ExecuteSqlResponse, String>>);
    let mut is_loading = use_signal(|| false);
    
    // Hook to execute SQL queries
    let execute_sql = use_execute_sql(id.clone());
    
    // Effect to turn off loading when result is available
    use_effect(move || {
        if result.read().is_some() {
            is_loading.set(false);
        }
    });
    
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
                            button {
                                class: "px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition",
                                onclick: move |_| {
                                    let formatted = query().split_whitespace().collect::<Vec<_>>().join(" ");
                                    query.set(formatted);
                                },
                                "Format"
                            }
                            button {
                                class: "px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition",
                                onclick: move |_| {
                                    query.set(String::new());
                                    result.set(None);
                                },
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
                                disabled: is_loading() || query().trim().is_empty(),
                                onclick: move |_| {
                                    is_loading.set(true);
                                    execute_sql.send((query(), result));
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
                        if let Some(Ok(resp)) = result() {
                            if !resp.rows.is_empty() {
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
                    }
                    // Results Display
                    match result() {
                        None => rsx! {
                            div { class: "flex flex-col items-center justify-center h-64 text-gray-400",
                                span { class: "text-5xl mb-4", "📊" }
                                p { class: "text-sm", "No results yet. Run a query to see results here." }
                            }
                        },
                        Some(Err(error)) => rsx! {
                            div { class: "flex flex-col items-center justify-center h-64 text-red-500",
                                span { class: "text-5xl mb-4", "⚠️" }
                                p { class: "text-sm font-semibold mb-2", "Error executing query:" }
                                pre { class: "text-xs bg-red-50 p-4 rounded-md max-w-full overflow-auto", "{error}" }
                            }
                        },
                        Some(Ok(resp)) => {
                            if resp.rows.is_empty() {
                                let message = if let Some(affected) = resp.rows_affected {
                                    format!("Query executed successfully. {} row(s) affected.", affected)
                                } else {
                                    "Query executed successfully. No rows returned.".to_string()
                                };
                                rsx! {
                                    div { class: "flex flex-col items-center justify-center h-64 text-gray-500",
                                        span { class: "text-5xl mb-4", "✓" }
                                        p { class: "text-sm font-semibold", "{message}" }
                                        p { class: "text-xs text-gray-400 mt-2", "Executed in {resp.execution_time_ms}ms" }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "bg-gray-50 rounded-md p-4 overflow-x-auto", {render_table(&resp.rows)} }
                                    div { class: "mt-4 flex items-center justify-between text-sm text-gray-600",
                                        span { "{resp.rows.len()} row(s) returned" }
                                        span { "Executed in {resp.execution_time_ms}ms" }
                                    }
                                }
                            }
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
                                        "INSERT INTO table_name (column1, column2) VALUES ('value1', 'value2');"
                                            .to_string(),
                                    )
                            },
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Insert Row" }
                            div { class: "text-xs text-gray-600 font-mono", "INSERT INTO ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| {
                                query
                                    .set(
                                        "UPDATE table_name SET column1 = 'value1' WHERE id = 'some-id';"
                                            .to_string(),
                                    )
                            },
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Update Row" }
                            div { class: "text-xs text-gray-600 font-mono", "UPDATE ... SET ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| query.set("DELETE FROM table_name WHERE id = 'some-id';".to_string()),
                            div { class: "font-medium text-gray-900 text-sm mb-1", "Delete Row" }
                            div { class: "text-xs text-gray-600 font-mono", "DELETE FROM ..." }
                        }
                        button {
                            class: "p-3 text-left bg-gray-50 hover:bg-gray-100 border border-gray-200 rounded-md transition",
                            onclick: move |_| {
                                query
                                    .set(
                                        "CREATE TABLE table_name (\n  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),\n  column1 TEXT NOT NULL,\n  column2 INTEGER,\n  created_at TIMESTAMP DEFAULT NOW(),\n  updated_at TIMESTAMP\n);"
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

/// Helper function to render JSON results as an HTML table
fn render_table(rows: &[JsonValue]) -> Element {
    if rows.is_empty() {
        return rsx! {
            p { class: "text-gray-500 text-sm", "No data to display" }
        };
    }

    // Extract column names from the first row
    let columns: Vec<String> = if let Some(first_row) = rows.first() {
        if let Some(obj) = first_row.as_object() {
            obj.keys().cloned().collect()
        } else {
            vec!["value".to_string()]
        }
    } else {
        vec![]
    };

    rsx! {
        div { class: "overflow-x-auto",
            table { class: "min-w-full divide-y divide-gray-200 border border-gray-300",
                thead { class: "bg-gray-100",
                    tr {
                        for col in &columns {
                            th { class: "px-4 py-2 text-left text-xs font-medium text-gray-700 uppercase tracking-wider border-r border-gray-300",
                                "{col}"
                            }
                        }
                    }
                }
                tbody { class: "bg-white divide-y divide-gray-200",
                    for row in rows {
                        tr { class: "hover:bg-gray-50",
                            for col in &columns {
                                td { class: "px-4 py-2 text-sm text-gray-900 border-r border-gray-200 font-mono",
                                    {
                                        if let Some(obj) = row.as_object() {
                                            if let Some(val) = obj.get(col) {
                                                format_json_value(val)
                                            } else {
                                                rsx! {
                                                    span { class: "text-gray-400", "null" }
                                                }
                                            }
                                        } else {
                                            rsx! {
                                            "{row}"
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
}

/// Helper to format JSON values for display
fn format_json_value(val: &JsonValue) -> Element {
    match val {
        JsonValue::Null => rsx! {
            span { class: "text-gray-400 italic", "null" }
        },
        JsonValue::Bool(b) => rsx! {
            span { class: "text-purple-600", "{b}" }
        },
        JsonValue::Number(n) => rsx! {
            span { class: "text-blue-600", "{n}" }
        },
        JsonValue::String(s) => rsx! {
            span { class: "text-green-700", "{s}" }
        },
        JsonValue::Array(_) | JsonValue::Object(_) => rsx! {
            span { class: "text-gray-600 text-xs", "{val}" }
        },
    }
}
