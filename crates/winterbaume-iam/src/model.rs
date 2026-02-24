//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-iam

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServiceLinkedRoleRequest")]
pub struct DeleteServiceLinkedRoleRequest {
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVirtualMFADevicesResult")]
pub struct ListVirtualMFADevicesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "VirtualMFADevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_m_f_a_devices: Option<virtualMFADeviceListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct virtualMFADeviceListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<VirtualMFADevice>,
}
impl From<Vec<VirtualMFADevice>> for virtualMFADeviceListType {
    fn from(v: Vec<VirtualMFADevice>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<VirtualMFADevice> for virtualMFADeviceListType {
    fn from_iter<I: IntoIterator<Item = VirtualMFADevice>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<VirtualMFADevice>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlVirtualMFADeviceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<VirtualMFADevice>,
}

impl From<Vec<VirtualMFADevice>> for XmlVirtualMFADeviceList {
    fn from(v: Vec<VirtualMFADevice>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<VirtualMFADevice> for XmlVirtualMFADeviceList {
    fn from_iter<I: IntoIterator<Item = VirtualMFADevice>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VirtualMFADevice")]
pub struct VirtualMFADevice {
    #[serde(rename = "Base32StringSeed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base32_string_seed: Option<String>,
    #[serde(rename = "EnableDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_date: Option<String>,
    #[serde(rename = "QRCodePNG")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_r_code_p_n_g: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct tagListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for tagListType {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for tagListType {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Tag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Tag>,
}

impl From<Vec<Tag>> for XmlTagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Tag> for XmlTagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tag")]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "User")]
pub struct User {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "PasswordLastUsed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_last_used: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachedPermissionsBoundary")]
pub struct AttachedPermissionsBoundary {
    #[serde(rename = "PermissionsBoundaryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_arn: Option<String>,
    #[serde(rename = "PermissionsBoundaryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOrganizationsAccessReportResult")]
pub struct GetOrganizationsAccessReportResponse {
    #[serde(rename = "AccessDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_details: Option<AccessDetails>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "JobCompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_completion_date: Option<String>,
    #[serde(rename = "JobCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_creation_date: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "NumberOfServicesAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_services_accessible: Option<i32>,
    #[serde(rename = "NumberOfServicesNotAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_services_not_accessed: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessDetails {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AccessDetail>,
}
impl From<Vec<AccessDetail>> for AccessDetails {
    fn from(v: Vec<AccessDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AccessDetail> for AccessDetails {
    fn from_iter<I: IntoIterator<Item = AccessDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AccessDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAccessDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AccessDetail>,
}

impl From<Vec<AccessDetail>> for XmlAccessDetailList {
    fn from(v: Vec<AccessDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AccessDetail> for XmlAccessDetailList {
    fn from_iter<I: IntoIterator<Item = AccessDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessDetail")]
pub struct AccessDetail {
    #[serde(rename = "EntityPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<String>,
    #[serde(rename = "LastAuthenticatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authenticated_time: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(rename = "TotalAuthenticatedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_authenticated_entities: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ErrorDetails")]
pub struct ErrorDetails {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePolicyResult")]
pub struct CreatePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Policy")]
pub struct Policy {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AttachmentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<i32>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "DefaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsAttachable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attachable: Option<bool>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundaryUsageCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_usage_count: Option<i32>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "UpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVirtualMFADevicesRequest")]
pub struct ListVirtualMFADevicesRequest {
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveClientIDFromOpenIDConnectProviderRequest")]
pub struct RemoveClientIDFromOpenIDConnectProviderRequest {
    #[serde(rename = "ClientID")]
    #[serde(default)]
    pub client_i_d: String,
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGroupPolicyRequest")]
pub struct DeleteGroupPolicyRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGroupRequest")]
pub struct CreateGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetRoleRequest")]
pub struct GetRoleRequest {
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAttachedUserPoliciesResult")]
pub struct ListAttachedUserPoliciesResponse {
    #[serde(rename = "AttachedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_policies: Option<attachedPoliciesListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct attachedPoliciesListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AttachedPolicy>,
}
impl From<Vec<AttachedPolicy>> for attachedPoliciesListType {
    fn from(v: Vec<AttachedPolicy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AttachedPolicy> for attachedPoliciesListType {
    fn from_iter<I: IntoIterator<Item = AttachedPolicy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AttachedPolicy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAttachedPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AttachedPolicy>,
}

impl From<Vec<AttachedPolicy>> for XmlAttachedPolicyList {
    fn from(v: Vec<AttachedPolicy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AttachedPolicy> for XmlAttachedPolicyList {
    fn from_iter<I: IntoIterator<Item = AttachedPolicy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachedPolicy")]
pub struct AttachedPolicy {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableOutboundWebIdentityFederationResult")]
pub struct EnableOutboundWebIdentityFederationResponse {
    #[serde(rename = "IssuerIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagInstanceProfileRequest")]
pub struct TagInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagServerCertificateRequest")]
pub struct TagServerCertificateRequest {
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    pub server_certificate_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServiceSpecificCredentialRequest")]
pub struct CreateServiceSpecificCredentialRequest {
    #[serde(rename = "CredentialAgeDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_age_days: Option<i32>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetInstanceProfileRequest")]
pub struct GetInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAccessKeyRequest")]
pub struct UpdateAccessKeyRequest {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePolicyRequest")]
pub struct CreatePolicyRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagOpenIDConnectProviderRequest")]
pub struct TagOpenIDConnectProviderRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOrganizationsFeaturesResult")]
pub struct ListOrganizationsFeaturesResponse {
    #[serde(rename = "EnabledFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<FeaturesListType>,
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FeaturesListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for FeaturesListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for FeaturesListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<String>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<String>,
}

impl From<Vec<String>> for XmlStringList {
    fn from(v: Vec<String>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<String> for XmlStringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSAMLProviderResult")]
pub struct CreateSAMLProviderResponse {
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_provider_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListUserPoliciesResult")]
pub struct ListUserPoliciesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<policyNameListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyNameListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for policyNameListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for policyNameListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInstanceProfilesRequest")]
pub struct ListInstanceProfilesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPolicyTagsRequest")]
pub struct ListPolicyTagsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRolePoliciesRequest")]
pub struct ListRolePoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDelegationRequestResult")]
pub struct CreateDelegationRequestResponse {
    #[serde(rename = "ConsoleDeepLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_deep_link: Option<String>,
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableOrganizationsRootCredentialsManagementResult")]
pub struct EnableOrganizationsRootCredentialsManagementResponse {
    #[serde(rename = "EnabledFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<FeaturesListType>,
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateServiceSpecificCredentialRequest")]
pub struct UpdateServiceSpecificCredentialRequest {
    #[serde(rename = "ServiceSpecificCredentialId")]
    #[serde(default)]
    pub service_specific_credential_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLoginProfileResult")]
pub struct CreateLoginProfileResponse {
    #[serde(rename = "LoginProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_profile: Option<LoginProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoginProfile")]
pub struct LoginProfile {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "PasswordResetRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOpenIDConnectProviderResult")]
pub struct CreateOpenIDConnectProviderResponse {
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_connect_provider_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationsRootSessionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGroupsForUserResult")]
pub struct ListGroupsForUserResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<groupListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct groupListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Group>,
}
impl From<Vec<Group>> for groupListType {
    fn from(v: Vec<Group>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Group> for groupListType {
    fn from_iter<I: IntoIterator<Item = Group>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Group>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Group>,
}

impl From<Vec<Group>> for XmlGroupList {
    fn from(v: Vec<Group>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Group> for XmlGroupList {
    fn from_iter<I: IntoIterator<Item = Group>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Group")]
pub struct Group {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSAMLProvidersResult")]
pub struct ListSAMLProvidersResponse {
    #[serde(rename = "SAMLProviderList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_provider_list: Option<SAMLProviderListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAMLProviderListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SAMLProviderListEntry>,
}
impl From<Vec<SAMLProviderListEntry>> for SAMLProviderListType {
    fn from(v: Vec<SAMLProviderListEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SAMLProviderListEntry> for SAMLProviderListType {
    fn from_iter<I: IntoIterator<Item = SAMLProviderListEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SAMLProviderListEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSAMLProviderListEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SAMLProviderListEntry>,
}

impl From<Vec<SAMLProviderListEntry>> for XmlSAMLProviderListEntryList {
    fn from(v: Vec<SAMLProviderListEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SAMLProviderListEntry> for XmlSAMLProviderListEntryList {
    fn from_iter<I: IntoIterator<Item = SAMLProviderListEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SAMLProviderListEntry")]
pub struct SAMLProviderListEntry {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "ValidUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessKeysResult")]
pub struct ListAccessKeysResponse {
    #[serde(rename = "AccessKeyMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_metadata: Option<accessKeyMetadataListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct accessKeyMetadataListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AccessKeyMetadata>,
}
impl From<Vec<AccessKeyMetadata>> for accessKeyMetadataListType {
    fn from(v: Vec<AccessKeyMetadata>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AccessKeyMetadata> for accessKeyMetadataListType {
    fn from_iter<I: IntoIterator<Item = AccessKeyMetadata>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AccessKeyMetadata>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAccessKeyMetadataList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AccessKeyMetadata>,
}

impl From<Vec<AccessKeyMetadata>> for XmlAccessKeyMetadataList {
    fn from(v: Vec<AccessKeyMetadata>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AccessKeyMetadata> for XmlAccessKeyMetadataList {
    fn from_iter<I: IntoIterator<Item = AccessKeyMetadata>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessKeyMetadata")]
pub struct AccessKeyMetadata {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagPolicyRequest")]
pub struct UntagPolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct tagKeyListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for tagKeyListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for tagKeyListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddRoleToInstanceProfileRequest")]
pub struct AddRoleToInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListUsersRequest")]
pub struct ListUsersRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAttachedRolePoliciesRequest")]
pub struct ListAttachedRolePoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveUserFromGroupRequest")]
pub struct RemoveUserFromGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOpenIDConnectProvidersResult")]
pub struct ListOpenIDConnectProvidersResponse {
    #[serde(rename = "OpenIDConnectProviderList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_connect_provider_list: Option<OpenIDConnectProviderListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenIDConnectProviderListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OpenIDConnectProviderListEntry>,
}
impl From<Vec<OpenIDConnectProviderListEntry>> for OpenIDConnectProviderListType {
    fn from(v: Vec<OpenIDConnectProviderListEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OpenIDConnectProviderListEntry> for OpenIDConnectProviderListType {
    fn from_iter<I: IntoIterator<Item = OpenIDConnectProviderListEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OpenIDConnectProviderListEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOpenIDConnectProviderListEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OpenIDConnectProviderListEntry>,
}

impl From<Vec<OpenIDConnectProviderListEntry>> for XmlOpenIDConnectProviderListEntryList {
    fn from(v: Vec<OpenIDConnectProviderListEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OpenIDConnectProviderListEntry> for XmlOpenIDConnectProviderListEntryList {
    fn from_iter<I: IntoIterator<Item = OpenIDConnectProviderListEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OpenIDConnectProviderListEntry")]
pub struct OpenIDConnectProviderListEntry {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePolicyVersionRequest")]
pub struct DeletePolicyVersionRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    pub version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetContextKeysForPrincipalPolicyRequest")]
pub struct GetContextKeysForPrincipalPolicyRequest {
    #[serde(rename = "PolicyInputList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_input_list: Option<SimulationPolicyListType>,
    #[serde(rename = "PolicySourceArn")]
    #[serde(default)]
    pub policy_source_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimulationPolicyListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SimulationPolicyListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SimulationPolicyListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSAMLProviderTagsResult")]
pub struct ListSAMLProviderTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateSAMLProviderRequest")]
pub struct UpdateSAMLProviderRequest {
    #[serde(rename = "AddPrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_private_key: Option<String>,
    #[serde(rename = "AssertionEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_encryption_mode: Option<String>,
    #[serde(rename = "RemovePrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_private_key: Option<String>,
    #[serde(rename = "SAMLMetadataDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_metadata_document: Option<String>,
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    pub s_a_m_l_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSSHPublicKeyResult")]
pub struct GetSSHPublicKeyResponse {
    #[serde(rename = "SSHPublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_h_public_key: Option<SSHPublicKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SSHPublicKey")]
pub struct SSHPublicKey {
    #[serde(rename = "Fingerprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(rename = "SSHPublicKeyBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_h_public_key_body: Option<String>,
    #[serde(rename = "SSHPublicKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_h_public_key_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UploadDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutRolePermissionsBoundaryRequest")]
pub struct PutRolePermissionsBoundaryRequest {
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    pub permissions_boundary: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagSAMLProviderRequest")]
pub struct UntagSAMLProviderRequest {
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    pub s_a_m_l_provider_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResyncMFADeviceRequest")]
pub struct ResyncMFADeviceRequest {
    #[serde(rename = "AuthenticationCode1")]
    #[serde(default)]
    pub authentication_code1: String,
    #[serde(rename = "AuthenticationCode2")]
    #[serde(default)]
    pub authentication_code2: String,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLoginProfileRequest")]
pub struct CreateLoginProfileRequest {
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "PasswordResetRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRoleRequest")]
pub struct CreateRoleRequest {
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(default)]
    pub assume_role_policy_document: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MaxSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i32>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServiceLastAccessedDetailsWithEntitiesRequest")]
pub struct GetServiceLastAccessedDetailsWithEntitiesRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    pub service_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPoliciesRequest")]
pub struct ListPoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "OnlyAttached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_attached: Option<bool>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    #[serde(rename = "PolicyUsageFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_usage_filter: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListUserTagsRequest")]
pub struct ListUserTagsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePolicyVersionResult")]
pub struct CreatePolicyVersionResponse {
    #[serde(rename = "PolicyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<PolicyVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyVersion")]
pub struct PolicyVersion {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "Document")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "IsDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSAMLProviderRequest")]
pub struct CreateSAMLProviderRequest {
    #[serde(rename = "AddPrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_private_key: Option<String>,
    #[serde(rename = "AssertionEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_encryption_mode: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SAMLMetadataDocument")]
    #[serde(default)]
    pub s_a_m_l_metadata_document: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeactivateMFADeviceRequest")]
pub struct DeactivateMFADeviceRequest {
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteRolePermissionsBoundaryRequest")]
pub struct DeleteRolePermissionsBoundaryRequest {
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadSigningCertificateResult")]
pub struct UploadSigningCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<SigningCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SigningCertificate")]
pub struct SigningCertificate {
    #[serde(rename = "CertificateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_body: Option<String>,
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UploadDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSigningCertificatesResult")]
pub struct ListSigningCertificatesResponse {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<certificateListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct certificateListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SigningCertificate>,
}
impl From<Vec<SigningCertificate>> for certificateListType {
    fn from(v: Vec<SigningCertificate>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SigningCertificate> for certificateListType {
    fn from_iter<I: IntoIterator<Item = SigningCertificate>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SigningCertificate>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSigningCertificateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SigningCertificate>,
}

impl From<Vec<SigningCertificate>> for XmlSigningCertificateList {
    fn from(v: Vec<SigningCertificate>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SigningCertificate> for XmlSigningCertificateList {
    fn from_iter<I: IntoIterator<Item = SigningCertificate>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSSHPublicKeysResult")]
pub struct ListSSHPublicKeysResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SSHPublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_h_public_keys: Option<SSHPublicKeyListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSHPublicKeyListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SSHPublicKeyMetadata>,
}
impl From<Vec<SSHPublicKeyMetadata>> for SSHPublicKeyListType {
    fn from(v: Vec<SSHPublicKeyMetadata>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SSHPublicKeyMetadata> for SSHPublicKeyListType {
    fn from_iter<I: IntoIterator<Item = SSHPublicKeyMetadata>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SSHPublicKeyMetadata>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSSHPublicKeyMetadataList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SSHPublicKeyMetadata>,
}

impl From<Vec<SSHPublicKeyMetadata>> for XmlSSHPublicKeyMetadataList {
    fn from(v: Vec<SSHPublicKeyMetadata>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SSHPublicKeyMetadata> for XmlSSHPublicKeyMetadataList {
    fn from_iter<I: IntoIterator<Item = SSHPublicKeyMetadata>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SSHPublicKeyMetadata")]
pub struct SSHPublicKeyMetadata {
    #[serde(rename = "SSHPublicKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_h_public_key_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UploadDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutUserPermissionsBoundaryRequest")]
pub struct PutUserPermissionsBoundaryRequest {
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    pub permissions_boundary: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInstanceProfilesResult")]
pub struct ListInstanceProfilesResponse {
    #[serde(rename = "InstanceProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profiles: Option<instanceProfileListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct instanceProfileListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<InstanceProfile>,
}
impl From<Vec<InstanceProfile>> for instanceProfileListType {
    fn from(v: Vec<InstanceProfile>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<InstanceProfile> for instanceProfileListType {
    fn from_iter<I: IntoIterator<Item = InstanceProfile>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<InstanceProfile>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInstanceProfileList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<InstanceProfile>,
}

impl From<Vec<InstanceProfile>> for XmlInstanceProfileList {
    fn from(v: Vec<InstanceProfile>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<InstanceProfile> for XmlInstanceProfileList {
    fn from_iter<I: IntoIterator<Item = InstanceProfile>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceProfile")]
pub struct InstanceProfile {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "InstanceProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_id: Option<String>,
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<roleListType>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct roleListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Role>,
}
impl From<Vec<Role>> for roleListType {
    fn from(v: Vec<Role>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Role> for roleListType {
    fn from_iter<I: IntoIterator<Item = Role>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Role>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRoleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Role>,
}

impl From<Vec<Role>> for XmlRoleList {
    fn from(v: Vec<Role>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Role> for XmlRoleList {
    fn from_iter<I: IntoIterator<Item = Role>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Role")]
pub struct Role {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MaxSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i32>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
    #[serde(rename = "RoleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(rename = "RoleLastUsed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_last_used: Option<RoleLastUsed>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RoleLastUsed")]
pub struct RoleLastUsed {
    #[serde(rename = "LastUsedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_date: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadServerCertificateResult")]
pub struct UploadServerCertificateResponse {
    #[serde(rename = "ServerCertificateMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_metadata: Option<ServerCertificateMetadata>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerCertificateMetadata")]
pub struct ServerCertificateMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "ServerCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_id: Option<String>,
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_name: Option<String>,
    #[serde(rename = "UploadDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateDelegationRequestRequest")]
pub struct AssociateDelegationRequestRequest {
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    pub delegation_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccountAuthorizationDetailsResult")]
pub struct GetAccountAuthorizationDetailsResponse {
    #[serde(rename = "GroupDetailList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_detail_list: Option<groupDetailListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ManagedPolicyDetailListType>,
    #[serde(rename = "RoleDetailList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_detail_list: Option<roleDetailListType>,
    #[serde(rename = "UserDetailList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_detail_list: Option<userDetailListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct groupDetailListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<GroupDetail>,
}
impl From<Vec<GroupDetail>> for groupDetailListType {
    fn from(v: Vec<GroupDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GroupDetail> for groupDetailListType {
    fn from_iter<I: IntoIterator<Item = GroupDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GroupDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGroupDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GroupDetail>,
}

impl From<Vec<GroupDetail>> for XmlGroupDetailList {
    fn from(v: Vec<GroupDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GroupDetail> for XmlGroupDetailList {
    fn from_iter<I: IntoIterator<Item = GroupDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GroupDetail")]
pub struct GroupDetail {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<attachedPoliciesListType>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "GroupPolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_policy_list: Option<policyDetailListType>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyDetailListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyDetail>,
}
impl From<Vec<PolicyDetail>> for policyDetailListType {
    fn from(v: Vec<PolicyDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyDetail> for policyDetailListType {
    fn from_iter<I: IntoIterator<Item = PolicyDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyDetail>,
}

impl From<Vec<PolicyDetail>> for XmlPolicyDetailList {
    fn from(v: Vec<PolicyDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyDetail> for XmlPolicyDetailList {
    fn from_iter<I: IntoIterator<Item = PolicyDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyDetail")]
pub struct PolicyDetail {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedPolicyDetailListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ManagedPolicyDetail>,
}
impl From<Vec<ManagedPolicyDetail>> for ManagedPolicyDetailListType {
    fn from(v: Vec<ManagedPolicyDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ManagedPolicyDetail> for ManagedPolicyDetailListType {
    fn from_iter<I: IntoIterator<Item = ManagedPolicyDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ManagedPolicyDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlManagedPolicyDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ManagedPolicyDetail>,
}

impl From<Vec<ManagedPolicyDetail>> for XmlManagedPolicyDetailList {
    fn from(v: Vec<ManagedPolicyDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ManagedPolicyDetail> for XmlManagedPolicyDetailList {
    fn from_iter<I: IntoIterator<Item = ManagedPolicyDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ManagedPolicyDetail")]
pub struct ManagedPolicyDetail {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AttachmentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<i32>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "DefaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsAttachable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attachable: Option<bool>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundaryUsageCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_usage_count: Option<i32>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyVersionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_list: Option<policyDocumentVersionListType>,
    #[serde(rename = "UpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyDocumentVersionListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyVersion>,
}
impl From<Vec<PolicyVersion>> for policyDocumentVersionListType {
    fn from(v: Vec<PolicyVersion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyVersion> for policyDocumentVersionListType {
    fn from_iter<I: IntoIterator<Item = PolicyVersion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyVersion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyVersionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyVersion>,
}

impl From<Vec<PolicyVersion>> for XmlPolicyVersionList {
    fn from(v: Vec<PolicyVersion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyVersion> for XmlPolicyVersionList {
    fn from_iter<I: IntoIterator<Item = PolicyVersion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct roleDetailListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RoleDetail>,
}
impl From<Vec<RoleDetail>> for roleDetailListType {
    fn from(v: Vec<RoleDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RoleDetail> for roleDetailListType {
    fn from_iter<I: IntoIterator<Item = RoleDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RoleDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRoleDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RoleDetail>,
}

impl From<Vec<RoleDetail>> for XmlRoleDetailList {
    fn from(v: Vec<RoleDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RoleDetail> for XmlRoleDetailList {
    fn from_iter<I: IntoIterator<Item = RoleDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RoleDetail")]
pub struct RoleDetail {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<attachedPoliciesListType>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "InstanceProfileList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_list: Option<instanceProfileListType>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
    #[serde(rename = "RoleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(rename = "RoleLastUsed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_last_used: Option<RoleLastUsed>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "RolePolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_policy_list: Option<policyDetailListType>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct userDetailListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UserDetail>,
}
impl From<Vec<UserDetail>> for userDetailListType {
    fn from(v: Vec<UserDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UserDetail> for userDetailListType {
    fn from_iter<I: IntoIterator<Item = UserDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UserDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUserDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UserDetail>,
}

impl From<Vec<UserDetail>> for XmlUserDetailList {
    fn from(v: Vec<UserDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UserDetail> for XmlUserDetailList {
    fn from_iter<I: IntoIterator<Item = UserDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UserDetail")]
pub struct UserDetail {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<attachedPoliciesListType>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "GroupList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<groupNameListType>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "UserPolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy_list: Option<policyDetailListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct groupNameListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for groupNameListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for groupNameListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteUserRequest")]
pub struct DeleteUserRequest {
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInstanceProfilesForRoleResult")]
pub struct ListInstanceProfilesForRoleResponse {
    #[serde(rename = "InstanceProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profiles: Option<instanceProfileListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GenerateOrganizationsAccessReportResult")]
pub struct GenerateOrganizationsAccessReportResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPoliciesGrantingServiceAccessResult")]
pub struct ListPoliciesGrantingServiceAccessResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PoliciesGrantingServiceAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies_granting_service_access: Option<listPolicyGrantingServiceAccessResponseListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct listPolicyGrantingServiceAccessResponseListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ListPoliciesGrantingServiceAccessEntry>,
}
impl From<Vec<ListPoliciesGrantingServiceAccessEntry>>
    for listPolicyGrantingServiceAccessResponseListType
{
    fn from(v: Vec<ListPoliciesGrantingServiceAccessEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ListPoliciesGrantingServiceAccessEntry>
    for listPolicyGrantingServiceAccessResponseListType
{
    fn from_iter<I: IntoIterator<Item = ListPoliciesGrantingServiceAccessEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ListPoliciesGrantingServiceAccessEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListPoliciesGrantingServiceAccessEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ListPoliciesGrantingServiceAccessEntry>,
}

impl From<Vec<ListPoliciesGrantingServiceAccessEntry>>
    for XmlListPoliciesGrantingServiceAccessEntryList
{
    fn from(v: Vec<ListPoliciesGrantingServiceAccessEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ListPoliciesGrantingServiceAccessEntry>
    for XmlListPoliciesGrantingServiceAccessEntryList
{
    fn from_iter<I: IntoIterator<Item = ListPoliciesGrantingServiceAccessEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPoliciesGrantingServiceAccessEntry")]
pub struct ListPoliciesGrantingServiceAccessEntry {
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<policyGrantingServiceAccessListType>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyGrantingServiceAccessListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyGrantingServiceAccess>,
}
impl From<Vec<PolicyGrantingServiceAccess>> for policyGrantingServiceAccessListType {
    fn from(v: Vec<PolicyGrantingServiceAccess>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyGrantingServiceAccess> for policyGrantingServiceAccessListType {
    fn from_iter<I: IntoIterator<Item = PolicyGrantingServiceAccess>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyGrantingServiceAccess>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyGrantingServiceAccessList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyGrantingServiceAccess>,
}

impl From<Vec<PolicyGrantingServiceAccess>> for XmlPolicyGrantingServiceAccessList {
    fn from(v: Vec<PolicyGrantingServiceAccess>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyGrantingServiceAccess> for XmlPolicyGrantingServiceAccessList {
    fn from_iter<I: IntoIterator<Item = PolicyGrantingServiceAccess>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyGrantingServiceAccess")]
pub struct PolicyGrantingServiceAccess {
    #[serde(rename = "EntityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(rename = "EntityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListServerCertificatesResult")]
pub struct ListServerCertificatesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ServerCertificateMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_metadata_list: Option<serverCertificateMetadataListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct serverCertificateMetadataListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServerCertificateMetadata>,
}
impl From<Vec<ServerCertificateMetadata>> for serverCertificateMetadataListType {
    fn from(v: Vec<ServerCertificateMetadata>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ServerCertificateMetadata> for serverCertificateMetadataListType {
    fn from_iter<I: IntoIterator<Item = ServerCertificateMetadata>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ServerCertificateMetadata>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlServerCertificateMetadataList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ServerCertificateMetadata>,
}

impl From<Vec<ServerCertificateMetadata>> for XmlServerCertificateMetadataList {
    fn from(v: Vec<ServerCertificateMetadata>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ServerCertificateMetadata> for XmlServerCertificateMetadataList {
    fn from_iter<I: IntoIterator<Item = ServerCertificateMetadata>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServiceLinkedRoleRequest")]
pub struct CreateServiceLinkedRoleRequest {
    #[serde(rename = "AWSServiceName")]
    #[serde(default)]
    pub a_w_s_service_name: String,
    #[serde(rename = "CustomSuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_suffix: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSAMLProvidersRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateUserRequest")]
pub struct CreateUserRequest {
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationsRootCredentialsManagementRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListServerCertificatesRequest")]
pub struct ListServerCertificatesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableOrganizationsRootSessionsResult")]
pub struct DisableOrganizationsRootSessionsResponse {
    #[serde(rename = "EnabledFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<FeaturesListType>,
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetUserPolicyRequest")]
pub struct GetUserPolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetServiceSpecificCredentialRequest")]
pub struct ResetServiceSpecificCredentialRequest {
    #[serde(rename = "ServiceSpecificCredentialId")]
    #[serde(default)]
    pub service_specific_credential_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccountPasswordPolicyResult")]
pub struct GetAccountPasswordPolicyResponse {
    #[serde(rename = "PasswordPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<PasswordPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PasswordPolicy")]
pub struct PasswordPolicy {
    #[serde(rename = "AllowUsersToChangePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_users_to_change_password: Option<bool>,
    #[serde(rename = "ExpirePasswords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_passwords: Option<bool>,
    #[serde(rename = "HardExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_expiry: Option<bool>,
    #[serde(rename = "MaxPasswordAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_password_age: Option<i32>,
    #[serde(rename = "MinimumPasswordLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_password_length: Option<i32>,
    #[serde(rename = "PasswordReusePrevention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_prevention: Option<i32>,
    #[serde(rename = "RequireLowercaseCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase_characters: Option<bool>,
    #[serde(rename = "RequireNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    #[serde(rename = "RequireSymbols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,
    #[serde(rename = "RequireUppercaseCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase_characters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateUserRequest")]
pub struct UpdateUserRequest {
    #[serde(rename = "NewPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_path: Option<String>,
    #[serde(rename = "NewUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_user_name: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDelegationRequestsResult")]
pub struct ListDelegationRequestsResponse {
    #[serde(rename = "DelegationRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_requests: Option<delegationRequestsListType>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "isTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct delegationRequestsListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DelegationRequest>,
}
impl From<Vec<DelegationRequest>> for delegationRequestsListType {
    fn from(v: Vec<DelegationRequest>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DelegationRequest> for delegationRequestsListType {
    fn from_iter<I: IntoIterator<Item = DelegationRequest>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DelegationRequest>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDelegationRequestList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DelegationRequest>,
}

impl From<Vec<DelegationRequest>> for XmlDelegationRequestList {
    fn from(v: Vec<DelegationRequest>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DelegationRequest> for XmlDelegationRequestList {
    fn from_iter<I: IntoIterator<Item = DelegationRequest>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DelegationRequest")]
pub struct DelegationRequest {
    #[serde(rename = "ApproverId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_request_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "OnlySendByOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_send_by_owner: Option<bool>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "PermissionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_policy: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DelegationPermission>,
    #[serde(rename = "RedirectUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "RejectionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
    #[serde(rename = "RequestMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[serde(rename = "RequestorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[serde(rename = "RequestorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_name: Option<String>,
    #[serde(rename = "RolePermissionRestrictionArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_permission_restriction_arns: Option<rolePermissionRestrictionArnListType>,
    #[serde(rename = "SessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_duration: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DelegationPermission")]
pub struct DelegationPermission {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<policyParameterListType>,
    #[serde(rename = "PolicyTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_template_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyParameterListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyParameter>,
}
impl From<Vec<PolicyParameter>> for policyParameterListType {
    fn from(v: Vec<PolicyParameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyParameter> for policyParameterListType {
    fn from_iter<I: IntoIterator<Item = PolicyParameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyParameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyParameter>,
}

impl From<Vec<PolicyParameter>> for XmlPolicyParameterList {
    fn from(v: Vec<PolicyParameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyParameter> for XmlPolicyParameterList {
    fn from_iter<I: IntoIterator<Item = PolicyParameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyParameter")]
pub struct PolicyParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<policyParameterValuesListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyParameterValuesListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for policyParameterValuesListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for policyParameterValuesListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct rolePermissionRestrictionArnListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for rolePermissionRestrictionArnListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for rolePermissionRestrictionArnListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetLoginProfileRequest")]
pub struct GetLoginProfileRequest {
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateSSHPublicKeyRequest")]
pub struct UpdateSSHPublicKeyRequest {
    #[serde(rename = "SSHPublicKeyId")]
    #[serde(default)]
    pub s_s_h_public_key_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SimulateCustomPolicyRequest")]
pub struct SimulateCustomPolicyRequest {
    #[serde(rename = "ActionNames")]
    #[serde(default)]
    pub action_names: ActionNameListType,
    #[serde(rename = "CallerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_arn: Option<String>,
    #[serde(rename = "ContextEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_entries: Option<ContextEntryListType>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PermissionsBoundaryPolicyInputList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_policy_input_list: Option<SimulationPolicyListType>,
    #[serde(rename = "PolicyInputList")]
    #[serde(default)]
    pub policy_input_list: SimulationPolicyListType,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<ResourceNameListType>,
    #[serde(rename = "ResourceHandlingOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_handling_option: Option<String>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionNameListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ActionNameListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ActionNameListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextEntryListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ContextEntry>,
}
impl From<Vec<ContextEntry>> for ContextEntryListType {
    fn from(v: Vec<ContextEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ContextEntry> for ContextEntryListType {
    fn from_iter<I: IntoIterator<Item = ContextEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ContextEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlContextEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ContextEntry>,
}

impl From<Vec<ContextEntry>> for XmlContextEntryList {
    fn from(v: Vec<ContextEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ContextEntry> for XmlContextEntryList {
    fn from_iter<I: IntoIterator<Item = ContextEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContextEntry")]
pub struct ContextEntry {
    #[serde(rename = "ContextKeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_key_name: Option<String>,
    #[serde(rename = "ContextKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_key_type: Option<String>,
    #[serde(rename = "ContextKeyValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_key_values: Option<ContextKeyValueListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextKeyValueListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ContextKeyValueListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ContextKeyValueListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceNameListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ResourceNameListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ResourceNameListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteRoleRequest")]
pub struct DeleteRoleRequest {
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccountAuthorizationDetailsRequest")]
pub struct GetAccountAuthorizationDetailsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<entityListType>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct entityListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for entityListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for entityListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListServiceSpecificCredentialsRequest")]
pub struct ListServiceSpecificCredentialsRequest {
    #[serde(rename = "AllUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_users: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachUserPolicyRequest")]
pub struct AttachUserPolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddClientIDToOpenIDConnectProviderRequest")]
pub struct AddClientIDToOpenIDConnectProviderRequest {
    #[serde(rename = "ClientID")]
    #[serde(default)]
    pub client_i_d: String,
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateVirtualMFADeviceRequest")]
pub struct CreateVirtualMFADeviceRequest {
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "VirtualMFADeviceName")]
    #[serde(default)]
    pub virtual_m_f_a_device_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGroupsResult")]
pub struct ListGroupsResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<groupListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateServerCertificateRequest")]
pub struct UpdateServerCertificateRequest {
    #[serde(rename = "NewPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_path: Option<String>,
    #[serde(rename = "NewServerCertificateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_server_certificate_name: Option<String>,
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    pub server_certificate_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGroupPoliciesRequest")]
pub struct ListGroupPoliciesRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServiceSpecificCredentialResult")]
pub struct CreateServiceSpecificCredentialResponse {
    #[serde(rename = "ServiceSpecificCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specific_credential: Option<ServiceSpecificCredential>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServiceSpecificCredential")]
pub struct ServiceSpecificCredential {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "ServiceCredentialAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_credential_alias: Option<String>,
    #[serde(rename = "ServiceCredentialSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_credential_secret: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServicePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_password: Option<String>,
    #[serde(rename = "ServiceSpecificCredentialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specific_credential_id: Option<String>,
    #[serde(rename = "ServiceUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_user_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetRoleResult")]
pub struct GetRoleResponse {
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableOrganizationsRootSessionsResult")]
pub struct EnableOrganizationsRootSessionsResponse {
    #[serde(rename = "EnabledFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<FeaturesListType>,
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDelegationRequestRequest")]
pub struct GetDelegationRequestRequest {
    #[serde(rename = "DelegationPermissionCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_permission_check: Option<bool>,
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    pub delegation_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagUserRequest")]
pub struct TagUserRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServiceLinkedRoleDeletionStatusResult")]
pub struct GetServiceLinkedRoleDeletionStatusResponse {
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<DeletionTaskFailureReasonType>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletionTaskFailureReasonType")]
pub struct DeletionTaskFailureReasonType {
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "RoleUsageList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_usage_list: Option<RoleUsageListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoleUsageListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RoleUsageType>,
}
impl From<Vec<RoleUsageType>> for RoleUsageListType {
    fn from(v: Vec<RoleUsageType>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RoleUsageType> for RoleUsageListType {
    fn from_iter<I: IntoIterator<Item = RoleUsageType>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RoleUsageType>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRoleUsageTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RoleUsageType>,
}

impl From<Vec<RoleUsageType>> for XmlRoleUsageTypeList {
    fn from(v: Vec<RoleUsageType>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RoleUsageType> for XmlRoleUsageTypeList {
    fn from_iter<I: IntoIterator<Item = RoleUsageType>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RoleUsageType")]
pub struct RoleUsageType {
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ArnListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArnListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ArnListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ArnListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRoleResult")]
pub struct CreateRoleResponse {
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateRoleRequest")]
pub struct UpdateRoleRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MaxSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i32>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateSAMLProviderResult")]
pub struct UpdateSAMLProviderResponse {
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_provider_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccountAliasesRequest")]
pub struct ListAccountAliasesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetRolePolicyResult")]
pub struct GetRolePolicyResponse {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGroupPolicyRequest")]
pub struct GetGroupPolicyRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSSHPublicKeysRequest")]
pub struct ListSSHPublicKeysRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableOrganizationsRootCredentialsManagementResult")]
pub struct DisableOrganizationsRootCredentialsManagementResponse {
    #[serde(rename = "EnabledFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<FeaturesListType>,
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccountAliasRequest")]
pub struct DeleteAccountAliasRequest {
    #[serde(rename = "AccountAlias")]
    #[serde(default)]
    pub account_alias: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHumanReadableSummaryResult")]
pub struct GetHumanReadableSummaryResponse {
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "SummaryContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_content: Option<String>,
    #[serde(rename = "SummaryState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSAMLProviderTagsRequest")]
pub struct ListSAMLProviderTagsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    pub s_a_m_l_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadSigningCertificateRequest")]
pub struct UploadSigningCertificateRequest {
    #[serde(rename = "CertificateBody")]
    #[serde(default)]
    pub certificate_body: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAssumeRolePolicyRequest")]
pub struct UpdateAssumeRolePolicyRequest {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessKeyRequest")]
pub struct CreateAccessKeyRequest {
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagRoleRequest")]
pub struct UntagRoleRequest {
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMFADevicesRequest")]
pub struct ListMFADevicesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationsRootCredentialsManagementRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccountAliasRequest")]
pub struct CreateAccountAliasRequest {
    #[serde(rename = "AccountAlias")]
    #[serde(default)]
    pub account_alias: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetServiceSpecificCredentialResult")]
pub struct ResetServiceSpecificCredentialResponse {
    #[serde(rename = "ServiceSpecificCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specific_credential: Option<ServiceSpecificCredential>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SendDelegationTokenRequest")]
pub struct SendDelegationTokenRequest {
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    pub delegation_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AcceptDelegationRequestRequest")]
pub struct AcceptDelegationRequestRequest {
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    pub delegation_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePolicyVersionRequest")]
pub struct CreatePolicyVersionRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "SetAsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadSSHPublicKeyRequest")]
pub struct UploadSSHPublicKeyRequest {
    #[serde(rename = "SSHPublicKeyBody")]
    #[serde(default)]
    pub s_s_h_public_key_body: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagMFADeviceRequest")]
pub struct TagMFADeviceRequest {
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateRoleDescriptionResult")]
pub struct UpdateRoleDescriptionResponse {
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServiceLastAccessedDetailsWithEntitiesResult")]
pub struct GetServiceLastAccessedDetailsWithEntitiesResponse {
    #[serde(rename = "EntityDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_details_list: Option<entityDetailsListType>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "JobCompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_completion_date: Option<String>,
    #[serde(rename = "JobCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_creation_date: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct entityDetailsListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EntityDetails>,
}
impl From<Vec<EntityDetails>> for entityDetailsListType {
    fn from(v: Vec<EntityDetails>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EntityDetails> for entityDetailsListType {
    fn from_iter<I: IntoIterator<Item = EntityDetails>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EntityDetails>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEntityDetailsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EntityDetails>,
}

impl From<Vec<EntityDetails>> for XmlEntityDetailsList {
    fn from(v: Vec<EntityDetails>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EntityDetails> for XmlEntityDetailsList {
    fn from_iter<I: IntoIterator<Item = EntityDetails>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EntityDetails")]
pub struct EntityDetails {
    #[serde(rename = "EntityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_info: Option<EntityInfo>,
    #[serde(rename = "LastAuthenticated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authenticated: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EntityInfo")]
pub struct EntityInfo {
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
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListServerCertificateTagsResult")]
pub struct ListServerCertificateTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListUserPoliciesRequest")]
pub struct ListUserPoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutRolePolicyRequest")]
pub struct PutRolePolicyRequest {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServerCertificateRequest")]
pub struct GetServerCertificateRequest {
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    pub server_certificate_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOpenIDConnectProviderRequest")]
pub struct GetOpenIDConnectProviderRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRoleTagsResult")]
pub struct ListRoleTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagInstanceProfileRequest")]
pub struct UntagInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadSSHPublicKeyResult")]
pub struct UploadSSHPublicKeyResponse {
    #[serde(rename = "SSHPublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_h_public_key: Option<SSHPublicKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServerCertificateRequest")]
pub struct DeleteServerCertificateRequest {
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    pub server_certificate_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOrganizationsAccessReportRequest")]
pub struct GetOrganizationsAccessReportRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "SortKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRolesRequest")]
pub struct ListRolesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInstanceProfilesForRoleRequest")]
pub struct ListInstanceProfilesForRoleRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SimulatePrincipalPolicyRequest")]
pub struct SimulatePrincipalPolicyRequest {
    #[serde(rename = "ActionNames")]
    #[serde(default)]
    pub action_names: ActionNameListType,
    #[serde(rename = "CallerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_arn: Option<String>,
    #[serde(rename = "ContextEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_entries: Option<ContextEntryListType>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PermissionsBoundaryPolicyInputList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_policy_input_list: Option<SimulationPolicyListType>,
    #[serde(rename = "PolicyInputList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_input_list: Option<SimulationPolicyListType>,
    #[serde(rename = "PolicySourceArn")]
    #[serde(default)]
    pub policy_source_arn: String,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<ResourceNameListType>,
    #[serde(rename = "ResourceHandlingOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_handling_option: Option<String>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableMFADeviceRequest")]
pub struct EnableMFADeviceRequest {
    #[serde(rename = "AuthenticationCode1")]
    #[serde(default)]
    pub authentication_code1: String,
    #[serde(rename = "AuthenticationCode2")]
    #[serde(default)]
    pub authentication_code2: String,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPolicyVersionResult")]
pub struct GetPolicyVersionResponse {
    #[serde(rename = "PolicyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<PolicyVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetContextKeysForPrincipalPolicyResult")]
pub struct GetContextKeysForPolicyResponse {
    #[serde(rename = "ContextKeyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_key_names: Option<ContextKeyNamesResultListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextKeyNamesResultListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ContextKeyNamesResultListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ContextKeyNamesResultListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPolicyVersionsResult")]
pub struct ListPolicyVersionsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<policyDocumentVersionListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagPolicyRequest")]
pub struct TagPolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMFADeviceResult")]
pub struct GetMFADeviceResponse {
    #[serde(rename = "Certifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certifications: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EnableDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_date: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSSHPublicKeyRequest")]
pub struct DeleteSSHPublicKeyRequest {
    #[serde(rename = "SSHPublicKeyId")]
    #[serde(default)]
    pub s_s_h_public_key_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGroupPoliciesResult")]
pub struct ListGroupPoliciesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<policyNameListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateVirtualMFADeviceResult")]
pub struct CreateVirtualMFADeviceResponse {
    #[serde(rename = "VirtualMFADevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_m_f_a_device: Option<VirtualMFADevice>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateInstanceProfileResult")]
pub struct CreateInstanceProfileResponse {
    #[serde(rename = "InstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHumanReadableSummaryRequest")]
pub struct GetHumanReadableSummaryRequest {
    #[serde(rename = "EntityArn")]
    #[serde(default)]
    pub entity_arn: String,
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetUserResult")]
pub struct GetUserResponse {
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachUserPolicyRequest")]
pub struct DetachUserPolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPolicyVersionRequest")]
pub struct GetPolicyVersionRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    pub version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListSigningCertificatesRequest")]
pub struct ListSigningCertificatesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPoliciesGrantingServiceAccessRequest")]
pub struct ListPoliciesGrantingServiceAccessRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ServiceNamespaces")]
    #[serde(default)]
    pub service_namespaces: serviceNamespaceListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct serviceNamespaceListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for serviceNamespaceListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for serviceNamespaceListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetInstanceProfileResult")]
pub struct GetInstanceProfileResponse {
    #[serde(rename = "InstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServiceLastAccessedDetailsRequest")]
pub struct GetServiceLastAccessedDetailsRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPolicyVersionsRequest")]
pub struct ListPolicyVersionsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessKeyLastUsedRequest")]
pub struct GetAccessKeyLastUsedRequest {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccountAliasesResult")]
pub struct ListAccountAliasesResponse {
    #[serde(rename = "AccountAliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aliases: Option<accountAliasListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct accountAliasListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for accountAliasListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for accountAliasListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangePasswordRequest")]
pub struct ChangePasswordRequest {
    #[serde(rename = "NewPassword")]
    #[serde(default)]
    pub new_password: String,
    #[serde(rename = "OldPassword")]
    #[serde(default)]
    pub old_password: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateOpenIDConnectProviderThumbprintRequest")]
pub struct UpdateOpenIDConnectProviderThumbprintRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
    #[serde(rename = "ThumbprintList")]
    #[serde(default)]
    pub thumbprint_list: thumbprintListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct thumbprintListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for thumbprintListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for thumbprintListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachGroupPolicyRequest")]
pub struct AttachGroupPolicyRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddUserToGroupRequest")]
pub struct AddUserToGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMFADevicesResult")]
pub struct ListMFADevicesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MFADevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a_devices: Option<mfaDeviceListType>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct mfaDeviceListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MFADevice>,
}
impl From<Vec<MFADevice>> for mfaDeviceListType {
    fn from(v: Vec<MFADevice>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MFADevice> for mfaDeviceListType {
    fn from_iter<I: IntoIterator<Item = MFADevice>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MFADevice>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMFADeviceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MFADevice>,
}

impl From<Vec<MFADevice>> for XmlMFADeviceList {
    fn from(v: Vec<MFADevice>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MFADevice> for XmlMFADeviceList {
    fn from_iter<I: IntoIterator<Item = MFADevice>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MFADevice")]
pub struct MFADevice {
    #[serde(rename = "EnableDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_date: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInstanceProfileTagsRequest")]
pub struct ListInstanceProfileTagsRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPolicyTagsResult")]
pub struct ListPolicyTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListServerCertificateTagsRequest")]
pub struct ListServerCertificateTagsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    pub server_certificate_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGroupsRequest")]
pub struct ListGroupsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServiceLinkedRoleResult")]
pub struct CreateServiceLinkedRoleResponse {
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteUserPolicyRequest")]
pub struct DeleteUserPolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOpenIDConnectProviderTagsRequest")]
pub struct ListOpenIDConnectProviderTagsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMFADeviceRequest")]
pub struct GetMFADeviceRequest {
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOpenIDConnectProviderRequest")]
pub struct CreateOpenIDConnectProviderRequest {
    #[serde(rename = "ClientIDList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_i_d_list: Option<clientIDListType>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "ThumbprintList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint_list: Option<thumbprintListType>,
    #[serde(rename = "Url")]
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct clientIDListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for clientIDListType {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for clientIDListType {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateRoleDescriptionRequest")]
pub struct UpdateRoleDescriptionRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateUserResult")]
pub struct CreateUserResponse {
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachRolePolicyRequest")]
pub struct DetachRolePolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSigningCertificateRequest")]
pub struct DeleteSigningCertificateRequest {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetUserRequest")]
pub struct GetUserRequest {
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateGroupRequest")]
pub struct UpdateGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "NewGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_group_name: Option<String>,
    #[serde(rename = "NewPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInstanceProfileTagsResult")]
pub struct ListInstanceProfileTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPolicyRequest")]
pub struct GetPolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteInstanceProfileRequest")]
pub struct DeleteInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAttachedGroupPoliciesRequest")]
pub struct ListAttachedGroupPoliciesRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateLoginProfileRequest")]
pub struct UpdateLoginProfileRequest {
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "PasswordResetRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reset_required: Option<bool>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteVirtualMFADeviceRequest")]
pub struct DeleteVirtualMFADeviceRequest {
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteUserPermissionsBoundaryRequest")]
pub struct DeleteUserPermissionsBoundaryRequest {
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagUserRequest")]
pub struct UntagUserRequest {
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCredentialReportResult")]
pub struct GetCredentialReportResponse {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "GeneratedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<String>,
    #[serde(rename = "ReportFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SimulatePrincipalPolicyResult")]
pub struct SimulatePolicyResponse {
    #[serde(rename = "EvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<EvaluationResultsListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationResultsListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EvaluationResult>,
}
impl From<Vec<EvaluationResult>> for EvaluationResultsListType {
    fn from(v: Vec<EvaluationResult>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EvaluationResult> for EvaluationResultsListType {
    fn from_iter<I: IntoIterator<Item = EvaluationResult>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EvaluationResult>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEvaluationResultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EvaluationResult>,
}

impl From<Vec<EvaluationResult>> for XmlEvaluationResultList {
    fn from(v: Vec<EvaluationResult>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EvaluationResult> for XmlEvaluationResultList {
    fn from_iter<I: IntoIterator<Item = EvaluationResult>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EvaluationResult")]
pub struct EvaluationResult {
    #[serde(rename = "EvalActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_action_name: Option<String>,
    #[serde(rename = "EvalDecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_decision: Option<String>,
    #[serde(rename = "EvalDecisionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_decision_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EvalResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_resource_name: Option<String>,
    #[serde(rename = "MatchedStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_statements: Option<StatementListType>,
    #[serde(rename = "MissingContextValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_context_values: Option<ContextKeyNamesResultListType>,
    #[serde(rename = "OrganizationsDecisionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations_decision_detail: Option<OrganizationsDecisionDetail>,
    #[serde(rename = "PermissionsBoundaryDecisionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_decision_detail: Option<PermissionsBoundaryDecisionDetail>,
    #[serde(rename = "ResourceSpecificResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specific_results: Option<ResourceSpecificResultListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatementListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Statement>,
}
impl From<Vec<Statement>> for StatementListType {
    fn from(v: Vec<Statement>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Statement> for StatementListType {
    fn from_iter<I: IntoIterator<Item = Statement>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Statement>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStatementList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Statement>,
}

impl From<Vec<Statement>> for XmlStatementList {
    fn from(v: Vec<Statement>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Statement> for XmlStatementList {
    fn from_iter<I: IntoIterator<Item = Statement>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Statement")]
pub struct Statement {
    #[serde(rename = "EndPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_position: Option<Position>,
    #[serde(rename = "SourcePolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_policy_id: Option<String>,
    #[serde(rename = "SourcePolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_policy_type: Option<String>,
    #[serde(rename = "StartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_position: Option<Position>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Position")]
pub struct Position {
    #[serde(rename = "Column")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    #[serde(rename = "Line")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OrganizationsDecisionDetail")]
pub struct OrganizationsDecisionDetail {
    #[serde(rename = "AllowedByOrganizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_by_organizations: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PermissionsBoundaryDecisionDetail")]
pub struct PermissionsBoundaryDecisionDetail {
    #[serde(rename = "AllowedByPermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_by_permissions_boundary: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceSpecificResultListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceSpecificResult>,
}
impl From<Vec<ResourceSpecificResult>> for ResourceSpecificResultListType {
    fn from(v: Vec<ResourceSpecificResult>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceSpecificResult> for ResourceSpecificResultListType {
    fn from_iter<I: IntoIterator<Item = ResourceSpecificResult>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceSpecificResult>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceSpecificResultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceSpecificResult>,
}

impl From<Vec<ResourceSpecificResult>> for XmlResourceSpecificResultList {
    fn from(v: Vec<ResourceSpecificResult>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceSpecificResult> for XmlResourceSpecificResultList {
    fn from_iter<I: IntoIterator<Item = ResourceSpecificResult>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceSpecificResult")]
pub struct ResourceSpecificResult {
    #[serde(rename = "EvalDecisionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_decision_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EvalResourceDecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_resource_decision: Option<String>,
    #[serde(rename = "EvalResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_resource_name: Option<String>,
    #[serde(rename = "MatchedStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_statements: Option<StatementListType>,
    #[serde(rename = "MissingContextValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_context_values: Option<ContextKeyNamesResultListType>,
    #[serde(rename = "PermissionsBoundaryDecisionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_decision_detail: Option<PermissionsBoundaryDecisionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDelegationRequestResult")]
pub struct GetDelegationRequestResponse {
    #[serde(rename = "DelegationRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_request: Option<DelegationRequest>,
    #[serde(rename = "PermissionCheckResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_check_result: Option<String>,
    #[serde(rename = "PermissionCheckStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_check_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServiceLinkedRoleDeletionStatusRequest")]
pub struct GetServiceLinkedRoleDeletionStatusRequest {
    #[serde(rename = "DeletionTaskId")]
    #[serde(default)]
    pub deletion_task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGroupsForUserRequest")]
pub struct ListGroupsForUserRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMFADeviceTagsResult")]
pub struct ListMFADeviceTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagServerCertificateRequest")]
pub struct UntagServerCertificateRequest {
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    pub server_certificate_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessKeysRequest")]
pub struct ListAccessKeysRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GenerateOrganizationsAccessReportRequest")]
pub struct GenerateOrganizationsAccessReportRequest {
    #[serde(rename = "EntityPath")]
    #[serde(default)]
    pub entity_path: String,
    #[serde(rename = "OrganizationsPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations_policy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAttachedGroupPoliciesResult")]
pub struct ListAttachedGroupPoliciesResponse {
    #[serde(rename = "AttachedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_policies: Option<attachedPoliciesListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSAMLProviderRequest")]
pub struct GetSAMLProviderRequest {
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    pub s_a_m_l_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GenerateServiceLastAccessedDetailsResult")]
pub struct GenerateServiceLastAccessedDetailsResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GenerateCredentialReportResult")]
pub struct GenerateCredentialReportResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDelegationRequestsRequest")]
pub struct ListDelegationRequestsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGroupResult")]
pub struct GetGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<userListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct userListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<User>,
}
impl From<Vec<User>> for userListType {
    fn from(v: Vec<User>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<User> for userListType {
    fn from_iter<I: IntoIterator<Item = User>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<User>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUserList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<User>,
}

impl From<Vec<User>> for XmlUserList {
    fn from(v: Vec<User>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<User> for XmlUserList {
    fn from_iter<I: IntoIterator<Item = User>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListUsersResult")]
pub struct ListUsersResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<userListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadServerCertificateRequest")]
pub struct UploadServerCertificateRequest {
    #[serde(rename = "CertificateBody")]
    #[serde(default)]
    pub certificate_body: String,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    pub private_key: String,
    #[serde(rename = "ServerCertificateName")]
    #[serde(default)]
    pub server_certificate_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGroupResult")]
pub struct CreateGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGroupRequest")]
pub struct DeleteGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOutboundWebIdentityFederationInfoResult")]
pub struct GetOutboundWebIdentityFederationInfoResponse {
    #[serde(rename = "IssuerIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_identifier: Option<String>,
    #[serde(rename = "JwtVendingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_vending_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationsFeaturesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAttachedRolePoliciesResult")]
pub struct ListAttachedRolePoliciesResponse {
    #[serde(rename = "AttachedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_policies: Option<attachedPoliciesListType>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRolePoliciesResult")]
pub struct ListRolePoliciesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<policyNameListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDelegationRequestRequest")]
pub struct UpdateDelegationRequestRequest {
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    pub delegation_request_id: String,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGroupPolicyResult")]
pub struct GetGroupPolicyResponse {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServiceSpecificCredentialRequest")]
pub struct DeleteServiceSpecificCredentialRequest {
    #[serde(rename = "ServiceSpecificCredentialId")]
    #[serde(default)]
    pub service_specific_credential_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAttachedUserPoliciesRequest")]
pub struct ListAttachedUserPoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetSecurityTokenServicePreferencesRequest")]
pub struct SetSecurityTokenServicePreferencesRequest {
    #[serde(rename = "GlobalEndpointTokenVersion")]
    #[serde(default)]
    pub global_endpoint_token_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOpenIDConnectProviderResult")]
pub struct GetOpenIDConnectProviderResponse {
    #[serde(rename = "ClientIDList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_i_d_list: Option<clientIDListType>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "ThumbprintList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint_list: Option<thumbprintListType>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutGroupPolicyRequest")]
pub struct PutGroupPolicyRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetRolePolicyRequest")]
pub struct GetRolePolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListServiceSpecificCredentialsResult")]
pub struct ListServiceSpecificCredentialsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ServiceSpecificCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specific_credentials: Option<ServiceSpecificCredentialsListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSpecificCredentialsListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceSpecificCredentialMetadata>,
}
impl From<Vec<ServiceSpecificCredentialMetadata>> for ServiceSpecificCredentialsListType {
    fn from(v: Vec<ServiceSpecificCredentialMetadata>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ServiceSpecificCredentialMetadata> for ServiceSpecificCredentialsListType {
    fn from_iter<I: IntoIterator<Item = ServiceSpecificCredentialMetadata>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ServiceSpecificCredentialMetadata>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlServiceSpecificCredentialMetadataList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ServiceSpecificCredentialMetadata>,
}

impl From<Vec<ServiceSpecificCredentialMetadata>> for XmlServiceSpecificCredentialMetadataList {
    fn from(v: Vec<ServiceSpecificCredentialMetadata>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ServiceSpecificCredentialMetadata> for XmlServiceSpecificCredentialMetadataList {
    fn from_iter<I: IntoIterator<Item = ServiceSpecificCredentialMetadata>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServiceSpecificCredentialMetadata")]
pub struct ServiceSpecificCredentialMetadata {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "ServiceCredentialAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_credential_alias: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceSpecificCredentialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specific_credential_id: Option<String>,
    #[serde(rename = "ServiceUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_user_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RejectDelegationRequestRequest")]
pub struct RejectDelegationRequestRequest {
    #[serde(rename = "DelegationRequestId")]
    #[serde(default)]
    pub delegation_request_id: String,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachGroupPolicyRequest")]
pub struct DetachGroupPolicyRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagSAMLProviderRequest")]
pub struct TagSAMLProviderRequest {
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    pub s_a_m_l_provider_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPolicyResult")]
pub struct GetPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSAMLProviderRequest")]
pub struct DeleteSAMLProviderRequest {
    #[serde(rename = "SAMLProviderArn")]
    #[serde(default)]
    pub s_a_m_l_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAccountPasswordPolicyRequest")]
pub struct UpdateAccountPasswordPolicyRequest {
    #[serde(rename = "AllowUsersToChangePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_users_to_change_password: Option<bool>,
    #[serde(rename = "HardExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_expiry: Option<bool>,
    #[serde(rename = "MaxPasswordAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_password_age: Option<i32>,
    #[serde(rename = "MinimumPasswordLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_password_length: Option<i32>,
    #[serde(rename = "PasswordReusePrevention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_reuse_prevention: Option<i32>,
    #[serde(rename = "RequireLowercaseCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase_characters: Option<bool>,
    #[serde(rename = "RequireNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    #[serde(rename = "RequireSymbols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,
    #[serde(rename = "RequireUppercaseCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase_characters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteRolePolicyRequest")]
pub struct DeleteRolePolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOpenIDConnectProviderTagsResult")]
pub struct ListOpenIDConnectProviderTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccountSummaryResult")]
pub struct GetAccountSummaryResponse {
    #[serde(rename = "SummaryMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_map: Option<std::collections::HashMap<String, i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagOpenIDConnectProviderRequest")]
pub struct UntagOpenIDConnectProviderRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGroupRequest")]
pub struct GetGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSAMLProviderResult")]
pub struct GetSAMLProviderResponse {
    #[serde(rename = "AssertionEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_encryption_mode: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "PrivateKeyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_list: Option<privateKeyList>,
    #[serde(rename = "SAMLMetadataDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_metadata_document: Option<String>,
    #[serde(rename = "SAMLProviderUUID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_provider_u_u_i_d: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
    #[serde(rename = "ValidUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct privateKeyList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SAMLPrivateKey>,
}
impl From<Vec<SAMLPrivateKey>> for privateKeyList {
    fn from(v: Vec<SAMLPrivateKey>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SAMLPrivateKey> for privateKeyList {
    fn from_iter<I: IntoIterator<Item = SAMLPrivateKey>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SAMLPrivateKey>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSAMLPrivateKeyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SAMLPrivateKey>,
}

impl From<Vec<SAMLPrivateKey>> for XmlSAMLPrivateKeyList {
    fn from(v: Vec<SAMLPrivateKey>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SAMLPrivateKey> for XmlSAMLPrivateKeyList {
    fn from_iter<I: IntoIterator<Item = SAMLPrivateKey>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SAMLPrivateKey")]
pub struct SAMLPrivateKey {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessKeyResult")]
pub struct CreateAccessKeyResponse {
    #[serde(rename = "AccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<AccessKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessKey")]
pub struct AccessKey {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetDefaultPolicyVersionRequest")]
pub struct SetDefaultPolicyVersionRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    pub version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveRoleFromInstanceProfileRequest")]
pub struct RemoveRoleFromInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPoliciesResult")]
pub struct ListPoliciesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<policyListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct policyListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Policy>,
}
impl From<Vec<Policy>> for policyListType {
    fn from(v: Vec<Policy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Policy> for policyListType {
    fn from_iter<I: IntoIterator<Item = Policy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Policy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Policy>,
}

impl From<Vec<Policy>> for XmlPolicyList {
    fn from(v: Vec<Policy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Policy> for XmlPolicyList {
    fn from_iter<I: IntoIterator<Item = Policy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteLoginProfileRequest")]
pub struct DeleteLoginProfileRequest {
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutUserPolicyRequest")]
pub struct PutUserPolicyRequest {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagRoleRequest")]
pub struct TagRoleRequest {
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: tagListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessKeyRequest")]
pub struct DeleteAccessKeyRequest {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePolicyRequest")]
pub struct DeletePolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRolesResult")]
pub struct ListRolesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<roleListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GenerateServiceLastAccessedDetailsRequest")]
pub struct GenerateServiceLastAccessedDetailsRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessKeyLastUsedResult")]
pub struct GetAccessKeyLastUsedResponse {
    #[serde(rename = "AccessKeyLastUsed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_last_used: Option<AccessKeyLastUsed>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessKeyLastUsed")]
pub struct AccessKeyLastUsed {
    #[serde(rename = "LastUsedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_date: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServiceLinkedRoleResult")]
pub struct DeleteServiceLinkedRoleResponse {
    #[serde(rename = "DeletionTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMFADeviceTagsRequest")]
pub struct ListMFADeviceTagsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagMFADeviceRequest")]
pub struct UntagMFADeviceRequest {
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: tagKeyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationsRootSessionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListEntitiesForPolicyRequest")]
pub struct ListEntitiesForPolicyRequest {
    #[serde(rename = "EntityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_filter: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "PathPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "PolicyUsageFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_usage_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRoleTagsRequest")]
pub struct ListRoleTagsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListEntitiesForPolicyResult")]
pub struct ListEntitiesForPolicyResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PolicyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_groups: Option<PolicyGroupListType>,
    #[serde(rename = "PolicyRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_roles: Option<PolicyRoleListType>,
    #[serde(rename = "PolicyUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_users: Option<PolicyUserListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyGroupListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyGroup>,
}
impl From<Vec<PolicyGroup>> for PolicyGroupListType {
    fn from(v: Vec<PolicyGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyGroup> for PolicyGroupListType {
    fn from_iter<I: IntoIterator<Item = PolicyGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyGroup>,
}

impl From<Vec<PolicyGroup>> for XmlPolicyGroupList {
    fn from(v: Vec<PolicyGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyGroup> for XmlPolicyGroupList {
    fn from_iter<I: IntoIterator<Item = PolicyGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyGroup")]
pub struct PolicyGroup {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyRoleListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyRole>,
}
impl From<Vec<PolicyRole>> for PolicyRoleListType {
    fn from(v: Vec<PolicyRole>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyRole> for PolicyRoleListType {
    fn from_iter<I: IntoIterator<Item = PolicyRole>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyRole>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyRoleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyRole>,
}

impl From<Vec<PolicyRole>> for XmlPolicyRoleList {
    fn from(v: Vec<PolicyRole>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyRole> for XmlPolicyRoleList {
    fn from_iter<I: IntoIterator<Item = PolicyRole>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyRole")]
pub struct PolicyRole {
    #[serde(rename = "RoleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyUserListType {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyUser>,
}
impl From<Vec<PolicyUser>> for PolicyUserListType {
    fn from(v: Vec<PolicyUser>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyUser> for PolicyUserListType {
    fn from_iter<I: IntoIterator<Item = PolicyUser>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyUser>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyUserList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyUser>,
}

impl From<Vec<PolicyUser>> for XmlPolicyUserList {
    fn from(v: Vec<PolicyUser>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyUser> for XmlPolicyUserList {
    fn from_iter<I: IntoIterator<Item = PolicyUser>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyUser")]
pub struct PolicyUser {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetContextKeysForCustomPolicyRequest")]
pub struct GetContextKeysForCustomPolicyRequest {
    #[serde(rename = "PolicyInputList")]
    #[serde(default)]
    pub policy_input_list: SimulationPolicyListType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateSigningCertificateRequest")]
pub struct UpdateSigningCertificateRequest {
    #[serde(rename = "CertificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteOpenIDConnectProviderRequest")]
pub struct DeleteOpenIDConnectProviderRequest {
    #[serde(rename = "OpenIDConnectProviderArn")]
    #[serde(default)]
    pub open_i_d_connect_provider_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetLoginProfileResult")]
pub struct GetLoginProfileResponse {
    #[serde(rename = "LoginProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_profile: Option<LoginProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServerCertificateResult")]
pub struct GetServerCertificateResponse {
    #[serde(rename = "ServerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<ServerCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerCertificate")]
pub struct ServerCertificate {
    #[serde(rename = "CertificateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_body: Option<String>,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(rename = "ServerCertificateMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_metadata: Option<ServerCertificateMetadata>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetUserPolicyResult")]
pub struct GetUserPolicyResponse {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListUserTagsResult")]
pub struct ListUserTagsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetSSHPublicKeyRequest")]
pub struct GetSSHPublicKeyRequest {
    #[serde(rename = "Encoding")]
    #[serde(default)]
    pub encoding: String,
    #[serde(rename = "SSHPublicKeyId")]
    #[serde(default)]
    pub s_s_h_public_key_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachRolePolicyRequest")]
pub struct AttachRolePolicyRequest {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    pub role_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpenIDConnectProvidersRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDelegationRequestRequest")]
pub struct CreateDelegationRequestRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    pub notification_channel: String,
    #[serde(rename = "OnlySendByOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_send_by_owner: Option<bool>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: DelegationPermission,
    #[serde(rename = "RedirectUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "RequestMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[serde(rename = "RequestorWorkflowId")]
    #[serde(default)]
    pub requestor_workflow_id: String,
    #[serde(rename = "SessionDuration")]
    #[serde(default)]
    pub session_duration: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateInstanceProfileRequest")]
pub struct CreateInstanceProfileRequest {
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    pub instance_profile_name: String,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<tagListType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetServiceLastAccessedDetailsResult")]
pub struct GetServiceLastAccessedDetailsResponse {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "JobCompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_completion_date: Option<String>,
    #[serde(rename = "JobCreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_creation_date: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ServicesLastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services_last_accessed: Option<ServicesLastAccessed>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServicesLastAccessed {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceLastAccessed>,
}
impl From<Vec<ServiceLastAccessed>> for ServicesLastAccessed {
    fn from(v: Vec<ServiceLastAccessed>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ServiceLastAccessed> for ServicesLastAccessed {
    fn from_iter<I: IntoIterator<Item = ServiceLastAccessed>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ServiceLastAccessed>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlServiceLastAccessedList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ServiceLastAccessed>,
}

impl From<Vec<ServiceLastAccessed>> for XmlServiceLastAccessedList {
    fn from(v: Vec<ServiceLastAccessed>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ServiceLastAccessed> for XmlServiceLastAccessedList {
    fn from_iter<I: IntoIterator<Item = ServiceLastAccessed>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServiceLastAccessed")]
pub struct ServiceLastAccessed {
    #[serde(rename = "LastAuthenticated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authenticated: Option<String>,
    #[serde(rename = "LastAuthenticatedEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authenticated_entity: Option<String>,
    #[serde(rename = "LastAuthenticatedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authenticated_region: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(rename = "TotalAuthenticatedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_authenticated_entities: Option<i32>,
    #[serde(rename = "TrackedActionsLastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked_actions_last_accessed: Option<TrackedActionsLastAccessed>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrackedActionsLastAccessed {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TrackedActionLastAccessed>,
}
impl From<Vec<TrackedActionLastAccessed>> for TrackedActionsLastAccessed {
    fn from(v: Vec<TrackedActionLastAccessed>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrackedActionLastAccessed> for TrackedActionsLastAccessed {
    fn from_iter<I: IntoIterator<Item = TrackedActionLastAccessed>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrackedActionLastAccessed>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrackedActionLastAccessedList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrackedActionLastAccessed>,
}

impl From<Vec<TrackedActionLastAccessed>> for XmlTrackedActionLastAccessedList {
    fn from(v: Vec<TrackedActionLastAccessed>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrackedActionLastAccessed> for XmlTrackedActionLastAccessedList {
    fn from_iter<I: IntoIterator<Item = TrackedActionLastAccessed>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrackedActionLastAccessed")]
pub struct TrackedActionLastAccessed {
    #[serde(rename = "ActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "LastAccessedEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_entity: Option<String>,
    #[serde(rename = "LastAccessedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_region: Option<String>,
    #[serde(rename = "LastAccessedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_time: Option<String>,
}
