//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-emrcontainers

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobRunRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobRunResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobTemplateRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "jobTemplateData")]
    #[serde(default)]
    pub job_template_data: JobTemplateData,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobTemplateData {
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ParametricConfigurationOverrides>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "jobDriver")]
    #[serde(default)]
    pub job_driver: JobDriver,
    #[serde(rename = "jobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "parameterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_configuration:
        Option<std::collections::HashMap<String, TemplateParameterConfiguration>>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    pub release_label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParametricConfigurationOverrides {
    #[serde(rename = "applicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration: Option<Vec<Configuration>>,
    #[serde(rename = "monitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<ParametricMonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(default)]
    pub classification: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParametricMonitoringConfiguration {
    #[serde(rename = "cloudWatchMonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_monitoring_configuration: Option<ParametricCloudWatchMonitoringConfiguration>,
    #[serde(rename = "persistentAppUI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i: Option<String>,
    #[serde(rename = "s3MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_monitoring_configuration: Option<ParametricS3MonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParametricCloudWatchMonitoringConfiguration {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParametricS3MonitoringConfiguration {
    #[serde(rename = "logUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDriver {
    #[serde(rename = "sparkSqlJobDriver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_sql_job_driver: Option<SparkSqlJobDriver>,
    #[serde(rename = "sparkSubmitJobDriver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_submit_job_driver: Option<SparkSubmitJobDriver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparkSqlJobDriver {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,
    #[serde(rename = "sparkSqlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_sql_parameters: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparkSubmitJobDriver {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: String,
    #[serde(rename = "entryPointArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point_arguments: Option<Vec<String>>,
    #[serde(rename = "sparkSubmitParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_submit_parameters: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateParameterConfiguration {
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateManagedEndpointRequest {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    pub release_label: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationOverrides {
    #[serde(rename = "applicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration: Option<Vec<Configuration>>,
    #[serde(rename = "monitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfiguration {
    #[serde(rename = "cloudWatchMonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_monitoring_configuration: Option<CloudWatchMonitoringConfiguration>,
    #[serde(rename = "containerLogRotationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_log_rotation_configuration: Option<ContainerLogRotationConfiguration>,
    #[serde(rename = "managedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_logs: Option<ManagedLogs>,
    #[serde(rename = "persistentAppUI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_u_i: Option<String>,
    #[serde(rename = "s3MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_monitoring_configuration: Option<S3MonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchMonitoringConfiguration {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerLogRotationConfiguration {
    #[serde(rename = "maxFilesToKeep")]
    #[serde(default)]
    pub max_files_to_keep: i32,
    #[serde(rename = "rotationSize")]
    #[serde(default)]
    pub rotation_size: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedLogs {
    #[serde(rename = "allowAWSToRetainLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_a_w_s_to_retain_logs: Option<String>,
    #[serde(rename = "encryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3MonitoringConfiguration {
    #[serde(rename = "logUri")]
    #[serde(default)]
    pub log_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateManagedEndpointResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "containerProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_provider: Option<ContainerProvider>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "securityConfigurationData")]
    #[serde(default)]
    pub security_configuration_data: SecurityConfigurationData,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerProvider {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<ContainerInfo>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerInfo {
    #[serde(rename = "eksInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_info: Option<EksInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nodeLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityConfigurationData {
    #[serde(rename = "authorizationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_configuration: Option<AuthorizationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationConfiguration {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "lakeFormationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_formation_configuration: Option<LakeFormationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "inTransitEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_encryption_configuration: Option<InTransitEncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InTransitEncryptionConfiguration {
    #[serde(rename = "tlsCertificateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificate_configuration: Option<TLSCertificateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TLSCertificateConfiguration {
    #[serde(rename = "certificateProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_type: Option<String>,
    #[serde(rename = "privateCertificateSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_certificate_secret_arn: Option<String>,
    #[serde(rename = "publicCertificateSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_certificate_secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LakeFormationConfiguration {
    #[serde(rename = "authorizedSessionTagValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_session_tag_value: Option<String>,
    #[serde(rename = "queryEngineRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_engine_role_arn: Option<String>,
    #[serde(rename = "secureNamespaceInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_namespace_info: Option<SecureNamespaceInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecureNamespaceInfo {
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualClusterRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "containerProvider")]
    #[serde(default)]
    pub container_provider: ContainerProvider,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "securityConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVirtualClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobTemplateRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteManagedEndpointRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteManagedEndpointResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualClusterRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobRunRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobRunResponse {
    #[serde(rename = "jobRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run: Option<JobRun>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobRun {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "finishedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "jobDriver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_driver: Option<JobDriver>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "retryPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy_configuration: Option<RetryPolicyConfiguration>,
    #[serde(rename = "retryPolicyExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy_execution: Option<RetryPolicyExecution>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryPolicyConfiguration {
    #[serde(rename = "maxAttempts")]
    #[serde(default)]
    pub max_attempts: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryPolicyExecution {
    #[serde(rename = "currentAttemptCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_attempt_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobTemplateRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobTemplateResponse {
    #[serde(rename = "jobTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobTemplate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "decryptionError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption_error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "jobTemplateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_data: Option<JobTemplateData>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedEndpointRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedEndpointResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateAuthority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<Certificate>,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "securityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    #[serde(rename = "serverUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityConfigurationRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityConfigurationResponse {
    #[serde(rename = "securityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<SecurityConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityConfigurationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration_data: Option<SecurityConfigurationData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualClusterRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualClusterResponse {
    #[serde(rename = "virtualCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster: Option<VirtualCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualCluster {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "containerProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_provider: Option<ContainerProvider>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedEndpointSessionCredentialsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "credentialType")]
    #[serde(default)]
    pub credential_type: String,
    #[serde(rename = "durationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
    #[serde(rename = "endpointIdentifier")]
    #[serde(default)]
    pub endpoint_identifier: String,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "logContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_context: Option<String>,
    #[serde(rename = "virtualClusterIdentifier")]
    #[serde(default)]
    pub virtual_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedEndpointSessionCredentialsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Credentials {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobRunsRequest {
    #[serde(rename = "createdAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobRunsResponse {
    #[serde(rename = "jobRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobTemplatesRequest {
    #[serde(rename = "createdAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
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
pub struct ListJobTemplatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<JobTemplate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedEndpointsRequest {
    #[serde(rename = "createdAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
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
    pub states: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedEndpointsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityConfigurationsRequest {
    #[serde(rename = "createdAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
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
pub struct ListSecurityConfigurationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configurations: Option<Vec<SecurityConfiguration>>,
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
pub struct ListVirtualClustersRequest {
    #[serde(rename = "containerProviderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_provider_id: Option<String>,
    #[serde(rename = "containerProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_provider_type: Option<String>,
    #[serde(rename = "createdAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(rename = "eksAccessEntryIntegrated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_access_entry_integrated: Option<bool>,
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
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualClustersResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_clusters: Option<Vec<VirtualCluster>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "configurationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "jobDriver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_driver: Option<JobDriver>,
    #[serde(rename = "jobTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_id: Option<String>,
    #[serde(rename = "jobTemplateParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "releaseLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    #[serde(rename = "retryPolicyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy_configuration: Option<RetryPolicyConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    pub virtual_cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartJobRunResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "virtualClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
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
