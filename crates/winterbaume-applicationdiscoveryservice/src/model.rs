//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationdiscoveryservice

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConfigurationItemsToApplicationRequest {
    #[serde(rename = "applicationConfigurationId")]
    #[serde(default)]
    pub application_configuration_id: String,
    #[serde(rename = "configurationIds")]
    #[serde(default)]
    pub configuration_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConfigurationItemsToApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteAgentsRequest {
    #[serde(rename = "deleteAgents")]
    #[serde(default)]
    pub delete_agents: Vec<DeleteAgent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgent {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteAgentsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchDeleteAgentError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteAgentError {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteImportDataRequest {
    #[serde(rename = "deleteHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_history: Option<bool>,
    #[serde(rename = "importTaskIds")]
    #[serde(default)]
    pub import_task_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteImportDataResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchDeleteImportDataError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteImportDataError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "importTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationResponse {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagsRequest {
    #[serde(rename = "configurationIds")]
    #[serde(default)]
    pub configuration_ids: Vec<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationsRequest {
    #[serde(rename = "configurationIds")]
    #[serde(default)]
    pub configuration_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsRequest {
    #[serde(rename = "configurationIds")]
    #[serde(default)]
    pub configuration_ids: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgentsRequest {
    #[serde(rename = "agentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct Filter {
    #[serde(default)]
    pub condition: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgentsResponse {
    #[serde(rename = "agentsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_info: Option<Vec<AgentInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentInfo {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentNetworkInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_info_list: Option<Vec<AgentNetworkInfo>>,
    #[serde(rename = "agentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    #[serde(rename = "collectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_status: Option<String>,
    #[serde(rename = "connectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(rename = "hostName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "lastHealthPingTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_health_ping_time: Option<String>,
    #[serde(rename = "registeredTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentNetworkInfo {
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchDeleteConfigurationTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchDeleteConfigurationTaskResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<BatchDeleteConfigurationTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteConfigurationTask {
    #[serde(rename = "configurationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    #[serde(rename = "deletedConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_configurations: Option<Vec<String>>,
    #[serde(rename = "deletionWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_warnings: Option<Vec<DeletionWarning>>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "failedConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_configurations: Option<Vec<FailedConfiguration>>,
    #[serde(rename = "requestedConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_configurations: Option<Vec<String>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletionWarning {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    #[serde(rename = "warningCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_code: Option<i32>,
    #[serde(rename = "warningText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedConfiguration {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "errorStatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationsRequest {
    #[serde(rename = "configurationIds")]
    #[serde(default)]
    pub configuration_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<std::collections::HashMap<String, String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContinuousExportsRequest {
    #[serde(rename = "exportIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_ids: Option<Vec<String>>,
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
pub struct DescribeContinuousExportsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<ContinuousExportDescription>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinuousExportDescription {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "schemaStorageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_storage_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    #[serde(rename = "stopTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportConfigurationsRequest {
    #[serde(rename = "exportIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_ids: Option<Vec<String>>,
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
pub struct DescribeExportConfigurationsResponse {
    #[serde(rename = "exportsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports_info: Option<Vec<ExportInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportInfo {
    #[serde(rename = "configurationsDownloadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations_download_url: Option<String>,
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "exportRequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_request_time: Option<f64>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "isTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "requestedEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_end_time: Option<f64>,
    #[serde(rename = "requestedStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_start_time: Option<f64>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportTasksRequest {
    #[serde(rename = "exportIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExportFilter>>,
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
pub struct ExportFilter {
    #[serde(default)]
    pub condition: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportTasksResponse {
    #[serde(rename = "exportsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports_info: Option<Vec<ExportInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportTasksRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ImportTaskFilter>>,
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
pub struct ImportTaskFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportTasksResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<ImportTask>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTask {
    #[serde(rename = "applicationImportFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_import_failure: Option<i32>,
    #[serde(rename = "applicationImportSuccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_import_success: Option<i32>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "errorsAndFailedEntriesZip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors_and_failed_entries_zip: Option<String>,
    #[serde(rename = "fileClassification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_classification: Option<String>,
    #[serde(rename = "importCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_completion_time: Option<f64>,
    #[serde(rename = "importDeletedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_deleted_time: Option<f64>,
    #[serde(rename = "importRequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_request_time: Option<f64>,
    #[serde(rename = "importTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_task_id: Option<String>,
    #[serde(rename = "importUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serverImportFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_import_failure: Option<i32>,
    #[serde(rename = "serverImportSuccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_import_success: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<TagFilter>>,
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
pub struct TagFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ConfigurationTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationTag {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    #[serde(rename = "configurationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "timeOfCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_creation: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConfigurationItemsFromApplicationRequest {
    #[serde(rename = "applicationConfigurationId")]
    #[serde(default)]
    pub application_configuration_id: String,
    #[serde(rename = "configurationIds")]
    #[serde(default)]
    pub configuration_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConfigurationItemsFromApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportConfigurationsResponse {
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDiscoverySummaryRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDiscoverySummaryResponse {
    #[serde(rename = "agentSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_summary: Option<CustomerAgentInfo>,
    #[serde(rename = "agentlessCollectorSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agentless_collector_summary: Option<CustomerAgentlessCollectorInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<i64>,
    #[serde(rename = "connectorSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_summary: Option<CustomerConnectorInfo>,
    #[serde(rename = "meCollectorSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub me_collector_summary: Option<CustomerMeCollectorInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<i64>,
    #[serde(rename = "serversMappedToApplications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers_mapped_to_applications: Option<i64>,
    #[serde(rename = "serversMappedtoTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers_mappedto_tags: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerAgentInfo {
    #[serde(rename = "activeAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_agents: Option<i32>,
    #[serde(rename = "blackListedAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_listed_agents: Option<i32>,
    #[serde(rename = "healthyAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_agents: Option<i32>,
    #[serde(rename = "shutdownAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_agents: Option<i32>,
    #[serde(rename = "totalAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_agents: Option<i32>,
    #[serde(rename = "unhealthyAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_agents: Option<i32>,
    #[serde(rename = "unknownAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_agents: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerAgentlessCollectorInfo {
    #[serde(rename = "activeAgentlessCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_agentless_collectors: Option<i32>,
    #[serde(rename = "denyListedAgentlessCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_listed_agentless_collectors: Option<i32>,
    #[serde(rename = "healthyAgentlessCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_agentless_collectors: Option<i32>,
    #[serde(rename = "shutdownAgentlessCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_agentless_collectors: Option<i32>,
    #[serde(rename = "totalAgentlessCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_agentless_collectors: Option<i32>,
    #[serde(rename = "unhealthyAgentlessCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_agentless_collectors: Option<i32>,
    #[serde(rename = "unknownAgentlessCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_agentless_collectors: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerConnectorInfo {
    #[serde(rename = "activeConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_connectors: Option<i32>,
    #[serde(rename = "blackListedConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_listed_connectors: Option<i32>,
    #[serde(rename = "healthyConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_connectors: Option<i32>,
    #[serde(rename = "shutdownConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_connectors: Option<i32>,
    #[serde(rename = "totalConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_connectors: Option<i32>,
    #[serde(rename = "unhealthyConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_connectors: Option<i32>,
    #[serde(rename = "unknownConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_connectors: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerMeCollectorInfo {
    #[serde(rename = "activeMeCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_me_collectors: Option<i32>,
    #[serde(rename = "denyListedMeCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_listed_me_collectors: Option<i32>,
    #[serde(rename = "healthyMeCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_me_collectors: Option<i32>,
    #[serde(rename = "shutdownMeCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_me_collectors: Option<i32>,
    #[serde(rename = "totalMeCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_me_collectors: Option<i32>,
    #[serde(rename = "unhealthyMeCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_me_collectors: Option<i32>,
    #[serde(rename = "unknownMeCollectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_me_collectors: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationsRequest {
    #[serde(rename = "configurationType")]
    #[serde(default)]
    pub configuration_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<OrderByElement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderByElement {
    #[serde(rename = "fieldName")]
    #[serde(default)]
    pub field_name: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServerNeighborsRequest {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    pub configuration_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "neighborConfigurationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbor_configuration_ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "portInformationNeeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_information_needed: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServerNeighborsResponse {
    #[serde(rename = "knownDependencyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_dependency_count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbors: Option<Vec<NeighborConnectionDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NeighborConnectionDetail {
    #[serde(rename = "connectionsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections_count: Option<i64>,
    #[serde(rename = "destinationPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[serde(rename = "destinationServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_server_id: Option<String>,
    #[serde(rename = "sourceServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_server_id: Option<String>,
    #[serde(rename = "transportProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBatchDeleteConfigurationTaskRequest {
    #[serde(rename = "configurationIds")]
    #[serde(default)]
    pub configuration_ids: Vec<String>,
    #[serde(rename = "configurationType")]
    #[serde(default)]
    pub configuration_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBatchDeleteConfigurationTaskResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContinuousExportRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContinuousExportResponse {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "schemaStorageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_storage_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataCollectionByAgentIdsRequest {
    #[serde(rename = "agentIds")]
    #[serde(default)]
    pub agent_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataCollectionByAgentIdsResponse {
    #[serde(rename = "agentsConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_configuration_status: Option<Vec<AgentConfigurationStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentConfigurationStatus {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "operationSucceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_succeeded: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExportTaskRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "exportDataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_data_format: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExportFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<ExportPreferences>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportPreferences {
    #[serde(rename = "ec2RecommendationsPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_recommendations_preferences: Option<Ec2RecommendationsExportPreferences>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2RecommendationsExportPreferences {
    #[serde(rename = "cpuPerformanceMetricBasis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_performance_metric_basis: Option<UsageMetricBasis>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "excludedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_instance_types: Option<Vec<String>>,
    #[serde(rename = "preferredRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_region: Option<String>,
    #[serde(rename = "ramPerformanceMetricBasis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_performance_metric_basis: Option<UsageMetricBasis>,
    #[serde(rename = "reservedInstanceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_options: Option<ReservedInstanceOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageMetricBasis {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "percentageAdjust")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_adjust: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedInstanceOptions {
    #[serde(rename = "offeringClass")]
    #[serde(default)]
    pub offering_class: String,
    #[serde(rename = "purchasingOption")]
    #[serde(default)]
    pub purchasing_option: String,
    #[serde(rename = "termLength")]
    #[serde(default)]
    pub term_length: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExportTaskResponse {
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportTaskRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "importUrl")]
    #[serde(default)]
    pub import_url: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportTaskResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<ImportTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContinuousExportRequest {
    #[serde(rename = "exportId")]
    #[serde(default)]
    pub export_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContinuousExportResponse {
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "stopTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDataCollectionByAgentIdsRequest {
    #[serde(rename = "agentIds")]
    #[serde(default)]
    pub agent_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDataCollectionByAgentIdsResponse {
    #[serde(rename = "agentsConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_configuration_status: Option<Vec<AgentConfigurationStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationRequest {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    pub configuration_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wave: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationResponse {}
