use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{AppAutoScalingError, ApplicationAutoScalingState};
use crate::views::ApplicationAutoScalingStateView;
use crate::wire;

/// Application Auto Scaling service handler that processes awsJson1.1 protocol requests.
pub struct ApplicationAutoScalingService {
    pub(crate) state: Arc<BackendState<ApplicationAutoScalingState>>,
    pub(crate) notifier: StateChangeNotifier<ApplicationAutoScalingStateView>,
}

impl ApplicationAutoScalingService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApplicationAutoScalingService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApplicationAutoScalingService {
    fn service_name(&self) -> &str {
        "application-autoscaling"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://application-autoscaling\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ApplicationAutoScalingService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "AnyScaleFrontendService.RegisterScalableTarget"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "RegisterScalableTarget" => {
                self.handle_register_scalable_target(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeregisterScalableTarget" => {
                self.handle_deregister_scalable_target(&state, body_bytes)
                    .await
            }
            "DescribeScalableTargets" => {
                self.handle_describe_scalable_targets(&state, body_bytes)
                    .await
            }
            "PutScalingPolicy" => {
                self.handle_put_scaling_policy(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeScalingPolicies" => {
                self.handle_describe_scaling_policies(&state, body_bytes)
                    .await
            }
            "DeleteScalingPolicy" => self.handle_delete_scaling_policy(&state, body_bytes).await,
            "PutScheduledAction" => {
                self.handle_put_scheduled_action(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteScheduledAction" => {
                self.handle_delete_scheduled_action(&state, body_bytes)
                    .await
            }
            "DescribeScheduledActions" => {
                self.handle_describe_scheduled_actions(&state, body_bytes)
                    .await
            }
            "DescribeScalingActivities" => {
                self.handle_describe_scaling_activities(&state, body_bytes)
                    .await
            }
            "GetPredictiveScalingForecast" => {
                self.handle_get_predictive_scaling_forecast(&state, body_bytes)
                    .await
            }
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for ApplicationAutoScaling"),
            ),
        }
    }

    async fn handle_register_scalable_target(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_scalable_target_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        if input.scalable_dimension.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScalableDimension'");
        }
        let service_namespace = input.service_namespace.as_str();
        let resource_id = input.resource_id.as_str();
        let scalable_dimension = input.scalable_dimension.as_str();
        let min_capacity = input.min_capacity.map(i64::from);
        let max_capacity = input.max_capacity.map(i64::from);
        let role_arn = input.role_a_r_n.as_deref();

        let suspended_state =
            input
                .suspended_state
                .as_ref()
                .map(|ss| crate::types::SuspendedState {
                    dynamic_scaling_in_suspended: ss.dynamic_scaling_in_suspended.unwrap_or(false),
                    dynamic_scaling_out_suspended: ss
                        .dynamic_scaling_out_suspended
                        .unwrap_or(false),
                    scheduled_scaling_suspended: ss.scheduled_scaling_suspended.unwrap_or(false),
                });

