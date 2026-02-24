use chrono::{DateTime, Utc};

/// An IAM user.
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub user_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub create_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub attached_policies: Vec<AttachedPolicy>,
    pub inline_policies: Vec<InlinePolicy>,
    pub permissions_boundary: Option<String>,
}

/// An IAM role.
#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub role_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub assume_role_policy_document: String,
    pub description: String,
    pub create_date: DateTime<Utc>,
    pub max_session_duration: i32,
    pub tags: Vec<Tag>,
    pub attached_policies: Vec<AttachedPolicy>,
    pub inline_policies: Vec<InlinePolicy>,
    pub permissions_boundary: Option<String>,
}

/// An IAM group.
#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub group_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub create_date: DateTime<Utc>,
    pub members: Vec<String>,
    pub attached_policies: Vec<AttachedPolicy>,
    pub inline_policies: Vec<InlinePolicy>,
}

/// An IAM access key.
#[derive(Debug, Clone)]
pub struct AccessKey {
    pub user_name: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub status: String,
    pub create_date: DateTime<Utc>,
}

/// An IAM managed policy.
#[derive(Debug, Clone)]
pub struct ManagedPolicy {
    pub policy_name: String,
    pub policy_id: String,
    pub arn: String,
    pub path: String,
    pub default_version_id: String,
    pub attachment_count: u32,
    pub description: String,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub is_attachable: bool,
    pub document: String,
    pub versions: Vec<PolicyVersionEntry>,
    pub tags: Vec<Tag>,
    /// Monotonically increasing counter for the next version number.
    pub next_version_number: u32,
}

/// A version of a managed policy.
#[derive(Debug, Clone)]
pub struct PolicyVersionEntry {
    pub version_id: String,
    pub document: String,
    pub is_default_version: bool,
    pub create_date: DateTime<Utc>,
}

/// A policy attached to a role/user/group (just name + ARN reference).
#[derive(Debug, Clone)]
pub struct AttachedPolicy {
    pub policy_name: String,
    pub policy_arn: String,
}

/// An inline policy (embedded in a user/role/group).
#[derive(Debug, Clone)]
pub struct InlinePolicy {
    pub policy_name: String,
    pub policy_document: String,
}

/// A resource tag.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// An IAM instance profile.
#[derive(Debug, Clone)]
pub struct InstanceProfile {
    pub name: String,
    pub instance_profile_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub create_date: DateTime<Utc>,
    pub roles: Vec<String>,
    pub tags: Vec<Tag>,
}

/// An IAM login profile (console password).
#[derive(Debug, Clone)]
pub struct LoginProfile {
    pub user_name: String,
    pub create_date: DateTime<Utc>,
    pub password_reset_required: bool,
}

/// An IAM OpenID Connect provider.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct OpenIDConnectProvider {
    pub arn: String,
    pub url: String,
    pub client_id_list: Vec<String>,
    pub thumbprint_list: Vec<String>,
    pub create_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
}

/// An IAM SAML provider.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct SAMLProvider {
    pub arn: String,
    pub name: String,
    pub saml_metadata_document: String,
    pub create_date: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub tags: Vec<Tag>,
}

/// An IAM virtual MFA device.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct VirtualMFADevice {
    pub serial_number: String,
    pub base32_string_seed: String,
    pub qr_code_png: String,
    pub user_name: Option<String>,
    pub enable_date: Option<DateTime<Utc>>,
    pub tags: Vec<Tag>,
}

/// An MFA device association.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct MFADeviceAssociation {
    pub user_name: String,
    pub serial_number: String,
    pub enable_date: DateTime<Utc>,
}

/// Account password policy.
#[derive(Debug, Clone)]
pub struct AccountPasswordPolicy {
    pub minimum_password_length: i32,
    pub require_symbols: bool,
    pub require_numbers: bool,
    pub require_uppercase_characters: bool,
    pub require_lowercase_characters: bool,
    pub allow_users_to_change_password: bool,
    pub max_password_age: Option<i32>,
    pub password_reuse_prevention: Option<i32>,
    pub hard_expiry: Option<bool>,
    pub expire_passwords: bool,
}

impl Default for AccountPasswordPolicy {
    fn default() -> Self {
        Self {
            minimum_password_length: 8,
            require_symbols: false,
            require_numbers: false,
            require_uppercase_characters: false,
            require_lowercase_characters: false,
            allow_users_to_change_password: false,
            max_password_age: None,
            password_reuse_prevention: None,
            hard_expiry: None,
            expire_passwords: false,
        }
    }
}

/// An IAM server certificate.
#[derive(Debug, Clone)]
pub struct ServerCertificateEntry {
    pub server_certificate_name: String,
    pub server_certificate_id: String,
    pub arn: String,
    pub path: String,
    pub certificate_body: String,
    pub certificate_chain: Option<String>,
    pub private_key: String,
    pub upload_date: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
    pub tags: Vec<Tag>,
}

/// An SSH public key.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct SSHPublicKeyEntry {
    pub user_name: String,
    pub ssh_public_key_id: String,
    pub fingerprint: String,
    pub ssh_public_key_body: String,
    pub status: String,
    pub upload_date: DateTime<Utc>,
}

/// A signing certificate.
#[derive(Debug, Clone)]
pub struct SigningCertificateEntry {
    pub user_name: String,
    pub certificate_id: String,
    pub certificate_body: String,
    pub status: String,
    pub upload_date: DateTime<Utc>,
}

/// An account alias.
#[derive(Debug, Clone)]
pub struct AccountAlias {
    pub alias: String,
}

/// A service-specific credential (e.g., CodeCommit HTTPS credentials).
#[derive(Debug, Clone)]
pub struct ServiceSpecificCredential {
    pub service_specific_credential_id: String,
    pub user_name: String,
    pub service_name: String,
    pub service_user_name: String,
    pub service_password: String,
    pub status: String,
    pub create_date: DateTime<Utc>,
}
