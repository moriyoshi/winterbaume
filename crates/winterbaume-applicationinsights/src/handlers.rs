use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::model;
use crate::state::{
    ApplicationInsightsError, ApplicationInsightsState, ApplicationRecord, ComponentRecord,
    LogPatternRecord, WorkloadRecord,
};
use crate::views::ApplicationInsightsStateView;
use crate::wire;

pub struct ApplicationInsightsService {
    pub(crate) state: Arc<BackendState<ApplicationInsightsState>>,
    pub(crate) notifier: StateChangeNotifier<ApplicationInsightsStateView>,
}

impl ApplicationInsightsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApplicationInsightsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApplicationInsightsService {
    fn service_name(&self) -> &str {
        "applicationinsights"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://applicationinsights\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<ApplicationInsightsState>>;

const MUTATING_ACTIONS: &[&str] = &[
    "AddWorkload",
    "CreateApplication",
    "CreateComponent",
    "CreateLogPattern",
    "DeleteApplication",
    "DeleteComponent",
    "DeleteLogPattern",
    "RemoveWorkload",
    "TagResource",
    "UntagResource",
    "UpdateApplication",
    "UpdateComponent",
    "UpdateComponentConfiguration",
    "UpdateLogPattern",
    "UpdateProblem",
    "UpdateWorkload",
];

impl ApplicationInsightsService {
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
            None => return json_error_response(400, "MissingAction", "Missing X-Amz-Target"),
        };

        // Validate JSON body up-front; typed deserialisers in `wire` re-parse per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "AddWorkload" => self.handle_add_workload(&state, body_bytes).await,
            "CreateApplication" => {
                self.handle_create_application(&state, account_id, body_bytes)
                    .await
            }
            "CreateComponent" => self.handle_create_component(&state, body_bytes).await,
            "CreateLogPattern" => self.handle_create_log_pattern(&state, body_bytes).await,
            "DeleteApplication" => self.handle_delete_application(&state, body_bytes).await,
            "DeleteComponent" => self.handle_delete_component(&state, body_bytes).await,
            "DeleteLogPattern" => self.handle_delete_log_pattern(&state, body_bytes).await,
            "DescribeApplication" => self.handle_describe_application(&state, body_bytes).await,
            "DescribeComponent" => self.handle_describe_component(&state, body_bytes).await,
            "DescribeComponentConfiguration" => {
                self.handle_describe_component_configuration(&state, body_bytes)
                    .await
            }
            "DescribeComponentConfigurationRecommendation" => {
                self.handle_describe_component_configuration_recommendation(&state, body_bytes)
                    .await
            }
            "DescribeLogPattern" => self.handle_describe_log_pattern(&state, body_bytes).await,
            "DescribeObservation" => self.handle_describe_observation(&state, body_bytes).await,
            "DescribeProblem" => self.handle_describe_problem(&state, body_bytes).await,
            "DescribeProblemObservations" => {
                self.handle_describe_problem_observations(&state, body_bytes)
                    .await
            }
            "DescribeWorkload" => self.handle_describe_workload(&state, body_bytes).await,
            "ListApplications" => self.handle_list_applications(&state).await,
            "ListComponents" => self.handle_list_components(&state, body_bytes).await,
            "ListConfigurationHistory" => self.handle_list_configuration_history(&state).await,
            "ListLogPatternSets" => self.handle_list_log_pattern_sets(&state, body_bytes).await,
            "ListLogPatterns" => self.handle_list_log_patterns(&state, body_bytes).await,
            "ListProblems" => self.handle_list_problems(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags(&state, body_bytes).await,
            "ListWorkloads" => self.handle_list_workloads(&state, body_bytes).await,
            "RemoveWorkload" => self.handle_remove_workload(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateApplication" => self.handle_update_application(&state, body_bytes).await,
            "UpdateComponent" => self.handle_update_component(&state, body_bytes).await,
            "UpdateComponentConfiguration" => {
                self.handle_update_component_configuration(&state, body_bytes)
                    .await
            }
            "UpdateLogPattern" => self.handle_update_log_pattern(&state, body_bytes).await,
            "UpdateProblem" => self.handle_update_problem(&state, body_bytes).await,
            "UpdateWorkload" => self.handle_update_workload(&state, body_bytes).await,
            other => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_application(
        &self,
        state: &SharedState,
        account_id: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_field(input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let app = ApplicationRecord {
            resource_group_name: rg.clone(),
            account_id: account_id.to_string(),
            remarks: None,
            life_cycle: "ACTIVE".to_string(),
            ops_item_sns_topic_arn: input.ops_item_s_n_s_topic_arn,
            sns_notification_arn: input.s_n_s_notification_arn,
            ops_center_enabled: input.ops_center_enabled,
            cwe_monitor_enabled: input.c_w_e_monitor_enabled,
            auto_config_enabled: input.auto_config_enabled,
            attach_missing_permission: input.attach_missing_permission,
            discovery_type: input.grouping_type,
            component_configurations: HashMap::new(),
        };
        let mut state = state.write().await;
        match state.create_application(app) {
            Ok(a) => {
                wire::serialize_create_application_response(&wire::CreateApplicationResponse {
                    application_info: Some(application_to_wire(a)),
                })
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_describe_application(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_application(&rg) {
            Ok(a) => {
                wire::serialize_describe_application_response(&wire::DescribeApplicationResponse {
                    application_info: Some(application_to_wire(a)),
                })
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_update_application(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let new_ops = input.ops_center_enabled;
        let new_cwe = input.c_w_e_monitor_enabled;
        let new_topic = input.ops_item_s_n_s_topic_arn;
        let new_sns = input.s_n_s_notification_arn;
        let new_attach = input.attach_missing_permission;
        let new_auto = input.auto_config_enabled;

        let mut state = state.write().await;
        match state.update_application(&rg, |a| {
            if let Some(v) = new_ops {
                a.ops_center_enabled = Some(v);
            }
            if let Some(v) = new_cwe {
                a.cwe_monitor_enabled = Some(v);
            }
            if let Some(v) = new_topic {
                a.ops_item_sns_topic_arn = Some(v);
            }
            if let Some(v) = new_sns {
                a.sns_notification_arn = Some(v);
            }
            if let Some(v) = new_attach {
                a.attach_missing_permission = Some(v);
            }
            if let Some(v) = new_auto {
                a.auto_config_enabled = Some(v);
            }
        }) {
            Ok(a) => {
                wire::serialize_update_application_response(&wire::UpdateApplicationResponse {
                    application_info: Some(application_to_wire(a)),
                })
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_delete_application(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.delete_application(&rg) {
            Ok(()) => {
                wire::serialize_delete_application_response(&wire::DeleteApplicationResponse {})
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_list_applications(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::ApplicationInfo> = state
            .list_applications()
            .into_iter()
            .map(application_to_wire)
            .collect();
        wire::serialize_list_applications_response(&wire::ListApplicationsResponse {
            application_info_list: Some(items),
            next_token: None,
        })
    }

    async fn handle_create_component(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_create_component_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let component_name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let component = ComponentRecord {
            component_name,
            resource_type: None,
            os_type: None,
            tier: None,
            monitor: false,
            component_remarks: None,
            configuration: None,
            auto_configuration_enabled: false,
            resource_list: input.resource_list,
            workload_id_per_resource: HashMap::new(),
        };
        let mut state = state.write().await;
        match state.create_component(&rg, component) {
            Ok(_) => wire::serialize_create_component_response(&wire::CreateComponentResponse {}),
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_describe_component(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_component_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_component(&rg, &name) {
            Ok(c) => {
                wire::serialize_describe_component_response(&wire::DescribeComponentResponse {
                    application_component: Some(component_to_wire(c)),
                    resource_list: Some(c.resource_list.clone()),
                })
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_describe_component_configuration(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_component_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_component(&rg, &name) {
            Ok(c) => wire::serialize_describe_component_configuration_response(
                &wire::DescribeComponentConfigurationResponse {
                    component_configuration: c
                        .configuration
                        .as_ref()
                        .and_then(|v| v.as_str().map(String::from)),
                    monitor: Some(c.monitor),
                    tier: c.tier.clone(),
                },
            ),
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_describe_component_configuration_recommendation(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_component_configuration_recommendation_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        if state.get_component(&rg, &name).is_err() {
            return ai_error_response(&ApplicationInsightsError::NotFound {
                resource_type: "Component",
                key: name,
            });
        }
        wire::serialize_describe_component_configuration_recommendation_response(
            &wire::DescribeComponentConfigurationRecommendationResponse {
                component_configuration: Some("{}".to_string()),
            },
        )
    }

    async fn handle_update_component(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_component_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let new_name = input.new_component_name;
        let new_resources = input.resource_list;
        let mut state = state.write().await;
        // If renaming, do remove + reinsert under new key.
        if let Some(nn) = new_name {
            match state.delete_component(&rg, &name) {
                Ok(()) => {}
                Err(e) => return ai_error_response(&e),
            }
            let component = ComponentRecord {
                component_name: nn.clone(),
                resource_type: None,
                os_type: None,
                tier: None,
                monitor: false,
                component_remarks: None,
                configuration: None,
                auto_configuration_enabled: false,
                resource_list: new_resources.unwrap_or_default(),
                workload_id_per_resource: HashMap::new(),
            };
            match state.create_component(&rg, component) {
                Ok(_) => {
                    wire::serialize_update_component_response(&wire::UpdateComponentResponse {})
                }
                Err(e) => ai_error_response(&e),
            }
        } else {
            match state.update_component(&rg, &name, |c| {
                if let Some(r) = new_resources {
                    c.resource_list = r;
                }
            }) {
                Ok(_) => {
                    wire::serialize_update_component_response(&wire::UpdateComponentResponse {})
                }
                Err(e) => ai_error_response(&e),
            }
        }
    }

    async fn handle_update_component_configuration(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_component_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let monitor = input.monitor.unwrap_or(false);
        let tier = input.tier;
        let config = input
            .component_configuration
            .map(|s| Value::String(s.to_string()));
        let auto_config = input.auto_config_enabled.unwrap_or(false);
        let mut state = state.write().await;
        match state.update_component(&rg, &name, |c| {
            c.monitor = monitor;
            c.tier = tier;
            if let Some(cfg) = config {
                c.configuration = Some(cfg);
            }
            c.auto_configuration_enabled = auto_config;
        }) {
            Ok(_) => wire::serialize_update_component_configuration_response(
                &wire::UpdateComponentConfigurationResponse {},
            ),
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_delete_component(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_component_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.delete_component(&rg, &name) {
            Ok(()) => wire::serialize_delete_component_response(&wire::DeleteComponentResponse {}),
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_list_components(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_components_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        let items: Vec<wire::ApplicationComponent> = state
            .list_components(&rg)
            .into_iter()
            .map(component_to_wire)
            .collect();
        wire::serialize_list_components_response(&wire::ListComponentsResponse {
            application_component_list: Some(items),
            next_token: None,
        })
    }

    async fn handle_create_log_pattern(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_create_log_pattern_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let pattern_set_name = match require_str(&input.pattern_set_name, "PatternSetName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let pattern_name = match require_str(&input.pattern_name, "PatternName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let pattern = match require_str(&input.pattern, "Pattern") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let rank = if input.rank == 0 { 1 } else { input.rank };
        let log_pattern = LogPatternRecord {
            pattern_set_name,
            pattern_name,
            pattern,
            rank,
        };
        let mut state = state.write().await;
        match state.create_log_pattern(&rg, log_pattern) {
            Ok(lp) => {
                wire::serialize_create_log_pattern_response(&wire::CreateLogPatternResponse {
                    log_pattern: Some(log_pattern_to_wire(lp)),
                    resource_group_name: Some(rg),
                })
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_describe_log_pattern(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_log_pattern_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let set = match require_str(&input.pattern_set_name, "PatternSetName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.pattern_name, "PatternName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_log_pattern(&rg, &set, &name) {
            Ok(lp) => {
                wire::serialize_describe_log_pattern_response(&wire::DescribeLogPatternResponse {
                    account_id: None,
                    log_pattern: Some(log_pattern_to_wire(lp)),
                    resource_group_name: Some(rg),
                })
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_update_log_pattern(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_log_pattern_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let set = match require_str(&input.pattern_set_name, "PatternSetName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.pattern_name, "PatternName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let pattern = input.pattern.unwrap_or_default();
        let rank = input.rank.unwrap_or(1);
        let mut state = state.write().await;
        match state.update_log_pattern(&rg, &set, &name, pattern, rank) {
            Ok(lp) => {
                wire::serialize_update_log_pattern_response(&wire::UpdateLogPatternResponse {
                    log_pattern: Some(log_pattern_to_wire(lp)),
                    resource_group_name: Some(rg),
                })
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_delete_log_pattern(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_log_pattern_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let set = match require_str(&input.pattern_set_name, "PatternSetName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(&input.pattern_name, "PatternName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.delete_log_pattern(&rg, &set, &name) {
            Ok(()) => {
                wire::serialize_delete_log_pattern_response(&wire::DeleteLogPatternResponse {})
            }
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_list_log_patterns(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_log_patterns_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let set = input.pattern_set_name.as_deref();
        let state = state.read().await;
        let items: Vec<wire::LogPattern> = state
            .list_log_patterns(&rg, set)
            .into_iter()
            .map(log_pattern_to_wire)
            .collect();
        wire::serialize_list_log_patterns_response(&wire::ListLogPatternsResponse {
            account_id: None,
            log_patterns: Some(items),
            next_token: None,
            resource_group_name: Some(rg),
        })
    }

    async fn handle_list_log_pattern_sets(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_log_pattern_sets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        let sets = state.list_log_pattern_sets(&rg);
        wire::serialize_list_log_pattern_sets_response(&wire::ListLogPatternSetsResponse {
            account_id: None,
            log_pattern_sets: Some(sets),
            next_token: None,
            resource_group_name: Some(rg),
        })
    }

    async fn handle_add_workload(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_add_workload_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let component_name = match require_str(&input.component_name, "ComponentName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let workload_obj = serde_json::to_value(&input.workload_configuration).unwrap_or_default();
        let workload_name = input
            .workload_configuration
            .workload_name
            .clone()
            .unwrap_or_else(|| "DEFAULT".to_string());
        let tier = input
            .workload_configuration
            .tier
            .clone()
            .unwrap_or_else(|| "DEFAULT".to_string());
        let id = format!("workload-{}", uuid::Uuid::new_v4().simple());
        let workload = WorkloadRecord {
            workload_id: id.clone(),
            component_name,
            workload_name,
            tier,
            workload_remarks: None,
            workload_configuration: Some(workload_obj),
        };
        let mut state = state.write().await;
        let w = state.add_workload(&rg, workload).unwrap();
        wire::serialize_add_workload_response(&wire::AddWorkloadResponse {
            workload_configuration: w.workload_configuration.as_ref().and_then(parse_value),
            workload_id: Some(w.workload_id.clone()),
        })
    }

    async fn handle_describe_workload(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_workload_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let id = match require_str(&input.workload_id, "WorkloadId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_workload(&rg, &id) {
            Ok(w) => wire::serialize_describe_workload_response(&wire::DescribeWorkloadResponse {
                workload_configuration: w.workload_configuration.as_ref().and_then(parse_value),
                workload_id: Some(w.workload_id.clone()),
                workload_remarks: w.workload_remarks.clone(),
            }),
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_update_workload(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_workload_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let id = match input.workload_id.as_deref() {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => {
                return json_error_response(400, "ValidationException", "WorkloadId is required");
            }
        };
        // The original code only updated when WorkloadConfiguration is present;
        // since the model treats it as required, we always pass a value.
        let new_workload =
            Some(serde_json::to_value(&input.workload_configuration).unwrap_or_default());
        let mut state = state.write().await;
        match state.update_workload(&rg, &id, |w| {
            if let Some(c) = new_workload {
                w.workload_configuration = Some(c);
            }
        }) {
            Ok(w) => wire::serialize_update_workload_response(&wire::UpdateWorkloadResponse {
                workload_configuration: w.workload_configuration.as_ref().and_then(parse_value),
                workload_id: Some(w.workload_id.clone()),
            }),
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_remove_workload(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_remove_workload_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let id = match require_str(&input.workload_id, "WorkloadId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.remove_workload(&rg, &id) {
            Ok(()) => wire::serialize_remove_workload_response(&wire::RemoveWorkloadResponse {}),
            Err(e) => ai_error_response(&e),
        }
    }

    async fn handle_list_workloads(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_workloads_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = match require_str(&input.resource_group_name, "ResourceGroupName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        let items: Vec<wire::Workload> = state
            .list_workloads(&rg)
            .into_iter()
            .map(workload_to_wire)
            .collect();
        wire::serialize_list_workloads_response(&wire::ListWorkloadsResponse {
            next_token: None,
            workload_list: Some(items),
        })
    }

    async fn handle_describe_observation(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_observation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let id = match require_str(&input.observation_id, "ObservationId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.observations.get(&id) {
            Some(v) => {
                let observation: Option<wire::Observation> = serde_json::from_value(v.clone()).ok();
                wire::serialize_describe_observation_response(&wire::DescribeObservationResponse {
                    observation,
                })
            }
            None => ai_error_response(&ApplicationInsightsError::NotFound {
                resource_type: "Observation",
                key: id,
            }),
        }
    }

    async fn handle_describe_problem(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_problem_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let id = match require_str(&input.problem_id, "ProblemId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.problems.iter().find(|p| p.id == id) {
            Some(p) => wire::serialize_describe_problem_response(&wire::DescribeProblemResponse {
                problem: Some(problem_to_wire(p)),
                s_n_s_notification_arn: None,
            }),
            None => ai_error_response(&ApplicationInsightsError::NotFound {
                resource_type: "Problem",
                key: id,
            }),
        }
    }

    async fn handle_describe_problem_observations(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_problem_observations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let id = match require_str(&input.problem_id, "ProblemId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        if state.problems.iter().all(|p| p.id != id) {
            return ai_error_response(&ApplicationInsightsError::NotFound {
                resource_type: "Problem",
                key: id,
            });
        }
        wire::serialize_describe_problem_observations_response(
            &wire::DescribeProblemObservationsResponse {
                related_observations: Some(wire::RelatedObservations {
                    observation_list: Some(vec![]),
                }),
            },
        )
    }

    async fn handle_list_problems(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_problems_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rg = input.resource_group_name.clone();
        let rg_filter = rg.as_deref();
        let state = state.read().await;
        let problems: Vec<wire::Problem> = state
            .problems
            .iter()
            .filter(|p| rg_filter.is_none_or(|r| p.resource_group_name == r))
            .map(problem_to_wire)
            .collect();
        wire::serialize_list_problems_response(&wire::ListProblemsResponse {
            account_id: None,
            next_token: None,
            problem_list: Some(problems),
            resource_group_name: rg,
        })
    }

    async fn handle_update_problem(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_problem_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let id = match require_str(&input.problem_id, "ProblemId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let new_status = input.update_status;
        let mut state = state.write().await;
        let p = state.problems.iter_mut().find(|p| p.id == id);
        match p {
            Some(p) => {
                if let Some(s) = new_status {
                    p.status = s;
                }
                wire::serialize_update_problem_response(&wire::UpdateProblemResponse {})
            }
            None => ai_error_response(&ApplicationInsightsError::NotFound {
                resource_type: "Problem",
                key: id,
            }),
        }
    }

    async fn handle_list_configuration_history(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let typed: Vec<wire::ConfigurationEvent> = state
            .configuration_history
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_configuration_history_response(
            &wire::ListConfigurationHistoryResponse {
                event_list: Some(typed),
                next_token: None,
            },
        )
    }

    async fn handle_tag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let arn = match require_str(&input.resource_a_r_n, "ResourceARN") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        let bag = state.tags.entry(arn).or_default();
        for model::Tag { key, value } in input.tags {
            bag.insert(key, value);
        }
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let arn = match require_str(&input.resource_a_r_n, "ResourceARN") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let keys = input.tag_keys;
        let mut state = state.write().await;
        if let Some(bag) = state.tags.get_mut(&arn) {
            for k in &keys {
                bag.remove(k);
            }
        }
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_list_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let arn = match require_str(&input.resource_a_r_n, "ResourceARN") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        let tags = state.tags.get(&arn).cloned().unwrap_or_default();
        let tags_wire: Vec<wire::Tag> = tags
            .into_iter()
            .map(|(k, v)| wire::Tag { key: k, value: v })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags_wire.is_empty() {
                None
            } else {
                Some(tags_wire)
            },
        })
    }
}

fn application_to_wire(a: &ApplicationRecord) -> wire::ApplicationInfo {
    wire::ApplicationInfo {
        account_id: Some(a.account_id.clone()),
        attach_missing_permission: a.attach_missing_permission,
        auto_config_enabled: a.auto_config_enabled,
        c_w_e_monitor_enabled: a.cwe_monitor_enabled,
        discovery_type: a.discovery_type.clone(),
        life_cycle: Some(a.life_cycle.clone()),
        ops_center_enabled: a.ops_center_enabled,
        ops_item_s_n_s_topic_arn: a.ops_item_sns_topic_arn.clone(),
        remarks: a.remarks.clone(),
        resource_group_name: Some(a.resource_group_name.clone()),
        s_n_s_notification_arn: a.sns_notification_arn.clone(),
    }
}

fn component_to_wire(c: &ComponentRecord) -> wire::ApplicationComponent {
    wire::ApplicationComponent {
        component_name: Some(c.component_name.clone()),
        component_remarks: c.component_remarks.clone(),
        detected_workload: None,
        monitor: Some(c.monitor),
        os_type: c.os_type.clone(),
        resource_type: c.resource_type.clone(),
        tier: c.tier.clone(),
    }
}

fn log_pattern_to_wire(lp: &LogPatternRecord) -> wire::LogPattern {
    wire::LogPattern {
        pattern: Some(lp.pattern.clone()),
        pattern_name: Some(lp.pattern_name.clone()),
        pattern_set_name: Some(lp.pattern_set_name.clone()),
        rank: Some(lp.rank),
    }
}

fn workload_to_wire(w: &WorkloadRecord) -> wire::Workload {
    wire::Workload {
        component_name: Some(w.component_name.clone()),
        missing_workload_config: Some(w.workload_configuration.is_none()),
        tier: Some(w.tier.clone()),
        workload_id: Some(w.workload_id.clone()),
        workload_name: Some(w.workload_name.clone()),
        workload_remarks: w.workload_remarks.clone(),
    }
}

fn problem_to_wire(p: &crate::state::ProblemRecord) -> wire::Problem {
    wire::Problem {
        account_id: None,
        affected_resource: p.affected_resource.clone(),
        end_time: p.end_time.map(|t| t as f64),
        feedback: if p.feedback.is_empty() {
            None
        } else {
            Some(p.feedback.clone())
        },
        id: Some(p.id.clone()),
        insights: None,
        last_recurrence_time: None,
        recurring_count: None,
        resolution_method: None,
        resource_group_name: Some(p.resource_group_name.clone()),
        severity_level: p.severity.clone(),
        short_name: None,
        start_time: p.start_time.map(|t| t as f64),
        status: Some(p.status.clone()),
        title: Some(p.title.clone()),
        visibility: None,
    }
}

/// Validate a required `String` field that comes through the typed request struct.
fn require_str(value: &str, field: &str) -> Result<String, MockResponse> {
    if value.is_empty() {
        Err(json_error_response(
            400,
            "ValidationException",
            &format!("{field} is required"),
        ))
    } else {
        Ok(value.to_string())
    }
}

/// Validate a required `Option<String>` field. Used when the model marks the field optional
/// at the type level but the operation requires it semantically.
fn require_field(value: Option<String>, field: &str) -> Result<String, MockResponse> {
    match value {
        Some(s) if !s.is_empty() => Ok(s),
        _ => Err(json_error_response(
            400,
            "ValidationException",
            &format!("{field} is required"),
        )),
    }
}

fn parse_value<T: serde::de::DeserializeOwned>(v: &Value) -> Option<T> {
    serde_json::from_value(v.clone()).ok()
}

fn ai_error_response(err: &ApplicationInsightsError) -> MockResponse {
    let (status, error_type) = match err {
        ApplicationInsightsError::NotFound { .. } => (404, "ResourceNotFoundException"),
        ApplicationInsightsError::AlreadyExists { .. } => (409, "ResourceInUseException"),
        ApplicationInsightsError::Validation { .. } => (400, "ValidationException"),
    };
    json_error_response(status, error_type, &err.to_string())
}
