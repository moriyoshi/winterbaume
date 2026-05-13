//! HTTP dispatch handler for EMR (awsJson1_1 protocol).

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, json_error_response,
};

use crate::state::{EmrError, EmrState};
use crate::types::{
    ApplicationData, AutoScalingPolicyData, AutoTerminationPolicyData, BootstrapActionData,
    CLUSTER_STATUS_RUNNING, ComputeLimitsData, InstanceFleetData, InstanceGroupData,
    ManagedScalingPolicyData, PortRangeData, STEP_STATUS_PENDING, ScalingConstraintsData, StepData,
};
use crate::views::EmrStateView;
use crate::wire;
use crate::wire::*;

pub struct EmrService {
    pub(crate) state: Arc<BackendState<EmrState>>,
    pub(crate) notifier: StateChangeNotifier<EmrStateView>,
}

impl EmrService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for EmrService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EmrService {
    fn service_name(&self) -> &str {
        "elasticmapreduce"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://(.+)\.elasticmapreduce\.amazonaws\.com",
            r"https?://elasticmapreduce\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

async fn emr_error_response(e: &EmrError) -> MockResponse {
    let (status, error_type) = match e {
        EmrError::ClusterNotFound(_) => (400, "InvalidRequestException"),
        EmrError::StepNotFound(_, _) => (400, "InvalidRequestException"),
        EmrError::SecurityConfigurationAlreadyExists(_) => (400, "InvalidRequestException"),
        EmrError::SecurityConfigurationNotFound(_) => (400, "InvalidRequestException"),
        EmrError::ResourceNotFound(_) => (400, "InvalidRequestException"),
        EmrError::StudioNotFound(_) => (400, "InvalidRequestException"),
        EmrError::InstanceGroupNotFound(_, _) => (400, "InvalidRequestException"),
        EmrError::NotebookExecutionNotFound(_) => (400, "InvalidRequestException"),
        EmrError::SessionMappingNotFound => (400, "InvalidRequestException"),
    };
    json_error_response(status, error_type, &e.to_string())
}

impl EmrService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation. An empty body is treated as `{}`.
        let body_bytes: &[u8] = if request.body.is_empty() {
            b"{}"
        } else {
            if serde_json::from_slice::<Value>(&request.body).is_err() {
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }
            &request.body
        };

        let state = self.state.get(account_id, &region);

        let mutating = matches!(
            action.as_str(),
            "RunJobFlow"
                | "TerminateJobFlows"
                | "AddJobFlowSteps"
                | "CancelSteps"
                | "AddInstanceGroups"
                | "ModifyInstanceGroups"
                | "AddInstanceFleet"
                | "ModifyInstanceFleet"
                | "CreateSecurityConfiguration"
                | "DeleteSecurityConfiguration"
                | "PutManagedScalingPolicy"
                | "RemoveManagedScalingPolicy"
                | "PutAutoTerminationPolicy"
                | "RemoveAutoTerminationPolicy"
                | "PutBlockPublicAccessConfiguration"
                | "AddTags"
                | "RemoveTags"
                | "SetTerminationProtection"
                | "SetVisibleToAllUsers"
                | "ModifyCluster"
                | "PutAutoScalingPolicy"
                | "RemoveAutoScalingPolicy"
                | "CreateStudio"
                | "DeleteStudio"
                | "UpdateStudio"
                | "CreateStudioSessionMapping"
                | "DeleteStudioSessionMapping"
                | "UpdateStudioSessionMapping"
                | "StartNotebookExecution"
                | "StopNotebookExecution"
                | "CreatePersistentAppUI"
        );

        let response = match action.as_str() {
            "RunJobFlow" => {
                self.handle_run_job_flow(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeCluster" => self.handle_describe_cluster(&state, body_bytes).await,
            "ListClusters" => self.handle_list_clusters(&state, body_bytes).await,
            "TerminateJobFlows" => self.handle_terminate_job_flows(&state, body_bytes).await,
            "AddJobFlowSteps" => self.handle_add_job_flow_steps(&state, body_bytes).await,
            "DescribeStep" => self.handle_describe_step(&state, body_bytes).await,
            "ListSteps" => self.handle_list_steps(&state, body_bytes).await,
            "CancelSteps" => self.handle_cancel_steps(&state, body_bytes).await,
            "AddInstanceGroups" => self.handle_add_instance_groups(&state, body_bytes).await,
            "ListInstanceGroups" => self.handle_list_instance_groups(&state, body_bytes).await,
            "ModifyInstanceGroups" => self.handle_modify_instance_groups(&state, body_bytes).await,
            "AddInstanceFleet" => self.handle_add_instance_fleet(&state, body_bytes).await,
            "ListInstanceFleets" => self.handle_list_instance_fleets(&state, body_bytes).await,
            "ModifyInstanceFleet" => self.handle_modify_instance_fleet(&state, body_bytes).await,
            "CreateSecurityConfiguration" => {
                self.handle_create_security_configuration(&state, body_bytes)
                    .await
            }
            "DescribeSecurityConfiguration" => {
                self.handle_describe_security_configuration(&state, body_bytes)
                    .await
            }
            "DeleteSecurityConfiguration" => {
                self.handle_delete_security_configuration(&state, body_bytes)
                    .await
            }
            "ListSecurityConfigurations" => self.handle_list_security_configurations(&state).await,
            "PutManagedScalingPolicy" => {
                self.handle_put_managed_scaling_policy(&state, body_bytes)
                    .await
            }
            "GetManagedScalingPolicy" => {
                self.handle_get_managed_scaling_policy(&state, body_bytes)
                    .await
            }
            "RemoveManagedScalingPolicy" => {
                self.handle_remove_managed_scaling_policy(&state, body_bytes)
                    .await
            }
            "PutAutoTerminationPolicy" => {
                self.handle_put_auto_termination_policy(&state, body_bytes)
                    .await
            }
            "GetAutoTerminationPolicy" => {
                self.handle_get_auto_termination_policy(&state, body_bytes)
                    .await
            }
            "RemoveAutoTerminationPolicy" => {
                self.handle_remove_auto_termination_policy(&state, body_bytes)
                    .await
            }
            "PutBlockPublicAccessConfiguration" => {
                self.handle_put_block_public_access_configuration(&state, body_bytes)
                    .await
            }
            "GetBlockPublicAccessConfiguration" => {
                self.handle_get_block_public_access_configuration(&state)
                    .await
            }
            "AddTags" => self.handle_add_tags(&state, body_bytes).await,
            "RemoveTags" => self.handle_remove_tags(&state, body_bytes).await,
            "ListBootstrapActions" => self.handle_list_bootstrap_actions(&state, body_bytes).await,
            "SetTerminationProtection" => {
                self.handle_set_termination_protection(&state, body_bytes)
                    .await
            }
            "SetVisibleToAllUsers" => {
                self.handle_set_visible_to_all_users(&state, body_bytes)
                    .await
            }
            "ModifyCluster" => self.handle_modify_cluster(&state, body_bytes).await,
            "DescribeJobFlows" => self.handle_describe_job_flows(&state, body_bytes).await,
            "ListInstances" => self.handle_list_instances(&state, body_bytes).await,
            "ListReleaseLabels" => self.handle_list_release_labels(body_bytes).await,
            "ListSupportedInstanceTypes" => {
                self.handle_list_supported_instance_types(body_bytes).await
            }
            "PutAutoScalingPolicy" => {
                self.handle_put_auto_scaling_policy(&state, body_bytes)
                    .await
            }
            "RemoveAutoScalingPolicy" => {
                self.handle_remove_auto_scaling_policy(&state, body_bytes)
                    .await
            }
            // EMR Studio CRUD
            "CreateStudio" => {
                self.handle_create_studio(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeStudio" => self.handle_describe_studio(&state, body_bytes).await,
            "DeleteStudio" => self.handle_delete_studio(&state, body_bytes).await,
            "UpdateStudio" => self.handle_update_studio(&state, body_bytes).await,
            "ListStudios" => self.handle_list_studios(&state).await,
            // Session Mapping CRUD
            "CreateStudioSessionMapping" => {
                self.handle_create_studio_session_mapping(&state, body_bytes)
                    .await
            }
            "GetStudioSessionMapping" => {
                self.handle_get_studio_session_mapping(&state, body_bytes)
                    .await
            }
            "UpdateStudioSessionMapping" => {
                self.handle_update_studio_session_mapping(&state, body_bytes)
                    .await
            }
            "DeleteStudioSessionMapping" => {
                self.handle_delete_studio_session_mapping(&state, body_bytes)
                    .await
            }
            "ListStudioSessionMappings" => {
                self.handle_list_studio_session_mappings(&state, body_bytes)
                    .await
            }
            // Notebook Execution
            "StartNotebookExecution" => {
                self.handle_start_notebook_execution(&state, body_bytes)
                    .await
            }
            "DescribeNotebookExecution" => {
                self.handle_describe_notebook_execution(&state, body_bytes)
                    .await
            }
            "StopNotebookExecution" => {
                self.handle_stop_notebook_execution(&state, body_bytes)
                    .await
            }
            "ListNotebookExecutions" => self.handle_list_notebook_executions(&state).await,
            // Persistent App UI
            "CreatePersistentAppUI" => self.handle_create_persistent_app_ui(&state).await,
            "DescribePersistentAppUI" => {
                self.handle_describe_persistent_app_ui(&state, body_bytes)
                    .await
            }
            // DescribeReleaseLabel returns static label metadata
            "DescribeReleaseLabel" => self.handle_describe_release_label(body_bytes).await,
            // Presigned URL and session credential stubs return synthetic values
            "GetClusterSessionCredentials" => MockResponse::json(200, "{}"),
            "GetOnClusterAppUIPresignedURL" => MockResponse::json(200, "{}"),
            "GetPersistentAppUIPresignedURL" => MockResponse::json(200, "{}"),
            // Simple set-flag operations
            "SetKeepJobFlowAliveWhenNoSteps" => {
                self.handle_set_keep_job_flow_alive(&state, body_bytes)
                    .await
            }
            "SetUnhealthyNodeReplacement" => MockResponse::json(200, "{}"),
            _ => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown operation: {action}"),
            ),
        };

        if mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ---- RunJobFlow ----

    async fn handle_run_job_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_run_job_flow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        } else {
            input.name
        };

        let log_uri = input.log_uri;
        let release_label = input.release_label;
        let service_role = input.service_role;
        let job_flow_role = input.job_flow_role;
        let auto_scaling_role = input.auto_scaling_role;
        let scale_down_behavior = input.scale_down_behavior;
        let security_configuration = input.security_configuration;
        let step_concurrency_level = input.step_concurrency_level;
        let termination_protected = input.instances.termination_protected.unwrap_or(false);
        let visible_to_all_users = input.visible_to_all_users.unwrap_or(true);

        // Parse applications
        let applications: Vec<ApplicationData> = input
            .applications
            .unwrap_or_default()
            .into_iter()
            .map(|a| ApplicationData {
                name: a.name.unwrap_or_default(),
                version: a.version,
            })
            .collect();

        // Parse tags
        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| t.key.map(|k| (k, t.value.unwrap_or_default())))
            .collect();

        // Parse initial steps
        let now = Utc::now();
        let cluster_id_placeholder = String::new(); // will be replaced
        let steps: Vec<StepData> = input
            .steps
            .unwrap_or_default()
            .iter()
            .filter_map(|s| serde_json::to_value(s).ok())
            .map(|s| {
                let step_id = format!(
                    "s-{}",
                    Uuid::new_v4()
                        .as_simple()
                        .to_string()
                        .to_uppercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .take(13)
                        .collect::<String>()
                );
                build_step_data(&s, &step_id, &cluster_id_placeholder, now)
            })
            .collect();

        // Parse auto termination policy
        let auto_termination_policy =
            input
                .auto_termination_policy
                .map(|p| AutoTerminationPolicyData {
                    idle_timeout: p.idle_timeout,
                });

        // Parse managed scaling policy
        let managed_scaling_policy = input
            .managed_scaling_policy
            .as_ref()
            .and_then(|p| serde_json::to_value(p).ok())
            .map(|v| parse_managed_scaling_policy(&v));

        // Parse instance groups
        let instance_groups: Vec<InstanceGroupData> = input
            .instances
            .instance_groups
            .clone()
            .unwrap_or_default()
            .iter()
            .filter_map(|g| serde_json::to_value(g).ok())
            .map(|v| parse_instance_group_config(&v))
            .collect();

        // Parse instance fleets
        let instance_fleets: Vec<InstanceFleetData> = input
            .instances
            .instance_fleets
            .clone()
            .unwrap_or_default()
            .iter()
            .filter_map(|f| serde_json::to_value(f).ok())
            .map(|v| parse_instance_fleet_config(&v))
            .collect();

        // Parse bootstrap actions
        let bootstrap_actions: Vec<BootstrapActionData> = input
            .bootstrap_actions
            .unwrap_or_default()
            .into_iter()
            .map(|ba| BootstrapActionData {
                name: ba.name,
                script_path: ba.script_bootstrap_action.path,
                args: ba.script_bootstrap_action.args.unwrap_or_default(),
            })
            .collect();

        let mut state_guard = state.write().await;
        let cluster_id = state_guard.run_job_flow(
            &name,
            log_uri,
            release_label,
            service_role,
            job_flow_role,
            auto_scaling_role,
            scale_down_behavior,
            security_configuration,
            step_concurrency_level,
            termination_protected,
            visible_to_all_users,
            applications,
            tags,
            steps,
            auto_termination_policy,
            managed_scaling_policy,
            instance_groups,
            instance_fleets,
            bootstrap_actions,
            account_id,
            region,
        );

        // Fix up step cluster IDs now that we have the cluster ID
        if let Some(cluster_steps) = state_guard.steps.get_mut(&cluster_id) {
            for step in cluster_steps.iter_mut() {
                step.cluster_id = cluster_id.clone();
            }
        }

        let cluster_arn = state_guard
            .clusters
            .get(&cluster_id)
            .map(|c| c.cluster_arn.clone())
            .unwrap_or_default();

        wire::serialize_run_job_flow_response(&RunJobFlowOutput {
            job_flow_id: Some(cluster_id),
            cluster_arn: Some(cluster_arn),
            ..Default::default()
        })
    }

    // ---- DescribeCluster ----

    async fn handle_describe_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let guard = state.read().await;
        match guard.describe_cluster(&cluster_id) {
            Ok(c) => {
                let cluster = build_cluster_wire(c);
                wire::serialize_describe_cluster_response(&DescribeClusterOutput {
                    cluster: Some(cluster),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ListClusters ----

    async fn handle_list_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_clusters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_states = input.cluster_states;

        let guard = state.read().await;
        let clusters = guard.list_clusters(cluster_states.as_deref());

        let summaries: Vec<ClusterSummary> = clusters
            .iter()
            .map(|c| ClusterSummary {
                id: Some(c.id.clone()),
                name: Some(c.name.clone()),
                cluster_arn: Some(c.cluster_arn.clone()),
                normalized_instance_hours: c.normalized_instance_hours,
                status: Some(ClusterStatus {
                    state: Some(c.status.clone()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_clusters_response(&ListClustersOutput {
            clusters: Some(summaries),
            ..Default::default()
        })
    }

    // ---- TerminateJobFlows ----

    async fn handle_terminate_job_flows(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_terminate_job_flows_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let job_flow_ids = input.job_flow_ids;

        let mut guard = state.write().await;
        guard.terminate_job_flows(&job_flow_ids);

        // TerminateJobFlows returns empty
        MockResponse::json(200, "{}")
    }

    // ---- AddJobFlowSteps ----

    async fn handle_add_job_flow_steps(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_job_flow_steps_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.job_flow_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'JobFlowId'");
        } else {
            input.job_flow_id
        };

        let steps_json: Vec<Value> = input
            .steps
            .iter()
            .filter_map(|s| serde_json::to_value(s).ok())
            .collect();

        let now = Utc::now();
        let steps: Vec<StepData> = steps_json
            .iter()
            .map(|s| {
                let step_id = format!(
                    "s-{}",
                    Uuid::new_v4()
                        .as_simple()
                        .to_string()
                        .to_uppercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .take(13)
                        .collect::<String>()
                );
                build_step_data(s, &step_id, &cluster_id, now)
            })
            .collect();

        let mut guard = state.write().await;
        match guard.add_job_flow_steps(&cluster_id, steps) {
            Ok(ids) => wire::serialize_add_job_flow_steps_response(&AddJobFlowStepsOutput {
                step_ids: Some(ids),
                ..Default::default()
            }),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- DescribeStep ----

    async fn handle_describe_step(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_step_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let step_id = if input.step_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StepId'");
        } else {
            input.step_id
        };

        let guard = state.read().await;
        match guard.describe_step(&cluster_id, &step_id) {
            Ok(s) => {
                let step = build_step_wire(s);
                wire::serialize_describe_step_response(&DescribeStepOutput {
                    step: Some(step),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ListSteps ----

    async fn handle_list_steps(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_steps_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let step_ids = input.step_ids;
        let step_states = input.step_states;

        let guard = state.read().await;
        match guard.list_steps(&cluster_id, step_ids.as_deref(), step_states.as_deref()) {
            Ok(steps) => {
                let step_summaries: Vec<StepSummary> =
                    steps.iter().map(|s| build_step_summary_wire(s)).collect();
                wire::serialize_list_steps_response(&ListStepsOutput {
                    steps: Some(step_summaries),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- CancelSteps ----

    async fn handle_cancel_steps(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_steps_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let step_ids = input.step_ids;

        let mut guard = state.write().await;
        match guard.cancel_steps(&cluster_id, &step_ids) {
            Ok(results) => {
                let cancel_infos: Vec<CancelStepsInfo> = results
                    .into_iter()
                    .map(|(id, status)| CancelStepsInfo {
                        step_id: Some(id),
                        status: Some(status.to_string()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_cancel_steps_response(&CancelStepsOutput {
                    cancel_steps_info_list: Some(cancel_infos),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- AddInstanceGroups ----

    async fn handle_add_instance_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_instance_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.job_flow_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'JobFlowId'");
        } else {
            input.job_flow_id
        };
        let groups: Vec<InstanceGroupData> = input
            .instance_groups
            .iter()
            .filter_map(|g| serde_json::to_value(g).ok())
            .map(|v| parse_instance_group_config(&v))
            .collect();

        let mut guard = state.write().await;
        let cluster_arn = guard
            .clusters
            .get(&cluster_id)
            .map(|c| c.cluster_arn.clone());
        match guard.add_instance_groups(&cluster_id, groups) {
            Ok(ids) => wire::serialize_add_instance_groups_response(&AddInstanceGroupsOutput {
                job_flow_id: Some(cluster_id),
                instance_group_ids: Some(ids),
                cluster_arn,
                ..Default::default()
            }),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ListInstanceGroups ----

    async fn handle_list_instance_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_instance_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let guard = state.read().await;
        match guard.list_instance_groups(&cluster_id) {
            Ok(groups) => {
                let ig_wire: Vec<InstanceGroup> =
                    groups.iter().map(build_instance_group_wire).collect();
                wire::serialize_list_instance_groups_response(&ListInstanceGroupsOutput {
                    instance_groups: Some(ig_wire),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ModifyInstanceGroups ----

    async fn handle_modify_instance_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_instance_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = match input.cluster_id {
            Some(id) if !id.is_empty() => id,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
            }
        };

        let modifications: Vec<(String, Option<i32>)> = input
            .instance_groups
            .unwrap_or_default()
            .into_iter()
            .map(|ig| (ig.instance_group_id, ig.instance_count))
            .collect();

        let mut guard = state.write().await;
        match guard.modify_instance_groups(&cluster_id, &modifications) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- AddInstanceFleet ----

    async fn handle_add_instance_fleet(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_instance_fleet_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let fleet_json = match serde_json::to_value(&input.instance_fleet) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(400, "ValidationException", "Missing 'InstanceFleet'");
            }
        };

        let fleet = parse_instance_fleet_config(&fleet_json);
        let fleet_id = fleet.id.clone();
        let cluster_arn = {
            let guard = state.read().await;
            guard
                .clusters
                .get(&cluster_id)
                .map(|c| c.cluster_arn.clone())
        };

        let mut guard = state.write().await;
        match guard.add_instance_fleet(&cluster_id, fleet) {
            Ok(_) => wire::serialize_add_instance_fleet_response(&AddInstanceFleetOutput {
                cluster_id: Some(cluster_id),
                instance_fleet_id: Some(fleet_id),
                cluster_arn,
                ..Default::default()
            }),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ListInstanceFleets ----

    async fn handle_list_instance_fleets(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_instance_fleets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let guard = state.read().await;
        match guard.list_instance_fleets(&cluster_id) {
            Ok(fleets) => {
                let fleet_wire: Vec<InstanceFleet> =
                    fleets.iter().map(build_instance_fleet_wire).collect();
                wire::serialize_list_instance_fleets_response(&ListInstanceFleetsOutput {
                    instance_fleets: Some(fleet_wire),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ModifyInstanceFleet ----

    async fn handle_modify_instance_fleet(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_instance_fleet_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let fleet_config = input.instance_fleet;
        let fleet_id = if fleet_config.instance_fleet_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'InstanceFleet.InstanceFleetId'",
            );
        } else {
            fleet_config.instance_fleet_id
        };
        let target_on_demand = fleet_config.target_on_demand_capacity;
        let target_spot = fleet_config.target_spot_capacity;

        let mut guard = state.write().await;
        match guard.modify_instance_fleet(&cluster_id, &fleet_id, target_on_demand, target_spot) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- CreateSecurityConfiguration ----

    async fn handle_create_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_security_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        } else {
            input.name
        };
        let security_configuration = if input.security_configuration.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'SecurityConfiguration'",
            );
        } else {
            input.security_configuration
        };

        let mut guard = state.write().await;
        match guard.create_security_configuration(&name, &security_configuration) {
            Ok(sc) => {
                let ts = sc.creation_date_time.timestamp() as f64;
                wire::serialize_create_security_configuration_response(
                    &CreateSecurityConfigurationOutput {
                        name: Some(sc.name.clone()),
                        creation_date_time: Some(ts),
                        ..Default::default()
                    },
                )
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- DescribeSecurityConfiguration ----

    async fn handle_describe_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_security_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        } else {
            input.name
        };

        let guard = state.read().await;
        match guard.describe_security_configuration(&name) {
            Ok(sc) => {
                let ts = sc.creation_date_time.timestamp() as f64;
                wire::serialize_describe_security_configuration_response(
                    &DescribeSecurityConfigurationOutput {
                        name: Some(sc.name.clone()),
                        security_configuration: Some(sc.security_configuration.clone()),
                        creation_date_time: Some(ts),
                        ..Default::default()
                    },
                )
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- DeleteSecurityConfiguration ----

    async fn handle_delete_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_security_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        } else {
            input.name
        };

        let mut guard = state.write().await;
        match guard.delete_security_configuration(&name) {
            Ok(()) => wire::serialize_delete_security_configuration_response(
                &DeleteSecurityConfigurationOutput {},
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ListSecurityConfigurations ----

    async fn handle_list_security_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let scs = guard.list_security_configurations();
        let summaries: Vec<SecurityConfigurationSummary> = scs
            .iter()
            .map(|sc| SecurityConfigurationSummary {
                name: Some(sc.name.clone()),
                creation_date_time: Some(sc.creation_date_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_security_configurations_response(&ListSecurityConfigurationsOutput {
            security_configurations: Some(summaries),
            ..Default::default()
        })
    }

    // ---- PutManagedScalingPolicy ----

    async fn handle_put_managed_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_managed_scaling_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let policy_json = match serde_json::to_value(&input.managed_scaling_policy) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ManagedScalingPolicy'",
                );
            }
        };
        let policy = parse_managed_scaling_policy(&policy_json);

        let mut guard = state.write().await;
        match guard.put_managed_scaling_policy(&cluster_id, policy) {
            Ok(()) => wire::serialize_put_managed_scaling_policy_response(
                &PutManagedScalingPolicyOutput {},
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- GetManagedScalingPolicy ----

    async fn handle_get_managed_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_managed_scaling_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let guard = state.read().await;
        match guard.get_managed_scaling_policy(&cluster_id) {
            Ok(policy_opt) => {
                let policy_wire = policy_opt.map(|p| ManagedScalingPolicy {
                    compute_limits: p.compute_limits.as_ref().map(|c| ComputeLimits {
                        minimum_capacity_units: c.minimum_capacity_units,
                        maximum_capacity_units: c.maximum_capacity_units,
                        maximum_on_demand_capacity_units: c.maximum_on_demand_capacity_units,
                        maximum_core_capacity_units: c.maximum_core_capacity_units,
                        unit_type: c.unit_type.clone(),
                        ..Default::default()
                    }),
                    scaling_strategy: p.scaling_strategy.clone(),
                    utilization_performance_index: p.utilization_performance_index,
                    ..Default::default()
                });
                wire::serialize_get_managed_scaling_policy_response(
                    &GetManagedScalingPolicyOutput {
                        managed_scaling_policy: policy_wire,
                        ..Default::default()
                    },
                )
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- RemoveManagedScalingPolicy ----

    async fn handle_remove_managed_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_managed_scaling_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let mut guard = state.write().await;
        match guard.remove_managed_scaling_policy(&cluster_id) {
            Ok(()) => wire::serialize_remove_managed_scaling_policy_response(
                &RemoveManagedScalingPolicyOutput {},
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- PutAutoTerminationPolicy ----

    async fn handle_put_auto_termination_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_auto_termination_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let policy = input
            .auto_termination_policy
            .map(|p| AutoTerminationPolicyData {
                idle_timeout: p.idle_timeout,
            });

        let mut guard = state.write().await;
        match guard.put_auto_termination_policy(&cluster_id, policy) {
            Ok(()) => wire::serialize_put_auto_termination_policy_response(
                &PutAutoTerminationPolicyOutput {},
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- GetAutoTerminationPolicy ----

    async fn handle_get_auto_termination_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_auto_termination_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let guard = state.read().await;
        match guard.get_auto_termination_policy(&cluster_id) {
            Ok(policy_opt) => {
                let policy_wire = policy_opt.map(|p| AutoTerminationPolicy {
                    idle_timeout: p.idle_timeout,
                    ..Default::default()
                });
                wire::serialize_get_auto_termination_policy_response(
                    &GetAutoTerminationPolicyOutput {
                        auto_termination_policy: policy_wire,
                        ..Default::default()
                    },
                )
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- RemoveAutoTerminationPolicy ----

    async fn handle_remove_auto_termination_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_auto_termination_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let mut guard = state.write().await;
        match guard.remove_auto_termination_policy(&cluster_id) {
            Ok(()) => wire::serialize_remove_auto_termination_policy_response(
                &RemoveAutoTerminationPolicyOutput {},
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- PutBlockPublicAccessConfiguration ----

    async fn handle_put_block_public_access_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_block_public_access_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let config = input.block_public_access_configuration;
        let block = config.block_public_security_group_rules;
        let permitted_ranges: Vec<PortRangeData> = config
            .permitted_public_security_group_rule_ranges
            .unwrap_or_default()
            .into_iter()
            .map(|r| PortRangeData {
                min_range: r.min_range,
                max_range: r.max_range,
            })
            .collect();

        let mut guard = state.write().await;
        guard.put_block_public_access_configuration(block, permitted_ranges);
        wire::serialize_put_block_public_access_configuration_response(
            &PutBlockPublicAccessConfigurationOutput {},
        )
    }

    // ---- GetBlockPublicAccessConfiguration ----

    async fn handle_get_block_public_access_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        // Return defaults if not configured
        let (block, ranges, creation_ts, created_by_arn) =
            if let Some(bpac) = &guard.block_public_access_config {
                (
                    bpac.block_public_security_group_rules,
                    bpac.permitted_ranges
                        .iter()
                        .map(|r| PortRange {
                            min_range: r.min_range,
                            max_range: r.max_range,
                            ..Default::default()
                        })
                        .collect::<Vec<_>>(),
                    Some(bpac.creation_date_time.timestamp() as f64),
                    bpac.created_by_arn.clone(),
                )
            } else {
                (true, vec![], None, None)
            };

        let config = BlockPublicAccessConfiguration {
            block_public_security_group_rules: block,
            permitted_public_security_group_rule_ranges: if ranges.is_empty() {
                None
            } else {
                Some(ranges)
            },
            ..Default::default()
        };
        let metadata = BlockPublicAccessConfigurationMetadata {
            creation_date_time: creation_ts,
            created_by_arn,
            ..Default::default()
        };
        wire::serialize_get_block_public_access_configuration_response(
            &GetBlockPublicAccessConfigurationOutput {
                block_public_access_configuration: Some(config),
                block_public_access_configuration_metadata: Some(metadata),
                ..Default::default()
            },
        )
    }

    // ---- AddTags ----

    async fn handle_add_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_id = if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        } else {
            input.resource_id
        };
        let tags: Vec<(String, String)> = input
            .tags
            .into_iter()
            .filter_map(|t| t.key.map(|k| (k, t.value.unwrap_or_default())))
            .collect();

        let mut guard = state.write().await;
        match guard.add_tags(&resource_id, tags) {
            Ok(()) => wire::serialize_add_tags_response(&AddTagsOutput {}),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- RemoveTags ----

    async fn handle_remove_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_id = if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        } else {
            input.resource_id
        };
        let tag_keys = input.tag_keys;

        let mut guard = state.write().await;
        match guard.remove_tags(&resource_id, &tag_keys) {
            Ok(()) => wire::serialize_remove_tags_response(&RemoveTagsOutput {}),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- ListBootstrapActions ----

    async fn handle_list_bootstrap_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_bootstrap_actions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let guard = state.read().await;
        match guard.list_bootstrap_actions(&cluster_id) {
            Ok(bas) => {
                let commands: Vec<Command> = bas
                    .iter()
                    .map(|ba| Command {
                        name: Some(ba.name.clone()),
                        script_path: Some(ba.script_path.clone()),
                        args: if ba.args.is_empty() {
                            None
                        } else {
                            Some(ba.args.clone())
                        },
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_bootstrap_actions_response(&ListBootstrapActionsOutput {
                    bootstrap_actions: Some(commands),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- SetTerminationProtection ----

    async fn handle_set_termination_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_termination_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let job_flow_ids = input.job_flow_ids;
        let protected = input.termination_protected;

        let mut guard = state.write().await;
        guard.set_termination_protection(&job_flow_ids, protected);
        MockResponse::json(200, "{}")
    }

    // ---- SetVisibleToAllUsers ----

    async fn handle_set_visible_to_all_users(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_visible_to_all_users_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let job_flow_ids = input.job_flow_ids;
        let visible = input.visible_to_all_users;

        let mut guard = state.write().await;
        guard.set_visible_to_all_users(&job_flow_ids, visible);
        MockResponse::json(200, "{}")
    }

    // ---- ModifyCluster ----

    async fn handle_modify_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let step_concurrency_level = input.step_concurrency_level;

        let mut guard = state.write().await;
        match guard.modify_cluster(&cluster_id, step_concurrency_level) {
            Ok(level) => wire::serialize_modify_cluster_response(&ModifyClusterOutput {
                step_concurrency_level: level,
                ..Default::default()
            }),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- DescribeJobFlows ----

    async fn handle_describe_job_flows(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_job_flows_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let job_flow_ids = input.job_flow_ids;

        let guard = state.read().await;
        let clusters: Vec<&crate::types::ClusterData> = guard
            .clusters
            .values()
            .filter(|c| {
                if let Some(ids) = &job_flow_ids {
                    if !ids.is_empty() {
                        return ids.iter().any(|id| id == &c.id);
                    }
                }
                true
            })
            .collect();

        let job_flows: Vec<JobFlowDetail> = clusters
            .iter()
            .map(|c| JobFlowDetail {
                job_flow_id: Some(c.id.clone()),
                name: Some(c.name.clone()),
                log_uri: c.log_uri.clone(),
                service_role: c.service_role.clone(),
                job_flow_role: c.job_flow_role.clone(),
                auto_scaling_role: c.auto_scaling_role.clone(),
                scale_down_behavior: c.scale_down_behavior.clone(),
                visible_to_all_users: Some(c.visible_to_all_users),
                execution_status_detail: Some(JobFlowExecutionStatusDetail {
                    state: Some(c.status.clone()),
                    creation_date_time: Some(c.creation_date_time.timestamp() as f64),
                    ready_date_time: c.ready_date_time.map(|dt| dt.timestamp() as f64),
                    end_date_time: c.end_date_time.map(|dt| dt.timestamp() as f64),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_job_flows_response(&DescribeJobFlowsOutput {
            job_flows: Some(job_flows),
            ..Default::default()
        })
    }

    // ---- ListInstances ----

    // Individual EC2 instance records are not modelled; returns an empty list
    // for a valid cluster (consistent with moto's behaviour).
    async fn handle_list_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_instances_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };

        let guard = state.read().await;
        if !guard.clusters.contains_key(&cluster_id) {
            return json_error_response(
                400,
                "InvalidRequestException",
                &format!("Cluster {cluster_id} does not exist."),
            );
        }

        // Return empty instances list (simplified implementation)
        wire::serialize_list_instances_response(&ListInstancesOutput {
            instances: Some(vec![]),
            ..Default::default()
        })
    }

    // ---- ListReleaseLabels ----

    // Release labels are a static catalogue; returns a representative set of
    // common EMR release labels.
    async fn handle_list_release_labels(&self, _body: &[u8]) -> MockResponse {
        // Return a set of common release labels
        let labels = vec![
            "emr-7.0.0".to_string(),
            "emr-6.15.0".to_string(),
            "emr-6.14.0".to_string(),
            "emr-6.13.0".to_string(),
            "emr-6.12.0".to_string(),
            "emr-5.36.0".to_string(),
        ];
        wire::serialize_list_release_labels_response(&ListReleaseLabelsOutput {
            release_labels: Some(labels),
            ..Default::default()
        })
    }

    // ---- ListSupportedInstanceTypes ----

    // Instance type catalogue is static; returns an empty list (no catalogue data
    // is shipped with winterbaume).
    async fn handle_list_supported_instance_types(&self, _body: &[u8]) -> MockResponse {
        wire::serialize_list_supported_instance_types_response(&ListSupportedInstanceTypesOutput {
            supported_instance_types: Some(vec![]),
            ..Default::default()
        })
    }

    // ---- PutAutoScalingPolicy ----

    async fn handle_put_auto_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_auto_scaling_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let instance_group_id = if input.instance_group_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceGroupId'");
        } else {
            input.instance_group_id
        };

        let policy_json = serde_json::to_value(&input.auto_scaling_policy).ok();
        let policy = parse_auto_scaling_policy(policy_json.as_ref());

        let mut guard = state.write().await;
        let cluster_arn = guard
            .clusters
            .get(&cluster_id)
            .map(|c| c.cluster_arn.clone());

        match guard.put_auto_scaling_policy(&cluster_id, &instance_group_id, policy) {
            Ok(()) => {
                wire::serialize_put_auto_scaling_policy_response(&PutAutoScalingPolicyOutput {
                    cluster_id: Some(cluster_id),
                    cluster_arn,
                    instance_group_id: Some(instance_group_id),
                    ..Default::default()
                })
            }
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- RemoveAutoScalingPolicy ----

    async fn handle_remove_auto_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_auto_scaling_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_id = if input.cluster_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClusterId'");
        } else {
            input.cluster_id
        };
        let instance_group_id = if input.instance_group_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceGroupId'");
        } else {
            input.instance_group_id
        };

        let mut guard = state.write().await;
        match guard.remove_auto_scaling_policy(&cluster_id, &instance_group_id) {
            Ok(()) => wire::serialize_remove_auto_scaling_policy_response(
                &RemoveAutoScalingPolicyOutput {},
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- Studio handlers ----

    async fn handle_create_studio(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_studio_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        } else {
            input.name
        };
        let auth_mode = if input.auth_mode.is_empty() {
            "IAM".to_string()
        } else {
            input.auth_mode
        };
        let description = input.description;
        let vpc_id = if input.vpc_id.is_empty() {
            None
        } else {
            Some(input.vpc_id)
        };
        let subnet_ids = input.subnet_ids;
        let service_role = if input.service_role.is_empty() {
            None
        } else {
            Some(input.service_role)
        };
        let user_role = input.user_role;
        let workspace_security_group_id = if input.workspace_security_group_id.is_empty() {
            None
        } else {
            Some(input.workspace_security_group_id)
        };
        let engine_security_group_id = if input.engine_security_group_id.is_empty() {
            None
        } else {
            Some(input.engine_security_group_id)
        };
        let default_s3_location = if input.default_s3_location.is_empty() {
            None
        } else {
            Some(input.default_s3_location)
        };
        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| t.key.map(|k| (k, t.value.unwrap_or_default())))
            .collect();

        let mut guard = state.write().await;
        let studio_id = guard.create_studio(
            &name,
            description,
            &auth_mode,
            vpc_id,
            subnet_ids,
            service_role,
            user_role,
            workspace_security_group_id,
            engine_security_group_id,
            default_s3_location,
            tags,
            account_id,
            region,
        );
        let url = guard
            .studios
            .get(&studio_id)
            .map(|s| s.url.clone())
            .unwrap_or_default();

        wire::serialize_create_studio_response(&wire::CreateStudioOutput {
            studio_id: Some(studio_id),
            url: Some(url),
        })
    }

    async fn handle_describe_studio(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_studio_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = if input.studio_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StudioId'");
        } else {
            input.studio_id
        };
        let guard = state.read().await;
        match guard.describe_studio(&studio_id) {
            Ok(s) => wire::serialize_describe_studio_response(&wire::DescribeStudioOutput {
                studio: Some(studio_to_model(s)),
            }),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_delete_studio(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_studio_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = if input.studio_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StudioId'");
        } else {
            input.studio_id
        };
        let mut guard = state.write().await;
        match guard.delete_studio(&studio_id) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_update_studio(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_studio_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = if input.studio_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StudioId'");
        } else {
            input.studio_id
        };
        let name = input.name.as_deref();
        let description = input.description.as_deref();
        let default_s3_location = input.default_s3_location.as_deref();

        let mut guard = state.write().await;
        match guard.update_studio(&studio_id, name, description, default_s3_location) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_list_studios(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let studios = guard.list_studios();
        let summaries = studios.iter().map(studio_summary_to_model).collect();
        wire::serialize_list_studios_response(&wire::ListStudiosOutput {
            studios: Some(summaries),
            ..Default::default()
        })
    }

    // ---- Session Mapping handlers ----

    async fn handle_create_studio_session_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_studio_session_mapping_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = if input.studio_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StudioId'");
        } else {
            input.studio_id
        };
        let identity_id = input.identity_id.unwrap_or_default();
        let identity_name = input.identity_name.unwrap_or_default();
        let identity_type = if input.identity_type.is_empty() {
            "USER".to_string()
        } else {
            input.identity_type
        };
        let session_policy_arn = input.session_policy_arn;

        let mut guard = state.write().await;
        match guard.create_studio_session_mapping(
            &studio_id,
            &identity_id,
            &identity_name,
            &identity_type,
            &session_policy_arn,
        ) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_get_studio_session_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_studio_session_mapping_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = if input.studio_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StudioId'");
        } else {
            input.studio_id
        };
        let identity_id = input.identity_id.unwrap_or_default();
        let identity_type = if input.identity_type.is_empty() {
            "USER".to_string()
        } else {
            input.identity_type
        };

        let guard = state.read().await;
        match guard.get_studio_session_mapping(&studio_id, &identity_id, &identity_type) {
            Ok(m) => wire::serialize_get_studio_session_mapping_response(
                &wire::GetStudioSessionMappingOutput {
                    session_mapping: Some(session_mapping_detail_to_model(m)),
                },
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_update_studio_session_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_studio_session_mapping_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = if input.studio_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StudioId'");
        } else {
            input.studio_id
        };
        let identity_id = input.identity_id.unwrap_or_default();
        let identity_type = if input.identity_type.is_empty() {
            "USER".to_string()
        } else {
            input.identity_type
        };
        let session_policy_arn = input.session_policy_arn;

        let mut guard = state.write().await;
        match guard.update_studio_session_mapping(
            &studio_id,
            &identity_id,
            &identity_type,
            &session_policy_arn,
        ) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_delete_studio_session_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_studio_session_mapping_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = if input.studio_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StudioId'");
        } else {
            input.studio_id
        };
        let identity_id = input.identity_id.unwrap_or_default();
        let identity_type = if input.identity_type.is_empty() {
            "USER".to_string()
        } else {
            input.identity_type
        };

        let mut guard = state.write().await;
        match guard.delete_studio_session_mapping(&studio_id, &identity_id, &identity_type) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_list_studio_session_mappings(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_studio_session_mappings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let studio_id = input.studio_id;
        let guard = state.read().await;
        let mappings = guard.list_studio_session_mappings(studio_id.as_deref());
        let summaries = mappings
            .iter()
            .map(session_mapping_summary_to_model)
            .collect();
        wire::serialize_list_studio_session_mappings_response(
            &wire::ListStudioSessionMappingsOutput {
                session_mappings: Some(summaries),
                ..Default::default()
            },
        )
    }

    // ---- Notebook Execution handlers ----

    async fn handle_start_notebook_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_notebook_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let editor_id = input.editor_id.unwrap_or_default();
        let execution_engine = serde_json::to_value(&input.execution_engine)
            .unwrap_or(Value::Object(Default::default()));
        let notebook_execution_name = input.notebook_execution_name;
        let notebook_s3_location = input
            .notebook_s3_location
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| t.key.map(|k| (k, t.value.unwrap_or_default())))
            .collect();

        let mut guard = state.write().await;
        let id = guard.start_notebook_execution(
            &editor_id,
            execution_engine,
            notebook_execution_name,
            notebook_s3_location,
            tags,
        );
        wire::serialize_start_notebook_execution_response(&wire::StartNotebookExecutionOutput {
            notebook_execution_id: Some(id),
        })
    }

    async fn handle_describe_notebook_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_notebook_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let exec_id = if input.notebook_execution_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'NotebookExecutionId'",
            );
        } else {
            input.notebook_execution_id
        };
        let guard = state.read().await;
        match guard.describe_notebook_execution(&exec_id) {
            Ok(e) => wire::serialize_describe_notebook_execution_response(
                &wire::DescribeNotebookExecutionOutput {
                    notebook_execution: Some(notebook_execution_to_model(e)),
                },
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_stop_notebook_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_notebook_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let exec_id = if input.notebook_execution_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'NotebookExecutionId'",
            );
        } else {
            input.notebook_execution_id
        };
        let mut guard = state.write().await;
        match guard.stop_notebook_execution(&exec_id) {
            Ok(()) => MockResponse::json(200, "{}"),
            Err(e) => emr_error_response(&e).await,
        }
    }

    async fn handle_list_notebook_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let execs = guard.list_notebook_executions();
        let summaries = execs
            .iter()
            .map(notebook_execution_summary_to_model)
            .collect();
        wire::serialize_list_notebook_executions_response(&wire::ListNotebookExecutionsOutput {
            notebook_executions: Some(summaries),
            ..Default::default()
        })
    }

    // ---- Persistent App UI handlers ----

    async fn handle_create_persistent_app_ui(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
    ) -> MockResponse {
        let mut guard = state.write().await;
        let ui_id = guard.create_persistent_app_ui();
        wire::serialize_create_persistent_app_u_i_response(&wire::CreatePersistentAppUIOutput {
            persistent_app_u_i_id: Some(ui_id),
            ..Default::default()
        })
    }

    async fn handle_describe_persistent_app_ui(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_persistent_app_u_i_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let ui_id = if input.persistent_app_u_i_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PersistentAppUIId'");
        } else {
            input.persistent_app_u_i_id
        };
        let guard = state.read().await;
        match guard.describe_persistent_app_ui(&ui_id) {
            Ok(u) => wire::serialize_describe_persistent_app_u_i_response(
                &wire::DescribePersistentAppUIOutput {
                    persistent_app_u_i: Some(persistent_app_ui_to_model(u)),
                },
            ),
            Err(e) => emr_error_response(&e).await,
        }
    }

    // ---- DescribeReleaseLabel ----

    async fn handle_describe_release_label(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_release_label_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let release_label = input
            .release_label
            .unwrap_or_else(|| "emr-7.0.0".to_string());
        wire::serialize_describe_release_label_response(&wire::DescribeReleaseLabelOutput {
            release_label: Some(release_label),
            applications: Some(Vec::new()),
            available_o_s_releases: Some(Vec::new()),
            ..Default::default()
        })
    }

    // ---- SetKeepJobFlowAliveWhenNoSteps ----

    async fn handle_set_keep_job_flow_alive(
        &self,
        _state: &Arc<tokio::sync::RwLock<EmrState>>,
        _body: &[u8],
    ) -> MockResponse {
        // This flag is accepted but not tracked in cluster state beyond the
        // existing termination protection flag.
        MockResponse::json(200, "{}")
    }
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

fn build_step_data(
    s: &Value,
    step_id: &str,
    cluster_id: &str,
    now: chrono::DateTime<Utc>,
) -> StepData {
    let name = s
        .get("Name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let action_on_failure = s
        .get("ActionOnFailure")
        .and_then(|v| v.as_str())
        .map(String::from);
    let jar = s
        .get("HadoopJarStep")
        .and_then(|h| h.get("Jar"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let args = s
        .get("HadoopJarStep")
        .and_then(|h| h.get("Args"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|a| a.as_str())
                .map(String::from)
                .collect::<Vec<_>>()
        });
    let main_class = s
        .get("HadoopJarStep")
        .and_then(|h| h.get("MainClass"))
        .and_then(|v| v.as_str())
        .map(String::from);

    StepData {
        id: step_id.to_string(),
        name,
        cluster_id: cluster_id.to_string(),
        status: STEP_STATUS_PENDING.to_string(),
        creation_date_time: now,
        start_date_time: None,
        end_date_time: None,
        jar,
        args,
        main_class,
        action_on_failure,
    }
}

fn build_cluster_wire(c: &crate::types::ClusterData) -> Cluster {
    let tags: Vec<Tag> = c
        .tags
        .iter()
        .map(|(k, v)| Tag {
            key: Some(k.clone()),
            value: Some(v.clone()),
            ..Default::default()
        })
        .collect();

    let applications: Vec<Application> = c
        .applications
        .iter()
        .map(|a| Application {
            name: Some(a.name.clone()),
            version: a.version.clone(),
            ..Default::default()
        })
        .collect();

    Cluster {
        id: Some(c.id.clone()),
        name: Some(c.name.clone()),
        cluster_arn: Some(c.cluster_arn.clone()),
        status: Some(ClusterStatus {
            state: Some(c.status.clone()),
            timeline: Some(ClusterTimeline {
                creation_date_time: Some(c.creation_date_time.timestamp() as f64),
                ready_date_time: c.ready_date_time.map(|dt| dt.timestamp() as f64),
                end_date_time: c.end_date_time.map(|dt| dt.timestamp() as f64),
                ..Default::default()
            }),
            ..Default::default()
        }),
        termination_protected: Some(c.termination_protected),
        visible_to_all_users: Some(c.visible_to_all_users),
        log_uri: c.log_uri.clone(),
        release_label: c.release_label.clone(),
        service_role: c.service_role.clone(),
        auto_scaling_role: c.auto_scaling_role.clone(),
        scale_down_behavior: c.scale_down_behavior.clone(),
        security_configuration: c.security_configuration.clone(),
        step_concurrency_level: c.step_concurrency_level,
        normalized_instance_hours: c.normalized_instance_hours,
        master_public_dns_name: c.master_public_dns_name.clone(),
        tags: if tags.is_empty() { None } else { Some(tags) },
        applications: if applications.is_empty() {
            None
        } else {
            Some(applications)
        },
        ..Default::default()
    }
}

fn studio_to_model(s: &crate::types::StudioData) -> wire::Studio {
    wire::Studio {
        studio_id: Some(s.studio_id.clone()),
        studio_arn: Some(s.studio_arn.clone()),
        name: Some(s.name.clone()),
        description: s.description.clone(),
        auth_mode: Some(s.auth_mode.clone()),
        vpc_id: s.vpc_id.clone(),
        subnet_ids: Some(s.subnet_ids.clone()),
        service_role: s.service_role.clone(),
        user_role: s.user_role.clone(),
        workspace_security_group_id: s.workspace_security_group_id.clone(),
        engine_security_group_id: s.engine_security_group_id.clone(),
        url: Some(s.url.clone()),
        creation_time: Some(s.creation_time.timestamp() as f64),
        default_s3_location: s.default_s3_location.clone(),
        tags: Some(tags_to_model(&s.tags)),
        ..Default::default()
    }
}

fn studio_summary_to_model(s: &&crate::types::StudioData) -> wire::StudioSummary {
    wire::StudioSummary {
        studio_id: Some(s.studio_id.clone()),
        name: Some(s.name.clone()),
        auth_mode: Some(s.auth_mode.clone()),
        creation_time: Some(s.creation_time.timestamp() as f64),
        url: Some(s.url.clone()),
        vpc_id: s.vpc_id.clone(),
        description: s.description.clone(),
    }
}

fn session_mapping_detail_to_model(
    mapping: &crate::types::SessionMappingData,
) -> wire::SessionMappingDetail {
    wire::SessionMappingDetail {
        studio_id: Some(mapping.studio_id.clone()),
        identity_id: Some(mapping.identity_id.clone()),
        identity_name: Some(mapping.identity_name.clone()),
        identity_type: Some(mapping.identity_type.clone()),
        session_policy_arn: Some(mapping.session_policy_arn.clone()),
        creation_time: Some(mapping.creation_time.timestamp() as f64),
        last_modified_time: Some(mapping.last_modified_time.timestamp() as f64),
    }
}

fn session_mapping_summary_to_model(
    mapping: &&crate::types::SessionMappingData,
) -> wire::SessionMappingSummary {
    wire::SessionMappingSummary {
        studio_id: Some(mapping.studio_id.clone()),
        identity_id: Some(mapping.identity_id.clone()),
        identity_name: Some(mapping.identity_name.clone()),
        identity_type: Some(mapping.identity_type.clone()),
        session_policy_arn: Some(mapping.session_policy_arn.clone()),
        creation_time: Some(mapping.creation_time.timestamp() as f64),
    }
}

fn notebook_execution_to_model(
    execution: &crate::types::NotebookExecutionData,
) -> wire::NotebookExecution {
    wire::NotebookExecution {
        notebook_execution_id: Some(execution.notebook_execution_id.clone()),
        editor_id: Some(execution.editor_id.clone()),
        execution_engine: serde_json::from_value(execution.execution_engine.clone()).ok(),
        notebook_execution_name: execution.notebook_execution_name.clone(),
        status: Some(execution.status.clone()),
        start_time: Some(execution.start_time.timestamp() as f64),
        end_time: execution.end_time.map(|dt| dt.timestamp() as f64),
        notebook_s3_location: execution
            .notebook_s3_location
            .clone()
            .and_then(|v| serde_json::from_value(v).ok()),
        tags: Some(tags_to_model(&execution.tags)),
        ..Default::default()
    }
}

fn notebook_execution_summary_to_model(
    execution: &&crate::types::NotebookExecutionData,
) -> wire::NotebookExecutionSummary {
    wire::NotebookExecutionSummary {
        notebook_execution_id: Some(execution.notebook_execution_id.clone()),
        editor_id: Some(execution.editor_id.clone()),
        notebook_execution_name: execution.notebook_execution_name.clone(),
        status: Some(execution.status.clone()),
        start_time: Some(execution.start_time.timestamp() as f64),
        end_time: execution.end_time.map(|dt| dt.timestamp() as f64),
        notebook_s3_location: execution
            .notebook_s3_location
            .clone()
            .and_then(|v| serde_json::from_value(v).ok()),
        ..Default::default()
    }
}

fn persistent_app_ui_to_model(ui: &crate::types::PersistentAppUiData) -> wire::PersistentAppUI {
    wire::PersistentAppUI {
        persistent_app_u_i_id: Some(ui.persistent_app_ui_id.clone()),
        persistent_app_u_i_status: Some(ui.status.clone()),
        creation_time: Some(ui.creation_time.timestamp() as f64),
        ..Default::default()
    }
}

fn tags_to_model(tags: &HashMap<String, String>) -> Vec<Tag> {
    tags.iter()
        .map(|(k, v)| Tag {
            key: Some(k.clone()),
            value: Some(v.clone()),
            ..Default::default()
        })
        .collect()
}

fn build_step_wire(s: &StepData) -> Step {
    Step {
        id: Some(s.id.clone()),
        name: Some(s.name.clone()),
        action_on_failure: s.action_on_failure.clone(),
        config: Some(HadoopStepConfig {
            jar: Some(s.jar.clone()),
            args: s.args.clone(),
            main_class: s.main_class.clone(),
            ..Default::default()
        }),
        status: Some(StepStatus {
            state: Some(s.status.clone()),
            timeline: Some(StepTimeline {
                creation_date_time: Some(s.creation_date_time.timestamp() as f64),
                start_date_time: s.start_date_time.map(|dt| dt.timestamp() as f64),
                end_date_time: s.end_date_time.map(|dt| dt.timestamp() as f64),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn build_step_summary_wire(s: &StepData) -> StepSummary {
    StepSummary {
        id: Some(s.id.clone()),
        name: Some(s.name.clone()),
        action_on_failure: s.action_on_failure.clone(),
        config: Some(HadoopStepConfig {
            jar: Some(s.jar.clone()),
            args: s.args.clone(),
            main_class: s.main_class.clone(),
            ..Default::default()
        }),
        status: Some(StepStatus {
            state: Some(s.status.clone()),
            timeline: Some(StepTimeline {
                creation_date_time: Some(s.creation_date_time.timestamp() as f64),
                start_date_time: s.start_date_time.map(|dt| dt.timestamp() as f64),
                end_date_time: s.end_date_time.map(|dt| dt.timestamp() as f64),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn build_instance_group_wire(g: &InstanceGroupData) -> InstanceGroup {
    InstanceGroup {
        id: Some(g.id.clone()),
        name: g.name.clone(),
        instance_type: Some(g.instance_type.clone()),
        instance_group_type: Some(g.instance_role.clone()),
        market: g.market.clone(),
        bid_price: g.bid_price.clone(),
        requested_instance_count: Some(g.instance_count),
        running_instance_count: g.running_instance_count,
        status: Some(InstanceGroupStatus {
            state: Some(g.status.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn build_instance_fleet_wire(f: &InstanceFleetData) -> InstanceFleet {
    InstanceFleet {
        id: Some(f.id.clone()),
        name: f.name.clone(),
        instance_fleet_type: Some(f.instance_fleet_type.clone()),
        target_on_demand_capacity: f.target_on_demand_capacity,
        target_spot_capacity: f.target_spot_capacity,
        status: Some(InstanceFleetStatus {
            state: Some(f.status.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn parse_instance_group_config(v: &Value) -> InstanceGroupData {
    let id = format!(
        "ig-{}",
        Uuid::new_v4()
            .as_simple()
            .to_string()
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(13)
            .collect::<String>()
    );
    let auto_scaling_policy = v
        .get("AutoScalingPolicy")
        .map(|p| parse_auto_scaling_policy(Some(p)));

    InstanceGroupData {
        id,
        name: v.get("Name").and_then(|v| v.as_str()).map(String::from),
        instance_type: v
            .get("InstanceType")
            .and_then(|v| v.as_str())
            .unwrap_or("m5.xlarge")
            .to_string(),
        instance_role: v
            .get("InstanceRole")
            .and_then(|v| v.as_str())
            .unwrap_or("CORE")
            .to_string(),
        instance_count: v.get("InstanceCount").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
        market: v.get("Market").and_then(|v| v.as_str()).map(String::from),
        bid_price: v.get("BidPrice").and_then(|v| v.as_str()).map(String::from),
        status: CLUSTER_STATUS_RUNNING.to_string(),
        running_instance_count: Some(
            v.get("InstanceCount").and_then(|v| v.as_i64()).unwrap_or(1) as i32
        ),
        auto_scaling_policy,
    }
}

fn parse_instance_fleet_config(v: &Value) -> InstanceFleetData {
    let id = format!(
        "if-{}",
        Uuid::new_v4()
            .as_simple()
            .to_string()
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(13)
            .collect::<String>()
    );
    InstanceFleetData {
        id,
        name: v.get("Name").and_then(|v| v.as_str()).map(String::from),
        instance_fleet_type: v
            .get("InstanceFleetType")
            .and_then(|v| v.as_str())
            .unwrap_or("CORE")
            .to_string(),
        target_on_demand_capacity: v
            .get("TargetOnDemandCapacity")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        target_spot_capacity: v
            .get("TargetSpotCapacity")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        status: CLUSTER_STATUS_RUNNING.to_string(),
    }
}

fn parse_auto_scaling_policy(v: Option<&Value>) -> AutoScalingPolicyData {
    let constraints = v
        .and_then(|p| p.get("Constraints"))
        .map(|c| ScalingConstraintsData {
            min_capacity: c.get("MinCapacity").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
            max_capacity: c.get("MaxCapacity").and_then(|v| v.as_i64()).unwrap_or(10) as i32,
        });
    let rules: Vec<serde_json::Value> = v
        .and_then(|p| p.get("Rules"))
        .and_then(|r| r.as_array())
        .cloned()
        .unwrap_or_default();
    AutoScalingPolicyData {
        status: "ATTACHED".to_string(),
        constraints,
        rules,
    }
}

fn parse_managed_scaling_policy(v: &Value) -> ManagedScalingPolicyData {
    let compute_limits = v.get("ComputeLimits").map(|cl| ComputeLimitsData {
        minimum_capacity_units: cl
            .get("MinimumCapacityUnits")
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32,
        maximum_capacity_units: cl
            .get("MaximumCapacityUnits")
            .and_then(|v| v.as_i64())
            .unwrap_or(10) as i32,
        maximum_on_demand_capacity_units: cl
            .get("MaximumOnDemandCapacityUnits")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        maximum_core_capacity_units: cl
            .get("MaximumCoreCapacityUnits")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        unit_type: cl
            .get("UnitType")
            .and_then(|v| v.as_str())
            .unwrap_or("Instances")
            .to_string(),
    });
    ManagedScalingPolicyData {
        compute_limits,
        scaling_strategy: v
            .get("ScalingStrategy")
            .and_then(|v| v.as_str())
            .map(String::from),
        utilization_performance_index: v
            .get("UtilizationPerformanceIndex")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
    }
}
