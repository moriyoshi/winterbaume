//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ram

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptResourceShareInvitationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShareInvitationArn")]
    #[serde(default)]
    pub resource_share_invitation_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptResourceShareInvitationResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShareInvitation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation: Option<ResourceShareInvitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceShareInvitation {
    #[serde(rename = "invitationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_timestamp: Option<f64>,
    #[serde(rename = "receiverAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_account_id: Option<String>,
    #[serde(rename = "receiverArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_arn: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "resourceShareAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
    #[serde(rename = "resourceShareInvitationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation_arn: Option<String>,
    #[serde(rename = "resourceShareName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_name: Option<String>,
    #[serde(rename = "senderAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceShareAssociation {
    #[serde(rename = "associatedEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_entity: Option<String>,
    #[serde(rename = "associationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "resourceShareName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceSharePermissionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
    #[serde(rename = "permissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_version: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceSharePermissionResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceShareRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceShareResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShareAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePermissionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "policyTemplate")]
    #[serde(default)]
    pub policy_template: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePermissionResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<ResourceSharePermissionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceSharePermissionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "defaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    #[serde(rename = "featureSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    #[serde(rename = "isResourceTypeDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resource_type_default: Option<bool>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "permissionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePermissionVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
    #[serde(rename = "policyTemplate")]
    #[serde(default)]
    pub policy_template: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePermissionVersionResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<ResourceSharePermissionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceSharePermissionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "defaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    #[serde(rename = "featureSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    #[serde(rename = "isResourceTypeDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resource_type_default: Option<bool>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "permissionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceShareRequest {
    #[serde(rename = "allowExternalPrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "permissionArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceShareConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_configuration: Option<ResourceShareConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceShareConfiguration {
    #[serde(rename = "retainSharingOnAccountLeaveOrganization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_sharing_on_account_leave_organization: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceShareResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShare")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share: Option<ResourceShare>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceShare {
    #[serde(rename = "allowExternalPrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "featureSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owningAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_account_id: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "resourceShareConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_configuration: Option<ResourceShareConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_status: Option<String>,
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
    #[serde(rename = "permissionVersion")]
    #[serde(default)]
    pub permission_version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionVersionResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_status: Option<String>,
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceShareRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceShareResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceSharePermissionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceSharePermissionResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceShareRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceShareResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShareAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSharingWithAwsOrganizationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSharingWithAwsOrganizationResponse {
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPermissionRequest {
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
    #[serde(rename = "permissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPermissionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<ResourceSharePermissionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePoliciesRequest {
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
    pub principal: Option<String>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    pub resource_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceShareAssociationsRequest {
    #[serde(rename = "associationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "associationType")]
    #[serde(default)]
    pub association_type: String,
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
    pub principal: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceShareArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceShareAssociationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceShareAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_associations: Option<Vec<ResourceShareAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceShareInvitationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceShareArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    #[serde(rename = "resourceShareInvitationArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceShareInvitationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceShareInvitations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitations: Option<Vec<ResourceShareInvitation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceSharesRequest {
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
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_arn: Option<String>,
    #[serde(rename = "permissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_version: Option<i32>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    pub resource_owner: String,
    #[serde(rename = "resourceShareArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    #[serde(rename = "resourceShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_status: Option<String>,
    #[serde(rename = "tagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagFilter {
    #[serde(rename = "tagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "tagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceSharesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceShares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_shares: Option<Vec<ResourceShare>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPendingInvitationResourcesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceRegionScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region_scope: Option<String>,
    #[serde(rename = "resourceShareInvitationArn")]
    #[serde(default)]
    pub resource_share_invitation_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPendingInvitationResourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "resourceGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_arn: Option<String>,
    #[serde(rename = "resourceRegionScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region_scope: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionAssociationsRequest {
    #[serde(rename = "associationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "defaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    #[serde(rename = "featureSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_arn: Option<String>,
    #[serde(rename = "permissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_version: Option<i32>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionAssociationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<AssociatedPermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedPermission {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "defaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<bool>,
    #[serde(rename = "featureSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "permissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_version: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionVersionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionVersionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourceSharePermissionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "permissionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourceSharePermissionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalsRequest {
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
    pub principals: Option<Vec<String>>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    pub resource_owner: String,
    #[serde(rename = "resourceShareArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<Principal>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Principal {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReplacePermissionAssociationsWorkRequest {
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
    pub status: Option<String>,
    #[serde(rename = "workIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReplacePermissionAssociationsWorkResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replacePermissionAssociationsWorks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_permission_associations_works: Option<Vec<ReplacePermissionAssociationsWork>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplacePermissionAssociationsWork {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "fromPermissionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_permission_arn: Option<String>,
    #[serde(rename = "fromPermissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_permission_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "toPermissionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_permission_arn: Option<String>,
    #[serde(rename = "toPermissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_permission_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceSharePermissionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceSharePermissionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourceSharePermissionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceTypesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceRegionScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceTypesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<ServiceNameAndResourceType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNameAndResourceType {
    #[serde(rename = "resourceRegionScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region_scope: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesRequest {
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
    pub principal: Option<String>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    pub resource_owner: String,
    #[serde(rename = "resourceRegionScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region_scope: Option<String>,
    #[serde(rename = "resourceShareArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourceAssociationsRequest {
    #[serde(rename = "associationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceShareArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arns: Option<Vec<String>>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourceAssociationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sourceAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_associations: Option<Vec<AssociatedSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedSource {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromotePermissionCreatedFromPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromotePermissionCreatedFromPolicyResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<ResourceSharePermissionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromoteResourceShareCreatedFromPolicyRequest {
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromoteResourceShareCreatedFromPolicyResponse {
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectResourceShareInvitationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShareInvitationArn")]
    #[serde(default)]
    pub resource_share_invitation_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectResourceShareInvitationResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShareInvitation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_invitation: Option<ResourceShareInvitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplacePermissionAssociationsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "fromPermissionArn")]
    #[serde(default)]
    pub from_permission_arn: String,
    #[serde(rename = "fromPermissionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_permission_version: Option<i32>,
    #[serde(rename = "toPermissionArn")]
    #[serde(default)]
    pub to_permission_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplacePermissionAssociationsResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "replacePermissionAssociationsWork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_permission_associations_work: Option<ReplacePermissionAssociationsWork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetDefaultPermissionVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "permissionArn")]
    #[serde(default)]
    pub permission_arn: String,
    #[serde(rename = "permissionVersion")]
    #[serde(default)]
    pub permission_version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetDefaultPermissionVersionResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "returnValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceShareRequest {
    #[serde(rename = "allowExternalPrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_principals: Option<bool>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceShareArn")]
    #[serde(default)]
    pub resource_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceShareResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceShare")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share: Option<ResourceShare>,
}
