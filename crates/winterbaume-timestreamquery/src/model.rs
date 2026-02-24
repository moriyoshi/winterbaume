//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-timestreamquery

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelQueryRequest {
    #[serde(rename = "QueryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelQueryResponse {
    #[serde(rename = "CancellationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledQueryRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ErrorReportConfiguration")]
    #[serde(default)]
    pub error_report_configuration: ErrorReportConfiguration,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    pub notification_configuration: NotificationConfiguration,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    pub schedule_configuration: ScheduleConfiguration,
    #[serde(rename = "ScheduledQueryExecutionRoleArn")]
    #[serde(default)]
    pub scheduled_query_execution_role_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<TargetConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorReportConfiguration {
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3Configuration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Configuration {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "EncryptionOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_option: Option<String>,
    #[serde(rename = "ObjectKeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationConfiguration {
    #[serde(rename = "SnsConfiguration")]
    #[serde(default)]
    pub sns_configuration: SnsConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnsConfiguration {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleConfiguration {
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetConfiguration {
    #[serde(rename = "TimestreamConfiguration")]
    #[serde(default)]
    pub timestream_configuration: TimestreamConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestreamConfiguration {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "DimensionMappings")]
    #[serde(default)]
    pub dimension_mappings: Vec<DimensionMapping>,
    #[serde(rename = "MeasureNameColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name_column: Option<String>,
    #[serde(rename = "MixedMeasureMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_measure_mappings: Option<Vec<MixedMeasureMapping>>,
    #[serde(rename = "MultiMeasureMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_measure_mappings: Option<MultiMeasureMappings>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TimeColumn")]
    #[serde(default)]
    pub time_column: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionMapping {
    #[serde(rename = "DimensionValueType")]
    #[serde(default)]
    pub dimension_value_type: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MixedMeasureMapping {
    #[serde(rename = "MeasureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name: Option<String>,
    #[serde(rename = "MeasureValueType")]
    #[serde(default)]
    pub measure_value_type: String,
    #[serde(rename = "MultiMeasureAttributeMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_measure_attribute_mappings: Option<Vec<MultiMeasureAttributeMapping>>,
    #[serde(rename = "SourceColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column: Option<String>,
    #[serde(rename = "TargetMeasureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_measure_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiMeasureAttributeMapping {
    #[serde(rename = "MeasureValueType")]
    #[serde(default)]
    pub measure_value_type: String,
    #[serde(rename = "SourceColumn")]
    #[serde(default)]
    pub source_column: String,
    #[serde(rename = "TargetMultiMeasureAttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_multi_measure_attribute_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiMeasureMappings {
    #[serde(rename = "MultiMeasureAttributeMappings")]
    #[serde(default)]
    pub multi_measure_attribute_mappings: Vec<MultiMeasureAttributeMapping>,
    #[serde(rename = "TargetMultiMeasureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_multi_measure_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledQueryResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledQueryRequest {
    #[serde(rename = "ScheduledQueryArn")]
    #[serde(default)]
    pub scheduled_query_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSettingsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSettingsResponse {
    #[serde(rename = "MaxQueryTCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_query_t_c_u: Option<i32>,
    #[serde(rename = "QueryCompute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_compute: Option<QueryComputeResponse>,
    #[serde(rename = "QueryPricingModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_pricing_model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryComputeResponse {
    #[serde(rename = "ComputeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_mode: Option<String>,
    #[serde(rename = "ProvisionedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity: Option<ProvisionedCapacityResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedCapacityResponse {
    #[serde(rename = "ActiveQueryTCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_query_t_c_u: Option<i32>,
    #[serde(rename = "LastUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<LastUpdate>,
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<AccountSettingsNotificationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastUpdate {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TargetQueryTCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_query_t_c_u: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSettingsNotificationConfiguration {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SnsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_configuration: Option<SnsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointsResponse {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "CachePeriodInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_period_in_minutes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduledQueryRequest {
    #[serde(rename = "ScheduledQueryArn")]
    #[serde(default)]
    pub scheduled_query_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduledQueryResponse {
    #[serde(rename = "ScheduledQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query: Option<ScheduledQueryDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledQueryDescription {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ErrorReportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_report_configuration: Option<ErrorReportConfiguration>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LastRunSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_summary: Option<ScheduledQueryRunSummary>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextInvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invocation_time: Option<f64>,
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    #[serde(rename = "PreviousInvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_invocation_time: Option<f64>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "RecentlyFailedRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recently_failed_runs: Option<Vec<ScheduledQueryRunSummary>>,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_configuration: Option<ScheduleConfiguration>,
    #[serde(rename = "ScheduledQueryExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query_execution_role_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TargetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<TargetConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledQueryRunSummary {
    #[serde(rename = "ErrorReportLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_report_location: Option<ErrorReportLocation>,
    #[serde(rename = "ExecutionStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stats: Option<ExecutionStats>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "InvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_time: Option<f64>,
    #[serde(rename = "QueryInsightsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_insights_response: Option<ScheduledQueryInsightsResponse>,
    #[serde(rename = "RunStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    #[serde(rename = "TriggerTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorReportLocation {
    #[serde(rename = "S3ReportLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_report_location: Option<S3ReportLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ReportLocation {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "ObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionStats {
    #[serde(rename = "BytesMetered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_metered: Option<i64>,
    #[serde(rename = "CumulativeBytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_bytes_scanned: Option<i64>,
    #[serde(rename = "DataWrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_writes: Option<i64>,
    #[serde(rename = "ExecutionTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_in_millis: Option<i64>,
    #[serde(rename = "QueryResultRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_result_rows: Option<i64>,
    #[serde(rename = "RecordsIngested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_ingested: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledQueryInsightsResponse {
    #[serde(rename = "OutputBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bytes: Option<i64>,
    #[serde(rename = "OutputRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rows: Option<i64>,
    #[serde(rename = "QuerySpatialCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_spatial_coverage: Option<QuerySpatialCoverage>,
    #[serde(rename = "QueryTableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_table_count: Option<i64>,
    #[serde(rename = "QueryTemporalRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_temporal_range: Option<QueryTemporalRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuerySpatialCoverage {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<QuerySpatialCoverageMax>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuerySpatialCoverageMax {
    #[serde(rename = "PartitionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<Vec<String>>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryTemporalRange {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<QueryTemporalRangeMax>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryTemporalRangeMax {
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteScheduledQueryRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InvocationTime")]
    #[serde(default)]
    pub invocation_time: f64,
    #[serde(rename = "QueryInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_insights: Option<ScheduledQueryInsights>,
    #[serde(rename = "ScheduledQueryArn")]
    #[serde(default)]
    pub scheduled_query_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledQueryInsights {
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduledQueriesRequest {
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
pub struct ListScheduledQueriesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScheduledQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_queries: Option<Vec<ScheduledQuery>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledQuery {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ErrorReportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_report_configuration: Option<ErrorReportConfiguration>,
    #[serde(rename = "LastRunStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_status: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextInvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invocation_time: Option<f64>,
    #[serde(rename = "PreviousInvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_invocation_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TargetDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_destination: Option<TargetDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetDestination {
    #[serde(rename = "TimestreamDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestream_destination: Option<TimestreamDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestreamDestination {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrepareQueryRequest {
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "ValidateOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrepareQueryResponse {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<SelectColumn>>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterMapping>>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectColumn {
    #[serde(rename = "Aliased")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliased: Option<bool>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Type {
    #[serde(rename = "ArrayColumnInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_column_info: Option<Box<ColumnInfo>>,
    #[serde(rename = "RowColumnInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_column_info: Option<Vec<ColumnInfo>>,
    #[serde(rename = "ScalarType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_type: Option<String>,
    #[serde(rename = "TimeSeriesMeasureValueColumnInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_measure_value_column_info: Option<Box<ColumnInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnInfo {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<Type>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterMapping {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "MaxRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rows: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_insights: Option<QueryInsights>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryInsights {
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryResponse {
    #[serde(rename = "ColumnInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_info: Option<Vec<ColumnInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryInsightsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_insights_response: Option<QueryInsightsResponse>,
    #[serde(rename = "QueryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_status: Option<QueryStatus>,
    #[serde(rename = "Rows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Row>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryInsightsResponse {
    #[serde(rename = "OutputBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bytes: Option<i64>,
    #[serde(rename = "OutputRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rows: Option<i64>,
    #[serde(rename = "QuerySpatialCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_spatial_coverage: Option<QuerySpatialCoverage>,
    #[serde(rename = "QueryTableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_table_count: Option<i64>,
    #[serde(rename = "QueryTemporalRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_temporal_range: Option<QueryTemporalRange>,
    #[serde(rename = "UnloadPartitionCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unload_partition_count: Option<i64>,
    #[serde(rename = "UnloadWrittenBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unload_written_bytes: Option<i64>,
    #[serde(rename = "UnloadWrittenRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unload_written_rows: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStatus {
    #[serde(rename = "CumulativeBytesMetered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_bytes_metered: Option<i64>,
    #[serde(rename = "CumulativeBytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_bytes_scanned: Option<i64>,
    #[serde(rename = "ProgressPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Row {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Datum>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Datum {
    #[serde(rename = "ArrayValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_value: Option<Vec<Datum>>,
    #[serde(rename = "NullValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value: Option<bool>,
    #[serde(rename = "RowValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_value: Option<Box<Row>>,
    #[serde(rename = "ScalarValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_value: Option<String>,
    #[serde(rename = "TimeSeriesValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_value: Option<Vec<TimeSeriesDataPoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeSeriesDataPoint {
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<Datum>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsRequest {
    #[serde(rename = "MaxQueryTCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_query_t_c_u: Option<i32>,
    #[serde(rename = "QueryCompute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_compute: Option<QueryComputeRequest>,
    #[serde(rename = "QueryPricingModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_pricing_model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryComputeRequest {
    #[serde(rename = "ComputeMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_mode: Option<String>,
    #[serde(rename = "ProvisionedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity: Option<ProvisionedCapacityRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedCapacityRequest {
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<AccountSettingsNotificationConfiguration>,
    #[serde(rename = "TargetQueryTCU")]
    #[serde(default)]
    pub target_query_t_c_u: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsResponse {
    #[serde(rename = "MaxQueryTCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_query_t_c_u: Option<i32>,
    #[serde(rename = "QueryCompute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_compute: Option<QueryComputeResponse>,
    #[serde(rename = "QueryPricingModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_pricing_model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledQueryRequest {
    #[serde(rename = "ScheduledQueryArn")]
    #[serde(default)]
    pub scheduled_query_arn: String,
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
}
