//! Serde-compatible view types for IAM state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::IamService;
use crate::state::IamState;
use crate::types::{
    AccessKey, AccountPasswordPolicy, AttachedPolicy, Group, InlinePolicy, InstanceProfile,
    LoginProfile, MFADeviceAssociation, ManagedPolicy, OpenIDConnectProvider, PolicyVersionEntry,
    Role, SAMLProvider, SSHPublicKeyEntry, ServerCertificateEntry, ServiceSpecificCredential,
    SigningCertificateEntry, Tag, User, VirtualMFADevice,
};

/// Serializable view of the IAM state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IamStateView {
    /// Users keyed by username.
    #[serde(default)]
    pub users: HashMap<String, UserView>,
    /// Roles keyed by role name.
    #[serde(default)]
    pub roles: HashMap<String, RoleView>,
    /// Groups keyed by group name.
    #[serde(default)]
    pub groups: HashMap<String, GroupView>,
    /// Managed policies keyed by policy ARN.
    #[serde(default)]
    pub policies: HashMap<String, ManagedPolicyView>,
    /// Instance profiles keyed by name.
    #[serde(default)]
    pub instance_profiles: HashMap<String, InstanceProfileView>,
    /// Access keys keyed by access_key_id.
    #[serde(default)]
    pub access_keys: HashMap<String, AccessKeyView>,
    /// Login profiles keyed by user name.
    #[serde(default)]
    pub login_profiles: HashMap<String, LoginProfileView>,
    /// OpenID Connect providers keyed by ARN.
    #[serde(default)]
    pub oidc_providers: HashMap<String, OidcProviderView>,
    /// SAML providers keyed by ARN.
    #[serde(default)]
    pub saml_providers: HashMap<String, SamlProviderView>,
    /// Virtual MFA devices keyed by serial number.
    #[serde(default)]
    pub virtual_mfa_devices: HashMap<String, VirtualMfaDeviceView>,
    /// MFA device associations (user + device pairs).
    #[serde(default)]
    pub mfa_associations: Vec<MfaAssociationView>,
    /// Account password policy (None = not set).
    #[serde(default)]
    pub account_password_policy: Option<AccountPasswordPolicyView>,
    /// Account aliases.
    #[serde(default)]
    pub account_aliases: Vec<String>,
    /// Server certificates keyed by name.
    #[serde(default)]
    pub server_certificates: HashMap<String, ServerCertificateView>,
    /// SSH public keys keyed by ssh_public_key_id.
    #[serde(default)]
    pub ssh_public_keys: HashMap<String, SshPublicKeyView>,
    /// Signing certificates keyed by certificate_id.
    #[serde(default)]
    pub signing_certificates: HashMap<String, SigningCertificateView>,
    /// Service-specific credentials keyed by service_specific_credential_id.
    #[serde(default)]
    pub service_specific_credentials: HashMap<String, ServiceSpecificCredentialView>,
}

/// Serializable view of an IAM user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserView {
    pub name: String,
    pub user_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub create_date: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub attached_policies: Vec<AttachedPolicyView>,
    #[serde(default)]
    pub inline_policies: Vec<InlinePolicyView>,
    #[serde(default)]
    pub permissions_boundary: Option<String>,
}

/// Serializable view of an IAM role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleView {
    pub name: String,
    pub role_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub assume_role_policy_document: String,
    pub description: String,
    pub create_date: Option<String>,
    pub max_session_duration: i32,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub attached_policies: Vec<AttachedPolicyView>,
    #[serde(default)]
    pub inline_policies: Vec<InlinePolicyView>,
    pub permissions_boundary: Option<String>,
}

/// Serializable view of an IAM group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupView {
    pub name: String,
    pub group_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub create_date: Option<String>,
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub attached_policies: Vec<AttachedPolicyView>,
    #[serde(default)]
    pub inline_policies: Vec<InlinePolicyView>,
}

/// Serializable view of a managed policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedPolicyView {
    pub policy_name: String,
    pub policy_id: String,
    pub arn: String,
    pub path: String,
    pub default_version_id: String,
    pub description: String,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
    pub is_attachable: bool,
    pub document: String,
    #[serde(default)]
    pub versions: Vec<PolicyVersionView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub attachment_count: u32,
}

