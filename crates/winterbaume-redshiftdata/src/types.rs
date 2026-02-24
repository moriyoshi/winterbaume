use chrono::{DateTime, Utc};

/// Status of a Redshift Data API statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementStatus {
    Submitted,
    Started,
    Finished,
    Failed,
    Aborted,
}

impl StatementStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Submitted => "SUBMITTED",
            Self::Started => "STARTED",
            Self::Finished => "FINISHED",
            Self::Failed => "FAILED",
            Self::Aborted => "ABORTED",
        }
    }
}

/// A parameter for a Redshift Data API statement.
#[derive(Debug, Clone)]
pub struct StatementParameter {
    pub name: String,
    pub value: String,
}

/// A statement executed via the Redshift Data API.
#[derive(Debug, Clone)]
pub struct Statement {
    pub id: String,
    pub sql: String,
    pub cluster_identifier: Option<String>,
    pub workgroup_name: Option<String>,
    pub database: String,
    pub db_user: Option<String>,
    pub secret_arn: Option<String>,
    pub status: StatementStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub result_rows: i64,
    pub result_size: i64,
    pub has_result_set: bool,
    pub query_string: String,
    pub parameters: Vec<StatementParameter>,
    pub sqls: Vec<String>,
    pub statement_name: Option<String>,
    pub is_batch: bool,
    /// Column metadata from the query backend: `(name, type_str)` pairs.
    pub result_columns: Vec<(String, String)>,
    /// Row data from the query backend.
    pub result_data: Vec<Vec<Option<String>>>,
    /// Error message when `status == Failed`.
    pub error_message: Option<String>,
}
