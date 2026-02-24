use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Server {
    pub arn: String,
    pub server_id: String,
    pub state: String,
    pub endpoint_type: String,
    pub identity_provider_type: String,
    pub protocols: Vec<String>,
    pub domain: String,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub users: HashMap<String, User>,
    // Optional mutable fields updated via UpdateServer
    pub logging_role: Option<String>,
    pub certificate: Option<String>,
    pub host_certificate_chain: Option<String>,
    pub security_policy_name: Option<String>,
    /// Opaque storage for nested Terraform blocks (endpoint_details, protocol_details, etc.)
    pub extra_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct Agreement {
    pub agreement_id: String,
    pub arn: String,
    pub server_id: String,
    pub local_profile_id: String,
    pub partner_profile_id: String,
    pub base_directory: String,
    pub access_role: String,
    pub status: String,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub certificate_id: String,
    pub arn: String,
    pub usage: String,
    pub status: String,
    pub certificate: String,
    pub certificate_chain: Option<String>,
    pub private_key: Option<String>,
    pub active_date: Option<f64>,
    pub inactive_date: Option<f64>,
    pub description: Option<String>,
    pub certificate_type: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct Connector {
    pub connector_id: String,
    pub arn: String,
    pub url: Option<String>,
    pub as2_config: Option<serde_json::Value>,
    pub sftp_config: Option<serde_json::Value>,
    pub access_role: String,
    pub logging_role: Option<String>,
    pub status: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub profile_id: String,
    pub arn: String,
    pub profile_type: String,
    pub as2_id: String,
    pub certificate_ids: Vec<String>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct WebApp {
    pub web_app_id: String,
    pub arn: String,
    pub web_app_endpoint: Option<String>,
    pub tags: Vec<Tag>,
    pub customization: Option<WebAppCustomization>,
}

#[derive(Debug, Clone)]
pub struct WebAppCustomization {
    pub title: Option<String>,
    pub logo_file: Option<String>,
    pub favicon_file: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Workflow {
    pub workflow_id: String,
    pub arn: String,
    pub steps: Vec<serde_json::Value>,
    pub on_exception_steps: Vec<serde_json::Value>,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    pub fn to_map(tags: &[Tag]) -> HashMap<String, String> {
        tags.iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub server_id: String,
    pub user_name: String,
    pub arn: String,
    pub role: String,
    pub home_directory: Option<String>,
    pub home_directory_type: String,
    pub tags: Vec<Tag>,
    pub ssh_public_keys: Vec<SshPublicKey>,
    /// Opaque storage for nested Terraform blocks (home_directory_mappings, posix_profile)
    pub extra_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct SshPublicKey {
    pub ssh_public_key_id: String,
    pub ssh_public_key_body: String,
    pub date_imported: DateTime<Utc>,
}
