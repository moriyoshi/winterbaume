use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Application {
    pub configuration_id: String,
    pub name: String,
    pub description: Option<String>,
    pub wave: Option<String>,
    pub time_of_creation: i64,
    pub last_modified_time_stamp: i64,
}

#[derive(Debug, Clone)]
pub struct ConfigurationTag {
    pub configuration_id: String,
    pub configuration_type: String,
    pub key: String,
    pub value: String,
    pub time_of_creation: i64,
}

#[derive(Debug, Clone)]
pub struct ImportTask {
    pub id: String,
    pub name: String,
    pub status: String,
    pub import_url: String,
    pub import_request_time: i64,
    pub import_completion_time: Option<i64>,
    pub server_import_success: i32,
    pub server_import_failure: i32,
    pub application_import_success: i32,
    pub application_import_failure: i32,
    pub error_url: Option<String>,
    pub import_deleted_time: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ExportTask {
    pub id: String,
    pub status: String,
    pub status_message: Option<String>,
    pub configurations_download_url: Option<String>,
    pub export_request_time: i64,
    pub is_truncated: bool,
    pub requested_start_time: Option<i64>,
    pub requested_end_time: Option<i64>,
    pub data_source: String,
    pub filters: Vec<Value>,
    pub preferences: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ContinuousExport {
    pub export_id: String,
    pub status: String,
    pub status_detail: Option<String>,
    pub s3_bucket: Option<String>,
    pub start_time: Option<i64>,
    pub stop_time: Option<i64>,
    pub data_source: String,
    pub schema_storage_config: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct BatchDeleteTask {
    pub id: String,
    pub configuration_type: String,
    pub status: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
}
