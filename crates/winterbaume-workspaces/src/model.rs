//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-workspaces

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAccountLinkInvitationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    pub link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAccountLinkInvitationResult {
    #[serde(rename = "AccountLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link: Option<AccountLink>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountLink {
    #[serde(rename = "AccountLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link_id: Option<String>,
    #[serde(rename = "AccountLinkStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link_status: Option<String>,
    #[serde(rename = "SourceAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account_id: Option<String>,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConnectionAliasRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConnectionAliasResult {
    #[serde(rename = "ConnectionIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateIpGroupsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "GroupIds")]
    #[serde(default)]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateIpGroupsResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateWorkspaceApplicationRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateWorkspaceApplicationResult {
    #[serde(rename = "Association")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<WorkspaceResourceAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceResourceAssociation {
    #[serde(rename = "AssociatedResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_id: Option<String>,
    #[serde(rename = "AssociatedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_type: Option<String>,
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<AssociationStateReason>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationStateReason {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizeIpRulesRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "UserRules")]
    #[serde(default)]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpRuleItem {
    #[serde(rename = "ipRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_rule: Option<String>,
    #[serde(rename = "ruleDesc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_desc: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizeIpRulesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyWorkspaceImageRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SourceImageId")]
    #[serde(default)]
    pub source_image_id: String,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    pub source_region: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyWorkspaceImageResult {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountLinkInvitationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    pub target_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountLinkInvitationResult {
    #[serde(rename = "AccountLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link: Option<AccountLink>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectClientAddInRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "URL")]
    #[serde(default)]
    pub u_r_l: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectClientAddInResult {
    #[serde(rename = "AddInId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_in_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionAliasRequest {
    #[serde(rename = "ConnectionString")]
    #[serde(default)]
    pub connection_string: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionAliasResult {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIpGroupRequest {
    #[serde(rename = "GroupDesc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rules: Option<Vec<IpRuleItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIpGroupResult {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStandbyWorkspacesRequest {
    #[serde(rename = "PrimaryRegion")]
    #[serde(default)]
    pub primary_region: String,
    #[serde(rename = "StandbyWorkspaces")]
    #[serde(default)]
    pub standby_workspaces: Vec<StandbyWorkspace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandbyWorkspace {
    #[serde(rename = "DataReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "PrimaryWorkspaceId")]
    #[serde(default)]
    pub primary_workspace_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStandbyWorkspacesResult {
    #[serde(rename = "FailedStandbyRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_standby_requests: Option<Vec<FailedCreateStandbyWorkspacesRequest>>,
    #[serde(rename = "PendingStandbyRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_standby_requests: Option<Vec<PendingCreateStandbyWorkspacesRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedCreateStandbyWorkspacesRequest {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "StandbyWorkspaceRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_workspace_request: Option<StandbyWorkspace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingCreateStandbyWorkspacesRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagsRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagsResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUpdatedWorkspaceImageRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SourceImageId")]
    #[serde(default)]
    pub source_image_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUpdatedWorkspaceImageResult {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceBundleRequest {
    #[serde(rename = "BundleDescription")]
    #[serde(default)]
    pub bundle_description: String,
    #[serde(rename = "BundleName")]
    #[serde(default)]
    pub bundle_name: String,
    #[serde(rename = "ComputeType")]
    #[serde(default)]
    pub compute_type: ComputeType,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    pub image_id: String,
    #[serde(rename = "RootStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_storage: Option<RootStorage>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserStorage")]
    #[serde(default)]
    pub user_storage: UserStorage,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeType {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RootStorage {
    #[serde(rename = "Capacity")]
    #[serde(default)]
    pub capacity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserStorage {
    #[serde(rename = "Capacity")]
    #[serde(default)]
    pub capacity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceBundleResult {
    #[serde(rename = "WorkspaceBundle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_bundle: Option<WorkspaceBundle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceBundle {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "BundleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    #[serde(rename = "ComputeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<ComputeType>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "RootStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_storage: Option<RootStorage>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UserStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_storage: Option<UserStorage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceImageRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceImageResult {
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystem>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "RequiredTenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_tenancy: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperatingSystem {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspacesPoolRequest {
    #[serde(rename = "ApplicationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings: Option<ApplicationSettingsRequest>,
    #[serde(rename = "BundleId")]
    #[serde(default)]
    pub bundle_id: String,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    pub capacity: Capacity,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "PoolName")]
    #[serde(default)]
    pub pool_name: String,
    #[serde(rename = "RunningMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TimeoutSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_settings: Option<TimeoutSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSettingsRequest {
    #[serde(rename = "SettingsGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_group: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capacity {
    #[serde(rename = "DesiredUserSessions")]
    #[serde(default)]
    pub desired_user_sessions: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeoutSettings {
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timeout_in_seconds: Option<i32>,
    #[serde(rename = "IdleDisconnectTimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_disconnect_timeout_in_seconds: Option<i32>,
    #[serde(rename = "MaxUserDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspacesPoolResult {
    #[serde(rename = "WorkspacesPool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_pool: Option<WorkspacesPool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspacesPool {
    #[serde(rename = "ApplicationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings: Option<ApplicationSettingsResponse>,
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "CapacityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_status: Option<CapacityStatus>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<WorkspacesPoolError>>,
    #[serde(rename = "PoolArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_arn: Option<String>,
    #[serde(rename = "PoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "PoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    #[serde(rename = "RunningMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TimeoutSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_settings: Option<TimeoutSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSettingsResponse {
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "SettingsGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_group: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityStatus {
    #[serde(rename = "ActiveUserSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_user_sessions: Option<i32>,
    #[serde(rename = "ActualUserSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_user_sessions: Option<i32>,
    #[serde(rename = "AvailableUserSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_user_sessions: Option<i32>,
    #[serde(rename = "DesiredUserSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_user_sessions: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspacesPoolError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspacesRequest {
    #[serde(rename = "Workspaces")]
    #[serde(default)]
    pub workspaces: Vec<WorkspaceRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    pub bundle_id: String,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "Ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    #[serde(rename = "WorkspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[serde(rename = "WorkspaceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceProperties {
    #[serde(rename = "ComputeTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type_name: Option<String>,
    #[serde(rename = "GlobalAccelerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_accelerator: Option<GlobalAcceleratorForWorkSpace>,
    #[serde(rename = "OperatingSystemName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_name: Option<String>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "RootVolumeSizeGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_size_gib: Option<i32>,
    #[serde(rename = "RunningMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    #[serde(rename = "RunningModeAutoStopTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode_auto_stop_timeout_in_minutes: Option<i32>,
    #[serde(rename = "UserVolumeSizeGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_size_gib: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalAcceleratorForWorkSpace {
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
    #[serde(rename = "PreferredProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspacesResult {
    #[serde(rename = "FailedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedCreateWorkspaceRequest>>,
    #[serde(rename = "PendingRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_requests: Option<Vec<Workspace>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedCreateWorkspaceRequest {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "WorkspaceRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_request: Option<WorkspaceRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Workspace {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "ComputerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "DataReplicationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_settings: Option<DataReplicationSettings>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "Ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "ModificationStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_states: Option<Vec<ModificationState>>,
    #[serde(rename = "RelatedWorkspaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_workspaces: Option<Vec<RelatedWorkspaceProperties>>,
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<bool>,
    #[serde(rename = "StandbyWorkspacesProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_workspaces_properties: Option<Vec<StandbyWorkspacesProperties>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<bool>,
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "WorkspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[serde(rename = "WorkspaceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataReplicationSettings {
    #[serde(rename = "DataReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication: Option<String>,
    #[serde(rename = "RecoverySnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_snapshot_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModificationState {
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedWorkspaceProperties {
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandbyWorkspacesProperties {
    #[serde(rename = "DataReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication: Option<String>,
    #[serde(rename = "RecoverySnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_snapshot_time: Option<f64>,
    #[serde(rename = "StandbyWorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountLinkInvitationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    pub link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountLinkInvitationResult {
    #[serde(rename = "AccountLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link: Option<AccountLink>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClientBrandingRequest {
    #[serde(rename = "Platforms")]
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClientBrandingResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectClientAddInRequest {
    #[serde(rename = "AddInId")]
    #[serde(default)]
    pub add_in_id: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectClientAddInResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionAliasRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionAliasResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIpGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIpGroupResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceBundleRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceBundleResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceImageRequest {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    pub image_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceImageResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployWorkspaceApplicationsRequest {
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployWorkspaceApplicationsResult {
    #[serde(rename = "Deployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<WorkSpaceApplicationDeployment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkSpaceApplicationDeployment {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<WorkspaceResourceAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterWorkspaceDirectoryRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterWorkspaceDirectoryResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountModificationsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountModificationsResult {
    #[serde(rename = "AccountModifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_modifications: Option<Vec<AccountModification>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountModification {
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ModificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_state: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountResult {
    #[serde(rename = "DedicatedTenancyAccountType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_account_type: Option<String>,
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationAssociationsRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "AssociatedResourceTypes")]
    #[serde(default)]
    pub associated_resource_types: Vec<String>,
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
pub struct DescribeApplicationAssociationsResult {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<ApplicationResourceAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationResourceAssociation {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "AssociatedResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_id: Option<String>,
    #[serde(rename = "AssociatedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_type: Option<String>,
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<AssociationStateReason>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationsRequest {
    #[serde(rename = "ApplicationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<String>>,
    #[serde(rename = "ComputeTypeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type_names: Option<Vec<String>>,
    #[serde(rename = "LicenseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperatingSystemNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_names: Option<Vec<String>>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationsResult {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<WorkSpaceApplication>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkSpaceApplication {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LicenseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "SupportedComputeTypeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_compute_type_names: Option<Vec<String>>,
    #[serde(rename = "SupportedOperatingSystemNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_operating_system_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBundleAssociationsRequest {
    #[serde(rename = "AssociatedResourceTypes")]
    #[serde(default)]
    pub associated_resource_types: Vec<String>,
    #[serde(rename = "BundleId")]
    #[serde(default)]
    pub bundle_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBundleAssociationsResult {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<BundleResourceAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BundleResourceAssociation {
    #[serde(rename = "AssociatedResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_id: Option<String>,
    #[serde(rename = "AssociatedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_type: Option<String>,
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<AssociationStateReason>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClientBrandingRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClientBrandingResult {
    #[serde(rename = "DeviceTypeAndroid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_android: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeIos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_ios: Option<IosClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeLinux")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_linux: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeOsx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_osx: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeWeb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_web: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_windows: Option<DefaultClientBrandingAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultClientBrandingAttributes {
    #[serde(rename = "ForgotPasswordLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgot_password_link: Option<String>,
    #[serde(rename = "LoginMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LogoUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_link: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IosClientBrandingAttributes {
    #[serde(rename = "ForgotPasswordLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgot_password_link: Option<String>,
    #[serde(rename = "LoginMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Logo2xUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo2x_url: Option<String>,
    #[serde(rename = "Logo3xUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo3x_url: Option<String>,
    #[serde(rename = "LogoUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_link: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClientPropertiesRequest {
    #[serde(rename = "ResourceIds")]
    #[serde(default)]
    pub resource_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClientPropertiesResult {
    #[serde(rename = "ClientPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_properties_list: Option<Vec<ClientPropertiesResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientPropertiesResult {
    #[serde(rename = "ClientProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_properties: Option<ClientProperties>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientProperties {
    #[serde(rename = "LogUploadEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_upload_enabled: Option<String>,
    #[serde(rename = "ReconnectEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_enabled: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectClientAddInsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectClientAddInsResult {
    #[serde(rename = "AddIns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ins: Option<Vec<ConnectClientAddIn>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectClientAddIn {
    #[serde(rename = "AddInId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_in_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "URL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionAliasPermissionsRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
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
pub struct DescribeConnectionAliasPermissionsResult {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    #[serde(rename = "ConnectionAliasPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_alias_permissions: Option<Vec<ConnectionAliasPermission>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionAliasPermission {
    #[serde(rename = "AllowAssociation")]
    #[serde(default)]
    pub allow_association: bool,
    #[serde(rename = "SharedAccountId")]
    #[serde(default)]
    pub shared_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionAliasesRequest {
    #[serde(rename = "AliasIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_ids: Option<Vec<String>>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionAliasesResult {
    #[serde(rename = "ConnectionAliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_aliases: Option<Vec<ConnectionAlias>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionAlias {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<ConnectionAliasAssociation>>,
    #[serde(rename = "ConnectionString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionAliasAssociation {
    #[serde(rename = "AssociatedAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_account_id: Option<String>,
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "ConnectionIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_identifier: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomWorkspaceImageImportRequest {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    pub image_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomWorkspaceImageImportResult {
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<CustomWorkspaceImageImportErrorDetails>>,
    #[serde(rename = "ImageBuilderInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_builder_instance_id: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "ImageSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_source: Option<ImageSourceIdentifier>,
    #[serde(rename = "InfrastructureConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure_configuration_arn: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ProgressPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomWorkspaceImageImportErrorDetails {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageSourceIdentifier {
    #[serde(rename = "Ec2ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_image_id: Option<String>,
    #[serde(rename = "Ec2ImportTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_import_task_id: Option<String>,
    #[serde(rename = "ImageBuildVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_build_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageAssociationsRequest {
    #[serde(rename = "AssociatedResourceTypes")]
    #[serde(default)]
    pub associated_resource_types: Vec<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    pub image_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageAssociationsResult {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<ImageResourceAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageResourceAssociation {
    #[serde(rename = "AssociatedResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_id: Option<String>,
    #[serde(rename = "AssociatedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_type: Option<String>,
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<AssociationStateReason>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIpGroupsRequest {
    #[serde(rename = "GroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
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
pub struct DescribeIpGroupsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<WorkspacesIpGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspacesIpGroup {
    #[serde(rename = "groupDesc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "userRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rules: Option<Vec<IpRuleItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsResult {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceAssociationsRequest {
    #[serde(rename = "AssociatedResourceTypes")]
    #[serde(default)]
    pub associated_resource_types: Vec<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceAssociationsResult {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<WorkspaceResourceAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceBundlesRequest {
    #[serde(rename = "BundleIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceBundlesResult {
    #[serde(rename = "Bundles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<WorkspaceBundle>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceDirectoriesRequest {
    #[serde(rename = "DirectoryIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeWorkspaceDirectoriesFilter>>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspaceDirectoryNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_directory_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceDirectoriesFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceDirectoriesResult {
    #[serde(rename = "Directories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<WorkspaceDirectory>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceDirectory {
    #[serde(rename = "ActiveDirectoryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_config: Option<ActiveDirectoryConfig>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "CertificateBasedAuthProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_based_auth_properties: Option<CertificateBasedAuthProperties>,
    #[serde(rename = "CustomerUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_name: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "DirectoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<String>,
    #[serde(rename = "DirectoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<String>,
    #[serde(rename = "DnsIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "DnsIpv6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ipv6_addresses: Option<Vec<String>>,
    #[serde(rename = "EndpointEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_encryption_mode: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "IDCConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d_c_config: Option<IDCConfig>,
    #[serde(rename = "IamRoleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_id: Option<String>,
    #[serde(rename = "MicrosoftEntraConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_entra_config: Option<MicrosoftEntraConfig>,
    #[serde(rename = "RegistrationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
    #[serde(rename = "SamlProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_properties: Option<SamlProperties>,
    #[serde(rename = "SelfservicePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfservice_permissions: Option<SelfservicePermissions>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StreamingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_properties: Option<StreamingProperties>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "Tenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
    #[serde(rename = "UserIdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity_type: Option<String>,
    #[serde(rename = "WorkspaceAccessProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access_properties: Option<WorkspaceAccessProperties>,
    #[serde(rename = "WorkspaceCreationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_creation_properties: Option<DefaultWorkspaceCreationProperties>,
    #[serde(rename = "WorkspaceDirectoryDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_directory_description: Option<String>,
    #[serde(rename = "WorkspaceDirectoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_directory_name: Option<String>,
    #[serde(rename = "WorkspaceSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_security_group_id: Option<String>,
    #[serde(rename = "WorkspaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_type: Option<String>,
    #[serde(rename = "ipGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActiveDirectoryConfig {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "ServiceAccountSecretArn")]
    #[serde(default)]
    pub service_account_secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateBasedAuthProperties {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IDCConfig {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MicrosoftEntraConfig {
    #[serde(rename = "ApplicationConfigSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_config_secret_arn: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamlProperties {
    #[serde(rename = "RelayStateParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_state_parameter_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserAccessUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfservicePermissions {
    #[serde(rename = "ChangeComputeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_compute_type: Option<String>,
    #[serde(rename = "IncreaseVolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increase_volume_size: Option<String>,
    #[serde(rename = "RebuildWorkspace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild_workspace: Option<String>,
    #[serde(rename = "RestartWorkspace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_workspace: Option<String>,
    #[serde(rename = "SwitchRunningMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_running_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamingProperties {
    #[serde(rename = "GlobalAccelerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_accelerator: Option<GlobalAcceleratorForDirectory>,
    #[serde(rename = "StorageConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_connectors: Option<Vec<StorageConnector>>,
    #[serde(rename = "StreamingExperiencePreferredProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_experience_preferred_protocol: Option<String>,
    #[serde(rename = "UserSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<Vec<UserSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalAcceleratorForDirectory {
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
    #[serde(rename = "PreferredProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageConnector {
    #[serde(rename = "ConnectorType")]
    #[serde(default)]
    pub connector_type: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSetting {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "MaximumLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i32>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    pub permission: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceAccessProperties {
    #[serde(rename = "AccessEndpointConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoint_config: Option<AccessEndpointConfig>,
    #[serde(rename = "DeviceTypeAndroid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_android: Option<String>,
    #[serde(rename = "DeviceTypeChromeOs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_chrome_os: Option<String>,
    #[serde(rename = "DeviceTypeIos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_ios: Option<String>,
    #[serde(rename = "DeviceTypeLinux")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_linux: Option<String>,
    #[serde(rename = "DeviceTypeOsx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_osx: Option<String>,
    #[serde(rename = "DeviceTypeWeb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_web: Option<String>,
    #[serde(rename = "DeviceTypeWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_windows: Option<String>,
    #[serde(rename = "DeviceTypeWorkSpacesThinClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_work_spaces_thin_client: Option<String>,
    #[serde(rename = "DeviceTypeZeroClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_zero_client: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessEndpointConfig {
    #[serde(rename = "AccessEndpoints")]
    #[serde(default)]
    pub access_endpoints: Vec<AccessEndpoint>,
    #[serde(rename = "InternetFallbackProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_fallback_protocols: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessEndpoint {
    #[serde(rename = "AccessEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_endpoint_type: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultWorkspaceCreationProperties {
    #[serde(rename = "CustomSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_id: Option<String>,
    #[serde(rename = "DefaultOu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ou: Option<String>,
    #[serde(rename = "EnableInternetAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_internet_access: Option<bool>,
    #[serde(rename = "EnableMaintenanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_maintenance_mode: Option<bool>,
    #[serde(rename = "InstanceIamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_iam_role_arn: Option<String>,
    #[serde(rename = "UserEnabledAsLocalAdministrator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_enabled_as_local_administrator: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceImagePermissionsRequest {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    pub image_id: String,
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
pub struct DescribeWorkspaceImagePermissionsResult {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "ImagePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_permissions: Option<Vec<ImagePermission>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImagePermission {
    #[serde(rename = "SharedAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceImagesRequest {
    #[serde(rename = "ImageIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(rename = "ImageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
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
pub struct DescribeWorkspaceImagesResult {
    #[serde(rename = "Images")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<WorkspaceImage>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceImage {
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetails>>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystem>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "RequiredTenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_tenancy: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Updates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<UpdateResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetails {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResult {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "UpdateAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceSnapshotsRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceSnapshotsResult {
    #[serde(rename = "RebuildSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild_snapshots: Option<Vec<Snapshot>>,
    #[serde(rename = "RestoreSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_snapshots: Option<Vec<Snapshot>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Snapshot {
    #[serde(rename = "SnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesConnectionStatusRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesConnectionStatusResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspacesConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_connection_status: Option<Vec<WorkspaceConnectionStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceConnectionStatus {
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "ConnectionStateCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state_check_timestamp: Option<f64>,
    #[serde(rename = "LastKnownUserConnectionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_user_connection_timestamp: Option<f64>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesPoolSessionsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PoolId")]
    #[serde(default)]
    pub pool_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesPoolSessionsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<WorkspacesPoolSession>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspacesPoolSession {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "NetworkAccessConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access_configuration: Option<NetworkAccessConfiguration>,
    #[serde(rename = "PoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkAccessConfiguration {
    #[serde(rename = "EniId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    #[serde(rename = "EniPrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_private_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesPoolsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeWorkspacesPoolsFilter>>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PoolIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesPoolsFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesPoolsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspacesPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_pools: Option<Vec<WorkspacesPool>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "WorkspaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids: Option<Vec<String>>,
    #[serde(rename = "WorkspaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspacesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Workspaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<Workspace>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConnectionAliasRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConnectionAliasResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateIpGroupsRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "GroupIds")]
    #[serde(default)]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateIpGroupsResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateWorkspaceApplicationRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateWorkspaceApplicationResult {
    #[serde(rename = "Association")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<WorkspaceResourceAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountLinkRequest {
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "LinkedAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountLinkResult {
    #[serde(rename = "AccountLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link: Option<AccountLink>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportClientBrandingRequest {
    #[serde(rename = "DeviceTypeAndroid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_android: Option<DefaultImportClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeIos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_ios: Option<IosImportClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeLinux")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_linux: Option<DefaultImportClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeOsx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_osx: Option<DefaultImportClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeWeb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_web: Option<DefaultImportClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_windows: Option<DefaultImportClientBrandingAttributes>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultImportClientBrandingAttributes {
    #[serde(rename = "ForgotPasswordLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgot_password_link: Option<String>,
    #[serde(rename = "LoginMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Logo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_link: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IosImportClientBrandingAttributes {
    #[serde(rename = "ForgotPasswordLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgot_password_link: Option<String>,
    #[serde(rename = "LoginMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_message: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Logo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "Logo2x")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo2x: Option<String>,
    #[serde(rename = "Logo3x")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo3x: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_link: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportClientBrandingResult {
    #[serde(rename = "DeviceTypeAndroid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_android: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeIos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_ios: Option<IosClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeLinux")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_linux: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeOsx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_osx: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeWeb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_web: Option<DefaultClientBrandingAttributes>,
    #[serde(rename = "DeviceTypeWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type_windows: Option<DefaultClientBrandingAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCustomWorkspaceImageRequest {
    #[serde(rename = "ComputeType")]
    #[serde(default)]
    pub compute_type: String,
    #[serde(rename = "ImageDescription")]
    #[serde(default)]
    pub image_description: String,
    #[serde(rename = "ImageName")]
    #[serde(default)]
    pub image_name: String,
    #[serde(rename = "ImageSource")]
    #[serde(default)]
    pub image_source: ImageSourceIdentifier,
    #[serde(rename = "InfrastructureConfigurationArn")]
    #[serde(default)]
    pub infrastructure_configuration_arn: String,
    #[serde(rename = "OsVersion")]
    #[serde(default)]
    pub os_version: String,
    #[serde(rename = "Platform")]
    #[serde(default)]
    pub platform: String,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCustomWorkspaceImageResult {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportWorkspaceImageRequest {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<String>>,
    #[serde(rename = "Ec2ImageId")]
    #[serde(default)]
    pub ec2_image_id: String,
    #[serde(rename = "ImageDescription")]
    #[serde(default)]
    pub image_description: String,
    #[serde(rename = "ImageName")]
    #[serde(default)]
    pub image_name: String,
    #[serde(rename = "IngestionProcess")]
    #[serde(default)]
    pub ingestion_process: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportWorkspaceImageResult {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountLinksRequest {
    #[serde(rename = "LinkStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_status_filter: Option<Vec<String>>,
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
pub struct ListAccountLinksResult {
    #[serde(rename = "AccountLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_links: Option<Vec<AccountLink>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableManagementCidrRangesRequest {
    #[serde(rename = "ManagementCidrRangeConstraint")]
    #[serde(default)]
    pub management_cidr_range_constraint: String,
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
pub struct ListAvailableManagementCidrRangesResult {
    #[serde(rename = "ManagementCidrRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_cidr_ranges: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MigrateWorkspaceRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    pub bundle_id: String,
    #[serde(rename = "SourceWorkspaceId")]
    #[serde(default)]
    pub source_workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MigrateWorkspaceResult {
    #[serde(rename = "SourceWorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_workspace_id: Option<String>,
    #[serde(rename = "TargetWorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyAccountRequest {
    #[serde(rename = "DedicatedTenancyManagementCidrRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_management_cidr_range: Option<String>,
    #[serde(rename = "DedicatedTenancySupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_tenancy_support: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyAccountResult {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyCertificateBasedAuthPropertiesRequest {
    #[serde(rename = "CertificateBasedAuthProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_based_auth_properties: Option<CertificateBasedAuthProperties>,
    #[serde(rename = "PropertiesToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties_to_delete: Option<Vec<String>>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyCertificateBasedAuthPropertiesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyClientPropertiesRequest {
    #[serde(rename = "ClientProperties")]
    #[serde(default)]
    pub client_properties: ClientProperties,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyClientPropertiesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyEndpointEncryptionModeRequest {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    pub directory_id: String,
    #[serde(rename = "EndpointEncryptionMode")]
    #[serde(default)]
    pub endpoint_encryption_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyEndpointEncryptionModeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifySamlPropertiesRequest {
    #[serde(rename = "PropertiesToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties_to_delete: Option<Vec<String>>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "SamlProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_properties: Option<SamlProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifySamlPropertiesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifySelfservicePermissionsRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "SelfservicePermissions")]
    #[serde(default)]
    pub selfservice_permissions: SelfservicePermissions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifySelfservicePermissionsResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyStreamingPropertiesRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "StreamingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_properties: Option<StreamingProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyStreamingPropertiesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspaceAccessPropertiesRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "WorkspaceAccessProperties")]
    #[serde(default)]
    pub workspace_access_properties: WorkspaceAccessProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspaceAccessPropertiesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspaceCreationPropertiesRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "WorkspaceCreationProperties")]
    #[serde(default)]
    pub workspace_creation_properties: WorkspaceCreationProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceCreationProperties {
    #[serde(rename = "CustomSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_id: Option<String>,
    #[serde(rename = "DefaultOu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ou: Option<String>,
    #[serde(rename = "EnableInternetAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_internet_access: Option<bool>,
    #[serde(rename = "EnableMaintenanceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_maintenance_mode: Option<bool>,
    #[serde(rename = "InstanceIamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_iam_role_arn: Option<String>,
    #[serde(rename = "UserEnabledAsLocalAdministrator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_enabled_as_local_administrator: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspaceCreationPropertiesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspacePropertiesRequest {
    #[serde(rename = "DataReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
    #[serde(rename = "WorkspaceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_properties: Option<WorkspaceProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspacePropertiesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspaceStateRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
    #[serde(rename = "WorkspaceState")]
    #[serde(default)]
    pub workspace_state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyWorkspaceStateResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootWorkspacesRequest {
    #[serde(rename = "RebootWorkspaceRequests")]
    #[serde(default)]
    pub reboot_workspace_requests: Vec<RebootRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootWorkspacesResult {
    #[serde(rename = "FailedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedWorkspaceChangeRequest {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebuildWorkspacesRequest {
    #[serde(rename = "RebuildWorkspaceRequests")]
    #[serde(default)]
    pub rebuild_workspace_requests: Vec<RebuildRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebuildRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebuildWorkspacesResult {
    #[serde(rename = "FailedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterWorkspaceDirectoryRequest {
    #[serde(rename = "ActiveDirectoryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_config: Option<ActiveDirectoryConfig>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "EnableSelfService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_self_service: Option<bool>,
    #[serde(rename = "IdcInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_instance_arn: Option<String>,
    #[serde(rename = "MicrosoftEntraConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_entra_config: Option<MicrosoftEntraConfig>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Tenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
    #[serde(rename = "UserIdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity_type: Option<String>,
    #[serde(rename = "WorkspaceDirectoryDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_directory_description: Option<String>,
    #[serde(rename = "WorkspaceDirectoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_directory_name: Option<String>,
    #[serde(rename = "WorkspaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterWorkspaceDirectoryResult {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectAccountLinkInvitationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    pub link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectAccountLinkInvitationResult {
    #[serde(rename = "AccountLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_link: Option<AccountLink>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreWorkspaceRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreWorkspaceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeIpRulesRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "UserRules")]
    #[serde(default)]
    pub user_rules: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeIpRulesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWorkspacesPoolRequest {
    #[serde(rename = "PoolId")]
    #[serde(default)]
    pub pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWorkspacesPoolResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWorkspacesRequest {
    #[serde(rename = "StartWorkspaceRequests")]
    #[serde(default)]
    pub start_workspace_requests: Vec<StartRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWorkspacesResult {
    #[serde(rename = "FailedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopWorkspacesPoolRequest {
    #[serde(rename = "PoolId")]
    #[serde(default)]
    pub pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopWorkspacesPoolResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopWorkspacesRequest {
    #[serde(rename = "StopWorkspaceRequests")]
    #[serde(default)]
    pub stop_workspace_requests: Vec<StopRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopWorkspacesResult {
    #[serde(rename = "FailedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateWorkspacesPoolRequest {
    #[serde(rename = "PoolId")]
    #[serde(default)]
    pub pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateWorkspacesPoolResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateWorkspacesPoolSessionRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateWorkspacesPoolSessionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateWorkspacesRequest {
    #[serde(rename = "TerminateWorkspaceRequests")]
    #[serde(default)]
    pub terminate_workspace_requests: Vec<TerminateRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateRequest {
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateWorkspacesResult {
    #[serde(rename = "FailedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedWorkspaceChangeRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectClientAddInRequest {
    #[serde(rename = "AddInId")]
    #[serde(default)]
    pub add_in_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "URL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectClientAddInResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionAliasPermissionRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
    #[serde(rename = "ConnectionAliasPermission")]
    #[serde(default)]
    pub connection_alias_permission: ConnectionAliasPermission,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionAliasPermissionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRulesOfIpGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "UserRules")]
    #[serde(default)]
    pub user_rules: Vec<IpRuleItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRulesOfIpGroupResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceBundleRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceBundleResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceImagePermissionRequest {
    #[serde(rename = "AllowCopyImage")]
    #[serde(default)]
    pub allow_copy_image: bool,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    pub image_id: String,
    #[serde(rename = "SharedAccountId")]
    #[serde(default)]
    pub shared_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceImagePermissionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspacesPoolRequest {
    #[serde(rename = "ApplicationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings: Option<ApplicationSettingsRequest>,
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<Capacity>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "PoolId")]
    #[serde(default)]
    pub pool_id: String,
    #[serde(rename = "RunningMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_mode: Option<String>,
    #[serde(rename = "TimeoutSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_settings: Option<TimeoutSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspacesPoolResult {
    #[serde(rename = "WorkspacesPool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces_pool: Option<WorkspacesPool>,
}
