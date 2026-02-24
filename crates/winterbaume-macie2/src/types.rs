use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// Represents the Macie session for an account+region.
#[derive(Debug, Clone)]
pub struct MacieSession {
    pub status: String,
    pub finding_publishing_frequency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub service_role: String,
}

/// Represents a Macie member account.
#[derive(Debug, Clone)]
pub struct MacieMember {
    pub account_id: String,
    pub email: String,
    pub relationship_status: String,
    pub invited_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a pending invitation.
#[derive(Debug, Clone)]
pub struct MacieInvitation {
    pub account_id: String,
    pub invitation_id: String,
    pub invited_at: DateTime<Utc>,
    pub relationship_status: String,
}

/// Represents an organization admin account.
#[derive(Debug, Clone)]
pub struct MacieAdminAccount {
    pub account_id: String,
    pub status: String,
}

/// Represents the administrator account relationship.
#[derive(Debug, Clone)]
pub struct MacieAdministrator {
    pub account_id: String,
    pub invitation_id: String,
    pub invited_at: DateTime<Utc>,
    pub relationship_status: String,
}

// ── Custom data identifiers ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieCustomDataIdentifier {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub regex: String,
    pub keywords: Vec<String>,
    pub ignore_words: Vec<String>,
    pub maximum_match_distance: i32,
    pub severity_levels: Vec<MacieSeverityLevel>,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct MacieSeverityLevel {
    pub occurrences_threshold: i64,
    pub severity: String,
}

// ── Allow lists ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieAllowList {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub criteria: MacieAllowListCriteria,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status_code: String,
}

#[derive(Debug, Clone)]
pub struct MacieAllowListCriteria {
    pub regex: Option<String>,
    pub s3_words_list: Option<MacieS3WordsList>,
}

#[derive(Debug, Clone)]
pub struct MacieS3WordsList {
    pub bucket_name: String,
    pub object_key: String,
}

// ── Findings filters ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieFindingsFilter {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub action: String,
    pub position: i32,
    pub finding_criteria: serde_json::Value,
    pub tags: HashMap<String, String>,
}

// ── Classification jobs ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieClassificationJob {
    pub job_id: String,
    pub job_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub job_type: String,
    pub job_status: String,
    pub client_token: String,
    pub s3_job_definition: serde_json::Value,
    pub allow_list_ids: Vec<String>,
    pub custom_data_identifier_ids: Vec<String>,
    pub managed_data_identifier_ids: Vec<String>,
    pub managed_data_identifier_selector: Option<String>,
    pub sampling_percentage: Option<i32>,
    pub schedule_frequency: Option<serde_json::Value>,
    pub initial_run: bool,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub last_run_time: Option<DateTime<Utc>>,
}

// ── Sensitivity inspection templates ─────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieSensitivityInspectionTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub excludes_managed_data_identifier_ids: Vec<String>,
    pub includes_allow_list_ids: Vec<String>,
    pub includes_custom_data_identifier_ids: Vec<String>,
    pub includes_managed_data_identifier_ids: Vec<String>,
}

// ── Automated discovery configuration ────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieAutomatedDiscoveryConfig {
    pub status: String,
    pub auto_enable_organization_members: Option<String>,
    pub classification_scope_id: String,
    pub sensitivity_inspection_template_id: String,
    pub first_enabled_at: Option<DateTime<Utc>>,
    pub disabled_at: Option<DateTime<Utc>>,
    pub last_updated_at: DateTime<Utc>,
}

/// Per-account automated discovery status override.
#[derive(Debug, Clone)]
pub struct MacieAutomatedDiscoveryAccount {
    pub account_id: String,
    pub status: String,
}

// ── Reveal configuration ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieRevealConfig {
    pub status: String,
    pub kms_key_id: Option<String>,
    pub retrieval_mode: String,
    pub role_name: Option<String>,
}

// ── Organisation configuration ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieOrgConfig {
    pub auto_enable: bool,
    pub max_account_limit_reached: bool,
}

// ── Classification export configuration ──────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieClassificationExportConfig {
    pub raw: serde_json::Value,
}

// ── Findings publication configuration ───────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieFindingsPublicationConfig {
    pub publish_classification_findings: bool,
    pub publish_policy_findings: bool,
}

// ── Findings ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MacieFinding {
    pub id: String,
    pub finding_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub category: String,
    pub sample: bool,
}
