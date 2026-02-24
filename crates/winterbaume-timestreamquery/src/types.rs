use chrono::{DateTime, Utc};

/// A scheduled query.
#[derive(Debug, Clone)]
pub struct ScheduledQuery {
    pub arn: String,
    pub name: String,
    pub query_string: String,
    pub state: ScheduledQueryState,
    pub schedule_expression: String,
    pub target_database: String,
    pub target_table: String,
    pub s3_error_report_bucket: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_run_status: Option<String>,
    pub notification_topic_arn: String,
    pub role_arn: String,
}

/// Scheduled query state.
#[derive(Debug, Clone, PartialEq)]
pub enum ScheduledQueryState {
    Enabled,
    Disabled,
}

impl ScheduledQueryState {
    pub fn as_str(&self) -> &str {
        match self {
            ScheduledQueryState::Enabled => "ENABLED",
            ScheduledQueryState::Disabled => "DISABLED",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ENABLED" => Some(ScheduledQueryState::Enabled),
            "DISABLED" => Some(ScheduledQueryState::Disabled),
            _ => None,
        }
    }
}

/// A column in a query result.
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub column_type: String,
}

/// A row in a query result.
#[derive(Debug, Clone)]
pub struct Row {
    pub data: Vec<Datum>,
}

/// A single value in a query result row.
#[derive(Debug, Clone)]
pub struct Datum {
    pub scalar_value: Option<String>,
}
