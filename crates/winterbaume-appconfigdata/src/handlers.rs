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
    /// Optional shared `winterbaume-appconfig` state. When wired,
    /// `GetLatestConfiguration` resolves the session's
    /// `( application_id, environment_id, configuration_profile_id )`
    /// against the parent crate's deployed-configuration state and
    /// returns the actual content. When `None` ( the default for
    /// `Self::new()` ), the legacy empty-body behaviour is preserved.
    pub(crate) appconfig_state: Option<Arc<BackendState<winterbaume_appconfig::AppConfigState>>>,
}

impl AppConfigDataService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            appconfig_state: None,
        }
    }

    /// Construct an `AppConfigDataService` that resolves
    /// `GetLatestConfiguration` against the parent `winterbaume-appconfig`
    /// state.
    ///
    /// Pass the same `Arc` returned by
    /// [`winterbaume_appconfig::AppConfigService::shared_state`] so the
    /// control plane ( `winterbaume-appconfig`'s
    /// `CreateHostedConfigurationVersion` + `StartDeployment` ) and the
    /// data plane ( `winterbaume-appconfigdata`'s `GetLatestConfiguration` )
    /// agree on what is deployed:
    ///
    /// ```no_run
    /// use winterbaume_appconfig::AppConfigService;
    /// use winterbaume_appconfigdata::AppConfigDataService;
    /// use winterbaume_core::MockAws;
    ///
    /// let appconfig = AppConfigService::new();
    /// let appconfig_data = AppConfigDataService::with_appconfig_state(
    ///     appconfig.shared_state(),
    /// );
    /// let _mock = MockAws::builder()
    ///     .with_service(appconfig)
    ///     .with_service(appconfig_data)
    ///     .build();
    /// ```
    pub fn with_appconfig_state(
        appconfig_state: Arc<BackendState<winterbaume_appconfig::AppConfigState>>,
    ) -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            appconfig_state: Some(appconfig_state),
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
                self.handle_get_latest_configuration(&state, &request.uri, account_id, &region)
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
        account_id: &str,
        region: &str,
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

        // Validate the token, capture the session's resolution keys, then
        // rotate the token for the next poll.
        let mut state = state.write().await;
        let session = match state.get_session(&token) {
            Ok(s) => s.clone(),
            Err(_) => {
                return appconfigdata_error_response(&AppConfigDataError::BadConfigurationToken {
                    token,
                });
            }
        };
        let next_token = match state.rotate_token(&token) {
            Ok(t) => t,
            Err(e) => return appconfigdata_error_response(&e),
        };
        drop(state);

        // Resolve the deployed configuration through the parent
        // `winterbaume-appconfig` state when wired. Without it, fall back
        // to the legacy empty body so callers that only ever use the data
        // plane keep working.
        let (content_type, body): (String, bytes::Bytes) =
            if let Some(appconfig_state) = self.appconfig_state.as_ref() {
                let parent = appconfig_state.get(account_id, region);
                let parent = parent.read().await;
                match parent.get_deployed_configuration(
                    &session.application_id,
                    &session.environment_id,
                    &session.configuration_profile_id,
                ) {
                    Some((ct, content)) => (ct.to_string(), bytes::Bytes::copy_from_slice(content)),
                    None => ("application/octet-stream".to_string(), bytes::Bytes::new()),
                }
            } else {
                ("application/octet-stream".to_string(), bytes::Bytes::new())
            };

        let mut resp = MockResponse {
            status: 200,
            headers: http::HeaderMap::new(),
            body,
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
            content_type
                .parse()
                .unwrap_or_else(|_| "application/octet-stream".parse().unwrap()),
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
