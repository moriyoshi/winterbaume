//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-fis

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateRequest {
    #[serde(default)]
    pub actions: std::collections::HashMap<String, CreateExperimentTemplateActionInput>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "experimentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_options: Option<CreateExperimentTemplateExperimentOptionsInput>,
    #[serde(rename = "experimentReportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_report_configuration: Option<CreateExperimentTemplateReportConfigurationInput>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<CreateExperimentTemplateLogConfigurationInput>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "stopConditions")]
    #[serde(default)]
    pub stop_conditions: Vec<CreateExperimentTemplateStopConditionInput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, CreateExperimentTemplateTargetInput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateActionInput {
    #[serde(rename = "actionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "startAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateExperimentOptionsInput {
    #[serde(rename = "accountTargeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_targeting: Option<String>,
    #[serde(rename = "emptyTargetResolutionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_target_resolution_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateReportConfigurationInput {
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<ExperimentTemplateReportConfigurationDataSourcesInput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<ExperimentTemplateReportConfigurationOutputsInput>,
    #[serde(rename = "postExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_experiment_duration: Option<String>,
    #[serde(rename = "preExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_experiment_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateReportConfigurationDataSourcesInput {
    #[serde(rename = "cloudWatchDashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_dashboards: Option<Vec<ReportConfigurationCloudWatchDashboardInput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportConfigurationCloudWatchDashboardInput {
    #[serde(rename = "dashboardIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateReportConfigurationOutputsInput {
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<ReportConfigurationS3OutputInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportConfigurationS3OutputInput {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateLogConfigurationInput {
    #[serde(rename = "cloudWatchLogsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration:
        Option<ExperimentTemplateCloudWatchLogsLogConfigurationInput>,
    #[serde(rename = "logSchemaVersion")]
    #[serde(default)]
    pub log_schema_version: i32,
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<ExperimentTemplateS3LogConfigurationInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateCloudWatchLogsLogConfigurationInput {
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    pub log_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateS3LogConfigurationInput {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateStopConditionInput {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateTargetInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExperimentTemplateTargetInputFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "selectionMode")]
    #[serde(default)]
    pub selection_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateTargetInputFilter {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExperimentTemplateResponse {
    #[serde(rename = "experimentTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template: Option<ExperimentTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<std::collections::HashMap<String, ExperimentTemplateAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "experimentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_options: Option<ExperimentTemplateExperimentOptions>,
    #[serde(rename = "experimentReportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_report_configuration: Option<ExperimentTemplateReportConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<ExperimentTemplateLogConfiguration>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "stopConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_conditions: Option<Vec<ExperimentTemplateStopCondition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetAccountConfigurationsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configurations_count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, ExperimentTemplateTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateAction {
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "startAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateExperimentOptions {
    #[serde(rename = "accountTargeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_targeting: Option<String>,
    #[serde(rename = "emptyTargetResolutionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_target_resolution_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateReportConfiguration {
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<ExperimentTemplateReportConfigurationDataSources>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<ExperimentTemplateReportConfigurationOutputs>,
    #[serde(rename = "postExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_experiment_duration: Option<String>,
    #[serde(rename = "preExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_experiment_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateReportConfigurationDataSources {
    #[serde(rename = "cloudWatchDashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_dashboards:
        Option<Vec<ExperimentTemplateReportConfigurationCloudWatchDashboard>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateReportConfigurationCloudWatchDashboard {
    #[serde(rename = "dashboardIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateReportConfigurationOutputs {
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<ReportConfigurationS3Output>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportConfigurationS3Output {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateLogConfiguration {
    #[serde(rename = "cloudWatchLogsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration: Option<ExperimentTemplateCloudWatchLogsLogConfiguration>,
    #[serde(rename = "logSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_schema_version: Option<i32>,
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<ExperimentTemplateS3LogConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateCloudWatchLogsLogConfiguration {
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateS3LogConfiguration {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateStopCondition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExperimentTemplateTargetFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "selectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateTargetFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTargetAccountConfigurationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    pub experiment_template_id: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTargetAccountConfigurationResponse {
    #[serde(rename = "targetAccountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configuration: Option<TargetAccountConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetAccountConfiguration {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExperimentTemplateRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExperimentTemplateResponse {
    #[serde(rename = "experimentTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template: Option<ExperimentTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTargetAccountConfigurationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    pub experiment_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTargetAccountConfigurationResponse {
    #[serde(rename = "targetAccountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configuration: Option<TargetAccountConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActionRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, ActionParameter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, ActionTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionParameter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTarget {
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExperimentRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExperimentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment: Option<Experiment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Experiment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<std::collections::HashMap<String, ExperimentAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "experimentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_options: Option<ExperimentOptions>,
    #[serde(rename = "experimentReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_report: Option<ExperimentReport>,
    #[serde(rename = "experimentReportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_report_configuration: Option<ExperimentReportConfiguration>,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<ExperimentLogConfiguration>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ExperimentState>,
    #[serde(rename = "stopConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_conditions: Option<Vec<ExperimentStopCondition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetAccountConfigurationsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configurations_count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, ExperimentTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentAction {
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "startAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<Vec<String>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ExperimentActionState>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentActionState {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentOptions {
    #[serde(rename = "accountTargeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_targeting: Option<String>,
    #[serde(rename = "actionsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_mode: Option<String>,
    #[serde(rename = "emptyTargetResolutionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_target_resolution_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReport {
    #[serde(rename = "s3Reports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reports: Option<Vec<ExperimentReportS3Report>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ExperimentReportState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportS3Report {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "reportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportState {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ExperimentReportError>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportConfiguration {
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<ExperimentReportConfigurationDataSources>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<ExperimentReportConfigurationOutputs>,
    #[serde(rename = "postExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_experiment_duration: Option<String>,
    #[serde(rename = "preExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_experiment_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportConfigurationDataSources {
    #[serde(rename = "cloudWatchDashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_dashboards: Option<Vec<ExperimentReportConfigurationCloudWatchDashboard>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportConfigurationCloudWatchDashboard {
    #[serde(rename = "dashboardIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportConfigurationOutputs {
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<ExperimentReportConfigurationOutputsS3Configuration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentReportConfigurationOutputsS3Configuration {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentLogConfiguration {
    #[serde(rename = "cloudWatchLogsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration: Option<ExperimentCloudWatchLogsLogConfiguration>,
    #[serde(rename = "logSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_schema_version: Option<i32>,
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<ExperimentS3LogConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentCloudWatchLogsLogConfiguration {
    #[serde(rename = "logGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentS3LogConfiguration {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentState {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ExperimentError>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentError {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentStopCondition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExperimentTargetFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "selectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTargetFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExperimentTargetAccountConfigurationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "experimentId")]
    #[serde(default)]
    pub experiment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExperimentTargetAccountConfigurationResponse {
    #[serde(rename = "targetAccountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configuration: Option<ExperimentTargetAccountConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTargetAccountConfiguration {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExperimentTemplateRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExperimentTemplateResponse {
    #[serde(rename = "experimentTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template: Option<ExperimentTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSafetyLeverRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSafetyLeverResponse {
    #[serde(rename = "safetyLever")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_lever: Option<SafetyLever>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SafetyLever {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SafetyLeverState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SafetyLeverState {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetAccountConfigurationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    pub experiment_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetAccountConfigurationResponse {
    #[serde(rename = "targetAccountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configuration: Option<TargetAccountConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetResourceTypeRequest {
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTargetResourceTypeResponse {
    #[serde(rename = "targetResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<TargetResourceType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetResourceType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, TargetResourceTypeParameter>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetResourceTypeParameter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActionsRequest {
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
pub struct ListActionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ActionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, ActionTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExperimentResolvedTargetsRequest {
    #[serde(rename = "experimentId")]
    #[serde(default)]
    pub experiment_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExperimentResolvedTargetsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resolvedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_targets: Option<Vec<ResolvedTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolvedTarget {
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "targetInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_information: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExperimentTargetAccountConfigurationsRequest {
    #[serde(rename = "experimentId")]
    #[serde(default)]
    pub experiment_id: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExperimentTargetAccountConfigurationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetAccountConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configurations: Option<Vec<ExperimentTargetAccountConfigurationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTargetAccountConfigurationSummary {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExperimentTemplatesRequest {
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
pub struct ListExperimentTemplatesResponse {
    #[serde(rename = "experimentTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_templates: Option<Vec<ExperimentTemplateSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentTemplateSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExperimentsRequest {
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template_id: Option<String>,
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
pub struct ListExperimentsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiments: Option<Vec<ExperimentSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExperimentSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "experimentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_options: Option<ExperimentOptions>,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ExperimentState>,
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
pub struct ListTargetAccountConfigurationsRequest {
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    pub experiment_template_id: String,
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
pub struct ListTargetAccountConfigurationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetAccountConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configurations: Option<Vec<TargetAccountConfigurationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetAccountConfigurationSummary {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetResourceTypesRequest {
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
pub struct ListTargetResourceTypesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_types: Option<Vec<TargetResourceTypeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetResourceTypeSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExperimentRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "experimentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_options: Option<StartExperimentExperimentOptionsInput>,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    pub experiment_template_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExperimentExperimentOptionsInput {
    #[serde(rename = "actionsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExperimentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment: Option<Experiment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopExperimentRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopExperimentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment: Option<Experiment>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<std::collections::HashMap<String, UpdateExperimentTemplateActionInputItem>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "experimentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_options: Option<UpdateExperimentTemplateExperimentOptionsInput>,
    #[serde(rename = "experimentReportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_report_configuration: Option<UpdateExperimentTemplateReportConfigurationInput>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<UpdateExperimentTemplateLogConfigurationInput>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "stopConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_conditions: Option<Vec<UpdateExperimentTemplateStopConditionInput>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, UpdateExperimentTemplateTargetInput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateActionInputItem {
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "startAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateExperimentOptionsInput {
    #[serde(rename = "emptyTargetResolutionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_target_resolution_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateReportConfigurationInput {
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<ExperimentTemplateReportConfigurationDataSourcesInput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<ExperimentTemplateReportConfigurationOutputsInput>,
    #[serde(rename = "postExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_experiment_duration: Option<String>,
    #[serde(rename = "preExperimentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_experiment_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateLogConfigurationInput {
    #[serde(rename = "cloudWatchLogsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration:
        Option<ExperimentTemplateCloudWatchLogsLogConfigurationInput>,
    #[serde(rename = "logSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_schema_version: Option<i32>,
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<ExperimentTemplateS3LogConfigurationInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateStopConditionInput {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateTargetInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExperimentTemplateTargetInputFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "selectionMode")]
    #[serde(default)]
    pub selection_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExperimentTemplateResponse {
    #[serde(rename = "experimentTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template: Option<ExperimentTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSafetyLeverStateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub state: UpdateSafetyLeverStateInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSafetyLeverStateInput {
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSafetyLeverStateResponse {
    #[serde(rename = "safetyLever")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_lever: Option<SafetyLever>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTargetAccountConfigurationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    pub experiment_template_id: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTargetAccountConfigurationResponse {
    #[serde(rename = "targetAccountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_configuration: Option<TargetAccountConfiguration>,
}
