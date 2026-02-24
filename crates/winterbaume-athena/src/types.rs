use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct WorkGroup {
    pub name: String,
    pub state: String,
    pub description: String,
    pub creation_time: DateTime<Utc>,
    pub output_location: String,
    pub enforce_work_group_configuration: bool,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CapacityReservationData {
    pub name: String,
    pub target_dpus: i32,
    pub allocated_dpus: i32,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DataCatalogData {
    pub name: String,
    pub catalog_type: String,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct NamedQueryData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub database: String,
    pub query_string: String,
    pub work_group: String,
}

#[derive(Debug, Clone)]
pub struct PreparedStatementData {
    pub statement_name: String,
    pub work_group_name: String,
    pub query_statement: String,
    pub description: String,
    pub last_modified_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct QueryExecutionData {
    pub query_execution_id: String,
    pub query: String,
    pub work_group: String,
    pub database: Option<String>,
    pub catalog: Option<String>,
    pub output_location: Option<String>,
    pub status: String,
    pub submission_time: DateTime<Utc>,
    pub completion_time: Option<DateTime<Utc>>,
    /// Column metadata returned by the query backend: `(name, type_str)` pairs.
    pub result_columns: Vec<(String, String)>,
    /// Row data returned by the query backend.
    pub result_rows: Vec<Vec<Option<String>>>,
    /// Error message when `status == "FAILED"`.
    pub error_message: Option<String>,
}
