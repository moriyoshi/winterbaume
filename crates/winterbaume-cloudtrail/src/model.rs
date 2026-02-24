//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudtrail

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    pub tags_list: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelQueryRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store: Option<String>,
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelQueryResponse {
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelRequest {
    #[serde(rename = "Destinations")]
    #[serde(default)]
    pub destinations: Vec<Destination>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelResponse {
    #[serde(rename = "ChannelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDashboardRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<RefreshSchedule>,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "Widgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<RequestWidget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshSchedule {
    #[serde(rename = "Frequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RefreshScheduleFrequency>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TimeOfDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshScheduleFrequency {
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestWidget {
    #[serde(rename = "QueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<String>>,
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    pub query_statement: String,
    #[serde(rename = "ViewProperties")]
    #[serde(default)]
    pub view_properties: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDashboardResponse {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<RefreshSchedule>,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Widgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<Widget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Widget {
    #[serde(rename = "QueryAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_alias: Option<String>,
    #[serde(rename = "QueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<String>>,
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_statement: Option<String>,
    #[serde(rename = "ViewProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventDataStoreRequest {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OrganizationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "StartIngestion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ingestion: Option<bool>,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedEventSelector {
    #[serde(rename = "FieldSelectors")]
    #[serde(default)]
    pub field_selectors: Vec<AdvancedFieldSelector>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedFieldSelector {
    #[serde(rename = "EndsWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<Vec<String>>,
    #[serde(rename = "Equals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<Vec<String>>,
    #[serde(rename = "Field")]
    #[serde(default)]
    pub field: String,
    #[serde(rename = "NotEndsWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ends_with: Option<Vec<String>>,
    #[serde(rename = "NotEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<Vec<String>>,
    #[serde(rename = "NotStartsWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_starts_with: Option<Vec<String>>,
    #[serde(rename = "StartsWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventDataStoreResponse {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OrganizationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrailRequest {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "EnableLogFileValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_file_validation: Option<bool>,
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "SnsTopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrailResponse {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "SnsTopicARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_a_r_n: Option<String>,
    #[serde(rename = "SnsTopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelRequest {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardRequest {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventDataStoreRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventDataStoreResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrailRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrailResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterOrganizationDelegatedAdminRequest {
    #[serde(rename = "DelegatedAdminAccountId")]
    #[serde(default)]
    pub delegated_admin_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterOrganizationDelegatedAdminResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueryRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store: Option<String>,
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "QueryAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_alias: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "RefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueryResponse {
    #[serde(rename = "DeliveryS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_uri: Option<String>,
    #[serde(rename = "DeliveryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "Prompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_statistics: Option<QueryStatisticsForDescribeQuery>,
    #[serde(rename = "QueryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_status: Option<String>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStatisticsForDescribeQuery {
    #[serde(rename = "BytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<i64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EventsMatched")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_matched: Option<i64>,
    #[serde(rename = "EventsScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_scanned: Option<i64>,
    #[serde(rename = "ExecutionTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_in_millis: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrailsRequest {
    #[serde(rename = "includeShadowTrails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shadow_trails: Option<bool>,
    #[serde(rename = "trailNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrailsResponse {
    #[serde(rename = "trailList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_list: Option<Vec<Trail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Trail {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "HasCustomEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_custom_event_selectors: Option<bool>,
    #[serde(rename = "HasInsightSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_insight_selectors: Option<bool>,
    #[serde(rename = "HomeRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "SnsTopicARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_a_r_n: Option<String>,
    #[serde(rename = "SnsTopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableFederationRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableFederationResponse {
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "FederationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableFederationRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
    #[serde(rename = "FederationRoleArn")]
    #[serde(default)]
    pub federation_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableFederationResponse {
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "FederationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_role_arn: Option<String>,
    #[serde(rename = "FederationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateQueryRequest {
    #[serde(rename = "EventDataStores")]
    #[serde(default)]
    pub event_data_stores: Vec<String>,
    #[serde(rename = "Prompt")]
    #[serde(default)]
    pub prompt: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateQueryResponse {
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "QueryAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_alias: Option<String>,
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_statement: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelRequest {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelResponse {
    #[serde(rename = "ChannelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "IngestionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_status: Option<IngestionStatus>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "SourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_config: Option<SourceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionStatus {
    #[serde(rename = "LatestIngestionAttemptEventID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ingestion_attempt_event_i_d: Option<String>,
    #[serde(rename = "LatestIngestionAttemptTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ingestion_attempt_time: Option<f64>,
    #[serde(rename = "LatestIngestionErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ingestion_error_code: Option<String>,
    #[serde(rename = "LatestIngestionSuccessEventID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ingestion_success_event_i_d: Option<String>,
    #[serde(rename = "LatestIngestionSuccessTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ingestion_success_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceConfig {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "ApplyToAllRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to_all_regions: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardRequest {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "LastRefreshFailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_failure_reason: Option<String>,
    #[serde(rename = "LastRefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_id: Option<String>,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<RefreshSchedule>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
    #[serde(rename = "Widgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<Widget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventConfigurationRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store: Option<String>,
    #[serde(rename = "TrailName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventConfigurationResponse {
    #[serde(rename = "AggregationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_configurations: Option<Vec<AggregationConfiguration>>,
    #[serde(rename = "ContextKeySelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_key_selectors: Option<Vec<ContextKeySelector>>,
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "MaxEventSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_event_size: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationConfiguration {
    #[serde(rename = "EventCategory")]
    #[serde(default)]
    pub event_category: String,
    #[serde(rename = "Templates")]
    #[serde(default)]
    pub templates: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextKeySelector {
    #[serde(rename = "Equals")]
    #[serde(default)]
    pub equals: Vec<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventDataStoreRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventDataStoreResponse {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "FederationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_role_arn: Option<String>,
    #[serde(rename = "FederationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_status: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OrganizationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<PartitionKey>>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionKey {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventSelectorsRequest {
    #[serde(rename = "TrailName")]
    #[serde(default)]
    pub trail_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventSelectorsResponse {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "EventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selectors: Option<Vec<EventSelector>>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSelector {
    #[serde(rename = "DataResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_resources: Option<Vec<DataResource>>,
    #[serde(rename = "ExcludeManagementEventSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_management_event_sources: Option<Vec<String>>,
    #[serde(rename = "IncludeManagementEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_management_events: Option<bool>,
    #[serde(rename = "ReadWriteType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_write_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataResource {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportRequest {
    #[serde(rename = "ImportId")]
    #[serde(default)]
    pub import_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,
    #[serde(rename = "EndEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_event_time: Option<f64>,
    #[serde(rename = "ImportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "ImportSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source: Option<ImportSource>,
    #[serde(rename = "ImportStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_statistics: Option<ImportStatistics>,
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "StartEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_event_time: Option<f64>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSource {
    #[serde(rename = "S3")]
    #[serde(default)]
    pub s3: S3ImportSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ImportSource {
    #[serde(rename = "S3BucketAccessRoleArn")]
    #[serde(default)]
    pub s3_bucket_access_role_arn: String,
    #[serde(rename = "S3BucketRegion")]
    #[serde(default)]
    pub s3_bucket_region: String,
    #[serde(rename = "S3LocationUri")]
    #[serde(default)]
    pub s3_location_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportStatistics {
    #[serde(rename = "EventsCompleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_completed: Option<i64>,
    #[serde(rename = "FailedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<i64>,
    #[serde(rename = "FilesCompleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_completed: Option<i64>,
    #[serde(rename = "PrefixesCompleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefixes_completed: Option<i64>,
    #[serde(rename = "PrefixesFound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefixes_found: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightSelectorsRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store: Option<String>,
    #[serde(rename = "TrailName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightSelectorsResponse {
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "InsightSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_selectors: Option<Vec<InsightSelector>>,
    #[serde(rename = "InsightsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_destination: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightSelector {
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    #[serde(rename = "InsightType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryResultsRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store: Option<String>,
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "MaxQueryResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_query_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryResultsResponse {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryResultRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_result_rows: Option<Vec<Vec<std::collections::HashMap<String, String>>>>,
    #[serde(rename = "QueryStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_statistics: Option<QueryStatistics>,
    #[serde(rename = "QueryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStatistics {
    #[serde(rename = "BytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<i64>,
    #[serde(rename = "ResultsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_count: Option<i32>,
    #[serde(rename = "TotalResultsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "DelegatedAdminResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin_resource_policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrailRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrailResponse {
    #[serde(rename = "Trail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail: Option<Trail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrailStatusRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrailStatusResponse {
    #[serde(rename = "IsLogging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_logging: Option<bool>,
    #[serde(rename = "LatestCloudWatchLogsDeliveryError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cloud_watch_logs_delivery_error: Option<String>,
    #[serde(rename = "LatestCloudWatchLogsDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cloud_watch_logs_delivery_time: Option<f64>,
    #[serde(rename = "LatestDeliveryAttemptSucceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_attempt_succeeded: Option<String>,
    #[serde(rename = "LatestDeliveryAttemptTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_attempt_time: Option<String>,
    #[serde(rename = "LatestDeliveryError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_error: Option<String>,
    #[serde(rename = "LatestDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_time: Option<f64>,
    #[serde(rename = "LatestDigestDeliveryError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_digest_delivery_error: Option<String>,
    #[serde(rename = "LatestDigestDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_digest_delivery_time: Option<f64>,
    #[serde(rename = "LatestNotificationAttemptSucceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_attempt_succeeded: Option<String>,
    #[serde(rename = "LatestNotificationAttemptTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_attempt_time: Option<String>,
    #[serde(rename = "LatestNotificationError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_error: Option<String>,
    #[serde(rename = "LatestNotificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_time: Option<f64>,
    #[serde(rename = "StartLoggingTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_logging_time: Option<f64>,
    #[serde(rename = "StopLoggingTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_logging_time: Option<f64>,
    #[serde(rename = "TimeLoggingStarted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_logging_started: Option<String>,
    #[serde(rename = "TimeLoggingStopped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_logging_stopped: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelsResponse {
    #[serde(rename = "Channels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Channel {
    #[serde(rename = "ChannelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashboardsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashboardsResponse {
    #[serde(rename = "Dashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<DashboardDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardDetail {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventDataStoresRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventDataStoresResponse {
    #[serde(rename = "EventDataStores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_stores: Option<Vec<EventDataStore>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDataStore {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OrganizationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportFailuresRequest {
    #[serde(rename = "ImportId")]
    #[serde(default)]
    pub import_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportFailuresResponse {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<ImportFailureListItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportFailureListItem {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportsRequest {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportsResponse {
    #[serde(rename = "Imports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<Vec<ImportsListItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportsListItem {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,
    #[serde(rename = "ImportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsDataRequest {
    #[serde(rename = "DataType")]
    #[serde(default)]
    pub data_type: String,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InsightSource")]
    #[serde(default)]
    pub insight_source: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsDataResponse {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "CloudTrailEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail_event: Option<String>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "EventName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "EventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(rename = "EventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    #[serde(rename = "ReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsMetricDataRequest {
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "EventName")]
    #[serde(default)]
    pub event_name: String,
    #[serde(rename = "EventSource")]
    #[serde(default)]
    pub event_source: String,
    #[serde(rename = "InsightType")]
    #[serde(default)]
    pub insight_type: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "TrailName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsMetricDataResponse {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "EventName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "EventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(rename = "InsightType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_type: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Timestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<f64>>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPublicKeysRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPublicKeysResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PublicKeyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_list: Option<Vec<PublicKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicKey {
    #[serde(rename = "Fingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(rename = "ValidityEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_end_time: Option<f64>,
    #[serde(rename = "ValidityStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_start_time: Option<f64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueriesRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_status: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueriesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Queries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<Query>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Query {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceIdList")]
    #[serde(default)]
    pub resource_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceTagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_list: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrailsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrailsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Trails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trails: Option<Vec<TrailInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrailInfo {
    #[serde(rename = "HomeRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookupEventsRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EventCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category: Option<String>,
    #[serde(rename = "LookupAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_attributes: Option<Vec<LookupAttribute>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookupAttribute {
    #[serde(rename = "AttributeKey")]
    #[serde(default)]
    pub attribute_key: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    pub attribute_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookupEventsResponse {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventConfigurationRequest {
    #[serde(rename = "AggregationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_configurations: Option<Vec<AggregationConfiguration>>,
    #[serde(rename = "ContextKeySelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_key_selectors: Option<Vec<ContextKeySelector>>,
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store: Option<String>,
    #[serde(rename = "MaxEventSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_event_size: Option<String>,
    #[serde(rename = "TrailName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventConfigurationResponse {
    #[serde(rename = "AggregationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_configurations: Option<Vec<AggregationConfiguration>>,
    #[serde(rename = "ContextKeySelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_key_selectors: Option<Vec<ContextKeySelector>>,
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "MaxEventSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_event_size: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventSelectorsRequest {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "EventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selectors: Option<Vec<EventSelector>>,
    #[serde(rename = "TrailName")]
    #[serde(default)]
    pub trail_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventSelectorsResponse {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "EventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selectors: Option<Vec<EventSelector>>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInsightSelectorsRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store: Option<String>,
    #[serde(rename = "InsightSelectors")]
    #[serde(default)]
    pub insight_selectors: Vec<InsightSelector>,
    #[serde(rename = "InsightsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_destination: Option<String>,
    #[serde(rename = "TrailName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInsightSelectorsResponse {
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "InsightSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_selectors: Option<Vec<InsightSelector>>,
    #[serde(rename = "InsightsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_destination: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    pub resource_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "DelegatedAdminResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin_resource_policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterOrganizationDelegatedAdminRequest {
    #[serde(rename = "MemberAccountId")]
    #[serde(default)]
    pub member_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterOrganizationDelegatedAdminResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagsList")]
    #[serde(default)]
    pub tags_list: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreEventDataStoreRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreEventDataStoreResponse {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OrganizationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchSampleQueriesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchPhrase")]
    #[serde(default)]
    pub search_phrase: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchSampleQueriesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SearchSampleQueriesSearchResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchSampleQueriesSearchResult {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Relevance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevance: Option<f32>,
    #[serde(rename = "SQL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_q_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDashboardRefreshRequest {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "QueryParameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameter_values: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDashboardRefreshResponse {
    #[serde(rename = "RefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEventDataStoreIngestionRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEventDataStoreIngestionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportRequest {
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,
    #[serde(rename = "EndEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_event_time: Option<f64>,
    #[serde(rename = "ImportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "ImportSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source: Option<ImportSource>,
    #[serde(rename = "StartEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_event_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,
    #[serde(rename = "EndEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_event_time: Option<f64>,
    #[serde(rename = "ImportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "ImportSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source: Option<ImportSource>,
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "StartEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_event_time: Option<f64>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLoggingRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLoggingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryRequest {
    #[serde(rename = "DeliveryS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_uri: Option<String>,
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "QueryAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_alias: Option<String>,
    #[serde(rename = "QueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<String>>,
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_statement: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryResponse {
    #[serde(rename = "EventDataStoreOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_owner_account_id: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEventDataStoreIngestionRequest {
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEventDataStoreIngestionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopImportRequest {
    #[serde(rename = "ImportId")]
    #[serde(default)]
    pub import_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopImportResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,
    #[serde(rename = "EndEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_event_time: Option<f64>,
    #[serde(rename = "ImportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "ImportSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source: Option<ImportSource>,
    #[serde(rename = "ImportStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_statistics: Option<ImportStatistics>,
    #[serde(rename = "ImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "StartEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_event_time: Option<f64>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopLoggingRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopLoggingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelRequest {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelResponse {
    #[serde(rename = "ChannelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardRequest {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<RefreshSchedule>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "Widgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<RequestWidget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<RefreshSchedule>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
    #[serde(rename = "Widgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<Widget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventDataStoreRequest {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "EventDataStore")]
    #[serde(default)]
    pub event_data_store: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OrganizationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventDataStoreResponse {
    #[serde(rename = "AdvancedEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
    #[serde(rename = "FederationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_role_arn: Option<String>,
    #[serde(rename = "FederationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_status: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiRegionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OrganizationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_enabled: Option<bool>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
    #[serde(rename = "UpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrailRequest {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "EnableLogFileValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_file_validation: Option<bool>,
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "SnsTopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrailResponse {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "SnsTopicARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_a_r_n: Option<String>,
    #[serde(rename = "SnsTopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    #[serde(rename = "TrailARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_a_r_n: Option<String>,
}
