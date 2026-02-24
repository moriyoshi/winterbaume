//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codepipeline

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcknowledgeJobInput {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub nonce: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcknowledgeJobOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcknowledgeThirdPartyJobInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub nonce: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcknowledgeThirdPartyJobOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomActionTypeInput {
    #[serde(default)]
    pub category: String,
    #[serde(rename = "configurationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<Vec<ActionConfigurationProperty>>,
    #[serde(rename = "inputArtifactDetails")]
    #[serde(default)]
    pub input_artifact_details: ArtifactDetails,
    #[serde(rename = "outputArtifactDetails")]
    #[serde(default)]
    pub output_artifact_details: ArtifactDetails,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ActionTypeSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionConfigurationProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub key: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queryable: Option<bool>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub secret: bool,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArtifactDetails {
    #[serde(rename = "maximumCount")]
    #[serde(default)]
    pub maximum_count: i32,
    #[serde(rename = "minimumCount")]
    #[serde(default)]
    pub minimum_count: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeSettings {
    #[serde(rename = "entityUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url_template: Option<String>,
    #[serde(rename = "executionUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_url_template: Option<String>,
    #[serde(rename = "revisionUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url_template: Option<String>,
    #[serde(rename = "thirdPartyConfigurationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_configuration_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomActionTypeOutput {
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ActionType>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionType {
    #[serde(rename = "actionConfigurationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration_properties: Option<Vec<ActionConfigurationProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ActionTypeId>,
    #[serde(rename = "inputArtifactDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifact_details: Option<ArtifactDetails>,
    #[serde(rename = "outputArtifactDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifact_details: Option<ArtifactDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ActionTypeSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeId {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipelineInput {
    #[serde(default)]
    pub pipeline: PipelineDeclaration,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineDeclaration {
    #[serde(rename = "artifactStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_store: Option<ArtifactStore>,
    #[serde(rename = "artifactStores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_stores: Option<std::collections::HashMap<String, ArtifactStore>>,
    #[serde(rename = "executionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "pipelineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_type: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    pub stages: Vec<StageDeclaration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<PipelineTriggerDeclaration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<PipelineVariableDeclaration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArtifactStore {
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    #[serde(default)]
    pub location: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionKey {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageDeclaration {
    #[serde(default)]
    pub actions: Vec<ActionDeclaration>,
    #[serde(rename = "beforeEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_entry: Option<BeforeEntryConditions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockers: Option<Vec<BlockerDeclaration>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "onFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<FailureConditions>,
    #[serde(rename = "onSuccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success: Option<SuccessConditions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionDeclaration {
    #[serde(rename = "actionTypeId")]
    #[serde(default)]
    pub action_type_id: ActionTypeId,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    #[serde(rename = "inputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<InputArtifact>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "outputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<OutputArtifact>>,
    #[serde(rename = "outputVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_variables: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "runOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_order: Option<i32>,
    #[serde(rename = "timeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentVariable {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputArtifact {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputArtifact {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BeforeEntryConditions {
    #[serde(default)]
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RuleDeclaration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleDeclaration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "inputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<InputArtifact>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "ruleTypeId")]
    #[serde(default)]
    pub rule_type_id: RuleTypeId,
    #[serde(rename = "timeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleTypeId {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockerDeclaration {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureConditions {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "retryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_configuration: Option<RetryConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryConfiguration {
    #[serde(rename = "retryMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessConditions {
    #[serde(default)]
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineTriggerDeclaration {
    #[serde(rename = "gitConfiguration")]
    #[serde(default)]
    pub git_configuration: GitConfiguration,
    #[serde(rename = "providerType")]
    #[serde(default)]
    pub provider_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitConfiguration {
    #[serde(rename = "pullRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<Vec<GitPullRequestFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<Vec<GitPushFilter>>,
    #[serde(rename = "sourceActionName")]
    #[serde(default)]
    pub source_action_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitPullRequestFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<GitBranchFilterCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "filePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<GitFilePathFilterCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitBranchFilterCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitFilePathFilterCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitPushFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<GitBranchFilterCriteria>,
    #[serde(rename = "filePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<GitFilePathFilterCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<GitTagFilterCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitTagFilterCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineVariableDeclaration {
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
pub struct CreatePipelineOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomActionTypeInput {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipelineInput {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebhookInput {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebhookOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterWebhookWithThirdPartyInput {
    #[serde(rename = "webhookName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterWebhookWithThirdPartyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableStageTransitionInput {
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
    #[serde(rename = "transitionType")]
    #[serde(default)]
    pub transition_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableStageTransitionInput {
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
    #[serde(rename = "transitionType")]
    #[serde(default)]
    pub transition_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActionTypeInput {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActionTypeOutput {
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ActionTypeDeclaration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeDeclaration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub executor: ActionTypeExecutor,
    #[serde(default)]
    pub id: ActionTypeIdentifier,
    #[serde(rename = "inputArtifactDetails")]
    #[serde(default)]
    pub input_artifact_details: ActionTypeArtifactDetails,
    #[serde(rename = "outputArtifactDetails")]
    #[serde(default)]
    pub output_artifact_details: ActionTypeArtifactDetails,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ActionTypePermissions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ActionTypeProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<ActionTypeUrls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeExecutor {
    #[serde(default)]
    pub configuration: ExecutorConfiguration,
    #[serde(rename = "jobTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_timeout: Option<i32>,
    #[serde(rename = "policyStatementsTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_statements_template: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutorConfiguration {
    #[serde(rename = "jobWorkerExecutorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_worker_executor_configuration: Option<JobWorkerExecutorConfiguration>,
    #[serde(rename = "lambdaExecutorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_executor_configuration: Option<LambdaExecutorConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobWorkerExecutorConfiguration {
    #[serde(rename = "pollingAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polling_accounts: Option<Vec<String>>,
    #[serde(rename = "pollingServicePrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polling_service_principals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaExecutorConfiguration {
    #[serde(rename = "lambdaFunctionArn")]
    #[serde(default)]
    pub lambda_function_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeIdentifier {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeArtifactDetails {
    #[serde(rename = "maximumCount")]
    #[serde(default)]
    pub maximum_count: i32,
    #[serde(rename = "minimumCount")]
    #[serde(default)]
    pub minimum_count: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypePermissions {
    #[serde(rename = "allowedAccounts")]
    #[serde(default)]
    pub allowed_accounts: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub key: bool,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "noEcho")]
    #[serde(default)]
    pub no_echo: bool,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queryable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTypeUrls {
    #[serde(rename = "configurationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,
    #[serde(rename = "entityUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url_template: Option<String>,
    #[serde(rename = "executionUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_url_template: Option<String>,
    #[serde(rename = "revisionUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url_template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobDetailsInput {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobDetailsOutput {
    #[serde(rename = "jobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDetails {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobData {
    #[serde(rename = "actionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<ActionConfiguration>,
    #[serde(rename = "actionTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    #[serde(rename = "artifactCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_credentials: Option<AWSSessionCredentials>,
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    #[serde(rename = "inputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<Artifact>>,
    #[serde(rename = "outputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<Artifact>>,
    #[serde(rename = "pipelineContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_context: Option<PipelineContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSSessionCredentials {
    #[serde(rename = "accessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "secretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "sessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Artifact {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ArtifactLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArtifactLocation {
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3ArtifactLocation>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ArtifactLocation {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "objectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionContext>,
    #[serde(rename = "pipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<StageContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionContext {
    #[serde(rename = "actionExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineExecutionInput {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    pub pipeline_execution_id: String,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineExecutionOutput {
    #[serde(rename = "pipelineExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution: Option<PipelineExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineExecution {
    #[serde(rename = "artifactRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_revisions: Option<Vec<ArtifactRevision>>,
    #[serde(rename = "executionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<String>,
    #[serde(rename = "executionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "pipelineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i32>,
    #[serde(rename = "rollbackMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_metadata: Option<PipelineRollbackMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<ExecutionTrigger>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<ResolvedPipelineVariable>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArtifactRevision {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "revisionChangeIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_change_identifier: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "revisionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
    #[serde(rename = "revisionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineRollbackMetadata {
    #[serde(rename = "rollbackTargetPipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_target_pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionTrigger {
    #[serde(rename = "triggerDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_detail: Option<String>,
    #[serde(rename = "triggerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolvedPipelineVariable {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resolvedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineInput {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PipelineMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "pipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "pollingDisabledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polling_disabled_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineStateInput {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineStateOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "pipelineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i32>,
    #[serde(rename = "stageStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_states: Option<Vec<StageState>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageState {
    #[serde(rename = "actionStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_states: Option<Vec<ActionState>>,
    #[serde(rename = "beforeEntryConditionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_entry_condition_state: Option<StageConditionState>,
    #[serde(rename = "inboundExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_execution: Option<StageExecution>,
    #[serde(rename = "inboundExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_executions: Option<Vec<StageExecution>>,
    #[serde(rename = "inboundTransitionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transition_state: Option<TransitionState>,
    #[serde(rename = "latestExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<StageExecution>,
    #[serde(rename = "onFailureConditionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure_condition_state: Option<StageConditionState>,
    #[serde(rename = "onSuccessConditionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success_condition_state: Option<StageConditionState>,
    #[serde(rename = "retryStageMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_stage_metadata: Option<RetryStageMetadata>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionState {
    #[serde(rename = "actionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "currentRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<ActionRevision>,
    #[serde(rename = "entityUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url: Option<String>,
    #[serde(rename = "latestExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<ActionExecution>,
    #[serde(rename = "revisionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionRevision {
    #[serde(default)]
    pub created: f64,
    #[serde(rename = "revisionChangeId")]
    #[serde(default)]
    pub revision_change_id: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    pub revision_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionExecution {
    #[serde(rename = "actionExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_id: Option<String>,
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "externalExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    #[serde(rename = "externalExecutionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_url: Option<String>,
    #[serde(rename = "lastStatusChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<f64>,
    #[serde(rename = "lastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "logStreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_a_r_n: Option<String>,
    #[serde(rename = "percentComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageConditionState {
    #[serde(rename = "conditionStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_states: Option<Vec<ConditionState>>,
    #[serde(rename = "latestExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<StageConditionsExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionState {
    #[serde(rename = "latestExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<ConditionExecution>,
    #[serde(rename = "ruleStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_states: Option<Vec<RuleState>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionExecution {
    #[serde(rename = "lastStatusChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleState {
    #[serde(rename = "currentRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<RuleRevision>,
    #[serde(rename = "entityUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url: Option<String>,
    #[serde(rename = "latestExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<RuleExecution>,
    #[serde(rename = "revisionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleRevision {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "revisionChangeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_change_id: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleExecution {
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "externalExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    #[serde(rename = "externalExecutionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_url: Option<String>,
    #[serde(rename = "lastStatusChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<f64>,
    #[serde(rename = "lastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "ruleExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_execution_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageConditionsExecution {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageExecution {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitionState {
    #[serde(rename = "disabledReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastChangedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_at: Option<f64>,
    #[serde(rename = "lastChangedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryStageMetadata {
    #[serde(rename = "autoStageRetryAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_stage_retry_attempt: Option<i32>,
    #[serde(rename = "latestRetryTrigger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_retry_trigger: Option<String>,
    #[serde(rename = "manualStageRetryAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_stage_retry_attempt: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThirdPartyJobDetailsInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThirdPartyJobDetailsOutput {
    #[serde(rename = "jobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<ThirdPartyJobDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThirdPartyJobDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ThirdPartyJobData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThirdPartyJobData {
    #[serde(rename = "actionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configuration: Option<ActionConfiguration>,
    #[serde(rename = "actionTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    #[serde(rename = "artifactCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_credentials: Option<AWSSessionCredentials>,
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    #[serde(rename = "inputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<Artifact>>,
    #[serde(rename = "outputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<Artifact>>,
    #[serde(rename = "pipelineContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_context: Option<PipelineContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActionExecutionsInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ActionExecutionFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionExecutionFilter {
    #[serde(rename = "latestInPipelineExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_in_pipeline_execution: Option<LatestInPipelineExecutionFilter>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LatestInPipelineExecutionFilter {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    pub pipeline_execution_id: String,
    #[serde(rename = "startTimeRange")]
    #[serde(default)]
    pub start_time_range: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActionExecutionsOutput {
    #[serde(rename = "actionExecutionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_details: Option<Vec<ActionExecutionDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionExecutionDetail {
    #[serde(rename = "actionExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_execution_id: Option<String>,
    #[serde(rename = "actionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ActionExecutionInput>,
    #[serde(rename = "lastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ActionExecutionOutput>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    #[serde(rename = "pipelineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i32>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionExecutionInput {
    #[serde(rename = "actionTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type_id: Option<ActionTypeId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "inputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<ArtifactDetail>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resolvedConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArtifactDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3location: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionExecutionOutput {
    #[serde(rename = "executionResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_result: Option<ActionExecutionResult>,
    #[serde(rename = "outputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<ArtifactDetail>>,
    #[serde(rename = "outputVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionExecutionResult {
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "externalExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    #[serde(rename = "externalExecutionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_summary: Option<String>,
    #[serde(rename = "externalExecutionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_url: Option<String>,
    #[serde(rename = "logStreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActionTypesInput {
    #[serde(rename = "actionOwnerFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_owner_filter: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "regionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActionTypesOutput {
    #[serde(rename = "actionTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_types: Option<Vec<ActionType>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeployActionExecutionTargetsInput {
    #[serde(rename = "actionExecutionId")]
    #[serde(default)]
    pub action_execution_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<TargetFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeployActionExecutionTargetsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<DeployActionExecutionTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployActionExecutionTarget {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DeployTargetEvent>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "targetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployTargetEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<DeployTargetEventContext>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployTargetEventContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ssmCommandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_command_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineExecutionsInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<PipelineExecutionFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineExecutionFilter {
    #[serde(rename = "succeededInStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_in_stage: Option<SucceededInStageFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SucceededInStageFilter {
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineExecutionsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pipelineExecutionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_summaries: Option<Vec<PipelineExecutionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineExecutionSummary {
    #[serde(rename = "executionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<String>,
    #[serde(rename = "executionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
    #[serde(rename = "lastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    #[serde(rename = "rollbackMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_metadata: Option<PipelineRollbackMetadata>,
    #[serde(rename = "sourceRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_revisions: Option<Vec<SourceRevision>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<String>,
    #[serde(rename = "stopTrigger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_trigger: Option<StopExecutionTrigger>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<ExecutionTrigger>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceRevision {
    #[serde(rename = "actionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "revisionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
    #[serde(rename = "revisionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopExecutionTrigger {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelinesInput {
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
pub struct ListPipelinesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<Vec<PipelineSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "executionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleExecutionsInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RuleExecutionFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleExecutionFilter {
    #[serde(rename = "latestInPipelineExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_in_pipeline_execution: Option<LatestInPipelineExecutionFilter>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleExecutionsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ruleExecutionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_execution_details: Option<Vec<RuleExecutionDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleExecutionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<RuleExecutionInput>,
    #[serde(rename = "lastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<RuleExecutionOutput>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
    #[serde(rename = "pipelineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_version: Option<i32>,
    #[serde(rename = "ruleExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_execution_id: Option<String>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleExecutionInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "inputArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<ArtifactDetail>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resolvedConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "ruleTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type_id: Option<RuleTypeId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleExecutionOutput {
    #[serde(rename = "executionResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_result: Option<RuleExecutionResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleExecutionResult {
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "externalExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    #[serde(rename = "externalExecutionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_summary: Option<String>,
    #[serde(rename = "externalExecutionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleTypesInput {
    #[serde(rename = "regionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_filter: Option<String>,
    #[serde(rename = "ruleOwnerFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_owner_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleTypesOutput {
    #[serde(rename = "ruleTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_types: Option<Vec<RuleType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RuleTypeId>,
    #[serde(rename = "inputArtifactDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifact_details: Option<ArtifactDetails>,
    #[serde(rename = "ruleConfigurationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_configuration_properties: Option<Vec<RuleConfigurationProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<RuleTypeSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleConfigurationProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queryable: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleTypeSettings {
    #[serde(rename = "entityUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url_template: Option<String>,
    #[serde(rename = "executionUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_url_template: Option<String>,
    #[serde(rename = "revisionUrlTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_url_template: Option<String>,
    #[serde(rename = "thirdPartyConfigurationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_configuration_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWebhooksInput {
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
pub struct ListWebhooksOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<ListWebhookItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWebhookItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<WebhookDefinition>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "lastTriggered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebhookDefinition {
    #[serde(default)]
    pub authentication: String,
    #[serde(rename = "authenticationConfiguration")]
    #[serde(default)]
    pub authentication_configuration: WebhookAuthConfiguration,
    #[serde(default)]
    pub filters: Vec<WebhookFilterRule>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "targetAction")]
    #[serde(default)]
    pub target_action: String,
    #[serde(rename = "targetPipeline")]
    #[serde(default)]
    pub target_pipeline: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebhookAuthConfiguration {
    #[serde(rename = "AllowedIPRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_i_p_range: Option<String>,
    #[serde(rename = "SecretToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebhookFilterRule {
    #[serde(rename = "jsonPath")]
    #[serde(default)]
    pub json_path: String,
    #[serde(rename = "matchEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverrideStageConditionInput {
    #[serde(rename = "conditionType")]
    #[serde(default)]
    pub condition_type: String,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    pub pipeline_execution_id: String,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForJobsInput {
    #[serde(rename = "actionTypeId")]
    #[serde(default)]
    pub action_type_id: ActionTypeId,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(rename = "queryParam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_param: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForJobsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Job {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForThirdPartyJobsInput {
    #[serde(rename = "actionTypeId")]
    #[serde(default)]
    pub action_type_id: ActionTypeId,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForThirdPartyJobsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<ThirdPartyJob>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThirdPartyJob {
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutActionRevisionInput {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
    #[serde(rename = "actionRevision")]
    #[serde(default)]
    pub action_revision: ActionRevision,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutActionRevisionOutput {
    #[serde(rename = "newRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_revision: Option<bool>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApprovalResultInput {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(default)]
    pub result: ApprovalResult,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
    #[serde(default)]
    pub token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalResult {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutApprovalResultOutput {
    #[serde(rename = "approvedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutJobFailureResultInput {
    #[serde(rename = "failureDetails")]
    #[serde(default)]
    pub failure_details: FailureDetails,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureDetails {
    #[serde(rename = "externalExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    #[serde(default)]
    pub message: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutJobSuccessResultInput {
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "currentRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<CurrentRevision>,
    #[serde(rename = "executionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ExecutionDetails>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "outputVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrentRevision {
    #[serde(rename = "changeIdentifier")]
    #[serde(default)]
    pub change_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(default)]
    pub revision: String,
    #[serde(rename = "revisionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_summary: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionDetails {
    #[serde(rename = "externalExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_execution_id: Option<String>,
    #[serde(rename = "percentComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutThirdPartyJobFailureResultInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "failureDetails")]
    #[serde(default)]
    pub failure_details: FailureDetails,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutThirdPartyJobSuccessResultInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "continuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "currentRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<CurrentRevision>,
    #[serde(rename = "executionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ExecutionDetails>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutWebhookInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub webhook: WebhookDefinition,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutWebhookOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<ListWebhookItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterWebhookWithThirdPartyInput {
    #[serde(rename = "webhookName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterWebhookWithThirdPartyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryStageExecutionInput {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    pub pipeline_execution_id: String,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(rename = "retryMode")]
    #[serde(default)]
    pub retry_mode: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryStageExecutionOutput {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackStageInput {
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
    #[serde(rename = "targetPipelineExecutionId")]
    #[serde(default)]
    pub target_pipeline_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackStageOutput {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPipelineExecutionInput {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sourceRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_revisions: Option<Vec<SourceRevisionOverride>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<PipelineVariable>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceRevisionOverride {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
    #[serde(rename = "revisionType")]
    #[serde(default)]
    pub revision_type: String,
    #[serde(rename = "revisionValue")]
    #[serde(default)]
    pub revision_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineVariable {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPipelineExecutionOutput {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPipelineExecutionInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandon: Option<bool>,
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    pub pipeline_execution_id: String,
    #[serde(rename = "pipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPipelineExecutionOutput {
    #[serde(rename = "pipelineExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionTypeInput {
    #[serde(rename = "actionType")]
    #[serde(default)]
    pub action_type: ActionTypeDeclaration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipelineInput {
    #[serde(default)]
    pub pipeline: PipelineDeclaration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipelineOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PipelineDeclaration>,
}
