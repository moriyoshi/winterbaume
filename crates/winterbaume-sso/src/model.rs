//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sso

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogoutRequest {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoleCredentialsRequest {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountRolesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "roleList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_list: Option<Vec<RoleInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoleInfo {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "roleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsRequest {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountsResponse {
    #[serde(rename = "accountList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_list: Option<Vec<AccountInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountInfo {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "accountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "emailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountRolesRequest {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    pub access_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoleCredentialsResponse {
    #[serde(rename = "roleCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_credentials: Option<RoleCredentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoleCredentials {
    #[serde(rename = "accessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
    #[serde(rename = "secretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "sessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}
