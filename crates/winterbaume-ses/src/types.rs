//! Domain types for the SES v1 service.

use chrono::{DateTime, Utc};

/// An SES identity (email address or domain).
#[derive(Debug, Clone)]
pub struct Identity {
    pub name: String,
    pub identity_type: IdentityType,
    pub verification_status: VerificationStatus,
    pub verification_token: Option<String>,
    /// For domain identities: DKIM tokens.
    pub dkim_tokens: Vec<String>,
    pub dkim_enabled: bool,
    pub mail_from_domain: Option<String>,
    pub bounce_topic: Option<String>,
    pub complaint_topic: Option<String>,
    pub delivery_topic: Option<String>,
    pub forwarding_enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentityType {
    EmailAddress,
    Domain,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    Pending,
    Success,
    Failed,
    TemporaryFailure,
    NotStarted,
}

impl VerificationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            VerificationStatus::Pending => "Pending",
            VerificationStatus::Success => "Success",
            VerificationStatus::Failed => "Failed",
            VerificationStatus::TemporaryFailure => "TemporaryFailure",
            VerificationStatus::NotStarted => "NotStarted",
        }
    }
}

/// A configuration set.
#[derive(Debug, Clone)]
pub struct ConfigurationSet {
    pub name: String,
    pub event_destinations: Vec<EventDestination>,
    pub created_at: DateTime<Utc>,
}

/// An event destination within a configuration set.
#[derive(Debug, Clone)]
pub struct EventDestination {
    pub name: String,
    pub enabled: bool,
    pub matching_event_types: Vec<String>,
}

/// A receipt rule set.
#[derive(Debug, Clone)]
pub struct ReceiptRuleSet {
    pub name: String,
    pub rules: Vec<ReceiptRule>,
    pub created_at: DateTime<Utc>,
}

/// A receipt rule.
#[derive(Debug, Clone)]
pub struct ReceiptRule {
    pub name: String,
    pub enabled: bool,
    pub scan_enabled: bool,
    pub tls_policy: Option<String>,
}

/// An email template.
#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub subject_part: Option<String>,
    pub html_part: Option<String>,
    pub text_part: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// A record of an email previously sent through `SendEmail`.
///
/// SES v1 has no public API for inspecting individual sent messages;
/// this record is part of the mock state so test code can verify what
/// was sent.
#[derive(Debug, Clone)]
pub struct SentEmail {
    pub message_id: String,
    pub source: String,
    pub to_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub bcc_addresses: Vec<String>,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
    pub timestamp: DateTime<Utc>,
}
