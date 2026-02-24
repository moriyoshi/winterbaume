use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A Cognito user pool.
#[derive(Debug, Clone)]
pub struct UserPool {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub created_date: DateTime<Utc>,
    pub clients: HashMap<String, UserPoolClient>,
    pub users: HashMap<String, CognitoUser>,
    pub groups: HashMap<String, Group>,
    pub identity_providers: HashMap<String, IdentityProvider>,
    pub resource_servers: HashMap<String, ResourceServer>,
    pub domain: Option<UserPoolDomain>,
    pub mfa_config: Option<MfaConfig>,
    pub custom_attributes: Vec<String>,
    pub tags: HashMap<String, String>,
    /// Devices keyed by "username::deviceKey"
    pub devices: HashMap<String, Device>,
    pub ui_customization: Option<UICustomization>,
    pub managed_login_brandings: HashMap<String, ManagedLoginBranding>,
    pub risk_configuration: Option<serde_json::Value>,
    pub log_delivery_config: Option<Vec<serde_json::Value>>,
    pub user_import_jobs: HashMap<String, UserImportJob>,
    pub terms: HashMap<String, serde_json::Value>,
}

/// A user pool client (app client).
#[derive(Debug, Clone)]
pub struct UserPoolClient {
    pub id: String,
    pub name: String,
    pub user_pool_id: String,
    pub client_secret: Option<String>,
    pub explicit_auth_flows: Vec<String>,
    pub allowed_oauth_flows: Vec<String>,
    pub allowed_oauth_scopes: Vec<String>,
    pub callback_urls: Vec<String>,
    pub logout_urls: Vec<String>,
    pub allowed_oauth_flows_user_pool_client: bool,
    pub refresh_token_validity: Option<i32>,
    pub supported_identity_providers: Vec<String>,
}

/// A user in a user pool.
#[derive(Debug, Clone)]
pub struct CognitoUser {
    pub username: String,
    pub status: String,
    pub created_date: DateTime<Utc>,
    pub attributes: HashMap<String, String>,
    pub enabled: bool,
    pub password: Option<String>,
    pub confirmed: bool,
    pub groups: Vec<String>,
    pub linked_providers: Vec<LinkedProvider>,
}

#[derive(Debug, Clone)]
pub struct LinkedProvider {
    pub provider_name: Option<String>,
    pub provider_attribute_name: Option<String>,
    pub provider_attribute_value: Option<String>,
}

/// A user pool group.
#[derive(Debug, Clone)]
pub struct Group {
    pub group_name: String,
    pub user_pool_id: String,
    pub description: Option<String>,
    pub role_arn: Option<String>,
    pub precedence: Option<i32>,
    pub created_date: DateTime<Utc>,
    pub last_modified_date: DateTime<Utc>,
}

/// An identity provider in a user pool.
#[derive(Debug, Clone)]
pub struct IdentityProvider {
    pub provider_name: String,
    pub provider_type: String,
    pub user_pool_id: String,
    pub provider_details: HashMap<String, String>,
    pub attribute_mapping: HashMap<String, String>,
    pub idp_identifiers: Vec<String>,
    pub created_date: DateTime<Utc>,
    pub last_modified_date: DateTime<Utc>,
}

/// A resource server.
#[derive(Debug, Clone)]
pub struct ResourceServer {
    pub identifier: String,
    pub name: String,
    pub user_pool_id: String,
    pub scopes: Vec<ResourceServerScope>,
}

#[derive(Debug, Clone)]
pub struct ResourceServerScope {
    pub scope_name: String,
    pub scope_description: String,
}

/// A user pool domain.
#[derive(Debug, Clone)]
pub struct UserPoolDomain {
    pub domain: String,
    pub user_pool_id: String,
    pub status: String,
    pub cloud_front_distribution: Option<String>,
    pub custom_domain_config: Option<CustomDomainConfig>,
}

#[derive(Debug, Clone)]
pub struct CustomDomainConfig {
    pub certificate_arn: String,
}

/// MFA configuration for a user pool.
#[derive(Debug, Clone)]
pub struct MfaConfig {
    pub mfa_configuration: String,
    pub sms_mfa_configuration: Option<SmsMfaConfig>,
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfig>,
}

#[derive(Debug, Clone)]
pub struct SmsMfaConfig {
    pub sms_authentication_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SoftwareTokenMfaConfig {
    pub enabled: bool,
}

/// A device registered to a user.
#[derive(Debug, Clone)]
pub struct Device {
    pub device_key: String,
    pub device_attributes: HashMap<String, String>,
    pub device_created_date: chrono::DateTime<chrono::Utc>,
    pub device_last_modified_date: chrono::DateTime<chrono::Utc>,
}

/// A user import job.
#[derive(Debug, Clone)]
pub struct UserImportJob {
    pub job_id: String,
    pub job_name: String,
    pub job_arn: String,
    pub user_pool_id: String,
    pub pre_signed_url: Option<String>,
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

/// UI customization for a user pool.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct UICustomization {
    pub user_pool_id: String,
    pub client_id: Option<String>,
    pub css: Option<String>,
    pub css_version: Option<String>,
    pub image_url: Option<String>,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub last_modified_date: chrono::DateTime<chrono::Utc>,
}

/// Managed login branding for a user pool.
#[derive(Debug, Clone)]
pub struct ManagedLoginBranding {
    pub branding_id: String,
    pub user_pool_id: String,
    pub settings: Option<serde_json::Value>,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub last_modified_date: chrono::DateTime<chrono::Utc>,
}
