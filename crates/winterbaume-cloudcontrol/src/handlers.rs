use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::cfn_schema::ShapeContext;
use crate::state::{CloudControlError, CloudControlState};
use crate::types::ResourceRequest;
use crate::views::CloudControlStateView;
use crate::wire;

pub struct CloudControlService {
    pub(crate) state: Arc<BackendState<CloudControlState>>,
    pub(crate) notifier: StateChangeNotifier<CloudControlStateView>,
}

impl CloudControlService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudControlService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudControlService {
    fn service_name(&self) -> &str {
        "cloudcontrolapi"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://cloudcontrolapi\..*\.amazonaws\.com",
            r"https?://cloudcontrolapi\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

/// Map domain errors to AWS-facing error responses.
/// Exhaustive match -- no wildcard arm.
fn service_error_response(err: &CloudControlError) -> MockResponse {
    let (status, error_type) = match err {
        CloudControlError::AlreadyExists { .. } => (400, "AlreadyExistsException"),
        CloudControlError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        CloudControlError::RequestTokenNotFound { .. } => (404, "RequestTokenNotFoundException"),
        CloudControlError::TypeNotFound { .. } => (404, "TypeNotFoundException"),
        CloudControlError::InvalidRequest { .. } => (400, "InvalidRequestException"),
        CloudControlError::NotCancellable { .. } => (400, "ConcurrentModificationException"),
    };
    json_error_response(status, error_type, &err.to_string())
}

/// Mutating action names for state change notification.
const MUTATING_ACTIONS: &[&str] = &[
    "CreateResource",
    "DeleteResource",
    "UpdateResource",
    "CancelResourceRequest",
];

impl CloudControlService {
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

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let ctx = ShapeContext {
            region: &region,
            account_id,
        };

        let response = match action.as_str() {
            "CreateResource" => self.handle_create_resource(&state, body_bytes, &ctx).await,
            "DeleteResource" => self.handle_delete_resource(&state, body_bytes).await,
            "UpdateResource" => self.handle_update_resource(&state, body_bytes, &ctx).await,
            "GetResource" => self.handle_get_resource(&state, body_bytes).await,
            "ListResources" => self.handle_list_resources(&state, body_bytes).await,
            "GetResourceRequestStatus" => {
                self.handle_get_resource_request_status(&state, body_bytes)
                    .await
            }
            "ListResourceRequests" => self.handle_list_resource_requests(&state, body_bytes).await,
            "CancelResourceRequest" => {
                self.handle_cancel_resource_request(&state, body_bytes)
                    .await
            }
            _ => json_error_response(
                501,
                "NotImplementedError",
                &format!(
                    "{} is not yet implemented in winterbaume-cloudcontrol",
                    action
                ),
            ),
        };

        // Notify state change for mutating actions
        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            use winterbaume_core::StatefulService;
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_create_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
        ctx: &ShapeContext<'_>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.type_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'TypeName'",
            );
        }
        if input.desired_state.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'DesiredState'",
            );
        }
        let type_name = input.type_name.as_str();
        let desired_state = input.desired_state.as_str();

        let mut guard = state.write().await;
        match guard.create_resource(type_name, desired_state, ctx) {
            Ok(request) => create_response(&request),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.type_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'TypeName'",
            );
        }
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'Identifier'",
            );
        }
        let type_name = input.type_name.as_str();
        let identifier = input.identifier.as_str();

        let mut guard = state.write().await;
        match guard.delete_resource(type_name, identifier) {
            Ok(request) => delete_response(&request),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_update_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
        ctx: &ShapeContext<'_>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.type_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'TypeName'",
            );
        }
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'Identifier'",
            );
        }
        if input.patch_document.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'PatchDocument'",
            );
        }
        let type_name = input.type_name.as_str();
        let identifier = input.identifier.as_str();
        let patch_document = input.patch_document.as_str();

        let mut guard = state.write().await;
        match guard.update_resource(type_name, identifier, patch_document, ctx) {
            Ok(request) => update_response(&request),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.type_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'TypeName'",
            );
        }
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'Identifier'",
            );
        }
        let type_name = input.type_name.as_str();
        let identifier = input.identifier.as_str();

        let guard = state.read().await;
        match guard.get_resource(type_name, identifier) {
            Ok(resource) => wire::serialize_get_resource_response(&wire::GetResourceOutput {
                type_name: Some(resource.type_name.clone()),
                resource_description: Some(wire::ResourceDescription {
                    identifier: Some(resource.identifier.clone()),
                    properties: Some(resource.resource_model.clone()),
                }),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_resources_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.type_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'TypeName'",
            );
        }
        let type_name = input.type_name.as_str();

        let guard = state.read().await;
        let resources = guard.list_resources(type_name);
        let descriptions: Vec<wire::ResourceDescription> = resources
            .iter()
            .map(|r| wire::ResourceDescription {
                identifier: Some(r.identifier.clone()),
                properties: Some(r.resource_model.clone()),
            })
            .collect();

        wire::serialize_list_resources_response(&wire::ListResourcesOutput {
            type_name: Some(type_name.to_string()),
            resource_descriptions: Some(descriptions),
            next_token: None,
        })
    }

    async fn handle_get_resource_request_status(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_request_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.request_token.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'RequestToken'",
            );
        }
        let request_token = input.request_token.as_str();

        let guard = state.read().await;
        match guard.get_resource_request_status(request_token) {
            Ok(request) => wire::serialize_get_resource_request_status_response(
                &wire::GetResourceRequestStatusOutput {
                    progress_event: Some(build_progress_event(request)),
                    hooks_progress_event: None,
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_resource_requests(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_resource_requests_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let filter = input.resource_request_status_filter;
        let operation_filter: Option<Vec<String>> =
            filter.as_ref().and_then(|f| f.operations.clone());
        let status_filter: Option<Vec<String>> =
            filter.as_ref().and_then(|f| f.operation_statuses.clone());

        let op_refs: Option<Vec<&str>> = operation_filter
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect());
        let status_refs: Option<Vec<&str>> = status_filter
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect());

        let guard = state.read().await;
        let requests = guard.list_resource_requests(op_refs.as_deref(), status_refs.as_deref());
        let summaries: Vec<wire::ProgressEvent> =
            requests.iter().map(|r| build_progress_event(r)).collect();

        wire::serialize_list_resource_requests_response(&wire::ListResourceRequestsOutput {
            resource_request_status_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_cancel_resource_request(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudControlState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_resource_request_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.request_token.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'RequestToken'",
            );
        }
        let request_token = input.request_token.as_str();

        let mut guard = state.write().await;
        match guard.cancel_resource_request(request_token) {
            Ok(request) => cancel_response(&request),
            Err(e) => service_error_response(&e),
        }
    }
}

/// Build a ProgressEvent wire type from a ResourceRequest.
fn build_progress_event(request: &ResourceRequest) -> wire::ProgressEvent {
    wire::ProgressEvent {
        type_name: Some(request.type_name.clone()),
        identifier: Some(request.identifier.clone()),
        request_token: Some(request.request_token.clone()),
        hooks_request_token: None,
        operation: Some(request.operation.as_str().to_string()),
        operation_status: Some(request.operation_status.as_str().to_string()),
        event_time: Some(request.event_time.timestamp() as f64),
        resource_model: request.resource_model.clone(),
        status_message: request.status_message.clone(),
        error_code: request.error_code.clone(),
        retry_after: None,
    }
}

/// Build a create response wrapping a ProgressEvent.
fn create_response(request: &ResourceRequest) -> MockResponse {
    wire::serialize_create_resource_response(&wire::CreateResourceOutput {
        progress_event: Some(build_progress_event(request)),
    })
}

/// Build a delete response wrapping a ProgressEvent.
fn delete_response(request: &ResourceRequest) -> MockResponse {
    wire::serialize_delete_resource_response(&wire::DeleteResourceOutput {
        progress_event: Some(build_progress_event(request)),
    })
}

/// Build an update response wrapping a ProgressEvent.
fn update_response(request: &ResourceRequest) -> MockResponse {
    wire::serialize_update_resource_response(&wire::UpdateResourceOutput {
        progress_event: Some(build_progress_event(request)),
    })
}

/// Build a cancel response wrapping a ProgressEvent.
fn cancel_response(request: &ResourceRequest) -> MockResponse {
    wire::serialize_cancel_resource_request_response(&wire::CancelResourceRequestOutput {
        progress_event: Some(build_progress_event(request)),
    })
}
