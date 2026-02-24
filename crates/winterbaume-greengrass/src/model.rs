//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-greengrass

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateRoleToGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateRoleToGroupResponse {
    #[serde(rename = "AssociatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateServiceRoleToAccountRequest {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateServiceRoleToAccountResponse {
    #[serde(rename = "AssociatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorDefinitionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<ConnectorDefinitionVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorDefinitionVersion {
    #[serde(rename = "Connectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<Connector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connector {
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    pub connector_arn: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorDefinitionVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "ConnectorDefinitionId")]
    #[serde(default)]
    pub connector_definition_id: String,
    #[serde(rename = "Connectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<Connector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreDefinitionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<CoreDefinitionVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreDefinitionVersion {
    #[serde(rename = "Cores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Core {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "SyncShadow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    #[serde(rename = "ThingArn")]
    #[serde(default)]
    pub thing_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreDefinitionVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "CoreDefinitionId")]
    #[serde(default)]
    pub core_definition_id: String,
    #[serde(rename = "Cores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<Core>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    pub deployment_type: String,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "GroupVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentResponse {
    #[serde(rename = "DeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeviceDefinitionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<DeviceDefinitionVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceDefinitionVersion {
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "SyncShadow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,
    #[serde(rename = "ThingArn")]
    #[serde(default)]
    pub thing_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeviceDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeviceDefinitionVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "DeviceDefinitionId")]
    #[serde(default)]
    pub device_definition_id: String,
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeviceDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionDefinitionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<FunctionDefinitionVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionDefinitionVersion {
    #[serde(rename = "DefaultConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_config: Option<FunctionDefaultConfig>,
    #[serde(rename = "Functions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionDefaultConfig {
    #[serde(rename = "Execution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<FunctionDefaultExecutionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionDefaultExecutionConfig {
    #[serde(rename = "IsolationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation_mode: Option<String>,
    #[serde(rename = "RunAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as: Option<FunctionRunAsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionRunAsConfig {
    #[serde(rename = "Gid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Function {
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "FunctionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionConfiguration {
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<FunctionConfigurationEnvironment>,
    #[serde(rename = "ExecArgs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_args: Option<String>,
    #[serde(rename = "Executable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
    #[serde(rename = "FunctionRuntimeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_runtime_override: Option<String>,
    #[serde(rename = "MemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,
    #[serde(rename = "Pinned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionConfigurationEnvironment {
    #[serde(rename = "AccessSysfs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_sysfs: Option<bool>,
    #[serde(rename = "Execution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<FunctionExecutionConfig>,
    #[serde(rename = "ResourceAccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access_policies: Option<Vec<ResourceAccessPolicy>>,
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionExecutionConfig {
    #[serde(rename = "IsolationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation_mode: Option<String>,
    #[serde(rename = "RunAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as: Option<FunctionRunAsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceAccessPolicy {
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionDefinitionVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "DefaultConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_config: Option<FunctionDefaultConfig>,
    #[serde(rename = "FunctionDefinitionId")]
    #[serde(default)]
    pub function_definition_id: String,
    #[serde(rename = "Functions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupCertificateAuthorityRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupCertificateAuthorityResponse {
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<GroupVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupVersion {
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_definition_version_arn: Option<String>,
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    #[serde(rename = "ResourceDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition_version_arn: Option<String>,
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_definition_version_arn: Option<String>,
    #[serde(rename = "CoreDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_definition_version_arn: Option<String>,
    #[serde(rename = "DeviceDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_definition_version_arn: Option<String>,
    #[serde(rename = "FunctionDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_definition_version_arn: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "LoggerDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger_definition_version_arn: Option<String>,
    #[serde(rename = "ResourceDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition_version_arn: Option<String>,
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_definition_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoggerDefinitionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<LoggerDefinitionVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggerDefinitionVersion {
    #[serde(rename = "Loggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Logger {
    #[serde(rename = "Component")]
    #[serde(default)]
    pub component: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Level")]
    #[serde(default)]
    pub level: String,
    #[serde(rename = "Space")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoggerDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoggerDefinitionVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "LoggerDefinitionId")]
    #[serde(default)]
    pub logger_definition_id: String,
    #[serde(rename = "Loggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loggers: Option<Vec<Logger>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoggerDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceDefinitionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<ResourceDefinitionVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDefinitionVersion {
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ResourceDataContainer")]
    #[serde(default)]
    pub resource_data_container: ResourceDataContainer,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataContainer {
    #[serde(rename = "LocalDeviceResourceData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_device_resource_data: Option<LocalDeviceResourceData>,
    #[serde(rename = "LocalVolumeResourceData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_volume_resource_data: Option<LocalVolumeResourceData>,
    #[serde(rename = "S3MachineLearningModelResourceData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_machine_learning_model_resource_data: Option<S3MachineLearningModelResourceData>,
    #[serde(rename = "SageMakerMachineLearningModelResourceData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_machine_learning_model_resource_data:
        Option<SageMakerMachineLearningModelResourceData>,
    #[serde(rename = "SecretsManagerSecretResourceData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_resource_data: Option<SecretsManagerSecretResourceData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalDeviceResourceData {
    #[serde(rename = "GroupOwnerSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner_setting: Option<GroupOwnerSetting>,
    #[serde(rename = "SourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupOwnerSetting {
    #[serde(rename = "AutoAddGroupOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_group_owner: Option<bool>,
    #[serde(rename = "GroupOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalVolumeResourceData {
    #[serde(rename = "DestinationPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    #[serde(rename = "GroupOwnerSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner_setting: Option<GroupOwnerSetting>,
    #[serde(rename = "SourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3MachineLearningModelResourceData {
    #[serde(rename = "DestinationPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    #[serde(rename = "OwnerSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDownloadOwnerSetting {
    #[serde(rename = "GroupOwner")]
    #[serde(default)]
    pub group_owner: String,
    #[serde(rename = "GroupPermission")]
    #[serde(default)]
    pub group_permission: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SageMakerMachineLearningModelResourceData {
    #[serde(rename = "DestinationPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_path: Option<String>,
    #[serde(rename = "OwnerSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,
    #[serde(rename = "SageMakerJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecretsManagerSecretResourceData {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AdditionalStagingLabelsToDownload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_staging_labels_to_download: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceDefinitionVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "ResourceDefinitionId")]
    #[serde(default)]
    pub resource_definition_id: String,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSoftwareUpdateJobRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "S3UrlSignerRole")]
    #[serde(default)]
    pub s3_url_signer_role: String,
    #[serde(rename = "SoftwareToUpdate")]
    #[serde(default)]
    pub software_to_update: String,
    #[serde(rename = "UpdateAgentLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_agent_log_level: Option<String>,
    #[serde(rename = "UpdateTargets")]
    #[serde(default)]
    pub update_targets: Vec<String>,
    #[serde(rename = "UpdateTargetsArchitecture")]
    #[serde(default)]
    pub update_targets_architecture: String,
    #[serde(rename = "UpdateTargetsOperatingSystem")]
    #[serde(default)]
    pub update_targets_operating_system: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSoftwareUpdateJobResponse {
    #[serde(rename = "IotJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_job_arn: Option<String>,
    #[serde(rename = "IotJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_job_id: Option<String>,
    #[serde(rename = "PlatformSoftwareVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_software_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionDefinitionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "InitialVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<SubscriptionDefinitionVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscriptionDefinitionVersion {
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subscription {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: String,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionDefinitionVersionRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "SubscriptionDefinitionId")]
    #[serde(default)]
    pub subscription_definition_id: String,
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorDefinitionRequest {
    #[serde(rename = "ConnectorDefinitionId")]
    #[serde(default)]
    pub connector_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreDefinitionRequest {
    #[serde(rename = "CoreDefinitionId")]
    #[serde(default)]
    pub core_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeviceDefinitionRequest {
    #[serde(rename = "DeviceDefinitionId")]
    #[serde(default)]
    pub device_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeviceDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionDefinitionRequest {
    #[serde(rename = "FunctionDefinitionId")]
    #[serde(default)]
    pub function_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoggerDefinitionRequest {
    #[serde(rename = "LoggerDefinitionId")]
    #[serde(default)]
    pub logger_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoggerDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceDefinitionRequest {
    #[serde(rename = "ResourceDefinitionId")]
    #[serde(default)]
    pub resource_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubscriptionDefinitionRequest {
    #[serde(rename = "SubscriptionDefinitionId")]
    #[serde(default)]
    pub subscription_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubscriptionDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateRoleFromGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateRoleFromGroupResponse {
    #[serde(rename = "DisassociatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateServiceRoleFromAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateServiceRoleFromAccountResponse {
    #[serde(rename = "DisassociatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssociatedRoleRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssociatedRoleResponse {
    #[serde(rename = "AssociatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBulkDeploymentStatusRequest {
    #[serde(rename = "BulkDeploymentId")]
    #[serde(default)]
    pub bulk_deployment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBulkDeploymentStatusResponse {
    #[serde(rename = "BulkDeploymentMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_metrics: Option<BulkDeploymentMetrics>,
    #[serde(rename = "BulkDeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_status: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkDeploymentMetrics {
    #[serde(rename = "InvalidInputRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_input_records: Option<i32>,
    #[serde(rename = "RecordsProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_processed: Option<i32>,
    #[serde(rename = "RetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_attempts: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetail {
    #[serde(rename = "DetailedErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_error_code: Option<String>,
    #[serde(rename = "DetailedErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectivityInfoRequest {
    #[serde(rename = "ThingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectivityInfoResponse {
    #[serde(rename = "ConnectivityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectivityInfo {
    #[serde(rename = "HostAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "PortNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorDefinitionRequest {
    #[serde(rename = "ConnectorDefinitionId")]
    #[serde(default)]
    pub connector_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorDefinitionVersionRequest {
    #[serde(rename = "ConnectorDefinitionId")]
    #[serde(default)]
    pub connector_definition_id: String,
    #[serde(rename = "ConnectorDefinitionVersionId")]
    #[serde(default)]
    pub connector_definition_version_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ConnectorDefinitionVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreDefinitionRequest {
    #[serde(rename = "CoreDefinitionId")]
    #[serde(default)]
    pub core_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreDefinitionVersionRequest {
    #[serde(rename = "CoreDefinitionId")]
    #[serde(default)]
    pub core_definition_id: String,
    #[serde(rename = "CoreDefinitionVersionId")]
    #[serde(default)]
    pub core_definition_version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<CoreDefinitionVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentStatusRequest {
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentStatusResponse {
    #[serde(rename = "DeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceDefinitionRequest {
    #[serde(rename = "DeviceDefinitionId")]
    #[serde(default)]
    pub device_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceDefinitionVersionRequest {
    #[serde(rename = "DeviceDefinitionId")]
    #[serde(default)]
    pub device_definition_id: String,
    #[serde(rename = "DeviceDefinitionVersionId")]
    #[serde(default)]
    pub device_definition_version_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DeviceDefinitionVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionDefinitionRequest {
    #[serde(rename = "FunctionDefinitionId")]
    #[serde(default)]
    pub function_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionDefinitionVersionRequest {
    #[serde(rename = "FunctionDefinitionId")]
    #[serde(default)]
    pub function_definition_id: String,
    #[serde(rename = "FunctionDefinitionVersionId")]
    #[serde(default)]
    pub function_definition_version_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FunctionDefinitionVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityId")]
    #[serde(default)]
    pub certificate_authority_id: String,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupCertificateAuthorityResponse {
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
    #[serde(rename = "PemEncodedCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_encoded_certificate: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupCertificateConfigurationRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupCertificateConfigurationResponse {
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupVersionRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "GroupVersionId")]
    #[serde(default)]
    pub group_version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<GroupVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggerDefinitionRequest {
    #[serde(rename = "LoggerDefinitionId")]
    #[serde(default)]
    pub logger_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggerDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggerDefinitionVersionRequest {
    #[serde(rename = "LoggerDefinitionId")]
    #[serde(default)]
    pub logger_definition_id: String,
    #[serde(rename = "LoggerDefinitionVersionId")]
    #[serde(default)]
    pub logger_definition_version_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggerDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<LoggerDefinitionVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDefinitionRequest {
    #[serde(rename = "ResourceDefinitionId")]
    #[serde(default)]
    pub resource_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDefinitionVersionRequest {
    #[serde(rename = "ResourceDefinitionId")]
    #[serde(default)]
    pub resource_definition_id: String,
    #[serde(rename = "ResourceDefinitionVersionId")]
    #[serde(default)]
    pub resource_definition_version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ResourceDefinitionVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceRoleForAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceRoleForAccountResponse {
    #[serde(rename = "AssociatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_at: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSubscriptionDefinitionRequest {
    #[serde(rename = "SubscriptionDefinitionId")]
    #[serde(default)]
    pub subscription_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSubscriptionDefinitionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSubscriptionDefinitionVersionRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SubscriptionDefinitionId")]
    #[serde(default)]
    pub subscription_definition_id: String,
    #[serde(rename = "SubscriptionDefinitionVersionId")]
    #[serde(default)]
    pub subscription_definition_version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSubscriptionDefinitionVersionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<SubscriptionDefinitionVersion>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThingRuntimeConfigurationRequest {
    #[serde(rename = "ThingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThingRuntimeConfigurationResponse {
    #[serde(rename = "RuntimeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeConfiguration {
    #[serde(rename = "TelemetryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_configuration: Option<TelemetryConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TelemetryConfiguration {
    #[serde(rename = "ConfigurationSyncStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_sync_status: Option<String>,
    #[serde(rename = "Telemetry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBulkDeploymentDetailedReportsRequest {
    #[serde(rename = "BulkDeploymentId")]
    #[serde(default)]
    pub bulk_deployment_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBulkDeploymentDetailedReportsResponse {
    #[serde(rename = "Deployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<BulkDeploymentResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkDeploymentResult {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "DeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "DeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "GroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBulkDeploymentsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBulkDeploymentsResponse {
    #[serde(rename = "BulkDeployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployments: Option<Vec<BulkDeployment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkDeployment {
    #[serde(rename = "BulkDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_arn: Option<String>,
    #[serde(rename = "BulkDeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorDefinitionVersionsRequest {
    #[serde(rename = "ConnectorDefinitionId")]
    #[serde(default)]
    pub connector_definition_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorDefinitionVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VersionInformation {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorDefinitionsResponse {
    #[serde(rename = "Definitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefinitionInformation {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreDefinitionVersionsRequest {
    #[serde(rename = "CoreDefinitionId")]
    #[serde(default)]
    pub core_definition_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreDefinitionVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreDefinitionsResponse {
    #[serde(rename = "Definitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentsRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentsResponse {
    #[serde(rename = "Deployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "DeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "GroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeviceDefinitionVersionsRequest {
    #[serde(rename = "DeviceDefinitionId")]
    #[serde(default)]
    pub device_definition_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeviceDefinitionVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeviceDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeviceDefinitionsResponse {
    #[serde(rename = "Definitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionDefinitionVersionsRequest {
    #[serde(rename = "FunctionDefinitionId")]
    #[serde(default)]
    pub function_definition_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionDefinitionVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionDefinitionsResponse {
    #[serde(rename = "Definitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupCertificateAuthoritiesRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupCertificateAuthoritiesResponse {
    #[serde(rename = "GroupCertificateAuthorities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authorities: Option<Vec<GroupCertificateAuthorityProperties>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupCertificateAuthorityProperties {
    #[serde(rename = "GroupCertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_arn: Option<String>,
    #[serde(rename = "GroupCertificateAuthorityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_certificate_authority_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupVersionsRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupInformation {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "LatestVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLoggerDefinitionVersionsRequest {
    #[serde(rename = "LoggerDefinitionId")]
    #[serde(default)]
    pub logger_definition_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLoggerDefinitionVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLoggerDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLoggerDefinitionsResponse {
    #[serde(rename = "Definitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceDefinitionVersionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceDefinitionId")]
    #[serde(default)]
    pub resource_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceDefinitionVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceDefinitionsResponse {
    #[serde(rename = "Definitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSubscriptionDefinitionVersionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SubscriptionDefinitionId")]
    #[serde(default)]
    pub subscription_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSubscriptionDefinitionVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSubscriptionDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSubscriptionDefinitionsResponse {
    #[serde(rename = "Definitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<DefinitionInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
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
pub struct ResetDeploymentsRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetDeploymentsResponse {
    #[serde(rename = "DeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_arn: Option<String>,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBulkDeploymentRequest {
    #[serde(rename = "AmznClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amzn_client_token: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "InputFileUri")]
    #[serde(default)]
    pub input_file_uri: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBulkDeploymentResponse {
    #[serde(rename = "BulkDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_arn: Option<String>,
    #[serde(rename = "BulkDeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_deployment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBulkDeploymentRequest {
    #[serde(rename = "BulkDeploymentId")]
    #[serde(default)]
    pub bulk_deployment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBulkDeploymentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
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
pub struct UpdateConnectivityInfoRequest {
    #[serde(rename = "ConnectivityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_info: Option<Vec<ConnectivityInfo>>,
    #[serde(rename = "ThingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectivityInfoResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorDefinitionRequest {
    #[serde(rename = "ConnectorDefinitionId")]
    #[serde(default)]
    pub connector_definition_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCoreDefinitionRequest {
    #[serde(rename = "CoreDefinitionId")]
    #[serde(default)]
    pub core_definition_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCoreDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceDefinitionRequest {
    #[serde(rename = "DeviceDefinitionId")]
    #[serde(default)]
    pub device_definition_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionDefinitionRequest {
    #[serde(rename = "FunctionDefinitionId")]
    #[serde(default)]
    pub function_definition_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupCertificateConfigurationRequest {
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupCertificateConfigurationResponse {
    #[serde(rename = "CertificateAuthorityExpiryInMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_expiry_in_milliseconds: Option<String>,
    #[serde(rename = "CertificateExpiryInMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_expiry_in_milliseconds: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLoggerDefinitionRequest {
    #[serde(rename = "LoggerDefinitionId")]
    #[serde(default)]
    pub logger_definition_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLoggerDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceDefinitionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceDefinitionId")]
    #[serde(default)]
    pub resource_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionDefinitionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SubscriptionDefinitionId")]
    #[serde(default)]
    pub subscription_definition_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingRuntimeConfigurationRequest {
    #[serde(rename = "TelemetryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_configuration: Option<TelemetryConfigurationUpdate>,
    #[serde(rename = "ThingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TelemetryConfigurationUpdate {
    #[serde(rename = "Telemetry")]
    #[serde(default)]
    pub telemetry: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingRuntimeConfigurationResponse {}
