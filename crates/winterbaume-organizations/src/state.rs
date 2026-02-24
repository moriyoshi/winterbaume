use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::Utc;

use crate::types::*;

#[derive(Debug)]
pub struct OrganizationsState {
    pub organization: Option<Organization>,
    pub accounts: HashMap<String, Account>,
    pub ous: HashMap<String, OrganizationalUnit>,
    pub delegated_admins: Vec<DelegatedAdministrator>,
    pub root: Option<OrgRoot>,
    pub policies: HashMap<String, OrgPolicy>,
    pub policy_attachments: Vec<PolicyAttachment>,
    pub enabled_services: Vec<EnabledService>,
    pub tags: HashMap<String, Vec<OrgTag>>,
    pub handshakes: HashMap<String, Handshake>,
    pub resource_policy: Option<String>,
    pub responsibility_transfers: HashMap<String, ResponsibilityTransfer>,
    next_account_num: u64,
    next_policy_num: u64,
    next_handshake_num: u64,
    next_transfer_num: u64,
}

impl Default for OrganizationsState {
    fn default() -> Self {
        Self {
            organization: None,
            accounts: HashMap::new(),
            ous: HashMap::new(),
            delegated_admins: Vec::new(),
            root: None,
            policies: HashMap::new(),
            policy_attachments: Vec::new(),
            enabled_services: Vec::new(),
            tags: HashMap::new(),
            handshakes: HashMap::new(),
            resource_policy: None,
            responsibility_transfers: HashMap::new(),
            next_account_num: 1,
            next_policy_num: 1,
            next_handshake_num: 1,
            next_transfer_num: 1,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OrganizationsError {
    #[error("Your account is not a member of an organization.")]
    AwsOrganizationsNotInUse,

    #[error("The account is already a member of an organization.")]
    AlreadyInOrganization,

    #[error("To delete an organization you must first remove all member accounts.")]
    OrganizationNotEmpty,

    #[error("You specified an account that doesn't exist.")]
    AccountNotFound,

    #[error("The provided account is already closed.")]
    AccountAlreadyClosed,

    #[error("You cannot remove the master account from the organization.")]
    ConstraintViolationRemoveMaster,

    #[error(
        "You cannot register master account/yourself as delegated administrator for your organization."
    )]
    ConstraintViolationRegisterMaster,

    #[error("We can't find an account creation request with the ID {0}.")]
    CreateAccountStatusNotFound(String),

    #[error("You specified an unrecognized service principal.")]
    InvalidInputUnrecognizedServicePrincipal,

    #[error("You specified an invalid value for policy type.")]
    InvalidInputInvalidPolicyType,

    #[error("You specified an invalid child type.")]
    InvalidInputInvalidChildType,

    #[error("The provided account is already a delegated administrator for your organization.")]
    AccountAlreadyRegistered,

    #[error(
        "The provided account is not a registered delegated administrator for your organization."
    )]
    AccountNotRegistered,

    #[error("You specified an organizational unit that doesn't exist.")]
    OrganizationalUnitNotFound,

    #[error("The specified organizational unit is not empty.")]
    OrganizationalUnitNotEmpty,

    #[error("You specified a policy that doesn't exist.")]
    PolicyNotFound,

    #[error("The policy is attached to one or more entities. You must detach it before deleting.")]
    PolicyInUse,

    #[error("The selected policy is already attached to the specified target.")]
    DuplicatePolicyAttachment,

    #[error("The policy is not attached to the specified target.")]
    PolicyNotAttached,

    #[error("This operation can be performed only for enabled policy types.")]
    PolicyTypeNotEnabledForOperation,

    #[error("The specified policy type is not currently enabled.")]
    PolicyTypeNotEnabled,

    #[error("You specified a root that doesn't exist.")]
    RootNotFound,

    #[error("The specified policy type is already enabled.")]
    PolicyTypeAlreadyEnabled,

    #[error("You specified a parent that doesn't exist.")]
    ParentNotFound,

    #[error("You specified a target that doesn't exist.")]
    TargetNotFound,

    #[error("You specified a child that doesn't exist.")]
    ChildNotFound,

    #[error("The source parent is not the current parent of the account.")]
    SourceParentNotFound,

    #[error("This organization does not have a resource policy.")]
    ResourcePolicyNotFound,

    #[error("We can't find a handshake with that ID.")]
    HandshakeNotFound,

    #[error("You can't accept a handshake that is not in the OPEN state.")]
    InvalidHandshakeTransitionAccept,

    #[error("You can't cancel a handshake that is not in the OPEN state.")]
    InvalidHandshakeTransitionCancel,

    #[error("You can't decline a handshake that is not in the OPEN state.")]
    InvalidHandshakeTransitionDecline,

    #[error("The specified responsibility transfer does not exist.")]
    ResponsibilityTransferNotFound,
}

impl OrganizationsState {
    /// Returns the next account sequence number for snapshot purposes.
    pub fn next_account_num(&self) -> u64 {
        self.next_account_num
    }

    /// Returns the next policy sequence number for snapshot purposes.
    pub fn next_policy_num(&self) -> u64 {
        self.next_policy_num
    }

    /// Returns the next handshake sequence number for snapshot purposes.
    pub fn next_handshake_num(&self) -> u64 {
        self.next_handshake_num
    }

    /// Returns the next transfer sequence number for snapshot purposes.
    pub fn next_transfer_num(&self) -> u64 {
        self.next_transfer_num
    }