/// Serializable view of a policy version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyVersionView {
    pub version_id: String,
    pub document: String,
    pub is_default_version: bool,
    pub create_date: Option<String>,
}

/// Serializable view of an attached policy reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachedPolicyView {
    pub policy_name: String,
    pub policy_arn: String,
}

/// Serializable view of an inline policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlinePolicyView {
    pub policy_name: String,
    pub policy_document: String,
}

/// Serializable view of an instance profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceProfileView {
    pub name: String,
    pub instance_profile_id: String,
    pub account_id: String,
    pub path: String,
    pub arn: String,
    pub create_date: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an IAM access key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessKeyView {
    pub user_name: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub status: String,
    pub create_date: Option<String>,
}

/// Serializable view of an IAM login profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginProfileView {
    pub user_name: String,
    pub create_date: Option<String>,
    pub password_reset_required: bool,
}

/// Serializable view of an IAM OpenID Connect provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcProviderView {
    pub arn: String,
    pub url: String,
    #[serde(default)]
    pub client_id_list: Vec<String>,
    #[serde(default)]
    pub thumbprint_list: Vec<String>,
    pub create_date: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an IAM SAML provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlProviderView {
    pub arn: String,
    pub name: String,
    pub saml_metadata_document: String,
    pub create_date: Option<String>,
    pub valid_until: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an IAM virtual MFA device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMfaDeviceView {
    pub serial_number: String,
    pub base32_string_seed: String,
    pub qr_code_png: String,
    pub user_name: Option<String>,
    pub enable_date: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an MFA device association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaAssociationView {
    pub user_name: String,
    pub serial_number: String,
    pub enable_date: String,
}

/// Serializable view of an account password policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPasswordPolicyView {
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

/// Serializable view of an IAM server certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCertificateView {
    pub server_certificate_name: String,
    pub server_certificate_id: String,
    pub arn: String,
    pub path: String,
    pub certificate_body: String,
    pub certificate_chain: Option<String>,
    pub private_key: String,
    pub upload_date: Option<String>,
    pub expiration: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an SSH public key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshPublicKeyView {
    pub user_name: String,
    pub ssh_public_key_id: String,
    pub fingerprint: String,
    pub ssh_public_key_body: String,
    pub status: String,
    pub upload_date: Option<String>,
}

/// Serializable view of a signing certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningCertificateView {
    pub user_name: String,
    pub certificate_id: String,
    pub certificate_body: String,
    pub status: String,
    pub upload_date: Option<String>,
}

/// Serializable view of a service-specific credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpecificCredentialView {
    pub service_specific_credential_id: String,
    pub user_name: String,
    pub service_name: String,
    pub service_user_name: String,
    pub service_password: String,
    pub status: String,
    pub create_date: Option<String>,
}

// --- Helper: tag conversions ---

fn tags_to_map(tags: &[Tag]) -> HashMap<String, String> {
    tags.iter()
        .map(|t| (t.key.clone(), t.value.clone()))
        .collect()
}

fn map_to_tags(map: &HashMap<String, String>) -> Vec<Tag> {
    map.iter()
        .map(|(k, v)| Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect()
}

fn parse_datetime(s: Option<&str>) -> DateTime<Utc> {
    s.and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now)
}

fn parse_datetime_opt(s: Option<&str>) -> Option<DateTime<Utc>> {
    s.and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
}

// --- From internal types to view types ---

