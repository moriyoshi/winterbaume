use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogGroup {
    pub name: String,
    pub arn: String,
    pub creation_time: i64,
    pub retention_in_days: Option<i32>,
    pub streams: HashMap<String, LogStream>,
    pub tags: HashMap<String, String>,
    pub kms_key_id: Option<String>,
    pub data_protection_policy: Option<String>,
    pub deletion_protection_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct LogStream {
    pub name: String,
    pub arn: String,
    pub creation_time: i64,
    pub events: Vec<LogEvent>,
    pub first_event_timestamp: Option<i64>,
    pub last_event_timestamp: Option<i64>,
    pub upload_sequence_token: String,
}

#[derive(Debug, Clone)]
pub struct LogEvent {
    pub timestamp: i64,
    pub message: String,
    pub ingestion_time: i64,
}

#[derive(Debug, Clone)]
pub struct MetricFilter {
    pub filter_name: String,
    pub log_group_name: String,
    pub filter_pattern: String,
    pub metric_transformations: Vec<MetricTransformation>,
    pub creation_time: i64,
}

#[derive(Debug, Clone)]
pub struct MetricTransformation {
    pub metric_namespace: String,
    pub metric_name: String,
    pub metric_value: String,
}

#[derive(Debug, Clone)]
pub struct SubscriptionFilter {
    pub filter_name: String,
    pub log_group_name: String,
    pub filter_pattern: String,
    pub destination_arn: String,
    pub role_arn: Option<String>,
    pub creation_time: i64,
}

#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub policy_name: String,
    pub policy_document: String,
    pub last_updated_time: i64,
}

#[derive(Debug, Clone)]
pub struct Destination {
    pub destination_name: String,
    pub target_arn: String,
    pub role_arn: String,
    pub access_policy: Option<String>,
    pub arn: String,
    pub creation_time: i64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ExportTask {
    pub task_id: String,
    pub task_name: Option<String>,
    pub log_group_name: String,
    pub destination: String,
    pub from_time: i64,
    pub to_time: i64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub query_id: String,
    pub log_group_name: String,
    pub query_string: String,
    pub status: String,
    pub create_time: i64,
}

#[derive(Debug, Clone)]
pub struct DeliverySource {
    pub name: String,
    pub log_type: Option<String>,
    pub service: Option<String>,
    pub resource_arns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeliveryDestination {
    pub name: String,
    pub arn: String,
    pub delivery_destination_type: Option<String>,
    pub output_format: Option<String>,
    pub delivery_destination_configuration: Option<DeliveryDestinationConfiguration>,
}

#[derive(Debug, Clone)]
pub struct DeliveryDestinationConfiguration {
    pub destination_resource_arn: String,
}

#[derive(Debug, Clone)]
pub struct DeliveryDestinationPolicy {
    pub delivery_destination_name: String,
    pub policy: String,
}

#[derive(Debug, Clone)]
pub struct Delivery {
    pub id: String,
    pub source: String,
    pub destination: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AccountPolicy {
    pub policy_name: String,
    pub policy_document: String,
    pub policy_type: String,
    pub scope: Option<String>,
    pub selection_criteria: Option<String>,
    pub account_id: String,
    pub last_updated_time: i64,
}

#[derive(Debug, Clone)]
pub struct QueryDefinition {
    pub query_definition_id: String,
    pub name: String,
    pub query_string: String,
    pub log_group_names: Vec<String>,
    pub last_modified: i64,
}

#[derive(Debug, Clone)]
pub struct LogAnomalyDetector {
    pub anomaly_detector_arn: String,
    pub detector_name: String,
    pub log_group_arn_list: Vec<String>,
    pub evaluation_frequency: Option<String>,
    pub filter_pattern: Option<String>,
    pub anomaly_visibility_time: Option<i64>,
    pub status: String,
    pub creation_time: i64,
    pub last_modified_time: i64,
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Anomaly {
    pub anomaly_id: String,
    pub anomaly_detector_arn: String,
    pub description: String,
    pub first_seen: i64,
    pub last_seen: i64,
    pub state: String,
    pub suppressed: bool,
    pub suppressed_date: Option<i64>,
    pub suppressed_until: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct IndexPolicy {
    pub policy_name: String,
    pub log_group_identifier: String,
    pub policy_document: String,
    pub last_update_time: i64,
}

#[derive(Debug, Clone)]
pub struct Transformer {
    pub log_group_identifier: String,
    pub transformers: Vec<serde_json::Value>,
    pub creation_time: i64,
    pub last_modified_time: i64,
}

#[derive(Debug, Clone)]
pub struct Integration {
    pub integration_name: String,
    pub integration_type: String,
    pub resource_config: serde_json::Value,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ImportTask {
    pub task_id: String,
    pub task_name: Option<String>,
    pub log_group_name: String,
    pub from_time: i64,
    pub to_time: i64,
    pub source: serde_json::Value,
    pub status: String,
    pub creation_time: i64,
}

#[derive(Debug, Clone)]
pub struct ScheduledQuery {
    pub scheduled_query_id: String,
    pub name: String,
    pub query_string: String,
    pub log_group_name: String,
    pub schedule_expression: String,
    pub status: String,
    pub creation_time: i64,
}