    /// Restores sequence counters from a snapshot.
    pub fn restore_counters(
        &mut self,
        next_account_num: u64,
        next_policy_num: u64,
        next_handshake_num: u64,
        next_transfer_num: u64,
    ) {
        self.next_account_num = next_account_num;
        self.next_policy_num = next_policy_num;
        self.next_handshake_num = next_handshake_num;
        self.next_transfer_num = next_transfer_num;
    }

    fn require_org(&self) -> Result<&Organization, OrganizationsError> {
        self.organization
            .as_ref()
            .ok_or(OrganizationsError::AwsOrganizationsNotInUse)
    }

    pub fn create_organization(
        &mut self,
        account_id: &str,
    ) -> Result<&Organization, OrganizationsError> {
        if self.organization.is_some() {
            return Err(OrganizationsError::AlreadyInOrganization);
        }

        let org_id = format!("o-{}", &uuid::Uuid::new_v4().to_string()[..10]);
        let arn = format!("arn:aws:organizations::{account_id}:organization/{org_id}");

        let org = Organization {
            id: org_id.clone(),
            arn,
            master_account_id: account_id.to_string(),
            master_account_email: format!("master@{account_id}.example.com"),
        };

        self.organization = Some(org);

        // Create root
        let root_id = format!("r-{}", &uuid::Uuid::new_v4().to_string()[..4]);
        let root_arn = format!("arn:aws:organizations::{account_id}:root/{org_id}/{root_id}");
        self.root = Some(OrgRoot {
            id: root_id.clone(),
            arn: root_arn,
            name: "Root".to_string(),
            policy_types: Vec::new(),
        });

        // Create a master account entry (moto does this implicitly)
        let master_arn =
            format!("arn:aws:organizations::{account_id}:account/{org_id}/{account_id}");
        let master_account = Account {
            id: account_id.to_string(),
            arn: master_arn,
            name: "master".to_string(),
            email: format!("master@{account_id}.example.com"),
            status: "ACTIVE".to_string(),
            joined_method: "INVITED".to_string(),
            joined_timestamp: Utc::now(),
            create_account_status_id: format!("car-{}", &uuid::Uuid::new_v4().to_string()[..8]),
            parent_id: root_id,
        };
        self.accounts.insert(account_id.to_string(), master_account);

        Ok(self.organization.as_ref().unwrap())
    }

    pub fn delete_organization(&mut self) -> Result<(), OrganizationsError> {
        self.require_org()?;

        // Cannot delete if there are member accounts (non-master)
        let master_id = self
            .organization
            .as_ref()
            .unwrap()
            .master_account_id
            .clone();
        let member_count = self
            .accounts
            .values()
            .filter(|a| a.id != master_id && a.status == "ACTIVE")
            .count();
        if member_count > 0 {
            return Err(OrganizationsError::OrganizationNotEmpty);
        }

        self.organization = None;
        self.root = None;
        self.accounts.clear();
        self.ous.clear();
        self.policies.clear();
        self.policy_attachments.clear();
        self.enabled_services.clear();
        self.tags.clear();
        self.delegated_admins.clear();
        Ok(())
    }

    pub fn describe_organization(&self) -> Result<&Organization, OrganizationsError> {
        self.require_org()
    }

    pub fn create_account(
        &mut self,
        name: &str,
        email: &str,
        account_id: &str,
    ) -> Result<&Account, OrganizationsError> {
        let org = match &self.organization {
            Some(o) => o.clone(),
            None => {
                return Err(OrganizationsError::AwsOrganizationsNotInUse);
            }
        };

        let root_id = self.root.as_ref().unwrap().id.clone();

        let mut hasher = DefaultHasher::new();
        self.next_account_num.hash(&mut hasher);
        name.hash(&mut hasher);
        email.hash(&mut hasher);
        let hash = hasher.finish();
        // Map to 12-digit range [100000000000, 999999999999]
        let new_account_id = format!("{:012}", 100000000000u64 + (hash % 900000000000));
        self.next_account_num += 1;
        let arn = format!(
            "arn:aws:organizations::{account_id}:account/{}/{}",
            org.id, new_account_id
        );

        let create_account_status_id = format!("car-{}", &uuid::Uuid::new_v4().to_string()[..8]);

        let account = Account {
            id: new_account_id.clone(),
            arn,
            name: name.to_string(),
            email: email.to_string(),
            status: "ACTIVE".to_string(),
            joined_method: "CREATED".to_string(),
            joined_timestamp: Utc::now(),
            create_account_status_id,
            parent_id: root_id,
        };

        self.accounts.insert(new_account_id.clone(), account);
        Ok(self.accounts.get(&new_account_id).unwrap())
    }

    pub fn describe_account(&self, account_id: &str) -> Result<&Account, OrganizationsError> {
        self.accounts
            .get(account_id)
            .ok_or(OrganizationsError::AccountNotFound)
    }