impl From<&IamState> for IamStateView {
    fn from(state: &IamState) -> Self {
        IamStateView {
            users: state
                .users
                .iter()
                .map(|(k, v)| (k.clone(), UserView::from(v)))
                .collect(),
            roles: state
                .roles
                .iter()
                .map(|(k, v)| (k.clone(), RoleView::from(v)))
                .collect(),
            groups: state
                .groups
                .iter()
                .map(|(k, v)| (k.clone(), GroupView::from(v)))
                .collect(),
            policies: state
                .policies
                .iter()
                .map(|(k, v)| (k.clone(), ManagedPolicyView::from(v)))
                .collect(),
            instance_profiles: state
                .instance_profiles
                .iter()
                .map(|(k, v)| (k.clone(), InstanceProfileView::from(v)))
                .collect(),
            access_keys: state
                .access_keys
                .iter()
                .map(|(k, v)| (k.clone(), AccessKeyView::from(v)))
                .collect(),
            login_profiles: state
                .login_profiles
                .iter()
                .map(|(k, v)| (k.clone(), LoginProfileView::from(v)))
                .collect(),
            oidc_providers: state
                .oidc_providers
                .iter()
                .map(|(k, v)| (k.clone(), OidcProviderView::from(v)))
                .collect(),
            saml_providers: state
                .saml_providers
                .iter()
                .map(|(k, v)| (k.clone(), SamlProviderView::from(v)))
                .collect(),
            virtual_mfa_devices: state
                .virtual_mfa_devices
                .iter()
                .map(|(k, v)| (k.clone(), VirtualMfaDeviceView::from(v)))
                .collect(),
            mfa_associations: state
                .mfa_associations
                .iter()
                .map(MfaAssociationView::from)
                .collect(),
            account_password_policy: state
                .account_password_policy
                .as_ref()
                .map(AccountPasswordPolicyView::from),
            account_aliases: state.account_aliases.clone(),
            server_certificates: state
                .server_certificates
                .iter()
                .map(|(k, v)| (k.clone(), ServerCertificateView::from(v)))
                .collect(),
            ssh_public_keys: state
                .ssh_public_keys
                .iter()
                .map(|(k, v)| (k.clone(), SshPublicKeyView::from(v)))
                .collect(),
            signing_certificates: state
                .signing_certificates
                .iter()
                .map(|(k, v)| (k.clone(), SigningCertificateView::from(v)))
                .collect(),
            service_specific_credentials: state
                .service_specific_credentials
                .iter()
                .map(|(k, v)| (k.clone(), ServiceSpecificCredentialView::from(v)))
                .collect(),
        }
    }
}

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        UserView {
            name: u.name.clone(),
            user_id: u.user_id.clone(),
            account_id: u.account_id.clone(),
            path: u.path.clone(),
            arn: u.arn.clone(),
            create_date: Some(u.create_date.to_rfc3339()),
            tags: tags_to_map(&u.tags),
            attached_policies: u
                .attached_policies
                .iter()
                .map(AttachedPolicyView::from)
                .collect(),
            inline_policies: u
                .inline_policies
                .iter()
                .map(InlinePolicyView::from)
                .collect(),
            permissions_boundary: u.permissions_boundary.clone(),
        }
    }
}

impl From<&Role> for RoleView {
    fn from(r: &Role) -> Self {
        RoleView {
            name: r.name.clone(),
            role_id: r.role_id.clone(),
            account_id: r.account_id.clone(),
            path: r.path.clone(),
            arn: r.arn.clone(),
            assume_role_policy_document: r.assume_role_policy_document.clone(),
            description: r.description.clone(),
            create_date: Some(r.create_date.to_rfc3339()),
            max_session_duration: r.max_session_duration,
            tags: tags_to_map(&r.tags),
            attached_policies: r
                .attached_policies
                .iter()
                .map(AttachedPolicyView::from)
                .collect(),
            inline_policies: r
                .inline_policies
                .iter()
                .map(InlinePolicyView::from)
                .collect(),
            permissions_boundary: r.permissions_boundary.clone(),
        }
    }
}

impl From<&Group> for GroupView {
    fn from(g: &Group) -> Self {
        GroupView {
            name: g.name.clone(),
            group_id: g.group_id.clone(),
            account_id: g.account_id.clone(),
            path: g.path.clone(),
            arn: g.arn.clone(),
            create_date: Some(g.create_date.to_rfc3339()),
            members: g.members.clone(),
            attached_policies: g
                .attached_policies
                .iter()
                .map(AttachedPolicyView::from)
                .collect(),
            inline_policies: g
                .inline_policies
                .iter()
                .map(InlinePolicyView::from)
                .collect(),
        }
    }
}

