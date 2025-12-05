use serde::{Deserialize, Serialize};

/// Supported column data types for user-created tables
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColumnDataType {
    Text,
    Integer,
    BigInt,
    Decimal,
    Boolean,
    Timestamp,
    Date,
    Json,
    Uuid,
}

impl ColumnDataType {
    pub fn to_postgres_type(&self) -> &str {
        match self {
            ColumnDataType::Text => "TEXT",
            ColumnDataType::Integer => "INTEGER",
            ColumnDataType::BigInt => "BIGINT",
            ColumnDataType::Decimal => "DECIMAL",
            ColumnDataType::Boolean => "BOOLEAN",
            ColumnDataType::Timestamp => "TIMESTAMP",
            ColumnDataType::Date => "DATE",
            ColumnDataType::Json => "JSONB",
            ColumnDataType::Uuid => "UUID",
        }
    }
}

/// Column definition for creating a new table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub display_name: String,
    pub data_type: ColumnDataType,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub default_value: Option<String>,
}

/// Request to create a new table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableRequest {
    pub table_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub columns: Vec<ColumnDefinition>,
}

/// Column information in response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColumnResponse {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub default_value: Option<String>,
    pub column_order: i32,
}

/// Table definition response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableResponse {
    pub id: String,
    pub table_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub row_count: i32,
    pub columns: Vec<ColumnResponse>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

/// Request to add a column to an existing table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddColumnRequest {
    pub column: ColumnDefinition,
}

/// Response when listing tables (without column details)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableSummary {
    pub id: String,
    pub table_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub row_count: i32,
    pub created_at: String,
}
