use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::{AutoScalingPlansError, AutoScalingPlansState};
use crate::types::ScalingPlan;
use crate::views::AutoScalingPlansStateView;
use crate::wire;

pub struct AutoScalingPlansService {
    pub(crate) state: Arc<BackendState<AutoScalingPlansState>>,
    pub(crate) notifier: StateChangeNotifier<AutoScalingPlansStateView>,
}

impl AutoScalingPlansService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AutoScalingPlansService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AutoScalingPlansService {
    fn service_name(&self) -> &str {
        "autoscaling-plans"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://autoscaling-plans\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AutoScalingPlansService {
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

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateScalingPlan" => self.handle_create_scaling_plan(&state, body_bytes).await,
            "DeleteScalingPlan" => self.handle_delete_scaling_plan(&state, body_bytes).await,
            "DescribeScalingPlans" => self.handle_describe_scaling_plans(&state, body_bytes).await,
            "DescribeScalingPlanResources" => {
                self.handle_describe_scaling_plan_resources(&state, body_bytes)
                    .await
            }
            "GetScalingPlanResourceForecastData" => {
                self.handle_get_scaling_plan_resource_forecast_data(&state, body_bytes)
                    .await
            }
            "UpdateScalingPlan" => self.handle_update_scaling_plan(&state, body_bytes).await,
            other => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(
                action.as_str(),
                "CreateScalingPlan" | "UpdateScalingPlan" | "DeleteScalingPlan"
            )
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_scaling_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingPlansState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_scaling_plan_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scaling_plan_name.is_empty() {
            return json_error_response(400, "ValidationException", "ScalingPlanName is required");
        }
        let name = input.scaling_plan_name.clone();
        let app_source = serde_json::to_value(&input.application_source)
            .unwrap_or(Value::Object(Default::default()));
        let scaling_instructions: Vec<Value> = input
            .scaling_instructions
            .iter()
            .filter_map(|si| serde_json::to_value(si).ok())
            .collect();

        let now = chrono::Utc::now().timestamp();
        let plan = ScalingPlan {
            scaling_plan_name: name.clone(),
            scaling_plan_version: 1,
            application_source: app_source,
            scaling_instructions,
            status_code: "Active".to_string(),
            status_message: None,
            status_start_time: now,
            creation_time: now,
        };

        let mut state = state.write().await;
        match state.create_plan(plan) {
            Ok(version) => {
                let response = wire::CreateScalingPlanResponse {
                    scaling_plan_version: Some(version),
                };
                wire::serialize_create_scaling_plan_response(&response)
            }
            Err(e) => asp_error_response(&e),
        }
    }