impl From<&ManagedPolicy> for ManagedPolicyView {
    fn from(p: &ManagedPolicy) -> Self {
        ManagedPolicyView {
            policy_name: p.policy_name.clone(),
            policy_id: p.policy_id.clone(),
            arn: p.arn.clone(),
            path: p.path.clone(),
            default_version_id: p.default_version_id.clone(),
            description: p.description.clone(),
            create_date: Some(p.create_date.to_rfc3339()),
            update_date: Some(p.update_date.to_rfc3339()),
            is_attachable: p.is_attachable,
            document: p.document.clone(),
            versions: p.versions.iter().map(PolicyVersionView::from).collect(),
            tags: tags_to_map(&p.tags),
            attachment_count: p.attachment_count,
        }
    }
}

impl From<&PolicyVersionEntry> for PolicyVersionView {
    fn from(v: &PolicyVersionEntry) -> Self {
        PolicyVersionView {
            version_id: v.version_id.clone(),
            document: v.document.clone(),
            is_default_version: v.is_default_version,
            create_date: Some(v.create_date.to_rfc3339()),
        }
    }
}

impl From<&AttachedPolicy> for AttachedPolicyView {
    fn from(ap: &AttachedPolicy) -> Self {
        AttachedPolicyView {
            policy_name: ap.policy_name.clone(),
            policy_arn: ap.policy_arn.clone(),
        }
    }
}

impl From<&InlinePolicy> for InlinePolicyView {
    fn from(ip: &InlinePolicy) -> Self {
        InlinePolicyView {
            policy_name: ip.policy_name.clone(),
            policy_document: ip.policy_document.clone(),
        }
    }
}

impl From<&InstanceProfile> for InstanceProfileView {
    fn from(ip: &InstanceProfile) -> Self {
        InstanceProfileView {
            name: ip.name.clone(),
            instance_profile_id: ip.instance_profile_id.clone(),
            account_id: ip.account_id.clone(),
            path: ip.path.clone(),
            arn: ip.arn.clone(),
            create_date: Some(ip.create_date.to_rfc3339()),
            roles: ip.roles.clone(),
            tags: tags_to_map(&ip.tags),
        }
    }
}

impl From<&AccessKey> for AccessKeyView {
    fn from(k: &AccessKey) -> Self {
        AccessKeyView {
            user_name: k.user_name.clone(),
            access_key_id: k.access_key_id.clone(),
            secret_access_key: k.secret_access_key.clone(),
            status: k.status.clone(),
            create_date: Some(k.create_date.to_rfc3339()),
        }
    }
}

impl From<&LoginProfile> for LoginProfileView {
    fn from(lp: &LoginProfile) -> Self {
        LoginProfileView {
            user_name: lp.user_name.clone(),
            create_date: Some(lp.create_date.to_rfc3339()),
            password_reset_required: lp.password_reset_required,
        }
    }
}

impl From<&OpenIDConnectProvider> for OidcProviderView {
    fn from(p: &OpenIDConnectProvider) -> Self {
        OidcProviderView {
            arn: p.arn.clone(),
            url: p.url.clone(),
            client_id_list: p.client_id_list.clone(),
            thumbprint_list: p.thumbprint_list.clone(),
            create_date: Some(p.create_date.to_rfc3339()),
            tags: tags_to_map(&p.tags),
        }
    }
}

impl From<&SAMLProvider> for SamlProviderView {
    fn from(p: &SAMLProvider) -> Self {
        SamlProviderView {
            arn: p.arn.clone(),
            name: p.name.clone(),
            saml_metadata_document: p.saml_metadata_document.clone(),
            create_date: Some(p.create_date.to_rfc3339()),
            valid_until: p.valid_until.map(|dt| dt.to_rfc3339()),
            tags: tags_to_map(&p.tags),
        }
    }
}

impl From<&VirtualMFADevice> for VirtualMfaDeviceView {
    fn from(d: &VirtualMFADevice) -> Self {
        VirtualMfaDeviceView {
            serial_number: d.serial_number.clone(),
            base32_string_seed: d.base32_string_seed.clone(),
            qr_code_png: d.qr_code_png.clone(),
            user_name: d.user_name.clone(),
            enable_date: d.enable_date.map(|dt| dt.to_rfc3339()),
            tags: tags_to_map(&d.tags),
        }
    }
}

impl From<&MFADeviceAssociation> for MfaAssociationView {
    fn from(a: &MFADeviceAssociation) -> Self {
        MfaAssociationView {
            user_name: a.user_name.clone(),
            serial_number: a.serial_number.clone(),
            enable_date: a.enable_date.to_rfc3339(),
        }
    }
}

