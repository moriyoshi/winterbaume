//! Serde-compatible view types for ECS state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EcsService;
use crate::state::EcsState;
use crate::types::*;

/// Serializable view of the entire ECS state for one account/region.
/// Covers durable resources only; transient runtime state (running task
/// instances, container statuses, pending operations) is intentionally
/// excluded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsStateView {
    /// Clusters keyed by cluster name.
    #[serde(default)]
    pub clusters: HashMap<String, EcsClusterView>,
    /// Task definitions keyed by ARN.
    #[serde(default)]
    pub task_definitions: HashMap<String, TaskDefinitionView>,
    /// Services keyed by service ARN.
    #[serde(default)]
    pub services: HashMap<String, EcsServiceDefView>,
    /// Capacity providers keyed by name.
    #[serde(default)]
    pub capacity_providers: HashMap<String, EcsCapacityProviderView>,
    /// Account settings keyed by setting name.
    #[serde(default)]
    pub account_settings: HashMap<String, EcsAccountSettingView>,
    /// Resource tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, Vec<EcsTagView>>,
}

// ---------------------------------------------------------------------------
// View types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsTagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsClusterView {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub registered_container_instances_count: i32,
    pub running_tasks_count: i32,
    pub capacity_providers: Vec<String>,
    pub default_capacity_provider_strategy: Vec<CapacityProviderStrategyItemView>,
    #[serde(default)]
    pub tags: Vec<EcsTagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityProviderStrategyItemView {
    pub capacity_provider: String,
    pub weight: i32,
    pub base: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDefinitionView {
    pub name: String,
    pub image: String,
    pub cpu: i32,
    pub memory: Option<i32>,
    pub memory_reservation: Option<i32>,
    pub essential: bool,
    pub environment: Vec<EnvVarView>,
    pub log_configuration: Option<LogConfigurationView>,
    pub port_mappings: Vec<PortMappingView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVarView {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfigurationView {
    pub log_driver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMappingView {
    pub container_port: i32,
    pub host_port: i32,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDefinitionView {
    pub family: String,
    pub revision: i32,
    pub arn: String,
    pub container_definitions: Vec<ContainerDefinitionView>,
    pub status: String,
    pub network_mode: String,
    pub task_role_arn: Option<String>,
    pub execution_role_arn: Option<String>,
    pub requires_compatibilities: Vec<String>,
    pub cpu: Option<String>,
    pub memory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsLoadBalancerView {
    pub target_group_arn: Option<String>,
    pub load_balancer_name: Option<String>,
    pub container_name: Option<String>,
    pub container_port: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsServiceDefView {
    pub name: String,
    pub arn: String,
    pub cluster_arn: String,
    pub task_definition: String,
    pub desired_count: i32,
    pub running_count: i32,
    pub pending_count: i32,
    pub status: String,
    pub scheduling_strategy: String,
    pub launch_type: String,
    #[serde(default)]
    pub tags: Vec<EcsTagView>,
    pub load_balancers: Vec<EcsLoadBalancerView>,
    pub deployment_controller_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsCapacityProviderView {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub auto_scaling_group_arn: String,
    #[serde(default)]
    pub tags: Vec<EcsTagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsAccountSettingView {
    pub name: String,
    pub value: String,
    pub principal_arn: String,
}

// ---------------------------------------------------------------------------
// From internal types -> view types
// ---------------------------------------------------------------------------

impl From<&EcsTag> for EcsTagView {
    fn from(t: &EcsTag) -> Self {
        EcsTagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<&CapacityProviderStrategyItem> for CapacityProviderStrategyItemView {
    fn from(i: &CapacityProviderStrategyItem) -> Self {
        CapacityProviderStrategyItemView {
            capacity_provider: i.capacity_provider.clone(),
            weight: i.weight,
            base: i.base,
        }
    }
}

impl From<&EcsCluster> for EcsClusterView {
    fn from(c: &EcsCluster) -> Self {
        EcsClusterView {
            name: c.name.clone(),
            arn: c.arn.clone(),
            status: c.status.clone(),
            registered_container_instances_count: c.registered_container_instances_count,
            running_tasks_count: c.running_tasks_count,
            capacity_providers: c.capacity_providers.clone(),
            default_capacity_provider_strategy: c
                .default_capacity_provider_strategy
                .iter()
                .map(CapacityProviderStrategyItemView::from)
                .collect(),
            tags: c.tags.iter().map(EcsTagView::from).collect(),
        }
    }
}

impl From<&ContainerDefinition> for ContainerDefinitionView {
    fn from(cd: &ContainerDefinition) -> Self {
        ContainerDefinitionView {
            name: cd.name.clone(),
            image: cd.image.clone(),
            cpu: cd.cpu,
            memory: cd.memory,
            memory_reservation: cd.memory_reservation,
            essential: cd.essential,
            environment: cd
                .environment
                .iter()
                .map(|e| EnvVarView {
                    name: e.name.clone(),
                    value: e.value.clone(),
                })
                .collect(),
            log_configuration: cd
                .log_configuration
                .as_ref()
                .map(|lc| LogConfigurationView {
                    log_driver: lc.log_driver.clone(),
                }),
            port_mappings: cd
                .port_mappings
                .iter()
                .map(|pm| PortMappingView {
                    container_port: pm.container_port,
                    host_port: pm.host_port,
                    protocol: pm.protocol.clone(),
                })
                .collect(),
        }
    }
}

impl From<&TaskDefinition> for TaskDefinitionView {
    fn from(td: &TaskDefinition) -> Self {
        TaskDefinitionView {
            family: td.family.clone(),
            revision: td.revision,
            arn: td.arn.clone(),
            container_definitions: td
                .container_definitions
                .iter()
                .map(ContainerDefinitionView::from)
                .collect(),
            status: td.status.clone(),
            network_mode: td.network_mode.clone(),
            task_role_arn: td.task_role_arn.clone(),
            execution_role_arn: td.execution_role_arn.clone(),
            requires_compatibilities: td.requires_compatibilities.clone(),
            cpu: td.cpu.clone(),
            memory: td.memory.clone(),
        }
    }
}

impl From<&EcsLoadBalancer> for EcsLoadBalancerView {
    fn from(lb: &EcsLoadBalancer) -> Self {
        EcsLoadBalancerView {
            target_group_arn: lb.target_group_arn.clone(),
            load_balancer_name: lb.load_balancer_name.clone(),
            container_name: lb.container_name.clone(),
            container_port: lb.container_port,
        }
    }
}

impl From<&EcsServiceDef> for EcsServiceDefView {
    fn from(s: &EcsServiceDef) -> Self {
        EcsServiceDefView {
            name: s.name.clone(),
            arn: s.arn.clone(),
            cluster_arn: s.cluster_arn.clone(),
            task_definition: s.task_definition.clone(),
            desired_count: s.desired_count,
            running_count: s.running_count,
            pending_count: s.pending_count,
            status: s.status.clone(),
            scheduling_strategy: s.scheduling_strategy.clone(),
            launch_type: s.launch_type.clone(),
            tags: s.tags.iter().map(EcsTagView::from).collect(),
            load_balancers: s
                .load_balancers
                .iter()
                .map(EcsLoadBalancerView::from)
                .collect(),
            deployment_controller_type: s.deployment_controller_type.clone(),
        }
    }
}

impl From<&EcsCapacityProvider> for EcsCapacityProviderView {
    fn from(cp: &EcsCapacityProvider) -> Self {
        EcsCapacityProviderView {
            name: cp.name.clone(),
            arn: cp.arn.clone(),
            status: cp.status.clone(),
            auto_scaling_group_arn: cp.auto_scaling_group_arn.clone(),
            tags: cp.tags.iter().map(EcsTagView::from).collect(),
        }
    }
}

impl From<&EcsAccountSetting> for EcsAccountSettingView {
    fn from(s: &EcsAccountSetting) -> Self {
        EcsAccountSettingView {
            name: s.name.clone(),
            value: s.value.clone(),
            principal_arn: s.principal_arn.clone(),
        }
    }
}

impl From<&EcsState> for EcsStateView {
    fn from(state: &EcsState) -> Self {
        EcsStateView {
            clusters: state
                .clusters
                .iter()
                .map(|(k, v)| (k.clone(), EcsClusterView::from(v)))
                .collect(),
            task_definitions: state
                .task_definitions
                .iter()
                .map(|(k, v)| (k.clone(), TaskDefinitionView::from(v)))
                .collect(),
            services: state
                .services
                .iter()
                .map(|(k, v)| (k.clone(), EcsServiceDefView::from(v)))
                .collect(),
            capacity_providers: state
                .capacity_providers
                .iter()
                .map(|(k, v)| (k.clone(), EcsCapacityProviderView::from(v)))
                .collect(),
            account_settings: state
                .account_settings
                .iter()
                .map(|(k, v)| (k.clone(), EcsAccountSettingView::from(v)))
                .collect(),
            resource_tags: state
                .resource_tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(EcsTagView::from).collect()))
                .collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// From view types -> internal types
// ---------------------------------------------------------------------------

impl From<EcsTagView> for EcsTag {
    fn from(v: EcsTagView) -> Self {
        EcsTag {
            key: v.key,
            value: v.value,
        }
    }
}

impl From<CapacityProviderStrategyItemView> for CapacityProviderStrategyItem {
    fn from(v: CapacityProviderStrategyItemView) -> Self {
        CapacityProviderStrategyItem {
            capacity_provider: v.capacity_provider,
            weight: v.weight,
            base: v.base,
        }
    }
}

impl From<EcsClusterView> for EcsCluster {
    fn from(v: EcsClusterView) -> Self {
        EcsCluster {
            name: v.name,
            arn: v.arn,
            status: v.status,
            registered_container_instances_count: v.registered_container_instances_count,
            running_tasks_count: v.running_tasks_count,
            capacity_providers: v.capacity_providers,
            default_capacity_provider_strategy: v
                .default_capacity_provider_strategy
                .into_iter()
                .map(CapacityProviderStrategyItem::from)
                .collect(),
            tags: v.tags.into_iter().map(EcsTag::from).collect(),
        }
    }
}

impl From<ContainerDefinitionView> for ContainerDefinition {
    fn from(v: ContainerDefinitionView) -> Self {
        ContainerDefinition {
            name: v.name,
            image: v.image,
            cpu: v.cpu,
            memory: v.memory,
            memory_reservation: v.memory_reservation,
            essential: v.essential,
            environment: v
                .environment
                .into_iter()
                .map(|e| EnvVar {
                    name: e.name,
                    value: e.value,
                })
                .collect(),
            log_configuration: v.log_configuration.map(|lc| LogConfiguration {
                log_driver: lc.log_driver,
            }),
            port_mappings: v
                .port_mappings
                .into_iter()
                .map(|pm| PortMapping {
                    container_port: pm.container_port,
                    host_port: pm.host_port,
                    protocol: pm.protocol,
                })
                .collect(),
        }
    }
}

impl From<TaskDefinitionView> for TaskDefinition {
    fn from(v: TaskDefinitionView) -> Self {
        TaskDefinition {
            family: v.family,
            revision: v.revision,
            arn: v.arn,
            container_definitions: v
                .container_definitions
                .into_iter()
                .map(ContainerDefinition::from)
                .collect(),
            status: v.status,
            network_mode: v.network_mode,
            task_role_arn: v.task_role_arn,
            execution_role_arn: v.execution_role_arn,
            requires_compatibilities: v.requires_compatibilities,
            cpu: v.cpu,
            memory: v.memory,
        }
    }
}

impl From<EcsLoadBalancerView> for EcsLoadBalancer {
    fn from(v: EcsLoadBalancerView) -> Self {
        EcsLoadBalancer {
            target_group_arn: v.target_group_arn,
            load_balancer_name: v.load_balancer_name,
            container_name: v.container_name,
            container_port: v.container_port,
        }
    }
}

impl From<EcsServiceDefView> for EcsServiceDef {
    fn from(v: EcsServiceDefView) -> Self {
        EcsServiceDef {
            name: v.name,
            arn: v.arn,
            cluster_arn: v.cluster_arn,
            task_definition: v.task_definition,
            desired_count: v.desired_count,
            running_count: v.running_count,
            pending_count: v.pending_count,
            status: v.status,
            scheduling_strategy: v.scheduling_strategy,
            launch_type: v.launch_type,
            tags: v.tags.into_iter().map(EcsTag::from).collect(),
            load_balancers: v
                .load_balancers
                .into_iter()
                .map(EcsLoadBalancer::from)
                .collect(),
            deployment_controller_type: v.deployment_controller_type,
        }
    }
}

impl From<EcsCapacityProviderView> for EcsCapacityProvider {
    fn from(v: EcsCapacityProviderView) -> Self {
        EcsCapacityProvider {
            name: v.name,
            arn: v.arn,
            status: v.status,
            auto_scaling_group_arn: v.auto_scaling_group_arn,
            tags: v.tags.into_iter().map(EcsTag::from).collect(),
        }
    }
}

impl From<EcsAccountSettingView> for EcsAccountSetting {
    fn from(v: EcsAccountSettingView) -> Self {
        EcsAccountSetting {
            name: v.name,
            value: v.value,
            principal_arn: v.principal_arn,
        }
    }
}

impl From<EcsStateView> for EcsState {
    fn from(view: EcsStateView) -> Self {
        EcsState {
            clusters: view
                .clusters
                .into_iter()
                .map(|(k, v)| (k, EcsCluster::from(v)))
                .collect(),
            task_definitions: view
                .task_definitions
                .into_iter()
                .map(|(k, v)| (k, TaskDefinition::from(v)))
                .collect(),
            services: view
                .services
                .into_iter()
                .map(|(k, v)| (k, EcsServiceDef::from(v)))
                .collect(),
            capacity_providers: view
                .capacity_providers
                .into_iter()
                .map(|(k, v)| (k, EcsCapacityProvider::from(v)))
                .collect(),
            container_instances: HashMap::new(),
            tasks: HashMap::new(),
            task_sets: HashMap::new(),
            account_settings: view
                .account_settings
                .into_iter()
                .map(|(k, v)| (k, EcsAccountSetting::from(v)))
                .collect(),
            attributes: Vec::new(),
            resource_tags: view
                .resource_tags
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(EcsTag::from).collect()))
                .collect(),
            task_protections: HashMap::new(),
            account_settings_defaults: HashMap::new(),
            cluster_settings: HashMap::new(),
            express_gateway_services: HashMap::new(),
            poll_endpoints: HashMap::new(),
            attachment_state_changes: Vec::new(),
            container_state_changes: Vec::new(),
            task_state_changes: Vec::new(),
            service_deployments: HashMap::new(),
            service_revisions: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for EcsService {
    type StateView = EcsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        EcsStateView::from(&*guard)
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
            *guard = EcsState::from(view);
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
            for (k, v) in view.clusters {
                guard.clusters.insert(k, EcsCluster::from(v));
            }
            for (k, v) in view.task_definitions {
                guard.task_definitions.insert(k, TaskDefinition::from(v));
            }
            for (k, v) in view.services {
                guard.services.insert(k, EcsServiceDef::from(v));
            }
            for (k, v) in view.capacity_providers {
                guard
                    .capacity_providers
                    .insert(k, EcsCapacityProvider::from(v));
            }
            for (k, v) in view.account_settings {
                guard.account_settings.insert(k, EcsAccountSetting::from(v));
            }
            for (k, v) in view.resource_tags {
                guard
                    .resource_tags
                    .insert(k, v.into_iter().map(EcsTag::from).collect());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
