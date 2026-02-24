use chrono::{DateTime, Utc};

/// An AWS Organization.
#[derive(Debug, Clone)]
pub struct Organization {
    pub id: String,
    pub arn: String,
    pub master_account_id: String,
    pub master_account_email: String,
}

/// An AWS account in an organization.
#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub email: String,
    pub status: String,
    pub joined_method: String,
    pub joined_timestamp: DateTime<Utc>,
    pub create_account_status_id: String,
    pub parent_id: String,
}

/// An organizational unit.
#[derive(Debug, Clone)]
pub struct OrganizationalUnit {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub parent_id: String,
}

/// A delegated administrator entry.
#[derive(Debug, Clone)]
pub struct DelegatedAdministrator {
    pub account: Account,
    pub delegation_enabled_date: DateTime<Utc>,
    pub services: Vec<DelegatedService>,
}

/// A delegated service.
#[derive(Debug, Clone)]
pub struct DelegatedService {
    pub service_principal: String,
    pub delegation_enabled_date: DateTime<Utc>,
}

/// An organization root.
#[derive(Debug, Clone)]
pub struct OrgRoot {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub policy_types: Vec<PolicyTypeStatus>,
}

/// Policy type status on a root.
#[derive(Debug, Clone)]
pub struct PolicyTypeStatus {
    pub policy_type: String,
    pub status: String,
}

/// A service control or other organization policy.
#[derive(Debug, Clone)]
pub struct OrgPolicy {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: String,
    pub policy_type: String,
    pub content: String,
    pub aws_managed: bool,
}

/// A policy attachment (policy -> target).
#[derive(Debug, Clone)]
pub struct PolicyAttachment {
    pub policy_id: String,
    pub target_id: String,
}

/// Enabled AWS service access entry.
#[derive(Debug, Clone)]
pub struct EnabledService {
    pub service_principal: String,
    pub date_enabled: DateTime<Utc>,
}

/// Tag on an organization resource.
#[derive(Debug, Clone)]
pub struct OrgTag {
    pub key: String,
    pub value: String,
}

/// Supported service principals for delegated administrators.
pub const SUPPORTED_DELEGATED_ADMIN_SERVICES: &[&str] = &[
    "config-multiaccountsetup.amazonaws.com",
    "guardduty.amazonaws.com",
    "access-analyzer.amazonaws.com",
    "macie.amazonaws.com",
    "servicecatalog.amazonaws.com",
    "ssm.amazonaws.com",
];

/// Valid policy types.
pub const VALID_POLICY_TYPES: &[&str] = &[
    "SERVICE_CONTROL_POLICY",
    "TAG_POLICY",
    "BACKUP_POLICY",
    "AISERVICES_OPT_OUT_POLICY",
];

/// A handshake between organizations/accounts.
#[derive(Debug, Clone)]
pub struct Handshake {
    pub id: String,
    pub arn: String,
    pub state: String,
    pub action: String,
    pub parties: Vec<HandshakeParty>,
    pub expiration_timestamp: DateTime<Utc>,
    pub requested_timestamp: DateTime<Utc>,
    pub resources: Vec<HandshakeResource>,
}

/// A party to a handshake.
#[derive(Debug, Clone)]
pub struct HandshakeParty {
    pub id: String,
    pub party_type: String,
}

/// A resource attached to a handshake.
#[derive(Debug, Clone)]
pub struct HandshakeResource {
    pub resource_type: String,
    pub value: String,
}

/// A responsibility transfer between organizations.
#[derive(Debug, Clone)]
pub struct ResponsibilityTransfer {
    pub id: String,
    pub arn: String,
    pub status: String,
    pub name: String,
    pub transfer_type: String,
    pub source_account_id: String,
    pub target_account_id: String,
    pub start_timestamp: DateTime<Utc>,
    pub active_handshake_id: Option<String>,
}
