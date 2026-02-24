use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A WorkSpaces Web portal.
#[derive(Debug, Clone)]
pub struct Portal {
    pub portal_arn: String,
    pub portal_endpoint: String,
    pub display_name: String,
    pub portal_status: String,
    pub browser_type: String,
    pub renderer_type: String,
    pub creation_date: DateTime<Utc>,
    pub browser_settings_arn: Option<String>,
    pub network_settings_arn: Option<String>,
    pub user_access_logging_settings_arn: Option<String>,
    pub user_settings_arn: Option<String>,
    pub trust_store_arn: Option<String>,
    pub ip_access_settings_arn: Option<String>,
    pub data_protection_settings_arn: Option<String>,
    pub session_logger_arn: Option<String>,
    pub tags: HashMap<String, String>,
}

/// Browser settings resource.
#[derive(Debug, Clone)]
pub struct BrowserSettings {
    pub browser_settings_arn: String,
    pub browser_policy: Option<String>,
    pub associated_portal_arns: Vec<String>,
    pub tags: HashMap<String, String>,
}

/// Network settings resource.
#[derive(Debug, Clone)]
pub struct NetworkSettings {
    pub network_settings_arn: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub associated_portal_arns: Vec<String>,
    pub tags: HashMap<String, String>,
}

/// User access logging settings resource.
#[derive(Debug, Clone)]
pub struct UserAccessLoggingSettings {
    pub user_access_logging_settings_arn: String,
    pub kinesis_stream_arn: String,
    pub associated_portal_arns: Vec<String>,
    pub tags: HashMap<String, String>,
}

/// User settings resource.
#[derive(Debug, Clone)]
pub struct UserSettings {
    pub user_settings_arn: String,
    pub copy_allowed: String,
    pub paste_allowed: String,
    pub download_allowed: String,
    pub upload_allowed: String,
    pub print_allowed: String,
    pub disconnect_timeout_in_minutes: Option<i32>,
    pub idle_disconnect_timeout_in_minutes: Option<i32>,
    pub associated_portal_arns: Vec<String>,
    pub tags: HashMap<String, String>,
}

/// Identity provider resource.
#[derive(Debug, Clone)]
pub struct IdentityProvider {
    pub identity_provider_arn: String,
    pub portal_arn: String,
    pub identity_provider_name: String,
    pub identity_provider_type: String,
    pub identity_provider_details: HashMap<String, String>,
}

/// IP access settings resource.
#[derive(Debug, Clone)]
pub struct IpAccessSettings {
    pub ip_access_settings_arn: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub ip_rules: Vec<IpRule>,
    pub associated_portal_arns: Vec<String>,
    pub creation_date: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct IpRule {
    pub ip_range: String,
    pub description: Option<String>,
}

/// Trust store resource.
#[derive(Debug, Clone)]
pub struct TrustStore {
    pub trust_store_arn: String,
    pub certificate_list: Vec<TrustStoreCertificate>,
    pub associated_portal_arns: Vec<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TrustStoreCertificate {
    pub thumbprint: String,
    pub body: String,
    pub issuer: Option<String>,
    pub subject: Option<String>,
}

/// Data protection settings resource.
#[derive(Debug, Clone)]
pub struct DataProtectionSettings {
    pub data_protection_settings_arn: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub associated_portal_arns: Vec<String>,
    pub creation_date: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// Session logger resource.
#[derive(Debug, Clone)]
pub struct SessionLogger {
    pub session_logger_arn: String,
    pub display_name: Option<String>,
    pub associated_portal_arns: Vec<String>,
    pub creation_date: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// Session resource.
#[derive(Debug, Clone)]
pub struct Session {
    pub session_id: String,
    pub portal_id: String,
    pub username: Option<String>,
    pub status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub client_ip_addresses: Vec<String>,
}
