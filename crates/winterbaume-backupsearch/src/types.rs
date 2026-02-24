use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SearchJob {
    pub identifier: String,
    pub arn: String,
    pub name: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub encryption_key_arn: Option<String>,
    pub search_scope: Option<Value>,
    pub item_filters: Option<Value>,
    pub creation_time: i64,
    pub completion_time: Option<i64>,
    pub items_matched: i64,
    pub items_scanned: i64,
    pub recovery_points_scanned: i32,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SearchResultExportJob {
    pub identifier: String,
    pub arn: String,
    pub search_job_identifier: String,
    pub status: String,
    pub status_message: Option<String>,
    pub export_specification: Option<Value>,
    pub role_arn: Option<String>,
    pub creation_time: i64,
    pub completion_time: Option<i64>,
    pub tags: HashMap<String, String>,
}
