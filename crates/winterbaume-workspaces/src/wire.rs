//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-workspaces

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_accept_account_link_invitation_response(
    result: &AcceptAccountLinkInvitationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_connection_alias_response(
    result: &AssociateConnectionAliasResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_ip_groups_response(result: &AssociateIpGroupsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_workspace_application_response(
    result: &AssociateWorkspaceApplicationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_authorize_ip_rules_response(result: &AuthorizeIpRulesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_copy_workspace_image_response(result: &CopyWorkspaceImageResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_account_link_invitation_response(
    result: &CreateAccountLinkInvitationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_connect_client_add_in_response(
    result: &CreateConnectClientAddInResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_connection_alias_response(
    result: &CreateConnectionAliasResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_ip_group_response(result: &CreateIpGroupResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_standby_workspaces_response(
    result: &CreateStandbyWorkspacesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_tags_response(result: &CreateTagsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_updated_workspace_image_response(
    result: &CreateUpdatedWorkspaceImageResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_workspace_bundle_response(
    result: &CreateWorkspaceBundleResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_workspace_image_response(
    result: &CreateWorkspaceImageResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_workspaces_response(result: &CreateWorkspacesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_workspaces_pool_response(
    result: &CreateWorkspacesPoolResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_account_link_invitation_response(
    result: &DeleteAccountLinkInvitationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_client_branding_response(
    result: &DeleteClientBrandingResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_connect_client_add_in_response(
    result: &DeleteConnectClientAddInResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_connection_alias_response(
    result: &DeleteConnectionAliasResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_ip_group_response(result: &DeleteIpGroupResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_tags_response(result: &DeleteTagsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_workspace_bundle_response(
    result: &DeleteWorkspaceBundleResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_workspace_image_response(
    result: &DeleteWorkspaceImageResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deploy_workspace_applications_response(
    result: &DeployWorkspaceApplicationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_workspace_directory_response(
    result: &DeregisterWorkspaceDirectoryResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_response(result: &DescribeAccountResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_modifications_response(
    result: &DescribeAccountModificationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_associations_response(
    result: &DescribeApplicationAssociationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_applications_response(
    result: &DescribeApplicationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_bundle_associations_response(
    result: &DescribeBundleAssociationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_client_branding_response(
    result: &DescribeClientBrandingResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_client_properties_response(
    result: &DescribeClientPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connect_client_add_ins_response(
    result: &DescribeConnectClientAddInsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connection_alias_permissions_response(
    result: &DescribeConnectionAliasPermissionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connection_aliases_response(
    result: &DescribeConnectionAliasesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_custom_workspace_image_import_response(
    result: &DescribeCustomWorkspaceImageImportResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_image_associations_response(
    result: &DescribeImageAssociationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_ip_groups_response(result: &DescribeIpGroupsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_tags_response(result: &DescribeTagsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspace_associations_response(
    result: &DescribeWorkspaceAssociationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspace_bundles_response(
    result: &DescribeWorkspaceBundlesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspace_directories_response(
    result: &DescribeWorkspaceDirectoriesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspace_image_permissions_response(
    result: &DescribeWorkspaceImagePermissionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspace_images_response(
    result: &DescribeWorkspaceImagesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspace_snapshots_response(
    result: &DescribeWorkspaceSnapshotsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspaces_response(result: &DescribeWorkspacesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspaces_connection_status_response(
    result: &DescribeWorkspacesConnectionStatusResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspaces_pool_sessions_response(
    result: &DescribeWorkspacesPoolSessionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_workspaces_pools_response(
    result: &DescribeWorkspacesPoolsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_connection_alias_response(
    result: &DisassociateConnectionAliasResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_ip_groups_response(
    result: &DisassociateIpGroupsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_workspace_application_response(
    result: &DisassociateWorkspaceApplicationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_account_link_response(result: &GetAccountLinkResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_client_branding_response(
    result: &ImportClientBrandingResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_custom_workspace_image_response(
    result: &ImportCustomWorkspaceImageResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_workspace_image_response(
    result: &ImportWorkspaceImageResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_account_links_response(result: &ListAccountLinksResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_available_management_cidr_ranges_response(
    result: &ListAvailableManagementCidrRangesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_migrate_workspace_response(result: &MigrateWorkspaceResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_account_response(result: &ModifyAccountResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_certificate_based_auth_properties_response(
    result: &ModifyCertificateBasedAuthPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_client_properties_response(
    result: &ModifyClientPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_endpoint_encryption_mode_response(
    result: &ModifyEndpointEncryptionModeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_saml_properties_response(
    result: &ModifySamlPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_selfservice_permissions_response(
    result: &ModifySelfservicePermissionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_streaming_properties_response(
    result: &ModifyStreamingPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_workspace_access_properties_response(
    result: &ModifyWorkspaceAccessPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_workspace_creation_properties_response(
    result: &ModifyWorkspaceCreationPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_workspace_properties_response(
    result: &ModifyWorkspacePropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_workspace_state_response(
    result: &ModifyWorkspaceStateResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reboot_workspaces_response(result: &RebootWorkspacesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_rebuild_workspaces_response(result: &RebuildWorkspacesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_workspace_directory_response(
    result: &RegisterWorkspaceDirectoryResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reject_account_link_invitation_response(
    result: &RejectAccountLinkInvitationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_workspace_response(result: &RestoreWorkspaceResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_revoke_ip_rules_response(result: &RevokeIpRulesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_workspaces_response(result: &StartWorkspacesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_workspaces_pool_response(
    result: &StartWorkspacesPoolResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_workspaces_response(result: &StopWorkspacesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_workspaces_pool_response(result: &StopWorkspacesPoolResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_terminate_workspaces_response(result: &TerminateWorkspacesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_terminate_workspaces_pool_response(
    result: &TerminateWorkspacesPoolResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_terminate_workspaces_pool_session_response(
    result: &TerminateWorkspacesPoolSessionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_connect_client_add_in_response(
    result: &UpdateConnectClientAddInResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_connection_alias_permission_response(
    result: &UpdateConnectionAliasPermissionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_rules_of_ip_group_response(
    result: &UpdateRulesOfIpGroupResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_workspace_bundle_response(
    result: &UpdateWorkspaceBundleResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_workspace_image_permission_response(
    result: &UpdateWorkspaceImagePermissionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_workspaces_pool_response(
    result: &UpdateWorkspacesPoolResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_accept_account_link_invitation_request(
    body: &[u8],
) -> Result<AcceptAccountLinkInvitationRequest, String> {
    if body.is_empty() {
        return Ok(AcceptAccountLinkInvitationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AcceptAccountLinkInvitation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_connection_alias_request(
    body: &[u8],
) -> Result<AssociateConnectionAliasRequest, String> {
    if body.is_empty() {
        return Ok(AssociateConnectionAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateConnectionAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_ip_groups_request(
    body: &[u8],
) -> Result<AssociateIpGroupsRequest, String> {
    if body.is_empty() {
        return Ok(AssociateIpGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateIpGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_workspace_application_request(
    body: &[u8],
) -> Result<AssociateWorkspaceApplicationRequest, String> {
    if body.is_empty() {
        return Ok(AssociateWorkspaceApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateWorkspaceApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_authorize_ip_rules_request(
    body: &[u8],
) -> Result<AuthorizeIpRulesRequest, String> {
    if body.is_empty() {
        return Ok(AuthorizeIpRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AuthorizeIpRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_copy_workspace_image_request(
    body: &[u8],
) -> Result<CopyWorkspaceImageRequest, String> {
    if body.is_empty() {
        return Ok(CopyWorkspaceImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CopyWorkspaceImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_account_link_invitation_request(
    body: &[u8],
) -> Result<CreateAccountLinkInvitationRequest, String> {
    if body.is_empty() {
        return Ok(CreateAccountLinkInvitationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAccountLinkInvitation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_connect_client_add_in_request(
    body: &[u8],
) -> Result<CreateConnectClientAddInRequest, String> {
    if body.is_empty() {
        return Ok(CreateConnectClientAddInRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConnectClientAddIn request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_connection_alias_request(
    body: &[u8],
) -> Result<CreateConnectionAliasRequest, String> {
    if body.is_empty() {
        return Ok(CreateConnectionAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConnectionAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_ip_group_request(body: &[u8]) -> Result<CreateIpGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateIpGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateIpGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_standby_workspaces_request(
    body: &[u8],
) -> Result<CreateStandbyWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(CreateStandbyWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStandbyWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_tags_request(body: &[u8]) -> Result<CreateTagsRequest, String> {
    if body.is_empty() {
        return Ok(CreateTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_updated_workspace_image_request(
    body: &[u8],
) -> Result<CreateUpdatedWorkspaceImageRequest, String> {
    if body.is_empty() {
        return Ok(CreateUpdatedWorkspaceImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUpdatedWorkspaceImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_workspace_bundle_request(
    body: &[u8],
) -> Result<CreateWorkspaceBundleRequest, String> {
    if body.is_empty() {
        return Ok(CreateWorkspaceBundleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWorkspaceBundle request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_workspace_image_request(
    body: &[u8],
) -> Result<CreateWorkspaceImageRequest, String> {
    if body.is_empty() {
        return Ok(CreateWorkspaceImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWorkspaceImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_workspaces_request(
    body: &[u8],
) -> Result<CreateWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(CreateWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_workspaces_pool_request(
    body: &[u8],
) -> Result<CreateWorkspacesPoolRequest, String> {
    if body.is_empty() {
        return Ok(CreateWorkspacesPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWorkspacesPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_account_link_invitation_request(
    body: &[u8],
) -> Result<DeleteAccountLinkInvitationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAccountLinkInvitationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAccountLinkInvitation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_client_branding_request(
    body: &[u8],
) -> Result<DeleteClientBrandingRequest, String> {
    if body.is_empty() {
        return Ok(DeleteClientBrandingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteClientBranding request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connect_client_add_in_request(
    body: &[u8],
) -> Result<DeleteConnectClientAddInRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConnectClientAddInRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnectClientAddIn request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connection_alias_request(
    body: &[u8],
) -> Result<DeleteConnectionAliasRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConnectionAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnectionAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_ip_group_request(body: &[u8]) -> Result<DeleteIpGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIpGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIpGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_tags_request(body: &[u8]) -> Result<DeleteTagsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_workspace_bundle_request(
    body: &[u8],
) -> Result<DeleteWorkspaceBundleRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWorkspaceBundleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWorkspaceBundle request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_workspace_image_request(
    body: &[u8],
) -> Result<DeleteWorkspaceImageRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWorkspaceImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWorkspaceImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deploy_workspace_applications_request(
    body: &[u8],
) -> Result<DeployWorkspaceApplicationsRequest, String> {
    if body.is_empty() {
        return Ok(DeployWorkspaceApplicationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeployWorkspaceApplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_workspace_directory_request(
    body: &[u8],
) -> Result<DeregisterWorkspaceDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterWorkspaceDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterWorkspaceDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_request(body: &[u8]) -> Result<DescribeAccountRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_modifications_request(
    body: &[u8],
) -> Result<DescribeAccountModificationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccountModificationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccountModifications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_associations_request(
    body: &[u8],
) -> Result<DescribeApplicationAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationAssociationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplicationAssociations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_applications_request(
    body: &[u8],
) -> Result<DescribeApplicationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_bundle_associations_request(
    body: &[u8],
) -> Result<DescribeBundleAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBundleAssociationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBundleAssociations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_client_branding_request(
    body: &[u8],
) -> Result<DescribeClientBrandingRequest, String> {
    if body.is_empty() {
        return Ok(DescribeClientBrandingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeClientBranding request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_client_properties_request(
    body: &[u8],
) -> Result<DescribeClientPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeClientPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeClientProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connect_client_add_ins_request(
    body: &[u8],
) -> Result<DescribeConnectClientAddInsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectClientAddInsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnectClientAddIns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connection_alias_permissions_request(
    body: &[u8],
) -> Result<DescribeConnectionAliasPermissionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionAliasPermissionsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeConnectionAliasPermissions request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connection_aliases_request(
    body: &[u8],
) -> Result<DescribeConnectionAliasesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionAliasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnectionAliases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_custom_workspace_image_import_request(
    body: &[u8],
) -> Result<DescribeCustomWorkspaceImageImportRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCustomWorkspaceImageImportRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeCustomWorkspaceImageImport request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_image_associations_request(
    body: &[u8],
) -> Result<DescribeImageAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeImageAssociationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImageAssociations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_ip_groups_request(
    body: &[u8],
) -> Result<DescribeIpGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeIpGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeIpGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_tags_request(body: &[u8]) -> Result<DescribeTagsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspace_associations_request(
    body: &[u8],
) -> Result<DescribeWorkspaceAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspaceAssociationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspaceAssociations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspace_bundles_request(
    body: &[u8],
) -> Result<DescribeWorkspaceBundlesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspaceBundlesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspaceBundles request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspace_directories_request(
    body: &[u8],
) -> Result<DescribeWorkspaceDirectoriesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspaceDirectoriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspaceDirectories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspace_image_permissions_request(
    body: &[u8],
) -> Result<DescribeWorkspaceImagePermissionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspaceImagePermissionsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeWorkspaceImagePermissions request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspace_images_request(
    body: &[u8],
) -> Result<DescribeWorkspaceImagesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspaceImagesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspaceImages request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspace_snapshots_request(
    body: &[u8],
) -> Result<DescribeWorkspaceSnapshotsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspaceSnapshotsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspaceSnapshots request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspaces_request(
    body: &[u8],
) -> Result<DescribeWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspaces_connection_status_request(
    body: &[u8],
) -> Result<DescribeWorkspacesConnectionStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspacesConnectionStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeWorkspacesConnectionStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspaces_pool_sessions_request(
    body: &[u8],
) -> Result<DescribeWorkspacesPoolSessionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspacesPoolSessionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspacesPoolSessions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_workspaces_pools_request(
    body: &[u8],
) -> Result<DescribeWorkspacesPoolsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWorkspacesPoolsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWorkspacesPools request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_connection_alias_request(
    body: &[u8],
) -> Result<DisassociateConnectionAliasRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateConnectionAliasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateConnectionAlias request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_ip_groups_request(
    body: &[u8],
) -> Result<DisassociateIpGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateIpGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateIpGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_workspace_application_request(
    body: &[u8],
) -> Result<DisassociateWorkspaceApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateWorkspaceApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateWorkspaceApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_account_link_request(body: &[u8]) -> Result<GetAccountLinkRequest, String> {
    if body.is_empty() {
        return Ok(GetAccountLinkRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAccountLink request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_client_branding_request(
    body: &[u8],
) -> Result<ImportClientBrandingRequest, String> {
    if body.is_empty() {
        return Ok(ImportClientBrandingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportClientBranding request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_custom_workspace_image_request(
    body: &[u8],
) -> Result<ImportCustomWorkspaceImageRequest, String> {
    if body.is_empty() {
        return Ok(ImportCustomWorkspaceImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportCustomWorkspaceImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_workspace_image_request(
    body: &[u8],
) -> Result<ImportWorkspaceImageRequest, String> {
    if body.is_empty() {
        return Ok(ImportWorkspaceImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportWorkspaceImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_account_links_request(
    body: &[u8],
) -> Result<ListAccountLinksRequest, String> {
    if body.is_empty() {
        return Ok(ListAccountLinksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAccountLinks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_available_management_cidr_ranges_request(
    body: &[u8],
) -> Result<ListAvailableManagementCidrRangesRequest, String> {
    if body.is_empty() {
        return Ok(ListAvailableManagementCidrRangesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAvailableManagementCidrRanges request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_migrate_workspace_request(
    body: &[u8],
) -> Result<MigrateWorkspaceRequest, String> {
    if body.is_empty() {
        return Ok(MigrateWorkspaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MigrateWorkspace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_account_request(body: &[u8]) -> Result<ModifyAccountRequest, String> {
    if body.is_empty() {
        return Ok(ModifyAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_certificate_based_auth_properties_request(
    body: &[u8],
) -> Result<ModifyCertificateBasedAuthPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ModifyCertificateBasedAuthPropertiesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ModifyCertificateBasedAuthProperties request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_client_properties_request(
    body: &[u8],
) -> Result<ModifyClientPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ModifyClientPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyClientProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_endpoint_encryption_mode_request(
    body: &[u8],
) -> Result<ModifyEndpointEncryptionModeRequest, String> {
    if body.is_empty() {
        return Ok(ModifyEndpointEncryptionModeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyEndpointEncryptionMode request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_saml_properties_request(
    body: &[u8],
) -> Result<ModifySamlPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ModifySamlPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifySamlProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_selfservice_permissions_request(
    body: &[u8],
) -> Result<ModifySelfservicePermissionsRequest, String> {
    if body.is_empty() {
        return Ok(ModifySelfservicePermissionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifySelfservicePermissions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_streaming_properties_request(
    body: &[u8],
) -> Result<ModifyStreamingPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ModifyStreamingPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyStreamingProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_workspace_access_properties_request(
    body: &[u8],
) -> Result<ModifyWorkspaceAccessPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ModifyWorkspaceAccessPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyWorkspaceAccessProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_workspace_creation_properties_request(
    body: &[u8],
) -> Result<ModifyWorkspaceCreationPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ModifyWorkspaceCreationPropertiesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ModifyWorkspaceCreationProperties request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_workspace_properties_request(
    body: &[u8],
) -> Result<ModifyWorkspacePropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ModifyWorkspacePropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyWorkspaceProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_workspace_state_request(
    body: &[u8],
) -> Result<ModifyWorkspaceStateRequest, String> {
    if body.is_empty() {
        return Ok(ModifyWorkspaceStateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyWorkspaceState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reboot_workspaces_request(
    body: &[u8],
) -> Result<RebootWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(RebootWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RebootWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_rebuild_workspaces_request(
    body: &[u8],
) -> Result<RebuildWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(RebuildWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RebuildWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_workspace_directory_request(
    body: &[u8],
) -> Result<RegisterWorkspaceDirectoryRequest, String> {
    if body.is_empty() {
        return Ok(RegisterWorkspaceDirectoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterWorkspaceDirectory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reject_account_link_invitation_request(
    body: &[u8],
) -> Result<RejectAccountLinkInvitationRequest, String> {
    if body.is_empty() {
        return Ok(RejectAccountLinkInvitationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RejectAccountLinkInvitation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_workspace_request(
    body: &[u8],
) -> Result<RestoreWorkspaceRequest, String> {
    if body.is_empty() {
        return Ok(RestoreWorkspaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreWorkspace request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_revoke_ip_rules_request(body: &[u8]) -> Result<RevokeIpRulesRequest, String> {
    if body.is_empty() {
        return Ok(RevokeIpRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RevokeIpRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_workspaces_request(body: &[u8]) -> Result<StartWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(StartWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_workspaces_pool_request(
    body: &[u8],
) -> Result<StartWorkspacesPoolRequest, String> {
    if body.is_empty() {
        return Ok(StartWorkspacesPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartWorkspacesPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_workspaces_request(body: &[u8]) -> Result<StopWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(StopWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_workspaces_pool_request(
    body: &[u8],
) -> Result<StopWorkspacesPoolRequest, String> {
    if body.is_empty() {
        return Ok(StopWorkspacesPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopWorkspacesPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_terminate_workspaces_request(
    body: &[u8],
) -> Result<TerminateWorkspacesRequest, String> {
    if body.is_empty() {
        return Ok(TerminateWorkspacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateWorkspaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_terminate_workspaces_pool_request(
    body: &[u8],
) -> Result<TerminateWorkspacesPoolRequest, String> {
    if body.is_empty() {
        return Ok(TerminateWorkspacesPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateWorkspacesPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_terminate_workspaces_pool_session_request(
    body: &[u8],
) -> Result<TerminateWorkspacesPoolSessionRequest, String> {
    if body.is_empty() {
        return Ok(TerminateWorkspacesPoolSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateWorkspacesPoolSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_connect_client_add_in_request(
    body: &[u8],
) -> Result<UpdateConnectClientAddInRequest, String> {
    if body.is_empty() {
        return Ok(UpdateConnectClientAddInRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConnectClientAddIn request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_connection_alias_permission_request(
    body: &[u8],
) -> Result<UpdateConnectionAliasPermissionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateConnectionAliasPermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConnectionAliasPermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_rules_of_ip_group_request(
    body: &[u8],
) -> Result<UpdateRulesOfIpGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateRulesOfIpGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRulesOfIpGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_workspace_bundle_request(
    body: &[u8],
) -> Result<UpdateWorkspaceBundleRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWorkspaceBundleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWorkspaceBundle request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_workspace_image_permission_request(
    body: &[u8],
) -> Result<UpdateWorkspaceImagePermissionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWorkspaceImagePermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWorkspaceImagePermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_workspaces_pool_request(
    body: &[u8],
) -> Result<UpdateWorkspacesPoolRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWorkspacesPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWorkspacesPool request: {e}"))
}
