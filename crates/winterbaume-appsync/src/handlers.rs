use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{AppSyncError, AppSyncState};
use crate::views::AppsyncStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AppSyncService {
    pub(crate) state: Arc<BackendState<AppSyncState>>,
    pub(crate) notifier: StateChangeNotifier<AppsyncStateView>,
}

impl AppSyncService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppSyncService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppSyncService {
    fn service_name(&self) -> &str {
        "appsync"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://appsync\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AppSyncService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let query_map: HashMap<String, String> =
            winterbaume_core::parse_query_string(&query_string);

        let response = if segments.is_empty() {
            rest_json_error(404, "UnknownOperationException", "Not found")
        } else if segments.len() >= 3 && segments[0] == "v1" && segments[1] == "tags" {
            // ============ v1/tags/{resourceArn+} ============
            let resource_arn = percent_decode(&segments[2..].join("/"));
            let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
            match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                        .await
                }
                "POST" => {
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    self.handle_untag_resource(&state, &request, labels, &query_map, &query_string)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else if segments.len() >= 2 && segments[0] == "v2" && segments[1] == "apis" {
            // ============ v2 routes ============
            self.dispatch_v2(
                method, &segments, &request, &query_map, account_id, &region, &state,
            )
            .await
        } else if segments.len() >= 2 && segments[0] == "v1" && segments[1] == "apis" {
            // ============ v1 routes ============
            self.dispatch_v1(
                method, &segments, &request, &query_map, account_id, &region, &state,
            )
            .await
        } else {
            rest_json_error(404, "UnknownOperationException", "Not found")
        };

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_v2(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
        account_id: &str,
        region: &str,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
    ) -> MockResponse {
        // v2, apis, ...
        match (method, segments.len()) {
            // POST /v2/apis - CreateApi
            ("POST", 2) => {
                self.handle_create_api(state, request, &[], query_map, account_id, region)
                    .await
            }
            // GET /v2/apis - ListApis
            ("GET", 2) => self.handle_list_apis(state, request, &[], query_map).await,
            // GET /v2/apis/{apiId} - GetApi
            ("GET", 3) => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2])];
                self.handle_get_api(state, request, labels, query_map).await
            }
            // DELETE /v2/apis/{apiId} - DeleteApi
            ("DELETE", 3) => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2])];
                self.handle_delete_api(state, request, labels, query_map)
                    .await
            }
            // POST /v2/apis/{apiId}/channelNamespaces - CreateChannelNamespace
            ("POST", 4) if segments[3] == "channelNamespaces" => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2])];
                self.handle_create_channel_namespace(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // GET /v2/apis/{apiId}/channelNamespaces - ListChannelNamespaces
            ("GET", 4) if segments[3] == "channelNamespaces" => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2])];
                self.handle_list_channel_namespaces(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/apis/{apiId}/channelNamespaces/{name} - DeleteChannelNamespace
            ("DELETE", 5) if segments[3] == "channelNamespaces" => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2]), ("name", segments[4])];
                self.handle_delete_channel_namespace(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_v1(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
        account_id: &str,
        region: &str,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
    ) -> MockResponse {
        // segments: ["v1", "apis", ...]
        match (method, segments.len()) {
            // POST /v1/apis - CreateGraphqlApi
            ("POST", 2) => {
                self.handle_create_graphql_api(state, request, &[], query_map, account_id, region)
                    .await
            }
            // GET /v1/apis - ListGraphqlApis
            ("GET", 2) => {
                self.handle_list_graphql_apis(state, request, &[], query_map)
                    .await
            }
            // GET /v1/apis/{apiId} - GetGraphqlApi
            ("GET", 3) => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2])];
                self.handle_get_graphql_api(state, request, labels, query_map)
                    .await
            }
            // POST /v1/apis/{apiId} - UpdateGraphqlApi
            ("POST", 3) => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2])];
                self.handle_update_graphql_api(state, request, labels, query_map)
                    .await
            }
            // DELETE /v1/apis/{apiId} - DeleteGraphqlApi
            ("DELETE", 3) => {
                let labels: &[(&str, &str)] = &[("apiId", segments[2])];
                self.handle_delete_graphql_api(state, request, labels, query_map)
                    .await
            }
            // Routes with 4 segments: /v1/apis/{apiId}/{sub}
            (_, 4) => {
                let api_id = segments[2];
                let sub = segments[3];
                let labels_api: &[(&str, &str)] = &[("apiId", api_id)];
                match (method, sub) {
                    // POST /v1/apis/{apiId}/ApiCaches - CreateApiCache
                    ("POST", "ApiCaches") => {
                        self.handle_create_api_cache(state, request, labels_api, query_map)
                            .await
                    }
                    // GET /v1/apis/{apiId}/ApiCaches - GetApiCache
                    ("GET", "ApiCaches") => {
                        self.handle_get_api_cache(state, request, labels_api, query_map)
                            .await
                    }
                    // DELETE /v1/apis/{apiId}/ApiCaches - DeleteApiCache
                    ("DELETE", "ApiCaches") => {
                        self.handle_delete_api_cache(state, request, labels_api, query_map)
                            .await
                    }
                    // POST /v1/apis/{apiId}/apikeys - CreateApiKey
                    ("POST", "apikeys") => {
                        self.handle_create_api_key(state, request, labels_api, query_map)
                            .await
                    }
                    // GET /v1/apis/{apiId}/apikeys - ListApiKeys
                    ("GET", "apikeys") => {
                        self.handle_list_api_keys(state, request, labels_api, query_map)
                            .await
                    }
                    // DELETE /v1/apis/{apiId}/FlushCache - FlushApiCache
                    ("DELETE", "FlushCache") => {
                        self.handle_flush_api_cache(state, request, labels_api, query_map)
                            .await
                    }
                    // POST /v1/apis/{apiId}/schemacreation - StartSchemaCreation
                    ("POST", "schemacreation") => {
                        self.handle_start_schema_creation(state, request, labels_api, query_map)
                            .await
                    }
                    // GET /v1/apis/{apiId}/schemacreation - GetSchemaCreationStatus
                    ("GET", "schemacreation") => {
                        self.handle_get_schema_creation_status(
                            state, request, labels_api, query_map,
                        )
                        .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            // Routes with 5 segments: /v1/apis/{apiId}/{sub}/{subId}
            (_, 5) => {
                let api_id = segments[2];
                let sub = segments[3];
                let sub_id = segments[4];
                match (method, sub) {
                    // POST /v1/apis/{apiId}/ApiCaches/update - UpdateApiCache
                    ("POST", "ApiCaches") if sub_id == "update" => {
                        let labels: &[(&str, &str)] = &[("apiId", api_id)];
                        self.handle_update_api_cache(state, request, labels, query_map)
                            .await
                    }
                    // DELETE /v1/apis/{apiId}/apikeys/{id} - DeleteApiKey
                    ("DELETE", "apikeys") => {
                        let labels: &[(&str, &str)] = &[("apiId", api_id), ("id", sub_id)];
                        self.handle_delete_api_key(state, request, labels, query_map)
                            .await
                    }
                    // POST /v1/apis/{apiId}/apikeys/{id} - UpdateApiKey
                    ("POST", "apikeys") => {
                        let labels: &[(&str, &str)] = &[("apiId", api_id), ("id", sub_id)];
                        self.handle_update_api_key(state, request, labels, query_map)
                            .await
                    }
                    // GET /v1/apis/{apiId}/types/{typeName} - GetType
                    ("GET", "types") => {
                        let labels: &[(&str, &str)] = &[("apiId", api_id), ("typeName", sub_id)];
                        self.handle_get_type(state, request, labels, query_map)
                            .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ===================== GraphQL API (v1) handlers =====================

    async fn handle_create_graphql_api(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_graphql_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        if input.authentication_type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'authenticationType'");
        }
        let tags = input.tags.unwrap_or_default();

        let mut s = state.write().await;
        match s.create_graphql_api(
            &input.name,
            &input.authentication_type,
            account_id,
            region,
            tags,
        ) {
            Ok(api) => {
                let resp = wire::CreateGraphqlApiResponse {
                    graphql_api: Some(graphql_api_to_wire(api)),
                };
                wire::serialize_create_graphql_api_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_get_graphql_api(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_graphql_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_graphql_api(&input.api_id) {
            Ok(api) => {
                let resp = wire::GetGraphqlApiResponse {
                    graphql_api: Some(graphql_api_to_wire(api)),
                };
                wire::serialize_get_graphql_api_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_delete_graphql_api(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_graphql_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_graphql_api(&input.api_id) {
            Ok(()) => {
                let resp = wire::DeleteGraphqlApiResponse {};
                wire::serialize_delete_graphql_api_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_list_graphql_apis(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_graphql_apis_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let apis = s.list_graphql_apis();
        let entries: Vec<wire::GraphqlApi> =
            apis.iter().map(|api| graphql_api_to_wire(api)).collect();
        let resp = wire::ListGraphqlApisResponse {
            graphql_apis: Some(entries),
            next_token: None,
        };
        wire::serialize_list_graphql_apis_response(&resp)
    }

    async fn handle_update_graphql_api(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_graphql_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            None
        } else {
            Some(input.name.as_str())
        };
        let authentication_type = if input.authentication_type.is_empty() {
            None
        } else {
            Some(input.authentication_type.as_str())
        };

        let mut s = state.write().await;
        match s.update_graphql_api(&input.api_id, name, authentication_type) {
            Ok(api) => {
                let resp = wire::UpdateGraphqlApiResponse {
                    graphql_api: Some(graphql_api_to_wire(api)),
                };
                wire::serialize_update_graphql_api_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    // ===================== Event API (v2) handlers =====================

    async fn handle_create_api(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let owner_contact = input.owner_contact.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut s = state.write().await;
        match s.create_api(&input.name, account_id, region, owner_contact, tags) {
            Ok(api) => {
                let resp = wire::CreateApiResponse {
                    api: Some(api_to_wire(api)),
                };
                wire::serialize_create_api_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_get_api(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_api(&input.api_id) {
            Ok(api) => {
                let resp = wire::GetApiResponse {
                    api: Some(api_to_wire(api)),
                };
                wire::serialize_get_api_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_delete_api(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_api(&input.api_id) {
            Ok(()) => {
                let resp = wire::DeleteApiResponse {};
                wire::serialize_delete_api_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_list_apis(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_apis_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let apis = s.list_apis();
        let entries: Vec<wire::Api> = apis.iter().map(|api| api_to_wire(api)).collect();
        let resp = wire::ListApisResponse {
            apis: Some(entries),
            next_token: None,
        };
        wire::serialize_list_apis_response(&resp)
    }

    // ===================== API Cache handlers =====================

    async fn handle_create_api_cache(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_api_cache_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.api_caching_behavior.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'apiCachingBehavior'");
        }
        if input.r#type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'type'");
        }
        let ttl = if input.ttl == 0 { 3600 } else { input.ttl };
        let at_rest = input.at_rest_encryption_enabled.unwrap_or(false);
        let transit = input.transit_encryption_enabled.unwrap_or(false);
        let health_metrics = input.health_metrics_config.as_deref();

        let mut s = state.write().await;
        match s.create_api_cache(
            &input.api_id,
            &input.api_caching_behavior,
            &input.r#type,
            ttl,
            at_rest,
            transit,
            health_metrics,
        ) {
            Ok(cache) => {
                let resp = wire::CreateApiCacheResponse {
                    api_cache: Some(api_cache_to_wire(cache)),
                };
                wire::serialize_create_api_cache_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_get_api_cache(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_api_cache_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_api_cache(&input.api_id) {
            Ok(cache) => {
                let resp = wire::GetApiCacheResponse {
                    api_cache: Some(api_cache_to_wire(cache)),
                };
                wire::serialize_get_api_cache_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_delete_api_cache(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_api_cache_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_api_cache(&input.api_id) {
            Ok(()) => {
                let resp = wire::DeleteApiCacheResponse {};
                wire::serialize_delete_api_cache_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_update_api_cache(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_api_cache_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.api_caching_behavior.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'apiCachingBehavior'");
        }
        if input.r#type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'type'");
        }
        let ttl = if input.ttl == 0 { 3600 } else { input.ttl };
        let health_metrics = input.health_metrics_config.as_deref();

        let mut s = state.write().await;
        match s.update_api_cache(
            &input.api_id,
            &input.api_caching_behavior,
            &input.r#type,
            ttl,
            health_metrics,
        ) {
            Ok(cache) => {
                let resp = wire::UpdateApiCacheResponse {
                    api_cache: Some(api_cache_to_wire(cache)),
                };
                wire::serialize_update_api_cache_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_flush_api_cache(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_flush_api_cache_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.flush_api_cache(&input.api_id) {
            Ok(()) => {
                let resp = wire::FlushApiCacheResponse {};
                wire::serialize_flush_api_cache_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    // ===================== API Key handlers =====================

    async fn handle_create_api_key(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_api_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();
        let expires = input.expires.unwrap_or_else(|| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            now + 7 * 24 * 60 * 60 // default: 7 days from now
        });

        let mut s = state.write().await;
        match s.create_api_key(&input.api_id, description, expires) {
            Ok(key) => {
                let resp = wire::CreateApiKeyResponse {
                    api_key: Some(api_key_to_wire(key)),
                };
                wire::serialize_create_api_key_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_delete_api_key(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_api_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_api_key(&input.api_id, &input.id) {
            Ok(()) => {
                let resp = wire::DeleteApiKeyResponse {};
                wire::serialize_delete_api_key_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_list_api_keys(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_api_keys_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_api_keys(&input.api_id) {
            Ok(keys) => {
                let entries: Vec<wire::ApiKey> = keys.iter().map(|k| api_key_to_wire(k)).collect();
                let resp = wire::ListApiKeysResponse {
                    api_keys: Some(entries),
                    next_token: None,
                };
                wire::serialize_list_api_keys_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_update_api_key(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_api_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();
        let expires = input.expires;

        let mut s = state.write().await;
        match s.update_api_key(&input.api_id, &input.id, description, expires) {
            Ok(key) => {
                let resp = wire::UpdateApiKeyResponse {
                    api_key: Some(api_key_to_wire(key)),
                };
                wire::serialize_update_api_key_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    // ===================== Channel Namespace handlers (v2) =====================

    async fn handle_create_channel_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_channel_namespace_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let tags = input.tags.unwrap_or_default();

        let mut s = state.write().await;
        match s.create_channel_namespace(&input.api_id, &input.name, account_id, region, tags) {
            Ok(ns) => {
                let resp = wire::CreateChannelNamespaceResponse {
                    channel_namespace: Some(channel_namespace_to_wire(ns)),
                };
                wire::serialize_create_channel_namespace_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_delete_channel_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_channel_namespace_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_channel_namespace(&input.api_id, &input.name) {
            Ok(()) => {
                let resp = wire::DeleteChannelNamespaceResponse {};
                wire::serialize_delete_channel_namespace_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_list_channel_namespaces(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_channel_namespaces_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_channel_namespaces(&input.api_id) {
            Ok(nss) => {
                let entries: Vec<wire::ChannelNamespace> =
                    nss.iter().map(|ns| channel_namespace_to_wire(ns)).collect();
                let resp = wire::ListChannelNamespacesResponse {
                    channel_namespaces: Some(entries),
                    next_token: None,
                };
                wire::serialize_list_channel_namespaces_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    // ===================== Schema handlers =====================

    async fn handle_start_schema_creation(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_schema_creation_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut s = state.write().await;
        match s.start_schema_creation(&input.api_id, input.definition.as_bytes()) {
            Ok(status) => {
                let resp = wire::StartSchemaCreationResponse {
                    status: Some(status.status.clone()),
                };
                wire::serialize_start_schema_creation_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    async fn handle_get_schema_creation_status(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_schema_creation_status_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let s = state.read().await;
        match s.get_schema_creation_status(&input.api_id) {
            Ok(status) => {
                let resp = wire::GetSchemaCreationStatusResponse {
                    status: Some(status.status.clone()),
                    details: status.details.clone(),
                };
                wire::serialize_get_schema_creation_status_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    // ===================== Type handlers =====================

    async fn handle_get_type(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_type_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let format = if input.format.is_empty() {
            "SDL"
        } else {
            input.format.as_str()
        };
        let s = state.read().await;
        match s.get_type(&input.api_id, &input.type_name, format) {
            Ok(t) => {
                let resp = wire::GetTypeResponse {
                    r#type: Some(type_to_wire(t)),
                };
                wire::serialize_get_type_response(&resp)
            }
            Err(e) => appsync_error_response(&e),
        }
    }

    // ===================== Tag handlers =====================

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let tags = s.list_tags_for_resource(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        s.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppSyncState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        raw_query_string: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The AWS SDK encodes tagKeys as repeated query params
        // (`?tagKeys=key1&tagKeys=key2`), but `parse_query_string` only retains the
        // last value, and the wire deserializer comma-splits a single value. Re-extract
        // from the raw query string to preserve multi-key untag semantics.
        let tag_keys = extract_query_list(raw_query_string, "tagKeys");
        let tag_keys = if tag_keys.is_empty() {
            input.tag_keys
        } else {
            tag_keys
        };
        let mut s = state.write().await;
        s.untag_resource(&input.resource_arn, &tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

// ===================== Wire type converters =====================

fn graphql_api_to_wire(api: &crate::types::GraphqlApi) -> wire::GraphqlApi {
    wire::GraphqlApi {
        api_id: Some(api.api_id.clone()),
        name: Some(api.name.clone()),
        authentication_type: Some(api.authentication_type.clone()),
        arn: Some(api.arn.clone()),
        uris: Some(api.uris.clone()),
        tags: if api.tags.is_empty() {
            None
        } else {
            Some(api.tags.clone())
        },
        ..Default::default()
    }
}

fn api_to_wire(api: &crate::types::Api) -> wire::Api {
    wire::Api {
        api_id: Some(api.api_id.clone()),
        name: Some(api.name.clone()),
        api_arn: Some(api.api_arn.clone()),
        created: Some(api.created),
        tags: if api.tags.is_empty() {
            None
        } else {
            Some(api.tags.clone())
        },
        owner_contact: api.owner_contact.clone(),
        ..Default::default()
    }
}

fn api_cache_to_wire(cache: &crate::types::ApiCacheEntry) -> wire::ApiCache {
    wire::ApiCache {
        api_caching_behavior: Some(cache.api_caching_behavior.clone()),
        r#type: Some(cache.r#type.clone()),
        ttl: Some(cache.ttl),
        at_rest_encryption_enabled: Some(cache.at_rest_encryption_enabled),
        transit_encryption_enabled: Some(cache.transit_encryption_enabled),
        status: Some(cache.status.clone()),
        health_metrics_config: cache.health_metrics_config.clone(),
    }
}

fn api_key_to_wire(key: &crate::types::ApiKeyEntry) -> wire::ApiKey {
    wire::ApiKey {
        id: Some(key.id.clone()),
        description: key.description.clone(),
        expires: Some(key.expires),
        deletes: Some(key.deletes),
    }
}

fn channel_namespace_to_wire(ns: &crate::types::ChannelNamespaceEntry) -> wire::ChannelNamespace {
    wire::ChannelNamespace {
        api_id: Some(ns.api_id.clone()),
        name: Some(ns.name.clone()),
        channel_namespace_arn: Some(ns.channel_namespace_arn.clone()),
        created: Some(ns.created),
        last_modified: Some(ns.last_modified),
        tags: if ns.tags.is_empty() {
            None
        } else {
            Some(ns.tags.clone())
        },
        ..Default::default()
    }
}

fn type_to_wire(t: &crate::types::TypeEntry) -> wire::Type {
    wire::Type {
        name: Some(t.name.clone()),
        definition: t.definition.clone(),
        format: Some(t.format.clone()),
        arn: Some(t.arn.clone()),
        ..Default::default()
    }
}

// ===================== Utility functions =====================

fn extract_path_and_query(uri: &str) -> (String, String) {
    let relevant = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };
    if let Some(q) = relevant.find('?') {
        (relevant[..q].to_string(), relevant[q + 1..].to_string())
    } else {
        (relevant.to_string(), String::new())
    }
}

fn extract_query_list(query_string: &str, key: &str) -> Vec<String> {
    query_string
        .split('&')
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=')?;

            if k == key {
                let decoded = v
                    .replace("%20", " ")
                    .replace("%3A", ":")
                    .replace("%2F", "/")
                    .replace('+', " ");
                Some(decoded)
            } else {
                None
            }
        })
        .collect()
}

fn percent_decode(s: &str) -> String {
    s.replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%20", " ")
        .replace('+', " ")
}

fn appsync_error_response(err: &AppSyncError) -> MockResponse {
    let (status, error_type) = match err {
        AppSyncError::GraphqlApiNotFound { .. } => (404u16, "NotFoundException"),
        AppSyncError::ApiNotFound { .. } => (404, "NotFoundException"),
        AppSyncError::ApiCacheNotFound { .. } => (404, "NotFoundException"),
        AppSyncError::ApiCacheAlreadyExists { .. } => (400, "BadRequestException"),
        AppSyncError::ApiKeyNotFound { .. } => (404, "NotFoundException"),
        AppSyncError::ChannelNamespaceNotFound { .. } => (404, "NotFoundException"),
        AppSyncError::SchemaNotFound { .. } => (404, "NotFoundException"),
        AppSyncError::TypeNotFound { .. } => (404, "NotFoundException"),
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
