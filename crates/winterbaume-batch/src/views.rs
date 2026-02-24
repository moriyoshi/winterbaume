//! Serde-compatible view types for Batch state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BatchService;
use crate::state::BatchState;
use crate::types::{
    ComputeEnvironment, ComputeEnvironmentOrder, ConsumableResource, ContainerProperties,
    FairsharePolicy, JobDefinition, JobQueue, ResourceRequirement, SchedulingPolicy,
    ServiceEnvironment, ShareAttributes,
};

/// Serializable view of the entire Batch state for one account/region.
/// Transient job execution state (`jobs`, `service_jobs`) is intentionally excluded.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchStateView {
    /// Compute environments keyed by name.
    #[serde(default)]
    pub compute_environments: HashMap<String, ComputeEnvironmentView>,
    /// Job queues keyed by name.
    #[serde(default)]
    pub job_queues: HashMap<String, JobQueueView>,
    /// Job definitions keyed by name (each entry holds all revisions).
    #[serde(default)]
    pub job_definitions: HashMap<String, Vec<JobDefinitionView>>,
    /// Scheduling policies keyed by name.
    #[serde(default)]
    pub scheduling_policies: HashMap<String, SchedulingPolicyView>,
    /// Consumable resources keyed by name.
    #[serde(default)]
    pub consumable_resources: HashMap<String, ConsumableResourceView>,
    /// Service environments keyed by name.
    #[serde(default)]
    pub service_environments: HashMap<String, ServiceEnvironmentView>,
}

