use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    protocol::common::{extract_path, percent_decode},
};

use crate::state::ConnectContactLensState;
use crate::views::ConnectContactLensStateView;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ConnectContactLensService {
    pub(crate) state: Arc<BackendState<ConnectContactLensState>>,
    pub(crate) notifier: StateChangeNotifier<ConnectContactLensStateView>,
}

impl ConnectContactLensService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ConnectContactLensService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ConnectContactLensService {
    fn service_name(&self) -> &str {
        "contact-lens"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://contact-lens\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ConnectContactLensService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segments: Vec<&str> = raw_segments.iter().map(|s| s.as_str()).collect();
        let method = request.method.as_str().to_string();
        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            serde_json::from_slice(&request.body).unwrap_or(json!({}))
        };

        match (method.as_str(), segments.as_slice()) {
            ("POST", ["realtime-contact-analysis", "analysis-segments"]) => {
                let instance_id = body
                    .get("InstanceId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let contact_id = body
                    .get("ContactId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                if instance_id.is_empty() || contact_id.is_empty() {
                    return rest_json_error(
                        400,
                        "InvalidRequestException",
                        "InstanceId and ContactId are required",
                    );
                }
                let guard = state.read().await;
                let segments = guard.get_segments(&instance_id, &contact_id);
                rest_json_response(200, json!({"Segments": segments, "NextToken": Value::Null}))
            }
            _ => rest_json_error(404, "ResourceNotFoundException", "No route matches"),
        }
    }
}

fn rest_json_response(status: u16, body: Value) -> MockResponse {
    MockResponse::rest_json(status, body.to_string())
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
