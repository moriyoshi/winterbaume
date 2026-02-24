use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{SsoError, SsoState};
use crate::views::SsoStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SsoService {
    pub(crate) state: Arc<BackendState<SsoState>>,
    pub(crate) notifier: StateChangeNotifier<SsoStateView>,
}

impl SsoService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Get a reference to the backend state for pre-populating test data.
    pub fn backend_state(&self) -> &Arc<BackendState<SsoState>> {
        &self.state
    }
}

impl Default for SsoService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SsoService {
    fn service_name(&self) -> &str {
        "sso"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://portal\.sso\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SsoService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = extract_sso_region(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        // Extract access token from header
        let access_token = request
            .headers
            .get("x-amz-sso_bearer_token")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Extract query parameters
        let query = extract_query(&request.uri);

        let response = match (method, path.as_str()) {
            // GET /assignment/accounts - ListAccounts
            ("GET", "/assignment/accounts") => {
                let token = match &access_token {
                    Some(t) => t,
                    None => {
                        return rest_json_error(
                            401,
                            "UnauthorizedException",
                            "Missing access token",
                        );
                    }
                };
                self.handle_list_accounts(&state, token).await
            }
            // GET /assignment/roles - ListAccountRoles
            ("GET", "/assignment/roles") => {
                let token = match &access_token {
                    Some(t) => t,
                    None => {
                        return rest_json_error(
                            401,
                            "UnauthorizedException",
                            "Missing access token",
                        );
                    }
                };
                let acct_id = match query.get("account_id") {
                    Some(id) => id.clone(),
                    None => {
                        return rest_json_error(
                            400,
                            "InvalidRequestException",
                            "Missing required parameter: account_id",
                        );
                    }
                };
                self.handle_list_account_roles(&state, token, &acct_id)
                    .await
            }
            // GET /federation/credentials - GetRoleCredentials
            ("GET", "/federation/credentials") => {
                let token = match &access_token {
                    Some(t) => t,
                    None => {
                        return rest_json_error(
                            401,
                            "UnauthorizedException",
                            "Missing access token",
                        );
                    }
                };
                let acct_id = match query.get("account_id") {
                    Some(id) => id.clone(),
                    None => {
                        return rest_json_error(
                            400,
                            "InvalidRequestException",
                            "Missing required parameter: account_id",
                        );
                    }
                };
                let role_name = match query.get("role_name") {
                    Some(r) => r.clone(),
                    None => {
                        return rest_json_error(
                            400,
                            "InvalidRequestException",
                            "Missing required parameter: role_name",
                        );
                    }
                };
                self.handle_get_role_credentials(&state, token, &acct_id, &role_name)
                    .await
            }
            // POST /logout - Logout
            ("POST", "/logout") => {
                let token = match &access_token {
                    Some(t) => t,
                    None => {
                        return rest_json_error(
                            401,
                            "UnauthorizedException",
                            "Missing access token",
                        );
                    }
                };
                self.handle_logout(&state, token).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_list_accounts(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoState>>,
        token: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if let Err(e) = state.validate_token(token) {
            return sso_error_response(&e);
        }
        let accounts = state.list_accounts();
        let entries: Vec<crate::model::AccountInfo> =
            accounts.iter().map(|a| account_to_model(a)).collect();
        wire::serialize_list_accounts_response(&crate::model::ListAccountsResponse {
            account_list: Some(entries),
            next_token: None,
        })
    }

    async fn handle_list_account_roles(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoState>>,
        token: &str,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if let Err(e) = state.validate_token(token) {
            return sso_error_response(&e);
        }
        match state.list_account_roles(account_id) {
            Ok(roles) => {
                let entries: Vec<crate::model::RoleInfo> =
                    roles.iter().map(|r| role_to_model(r)).collect();
                wire::serialize_list_account_roles_response(
                    &crate::model::ListAccountRolesResponse {
                        role_list: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => sso_error_response(&e),
        }
    }

    async fn handle_get_role_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoState>>,
        token: &str,
        account_id: &str,
        role_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if let Err(e) = state.validate_token(token) {
            return sso_error_response(&e);
        }
        match state.get_role_credentials(account_id, role_name) {
            Ok(creds) => wire::serialize_get_role_credentials_response(
                &crate::model::GetRoleCredentialsResponse {
                    role_credentials: Some(credentials_to_model(&creds)),
                },
            ),
            Err(e) => sso_error_response(&e),
        }
    }

    async fn handle_logout(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoState>>,
        token: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.logout(token) {
            Ok(()) => wire::serialize_logout_response(),
            Err(e) => sso_error_response(&e),
        }
    }
}

// --- Conversion helpers ---

fn account_to_model(a: &crate::types::SsoAccount) -> crate::model::AccountInfo {
    crate::model::AccountInfo {
        account_id: Some(a.account_id.clone()),
        account_name: Some(a.account_name.clone()),
        email_address: Some(a.email_address.clone()),
    }
}

fn role_to_model(r: &crate::types::SsoAccountRole) -> crate::model::RoleInfo {
    crate::model::RoleInfo {
        account_id: Some(r.account_id.clone()),
        role_name: Some(r.role_name.clone()),
    }
}

fn credentials_to_model(c: &crate::types::SsoRoleCredentials) -> crate::model::RoleCredentials {
    crate::model::RoleCredentials {
        access_key_id: Some(c.access_key_id.clone()),
        secret_access_key: Some(c.secret_access_key.clone()),
        session_token: Some(c.session_token.clone()),
        expiration: Some(c.expiration),
    }
}

/// Extract region from SSO URL pattern: portal.sso.{region}.amazonaws.com
fn extract_sso_region(uri: &str) -> String {
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))
        .unwrap_or(uri);
    let host = after_scheme.split('/').next().unwrap_or("");
    let host = host.split(':').next().unwrap_or("");
    let parts: Vec<&str> = host.split('.').collect();
    // portal.sso.{region}.amazonaws.com => parts[2] is the region
    if parts.len() >= 5 && parts[0] == "portal" && parts[1] == "sso" {
        return parts[2].to_string();
    }
    // Fallback: try standard extraction
    if parts.len() >= 4 && parts[parts.len() - 2] == "amazonaws" {
        return parts[1].to_string();
    }
    "us-east-1".to_string()
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        let path_part = uri.split("//").nth(1).unwrap_or(uri);
        let after_host = path_part.find('/').map(|i| &path_part[i..]).unwrap_or("/");
        after_host.split('?').next().unwrap_or("/").to_string()
    }
}

fn extract_query(uri: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if let Some(q) = uri.find('?') {
        let query_str = &uri[q + 1..];
        for pair in query_str.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                map.insert(percent_decode(key), percent_decode(value));
            }
        }
    }
    map
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn sso_error_response(err: &SsoError) -> MockResponse {
    let (status, error_type) = match err {
        SsoError::InvalidAccessToken => (401, "UnauthorizedException"),
        SsoError::SessionInvalidated => (401, "UnauthorizedException"),
        SsoError::AccountNotFound { .. } => (404, "ResourceNotFoundException"),
        SsoError::RoleNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
