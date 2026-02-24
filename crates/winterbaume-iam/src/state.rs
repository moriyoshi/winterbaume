use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

const LIMIT_KEYS_PER_USER: usize = 2;
const MAX_POLICY_VERSIONS: usize = 5;

/// In-memory state for the IAM service.
#[derive(Debug, Default)]
pub struct IamState {
    /// Users keyed by username.
    pub users: HashMap<String, User>,
    /// Roles keyed by role name.
    pub roles: HashMap<String, Role>,
    /// Groups keyed by group name.
    pub groups: HashMap<String, Group>,
    /// Access keys keyed by access_key_id.
    pub access_keys: HashMap<String, AccessKey>,
    /// Managed policies keyed by policy ARN.
    pub policies: HashMap<String, ManagedPolicy>,
    /// Instance profiles keyed by name.
    pub instance_profiles: HashMap<String, InstanceProfile>,
    /// Login profiles keyed by user name.
    pub login_profiles: HashMap<String, LoginProfile>,
    /// OpenID Connect providers keyed by ARN.
    pub oidc_providers: HashMap<String, OpenIDConnectProvider>,
    /// SAML providers keyed by ARN.
    pub saml_providers: HashMap<String, SAMLProvider>,
    /// Virtual MFA devices keyed by serial number.
    pub virtual_mfa_devices: HashMap<String, VirtualMFADevice>,
    /// MFA device associations keyed by (user_name, serial_number).
    pub mfa_associations: Vec<MFADeviceAssociation>,
    /// Account password policy (None = not set).
    pub account_password_policy: Option<AccountPasswordPolicy>,
    /// Account aliases.
    pub account_aliases: Vec<String>,
    /// Server certificates keyed by name.
    pub server_certificates: HashMap<String, ServerCertificateEntry>,
    /// SSH public keys keyed by ssh_public_key_id.
    pub ssh_public_keys: HashMap<String, SSHPublicKeyEntry>,
    /// Signing certificates keyed by certificate_id.
    pub signing_certificates: HashMap<String, SigningCertificateEntry>,
    /// Service linked role deletion tasks. Key: deletion_task_id, Value: status.
    pub service_linked_role_deletions: HashMap<String, String>,
    /// Access key last used info keyed by access_key_id.
    pub access_key_last_used: HashMap<String, AccessKeyLastUsedInfo>,
    /// Service-specific credentials keyed by service_specific_credential_id.
    pub service_specific_credentials: HashMap<String, ServiceSpecificCredential>,
    /// Whether a credential report has been generated (true = COMPLETE, false = not generated).
    pub credential_report_generated: bool,
    /// Service-last-accessed jobs keyed by job_id. Value is the ARN that was submitted.
    pub service_last_accessed_jobs: HashMap<String, String>,
    /// Organizations root credentials management enabled features.
    /// None = not yet enabled/disabled (treat as disabled). Some(vec) = enabled with given features.
    pub org_root_credentials_management_features: Option<Vec<String>>,
    /// Organizations root sessions enabled features.
    /// None = not yet enabled/disabled (treat as disabled). Some(vec) = enabled with given features.
    pub org_root_sessions_features: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct AccessKeyLastUsedInfo {
    pub last_used_date: Option<chrono::DateTime<chrono::Utc>>,
    pub service_name: Option<String>,
    pub region: Option<String>,
}

/// Error type for IAM operations.
#[derive(Debug, thiserror::Error)]
pub enum IamError {
    // EntityAlreadyExists (409)
    #[error("User {0} already exists")]
    UserAlreadyExists(String),
    #[error("Role with name {0} already exists.")]
    RoleAlreadyExists(String),
    #[error("Group with name {0} already exists.")]
    GroupAlreadyExists(String),
    #[error("Group {0} already exists")]
    GroupAlreadyExistsRename(String),
    #[error("A policy called {0} already exists. Duplicate names are not allowed.")]
    PolicyAlreadyExists(String),
    #[error("Instance profile {0} already exists.")]
    InstanceProfileAlreadyExists(String),
    #[error("Login profile for user {0} already exists.")]
    LoginProfileAlreadyExists(String),
    #[error("Unknown")]
    OidcProviderAlreadyExists,
    #[error("A SAML provider with name {0} already exists.")]
    SamlProviderAlreadyExists(String),
    #[error("MFA device {0} already exists.")]
    MfaDeviceAlreadyExists(String),
    #[error("An account alias {0} already exists.")]
    AccountAliasAlreadyExists(String),
    #[error("Server certificate {0} already exists.")]
    ServerCertificateAlreadyExists(String),

    // NoSuchEntity (404)
    #[error("The user with name {0} cannot be found.")]
    UserNotFound(String),
    #[error("User {0} cannot be found.")]
    UserNotFoundAlt(String),
    #[error("User {0} does not exist.")]
    UserNotFoundSsc(String),
    #[error("Role with name {0} cannot be found.")]
    RoleNotFound(String),
    #[error("Group {0} cannot be found.")]
    GroupNotFound(String),
    #[error("The user with name {0} is not in group {1}.")]
    UserNotInGroup(String, String),
    #[error("The Access Key with id {0} cannot be found.")]
    AccessKeyNotFound(String),
    #[error("Policy {0} does not exist.")]
    PolicyNotFound(String),
    #[error("Policy {0} was not found.")]
    PolicyNotFoundDelete(String),
    #[error("Policy {0} is not attachable.")]
    PolicyNotAttachable(String),
    #[error("Policy {0} does not exist or is not attachable.")]
    PolicyNotFoundOrNotAttachable(String),
    #[error("Policy version {0} does not exist for policy {1}.")]
    PolicyVersionNotFound(String, String),
    #[error("Policy version {0} does not exist.")]
    PolicyVersionNotFoundSimple(String),
    #[error("Policy {0} was not found on role {1}.")]
    PolicyNotOnRole(String, String),
    #[error("Policy {0} was not found on user {1}.")]
    PolicyNotOnUser(String, String),
    #[error("Policy {0} was not found on group {1}.")]
    PolicyNotOnGroup(String, String),
    #[error("The user policy with name {0} cannot be found.")]
    UserInlinePolicyNotFound(String),
    #[error("The role policy with name {0} cannot be found.")]
    RoleInlinePolicyNotFound(String),
    #[error("The group policy with name {0} cannot be found.")]
    GroupInlinePolicyNotFound(String),
    #[error("Instance profile {0} cannot be found.")]
    InstanceProfileNotFound(String),
    #[error("Role {0} is not associated with instance profile {1}.")]
    RoleNotInInstanceProfile(String, String),
    #[error("Login profile for user {0} cannot be found.")]
    LoginProfileNotFound(String),
    #[error("OpenID Connect provider {0} does not exist.")]
    OidcProviderNotFound(String),
    #[error("SAML provider {0} does not exist.")]
    SamlProviderNotFound(String),
    #[error("MFA device {0} does not exist.")]
    MfaDeviceNotFound(String),
    #[error("MFA device {0} is not associated with user {1}.")]
    MfaDeviceNotAssociated(String, String),
    #[error("The Password Policy with domain name IAM not found.")]
    PasswordPolicyNotFound,
    #[error("The account alias {0} does not exist.")]
    AccountAliasNotFound(String),
    #[error("Server certificate {0} does not exist.")]
    ServerCertificateNotFound(String),
    #[error("SSH public key {0} cannot be found.")]
    SshPublicKeyNotFound(String),
    #[error("Signing certificate {0} cannot be found.")]
    SigningCertificateNotFound(String),
    #[error("Deletion task {0} does not exist.")]
    DeletionTaskNotFound(String),
    #[error("Service specific credential {0} does not exist.")]
    ServiceSpecificCredentialNotFound(String),
    #[error("Service specific credential {0} does not belong to user {1}.")]
    ServiceSpecificCredentialWrongUser(String, String),

    // DeleteConflict (409)
    #[error("Cannot delete entity, must delete access keys first for user {0}.")]
    DeleteConflictUserHasAccessKeys(String),
    #[error("Cannot delete entity, must detach all policies first for user {0}.")]
    DeleteConflictUserHasAttachedPolicies(String),
    #[error("Cannot delete entity, must delete all inline policies first for user {0}.")]
    DeleteConflictUserHasInlinePolicies(String),
    #[error("Cannot delete entity, must detach all policies first for role {0}.")]
    DeleteConflictRoleHasAttachedPolicies(String),
    #[error("Cannot delete entity, must delete all inline policies first for role {0}.")]
    DeleteConflictRoleHasInlinePolicies(String),
    #[error("Cannot delete entity, must remove role from all instance profiles first.")]
    DeleteConflictRoleInInstanceProfile,
    #[error("Cannot delete entity, must remove all members first.")]
    DeleteConflictGroupHasMembers,
    #[error("Cannot delete a policy attached to entities.")]
    DeleteConflictPolicyAttached,
    #[error("Cannot delete entity, must remove all roles first.")]
    DeleteConflictInstanceProfileHasRoles,
    #[error("Cannot delete the default version of a policy.")]
    DeleteConflictPolicyDefaultVersion,

    // LimitExceeded (409)
    #[error("Cannot exceed quota for AccessKeysPerUser: {0}")]
    LimitExceededAccessKeys(usize),
    #[error("A managed policy can have up to {0} versions.")]
    LimitExceededPolicyVersions(usize),
    #[error("Role {0} is already associated with instance profile {1}.")]
    LimitExceededRoleAlreadyInInstanceProfile(String, String),

    // InvalidInput (400)
    #[error("Service role name {0} has been taken.")]
    ServiceRoleNameTaken(String),
}

impl IamState {
    // ==================== User operations ====================

    pub fn create_user(
        &mut self,
        account_id: &str,
        user_name: &str,
        path: &str,
        tags: Vec<Tag>,
    ) -> Result<&User, IamError> {
        if self.users.contains_key(user_name) {
            return Err(IamError::UserAlreadyExists(user_name.to_string()));
        }

        let user_id = generate_resource_id("AIDA");
        let arn = format!("arn:aws:iam::{account_id}:user{path}{user_name}");

        let user = User {
            name: user_name.to_string(),
            user_id,
            account_id: account_id.to_string(),
            path: path.to_string(),
            arn,
            create_date: Utc::now(),
            tags,
            attached_policies: Vec::new(),
            inline_policies: Vec::new(),
            permissions_boundary: None,
        };

        self.users.insert(user_name.to_string(), user);
        Ok(self.users.get(user_name).unwrap())
    }

