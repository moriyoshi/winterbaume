use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Application {
    pub application_name: String,
    pub application_arn: String,
    pub application_status: String,
    pub application_version_id: i64,
    pub runtime_environment: String,
    pub service_execution_role: Option<String>,
    pub create_timestamp: DateTime<Utc>,
    pub last_update_timestamp: DateTime<Utc>,
    pub application_description: Option<String>,
    pub tags: HashMap<String, String>,
    pub snapshots: Vec<Snapshot>,
    pub application_configuration: Option<serde_json::Value>,
    pub cloudwatch_logging_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub snapshot_name: String,
    pub application_version_id: i64,
    pub runtime_environment: String,
    pub snapshot_creation_timestamp: DateTime<Utc>,
}
