use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A profile represents a list of IAM roles that Roles Anywhere is trusted to assume.
#[derive(Debug, Clone)]
pub struct Profile {
    pub profile_id: String,
    pub profile_arn: String,
    pub name: String,
    pub enabled: bool,
    pub role_arns: Vec<String>,
    pub managed_policy_arns: Vec<String>,
    pub session_policy: Option<String>,
    pub duration_seconds: Option<i32>,
    pub require_instance_properties: Option<bool>,
    pub accept_role_session_name: Option<bool>,
    pub attribute_mappings: Vec<AttributeMapping>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// A trust anchor establishes trust between IAM Roles Anywhere and a CA.
#[derive(Debug, Clone)]
pub struct TrustAnchor {
    pub trust_anchor_id: String,
    pub trust_anchor_arn: String,
    pub name: String,
    pub source: Source,
    pub enabled: bool,
    pub notification_settings: Vec<NotificationSettingDetail>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// A certificate revocation list (CRL).
#[derive(Debug, Clone)]
pub struct Crl {
    pub crl_id: String,
    pub crl_arn: String,
    pub name: String,
    pub enabled: bool,
    pub crl_data: Vec<u8>,
    pub trust_anchor_arn: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// Trust anchor source configuration.
#[derive(Debug, Clone)]
pub struct Source {
    pub source_type: Option<String>,
    pub source_data: Option<SourceData>,
}

/// Source data for trust anchors (union type).
#[derive(Debug, Clone)]
pub enum SourceData {
    X509CertificateData(String),
    AcmPcaArn(String),
}

/// Notification setting detail attached to a trust anchor.
#[derive(Debug, Clone)]
pub struct NotificationSettingDetail {
    pub enabled: bool,
    pub event: String,
    pub threshold: Option<i32>,
    pub channel: Option<String>,
    pub configured_by: Option<String>,
}

/// Attribute mapping rule for profiles.
#[derive(Debug, Clone)]
pub struct AttributeMapping {
    pub certificate_field: String,
    pub mapping_rules: Vec<MappingRule>,
}

/// A single mapping entry for a specifier.
#[derive(Debug, Clone)]
pub struct MappingRule {
    pub specifier: String,
}

/// Tag key-value pair.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}
