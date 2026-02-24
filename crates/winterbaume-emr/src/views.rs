//! Serde-compatible view types for EMR state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EmrService;
use crate::state::EmrState;
use crate::types::{
    ApplicationData, AutoScalingPolicyData, AutoTerminationPolicyData,
    BlockPublicAccessConfigurationData, BootstrapActionData, ClusterData, ComputeLimitsData,
    InstanceFleetData, InstanceGroupData, ManagedScalingPolicyData, NotebookExecutionData,
    PersistentAppUiData, PortRangeData, ScalingConstraintsData, SecurityConfigurationData,
    SessionMappingData, StepData, StudioData,
};

/// Serializable view of the entire EMR state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmrStateView {
    #[serde(default)]
    pub clusters: HashMap<String, ClusterView>,
    /// cluster_id -> steps
    #[serde(default)]
    pub steps: HashMap<String, Vec<StepView>>,
    #[serde(default)]
    pub security_configurations: HashMap<String, SecurityConfigurationView>,
    #[serde(default)]
    pub block_public_access_config: Option<BlockPublicAccessConfigView>,
    #[serde(default)]
    pub studios: HashMap<String, StudioView>,
    #[serde(default)]
    pub session_mappings: Vec<SessionMappingView>,
    #[serde(default)]
    pub notebook_executions: HashMap<String, NotebookExecutionView>,
    #[serde(default)]
    pub persistent_app_uis: HashMap<String, PersistentAppUiView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterView {
    pub id: String,
    pub name: String,
    pub status: String,
    pub creation_date_time: String,
    pub ready_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub termination_protected: bool,
    pub visible_to_all_users: bool,
    pub log_uri: Option<String>,
    pub release_label: Option<String>,
    #[serde(default)]
    pub applications: Vec<ApplicationView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub service_role: Option<String>,
    pub job_flow_role: Option<String>,
    pub auto_scaling_role: Option<String>,
    pub scale_down_behavior: Option<String>,
    pub security_configuration: Option<String>,
    pub step_concurrency_level: Option<i32>,
    pub auto_termination_policy: Option<AutoTerminationPolicyView>,
    pub managed_scaling_policy: Option<ManagedScalingPolicyView>,
    pub cluster_arn: String,
    pub normalized_instance_hours: Option<i32>,
    pub master_public_dns_name: Option<String>,
    #[serde(default)]
    pub instance_groups: Vec<InstanceGroupView>,
    #[serde(default)]
    pub instance_fleets: Vec<InstanceFleetView>,
    #[serde(default)]
    pub bootstrap_actions: Vec<BootstrapActionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTerminationPolicyView {
    pub idle_timeout: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedScalingPolicyView {
    pub compute_limits: Option<ComputeLimitsView>,
    pub scaling_strategy: Option<String>,
    pub utilization_performance_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeLimitsView {
    pub minimum_capacity_units: i32,
    pub maximum_capacity_units: i32,
    pub maximum_on_demand_capacity_units: Option<i32>,
    pub maximum_core_capacity_units: Option<i32>,
    pub unit_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceGroupView {
    pub id: String,
    pub name: Option<String>,
    pub instance_type: String,
    pub instance_role: String,
    pub instance_count: i32,
    pub market: Option<String>,
    pub bid_price: Option<String>,
    pub status: String,
    pub running_instance_count: Option<i32>,
    #[serde(default)]
    pub auto_scaling_policy: Option<AutoScalingPolicyView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingPolicyView {
    pub status: String,
    pub constraints: Option<ScalingConstraintsView>,
    #[serde(default)]
    pub rules: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConstraintsView {
    pub min_capacity: i32,
    pub max_capacity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceFleetView {
    pub id: String,
    pub name: Option<String>,
    pub instance_fleet_type: String,
    pub target_on_demand_capacity: Option<i32>,
    pub target_spot_capacity: Option<i32>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapActionView {
    pub name: String,
    pub script_path: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepView {
    pub id: String,
    pub name: String,
    pub cluster_id: String,
    pub status: String,
    pub creation_date_time: String,
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub jar: String,
    pub args: Option<Vec<String>>,
    pub main_class: Option<String>,
    pub action_on_failure: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfigurationView {
    pub name: String,
    pub security_configuration: String,
    pub creation_date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockPublicAccessConfigView {
    pub block_public_security_group_rules: bool,
    #[serde(default)]
    pub permitted_ranges: Vec<PortRangeView>,
    pub creation_date_time: String,
    pub created_by_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRangeView {
    pub min_range: i32,
    pub max_range: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioView {
    pub studio_id: String,
    pub name: String,
    pub description: Option<String>,
    pub auth_mode: String,
    pub vpc_id: Option<String>,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    pub service_role: Option<String>,
    pub user_role: Option<String>,
    pub workspace_security_group_id: Option<String>,
    pub engine_security_group_id: Option<String>,
    pub studio_arn: String,
    pub url: String,
    pub creation_time: String,
    pub default_s3_location: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMappingView {
    pub studio_id: String,
    pub identity_id: String,
    pub identity_name: String,
    pub identity_type: String,
    pub session_policy_arn: String,
    pub creation_time: String,
    pub last_modified_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookExecutionView {
    pub notebook_execution_id: String,
    pub editor_id: String,
    pub execution_engine: serde_json::Value,
    pub notebook_execution_name: Option<String>,
    pub status: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub notebook_s3_location: Option<serde_json::Value>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentAppUiView {
    pub persistent_app_ui_id: String,
    pub status: String,
    pub creation_time: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&ClusterData> for ClusterView {
    fn from(c: &ClusterData) -> Self {
        ClusterView {
            id: c.id.clone(),
            name: c.name.clone(),
            status: c.status.clone(),
            creation_date_time: c.creation_date_time.to_rfc3339(),
            ready_date_time: c.ready_date_time.map(|dt| dt.to_rfc3339()),
            end_date_time: c.end_date_time.map(|dt| dt.to_rfc3339()),
            termination_protected: c.termination_protected,
            visible_to_all_users: c.visible_to_all_users,
            log_uri: c.log_uri.clone(),
            release_label: c.release_label.clone(),
            applications: c.applications.iter().map(ApplicationView::from).collect(),
            tags: c.tags.clone(),
            service_role: c.service_role.clone(),
            job_flow_role: c.job_flow_role.clone(),
            auto_scaling_role: c.auto_scaling_role.clone(),
            scale_down_behavior: c.scale_down_behavior.clone(),
            security_configuration: c.security_configuration.clone(),
            step_concurrency_level: c.step_concurrency_level,
            auto_termination_policy: c
                .auto_termination_policy
                .as_ref()
                .map(AutoTerminationPolicyView::from),
            managed_scaling_policy: c
                .managed_scaling_policy
                .as_ref()
                .map(ManagedScalingPolicyView::from),
            cluster_arn: c.cluster_arn.clone(),
            normalized_instance_hours: c.normalized_instance_hours,
            master_public_dns_name: c.master_public_dns_name.clone(),
            instance_groups: c
                .instance_groups
                .iter()
                .map(InstanceGroupView::from)
                .collect(),
            instance_fleets: c
                .instance_fleets
                .iter()
                .map(InstanceFleetView::from)
                .collect(),
            bootstrap_actions: c
                .bootstrap_actions
                .iter()
                .map(BootstrapActionView::from)
                .collect(),
        }
    }
}

impl From<&ApplicationData> for ApplicationView {
    fn from(a: &ApplicationData) -> Self {
        ApplicationView {
            name: a.name.clone(),
            version: a.version.clone(),
        }
    }
}

impl From<&AutoTerminationPolicyData> for AutoTerminationPolicyView {
    fn from(p: &AutoTerminationPolicyData) -> Self {
        AutoTerminationPolicyView {
            idle_timeout: p.idle_timeout,
        }
    }
}

impl From<&ManagedScalingPolicyData> for ManagedScalingPolicyView {
    fn from(p: &ManagedScalingPolicyData) -> Self {
        ManagedScalingPolicyView {
            compute_limits: p.compute_limits.as_ref().map(ComputeLimitsView::from),
            scaling_strategy: p.scaling_strategy.clone(),
            utilization_performance_index: p.utilization_performance_index,
        }
    }
}

impl From<&ComputeLimitsData> for ComputeLimitsView {
    fn from(c: &ComputeLimitsData) -> Self {
        ComputeLimitsView {
            minimum_capacity_units: c.minimum_capacity_units,
            maximum_capacity_units: c.maximum_capacity_units,
            maximum_on_demand_capacity_units: c.maximum_on_demand_capacity_units,
            maximum_core_capacity_units: c.maximum_core_capacity_units,
            unit_type: c.unit_type.clone(),
        }
    }
}

impl From<&InstanceGroupData> for InstanceGroupView {
    fn from(g: &InstanceGroupData) -> Self {
        InstanceGroupView {
            id: g.id.clone(),
            name: g.name.clone(),
            instance_type: g.instance_type.clone(),
            instance_role: g.instance_role.clone(),
            instance_count: g.instance_count,
            market: g.market.clone(),
            bid_price: g.bid_price.clone(),
            status: g.status.clone(),
            running_instance_count: g.running_instance_count,
            auto_scaling_policy: g
                .auto_scaling_policy
                .as_ref()
                .map(|p| AutoScalingPolicyView {
                    status: p.status.clone(),
                    constraints: p.constraints.as_ref().map(|c| ScalingConstraintsView {
                        min_capacity: c.min_capacity,
                        max_capacity: c.max_capacity,
                    }),
                    rules: p.rules.clone(),
                }),
        }
    }
}

impl From<&InstanceFleetData> for InstanceFleetView {
    fn from(f: &InstanceFleetData) -> Self {
        InstanceFleetView {
            id: f.id.clone(),
            name: f.name.clone(),
            instance_fleet_type: f.instance_fleet_type.clone(),
            target_on_demand_capacity: f.target_on_demand_capacity,
            target_spot_capacity: f.target_spot_capacity,
            status: f.status.clone(),
        }
    }
}

impl From<&BootstrapActionData> for BootstrapActionView {
    fn from(b: &BootstrapActionData) -> Self {
        BootstrapActionView {
            name: b.name.clone(),
            script_path: b.script_path.clone(),
            args: b.args.clone(),
        }
    }
}

impl From<&StepData> for StepView {
    fn from(s: &StepData) -> Self {
        StepView {
            id: s.id.clone(),
            name: s.name.clone(),
            cluster_id: s.cluster_id.clone(),
            status: s.status.clone(),
            creation_date_time: s.creation_date_time.to_rfc3339(),
            start_date_time: s.start_date_time.map(|dt| dt.to_rfc3339()),
            end_date_time: s.end_date_time.map(|dt| dt.to_rfc3339()),
            jar: s.jar.clone(),
            args: s.args.clone(),
            main_class: s.main_class.clone(),
            action_on_failure: s.action_on_failure.clone(),
        }
    }
}

impl From<&SecurityConfigurationData> for SecurityConfigurationView {
    fn from(sc: &SecurityConfigurationData) -> Self {
        SecurityConfigurationView {
            name: sc.name.clone(),
            security_configuration: sc.security_configuration.clone(),
            creation_date_time: sc.creation_date_time.to_rfc3339(),
        }
    }
}

impl From<&BlockPublicAccessConfigurationData> for BlockPublicAccessConfigView {
    fn from(bpac: &BlockPublicAccessConfigurationData) -> Self {
        BlockPublicAccessConfigView {
            block_public_security_group_rules: bpac.block_public_security_group_rules,
            permitted_ranges: bpac
                .permitted_ranges
                .iter()
                .map(|r| PortRangeView {
                    min_range: r.min_range,
                    max_range: r.max_range,
                })
                .collect(),
            creation_date_time: bpac.creation_date_time.to_rfc3339(),
            created_by_arn: bpac.created_by_arn.clone(),
        }
    }
}

impl From<&StudioData> for StudioView {
    fn from(s: &StudioData) -> Self {
        StudioView {
            studio_id: s.studio_id.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            auth_mode: s.auth_mode.clone(),
            vpc_id: s.vpc_id.clone(),
            subnet_ids: s.subnet_ids.clone(),
            service_role: s.service_role.clone(),
            user_role: s.user_role.clone(),
            workspace_security_group_id: s.workspace_security_group_id.clone(),
            engine_security_group_id: s.engine_security_group_id.clone(),
            studio_arn: s.studio_arn.clone(),
            url: s.url.clone(),
            creation_time: s.creation_time.to_rfc3339(),
            default_s3_location: s.default_s3_location.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<&SessionMappingData> for SessionMappingView {
    fn from(m: &SessionMappingData) -> Self {
        SessionMappingView {
            studio_id: m.studio_id.clone(),
            identity_id: m.identity_id.clone(),
            identity_name: m.identity_name.clone(),
            identity_type: m.identity_type.clone(),
            session_policy_arn: m.session_policy_arn.clone(),
            creation_time: m.creation_time.to_rfc3339(),
            last_modified_time: m.last_modified_time.to_rfc3339(),
        }
    }
}

impl From<&NotebookExecutionData> for NotebookExecutionView {
    fn from(e: &NotebookExecutionData) -> Self {
        NotebookExecutionView {
            notebook_execution_id: e.notebook_execution_id.clone(),
            editor_id: e.editor_id.clone(),
            execution_engine: e.execution_engine.clone(),
            notebook_execution_name: e.notebook_execution_name.clone(),
            status: e.status.clone(),
            start_time: e.start_time.to_rfc3339(),
            end_time: e.end_time.map(|dt| dt.to_rfc3339()),
            notebook_s3_location: e.notebook_s3_location.clone(),
            tags: e.tags.clone(),
        }
    }
}

impl From<&PersistentAppUiData> for PersistentAppUiView {
    fn from(u: &PersistentAppUiData) -> Self {
        PersistentAppUiView {
            persistent_app_ui_id: u.persistent_app_ui_id.clone(),
            status: u.status.clone(),
            creation_time: u.creation_time.to_rfc3339(),
        }
    }
}

// ---------------------------------------------------------------------------
// Into conversions (View -> State)
// ---------------------------------------------------------------------------

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now)
}

fn parse_opt_dt(s: Option<&str>) -> Option<DateTime<Utc>> {
    s.and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
}

impl From<ClusterView> for ClusterData {
    fn from(v: ClusterView) -> Self {
        ClusterData {
            id: v.id,
            name: v.name,
            status: v.status,
            creation_date_time: parse_dt(&v.creation_date_time),
            ready_date_time: parse_opt_dt(v.ready_date_time.as_deref()),
            end_date_time: parse_opt_dt(v.end_date_time.as_deref()),
            termination_protected: v.termination_protected,
            visible_to_all_users: v.visible_to_all_users,
            log_uri: v.log_uri,
            release_label: v.release_label,
            applications: v
                .applications
                .into_iter()
                .map(|a| ApplicationData {
                    name: a.name,
                    version: a.version,
                })
                .collect(),
            tags: v.tags,
            service_role: v.service_role,
            job_flow_role: v.job_flow_role,
            auto_scaling_role: v.auto_scaling_role,
            scale_down_behavior: v.scale_down_behavior,
            security_configuration: v.security_configuration,
            step_concurrency_level: v.step_concurrency_level,
            auto_termination_policy: v
                .auto_termination_policy
                .map(|p| AutoTerminationPolicyData {
                    idle_timeout: p.idle_timeout,
                }),
            managed_scaling_policy: v.managed_scaling_policy.map(|p| ManagedScalingPolicyData {
                compute_limits: p.compute_limits.map(|c| ComputeLimitsData {
                    minimum_capacity_units: c.minimum_capacity_units,
                    maximum_capacity_units: c.maximum_capacity_units,
                    maximum_on_demand_capacity_units: c.maximum_on_demand_capacity_units,
                    maximum_core_capacity_units: c.maximum_core_capacity_units,
                    unit_type: c.unit_type,
                }),
                scaling_strategy: p.scaling_strategy,
                utilization_performance_index: p.utilization_performance_index,
            }),
            cluster_arn: v.cluster_arn,
            normalized_instance_hours: v.normalized_instance_hours,
            master_public_dns_name: v.master_public_dns_name,
            instance_groups: v
                .instance_groups
                .into_iter()
                .map(|g| InstanceGroupData {
                    id: g.id,
                    name: g.name,
                    instance_type: g.instance_type,
                    instance_role: g.instance_role,
                    instance_count: g.instance_count,
                    market: g.market,
                    bid_price: g.bid_price,
                    status: g.status,
                    running_instance_count: g.running_instance_count,
                    auto_scaling_policy: g.auto_scaling_policy.map(|p| AutoScalingPolicyData {
                        status: p.status,
                        constraints: p.constraints.map(|c| ScalingConstraintsData {
                            min_capacity: c.min_capacity,
                            max_capacity: c.max_capacity,
                        }),
                        rules: p.rules,
                    }),
                })
                .collect(),
            instance_fleets: v
                .instance_fleets
                .into_iter()
                .map(|f| InstanceFleetData {
                    id: f.id,
                    name: f.name,
                    instance_fleet_type: f.instance_fleet_type,
                    target_on_demand_capacity: f.target_on_demand_capacity,
                    target_spot_capacity: f.target_spot_capacity,
                    status: f.status,
                })
                .collect(),
            bootstrap_actions: v
                .bootstrap_actions
                .into_iter()
                .map(|b| BootstrapActionData {
                    name: b.name,
                    script_path: b.script_path,
                    args: b.args,
                })
                .collect(),
        }
    }
}

impl From<StepView> for StepData {
    fn from(v: StepView) -> Self {
        StepData {
            id: v.id,
            name: v.name,
            cluster_id: v.cluster_id,
            status: v.status,
            creation_date_time: parse_dt(&v.creation_date_time),
            start_date_time: parse_opt_dt(v.start_date_time.as_deref()),
            end_date_time: parse_opt_dt(v.end_date_time.as_deref()),
            jar: v.jar,
            args: v.args,
            main_class: v.main_class,
            action_on_failure: v.action_on_failure,
        }
    }
}

impl From<SecurityConfigurationView> for SecurityConfigurationData {
    fn from(v: SecurityConfigurationView) -> Self {
        SecurityConfigurationData {
            name: v.name,
            security_configuration: v.security_configuration,
            creation_date_time: parse_dt(&v.creation_date_time),
        }
    }
}

impl From<StudioView> for StudioData {
    fn from(v: StudioView) -> Self {
        StudioData {
            studio_id: v.studio_id,
            name: v.name,
            description: v.description,
            auth_mode: v.auth_mode,
            vpc_id: v.vpc_id,
            subnet_ids: v.subnet_ids,
            service_role: v.service_role,
            user_role: v.user_role,
            workspace_security_group_id: v.workspace_security_group_id,
            engine_security_group_id: v.engine_security_group_id,
            studio_arn: v.studio_arn,
            url: v.url,
            creation_time: parse_dt(&v.creation_time),
            default_s3_location: v.default_s3_location,
            tags: v.tags,
        }
    }
}

impl From<SessionMappingView> for SessionMappingData {
    fn from(v: SessionMappingView) -> Self {
        SessionMappingData {
            studio_id: v.studio_id,
            identity_id: v.identity_id,
            identity_name: v.identity_name,
            identity_type: v.identity_type,
            session_policy_arn: v.session_policy_arn,
            creation_time: parse_dt(&v.creation_time),
            last_modified_time: parse_dt(&v.last_modified_time),
        }
    }
}

impl From<NotebookExecutionView> for NotebookExecutionData {
    fn from(v: NotebookExecutionView) -> Self {
        NotebookExecutionData {
            notebook_execution_id: v.notebook_execution_id,
            editor_id: v.editor_id,
            execution_engine: v.execution_engine,
            notebook_execution_name: v.notebook_execution_name,
            status: v.status,
            start_time: parse_dt(&v.start_time),
            end_time: parse_opt_dt(v.end_time.as_deref()),
            notebook_s3_location: v.notebook_s3_location,
            tags: v.tags,
        }
    }
}

impl From<PersistentAppUiView> for PersistentAppUiData {
    fn from(v: PersistentAppUiView) -> Self {
        PersistentAppUiData {
            persistent_app_ui_id: v.persistent_app_ui_id,
            status: v.status,
            creation_time: parse_dt(&v.creation_time),
        }
    }
}

impl From<BlockPublicAccessConfigView> for BlockPublicAccessConfigurationData {
    fn from(v: BlockPublicAccessConfigView) -> Self {
        BlockPublicAccessConfigurationData {
            block_public_security_group_rules: v.block_public_security_group_rules,
            permitted_ranges: v
                .permitted_ranges
                .into_iter()
                .map(|r| PortRangeData {
                    min_range: r.min_range,
                    max_range: r.max_range,
                })
                .collect(),
            creation_date_time: parse_dt(&v.creation_date_time),
            created_by_arn: v.created_by_arn,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for EmrService {
    type StateView = EmrStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let clusters = guard
            .clusters
            .iter()
            .map(|(k, v)| (k.clone(), ClusterView::from(v)))
            .collect();

        let steps = guard
            .steps
            .iter()
            .map(|(cid, sv)| (cid.clone(), sv.iter().map(StepView::from).collect()))
            .collect();

        let security_configurations = guard
            .security_configurations
            .iter()
            .map(|(k, v)| (k.clone(), SecurityConfigurationView::from(v)))
            .collect();

        let block_public_access_config = guard
            .block_public_access_config
            .as_ref()
            .map(BlockPublicAccessConfigView::from);

        let studios = guard
            .studios
            .iter()
            .map(|(k, v)| (k.clone(), StudioView::from(v)))
            .collect();

        let session_mappings = guard
            .session_mappings
            .iter()
            .map(SessionMappingView::from)
            .collect();

        let notebook_executions = guard
            .notebook_executions
            .iter()
            .map(|(k, v)| (k.clone(), NotebookExecutionView::from(v)))
            .collect();

        let persistent_app_uis = guard
            .persistent_app_uis
            .iter()
            .map(|(k, v)| (k.clone(), PersistentAppUiView::from(v)))
            .collect();

        EmrStateView {
            clusters,
            steps,
            security_configurations,
            block_public_access_config,
            studios,
            session_mappings,
            notebook_executions,
            persistent_app_uis,
        }
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
            *guard = EmrState {
                clusters: view
                    .clusters
                    .into_values()
                    .map(|cv| {
                        let id = cv.id.clone();
                        (id, ClusterData::from(cv))
                    })
                    .collect(),
                steps: view
                    .steps
                    .into_iter()
                    .map(|(k, sv)| (k, sv.into_iter().map(StepData::from).collect()))
                    .collect(),
                security_configurations: view
                    .security_configurations
                    .into_values()
                    .map(|scv| {
                        let name = scv.name.clone();
                        (name, SecurityConfigurationData::from(scv))
                    })
                    .collect(),
                block_public_access_config: view
                    .block_public_access_config
                    .map(BlockPublicAccessConfigurationData::from),
                studios: view
                    .studios
                    .into_iter()
                    .map(|(k, v)| (k, StudioData::from(v)))
                    .collect(),
                session_mappings: view
                    .session_mappings
                    .into_iter()
                    .map(SessionMappingData::from)
                    .collect(),
                notebook_executions: view
                    .notebook_executions
                    .into_iter()
                    .map(|(k, v)| (k, NotebookExecutionData::from(v)))
                    .collect(),
                persistent_app_uis: view
                    .persistent_app_uis
                    .into_iter()
                    .map(|(k, v)| (k, PersistentAppUiData::from(v)))
                    .collect(),
            };
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
            for cv in view.clusters.into_values() {
                let id = cv.id.clone();
                guard.clusters.insert(id, ClusterData::from(cv));
            }
            for (k, sv) in view.steps {
                guard
                    .steps
                    .entry(k)
                    .or_default()
                    .extend(sv.into_iter().map(StepData::from));
            }
            for scv in view.security_configurations.into_values() {
                let name = scv.name.clone();
                guard
                    .security_configurations
                    .insert(name, SecurityConfigurationData::from(scv));
            }
            if let Some(bpac) = view.block_public_access_config {
                guard.block_public_access_config =
                    Some(BlockPublicAccessConfigurationData::from(bpac));
            }
            for (k, v) in view.studios {
                guard.studios.insert(k, StudioData::from(v));
            }
            guard.session_mappings.extend(
                view.session_mappings
                    .into_iter()
                    .map(SessionMappingData::from),
            );
            for (k, v) in view.notebook_executions {
                guard
                    .notebook_executions
                    .insert(k, NotebookExecutionData::from(v));
            }
            for (k, v) in view.persistent_app_uis {
                guard
                    .persistent_app_uis
                    .insert(k, PersistentAppUiData::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