impl From<&AccountPasswordPolicy> for AccountPasswordPolicyView {
    fn from(p: &AccountPasswordPolicy) -> Self {
        AccountPasswordPolicyView {
            minimum_password_length: p.minimum_password_length,
            require_symbols: p.require_symbols,
            require_numbers: p.require_numbers,
            require_uppercase_characters: p.require_uppercase_characters,
            require_lowercase_characters: p.require_lowercase_characters,
            allow_users_to_change_password: p.allow_users_to_change_password,
            max_password_age: p.max_password_age,
            password_reuse_prevention: p.password_reuse_prevention,
            hard_expiry: p.hard_expiry,
            expire_passwords: p.expire_passwords,
        }
    }
}

impl From<&ServerCertificateEntry> for ServerCertificateView {
    fn from(c: &ServerCertificateEntry) -> Self {
        ServerCertificateView {
            server_certificate_name: c.server_certificate_name.clone(),
            server_certificate_id: c.server_certificate_id.clone(),
            arn: c.arn.clone(),
            path: c.path.clone(),
            certificate_body: c.certificate_body.clone(),
            certificate_chain: c.certificate_chain.clone(),
            private_key: c.private_key.clone(),
            upload_date: Some(c.upload_date.to_rfc3339()),
            expiration: c.expiration.map(|dt| dt.to_rfc3339()),
            tags: tags_to_map(&c.tags),
        }
    }
}

impl From<&SSHPublicKeyEntry> for SshPublicKeyView {
    fn from(k: &SSHPublicKeyEntry) -> Self {
        SshPublicKeyView {
            user_name: k.user_name.clone(),
            ssh_public_key_id: k.ssh_public_key_id.clone(),
            fingerprint: k.fingerprint.clone(),
            ssh_public_key_body: k.ssh_public_key_body.clone(),
            status: k.status.clone(),
            upload_date: Some(k.upload_date.to_rfc3339()),
        }
    }
}

impl From<&SigningCertificateEntry> for SigningCertificateView {
    fn from(c: &SigningCertificateEntry) -> Self {
        SigningCertificateView {
            user_name: c.user_name.clone(),
            certificate_id: c.certificate_id.clone(),
            certificate_body: c.certificate_body.clone(),
            status: c.status.clone(),
            upload_date: Some(c.upload_date.to_rfc3339()),
        }
    }
}

impl From<&ServiceSpecificCredential> for ServiceSpecificCredentialView {
    fn from(c: &ServiceSpecificCredential) -> Self {
        ServiceSpecificCredentialView {
            service_specific_credential_id: c.service_specific_credential_id.clone(),
            user_name: c.user_name.clone(),
            service_name: c.service_name.clone(),
            service_user_name: c.service_user_name.clone(),
            service_password: c.service_password.clone(),
            status: c.status.clone(),
            create_date: Some(c.create_date.to_rfc3339()),
        }
    }
}

// --- From view types to internal types ---

impl From<UserView> for User {
    fn from(v: UserView) -> Self {
        User {
            name: v.name,
            user_id: v.user_id,
            account_id: v.account_id,
            path: v.path,
            arn: v.arn,
            create_date: parse_datetime(v.create_date.as_deref()),
            tags: map_to_tags(&v.tags),
            attached_policies: v
                .attached_policies
                .into_iter()
                .map(AttachedPolicy::from)
                .collect(),
            inline_policies: v
                .inline_policies
                .into_iter()
                .map(InlinePolicy::from)
                .collect(),
            permissions_boundary: v.permissions_boundary,
        }
    }
}

impl From<RoleView> for Role {
    fn from(v: RoleView) -> Self {
        Role {
            name: v.name,
            role_id: v.role_id,
            account_id: v.account_id,
            path: v.path,
            arn: v.arn,
            assume_role_policy_document: v.assume_role_policy_document,
            description: v.description,
            create_date: parse_datetime(v.create_date.as_deref()),
            max_session_duration: v.max_session_duration,
            tags: map_to_tags(&v.tags),
            attached_policies: v
                .attached_policies
                .into_iter()
                .map(AttachedPolicy::from)
                .collect(),
            inline_policies: v
                .inline_policies
                .into_iter()
                .map(InlinePolicy::from)
                .collect(),
            permissions_boundary: v.permissions_boundary,
        }
    }
}

