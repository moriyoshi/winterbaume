use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{ControlCatalogError, ControlCatalogState};
use crate::views::ControlCatalogStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ControlCatalogService {
    pub(crate) state: Arc<BackendState<ControlCatalogState>>,
    pub(crate) notifier: StateChangeNotifier<ControlCatalogStateView>,
}

impl ControlCatalogService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ControlCatalogService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ControlCatalogService {
    fn service_name(&self) -> &str {
        "controlcatalog"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://controlcatalog\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<ControlCatalogState>>;

impl ControlCatalogService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/');

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let segments: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        match segs.as_slice() {
            ["get-control"] => self.handle_get_control(&state, &body).await,
            ["common-controls"] => self.handle_list_common_controls(&state).await,
            ["list-control-mappings"] => self.handle_list_control_mappings().await,
            ["list-controls"] => self.handle_list_controls(&state).await,
            ["domains"] => self.handle_list_domains(&state).await,
            ["objectives"] => self.handle_list_objectives(&state).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_get_control(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = body
            .get("ControlArn")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if arn.is_empty() {
            return rest_json_error(400, "ValidationException", "ControlArn is required");
        }
        let mut state = state.write().await;
        state.ensure_seeded();
        match state.get_control(arn) {
            Ok(v) => {
                if let Ok(parsed) = serde_json::from_value::<wire::GetControlResponse>(v.clone()) {
                    wire::serialize_get_control_response(&parsed)
                } else {
                    rest_json_error(500, "InternalServerException", "Decode failure")
                }
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_controls(&self, state: &SharedState) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_seeded();
        let controls: Vec<wire::ControlSummary> = state
            .list_controls()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_controls_response(&wire::ListControlsResponse {
            controls: Some(controls),
            next_token: None,
        })
    }

    async fn handle_list_domains(&self, state: &SharedState) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_seeded();
        let domains: Vec<wire::DomainSummary> = state
            .list_domains()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_domains_response(&wire::ListDomainsResponse {
            domains: Some(domains),
            next_token: None,
        })
    }

    async fn handle_list_objectives(&self, state: &SharedState) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_seeded();
        let objectives: Vec<wire::ObjectiveSummary> = state
            .list_objectives()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_objectives_response(&wire::ListObjectivesResponse {
            next_token: None,
            objectives: Some(objectives),
        })
    }

    async fn handle_list_common_controls(&self, state: &SharedState) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_seeded();
        let common_controls: Vec<wire::CommonControlSummary> = state
            .list_common_controls()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_common_controls_response(&wire::ListCommonControlsResponse {
            common_controls: Some(common_controls),
            next_token: None,
        })
    }

    async fn handle_list_control_mappings(&self) -> MockResponse {
        wire::serialize_list_control_mappings_response(&wire::ListControlMappingsResponse {
            control_mappings: Some(vec![]),
            next_token: None,
        })
    }
}

fn err_response(err: &ControlCatalogError) -> MockResponse {
    let (status, error_type) = match err {
        ControlCatalogError::ControlNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
