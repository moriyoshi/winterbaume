//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-organizations

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRootsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Roots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<Vec<Root>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PolicyTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_types: Option<Vec<PolicyTypeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyTypeSummary {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
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
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHandshakeRequest {
    #[serde(rename = "HandshakeId")]
    #[serde(default)]
    pub handshake_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsResponse {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JoinedMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_method: Option<String>,
    #[serde(rename = "JoinedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResponsibilityTransferResponse {
    #[serde(rename = "ResponsibilityTransfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility_transfer: Option<ResponsibilityTransfer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponsibilityTransfer {
    #[serde(rename = "ActiveHandshakeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_handshake_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransferParticipant>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TransferParticipant>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferParticipant {
    #[serde(rename = "ManagementAccountEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_account_email: Option<String>,
    #[serde(rename = "ManagementAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHandshakesForAccountRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
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
pub struct HandshakeFilter {
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "ParentHandshakeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_handshake_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDelegatedAdministratorRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    pub service_principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChildrenResponse {
    #[serde(rename = "Children")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Child>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Child {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationalUnitResponse {
    #[serde(rename = "OrganizationalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationalUnit {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
pub struct UpdatePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Policy {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "PolicySummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_summary: Option<PolicySummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicySummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AwsManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_managed: Option<bool>,
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
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResponsibilityTransferRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsWithInvalidEffectivePolicyRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    pub policy_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPoliciesRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    pub filter: String,
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
pub struct TerminateResponsibilityTransferRequest {
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListParentsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Parents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parent {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGovCloudAccountResponse {
    #[serde(rename = "CreateAccountStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountStatus {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "CompletedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "GovCloudAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gov_cloud_account_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "RequestedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResponsibilityTransferResponse {
    #[serde(rename = "ResponsibilityTransfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility_transfer: Option<ResponsibilityTransfer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDelegatedServicesForAccountResponse {
    #[serde(rename = "DelegatedServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_services: Option<Vec<DelegatedService>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DelegatedService {
    #[serde(rename = "DelegationEnabledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_enabled_date: Option<f64>,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOrganizationalUnitResponse {
    #[serde(rename = "OrganizationalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChildrenRequest {
    #[serde(rename = "ChildType")]
    #[serde(default)]
    pub child_type: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    pub parent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePolicyRequest {
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDelegatedAdministratorsResponse {
    #[serde(rename = "DelegatedAdministrators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_administrators: Option<Vec<DelegatedAdministrator>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DelegatedAdministrator {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DelegationEnabledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_enabled_date: Option<f64>,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JoinedMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_method: Option<String>,
    #[serde(rename = "JoinedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInboundResponsibilityTransfersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResponsibilityTransfers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility_transfers: Option<Vec<ResponsibilityTransfer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationalUnitRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    pub organizational_unit_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAWSServiceAccessForOrganizationResponse {
    #[serde(rename = "EnabledServicePrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_service_principals: Option<Vec<EnabledServicePrincipal>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnabledServicePrincipal {
    #[serde(rename = "DateEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_enabled: Option<f64>,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCreateAccountStatusResponse {
    #[serde(rename = "CreateAccountStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloseAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOrganizationalUnitRequest {
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    pub organizational_unit_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsForParentResponse {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateResponsibilityTransferResponse {
    #[serde(rename = "ResponsibilityTransfer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility_transfer: Option<ResponsibilityTransfer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteAccountToOrganizationResponse {
    #[serde(rename = "Handshake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Handshake {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ExpirationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Parties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<HandshakeParty>>,
    #[serde(rename = "RequestedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HandshakeParty {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HandshakeResource {
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
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
pub struct CreateOrganizationalUnitRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    pub parent_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDelegatedServicesForAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
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
pub struct ListHandshakesForAccountResponse {
    #[serde(rename = "Handshakes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelHandshakeRequest {
    #[serde(rename = "HandshakeId")]
    #[serde(default)]
    pub handshake_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEffectivePolicyRequest {
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    pub policy_type: String,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEffectivePolicyResponse {
    #[serde(rename = "EffectivePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_policy: Option<EffectivePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectivePolicy {
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "PolicyContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_content: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOrganizationResponse {
    #[serde(rename = "Organization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Organization {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailablePolicyTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_policy_types: Option<Vec<PolicyTypeSummary>>,
    #[serde(rename = "FeatureSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MasterAccountArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_arn: Option<String>,
    #[serde(rename = "MasterAccountEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_email: Option<String>,
    #[serde(rename = "MasterAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListParentsRequest {
    #[serde(rename = "ChildId")]
    #[serde(default)]
    pub child_id: String,
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
pub struct ListTargetsForPolicyRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MoveAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "DestinationParentId")]
    #[serde(default)]
    pub destination_parent_id: String,
    #[serde(rename = "SourceParentId")]
    #[serde(default)]
    pub source_parent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountResponse {
    #[serde(rename = "CreateAccountStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnablePolicyTypeRequest {
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    pub policy_type: String,
    #[serde(rename = "RootId")]
    #[serde(default)]
    pub root_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCreateAccountStatusRequest {
    #[serde(rename = "CreateAccountRequestId")]
    #[serde(default)]
    pub create_account_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptHandshakeResponse {
    #[serde(rename = "Handshake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisablePolicyTypeResponse {
    #[serde(rename = "Root")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInboundResponsibilityTransfersRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableAWSServiceAccessRequest {
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    pub service_principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationResponse {
    #[serde(rename = "Organization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationalUnitsForParentResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationalUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_units: Option<Vec<OrganizationalUnit>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelHandshakeResponse {
    #[serde(rename = "Handshake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePolicyRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
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
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsWithInvalidEffectivePolicyResponse {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationalUnitsForParentRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    pub parent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptHandshakeRequest {
    #[serde(rename = "HandshakeId")]
    #[serde(default)]
    pub handshake_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcePolicy {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ResourcePolicySummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy_summary: Option<ResourcePolicySummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcePolicySummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDelegatedAdministratorsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePolicyRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsForPolicyResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<PolicyTargetSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyTargetSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableAWSServiceAccessRequest {
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    pub service_principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteOrganizationToTransferResponsibilityRequest {
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "SourceName")]
    #[serde(default)]
    pub source_name: String,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    pub start_timestamp: f64,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: HandshakeParty,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAccountFromOrganizationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineHandshakeRequest {
    #[serde(rename = "HandshakeId")]
    #[serde(default)]
    pub handshake_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteAccountToOrganizationRequest {
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: HandshakeParty,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHandshakeResponse {
    #[serde(rename = "Handshake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCreateAccountStatusRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "States")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCreateAccountStatusResponse {
    #[serde(rename = "CreateAccountStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_statuses: Option<Vec<CreateAccountStatus>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePolicyRequest {
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResponsibilityTransferRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRootsRequest {
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
pub struct DescribeOrganizationalUnitRequest {
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    pub organizational_unit_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableAllFeaturesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableAllFeaturesResponse {
    #[serde(rename = "Handshake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutboundResponsibilityTransfersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResponsibilityTransfers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility_transfers: Option<Vec<ResponsibilityTransfer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOrganizationRequest {
    #[serde(rename = "FeatureSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAWSServiceAccessForOrganizationRequest {
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
pub struct AttachPolicyRequest {
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    pub target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsForParentRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    pub parent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHandshakesForOrganizationResponse {
    #[serde(rename = "Handshakes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutboundResponsibilityTransfersRequest {
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
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineHandshakeResponse {
    #[serde(rename = "Handshake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountRequest {
    #[serde(rename = "AccountName")]
    #[serde(default)]
    pub account_name: String,
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,
    #[serde(rename = "IamUserAccessToBilling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_access_to_billing: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountResponse {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyResponse {
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisablePolicyTypeRequest {
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    pub policy_type: String,
    #[serde(rename = "RootId")]
    #[serde(default)]
    pub root_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEffectivePolicyValidationErrorsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    pub policy_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEffectivePolicyValidationErrorsResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "EffectivePolicyValidationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_policy_validation_errors: Option<Vec<EffectivePolicyValidationError>>,
    #[serde(rename = "EvaluationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_timestamp: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectivePolicyValidationError {
    #[serde(rename = "ContributingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributing_policies: Option<Vec<String>>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "PathToError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_to_error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHandshakesForOrganizationRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
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
pub struct DescribeOrganizationalUnitResponse {
    #[serde(rename = "OrganizationalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteOrganizationToTransferResponsibilityResponse {
    #[serde(rename = "Handshake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterDelegatedAdministratorRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    pub service_principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPoliciesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGovCloudAccountRequest {
    #[serde(rename = "AccountName")]
    #[serde(default)]
    pub account_name: String,
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,
    #[serde(rename = "IamUserAccessToBilling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_access_to_billing: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPoliciesForTargetRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    pub filter: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    pub target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsRequest {
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
pub struct DetachPolicyRequest {
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    pub target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnablePolicyTypeResponse {
    #[serde(rename = "Root")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPoliciesForTargetResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}
