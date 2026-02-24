use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An email identity (domain, email address, etc.).
#[derive(Debug, Clone)]
pub struct EmailIdentity {
    pub name: String,
    pub identity_type: String,
    pub verified: bool,
    pub created_timestamp: DateTime<Utc>,
    pub policies: HashMap<String, String>,
    pub tags: HashMap<String, String>,
    /// Associated configuration set name.
    pub configuration_set_name: Option<String>,
    /// DKIM signing enabled.
    pub dkim_signing_enabled: bool,
    /// DKIM signing key type (e.g. "RSA_1024_BIT").
    pub dkim_signing_key_type: Option<String>,
    /// DKIM domain signing selector.
    pub dkim_domain: Option<String>,
    /// Feedback forwarding enabled.
    pub feedback_forwarding_enabled: bool,
    /// Mail-from domain.
    pub mail_from_domain: Option<String>,
    /// Mail-from MX failure action.
    pub behavior_on_mx_failure: Option<String>,
}

/// A sent email record.
#[derive(Debug, Clone)]
pub struct SentEmail {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
    pub message_id: String,
}

/// A configuration set.
#[derive(Debug, Clone)]
pub struct ConfigurationSet {
    pub name: String,
    pub tags: HashMap<String, String>,
    /// Event destinations keyed by destination name.
    pub event_destinations: HashMap<String, EventDestination>,
    /// Archiving options.
    pub archiving_options: Option<serde_json::Value>,
    /// Delivery options.
    pub delivery_options: Option<serde_json::Value>,
    /// Reputation options.
    pub reputation_options: Option<serde_json::Value>,
    /// Sending options.
    pub sending_options: Option<serde_json::Value>,
    /// Suppression options.
    pub suppression_options: Option<serde_json::Value>,
    /// Tracking options.
    pub tracking_options: Option<serde_json::Value>,
    /// VDM options.
    pub vdm_options: Option<serde_json::Value>,
}

impl ConfigurationSet {
    pub fn new(name: &str, tags: HashMap<String, String>) -> Self {
        Self {
            name: name.to_string(),
            tags,
            event_destinations: HashMap::new(),
            archiving_options: None,
            delivery_options: None,
            reputation_options: None,
            sending_options: None,
            suppression_options: None,
            tracking_options: None,
            vdm_options: None,
        }
    }
}

/// An event destination within a configuration set.
#[derive(Debug, Clone)]
pub struct EventDestination {
    pub name: String,
    pub enabled: bool,
    pub matching_event_types: Vec<String>,
    pub destination: serde_json::Value,
}

/// A contact list with embedded contacts.
#[derive(Debug, Clone)]
pub struct ContactList {
    pub name: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub created_timestamp: DateTime<Utc>,
    pub last_updated_timestamp: DateTime<Utc>,
    pub contacts: HashMap<String, Contact>,
}

/// A contact within a contact list.
#[derive(Debug, Clone)]
pub struct Contact {
    pub email_address: String,
    pub topic_preferences: Vec<TopicPreference>,
    pub unsubscribe_all: bool,
    pub created_timestamp: DateTime<Utc>,
    pub last_updated_timestamp: DateTime<Utc>,
}

/// A topic preference for a contact.
#[derive(Debug, Clone)]
pub struct TopicPreference {
    pub topic_name: String,
    pub subscription_status: String,
}

/// A dedicated IP pool.
#[derive(Debug, Clone)]
pub struct DedicatedIpPool {
    pub pool_name: String,
    pub scaling_mode: String,
    pub tags: HashMap<String, String>,
}

/// A dedicated IP address.
#[derive(Debug, Clone)]
pub struct DedicatedIp {
    pub ip: String,
    pub warmup_status: String,
    pub warmup_percentage: i32,
    pub pool_name: Option<String>,
}

/// An email template.
#[derive(Debug, Clone)]
pub struct EmailTemplate {
    pub template_name: String,
    pub subject_part: Option<String>,
    pub text_part: Option<String>,
    pub html_part: Option<String>,
    pub created_timestamp: DateTime<Utc>,
}

/// A custom verification email template.
#[derive(Debug, Clone)]
pub struct CustomVerificationEmailTemplate {
    pub template_name: String,
    pub from_email_address: String,
    pub template_subject: String,
    pub template_content: String,
    pub success_redirection_url: String,
    pub failure_redirection_url: String,
    pub created_timestamp: DateTime<Utc>,
}