    pub fn get_user(&self, user_name: &str) -> Result<&User, IamError> {
        self.users
            .get(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))
    }

    pub fn delete_user(&mut self, user_name: &str) -> Result<(), IamError> {
        let user = self
            .users
            .get(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        // Check for access keys
        let has_keys = self.access_keys.values().any(|k| k.user_name == user_name);
        if has_keys {
            return Err(IamError::DeleteConflictUserHasAccessKeys(
                user_name.to_string(),
            ));
        }

        // Check for attached policies
        if !user.attached_policies.is_empty() {
            return Err(IamError::DeleteConflictUserHasAttachedPolicies(
                user_name.to_string(),
            ));
        }

        // Check for inline policies
        if !user.inline_policies.is_empty() {
            return Err(IamError::DeleteConflictUserHasInlinePolicies(
                user_name.to_string(),
            ));
        }

        self.users.remove(user_name);
        Ok(())
    }

    pub fn list_users(&self, path_prefix: Option<&str>) -> Vec<&User> {
        let prefix = path_prefix.unwrap_or("/");
        self.users
            .values()
            .filter(|u| u.path.starts_with(prefix))
            .collect()
    }

    pub fn update_user(
        &mut self,
        user_name: &str,
        new_user_name: Option<&str>,
        new_path: Option<&str>,
    ) -> Result<(), IamError> {
        let _user = self
            .users
            .get(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        if let Some(new_name) = new_user_name
            && new_name != user_name
            && self.users.contains_key(new_name)
        {
            return Err(IamError::UserAlreadyExists(new_name.to_string()));
        }

        let mut user = self.users.remove(user_name).unwrap();
        if let Some(new_path) = new_path {
            user.path = new_path.to_string();
            user.arn = format!(
                "arn:aws:iam::{}:user{}{}",
                user.account_id,
                new_path,
                new_user_name.unwrap_or(&user.name)
            );
        }
        let final_name = if let Some(new_name) = new_user_name {
            user.name = new_name.to_string();
            if new_path.is_none() {
                user.arn = format!(
                    "arn:aws:iam::{}:user{}{}",
                    user.account_id, user.path, new_name
                );
            }
            new_name.to_string()
        } else {
            user_name.to_string()
        };

        // Update group memberships
        for group in self.groups.values_mut() {
            if let Some(pos) = group.members.iter().position(|m| m == user_name) {
                group.members[pos] = final_name.clone();
            }
        }

        // Update access keys
        for key in self.access_keys.values_mut() {
            if key.user_name == user_name {
                key.user_name = final_name.clone();
            }
        }

        self.users.insert(final_name, user);
        Ok(())
    }

    // ==================== Role operations ====================

    pub fn create_role(
        &mut self,
        account_id: &str,
        role_name: &str,
        path: &str,
        assume_role_policy_document: &str,
        description: &str,
        max_session_duration: i32,
        tags: Vec<Tag>,
    ) -> Result<&Role, IamError> {
        if self.roles.contains_key(role_name) {
            return Err(IamError::RoleAlreadyExists(role_name.to_string()));
        }

        let role_id = generate_resource_id("AROA");
        let arn = format!("arn:aws:iam::{account_id}:role{path}{role_name}");

        let role = Role {
            name: role_name.to_string(),
            role_id,
            account_id: account_id.to_string(),
            path: path.to_string(),
            arn,
            assume_role_policy_document: assume_role_policy_document.to_string(),
            description: description.to_string(),
            create_date: Utc::now(),
            max_session_duration,
            tags,
            attached_policies: Vec::new(),
            inline_policies: Vec::new(),
            permissions_boundary: None,
        };

        self.roles.insert(role_name.to_string(), role);
        Ok(self.roles.get(role_name).unwrap())
    }

    pub fn get_role(&self, role_name: &str) -> Result<&Role, IamError> {
        self.roles
            .get(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))
    }

    pub fn delete_role(&mut self, role_name: &str) -> Result<(), IamError> {
        let role = self
            .roles
            .get(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        if !role.attached_policies.is_empty() {
            return Err(IamError::DeleteConflictRoleHasAttachedPolicies(
                role_name.to_string(),
            ));
        }

        if !role.inline_policies.is_empty() {
            return Err(IamError::DeleteConflictRoleHasInlinePolicies(
                role_name.to_string(),
            ));
        }

        // Check if role is attached to any instance profile
        let attached_to_profile = self
            .instance_profiles
            .values()
            .any(|ip| ip.roles.iter().any(|r| r == role_name));
        if attached_to_profile {
            return Err(IamError::DeleteConflictRoleInInstanceProfile);
        }

        self.roles.remove(role_name);
        Ok(())
    }

    pub fn list_roles(&self, path_prefix: Option<&str>) -> Vec<&Role> {
        let prefix = path_prefix.unwrap_or("/");
        let mut roles: Vec<&Role> = self
            .roles
            .values()
            .filter(|r| r.path.starts_with(prefix))
            .collect();
        roles.sort_by(|a, b| a.name.cmp(&b.name));
        roles
    }

    pub fn update_role(
        &mut self,
        role_name: &str,
        description: Option<&str>,
        max_session_duration: Option<i32>,
    ) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;
        if let Some(desc) = description {
            role.description = desc.to_string();
        }
        if let Some(dur) = max_session_duration {
            role.max_session_duration = dur;
        }
        Ok(())
    }

    pub fn update_role_description(
        &mut self,
        role_name: &str,
        description: &str,
    ) -> Result<&Role, IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;
        role.description = description.to_string();
        Ok(role)
    }

