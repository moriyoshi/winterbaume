//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudwatchlogs

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateKmsKeyRequest {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    pub kms_key_id: String,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSourceToS3TableIntegrationRequest {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    pub integration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSourceToS3TableIntegrationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelExportTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelImportTaskRequest {
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelImportTaskResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_statistics: Option<ImportStatistics>,
    #[serde(rename = "importStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportStatistics {
    #[serde(rename = "bytesImported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_imported: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeliveryRequest {
    #[serde(rename = "deliveryDestinationArn")]
    #[serde(default)]
    pub delivery_destination_arn: String,
    #[serde(rename = "deliverySourceName")]
    #[serde(default)]
    pub delivery_source_name: String,
    #[serde(rename = "fieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(rename = "recordFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_fields: Option<Vec<String>>,
    #[serde(rename = "s3DeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delivery_configuration: Option<S3DeliveryConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DeliveryConfiguration {
    #[serde(rename = "enableHiveCompatiblePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_hive_compatible_path: Option<bool>,
    #[serde(rename = "suffixPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeliveryResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Delivery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Delivery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "deliveryDestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_arn: Option<String>,
    #[serde(rename = "deliveryDestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_type: Option<String>,
    #[serde(rename = "deliverySourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_source_name: Option<String>,
    #[serde(rename = "fieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "recordFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_fields: Option<Vec<String>>,
    #[serde(rename = "s3DeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delivery_configuration: Option<S3DeliveryConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportTaskRequest {
    #[serde(default)]
    pub destination: String,
    #[serde(rename = "destinationPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix: Option<String>,
    #[serde(default)]
    pub from: i64,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "taskName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[serde(default)]
    pub to: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportTaskResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateImportTaskRequest {
    #[serde(rename = "importFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_filter: Option<ImportFilter>,
    #[serde(rename = "importRoleArn")]
    #[serde(default)]
    pub import_role_arn: String,
    #[serde(rename = "importSourceArn")]
    #[serde(default)]
    pub import_source_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportFilter {
    #[serde(rename = "endEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_event_time: Option<i64>,
    #[serde(rename = "startEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_event_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateImportTaskResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "importDestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination_arn: Option<String>,
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogAnomalyDetectorRequest {
    #[serde(rename = "anomalyVisibilityTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_visibility_time: Option<i64>,
    #[serde(rename = "detectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_name: Option<String>,
    #[serde(rename = "evaluationFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "logGroupArnList")]
    #[serde(default)]
    pub log_group_arn_list: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogAnomalyDetectorResponse {
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogGroupRequest {
    #[serde(rename = "deletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "logGroupClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLogStreamRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    pub log_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLookupTableRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "lookupTableName")]
    #[serde(default)]
    pub lookup_table_name: String,
    #[serde(rename = "tableBody")]
    #[serde(default)]
    pub table_body: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLookupTableResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "lookupTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledQueryRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    pub query_language: String,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "scheduleEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_end_time: Option<i64>,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "scheduleStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start_time: Option<i64>,
    #[serde(rename = "startTimeOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_offset: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConfiguration {
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3Configuration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Configuration {
    #[serde(rename = "destinationIdentifier")]
    #[serde(default)]
    pub destination_identifier: String,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledQueryResponse {
    #[serde(rename = "scheduledQueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountPolicyRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "policyType")]
    #[serde(default)]
    pub policy_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataProtectionPolicyRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeliveryDestinationPolicyRequest {
    #[serde(rename = "deliveryDestinationName")]
    #[serde(default)]
    pub delivery_destination_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeliveryDestinationRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeliveryRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeliverySourceRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDestinationRequest {
    #[serde(rename = "destinationName")]
    #[serde(default)]
    pub destination_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexPolicyRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "integrationName")]
    #[serde(default)]
    pub integration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLogAnomalyDetectorRequest {
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    pub anomaly_detector_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLogGroupRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLogStreamRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    pub log_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLookupTableRequest {
    #[serde(rename = "lookupTableArn")]
    #[serde(default)]
    pub lookup_table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMetricFilterRequest {
    #[serde(rename = "filterName")]
    #[serde(default)]
    pub filter_name: String,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueryDefinitionRequest {
    #[serde(rename = "queryDefinitionId")]
    #[serde(default)]
    pub query_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueryDefinitionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "expectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRetentionPolicyRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledQueryRequest {
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledQueryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubscriptionFilterRequest {
    #[serde(rename = "filterName")]
    #[serde(default)]
    pub filter_name: String,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTransformerRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountPoliciesRequest {
    #[serde(rename = "accountIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifiers: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyType")]
    #[serde(default)]
    pub policy_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountPoliciesResponse {
    #[serde(rename = "accountPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_policies: Option<Vec<AccountPolicy>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountPolicy {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "selectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_criteria: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationTemplatesRequest {
    #[serde(rename = "deliveryDestinationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_types: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationTemplatesResponse {
    #[serde(rename = "configurationTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_templates: Option<Vec<ConfigurationTemplate>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationTemplate {
    #[serde(rename = "allowedActionForAllowVendedLogsDeliveryForResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_action_for_allow_vended_logs_delivery_for_resource: Option<String>,
    #[serde(rename = "allowedFieldDelimiters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_field_delimiters: Option<Vec<String>>,
    #[serde(rename = "allowedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_fields: Option<Vec<RecordField>>,
    #[serde(rename = "allowedOutputFormats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_output_formats: Option<Vec<String>>,
    #[serde(rename = "allowedSuffixPathFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_suffix_path_fields: Option<Vec<String>>,
    #[serde(rename = "defaultDeliveryConfigValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_delivery_config_values: Option<ConfigurationTemplateDeliveryConfigValues>,
    #[serde(rename = "deliveryDestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_type: Option<String>,
    #[serde(rename = "logType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordField {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationTemplateDeliveryConfigValues {
    #[serde(rename = "fieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(rename = "recordFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_fields: Option<Vec<String>>,
    #[serde(rename = "s3DeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delivery_configuration: Option<S3DeliveryConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveriesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveriesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<Vec<Delivery>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryDestinationsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryDestinationsResponse {
    #[serde(rename = "deliveryDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destinations: Option<Vec<DeliveryDestination>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryDestination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "deliveryDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_configuration: Option<DeliveryDestinationConfiguration>,
    #[serde(rename = "deliveryDestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryDestinationConfiguration {
    #[serde(rename = "destinationResourceArn")]
    #[serde(default)]
    pub destination_resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliverySourcesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliverySourcesResponse {
    #[serde(rename = "deliverySources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_sources: Option<Vec<DeliverySource>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliverySource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "logType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDestinationsRequest {
    #[serde(rename = "DestinationNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name_prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDestinationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "accessPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "destinationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportTasksRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportTasksResponse {
    #[serde(rename = "exportTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_tasks: Option<Vec<ExportTask>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportTask {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "destinationPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix: Option<String>,
    #[serde(rename = "executionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<ExportTaskExecutionInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ExportTaskStatus>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "taskName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportTaskExecutionInfo {
    #[serde(rename = "completionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<i64>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportTaskStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFieldIndexesRequest {
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    pub log_group_identifiers: Vec<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFieldIndexesResponse {
    #[serde(rename = "fieldIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_indexes: Option<Vec<FieldIndex>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldIndex {
    #[serde(rename = "fieldIndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_index_name: Option<String>,
    #[serde(rename = "firstEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_event_time: Option<i64>,
    #[serde(rename = "lastEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_time: Option<i64>,
    #[serde(rename = "lastScanTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scan_time: Option<i64>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportTaskBatchesRequest {
    #[serde(rename = "batchImportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_import_status: Option<Vec<String>>,
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportTaskBatchesResponse {
    #[serde(rename = "importBatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_batches: Option<Vec<ImportBatch>>,
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source_arn: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportBatch {
    #[serde(rename = "batchId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportTasksRequest {
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source_arn: Option<String>,
    #[serde(rename = "importStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportTasksResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<Vec<Import>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Import {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "importDestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination_arn: Option<String>,
    #[serde(rename = "importFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_filter: Option<ImportFilter>,
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source_arn: Option<String>,
    #[serde(rename = "importStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_statistics: Option<ImportStatistics>,
    #[serde(rename = "importStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIndexPoliciesRequest {
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    pub log_group_identifiers: Vec<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIndexPoliciesResponse {
    #[serde(rename = "indexPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_policies: Option<Vec<IndexPolicy>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexPolicy {
    #[serde(rename = "lastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<i64>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLogGroupsRequest {
    #[serde(rename = "accountIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifiers: Option<Vec<String>>,
    #[serde(rename = "includeLinkedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<String>,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(rename = "logGroupNamePattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name_pattern: Option<String>,
    #[serde(rename = "logGroupNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name_prefix: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLogGroupsResponse {
    #[serde(rename = "logGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_groups: Option<Vec<LogGroup>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "bearerTokenAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_token_authentication_enabled: Option<bool>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "dataProtectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_status: Option<String>,
    #[serde(rename = "deletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "inheritedProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_properties: Option<Vec<String>>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
    #[serde(rename = "logGroupClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "metricFilterCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter_count: Option<i32>,
    #[serde(rename = "retentionInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
    #[serde(rename = "storedBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLogStreamsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descending: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLogStreamsResponse {
    #[serde(rename = "logStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_streams: Option<Vec<LogStream>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogStream {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "firstEventTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_event_timestamp: Option<i64>,
    #[serde(rename = "lastEventTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_timestamp: Option<i64>,
    #[serde(rename = "lastIngestionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ingestion_time: Option<i64>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "storedBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_bytes: Option<i64>,
    #[serde(rename = "uploadSequenceToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_sequence_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLookupTablesRequest {
    #[serde(rename = "lookupTableNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_name_prefix: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLookupTablesResponse {
    #[serde(rename = "lookupTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_tables: Option<Vec<LookupTable>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookupTable {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "lookupTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_arn: Option<String>,
    #[serde(rename = "lookupTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_name: Option<String>,
    #[serde(rename = "recordsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_count: Option<i64>,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "tableFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetricFiltersRequest {
    #[serde(rename = "filterNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name_prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "metricNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetricFiltersResponse {
    #[serde(rename = "metricFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filters: Option<Vec<MetricFilter>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricFilter {
    #[serde(rename = "applyOnTransformedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on_transformed_logs: Option<bool>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "emitSystemFieldDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_system_field_dimensions: Option<Vec<String>>,
    #[serde(rename = "fieldSelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_selection_criteria: Option<String>,
    #[serde(rename = "filterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "metricTransformations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_transformations: Option<Vec<MetricTransformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricTransformation {
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "metricNamespace")]
    #[serde(default)]
    pub metric_namespace: String,
    #[serde(rename = "metricValue")]
    #[serde(default)]
    pub metric_value: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueriesRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueriesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<QueryInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryInfo {
    #[serde(rename = "bytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<f64>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "queryDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_duration: Option<i64>,
    #[serde(rename = "queryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "userIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueryDefinitionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queryDefinitionNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_definition_name_prefix: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueryDefinitionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queryDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_definitions: Option<Vec<QueryDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryDefinition {
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<i64>,
    #[serde(rename = "logGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<QueryParameter>>,
    #[serde(rename = "queryDefinitionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_definition_id: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryParameter {
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePoliciesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_scope: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourcePolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policies: Option<Vec<ResourcePolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcePolicy {
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_scope: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubscriptionFiltersRequest {
    #[serde(rename = "filterNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name_prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubscriptionFiltersResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "subscriptionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_filters: Option<Vec<SubscriptionFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscriptionFilter {
    #[serde(rename = "applyOnTransformedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on_transformed_logs: Option<bool>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(rename = "emitSystemFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_system_fields: Option<Vec<String>>,
    #[serde(rename = "fieldSelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_selection_criteria: Option<String>,
    #[serde(rename = "filterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateKmsKeyRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSourceFromS3TableIntegrationRequest {
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSourceFromS3TableIntegrationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterLogEventsRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interleaved: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "logStreamNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmask: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterLogEventsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<FilteredLogEvent>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "searchedLogStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_log_streams: Option<Vec<SearchedLogStream>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilteredLogEvent {
    #[serde(rename = "eventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "ingestionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time: Option<i64>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchedLogStream {
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "searchedCompletely")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_completely: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataProtectionPolicyRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataProtectionPolicyResponse {
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliveryDestinationPolicyRequest {
    #[serde(rename = "deliveryDestinationName")]
    #[serde(default)]
    pub delivery_destination_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliveryDestinationPolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Policy {
    #[serde(rename = "deliveryDestinationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliveryDestinationRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliveryDestinationResponse {
    #[serde(rename = "deliveryDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination: Option<DeliveryDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliveryRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliveryResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Delivery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliverySourceRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliverySourceResponse {
    #[serde(rename = "deliverySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_source: Option<DeliverySource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationRequest {
    #[serde(rename = "integrationName")]
    #[serde(default)]
    pub integration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResponse {
    #[serde(rename = "integrationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_details: Option<IntegrationDetails>,
    #[serde(rename = "integrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "integrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_status: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationDetails {
    #[serde(rename = "openSearchIntegrationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_integration_details: Option<OpenSearchIntegrationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchIntegrationDetails {
    #[serde(rename = "accessPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy: Option<OpenSearchDataAccessPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<OpenSearchApplication>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<OpenSearchCollection>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<OpenSearchDataSource>,
    #[serde(rename = "encryptionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_policy: Option<OpenSearchEncryptionPolicy>,
    #[serde(rename = "lifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy: Option<OpenSearchLifecyclePolicy>,
    #[serde(rename = "networkPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<OpenSearchNetworkPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<OpenSearchWorkspace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchDataAccessPolicy {
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchResourceStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchApplication {
    #[serde(rename = "applicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "applicationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_endpoint: Option<String>,
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchCollection {
    #[serde(rename = "collectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    #[serde(rename = "collectionEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchDataSource {
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchEncryptionPolicy {
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchLifecyclePolicy {
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchNetworkPolicy {
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchWorkspace {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenSearchResourceStatus>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogAnomalyDetectorRequest {
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    pub anomaly_detector_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogAnomalyDetectorResponse {
    #[serde(rename = "anomalyDetectorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_status: Option<String>,
    #[serde(rename = "anomalyVisibilityTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_visibility_time: Option<i64>,
    #[serde(rename = "creationTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_stamp: Option<i64>,
    #[serde(rename = "detectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_name: Option<String>,
    #[serde(rename = "evaluationFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "lastModifiedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_stamp: Option<i64>,
    #[serde(rename = "logGroupArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogEventsRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    pub log_stream_name: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startFromHead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_head: Option<bool>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmask: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogEventsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<OutputLogEvent>>,
    #[serde(rename = "nextBackwardToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_backward_token: Option<String>,
    #[serde(rename = "nextForwardToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_forward_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputLogEvent {
    #[serde(rename = "ingestionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogFieldsRequest {
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
    #[serde(rename = "dataSourceType")]
    #[serde(default)]
    pub data_source_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogFieldsResponse {
    #[serde(rename = "logFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_fields: Option<Vec<LogFieldsListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogFieldsListItem {
    #[serde(rename = "logFieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_field_name: Option<String>,
    #[serde(rename = "logFieldType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_field_type: Option<Box<LogFieldType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogFieldType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<Box<LogFieldType>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<LogFieldsListItem>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogGroupFieldsRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogGroupFieldsResponse {
    #[serde(rename = "logGroupFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_fields: Option<Vec<LogGroupField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogGroupField {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogObjectRequest {
    #[serde(rename = "logObjectPointer")]
    #[serde(default)]
    pub log_object_pointer: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmask: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogObjectResponse {
    #[serde(rename = "fieldStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_stream: Option<GetLogObjectResponseStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogObjectResponseStream {
    #[serde(rename = "InternalStreamingException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_streaming_exception: Option<InternalStreamingException>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<FieldsData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalStreamingException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldsData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogRecordRequest {
    #[serde(rename = "logRecordPointer")]
    #[serde(default)]
    pub log_record_pointer: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmask: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogRecordResponse {
    #[serde(rename = "logRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_record: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLookupTableRequest {
    #[serde(rename = "lookupTableArn")]
    #[serde(default)]
    pub lookup_table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLookupTableResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "lookupTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_arn: Option<String>,
    #[serde(rename = "lookupTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_name: Option<String>,
    #[serde(rename = "sizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "tableBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryResultsRequest {
    #[serde(rename = "maxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryResultsResponse {
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Vec<ResultField>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<QueryStatistics>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultField {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStatistics {
    #[serde(rename = "bytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<f64>,
    #[serde(rename = "estimatedBytesSkipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_bytes_skipped: Option<f64>,
    #[serde(rename = "estimatedRecordsSkipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_records_skipped: Option<f64>,
    #[serde(rename = "logGroupsScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_groups_scanned: Option<f64>,
    #[serde(rename = "recordsMatched")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_matched: Option<f64>,
    #[serde(rename = "recordsScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_scanned: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduledQueryHistoryRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: i64,
    #[serde(rename = "executionStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_statuses: Option<Vec<String>>,
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduledQueryHistoryResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scheduledQueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query_arn: Option<String>,
    #[serde(rename = "triggerHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_history: Option<Vec<TriggerHistoryRecord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerHistoryRecord {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<ScheduledQueryDestination>>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "executionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(rename = "queryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "triggeredTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledQueryDestination {
    #[serde(rename = "destinationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_identifier: Option<String>,
    #[serde(rename = "destinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "processedIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduledQueryRequest {
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduledQueryResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "lastExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_status: Option<String>,
    #[serde(rename = "lastTriggeredTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered_time: Option<i64>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "scheduleEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_end_time: Option<i64>,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "scheduleStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start_time: Option<i64>,
    #[serde(rename = "scheduledQueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query_arn: Option<String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_offset: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransformerRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransformerResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<i64>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "transformerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformer_config: Option<Vec<Processor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Processor {
    #[serde(rename = "addKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_keys: Option<AddKeys>,
    #[serde(rename = "copyValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_value: Option<CopyValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CSV>,
    #[serde(rename = "dateTimeConverter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_converter: Option<DateTimeConverter>,
    #[serde(rename = "deleteKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_keys: Option<DeleteKeys>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok: Option<Grok>,
    #[serde(rename = "listToMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_to_map: Option<ListToMap>,
    #[serde(rename = "lowerCaseString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_case_string: Option<LowerCaseString>,
    #[serde(rename = "moveKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_keys: Option<MoveKeys>,
    #[serde(rename = "parseCloudfront")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_cloudfront: Option<ParseCloudfront>,
    #[serde(rename = "parseJSON")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_j_s_o_n: Option<ParseJSON>,
    #[serde(rename = "parseKeyValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_key_value: Option<ParseKeyValue>,
    #[serde(rename = "parsePostgres")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_postgres: Option<ParsePostgres>,
    #[serde(rename = "parseRoute53")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_route53: Option<ParseRoute53>,
    #[serde(rename = "parseToOCSF")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_to_o_c_s_f: Option<ParseToOCSF>,
    #[serde(rename = "parseVPC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_v_p_c: Option<ParseVPC>,
    #[serde(rename = "parseWAF")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_w_a_f: Option<ParseWAF>,
    #[serde(rename = "renameKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_keys: Option<RenameKeys>,
    #[serde(rename = "splitString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_string: Option<SplitString>,
    #[serde(rename = "substituteString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitute_string: Option<SubstituteString>,
    #[serde(rename = "trimString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_string: Option<TrimString>,
    #[serde(rename = "typeConverter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_converter: Option<TypeConverter>,
    #[serde(rename = "upperCaseString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_case_string: Option<UpperCaseString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddKeys {
    #[serde(default)]
    pub entries: Vec<AddKeyEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddKeyEntry {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "overwriteIfExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_if_exists: Option<bool>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyValue {
    #[serde(default)]
    pub entries: Vec<CopyValueEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyValueEntry {
    #[serde(rename = "overwriteIfExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_if_exists: Option<bool>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CSV {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "quoteCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeConverter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "matchPatterns")]
    #[serde(default)]
    pub match_patterns: Vec<String>,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "sourceTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_timezone: Option<String>,
    #[serde(default)]
    pub target: String,
    #[serde(rename = "targetFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_format: Option<String>,
    #[serde(rename = "targetTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKeys {
    #[serde(rename = "withKeys")]
    #[serde(default)]
    pub with_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Grok {
    #[serde(rename = "match")]
    #[serde(default)]
    pub r#match: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListToMap {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
    #[serde(rename = "flattenedElement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flattened_element: Option<String>,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "valueKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LowerCaseString {
    #[serde(rename = "withKeys")]
    #[serde(default)]
    pub with_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MoveKeys {
    #[serde(default)]
    pub entries: Vec<MoveKeyEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MoveKeyEntry {
    #[serde(rename = "overwriteIfExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_if_exists: Option<bool>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParseCloudfront {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParseJSON {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParseKeyValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "fieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(rename = "keyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    #[serde(rename = "keyValueDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value_delimiter: Option<String>,
    #[serde(rename = "nonMatchValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_match_value: Option<String>,
    #[serde(rename = "overwriteIfExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_if_exists: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParsePostgres {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParseRoute53 {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParseToOCSF {
    #[serde(rename = "eventSource")]
    #[serde(default)]
    pub event_source: String,
    #[serde(rename = "mappingVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_version: Option<String>,
    #[serde(rename = "ocsfVersion")]
    #[serde(default)]
    pub ocsf_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParseVPC {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParseWAF {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenameKeys {
    #[serde(default)]
    pub entries: Vec<RenameKeyEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenameKeyEntry {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "overwriteIfExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_if_exists: Option<bool>,
    #[serde(rename = "renameTo")]
    #[serde(default)]
    pub rename_to: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplitString {
    #[serde(default)]
    pub entries: Vec<SplitStringEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplitStringEntry {
    #[serde(default)]
    pub delimiter: String,
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubstituteString {
    #[serde(default)]
    pub entries: Vec<SubstituteStringEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubstituteStringEntry {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub to: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrimString {
    #[serde(rename = "withKeys")]
    #[serde(default)]
    pub with_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypeConverter {
    #[serde(default)]
    pub entries: Vec<TypeConverterEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypeConverterEntry {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpperCaseString {
    #[serde(rename = "withKeys")]
    #[serde(default)]
    pub with_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAggregateLogGroupSummariesRequest {
    #[serde(rename = "accountIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifiers: Option<Vec<String>>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSourceFilter>>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    pub group_by: String,
    #[serde(rename = "includeLinkedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<String>,
    #[serde(rename = "logGroupNamePattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name_pattern: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceFilter {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAggregateLogGroupSummariesResponse {
    #[serde(rename = "aggregateLogGroupSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_log_group_summaries: Option<Vec<AggregateLogGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateLogGroupSummary {
    #[serde(rename = "groupingIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_identifiers: Option<Vec<GroupingIdentifier>>,
    #[serde(rename = "logGroupCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupingIdentifier {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnomaliesRequest {
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "suppressionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnomaliesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomalies: Option<Vec<Anomaly>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Anomaly {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_arn: Option<String>,
    #[serde(rename = "anomalyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "firstSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub histogram: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "isPatternLevelSuppression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pattern_level_suppression: Option<bool>,
    #[serde(rename = "lastSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<i64>,
    #[serde(rename = "logGroupArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn_list: Option<Vec<String>>,
    #[serde(rename = "logSamples")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_samples: Option<Vec<LogEvent>>,
    #[serde(rename = "patternId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_id: Option<String>,
    #[serde(rename = "patternRegex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_regex: Option<String>,
    #[serde(rename = "patternString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_string: Option<String>,
    #[serde(rename = "patternTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_tokens: Option<Vec<PatternToken>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<bool>,
    #[serde(rename = "suppressedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_date: Option<i64>,
    #[serde(rename = "suppressedUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_until: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatternToken {
    #[serde(rename = "dynamicTokenPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_token_position: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumerations: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "inferredTokenName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred_token_name: Option<String>,
    #[serde(rename = "isDynamic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
    #[serde(rename = "tokenString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntegrationsRequest {
    #[serde(rename = "integrationNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name_prefix: Option<String>,
    #[serde(rename = "integrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_status: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntegrationsResponse {
    #[serde(rename = "integrationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_summaries: Option<Vec<IntegrationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationSummary {
    #[serde(rename = "integrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "integrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_status: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogAnomalyDetectorsRequest {
    #[serde(rename = "filterLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_log_group_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogAnomalyDetectorsResponse {
    #[serde(rename = "anomalyDetectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detectors: Option<Vec<AnomalyDetector>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetector {
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_arn: Option<String>,
    #[serde(rename = "anomalyDetectorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_status: Option<String>,
    #[serde(rename = "anomalyVisibilityTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_visibility_time: Option<i64>,
    #[serde(rename = "creationTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_stamp: Option<i64>,
    #[serde(rename = "detectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_name: Option<String>,
    #[serde(rename = "evaluationFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "lastModifiedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time_stamp: Option<i64>,
    #[serde(rename = "logGroupArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogGroupsForQueryRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogGroupsForQueryResponse {
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogGroupsRequest {
    #[serde(rename = "accountIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifiers: Option<Vec<String>>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSourceFilter>>,
    #[serde(rename = "fieldIndexNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_index_names: Option<Vec<String>>,
    #[serde(rename = "includeLinkedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<String>,
    #[serde(rename = "logGroupNamePattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name_pattern: Option<String>,
    #[serde(rename = "logGroupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_tags: Option<Vec<TagFilter>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagFilter {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLogGroupsResponse {
    #[serde(rename = "logGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_groups: Option<Vec<LogGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogGroupSummary {
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
    #[serde(rename = "logGroupClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_class: Option<String>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduledQueriesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduledQueriesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scheduledQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_queries: Option<Vec<ScheduledQuerySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledQuerySummary {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(rename = "lastExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_status: Option<String>,
    #[serde(rename = "lastTriggeredTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered_time: Option<i64>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "scheduledQueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourcesForS3TableIntegrationRequest {
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    pub integration_arn: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourcesForS3TableIntegrationResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<S3TableIntegrationSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3TableIntegrationSource {
    #[serde(rename = "createdTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<i64>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "parentSourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_source_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsLogGroupRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsLogGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountPolicyRequest {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "policyType")]
    #[serde(default)]
    pub policy_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "selectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_criteria: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountPolicyResponse {
    #[serde(rename = "accountPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_policy: Option<AccountPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutBearerTokenAuthenticationRequest {
    #[serde(rename = "bearerTokenAuthenticationEnabled")]
    #[serde(default)]
    pub bearer_token_authentication_enabled: bool,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataProtectionPolicyRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataProtectionPolicyResponse {
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliveryDestinationPolicyRequest {
    #[serde(rename = "deliveryDestinationName")]
    #[serde(default)]
    pub delivery_destination_name: String,
    #[serde(rename = "deliveryDestinationPolicy")]
    #[serde(default)]
    pub delivery_destination_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliveryDestinationPolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliveryDestinationRequest {
    #[serde(rename = "deliveryDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_configuration: Option<DeliveryDestinationConfiguration>,
    #[serde(rename = "deliveryDestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination_type: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "outputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliveryDestinationResponse {
    #[serde(rename = "deliveryDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_destination: Option<DeliveryDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliverySourceRequest {
    #[serde(rename = "logType")]
    #[serde(default)]
    pub log_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliverySourceResponse {
    #[serde(rename = "deliverySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_source: Option<DeliverySource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDestinationPolicyRequest {
    #[serde(rename = "accessPolicy")]
    #[serde(default)]
    pub access_policy: String,
    #[serde(rename = "destinationName")]
    #[serde(default)]
    pub destination_name: String,
    #[serde(rename = "forceUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDestinationRequest {
    #[serde(rename = "destinationName")]
    #[serde(default)]
    pub destination_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDestinationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutIndexPolicyRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutIndexPolicyResponse {
    #[serde(rename = "indexPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_policy: Option<IndexPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutIntegrationRequest {
    #[serde(rename = "integrationName")]
    #[serde(default)]
    pub integration_name: String,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    pub integration_type: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    pub resource_config: ResourceConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceConfig {
    #[serde(rename = "openSearchResourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_resource_config: Option<OpenSearchResourceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchResourceConfig {
    #[serde(rename = "applicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "dashboardViewerPrincipals")]
    #[serde(default)]
    pub dashboard_viewer_principals: Vec<String>,
    #[serde(rename = "dataSourceRoleArn")]
    #[serde(default)]
    pub data_source_role_arn: String,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "retentionDays")]
    #[serde(default)]
    pub retention_days: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutIntegrationResponse {
    #[serde(rename = "integrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "integrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLogEventsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
    #[serde(rename = "logEvents")]
    #[serde(default)]
    pub log_events: Vec<InputLogEvent>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    pub log_stream_name: String,
    #[serde(rename = "sequenceToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Entity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "keyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputLogEvent {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub timestamp: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLogEventsResponse {
    #[serde(rename = "nextSequenceToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_sequence_token: Option<String>,
    #[serde(rename = "rejectedEntityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_entity_info: Option<RejectedEntityInfo>,
    #[serde(rename = "rejectedLogEventsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_log_events_info: Option<RejectedLogEventsInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectedEntityInfo {
    #[serde(rename = "errorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectedLogEventsInfo {
    #[serde(rename = "expiredLogEventEndIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_log_event_end_index: Option<i32>,
    #[serde(rename = "tooNewLogEventStartIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_new_log_event_start_index: Option<i32>,
    #[serde(rename = "tooOldLogEventEndIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_old_log_event_end_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLogGroupDeletionProtectionRequest {
    #[serde(rename = "deletionProtectionEnabled")]
    #[serde(default)]
    pub deletion_protection_enabled: bool,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetricFilterRequest {
    #[serde(rename = "applyOnTransformedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on_transformed_logs: Option<bool>,
    #[serde(rename = "emitSystemFieldDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_system_field_dimensions: Option<Vec<String>>,
    #[serde(rename = "fieldSelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_selection_criteria: Option<String>,
    #[serde(rename = "filterName")]
    #[serde(default)]
    pub filter_name: String,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    pub filter_pattern: String,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "metricTransformations")]
    #[serde(default)]
    pub metric_transformations: Vec<MetricTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutQueryDefinitionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "logGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_names: Option<Vec<String>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<QueryParameter>>,
    #[serde(rename = "queryDefinitionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_definition_id: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutQueryDefinitionResponse {
    #[serde(rename = "queryDefinitionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_definition_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "expectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "resourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRetentionPolicyRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "retentionInDays")]
    #[serde(default)]
    pub retention_in_days: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSubscriptionFilterRequest {
    #[serde(rename = "applyOnTransformedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on_transformed_logs: Option<bool>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    pub destination_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(rename = "emitSystemFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_system_fields: Option<Vec<String>>,
    #[serde(rename = "fieldSelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_selection_criteria: Option<String>,
    #[serde(rename = "filterName")]
    #[serde(default)]
    pub filter_name: String,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    pub filter_pattern: String,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTransformerRequest {
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    pub log_group_identifier: String,
    #[serde(rename = "transformerConfig")]
    #[serde(default)]
    pub transformer_config: Vec<Processor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLiveTailRequest {
    #[serde(rename = "logEventFilterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_event_filter_pattern: Option<String>,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    pub log_group_identifiers: Vec<String>,
    #[serde(rename = "logStreamNamePrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefixes: Option<Vec<String>>,
    #[serde(rename = "logStreamNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLiveTailResponse {
    #[serde(rename = "responseStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_stream: Option<StartLiveTailResponseStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLiveTailResponseStream {
    #[serde(rename = "SessionStreamingException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_streaming_exception: Option<SessionStreamingException>,
    #[serde(rename = "SessionTimeoutException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_exception: Option<SessionTimeoutException>,
    #[serde(rename = "sessionStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_start: Option<LiveTailSessionStart>,
    #[serde(rename = "sessionUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_update: Option<LiveTailSessionUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionStreamingException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionTimeoutException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LiveTailSessionStart {
    #[serde(rename = "logEventFilterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_event_filter_pattern: Option<String>,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(rename = "logStreamNamePrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefixes: Option<Vec<String>>,
    #[serde(rename = "logStreamNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_names: Option<Vec<String>>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LiveTailSessionUpdate {
    #[serde(rename = "sessionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_metadata: Option<LiveTailSessionMetadata>,
    #[serde(rename = "sessionResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_results: Option<Vec<LiveTailSessionLogEvent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LiveTailSessionMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LiveTailSessionLogEvent {
    #[serde(rename = "ingestionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time: Option<i64>,
    #[serde(rename = "logGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifier: Option<String>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "logGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_names: Option<Vec<String>>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryResponse {
    #[serde(rename = "queryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopQueryRequest {
    #[serde(rename = "queryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopQueryResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagLogGroupRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestMetricFilterRequest {
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    pub filter_pattern: String,
    #[serde(rename = "logEventMessages")]
    #[serde(default)]
    pub log_event_messages: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestMetricFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<MetricFilterMatchRecord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricFilterMatchRecord {
    #[serde(rename = "eventMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_message: Option<String>,
    #[serde(rename = "eventNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_number: Option<i64>,
    #[serde(rename = "extractedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_values: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestTransformerRequest {
    #[serde(rename = "logEventMessages")]
    #[serde(default)]
    pub log_event_messages: Vec<String>,
    #[serde(rename = "transformerConfig")]
    #[serde(default)]
    pub transformer_config: Vec<Processor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestTransformerResponse {
    #[serde(rename = "transformedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformed_logs: Option<Vec<TransformedLogRecord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformedLogRecord {
    #[serde(rename = "eventMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_message: Option<String>,
    #[serde(rename = "eventNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_number: Option<i64>,
    #[serde(rename = "transformedEventMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformed_event_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagLogGroupRequest {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnomalyRequest {
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    pub anomaly_detector_arn: String,
    #[serde(rename = "anomalyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<bool>,
    #[serde(rename = "patternId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_id: Option<String>,
    #[serde(rename = "suppressionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_period: Option<SuppressionPeriod>,
    #[serde(rename = "suppressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionPeriod {
    #[serde(rename = "suppressionUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeliveryConfigurationRequest {
    #[serde(rename = "fieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "recordFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_fields: Option<Vec<String>>,
    #[serde(rename = "s3DeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delivery_configuration: Option<S3DeliveryConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeliveryConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLogAnomalyDetectorRequest {
    #[serde(rename = "anomalyDetectorArn")]
    #[serde(default)]
    pub anomaly_detector_arn: String,
    #[serde(rename = "anomalyVisibilityTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_visibility_time: Option<i64>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "evaluationFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,
    #[serde(rename = "filterPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLookupTableRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "lookupTableArn")]
    #[serde(default)]
    pub lookup_table_arn: String,
    #[serde(rename = "tableBody")]
    #[serde(default)]
    pub table_body: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLookupTableResponse {
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "lookupTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledQueryRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    pub query_language: String,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "scheduleEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_end_time: Option<i64>,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "scheduleStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start_time: Option<i64>,
    #[serde(rename = "startTimeOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_offset: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledQueryResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "lastExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_status: Option<String>,
    #[serde(rename = "lastTriggeredTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered_time: Option<i64>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[serde(rename = "logGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_identifiers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "queryLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "scheduleEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_end_time: Option<i64>,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "scheduleStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start_time: Option<i64>,
    #[serde(rename = "scheduledQueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_query_arn: Option<String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_offset: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}
