use dioxus::prelude::*;
use shared::models::{CreateTableRequest, ColumnDefinition, ColumnDataType};
use crate::hooks::{use_list_tables, use_create_table, use_delete_table, use_table_rows, use_get_table};
use serde_json::Value as JsonValue;

/// The Table Editor page - Interface for managing table schemas
#[component]
pub fn TableEditor(id: String) -> Element {
    let mut selected_table = use_signal(|| None::<String>);
    let mut show_create_modal = use_signal(|| false);
    let mut show_delete_confirm = use_signal(|| false);
    
    // Fetch tables for this project
    let mut tables_resource = use_list_tables(id.clone());
    let create_table_action = use_create_table(id.clone());
    let delete_table_action = use_delete_table(id.clone());
    
    // Fetch table details when a table is selected - always call hooks unconditionally
    let table_details_resource = use_get_table(
        id.clone(), 
        selected_table().unwrap_or_else(|| "".to_string())
    );
    
    // Fetch table rows when a table is selected
    let table_rows_resource = use_table_rows(id.clone(), selected_table());
    
    // Form state for creating a new table
    let mut table_name = use_signal(|| String::new());
    let mut display_name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut columns = use_signal(|| Vec::<ColumnDefinition>::new());
    
    let handle_create_table = move |_| {
        let request = CreateTableRequest {
            table_name: table_name(),
            display_name: display_name(),
            description: Some(description()).filter(|s| !s.is_empty()),
            columns: columns(),
        };
        
        create_table_action.send(request);
        show_create_modal.set(false);
        
        // Reset form
        table_name.set(String::new());
        display_name.set(String::new());
        description.set(String::new());
        columns.set(Vec::new());
        
        // Refresh tables list
        tables_resource.restart();
    };
    
    let handle_delete_table = move |_| {
        if let Some(name) = selected_table() {
            delete_table_action.send(name);
            show_delete_confirm.set(false);
            selected_table.set(None);
            
            // Refresh tables list
            tables_resource.restart();
        }
    };
    
    let add_column = move |_| {
        let mut cols = columns();
        cols.push(ColumnDefinition {
            name: String::new(),
            display_name: String::new(),
            data_type: ColumnDataType::Text,
            is_nullable: true,
            is_primary_key: false,
            is_unique: false,
            default_value: None,
        });
        columns.set(cols);
    };
    
    let mut remove_column = move |index: usize| {
        let mut cols = columns();
        cols.remove(index);
        columns.set(cols);
    };
    
    rsx! {
        div { class: "flex h-screen bg-gray-50",
            // Left Sidebar - Tables List
            div { class: "w-80 bg-white border-r border-gray-200 flex flex-col",
                // Sidebar Header
                div { class: "p-6 border-b border-gray-200",
                    h2 { class: "text-xl font-bold text-gray-900", "Tables" }
                    p { class: "text-sm text-gray-500 mt-1", "Project: {id}" }
                }
                // Tables List
                div { class: "flex-1 overflow-y-auto p-4",
                    match &*tables_resource.read_unchecked() {
                        Some(Ok(tables)) if tables.is_empty() => rsx! {
                            div { class: "text-center py-12",
                                p { class: "text-gray-500 mb-4", "No tables yet" }
                                button {
                                    class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition",
                                    onclick: move |_| show_create_modal.set(true),
                                    "+ Create Your First Table"
                                }
                            }
                        },
                        Some(Ok(tables)) => rsx! {
                            div { class: "space-y-2",
                                for table in tables {
                                    button {
                                        key: "{table.table_name}",
                                        class: if selected_table() == Some(table.table_name.clone()) { "w-full text-left px-4 py-3 bg-blue-50 border-l-4 border-blue-600 rounded-r hover:bg-blue-100 transition" } else { "w-full text-left px-4 py-3 border-l-4 border-transparent hover:bg-gray-50 rounded transition" },
                                        onclick: {
                                            let table_name = table.table_name.clone();
                                            move |_| selected_table.set(Some(table_name.clone()))
                                        },
                                        div {
                                            p { class: "font-semibold text-gray-900", "{table.display_name}" }
                                            p { class: "text-sm text-gray-500 mt-1", "{table.row_count} rows" }
                                            if let Some(desc) = &table.description {
                                                p { class: "text-xs text-gray-400 mt-1 truncate", "{desc}" }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "p-4 bg-red-50 border border-red-200 rounded text-red-700 text-sm",
                                "Error loading tables: {e}"
                            }
                        },
                        None => rsx! {
                            div { class: "flex items-center justify-center py-12",
                                div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" }
                            }
                        },
                    }
                }
                // Add Table Button
                div { class: "p-4 border-t border-gray-200",
                    button {
                        class: "w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-md transition flex items-center justify-center gap-2",
                        onclick: move |_| show_create_modal.set(true),
                        span { "+" }
                        span { "New Table" }
                    }
                }
            }
            // Main Content Area
            div { class: "flex-1 flex flex-col overflow-hidden",
                if let Some(ref name) = selected_table() {
                    // Show selected table with data
                    div { class: "flex-1 flex flex-col",
                        // Table Header
                        div { class: "bg-white border-b border-gray-200 px-6 py-4",
                            div { class: "flex items-center justify-between",
                                div {
                                    h2 { class: "text-2xl font-bold text-gray-900",
                                        "{name}"
                                    }
                                    match &*table_details_resource.read_unchecked() {
                                        Some(Ok(details)) => rsx! {
                                            p { class: "text-sm text-gray-600 mt-1",
                                                "{details.row_count} rows • {details.columns.len()} columns"
                                            }
                                        },
                                        _ => rsx! {
                                            p { class: "text-sm text-gray-600 mt-1", "Loading..." }
                                        },
                                    }
                                }
                                button {
                                    class: "px-4 py-2 bg-red-600 hover:bg-red-700 text-white font-medium rounded-md transition",
                                    onclick: move |_| show_delete_confirm.set(true),
                                    "Delete Table"
                                }
                            }
                        }
                        // Table Data - Excel-like view
                        div { class: "flex-1 overflow-auto bg-gray-50",
                            match (
                                &*table_details_resource.read_unchecked(),
                                &*table_rows_resource.read_unchecked(),
                            ) {
                                (Some(Ok(details)), Some(Ok(rows))) => rsx! {
                                    div { class: "inline-block min-w-full",
                                        table { class: "border-collapse",
                                            // Header row
                                            thead { class: "bg-gray-100 sticky top-0 z-10",
                                                tr {
                                                    // Row number header
                                                    th { class: "border border-gray-300 bg-gray-200 px-4 py-2 text-center text-xs font-semibold text-gray-600 w-16",
                                                        "#"
                                                    }
                                                    // Column headers
                                                    for col in &details.columns {
                                                        th { class: "border border-gray-300 bg-white px-4 py-3 text-left text-xs font-semibold text-gray-700 min-w-[150px]",
                                                            div { class: "flex flex-col",
                                                                div { class: "font-bold text-sm", "{col.display_name}" }
                                                                div { class: "text-gray-500 font-normal mt-1",
                                                                    span { class: "text-xs", "{col.data_type}" }
                                                                    if !col.is_nullable {
                                                                        span { class: "ml-2 text-red-600", "*" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            // Data rows
                                            tbody {
                                                if rows.is_empty() {
                                                    tr {
                                                        td {
                                                            colspan: "{details.columns.len() + 1}",
                                                            class: "border border-gray-300 px-4 py-12 text-center text-gray-500",
                                                            div { class: "flex flex-col items-center",
                                                                svg {
                                                                    class: "h-12 w-12 text-gray-300 mb-3",
                                                                    fill: "none",
                                                                    stroke: "currentColor",
                                                                    view_box: "0 0 24 24",
                                                                    path {
                                                                        stroke_linecap: "round",
                                                                        stroke_linejoin: "round",
                                                                        stroke_width: "2",
                                                                        d: "M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4",
                                                                    }
                                                                }
                                                                p { class: "font-medium", "No data in this table" }
                                                                p { class: "text-sm mt-1", "Add rows using the SQL Editor or API" }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    for (row_idx , row) in rows.iter().enumerate() {
                                                        tr {
                                                            key: "{row_idx}",
                                                            class: "hover:bg-blue-50 transition",
                                                            // Row number
                                                            td { class: "border border-gray-300 bg-gray-100 px-4 py-2 text-center text-xs font-semibold text-gray-600",
                                                                "{row_idx + 1}"
                                                            }
                                                            // Data cells
                                                            for col in &details.columns {
                                                                td { class: "border border-gray-300 px-4 py-2 text-sm",
                                                                    {
                                                                        if let Some(obj) = row.as_object() {
                                                                            if let Some(value) = obj.get(&col.name) {
                                                                                render_cell_value(value)
                                                                            } else {
                                                                                rsx! {
                                                                                    span { class: "text-gray-400 italic", "null" }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            rsx! {
                                                                                span { class: "text-gray-400 italic", "-" }
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
                                },
                                (Some(Ok(_)), Some(Err(e))) => rsx! {
                                    div { class: "flex items-center justify-center h-full p-8",
                                        div { class: "text-center",
                                            p { class: "text-red-600 font-semibold mb-2", "Error loading table data" }
                                            p { class: "text-sm text-gray-600", "{e}" }
                                        }
                                    }
                                },
                                (Some(Err(e)), _) => rsx! {
                                    div { class: "flex items-center justify-center h-full p-8",
                                        div { class: "text-center",
                                            p { class: "text-red-600 font-semibold mb-2", "Error loading table schema" }
                                            p { class: "text-sm text-gray-600", "{e}" }
                                        }
                                    }
                                },
                                _ => rsx! {
                                    div { class: "flex items-center justify-center h-full",
                                        div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600" }
                                    }
                                },
                            }
                        }
                    }
                } else {
                    // No table selected
                    div { class: "flex items-center justify-center h-full text-gray-400",
                        div { class: "text-center",
                            svg {
                                class: "mx-auto h-24 w-24 text-gray-300 mb-4",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "1.5",
                                    d: "M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z",
                                }
                            }
                            p { class: "text-lg font-medium", "Select a table to view its data" }
                            p { class: "text-sm mt-2", "or create a new table to get started" }
                        }
                    }
                }
            }
        }
        // Create Table Modal
        if show_create_modal() {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4",
                onclick: move |_| show_create_modal.set(false),
                div {
                    class: "bg-white rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto",
                    onclick: move |e| e.stop_propagation(),
                    // Modal Header
                    div { class: "px-6 py-4 border-b border-gray-200 flex items-center justify-between sticky top-0 bg-white",
                        h3 { class: "text-xl font-bold text-gray-900", "Create New Table" }
                        button {
                            class: "text-gray-400 hover:text-gray-600 text-2xl leading-none",
                            onclick: move |_| show_create_modal.set(false),
                            "×"
                        }
                    }
                    // Modal Body
                    div { class: "px-6 py-6 space-y-6",
                        // Table Name
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-2",
                                "Table Name (database identifier)"
                            }
                            input {
                                r#type: "text",
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                                placeholder: "e.g. user_profiles",
                                value: "{table_name}",
                                oninput: move |e| table_name.set(e.value().clone()),
                            }
                            p { class: "text-xs text-gray-500 mt-1",
                                "Use lowercase letters, numbers, and underscores only"
                            }
                        }
                        // Display Name
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-2",
                                "Display Name"
                            }
                            input {
                                r#type: "text",
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                                placeholder: "e.g. User Profiles",
                                value: "{display_name}",
                                oninput: move |e| display_name.set(e.value().clone()),
                            }
                        }
                        // Description
                        div {
                            label { class: "block text-sm font-medium text-gray-700 mb-2",
                                "Description (optional)"
                            }
                            textarea {
                                class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                                rows: "2",
                                placeholder: "Describe what this table stores...",
                                value: "{description}",
                                oninput: move |e| description.set(e.value().clone()),
                            }
                        }
                        // Columns
                        div {
                            div { class: "flex items-center justify-between mb-4",
                                label { class: "block text-sm font-medium text-gray-700",
                                    "Columns"
                                }
                                button {
                                    class: "px-3 py-1 bg-green-600 hover:bg-green-700 text-white text-sm font-medium rounded transition",
                                    onclick: add_column,
                                    "+ Add Column"
                                }
                            }
                            // Info message about auto-generated columns
                            div { class: "mb-4 p-3 bg-blue-50 border border-blue-200 rounded text-sm text-blue-700",
                                "ℹ️ All tables automatically include: "
                                span { class: "font-semibold", "id" }
                                ", "
                                span { class: "font-semibold", "created_at" }
                                ", and "
                                span { class: "font-semibold", "updated_at" }
                                " columns"
                            }
                            {
                                let cols = columns();
                                if cols.is_empty() {
                                    rsx! {
                                        div { class: "text-center py-8 bg-gray-50 rounded border-2 border-dashed border-gray-300",
                                            p { class: "text-gray-500", "No custom columns added yet" }
                                            p { class: "text-sm text-gray-400 mt-1", "Click '+ Add Column' to define your table schema" }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { class: "space-y-4",
                                            for (idx , col) in cols.iter().enumerate() {
                                                div {
                                                    key: "{idx}",
                                                    class: "p-4 border border-gray-200 rounded-lg space-y-3",
                                                    div { class: "flex items-center justify-between mb-3",
                                                        h4 { class: "font-medium text-gray-900", "Column {idx + 1}" }
                                                        button {
                                                            class: "text-red-600 hover:text-red-700 text-sm font-medium",
                                                            onclick: move |_| remove_column(idx),
                                                            "Remove"
                                                        }
                                                    }
                                                    div { class: "grid grid-cols-2 gap-3",
                                                        div {
                                                            label { class: "block text-xs font-medium text-gray-600 mb-1", "Column Name" }
                                                            input {
                                                                r#type: "text",
                                                                class: "w-full px-3 py-2 text-sm border border-gray-300 rounded focus:ring-1 focus:ring-blue-500",
                                                                placeholder: "e.g. email",
                                                                value: "{col.name}",
                                                                oninput: {
                                                                    let idx = idx;
                                                                    move |e| {
                                                                        let mut cols = columns();
                                                                        cols[idx].name = e.value().clone();
                                                                        columns.set(cols);
                                                                    }
                                                                },
                                                            }
                                                        }
                                                        div {
                                                            label { class: "block text-xs font-medium text-gray-600 mb-1", "Display Name" }
                                                            input {
                                                                r#type: "text",
                                                                class: "w-full px-3 py-2 text-sm border border-gray-300 rounded focus:ring-1 focus:ring-blue-500",
                                                                placeholder: "e.g. Email Address",
                                                                value: "{col.display_name}",
                                                                oninput: {
                                                                    let idx = idx;
                                                                    move |e| {
                                                                        let mut cols = columns();
                                                                        cols[idx].display_name = e.value().clone();
                                                                        columns.set(cols);
                                                                    }
                                                                },
                                                            }
                                                        }
                                                        div {
                                                            label { class: "block text-xs font-medium text-gray-600 mb-1", "Data Type" }
                                                            select {
                                                                class: "w-full px-3 py-2 text-sm border border-gray-300 rounded focus:ring-1 focus:ring-blue-500",
                                                                onchange: {
                                                                    let idx = idx;
                                                                    move |e| {
                                                                        let mut cols = columns();
                                                                        cols[idx].data_type = match e.value().as_str() {
                                                                            "Integer" => ColumnDataType::Integer,
                                                                            "BigInt" => ColumnDataType::BigInt,
                                                                            "Decimal" => ColumnDataType::Decimal,
                                                                            "Boolean" => ColumnDataType::Boolean,
                                                                            "Timestamp" => ColumnDataType::Timestamp,
                                                                            "Date" => ColumnDataType::Date,
                                                                            "Json" => ColumnDataType::Json,
                                                                            "Uuid" => ColumnDataType::Uuid,
                                                                            _ => ColumnDataType::Text,
                                                                        };
                                                                        columns.set(cols);
                                                                    }
                                                                },
                                                                option { value: "Text", "Text" }
                                                                option { value: "Integer", "Integer" }
                                                                option { value: "BigInt", "BigInt" }
                                                                option { value: "Decimal", "Decimal" }
                                                                option { value: "Boolean", "Boolean" }
                                                                option { value: "Timestamp", "Timestamp" }
                                                                option { value: "Date", "Date" }
                                                                option { value: "Json", "JSON" }
                                                                option { value: "Uuid", "UUID" }
                                                            }
                                                        }
                                                        div { class: "col-span-2 flex gap-4",
                                                            label { class: "flex items-center gap-2 text-sm text-gray-700",
                                                                input {
                                                                    r#type: "checkbox",
                                                                    class: "rounded border-gray-300",
                                                                    checked: col.is_nullable,
                                                                    onchange: {
                                                                        let idx = idx;
                                                                        move |e| {
                                                                            let mut cols = columns();
                                                                            cols[idx].is_nullable = e.checked();
                                                                            columns.set(cols);
                                                                        }
                                                                    },
                                                                }
                                                                "Nullable"
                                                            }
                                                            label { class: "flex items-center gap-2 text-sm text-gray-700",
                                                                input {
                                                                    r#type: "checkbox",
                                                                    class: "rounded border-gray-300",
                                                                    checked: col.is_primary_key,
                                                                    onchange: {
                                                                        let idx = idx;
                                                                        move |e| {
                                                                            let mut cols = columns();
                                                                            cols[idx].is_primary_key = e.checked();
                                                                            columns.set(cols);
                                                                        }
                                                                    },
                                                                }
                                                                "Primary Key"
                                                            }
                                                            label { class: "flex items-center gap-2 text-sm text-gray-700",
                                                                input {
                                                                    r#type: "checkbox",
                                                                    class: "rounded border-gray-300",
                                                                    checked: col.is_unique,
                                                                    onchange: {
                                                                        let idx = idx;
                                                                        move |e| {
                                                                            let mut cols = columns();
                                                                            cols[idx].is_unique = e.checked();
                                                                            columns.set(cols);
                                                                        }
                                                                    },
                                                                }
                                                                "Unique"
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
                    // Modal Footer
                    div { class: "px-6 py-4 border-t border-gray-200 flex justify-end gap-3 sticky bottom-0 bg-white",
                        button {
                            class: "px-4 py-2 border border-gray-300 text-gray-700 font-medium rounded-md hover:bg-gray-50 transition",
                            onclick: move |_| show_create_modal.set(false),
                            "Cancel"
                        }
                        button {
                            class: "px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-md transition disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: table_name().is_empty() || display_name().is_empty(),
                            onclick: handle_create_table,
                            "Create Table"
                        }
                    }
                }
            }
        }
        // Delete Confirmation Modal
        if show_delete_confirm() {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4",
                onclick: move |_| show_delete_confirm.set(false),
                div {
                    class: "bg-white rounded-lg shadow-xl max-w-md w-full p-6",
                    onclick: move |e| e.stop_propagation(),
                    h3 { class: "text-lg font-bold text-gray-900 mb-4", "Delete Table?" }
                    p { class: "text-gray-600 mb-6",
                        "Are you sure you want to delete this table? This action cannot be undone and all data will be permanently lost."
                    }
                    div { class: "flex justify-end gap-3",
                        button {
                            class: "px-4 py-2 border border-gray-300 text-gray-700 font-medium rounded-md hover:bg-gray-50 transition",
                            onclick: move |_| show_delete_confirm.set(false),
                            "Cancel"
                        }
                        button {
                            class: "px-4 py-2 bg-red-600 hover:bg-red-700 text-white font-medium rounded-md transition",
                            onclick: handle_delete_table,
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

/// Helper function to render a JSON value in a table cell
fn render_cell_value(value: &JsonValue) -> Element {
    match value {
        JsonValue::Null => rsx! {
            span { class: "text-gray-400 italic", "null" }
        },
        JsonValue::Bool(b) => rsx! {
            span { class: "text-purple-600 font-mono", "{b}" }
        },
        JsonValue::Number(n) => rsx! {
            span { class: "text-blue-600 font-mono", "{n}" }
        },
        JsonValue::String(s) => {
            // Truncate long strings
            let display_text = if s.len() > 100 {
                format!("{}...", &s[..100])
            } else {
                s.to_string()
            };
            rsx! {
                span { class: "text-gray-900", "{display_text}" }
            }
        },
        JsonValue::Array(arr) => rsx! {
            span { class: "text-gray-600 text-xs font-mono", "[{arr.len()} items]" }
        },
        JsonValue::Object(obj) => rsx! {
            span { class: "text-gray-600 text-xs font-mono", "{{{obj.len()} fields}}" }
        },
    }
}