    async fn handle_delete_scaling_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingPlansState>>,
        body: &[u8],
    ) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_delete_scaling_plan_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scaling_plan_name.is_empty() {
            return json_error_response(400, "ValidationException", "ScalingPlanName is required");
        }
        let name = input.scaling_plan_name.clone();
        // Default to 1 when ScalingPlanVersion is absent (matches previous behaviour).
        let version = if raw.get("ScalingPlanVersion").is_some() {
            input.scaling_plan_version
        } else {
            1
        };
        let mut state = state.write().await;
        match state.delete_plan(&name, version) {
            Ok(()) => {
                wire::serialize_delete_scaling_plan_response(&wire::DeleteScalingPlanResponse {})
            }
            Err(e) => asp_error_response(&e),
        }
    }

    async fn handle_describe_scaling_plans(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingPlansState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scaling_plans_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let names: Option<Vec<String>> = input.scaling_plan_names.clone();
        let state = state.read().await;
        let plans: Vec<wire::ScalingPlan> = state
            .list_plans(names.as_deref())
            .iter()
            .map(|p| plan_to_wire(p))
            .collect();
        let response = wire::DescribeScalingPlansResponse {
            scaling_plans: Some(plans),
            next_token: None,
        };
        wire::serialize_describe_scaling_plans_response(&response)
    }

    async fn handle_describe_scaling_plan_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingPlansState>>,
        body: &[u8],
    ) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_describe_scaling_plan_resources_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scaling_plan_name.is_empty() {
            return json_error_response(400, "ValidationException", "ScalingPlanName is required");
        }
        let name = input.scaling_plan_name.clone();
        let version = if raw.get("ScalingPlanVersion").is_some() {
            input.scaling_plan_version
        } else {
            1
        };
        let state = state.read().await;
        if state.get_plan(&name, version).is_err() {
            return asp_error_response(&AutoScalingPlansError::PlanNotFound { name, version });
        }
        // Mock returns no resources by default; real Auto Scaling Plans
        // discovers them from CloudFormation/tag filters in the application
        // source.
        let response = wire::DescribeScalingPlanResourcesResponse {
            scaling_plan_resources: Some(vec![]),
            next_token: None,
        };
        wire::serialize_describe_scaling_plan_resources_response(&response)
    }

    async fn handle_get_scaling_plan_resource_forecast_data(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingPlansState>>,
        body: &[u8],
    ) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_get_scaling_plan_resource_forecast_data_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scaling_plan_name.is_empty() {
            return json_error_response(400, "ValidationException", "ScalingPlanName is required");
        }
        let name = input.scaling_plan_name.clone();
        let version = if raw.get("ScalingPlanVersion").is_some() {
            input.scaling_plan_version
        } else {
            1
        };
        let state = state.read().await;
        if state.get_plan(&name, version).is_err() {
            return asp_error_response(&AutoScalingPlansError::PlanNotFound { name, version });
        }
        let response = wire::GetScalingPlanResourceForecastDataResponse {
            datapoints: Some(vec![]),
        };
        wire::serialize_get_scaling_plan_resource_forecast_data_response(&response)
    }

    async fn handle_update_scaling_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<AutoScalingPlansState>>,
        body: &[u8],
    ) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_update_scaling_plan_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scaling_plan_name.is_empty() {
            return json_error_response(400, "ValidationException", "ScalingPlanName is required");
        }
        let name = input.scaling_plan_name.clone();
        if raw.get("ScalingPlanVersion").is_none() {
            return json_error_response(
                400,
                "ValidationException",
                "ScalingPlanVersion is required",
            );
        }
        let version = input.scaling_plan_version;
        let app_source = input
            .application_source
            .as_ref()
            .and_then(|a| serde_json::to_value(a).ok());
        let scaling_instructions = input.scaling_instructions.as_ref().map(|si| {
            si.iter()
                .filter_map(|s| serde_json::to_value(s).ok())
                .collect::<Vec<_>>()
        });
        let mut state = state.write().await;
        match state.update_plan(&name, version, app_source, scaling_instructions) {
            Ok(()) => {
                wire::serialize_update_scaling_plan_response(&wire::UpdateScalingPlanResponse {})
            }
            Err(e) => asp_error_response(&e),
        }
    }
}

fn plan_to_wire(p: &ScalingPlan) -> wire::ScalingPlan {
    wire::ScalingPlan {
        scaling_plan_name: Some(p.scaling_plan_name.clone()),
        scaling_plan_version: Some(p.scaling_plan_version),
        application_source: serde_json::from_value(p.application_source.clone()).ok(),
        scaling_instructions: Some(
            p.scaling_instructions
                .iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect(),
        ),
        status_code: Some(p.status_code.clone()),
        status_message: p.status_message.clone(),
        status_start_time: Some(p.status_start_time as f64),
        creation_time: Some(p.creation_time as f64),
    }
}

fn asp_error_response(err: &AutoScalingPlansError) -> MockResponse {
    let (status, error_type) = match err {
        AutoScalingPlansError::PlanNotFound { .. } => (400, "ObjectNotFoundException"),
        AutoScalingPlansError::PlanAlreadyExists { .. } => (400, "ConcurrentUpdateException"),
        AutoScalingPlansError::Validation { .. } => (400, "ValidationException"),
    };
    json_error_response(status, error_type, &err.to_string())
}