// --- Sub-view types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeEnvironmentView {
    pub compute_environment_name: String,
    pub compute_environment_arn: String,
    pub ce_type: String,
    pub state: String,
    pub status: String,
    pub status_reason: String,
    pub service_role: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub compute_resources: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub eks_configuration: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobQueueView {
    pub job_queue_name: String,
    pub job_queue_arn: String,
    pub state: String,
    pub status: String,
    pub status_reason: String,
    pub priority: i32,
    #[serde(default)]
    pub compute_environment_order: Vec<ComputeEnvironmentOrderView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_at: Option<String>,
    pub scheduling_policy_arn: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub job_state_time_limit_action: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeEnvironmentOrderView {
    pub order: i32,
    pub compute_environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDefinitionView {
    pub job_definition_name: String,
    pub job_definition_arn: String,
    pub revision: i32,
    pub status: String,
    pub job_definition_type: String,
    pub container_properties: Option<ContainerPropertiesView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub eks_properties: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub retry_strategy: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerPropertiesView {
    pub image: String,
    #[serde(default)]
    pub command: Vec<String>,
    #[serde(default)]
    pub resource_requirements: Vec<ResourceRequirementView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirementView {
    pub resource_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingPolicyView {
    pub name: String,
    pub arn: String,
    pub fairshare_policy: Option<FairsharePolicyView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairsharePolicyView {
    pub compute_reservation: Option<i32>,
    pub share_decay_seconds: Option<i32>,
    #[serde(default)]
    pub share_distribution: Vec<ShareAttributesView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareAttributesView {
    pub share_identifier: String,
    pub weight_factor: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableResourceView {
    pub consumable_resource_name: String,
    pub consumable_resource_arn: String,
    pub total_quantity: i64,
    pub in_use_quantity: i64,
    pub resource_type: Option<String>,
    pub created_at: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEnvironmentView {
    pub service_environment_name: String,
    pub service_environment_arn: String,
    pub service_environment_type: String,
    pub state: String,
    pub status: String,
    pub created_at: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&BatchState> for BatchStateView {
    fn from(state: &BatchState) -> Self {
        BatchStateView {
            compute_environments: state
                .compute_environments
                .iter()
                .map(|(k, v)| (k.clone(), ComputeEnvironmentView::from(v)))
                .collect(),
            job_queues: state
                .job_queues
                .iter()
                .map(|(k, v)| (k.clone(), JobQueueView::from(v)))
                .collect(),
            job_definitions: state
                .job_definitions
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(JobDefinitionView::from).collect()))
                .collect(),
            scheduling_policies: state
                .scheduling_policies
                .iter()
                .map(|(k, v)| (k.clone(), SchedulingPolicyView::from(v)))
                .collect(),
            consumable_resources: state
                .consumable_resources
                .iter()
                .map(|(k, v)| (k.clone(), ConsumableResourceView::from(v)))
                .collect(),
            service_environments: state
                .service_environments
                .iter()
                .map(|(k, v)| (k.clone(), ServiceEnvironmentView::from(v)))
                .collect(),
        }
    }
}

impl From<&ConsumableResource> for ConsumableResourceView {
    fn from(cr: &ConsumableResource) -> Self {
        ConsumableResourceView {
            consumable_resource_name: cr.consumable_resource_name.clone(),
            consumable_resource_arn: cr.consumable_resource_arn.clone(),
            total_quantity: cr.total_quantity,
            in_use_quantity: cr.in_use_quantity,
            resource_type: cr.resource_type.clone(),
            created_at: cr.created_at,
            tags: cr.tags.clone(),
        }
    }
}

impl From<&ServiceEnvironment> for ServiceEnvironmentView {
    fn from(se: &ServiceEnvironment) -> Self {
        ServiceEnvironmentView {
            service_environment_name: se.service_environment_name.clone(),
            service_environment_arn: se.service_environment_arn.clone(),
            service_environment_type: se.service_environment_type.clone(),
            state: se.state.clone(),
            status: se.status.clone(),
            created_at: se.created_at,
            tags: se.tags.clone(),
        }
    }
}

impl From<&ComputeEnvironment> for ComputeEnvironmentView {
    fn from(ce: &ComputeEnvironment) -> Self {
        ComputeEnvironmentView {
            compute_environment_name: ce.compute_environment_name.clone(),
            compute_environment_arn: ce.compute_environment_arn.clone(),
            ce_type: ce.ce_type.clone(),
            state: ce.state.clone(),
            status: ce.status.clone(),
            status_reason: ce.status_reason.clone(),
            service_role: ce.service_role.clone(),
            tags: ce.tags.clone(),
            created_at: Some(ce.created_at.to_rfc3339()),
            compute_resources: vec![],
            eks_configuration: vec![],
        }
    }
}

impl From<&JobQueue> for JobQueueView {
    fn from(jq: &JobQueue) -> Self {
        JobQueueView {
            job_queue_name: jq.job_queue_name.clone(),
            job_queue_arn: jq.job_queue_arn.clone(),
            state: jq.state.clone(),
            status: jq.status.clone(),
            status_reason: jq.status_reason.clone(),
            priority: jq.priority,
            compute_environment_order: jq
                .compute_environment_order
                .iter()
                .map(ComputeEnvironmentOrderView::from)
                .collect(),
            tags: jq.tags.clone(),
            created_at: Some(jq.created_at.to_rfc3339()),
            scheduling_policy_arn: jq.scheduling_policy_arn.clone(),
            job_state_time_limit_action: vec![],
        }
    }
}

impl From<&ComputeEnvironmentOrder> for ComputeEnvironmentOrderView {
    fn from(ceo: &ComputeEnvironmentOrder) -> Self {
        ComputeEnvironmentOrderView {
            order: ceo.order,
            compute_environment: ceo.compute_environment.clone(),
        }
    }
}

impl From<&JobDefinition> for JobDefinitionView {
    fn from(jd: &JobDefinition) -> Self {
        JobDefinitionView {
            job_definition_name: jd.job_definition_name.clone(),
            job_definition_arn: jd.job_definition_arn.clone(),
            revision: jd.revision,
            status: jd.status.clone(),
            job_definition_type: jd.job_definition_type.clone(),
            container_properties: jd
                .container_properties
                .as_ref()
                .map(ContainerPropertiesView::from),
            tags: jd.tags.clone(),
            created_at: Some(jd.created_at.to_rfc3339()),
            eks_properties: vec![],
            retry_strategy: vec![],
        }
    }
}

impl From<&ContainerProperties> for ContainerPropertiesView {
    fn from(cp: &ContainerProperties) -> Self {
        ContainerPropertiesView {
            image: cp.image.clone(),
            command: cp.command.clone(),
            resource_requirements: cp
                .resource_requirements
                .iter()
                .map(ResourceRequirementView::from)
                .collect(),
        }
    }
}

impl From<&ResourceRequirement> for ResourceRequirementView {
    fn from(rr: &ResourceRequirement) -> Self {
        ResourceRequirementView {
            resource_type: rr.resource_type.clone(),
            value: rr.value.clone(),
        }
    }
}

impl From<&SchedulingPolicy> for SchedulingPolicyView {
    fn from(sp: &SchedulingPolicy) -> Self {
        SchedulingPolicyView {
            name: sp.name.clone(),
            arn: sp.arn.clone(),
            fairshare_policy: sp.fairshare_policy.as_ref().map(FairsharePolicyView::from),
            tags: sp.tags.clone(),
        }
    }
}

impl From<&FairsharePolicy> for FairsharePolicyView {
    fn from(fp: &FairsharePolicy) -> Self {
        FairsharePolicyView {
            compute_reservation: fp.compute_reservation,
            share_decay_seconds: fp.share_decay_seconds,
            share_distribution: fp
                .share_distribution
                .iter()
                .map(ShareAttributesView::from)
                .collect(),
        }
    }
}

impl From<&ShareAttributes> for ShareAttributesView {
    fn from(sa: &ShareAttributes) -> Self {
        ShareAttributesView {
            share_identifier: sa.share_identifier.clone(),
            weight_factor: sa.weight_factor,
        }
    }
}

// --- From view types to internal types ---

impl From<BatchStateView> for BatchState {
    fn from(view: BatchStateView) -> Self {
        let mut state = BatchState::default();
        state.compute_environments = view
            .compute_environments
            .into_values()
            .map(|v| {
                let ce = ComputeEnvironment::from(v);
                (ce.compute_environment_name.clone(), ce)
            })
            .collect();
        state.job_queues = view
            .job_queues
            .into_values()
            .map(|v| {
                let jq = JobQueue::from(v);
                (jq.job_queue_name.clone(), jq)
            })
            .collect();
        state.job_definitions = view
            .job_definitions
            .into_iter()
            .map(|(k, revs)| (k, revs.into_iter().map(JobDefinition::from).collect()))
            .collect();
        state.scheduling_policies = view
            .scheduling_policies
            .into_values()
            .map(|v| {
                let sp = SchedulingPolicy::from(v);
                (sp.name.clone(), sp)
            })
            .collect();
        state.consumable_resources = view
            .consumable_resources
            .into_values()
            .map(|v| {
                let cr = ConsumableResource::from(v);
                (cr.consumable_resource_name.clone(), cr)
            })
            .collect();
        state.service_environments = view
            .service_environments
            .into_values()
            .map(|v| {
                let se = ServiceEnvironment::from(v);
                (se.service_environment_name.clone(), se)
            })
            .collect();
        state
    }
}

impl From<ConsumableResourceView> for ConsumableResource {
    fn from(v: ConsumableResourceView) -> Self {
        ConsumableResource {
            consumable_resource_name: v.consumable_resource_name,
            consumable_resource_arn: v.consumable_resource_arn,
            total_quantity: v.total_quantity,
            in_use_quantity: v.in_use_quantity,
            resource_type: v.resource_type,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl From<ServiceEnvironmentView> for ServiceEnvironment {
    fn from(v: ServiceEnvironmentView) -> Self {
        ServiceEnvironment {
            service_environment_name: v.service_environment_name,
            service_environment_arn: v.service_environment_arn,
            service_environment_type: v.service_environment_type,
            state: v.state,
            status: v.status,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl From<ComputeEnvironmentView> for ComputeEnvironment {
    fn from(v: ComputeEnvironmentView) -> Self {
        let created_at = v
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ComputeEnvironment {
            compute_environment_name: v.compute_environment_name,
            compute_environment_arn: v.compute_environment_arn,
            ce_type: v.ce_type,
            state: v.state,
            status: v.status,
            status_reason: v.status_reason,
            service_role: v.service_role,
            tags: v.tags,
            created_at,
        }
    }
}

impl From<JobQueueView> for JobQueue {
    fn from(v: JobQueueView) -> Self {
        let created_at = v
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        JobQueue {
            job_queue_name: v.job_queue_name,
            job_queue_arn: v.job_queue_arn,
            state: v.state,
            status: v.status,
            status_reason: v.status_reason,
            priority: v.priority,
            compute_environment_order: v
                .compute_environment_order
                .into_iter()
                .map(ComputeEnvironmentOrder::from)
                .collect(),
            tags: v.tags,
            created_at,
            scheduling_policy_arn: v.scheduling_policy_arn,
        }
    }
}

impl From<ComputeEnvironmentOrderView> for ComputeEnvironmentOrder {
    fn from(v: ComputeEnvironmentOrderView) -> Self {
        ComputeEnvironmentOrder {
            order: v.order,
            compute_environment: v.compute_environment,
        }
    }
}

impl From<JobDefinitionView> for JobDefinition {
    fn from(v: JobDefinitionView) -> Self {
        let created_at = v
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        JobDefinition {
            job_definition_name: v.job_definition_name,
            job_definition_arn: v.job_definition_arn,
            revision: v.revision,
            status: v.status,
            job_definition_type: v.job_definition_type,
            container_properties: v.container_properties.map(ContainerProperties::from),
            tags: v.tags,
            created_at,
        }
    }
}

impl From<ContainerPropertiesView> for ContainerProperties {
    fn from(v: ContainerPropertiesView) -> Self {
        ContainerProperties {
            image: v.image,
            command: v.command,
            resource_requirements: v
                .resource_requirements
                .into_iter()
                .map(ResourceRequirement::from)
                .collect(),
        }
    }
}

impl From<ResourceRequirementView> for ResourceRequirement {
    fn from(v: ResourceRequirementView) -> Self {
        ResourceRequirement {
            resource_type: v.resource_type,
            value: v.value,
        }
    }
}

impl From<SchedulingPolicyView> for SchedulingPolicy {
    fn from(v: SchedulingPolicyView) -> Self {
        SchedulingPolicy {
            name: v.name,
            arn: v.arn,
            fairshare_policy: v.fairshare_policy.map(FairsharePolicy::from),
            tags: v.tags,
        }
    }
}

impl From<FairsharePolicyView> for FairsharePolicy {
    fn from(v: FairsharePolicyView) -> Self {
        FairsharePolicy {
            compute_reservation: v.compute_reservation,
            share_decay_seconds: v.share_decay_seconds,
            share_distribution: v
                .share_distribution
                .into_iter()
                .map(ShareAttributes::from)
                .collect(),
        }
    }
}

impl From<ShareAttributesView> for ShareAttributes {
    fn from(v: ShareAttributesView) -> Self {
        ShareAttributes {
            share_identifier: v.share_identifier,
            weight_factor: v.weight_factor,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for BatchService {
    type StateView = BatchStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BatchStateView::from(&*guard)
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
            *guard = BatchState::from(view);
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
            for (name, ce_view) in view.compute_environments {
                guard
                    .compute_environments
                    .insert(name, ComputeEnvironment::from(ce_view));
            }
            for (name, jq_view) in view.job_queues {
                guard.job_queues.insert(name, JobQueue::from(jq_view));
            }
            for (name, revs) in view.job_definitions {
                guard
                    .job_definitions
                    .insert(name, revs.into_iter().map(JobDefinition::from).collect());
            }
            for (name, sp_view) in view.scheduling_policies {
                guard
                    .scheduling_policies
                    .insert(name, SchedulingPolicy::from(sp_view));
            }
            for (name, cr_view) in view.consumable_resources {
                guard
                    .consumable_resources
                    .insert(name, ConsumableResource::from(cr_view));
            }
            for (name, se_view) in view.service_environments {
                guard
                    .service_environments
                    .insert(name, ServiceEnvironment::from(se_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
