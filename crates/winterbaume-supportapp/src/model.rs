//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-supportapp

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlackChannelConfigurationRequest {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "channelRoleArn")]
    #[serde(default)]
    pub channel_role_arn: String,
    #[serde(rename = "notifyOnAddCorrespondenceToCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_add_correspondence_to_case: Option<bool>,
    #[serde(rename = "notifyOnCaseSeverity")]
    #[serde(default)]
    pub notify_on_case_severity: String,
    #[serde(rename = "notifyOnCreateOrReopenCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_create_or_reopen_case: Option<bool>,
    #[serde(rename = "notifyOnResolveCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_resolve_case: Option<bool>,
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlackChannelConfigurationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountAliasRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountAliasResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackChannelConfigurationRequest {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackChannelConfigurationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackWorkspaceConfigurationRequest {
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlackWorkspaceConfigurationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountAliasRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountAliasResult {
    #[serde(rename = "accountAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlackChannelConfigurationsRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlackChannelConfigurationsResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "slackChannelConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_channel_configurations: Option<Vec<SlackChannelConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackChannelConfiguration {
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "channelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "channelRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_role_arn: Option<String>,
    #[serde(rename = "notifyOnAddCorrespondenceToCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_add_correspondence_to_case: Option<bool>,
    #[serde(rename = "notifyOnCaseSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_case_severity: Option<String>,
    #[serde(rename = "notifyOnCreateOrReopenCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_create_or_reopen_case: Option<bool>,
    #[serde(rename = "notifyOnResolveCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_resolve_case: Option<bool>,
    #[serde(rename = "teamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlackWorkspaceConfigurationsRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlackWorkspaceConfigurationsResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "slackWorkspaceConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_workspace_configurations: Option<Vec<SlackWorkspaceConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackWorkspaceConfiguration {
    #[serde(rename = "allowOrganizationMemberAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_organization_member_account: Option<bool>,
    #[serde(rename = "teamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "teamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountAliasRequest {
    #[serde(rename = "accountAlias")]
    #[serde(default)]
    pub account_alias: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountAliasResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterSlackWorkspaceForOrganizationRequest {
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterSlackWorkspaceForOrganizationResult {
    #[serde(rename = "accountType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(rename = "teamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "teamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSlackChannelConfigurationRequest {
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "channelRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_role_arn: Option<String>,
    #[serde(rename = "notifyOnAddCorrespondenceToCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_add_correspondence_to_case: Option<bool>,
    #[serde(rename = "notifyOnCaseSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_case_severity: Option<String>,
    #[serde(rename = "notifyOnCreateOrReopenCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_create_or_reopen_case: Option<bool>,
    #[serde(rename = "notifyOnResolveCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_resolve_case: Option<bool>,
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSlackChannelConfigurationResult {
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "channelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "channelRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_role_arn: Option<String>,
    #[serde(rename = "notifyOnAddCorrespondenceToCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_add_correspondence_to_case: Option<bool>,
    #[serde(rename = "notifyOnCaseSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_case_severity: Option<String>,
    #[serde(rename = "notifyOnCreateOrReopenCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_create_or_reopen_case: Option<bool>,
    #[serde(rename = "notifyOnResolveCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_resolve_case: Option<bool>,
    #[serde(rename = "teamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}