impl From<GroupView> for Group {
    fn from(v: GroupView) -> Self {
        Group {
            name: v.name,
            group_id: v.group_id,
            account_id: v.account_id,
            path: v.path,
            arn: v.arn,
            create_date: parse_datetime(v.create_date.as_deref()),
            members: v.members,
            attached_policies: v
                .attached_policies
                .into_iter()
                .map(AttachedPolicy::from)
                .collect(),
            inline_policies: v
                .inline_policies
                .into_iter()
                .map(InlinePolicy::from)
                .collect(),
        }
    }
}

impl From<ManagedPolicyView> for ManagedPolicy {
    fn from(v: ManagedPolicyView) -> Self {
        ManagedPolicy {
            policy_name: v.policy_name,
            policy_id: v.policy_id,
            arn: v.arn,
            path: v.path,
            default_version_id: v.default_version_id,
            description: v.description,
            create_date: parse_datetime(v.create_date.as_deref()),
            update_date: parse_datetime(v.update_date.as_deref()),
            is_attachable: v.is_attachable,
            document: v.document,
            versions: v
                .versions
                .into_iter()
                .map(PolicyVersionEntry::from)
                .collect(),
            tags: map_to_tags(&v.tags),
            next_version_number: 1,
            attachment_count: v.attachment_count,
        }
    }
}

impl From<PolicyVersionView> for PolicyVersionEntry {
    fn from(v: PolicyVersionView) -> Self {
        PolicyVersionEntry {
            version_id: v.version_id,
            document: v.document,
            is_default_version: v.is_default_version,
            create_date: parse_datetime(v.create_date.as_deref()),
        }
    }
}

impl From<AttachedPolicyView> for AttachedPolicy {
    fn from(v: AttachedPolicyView) -> Self {
        AttachedPolicy {
            policy_name: v.policy_name,
            policy_arn: v.policy_arn,
        }
    }
}

impl From<InlinePolicyView> for InlinePolicy {
    fn from(v: InlinePolicyView) -> Self {
        InlinePolicy {
            policy_name: v.policy_name,
            policy_document: v.policy_document,
        }
    }
}

impl From<InstanceProfileView> for InstanceProfile {
    fn from(v: InstanceProfileView) -> Self {
        InstanceProfile {
            name: v.name,
            instance_profile_id: v.instance_profile_id,
            account_id: v.account_id,
            path: v.path,
            arn: v.arn,
            create_date: parse_datetime(v.create_date.as_deref()),
            roles: v.roles,
            tags: map_to_tags(&v.tags),
        }
    }
}

impl From<AccessKeyView> for AccessKey {
    fn from(v: AccessKeyView) -> Self {
        AccessKey {
            user_name: v.user_name,
            access_key_id: v.access_key_id,
            secret_access_key: v.secret_access_key,
            status: v.status,
            create_date: parse_datetime(v.create_date.as_deref()),
        }
    }
}

impl From<LoginProfileView> for LoginProfile {
    fn from(v: LoginProfileView) -> Self {
        LoginProfile {
            user_name: v.user_name,
            create_date: parse_datetime(v.create_date.as_deref()),
            password_reset_required: v.password_reset_required,
        }
    }
}

impl From<OidcProviderView> for OpenIDConnectProvider {
    fn from(v: OidcProviderView) -> Self {
        OpenIDConnectProvider {
            arn: v.arn,
            url: v.url,
            client_id_list: v.client_id_list,
            thumbprint_list: v.thumbprint_list,
            create_date: parse_datetime(v.create_date.as_deref()),
            tags: map_to_tags(&v.tags),
        }
    }
}

impl From<SamlProviderView> for SAMLProvider {
    fn from(v: SamlProviderView) -> Self {
        SAMLProvider {
            arn: v.arn,
            name: v.name,
            saml_metadata_document: v.saml_metadata_document,
            create_date: parse_datetime(v.create_date.as_deref()),
            valid_until: parse_datetime_opt(v.valid_until.as_deref()),
            tags: map_to_tags(&v.tags),
        }
    }
}

