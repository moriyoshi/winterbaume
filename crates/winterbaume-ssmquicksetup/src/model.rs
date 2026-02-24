//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ssmquicksetup

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationManagerInput {
    #[serde(rename = "ConfigurationDefinitions")]
    #[serde(default)]
    pub configuration_definitions: Vec<ConfigurationDefinitionInput>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationDefinitionInput {
    #[serde(rename = "LocalDeploymentAdministrationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_deployment_administration_role_arn: Option<String>,
    #[serde(rename = "LocalDeploymentExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_deployment_execution_role_name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: std::collections::HashMap<String, String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "TypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationManagerOutput {
    #[serde(rename = "ManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationManagerInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationManagerInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationManagerOutput {
    #[serde(rename = "ConfigurationDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_definitions: Option<Vec<ConfigurationDefinition>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
    #[serde(rename = "ManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StatusSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summaries: Option<Vec<StatusSummary>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationDefinition {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LocalDeploymentAdministrationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_deployment_administration_role_arn: Option<String>,
    #[serde(rename = "LocalDeploymentExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_deployment_execution_role_name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusSummary {
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "StatusType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationOutput {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "ConfigurationDefinitionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_definition_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
    #[serde(rename = "ManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_arn: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StatusSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summaries: Option<Vec<StatusSummary>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceSettingsOutput {
    #[serde(rename = "ServiceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_settings: Option<ServiceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSettings {
    #[serde(rename = "ExplorerEnablingRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer_enabling_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationManagersInput {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "StartingToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationManagersOutput {
    #[serde(rename = "ConfigurationManagersList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_managers_list: Option<Vec<ConfigurationManagerSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationManagerSummary {
    #[serde(rename = "ConfigurationDefinitionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_definition_summaries: Option<Vec<ConfigurationDefinitionSummary>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StatusSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summaries: Option<Vec<StatusSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationDefinitionSummary {
    #[serde(rename = "FirstClassParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_class_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationsInput {
    #[serde(rename = "ConfigurationDefinitionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_definition_id: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "ManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_arn: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "StartingToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationsOutput {
    #[serde(rename = "ConfigurationsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations_list: Option<Vec<ConfigurationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationSummary {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "ConfigurationDefinitionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_definition_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "FirstClassParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_class_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_arn: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StatusSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summaries: Option<Vec<StatusSummary>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQuickSetupTypesOutput {
    #[serde(rename = "QuickSetupTypeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_setup_type_list: Option<Vec<QuickSetupTypeOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickSetupTypeOutput {
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagEntry {
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
pub struct TagResourceInput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationDefinitionInput {
    #[serde(rename = "LocalDeploymentAdministrationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_deployment_administration_role_arn: Option<String>,
    #[serde(rename = "LocalDeploymentExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_deployment_execution_role_name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationManagerInput {
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
pub struct UpdateServiceSettingsInput {
    #[serde(rename = "ExplorerEnablingRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer_enabling_role_arn: Option<String>,
}
