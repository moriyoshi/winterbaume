//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amp

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAlertManagerDefinitionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub data: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAlertManagerDefinitionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertManagerDefinitionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlertManagerDefinitionStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnomalyDetectorRequest {
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub configuration: AnomalyDetectorConfiguration,
    #[serde(rename = "evaluationIntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_interval_in_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "missingDataAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_data_action: Option<AnomalyDetectorMissingDataAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetectorConfiguration {
    #[serde(rename = "randomCutForest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_cut_forest: Option<RandomCutForestConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RandomCutForestConfiguration {
    #[serde(rename = "ignoreNearExpectedFromAbove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_near_expected_from_above: Option<IgnoreNearExpected>,
    #[serde(rename = "ignoreNearExpectedFromBelow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_near_expected_from_below: Option<IgnoreNearExpected>,
    #[serde(default)]
    pub query: String,
    #[serde(rename = "sampleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i32>,
    #[serde(rename = "shingleSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shingle_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IgnoreNearExpected {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetectorMissingDataAction {
    #[serde(rename = "markAsAnomaly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_anomaly: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnomalyDetectorResponse {
    #[serde(rename = "anomalyDetectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AnomalyDetectorStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetectorStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoggingConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    pub log_group_arn: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoggingConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LoggingConfigurationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfigurationStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQueryLoggingConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub destinations: Vec<LoggingDestination>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingDestination {
    #[serde(rename = "cloudWatchLogs")]
    #[serde(default)]
    pub cloud_watch_logs: CloudWatchLogDestination,
    #[serde(default)]
    pub filters: LoggingFilter,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogDestination {
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    pub log_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingFilter {
    #[serde(rename = "qspThreshold")]
    #[serde(default)]
    pub qsp_threshold: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQueryLoggingConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryLoggingConfigurationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryLoggingConfigurationStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleGroupsNamespaceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleGroupsNamespaceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleGroupsNamespaceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupsNamespaceStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScraperRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "roleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_configuration: Option<RoleConfiguration>,
    #[serde(rename = "scrapeConfiguration")]
    #[serde(default)]
    pub scrape_configuration: ScrapeConfiguration,
    #[serde(default)]
    pub source: Source,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "ampConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amp_configuration: Option<AmpConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmpConfiguration {
    #[serde(rename = "workspaceArn")]
    #[serde(default)]
    pub workspace_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoleConfiguration {
    #[serde(rename = "sourceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_role_arn: Option<String>,
    #[serde(rename = "targetRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScrapeConfiguration {
    #[serde(rename = "configurationBlob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_blob: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "eksConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_configuration: Option<EksConfiguration>,
    #[serde(rename = "vpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksConfiguration {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    pub cluster_arn: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfiguration {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScraperResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScraperStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScraperStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkspaceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAlertManagerDefinitionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnomalyDetectorRequest {
    #[serde(rename = "anomalyDetectorId")]
    #[serde(default)]
    pub anomaly_detector_id: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoggingConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueryLoggingConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleGroupsNamespaceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScraperLoggingConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    pub scraper_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScraperRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    pub scraper_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScraperResponse {
    #[serde(rename = "scraperId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScraperStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlertManagerDefinitionRequest {
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlertManagerDefinitionResponse {
    #[serde(rename = "alertManagerDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_manager_definition: Option<AlertManagerDefinitionDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlertManagerDefinitionDescription {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertManagerDefinitionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnomalyDetectorRequest {
    #[serde(rename = "anomalyDetectorId")]
    #[serde(default)]
    pub anomaly_detector_id: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnomalyDetectorResponse {
    #[serde(rename = "anomalyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector: Option<AnomalyDetectorDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetectorDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "anomalyDetectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AnomalyDetectorConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "evaluationIntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_interval_in_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "missingDataAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_data_action: Option<AnomalyDetectorMissingDataAction>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AnomalyDetectorStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLoggingConfigurationRequest {
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLoggingConfigurationResponse {
    #[serde(rename = "loggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfigurationMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfigurationMetadata {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LoggingConfigurationStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueryLoggingConfigurationRequest {
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueryLoggingConfigurationResponse {
    #[serde(rename = "queryLoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_logging_configuration: Option<QueryLoggingConfigurationMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryLoggingConfigurationMetadata {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<LoggingDestination>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryLoggingConfigurationStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyRequest {
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyResponse {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupsNamespaceRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleGroupsNamespaceResponse {
    #[serde(rename = "ruleGroupsNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups_namespace: Option<RuleGroupsNamespaceDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupsNamespaceDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleGroupsNamespaceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScraperLoggingConfigurationRequest {
    #[serde(rename = "scraperId")]
    #[serde(default)]
    pub scraper_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScraperLoggingConfigurationResponse {
    #[serde(rename = "loggingDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_destination: Option<ScraperLoggingDestination>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "scraperComponents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_components: Option<Vec<ScraperComponent>>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScraperLoggingConfigurationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScraperLoggingDestination {
    #[serde(rename = "cloudWatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<CloudWatchLogDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScraperComponent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ComponentConfig>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScraperLoggingConfigurationStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScraperRequest {
    #[serde(rename = "scraperId")]
    #[serde(default)]
    pub scraper_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScraperResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper: Option<ScraperDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScraperDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "roleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_configuration: Option<RoleConfiguration>,
    #[serde(rename = "scrapeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_configuration: Option<ScrapeConfiguration>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScraperStatus>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceConfigurationRequest {
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceConfigurationResponse {
    #[serde(rename = "workspaceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_configuration: Option<WorkspaceConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceConfigurationDescription {
    #[serde(rename = "limitsPerLabelSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_per_label_set: Option<Vec<LimitsPerLabelSet>>,
    #[serde(rename = "retentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkspaceConfigurationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LimitsPerLabelSet {
    #[serde(rename = "labelSet")]
    #[serde(default)]
    pub label_set: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub limits: LimitsPerLabelSetEntry,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LimitsPerLabelSetEntry {
    #[serde(rename = "maxSeries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_series: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceConfigurationStatus {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceRequest {
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "prometheusEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkspaceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDefaultScraperConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDefaultScraperConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnomalyDetectorsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnomalyDetectorsResponse {
    #[serde(rename = "anomalyDetectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detectors: Option<Vec<AnomalyDetectorSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetectorSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "anomalyDetectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AnomalyDetectorStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleGroupsNamespacesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleGroupsNamespacesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ruleGroupsNamespaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups_namespaces: Option<Vec<RuleGroupsNamespaceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupsNamespaceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleGroupsNamespaceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScrapersRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
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
pub struct ListScrapersResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrapers: Option<Vec<ScraperSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScraperSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "roleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_configuration: Option<RoleConfiguration>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScraperStatus>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
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
pub struct ListWorkspacesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
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
pub struct ListWorkspacesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<WorkspaceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkspaceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAlertManagerDefinitionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub data: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAlertManagerDefinitionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AlertManagerDefinitionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAnomalyDetectorRequest {
    #[serde(rename = "anomalyDetectorId")]
    #[serde(default)]
    pub anomaly_detector_id: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub configuration: AnomalyDetectorConfiguration,
    #[serde(rename = "evaluationIntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_interval_in_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "missingDataAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_data_action: Option<AnomalyDetectorMissingDataAction>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAnomalyDetectorResponse {
    #[serde(rename = "anomalyDetectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AnomalyDetectorStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "policyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRuleGroupsNamespaceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRuleGroupsNamespaceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleGroupsNamespaceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
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
pub struct TagResourceResponse {}

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
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLoggingConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    pub log_group_arn: String,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLoggingConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LoggingConfigurationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueryLoggingConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub destinations: Vec<LoggingDestination>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueryLoggingConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryLoggingConfigurationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScraperLoggingConfigurationRequest {
    #[serde(rename = "loggingDestination")]
    #[serde(default)]
    pub logging_destination: ScraperLoggingDestination,
    #[serde(rename = "scraperComponents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_components: Option<Vec<ScraperComponent>>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    pub scraper_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScraperLoggingConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScraperLoggingConfigurationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScraperRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "roleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_configuration: Option<RoleConfiguration>,
    #[serde(rename = "scrapeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_configuration: Option<ScrapeConfiguration>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    pub scraper_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScraperResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "scraperId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraper_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScraperStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceAliasRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "limitsPerLabelSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_per_label_set: Option<Vec<LimitsPerLabelSet>>,
    #[serde(rename = "retentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<i32>,
    #[serde(rename = "workspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkspaceConfigurationStatus>,
}
