//! State management for the SSO Admin service.

use std::collections::{HashMap, HashSet};

use chrono::Utc;
use uuid::Uuid;

use crate::types::{AccountAssignmentKey, PermissionSetData};

#[derive(Debug, Default)]
pub struct SsoAdminState {
    /// permissionSetArn -> PermissionSetData
    pub permission_sets: HashMap<String, PermissionSetData>,
    /// Set of (accountId, permissionSetArn, principalType, principalId)
    pub account_assignments: HashSet<AccountAssignmentKey>,
    /// requestId -> AccountAssignmentOperationStatus (for in-flight tracking)
    pub assignment_statuses: HashMap<String, AssignmentStatus>,
}

#[derive(Debug, Clone)]
pub struct AssignmentStatus {
    pub request_id: String,
    pub status: String,
    pub permission_set_arn: String,
    pub principal_type: String,
    pub principal_id: String,
    pub target_id: String,
    pub target_type: String,
    pub created_date: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum SsoAdminError {
    #[error("Permission set with name '{0}' already exists.")]
    PermissionSetAlreadyExists(String),
    #[error("Permission set '{0}' not found.")]
    PermissionSetNotFound(String),
    #[error("Request '{0}' not found.")]
    RequestNotFound(String),
    #[error("Resource '{0}' not found.")]
    ResourceNotFound(String),
}

impl SsoAdminState {
    // ---- PermissionSet operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_permission_set(
        &mut self,
        name: &str,
        description: Option<String>,
        session_duration: Option<String>,
        relay_state: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&PermissionSetData, SsoAdminError> {
        // Check for duplicate name
        if self.permission_sets.values().any(|ps| ps.name == name) {
            return Err(SsoAdminError::PermissionSetAlreadyExists(name.to_string()));
        }
        let id = Uuid::new_v4().as_simple().to_string();
        let arn = format!("arn:aws:sso:::permissionSet/{}/{}", &id[..16], &id[16..32]);
        let _ = (account_id, region); // used for ARN in real impl; simplified here
        let ps = PermissionSetData {
            permission_set_arn: arn.clone(),
            name: name.to_string(),
            description,
            session_duration,
            relay_state,
            inline_policy: None,
            managed_policies: Vec::new(),
            customer_managed_policies: Vec::new(),
            tags,
            created_date: Utc::now().timestamp() as f64,
        };
        self.permission_sets.insert(arn.clone(), ps);
        Ok(self.permission_sets.get(&arn).unwrap())
    }

    pub fn describe_permission_set(
        &self,
        permission_set_arn: &str,
    ) -> Result<&PermissionSetData, SsoAdminError> {
        self.permission_sets
            .get(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))
    }

    pub fn list_permission_sets(&self) -> Vec<&PermissionSetData> {
        self.permission_sets.values().collect()
    }

    pub fn delete_permission_set(&mut self, permission_set_arn: &str) -> Result<(), SsoAdminError> {
        if self.permission_sets.remove(permission_set_arn).is_none() {
            return Err(SsoAdminError::PermissionSetNotFound(
                permission_set_arn.to_string(),
            ));
        }
        Ok(())
    }

    pub fn update_permission_set(
        &mut self,
        permission_set_arn: &str,
        description: Option<String>,
        session_duration: Option<String>,
        relay_state: Option<String>,
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        if let Some(d) = description {
            ps.description = Some(d);
        }
        if let Some(sd) = session_duration {
            ps.session_duration = Some(sd);
        }
        if let Some(rs) = relay_state {
            ps.relay_state = Some(rs);
        }
        Ok(())
    }

    // ---- AccountAssignment operations ----

    pub fn create_account_assignment(
        &mut self,
        permission_set_arn: &str,
        principal_type: &str,
        principal_id: &str,
        target_id: &str,
        target_type: &str,
    ) -> Result<AssignmentStatus, SsoAdminError> {
        if !self.permission_sets.contains_key(permission_set_arn) {
            return Err(SsoAdminError::PermissionSetNotFound(
                permission_set_arn.to_string(),
            ));
        }
        let key = AccountAssignmentKey {
            account_id: target_id.to_string(),
            permission_set_arn: permission_set_arn.to_string(),
            principal_type: principal_type.to_string(),
            principal_id: principal_id.to_string(),
        };
        self.account_assignments.insert(key);
        let request_id = Uuid::new_v4().to_string();
        let status = AssignmentStatus {
            request_id: request_id.clone(),
            status: "SUCCEEDED".to_string(),
            permission_set_arn: permission_set_arn.to_string(),
            principal_type: principal_type.to_string(),
            principal_id: principal_id.to_string(),
            target_id: target_id.to_string(),
            target_type: target_type.to_string(),
            created_date: Utc::now().timestamp() as f64,
        };
        self.assignment_statuses
            .insert(request_id.clone(), status.clone());
        Ok(status)
    }

    pub fn delete_account_assignment(
        &mut self,
        permission_set_arn: &str,
        principal_type: &str,
        principal_id: &str,
        target_id: &str,
        target_type: &str,
    ) -> Result<AssignmentStatus, SsoAdminError> {
        let key = AccountAssignmentKey {
            account_id: target_id.to_string(),
            permission_set_arn: permission_set_arn.to_string(),
            principal_type: principal_type.to_string(),
            principal_id: principal_id.to_string(),
        };
        self.account_assignments.remove(&key);
        let request_id = Uuid::new_v4().to_string();
        let status = AssignmentStatus {
            request_id: request_id.clone(),
            status: "SUCCEEDED".to_string(),
            permission_set_arn: permission_set_arn.to_string(),
            principal_type: principal_type.to_string(),
            principal_id: principal_id.to_string(),
            target_id: target_id.to_string(),
            target_type: target_type.to_string(),
            created_date: Utc::now().timestamp() as f64,
        };
        self.assignment_statuses
            .insert(request_id.clone(), status.clone());
        Ok(status)
    }