impl From<VirtualMfaDeviceView> for VirtualMFADevice {
    fn from(v: VirtualMfaDeviceView) -> Self {
        VirtualMFADevice {
            serial_number: v.serial_number,
            base32_string_seed: v.base32_string_seed,
            qr_code_png: v.qr_code_png,
            user_name: v.user_name,
            enable_date: parse_datetime_opt(v.enable_date.as_deref()),
            tags: map_to_tags(&v.tags),
        }
    }
}

impl From<MfaAssociationView> for MFADeviceAssociation {
    fn from(v: MfaAssociationView) -> Self {
        MFADeviceAssociation {
            user_name: v.user_name,
            serial_number: v.serial_number,
            enable_date: parse_datetime(Some(&v.enable_date)),
        }
    }
}

impl From<AccountPasswordPolicyView> for AccountPasswordPolicy {
    fn from(v: AccountPasswordPolicyView) -> Self {
        AccountPasswordPolicy {
            minimum_password_length: v.minimum_password_length,
            require_symbols: v.require_symbols,
            require_numbers: v.require_numbers,
            require_uppercase_characters: v.require_uppercase_characters,
            require_lowercase_characters: v.require_lowercase_characters,
            allow_users_to_change_password: v.allow_users_to_change_password,
            max_password_age: v.max_password_age,
            password_reuse_prevention: v.password_reuse_prevention,
            hard_expiry: v.hard_expiry,
            expire_passwords: v.expire_passwords,
        }
    }
}

impl From<ServerCertificateView> for ServerCertificateEntry {
    fn from(v: ServerCertificateView) -> Self {
        ServerCertificateEntry {
            server_certificate_name: v.server_certificate_name,
            server_certificate_id: v.server_certificate_id,
            arn: v.arn,
            path: v.path,
            certificate_body: v.certificate_body,
            certificate_chain: v.certificate_chain,
            private_key: v.private_key,
            upload_date: parse_datetime(v.upload_date.as_deref()),
            expiration: parse_datetime_opt(v.expiration.as_deref()),
            tags: map_to_tags(&v.tags),
        }
    }
}

impl From<SshPublicKeyView> for SSHPublicKeyEntry {
    fn from(v: SshPublicKeyView) -> Self {
        SSHPublicKeyEntry {
            user_name: v.user_name,
            ssh_public_key_id: v.ssh_public_key_id,
            fingerprint: v.fingerprint,
            ssh_public_key_body: v.ssh_public_key_body,
            status: v.status,
            upload_date: parse_datetime(v.upload_date.as_deref()),
        }
    }
}

impl From<SigningCertificateView> for SigningCertificateEntry {
    fn from(v: SigningCertificateView) -> Self {
        SigningCertificateEntry {
            user_name: v.user_name,
            certificate_id: v.certificate_id,
            certificate_body: v.certificate_body,
            status: v.status,
            upload_date: parse_datetime(v.upload_date.as_deref()),
        }
    }
}

impl From<ServiceSpecificCredentialView> for ServiceSpecificCredential {
    fn from(v: ServiceSpecificCredentialView) -> Self {
        ServiceSpecificCredential {
            service_specific_credential_id: v.service_specific_credential_id,
            user_name: v.user_name,
            service_name: v.service_name,
            service_user_name: v.service_user_name,
            service_password: v.service_password,
            status: v.status,
            create_date: parse_datetime(v.create_date.as_deref()),
        }
    }
}