/// An import job.
#[derive(Debug, Clone)]
pub struct ImportJob {
    pub job_id: String,
    pub import_destination: serde_json::Value,
    pub import_data_source: serde_json::Value,
    pub job_status: String,
    pub created_timestamp: DateTime<Utc>,
}

/// An export job.
#[derive(Debug, Clone)]
pub struct ExportJob {
    pub job_id: String,
    pub export_source_type: String,
    pub job_status: String,
    pub created_timestamp: DateTime<Utc>,
    pub export_destination: serde_json::Value,
}

/// A multi-region endpoint.
#[derive(Debug, Clone)]
pub struct MultiRegionEndpoint {
    pub endpoint_name: String,
    pub endpoint_id: String,
    pub status: String,
    pub regions: Vec<String>,
    pub created_timestamp: DateTime<Utc>,
    pub last_updated_timestamp: DateTime<Utc>,
}

/// Account-level settings.
#[derive(Debug, Clone)]
pub struct AccountSettings {
    pub sending_enabled: bool,
    pub production_access_enabled: bool,
    pub send_quota: SendQuota,
    pub dedicated_ip_auto_warmup_enabled: bool,
    pub enforcement_status: String,
    pub details: Option<serde_json::Value>,
    /// Account-level suppression list reasons.
    pub suppressed_reasons: Vec<String>,
    /// VDM attributes.
    pub vdm_attributes: Option<serde_json::Value>,
}

impl Default for AccountSettings {
    fn default() -> Self {
        Self {
            sending_enabled: true,
            production_access_enabled: true,
            send_quota: SendQuota::default(),
            dedicated_ip_auto_warmup_enabled: false,
            enforcement_status: "HEALTHY".to_string(),
            details: None,
            suppressed_reasons: Vec::new(),
            vdm_attributes: None,
        }
    }
}

/// Send quota details.
#[derive(Debug, Clone)]
pub struct SendQuota {
    pub max_24_hour_send: f64,
    pub max_send_rate: f64,
    pub sent_last_24_hours: f64,
}

impl Default for SendQuota {
    fn default() -> Self {
        Self {
            max_24_hour_send: 200.0,
            max_send_rate: 1.0,
            sent_last_24_hours: 0.0,
        }
    }
}

/// Deliverability dashboard subscription.
#[derive(Debug, Clone)]
pub struct DeliverabilityDashboardOptions {
    pub dashboard_enabled: bool,
    pub subscription_expiry_date: Option<f64>,
    pub account_status: String,
}

impl Default for DeliverabilityDashboardOptions {
    fn default() -> Self {
        Self {
            dashboard_enabled: false,
            subscription_expiry_date: None,
            account_status: "DISABLED".to_string(),
        }
    }
}

/// A suppressed destination entry (account-level suppression list).
#[derive(Debug, Clone)]
pub struct SuppressedDestinationEntry {
    pub email_address: String,
    pub reason: String,
    pub last_update_time: f64,
}

/// A tenant.
#[derive(Debug, Clone)]
pub struct Tenant {
    pub tenant_name: String,
    pub tenant_id: String,
    pub tags: HashMap<String, String>,
    pub created_timestamp: DateTime<Utc>,
}

/// A tenant resource association.
#[derive(Debug, Clone)]
pub struct TenantResourceAssociation {
    pub tenant_name: String,
    pub resource_arn: String,
}

/// A deliverability test report (predictive inbox placement test).
#[derive(Debug, Clone)]
pub struct DeliverabilityTestReportRecord {
    pub report_id: String,
    pub report_name: Option<String>,
    pub from_email_address: String,
    pub create_date: f64,
    pub deliverability_test_status: String,
}

/// A reputation entity (e.g., email identity, configuration set).
#[derive(Debug, Clone)]
pub struct ReputationEntityRecord {
    /// Unique key: "{entity_type}:{entity_reference}"
    pub entity_type: String,
    pub entity_reference: String,
    /// Customer-managed sending status: "ENABLED" | "DISABLED".
    pub customer_managed_sending_status: String,
    /// Customer-managed policy (JSON string, opaque to mock).
    pub policy: Option<String>,
}
