use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Flow {
    pub flow_name: String,
    pub flow_arn: String,
    pub description: Option<String>,
    pub kms_arn: Option<String>,
    /// One of `Active`, `Deprecated`, `Deleted`, `Draft`, `Errored`, `Suspended`.
    pub flow_status: String,
    pub flow_status_message: Option<String>,
    pub trigger_config: Value,
    pub source_flow_config: Value,
    pub destination_flow_config_list: Value,
    pub tasks: Value,
    pub metadata_catalog_config: Option<Value>,
    pub created_at: i64,
    pub last_updated_at: i64,
    pub created_by: String,
    pub last_updated_by: String,
    pub schema_version: i64,
    pub tags: HashMap<String, String>,
    /// Last execution id from StartFlow.
    pub last_execution_id: Option<String>,
}
