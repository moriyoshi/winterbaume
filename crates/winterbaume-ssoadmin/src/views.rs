//! Serde-compatible view types for SSO Admin state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SsoAdminService;
use crate::state::{AssignmentStatus, SsoAdminState};
use crate::types::{AccountAssignmentKey, PermissionSetData};

/// Serializable view of the entire SSO Admin state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SsoAdminStateView {
    #[serde(default)]
    pub permission_sets: HashMap<String, PermissionSetView>,
    #[serde(default)]
    pub account_assignments: Vec<AccountAssignmentView>,
    #[serde(default)]
    pub assignment_statuses: HashMap<String, AssignmentStatusView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSetView {
    pub permission_set_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub session_duration: Option<String>,
    pub relay_state: Option<String>,
    pub inline_policy: Option<String>,
    #[serde(default)]
    pub managed_policies: Vec<String>,
    #[serde(default)]
    pub customer_managed_policies: Vec<(String, Option<String>)>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_date: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAssignmentView {
    pub account_id: String,
    pub permission_set_arn: String,
    pub principal_type: String,
    pub principal_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentStatusView {
    pub request_id: String,
    pub status: String,
    pub permission_set_arn: String,
    pub principal_type: String,
    pub principal_id: String,
    pub target_id: String,
    pub target_type: String,
    pub created_date: f64,
}

impl From<&PermissionSetData> for PermissionSetView {
    fn from(ps: &PermissionSetData) -> Self {
        Self {
            permission_set_arn: ps.permission_set_arn.clone(),
            name: ps.name.clone(),
            description: ps.description.clone(),
            session_duration: ps.session_duration.clone(),
            relay_state: ps.relay_state.clone(),
            inline_policy: ps.inline_policy.clone(),
            managed_policies: ps.managed_policies.clone(),
            customer_managed_policies: ps.customer_managed_policies.clone(),
            tags: ps.tags.clone(),
            created_date: ps.created_date,
        }
    }
}

impl From<PermissionSetView> for PermissionSetData {
    fn from(view: PermissionSetView) -> Self {
        Self {
            permission_set_arn: view.permission_set_arn,
            name: view.name,
            description: view.description,
            session_duration: view.session_duration,
            relay_state: view.relay_state,
            inline_policy: view.inline_policy,
            managed_policies: view.managed_policies,
            customer_managed_policies: view.customer_managed_policies,
            tags: view.tags,
            created_date: view.created_date,
        }
    }
}

impl From<&AccountAssignmentKey> for AccountAssignmentView {
    fn from(key: &AccountAssignmentKey) -> Self {
        Self {
            account_id: key.account_id.clone(),
            permission_set_arn: key.permission_set_arn.clone(),
            principal_type: key.principal_type.clone(),
            principal_id: key.principal_id.clone(),
        }
    }
}

impl From<AccountAssignmentView> for AccountAssignmentKey {
    fn from(view: AccountAssignmentView) -> Self {
        Self {
            account_id: view.account_id,
            permission_set_arn: view.permission_set_arn,
            principal_type: view.principal_type,
            principal_id: view.principal_id,
        }
    }
}

impl From<&AssignmentStatus> for AssignmentStatusView {
    fn from(s: &AssignmentStatus) -> Self {
        Self {
            request_id: s.request_id.clone(),
            status: s.status.clone(),
            permission_set_arn: s.permission_set_arn.clone(),
            principal_type: s.principal_type.clone(),
            principal_id: s.principal_id.clone(),
            target_id: s.target_id.clone(),
            target_type: s.target_type.clone(),
            created_date: s.created_date,
        }
    }
}

impl From<AssignmentStatusView> for AssignmentStatus {
    fn from(view: AssignmentStatusView) -> Self {
        Self {
            request_id: view.request_id,
            status: view.status,
            permission_set_arn: view.permission_set_arn,
            principal_type: view.principal_type,
            principal_id: view.principal_id,
            target_id: view.target_id,
            target_type: view.target_type,
            created_date: view.created_date,
        }
    }
}

impl From<&SsoAdminState> for SsoAdminStateView {
    fn from(state: &SsoAdminState) -> Self {
        Self {
            permission_sets: state
                .permission_sets
                .iter()
                .map(|(k, v)| (k.clone(), PermissionSetView::from(v)))
                .collect(),
            account_assignments: state
                .account_assignments
                .iter()
                .map(AccountAssignmentView::from)
                .collect(),
            assignment_statuses: state
                .assignment_statuses
                .iter()
                .map(|(k, v)| (k.clone(), AssignmentStatusView::from(v)))
                .collect(),
        }
    }
}

impl From<SsoAdminStateView> for SsoAdminState {
    fn from(view: SsoAdminStateView) -> Self {
        Self {
            permission_sets: view
                .permission_sets
                .into_iter()
                .map(|(k, v)| (k, PermissionSetData::from(v)))
                .collect(),
            account_assignments: view
                .account_assignments
                .into_iter()
                .map(AccountAssignmentKey::from)
                .collect(),
            assignment_statuses: view
                .assignment_statuses
                .into_iter()
                .map(|(k, v)| (k, AssignmentStatus::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for SsoAdminService {
    type StateView = SsoAdminStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SsoAdminStateView::from(&*guard)
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
            *guard = SsoAdminState::from(view);
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
            for (arn, ps_view) in view.permission_sets {
                guard
                    .permission_sets
                    .insert(arn, PermissionSetData::from(ps_view));
            }
            for assignment_view in view.account_assignments {
                guard
                    .account_assignments
                    .insert(AccountAssignmentKey::from(assignment_view));
            }
            for (id, status_view) in view.assignment_statuses {
                guard
                    .assignment_statuses
                    .insert(id, AssignmentStatus::from(status_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
