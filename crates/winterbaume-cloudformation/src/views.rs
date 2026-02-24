//! Serde-compatible view types for CloudFormation state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudFormationService;
use crate::state::CloudFormationState;
use crate::types::{
    ChangeSet, Stack, StackEvent, StackExport, StackInstance, StackInstanceKey, StackParameter,
    StackResource, StackSet, StackSetOperation, StackTag,
};

// ---------------------------------------------------------------------------
// View types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudFormationStateView {
    #[serde(default)]
    pub stacks: HashMap<String, StackView>,
    #[serde(default)]
    pub stack_sets: HashMap<String, StackSetView>,
    #[serde(default)]
    pub stack_instances: Vec<StackInstanceView>,
    #[serde(default)]
    pub registered_types: Vec<crate::types::RegisteredType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackView {
    pub stack_id: String,
    pub stack_name: String,
    pub stack_status: String,
    pub creation_time: String,
    pub last_updated_time: Option<String>,
    pub deletion_time: Option<String>,
    pub description: Option<String>,
    pub template_body: Option<String>,
    pub stack_policy_body: Option<String>,
    pub parameters: Vec<StackParameter>,
    pub outputs: Vec<crate::types::StackOutput>,
    pub tags: Vec<StackTag>,
    pub capabilities: Vec<String>,
    pub resources: Vec<StackResource>,
    pub events: Vec<StackEvent>,
    pub change_sets: Vec<ChangeSet>,
    pub exports: Vec<StackExport>,
    pub role_arn: Option<String>,
    pub timeout_in_minutes: Option<i32>,
    pub disable_rollback: bool,
    #[serde(default)]
    pub enable_termination_protection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackSetView {
    pub stack_set_id: String,
    pub stack_set_name: String,
    pub stack_set_arn: String,
    pub status: String,
    pub description: Option<String>,
    pub template_body: Option<String>,
    pub parameters: Vec<StackParameter>,
    pub tags: Vec<StackTag>,
    pub capabilities: Vec<String>,
    pub operations: Vec<StackSetOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackInstanceView {
    pub stack_set_name: String,
    pub account: String,
    pub region: String,
    pub stack_id: Option<String>,
    pub status: String,
    pub status_reason: Option<String>,
    pub stack_set_id: String,
    pub parameter_overrides: Vec<StackParameter>,
}

// ---------------------------------------------------------------------------
// Conversions: State -> View
// ---------------------------------------------------------------------------

impl From<&Stack> for StackView {
    fn from(s: &Stack) -> Self {
        Self {
            stack_id: s.stack_id.clone(),
            stack_name: s.stack_name.clone(),
            stack_status: s.stack_status.clone(),
            creation_time: s.creation_time.clone(),
            last_updated_time: s.last_updated_time.clone(),
            deletion_time: s.deletion_time.clone(),
            description: s.description.clone(),
            template_body: s.template_body.clone(),
            stack_policy_body: s.stack_policy_body.clone(),
            parameters: s.parameters.clone(),
            outputs: s.outputs.clone(),
            tags: s.tags.clone(),
            capabilities: s.capabilities.clone(),
            resources: s.resources.clone(),
            events: s.events.clone(),
            change_sets: s.change_sets.clone(),
            exports: s.exports.clone(),
            role_arn: s.role_arn.clone(),
            timeout_in_minutes: s.timeout_in_minutes,
            disable_rollback: s.disable_rollback,
            enable_termination_protection: s.enable_termination_protection,
        }
    }
}

impl From<StackView> for Stack {
    fn from(v: StackView) -> Self {
        Self {
            stack_id: v.stack_id,
            stack_name: v.stack_name,
            stack_status: v.stack_status,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            deletion_time: v.deletion_time,
            description: v.description,
            template_body: v.template_body,
            stack_policy_body: v.stack_policy_body,
            parameters: v.parameters,
            outputs: v.outputs,
            tags: v.tags,
            capabilities: v.capabilities,
            resources: v.resources,
            events: v.events,
            change_sets: v.change_sets,
            exports: v.exports,
            role_arn: v.role_arn,
            timeout_in_minutes: v.timeout_in_minutes,
            disable_rollback: v.disable_rollback,
            enable_termination_protection: v.enable_termination_protection,
        }
    }
}

impl From<&StackSet> for StackSetView {
    fn from(ss: &StackSet) -> Self {
        Self {
            stack_set_id: ss.stack_set_id.clone(),
            stack_set_name: ss.stack_set_name.clone(),
            stack_set_arn: ss.stack_set_arn.clone(),
            status: ss.status.clone(),
            description: ss.description.clone(),
            template_body: ss.template_body.clone(),
            parameters: ss.parameters.clone(),
            tags: ss.tags.clone(),
            capabilities: ss.capabilities.clone(),
            operations: ss.operations.clone(),
        }
    }
}

impl From<StackSetView> for StackSet {
    fn from(v: StackSetView) -> Self {
        Self {
            stack_set_id: v.stack_set_id,
            stack_set_name: v.stack_set_name,
            stack_set_arn: v.stack_set_arn,
            status: v.status,
            description: v.description,
            template_body: v.template_body,
            parameters: v.parameters,
            tags: v.tags,
            capabilities: v.capabilities,
            operations: v.operations,
        }
    }
}

impl From<&StackInstance> for StackInstanceView {
    fn from(i: &StackInstance) -> Self {
        Self {
            stack_set_name: i.stack_set_name.clone(),
            account: i.account.clone(),
            region: i.region.clone(),
            stack_id: i.stack_id.clone(),
            status: i.status.clone(),
            status_reason: i.status_reason.clone(),
            stack_set_id: i.stack_set_id.clone(),
            parameter_overrides: i.parameter_overrides.clone(),
        }
    }
}

impl From<StackInstanceView> for StackInstance {
    fn from(v: StackInstanceView) -> Self {
        Self {
            stack_set_name: v.stack_set_name,
            account: v.account,
            region: v.region,
            stack_id: v.stack_id,
            status: v.status,
            status_reason: v.status_reason,
            stack_set_id: v.stack_set_id,
            parameter_overrides: v.parameter_overrides,
        }
    }
}

impl From<&CloudFormationState> for CloudFormationStateView {
    fn from(state: &CloudFormationState) -> Self {
        Self {
            stacks: state
                .stacks
                .iter()
                .map(|(k, v)| (k.clone(), StackView::from(v)))
                .collect(),
            stack_sets: state
                .stack_sets
                .iter()
                .map(|(k, v)| (k.clone(), StackSetView::from(v)))
                .collect(),
            stack_instances: state
                .stack_instances
                .values()
                .map(StackInstanceView::from)
                .collect(),
            registered_types: state.registered_types.clone(),
        }
    }
}

impl From<CloudFormationStateView> for CloudFormationState {
    fn from(view: CloudFormationStateView) -> Self {
        let stack_instances = view
            .stack_instances
            .into_iter()
            .map(|v| {
                let key = StackInstanceKey {
                    stack_set_name: v.stack_set_name.clone(),
                    account: v.account.clone(),
                    region: v.region.clone(),
                };
                (key, StackInstance::from(v))
            })
            .collect();
        Self {
            stacks: view
                .stacks
                .into_iter()
                .map(|(k, v)| (k, Stack::from(v)))
                .collect(),
            stack_sets: view
                .stack_sets
                .into_iter()
                .map(|(k, v)| (k, StackSet::from(v)))
                .collect(),
            stack_instances,
            registered_types: view.registered_types,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for CloudFormationService {
    type StateView = CloudFormationStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudFormationStateView::from(&*guard)
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
            *guard = CloudFormationState::from(view);
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
            for (k, v) in view.stacks {
                guard.stacks.insert(k, Stack::from(v));
            }
            for (k, v) in view.stack_sets {
                guard.stack_sets.insert(k, StackSet::from(v));
            }
            for v in view.stack_instances {
                let key = StackInstanceKey {
                    stack_set_name: v.stack_set_name.clone(),
                    account: v.account.clone(),
                    region: v.region.clone(),
                };
                guard.stack_instances.insert(key, StackInstance::from(v));
            }
            for rt in view.registered_types {
                if !guard
                    .registered_types
                    .iter()
                    .any(|t| t.type_name == rt.type_name && t.type_kind == rt.type_kind)
                {
                    guard.registered_types.push(rt);
                }
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
