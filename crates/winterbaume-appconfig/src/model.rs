//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appconfig

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSettings {
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<DeletionProtectionSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletionProtectionSettings {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ProtectionPeriodInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_period_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Applications {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Application>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(rename = "ConfigurationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationProfile {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RetrievalRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_role_arn: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Validators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<Validator>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Validator {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationProfiles {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ConfigurationProfileSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationProfileSummary {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ValidatorTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationProfileRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LocationUri")]
    #[serde(default)]
    pub location_uri: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RetrievalRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Validators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<Validator>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentStrategyRequest {
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(default)]
    pub deployment_duration_in_minutes: i32,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i32>,
    #[serde(rename = "GrowthFactor")]
    #[serde(default)]
    pub growth_factor: f32,
    #[serde(rename = "GrowthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ReplicateTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicate_to: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEnvironmentRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Monitors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<Monitor>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Monitor {
    #[serde(rename = "AlarmArn")]
    #[serde(default)]
    pub alarm_arn: String,
    #[serde(rename = "AlarmRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExtensionAssociationRequest {
    #[serde(rename = "ExtensionIdentifier")]
    #[serde(default)]
    pub extension_identifier: String,
    #[serde(rename = "ExtensionVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_version_number: Option<i32>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExtensionRequest {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: std::collections::HashMap<String, Vec<Action>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LatestVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Parameter>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameter {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Dynamic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    #[serde(rename = "Required")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHostedConfigurationVersionRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LatestVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i32>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationProfileRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "DeletionProtectionCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_check: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeploymentStrategyRequest {
    #[serde(rename = "DeploymentStrategyId")]
    #[serde(default)]
    pub deployment_strategy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEnvironmentRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "DeletionProtectionCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_check: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    pub environment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExtensionAssociationRequest {
    #[serde(rename = "ExtensionAssociationId")]
    #[serde(default)]
    pub extension_association_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExtensionRequest {
    #[serde(rename = "ExtensionIdentifier")]
    #[serde(default)]
    pub extension_identifier: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHostedConfigurationVersionRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "AppliedExtensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_extensions: Option<Vec<AppliedExtension>>,
    #[serde(rename = "CompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "ConfigurationLocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_location_uri: Option<String>,
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_profile_id: Option<String>,
    #[serde(rename = "ConfigurationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i32>,
    #[serde(rename = "DeploymentNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_number: Option<i32>,
    #[serde(rename = "DeploymentStrategyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_strategy_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EventLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_log: Option<Vec<DeploymentEvent>>,
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i32>,
    #[serde(rename = "GrowthFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    #[serde(rename = "GrowthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "PercentageComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete: Option<f32>,
    #[serde(rename = "StartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppliedExtension {
    #[serde(rename = "ExtensionAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_association_id: Option<String>,
    #[serde(rename = "ExtensionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentEvent {
    #[serde(rename = "ActionInvocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_invocations: Option<Vec<ActionInvocation>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "OccurredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<String>,
    #[serde(rename = "TriggeredBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionInvocation {
    #[serde(rename = "ActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ExtensionIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_identifier: Option<String>,
    #[serde(rename = "InvocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentStrategies {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DeploymentStrategy>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentStrategy {
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i32>,
    #[serde(rename = "GrowthFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    #[serde(rename = "GrowthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReplicateTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicate_to: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deployments {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DeploymentSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentSummary {
    #[serde(rename = "CompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    #[serde(rename = "ConfigurationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i32>,
    #[serde(rename = "DeploymentNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_number: Option<i32>,
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i32>,
    #[serde(rename = "GrowthFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    #[serde(rename = "GrowthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
    #[serde(rename = "PercentageComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_complete: Option<f32>,
    #[serde(rename = "StartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Environment {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Monitors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<Monitor>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Environments {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Environment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Extension {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<std::collections::HashMap<String, Vec<Action>>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Parameter>>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtensionAssociation {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ExtensionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_arn: Option<String>,
    #[serde(rename = "ExtensionVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_version_number: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtensionAssociations {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ExtensionAssociationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtensionAssociationSummary {
    #[serde(rename = "ExtensionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Extensions {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ExtensionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtensionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationProfileRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationRequest {
    #[serde(rename = "Application")]
    #[serde(default)]
    pub application: String,
    #[serde(rename = "ClientConfigurationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_configuration_version: Option<String>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: String,
    #[serde(rename = "Environment")]
    #[serde(default)]
    pub environment: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "DeploymentNumber")]
    #[serde(default)]
    pub deployment_number: i32,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    pub environment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentStrategyRequest {
    #[serde(rename = "DeploymentStrategyId")]
    #[serde(default)]
    pub deployment_strategy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEnvironmentRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    pub environment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExtensionAssociationRequest {
    #[serde(rename = "ExtensionAssociationId")]
    #[serde(default)]
    pub extension_association_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExtensionRequest {
    #[serde(rename = "ExtensionIdentifier")]
    #[serde(default)]
    pub extension_identifier: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHostedConfigurationVersionRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostedConfigurationVersion {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_profile_id: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostedConfigurationVersions {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<HostedConfigurationVersionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostedConfigurationVersionSummary {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_profile_id: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsRequest {
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
pub struct ListConfigurationProfilesRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentStrategiesRequest {
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
pub struct ListDeploymentsRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    pub environment_id: String,
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
pub struct ListEnvironmentsRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
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
pub struct ListExtensionAssociationsRequest {
    #[serde(rename = "ExtensionIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_identifier: Option<String>,
    #[serde(rename = "ExtensionVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_version_number: Option<i32>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExtensionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHostedConfigurationVersionsRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTags {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeploymentRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "ConfigurationVersion")]
    #[serde(default)]
    pub configuration_version: String,
    #[serde(rename = "DeploymentStrategyId")]
    #[serde(default)]
    pub deployment_strategy_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DynamicExtensionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_extension_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDeploymentRequest {
    #[serde(rename = "AllowRevert")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_revert: Option<bool>,
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "DeploymentNumber")]
    #[serde(default)]
    pub deployment_number: i32,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    pub environment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
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
pub struct UpdateAccountSettingsRequest {
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<DeletionProtectionSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationProfileRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RetrievalRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_role_arn: Option<String>,
    #[serde(rename = "Validators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<Validator>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeploymentStrategyRequest {
    #[serde(rename = "DeploymentDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_duration_in_minutes: Option<i32>,
    #[serde(rename = "DeploymentStrategyId")]
    #[serde(default)]
    pub deployment_strategy_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FinalBakeTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_bake_time_in_minutes: Option<i32>,
    #[serde(rename = "GrowthFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_factor: Option<f32>,
    #[serde(rename = "GrowthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEnvironmentRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(rename = "Monitors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<Monitor>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExtensionAssociationRequest {
    #[serde(rename = "ExtensionAssociationId")]
    #[serde(default)]
    pub extension_association_id: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExtensionRequest {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<std::collections::HashMap<String, Vec<Action>>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExtensionIdentifier")]
    #[serde(default)]
    pub extension_identifier: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Parameter>>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateConfigurationRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "ConfigurationProfileId")]
    #[serde(default)]
    pub configuration_profile_id: String,
    #[serde(rename = "ConfigurationVersion")]
    #[serde(default)]
    pub configuration_version: String,
}
