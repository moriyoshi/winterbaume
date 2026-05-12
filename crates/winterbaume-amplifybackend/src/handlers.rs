use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use uuid::Uuid;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{AmplifyBackendError, AmplifyBackendState};
use crate::views::AmplifyBackendStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AmplifyBackendService {
    pub(crate) state: Arc<BackendState<AmplifyBackendState>>,
    pub(crate) notifier: StateChangeNotifier<AmplifyBackendStateView>,
}

impl AmplifyBackendService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AmplifyBackendService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AmplifyBackendService {
    fn service_name(&self) -> &str {
        "amplifybackend"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://amplifybackend\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AmplifyBackendService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = match request.uri.find('?') {
            Some(idx) => request.uri[idx + 1..].to_string(),
            None => String::new(),
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str().to_uppercase();

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(String::as_str).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["backend"]) => {
                self.handle_create_backend(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", ["backend", app_id, "details"]) => {
                let labels: &[(&str, &str)] = &[("AppId", app_id)];
                self.handle_get_backend(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["backend", app_id, "environments", env, "remove"]) => {
                let labels: &[(&str, &str)] = &[("AppId", app_id), ("BackendEnvironmentName", env)];
                self.handle_delete_backend(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["backend", app_id, "environments", env, "clone"]) => {
                let labels: &[(&str, &str)] = &[("AppId", app_id), ("BackendEnvironmentName", env)];
                self.handle_clone_backend(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(
                501,
                "NotImplementedException",
                &format!("Operation not implemented: {} {}", method, path),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method == "POST" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_backend(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyBackendState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_backend_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.app_id.is_empty() {
            return rest_json_error(400, "BadRequestException", "appId is required");
        }
        if input.app_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "appName is required");
        }
        if input.backend_environment_name.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "backendEnvironmentName is required",
            );
        }

        let mut state = state.write().await;
        match state.create_backend(
            &input.app_id,
            &input.app_name,
            &input.backend_environment_name,
            input.resource_name.clone(),
        ) {
            Ok(_) => {
                let response = wire::CreateBackendResponse {
                    app_id: Some(input.app_id.clone()),
                    backend_environment_name: Some(input.backend_environment_name.clone()),
                    job_id: Some(Uuid::new_v4().to_string()),
                    operation: Some("CREATE".to_string()),
                    status: Some("COMPLETED".to_string()),
                    error: None,
                };
                wire::serialize_create_backend_response(&response)
            }
            Err(e) => amplifybackend_error_response(&e),
        }
    }

    async fn handle_get_backend(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyBackendState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_backend_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let app_id = input.app_id.as_str();
        let env_filter = input.backend_environment_name.as_deref();

        let state = state.read().await;
        let envs = state.list_environments_for_app(app_id);
        if envs.is_empty() {
            return amplifybackend_error_response(&AmplifyBackendError::NotFound {
                app_id: app_id.to_string(),
                environment: env_filter.unwrap_or("").to_string(),
            });
        }

        // If a specific environment was requested, return its details.
        let env_name = match env_filter {
            Some(e) => {
                if !envs.contains(&e.to_string()) {
                    return amplifybackend_error_response(&AmplifyBackendError::NotFound {
                        app_id: app_id.to_string(),
                        environment: e.to_string(),
                    });
                }
                e.to_string()
            }
            None => envs.first().unwrap().clone(),
        };

        let backend = match state.get_backend(app_id, &env_name) {
            Ok(b) => b,
            Err(e) => return amplifybackend_error_response(&e),
        };

        let response = wire::GetBackendResponse {
            app_id: Some(backend.app_id.clone()),
            app_name: Some(backend.app_name.clone()),
            backend_environment_name: Some(backend.backend_environment_name.clone()),
            backend_environment_list: Some(envs),
            amplify_meta_config: backend.amplify_meta_config.clone(),
            amplify_feature_flags: backend.amplify_feature_flags.clone(),
            error: None,
        };
        wire::serialize_get_backend_response(&response)
    }

    async fn handle_delete_backend(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyBackendState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_backend_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_backend(&input.app_id, &input.backend_environment_name) {
            Ok(()) => {
                let response = wire::DeleteBackendResponse {
                    app_id: Some(input.app_id.clone()),
                    backend_environment_name: Some(input.backend_environment_name.clone()),
                    job_id: Some(Uuid::new_v4().to_string()),
                    operation: Some("DELETE".to_string()),
                    status: Some("COMPLETED".to_string()),
                    error: None,
                };
                wire::serialize_delete_backend_response(&response)
            }
            Err(e) => amplifybackend_error_response(&e),
        }
    }

    async fn handle_clone_backend(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyBackendState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_clone_backend_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.target_environment_name.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "targetEnvironmentName is required",
            );
        }
        let mut state = state.write().await;
        match state.clone_backend(
            &input.app_id,
            &input.backend_environment_name,
            &input.target_environment_name,
        ) {
            Ok(_) => {
                let response = wire::CloneBackendResponse {
                    app_id: Some(input.app_id.clone()),
                    backend_environment_name: Some(input.target_environment_name.clone()),
                    job_id: Some(Uuid::new_v4().to_string()),
                    operation: Some("CLONE".to_string()),
                    status: Some("COMPLETED".to_string()),
                    error: None,
                };
                wire::serialize_clone_backend_response(&response)
            }
            Err(e) => amplifybackend_error_response(&e),
        }
    }
}

fn amplifybackend_error_response(err: &AmplifyBackendError) -> MockResponse {
    let (status, error_type) = match err {
        AmplifyBackendError::NotFound { .. } => (404, "NotFoundException"),
        AmplifyBackendError::AlreadyExists { .. } => (400, "BadRequestException"),
        AmplifyBackendError::Validation { .. } => (400, "BadRequestException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