        let mut state = state.write().await;
        match state.register_scalable_target(
            service_namespace,
            resource_id,
            scalable_dimension,
            min_capacity,
            max_capacity,
            role_arn,
            suspended_state,
            account_id,
            region,
        ) {
            Ok(scalable_target_arn) => wire::serialize_register_scalable_target_response(
                &wire::RegisterScalableTargetResponse {
                    scalable_target_a_r_n: Some(scalable_target_arn),
                },
            ),
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_deregister_scalable_target(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_scalable_target_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        if input.scalable_dimension.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScalableDimension'");
        }

        let mut state = state.write().await;
        match state.deregister_scalable_target(
            &input.service_namespace,
            &input.resource_id,
            &input.scalable_dimension,
        ) {
            Ok(()) => wire::serialize_deregister_scalable_target_response(
                &wire::DeregisterScalableTargetResponse {},
            ),
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_describe_scalable_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scalable_targets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        let resource_ids = input.resource_ids;
        let scalable_dimension = input.scalable_dimension.as_deref();

        let state = state.read().await;
        let targets = state.describe_scalable_targets(
            &input.service_namespace,
            resource_ids.as_deref(),
            scalable_dimension,
        );

        let entries: Vec<wire::ScalableTarget> =
            targets.iter().map(|t| to_wire_scalable_target(t)).collect();

        wire::serialize_describe_scalable_targets_response(&wire::DescribeScalableTargetsResponse {
            scalable_targets: Some(entries),
            next_token: None,
        })
    }

    async fn handle_put_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_scaling_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.policy_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PolicyName'");
        }
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        if input.scalable_dimension.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScalableDimension'");
        }
        let policy_type = input.policy_type.as_deref();
        let target_tracking_config = input
            .target_tracking_scaling_policy_configuration
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let step_scaling_config = input
            .step_scaling_policy_configuration
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.put_scaling_policy(
            &input.policy_name,
            &input.service_namespace,
            &input.resource_id,
            &input.scalable_dimension,
            policy_type,
            target_tracking_config,
            step_scaling_config,
            account_id,
            region,
        ) {
            Ok(policy) => {
                wire::serialize_put_scaling_policy_response(&wire::PutScalingPolicyResponse {
                    policy_a_r_n: Some(policy.policy_arn.clone()),
                    alarms: Some(vec![]),
                })
            }
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_describe_scaling_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scaling_policies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        let resource_id = input.resource_id.as_deref();
        let scalable_dimension = input.scalable_dimension.as_deref();
        let policy_names = input.policy_names;

        let state = state.read().await;
        let policies = state.describe_scaling_policies(
            &input.service_namespace,
            resource_id,
            scalable_dimension,
            policy_names.as_deref(),
        );

        let entries: Vec<wire::ScalingPolicy> =
            policies.iter().map(|p| to_wire_scaling_policy(p)).collect();

        wire::serialize_describe_scaling_policies_response(&wire::DescribeScalingPoliciesResponse {
            scaling_policies: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_scaling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_scaling_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.policy_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PolicyName'");
        }
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        if input.scalable_dimension.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScalableDimension'");
        }

        let mut state = state.write().await;
        match state.delete_scaling_policy(
            &input.policy_name,
            &input.service_namespace,
            &input.resource_id,
            &input.scalable_dimension,
        ) {
            Ok(()) => wire::serialize_delete_scaling_policy_response(
                &wire::DeleteScalingPolicyResponse {},
            ),
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_put_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_scheduled_action_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        if input.scalable_dimension.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScalableDimension'");
        }
        if input.scheduled_action_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'ScheduledActionName'",
            );
        }

        let schedule = input.schedule.as_deref();
        let start_time = input.start_time;
        let end_time = input.end_time;
        let timezone = input.timezone.as_deref();
        let scalable_target_action =
            input
                .scalable_target_action
                .as_ref()
                .map(|sta| crate::types::ScalableTargetAction {
                    min_capacity: sta.min_capacity,
                    max_capacity: sta.max_capacity,
                });

        let mut state = state.write().await;
        match state.put_scheduled_action(
            &input.service_namespace,
            &input.resource_id,
            &input.scalable_dimension,
            &input.scheduled_action_name,
            schedule,
            start_time,
            end_time,
            timezone,
            scalable_target_action,
            account_id,
            region,
        ) {
            Ok(()) => {
                wire::serialize_put_scheduled_action_response(&wire::PutScheduledActionResponse {})
            }
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_delete_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_scheduled_action_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        if input.scalable_dimension.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScalableDimension'");
        }
        if input.scheduled_action_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'ScheduledActionName'",
            );
        }

        let mut state = state.write().await;
        match state.delete_scheduled_action(
            &input.service_namespace,
            &input.resource_id,
            &input.scalable_dimension,
            &input.scheduled_action_name,
        ) {
            Ok(()) => wire::serialize_delete_scheduled_action_response(
                &wire::DeleteScheduledActionResponse {},
            ),
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_describe_scheduled_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scheduled_actions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        let resource_id = input.resource_id.as_deref();
        let scalable_dimension = input.scalable_dimension.as_deref();
        let scheduled_action_names = input.scheduled_action_names;

        let state = state.read().await;
        let actions = state.describe_scheduled_actions(
            &input.service_namespace,
            resource_id,
            scalable_dimension,
            scheduled_action_names.as_deref(),
        );

        let entries: Vec<wire::ScheduledAction> = actions
            .iter()
            .map(|a| wire::ScheduledAction {
                scheduled_action_name: Some(a.scheduled_action_name.clone()),
                scheduled_action_a_r_n: Some(a.scheduled_action_arn.clone()),
                service_namespace: Some(a.service_namespace.clone()),
                resource_id: Some(a.resource_id.clone()),
                scalable_dimension: Some(a.scalable_dimension.clone()),
                schedule: a.schedule.clone(),
                start_time: a.start_time,
                end_time: a.end_time,
                timezone: a.timezone.clone(),
                scalable_target_action: a.scalable_target_action.as_ref().map(|sta| {
                    wire::ScalableTargetAction {
                        min_capacity: sta.min_capacity,
                        max_capacity: sta.max_capacity,
                    }
                }),
                creation_time: Some(a.creation_time.timestamp() as f64),
            })
            .collect();

        wire::serialize_describe_scheduled_actions_response(
            &wire::DescribeScheduledActionsResponse {
                scheduled_actions: Some(entries),
                next_token: None,
            },
        )
    }

    async fn handle_describe_scaling_activities(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scaling_activities_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_namespace.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServiceNamespace'");
        }
        let resource_id = input.resource_id.as_deref();
        let scalable_dimension = input.scalable_dimension.as_deref();

        let state = state.read().await;
        state.describe_scaling_activities(
            &input.service_namespace,
            resource_id,
            scalable_dimension,
        );

        wire::serialize_describe_scaling_activities_response(
            &wire::DescribeScalingActivitiesResponse {
                scaling_activities: Some(vec![]),
                next_token: None,
            },
        )
    }

    // STUB[no-telemetry]: Predictive scaling forecasts are driven by real AWS capacity telemetry;
    //   no equivalent data exists in the mock. Always returns empty forecast.
    async fn handle_get_predictive_scaling_forecast(
        &self,
        _state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        _body: &[u8],
    ) -> MockResponse {
        // Predictive scaling forecast is not simulated; return empty forecast data
        wire::serialize_get_predictive_scaling_forecast_response(
            &wire::GetPredictiveScalingForecastResponse {
                capacity_forecast: Some(wire::CapacityForecast {
                    timestamps: Some(vec![]),
                    values: Some(vec![]),
                }),
                load_forecast: Some(vec![]),
                update_time: None,
                ..Default::default()
            },
        )
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }
        if input.tags.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Tags'");
        }

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }
        if input.tag_keys.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TagKeys'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => app_autoscaling_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationAutoScalingState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_a_r_n) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse { tags: Some(tags) },
            ),
            Err(e) => app_autoscaling_error_response(&e),
        }
    }
}

