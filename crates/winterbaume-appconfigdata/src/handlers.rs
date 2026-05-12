use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, rest_json_error,
};

use crate::state::{AppConfigDataError, AppConfigDataState};
use crate::views::AppConfigDataStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AppConfigDataService {
    pub(crate) state: Arc<BackendState<AppConfigDataState>>,
    pub(crate) notifier: StateChangeNotifier<AppConfigDataStateView>,
}

impl AppConfigDataService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppConfigDataService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppConfigDataService {
    fn service_name(&self) -> &str {
        "appconfigdata"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://appconfigdata\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AppConfigDataService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let response = match (method.as_str(), path.as_str()) {
            ("POST", "/configurationsessions") => {
                self.handle_start_configuration_session(&state, &body).await
            }
            ("GET", "/configuration") => {
                self.handle_get_latest_configuration(&state, &request.uri)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method == "POST" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_start_configuration_session(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigDataState>>,
        body: &Value,
    ) -> MockResponse {
        let app = match body.get("ApplicationIdentifier").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => {
                return rest_json_error(
                    400,
                    "BadRequestException",
                    "ApplicationIdentifier is required",
                );
            }
        };
        let env = match body.get("EnvironmentIdentifier").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => {
                return rest_json_error(
                    400,
                    "BadRequestException",
                    "EnvironmentIdentifier is required",
                );
            }
        };
        let profile = match body
            .get("ConfigurationProfileIdentifier")
            .and_then(|v| v.as_str())
        {
            Some(v) => v,
            None => {
                return rest_json_error(
                    400,
                    "BadRequestException",
                    "ConfigurationProfileIdentifier is required",
                );
            }
        };
        let min_poll = body
            .get("RequiredMinimumPollIntervalInSeconds")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let mut state = state.write().await;
        let session = state.start_session(app, env, profile, min_poll);
        let response = wire::StartConfigurationSessionResponse {
            initial_configuration_token: Some(session.token.clone()),
        };
        let mut resp = wire::serialize_start_configuration_session_response(&response);
        resp.status = 201;
        resp
    }

    async fn handle_get_latest_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigDataState>>,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let token = match qs.get("configuration_token") {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => {
                return rest_json_error(
                    400,
                    "BadRequestException",
                    "configuration_token is required",
                );
            }
        };

        // Validate the token, then rotate it for the next poll.
        let mut state = state.write().await;
        if state.get_session(&token).is_err() {
            return appconfigdata_error_response(&AppConfigDataError::BadConfigurationToken {
                token,
            });
        }
        let next_token = match state.rotate_token(&token) {
            Ok(t) => t,
            Err(e) => return appconfigdata_error_response(&e),
        };

        // Empty configuration body. Real AppConfig integration would source this
        // from the appconfig crate's hosted-configuration-version state.
        let mut resp = MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
        };
        resp.headers.insert(
            HeaderName::from_static("next-poll-configuration-token"),
            next_token.parse().unwrap(),
        );
        resp.headers.insert(
            HeaderName::from_static("next-poll-interval-in-seconds"),
            "60".parse().unwrap(),
        );
        resp.headers.insert(
            http::header::CONTENT_TYPE,
            "application/octet-stream".parse().unwrap(),
        );
        resp
    }
}

fn appconfigdata_error_response(err: &AppConfigDataError) -> MockResponse {
    let (status, error_type) = match err {
        AppConfigDataError::BadConfigurationToken { .. } => (400, "BadRequestException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
