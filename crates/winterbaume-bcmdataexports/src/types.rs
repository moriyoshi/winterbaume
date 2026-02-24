use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Export {
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub data_query: Value,
    pub destination_configurations: Value,
    pub refresh_cadence: Value,
    pub created_at: String,
    pub last_updated_at: String,
    pub last_refreshed_at: Option<String>,
    pub status_code: String,
    pub status_reason: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub id: String,
    pub export_arn: String,
    pub status_code: String,
    pub status_reason: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
    pub completed_at: Option<String>,
}
