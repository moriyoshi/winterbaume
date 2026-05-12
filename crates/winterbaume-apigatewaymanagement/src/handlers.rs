use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{ApiGatewayManagementApiError, ApiGatewayManagementApiState};
use crate::views::ApiGatewayManagementApiStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ApiGatewayManagementService {
    pub(crate) state: Arc<BackendState<ApiGatewayManagementApiState>>,
    pub(crate) notifier: StateChangeNotifier<ApiGatewayManagementApiStateView>,
}

impl ApiGatewayManagementService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApiGatewayManagementService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApiGatewayManagementService {
    fn service_name(&self) -> &str {
        "apigatewaymanagementapi"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://([^.]+\.)*execute-api\.[^.]+\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ApiGatewayManagementService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        // Path: /{stage}/@connections/{connectionId}
        // Segments after trimming leading slash: ["stage", "@connections", "connectionId"]
        // or with empty stage: ["@connections", "connectionId"]
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Find the "@connections" segment index
        let conn_idx = segments.iter().position(|s| *s == "@connections");

        let response = match (method, conn_idx) {
            // GET /{stage}/@connections/{connectionId}
            ("GET", Some(idx)) if idx + 1 < segments.len() => {
                let connection_id = percent_decode(segments[idx + 1]);
                self.handle_get_connection(&state, &connection_id).await
            }
            // DELETE /{stage}/@connections/{connectionId}
            ("DELETE", Some(idx)) if idx + 1 < segments.len() => {
                let connection_id = percent_decode(segments[idx + 1]);
                self.handle_delete_connection(&state, &connection_id).await
            }
            // POST /{stage}/@connections/{connectionId}
            ("POST", Some(idx)) if idx + 1 < segments.len() => {
                let connection_id = percent_decode(segments[idx + 1]);
                self.handle_post_to_connection(&state, &connection_id, request.body.to_vec())
                    .await
            }
            _ => error_response(ApiGatewayManagementApiError::NotFound),
        };

        if response.status / 100 == 2 && matches!(method, "DELETE" | "POST") {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_get_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<ApiGatewayManagementApiState>>,
        connection_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        let conn = guard.get_connection(connection_id);
        wire::serialize_get_connection_response(&wire::GetConnectionResponse {
            connected_at: Some(conn.connected_at.to_rfc3339()),
            last_active_at: Some(conn.last_active_at.to_rfc3339()),
            identity: Some(wire::Identity {
                source_ip: Some(conn.source_ip.clone()),
                user_agent: Some(conn.user_agent.clone()),
            }),
        })
    }

    async fn handle_delete_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<ApiGatewayManagementApiState>>,
        connection_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        guard.delete_connection(connection_id);
        wire::serialize_delete_connection_response()
    }

    async fn handle_post_to_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<ApiGatewayManagementApiState>>,
        connection_id: &str,
        data: Vec<u8>,
    ) -> MockResponse {
        let mut guard = state.write().await;
        guard.post_to_connection(connection_id, data);
        wire::serialize_post_to_connection_response()
    }
}

fn extract_path(uri: &str) -> String {
    let without_query = uri.split('?').next().unwrap_or(uri);
    // Strip scheme and host
    if let Some(path_start) = without_query.find("://") {
        let after_scheme = &without_query[path_start + 3..];
        if let Some(slash) = after_scheme.find('/') {
            return after_scheme[slash..].to_string();
        }
    }
    without_query.to_string()
}

fn percent_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            let hex = format!("{}{}", h1, h2);
            if let Ok(b) = u8::from_str_radix(&hex, 16) {
                result.push(b as char);
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn error_response(err: ApiGatewayManagementApiError) -> MockResponse {
    let message = err.to_string();
    let (status, error_type) = match err {
        ApiGatewayManagementApiError::NotFound => (404u16, "NotFoundException"),
    };
    rest_json_error(status, error_type, &message)
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
