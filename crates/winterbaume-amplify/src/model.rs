//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amplify

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppRequest {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    #[serde(rename = "buildSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    #[serde(rename = "cacheConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_config: Option<CacheConfig>,
    #[serde(rename = "computeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_role_arn: Option<String>,
    #[serde(rename = "customHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<String>,
    #[serde(rename = "customRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    #[serde(rename = "enableBasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "enableBranchAutoBuild")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_build: Option<bool>,
    #[serde(rename = "enableBranchAutoDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_deletion: Option<bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    #[serde(rename = "jobConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_config: Option<JobConfig>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "oauthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoBranchCreationConfig {
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    #[serde(rename = "buildSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    #[serde(rename = "enableAutoBuild")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    #[serde(rename = "enableBasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "enablePerformanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    #[serde(rename = "enablePullRequestPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheConfig {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobConfig {
    #[serde(rename = "buildComputeType")]
    #[serde(default)]
    pub build_compute_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct App {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    #[serde(rename = "buildSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    #[serde(rename = "cacheConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_config: Option<CacheConfig>,
    #[serde(rename = "computeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_role_arn: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "customHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<String>,
    #[serde(rename = "customRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    #[serde(rename = "defaultDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_domain: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    #[serde(rename = "enableBasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "enableBranchAutoBuild")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_build: Option<bool>,
    #[serde(rename = "enableBranchAutoDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_deletion: Option<bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    #[serde(rename = "jobConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_config: Option<JobConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "productionBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_branch: Option<ProductionBranch>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "repositoryCloneMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_clone_method: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[serde(rename = "wafConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_configuration: Option<WafConfiguration>,
    #[serde(rename = "webhookCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_create_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductionBranch {
    #[serde(rename = "branchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "lastDeployTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deploy_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WafConfiguration {
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "wafStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_status: Option<String>,
    #[serde(rename = "webAclArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendEnvironmentRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "deploymentArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_artifacts: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    pub environment_name: String,
    #[serde(rename = "stackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendEnvironmentResult {
    #[serde(rename = "backendEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment: Option<BackendEnvironment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendEnvironment {
    #[serde(rename = "backendEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_arn: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "deploymentArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_artifacts: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "stackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBranchRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<Backend>,
    #[serde(rename = "backendEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_arn: Option<String>,
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "buildSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    #[serde(rename = "computeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "enableAutoBuild")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    #[serde(rename = "enableBasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "enableNotification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_notification: Option<bool>,
    #[serde(rename = "enablePerformanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    #[serde(rename = "enablePullRequestPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,
    #[serde(rename = "enableSkewProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_skew_protection: Option<bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Backend {
    #[serde(rename = "stackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBranchResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<Branch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Branch {
    #[serde(rename = "activeJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_job_id: Option<String>,
    #[serde(rename = "associatedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resources: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<Backend>,
    #[serde(rename = "backendEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_arn: Option<String>,
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    #[serde(rename = "branchArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_arn: Option<String>,
    #[serde(rename = "branchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "buildSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    #[serde(rename = "computeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_role_arn: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "customDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domains: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_branch: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "enableAutoBuild")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    #[serde(rename = "enableBasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "enableNotification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_notification: Option<bool>,
    #[serde(rename = "enablePerformanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    #[serde(rename = "enablePullRequestPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,
    #[serde(rename = "enableSkewProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_skew_protection: Option<bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    #[serde(rename = "sourceBranch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "totalNumberOfJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_jobs: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "fileMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentResult {
    #[serde(rename = "fileUploadUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_upload_urls: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "zipUploadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_upload_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainAssociationRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "autoSubDomainCreationPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,
    #[serde(rename = "autoSubDomainIAMRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_i_a_m_role: Option<String>,
    #[serde(rename = "certificateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_settings: Option<CertificateSettings>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "enableAutoSubDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sub_domain: Option<bool>,
    #[serde(rename = "subDomainSettings")]
    #[serde(default)]
    pub sub_domain_settings: Vec<SubDomainSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateSettings {
    #[serde(rename = "customCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_certificate_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubDomainSetting {
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainAssociationResult {
    #[serde(rename = "domainAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_association: Option<DomainAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainAssociation {
    #[serde(rename = "autoSubDomainCreationPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,
    #[serde(rename = "autoSubDomainIAMRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_i_a_m_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
    #[serde(rename = "certificateVerificationDNSRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_verification_d_n_s_record: Option<String>,
    #[serde(rename = "domainAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_association_arn: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<String>,
    #[serde(rename = "enableAutoSubDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sub_domain: Option<bool>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "subDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_domains: Option<Vec<SubDomain>>,
    #[serde(rename = "updateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(rename = "certificateVerificationDNSRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_verification_d_n_s_record: Option<String>,
    #[serde(rename = "customCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_certificate_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubDomain {
    #[serde(rename = "dnsRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_record: Option<String>,
    #[serde(rename = "subDomainSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_domain_setting: Option<SubDomainSetting>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebhookRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWebhookResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Webhook {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "branchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[serde(rename = "webhookArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_arn: Option<String>,
    #[serde(rename = "webhookId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    #[serde(rename = "webhookUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendEnvironmentRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    pub environment_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendEnvironmentResult {
    #[serde(rename = "backendEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment: Option<BackendEnvironment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBranchRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBranchResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<Branch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainAssociationRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainAssociationResult {
    #[serde(rename = "domainAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_association: Option<DomainAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobResult {
    #[serde(rename = "jobSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summary: Option<JobSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobSummary {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "commitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_time: Option<f64>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "sourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "sourceUrlType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url_type: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebhookRequest {
    #[serde(rename = "webhookId")]
    #[serde(default)]
    pub webhook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebhookResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateAccessLogsRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateAccessLogsResult {
    #[serde(rename = "logUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetArtifactUrlRequest {
    #[serde(rename = "artifactId")]
    #[serde(default)]
    pub artifact_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetArtifactUrlResult {
    #[serde(rename = "artifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[serde(rename = "artifactUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendEnvironmentRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    pub environment_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendEnvironmentResult {
    #[serde(rename = "backendEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment: Option<BackendEnvironment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBranchRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBranchResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<Branch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainAssociationRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainAssociationResult {
    #[serde(rename = "domainAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_association: Option<DomainAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Job {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<JobSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Step {
    #[serde(rename = "artifactsUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "logUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stepName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
    #[serde(rename = "testArtifactsUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_artifacts_url: Option<String>,
    #[serde(rename = "testConfigUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_config_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWebhookRequest {
    #[serde(rename = "webhookId")]
    #[serde(default)]
    pub webhook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWebhookResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppsRequest {
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
pub struct ListAppsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<App>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListArtifactsRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
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
pub struct ListArtifactsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Artifact>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Artifact {
    #[serde(rename = "artifactFileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_file_name: Option<String>,
    #[serde(rename = "artifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackendEnvironmentsRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
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
pub struct ListBackendEnvironmentsResult {
    #[serde(rename = "backendEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environments: Option<Vec<BackendEnvironment>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBranchesRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
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
pub struct ListBranchesResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<Branch>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainAssociationsRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
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
pub struct ListDomainAssociationsResult {
    #[serde(rename = "domainAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_associations: Option<Vec<DomainAssociation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
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
pub struct ListJobsResult {
    #[serde(rename = "jobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summaries: Option<Vec<JobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
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
pub struct ListWebhooksRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
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
pub struct ListWebhooksResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<Webhook>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeploymentRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "sourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "sourceUrlType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeploymentResult {
    #[serde(rename = "jobSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summary: Option<JobSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "commitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "commitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_time: Option<f64>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_reason: Option<String>,
    #[serde(rename = "jobType")]
    #[serde(default)]
    pub job_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobResult {
    #[serde(rename = "jobSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summary: Option<JobSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopJobRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopJobResult {
    #[serde(rename = "jobSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summary: Option<JobSummary>,
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
pub struct UpdateAppRequest {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "autoBranchCreationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_config: Option<AutoBranchCreationConfig>,
    #[serde(rename = "autoBranchCreationPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_branch_creation_patterns: Option<Vec<String>>,
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    #[serde(rename = "buildSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    #[serde(rename = "cacheConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_config: Option<CacheConfig>,
    #[serde(rename = "computeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_role_arn: Option<String>,
    #[serde(rename = "customHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<String>,
    #[serde(rename = "customRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<CustomRule>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enableAutoBranchCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_branch_creation: Option<bool>,
    #[serde(rename = "enableBasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "enableBranchAutoBuild")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_build: Option<bool>,
    #[serde(rename = "enableBranchAutoDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_branch_auto_deletion: Option<bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "iamServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_service_role_arn: Option<String>,
    #[serde(rename = "jobConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_config: Option<JobConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "oauthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBranchRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<Backend>,
    #[serde(rename = "backendEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_arn: Option<String>,
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<String>,
    #[serde(rename = "branchName")]
    #[serde(default)]
    pub branch_name: String,
    #[serde(rename = "buildSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_spec: Option<String>,
    #[serde(rename = "computeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "enableAutoBuild")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_build: Option<bool>,
    #[serde(rename = "enableBasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(rename = "enableNotification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_notification: Option<bool>,
    #[serde(rename = "enablePerformanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_mode: Option<bool>,
    #[serde(rename = "enablePullRequestPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_pull_request_preview: Option<bool>,
    #[serde(rename = "enableSkewProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_skew_protection: Option<bool>,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(rename = "pullRequestEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBranchResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<Branch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainAssociationRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "autoSubDomainCreationPatterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_creation_patterns: Option<Vec<String>>,
    #[serde(rename = "autoSubDomainIAMRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sub_domain_i_a_m_role: Option<String>,
    #[serde(rename = "certificateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_settings: Option<CertificateSettings>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "enableAutoSubDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sub_domain: Option<bool>,
    #[serde(rename = "subDomainSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_domain_settings: Option<Vec<SubDomainSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainAssociationResult {
    #[serde(rename = "domainAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_association: Option<DomainAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebhookRequest {
    #[serde(rename = "branchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "webhookId")]
    #[serde(default)]
    pub webhook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWebhookResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Webhook>,
}