    pub fn describe_account_assignment_creation_status(
        &self,
        request_id: &str,
    ) -> Result<&AssignmentStatus, SsoAdminError> {
        self.assignment_statuses
            .get(request_id)
            .ok_or_else(|| SsoAdminError::RequestNotFound(request_id.to_string()))
    }

    pub fn describe_account_assignment_deletion_status(
        &self,
        request_id: &str,
    ) -> Result<&AssignmentStatus, SsoAdminError> {
        self.assignment_statuses
            .get(request_id)
            .ok_or_else(|| SsoAdminError::RequestNotFound(request_id.to_string()))
    }

    pub fn list_account_assignments(
        &self,
        account_id: &str,
        permission_set_arn: &str,
    ) -> Vec<&AccountAssignmentKey> {
        self.account_assignments
            .iter()
            .filter(|k| k.account_id == account_id && k.permission_set_arn == permission_set_arn)
            .collect()
    }

    pub fn list_accounts_for_provisioned_permission_set(
        &self,
        permission_set_arn: &str,
    ) -> Vec<String> {
        let mut accounts: Vec<String> = self
            .account_assignments
            .iter()
            .filter(|k| k.permission_set_arn == permission_set_arn)
            .map(|k| k.account_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        accounts.sort();
        accounts
    }

    pub fn list_permission_sets_provisioned_to_account(&self, account_id: &str) -> Vec<String> {
        let mut arns: Vec<String> = self
            .account_assignments
            .iter()
            .filter(|k| k.account_id == account_id)
            .map(|k| k.permission_set_arn.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        arns.sort();
        arns
    }

    // ---- Managed Policy operations ----

    pub fn attach_managed_policy(
        &mut self,
        permission_set_arn: &str,
        managed_policy_arn: &str,
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        if !ps
            .managed_policies
            .contains(&managed_policy_arn.to_string())
        {
            ps.managed_policies.push(managed_policy_arn.to_string());
        }
        Ok(())
    }

    pub fn detach_managed_policy(
        &mut self,
        permission_set_arn: &str,
        managed_policy_arn: &str,
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        ps.managed_policies.retain(|a| a != managed_policy_arn);
        Ok(())
    }

    pub fn list_managed_policies(
        &self,
        permission_set_arn: &str,
    ) -> Result<Vec<(String, String)>, SsoAdminError> {
        let ps = self
            .permission_sets
            .get(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        let policies = ps
            .managed_policies
            .iter()
            .map(|arn| {
                let name = arn.split('/').next_back().unwrap_or(arn).to_string();
                (arn.clone(), name)
            })
            .collect();
        Ok(policies)
    }

    // ---- Customer-Managed Policy operations ----

    pub fn attach_customer_managed_policy(
        &mut self,
        permission_set_arn: &str,
        policy_name: &str,
        policy_path: Option<String>,
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        let entry = (policy_name.to_string(), policy_path);
        if !ps.customer_managed_policies.contains(&entry) {
            ps.customer_managed_policies.push(entry);
        }
        Ok(())
    }

    pub fn detach_customer_managed_policy(
        &mut self,
        permission_set_arn: &str,
        policy_name: &str,
        policy_path: Option<&str>,
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        ps.customer_managed_policies
            .retain(|(n, p)| !(n == policy_name && p.as_deref() == policy_path));
        Ok(())
    }

    pub fn list_customer_managed_policies(
        &self,
        permission_set_arn: &str,
    ) -> Result<Vec<(String, Option<String>)>, SsoAdminError> {
        let ps = self
            .permission_sets
            .get(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        Ok(ps.customer_managed_policies.clone())
    }

    // ---- Inline Policy operations ----

    pub fn put_inline_policy(
        &mut self,
        permission_set_arn: &str,
        inline_policy: &str,
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        ps.inline_policy = Some(inline_policy.to_string());
        Ok(())
    }

    pub fn get_inline_policy(
        &self,
        permission_set_arn: &str,
    ) -> Result<Option<String>, SsoAdminError> {
        let ps = self
            .permission_sets
            .get(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        Ok(ps.inline_policy.clone())
    }

    pub fn delete_inline_policy(&mut self, permission_set_arn: &str) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(permission_set_arn)
            .ok_or_else(|| SsoAdminError::PermissionSetNotFound(permission_set_arn.to_string()))?;
        ps.inline_policy = None;
        Ok(())
    }

    // ---- Tags operations ----

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<(String, String)>, SsoAdminError> {
        let ps = self
            .permission_sets
            .get(resource_arn)
            .ok_or_else(|| SsoAdminError::ResourceNotFound(resource_arn.to_string()))?;
        Ok(ps
            .tags
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(resource_arn)
            .ok_or_else(|| SsoAdminError::ResourceNotFound(resource_arn.to_string()))?;
        for (k, v) in tags {
            ps.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), SsoAdminError> {
        let ps = self
            .permission_sets
            .get_mut(resource_arn)
            .ok_or_else(|| SsoAdminError::ResourceNotFound(resource_arn.to_string()))?;
        for key in tag_keys {
            ps.tags.remove(key);
        }
        Ok(())
    }
}