    pub fn list_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }

    pub fn close_account(&mut self, account_id: &str) -> Result<(), OrganizationsError> {
        if self.organization.is_none() {
            return Err(OrganizationsError::AwsOrganizationsNotInUse);
        }

        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(OrganizationsError::AccountNotFound)?;

        if account.status == "SUSPENDED" {
            return Err(OrganizationsError::AccountAlreadyClosed);
        }

        account.status = "SUSPENDED".to_string();
        Ok(())
    }

    pub fn remove_account_from_organization(
        &mut self,
        account_id: &str,
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;
        let master_id = self
            .organization
            .as_ref()
            .unwrap()
            .master_account_id
            .clone();
        if account_id == master_id {
            return Err(OrganizationsError::ConstraintViolationRemoveMaster);
        }

        if !self.accounts.contains_key(account_id) {
            return Err(OrganizationsError::AccountNotFound);
        }

        self.accounts.remove(account_id);
        // Remove any policy attachments for this account
        self.policy_attachments
            .retain(|a| a.target_id != account_id);
        // Remove tags
        self.tags.remove(account_id);
        Ok(())
    }

    pub fn describe_create_account_status(
        &self,
        request_id: &str,
    ) -> Result<&Account, OrganizationsError> {
        self.accounts
            .values()
            .find(|a| a.create_account_status_id == request_id)
            .ok_or_else(|| OrganizationsError::CreateAccountStatusNotFound(request_id.to_string()))
    }

    pub fn list_create_account_status(&self, states: Option<&[String]>) -> Vec<&Account> {
        let default_states = vec![
            "IN_PROGRESS".to_string(),
            "SUCCEEDED".to_string(),
            "FAILED".to_string(),
        ];
        let filter_states = states.unwrap_or(&default_states);
        // All accounts in our mock are SUCCEEDED
        self.accounts
            .values()
            .filter(|_| filter_states.iter().any(|s| s == "SUCCEEDED"))
            .collect()
    }

    pub fn register_delegated_administrator(
        &mut self,
        account_id: &str,
        service_principal: &str,
        master_account_id: &str,
    ) -> Result<(), OrganizationsError> {
        // Cannot register the master account
        if account_id == master_account_id {
            return Err(OrganizationsError::ConstraintViolationRegisterMaster);
        }

        // Account must exist
        let account = self
            .accounts
            .get(account_id)
            .ok_or(OrganizationsError::AccountNotFound)?
            .clone();

        // Validate service principal
        if !SUPPORTED_DELEGATED_ADMIN_SERVICES.contains(&service_principal) {
            return Err(OrganizationsError::InvalidInputUnrecognizedServicePrincipal);
        }

        // Find or create the admin entry
        let admin = self
            .delegated_admins
            .iter_mut()
            .find(|a| a.account.id == account_id);

        match admin {
            Some(admin) => {
                // Check if already registered for this service
                if admin
                    .services
                    .iter()
                    .any(|s| s.service_principal == service_principal)
                {
                    return Err(OrganizationsError::AccountAlreadyRegistered);
                }
                admin.services.push(DelegatedService {
                    service_principal: service_principal.to_string(),
                    delegation_enabled_date: Utc::now(),
                });
            }
            None => {
                let now = Utc::now();
                self.delegated_admins.push(DelegatedAdministrator {
                    account: account.clone(),
                    delegation_enabled_date: now,
                    services: vec![DelegatedService {
                        service_principal: service_principal.to_string(),
                        delegation_enabled_date: now,
                    }],
                });
            }
        }

        Ok(())
    }

    pub fn list_delegated_administrators(
        &self,
        service_principal: Option<&str>,
    ) -> Result<Vec<&DelegatedAdministrator>, OrganizationsError> {
        if let Some(sp) = service_principal {
            if !SUPPORTED_DELEGATED_ADMIN_SERVICES.contains(&sp) {
                return Err(OrganizationsError::InvalidInputUnrecognizedServicePrincipal);
            }
            Ok(self
                .delegated_admins
                .iter()
                .filter(|a| a.services.iter().any(|s| s.service_principal == sp))
                .collect())
        } else {
            Ok(self.delegated_admins.iter().collect())
        }
    }

    pub fn list_delegated_services_for_account(
        &self,
        account_id: &str,
    ) -> Result<Vec<&DelegatedService>, OrganizationsError> {
        let admin = self
            .delegated_admins
            .iter()
            .find(|a| a.account.id == account_id);

        match admin {
            Some(admin) => Ok(admin.services.iter().collect()),
            None => {
                // Check if the account exists at all
                if self.accounts.contains_key(account_id) {
                    Err(OrganizationsError::AccountNotRegistered)
                } else {
                    Err(OrganizationsError::AwsOrganizationsNotInUse)
                }
            }
        }
    }

    pub fn deregister_delegated_administrator(
        &mut self,
        account_id: &str,
        service_principal: &str,
        master_account_id: &str,
    ) -> Result<(), OrganizationsError> {
        if account_id == master_account_id {
            return Err(OrganizationsError::ConstraintViolationRegisterMaster);
        }

        let admin_idx = self
            .delegated_admins
            .iter()
            .position(|a| a.account.id == account_id);

        match admin_idx {
            Some(idx) => {
                let admin = &mut self.delegated_admins[idx];
                let svc_idx = admin
                    .services
                    .iter()
                    .position(|s| s.service_principal == service_principal);
                match svc_idx {
                    Some(si) => {
                        admin.services.remove(si);
                        if admin.services.is_empty() {
                            self.delegated_admins.remove(idx);
                        }
                        Ok(())
                    }
                    None => Err(OrganizationsError::InvalidInputUnrecognizedServicePrincipal),
                }
            }
            None => {
                // Check if account exists
                if self.accounts.contains_key(account_id) {
                    Err(OrganizationsError::AccountNotRegistered)
                } else {
                    Err(OrganizationsError::AccountNotFound)
                }
            }
        }
    }

    // ==================== Organizational Unit operations ====================

    pub fn create_organizational_unit(
        &mut self,
        parent_id: &str,
        name: &str,
        account_id: &str,
    ) -> Result<&OrganizationalUnit, OrganizationsError> {
        self.require_org()?;
        self.validate_parent(parent_id)?;

        let ou_id = format!("ou-{}", &uuid::Uuid::new_v4().to_string()[..12]);
        let org_id = &self.organization.as_ref().unwrap().id;
        let arn = format!("arn:aws:organizations::{account_id}:ou/{org_id}/{ou_id}");

        let ou = OrganizationalUnit {
            id: ou_id.clone(),
            arn,
            name: name.to_string(),
            parent_id: parent_id.to_string(),
        };

        self.ous.insert(ou_id.clone(), ou);
        Ok(self.ous.get(&ou_id).unwrap())
    }

    pub fn describe_organizational_unit(
        &self,
        ou_id: &str,
    ) -> Result<&OrganizationalUnit, OrganizationsError> {
        self.require_org()?;
        self.ous
            .get(ou_id)
            .ok_or(OrganizationsError::OrganizationalUnitNotFound)
    }

    pub fn update_organizational_unit(
        &mut self,
        ou_id: &str,
        name: Option<&str>,
    ) -> Result<&OrganizationalUnit, OrganizationsError> {
        self.require_org()?;
        let ou = self
            .ous
            .get_mut(ou_id)
            .ok_or(OrganizationsError::OrganizationalUnitNotFound)?;

        if let Some(n) = name {
            ou.name = n.to_string();
        }

        Ok(self.ous.get(ou_id).unwrap())
    }

    pub fn delete_organizational_unit(&mut self, ou_id: &str) -> Result<(), OrganizationsError> {
        self.require_org()?;
        if !self.ous.contains_key(ou_id) {
            return Err(OrganizationsError::OrganizationalUnitNotFound);
        }

        // Check if OU has children (accounts or sub-OUs)
        let has_child_accounts = self.accounts.values().any(|a| a.parent_id == ou_id);
        let has_child_ous = self.ous.values().any(|o| o.parent_id == ou_id);
        if has_child_accounts || has_child_ous {
            return Err(OrganizationsError::OrganizationalUnitNotEmpty);
        }

        self.ous.remove(ou_id);
        self.policy_attachments.retain(|a| a.target_id != ou_id);
        self.tags.remove(ou_id);
        Ok(())
    }

    pub fn list_organizational_units_for_parent(
        &self,
        parent_id: &str,
    ) -> Result<Vec<&OrganizationalUnit>, OrganizationsError> {
        self.require_org()?;
        self.validate_parent(parent_id)?;

        Ok(self
            .ous
            .values()
            .filter(|ou| ou.parent_id == parent_id)
            .collect())
    }

    // ==================== Root operations ====================

    pub fn list_roots(&self) -> Result<Vec<&OrgRoot>, OrganizationsError> {
        self.require_org()?;
        match &self.root {
            Some(r) => Ok(vec![r]),
            None => Ok(vec![]),
        }
    }

    // ==================== Policy operations ====================

    pub fn create_policy(
        &mut self,
        name: &str,
        description: &str,
        content: &str,
        policy_type: &str,
        account_id: &str,
    ) -> Result<&OrgPolicy, OrganizationsError> {
        self.require_org()?;

        if !VALID_POLICY_TYPES.contains(&policy_type) {
            return Err(OrganizationsError::InvalidInputInvalidPolicyType);
        }

        // Check if this policy type is enabled on the root
        if policy_type != "SERVICE_CONTROL_POLICY" {
            let root = self.root.as_ref().unwrap();
            let enabled = root
                .policy_types
                .iter()
                .any(|pt| pt.policy_type == policy_type && pt.status == "ENABLED");
            if !enabled {
                return Err(OrganizationsError::PolicyTypeNotEnabledForOperation);
            }
        }

        let policy_id = format!("p-{:08x}", self.next_policy_num);
        self.next_policy_num += 1;
        let org_id = &self.organization.as_ref().unwrap().id;
        let arn = format!(
            "arn:aws:organizations::{account_id}:policy/{org_id}/{policy_type}/{policy_id}"
        );

        let policy = OrgPolicy {
            id: policy_id.clone(),
            arn,
            name: name.to_string(),
            description: description.to_string(),
            policy_type: policy_type.to_string(),
            content: content.to_string(),
            aws_managed: false,
        };

        self.policies.insert(policy_id.clone(), policy);
        Ok(self.policies.get(&policy_id).unwrap())
    }

    pub fn describe_policy(&self, policy_id: &str) -> Result<&OrgPolicy, OrganizationsError> {
        self.require_org()?;
        self.policies
            .get(policy_id)
            .ok_or(OrganizationsError::PolicyNotFound)
    }

    pub fn update_policy(
        &mut self,
        policy_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        content: Option<&str>,
    ) -> Result<&OrgPolicy, OrganizationsError> {
        self.require_org()?;
        let policy = self
            .policies
            .get_mut(policy_id)
            .ok_or(OrganizationsError::PolicyNotFound)?;

        if let Some(n) = name {
            policy.name = n.to_string();
        }
        if let Some(d) = description {
            policy.description = d.to_string();
        }
        if let Some(c) = content {
            policy.content = c.to_string();
        }

        Ok(self.policies.get(policy_id).unwrap())
    }

    pub fn delete_policy(&mut self, policy_id: &str) -> Result<(), OrganizationsError> {
        self.require_org()?;
        if !self.policies.contains_key(policy_id) {
            return Err(OrganizationsError::PolicyNotFound);
        }

        // Cannot delete if still attached
        let attached = self
            .policy_attachments
            .iter()
            .any(|a| a.policy_id == policy_id);
        if attached {
            return Err(OrganizationsError::PolicyInUse);
        }

        self.policies.remove(policy_id);
        self.tags.remove(policy_id);
        Ok(())
    }

    pub fn list_policies(&self, filter: &str) -> Result<Vec<&OrgPolicy>, OrganizationsError> {
        self.require_org()?;
        Ok(self
            .policies
            .values()
            .filter(|p| p.policy_type == filter)
            .collect())
    }

    pub fn attach_policy(
        &mut self,
        policy_id: &str,
        target_id: &str,
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;

        if !self.policies.contains_key(policy_id) {
            return Err(OrganizationsError::PolicyNotFound);
        }

        // Validate target exists (root, OU, or account)
        self.validate_target(target_id)?;

        // Check for duplicate
        let already = self
            .policy_attachments
            .iter()
            .any(|a| a.policy_id == policy_id && a.target_id == target_id);
        if already {
            return Err(OrganizationsError::DuplicatePolicyAttachment);
        }

        self.policy_attachments.push(PolicyAttachment {
            policy_id: policy_id.to_string(),
            target_id: target_id.to_string(),
        });
        Ok(())
    }

    pub fn detach_policy(
        &mut self,
        policy_id: &str,
        target_id: &str,
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;

        if !self.policies.contains_key(policy_id) {
            return Err(OrganizationsError::PolicyNotFound);
        }

        let idx = self
            .policy_attachments
            .iter()
            .position(|a| a.policy_id == policy_id && a.target_id == target_id);

        match idx {
            Some(i) => {
                self.policy_attachments.remove(i);
                Ok(())
            }
            None => Err(OrganizationsError::PolicyNotAttached),
        }
    }

    pub fn list_policies_for_target(
        &self,
        target_id: &str,
        filter: &str,
    ) -> Result<Vec<&OrgPolicy>, OrganizationsError> {
        self.require_org()?;
        self.validate_target(target_id)?;

        let policy_ids: Vec<String> = self
            .policy_attachments
            .iter()
            .filter(|a| a.target_id == target_id)
            .map(|a| a.policy_id.clone())
            .collect();

        Ok(self
            .policies
            .values()
            .filter(|p| policy_ids.contains(&p.id) && p.policy_type == filter)
            .collect())
    }

    pub fn list_targets_for_policy(
        &self,
        policy_id: &str,
    ) -> Result<Vec<PolicyTargetInfo>, OrganizationsError> {
        self.require_org()?;

        if !self.policies.contains_key(policy_id) {
            return Err(OrganizationsError::PolicyNotFound);
        }

        let targets: Vec<PolicyTargetInfo> = self
            .policy_attachments
            .iter()
            .filter(|a| a.policy_id == policy_id)
            .filter_map(|a| self.resolve_target_info(&a.target_id))
            .collect();

        Ok(targets)
    }

    // ==================== Policy type operations ====================

    pub fn enable_policy_type(
        &mut self,
        root_id: &str,
        policy_type: &str,
    ) -> Result<&OrgRoot, OrganizationsError> {
        self.require_org()?;

        let root = self.root.as_mut().ok_or(OrganizationsError::RootNotFound)?;

        if root.id != root_id {
            return Err(OrganizationsError::RootNotFound);
        }

        if !VALID_POLICY_TYPES.contains(&policy_type) {
            return Err(OrganizationsError::InvalidInputInvalidPolicyType);
        }

        let already = root
            .policy_types
            .iter()
            .any(|pt| pt.policy_type == policy_type && pt.status == "ENABLED");
        if already {
            return Err(OrganizationsError::PolicyTypeAlreadyEnabled);
        }

        root.policy_types.push(PolicyTypeStatus {
            policy_type: policy_type.to_string(),
            status: "ENABLED".to_string(),
        });

        Ok(self.root.as_ref().unwrap())
    }

    pub fn disable_policy_type(
        &mut self,
        root_id: &str,
        policy_type: &str,
    ) -> Result<&OrgRoot, OrganizationsError> {
        self.require_org()?;

        let root = self.root.as_mut().ok_or(OrganizationsError::RootNotFound)?;

        if root.id != root_id {
            return Err(OrganizationsError::RootNotFound);
        }

        let idx = root
            .policy_types
            .iter()
            .position(|pt| pt.policy_type == policy_type && pt.status == "ENABLED");

        match idx {
            Some(i) => {
                root.policy_types.remove(i);
                Ok(self.root.as_ref().unwrap())
            }
            None => Err(OrganizationsError::PolicyTypeNotEnabled),
        }
    }

    // ==================== Service access operations ====================

    pub fn enable_aws_service_access(
        &mut self,
        service_principal: &str,
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;

        let already = self
            .enabled_services
            .iter()
            .any(|s| s.service_principal == service_principal);
        if !already {
            self.enabled_services.push(EnabledService {
                service_principal: service_principal.to_string(),
                date_enabled: Utc::now(),
            });
        }
        Ok(())
    }

    pub fn disable_aws_service_access(
        &mut self,
        service_principal: &str,
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;

        self.enabled_services
            .retain(|s| s.service_principal != service_principal);
        Ok(())
    }

    pub fn list_aws_service_access_for_organization(
        &self,
    ) -> Result<Vec<&EnabledService>, OrganizationsError> {
        self.require_org()?;
        Ok(self.enabled_services.iter().collect())
    }

    // ==================== Account parent/child operations ====================

    pub fn list_accounts_for_parent(
        &self,
        parent_id: &str,
    ) -> Result<Vec<&Account>, OrganizationsError> {
        self.require_org()?;
        self.validate_parent(parent_id)?;

        Ok(self
            .accounts
            .values()
            .filter(|a| a.parent_id == parent_id)
            .collect())
    }

    pub fn list_children(
        &self,
        parent_id: &str,
        child_type: &str,
    ) -> Result<Vec<ChildInfo>, OrganizationsError> {
        self.require_org()?;
        self.validate_parent(parent_id)?;

        let mut children = Vec::new();
        match child_type {
            "ACCOUNT" => {
                for account in self.accounts.values() {
                    if account.parent_id == parent_id {
                        children.push(ChildInfo {
                            id: account.id.clone(),
                            child_type: "ACCOUNT".to_string(),
                        });
                    }
                }
            }
            "ORGANIZATIONAL_UNIT" => {
                for ou in self.ous.values() {
                    if ou.parent_id == parent_id {
                        children.push(ChildInfo {
                            id: ou.id.clone(),
                            child_type: "ORGANIZATIONAL_UNIT".to_string(),
                        });
                    }
                }
            }
            _ => {
                return Err(OrganizationsError::InvalidInputInvalidChildType);
            }
        }
        Ok(children)
    }

    pub fn list_parents(&self, child_id: &str) -> Result<Vec<ParentInfo>, OrganizationsError> {
        self.require_org()?;

        // Check if child is an account
        if let Some(account) = self.accounts.get(child_id) {
            let parent_type = if self
                .root
                .as_ref()
                .is_some_and(|r| r.id == account.parent_id)
            {
                "ROOT"
            } else {
                "ORGANIZATIONAL_UNIT"
            };
            return Ok(vec![ParentInfo {
                id: account.parent_id.clone(),
                parent_type: parent_type.to_string(),
            }]);
        }

        // Check if child is an OU
        if let Some(ou) = self.ous.get(child_id) {
            let parent_type = if self.root.as_ref().is_some_and(|r| r.id == ou.parent_id) {
                "ROOT"
            } else {
                "ORGANIZATIONAL_UNIT"
            };
            return Ok(vec![ParentInfo {
                id: ou.parent_id.clone(),
                parent_type: parent_type.to_string(),
            }]);
        }

        Err(OrganizationsError::ChildNotFound)
    }

    pub fn move_account(
        &mut self,
        account_id: &str,
        source_parent_id: &str,
        destination_parent_id: &str,
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;
        self.validate_parent(source_parent_id)?;
        self.validate_parent(destination_parent_id)?;

        let account = self
            .accounts
            .get(account_id)
            .ok_or(OrganizationsError::AccountNotFound)?;

        if account.parent_id != source_parent_id {
            return Err(OrganizationsError::SourceParentNotFound);
        }

        let account = self.accounts.get_mut(account_id).unwrap();
        account.parent_id = destination_parent_id.to_string();
        Ok(())
    }

    // ==================== Tag operations ====================

    pub fn tag_resource(
        &mut self,
        resource_id: &str,
        tags: Vec<OrgTag>,
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;

        let entry = self.tags.entry(resource_id.to_string()).or_default();
        for tag in tags {
            // Update existing or add new
            if let Some(existing) = entry.iter_mut().find(|t| t.key == tag.key) {
                existing.value = tag.value;
            } else {
                entry.push(tag);
            }
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_id: &str,
        tag_keys: &[String],
    ) -> Result<(), OrganizationsError> {
        self.require_org()?;

        if let Some(tags) = self.tags.get_mut(resource_id) {
            tags.retain(|t| !tag_keys.contains(&t.key));
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_id: &str,
    ) -> Result<Vec<&OrgTag>, OrganizationsError> {
        self.require_org()?;

        Ok(self
            .tags
            .get(resource_id)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default())
    }

    // ==================== Helper methods ====================

    fn validate_parent(&self, parent_id: &str) -> Result<(), OrganizationsError> {
        let is_root = self.root.as_ref().is_some_and(|r| r.id == parent_id);
        let is_ou = self.ous.contains_key(parent_id);

        if !is_root && !is_ou {
            return Err(OrganizationsError::ParentNotFound);
        }
        Ok(())
    }

    fn validate_target(&self, target_id: &str) -> Result<(), OrganizationsError> {
        let is_root = self.root.as_ref().is_some_and(|r| r.id == target_id);
        let is_ou = self.ous.contains_key(target_id);
        let is_account = self.accounts.contains_key(target_id);

        if !is_root && !is_ou && !is_account {
            return Err(OrganizationsError::TargetNotFound);
        }
        Ok(())
    }

    fn resolve_target_info(&self, target_id: &str) -> Option<PolicyTargetInfo> {
        if let Some(root) = &self.root
            && root.id == target_id
        {
            return Some(PolicyTargetInfo {
                target_id: root.id.clone(),
                arn: root.arn.clone(),
                name: root.name.clone(),
                target_type: "ROOT".to_string(),
            });
        }
        if let Some(ou) = self.ous.get(target_id) {
            return Some(PolicyTargetInfo {
                target_id: ou.id.clone(),
                arn: ou.arn.clone(),
                name: ou.name.clone(),
                target_type: "ORGANIZATIONAL_UNIT".to_string(),
            });
        }
        if let Some(account) = self.accounts.get(target_id) {
            return Some(PolicyTargetInfo {
                target_id: account.id.clone(),
                arn: account.arn.clone(),
                name: account.name.clone(),
                target_type: "ACCOUNT".to_string(),
            });
        }
        None
    }

    // ==================== Handshake operations ====================

    fn make_handshake_id(&mut self) -> String {
        let id = format!("h-{:08x}", self.next_handshake_num);
        self.next_handshake_num += 1;
        id
    }

    pub fn invite_account_to_organization(
        &mut self,
        target_account_id: &str,
        master_account_id: &str,
        org_id: &str,
    ) -> Result<&Handshake, OrganizationsError> {
        self.require_org()?;
        let id = self.make_handshake_id();
        let arn =
            format!("arn:aws:organizations::{master_account_id}:handshake/{org_id}/invite/{id}");
        let now = Utc::now();
        let expiration = now + chrono::Duration::days(15);
        let handshake = Handshake {
            id: id.clone(),
            arn,
            state: "OPEN".to_string(),
            action: "INVITE".to_string(),
            parties: vec![
                HandshakeParty {
                    id: master_account_id.to_string(),
                    party_type: "ORGANIZATION".to_string(),
                },
                HandshakeParty {
                    id: target_account_id.to_string(),
                    party_type: "ACCOUNT".to_string(),
                },
            ],
            expiration_timestamp: expiration,
            requested_timestamp: now,
            resources: vec![
                HandshakeResource {
                    resource_type: "ORGANIZATION".to_string(),
                    value: org_id.to_string(),
                },
                HandshakeResource {
                    resource_type: "ACCOUNT".to_string(),
                    value: target_account_id.to_string(),
                },
            ],
        };
        self.handshakes.insert(id.clone(), handshake);
        Ok(self.handshakes.get(&id).unwrap())
    }

    pub fn accept_handshake(
        &mut self,
        handshake_id: &str,
    ) -> Result<&Handshake, OrganizationsError> {
        let hs = self
            .handshakes
            .get_mut(handshake_id)
            .ok_or(OrganizationsError::HandshakeNotFound)?;
        if hs.state != "OPEN" {
            return Err(OrganizationsError::InvalidHandshakeTransitionAccept);
        }
        hs.state = "ACCEPTED".to_string();
        Ok(self.handshakes.get(handshake_id).unwrap())
    }

    pub fn cancel_handshake(
        &mut self,
        handshake_id: &str,
    ) -> Result<&Handshake, OrganizationsError> {
        let hs = self
            .handshakes
            .get_mut(handshake_id)
            .ok_or(OrganizationsError::HandshakeNotFound)?;
        if hs.state != "OPEN" {
            return Err(OrganizationsError::InvalidHandshakeTransitionCancel);
        }
        hs.state = "CANCELED".to_string();
        Ok(self.handshakes.get(handshake_id).unwrap())
    }

    pub fn decline_handshake(
        &mut self,
        handshake_id: &str,
    ) -> Result<&Handshake, OrganizationsError> {
        let hs = self
            .handshakes
            .get_mut(handshake_id)
            .ok_or(OrganizationsError::HandshakeNotFound)?;
        if hs.state != "OPEN" {
            return Err(OrganizationsError::InvalidHandshakeTransitionDecline);
        }
        hs.state = "DECLINED".to_string();
        Ok(self.handshakes.get(handshake_id).unwrap())
    }

    pub fn describe_handshake(&self, handshake_id: &str) -> Result<&Handshake, OrganizationsError> {
        self.handshakes
            .get(handshake_id)
            .ok_or(OrganizationsError::HandshakeNotFound)
    }

    pub fn list_handshakes_for_account(&self) -> Vec<&Handshake> {
        self.handshakes.values().collect()
    }

    pub fn list_handshakes_for_organization(&self) -> Vec<&Handshake> {
        self.handshakes.values().collect()
    }

    pub fn enable_all_features(
        &mut self,
        master_account_id: &str,
    ) -> Result<&Handshake, OrganizationsError> {
        self.require_org()?;
        let org_id = self.organization.as_ref().unwrap().id.clone();
        let id = self.make_handshake_id();
        let arn = format!(
            "arn:aws:organizations::{master_account_id}:handshake/{org_id}/enable_all_features/{id}"
        );
        let now = Utc::now();
        let expiration = now + chrono::Duration::days(15);
        let handshake = Handshake {
            id: id.clone(),
            arn,
            state: "OPEN".to_string(),
            action: "ENABLE_ALL_FEATURES".to_string(),
            parties: vec![HandshakeParty {
                id: master_account_id.to_string(),
                party_type: "ORGANIZATION".to_string(),
            }],
            expiration_timestamp: expiration,
            requested_timestamp: now,
            resources: vec![HandshakeResource {
                resource_type: "ORGANIZATION".to_string(),
                value: org_id,
            }],
        };
        self.handshakes.insert(id.clone(), handshake);
        Ok(self.handshakes.get(&id).unwrap())
    }

    // ==================== GovCloud account ====================

    pub fn create_gov_cloud_account(
        &mut self,
        name: &str,
        email: &str,
        account_id: &str,
    ) -> Result<&Account, OrganizationsError> {
        // Reuse create_account — GovCloud is just a variant that also creates a gov account
        self.create_account(name, email, account_id)
    }

    // ==================== Resource policy ====================

    pub fn put_resource_policy(&mut self, content: &str) -> Result<(), OrganizationsError> {
        self.require_org()?;
        self.resource_policy = Some(content.to_string());
        Ok(())
    }

    pub fn describe_resource_policy(&self) -> Result<&String, OrganizationsError> {
        self.require_org()?;
        self.resource_policy
            .as_ref()
            .ok_or(OrganizationsError::ResourcePolicyNotFound)
    }

    pub fn delete_resource_policy(&mut self) -> Result<(), OrganizationsError> {
        self.require_org()?;
        self.resource_policy = None;
        Ok(())
    }

    // ==================== Responsibility transfer operations ====================

    fn make_transfer_id(&mut self) -> String {
        let id = format!("rt-{:08x}", self.next_transfer_num);
        self.next_transfer_num += 1;
        id
    }

    pub fn invite_organization_to_transfer_responsibility(
        &mut self,
        target_account_id: &str,
        transfer_type: &str,
        name: &str,
        master_account_id: &str,
        org_id: &str,
    ) -> Result<&Handshake, OrganizationsError> {
        self.require_org()?;

        let transfer_id = self.make_transfer_id();
        let transfer_arn =
            format!("arn:aws:organizations::{master_account_id}:transfer/{org_id}/{transfer_id}");
        let now = Utc::now();

        // Create a corresponding handshake
        let handshake_id = self.make_handshake_id();
        let handshake_arn = format!(
            "arn:aws:organizations::{master_account_id}:handshake/{org_id}/transfer_responsibility/{handshake_id}"
        );
        let expiration = now + chrono::Duration::days(15);

        let transfer = ResponsibilityTransfer {
            id: transfer_id.clone(),
            arn: transfer_arn,
            status: "OPEN".to_string(),
            name: name.to_string(),
            transfer_type: transfer_type.to_string(),
            source_account_id: master_account_id.to_string(),
            target_account_id: target_account_id.to_string(),
            start_timestamp: now,
            active_handshake_id: Some(handshake_id.clone()),
        };
        self.responsibility_transfers
            .insert(transfer_id.clone(), transfer);

        let handshake = Handshake {
            id: handshake_id.clone(),
            arn: handshake_arn,
            state: "OPEN".to_string(),
            action: "TRANSFER_RESPONSIBILITY".to_string(),
            parties: vec![
                HandshakeParty {
                    id: master_account_id.to_string(),
                    party_type: "ORGANIZATION".to_string(),
                },
                HandshakeParty {
                    id: target_account_id.to_string(),
                    party_type: "ACCOUNT".to_string(),
                },
            ],
            expiration_timestamp: expiration,
            requested_timestamp: now,
            resources: vec![HandshakeResource {
                resource_type: "ORGANIZATION".to_string(),
                value: org_id.to_string(),
            }],
        };
        self.handshakes.insert(handshake_id.clone(), handshake);
        Ok(self.handshakes.get(&handshake_id).unwrap())
    }

    pub fn describe_responsibility_transfer(
        &self,
        transfer_id: &str,
    ) -> Result<&ResponsibilityTransfer, OrganizationsError> {
        self.responsibility_transfers
            .get(transfer_id)
            .ok_or(OrganizationsError::ResponsibilityTransferNotFound)
    }

    pub fn list_inbound_responsibility_transfers(
        &self,
        account_id: &str,
    ) -> Vec<&ResponsibilityTransfer> {
        self.responsibility_transfers
            .values()
            .filter(|t| t.target_account_id == account_id)
            .collect()
    }

    pub fn list_outbound_responsibility_transfers(
        &self,
        account_id: &str,
    ) -> Vec<&ResponsibilityTransfer> {
        self.responsibility_transfers
            .values()
            .filter(|t| t.source_account_id == account_id)
            .collect()
    }

    pub fn terminate_responsibility_transfer(
        &mut self,
        transfer_id: &str,
    ) -> Result<&ResponsibilityTransfer, OrganizationsError> {
        let transfer = self
            .responsibility_transfers
            .get_mut(transfer_id)
            .ok_or(OrganizationsError::ResponsibilityTransferNotFound)?;
        transfer.status = "CANCELED".to_string();
        // Cancel associated handshake
        let hs_id = transfer.active_handshake_id.clone();
        if let Some(hid) = hs_id {
            if let Some(hs) = self.handshakes.get_mut(&hid) {
                hs.state = "CANCELED".to_string();
            }
        }
        Ok(self.responsibility_transfers.get(transfer_id).unwrap())
    }

    pub fn update_responsibility_transfer(
        &mut self,
        transfer_id: &str,
        name: Option<&str>,
    ) -> Result<&ResponsibilityTransfer, OrganizationsError> {
        let transfer = self
            .responsibility_transfers
            .get_mut(transfer_id)
            .ok_or(OrganizationsError::ResponsibilityTransferNotFound)?;
        if let Some(n) = name {
            transfer.name = n.to_string();
        }
        Ok(self.responsibility_transfers.get(transfer_id).unwrap())
    }
}

/// Info about a child in an organization.
pub struct ChildInfo {
    pub id: String,
    pub child_type: String,
}

/// Info about a parent in an organization.
pub struct ParentInfo {
    pub id: String,
    pub parent_type: String,
}

/// Info about a policy target.
pub struct PolicyTargetInfo {
    pub target_id: String,
    pub arn: String,
    pub name: String,
    pub target_type: String,
}
