use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{WorkspacesWebError, WorkspacesWebState};
use crate::types;
use crate::views::WorkspacesWebStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct WorkspacesWebService {
    pub(crate) state: Arc<BackendState<WorkspacesWebState>>,
    pub(crate) notifier: StateChangeNotifier<WorkspacesWebStateView>,
}

impl WorkspacesWebService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for WorkspacesWebService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for WorkspacesWebService {
    fn service_name(&self) -> &str {
        "workspaces-web"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://workspaces-web\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn tags_from_input(tags: &[wire::Tag]) -> HashMap<String, String> {
    tags.iter()
        .map(|t| (t.key.clone(), t.value.clone()))
        .collect()
}

impl WorkspacesWebService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        if segments.is_empty() || segments[0].is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let query_map = parse_query_string(&query_string);

        let response = match segments[0] {
            "portals" => {
                self.dispatch_portals(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "portalIdp" => {
                self.dispatch_portal_idp(method, &segments, &request, &state, &query_map)
                    .await
            }
            "browserSettings" => {
                self.dispatch_browser_settings(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "networkSettings" => {
                self.dispatch_network_settings(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "userAccessLoggingSettings" => {
                self.dispatch_user_access_logging_settings(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "userSettings" => {
                self.dispatch_user_settings(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "identityProviders" => {
                self.dispatch_identity_providers(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "ipAccessSettings" => {
                self.dispatch_ip_access_settings(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "trustStores" => {
                self.dispatch_trust_stores(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "dataProtectionSettings" => {
                self.dispatch_data_protection_settings(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "sessionLoggers" => {
                self.dispatch_session_loggers(
                    method, &segments, &request, &state, account_id, &region, &query_map,
                )
                .await
            }
            "tags" => {
                self.dispatch_tags(method, &segments, &request, &state, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ── Portal routes ───────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_portals(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_portal(state, request, &[], query, account_id, region)
                        .await
                }
                "GET" => self.handle_list_portals(state, request, &[], query).await,
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            // Check if the last segment is an association sub-resource
            let last = *segments.last().unwrap();
            // Handle /portals/{portalId}/sessions[/{sessionId}]
            if let Some(sessions_idx) = segments.iter().position(|&s| s == "sessions") {
                let portal_id = decode_arn_from_segments(&segments[1..sessions_idx]);
                if sessions_idx + 1 >= segments.len() {
                    if method == "GET" {
                        let labels: &[(&str, &str)] = &[("portalId", portal_id.as_str())];
                        return self
                            .handle_list_sessions(state, request, labels, query)
                            .await;
                    }
                } else {
                    let session_id = decode_arn_from_segments(&segments[sessions_idx + 1..]);
                    let labels: &[(&str, &str)] = &[
                        ("portalId", portal_id.as_str()),
                        ("sessionId", session_id.as_str()),
                    ];
                    match method {
                        "GET" => {
                            return self.handle_get_session(state, request, labels, query).await;
                        }
                        "DELETE" => {
                            return self
                                .handle_expire_session(state, request, labels, query)
                                .await;
                        }
                        _ => {}
                    }
                }
            }
            // Handle /portals/{portalArn+}/identityProviders
            if last == "identityProviders" && method == "GET" {
                let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                return self
                    .handle_list_identity_providers(state, request, labels, query)
                    .await;
            }
            match last {
                "browserSettings" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_browser_settings(state, request, labels, query)
                        .await
                }
                "browserSettings" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_browser_settings(state, request, labels, query)
                        .await
                }
                "networkSettings" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_network_settings(state, request, labels, query)
                        .await
                }
                "networkSettings" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_network_settings(state, request, labels, query)
                        .await
                }
                "userAccessLoggingSettings" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_user_access_logging_settings(
                        state, request, labels, query,
                    )
                    .await
                }
                "userAccessLoggingSettings" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_user_access_logging_settings(
                        state, request, labels, query,
                    )
                    .await
                }
                "userSettings" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_user_settings(state, request, labels, query)
                        .await
                }
                "userSettings" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_user_settings(state, request, labels, query)
                        .await
                }
                "ipAccessSettings" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_ip_access_settings(state, request, labels, query)
                        .await
                }
                "ipAccessSettings" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_ip_access_settings(state, request, labels, query)
                        .await
                }
                "trustStores" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_trust_store(state, request, labels, query)
                        .await
                }
                "trustStores" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_trust_store(state, request, labels, query)
                        .await
                }
                "dataProtectionSettings" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_data_protection_settings(state, request, labels, query)
                        .await
                }
                "dataProtectionSettings" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_data_protection_settings(state, request, labels, query)
                        .await
                }
                "sessionLogger" if method == "PUT" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_associate_session_logger(state, request, labels, query)
                        .await
                }
                "sessionLogger" if method == "DELETE" => {
                    let portal_arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    self.handle_disassociate_session_logger(state, request, labels, query)
                        .await
                }
                _ => {
                    let portal_arn = decode_arn_from_segments(&segments[1..]);
                    let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
                    match method {
                        "GET" => self.handle_get_portal(state, request, labels, query).await,
                        "DELETE" => {
                            self.handle_delete_portal(state, request, labels, query)
                                .await
                        }
                        "PUT" => {
                            self.handle_update_portal(state, request, labels, query)
                                .await
                        }
                        _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                    }
                }
            }
        }
    }

    async fn dispatch_portal_idp(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() > 1 && method == "GET" {
            let portal_arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("portalArn", portal_arn.as_str())];
            self.handle_get_portal_service_provider_metadata(state, request, labels, query)
                .await
        } else {
            rest_json_error(404, "UnknownOperationException", "Not found")
        }
    }

    // ── BrowserSettings routes ──────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_browser_settings(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_browser_settings(
                        state,
                        request,
                        &[],
                        query,
                        account_id,
                        region,
                    )
                    .await
                }
                "GET" => {
                    self.handle_list_browser_settings(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("browserSettingsArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_browser_settings(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_browser_settings(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_browser_settings(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── NetworkSettings routes ──────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_network_settings(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_network_settings(
                        state,
                        request,
                        &[],
                        query,
                        account_id,
                        region,
                    )
                    .await
                }
                "GET" => {
                    self.handle_list_network_settings(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("networkSettingsArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_network_settings(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_network_settings(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_network_settings(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── UserAccessLoggingSettings routes ────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_user_access_logging_settings(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_user_access_logging_settings(
                        state,
                        request,
                        &[],
                        query,
                        account_id,
                        region,
                    )
                    .await
                }
                "GET" => {
                    self.handle_list_user_access_logging_settings(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("userAccessLoggingSettingsArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_user_access_logging_settings(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_user_access_logging_settings(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_user_access_logging_settings(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── UserSettings routes ─────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_user_settings(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_user_settings(state, request, &[], query, account_id, region)
                        .await
                }
                "GET" => {
                    self.handle_list_user_settings(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("userSettingsArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_user_settings(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_user_settings(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_user_settings(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── IdentityProvider routes ─────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_identity_providers(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            if method == "POST" {
                self.handle_create_identity_provider(state, request, &[], query, account_id, region)
                    .await
            } else {
                rest_json_error(404, "UnknownOperationException", "Not found")
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("identityProviderArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_identity_provider(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_identity_provider(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_identity_provider(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── IpAccessSettings routes ─────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_ip_access_settings(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_ip_access_settings(
                        state,
                        request,
                        &[],
                        query,
                        account_id,
                        region,
                    )
                    .await
                }
                "GET" => {
                    self.handle_list_ip_access_settings(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("ipAccessSettingsArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_ip_access_settings(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_ip_access_settings(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_ip_access_settings(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── TrustStore routes ───────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_trust_stores(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_trust_store(state, request, &[], query, account_id, region)
                        .await
                }
                "GET" => {
                    self.handle_list_trust_stores(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let last = *segments.last().unwrap();
            if last == "certificate" {
                let arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                let labels: &[(&str, &str)] = &[("trustStoreArn", arn.as_str())];
                return self
                    .handle_get_trust_store_certificate(state, request, labels, query)
                    .await;
            }
            if last == "certificates" {
                let arn = decode_arn_from_segments(&segments[1..segments.len() - 1]);
                let labels: &[(&str, &str)] = &[("trustStoreArn", arn.as_str())];
                return self
                    .handle_list_trust_store_certificates(state, request, labels, query)
                    .await;
            }
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("trustStoreArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_trust_store(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_trust_store(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_trust_store(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── DataProtectionSettings routes ───────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_data_protection_settings(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_data_protection_settings(
                        state,
                        request,
                        &[],
                        query,
                        account_id,
                        region,
                    )
                    .await
                }
                "GET" => {
                    self.handle_list_data_protection_settings(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("dataProtectionSettingsArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_data_protection_settings(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_data_protection_settings(state, request, labels, query)
                        .await
                }
                "PATCH" => {
                    self.handle_update_data_protection_settings(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    // ── SessionLogger routes ─────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_session_loggers(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        account_id: &str,
        region: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() == 1 {
            match method {
                "POST" => {
                    self.handle_create_session_logger(
                        state,
                        request,
                        &[],
                        query,
                        account_id,
                        region,
                    )
                    .await
                }
                "GET" => {
                    self.handle_list_session_loggers(state, request, &[], query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            let arn = decode_arn_from_segments(&segments[1..]);
            let labels: &[(&str, &str)] = &[("sessionLoggerArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_get_session_logger(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_session_logger(state, request, labels, query)
                        .await
                }
                "POST" => {
                    self.handle_update_session_logger(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }
    }

    async fn dispatch_tags(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() < 2 {
            return rest_json_error(400, "ValidationException", "Missing resource ARN");
        }
        let resource_arn = decode_arn_from_segments(&segments[1..]);
        let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
        match method {
            "GET" => {
                self.handle_list_tags_for_resource(state, request, labels, query)
                    .await
            }
            "POST" => {
                self.handle_tag_resource(state, request, labels, query)
                    .await
            }
            "DELETE" => {
                self.handle_untag_resource(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ── Portal handlers ─────────────────────────────────────────────

    async fn handle_create_portal(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_portal_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let display_name = input.display_name.as_deref();
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));
        let mut state = state.write().await;
        match state.create_portal(display_name, account_id, region, tags) {
            Ok(portal) => {
                let resp = wire::CreatePortalResponse {
                    portal_arn: Some(portal.portal_arn.clone()),
                    portal_endpoint: Some(portal.portal_endpoint.clone()),
                };
                wire::serialize_create_portal_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_portal(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_portal_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_portal(&input.portal_arn) {
            Ok(portal) => {
                let resp = wire::GetPortalResponse {
                    portal: Some(portal_to_wire(portal)),
                };
                wire::serialize_get_portal_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_portal(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_portal_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_portal(&input.portal_arn) {
            Ok(()) => wire::serialize_delete_portal_response(&wire::DeletePortalResponse {}),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_portals(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_portals_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let portals = state.list_portals();
        let entries: Vec<wire::PortalSummary> =
            portals.iter().map(|p| portal_to_summary(p)).collect();
        let resp = wire::ListPortalsResponse {
            next_token: None,
            portals: Some(entries),
        };
        wire::serialize_list_portals_response(&resp)
    }

    async fn handle_update_portal(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_portal_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.update_portal(&input.portal_arn, input.display_name.as_deref()) {
            Ok(portal) => {
                let resp = wire::UpdatePortalResponse {
                    portal: Some(portal_to_wire(portal)),
                };
                wire::serialize_update_portal_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_portal_service_provider_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_portal_service_provider_metadata_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.get_portal(&input.portal_arn) {
            Ok(_) => {
                let saml_metadata = format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?><EntityDescriptor entityID="{0}"/>"#,
                    input.portal_arn
                );
                let resp = wire::GetPortalServiceProviderMetadataResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    service_provider_saml_metadata: Some(saml_metadata),
                };
                wire::serialize_get_portal_service_provider_metadata_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── Disassociate handlers ───────────────────────────────────────

    async fn handle_disassociate_browser_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_browser_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.disassociate_browser_settings(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_browser_settings_response(
                &wire::DisassociateBrowserSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_disassociate_network_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_network_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.disassociate_network_settings(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_network_settings_response(
                &wire::DisassociateNetworkSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_disassociate_user_access_logging_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_user_access_logging_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.disassociate_user_access_logging_settings(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_user_access_logging_settings_response(
                &wire::DisassociateUserAccessLoggingSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_disassociate_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_user_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.disassociate_user_settings(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_user_settings_response(
                &wire::DisassociateUserSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── BrowserSettings handlers ────────────────────────────────────

    async fn handle_create_browser_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_browser_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let browser_policy = input.browser_policy.as_deref();
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));

        let mut state = state.write().await;
        match state.create_browser_settings(browser_policy, account_id, region, tags) {
            Ok(bs) => {
                let resp = wire::CreateBrowserSettingsResponse {
                    browser_settings_arn: Some(bs.browser_settings_arn.clone()),
                };
                wire::serialize_create_browser_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_browser_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_browser_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_browser_settings(&input.browser_settings_arn) {
            Ok(bs) => {
                let resp = wire::GetBrowserSettingsResponse {
                    browser_settings: Some(wire::BrowserSettings {
                        browser_settings_arn: Some(bs.browser_settings_arn.clone()),
                        browser_policy: bs.browser_policy.clone(),
                        associated_portal_arns: Some(bs.associated_portal_arns.clone()),
                        ..Default::default()
                    }),
                };
                wire::serialize_get_browser_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_browser_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_browser_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_browser_settings(&input.browser_settings_arn) {
            Ok(()) => wire::serialize_delete_browser_settings_response(
                &wire::DeleteBrowserSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_browser_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_browser_settings_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let items = state.list_browser_settings();
        let entries: Vec<wire::BrowserSettingsSummary> = items
            .iter()
            .map(|bs| wire::BrowserSettingsSummary {
                browser_settings_arn: Some(bs.browser_settings_arn.clone()),
            })
            .collect();
        let resp = wire::ListBrowserSettingsResponse {
            browser_settings: Some(entries),
            next_token: None,
        };
        wire::serialize_list_browser_settings_response(&resp)
    }

    async fn handle_update_browser_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_browser_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.update_browser_settings(&input.browser_settings_arn, input.browser_policy) {
            Ok(bs) => {
                let resp = wire::UpdateBrowserSettingsResponse {
                    browser_settings: Some(wire::BrowserSettings {
                        browser_settings_arn: Some(bs.browser_settings_arn.clone()),
                        browser_policy: bs.browser_policy.clone(),
                        associated_portal_arns: Some(bs.associated_portal_arns.clone()),
                        ..Default::default()
                    }),
                };
                wire::serialize_update_browser_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── NetworkSettings handlers ────────────────────────────────────

    async fn handle_create_network_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_network_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.vpc_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'vpcId'");
        }
        if input.subnet_ids.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'subnetIds'");
        }
        if input.security_group_ids.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'securityGroupIds'");
        }
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));

        let mut state = state.write().await;
        match state.create_network_settings(
            &input.vpc_id,
            input.subnet_ids,
            input.security_group_ids,
            account_id,
            region,
            tags,
        ) {
            Ok(ns) => {
                let resp = wire::CreateNetworkSettingsResponse {
                    network_settings_arn: Some(ns.network_settings_arn.clone()),
                };
                wire::serialize_create_network_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_network_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_network_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_network_settings(&input.network_settings_arn) {
            Ok(ns) => {
                let resp = wire::GetNetworkSettingsResponse {
                    network_settings: Some(wire::NetworkSettings {
                        network_settings_arn: Some(ns.network_settings_arn.clone()),
                        vpc_id: Some(ns.vpc_id.clone()),
                        subnet_ids: Some(ns.subnet_ids.clone()),
                        security_group_ids: Some(ns.security_group_ids.clone()),
                        associated_portal_arns: Some(ns.associated_portal_arns.clone()),
                    }),
                };
                wire::serialize_get_network_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_network_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_network_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_network_settings(&input.network_settings_arn) {
            Ok(()) => wire::serialize_delete_network_settings_response(
                &wire::DeleteNetworkSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_network_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_network_settings_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let items = state.list_network_settings();
        let entries: Vec<wire::NetworkSettingsSummary> = items
            .iter()
            .map(|ns| wire::NetworkSettingsSummary {
                network_settings_arn: Some(ns.network_settings_arn.clone()),
                vpc_id: Some(ns.vpc_id.clone()),
            })
            .collect();
        let resp = wire::ListNetworkSettingsResponse {
            network_settings: Some(entries),
            next_token: None,
        };
        wire::serialize_list_network_settings_response(&resp)
    }

    async fn handle_update_network_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_network_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.update_network_settings(
            &input.network_settings_arn,
            input.vpc_id,
            input.subnet_ids,
            input.security_group_ids,
        ) {
            Ok(ns) => {
                let resp = wire::UpdateNetworkSettingsResponse {
                    network_settings: Some(wire::NetworkSettings {
                        network_settings_arn: Some(ns.network_settings_arn.clone()),
                        vpc_id: Some(ns.vpc_id.clone()),
                        subnet_ids: Some(ns.subnet_ids.clone()),
                        security_group_ids: Some(ns.security_group_ids.clone()),
                        associated_portal_arns: Some(ns.associated_portal_arns.clone()),
                    }),
                };
                wire::serialize_update_network_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── UserAccessLoggingSettings handlers ──────────────────────────

    async fn handle_create_user_access_logging_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_access_logging_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.kinesis_stream_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'kinesisStreamArn'");
        }
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));

        let mut state = state.write().await;
        match state.create_user_access_logging_settings(
            &input.kinesis_stream_arn,
            account_id,
            region,
            tags,
        ) {
            Ok(ual) => {
                let resp = wire::CreateUserAccessLoggingSettingsResponse {
                    user_access_logging_settings_arn: Some(
                        ual.user_access_logging_settings_arn.clone(),
                    ),
                };
                wire::serialize_create_user_access_logging_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_user_access_logging_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_access_logging_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_user_access_logging_settings(&input.user_access_logging_settings_arn) {
            Ok(ual) => {
                let resp = wire::GetUserAccessLoggingSettingsResponse {
                    user_access_logging_settings: Some(wire::UserAccessLoggingSettings {
                        user_access_logging_settings_arn: Some(
                            ual.user_access_logging_settings_arn.clone(),
                        ),
                        kinesis_stream_arn: Some(ual.kinesis_stream_arn.clone()),
                        associated_portal_arns: Some(ual.associated_portal_arns.clone()),
                    }),
                };
                wire::serialize_get_user_access_logging_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_user_access_logging_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_access_logging_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_user_access_logging_settings(&input.user_access_logging_settings_arn) {
            Ok(()) => wire::serialize_delete_user_access_logging_settings_response(
                &wire::DeleteUserAccessLoggingSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_user_access_logging_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_user_access_logging_settings_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let items = state.list_user_access_logging_settings();
        let entries: Vec<wire::UserAccessLoggingSettingsSummary> = items
            .iter()
            .map(|ual| wire::UserAccessLoggingSettingsSummary {
                user_access_logging_settings_arn: Some(
                    ual.user_access_logging_settings_arn.clone(),
                ),
                kinesis_stream_arn: Some(ual.kinesis_stream_arn.clone()),
            })
            .collect();
        let resp = wire::ListUserAccessLoggingSettingsResponse {
            user_access_logging_settings: Some(entries),
            next_token: None,
        };
        wire::serialize_list_user_access_logging_settings_response(&resp)
    }

    async fn handle_update_user_access_logging_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_access_logging_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.update_user_access_logging_settings(
            &input.user_access_logging_settings_arn,
            input.kinesis_stream_arn,
        ) {
            Ok(ual) => {
                let resp = wire::UpdateUserAccessLoggingSettingsResponse {
                    user_access_logging_settings: Some(wire::UserAccessLoggingSettings {
                        user_access_logging_settings_arn: Some(
                            ual.user_access_logging_settings_arn.clone(),
                        ),
                        kinesis_stream_arn: Some(ual.kinesis_stream_arn.clone()),
                        associated_portal_arns: Some(ual.associated_portal_arns.clone()),
                    }),
                };
                wire::serialize_update_user_access_logging_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── UserSettings handlers ───────────────────────────────────────

    async fn handle_create_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.copy_allowed.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'copyAllowed'");
        }
        if input.paste_allowed.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'pasteAllowed'");
        }
        if input.download_allowed.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'downloadAllowed'");
        }
        if input.upload_allowed.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'uploadAllowed'");
        }
        if input.print_allowed.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'printAllowed'");
        }
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));

        let mut state = state.write().await;
        match state.create_user_settings(
            &input.copy_allowed,
            &input.paste_allowed,
            &input.download_allowed,
            &input.upload_allowed,
            &input.print_allowed,
            input.disconnect_timeout_in_minutes,
            input.idle_disconnect_timeout_in_minutes,
            account_id,
            region,
            tags,
        ) {
            Ok(us) => {
                let resp = wire::CreateUserSettingsResponse {
                    user_settings_arn: Some(us.user_settings_arn.clone()),
                };
                wire::serialize_create_user_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_user_settings(&input.user_settings_arn) {
            Ok(us) => {
                let resp = wire::GetUserSettingsResponse {
                    user_settings: Some(wire::UserSettings {
                        user_settings_arn: Some(us.user_settings_arn.clone()),
                        copy_allowed: Some(us.copy_allowed.clone()),
                        paste_allowed: Some(us.paste_allowed.clone()),
                        download_allowed: Some(us.download_allowed.clone()),
                        upload_allowed: Some(us.upload_allowed.clone()),
                        print_allowed: Some(us.print_allowed.clone()),
                        disconnect_timeout_in_minutes: us.disconnect_timeout_in_minutes,
                        idle_disconnect_timeout_in_minutes: us.idle_disconnect_timeout_in_minutes,
                        associated_portal_arns: Some(us.associated_portal_arns.clone()),
                        ..Default::default()
                    }),
                };
                wire::serialize_get_user_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_user_settings(&input.user_settings_arn) {
            Ok(()) => {
                wire::serialize_delete_user_settings_response(&wire::DeleteUserSettingsResponse {})
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_user_settings_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let items = state.list_user_settings();
        let entries: Vec<wire::UserSettingsSummary> = items
            .iter()
            .map(|us| wire::UserSettingsSummary {
                user_settings_arn: Some(us.user_settings_arn.clone()),
                copy_allowed: Some(us.copy_allowed.clone()),
                paste_allowed: Some(us.paste_allowed.clone()),
                download_allowed: Some(us.download_allowed.clone()),
                upload_allowed: Some(us.upload_allowed.clone()),
                print_allowed: Some(us.print_allowed.clone()),
                disconnect_timeout_in_minutes: us.disconnect_timeout_in_minutes,
                idle_disconnect_timeout_in_minutes: us.idle_disconnect_timeout_in_minutes,
                ..Default::default()
            })
            .collect();
        let resp = wire::ListUserSettingsResponse {
            user_settings: Some(entries),
            next_token: None,
        };
        wire::serialize_list_user_settings_response(&resp)
    }

    async fn handle_update_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.update_user_settings(
            &input.user_settings_arn,
            input.copy_allowed,
            input.paste_allowed,
            input.download_allowed,
            input.upload_allowed,
            input.print_allowed,
            input.disconnect_timeout_in_minutes,
            input.idle_disconnect_timeout_in_minutes,
        ) {
            Ok(us) => {
                let resp = wire::UpdateUserSettingsResponse {
                    user_settings: Some(wire::UserSettings {
                        user_settings_arn: Some(us.user_settings_arn.clone()),
                        copy_allowed: Some(us.copy_allowed.clone()),
                        paste_allowed: Some(us.paste_allowed.clone()),
                        download_allowed: Some(us.download_allowed.clone()),
                        upload_allowed: Some(us.upload_allowed.clone()),
                        print_allowed: Some(us.print_allowed.clone()),
                        disconnect_timeout_in_minutes: us.disconnect_timeout_in_minutes,
                        idle_disconnect_timeout_in_minutes: us.idle_disconnect_timeout_in_minutes,
                        associated_portal_arns: Some(us.associated_portal_arns.clone()),
                        ..Default::default()
                    }),
                };
                wire::serialize_update_user_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── IdentityProvider handlers ───────────────────────────────────

    async fn handle_create_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_identity_provider_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.portal_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'portalArn'");
        }
        if input.identity_provider_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'identityProviderName'");
        }
        if input.identity_provider_type.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'identityProviderType'");
        }
        let mut st = state.write().await;
        match st.create_identity_provider(
            &input.portal_arn,
            &input.identity_provider_name,
            &input.identity_provider_type,
            input.identity_provider_details,
            account_id,
            region,
        ) {
            Ok(ip) => {
                let resp = wire::CreateIdentityProviderResponse {
                    identity_provider_arn: Some(ip.identity_provider_arn.clone()),
                };
                wire::serialize_create_identity_provider_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_identity_provider_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.get_identity_provider(&input.identity_provider_arn) {
            Ok(ip) => {
                let resp = wire::GetIdentityProviderResponse {
                    identity_provider: Some(wire::IdentityProvider {
                        identity_provider_arn: Some(ip.identity_provider_arn.clone()),
                        identity_provider_name: Some(ip.identity_provider_name.clone()),
                        identity_provider_type: Some(ip.identity_provider_type.clone()),
                        identity_provider_details: Some(ip.identity_provider_details.clone()),
                    }),
                };
                wire::serialize_get_identity_provider_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_identity_provider_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_identity_provider(&input.identity_provider_arn) {
            Ok(()) => wire::serialize_delete_identity_provider_response(
                &wire::DeleteIdentityProviderResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_identity_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_identity_providers_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let items = st.list_identity_providers(&input.portal_arn);
        let entries: Vec<wire::IdentityProviderSummary> = items
            .iter()
            .map(|ip| wire::IdentityProviderSummary {
                identity_provider_arn: Some(ip.identity_provider_arn.clone()),
                identity_provider_name: Some(ip.identity_provider_name.clone()),
                identity_provider_type: Some(ip.identity_provider_type.clone()),
            })
            .collect();
        let resp = wire::ListIdentityProvidersResponse {
            identity_providers: Some(entries),
            next_token: None,
        };
        wire::serialize_list_identity_providers_response(&resp)
    }

    async fn handle_update_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_identity_provider_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.update_identity_provider(
            &input.identity_provider_arn,
            input.identity_provider_name.as_deref(),
            input.identity_provider_type.as_deref(),
            input.identity_provider_details,
        ) {
            Ok(ip) => {
                let resp = wire::UpdateIdentityProviderResponse {
                    identity_provider: Some(wire::IdentityProvider {
                        identity_provider_arn: Some(ip.identity_provider_arn.clone()),
                        identity_provider_name: Some(ip.identity_provider_name.clone()),
                        identity_provider_type: Some(ip.identity_provider_type.clone()),
                        identity_provider_details: Some(ip.identity_provider_details.clone()),
                    }),
                };
                wire::serialize_update_identity_provider_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── IpAccessSettings handlers ───────────────────────────────────

    async fn handle_create_ip_access_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_ip_access_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let ip_rules: Vec<types::IpRule> = input
            .ip_rules
            .into_iter()
            .map(|r| types::IpRule {
                ip_range: r.ip_range,
                description: r.description,
            })
            .collect();
        let display_name = input.display_name;
        let description = input.description;
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));
        let mut st = state.write().await;
        match st.create_ip_access_settings(
            ip_rules,
            display_name,
            description,
            account_id,
            region,
            tags,
        ) {
            Ok(ias) => {
                let resp = wire::CreateIpAccessSettingsResponse {
                    ip_access_settings_arn: Some(ias.ip_access_settings_arn.clone()),
                };
                wire::serialize_create_ip_access_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_ip_access_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_ip_access_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.get_ip_access_settings(&input.ip_access_settings_arn) {
            Ok(ias) => {
                let resp = wire::GetIpAccessSettingsResponse {
                    ip_access_settings: Some(ip_access_settings_to_wire(ias)),
                };
                wire::serialize_get_ip_access_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_ip_access_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_ip_access_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.delete_ip_access_settings(&input.ip_access_settings_arn) {
            Ok(()) => wire::serialize_delete_ip_access_settings_response(
                &wire::DeleteIpAccessSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_ip_access_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_ip_access_settings_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let st = state.read().await;
        let items = st.list_ip_access_settings();
        let entries: Vec<wire::IpAccessSettingsSummary> = items
            .iter()
            .map(|ias| wire::IpAccessSettingsSummary {
                ip_access_settings_arn: Some(ias.ip_access_settings_arn.clone()),
                display_name: ias.display_name.clone(),
                description: ias.description.clone(),
                creation_date: Some(ias.creation_date.timestamp() as f64),
            })
            .collect();
        let resp = wire::ListIpAccessSettingsResponse {
            ip_access_settings: Some(entries),
            next_token: None,
        };
        wire::serialize_list_ip_access_settings_response(&resp)
    }

    async fn handle_update_ip_access_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_ip_access_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let ip_rules: Option<Vec<types::IpRule>> = input.ip_rules.map(|rules| {
            rules
                .into_iter()
                .map(|r| types::IpRule {
                    ip_range: r.ip_range,
                    description: r.description,
                })
                .collect()
        });
        let mut st = state.write().await;
        match st.update_ip_access_settings(
            &input.ip_access_settings_arn,
            ip_rules,
            input.display_name,
            input.description,
        ) {
            Ok(ias) => {
                let resp = wire::UpdateIpAccessSettingsResponse {
                    ip_access_settings: Some(ip_access_settings_to_wire(ias)),
                };
                wire::serialize_update_ip_access_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_associate_ip_access_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_ip_access_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.associate_ip_access_settings(&input.portal_arn, &input.ip_access_settings_arn) {
            Ok(()) => {
                let resp = wire::AssociateIpAccessSettingsResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    ip_access_settings_arn: Some(input.ip_access_settings_arn.clone()),
                };
                wire::serialize_associate_ip_access_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_disassociate_ip_access_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_ip_access_settings_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.disassociate_ip_access_settings(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_ip_access_settings_response(
                &wire::DisassociateIpAccessSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── TrustStore handlers ─────────────────────────────────────────

    async fn handle_create_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_trust_store_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));
        let mut st = state.write().await;
        match st.create_trust_store(input.certificate_list, account_id, region, tags) {
            Ok(ts) => {
                let resp = wire::CreateTrustStoreResponse {
                    trust_store_arn: Some(ts.trust_store_arn.clone()),
                };
                wire::serialize_create_trust_store_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_trust_store_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.get_trust_store(&input.trust_store_arn) {
            Ok(ts) => {
                let resp = wire::GetTrustStoreResponse {
                    trust_store: Some(wire::TrustStore {
                        trust_store_arn: Some(ts.trust_store_arn.clone()),
                        associated_portal_arns: Some(ts.associated_portal_arns.clone()),
                    }),
                };
                wire::serialize_get_trust_store_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_trust_store_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_trust_store(&input.trust_store_arn) {
            Ok(()) => {
                wire::serialize_delete_trust_store_response(&wire::DeleteTrustStoreResponse {})
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_trust_stores(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_trust_stores_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let st = state.read().await;
        let items = st.list_trust_stores();
        let entries: Vec<wire::TrustStoreSummary> = items
            .iter()
            .map(|ts| wire::TrustStoreSummary {
                trust_store_arn: Some(ts.trust_store_arn.clone()),
            })
            .collect();
        let resp = wire::ListTrustStoresResponse {
            trust_stores: Some(entries),
            next_token: None,
        };
        wire::serialize_list_trust_stores_response(&resp)
    }

    async fn handle_update_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_trust_store_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let certs_to_add = input.certificates_to_add.unwrap_or_default();
        let thumbprints_to_delete = input.certificates_to_delete.unwrap_or_default();
        let mut st = state.write().await;
        match st.update_trust_store(&input.trust_store_arn, certs_to_add, thumbprints_to_delete) {
            Ok(ts) => {
                let resp = wire::UpdateTrustStoreResponse {
                    trust_store_arn: Some(ts.trust_store_arn.clone()),
                };
                wire::serialize_update_trust_store_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_trust_store_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_trust_store_certificate_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let st = state.read().await;
        match st.get_trust_store(&input.trust_store_arn) {
            Ok(ts) => {
                let cert = ts
                    .certificate_list
                    .iter()
                    .find(|c| c.thumbprint == input.thumbprint);
                let wire_cert = cert.map(|c| wire::Certificate {
                    body: Some(c.body.clone()),
                    issuer: c.issuer.clone(),
                    subject: c.subject.clone(),
                    thumbprint: Some(c.thumbprint.clone()),
                    ..Default::default()
                });
                let resp = wire::GetTrustStoreCertificateResponse {
                    trust_store_arn: Some(input.trust_store_arn.clone()),
                    certificate: wire_cert,
                };
                wire::serialize_get_trust_store_certificate_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_trust_store_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_trust_store_certificates_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let st = state.read().await;
        match st.get_trust_store(&input.trust_store_arn) {
            Ok(ts) => {
                let certs: Vec<wire::CertificateSummary> = ts
                    .certificate_list
                    .iter()
                    .map(|c| wire::CertificateSummary {
                        thumbprint: Some(c.thumbprint.clone()),
                        issuer: c.issuer.clone(),
                        subject: c.subject.clone(),
                        ..Default::default()
                    })
                    .collect();
                let resp = wire::ListTrustStoreCertificatesResponse {
                    trust_store_arn: Some(input.trust_store_arn.clone()),
                    certificate_list: Some(certs),
                    next_token: None,
                };
                wire::serialize_list_trust_store_certificates_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_associate_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_trust_store_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.associate_trust_store(&input.portal_arn, &input.trust_store_arn) {
            Ok(()) => {
                let resp = wire::AssociateTrustStoreResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    trust_store_arn: Some(input.trust_store_arn.clone()),
                };
                wire::serialize_associate_trust_store_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_disassociate_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_trust_store_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.disassociate_trust_store(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_trust_store_response(
                &wire::DisassociateTrustStoreResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── DataProtectionSettings handlers ─────────────────────────────

    async fn handle_create_data_protection_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_data_protection_settings_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let display_name = input.display_name;
        let description = input.description;
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));
        let mut st = state.write().await;
        match st.create_data_protection_settings(
            display_name,
            description,
            account_id,
            region,
            tags,
        ) {
            Ok(dps) => {
                let resp = wire::CreateDataProtectionSettingsResponse {
                    data_protection_settings_arn: Some(dps.data_protection_settings_arn.clone()),
                };
                wire::serialize_create_data_protection_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_data_protection_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_data_protection_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let st = state.read().await;
        match st.get_data_protection_settings(&input.data_protection_settings_arn) {
            Ok(dps) => {
                let resp = wire::GetDataProtectionSettingsResponse {
                    data_protection_settings: Some(wire::DataProtectionSettings {
                        data_protection_settings_arn: Some(
                            dps.data_protection_settings_arn.clone(),
                        ),
                        display_name: dps.display_name.clone(),
                        description: dps.description.clone(),
                        associated_portal_arns: Some(dps.associated_portal_arns.clone()),
                        creation_date: Some(dps.creation_date.timestamp() as f64),
                        ..Default::default()
                    }),
                };
                wire::serialize_get_data_protection_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_data_protection_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_data_protection_settings_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.delete_data_protection_settings(&input.data_protection_settings_arn) {
            Ok(()) => wire::serialize_delete_data_protection_settings_response(
                &wire::DeleteDataProtectionSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_data_protection_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_data_protection_settings_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let st = state.read().await;
        let items = st.list_data_protection_settings();
        let entries: Vec<wire::DataProtectionSettingsSummary> = items
            .iter()
            .map(|dps| wire::DataProtectionSettingsSummary {
                data_protection_settings_arn: Some(dps.data_protection_settings_arn.clone()),
                display_name: dps.display_name.clone(),
                description: dps.description.clone(),
                creation_date: Some(dps.creation_date.timestamp() as f64),
            })
            .collect();
        let resp = wire::ListDataProtectionSettingsResponse {
            data_protection_settings: Some(entries),
            next_token: None,
        };
        wire::serialize_list_data_protection_settings_response(&resp)
    }

    async fn handle_update_data_protection_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_data_protection_settings_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.update_data_protection_settings(
            &input.data_protection_settings_arn,
            input.display_name,
            input.description,
        ) {
            Ok(dps) => {
                let resp = wire::UpdateDataProtectionSettingsResponse {
                    data_protection_settings: Some(wire::DataProtectionSettings {
                        data_protection_settings_arn: Some(
                            dps.data_protection_settings_arn.clone(),
                        ),
                        display_name: dps.display_name.clone(),
                        description: dps.description.clone(),
                        associated_portal_arns: Some(dps.associated_portal_arns.clone()),
                        creation_date: Some(dps.creation_date.timestamp() as f64),
                        ..Default::default()
                    }),
                };
                wire::serialize_update_data_protection_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_associate_data_protection_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_data_protection_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.associate_data_protection_settings(
            &input.portal_arn,
            &input.data_protection_settings_arn,
        ) {
            Ok(()) => {
                let resp = wire::AssociateDataProtectionSettingsResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    data_protection_settings_arn: Some(input.data_protection_settings_arn.clone()),
                };
                wire::serialize_associate_data_protection_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_disassociate_data_protection_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_data_protection_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.disassociate_data_protection_settings(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_data_protection_settings_response(
                &wire::DisassociateDataProtectionSettingsResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── SessionLogger handlers ──────────────────────────────────────

    async fn handle_create_session_logger(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_session_logger_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let display_name = input.display_name;
        let tags = tags_from_input(input.tags.as_deref().unwrap_or(&[]));
        let mut st = state.write().await;
        match st.create_session_logger(display_name, account_id, region, tags) {
            Ok(sl) => {
                let resp = wire::CreateSessionLoggerResponse {
                    session_logger_arn: Some(sl.session_logger_arn.clone()),
                };
                wire::serialize_create_session_logger_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_get_session_logger(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_session_logger_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.get_session_logger(&input.session_logger_arn) {
            Ok(sl) => {
                let resp = wire::GetSessionLoggerResponse {
                    session_logger: Some(wire::SessionLogger {
                        session_logger_arn: Some(sl.session_logger_arn.clone()),
                        display_name: sl.display_name.clone(),
                        associated_portal_arns: Some(sl.associated_portal_arns.clone()),
                        creation_date: Some(sl.creation_date.timestamp() as f64),
                        ..Default::default()
                    }),
                };
                wire::serialize_get_session_logger_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_delete_session_logger(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_session_logger_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_session_logger(&input.session_logger_arn) {
            Ok(()) => wire::serialize_delete_session_logger_response(
                &wire::DeleteSessionLoggerResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_session_loggers(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_session_loggers_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let st = state.read().await;
        let items = st.list_session_loggers();
        let entries: Vec<wire::SessionLoggerSummary> = items
            .iter()
            .map(|sl| wire::SessionLoggerSummary {
                session_logger_arn: Some(sl.session_logger_arn.clone()),
                display_name: sl.display_name.clone(),
                creation_date: Some(sl.creation_date.timestamp() as f64),
                log_configuration: None,
            })
            .collect();
        let resp = wire::ListSessionLoggersResponse {
            session_loggers: Some(entries),
            next_token: None,
        };
        wire::serialize_list_session_loggers_response(&resp)
    }

    async fn handle_update_session_logger(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_session_logger_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.update_session_logger(&input.session_logger_arn, input.display_name) {
            Ok(sl) => {
                let resp = wire::UpdateSessionLoggerResponse {
                    session_logger: Some(wire::SessionLogger {
                        session_logger_arn: Some(sl.session_logger_arn.clone()),
                        display_name: sl.display_name.clone(),
                        associated_portal_arns: Some(sl.associated_portal_arns.clone()),
                        creation_date: Some(sl.creation_date.timestamp() as f64),
                        ..Default::default()
                    }),
                };
                wire::serialize_update_session_logger_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_associate_session_logger(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_session_logger_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.associate_session_logger(&input.portal_arn, &input.session_logger_arn) {
            Ok(()) => {
                let resp = wire::AssociateSessionLoggerResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    session_logger_arn: Some(input.session_logger_arn.clone()),
                };
                wire::serialize_associate_session_logger_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_disassociate_session_logger(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_session_logger_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut st = state.write().await;
        match st.disassociate_session_logger(&input.portal_arn) {
            Ok(()) => wire::serialize_disassociate_session_logger_response(
                &wire::DisassociateSessionLoggerResponse {},
            ),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── Session handlers ─────────────────────────────────────────────

    async fn handle_get_session(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_session_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.get_session(&input.portal_id, &input.session_id) {
            Ok(s) => {
                let resp = wire::GetSessionResponse {
                    session: Some(wire::Session {
                        session_id: Some(s.session_id.clone()),
                        portal_arn: Some(s.portal_id.clone()),
                        username: s.username.clone(),
                        client_ip_addresses: Some(s.client_ip_addresses.clone()),
                        status: Some(s.status.clone()),
                        start_time: Some(s.start_time.timestamp() as f64),
                        end_time: s.end_time.map(|t| t.timestamp() as f64),
                    }),
                };
                wire::serialize_get_session_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_list_sessions(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_sessions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let sessions = st.list_sessions(&input.portal_id);
        let entries: Vec<wire::SessionSummary> = sessions
            .iter()
            .map(|s| wire::SessionSummary {
                session_id: Some(s.session_id.clone()),
                portal_arn: Some(s.portal_id.clone()),
                username: s.username.clone(),
                status: Some(s.status.clone()),
                start_time: Some(s.start_time.timestamp() as f64),
                end_time: s.end_time.map(|t| t.timestamp() as f64),
            })
            .collect();
        let resp = wire::ListSessionsResponse {
            sessions: Some(entries),
            next_token: None,
        };
        wire::serialize_list_sessions_response(&resp)
    }

    async fn handle_expire_session(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_expire_session_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.expire_session(&input.portal_id, &input.session_id) {
            Ok(()) => wire::serialize_expire_session_response(&wire::ExpireSessionResponse {}),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── Association handlers ────────────────────────────────────────

    async fn handle_associate_browser_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_browser_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.associate_browser_settings(&input.portal_arn, &input.browser_settings_arn) {
            Ok(()) => {
                let resp = wire::AssociateBrowserSettingsResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    browser_settings_arn: Some(input.browser_settings_arn.clone()),
                };
                wire::serialize_associate_browser_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_associate_network_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_network_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.associate_network_settings(&input.portal_arn, &input.network_settings_arn) {
            Ok(()) => {
                let resp = wire::AssociateNetworkSettingsResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    network_settings_arn: Some(input.network_settings_arn.clone()),
                };
                wire::serialize_associate_network_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_associate_user_access_logging_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_user_access_logging_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.associate_user_access_logging_settings(
            &input.portal_arn,
            &input.user_access_logging_settings_arn,
        ) {
            Ok(()) => {
                let resp = wire::AssociateUserAccessLoggingSettingsResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    user_access_logging_settings_arn: Some(
                        input.user_access_logging_settings_arn.clone(),
                    ),
                };
                wire::serialize_associate_user_access_logging_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_associate_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_user_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.associate_user_settings(&input.portal_arn, &input.user_settings_arn) {
            Ok(()) => {
                let resp = wire::AssociateUserSettingsResponse {
                    portal_arn: Some(input.portal_arn.clone()),
                    user_settings_arn: Some(input.user_settings_arn.clone()),
                };
                wire::serialize_associate_user_settings_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    // ── Tag handlers ────────────────────────────────────────────────

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();
                let resp = wire::ListTagsForResourceResponse {
                    tags: Some(wire_tags),
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tags = tags_from_input(&input.tags);
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'tags'");
        }
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => workspaces_web_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkspacesWebState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // Note: AWS SDK sends tagKeys as repeated query params (tagKeys=k1&tagKeys=k2).
        // The wire deserialiser splits a single comma-separated value, but our parse
        // routine stores duplicates as suffixed keys (`tagKeys`, `tagKeys_1`, ...).
        // Collect every value whose key starts with "tagKeys" to preserve the existing
        // multi-value behaviour.
        let _ = wire::deserialize_untag_resource_request(request, labels, query);
        let resource_arn = labels
            .iter()
            .find(|(k, _)| *k == "resourceArn")
            .map(|(_, v)| v.to_string())
            .unwrap_or_default();
        let tag_keys: Vec<String> = query
            .iter()
            .filter(|(k, _)| k.starts_with("tagKeys"))
            .map(|(_, v)| v.clone())
            .collect();

        let mut state = state.write().await;
        match state.untag_resource(&resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => workspaces_web_error_response(&e),
        }
    }
}

// ── Conversion helpers ──────────────────────────────────────────────

fn portal_to_wire(portal: &types::Portal) -> wire::Portal {
    wire::Portal {
        portal_arn: Some(portal.portal_arn.clone()),
        portal_endpoint: Some(portal.portal_endpoint.clone()),
        display_name: Some(portal.display_name.clone()),
        portal_status: Some(portal.portal_status.clone()),
        browser_type: Some(portal.browser_type.clone()),
        renderer_type: Some(portal.renderer_type.clone()),
        creation_date: Some(portal.creation_date.timestamp() as f64),
        browser_settings_arn: portal.browser_settings_arn.clone(),
        network_settings_arn: portal.network_settings_arn.clone(),
        user_access_logging_settings_arn: portal.user_access_logging_settings_arn.clone(),
        user_settings_arn: portal.user_settings_arn.clone(),
        trust_store_arn: portal.trust_store_arn.clone(),
        ip_access_settings_arn: portal.ip_access_settings_arn.clone(),
        data_protection_settings_arn: portal.data_protection_settings_arn.clone(),
        session_logger_arn: portal.session_logger_arn.clone(),
        ..Default::default()
    }
}

fn portal_to_summary(portal: &types::Portal) -> wire::PortalSummary {
    wire::PortalSummary {
        portal_arn: Some(portal.portal_arn.clone()),
        portal_endpoint: Some(portal.portal_endpoint.clone()),
        display_name: Some(portal.display_name.clone()),
        portal_status: Some(portal.portal_status.clone()),
        browser_type: Some(portal.browser_type.clone()),
        renderer_type: Some(portal.renderer_type.clone()),
        creation_date: Some(portal.creation_date.timestamp() as f64),
        browser_settings_arn: portal.browser_settings_arn.clone(),
        network_settings_arn: portal.network_settings_arn.clone(),
        user_access_logging_settings_arn: portal.user_access_logging_settings_arn.clone(),
        user_settings_arn: portal.user_settings_arn.clone(),
        trust_store_arn: portal.trust_store_arn.clone(),
        ip_access_settings_arn: portal.ip_access_settings_arn.clone(),
        data_protection_settings_arn: portal.data_protection_settings_arn.clone(),
        session_logger_arn: portal.session_logger_arn.clone(),
        ..Default::default()
    }
}

fn ip_access_settings_to_wire(ias: &types::IpAccessSettings) -> wire::IpAccessSettings {
    wire::IpAccessSettings {
        ip_access_settings_arn: Some(ias.ip_access_settings_arn.clone()),
        display_name: ias.display_name.clone(),
        description: ias.description.clone(),
        ip_rules: Some(
            ias.ip_rules
                .iter()
                .map(|r| wire::IpRule {
                    ip_range: r.ip_range.clone(),
                    description: r.description.clone(),
                })
                .collect(),
        ),
        associated_portal_arns: Some(ias.associated_portal_arns.clone()),
        creation_date: Some(ias.creation_date.timestamp() as f64),
        ..Default::default()
    }
}

// ── URI / query helpers ─────────────────────────────────────────────

fn extract_path_and_query(uri: &str) -> (String, String) {
    let after_host = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };

    if let Some(q) = after_host.find('?') {
        (after_host[..q].to_string(), after_host[q + 1..].to_string())
    } else {
        (after_host.to_string(), String::new())
    }
}

fn parse_query_string(qs: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if qs.is_empty() {
        return map;
    }
    for pair in qs.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let key = percent_decode(k);
            let val = percent_decode(v);
            // For repeated params (like tagKeys), store with index suffix
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(key.clone()) {
                e.insert(val);
            } else {
                let mut i = 1;
                loop {
                    let indexed_key = format!("{key}_{i}");
                    if let std::collections::hash_map::Entry::Vacant(e) = map.entry(indexed_key) {
                        e.insert(val);
                        break;
                    }
                    i += 1;
                }
            }
        }
    }
    map
}

/// Decode a resource ARN from path segments.
fn decode_arn_from_segments(segments: &[&str]) -> String {
    let joined = segments.join("/");
    percent_decode(&joined)
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

fn workspaces_web_error_response(err: &WorkspacesWebError) -> MockResponse {
    let (status, error_type) = match err {
        WorkspacesWebError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
        WorkspacesWebError::SessionNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
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
