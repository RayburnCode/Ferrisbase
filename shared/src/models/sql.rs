use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Request to execute a SQL query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteSqlRequest {
    pub query: String,
}

/// Response from executing a SQL query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteSqlResponse {
    pub rows: Vec<JsonValue>,
    pub rows_affected: Option<u64>,
    pub execution_time_ms: u128,
}
