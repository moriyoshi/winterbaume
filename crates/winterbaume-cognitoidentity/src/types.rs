use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct IdentityPool {
    pub identity_pool_id: String,
    pub identity_pool_name: String,
    pub allow_unauthenticated_identities: bool,
    pub supported_login_providers: HashMap<String, String>,
    pub developer_provider_name: Option<String>,
    pub open_id_connect_provider_arns: Vec<String>,
    pub cognito_identity_providers: Vec<CognitoIdentityProvider>,
    pub saml_provider_arns: Vec<String>,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CognitoIdentityProvider {
    pub provider_name: String,
    pub client_id: String,
    pub server_side_token_check: bool,
}

#[derive(Debug, Clone)]
pub struct Identity {
    pub identity_id: String,
    pub identity_pool_id: String,
    pub logins: Vec<String>,
    pub creation_date: DateTime<Utc>,
    pub last_modified_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct IdentityPoolRoles {
    pub roles: HashMap<String, String>,
    pub role_mappings: HashMap<String, serde_json::Value>,
}

/// Developer identity link: maps a developer user identifier to an identity ID.
#[derive(Debug, Clone)]
pub struct DeveloperIdentityLink {
    pub identity_id: String,
    pub identity_pool_id: String,
    pub developer_provider_name: String,
    pub developer_user_identifier: String,
}

#[derive(Debug, Clone)]
pub struct PrincipalTagEntry {
    pub identity_pool_id: String,
    pub identity_provider_name: String,
    pub use_defaults: bool,
    pub principal_tags: HashMap<String, String>,
}
