//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-identitystore

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupMembershipRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: MemberId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberId {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupMembershipResponse {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "MembershipId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupResponse {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    #[serde(rename = "Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(rename = "Birthdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Emails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Email>>,
    #[serde(rename = "Extensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(rename = "NickName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(rename = "PhoneNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    #[serde(rename = "Photos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<Photo>>,
    #[serde(rename = "PreferredLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "ProfileUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "UserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(rename = "Website")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "Formatted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(rename = "Locality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(rename = "PostalCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StreetAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Email {
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
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
pub struct Name {
    #[serde(rename = "FamilyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(rename = "Formatted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(rename = "GivenName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "HonorificPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    #[serde(rename = "HonorificSuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
    #[serde(rename = "MiddleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumber {
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
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
pub struct Photo {
    #[serde(rename = "Display")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Role {
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
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
pub struct CreateUserResponse {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupMembershipRequest {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "MembershipId")]
    #[serde(default)]
    pub membership_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupMembershipResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupMembershipRequest {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "MembershipId")]
    #[serde(default)]
    pub membership_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupMembershipResponse {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<MemberId>,
    #[serde(rename = "MembershipId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupResponse {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ExternalIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<ExternalId>>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalId {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    pub issuer: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserRequest {
    #[serde(rename = "Extensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserResponse {
    #[serde(rename = "Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(rename = "Birthdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Emails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Email>>,
    #[serde(rename = "Extensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "ExternalIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<ExternalId>>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(rename = "NickName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(rename = "PhoneNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    #[serde(rename = "Photos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<Photo>>,
    #[serde(rename = "PreferredLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "ProfileUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    #[serde(rename = "UserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(rename = "Website")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupIdRequest {
    #[serde(rename = "AlternateIdentifier")]
    #[serde(default)]
    pub alternate_identifier: AlternateIdentifier,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlternateIdentifier {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<ExternalId>,
    #[serde(rename = "UniqueAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_attribute: Option<UniqueAttribute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UniqueAttribute {
    #[serde(rename = "AttributePath")]
    #[serde(default)]
    pub attribute_path: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    pub attribute_value: serde_json::Value,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupIdResponse {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupMembershipIdRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: MemberId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupMembershipIdResponse {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "MembershipId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserIdRequest {
    #[serde(rename = "AlternateIdentifier")]
    #[serde(default)]
    pub alternate_identifier: AlternateIdentifier,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserIdResponse {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IsMemberInGroupsRequest {
    #[serde(rename = "GroupIds")]
    #[serde(default)]
    pub group_ids: Vec<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: MemberId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IsMemberInGroupsResponse {
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GroupMembershipExistenceResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupMembershipExistenceResult {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<MemberId>,
    #[serde(rename = "MembershipExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_exists: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupMembershipsForMemberRequest {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: MemberId,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupMembershipsForMemberResponse {
    #[serde(rename = "GroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_memberships: Option<Vec<GroupMembership>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupMembership {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<MemberId>,
    #[serde(rename = "MembershipId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupMembershipsRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
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
pub struct ListGroupMembershipsResponse {
    #[serde(rename = "GroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_memberships: Option<Vec<GroupMembership>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
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
pub struct Filter {
    #[serde(rename = "AttributePath")]
    #[serde(default)]
    pub attribute_path: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    pub attribute_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ExternalIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<ExternalId>>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersRequest {
    #[serde(rename = "Extensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
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
pub struct ListUsersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(rename = "Birthdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Emails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Email>>,
    #[serde(rename = "Extensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "ExternalIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<ExternalId>>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(rename = "NickName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(rename = "PhoneNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    #[serde(rename = "Photos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<Photo>>,
    #[serde(rename = "PreferredLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "ProfileUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    #[serde(rename = "UserType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(rename = "Website")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupRequest {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "Operations")]
    #[serde(default)]
    pub operations: Vec<AttributeOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeOperation {
    #[serde(rename = "AttributePath")]
    #[serde(default)]
    pub attribute_path: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    pub identity_store_id: String,
    #[serde(rename = "Operations")]
    #[serde(default)]
    pub operations: Vec<AttributeOperation>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserResponse {}