impl From<IamStateView> for IamState {
    fn from(view: IamStateView) -> Self {
        IamState {
            users: view
                .users
                .into_iter()
                .map(|(k, v)| (k, User::from(v)))
                .collect(),
            roles: view
                .roles
                .into_iter()
                .map(|(k, v)| (k, Role::from(v)))
                .collect(),
            groups: view
                .groups
                .into_iter()
                .map(|(k, v)| (k, Group::from(v)))
                .collect(),
            policies: view
                .policies
                .into_iter()
                .map(|(k, v)| (k, ManagedPolicy::from(v)))
                .collect(),
            instance_profiles: view
                .instance_profiles
                .into_iter()
                .map(|(k, v)| (k, InstanceProfile::from(v)))
                .collect(),
            access_keys: view
                .access_keys
                .into_iter()
                .map(|(k, v)| (k, AccessKey::from(v)))
                .collect(),
            login_profiles: view
                .login_profiles
                .into_iter()
                .map(|(k, v)| (k, LoginProfile::from(v)))
                .collect(),
            oidc_providers: view
                .oidc_providers
                .into_iter()
                .map(|(k, v)| (k, OpenIDConnectProvider::from(v)))
                .collect(),
            saml_providers: view
                .saml_providers
                .into_iter()
                .map(|(k, v)| (k, SAMLProvider::from(v)))
                .collect(),
            virtual_mfa_devices: view
                .virtual_mfa_devices
                .into_iter()
                .map(|(k, v)| (k, VirtualMFADevice::from(v)))
                .collect(),
            mfa_associations: view
                .mfa_associations
                .into_iter()
                .map(MFADeviceAssociation::from)
                .collect(),
            account_password_policy: view
                .account_password_policy
                .map(AccountPasswordPolicy::from),
            account_aliases: view.account_aliases,
            server_certificates: view
                .server_certificates
                .into_iter()
                .map(|(k, v)| (k, ServerCertificateEntry::from(v)))
                .collect(),
            ssh_public_keys: view
                .ssh_public_keys
                .into_iter()
                .map(|(k, v)| (k, SSHPublicKeyEntry::from(v)))
                .collect(),
            signing_certificates: view
                .signing_certificates
                .into_iter()
                .map(|(k, v)| (k, SigningCertificateEntry::from(v)))
                .collect(),
            service_specific_credentials: view
                .service_specific_credentials
                .into_iter()
                .map(|(k, v)| (k, ServiceSpecificCredential::from(v)))
                .collect(),
            // Transient state rebuilt on restore:
            //   - access_key_last_used: telemetry, not durable
            //   - service_linked_role_deletions: ephemeral task status
            ..Default::default()
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for IamService {
    type StateView = IamStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        IamStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = IamState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.users {
                guard.users.insert(k, User::from(v));
            }
            for (k, v) in view.roles {
                guard.roles.insert(k, Role::from(v));
            }
            for (k, v) in view.groups {
                guard.groups.insert(k, Group::from(v));
            }
            for (k, v) in view.policies {
                guard.policies.insert(k, ManagedPolicy::from(v));
            }
            for (k, v) in view.instance_profiles {
                guard.instance_profiles.insert(k, InstanceProfile::from(v));
            }
            for (k, v) in view.access_keys {
                guard.access_keys.insert(k, AccessKey::from(v));
            }
            for (k, v) in view.login_profiles {
                guard.login_profiles.insert(k, LoginProfile::from(v));
            }
            for (k, v) in view.oidc_providers {
                guard
                    .oidc_providers
                    .insert(k, OpenIDConnectProvider::from(v));
            }
            for (k, v) in view.saml_providers {
                guard.saml_providers.insert(k, SAMLProvider::from(v));
            }
            for (k, v) in view.virtual_mfa_devices {
                guard
                    .virtual_mfa_devices
                    .insert(k, VirtualMFADevice::from(v));
            }
            for assoc in view.mfa_associations {
                let assoc = MFADeviceAssociation::from(assoc);
                if !guard.mfa_associations.iter().any(|a| {
                    a.user_name == assoc.user_name && a.serial_number == assoc.serial_number
                }) {
                    guard.mfa_associations.push(assoc);
                }
            }
            if let Some(policy) = view.account_password_policy {
                guard.account_password_policy = Some(AccountPasswordPolicy::from(policy));
            }
            for alias in view.account_aliases {
                if !guard.account_aliases.contains(&alias) {
                    guard.account_aliases.push(alias);
                }
            }
            for (k, v) in view.server_certificates {
                guard
                    .server_certificates
                    .insert(k, ServerCertificateEntry::from(v));
            }
            for (k, v) in view.ssh_public_keys {
                guard.ssh_public_keys.insert(k, SSHPublicKeyEntry::from(v));
            }
            for (k, v) in view.signing_certificates {
                guard
                    .signing_certificates
                    .insert(k, SigningCertificateEntry::from(v));
            }
            for (k, v) in view.service_specific_credentials {
                guard
                    .service_specific_credentials
                    .insert(k, ServiceSpecificCredential::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