    pub fn update_assume_role_policy(
        &mut self,
        role_name: &str,
        policy_document: &str,
    ) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;
        role.assume_role_policy_document = policy_document.to_string();
        Ok(())
    }

    // ==================== Group operations ====================

    pub fn create_group(
        &mut self,
        account_id: &str,
        group_name: &str,
        path: &str,
    ) -> Result<&Group, IamError> {
        if self.groups.contains_key(group_name) {
            return Err(IamError::GroupAlreadyExists(group_name.to_string()));
        }

        let group_id = generate_resource_id("AGPA");
        let arn = format!("arn:aws:iam::{account_id}:group{path}{group_name}");

        let group = Group {
            name: group_name.to_string(),
            group_id,
            account_id: account_id.to_string(),
            path: path.to_string(),
            arn,
            create_date: Utc::now(),
            members: Vec::new(),
            attached_policies: Vec::new(),
            inline_policies: Vec::new(),
        };

        self.groups.insert(group_name.to_string(), group);
        Ok(self.groups.get(group_name).unwrap())
    }

    pub fn get_group(&self, group_name: &str) -> Result<(&Group, Vec<&User>), IamError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        let members: Vec<&User> = group
            .members
            .iter()
            .filter_map(|name| self.users.get(name))
            .collect();

        Ok((group, members))
    }

    pub fn delete_group(&mut self, group_name: &str) -> Result<(), IamError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        if !group.members.is_empty() {
            return Err(IamError::DeleteConflictGroupHasMembers);
        }

        self.groups.remove(group_name);
        Ok(())
    }

    pub fn list_groups(&self, path_prefix: Option<&str>) -> Vec<&Group> {
        let prefix = path_prefix.unwrap_or("/");
        let mut groups: Vec<&Group> = self
            .groups
            .values()
            .filter(|g| g.path.starts_with(prefix))
            .collect();
        groups.sort_by(|a, b| a.name.cmp(&b.name));
        groups
    }

    pub fn update_group(
        &mut self,
        group_name: &str,
        new_group_name: Option<&str>,
        new_path: Option<&str>,
    ) -> Result<(), IamError> {
        let _group = self
            .groups
            .get(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        if let Some(new_name) = new_group_name
            && new_name != group_name
            && self.groups.contains_key(new_name)
        {
            return Err(IamError::GroupAlreadyExistsRename(new_name.to_string()));
        }

        let mut group = self.groups.remove(group_name).unwrap();
        if let Some(new_path) = new_path {
            group.path = new_path.to_string();
            group.arn = format!(
                "arn:aws:iam::{}:group{}{}",
                group.account_id,
                new_path,
                new_group_name.unwrap_or(&group.name)
            );
        }
        let final_name = if let Some(new_name) = new_group_name {
            group.name = new_name.to_string();
            if new_path.is_none() {
                group.arn = format!(
                    "arn:aws:iam::{}:group{}{}",
                    group.account_id, group.path, new_name
                );
            }
            new_name.to_string()
        } else {
            group_name.to_string()
        };

        self.groups.insert(final_name, group);
        Ok(())
    }

    pub fn add_user_to_group(&mut self, group_name: &str, user_name: &str) -> Result<(), IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        if !group.members.contains(&user_name.to_string()) {
            group.members.push(user_name.to_string());
        }
        Ok(())
    }

    pub fn remove_user_from_group(
        &mut self,
        group_name: &str,
        user_name: &str,
    ) -> Result<(), IamError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        let initial_len = group.members.len();
        group.members.retain(|m| m != user_name);
        if group.members.len() == initial_len {
            return Err(IamError::UserNotInGroup(
                user_name.to_string(),
                group_name.to_string(),
            ));
        }
        Ok(())
    }

    // ==================== Access key operations ====================

    pub fn create_access_key(&mut self, user_name: &str) -> Result<&AccessKey, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        let existing_count = self
            .access_keys
            .values()
            .filter(|k| k.user_name == user_name)
            .count();
        if existing_count >= LIMIT_KEYS_PER_USER {
            return Err(IamError::LimitExceededAccessKeys(LIMIT_KEYS_PER_USER));
        }

        let access_key_id = format!("AKIA{}", random_alphanumeric(16));
        let secret_access_key = random_alphanumeric(40);

        let key = AccessKey {
            user_name: user_name.to_string(),
            access_key_id: access_key_id.clone(),
            secret_access_key,
            status: "Active".to_string(),
            create_date: Utc::now(),
        };

        self.access_keys.insert(access_key_id.clone(), key);
        Ok(self.access_keys.get(&access_key_id).unwrap())
    }

    pub fn list_access_keys(&self, user_name: &str) -> Result<Vec<&AccessKey>, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        Ok(self
            .access_keys
            .values()
            .filter(|k| k.user_name == user_name)
            .collect())
    }

    pub fn delete_access_key(
        &mut self,
        user_name: &str,
        access_key_id: &str,
    ) -> Result<(), IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        match self.access_keys.get(access_key_id) {
            Some(key) if key.user_name == user_name => {
                self.access_keys.remove(access_key_id);
                Ok(())
            }
            _ => Err(IamError::AccessKeyNotFound(access_key_id.to_string())),
        }
    }

    pub fn update_access_key(
        &mut self,
        user_name: &str,
        access_key_id: &str,
        status: &str,
    ) -> Result<(), IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        match self.access_keys.get_mut(access_key_id) {
            Some(key) if key.user_name == user_name => {
                key.status = status.to_string();
                Ok(())
            }
            _ => Err(IamError::AccessKeyNotFound(access_key_id.to_string())),
        }
    }

    pub fn get_access_key_last_used(
        &self,
        access_key_id: &str,
    ) -> Result<(&AccessKey, Option<&AccessKeyLastUsedInfo>), IamError> {
        let key = self
            .access_keys
            .get(access_key_id)
            .ok_or_else(|| IamError::AccessKeyNotFound(access_key_id.to_string()))?;
        let last_used = self.access_key_last_used.get(access_key_id);
        Ok((key, last_used))
    }

    // ==================== Policy operations ====================

    pub fn create_policy(
        &mut self,
        account_id: &str,
        policy_name: &str,
        path: &str,
        policy_document: &str,
        description: &str,
        tags: Vec<Tag>,
    ) -> Result<&ManagedPolicy, IamError> {
        let arn = format!("arn:aws:iam::{account_id}:policy{path}{policy_name}");

        if self.policies.contains_key(&arn) {
            return Err(IamError::PolicyAlreadyExists(policy_name.to_string()));
        }

        let policy_id = generate_policy_id();
        let now = Utc::now();

        let version = PolicyVersionEntry {
            version_id: "v1".to_string(),
            document: policy_document.to_string(),
            is_default_version: true,
            create_date: now,
        };

        let policy = ManagedPolicy {
            policy_name: policy_name.to_string(),
            policy_id,
            arn: arn.clone(),
            path: path.to_string(),
            default_version_id: "v1".to_string(),
            attachment_count: 0,
            description: description.to_string(),
            create_date: now,
            update_date: now,
            is_attachable: true,
            document: policy_document.to_string(),
            versions: vec![version],
            tags,
            next_version_number: 2,
        };

        self.policies.insert(arn.clone(), policy);
        Ok(self.policies.get(&arn).unwrap())
    }

    pub fn get_policy(&self, policy_arn: &str) -> Result<&ManagedPolicy, IamError> {
        self.policies
            .get(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))
    }

    pub fn delete_policy(&mut self, policy_arn: &str) -> Result<(), IamError> {
        if !self.policies.contains_key(policy_arn) {
            return Err(IamError::PolicyNotFoundDelete(policy_arn.to_string()));
        }

        let attached_to_role = self.roles.values().any(|r| {
            r.attached_policies
                .iter()
                .any(|p| p.policy_arn == policy_arn)
        });
        let attached_to_user = self.users.values().any(|u| {
            u.attached_policies
                .iter()
                .any(|p| p.policy_arn == policy_arn)
        });
        let attached_to_group = self.groups.values().any(|g| {
            g.attached_policies
                .iter()
                .any(|p| p.policy_arn == policy_arn)
        });
        if attached_to_role || attached_to_user || attached_to_group {
            return Err(IamError::DeleteConflictPolicyAttached);
        }

        self.policies.remove(policy_arn);
        Ok(())
    }

    pub fn list_policies(
        &self,
        scope: Option<&str>,
        path_prefix: Option<&str>,
    ) -> Vec<&ManagedPolicy> {
        let prefix = path_prefix.unwrap_or("/");
        let _scope = scope.unwrap_or("All");
        self.policies
            .values()
            .filter(|p| p.path.starts_with(prefix))
            .collect()
    }

    pub fn create_policy_version(
        &mut self,
        policy_arn: &str,
        policy_document: &str,
        set_as_default: bool,
    ) -> Result<&PolicyVersionEntry, IamError> {
        let policy = self
            .policies
            .get_mut(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;

        if policy.versions.len() >= MAX_POLICY_VERSIONS {
            return Err(IamError::LimitExceededPolicyVersions(MAX_POLICY_VERSIONS));
        }

        let version_num = policy.next_version_number;
        policy.next_version_number += 1;
        let version_id = format!("v{version_num}");

        if set_as_default {
            for v in &mut policy.versions {
                v.is_default_version = false;
            }
            policy.default_version_id = version_id.clone();
            policy.document = policy_document.to_string();
            policy.update_date = Utc::now();
        }

        policy.versions.push(PolicyVersionEntry {
            version_id: version_id.clone(),
            document: policy_document.to_string(),
            is_default_version: set_as_default,
            create_date: Utc::now(),
        });

        let policy = self.policies.get(policy_arn).unwrap();
        let version = policy
            .versions
            .iter()
            .find(|v| v.version_id == version_id)
            .unwrap();
        Ok(version)
    }

    pub fn get_policy_version(
        &self,
        policy_arn: &str,
        version_id: &str,
    ) -> Result<&PolicyVersionEntry, IamError> {
        let policy = self
            .policies
            .get(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;

        policy
            .versions
            .iter()
            .find(|v| v.version_id == version_id)
            .ok_or_else(|| {
                IamError::PolicyVersionNotFound(version_id.to_string(), policy_arn.to_string())
            })
    }

    pub fn delete_policy_version(
        &mut self,
        policy_arn: &str,
        version_id: &str,
    ) -> Result<(), IamError> {
        let policy = self
            .policies
            .get_mut(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;

        if policy.default_version_id == version_id {
            return Err(IamError::DeleteConflictPolicyDefaultVersion);
        }

        let initial_len = policy.versions.len();
        policy.versions.retain(|v| v.version_id != version_id);
        if policy.versions.len() == initial_len {
            return Err(IamError::PolicyVersionNotFound(
                version_id.to_string(),
                policy_arn.to_string(),
                // placeholder to keep original match shape - replaced below
            ));
        }
        Ok(())
    }

    pub fn list_policy_versions(
        &self,
        policy_arn: &str,
    ) -> Result<&[PolicyVersionEntry], IamError> {
        let policy = self
            .policies
            .get(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;
        Ok(&policy.versions)
    }

    pub fn set_default_policy_version(
        &mut self,
        policy_arn: &str,
        version_id: &str,
    ) -> Result<(), IamError> {
        let policy = self
            .policies
            .get_mut(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;

        let version_exists = policy.versions.iter().any(|v| v.version_id == version_id);
        if !version_exists {
            return Err(IamError::PolicyVersionNotFoundSimple(
                version_id.to_string(),
            ));
        }

        for v in &mut policy.versions {
            v.is_default_version = v.version_id == version_id;
            if v.is_default_version {
                policy.document = v.document.clone();
            }
        }
        policy.default_version_id = version_id.to_string();
        policy.update_date = Utc::now();
        Ok(())
    }

    // ==================== Policy attachment operations ====================

    pub fn attach_role_policy(
        &mut self,
        role_name: &str,
        policy_arn: &str,
    ) -> Result<(), IamError> {
        let policy_name = {
            let policy = self
                .policies
                .get(policy_arn)
                .ok_or_else(|| IamError::PolicyNotFoundOrNotAttachable(policy_arn.to_string()))?;
            if !policy.is_attachable {
                return Err(IamError::PolicyNotAttachable(policy_arn.to_string()));
            }
            policy.policy_name.clone()
        };

        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        if !role
            .attached_policies
            .iter()
            .any(|p| p.policy_arn == policy_arn)
        {
            role.attached_policies.push(AttachedPolicy {
                policy_name,
                policy_arn: policy_arn.to_string(),
            });

            if let Some(policy) = self.policies.get_mut(policy_arn) {
                policy.attachment_count += 1;
            }
        }

        Ok(())
    }

    pub fn detach_role_policy(
        &mut self,
        role_name: &str,
        policy_arn: &str,
    ) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        let initial_len = role.attached_policies.len();
        role.attached_policies
            .retain(|p| p.policy_arn != policy_arn);
        if role.attached_policies.len() == initial_len {
            return Err(IamError::PolicyNotOnRole(
                policy_arn.to_string(),
                role_name.to_string(),
            ));
        }

        if let Some(policy) = self.policies.get_mut(policy_arn) {
            policy.attachment_count = policy.attachment_count.saturating_sub(1);
        }

        Ok(())
    }

    pub fn list_attached_role_policies(
        &self,
        role_name: &str,
    ) -> Result<Vec<&AttachedPolicy>, IamError> {
        let role = self
            .roles
            .get(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        Ok(role.attached_policies.iter().collect())
    }

    pub fn attach_user_policy(
        &mut self,
        user_name: &str,
        policy_arn: &str,
    ) -> Result<(), IamError> {
        let policy_name = {
            let policy = self
                .policies
                .get(policy_arn)
                .ok_or_else(|| IamError::PolicyNotFoundOrNotAttachable(policy_arn.to_string()))?;
            policy.policy_name.clone()
        };

        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        if !user
            .attached_policies
            .iter()
            .any(|p| p.policy_arn == policy_arn)
        {
            user.attached_policies.push(AttachedPolicy {
                policy_name,
                policy_arn: policy_arn.to_string(),
            });

            if let Some(policy) = self.policies.get_mut(policy_arn) {
                policy.attachment_count += 1;
            }
        }

        Ok(())
    }

    pub fn detach_user_policy(
        &mut self,
        user_name: &str,
        policy_arn: &str,
    ) -> Result<(), IamError> {
        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        let initial_len = user.attached_policies.len();
        user.attached_policies
            .retain(|p| p.policy_arn != policy_arn);
        if user.attached_policies.len() == initial_len {
            return Err(IamError::PolicyNotOnUser(
                policy_arn.to_string(),
                user_name.to_string(),
            ));
        }

        if let Some(policy) = self.policies.get_mut(policy_arn) {
            policy.attachment_count = policy.attachment_count.saturating_sub(1);
        }

        Ok(())
    }

    pub fn list_attached_user_policies(
        &self,
        user_name: &str,
    ) -> Result<Vec<&AttachedPolicy>, IamError> {
        let user = self
            .users
            .get(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        Ok(user.attached_policies.iter().collect())
    }

    pub fn attach_group_policy(
        &mut self,
        group_name: &str,
        policy_arn: &str,
    ) -> Result<(), IamError> {
        let policy_name = {
            let policy = self
                .policies
                .get(policy_arn)
                .ok_or_else(|| IamError::PolicyNotFoundOrNotAttachable(policy_arn.to_string()))?;
            policy.policy_name.clone()
        };

        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        if !group
            .attached_policies
            .iter()
            .any(|p| p.policy_arn == policy_arn)
        {
            group.attached_policies.push(AttachedPolicy {
                policy_name,
                policy_arn: policy_arn.to_string(),
            });

            if let Some(policy) = self.policies.get_mut(policy_arn) {
                policy.attachment_count += 1;
            }
        }
        Ok(())
    }

    pub fn detach_group_policy(
        &mut self,
        group_name: &str,
        policy_arn: &str,
    ) -> Result<(), IamError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        let initial_len = group.attached_policies.len();
        group
            .attached_policies
            .retain(|p| p.policy_arn != policy_arn);
        if group.attached_policies.len() == initial_len {
            return Err(IamError::PolicyNotOnGroup(
                policy_arn.to_string(),
                group_name.to_string(),
            ));
        }

        if let Some(policy) = self.policies.get_mut(policy_arn) {
            policy.attachment_count = policy.attachment_count.saturating_sub(1);
        }
        Ok(())
    }

    pub fn list_attached_group_policies(
        &self,
        group_name: &str,
    ) -> Result<Vec<&AttachedPolicy>, IamError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;
        Ok(group.attached_policies.iter().collect())
    }

    // ==================== Inline policy operations ====================

    pub fn put_user_policy(
        &mut self,
        user_name: &str,
        policy_name: &str,
        policy_document: &str,
    ) -> Result<(), IamError> {
        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        if let Some(existing) = user
            .inline_policies
            .iter_mut()
            .find(|p| p.policy_name == policy_name)
        {
            existing.policy_document = policy_document.to_string();
        } else {
            user.inline_policies.push(InlinePolicy {
                policy_name: policy_name.to_string(),
                policy_document: policy_document.to_string(),
            });
        }
        Ok(())
    }

    pub fn get_user_policy(
        &self,
        user_name: &str,
        policy_name: &str,
    ) -> Result<&InlinePolicy, IamError> {
        let user = self
            .users
            .get(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        user.inline_policies
            .iter()
            .find(|p| p.policy_name == policy_name)
            .ok_or_else(|| IamError::UserInlinePolicyNotFound(policy_name.to_string()))
    }

    pub fn delete_user_policy(
        &mut self,
        user_name: &str,
        policy_name: &str,
    ) -> Result<(), IamError> {
        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        let initial_len = user.inline_policies.len();
        user.inline_policies
            .retain(|p| p.policy_name != policy_name);
        if user.inline_policies.len() == initial_len {
            return Err(IamError::UserInlinePolicyNotFound(policy_name.to_string()));
        }
        Ok(())
    }

    pub fn list_user_policies(&self, user_name: &str) -> Result<Vec<String>, IamError> {
        let user = self
            .users
            .get(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;
        Ok(user
            .inline_policies
            .iter()
            .map(|p| p.policy_name.clone())
            .collect())
    }

    pub fn put_role_policy(
        &mut self,
        role_name: &str,
        policy_name: &str,
        policy_document: &str,
    ) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        if let Some(existing) = role
            .inline_policies
            .iter_mut()
            .find(|p| p.policy_name == policy_name)
        {
            existing.policy_document = policy_document.to_string();
        } else {
            role.inline_policies.push(InlinePolicy {
                policy_name: policy_name.to_string(),
                policy_document: policy_document.to_string(),
            });
        }
        Ok(())
    }

    pub fn get_role_policy(
        &self,
        role_name: &str,
        policy_name: &str,
    ) -> Result<&InlinePolicy, IamError> {
        let role = self
            .roles
            .get(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        role.inline_policies
            .iter()
            .find(|p| p.policy_name == policy_name)
            .ok_or_else(|| IamError::RoleInlinePolicyNotFound(policy_name.to_string()))
    }

    pub fn delete_role_policy(
        &mut self,
        role_name: &str,
        policy_name: &str,
    ) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        let initial_len = role.inline_policies.len();
        role.inline_policies
            .retain(|p| p.policy_name != policy_name);
        if role.inline_policies.len() == initial_len {
            return Err(IamError::RoleInlinePolicyNotFound(policy_name.to_string()));
        }
        Ok(())
    }

    pub fn list_role_policies(&self, role_name: &str) -> Result<Vec<String>, IamError> {
        let role = self
            .roles
            .get(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;
        Ok(role
            .inline_policies
            .iter()
            .map(|p| p.policy_name.clone())
            .collect())
    }

    pub fn put_group_policy(
        &mut self,
        group_name: &str,
        policy_name: &str,
        policy_document: &str,
    ) -> Result<(), IamError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        if let Some(existing) = group
            .inline_policies
            .iter_mut()
            .find(|p| p.policy_name == policy_name)
        {
            existing.policy_document = policy_document.to_string();
        } else {
            group.inline_policies.push(InlinePolicy {
                policy_name: policy_name.to_string(),
                policy_document: policy_document.to_string(),
            });
        }
        Ok(())
    }

    pub fn get_group_policy(
        &self,
        group_name: &str,
        policy_name: &str,
    ) -> Result<&InlinePolicy, IamError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        group
            .inline_policies
            .iter()
            .find(|p| p.policy_name == policy_name)
            .ok_or_else(|| IamError::GroupInlinePolicyNotFound(policy_name.to_string()))
    }

    pub fn delete_group_policy(
        &mut self,
        group_name: &str,
        policy_name: &str,
    ) -> Result<(), IamError> {
        let group = self
            .groups
            .get_mut(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;

        let initial_len = group.inline_policies.len();
        group
            .inline_policies
            .retain(|p| p.policy_name != policy_name);
        if group.inline_policies.len() == initial_len {
            return Err(IamError::GroupInlinePolicyNotFound(policy_name.to_string()));
        }
        Ok(())
    }

    pub fn list_group_policies(&self, group_name: &str) -> Result<Vec<String>, IamError> {
        let group = self
            .groups
            .get(group_name)
            .ok_or_else(|| IamError::GroupNotFound(group_name.to_string()))?;
        Ok(group
            .inline_policies
            .iter()
            .map(|p| p.policy_name.clone())
            .collect())
    }

    // ==================== Tag operations ====================

    pub fn tag_user(&mut self, user_name: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        for new_tag in tags {
            if let Some(existing) = user.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                user.tags.push(new_tag);
            }
        }

        Ok(())
    }

    pub fn untag_user(&mut self, user_name: &str, tag_keys: &[String]) -> Result<(), IamError> {
        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        user.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_user_tags(&self, user_name: &str) -> Result<&[Tag], IamError> {
        let user = self
            .users
            .get(user_name)
            .ok_or_else(|| IamError::UserNotFound(user_name.to_string()))?;

        Ok(&user.tags)
    }

    pub fn tag_role(&mut self, role_name: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        for new_tag in tags {
            if let Some(existing) = role.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                role.tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_role(&mut self, role_name: &str, tag_keys: &[String]) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;

        role.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_role_tags(&self, role_name: &str) -> Result<&[Tag], IamError> {
        let role = self
            .roles
            .get(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;
        Ok(&role.tags)
    }

    pub fn tag_policy(&mut self, policy_arn: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let policy = self
            .policies
            .get_mut(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;

        for new_tag in tags {
            if let Some(existing) = policy.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                policy.tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_policy(&mut self, policy_arn: &str, tag_keys: &[String]) -> Result<(), IamError> {
        let policy = self
            .policies
            .get_mut(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;

        policy.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_policy_tags(&self, policy_arn: &str) -> Result<&[Tag], IamError> {
        let policy = self
            .policies
            .get(policy_arn)
            .ok_or_else(|| IamError::PolicyNotFound(policy_arn.to_string()))?;
        Ok(&policy.tags)
    }

    // ==================== Instance profile operations ====================

    pub fn create_instance_profile(
        &mut self,
        account_id: &str,
        name: &str,
        path: &str,
        tags: Vec<Tag>,
    ) -> Result<&InstanceProfile, IamError> {
        if self.instance_profiles.contains_key(name) {
            return Err(IamError::InstanceProfileAlreadyExists(name.to_string()));
        }

        let ip_id = generate_resource_id("AIPA");
        let arn = format!("arn:aws:iam::{account_id}:instance-profile{path}{name}");

        let ip = InstanceProfile {
            name: name.to_string(),
            instance_profile_id: ip_id,
            account_id: account_id.to_string(),
            path: path.to_string(),
            arn,
            create_date: Utc::now(),
            roles: Vec::new(),
            tags,
        };

        self.instance_profiles.insert(name.to_string(), ip);
        Ok(self.instance_profiles.get(name).unwrap())
    }

    pub fn get_instance_profile(&self, name: &str) -> Result<&InstanceProfile, IamError> {
        self.instance_profiles
            .get(name)
            .ok_or_else(|| IamError::InstanceProfileNotFound(name.to_string()))
    }

    pub fn delete_instance_profile(&mut self, name: &str) -> Result<(), IamError> {
        let ip = self
            .instance_profiles
            .get(name)
            .ok_or_else(|| IamError::InstanceProfileNotFound(name.to_string()))?;

        if !ip.roles.is_empty() {
            return Err(IamError::DeleteConflictInstanceProfileHasRoles);
        }

        self.instance_profiles.remove(name);
        Ok(())
    }

    pub fn add_role_to_instance_profile(
        &mut self,
        instance_profile_name: &str,
        role_name: &str,
    ) -> Result<(), IamError> {
        if !self.roles.contains_key(role_name) {
            return Err(IamError::RoleNotFound(role_name.to_string()));
        }

        let ip = self
            .instance_profiles
            .get_mut(instance_profile_name)
            .ok_or_else(|| IamError::InstanceProfileNotFound(instance_profile_name.to_string()))?;

        if ip.roles.contains(&role_name.to_string()) {
            return Err(IamError::LimitExceededRoleAlreadyInInstanceProfile(
                role_name.to_string(),
                instance_profile_name.to_string(),
            ));
        }

        ip.roles.push(role_name.to_string());
        Ok(())
    }

    pub fn remove_role_from_instance_profile(
        &mut self,
        instance_profile_name: &str,
        role_name: &str,
    ) -> Result<(), IamError> {
        let ip = self
            .instance_profiles
            .get_mut(instance_profile_name)
            .ok_or_else(|| IamError::InstanceProfileNotFound(instance_profile_name.to_string()))?;

        let initial_len = ip.roles.len();
        ip.roles.retain(|r| r != role_name);
        if ip.roles.len() == initial_len {
            return Err(IamError::RoleNotInInstanceProfile(
                role_name.to_string(),
                instance_profile_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn tag_instance_profile(&mut self, name: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let ip = self
            .instance_profiles
            .get_mut(name)
            .ok_or_else(|| IamError::InstanceProfileNotFound(name.to_string()))?;

        for new_tag in tags {
            if let Some(existing) = ip.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                ip.tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_instance_profile(
        &mut self,
        name: &str,
        tag_keys: &[String],
    ) -> Result<(), IamError> {
        let ip = self
            .instance_profiles
            .get_mut(name)
            .ok_or_else(|| IamError::InstanceProfileNotFound(name.to_string()))?;

        ip.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_instance_profile_tags(&self, name: &str) -> Result<&[Tag], IamError> {
        let ip = self
            .instance_profiles
            .get(name)
            .ok_or_else(|| IamError::InstanceProfileNotFound(name.to_string()))?;
        Ok(&ip.tags)
    }

    // ==================== Login profile operations ====================

    pub fn create_login_profile(
        &mut self,
        user_name: &str,
        _password: &str,
        password_reset_required: bool,
    ) -> Result<&LoginProfile, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        if self.login_profiles.contains_key(user_name) {
            return Err(IamError::LoginProfileAlreadyExists(user_name.to_string()));
        }

        let profile = LoginProfile {
            user_name: user_name.to_string(),
            create_date: Utc::now(),
            password_reset_required,
        };

        self.login_profiles.insert(user_name.to_string(), profile);
        Ok(self.login_profiles.get(user_name).unwrap())
    }

    pub fn get_login_profile(&self, user_name: &str) -> Result<&LoginProfile, IamError> {
        self.login_profiles
            .get(user_name)
            .ok_or_else(|| IamError::LoginProfileNotFound(user_name.to_string()))
    }

    pub fn update_login_profile(
        &mut self,
        user_name: &str,
        _password: Option<&str>,
        password_reset_required: Option<bool>,
    ) -> Result<(), IamError> {
        let profile = self
            .login_profiles
            .get_mut(user_name)
            .ok_or_else(|| IamError::LoginProfileNotFound(user_name.to_string()))?;

        if let Some(reset) = password_reset_required {
            profile.password_reset_required = reset;
        }
        Ok(())
    }

    pub fn delete_login_profile(&mut self, user_name: &str) -> Result<(), IamError> {
        if self.login_profiles.remove(user_name).is_none() {
            return Err(IamError::LoginProfileNotFound(user_name.to_string()));
        }
        Ok(())
    }

    // ==================== OIDC provider operations ====================

    pub fn create_oidc_provider(
        &mut self,
        account_id: &str,
        url: &str,
        client_id_list: Vec<String>,
        thumbprint_list: Vec<String>,
        tags: Vec<Tag>,
    ) -> Result<&OpenIDConnectProvider, IamError> {
        // Strip protocol and query string, per AWS behavior
        let stripped = url
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        // Strip query string
        let stripped = if let Some(pos) = stripped.find('?') {
            &stripped[..pos]
        } else {
            stripped
        };

        let arn = format!("arn:aws:iam::{account_id}:oidc-provider/{stripped}");

        if self.oidc_providers.contains_key(&arn) {
            return Err(IamError::OidcProviderAlreadyExists);
        }

        let provider = OpenIDConnectProvider {
            arn: arn.clone(),
            url: stripped.to_string(),
            client_id_list,
            thumbprint_list,
            create_date: Utc::now(),
            tags,
        };

        self.oidc_providers.insert(arn.clone(), provider);
        Ok(self.oidc_providers.get(&arn).unwrap())
    }

    pub fn get_oidc_provider(&self, arn: &str) -> Result<&OpenIDConnectProvider, IamError> {
        self.oidc_providers
            .get(arn)
            .ok_or_else(|| IamError::OidcProviderNotFound(arn.to_string()))
    }

    pub fn delete_oidc_provider(&mut self, arn: &str) -> Result<(), IamError> {
        // Deleting a non-existing OIDC provider is idempotent (no error), per AWS behavior
        self.oidc_providers.remove(arn);
        Ok(())
    }

    pub fn list_oidc_providers(&self) -> Vec<&OpenIDConnectProvider> {
        self.oidc_providers.values().collect()
    }

    pub fn update_oidc_provider_thumbprint(
        &mut self,
        arn: &str,
        thumbprint_list: Vec<String>,
    ) -> Result<(), IamError> {
        let provider = self
            .oidc_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::OidcProviderNotFound(arn.to_string()))?;
        provider.thumbprint_list = thumbprint_list;
        Ok(())
    }

    pub fn tag_oidc_provider(&mut self, arn: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let provider = self
            .oidc_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::OidcProviderNotFound(arn.to_string()))?;

        for new_tag in tags {
            if let Some(existing) = provider.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                provider.tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_oidc_provider(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), IamError> {
        let provider = self
            .oidc_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::OidcProviderNotFound(arn.to_string()))?;

        provider.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_oidc_provider_tags(&self, arn: &str) -> Result<&[Tag], IamError> {
        let provider = self
            .oidc_providers
            .get(arn)
            .ok_or_else(|| IamError::OidcProviderNotFound(arn.to_string()))?;
        Ok(&provider.tags)
    }

    // ==================== SAML provider operations ====================

    pub fn create_saml_provider(
        &mut self,
        account_id: &str,
        name: &str,
        saml_metadata_document: &str,
        tags: Vec<Tag>,
    ) -> Result<&SAMLProvider, IamError> {
        let arn = format!("arn:aws:iam::{account_id}:saml-provider/{name}");

        if self.saml_providers.contains_key(&arn) {
            return Err(IamError::SamlProviderAlreadyExists(name.to_string()));
        }

        let provider = SAMLProvider {
            arn: arn.clone(),
            name: name.to_string(),
            saml_metadata_document: saml_metadata_document.to_string(),
            create_date: Utc::now(),
            valid_until: None,
            tags,
        };

        self.saml_providers.insert(arn.clone(), provider);
        Ok(self.saml_providers.get(&arn).unwrap())
    }

    pub fn get_saml_provider(&self, arn: &str) -> Result<&SAMLProvider, IamError> {
        self.saml_providers
            .get(arn)
            .ok_or_else(|| IamError::SamlProviderNotFound(arn.to_string()))
    }

    pub fn delete_saml_provider(&mut self, arn: &str) -> Result<(), IamError> {
        if self.saml_providers.remove(arn).is_none() {
            return Err(IamError::SamlProviderNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn update_saml_provider(
        &mut self,
        arn: &str,
        saml_metadata_document: &str,
    ) -> Result<&SAMLProvider, IamError> {
        let provider = self
            .saml_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::SamlProviderNotFound(arn.to_string()))?;
        provider.saml_metadata_document = saml_metadata_document.to_string();
        Ok(provider)
    }

    pub fn list_saml_providers(&self) -> Vec<&SAMLProvider> {
        self.saml_providers.values().collect()
    }

    // ==================== Virtual MFA device operations ====================

    pub fn create_virtual_mfa_device(
        &mut self,
        account_id: &str,
        virtual_mfa_device_name: &str,
        path: &str,
        tags: Vec<Tag>,
    ) -> Result<&VirtualMFADevice, IamError> {
        let serial = format!("arn:aws:iam::{account_id}:mfa{path}{virtual_mfa_device_name}");

        if self.virtual_mfa_devices.contains_key(&serial) {
            return Err(IamError::MfaDeviceAlreadyExists(
                virtual_mfa_device_name.to_string(),
            ));
        }

        let device = VirtualMFADevice {
            serial_number: serial.clone(),
            base32_string_seed: random_alphanumeric(32),
            qr_code_png: random_alphanumeric(64),
            user_name: None,
            enable_date: None,
            tags,
        };

        self.virtual_mfa_devices.insert(serial.clone(), device);
        Ok(self.virtual_mfa_devices.get(&serial).unwrap())
    }

    pub fn delete_virtual_mfa_device(&mut self, serial_number: &str) -> Result<(), IamError> {
        if self.virtual_mfa_devices.remove(serial_number).is_none() {
            return Err(IamError::MfaDeviceNotFound(serial_number.to_string()));
        }
        // Also remove any MFA association
        self.mfa_associations
            .retain(|a| a.serial_number != serial_number);
        Ok(())
    }

    pub fn list_virtual_mfa_devices(&self) -> Vec<&VirtualMFADevice> {
        self.virtual_mfa_devices.values().collect()
    }

    pub fn enable_mfa_device(
        &mut self,
        user_name: &str,
        serial_number: &str,
        _authentication_code1: &str,
        _authentication_code2: &str,
    ) -> Result<(), IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        // Update virtual MFA device if it's a virtual one
        if let Some(device) = self.virtual_mfa_devices.get_mut(serial_number) {
            device.user_name = Some(user_name.to_string());
            device.enable_date = Some(Utc::now());
        }

        self.mfa_associations.push(MFADeviceAssociation {
            user_name: user_name.to_string(),
            serial_number: serial_number.to_string(),
            enable_date: Utc::now(),
        });
        Ok(())
    }

    pub fn deactivate_mfa_device(
        &mut self,
        user_name: &str,
        serial_number: &str,
    ) -> Result<(), IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        let initial_len = self.mfa_associations.len();
        self.mfa_associations
            .retain(|a| !(a.user_name == user_name && a.serial_number == serial_number));
        if self.mfa_associations.len() == initial_len {
            return Err(IamError::MfaDeviceNotAssociated(
                serial_number.to_string(),
                user_name.to_string(),
            ));
        }

        // Update virtual MFA device
        if let Some(device) = self.virtual_mfa_devices.get_mut(serial_number) {
            device.user_name = None;
            device.enable_date = None;
        }
        Ok(())
    }

    pub fn list_mfa_devices(
        &self,
        user_name: &str,
    ) -> Result<Vec<&MFADeviceAssociation>, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        Ok(self
            .mfa_associations
            .iter()
            .filter(|a| a.user_name == user_name)
            .collect())
    }

    // ==================== Account password policy ====================

    pub fn update_account_password_policy(&mut self, policy: AccountPasswordPolicy) {
        self.account_password_policy = Some(policy);
    }

    pub fn get_account_password_policy(&self) -> Result<&AccountPasswordPolicy, IamError> {
        self.account_password_policy
            .as_ref()
            .ok_or(IamError::PasswordPolicyNotFound)
    }

    pub fn delete_account_password_policy(&mut self) -> Result<(), IamError> {
        if self.account_password_policy.is_none() {
            return Err(IamError::PasswordPolicyNotFound);
        }
        self.account_password_policy = None;
        Ok(())
    }

    // ==================== Account alias operations ====================

    pub fn create_account_alias(&mut self, alias: &str) -> Result<(), IamError> {
        if self.account_aliases.contains(&alias.to_string()) {
            return Err(IamError::AccountAliasAlreadyExists(alias.to_string()));
        }
        self.account_aliases.push(alias.to_string());
        Ok(())
    }

    pub fn delete_account_alias(&mut self, alias: &str) -> Result<(), IamError> {
        let initial_len = self.account_aliases.len();
        self.account_aliases.retain(|a| a != alias);
        if self.account_aliases.len() == initial_len {
            return Err(IamError::AccountAliasNotFound(alias.to_string()));
        }
        Ok(())
    }

    pub fn list_account_aliases(&self) -> &[String] {
        &self.account_aliases
    }

    // ==================== Server certificate operations ====================

    pub fn upload_server_certificate(
        &mut self,
        account_id: &str,
        name: &str,
        path: &str,
        certificate_body: &str,
        private_key: &str,
        certificate_chain: Option<&str>,
        tags: Vec<Tag>,
    ) -> Result<&ServerCertificateEntry, IamError> {
        if self.server_certificates.contains_key(name) {
            return Err(IamError::ServerCertificateAlreadyExists(name.to_string()));
        }

        let cert_id = generate_resource_id("ASCCA");
        let arn = format!("arn:aws:iam::{account_id}:server-certificate{path}{name}");

        let cert = ServerCertificateEntry {
            server_certificate_name: name.to_string(),
            server_certificate_id: cert_id,
            arn,
            path: path.to_string(),
            certificate_body: certificate_body.to_string(),
            certificate_chain: certificate_chain.map(|s| s.to_string()),
            private_key: private_key.to_string(),
            upload_date: Utc::now(),
            expiration: None,
            tags,
        };

        self.server_certificates.insert(name.to_string(), cert);
        Ok(self.server_certificates.get(name).unwrap())
    }

    pub fn get_server_certificate(&self, name: &str) -> Result<&ServerCertificateEntry, IamError> {
        self.server_certificates
            .get(name)
            .ok_or_else(|| IamError::ServerCertificateNotFound(name.to_string()))
    }

    pub fn delete_server_certificate(&mut self, name: &str) -> Result<(), IamError> {
        if self.server_certificates.remove(name).is_none() {
            return Err(IamError::ServerCertificateNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_server_certificates(&self) -> Vec<&ServerCertificateEntry> {
        self.server_certificates.values().collect()
    }

    // ==================== SSH public key operations ====================

    pub fn upload_ssh_public_key(
        &mut self,
        user_name: &str,
        ssh_public_key_body: &str,
    ) -> Result<&SSHPublicKeyEntry, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        let key_id = format!("APKA{}", random_alphanumeric(16));
        let fingerprint = format!("{}:{}", &random_alphanumeric(2), &random_alphanumeric(47));

        let entry = SSHPublicKeyEntry {
            user_name: user_name.to_string(),
            ssh_public_key_id: key_id.clone(),
            fingerprint,
            ssh_public_key_body: ssh_public_key_body.to_string(),
            status: "Active".to_string(),
            upload_date: Utc::now(),
        };

        self.ssh_public_keys.insert(key_id.clone(), entry);
        Ok(self.ssh_public_keys.get(&key_id).unwrap())
    }

    pub fn get_ssh_public_key(
        &self,
        user_name: &str,
        ssh_public_key_id: &str,
    ) -> Result<&SSHPublicKeyEntry, IamError> {
        match self.ssh_public_keys.get(ssh_public_key_id) {
            Some(key) if key.user_name == user_name => Ok(key),
            _ => Err(IamError::SshPublicKeyNotFound(
                ssh_public_key_id.to_string(),
            )),
        }
    }

    pub fn update_ssh_public_key(
        &mut self,
        user_name: &str,
        ssh_public_key_id: &str,
        status: &str,
    ) -> Result<(), IamError> {
        match self.ssh_public_keys.get_mut(ssh_public_key_id) {
            Some(key) if key.user_name == user_name => {
                key.status = status.to_string();
                Ok(())
            }
            _ => Err(IamError::SshPublicKeyNotFound(
                ssh_public_key_id.to_string(),
            )),
        }
    }

    pub fn delete_ssh_public_key(
        &mut self,
        user_name: &str,
        ssh_public_key_id: &str,
    ) -> Result<(), IamError> {
        match self.ssh_public_keys.get(ssh_public_key_id) {
            Some(key) if key.user_name == user_name => {
                self.ssh_public_keys.remove(ssh_public_key_id);
                Ok(())
            }
            _ => Err(IamError::SshPublicKeyNotFound(
                ssh_public_key_id.to_string(),
            )),
        }
    }

    // ==================== Signing certificate operations ====================

    pub fn upload_signing_certificate(
        &mut self,
        user_name: &str,
        certificate_body: &str,
    ) -> Result<&SigningCertificateEntry, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        let cert_id = generate_resource_id("ASCCA");

        let entry = SigningCertificateEntry {
            user_name: user_name.to_string(),
            certificate_id: cert_id.clone(),
            certificate_body: certificate_body.to_string(),
            status: "Active".to_string(),
            upload_date: Utc::now(),
        };

        self.signing_certificates.insert(cert_id.clone(), entry);
        Ok(self.signing_certificates.get(&cert_id).unwrap())
    }

    pub fn list_signing_certificates(
        &self,
        user_name: &str,
    ) -> Result<Vec<&SigningCertificateEntry>, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFound(user_name.to_string()));
        }

        Ok(self
            .signing_certificates
            .values()
            .filter(|c| c.user_name == user_name)
            .collect())
    }

    pub fn update_signing_certificate(
        &mut self,
        user_name: &str,
        certificate_id: &str,
        status: &str,
    ) -> Result<(), IamError> {
        match self.signing_certificates.get_mut(certificate_id) {
            Some(cert) if cert.user_name == user_name => {
                cert.status = status.to_string();
                Ok(())
            }
            _ => Err(IamError::SigningCertificateNotFound(
                certificate_id.to_string(),
            )),
        }
    }

    pub fn delete_signing_certificate(
        &mut self,
        user_name: &str,
        certificate_id: &str,
    ) -> Result<(), IamError> {
        match self.signing_certificates.get(certificate_id) {
            Some(cert) if cert.user_name == user_name => {
                self.signing_certificates.remove(certificate_id);
                Ok(())
            }
            _ => Err(IamError::SigningCertificateNotFound(
                certificate_id.to_string(),
            )),
        }
    }

    // ==================== Service linked role ====================

    pub fn create_service_linked_role(
        &mut self,
        account_id: &str,
        aws_service_name: &str,
        description: &str,
        _custom_suffix: Option<&str>,
    ) -> Result<&Role, IamError> {
        let role_name = format!(
            "AWSServiceRoleFor{}",
            service_name_to_role_suffix(aws_service_name)
        );
        let path = format!("/aws-service-role/{aws_service_name}/");

        if self.roles.contains_key(&role_name) {
            return Err(IamError::ServiceRoleNameTaken(role_name.clone()));
        }

        let role_id = generate_resource_id("AROA");
        let arn = format!("arn:aws:iam::{account_id}:role{path}{role_name}");
        let trust_policy = format!(
            r#"{{"Version":"2012-10-17","Statement":[{{"Effect":"Allow","Principal":{{"Service":"{aws_service_name}"}},"Action":"sts:AssumeRole"}}]}}"#
        );

        let role = Role {
            name: role_name.clone(),
            role_id,
            account_id: account_id.to_string(),
            path,
            arn,
            assume_role_policy_document: trust_policy,
            description: description.to_string(),
            create_date: Utc::now(),
            max_session_duration: 3600,
            tags: Vec::new(),
            attached_policies: Vec::new(),
            inline_policies: Vec::new(),
            permissions_boundary: None,
        };

        self.roles.insert(role_name.clone(), role);
        Ok(self.roles.get(&role_name).unwrap())
    }

    pub fn delete_service_linked_role(&mut self, role_name: &str) -> Result<String, IamError> {
        if !self.roles.contains_key(role_name) {
            return Err(IamError::RoleNotFound(role_name.to_string()));
        }

        self.roles.remove(role_name);
        let task_id = uuid::Uuid::new_v4().to_string();
        self.service_linked_role_deletions
            .insert(task_id.clone(), "SUCCEEDED".to_string());
        Ok(task_id)
    }

    pub fn get_service_linked_role_deletion_status(
        &self,
        deletion_task_id: &str,
    ) -> Result<&str, IamError> {
        self.service_linked_role_deletions
            .get(deletion_task_id)
            .map(|s| s.as_str())
            .ok_or_else(|| IamError::DeletionTaskNotFound(deletion_task_id.to_string()))
    }

    // ==================== Permissions boundary ====================

    pub fn put_role_permissions_boundary(
        &mut self,
        role_name: &str,
        permissions_boundary: &str,
    ) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;
        role.permissions_boundary = Some(permissions_boundary.to_string());
        Ok(())
    }

    pub fn delete_role_permissions_boundary(&mut self, role_name: &str) -> Result<(), IamError> {
        let role = self
            .roles
            .get_mut(role_name)
            .ok_or_else(|| IamError::RoleNotFound(role_name.to_string()))?;
        role.permissions_boundary = None;
        Ok(())
    }

    // ==================== Account summary ====================

    pub fn get_account_summary(&self) -> HashMap<String, i64> {
        let mut summary = HashMap::new();

        // Quotas (fixed values matching AWS defaults)
        summary.insert("GroupPolicySizeQuota".to_string(), 5120);
        summary.insert("InstanceProfilesQuota".to_string(), 1000);
        summary.insert("GroupsPerUserQuota".to_string(), 10);
        summary.insert("AttachedPoliciesPerUserQuota".to_string(), 10);
        summary.insert("PoliciesQuota".to_string(), 1500);
        summary.insert("AccessKeysPerUserQuota".to_string(), 2);
        summary.insert("AssumeRolePolicySizeQuota".to_string(), 2048);
        summary.insert("PolicyVersionsInUseQuota".to_string(), 10000);
        summary.insert("VersionsPerPolicyQuota".to_string(), 5);
        summary.insert("AttachedPoliciesPerGroupQuota".to_string(), 10);
        summary.insert("PolicySizeQuota".to_string(), 6144);
        summary.insert("UsersQuota".to_string(), 5000);
        summary.insert("ServerCertificatesQuota".to_string(), 20);
        summary.insert("UserPolicySizeQuota".to_string(), 2048);
        summary.insert("RolesQuota".to_string(), 1000);
        summary.insert("SigningCertificatesPerUserQuota".to_string(), 2);
        summary.insert("RolePolicySizeQuota".to_string(), 10240);
        summary.insert("AttachedPoliciesPerRoleQuota".to_string(), 10);
        summary.insert("GroupsQuota".to_string(), 300);
        summary.insert("GlobalEndpointTokenVersion".to_string(), 1);

        // Counts
        summary.insert("Users".to_string(), self.users.len() as i64);
        summary.insert("Roles".to_string(), self.roles.len() as i64);
        summary.insert("Policies".to_string(), self.policies.len() as i64);
        summary.insert("Groups".to_string(), self.groups.len() as i64);
        summary.insert(
            "InstanceProfiles".to_string(),
            self.instance_profiles.len() as i64,
        );
        summary.insert(
            "Providers".to_string(),
            (self.oidc_providers.len() + self.saml_providers.len()) as i64,
        );
        summary.insert(
            "MFADevices".to_string(),
            self.virtual_mfa_devices.len() as i64,
        );
        summary.insert(
            "MFADevicesInUse".to_string(),
            self.mfa_associations.len() as i64,
        );
        summary.insert(
            "ServerCertificates".to_string(),
            self.server_certificates.len() as i64,
        );
        summary.insert("AccountMFAEnabled".to_string(), 0);
        summary.insert("AccountSigningCertificatesPresent".to_string(), 0);
        summary.insert("AccountAccessKeysPresent".to_string(), 0);
        summary.insert(
            "PolicyVersionsInUse".to_string(),
            self.policies
                .values()
                .map(|p| p.versions.len() as i64)
                .sum(),
        );

        summary
    }

    pub fn generate_credential_report(&mut self) -> bool {
        // Mark report as generated; return true if newly generated, false if already existed.
        let was_generated = self.credential_report_generated;
        self.credential_report_generated = true;
        !was_generated
    }

    pub fn get_credential_report(&self) -> Result<Vec<u8>, IamError> {
        // Return a minimal CSV credential report
        let mut report = String::from(
            "user,arn,user_creation_time,password_enabled,password_last_used,password_last_changed,password_next_rotation,mfa_active,access_key_1_active,access_key_1_last_rotated,access_key_1_last_used_date,access_key_1_last_used_region,access_key_1_last_used_service,access_key_2_active,access_key_2_last_rotated,access_key_2_last_used_date,access_key_2_last_used_region,access_key_2_last_used_service,cert_1_active,cert_1_last_rotated,cert_2_active,cert_2_last_rotated\n",
        );
        report.push_str("<root_account>,arn:aws:iam::123456789012:root,2024-01-01T00:00:00+00:00,not_supported,not_supported,not_supported,not_supported,false,false,N/A,N/A,N/A,N/A,false,N/A,N/A,N/A,N/A,false,N/A,false,N/A\n");
        Ok(report.into_bytes())
    }

    /// Create a service-last-accessed details job for the given ARN. Returns the job ID.
    pub fn create_service_last_accessed_job(&mut self, arn: &str) -> String {
        let job_id = uuid::Uuid::new_v4().to_string();
        self.service_last_accessed_jobs
            .insert(job_id.clone(), arn.to_string());
        job_id
    }

    /// Look up a service-last-accessed job by job_id. Returns the ARN if found.
    pub fn get_service_last_accessed_job(&self, job_id: &str) -> Option<&str> {
        self.service_last_accessed_jobs
            .get(job_id)
            .map(|s| s.as_str())
    }

    /// Enable Organizations root credentials management and return enabled features.
    pub fn enable_organizations_root_credentials_management(
        &mut self,
        org_id: &str,
    ) -> Vec<String> {
        let features = vec!["RootCredentialsManagement".to_string()];
        self.org_root_credentials_management_features = Some(features.clone());
        features
    }

    /// Disable Organizations root credentials management and return remaining enabled features.
    pub fn disable_organizations_root_credentials_management(&mut self) -> Vec<String> {
        self.org_root_credentials_management_features = None;
        vec![]
    }

    /// Enable Organizations root sessions and return enabled features.
    pub fn enable_organizations_root_sessions(&mut self, org_id: &str) -> Vec<String> {
        let features = vec!["RootSessions".to_string()];
        self.org_root_sessions_features = Some(features.clone());
        features
    }

    /// Disable Organizations root sessions and return remaining enabled features.
    pub fn disable_organizations_root_sessions(&mut self) -> Vec<String> {
        self.org_root_sessions_features = None;
        vec![]
    }

    /// Return enabled Organizations features (root credentials management + root sessions).
    pub fn list_organizations_root_enabled_features(&self) -> Vec<String> {
        let mut features = Vec::new();
        if self.org_root_credentials_management_features.is_some() {
            features.push("RootCredentialsManagement".to_string());
        }
        if self.org_root_sessions_features.is_some() {
            features.push("RootSessions".to_string());
        }
        features
    }

    pub fn get_account_authorization_details(
        &self,
    ) -> (
        &HashMap<String, User>,
        &HashMap<String, Role>,
        &HashMap<String, Group>,
        &HashMap<String, ManagedPolicy>,
    ) {
        (&self.users, &self.roles, &self.groups, &self.policies)
    }

    /// Check if a role exists (used by STS for cross-service validation).
    pub fn role_exists(&self, role_name: &str) -> bool {
        self.roles.contains_key(role_name)
    }

    // ==================== SAML provider tag operations ====================

    pub fn tag_saml_provider(&mut self, arn: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let provider = self
            .saml_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::SamlProviderNotFound(arn.to_string()))?;
        for new_tag in tags {
            if let Some(existing) = provider.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                provider.tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_saml_provider(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), IamError> {
        let provider = self
            .saml_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::SamlProviderNotFound(arn.to_string()))?;
        provider.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_saml_provider_tags(&self, arn: &str) -> Result<&[Tag], IamError> {
        let provider = self
            .saml_providers
            .get(arn)
            .ok_or_else(|| IamError::SamlProviderNotFound(arn.to_string()))?;
        Ok(&provider.tags)
    }

    // ==================== Server certificate tag operations ====================

    pub fn tag_server_certificate(&mut self, name: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let cert = self
            .server_certificates
            .get_mut(name)
            .ok_or_else(|| IamError::ServerCertificateNotFound(name.to_string()))?;
        for new_tag in tags {
            if let Some(existing) = cert.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                cert.tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_server_certificate(
        &mut self,
        name: &str,
        tag_keys: &[String],
    ) -> Result<(), IamError> {
        let cert = self
            .server_certificates
            .get_mut(name)
            .ok_or_else(|| IamError::ServerCertificateNotFound(name.to_string()))?;
        cert.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_server_certificate_tags(&self, name: &str) -> Result<&[Tag], IamError> {
        let cert = self
            .server_certificates
            .get(name)
            .ok_or_else(|| IamError::ServerCertificateNotFound(name.to_string()))?;
        Ok(&cert.tags)
    }

    pub fn update_server_certificate(
        &mut self,
        server_certificate_name: &str,
        new_path: Option<&str>,
        new_server_certificate_name: Option<&str>,
    ) -> Result<(), IamError> {
        let cert = self
            .server_certificates
            .get(server_certificate_name)
            .ok_or_else(|| {
                IamError::ServerCertificateNotFound(server_certificate_name.to_string())
            })?;

        let new_name = new_server_certificate_name
            .unwrap_or(server_certificate_name)
            .to_string();
        if new_name != server_certificate_name && self.server_certificates.contains_key(&new_name) {
            return Err(IamError::ServerCertificateAlreadyExists(new_name.clone()));
        }

        let mut cert = cert.clone();
        if let Some(p) = new_path {
            cert.path = p.to_string();
        }
        cert.server_certificate_name = new_name.clone();
        if self
            .server_certificates
            .remove(server_certificate_name)
            .is_none()
        {
            return Err(IamError::ServerCertificateNotFound(
                server_certificate_name.to_string(),
            ));
        }
        self.server_certificates.insert(new_name, cert);
        Ok(())
    }

    // ==================== SSH public key listing ====================

    pub fn list_ssh_public_keys(&self, user_name: Option<&str>) -> Vec<&SSHPublicKeyEntry> {
        let mut keys: Vec<&SSHPublicKeyEntry> = self
            .ssh_public_keys
            .values()
            .filter(|k| {
                if let Some(un) = user_name {
                    k.user_name == un
                } else {
                    true
                }
            })
            .collect();
        keys.sort_by(|a, b| a.ssh_public_key_id.cmp(&b.ssh_public_key_id));
        keys
    }

    // ==================== Instance profiles listing ====================

    pub fn list_instance_profiles(&self, path_prefix: Option<&str>) -> Vec<&InstanceProfile> {
        let prefix = path_prefix.unwrap_or("/");
        let mut ips: Vec<&InstanceProfile> = self
            .instance_profiles
            .values()
            .filter(|ip| ip.path.starts_with(prefix))
            .collect();
        ips.sort_by(|a, b| a.name.cmp(&b.name));
        ips
    }

    pub fn list_instance_profiles_for_role(&self, role_name: &str) -> Vec<&InstanceProfile> {
        let mut ips: Vec<&InstanceProfile> = self
            .instance_profiles
            .values()
            .filter(|ip| ip.roles.iter().any(|r| r == role_name))
            .collect();
        ips.sort_by(|a, b| a.name.cmp(&b.name));
        ips
    }

    // ==================== Groups for user ====================

    pub fn list_groups_for_user(&self, user_name: &str) -> Result<Vec<&Group>, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFoundAlt(user_name.to_string()));
        }
        let mut groups: Vec<&Group> = self
            .groups
            .values()
            .filter(|g| g.members.iter().any(|m| m == user_name))
            .collect();
        groups.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(groups)
    }

    // ==================== List entities for policy ====================

    pub fn list_entities_for_policy(
        &self,
        policy_arn: &str,
    ) -> Result<(Vec<&Group>, Vec<&Role>, Vec<&User>), IamError> {
        if !self.policies.contains_key(policy_arn) {
            return Err(IamError::PolicyNotFound(policy_arn.to_string()));
        }
        let groups: Vec<&Group> = self
            .groups
            .values()
            .filter(|g| {
                g.attached_policies
                    .iter()
                    .any(|p| p.policy_arn == policy_arn)
            })
            .collect();
        let roles: Vec<&Role> = self
            .roles
            .values()
            .filter(|r| {
                r.attached_policies
                    .iter()
                    .any(|p| p.policy_arn == policy_arn)
            })
            .collect();
        let users: Vec<&User> = self
            .users
            .values()
            .filter(|u| {
                u.attached_policies
                    .iter()
                    .any(|p| p.policy_arn == policy_arn)
            })
            .collect();
        Ok((groups, roles, users))
    }

    // ==================== User permissions boundary ====================

    pub fn put_user_permissions_boundary(
        &mut self,
        user_name: &str,
        permissions_boundary_arn: &str,
    ) -> Result<(), IamError> {
        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFoundAlt(user_name.to_string()))?;
        user.permissions_boundary = Some(permissions_boundary_arn.to_string());
        Ok(())
    }

    pub fn delete_user_permissions_boundary(&mut self, user_name: &str) -> Result<(), IamError> {
        let user = self
            .users
            .get_mut(user_name)
            .ok_or_else(|| IamError::UserNotFoundAlt(user_name.to_string()))?;
        user.permissions_boundary = None;
        Ok(())
    }

    // ==================== OIDC provider client ID operations ====================

    pub fn add_client_id_to_oidc_provider(
        &mut self,
        arn: &str,
        client_id: &str,
    ) -> Result<(), IamError> {
        let provider = self
            .oidc_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::OidcProviderNotFound(arn.to_string()))?;
        if !provider.client_id_list.contains(&client_id.to_string()) {
            provider.client_id_list.push(client_id.to_string());
        }
        Ok(())
    }

    pub fn remove_client_id_from_oidc_provider(
        &mut self,
        arn: &str,
        client_id: &str,
    ) -> Result<(), IamError> {
        let provider = self
            .oidc_providers
            .get_mut(arn)
            .ok_or_else(|| IamError::OidcProviderNotFound(arn.to_string()))?;
        provider.client_id_list.retain(|id| id != client_id);
        Ok(())
    }

    // ==================== MFA device tag operations ====================

    pub fn get_mfa_device(&self, serial_number: &str) -> Result<&VirtualMFADevice, IamError> {
        self.virtual_mfa_devices
            .get(serial_number)
            .ok_or_else(|| IamError::MfaDeviceNotFound(serial_number.to_string()))
    }

    pub fn tag_mfa_device(&mut self, serial_number: &str, tags: Vec<Tag>) -> Result<(), IamError> {
        let device = self
            .virtual_mfa_devices
            .get_mut(serial_number)
            .ok_or_else(|| IamError::MfaDeviceNotFound(serial_number.to_string()))?;
        for new_tag in tags {
            if let Some(existing) = device.tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                device.tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_mfa_device(
        &mut self,
        serial_number: &str,
        tag_keys: &[String],
    ) -> Result<(), IamError> {
        let device = self
            .virtual_mfa_devices
            .get_mut(serial_number)
            .ok_or_else(|| IamError::MfaDeviceNotFound(serial_number.to_string()))?;
        device.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_mfa_device_tags(&self, serial_number: &str) -> Result<&[Tag], IamError> {
        let device = self
            .virtual_mfa_devices
            .get(serial_number)
            .ok_or_else(|| IamError::MfaDeviceNotFound(serial_number.to_string()))?;
        Ok(&device.tags)
    }

    // ==================== Service-specific credential operations ====================

    pub fn create_service_specific_credential(
        &mut self,
        user_name: &str,
        service_name: &str,
    ) -> Result<&ServiceSpecificCredential, IamError> {
        if !self.users.contains_key(user_name) {
            return Err(IamError::UserNotFoundSsc(user_name.to_string()));
        }

        let id = generate_resource_id("APKA");
        let service_user_name = format!("{user_name}+{}-{}", service_name, &id[4..12]);
        let service_password = format!("{}{}", &id, random_alphanumeric(20));

        let cred = ServiceSpecificCredential {
            service_specific_credential_id: id.clone(),
            user_name: user_name.to_string(),
            service_name: service_name.to_string(),
            service_user_name,
            service_password,
            status: "Active".to_string(),
            create_date: chrono::Utc::now(),
        };

        self.service_specific_credentials.insert(id.clone(), cred);
        Ok(self.service_specific_credentials.get(&id).unwrap())
    }

    pub fn delete_service_specific_credential(
        &mut self,
        credential_id: &str,
        user_name: Option<&str>,
    ) -> Result<(), IamError> {
        let cred = self
            .service_specific_credentials
            .get(credential_id)
            .ok_or_else(|| {
                IamError::ServiceSpecificCredentialNotFound(credential_id.to_string())
            })?;
        if let Some(uname) = user_name {
            if cred.user_name != uname {
                return Err(IamError::ServiceSpecificCredentialWrongUser(
                    credential_id.to_string(),
                    uname.to_string(),
                ));
            }
        }
        self.service_specific_credentials.remove(credential_id);
        Ok(())
    }

    pub fn list_service_specific_credentials(
        &self,
        user_name: Option<&str>,
        service_name: Option<&str>,
    ) -> Vec<&ServiceSpecificCredential> {
        self.service_specific_credentials
            .values()
            .filter(|c| {
                user_name.is_none_or(|u| c.user_name == u)
                    && service_name.is_none_or(|s| c.service_name == s)
            })
            .collect()
    }

    pub fn reset_service_specific_credential(
        &mut self,
        credential_id: &str,
        user_name: Option<&str>,
    ) -> Result<&ServiceSpecificCredential, IamError> {
        let cred = self
            .service_specific_credentials
            .get(credential_id)
            .ok_or_else(|| {
                IamError::ServiceSpecificCredentialNotFound(credential_id.to_string())
            })?;
        if let Some(uname) = user_name {
            if cred.user_name != uname {
                return Err(IamError::ServiceSpecificCredentialWrongUser(
                    credential_id.to_string(),
                    uname.to_string(),
                ));
            }
        }
        let new_password = format!("{}{}", credential_id, random_alphanumeric(20));
        let cred = self
            .service_specific_credentials
            .get_mut(credential_id)
            .unwrap();
        cred.service_password = new_password;
        Ok(self
            .service_specific_credentials
            .get(credential_id)
            .unwrap())
    }

    pub fn update_service_specific_credential(
        &mut self,
        credential_id: &str,
        user_name: Option<&str>,
        status: &str,
    ) -> Result<(), IamError> {
        let cred = self
            .service_specific_credentials
            .get(credential_id)
            .ok_or_else(|| {
                IamError::ServiceSpecificCredentialNotFound(credential_id.to_string())
            })?;
        if let Some(uname) = user_name {
            if cred.user_name != uname {
                return Err(IamError::ServiceSpecificCredentialWrongUser(
                    credential_id.to_string(),
                    uname.to_string(),
                ));
            }
        }
        let cred = self
            .service_specific_credentials
            .get_mut(credential_id)
            .unwrap();
        cred.status = status.to_string();
        Ok(())
    }
}

fn generate_resource_id(prefix: &str) -> String {
    format!("{prefix}{}", random_alphanumeric(17))
}

fn generate_policy_id() -> String {
    format!("A{}", random_alphanumeric(20))
}

fn random_alphanumeric(len: usize) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut result = String::with_capacity(len);
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz";
    while result.len() < len {
        let uuid = uuid::Uuid::new_v4();
        let mut hasher = DefaultHasher::new();
        uuid.hash(&mut hasher);
        let hash = hasher.finish();
        for i in 0..8 {
            if result.len() >= len {
                break;
            }
            let idx = ((hash >> (i * 8)) & 0xFF) as usize % chars.len();
            result.push(chars[idx] as char);
        }
    }
    result
}

fn service_name_to_role_suffix(aws_service_name: &str) -> String {
    // Convert "elasticloadbalancing.amazonaws.com" -> "ElasticLoadBalancing"
    let name = aws_service_name
        .split('.')
        .next()
        .unwrap_or(aws_service_name);

    let mut result = String::new();
    let mut capitalize_next = true;
    for c in name.chars() {
        if c == '-' || c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}
