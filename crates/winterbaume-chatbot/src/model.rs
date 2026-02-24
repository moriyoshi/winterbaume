//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-chatbot

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackWorkspaceAuthorizationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlackChannelConfigurationResult {
    #[serde(rename = "ChannelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_configuration: Option<SlackChannelConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackChannelConfiguration {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    #[serde(rename = "GuardrailPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_policy_arns: Option<Vec<String>>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SlackChannelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_channel_id: Option<String>,
    #[serde(rename = "SlackChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_channel_name: Option<String>,
    #[serde(rename = "SlackTeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_team_id: Option<String>,
    #[serde(rename = "SlackTeamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_team_name: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagValue")]
    #[serde(default)]
    pub tag_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountPreferencesResult {
    #[serde(rename = "AccountPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_preferences: Option<AccountPreferences>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountPreferences {
    #[serde(rename = "TrainingDataCollectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_collection_enabled: Option<bool>,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackWorkspaceAuthorizationRequest {
    #[serde(rename = "SlackTeamId")]
    #[serde(default)]
    pub slack_team_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomActionsRequest {
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
pub struct DisassociateFromConfigurationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlackChannelConfigurationsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SlackChannelConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_channel_configurations: Option<Vec<SlackChannelConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateToConfigurationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMicrosoftTeamsUserIdentitiesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TeamsUserIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_user_identities: Option<Vec<TeamsUserIdentity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TeamsUserIdentity {
    #[serde(rename = "AwsUserIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_user_identity: Option<String>,
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "TeamsChannelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_channel_id: Option<String>,
    #[serde(rename = "TeamsTenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams_tenant_id: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTeamsChannelConfigurationsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TeamChannelConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_channel_configurations: Option<Vec<TeamsChannelConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TeamsChannelConfiguration {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    #[serde(rename = "GuardrailPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_policy_arns: Option<Vec<String>>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "TeamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTeamsConfiguredTeamResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChimeWebhookConfigurationRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlackChannelConfigurationsRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
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
pub struct UpdateCustomActionRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<CustomActionAttachment>>,
    #[serde(rename = "CustomActionArn")]
    #[serde(default)]
    pub custom_action_arn: String,
    #[serde(rename = "Definition")]
    #[serde(default)]
    pub definition: CustomActionDefinition,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomActionAttachment {
    #[serde(rename = "ButtonText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "Criteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<CustomActionAttachmentCriteria>>,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<String>,
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomActionAttachmentCriteria {
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "VariableName")]
    #[serde(default)]
    pub variable_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomActionDefinition {
    #[serde(rename = "CommandText")]
    #[serde(default)]
    pub command_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociationsResult {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<AssociationListing>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationListing {
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomActionRequest {
    #[serde(rename = "ActionName")]
    #[serde(default)]
    pub action_name: String,
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<CustomActionAttachment>>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    pub definition: CustomActionDefinition,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChimeWebhookConfigurationResult {
    #[serde(rename = "WebhookConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_configuration: Option<ChimeWebhookConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimeWebhookConfiguration {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WebhookDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMicrosoftTeamsUserIdentityResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTeamsChannelConfigurationRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    pub configuration_name: String,
    #[serde(rename = "GuardrailPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_policy_arns: Option<Vec<String>>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    pub team_id: String,
    #[serde(rename = "TeamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    pub tenant_id: String,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTeamsChannelConfigurationRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMicrosoftTeamsUserIdentitiesRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
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
pub struct UpdateSlackChannelConfigurationRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
    #[serde(rename = "GuardrailPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_policy_arns: Option<Vec<String>>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SlackChannelId")]
    #[serde(default)]
    pub slack_channel_id: String,
    #[serde(rename = "SlackChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_channel_name: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomActionRequest {
    #[serde(rename = "CustomActionArn")]
    #[serde(default)]
    pub custom_action_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTeamsChannelConfigurationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomActionResult {
    #[serde(rename = "CustomAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_action: Option<CustomAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomAction {
    #[serde(rename = "ActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<CustomActionAttachment>>,
    #[serde(rename = "CustomActionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_action_arn: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<CustomActionDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTeamsChannelConfigurationResult {
    #[serde(rename = "ChannelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_configuration: Option<TeamsChannelConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountPreferencesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTeamsChannelConfigurationRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlackWorkspacesRequest {
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
pub struct DisassociateFromConfigurationRequest {
    #[serde(rename = "ChatConfiguration")]
    #[serde(default)]
    pub chat_configuration: String,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMicrosoftTeamsConfiguredTeamsRequest {
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
pub struct DeleteSlackChannelConfigurationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChimeWebhookConfigurationRequest {
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    pub configuration_name: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    pub sns_topic_arns: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WebhookDescription")]
    #[serde(default)]
    pub webhook_description: String,
    #[serde(rename = "WebhookUrl")]
    #[serde(default)]
    pub webhook_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomActionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountPreferencesRequest {
    #[serde(rename = "TrainingDataCollectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_collection_enabled: Option<bool>,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlackUserIdentitiesRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
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
pub struct UpdateChimeWebhookConfigurationRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "WebhookDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_description: Option<String>,
    #[serde(rename = "WebhookUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSlackChannelConfigurationResult {
    #[serde(rename = "ChannelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_configuration: Option<SlackChannelConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTeamsChannelConfigurationResult {
    #[serde(rename = "ChannelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_configuration: Option<TeamsChannelConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountPreferencesResult {
    #[serde(rename = "AccountPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_preferences: Option<AccountPreferences>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChimeWebhookConfigurationsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WebhookConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_configurations: Option<Vec<ChimeWebhookConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTeamsConfiguredTeamRequest {
    #[serde(rename = "TeamId")]
    #[serde(default)]
    pub team_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlackUserIdentitiesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SlackUserIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_user_identities: Option<Vec<SlackUserIdentity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackUserIdentity {
    #[serde(rename = "AwsUserIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_user_identity: Option<String>,
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "SlackTeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_team_id: Option<String>,
    #[serde(rename = "SlackUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMicrosoftTeamsUserIdentityRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociationsRequest {
    #[serde(rename = "ChatConfiguration")]
    #[serde(default)]
    pub chat_configuration: String,
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
pub struct DescribeSlackWorkspacesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SlackWorkspaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_workspaces: Option<Vec<SlackWorkspace>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackWorkspace {
    #[serde(rename = "SlackTeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_team_id: Option<String>,
    #[serde(rename = "SlackTeamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_team_name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackUserIdentityRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
    #[serde(rename = "SlackTeamId")]
    #[serde(default)]
    pub slack_team_id: String,
    #[serde(rename = "SlackUserId")]
    #[serde(default)]
    pub slack_user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackChannelConfigurationRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomActionsResult {
    #[serde(rename = "CustomActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_actions: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChimeWebhookConfigurationsRequest {
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_configuration_arn: Option<String>,
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
pub struct DeleteSlackUserIdentityResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlackChannelConfigurationRequest {
    #[serde(rename = "ConfigurationName")]
    #[serde(default)]
    pub configuration_name: String,
    #[serde(rename = "GuardrailPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_policy_arns: Option<Vec<String>>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SlackChannelId")]
    #[serde(default)]
    pub slack_channel_id: String,
    #[serde(rename = "SlackChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_channel_name: Option<String>,
    #[serde(rename = "SlackTeamId")]
    #[serde(default)]
    pub slack_team_id: String,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMicrosoftTeamsConfiguredTeamsResult {
    #[serde(rename = "ConfiguredTeams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_teams: Option<Vec<ConfiguredTeam>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfiguredTeam {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "TeamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomActionRequest {
    #[serde(rename = "CustomActionArn")]
    #[serde(default)]
    pub custom_action_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChimeWebhookConfigurationResult {
    #[serde(rename = "WebhookConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_configuration: Option<ChimeWebhookConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomActionResult {
    #[serde(rename = "CustomActionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_action_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTeamsChannelConfigurationRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ChatConfigurationArn")]
    #[serde(default)]
    pub chat_configuration_arn: String,
    #[serde(rename = "GuardrailPolicyArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_policy_arns: Option<Vec<String>>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "SnsTopicArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arns: Option<Vec<String>>,
    #[serde(rename = "UserAuthorizationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_authorization_required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTeamsChannelConfigurationResult {
    #[serde(rename = "ChannelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_configuration: Option<TeamsChannelConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTeamsChannelConfigurationsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateToConfigurationRequest {
    #[serde(rename = "ChatConfiguration")]
    #[serde(default)]
    pub chat_configuration: String,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomActionResult {
    #[serde(rename = "CustomActionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_action_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChimeWebhookConfigurationResult {}
