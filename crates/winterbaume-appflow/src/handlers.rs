use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use uuid::Uuid;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{AppFlowError, AppFlowState};
use crate::types::Flow;
use crate::views::AppFlowStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AppFlowService {
    pub(crate) state: Arc<BackendState<AppFlowState>>,
    pub(crate) notifier: StateChangeNotifier<AppFlowStateView>,
}

impl AppFlowService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppFlowService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppFlowService {
    fn service_name(&self) -> &str {
        "appflow"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://appflow\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AppFlowService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        // Validate JSON body up-front; typed deserialisers re-parse per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "BadRequestException", "Invalid JSON body");
        }

        let query = parse_query_string(extract_query_string(&request.uri));

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(String::as_str).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["create-flow"]) => {
                self.handle_create_flow(&state, account_id, &region, &request, &query)
                    .await
            }
            ("POST", ["describe-flow"]) => {
                self.handle_describe_flow(&state, &request, &query).await
            }
            ("POST", ["delete-flow"]) => self.handle_delete_flow(&state, &request, &query).await,
            ("POST", ["list-flows"]) => self.handle_list_flows(&state, &request, &query).await,
            ("POST", ["update-flow"]) => self.handle_update_flow(&state, &request, &query).await,
            ("POST", ["start-flow"]) => self.handle_start_flow(&state, &request, &query).await,
            ("POST", ["stop-flow"]) => self.handle_stop_flow(&state, &request, &query).await,
            ("POST", ["tags", arn]) => {
                self.handle_tag_resource(&state, arn, &request, &query)
                    .await
            }
            ("DELETE", ["tags", arn]) => {
                self.handle_untag_resource(&state, arn, &request, &query)
                    .await
            }
            ("GET", ["tags", arn]) => self.handle_list_tags(&state, arn, &request, &query).await,
            _ => rest_json_error(
                501,
                "InternalServerException",
                &format!("Operation not implemented: {} {}", method, path),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "DELETE" | "PATCH" | "PUT")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        account_id: &str,
        region: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_flow_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.flow_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "flowName is required");
        }

        let now = chrono::Utc::now().timestamp();
        let user = format!("arn:aws:iam::{account_id}:user/mock-user");
        let arn = format!(
            "arn:aws:appflow:{region}:{account_id}:flow/{}",
            input.flow_name
        );

        let flow = Flow {
            flow_name: input.flow_name.clone(),
            flow_arn: arn,
            description: input.description.clone(),
            kms_arn: input.kms_arn.clone(),
            flow_status: "Active".to_string(),
            flow_status_message: None,
            trigger_config: serde_json::to_value(&input.trigger_config).unwrap_or(json!({})),
            source_flow_config: serde_json::to_value(&input.source_flow_config)
                .unwrap_or(json!({})),
            destination_flow_config_list: serde_json::to_value(&input.destination_flow_config_list)
                .unwrap_or(json!([])),
            tasks: serde_json::to_value(&input.tasks).unwrap_or(json!([])),
            metadata_catalog_config: input
                .metadata_catalog_config
                .as_ref()
                .map(|v| serde_json::to_value(v).unwrap_or(json!({}))),
            created_at: now,
            last_updated_at: now,
            created_by: user.clone(),
            last_updated_by: user,
            schema_version: 1,
            tags: input.tags.unwrap_or_default(),
            last_execution_id: None,
        };

        let mut state = state.write().await;
        match state.create_flow(flow) {
            Ok(f) => {
                let response = wire::CreateFlowResponse {
                    flow_arn: Some(f.flow_arn.clone()),
                    flow_status: Some(f.flow_status.clone()),
                };
                wire::serialize_create_flow_response(&response)
            }
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_describe_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_flow_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.flow_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "flowName is required");
        }
        let name = input.flow_name.as_str();
        let state = state.read().await;
        match state.get_flow(name) {
            Ok(f) => {
                let response = wire::DescribeFlowResponse {
                    flow_arn: Some(f.flow_arn.clone()),
                    flow_name: Some(f.flow_name.clone()),
                    description: f.description.clone(),
                    kms_arn: f.kms_arn.clone(),
                    flow_status: Some(f.flow_status.clone()),
                    flow_status_message: f.flow_status_message.clone(),
                    source_flow_config: serde_json::from_value(f.source_flow_config.clone()).ok(),
                    destination_flow_config_list: serde_json::from_value(
                        f.destination_flow_config_list.clone(),
                    )
                    .ok(),
                    trigger_config: serde_json::from_value(f.trigger_config.clone()).ok(),
                    tasks: serde_json::from_value(f.tasks.clone()).ok(),
                    metadata_catalog_config: f
                        .metadata_catalog_config
                        .clone()
                        .and_then(|v| serde_json::from_value(v).ok()),
                    created_at: Some(f.created_at as f64),
                    last_updated_at: Some(f.last_updated_at as f64),
                    created_by: Some(f.created_by.clone()),
                    last_updated_by: Some(f.last_updated_by.clone()),
                    tags: if f.tags.is_empty() {
                        None
                    } else {
                        Some(f.tags.clone())
                    },
                    schema_version: Some(f.schema_version),
                    last_run_execution_details: None,
                    last_run_metadata_catalog_details: None,
                };
                wire::serialize_describe_flow_response(&response)
            }
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_delete_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_flow_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.flow_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "flowName is required");
        }
        let mut state = state.write().await;
        match state.delete_flow(&input.flow_name) {
            Ok(()) => wire::serialize_delete_flow_response(&wire::DeleteFlowResponse {}),
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_list_flows(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_flows_request(request, &[], query) {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let flows: Vec<wire::FlowDefinition> = state
            .list_flows()
            .iter()
            .map(|f| wire::FlowDefinition {
                flow_name: Some(f.flow_name.clone()),
                flow_arn: Some(f.flow_arn.clone()),
                description: f.description.clone(),
                flow_status: Some(f.flow_status.clone()),
                created_at: Some(f.created_at as f64),
                last_updated_at: Some(f.last_updated_at as f64),
                created_by: Some(f.created_by.clone()),
                last_updated_by: Some(f.last_updated_by.clone()),
                tags: if f.tags.is_empty() {
                    None
                } else {
                    Some(f.tags.clone())
                },
                ..Default::default()
            })
            .collect();
        let response = wire::ListFlowsResponse {
            flows: Some(flows),
            next_token: None,
        };
        wire::serialize_list_flows_response(&response)
    }

    async fn handle_update_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_flow_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.flow_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "flowName is required");
        }

        // Preserve the original "field present" semantics by inspecting the raw body
        // for which optional fields the caller actually sent.
        let raw: Value = if request.body.is_empty() {
            json!({})
        } else {
            serde_json::from_slice(&request.body).unwrap_or(json!({}))
        };

        let now = chrono::Utc::now().timestamp();
        let description = input.description.clone();
        let trigger = if raw.get("triggerConfig").is_some() {
            Some(serde_json::to_value(&input.trigger_config).unwrap_or(json!({})))
        } else {
            None
        };
        let source = if raw.get("sourceFlowConfig").is_some() {
            Some(serde_json::to_value(&input.source_flow_config).unwrap_or(json!({})))
        } else {
            None
        };
        let destinations = if raw.get("destinationFlowConfigList").is_some() {
            Some(serde_json::to_value(&input.destination_flow_config_list).unwrap_or(json!([])))
        } else {
            None
        };
        let tasks = if raw.get("tasks").is_some() {
            Some(serde_json::to_value(&input.tasks).unwrap_or(json!([])))
        } else {
            None
        };
        let metadata = input
            .metadata_catalog_config
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(json!({})));
        let name = input.flow_name.clone();

        let mut state = state.write().await;
        match state.update_flow(&name, |f| {
            if let Some(d) = description {
                f.description = Some(d);
            }
            if let Some(v) = trigger {
                f.trigger_config = v;
            }
            if let Some(v) = source {
                f.source_flow_config = v;
            }
            if let Some(v) = destinations {
                f.destination_flow_config_list = v;
            }
            if let Some(v) = tasks {
                f.tasks = v;
            }
            if let Some(m) = metadata {
                f.metadata_catalog_config = Some(m);
            }
            f.last_updated_at = now;
        }) {
            Ok(f) => {
                let response = wire::UpdateFlowResponse {
                    flow_status: Some(f.flow_status.clone()),
                };
                wire::serialize_update_flow_response(&response)
            }
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_start_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_flow_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.flow_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "flowName is required");
        }
        let name = input.flow_name.clone();
        let execution_id = Uuid::new_v4().to_string();
        let mut state = state.write().await;
        match state.update_flow(&name, |f| {
            f.flow_status = "Active".to_string();
            f.last_execution_id = Some(execution_id.clone());
        }) {
            Ok(f) => {
                let response = wire::StartFlowResponse {
                    flow_arn: Some(f.flow_arn.clone()),
                    flow_status: Some(f.flow_status.clone()),
                    execution_id: Some(execution_id),
                };
                wire::serialize_start_flow_response(&response)
            }
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_stop_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_flow_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.flow_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "flowName is required");
        }
        let name = input.flow_name.clone();
        let mut state = state.write().await;
        match state.update_flow(&name, |f| {
            f.flow_status = "Suspended".to_string();
        }) {
            Ok(f) => {
                let response = wire::StopFlowResponse {
                    flow_arn: Some(f.flow_arn.clone()),
                    flow_status: Some(f.flow_status.clone()),
                };
                wire::serialize_stop_flow_response(&response)
            }
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        arn: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("resourceArn", arn)];
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "BadRequestException", "tags is required");
        }
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        arn: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("resourceArn", arn)];
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.tag_keys.is_empty() {
            return rest_json_error(400, "BadRequestException", "tagKeys is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => appflow_error_response(&e),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFlowState>>,
        arn: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("resourceArn", arn)];
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.list_tags(&input.resource_arn) {
            Ok(tags) => {
                let response = wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                };
                wire::serialize_list_tags_for_resource_response(&response)
            }
            Err(e) => appflow_error_response(&e),
        }
    }
}

fn appflow_error_response(err: &AppFlowError) -> MockResponse {
    let (status, error_type) = match err {
        AppFlowError::FlowNotFound { .. } => (404, "ResourceNotFoundException"),
        AppFlowError::FlowAlreadyExists { .. } => (409, "ConflictException"),
        AppFlowError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        AppFlowError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
