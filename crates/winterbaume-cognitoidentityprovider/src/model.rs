//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cognitoidentityprovider

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddCustomAttributesRequest {
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    pub custom_attributes: Vec<SchemaAttributeType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaAttributeType {
    #[serde(rename = "AttributeDataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_data_type: Option<String>,
    #[serde(rename = "DeveloperOnlyAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_only_attribute: Option<bool>,
    #[serde(rename = "Mutable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberAttributeConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_attribute_constraints: Option<NumberAttributeConstraintsType>,
    #[serde(rename = "Required")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "StringAttributeConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_attribute_constraints: Option<StringAttributeConstraintsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberAttributeConstraintsType {
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringAttributeConstraintsType {
    #[serde(rename = "MaxLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(rename = "MinLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddCustomAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddUserPoolClientSecretRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddUserPoolClientSecretResponse {
    #[serde(rename = "ClientSecretDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_descriptor: Option<ClientSecretDescriptorType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientSecretDescriptorType {
    #[serde(rename = "ClientSecretCreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_create_date: Option<f64>,
    #[serde(rename = "ClientSecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_id: Option<String>,
    #[serde(rename = "ClientSecretValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminAddUserToGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminConfirmSignUpRequest {
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminConfirmSignUpResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminCreateUserRequest {
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DesiredDeliveryMediums")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_delivery_mediums: Option<Vec<String>>,
    #[serde(rename = "ForceAliasCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_alias_creation: Option<bool>,
    #[serde(rename = "MessageAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_action: Option<String>,
    #[serde(rename = "TemporaryPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_password: Option<String>,
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
    #[serde(rename = "ValidationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<Vec<AttributeType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeType {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminCreateUserResponse {
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserType {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeType>>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "MFAOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a_options: Option<Vec<MFAOptionType>>,
    #[serde(rename = "UserCreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_create_date: Option<f64>,
    #[serde(rename = "UserLastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_last_modified_date: Option<f64>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MFAOptionType {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "DeliveryMedium")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_medium: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminDeleteUserAttributesRequest {
    #[serde(rename = "UserAttributeNames")]
    #[serde(default)]
    pub user_attribute_names: Vec<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminDeleteUserAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminDeleteUserRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminDisableProviderForUserRequest {
    #[serde(rename = "User")]
    #[serde(default)]
    pub user: ProviderUserIdentifierType,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProviderUserIdentifierType {
    #[serde(rename = "ProviderAttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_attribute_name: Option<String>,
    #[serde(rename = "ProviderAttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_attribute_value: Option<String>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminDisableProviderForUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminDisableUserRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminDisableUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminEnableUserRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminEnableUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminForgetDeviceRequest {
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    pub device_key: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminGetDeviceRequest {
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    pub device_key: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminGetDeviceResponse {
    #[serde(rename = "Device")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceType {
    #[serde(rename = "DeviceAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_attributes: Option<Vec<AttributeType>>,
    #[serde(rename = "DeviceCreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_create_date: Option<f64>,
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_key: Option<String>,
    #[serde(rename = "DeviceLastAuthenticatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_authenticated_date: Option<f64>,
    #[serde(rename = "DeviceLastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_modified_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminGetUserRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminGetUserResponse {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "MFAOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a_options: Option<Vec<MFAOptionType>>,
    #[serde(rename = "PreferredMfaSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa_setting: Option<String>,
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    #[serde(rename = "UserCreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_create_date: Option<f64>,
    #[serde(rename = "UserLastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_last_modified_date: Option<f64>,
    #[serde(rename = "UserMFASettingList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_m_f_a_setting_list: Option<Vec<String>>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminInitiateAuthRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "AuthFlow")]
    #[serde(default)]
    pub auth_flow: String,
    #[serde(rename = "AuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_data: Option<ContextDataType>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsMetadataType {
    #[serde(rename = "AnalyticsEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextDataType {
    #[serde(rename = "EncodedData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_data: Option<String>,
    #[serde(rename = "HttpHeaders")]
    #[serde(default)]
    pub http_headers: Vec<HttpHeader>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    pub ip_address: String,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    pub server_name: String,
    #[serde(rename = "ServerPath")]
    #[serde(default)]
    pub server_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpHeader {
    #[serde(rename = "headerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    #[serde(rename = "headerValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminInitiateAuthResponse {
    #[serde(rename = "AuthenticationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[serde(rename = "AvailableChallenges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_challenges: Option<Vec<String>>,
    #[serde(rename = "ChallengeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    #[serde(rename = "ChallengeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationResultType {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "ExpiresIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    #[serde(rename = "IdToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    #[serde(rename = "NewDeviceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_device_metadata: Option<NewDeviceMetadataType>,
    #[serde(rename = "RefreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "TokenType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewDeviceMetadataType {
    #[serde(rename = "DeviceGroupKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_group_key: Option<String>,
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminLinkProviderForUserRequest {
    #[serde(rename = "DestinationUser")]
    #[serde(default)]
    pub destination_user: ProviderUserIdentifierType,
    #[serde(rename = "SourceUser")]
    #[serde(default)]
    pub source_user: ProviderUserIdentifierType,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminLinkProviderForUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminListDevicesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminListDevicesResponse {
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceType>>,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminListGroupsForUserRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminListGroupsForUserResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupType>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupType {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Precedence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i32>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminListUserAuthEventsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminListUserAuthEventsResponse {
    #[serde(rename = "AuthEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_events: Option<Vec<AuthEventType>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthEventType {
    #[serde(rename = "ChallengeResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_responses: Option<Vec<ChallengeResponseType>>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "EventContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_context_data: Option<EventContextDataType>,
    #[serde(rename = "EventFeedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_feedback: Option<EventFeedbackType>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "EventResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_response: Option<String>,
    #[serde(rename = "EventRisk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_risk: Option<EventRiskType>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChallengeResponseType {
    #[serde(rename = "ChallengeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    #[serde(rename = "ChallengeResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_response: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventContextDataType {
    #[serde(rename = "City")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventFeedbackType {
    #[serde(rename = "FeedbackDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_date: Option<f64>,
    #[serde(rename = "FeedbackValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_value: Option<String>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventRiskType {
    #[serde(rename = "CompromisedCredentialsDetected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_credentials_detected: Option<bool>,
    #[serde(rename = "RiskDecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_decision: Option<String>,
    #[serde(rename = "RiskLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminRemoveUserFromGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminResetUserPasswordRequest {
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminResetUserPasswordResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminRespondToAuthChallengeRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "ChallengeName")]
    #[serde(default)]
    pub challenge_name: String,
    #[serde(rename = "ChallengeResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_responses: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_data: Option<ContextDataType>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminRespondToAuthChallengeResponse {
    #[serde(rename = "AuthenticationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[serde(rename = "ChallengeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    #[serde(rename = "ChallengeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminSetUserMFAPreferenceRequest {
    #[serde(rename = "EmailMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mfa_settings: Option<EmailMfaSettingsType>,
    #[serde(rename = "SMSMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_mfa_settings: Option<SMSMfaSettingsType>,
    #[serde(rename = "SoftwareTokenMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_settings: Option<SoftwareTokenMfaSettingsType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
    #[serde(rename = "WebAuthnMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_mfa_settings: Option<WebAuthnMfaSettingsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailMfaSettingsType {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PreferredMfa")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSMfaSettingsType {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PreferredMfa")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SoftwareTokenMfaSettingsType {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PreferredMfa")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebAuthnMfaSettingsType {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminSetUserMFAPreferenceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminSetUserPasswordRequest {
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "Permanent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminSetUserPasswordResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminSetUserSettingsRequest {
    #[serde(rename = "MFAOptions")]
    #[serde(default)]
    pub m_f_a_options: Vec<MFAOptionType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminSetUserSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUpdateAuthEventFeedbackRequest {
    #[serde(rename = "EventId")]
    #[serde(default)]
    pub event_id: String,
    #[serde(rename = "FeedbackValue")]
    #[serde(default)]
    pub feedback_value: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUpdateAuthEventFeedbackResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUpdateDeviceStatusRequest {
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    pub device_key: String,
    #[serde(rename = "DeviceRememberedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_remembered_status: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUpdateDeviceStatusResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUpdateUserAttributesRequest {
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    pub user_attributes: Vec<AttributeType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUpdateUserAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUserGlobalSignOutRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminUserGlobalSignOutResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSoftwareTokenRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSoftwareTokenResponse {
    #[serde(rename = "SecretCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_code: Option<String>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "PreviousPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_password: Option<String>,
    #[serde(rename = "ProposedPassword")]
    #[serde(default)]
    pub proposed_password: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangePasswordResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteWebAuthnRegistrationRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "Credential")]
    #[serde(default)]
    pub credential: serde_json::Value,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteWebAuthnRegistrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmDeviceRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    pub device_key: String,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "DeviceSecretVerifierConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_secret_verifier_config: Option<DeviceSecretVerifierConfigType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceSecretVerifierConfigType {
    #[serde(rename = "PasswordVerifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_verifier: Option<String>,
    #[serde(rename = "Salt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmDeviceResponse {
    #[serde(rename = "UserConfirmationNecessary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_confirmation_necessary: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmForgotPasswordRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConfirmationCode")]
    #[serde(default)]
    pub confirmation_code: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "SecretHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    #[serde(rename = "UserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserContextDataType {
    #[serde(rename = "EncodedData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_data: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmForgotPasswordResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmSignUpRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConfirmationCode")]
    #[serde(default)]
    pub confirmation_code: String,
    #[serde(rename = "ForceAliasCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_alias_creation: Option<bool>,
    #[serde(rename = "SecretHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "UserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmSignUpResponse {
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Precedence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i32>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIdentityProviderRequest {
    #[serde(rename = "AttributeMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "IdpIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    #[serde(rename = "ProviderDetails")]
    #[serde(default)]
    pub provider_details: std::collections::HashMap<String, String>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    pub provider_name: String,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    pub provider_type: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIdentityProviderResponse {
    #[serde(rename = "IdentityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<IdentityProviderType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityProviderType {
    #[serde(rename = "AttributeMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "IdpIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "ProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateManagedLoginBrandingRequest {
    #[serde(rename = "Assets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetType>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "UseCognitoProvidedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cognito_provided_values: Option<bool>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetType {
    #[serde(rename = "Bytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(rename = "Category")]
    #[serde(default)]
    pub category: String,
    #[serde(rename = "ColorMode")]
    #[serde(default)]
    pub color_mode: String,
    #[serde(rename = "Extension")]
    #[serde(default)]
    pub extension: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateManagedLoginBrandingResponse {
    #[serde(rename = "ManagedLoginBranding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_branding: Option<ManagedLoginBrandingType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedLoginBrandingType {
    #[serde(rename = "Assets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetType>>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "ManagedLoginBrandingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_branding_id: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "UseCognitoProvidedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cognito_provided_values: Option<bool>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceServerRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceServerScopeType {
    #[serde(rename = "ScopeDescription")]
    #[serde(default)]
    pub scope_description: String,
    #[serde(rename = "ScopeName")]
    #[serde(default)]
    pub scope_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceServerResponse {
    #[serde(rename = "ResourceServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server: Option<ResourceServerType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceServerType {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTermsRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "Enforcement")]
    #[serde(default)]
    pub enforcement: String,
    #[serde(rename = "Links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TermsName")]
    #[serde(default)]
    pub terms_name: String,
    #[serde(rename = "TermsSource")]
    #[serde(default)]
    pub terms_source: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTermsResponse {
    #[serde(rename = "Terms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<TermsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TermsType {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Enforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TermsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_id: Option<String>,
    #[serde(rename = "TermsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_name: Option<String>,
    #[serde(rename = "TermsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_source: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserImportJobRequest {
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    pub cloud_watch_logs_role_arn: String,
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserImportJobResponse {
    #[serde(rename = "UserImportJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserImportJobType {
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CompletionMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_message: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "FailedUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_users: Option<i64>,
    #[serde(rename = "ImportedUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_users: Option<i64>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "SkippedUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_users: Option<i64>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserPoolClientRequest {
    #[serde(rename = "AccessTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_validity: Option<i32>,
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    #[serde(rename = "AllowedOAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfigurationType>,
    #[serde(rename = "AuthSessionValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_session_validity: Option<i32>,
    #[serde(rename = "CallbackURLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_u_r_ls: Option<Vec<String>>,
    #[serde(rename = "ClientName")]
    #[serde(default)]
    pub client_name: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "DefaultRedirectURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_redirect_u_r_i: Option<String>,
    #[serde(rename = "EnablePropagateAdditionalUserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_propagate_additional_user_context_data: Option<bool>,
    #[serde(rename = "EnableTokenRevocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_token_revocation: Option<bool>,
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    #[serde(rename = "GenerateSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_secret: Option<bool>,
    #[serde(rename = "IdTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_validity: Option<i32>,
    #[serde(rename = "LogoutURLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_u_r_ls: Option<Vec<String>>,
    #[serde(rename = "PreventUserExistenceErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_user_existence_errors: Option<String>,
    #[serde(rename = "ReadAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    #[serde(rename = "RefreshTokenRotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_rotation: Option<RefreshTokenRotationType>,
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i32>,
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    #[serde(rename = "TokenValidityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_validity_units: Option<TokenValidityUnitsType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "WriteAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsConfigurationType {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "UserDataShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data_shared: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshTokenRotationType {
    #[serde(rename = "Feature")]
    #[serde(default)]
    pub feature: String,
    #[serde(rename = "RetryGracePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_grace_period_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TokenValidityUnitsType {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "IdToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    #[serde(rename = "RefreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserPoolClientResponse {
    #[serde(rename = "UserPoolClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPoolClientType {
    #[serde(rename = "AccessTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_validity: Option<i32>,
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    #[serde(rename = "AllowedOAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfigurationType>,
    #[serde(rename = "AuthSessionValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_session_validity: Option<i32>,
    #[serde(rename = "CallbackURLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_u_r_ls: Option<Vec<String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "ClientName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DefaultRedirectURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_redirect_u_r_i: Option<String>,
    #[serde(rename = "EnablePropagateAdditionalUserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_propagate_additional_user_context_data: Option<bool>,
    #[serde(rename = "EnableTokenRevocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_token_revocation: Option<bool>,
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    #[serde(rename = "IdTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_validity: Option<i32>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "LogoutURLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_u_r_ls: Option<Vec<String>>,
    #[serde(rename = "PreventUserExistenceErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_user_existence_errors: Option<String>,
    #[serde(rename = "ReadAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    #[serde(rename = "RefreshTokenRotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_rotation: Option<RefreshTokenRotationType>,
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i32>,
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    #[serde(rename = "TokenValidityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_validity_units: Option<TokenValidityUnitsType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "WriteAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserPoolDomainRequest {
    #[serde(rename = "CustomDomainConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_config: Option<CustomDomainConfigType>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "ManagedLoginVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_version: Option<i32>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDomainConfigType {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserPoolDomainResponse {
    #[serde(rename = "CloudFrontDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_domain: Option<String>,
    #[serde(rename = "ManagedLoginVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserPoolRequest {
    #[serde(rename = "AccountRecoverySetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recovery_setting: Option<AccountRecoverySettingType>,
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    #[serde(rename = "AliasAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_attributes: Option<Vec<String>>,
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<String>,
    #[serde(rename = "DeviceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    #[serde(rename = "EmailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<String>,
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<String>,
    #[serde(rename = "LambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[serde(rename = "MfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    #[serde(rename = "PoolName")]
    #[serde(default)]
    pub pool_name: String,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<SchemaAttributeType>>,
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    #[serde(rename = "SmsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<String>,
    #[serde(rename = "UserAttributeUpdateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute_update_settings: Option<UserAttributeUpdateSettingsType>,
    #[serde(rename = "UserPoolAddOns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_add_ons: Option<UserPoolAddOnsType>,
    #[serde(rename = "UserPoolTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserPoolTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tier: Option<String>,
    #[serde(rename = "UsernameAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_attributes: Option<Vec<String>>,
    #[serde(rename = "UsernameConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_configuration: Option<UsernameConfigurationType>,
    #[serde(rename = "VerificationMessageTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message_template: Option<VerificationMessageTemplateType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountRecoverySettingType {
    #[serde(rename = "RecoveryMechanisms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_mechanisms: Option<Vec<RecoveryOptionType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecoveryOptionType {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminCreateUserConfigType {
    #[serde(rename = "AllowAdminCreateUserOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_admin_create_user_only: Option<bool>,
    #[serde(rename = "InviteMessageTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_message_template: Option<MessageTemplateType>,
    #[serde(rename = "UnusedAccountValidityDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_account_validity_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageTemplateType {
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
    #[serde(rename = "EmailSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "SMSMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceConfigurationType {
    #[serde(rename = "ChallengeRequiredOnNewDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_required_on_new_device: Option<bool>,
    #[serde(rename = "DeviceOnlyRememberedOnUserPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_only_remembered_on_user_prompt: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailConfigurationType {
    #[serde(rename = "ConfigurationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    #[serde(rename = "EmailSendingAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_sending_account: Option<String>,
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "ReplyToEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_email_address: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaConfigType {
    #[serde(rename = "CreateAuthChallenge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_auth_challenge: Option<String>,
    #[serde(rename = "CustomEmailSender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_email_sender: Option<CustomEmailLambdaVersionConfigType>,
    #[serde(rename = "CustomMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[serde(rename = "CustomSMSSender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_s_m_s_sender: Option<CustomSMSLambdaVersionConfigType>,
    #[serde(rename = "DefineAuthChallenge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_auth_challenge: Option<String>,
    #[serde(rename = "InboundFederation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_federation: Option<InboundFederationLambdaType>,
    #[serde(rename = "KMSKeyID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_i_d: Option<String>,
    #[serde(rename = "PostAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_authentication: Option<String>,
    #[serde(rename = "PostConfirmation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_confirmation: Option<String>,
    #[serde(rename = "PreAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_authentication: Option<String>,
    #[serde(rename = "PreSignUp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_sign_up: Option<String>,
    #[serde(rename = "PreTokenGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_token_generation: Option<String>,
    #[serde(rename = "PreTokenGenerationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_token_generation_config: Option<PreTokenGenerationVersionConfigType>,
    #[serde(rename = "UserMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_migration: Option<String>,
    #[serde(rename = "VerifyAuthChallengeResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_auth_challenge_response: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomEmailLambdaVersionConfigType {
    #[serde(rename = "LambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
    #[serde(rename = "LambdaVersion")]
    #[serde(default)]
    pub lambda_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomSMSLambdaVersionConfigType {
    #[serde(rename = "LambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
    #[serde(rename = "LambdaVersion")]
    #[serde(default)]
    pub lambda_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundFederationLambdaType {
    #[serde(rename = "LambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
    #[serde(rename = "LambdaVersion")]
    #[serde(default)]
    pub lambda_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreTokenGenerationVersionConfigType {
    #[serde(rename = "LambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
    #[serde(rename = "LambdaVersion")]
    #[serde(default)]
    pub lambda_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPoolPolicyType {
    #[serde(rename = "PasswordPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<PasswordPolicyType>,
    #[serde(rename = "SignInPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_policy: Option<SignInPolicyType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PasswordPolicyType {
    #[serde(rename = "MinimumLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i32>,
    #[serde(rename = "PasswordHistorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_history_size: Option<i32>,
    #[serde(rename = "RequireLowercase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase: Option<bool>,
    #[serde(rename = "RequireNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    #[serde(rename = "RequireSymbols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_symbols: Option<bool>,
    #[serde(rename = "RequireUppercase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase: Option<bool>,
    #[serde(rename = "TemporaryPasswordValidityDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_password_validity_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignInPolicyType {
    #[serde(rename = "AllowedFirstAuthFactors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_first_auth_factors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmsConfigurationType {
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "SnsCallerArn")]
    #[serde(default)]
    pub sns_caller_arn: String,
    #[serde(rename = "SnsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAttributeUpdateSettingsType {
    #[serde(rename = "AttributesRequireVerificationBeforeUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_require_verification_before_update: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPoolAddOnsType {
    #[serde(rename = "AdvancedSecurityAdditionalFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_additional_flows: Option<AdvancedSecurityAdditionalFlowsType>,
    #[serde(rename = "AdvancedSecurityMode")]
    #[serde(default)]
    pub advanced_security_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedSecurityAdditionalFlowsType {
    #[serde(rename = "CustomAuthMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_auth_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsernameConfigurationType {
    #[serde(rename = "CaseSensitive")]
    #[serde(default)]
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerificationMessageTemplateType {
    #[serde(rename = "DefaultEmailOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_email_option: Option<String>,
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
    #[serde(rename = "EmailMessageByLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_by_link: Option<String>,
    #[serde(rename = "EmailSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "EmailSubjectByLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_subject_by_link: Option<String>,
    #[serde(rename = "SmsMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserPoolResponse {
    #[serde(rename = "UserPool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool: Option<UserPoolType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPoolType {
    #[serde(rename = "AccountRecoverySetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recovery_setting: Option<AccountRecoverySettingType>,
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    #[serde(rename = "AliasAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_attributes: Option<Vec<String>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "CustomDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<String>,
    #[serde(rename = "DeviceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "EmailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    #[serde(rename = "EmailConfigurationFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration_failure: Option<String>,
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<String>,
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<String>,
    #[serde(rename = "EstimatedNumberOfUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_number_of_users: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "MfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    #[serde(rename = "SchemaAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_attributes: Option<Vec<SchemaAttributeType>>,
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    #[serde(rename = "SmsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    #[serde(rename = "SmsConfigurationFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration_failure: Option<String>,
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserAttributeUpdateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute_update_settings: Option<UserAttributeUpdateSettingsType>,
    #[serde(rename = "UserPoolAddOns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_add_ons: Option<UserPoolAddOnsType>,
    #[serde(rename = "UserPoolTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserPoolTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tier: Option<String>,
    #[serde(rename = "UsernameAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_attributes: Option<Vec<String>>,
    #[serde(rename = "UsernameConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_configuration: Option<UsernameConfigurationType>,
    #[serde(rename = "VerificationMessageTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message_template: Option<VerificationMessageTemplateType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityProviderRequest {
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    pub provider_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteManagedLoginBrandingRequest {
    #[serde(rename = "ManagedLoginBrandingId")]
    #[serde(default)]
    pub managed_login_branding_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceServerRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTermsRequest {
    #[serde(rename = "TermsId")]
    #[serde(default)]
    pub terms_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserAttributesRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "UserAttributeNames")]
    #[serde(default)]
    pub user_attribute_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserPoolClientRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserPoolClientSecretRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientSecretId")]
    #[serde(default)]
    pub client_secret_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserPoolClientSecretResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserPoolDomainRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserPoolDomainResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserPoolRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebAuthnCredentialRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "CredentialId")]
    #[serde(default)]
    pub credential_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWebAuthnCredentialResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityProviderRequest {
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    pub provider_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityProviderResponse {
    #[serde(rename = "IdentityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<IdentityProviderType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedLoginBrandingByClientRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ReturnMergedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_merged_resources: Option<bool>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedLoginBrandingByClientResponse {
    #[serde(rename = "ManagedLoginBranding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_branding: Option<ManagedLoginBrandingType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedLoginBrandingRequest {
    #[serde(rename = "ManagedLoginBrandingId")]
    #[serde(default)]
    pub managed_login_branding_id: String,
    #[serde(rename = "ReturnMergedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_merged_resources: Option<bool>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedLoginBrandingResponse {
    #[serde(rename = "ManagedLoginBranding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_branding: Option<ManagedLoginBrandingType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourceServerRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourceServerResponse {
    #[serde(rename = "ResourceServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server: Option<ResourceServerType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRiskConfigurationRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRiskConfigurationResponse {
    #[serde(rename = "RiskConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_configuration: Option<RiskConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RiskConfigurationType {
    #[serde(rename = "AccountTakeoverRiskConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_takeover_risk_configuration: Option<AccountTakeoverRiskConfigurationType>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "CompromisedCredentialsRiskConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_credentials_risk_configuration:
        Option<CompromisedCredentialsRiskConfigurationType>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "RiskExceptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_exception_configuration: Option<RiskExceptionConfigurationType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountTakeoverRiskConfigurationType {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: AccountTakeoverActionsType,
    #[serde(rename = "NotifyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_configuration: Option<NotifyConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountTakeoverActionsType {
    #[serde(rename = "HighAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_action: Option<AccountTakeoverActionType>,
    #[serde(rename = "LowAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_action: Option<AccountTakeoverActionType>,
    #[serde(rename = "MediumAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_action: Option<AccountTakeoverActionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountTakeoverActionType {
    #[serde(rename = "EventAction")]
    #[serde(default)]
    pub event_action: String,
    #[serde(rename = "Notify")]
    #[serde(default)]
    pub notify: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyConfigurationType {
    #[serde(rename = "BlockEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_email: Option<NotifyEmailType>,
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "MfaEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_email: Option<NotifyEmailType>,
    #[serde(rename = "NoActionEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_action_email: Option<NotifyEmailType>,
    #[serde(rename = "ReplyTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyEmailType {
    #[serde(rename = "HtmlBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: String,
    #[serde(rename = "TextBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompromisedCredentialsRiskConfigurationType {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: CompromisedCredentialsActionsType,
    #[serde(rename = "EventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompromisedCredentialsActionsType {
    #[serde(rename = "EventAction")]
    #[serde(default)]
    pub event_action: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RiskExceptionConfigurationType {
    #[serde(rename = "BlockedIPRangeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_i_p_range_list: Option<Vec<String>>,
    #[serde(rename = "SkippedIPRangeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_i_p_range_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTermsRequest {
    #[serde(rename = "TermsId")]
    #[serde(default)]
    pub terms_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTermsResponse {
    #[serde(rename = "Terms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<TermsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserImportJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserImportJobResponse {
    #[serde(rename = "UserImportJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserPoolClientRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserPoolClientResponse {
    #[serde(rename = "UserPoolClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserPoolDomainRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserPoolDomainResponse {
    #[serde(rename = "DomainDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_description: Option<DomainDescriptionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainDescriptionType {
    #[serde(rename = "AWSAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_account_id: Option<String>,
    #[serde(rename = "CloudFrontDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_distribution: Option<String>,
    #[serde(rename = "CustomDomainConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_config: Option<CustomDomainConfigType>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "ManagedLoginVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_version: Option<i32>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserPoolRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserPoolResponse {
    #[serde(rename = "UserPool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool: Option<UserPoolType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForgetDeviceRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    pub device_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForgotPasswordRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SecretHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    #[serde(rename = "UserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForgotPasswordResponse {
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeDeliveryDetailsType {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "DeliveryMedium")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_medium: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCSVHeaderRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCSVHeaderResponse {
    #[serde(rename = "CSVHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_v_header: Option<Vec<String>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    pub device_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeviceResponse {
    #[serde(rename = "Device")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityProviderByIdentifierRequest {
    #[serde(rename = "IdpIdentifier")]
    #[serde(default)]
    pub idp_identifier: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityProviderByIdentifierResponse {
    #[serde(rename = "IdentityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<IdentityProviderType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogDeliveryConfigurationRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLogDeliveryConfigurationResponse {
    #[serde(rename = "LogDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDeliveryConfigurationType {
    #[serde(rename = "LogConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configurations: Option<Vec<LogConfigurationType>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfigurationType {
    #[serde(rename = "CloudWatchLogsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_configuration: Option<CloudWatchLogsConfigurationType>,
    #[serde(rename = "EventSource")]
    #[serde(default)]
    pub event_source: String,
    #[serde(rename = "FirehoseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_configuration: Option<FirehoseConfigurationType>,
    #[serde(rename = "LogLevel")]
    #[serde(default)]
    pub log_level: String,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3ConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogsConfigurationType {
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirehoseConfigurationType {
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ConfigurationType {
    #[serde(rename = "BucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningCertificateRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTokensFromRefreshTokenRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_key: Option<String>,
    #[serde(rename = "RefreshToken")]
    #[serde(default)]
    pub refresh_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTokensFromRefreshTokenResponse {
    #[serde(rename = "AuthenticationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUICustomizationRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUICustomizationResponse {
    #[serde(rename = "UICustomization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_i_customization: Option<UICustomizationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UICustomizationType {
    #[serde(rename = "CSS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_s: Option<String>,
    #[serde(rename = "CSSVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_s_version: Option<String>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserAttributeVerificationCodeRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserAttributeVerificationCodeResponse {
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserAuthFactorsRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserAuthFactorsResponse {
    #[serde(rename = "ConfiguredUserAuthFactors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_user_auth_factors: Option<Vec<String>>,
    #[serde(rename = "PreferredMfaSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa_setting: Option<String>,
    #[serde(rename = "UserMFASettingList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_m_f_a_setting_list: Option<Vec<String>>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserPoolMfaConfigRequest {
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserPoolMfaConfigResponse {
    #[serde(rename = "EmailMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mfa_configuration: Option<EmailMfaConfigType>,
    #[serde(rename = "MfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[serde(rename = "SmsMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_configuration: Option<SmsMfaConfigType>,
    #[serde(rename = "SoftwareTokenMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfigType>,
    #[serde(rename = "WebAuthnConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_configuration: Option<WebAuthnConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailMfaConfigType {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmsMfaConfigType {
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    #[serde(rename = "SmsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SoftwareTokenMfaConfigType {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebAuthnConfigurationType {
    #[serde(rename = "FactorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor_configuration: Option<String>,
    #[serde(rename = "RelyingPartyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relying_party_id: Option<String>,
    #[serde(rename = "UserVerification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_verification: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserResponse {
    #[serde(rename = "MFAOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a_options: Option<Vec<MFAOptionType>>,
    #[serde(rename = "PreferredMfaSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_mfa_setting: Option<String>,
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    #[serde(rename = "UserMFASettingList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_m_f_a_setting_list: Option<Vec<String>>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSignOutRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalSignOutResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateAuthRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "AuthFlow")]
    #[serde(default)]
    pub auth_flow: String,
    #[serde(rename = "AuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "UserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateAuthResponse {
    #[serde(rename = "AuthenticationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[serde(rename = "AvailableChallenges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_challenges: Option<Vec<String>>,
    #[serde(rename = "ChallengeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    #[serde(rename = "ChallengeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesResponse {
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceType>>,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsResponse {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupType>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityProvidersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityProvidersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Providers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<ProviderDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProviderDescription {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceServersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceServersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_servers: Option<Vec<ResourceServerType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTermsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTermsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Terms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<TermsDescriptionType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TermsDescriptionType {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Enforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "TermsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_id: Option<String>,
    #[serde(rename = "TermsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserImportJobsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    pub max_results: i32,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserImportJobsResponse {
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "UserImportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_jobs: Option<Vec<UserImportJobType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserPoolClientSecretsRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserPoolClientSecretsResponse {
    #[serde(rename = "ClientSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secrets: Option<Vec<ClientSecretDescriptorType>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserPoolClientsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserPoolClientsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolClients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_clients: Option<Vec<UserPoolClientDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPoolClientDescription {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "ClientName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserPoolsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    pub max_results: i32,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserPoolsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pools: Option<Vec<UserPoolDescriptionType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPoolDescriptionType {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersInGroupRequest {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersInGroupResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersRequest {
    #[serde(rename = "AttributesToGet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersResponse {
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWebAuthnCredentialsRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
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
pub struct ListWebAuthnCredentialsResponse {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<WebAuthnCredentialDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebAuthnCredentialDescription {
    #[serde(rename = "AuthenticatorAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_attachment: Option<String>,
    #[serde(rename = "AuthenticatorTransports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_transports: Option<Vec<String>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "CredentialId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    #[serde(rename = "FriendlyCredentialName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_credential_name: Option<String>,
    #[serde(rename = "RelyingPartyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relying_party_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResendConfirmationCodeRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SecretHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    #[serde(rename = "UserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResendConfirmationCodeResponse {
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RespondToAuthChallengeRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "ChallengeName")]
    #[serde(default)]
    pub challenge_name: String,
    #[serde(rename = "ChallengeResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_responses: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "UserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RespondToAuthChallengeResponse {
    #[serde(rename = "AuthenticationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[serde(rename = "ChallengeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    #[serde(rename = "ChallengeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeTokenRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "Token")]
    #[serde(default)]
    pub token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeTokenResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetLogDeliveryConfigurationRequest {
    #[serde(rename = "LogConfigurations")]
    #[serde(default)]
    pub log_configurations: Vec<LogConfigurationType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetLogDeliveryConfigurationResponse {
    #[serde(rename = "LogDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configuration: Option<LogDeliveryConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetRiskConfigurationRequest {
    #[serde(rename = "AccountTakeoverRiskConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_takeover_risk_configuration: Option<AccountTakeoverRiskConfigurationType>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "CompromisedCredentialsRiskConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_credentials_risk_configuration:
        Option<CompromisedCredentialsRiskConfigurationType>,
    #[serde(rename = "RiskExceptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_exception_configuration: Option<RiskExceptionConfigurationType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetRiskConfigurationResponse {
    #[serde(rename = "RiskConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_configuration: Option<RiskConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUICustomizationRequest {
    #[serde(rename = "CSS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_s: Option<String>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "ImageFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUICustomizationResponse {
    #[serde(rename = "UICustomization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_i_customization: Option<UICustomizationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUserMFAPreferenceRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "EmailMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mfa_settings: Option<EmailMfaSettingsType>,
    #[serde(rename = "SMSMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_mfa_settings: Option<SMSMfaSettingsType>,
    #[serde(rename = "SoftwareTokenMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_settings: Option<SoftwareTokenMfaSettingsType>,
    #[serde(rename = "WebAuthnMfaSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_mfa_settings: Option<WebAuthnMfaSettingsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUserMFAPreferenceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUserPoolMfaConfigRequest {
    #[serde(rename = "EmailMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mfa_configuration: Option<EmailMfaConfigType>,
    #[serde(rename = "MfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[serde(rename = "SmsMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_configuration: Option<SmsMfaConfigType>,
    #[serde(rename = "SoftwareTokenMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfigType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "WebAuthnConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_configuration: Option<WebAuthnConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUserPoolMfaConfigResponse {
    #[serde(rename = "EmailMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_mfa_configuration: Option<EmailMfaConfigType>,
    #[serde(rename = "MfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[serde(rename = "SmsMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_mfa_configuration: Option<SmsMfaConfigType>,
    #[serde(rename = "SoftwareTokenMfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfigType>,
    #[serde(rename = "WebAuthnConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_configuration: Option<WebAuthnConfigurationType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUserSettingsRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "MFAOptions")]
    #[serde(default)]
    pub m_f_a_options: Vec<MFAOptionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetUserSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignUpRequest {
    #[serde(rename = "AnalyticsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_metadata: Option<AnalyticsMetadataType>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "SecretHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    #[serde(rename = "UserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context_data: Option<UserContextDataType>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
    #[serde(rename = "ValidationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<Vec<AttributeType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignUpResponse {
    #[serde(rename = "CodeDeliveryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details: Option<CodeDeliveryDetailsType>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "UserConfirmed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_confirmed: Option<bool>,
    #[serde(rename = "UserSub")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_sub: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartUserImportJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartUserImportJobResponse {
    #[serde(rename = "UserImportJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWebAuthnRegistrationRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWebAuthnRegistrationResponse {
    #[serde(rename = "CredentialCreationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_creation_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopUserImportJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopUserImportJobResponse {
    #[serde(rename = "UserImportJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_import_job: Option<UserImportJobType>,
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
pub struct TagResourceResponse {}

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
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthEventFeedbackRequest {
    #[serde(rename = "EventId")]
    #[serde(default)]
    pub event_id: String,
    #[serde(rename = "FeedbackToken")]
    #[serde(default)]
    pub feedback_token: String,
    #[serde(rename = "FeedbackValue")]
    #[serde(default)]
    pub feedback_value: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthEventFeedbackResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceStatusRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "DeviceKey")]
    #[serde(default)]
    pub device_key: String,
    #[serde(rename = "DeviceRememberedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_remembered_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceStatusResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Precedence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i32>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIdentityProviderRequest {
    #[serde(rename = "AttributeMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "IdpIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_identifiers: Option<Vec<String>>,
    #[serde(rename = "ProviderDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    pub provider_name: String,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIdentityProviderResponse {
    #[serde(rename = "IdentityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<IdentityProviderType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateManagedLoginBrandingRequest {
    #[serde(rename = "Assets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetType>>,
    #[serde(rename = "ManagedLoginBrandingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_branding_id: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "UseCognitoProvidedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cognito_provided_values: Option<bool>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateManagedLoginBrandingResponse {
    #[serde(rename = "ManagedLoginBranding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_branding: Option<ManagedLoginBrandingType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceServerRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ResourceServerScopeType>>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceServerResponse {
    #[serde(rename = "ResourceServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_server: Option<ResourceServerType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTermsRequest {
    #[serde(rename = "Enforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<String>,
    #[serde(rename = "Links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TermsId")]
    #[serde(default)]
    pub terms_id: String,
    #[serde(rename = "TermsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_name: Option<String>,
    #[serde(rename = "TermsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_source: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTermsResponse {
    #[serde(rename = "Terms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<TermsType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserAttributesRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "ClientMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    pub user_attributes: Vec<AttributeType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserAttributesResponse {
    #[serde(rename = "CodeDeliveryDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_delivery_details_list: Option<Vec<CodeDeliveryDetailsType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserPoolClientRequest {
    #[serde(rename = "AccessTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_validity: Option<i32>,
    #[serde(rename = "AllowedOAuthFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_flows_user_pool_client: Option<bool>,
    #[serde(rename = "AllowedOAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfigurationType>,
    #[serde(rename = "AuthSessionValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_session_validity: Option<i32>,
    #[serde(rename = "CallbackURLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_u_r_ls: Option<Vec<String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(rename = "DefaultRedirectURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_redirect_u_r_i: Option<String>,
    #[serde(rename = "EnablePropagateAdditionalUserContextData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_propagate_additional_user_context_data: Option<bool>,
    #[serde(rename = "EnableTokenRevocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_token_revocation: Option<bool>,
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    #[serde(rename = "IdTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_validity: Option<i32>,
    #[serde(rename = "LogoutURLs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_u_r_ls: Option<Vec<String>>,
    #[serde(rename = "PreventUserExistenceErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_user_existence_errors: Option<String>,
    #[serde(rename = "ReadAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    #[serde(rename = "RefreshTokenRotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_rotation: Option<RefreshTokenRotationType>,
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i32>,
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,
    #[serde(rename = "TokenValidityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_validity_units: Option<TokenValidityUnitsType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "WriteAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserPoolClientResponse {
    #[serde(rename = "UserPoolClient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_client: Option<UserPoolClientType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserPoolDomainRequest {
    #[serde(rename = "CustomDomainConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_config: Option<CustomDomainConfigType>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "ManagedLoginVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_version: Option<i32>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserPoolDomainResponse {
    #[serde(rename = "CloudFrontDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_domain: Option<String>,
    #[serde(rename = "ManagedLoginVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_login_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserPoolRequest {
    #[serde(rename = "AccountRecoverySetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recovery_setting: Option<AccountRecoverySettingType>,
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<AdminCreateUserConfigType>,
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<String>,
    #[serde(rename = "DeviceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<DeviceConfigurationType>,
    #[serde(rename = "EmailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<EmailConfigurationType>,
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<String>,
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<String>,
    #[serde(rename = "LambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfigType>,
    #[serde(rename = "MfaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<UserPoolPolicyType>,
    #[serde(rename = "PoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    #[serde(rename = "SmsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<SmsConfigurationType>,
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<String>,
    #[serde(rename = "UserAttributeUpdateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute_update_settings: Option<UserAttributeUpdateSettingsType>,
    #[serde(rename = "UserPoolAddOns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_add_ons: Option<UserPoolAddOnsType>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "UserPoolTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserPoolTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tier: Option<String>,
    #[serde(rename = "VerificationMessageTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message_template: Option<VerificationMessageTemplateType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserPoolResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifySoftwareTokenRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "FriendlyDeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_device_name: Option<String>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "UserCode")]
    #[serde(default)]
    pub user_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifySoftwareTokenResponse {
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyUserAttributeRequest {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "Code")]
    #[serde(default)]
    pub code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyUserAttributeResponse {}