// --- State-to-wire conversion helpers ---

fn to_wire_scalable_target(t: &crate::types::ScalableTarget) -> wire::ScalableTarget {
    wire::ScalableTarget {
        service_namespace: Some(t.service_namespace.clone()),
        resource_id: Some(t.resource_id.clone()),
        scalable_dimension: Some(t.scalable_dimension.clone()),
        min_capacity: Some(t.min_capacity as i32),
        max_capacity: Some(t.max_capacity as i32),
        role_a_r_n: Some(t.role_arn.clone()),
        creation_time: Some(t.creation_time.timestamp() as f64),
        suspended_state: t.suspended_state.as_ref().map(|ss| wire::SuspendedState {
            dynamic_scaling_in_suspended: Some(ss.dynamic_scaling_in_suspended),
            dynamic_scaling_out_suspended: Some(ss.dynamic_scaling_out_suspended),
            scheduled_scaling_suspended: Some(ss.scheduled_scaling_suspended),
        }),
        ..Default::default()
    }
}

fn to_wire_scaling_policy(p: &crate::types::ScalingPolicy) -> wire::ScalingPolicy {
    wire::ScalingPolicy {
        policy_a_r_n: Some(p.policy_arn.clone()),
        policy_name: Some(p.policy_name.clone()),
        service_namespace: Some(p.service_namespace.clone()),
        resource_id: Some(p.resource_id.clone()),
        scalable_dimension: Some(p.scalable_dimension.clone()),
        policy_type: Some(p.policy_type.clone()),
        creation_time: Some(p.creation_time.timestamp() as f64),
        alarms: Some(vec![]),
        target_tracking_scaling_policy_configuration: p
            .target_tracking_scaling_policy_configuration
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok()),
        step_scaling_policy_configuration: p
            .step_scaling_policy_configuration
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok()),
        ..Default::default()
    }
}

// --- Utility functions ---

fn app_autoscaling_error_response(err: &AppAutoScalingError) -> MockResponse {
    let (status, error_type) = match err {
        AppAutoScalingError::ScalableTargetNotFound { .. } => (400u16, "ObjectNotFoundException"),
        AppAutoScalingError::ScalingPolicyNotFound { .. } => (400u16, "ObjectNotFoundException"),
        AppAutoScalingError::ScheduledActionNotFound { .. } => (400u16, "ObjectNotFoundException"),
        AppAutoScalingError::ResourceNotFound { .. } => (400u16, "ResourceNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
