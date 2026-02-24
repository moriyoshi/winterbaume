//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-apprunner

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceRequest {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_arn: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "HealthCheckConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_configuration: Option<HealthCheckConfiguration>,
    #[serde(rename = "InstanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_configuration: Option<InstanceConfiguration>,
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "ObservabilityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration: Option<ServiceObservabilityConfiguration>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(rename = "SourceConfiguration")]
    #[serde(default)]
    pub source_configuration: SourceConfiguration,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    pub kms_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheckConfiguration {
    #[serde(rename = "HealthyThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceConfiguration {
    #[serde(rename = "Cpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "InstanceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    #[serde(rename = "Memory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "EgressConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_configuration: Option<EgressConfiguration>,
    #[serde(rename = "IngressConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_configuration: Option<IngressConfiguration>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EgressConfiguration {
    #[serde(rename = "EgressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_type: Option<String>,
    #[serde(rename = "VpcConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connector_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngressConfiguration {
    #[serde(rename = "IsPubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_publicly_accessible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceObservabilityConfiguration {
    #[serde(rename = "ObservabilityConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_arn: Option<String>,
    #[serde(rename = "ObservabilityEnabled")]
    #[serde(default)]
    pub observability_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceConfiguration {
    #[serde(rename = "AuthenticationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<AuthenticationConfiguration>,
    #[serde(rename = "AutoDeploymentsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployments_enabled: Option<bool>,
    #[serde(rename = "CodeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository: Option<CodeRepository>,
    #[serde(rename = "ImageRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_repository: Option<ImageRepository>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationConfiguration {
    #[serde(rename = "AccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_role_arn: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeRepository {
    #[serde(rename = "CodeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_configuration: Option<CodeConfiguration>,
    #[serde(rename = "RepositoryUrl")]
    #[serde(default)]
    pub repository_url: String,
    #[serde(rename = "SourceCodeVersion")]
    #[serde(default)]
    pub source_code_version: SourceCodeVersion,
    #[serde(rename = "SourceDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeConfiguration {
    #[serde(rename = "CodeConfigurationValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_configuration_values: Option<CodeConfigurationValues>,
    #[serde(rename = "ConfigurationSource")]
    #[serde(default)]
    pub configuration_source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeConfigurationValues {
    #[serde(rename = "BuildCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_command: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "Runtime")]
    #[serde(default)]
    pub runtime: String,
    #[serde(rename = "RuntimeEnvironmentSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment_secrets: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RuntimeEnvironmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "StartCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_command: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceCodeVersion {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageRepository {
    #[serde(rename = "ImageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_configuration: Option<ImageConfiguration>,
    #[serde(rename = "ImageIdentifier")]
    #[serde(default)]
    pub image_identifier: String,
    #[serde(rename = "ImageRepositoryType")]
    #[serde(default)]
    pub image_repository_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageConfiguration {
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "RuntimeEnvironmentSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment_secrets: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RuntimeEnvironmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "StartCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_command: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutoScalingConfigurationsRequest {
    #[serde(rename = "AutoScalingConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_name: Option<String>,
    #[serde(rename = "LatestOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_only: Option<bool>,
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
pub struct DeleteVpcConnectorRequest {
    #[serde(rename = "VpcConnectorArn")]
    #[serde(default)]
    pub vpc_connector_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeObservabilityConfigurationResponse {
    #[serde(rename = "ObservabilityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration: Option<ObservabilityConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObservabilityConfiguration {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DeletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "Latest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    #[serde(rename = "ObservabilityConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_arn: Option<String>,
    #[serde(rename = "ObservabilityConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_name: Option<String>,
    #[serde(rename = "ObservabilityConfigurationRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_revision: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TraceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_configuration: Option<TraceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TraceConfiguration {
    #[serde(rename = "Vendor")]
    #[serde(default)]
    pub vendor: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomDomainsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteObservabilityConfigurationResponse {
    #[serde(rename = "ObservabilityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration: Option<ObservabilityConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutoScalingConfigurationResponse {
    #[serde(rename = "AutoScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration: Option<AutoScalingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingConfiguration {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_arn: Option<String>,
    #[serde(rename = "AutoScalingConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_name: Option<String>,
    #[serde(rename = "AutoScalingConfigurationRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_revision: Option<i32>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DeletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "HasAssociatedService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_associated_service: Option<bool>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Latest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i32>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceRequest {
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDefaultAutoScalingConfigurationRequest {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    pub auto_scaling_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectionsResponse {
    #[serde(rename = "ConnectionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_summary_list: Option<Vec<ConnectionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionSummary {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcConnectorRequest {
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    pub subnets: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcConnectorName")]
    #[serde(default)]
    pub vpc_connector_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesForAutoScalingConfigurationResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutoScalingConfigurationRequest {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    pub auto_scaling_configuration_arn: String,
    #[serde(rename = "DeleteAllRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_all_revisions: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeploymentRequest {
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcConnectorRequest {
    #[serde(rename = "VpcConnectorArn")]
    #[serde(default)]
    pub vpc_connector_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcIngressConnectionResponse {
    #[serde(rename = "VpcIngressConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection: Option<VpcIngressConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcIngressConnection {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DeletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "IngressVpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_vpc_configuration: Option<IngressVpcConfiguration>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcIngressConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection_arn: Option<String>,
    #[serde(rename = "VpcIngressConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngressVpcConfiguration {
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutoScalingConfigurationResponse {
    #[serde(rename = "AutoScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration: Option<AutoScalingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcConnectorResponse {
    #[serde(rename = "VpcConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connector: Option<VpcConnector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnector {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DeletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(rename = "VpcConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connector_arn: Option<String>,
    #[serde(rename = "VpcConnectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connector_name: Option<String>,
    #[serde(rename = "VpcConnectorRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connector_revision: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcIngressConnectionResponse {
    #[serde(rename = "VpcIngressConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection: Option<VpcIngressConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObservabilityConfigurationsRequest {
    #[serde(rename = "LatestOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_only: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObservabilityConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObservabilityConfigurationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObservabilityConfigurationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_summary_list: Option<Vec<ObservabilityConfigurationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObservabilityConfigurationSummary {
    #[serde(rename = "ObservabilityConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_arn: Option<String>,
    #[serde(rename = "ObservabilityConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_name: Option<String>,
    #[serde(rename = "ObservabilityConfigurationRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration_revision: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteObservabilityConfigurationRequest {
    #[serde(rename = "ObservabilityConfigurationArn")]
    #[serde(default)]
    pub observability_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcIngressConnectionRequest {
    #[serde(rename = "IngressVpcConfiguration")]
    #[serde(default)]
    pub ingress_vpc_configuration: IngressVpcConfiguration,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcIngressConnectionName")]
    #[serde(default)]
    pub vpc_ingress_connection_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(rename = "AutoScalingConfigurationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_summary: Option<AutoScalingConfigurationSummary>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DeletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "HealthCheckConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_configuration: Option<HealthCheckConfiguration>,
    #[serde(rename = "InstanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_configuration: Option<InstanceConfiguration>,
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "ObservabilityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration: Option<ServiceObservabilityConfiguration>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[serde(rename = "SourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_configuration: Option<SourceConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingConfigurationSummary {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_arn: Option<String>,
    #[serde(rename = "AutoScalingConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_name: Option<String>,
    #[serde(rename = "AutoScalingConfigurationRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_revision: Option<i32>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "HasAssociatedService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_associated_service: Option<bool>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateObservabilityConfigurationResponse {
    #[serde(rename = "ObservabilityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration: Option<ObservabilityConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesForAutoScalingConfigurationRequest {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    pub auto_scaling_configuration_arn: String,
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
pub struct ListVpcIngressConnectionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcIngressConnectionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection_summary_list: Option<Vec<VpcIngressConnectionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcIngressConnectionSummary {
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "VpcIngressConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PauseServiceRequest {
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcConnectorsRequest {
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
pub struct ListServicesRequest {
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
pub struct PauseServiceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeServiceRequest {
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceRequest {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_arn: Option<String>,
    #[serde(rename = "HealthCheckConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_configuration: Option<HealthCheckConfiguration>,
    #[serde(rename = "InstanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_configuration: Option<InstanceConfiguration>,
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "ObservabilityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observability_configuration: Option<ServiceObservabilityConfiguration>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
    #[serde(rename = "SourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_configuration: Option<SourceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOperationsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutoScalingConfigurationRequest {
    #[serde(rename = "AutoScalingConfigurationArn")]
    #[serde(default)]
    pub auto_scaling_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcIngressConnectionResponse {
    #[serde(rename = "VpcIngressConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection: Option<VpcIngressConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcConnectorResponse {
    #[serde(rename = "VpcConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connector: Option<VpcConnector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeploymentResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDefaultAutoScalingConfigurationResponse {
    #[serde(rename = "AutoScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration: Option<AutoScalingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcIngressConnectionRequest {
    #[serde(rename = "IngressVpcConfiguration")]
    #[serde(default)]
    pub ingress_vpc_configuration: IngressVpcConfiguration,
    #[serde(rename = "VpcIngressConnectionArn")]
    #[serde(default)]
    pub vpc_ingress_connection_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcConnectorResponse {
    #[serde(rename = "VpcConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connector: Option<VpcConnector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionRequest {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    pub provider_type: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateCustomDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeServiceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutoScalingConfigurationResponse {
    #[serde(rename = "AutoScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration: Option<AutoScalingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateCustomDomainResponse {
    #[serde(rename = "CustomDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(rename = "DNSTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_target: Option<String>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "VpcDNSTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_d_n_s_targets: Option<Vec<VpcDNSTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDomain {
    #[serde(rename = "CertificateValidationRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_validation_records: Option<Vec<CertificateValidationRecord>>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "EnableWWWSubdomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_w_w_w_subdomain: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateValidationRecord {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcDNSTarget {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcIngressConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutoScalingConfigurationsResponse {
    #[serde(rename = "AutoScalingConfigurationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_configuration_summary_list: Option<Vec<AutoScalingConfigurationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOperationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_summary_list: Option<Vec<OperationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationSummary {
    #[serde(rename = "EndedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "StartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_summary_list: Option<Vec<ServiceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSummary {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomDomainsResponse {
    #[serde(rename = "CustomDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domains: Option<Vec<CustomDomain>>,
    #[serde(rename = "DNSTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_target: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "VpcDNSTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_d_n_s_targets: Option<Vec<VpcDNSTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcIngressConnectionRequest {
    #[serde(rename = "VpcIngressConnectionArn")]
    #[serde(default)]
    pub vpc_ingress_connection_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateObservabilityConfigurationRequest {
    #[serde(rename = "ObservabilityConfigurationName")]
    #[serde(default)]
    pub observability_configuration_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TraceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_configuration: Option<TraceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcIngressConnectionsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListVpcIngressConnectionsFilter>,
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
pub struct ListVpcIngressConnectionsFilter {
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutoScalingConfigurationRequest {
    #[serde(rename = "AutoScalingConfigurationName")]
    #[serde(default)]
    pub auto_scaling_configuration_name: String,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i32>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "MinSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeObservabilityConfigurationRequest {
    #[serde(rename = "ObservabilityConfigurationArn")]
    #[serde(default)]
    pub observability_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcIngressConnectionRequest {
    #[serde(rename = "VpcIngressConnectionArn")]
    #[serde(default)]
    pub vpc_ingress_connection_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcIngressConnectionResponse {
    #[serde(rename = "VpcIngressConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ingress_connection: Option<VpcIngressConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceResponse {
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateCustomDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "EnableWWWSubdomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_w_w_w_subdomain: Option<bool>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateCustomDomainResponse {
    #[serde(rename = "CustomDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(rename = "DNSTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_target: Option<String>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "VpcDNSTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_d_n_s_targets: Option<Vec<VpcDNSTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceRequest {
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    pub service_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcConnectorsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connectors: Option<Vec<VpcConnector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionRequest {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    pub connection_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectionsRequest {
    #[serde(rename = "ConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}
