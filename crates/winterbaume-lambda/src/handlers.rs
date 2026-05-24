use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use base64::Engine as _;
use http::header::HeaderName;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    urldecode,
};

use crate::model;
use crate::state::{LambdaError, LambdaState};
use crate::views::LambdaStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");
const X_AMZ_EXECUTED_VERSION: HeaderName = HeaderName::from_static("x-amz-executed-version");

pub struct LambdaService {
    pub(crate) state: Arc<BackendState<LambdaState>>,
    pub(crate) notifier: StateChangeNotifier<LambdaStateView>,
}

impl LambdaService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }

    /// Resolve an `Invoke` Qualifier against this service's state for the
    /// given scope. Mirrors [`LambdaState::resolve_qualifier`] but exposed
    /// publicly so external crates (e.g. `winterbaume-terraform`) can apply
    /// the same alias/version validation that the Invoke handler now uses.
    pub async fn resolve_qualifier(
        &self,
        account_id: &str,
        region: &str,
        function_name: &str,
        qualifier: Option<&str>,
    ) -> Result<String, LambdaError> {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        guard.resolve_qualifier(function_name, qualifier)
    }
}

impl Default for LambdaService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for LambdaService {
    fn service_name(&self) -> &str {
        "lambda"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://lambda\..*\.amazonaws\.com",
            r"https?://lambda\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl LambdaService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query = extract_query(&request.uri);
        let method = request.method.as_str();

        let raw_segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        let segments: Vec<String> = raw_segments.iter().map(|s| percent_decode(s)).collect();

        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&query);

        // Route based on path prefix and API version
        // 2015-03-31/functions - core function operations
        // 2015-03-31/event-source-mappings - ESM operations
        // 2017-03-31/tags - tag operations
        // 2017-10-31/functions - concurrency operations
        // 2018-10-31/layers - layer operations
        // 2019-09-25/functions - event invoke config operations
        // 2019-09-30/functions - get function concurrency
        // 2020-06-30/functions - code signing config
        // 2021-10-31/functions - function URL config

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let api_version = segments[0].as_str();

        let response = match api_version {
            "2014-11-13" => {
                self.dispatch_2014(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2015-03-31" => {
                self.dispatch_2015(
                    &state,
                    method,
                    &segments[1..],
                    &request,
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            "2016-08-19" => {
                self.dispatch_2016_08_19(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2017-03-31" => {
                self.dispatch_tags(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2017-10-31" => {
                self.dispatch_concurrency(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2018-10-31" => {
                self.dispatch_layers(
                    &state,
                    method,
                    &segments[1..],
                    account_id,
                    &region,
                    &query,
                    &request,
                    &query_map,
                )
                .await
            }
            "2019-09-25" => {
                self.dispatch_event_invoke_config(
                    &state,
                    method,
                    &segments[1..],
                    &request,
                    &query_map,
                )
                .await
            }
            "2019-09-30" => {
                self.dispatch_2019_09_30(
                    &state,
                    method,
                    &segments[1..],
                    &query,
                    &request,
                    &query_map,
                )
                .await
            }
            "2020-04-22" => {
                self.dispatch_code_signing_crud(
                    &state,
                    method,
                    &segments[1..],
                    &request,
                    &query_map,
                )
                .await
            }
            "2020-06-30" => {
                self.dispatch_code_signing(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2021-07-20" => {
                self.dispatch_runtime_management_config(
                    &state,
                    method,
                    &segments[1..],
                    &request,
                    &query_map,
                )
                .await
            }
            "2021-10-31" => {
                self.dispatch_function_url(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2021-11-15" => {
                self.dispatch_2021_11_15(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2022-10-31" => {
                self.dispatch_capacity_providers(
                    &state,
                    method,
                    &segments[1..],
                    &request,
                    &query_map,
                )
                .await
            }
            "2024-08-31" => {
                self.dispatch_recursion_config(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2024-11-01" => {
                self.dispatch_function_configs(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            "2025-12-01" => {
                self.dispatch_2025_12(&state, method, &segments[1..], &request, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ========== 2015-03-31 routes ==========
    #[allow(clippy::too_many_arguments)]
    async fn dispatch_2015(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match segments[0].as_str() {
            "functions" => {
                self.dispatch_functions(
                    state,
                    method,
                    &segments[1..],
                    request,
                    query_map,
                    account_id,
                    region,
                )
                .await
            }
            "event-source-mappings" => {
                self.dispatch_esm(state, method, &segments[1..], request, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== 2016-08-19 routes ==========
    // Handles: GetAccountSettings
    async fn dispatch_2016_08_19(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if method == "GET" && segments.len() == 1 && segments[0] == "account-settings" {
            self.handle_get_account_settings(state, request, &[], query_map)
                .await
        } else {
            rest_json_error(404, "UnknownOperationException", "Not found")
        }
    }

    // ========== 2021-11-15 routes ==========
    // Handles: InvokeWithResponseStream
    async fn dispatch_2021_11_15(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        // POST /functions/{name}/response-streaming-invocations
        if method == "POST"
            && segments.len() == 3
            && segments[0] == "functions"
            && segments[2] == "response-streaming-invocations"
        {
            let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
            self.handle_invoke_with_response_stream(state, request, labels, query_map)
                .await
        } else {
            rest_json_error(404, "UnknownOperationException", "Not found")
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_functions(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            // POST /2015-03-31/functions - CreateFunction
            ("POST", 0) => {
                self.handle_create_function(state, request, &[], query_map, account_id, region)
                    .await
            }
            // GET /2015-03-31/functions - ListFunctions
            ("GET", 0) => self.handle_list_functions(state).await,
            // GET /2015-03-31/functions/{name}
            ("GET", 1) => {
                let labels: &[(&str, &str)] = &[("FunctionName", segments[0].as_str())];
                self.handle_get_function(state, request, labels, query_map)
                    .await
            }
            // DELETE /2015-03-31/functions/{name}
            ("DELETE", 1) => {
                let labels: &[(&str, &str)] = &[("FunctionName", segments[0].as_str())];
                self.handle_delete_function(state, request, labels, query_map)
                    .await
            }
            // Routes with function name + sub-resource
            (_, 2) => {
                let func_name = &segments[0];
                let sub = segments[1].as_str();
                let labels_fn: &[(&str, &str)] = &[("FunctionName", func_name.as_str())];
                match (method, sub) {
                    // POST invocations
                    ("POST", "invocations") => {
                        self.handle_invoke(state, request, labels_fn, query_map)
                            .await
                    }
                    // PUT code
                    ("PUT", "code") => {
                        self.handle_update_function_code(state, request, labels_fn, query_map)
                            .await
                    }
                    // GET configuration - GetFunctionConfiguration
                    ("GET", "configuration") => {
                        self.handle_get_function_configuration(state, request, labels_fn, query_map)
                            .await
                    }
                    // PUT configuration
                    ("PUT", "configuration") => {
                        self.handle_update_function_configuration(
                            state, request, labels_fn, query_map,
                        )
                        .await
                    }
                    // POST /functions/{name}/policy - AddPermission
                    ("POST", "policy") => {
                        self.handle_add_permission(state, request, labels_fn, query_map)
                            .await
                    }
                    // GET /functions/{name}/policy - GetPolicy
                    ("GET", "policy") => {
                        self.handle_get_policy(state, request, labels_fn, query_map)
                            .await
                    }
                    // POST /functions/{name}/aliases - CreateAlias
                    ("POST", "aliases") => {
                        self.handle_create_alias(state, request, labels_fn, query_map)
                            .await
                    }
                    // GET /functions/{name}/aliases - ListAliases
                    ("GET", "aliases") => {
                        self.handle_list_aliases(state, request, labels_fn, query_map)
                            .await
                    }
                    // POST /functions/{name}/versions - PublishVersion
                    ("POST", "versions") => {
                        self.handle_publish_version(state, request, labels_fn, query_map)
                            .await
                    }
                    // GET /functions/{name}/versions - ListVersionsByFunction
                    ("GET", "versions") => {
                        self.handle_list_versions_by_function(state, request, labels_fn, query_map)
                            .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            // Routes with function name + sub-resource + id
            (_, 3) => {
                let func_name = &segments[0];
                let sub = segments[1].as_str();
                let id = &segments[2];
                match (method, sub) {
                    // GET /functions/{name}/aliases/{alias} - GetAlias
                    ("GET", "aliases") => {
                        let labels: &[(&str, &str)] =
                            &[("FunctionName", func_name.as_str()), ("Name", id.as_str())];
                        self.handle_get_alias(state, request, labels, query_map)
                            .await
                    }
                    // PUT /functions/{name}/aliases/{alias} - UpdateAlias
                    ("PUT", "aliases") => {
                        let labels: &[(&str, &str)] =
                            &[("FunctionName", func_name.as_str()), ("Name", id.as_str())];
                        self.handle_update_alias(state, request, labels, query_map)
                            .await
                    }
                    // DELETE /functions/{name}/aliases/{alias} - DeleteAlias
                    ("DELETE", "aliases") => {
                        let labels: &[(&str, &str)] =
                            &[("FunctionName", func_name.as_str()), ("Name", id.as_str())];
                        self.handle_delete_alias(state, request, labels, query_map)
                            .await
                    }
                    // DELETE /functions/{name}/policy/{sid} - RemovePermission
                    ("DELETE", "policy") => {
                        let labels: &[(&str, &str)] = &[
                            ("FunctionName", func_name.as_str()),
                            ("StatementId", id.as_str()),
                        ];
                        self.handle_remove_permission(state, request, labels, query_map)
                            .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Event Source Mapping routes ==========
    async fn dispatch_esm(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        match (method, segments.len()) {
            // POST /event-source-mappings - CreateEventSourceMapping
            ("POST", 0) => {
                self.handle_create_event_source_mapping(state, request, &[], query_map)
                    .await
            }
            // GET /event-source-mappings - ListEventSourceMappings
            ("GET", 0) => {
                self.handle_list_event_source_mappings(state, request, &[], query_map)
                    .await
            }
            // GET /event-source-mappings/{uuid}
            ("GET", 1) => {
                let labels: &[(&str, &str)] = &[("UUID", segments[0].as_str())];
                self.handle_get_event_source_mapping(state, request, labels, query_map)
                    .await
            }
            // PUT /event-source-mappings/{uuid}
            ("PUT", 1) => {
                let labels: &[(&str, &str)] = &[("UUID", segments[0].as_str())];
                self.handle_update_event_source_mapping(state, request, labels, query_map)
                    .await
            }
            // DELETE /event-source-mappings/{uuid}
            ("DELETE", 1) => {
                let labels: &[(&str, &str)] = &[("UUID", segments[0].as_str())];
                self.handle_delete_event_source_mapping(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Tag routes (2017-03-31) ==========
    async fn dispatch_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        // tags/{Resource} - Resource is a full ARN which may contain colons
        // The ARN is URL-encoded in the path, so we need to reconstruct it
        if segments.is_empty() || segments[0] != "tags" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        if segments.len() < 2 {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        // Reconstruct the ARN from remaining segments (colons become path separators after URL parsing, but actually ARNs use colons not slashes)
        // The resource ARN is everything after "tags/"
        let arn = urldecode(&segments[1..].join("/"));
        let labels: &[(&str, &str)] = &[("Resource", arn.as_str())];

        match method {
            "GET" => {
                self.handle_list_tags(state, request, labels, query_map)
                    .await
            }
            "POST" => {
                self.handle_tag_resource(state, request, labels, query_map)
                    .await
            }
            "DELETE" => {
                self.handle_untag_resource(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Concurrency routes (2017-10-31) ==========
    async fn dispatch_concurrency(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        // functions/{name}/concurrency
        if segments.len() != 3 || segments[0] != "functions" || segments[2] != "concurrency" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
        match method {
            "PUT" => {
                self.handle_put_function_concurrency(state, request, labels, query_map)
                    .await
            }
            "DELETE" => {
                self.handle_delete_function_concurrency(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== 2019-09-30 routes ==========
    // Handles: GetFunctionConcurrency, ListProvisionedConcurrencyConfigs,
    //          GetProvisionedConcurrencyConfig, PutProvisionedConcurrencyConfig,
    //          DeleteProvisionedConcurrencyConfig
    async fn dispatch_2019_09_30(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        query: &str,
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() < 3 || segments[0] != "functions" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        let func_name = &segments[1];
        let labels: &[(&str, &str)] = &[("FunctionName", func_name.as_str())];
        match segments[2].as_str() {
            "concurrency" if segments.len() == 3 && method == "GET" => {
                self.handle_get_function_concurrency(state, request, labels, query_map)
                    .await
            }
            "provisioned-concurrency" if segments.len() == 3 => {
                // The Smithy wire fns for Get/Put/Delete ProvisionedConcurrencyConfig
                // bind the required `Qualifier` from the query string. PUT/DELETE without
                // an explicit qualifier historically default to "$LATEST"; we synthesize
                // that into the query map before delegating.
                let qualifier = parse_query_param(query, "Qualifier");
                match (method, qualifier) {
                    ("GET", None) => {
                        self.handle_list_provisioned_concurrency_configs(
                            state, request, labels, query_map,
                        )
                        .await
                    }
                    ("GET", Some(_)) => {
                        self.handle_get_provisioned_concurrency_config(
                            state, request, labels, query_map,
                        )
                        .await
                    }
                    ("PUT", Some(_)) => {
                        self.handle_put_provisioned_concurrency_config(
                            state, request, labels, query_map,
                        )
                        .await
                    }
                    ("PUT", None) => {
                        let mut qm = query_map.clone();
                        qm.insert("Qualifier".to_string(), "$LATEST".to_string());
                        self.handle_put_provisioned_concurrency_config(state, request, labels, &qm)
                            .await
                    }
                    ("DELETE", Some(_)) => {
                        self.handle_delete_provisioned_concurrency_config(
                            state, request, labels, query_map,
                        )
                        .await
                    }
                    ("DELETE", None) => {
                        let mut qm = query_map.clone();
                        qm.insert("Qualifier".to_string(), "$LATEST".to_string());
                        self.handle_delete_provisioned_concurrency_config(
                            state, request, labels, &qm,
                        )
                        .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Layer routes (2018-10-31) ==========
    #[allow(clippy::too_many_arguments)]
    async fn dispatch_layers(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        account_id: &str,
        region: &str,
        query: &str,
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        if segments[0] != "layers" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        // GET /2018-10-31/layers?find=LayerVersionByArn&Arn={arn} - GetLayerVersionByArn
        if method == "GET" && segments.len() == 1 && parse_query_param(query, "Arn").is_some() {
            return self
                .handle_get_layer_version_by_arn(state, request, &[], query_map)
                .await;
        }
        match (method, segments.len()) {
            // GET /layers - ListLayers
            ("GET", 1) => {
                self.handle_list_layers(state, request, &[], query_map)
                    .await
            }
            // POST /layers/{name}/versions - PublishLayerVersion
            ("POST", 3) if segments[2] == "versions" => {
                let labels: &[(&str, &str)] = &[("LayerName", segments[1].as_str())];
                self.handle_publish_layer_version(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // GET /layers/{name}/versions - ListLayerVersions
            ("GET", 3) if segments[2] == "versions" => {
                let labels: &[(&str, &str)] = &[("LayerName", segments[1].as_str())];
                self.handle_list_layer_versions(state, request, labels, query_map)
                    .await
            }
            // GET /layers/{name}/versions/{version} - GetLayerVersion
            ("GET", 4) if segments[2] == "versions" => {
                let labels: &[(&str, &str)] = &[
                    ("LayerName", segments[1].as_str()),
                    ("VersionNumber", segments[3].as_str()),
                ];
                self.handle_get_layer_version(state, request, labels, query_map)
                    .await
            }
            // DELETE /layers/{name}/versions/{version} - DeleteLayerVersion
            ("DELETE", 4) if segments[2] == "versions" => {
                let labels: &[(&str, &str)] = &[
                    ("LayerName", segments[1].as_str()),
                    ("VersionNumber", segments[3].as_str()),
                ];
                self.handle_delete_layer_version(state, request, labels, query_map)
                    .await
            }
            // POST /layers/{name}/versions/{version}/policy - AddLayerVersionPermission
            ("POST", 5) if segments[2] == "versions" && segments[4] == "policy" => {
                let labels: &[(&str, &str)] = &[
                    ("LayerName", segments[1].as_str()),
                    ("VersionNumber", segments[3].as_str()),
                ];
                self.handle_add_layer_version_permission(state, request, labels, query_map)
                    .await
            }
            // GET /layers/{name}/versions/{version}/policy - GetLayerVersionPolicy
            ("GET", 5) if segments[2] == "versions" && segments[4] == "policy" => {
                let labels: &[(&str, &str)] = &[
                    ("LayerName", segments[1].as_str()),
                    ("VersionNumber", segments[3].as_str()),
                ];
                self.handle_get_layer_version_policy(state, request, labels, query_map)
                    .await
            }
            // DELETE /layers/{name}/versions/{version}/policy/{sid} - RemoveLayerVersionPermission
            ("DELETE", 6) if segments[2] == "versions" && segments[4] == "policy" => {
                let labels: &[(&str, &str)] = &[
                    ("LayerName", segments[1].as_str()),
                    ("VersionNumber", segments[3].as_str()),
                    ("StatementId", segments[5].as_str()),
                ];
                self.handle_remove_layer_version_permission(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Event Invoke Config routes (2019-09-25) ==========
    async fn dispatch_event_invoke_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        // functions/{name}/event-invoke-config[/list]
        if segments.len() < 3 || segments[0] != "functions" || segments[2] != "event-invoke-config"
        {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];

        if segments.len() == 4 && segments[3] == "list" && method == "GET" {
            // GET /functions/{name}/event-invoke-config/list - ListFunctionEventInvokeConfigs
            return self
                .handle_list_function_event_invoke_configs(state, request, labels, query_map)
                .await;
        }

        if segments.len() != 3 {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match method {
            "PUT" => {
                self.handle_put_function_event_invoke_config(state, request, labels, query_map)
                    .await
            }
            "GET" => {
                self.handle_get_function_event_invoke_config(state, request, labels, query_map)
                    .await
            }
            "POST" => {
                self.handle_update_function_event_invoke_config(state, request, labels, query_map)
                    .await
            }
            "DELETE" => {
                self.handle_delete_function_event_invoke_config(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Code Signing Config routes (2020-06-30) ==========
    // Handles: GetFunctionCodeSigningConfig, PutFunctionCodeSigningConfig, DeleteFunctionCodeSigningConfig
    // Also: ListFunctionsByCodeSigningConfig (via code-signing-configs/{id}/functions)
    async fn dispatch_code_signing(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        match segments[0].as_str() {
            "functions" => {
                // functions/{name}/code-signing-config
                if segments.len() == 3 && segments[2] == "code-signing-config" {
                    let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
                    match method {
                        "GET" => {
                            self.handle_get_function_code_signing_config(
                                state, request, labels, query_map,
                            )
                            .await
                        }
                        "PUT" => {
                            self.handle_put_function_code_signing_config(
                                state, request, labels, query_map,
                            )
                            .await
                        }
                        "DELETE" => {
                            self.handle_delete_function_code_signing_config(
                                state, request, labels, query_map,
                            )
                            .await
                        }
                        _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                    }
                } else {
                    rest_json_error(404, "UnknownOperationException", "Not found")
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Code Signing Config CRUD (2020-04-22) ==========
    // Handles: CreateCodeSigningConfig, GetCodeSigningConfig, UpdateCodeSigningConfig,
    //          DeleteCodeSigningConfig, ListCodeSigningConfigs
    async fn dispatch_code_signing_crud(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.is_empty() || segments[0] != "code-signing-configs" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        match (method, segments.len()) {
            ("POST", 1) => {
                self.handle_create_code_signing_config(state, request, &[], query_map)
                    .await
            }
            ("GET", 1) => {
                self.handle_list_code_signing_configs(state, request, &[], query_map)
                    .await
            }
            ("GET", 2) => {
                let labels: &[(&str, &str)] = &[("CodeSigningConfigArn", segments[1].as_str())];
                self.handle_get_code_signing_config(state, request, labels, query_map)
                    .await
            }
            ("PUT", 2) => {
                let labels: &[(&str, &str)] = &[("CodeSigningConfigArn", segments[1].as_str())];
                self.handle_update_code_signing_config(state, request, labels, query_map)
                    .await
            }
            ("DELETE", 2) => {
                let labels: &[(&str, &str)] = &[("CodeSigningConfigArn", segments[1].as_str())];
                self.handle_delete_code_signing_config(state, request, labels, query_map)
                    .await
            }
            // GET /code-signing-configs/{id}/functions - ListFunctionsByCodeSigningConfig
            ("GET", 3) if segments[2] == "functions" => {
                let labels: &[(&str, &str)] = &[("CodeSigningConfigArn", segments[1].as_str())];
                self.handle_list_functions_by_code_signing_config(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Function URL Config routes (2021-10-31) ==========
    async fn dispatch_function_url(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() < 3 || segments[0] != "functions" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
        match (method, segments[2].as_str()) {
            // GET /functions/{name}/urls - ListFunctionUrlConfigs
            ("GET", "urls") if segments.len() == 3 => {
                self.handle_list_function_url_configs(state, request, labels, query_map)
                    .await
            }
            // POST /functions/{name}/url - CreateFunctionUrlConfig
            ("POST", "url") if segments.len() == 3 => {
                self.handle_create_function_url_config(state, request, labels, query_map)
                    .await
            }
            // GET /functions/{name}/url - GetFunctionUrlConfig
            ("GET", "url") if segments.len() == 3 => {
                self.handle_get_function_url_config(state, request, labels, query_map)
                    .await
            }
            // PUT /functions/{name}/url - UpdateFunctionUrlConfig
            ("PUT", "url") if segments.len() == 3 => {
                self.handle_update_function_url_config(state, request, labels, query_map)
                    .await
            }
            // DELETE /functions/{name}/url - DeleteFunctionUrlConfig
            ("DELETE", "url") if segments.len() == 3 => {
                self.handle_delete_function_url_config(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== InvokeAsync routes (2014-11-13) ==========
    async fn dispatch_2014(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        // POST /functions/{name}/invoke-async
        if segments.len() == 3
            && method == "POST"
            && segments[0] == "functions"
            && segments[2] == "invoke-async"
        {
            let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
            self.handle_invoke_async(state, request, labels, query_map)
                .await
        } else {
            rest_json_error(404, "UnknownOperationException", "Not found")
        }
    }

    // ========== Capacity Provider routes (2022-10-31) ==========
    async fn dispatch_capacity_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.is_empty() || segments[0] != "capacity-providers" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        match (method, segments.len()) {
            ("POST", 1) => {
                self.handle_create_capacity_provider(state, request, &[], query_map)
                    .await
            }
            ("GET", 1) => {
                self.handle_list_capacity_providers(state, request, &[], query_map)
                    .await
            }
            ("GET", 2) => {
                let labels: &[(&str, &str)] = &[("CapacityProviderName", segments[1].as_str())];
                self.handle_get_capacity_provider(state, request, labels, query_map)
                    .await
            }
            ("PUT", 2) => {
                let labels: &[(&str, &str)] = &[("CapacityProviderName", segments[1].as_str())];
                self.handle_update_capacity_provider(state, request, labels, query_map)
                    .await
            }
            ("DELETE", 2) => {
                let labels: &[(&str, &str)] = &[("CapacityProviderName", segments[1].as_str())];
                self.handle_delete_capacity_provider(state, request, labels, query_map)
                    .await
            }
            // GET /capacity-providers/{name}/functions - ListFunctionVersionsByCapacityProvider
            ("GET", 3) if segments[2] == "functions" => {
                let labels: &[(&str, &str)] = &[("CapacityProviderName", segments[1].as_str())];
                self.handle_list_function_versions_by_capacity_provider(
                    state, request, labels, query_map,
                )
                .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== 2024-08-31 routes ==========
    // Handles: GetFunctionRecursionConfig, PutFunctionRecursionConfig
    async fn dispatch_recursion_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() != 3 || segments[0] != "functions" || segments[2] != "recursion-config" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
        match method {
            "GET" => {
                self.handle_get_function_recursion_config(state, request, labels, query_map)
                    .await
            }
            "PUT" => {
                self.handle_put_function_recursion_config(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== 2021-07-20 routes ==========
    // Handles: GetRuntimeManagementConfig, PutRuntimeManagementConfig
    async fn dispatch_runtime_management_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.len() != 3
            || segments[0] != "functions"
            || segments[2] != "runtime-management-config"
        {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
        match method {
            "GET" => {
                self.handle_get_runtime_management_config(state, request, labels, query_map)
                    .await
            }
            "PUT" => {
                self.handle_put_runtime_management_config(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Function config routes (2024-11-01) ==========
    // Handles: GetFunctionScalingConfig, PutFunctionScalingConfig,
    //          GetLayerVersionByArn, ListFunctionUrlConfigs,
    //          ListDurableExecutionsByFunction + durable execution ops
    async fn dispatch_function_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        match segments[0].as_str() {
            "functions" if segments.len() >= 3 => {
                let func_name = &segments[1];
                let sub = segments[2].as_str();
                let labels_fn: &[(&str, &str)] = &[("FunctionName", func_name.as_str())];
                match (method, sub) {
                    ("GET", "scaling-config") => {
                        self.handle_get_function_scaling_config(
                            state, request, labels_fn, query_map,
                        )
                        .await
                    }
                    ("PUT", "scaling-config") => {
                        self.handle_put_function_scaling_config(
                            state, request, labels_fn, query_map,
                        )
                        .await
                    }
                    ("GET", "durable-executions") => {
                        self.handle_list_durable_executions_by_function(
                            state, request, labels_fn, query_map,
                        )
                        .await
                    }
                    ("POST", "durable-executions") if segments.len() >= 5 => {
                        let exec_id = &segments[3];
                        let action = segments[4].as_str();
                        match action {
                            "checkpoint" => {
                                let labels: &[(&str, &str)] =
                                    &[("DurableExecutionArn", exec_id.as_str())];
                                self.handle_checkpoint_durable_execution(
                                    state, request, labels, query_map,
                                )
                                .await
                            }
                            "callback-failure" => {
                                let labels: &[(&str, &str)] = &[("CallbackId", exec_id.as_str())];
                                self.handle_send_durable_execution_callback_failure(
                                    state, request, labels, query_map,
                                )
                                .await
                            }
                            "callback-heartbeat" => {
                                let labels: &[(&str, &str)] = &[("CallbackId", exec_id.as_str())];
                                self.handle_send_durable_execution_callback_heartbeat(
                                    state, request, labels, query_map,
                                )
                                .await
                            }
                            "callback-success" => {
                                let labels: &[(&str, &str)] = &[("CallbackId", exec_id.as_str())];
                                self.handle_send_durable_execution_callback_success(
                                    state, request, labels, query_map,
                                )
                                .await
                            }
                            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                        }
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            "functions" if segments.len() == 2 => {
                let labels: &[(&str, &str)] = &[("FunctionName", segments[1].as_str())];
                match method {
                    "GET" => {
                        self.handle_list_function_url_configs(state, request, labels, query_map)
                            .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            "durable-executions" if segments.len() >= 2 => {
                let exec_id = &segments[1];
                let labels: &[(&str, &str)] = &[("DurableExecutionArn", exec_id.as_str())];
                match (method, segments.len()) {
                    ("GET", 2) => {
                        self.handle_get_durable_execution(state, request, labels, query_map)
                            .await
                    }
                    ("GET", 3) if segments[2] == "history" => {
                        self.handle_get_durable_execution_history(state, request, labels, query_map)
                            .await
                    }
                    ("GET", 3) if segments[2] == "state" => {
                        self.handle_get_durable_execution_state(state, request, labels, query_map)
                            .await
                    }
                    ("DELETE", 2) => {
                        self.handle_stop_durable_execution(state, request, labels, query_map)
                            .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            "layers" if segments.len() == 1 => {
                // GET /layers?find=LayerVersionByArn&Arn=...
                // We handle this in dispatch_layers, but also expose here
                rest_json_error(404, "UnknownOperationException", "Not found")
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== 2025-12-01 routes ==========
    // Handles: SendDurableExecutionCallbackFailure, SendDurableExecutionCallbackHeartbeat,
    //          SendDurableExecutionCallbackSuccess
    async fn dispatch_2025_12(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        method: &str,
        segments: &[String],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
    ) -> MockResponse {
        // POST /2025-12-01/durable-execution-callbacks/{CallbackId}/fail
        // POST /2025-12-01/durable-execution-callbacks/{CallbackId}/heartbeat
        // POST /2025-12-01/durable-execution-callbacks/{CallbackId}/succeed
        match segments.first().map(|s| s.as_str()) {
            Some("durable-execution-callbacks") if segments.len() == 3 && method == "POST" => {
                let labels: &[(&str, &str)] = &[("CallbackId", segments[1].as_str())];
                match segments[2].as_str() {
                    "fail" => {
                        self.handle_send_durable_execution_callback_failure(
                            state, request, labels, query_map,
                        )
                        .await
                    }
                    "heartbeat" => {
                        self.handle_send_durable_execution_callback_heartbeat(
                            state, request, labels, query_map,
                        )
                        .await
                    }
                    "succeed" => {
                        self.handle_send_durable_execution_callback_success(
                            state, request, labels, query_map,
                        )
                        .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ========== Handler implementations ==========

    async fn handle_create_function(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_function_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.function_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'FunctionName'");
        }
        if input.role.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Role'");
        }
        let name = input.function_name.as_str();
        let runtime = input.runtime.as_deref().unwrap_or("python3.12");
        let handler = input.handler.as_deref().unwrap_or("index.handler");
        let role = input.role.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let memory_size = input.memory_size;
        let timeout = input.timeout;

        if let Some(ms) = memory_size
            && (!(128..=10240).contains(&ms))
        {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                &format!(
                    "Value {ms} at 'memorySize' failed to satisfy constraint: \
                         Member must have value between 128 and 10240"
                ),
            );
        }
        if let Some(t) = timeout
            && (!(1..=900).contains(&t))
        {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                &format!(
                    "Value {t} at 'timeout' failed to satisfy constraint: \
                         Member must have value between 1 and 900"
                ),
            );
        }

        // Extract zip content for CodeSha256 and CodeSize computation
        let zip_content = input
            .code
            .zip_file
            .as_ref()
            .and_then(|s| base64::engine::general_purpose::STANDARD.decode(s).ok());

        let (code_sha256, code_size) = if let Some(ref content) = zip_content {
            let mut hasher = Sha256::new();
            hasher.update(content);
            let hash = hasher.finalize();
            let sha256 = base64::engine::general_purpose::STANDARD.encode(hash);
            (sha256, content.len() as i64)
        } else {
            (crate::state::mock_code_sha256(name), 262144)
        };

        let environment = input
            .environment
            .and_then(|e| e.variables)
            .unwrap_or_default();

        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_function(
            name,
            runtime,
            handler,
            role,
            description,
            memory_size,
            timeout,
            environment,
            account_id,
            region,
            tags,
            &code_sha256,
            code_size,
        ) {
            Ok(func) => {
                let resp = function_configuration_from(func);
                wire::serialize_create_function_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_function(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_function_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = &input.function_name;
        let state = state.read().await;
        match state.get_function(name) {
            Ok(func) => {
                let config = function_configuration_from(func);
                let resp = model::GetFunctionResponse {
                    configuration: Some(config),
                    code: Some(model::FunctionCodeLocation {
                        repository_type: Some("S3".to_string()),
                        location: Some(format!(
                            "https://awslambda-us-east-1-tasks.s3.amazonaws.com/snapshots/123456789012/{}",
                            func.function_name
                        )),
                        ..Default::default()
                    }),
                    tags: Some(func.tags.clone()),
                    ..Default::default()
                };
                wire::serialize_get_function_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_function(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_function_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_function(&input.function_name) {
            Ok(()) => MockResponse::rest_json(204, "".to_string()),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_functions(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let funcs = state.list_functions();
        let entries: Vec<model::FunctionConfiguration> = funcs
            .iter()
            .map(|f| function_configuration_from(f))
            .collect();

        let resp = model::ListFunctionsResponse {
            functions: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_functions_response(&resp)
    }

    async fn handle_invoke(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_invoke_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.resolve_qualifier(&input.function_name, input.qualifier.as_deref()) {
            Ok(resolved_version) => {
                let mut response = MockResponse::rest_json(200, "{}".to_string());
                response
                    .headers
                    .insert(X_AMZ_EXECUTED_VERSION, resolved_version.parse().unwrap());
                response
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_update_function_code(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_function_code_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_function_code(&input.function_name, input.revision_id.as_deref()) {
            Ok(func) => {
                let resp = function_configuration_from(func);
                wire::serialize_update_function_code_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_update_function_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_function_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let memory_size = input.memory_size;
        let timeout = input.timeout;

        if let Some(ms) = memory_size
            && (!(128..=10240).contains(&ms))
        {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                &format!(
                    "Value {ms} at 'memorySize' failed to satisfy constraint: \
                         Member must have value between 128 and 10240"
                ),
            );
        }
        if let Some(t) = timeout
            && (!(1..=900).contains(&t))
        {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                &format!(
                    "Value {t} at 'timeout' failed to satisfy constraint: \
                         Member must have value between 1 and 900"
                ),
            );
        }

        let environment = input.environment.and_then(|e| e.variables);

        let mut state = state.write().await;
        match state.update_function_configuration(
            &input.function_name,
            input.description.as_deref(),
            input.handler.as_deref(),
            memory_size,
            timeout,
            input.runtime.as_deref(),
            environment,
            input.revision_id.as_deref(),
        ) {
            Ok(func) => {
                let resp = function_configuration_from(func);
                wire::serialize_update_function_configuration_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Alias handlers ==========

    async fn handle_create_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        let function_version = if input.function_version.is_empty() {
            "$LATEST"
        } else {
            input.function_version.as_str()
        };
        let description = input.description.as_deref().unwrap_or("");
        let routing_config = input
            .routing_config
            .and_then(|rc| rc.additional_version_weights);

        let mut state = state.write().await;
        match state.create_alias(
            &input.function_name,
            &input.name,
            function_version,
            description,
            routing_config,
        ) {
            Ok(alias) => {
                let resp = alias_configuration_from(alias);
                wire::serialize_create_alias_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_alias(&input.function_name, &input.name) {
            Ok(alias) => {
                let resp = alias_configuration_from(alias);
                wire::serialize_get_alias_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_update_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Preserve the body-presence semantics of RoutingConfig: Option<Option<...>>.
        // Outer None means "do not touch", outer Some means "set to inner value".
        let routing_config = input.routing_config.map(|rc| rc.additional_version_weights);

        let mut state = state.write().await;
        match state.update_alias(
            &input.function_name,
            &input.name,
            input.function_version.as_deref(),
            input.description.as_deref(),
            routing_config,
            input.revision_id.as_deref(),
        ) {
            Ok(alias) => {
                let resp = alias_configuration_from(alias);
                wire::serialize_update_alias_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_alias(&input.function_name, &input.name) {
            Ok(()) => wire::serialize_delete_alias_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_aliases(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_aliases_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_aliases(&input.function_name) {
            Ok(aliases) => {
                let entries: Vec<model::AliasConfiguration> = aliases
                    .iter()
                    .map(|a| alias_configuration_from(a))
                    .collect();
                let resp = model::ListAliasesResponse {
                    aliases: Some(entries),
                    ..Default::default()
                };
                wire::serialize_list_aliases_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Event Source Mapping handlers ==========

    async fn handle_create_event_source_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_event_source_mapping_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.function_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'FunctionName'");
        }
        let enabled = input.enabled.unwrap_or(true);

        let mut state = state.write().await;
        match state.create_event_source_mapping(
            &input.function_name,
            input.event_source_arn.as_deref(),
            input.batch_size,
            enabled,
            input.starting_position.as_deref(),
        ) {
            Ok(esm) => {
                let resp = esm_configuration_from(esm);
                wire::serialize_create_event_source_mapping_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_event_source_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_event_source_mapping_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_event_source_mapping(&input.u_u_i_d) {
            Ok(esm) => {
                let resp = esm_configuration_from(esm);
                wire::serialize_get_event_source_mapping_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_update_event_source_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_event_source_mapping_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.update_event_source_mapping(
            &input.u_u_i_d,
            input.function_name.as_deref(),
            input.batch_size,
            input.enabled,
        ) {
            Ok(esm) => {
                let resp = esm_configuration_from(esm);
                wire::serialize_update_event_source_mapping_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_event_source_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_event_source_mapping_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_event_source_mapping(&input.u_u_i_d) {
            Ok(esm) => {
                let resp = esm_configuration_from(&esm);
                wire::serialize_delete_event_source_mapping_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_event_source_mappings(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_event_source_mappings_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let mappings = state.list_event_source_mappings();
        let entries: Vec<model::EventSourceMappingConfiguration> =
            mappings.iter().map(|m| esm_configuration_from(m)).collect();
        let resp = model::ListEventSourceMappingsResponse {
            event_source_mappings: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_event_source_mappings_response(&resp)
    }

    // ========== Layer handlers ==========

    async fn handle_publish_layer_version(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_layer_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref().unwrap_or("");
        let compatible_runtimes = input.compatible_runtimes.unwrap_or_default();
        let compatible_architectures = input.compatible_architectures.unwrap_or_default();

        let mut state = state.write().await;
        match state.publish_layer_version(
            &input.layer_name,
            description,
            compatible_runtimes,
            compatible_architectures,
            input.license_info.as_deref(),
            account_id,
            region,
        ) {
            Ok(lv) => {
                let resp = layer_version_response_from(lv);
                wire::serialize_publish_layer_version_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_layer_version(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_layer_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_layer_version(&input.layer_name, input.version_number) {
            Ok(lv) => {
                let resp = get_layer_version_response_from(lv);
                wire::serialize_get_layer_version_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_layer_version(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_layer_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_layer_version(&input.layer_name, input.version_number) {
            Ok(()) => wire::serialize_delete_layer_version_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_layer_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_layer_versions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let versions = state.list_layer_versions(&input.layer_name);
        let entries: Vec<model::LayerVersionsListItem> = versions
            .iter()
            .map(|lv| model::LayerVersionsListItem {
                layer_version_arn: Some(lv.layer_version_arn.clone()),
                version: Some(lv.version),
                description: Some(lv.description.clone()),
                created_date: Some(lv.created_date.clone()),
                compatible_runtimes: if lv.compatible_runtimes.is_empty() {
                    None
                } else {
                    Some(lv.compatible_runtimes.clone())
                },
                compatible_architectures: if lv.compatible_architectures.is_empty() {
                    None
                } else {
                    Some(lv.compatible_architectures.clone())
                },
                ..Default::default()
            })
            .collect();
        let resp = model::ListLayerVersionsResponse {
            layer_versions: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_layer_versions_response(&resp)
    }

    async fn handle_list_layers(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_layers_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let layers = state.list_layers();
        let entries: Vec<model::LayersListItem> = layers
            .iter()
            .map(|(name, lv)| model::LayersListItem {
                layer_name: Some(name.to_string()),
                layer_arn: Some(lv.layer_arn.clone()),
                latest_matching_version: Some(model::LayerVersionsListItem {
                    layer_version_arn: Some(lv.layer_version_arn.clone()),
                    version: Some(lv.version),
                    description: Some(lv.description.clone()),
                    created_date: Some(lv.created_date.clone()),
                    compatible_runtimes: if lv.compatible_runtimes.is_empty() {
                        None
                    } else {
                        Some(lv.compatible_runtimes.clone())
                    },
                    ..Default::default()
                }),
            })
            .collect();
        let resp = model::ListLayersResponse {
            layers: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_layers_response(&resp)
    }

    async fn handle_add_layer_version_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_add_layer_version_permission_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.statement_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'StatementId'");
        }
        if input.principal.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Principal'");
        }
        let action = if input.action.is_empty() {
            "lambda:GetLayerVersion"
        } else {
            input.action.as_str()
        };

        let mut state = state.write().await;
        match state.add_layer_version_permission(
            &input.layer_name,
            input.version_number,
            &input.statement_id,
            action,
            &input.principal,
            input.organization_id.as_deref(),
            input.revision_id.as_deref(),
        ) {
            Ok((statement, revision_id)) => {
                let resp = model::AddLayerVersionPermissionResponse {
                    statement: Some(statement),
                    revision_id: Some(revision_id),
                };
                wire::serialize_add_layer_version_permission_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_remove_layer_version_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_remove_layer_version_permission_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.remove_layer_version_permission(
            &input.layer_name,
            input.version_number,
            &input.statement_id,
            input.revision_id.as_deref(),
        ) {
            Ok(_revision_id) => wire::serialize_remove_layer_version_permission_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_layer_version_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_layer_version_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_layer_version_policy(&input.layer_name, input.version_number) {
            Ok((policy, revision_id)) => {
                let resp = model::GetLayerVersionPolicyResponse {
                    policy: Some(policy),
                    revision_id: Some(revision_id),
                };
                wire::serialize_get_layer_version_policy_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Permission handlers ==========

    async fn handle_add_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_permission_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.statement_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'StatementId'");
        }
        if input.principal.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Principal'");
        }
        let action = if input.action.is_empty() {
            "lambda:InvokeFunction"
        } else {
            input.action.as_str()
        };

        let mut state = state.write().await;
        match state.add_permission(
            &input.function_name,
            &input.statement_id,
            action,
            &input.principal,
            input.source_arn.as_deref(),
            input.source_account.as_deref(),
            input.revision_id.as_deref(),
        ) {
            Ok((statement, _revision_id)) => {
                let resp = model::AddPermissionResponse {
                    statement: Some(statement),
                };
                wire::serialize_add_permission_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_remove_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_permission_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.remove_permission(
            &input.function_name,
            &input.statement_id,
            input.revision_id.as_deref(),
        ) {
            Ok(_revision_id) => wire::serialize_remove_permission_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_policy(&input.function_name) {
            Ok((policy, revision_id)) => {
                let resp = model::GetPolicyResponse {
                    policy: Some(policy),
                    revision_id: Some(revision_id),
                };
                wire::serialize_get_policy_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Function URL Config handlers ==========
    // POST /2015-03-31/functions/{FunctionName}/url - CreateFunctionUrlConfig
    async fn handle_create_function_url_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_function_url_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let auth_type = if input.auth_type.is_empty() {
            "NONE"
        } else {
            input.auth_type.as_str()
        };
        let cors = input.cors.as_ref().map(model_cors_to_types);

        let mut state = state.write().await;
        match state.create_function_url_config(
            &input.function_name,
            auth_type,
            cors,
            input.invoke_mode.as_deref(),
        ) {
            Ok(config) => {
                let resp = model::CreateFunctionUrlConfigResponse {
                    function_arn: Some(config.function_arn.clone()),
                    function_url: Some(config.function_url.clone()),
                    auth_type: Some(config.auth_type.clone()),
                    cors: config.cors.as_ref().map(cors_to_model),
                    invoke_mode: config.invoke_mode.clone(),
                    creation_time: Some(config.creation_time.clone()),
                    ..Default::default()
                };
                wire::serialize_create_function_url_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // GET /2015-03-31/functions/{FunctionName}/url - GetFunctionUrlConfig
    async fn handle_get_function_url_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_function_url_config_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_function_url_config(&input.function_name) {
            Ok(config) => {
                let resp = model::GetFunctionUrlConfigResponse {
                    function_arn: Some(config.function_arn.clone()),
                    function_url: Some(config.function_url.clone()),
                    auth_type: Some(config.auth_type.clone()),
                    cors: config.cors.as_ref().map(cors_to_model),
                    invoke_mode: config.invoke_mode.clone(),
                    creation_time: Some(config.creation_time.clone()),
                    last_modified_time: Some(config.last_modified_time.clone()),
                    ..Default::default()
                };
                wire::serialize_get_function_url_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // PUT /2015-03-31/functions/{FunctionName}/url - UpdateFunctionUrlConfig
    async fn handle_update_function_url_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_function_url_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        // The state setter expects Option<Option<CorsConfig>>: outer None = leave alone,
        // outer Some = set (Some or None). We map the deserialized Option<Cors>
        // into Some(...) when the field is present; absent fields stay as None.
        let cors = input.cors.as_ref().map(|c| Some(model_cors_to_types(c)));

        let mut state = state.write().await;
        match state.update_function_url_config(
            &input.function_name,
            input.auth_type.as_deref(),
            cors,
            input.invoke_mode.as_deref(),
        ) {
            Ok(config) => {
                let resp = model::UpdateFunctionUrlConfigResponse {
                    function_arn: Some(config.function_arn.clone()),
                    function_url: Some(config.function_url.clone()),
                    auth_type: Some(config.auth_type.clone()),
                    cors: config.cors.as_ref().map(cors_to_model),
                    invoke_mode: config.invoke_mode.clone(),
                    creation_time: Some(config.creation_time.clone()),
                    last_modified_time: Some(config.last_modified_time.clone()),
                    ..Default::default()
                };
                wire::serialize_update_function_url_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // DELETE /2015-03-31/functions/{FunctionName}/url - DeleteFunctionUrlConfig
    async fn handle_delete_function_url_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_function_url_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_function_url_config(&input.function_name) {
            Ok(()) => wire::serialize_delete_function_url_config_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Concurrency handlers ==========

    async fn handle_put_function_concurrency(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_function_concurrency_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Detect a missing field vs explicit zero by re-checking the raw body.
        // The Smithy struct gives default 0, but the original handler returned a
        // ValidationException when the field was absent; preserve that contract.
        let body_has_reserved = if request.body.is_empty() {
            false
        } else {
            serde_json::from_slice::<Value>(&request.body)
                .ok()
                .and_then(|v| v.get("ReservedConcurrentExecutions").cloned())
                .is_some()
        };
        if !body_has_reserved {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'ReservedConcurrentExecutions'",
            );
        }

        let mut state = state.write().await;
        match state
            .put_function_concurrency(&input.function_name, input.reserved_concurrent_executions)
        {
            Ok(val) => {
                let resp = model::Concurrency {
                    reserved_concurrent_executions: Some(val),
                };
                wire::serialize_put_function_concurrency_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_function_concurrency(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_function_concurrency_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_function_concurrency(&input.function_name) {
            Ok(val) => {
                let resp = model::GetFunctionConcurrencyResponse {
                    reserved_concurrent_executions: val,
                };
                wire::serialize_get_function_concurrency_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_function_concurrency(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_function_concurrency_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_function_concurrency(&input.function_name) {
            Ok(()) => wire::serialize_delete_function_concurrency_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Function Event Invoke Config handlers ==========

    async fn handle_put_function_event_invoke_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_function_event_invoke_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let on_success = input
            .destination_config
            .as_ref()
            .and_then(|d| d.on_success.as_ref())
            .and_then(|s| s.destination.as_deref());
        let on_failure = input
            .destination_config
            .as_ref()
            .and_then(|d| d.on_failure.as_ref())
            .and_then(|f| f.destination.as_deref());

        let mut state = state.write().await;
        match state.put_function_event_invoke_config(
            &input.function_name,
            input.maximum_retry_attempts,
            input.maximum_event_age_in_seconds,
            on_success,
            on_failure,
        ) {
            Ok(config) => {
                let resp = event_invoke_config_to_model(config);
                wire::serialize_put_function_event_invoke_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_function_event_invoke_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_function_event_invoke_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_function_event_invoke_config(&input.function_name) {
            Ok(config) => {
                let resp = event_invoke_config_to_model(config);
                wire::serialize_get_function_event_invoke_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_update_function_event_invoke_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_function_event_invoke_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Preserve the original "presence" semantics: outer Some means "set
        // (possibly to None)"; outer None means "leave alone". The wire fn collapses
        // missing/null into None on inner OnSuccess/OnFailure, so we treat
        // DestinationConfig presence as the gate for whether to touch the
        // destinations at all.
        let (on_success, on_failure) = match input.destination_config.as_ref() {
            Some(dc) => (
                Some(
                    dc.on_success
                        .as_ref()
                        .and_then(|s| s.destination.as_deref()),
                ),
                Some(
                    dc.on_failure
                        .as_ref()
                        .and_then(|f| f.destination.as_deref()),
                ),
            ),
            None => (None, None),
        };

        let mut state = state.write().await;
        match state.update_function_event_invoke_config(
            &input.function_name,
            input.maximum_retry_attempts,
            input.maximum_event_age_in_seconds,
            on_success,
            on_failure,
        ) {
            Ok(config) => {
                let resp = event_invoke_config_to_model(config);
                wire::serialize_update_function_event_invoke_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_function_event_invoke_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_function_event_invoke_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_function_event_invoke_config(&input.function_name) {
            Ok(()) => wire::serialize_delete_function_event_invoke_config_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_function_event_invoke_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_function_event_invoke_configs_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_function_event_invoke_configs(&input.function_name) {
            Ok(configs) => {
                let entries: Vec<model::FunctionEventInvokeConfig> = configs
                    .iter()
                    .map(|c| event_invoke_config_to_model(c))
                    .collect();
                let resp = model::ListFunctionEventInvokeConfigsResponse {
                    function_event_invoke_configs: Some(entries),
                    ..Default::default()
                };
                wire::serialize_list_function_event_invoke_configs_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Tag handlers ==========

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags(&input.resource) {
            Ok(tags) => {
                let resp = model::ListTagsResponse {
                    tags: Some(tags.clone()),
                };
                wire::serialize_list_tags_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.tag_resource(&input.resource, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // Note: AWS SDKs serialize the `tagKeys` httpQuery as repeated params
        // (e.g. ?tagKeys=a&tagKeys=b), but `parse_query_string` collapses repeated
        // keys to the last value. We re-parse from the raw URI here so multi-key
        // untag requests are not silently truncated.
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let raw_query = extract_query(&request.uri);
        let tag_keys = parse_tag_keys_from_query(&raw_query);
        let tag_keys = if tag_keys.is_empty() {
            input.tag_keys
        } else {
            tag_keys
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Version handlers ==========

    async fn handle_publish_version(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut state = state.write().await;
        match state.publish_version(
            &input.function_name,
            input.description.as_deref(),
            input.revision_id.as_deref(),
        ) {
            Ok((func, version_str)) => {
                let mut resp = function_configuration_from(func);
                resp.version = Some(version_str.clone());
                // The function ARN for a published version includes the version qualifier
                resp.function_arn = Some(format!("{}:{}", func.function_arn, version_str));
                // Use the description from the published version, not $LATEST
                if let Some(fv) = func.versions.iter().find(|v| v.version == version_str) {
                    resp.description = Some(fv.description.clone());
                }
                wire::serialize_publish_version_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_versions_by_function(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_versions_by_function_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let function_name = &input.function_name;
        let state = state.read().await;
        match state.list_versions_by_function(function_name) {
            Ok(versions) => {
                let entries: Vec<model::FunctionConfiguration> = versions
                    .iter()
                    .map(|(func, ver)| {
                        let mut config = function_configuration_from(func);
                        match ver {
                            Some(v) => {
                                config.version = Some(v.version.clone());
                                config.description = Some(v.description.clone());
                                config.code_sha256 = Some(v.code_sha256.clone());
                                config.code_size = Some(v.code_size);
                                // Include version qualifier in ARN
                                config.function_arn =
                                    Some(format!("{}:{}", func.function_arn, v.version));
                            }
                            None => {
                                // $LATEST version also includes qualifier in ARN
                                config.function_arn =
                                    Some(format!("{}:$LATEST", func.function_arn));
                            }
                        }
                        config
                    })
                    .collect();
                let resp = model::ListVersionsByFunctionResponse {
                    versions: Some(entries),
                    ..Default::default()
                };
                wire::serialize_list_versions_by_function_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Code Signing Config handler ==========

    async fn handle_get_function_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_function_code_signing_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_function_code_signing_config(&input.function_name) {
            Ok((func, csc_arn)) => {
                let resp = model::GetFunctionCodeSigningConfigResponse {
                    function_name: Some(func.function_name.clone()),
                    code_signing_config_arn: csc_arn.map(|s| s.to_string()),
                };
                wire::serialize_get_function_code_signing_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== GetFunctionConfiguration handler ==========

    async fn handle_get_function_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_function_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_function(&input.function_name) {
            Ok(func) => {
                let resp = function_configuration_from(func);
                wire::serialize_get_function_configuration_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== GetAccountSettings handler ==========

    async fn handle_get_account_settings(
        &self,
        _state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_account_settings_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let resp = model::GetAccountSettingsResponse {
            account_limit: Some(model::AccountLimit {
                total_code_size: Some(80530636800),
                code_size_unzipped: Some(262144000),
                code_size_zipped: Some(52428800),
                concurrent_executions: Some(1000),
                unreserved_concurrent_executions: Some(1000),
            }),
            account_usage: Some(model::AccountUsage {
                total_code_size: Some(0),
                function_count: Some(0),
            }),
        };
        wire::serialize_get_account_settings_response(&resp)
    }

    // ========== InvokeAsync handler ==========

    async fn handle_invoke_async(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_invoke_async_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_function(&input.function_name) {
            Ok(_) => {
                let resp = model::InvokeAsyncResponse { status: Some(202) };
                wire::serialize_invoke_async_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== InvokeWithResponseStream handler ==========

    async fn handle_invoke_with_response_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_invoke_with_response_stream_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.resolve_qualifier(&input.function_name, input.qualifier.as_deref()) {
            Ok(resolved_version) => {
                let resp = model::InvokeWithResponseStreamResponse {
                    executed_version: Some(resolved_version.clone()),
                    response_stream_content_type: Some(
                        "application/vnd.amazon.eventstream".to_string(),
                    ),
                    ..Default::default()
                };
                let mut response = wire::serialize_invoke_with_response_stream_response(&resp);
                // The wire layer leaves header-bound members for the handler
                // to set: X-Amz-Executed-Version comes from the @httpHeader
                // binding on the ExecutedVersion model member.
                response
                    .headers
                    .insert(X_AMZ_EXECUTED_VERSION, resolved_version.parse().unwrap());
                response
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Code Signing Config CRUD handlers ==========

    async fn handle_create_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_code_signing_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let allowed_publishers = input.allowed_publishers.signing_profile_version_arns;
        let untrusted = input
            .code_signing_policies
            .as_ref()
            .and_then(|p| p.untrusted_artifact_on_deployment.as_deref())
            .unwrap_or("Warn");

        let mut state = state.write().await;
        let entry = state.create_code_signing_config(
            input.description.as_deref(),
            allowed_publishers,
            untrusted,
        );
        let resp = model::CreateCodeSigningConfigResponse {
            code_signing_config: Some(csc_entry_to_model(entry)),
        };
        wire::serialize_create_code_signing_config_response(&resp)
    }

    async fn handle_get_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_code_signing_config_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_code_signing_config(&input.code_signing_config_arn) {
            Ok(entry) => {
                let resp = model::GetCodeSigningConfigResponse {
                    code_signing_config: Some(csc_entry_to_model(entry)),
                };
                wire::serialize_get_code_signing_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_update_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_code_signing_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let allowed_publishers = input
            .allowed_publishers
            .map(|p| p.signing_profile_version_arns);
        let untrusted = input
            .code_signing_policies
            .as_ref()
            .and_then(|p| p.untrusted_artifact_on_deployment.as_deref());

        let mut state = state.write().await;
        match state.update_code_signing_config(
            &input.code_signing_config_arn,
            input.description.as_deref(),
            allowed_publishers,
            untrusted,
        ) {
            Ok(entry) => {
                let resp = model::UpdateCodeSigningConfigResponse {
                    code_signing_config: Some(csc_entry_to_model(entry)),
                };
                wire::serialize_update_code_signing_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_code_signing_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_code_signing_config(&input.code_signing_config_arn) {
            Ok(()) => wire::serialize_delete_code_signing_config_response(
                &model::DeleteCodeSigningConfigResponse {},
            ),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_code_signing_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_code_signing_configs_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let entries: Vec<model::CodeSigningConfig> = state
            .list_code_signing_configs()
            .iter()
            .map(|e| csc_entry_to_model(e))
            .collect();
        let resp = model::ListCodeSigningConfigsResponse {
            code_signing_configs: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_code_signing_configs_response(&resp)
    }

    async fn handle_put_function_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_function_code_signing_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.code_signing_config_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'CodeSigningConfigArn'");
        }
        let mut state = state.write().await;
        match state
            .put_function_code_signing_config(&input.function_name, &input.code_signing_config_arn)
        {
            Ok(()) => {
                let resp = model::PutFunctionCodeSigningConfigResponse {
                    code_signing_config_arn: Some(input.code_signing_config_arn.clone()),
                    function_name: Some(input.function_name.clone()),
                };
                wire::serialize_put_function_code_signing_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_function_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_function_code_signing_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_function_code_signing_config(&input.function_name) {
            Ok(()) => wire::serialize_delete_function_code_signing_config_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_functions_by_code_signing_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_functions_by_code_signing_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_functions_by_code_signing_config(&input.code_signing_config_arn) {
            Ok(funcs) => {
                let resp = model::ListFunctionsByCodeSigningConfigResponse {
                    function_arns: Some(funcs),
                    ..Default::default()
                };
                wire::serialize_list_functions_by_code_signing_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Provisioned Concurrency handlers ==========

    async fn handle_put_provisioned_concurrency_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_provisioned_concurrency_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Detect missing field via raw body re-parse to preserve the original
        // Missing-field validation error wording.
        let body_has_field = if request.body.is_empty() {
            false
        } else {
            serde_json::from_slice::<Value>(&request.body)
                .ok()
                .and_then(|v| v.get("ProvisionedConcurrentExecutions").cloned())
                .is_some()
        };
        if !body_has_field {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'ProvisionedConcurrentExecutions'",
            );
        }
        let mut state = state.write().await;
        match state.put_provisioned_concurrency_config(
            &input.function_name,
            &input.qualifier,
            input.provisioned_concurrent_executions,
        ) {
            Ok(config) => {
                let resp = model::PutProvisionedConcurrencyConfigResponse {
                    requested_provisioned_concurrent_executions: Some(config.requested),
                    allocated_provisioned_concurrent_executions: Some(config.requested),
                    available_provisioned_concurrent_executions: Some(config.requested),
                    status: Some("READY".to_string()),
                    last_modified: Some(config.last_modified.clone()),
                    ..Default::default()
                };
                wire::serialize_put_provisioned_concurrency_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_get_provisioned_concurrency_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_provisioned_concurrency_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_provisioned_concurrency_config(&input.function_name, &input.qualifier) {
            Ok(config) => {
                let resp = model::GetProvisionedConcurrencyConfigResponse {
                    requested_provisioned_concurrent_executions: Some(config.requested),
                    allocated_provisioned_concurrent_executions: Some(config.requested),
                    available_provisioned_concurrent_executions: Some(config.requested),
                    status: Some("READY".to_string()),
                    last_modified: Some(config.last_modified.clone()),
                    ..Default::default()
                };
                wire::serialize_get_provisioned_concurrency_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_provisioned_concurrency_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_provisioned_concurrency_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_provisioned_concurrency_config(&input.function_name, &input.qualifier) {
            Ok(()) => wire::serialize_delete_provisioned_concurrency_config_response(),
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_provisioned_concurrency_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_provisioned_concurrency_configs_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let configs: Vec<model::ProvisionedConcurrencyConfigListItem> = state
            .list_provisioned_concurrency_configs(&input.function_name)
            .iter()
            .map(|c| model::ProvisionedConcurrencyConfigListItem {
                function_arn: Some(c.function_arn.clone()),
                requested_provisioned_concurrent_executions: Some(c.requested),
                allocated_provisioned_concurrent_executions: Some(c.requested),
                available_provisioned_concurrent_executions: Some(c.requested),
                status: Some("READY".to_string()),
                last_modified: Some(c.last_modified.clone()),
                ..Default::default()
            })
            .collect();
        let resp = model::ListProvisionedConcurrencyConfigsResponse {
            provisioned_concurrency_configs: Some(configs),
            ..Default::default()
        };
        wire::serialize_list_provisioned_concurrency_configs_response(&resp)
    }

    // ========== Capacity Provider handlers ==========

    async fn handle_create_capacity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_capacity_provider_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.capacity_provider_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'CapacityProviderName'");
        }
        let mut state = state.write().await;
        let entry = state.create_capacity_provider(&input.capacity_provider_name);
        let resp = model::CreateCapacityProviderResponse {
            capacity_provider: Some(model::CapacityProvider {
                capacity_provider_arn: Some(entry.capacity_provider_arn.clone()),
                state: Some(entry.state.clone()),
                last_modified: Some(entry.last_modified.clone()),
                ..Default::default()
            }),
        };
        wire::serialize_create_capacity_provider_response(&resp)
    }

    async fn handle_get_capacity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_capacity_provider_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_capacity_provider(&input.capacity_provider_name) {
            Ok(entry) => {
                let resp = model::GetCapacityProviderResponse {
                    capacity_provider: Some(model::CapacityProvider {
                        capacity_provider_arn: Some(entry.capacity_provider_arn.clone()),
                        state: Some(entry.state.clone()),
                        last_modified: Some(entry.last_modified.clone()),
                        ..Default::default()
                    }),
                };
                wire::serialize_get_capacity_provider_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_update_capacity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_capacity_provider_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_capacity_provider(&input.capacity_provider_name) {
            Ok(entry) => {
                let resp = model::UpdateCapacityProviderResponse {
                    capacity_provider: Some(model::CapacityProvider {
                        capacity_provider_arn: Some(entry.capacity_provider_arn.clone()),
                        state: Some(entry.state.clone()),
                        last_modified: Some(entry.last_modified.clone()),
                        ..Default::default()
                    }),
                };
                wire::serialize_update_capacity_provider_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_delete_capacity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_capacity_provider_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_capacity_provider(&input.capacity_provider_name) {
            Ok(entry) => {
                let resp = model::DeleteCapacityProviderResponse {
                    capacity_provider: Some(model::CapacityProvider {
                        capacity_provider_arn: Some(entry.capacity_provider_arn.clone()),
                        state: Some(entry.state.clone()),
                        ..Default::default()
                    }),
                };
                wire::serialize_delete_capacity_provider_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_list_capacity_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_capacity_providers_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let items: Vec<model::CapacityProvider> = state
            .list_capacity_providers()
            .iter()
            .map(|e| model::CapacityProvider {
                capacity_provider_arn: Some(e.capacity_provider_arn.clone()),
                state: Some(e.state.clone()),
                last_modified: Some(e.last_modified.clone()),
                ..Default::default()
            })
            .collect();
        let resp = model::ListCapacityProvidersResponse {
            capacity_providers: Some(items),
            ..Default::default()
        };
        wire::serialize_list_capacity_providers_response(&resp)
    }

    async fn handle_list_function_versions_by_capacity_provider(
        &self,
        _state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_function_versions_by_capacity_provider_request(
            request, labels, query,
        ) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let resp = model::ListFunctionVersionsByCapacityProviderResponse {
            function_versions: Some(vec![]),
            ..Default::default()
        };
        wire::serialize_list_function_versions_by_capacity_provider_response(&resp)
    }

    // ========== Function Recursion Config handlers ==========

    async fn handle_get_function_recursion_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_function_recursion_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_function_recursion_config(&input.function_name) {
            Ok(recursive_loop) => {
                let resp = model::GetFunctionRecursionConfigResponse {
                    recursive_loop: Some(recursive_loop),
                };
                wire::serialize_get_function_recursion_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_put_function_recursion_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_function_recursion_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let recursive_loop = if input.recursive_loop.is_empty() {
            "Allow"
        } else {
            input.recursive_loop.as_str()
        };
        let mut state = state.write().await;
        match state.put_function_recursion_config(&input.function_name, recursive_loop) {
            Ok(val) => {
                let resp = model::PutFunctionRecursionConfigResponse {
                    recursive_loop: Some(val),
                };
                wire::serialize_put_function_recursion_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Function Scaling Config handlers ==========

    async fn handle_get_function_scaling_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_function_scaling_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_function_scaling_config(&input.function_name) {
            Ok(max_concurrency) => {
                let scaling_config = model::FunctionScalingConfig {
                    max_execution_environments: max_concurrency,
                    ..Default::default()
                };
                let resp = model::GetFunctionScalingConfigResponse {
                    requested_function_scaling_config: Some(scaling_config.clone()),
                    applied_function_scaling_config: Some(scaling_config),
                    function_arn: None,
                };
                wire::serialize_get_function_scaling_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_put_function_scaling_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_function_scaling_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let max_execution_environments = input
            .function_scaling_config
            .as_ref()
            .and_then(|c| c.max_execution_environments);
        let mut state = state.write().await;
        match state.put_function_scaling_config(&input.function_name, max_execution_environments) {
            Ok(_) => {
                let resp = model::PutFunctionScalingConfigResponse {
                    function_state: Some("Active".to_string()),
                };
                wire::serialize_put_function_scaling_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== Runtime Management Config handlers ==========

    async fn handle_get_runtime_management_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_runtime_management_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_runtime_management_config(&input.function_name) {
            Ok(config) => {
                let func_arn = state
                    .get_function(&input.function_name)
                    .map(|f| f.function_arn.clone())
                    .ok();
                let resp = model::GetRuntimeManagementConfigResponse {
                    function_arn: func_arn,
                    update_runtime_on: Some(config.update_runtime_on),
                    runtime_version_arn: config.runtime_version_arn,
                };
                wire::serialize_get_runtime_management_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    async fn handle_put_runtime_management_config(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_runtime_management_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let update_runtime_on = if input.update_runtime_on.is_empty() {
            "Auto"
        } else {
            input.update_runtime_on.as_str()
        };
        let mut state = state.write().await;
        match state.put_runtime_management_config(
            &input.function_name,
            update_runtime_on,
            input.runtime_version_arn.as_deref(),
        ) {
            Ok(config) => {
                let func_arn = state
                    .get_function(&input.function_name)
                    .map(|f| f.function_arn.clone())
                    .ok();
                let resp = model::PutRuntimeManagementConfigResponse {
                    function_arn: func_arn,
                    update_runtime_on: Some(config.update_runtime_on),
                    runtime_version_arn: config.runtime_version_arn,
                };
                wire::serialize_put_runtime_management_config_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== GetLayerVersionByArn handler ==========

    async fn handle_get_layer_version_by_arn(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_layer_version_by_arn_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_layer_version_by_arn(&input.arn) {
            Ok(lv) => {
                let resp = get_layer_version_response_from(lv);
                wire::serialize_get_layer_version_by_arn_response(&resp)
            }
            Err(e) => lambda_error_response(&e),
        }
    }

    // ========== ListFunctionUrlConfigs handler ==========

    async fn handle_list_function_url_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_function_url_configs_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let configs: Vec<model::FunctionUrlConfig> = state
            .function_url_configs
            .values()
            .map(|c| model::FunctionUrlConfig {
                function_arn: Some(c.function_arn.clone()),
                function_url: Some(c.function_url.clone()),
                auth_type: Some(c.auth_type.clone()),
                cors: c.cors.as_ref().map(cors_to_model),
                invoke_mode: c.invoke_mode.clone(),
                creation_time: Some(c.creation_time.clone()),
                last_modified_time: Some(c.last_modified_time.clone()),
            })
            .collect();
        let resp = model::ListFunctionUrlConfigsResponse {
            function_url_configs: Some(configs),
            ..Default::default()
        };
        wire::serialize_list_function_url_configs_response(&resp)
    }

    // ========== Durable Execution handlers ==========

    async fn handle_get_durable_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_durable_execution_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let exec_id = input.durable_execution_arn.as_str();
        let state_guard = state.read().await;
        match state_guard.get_durable_execution(exec_id) {
            Ok(exec) => {
                let resp = model::GetDurableExecutionResponse {
                    durable_execution_arn: Some(exec.durable_execution_arn.clone()),
                    durable_execution_name: Some(exec.durable_execution_name.clone()),
                    function_arn: Some(exec.function_arn.clone()),
                    status: Some(exec.status.clone()),
                    start_timestamp: exec.start_timestamp,
                    end_timestamp: exec.end_timestamp,
                    input_payload: exec.input_payload.clone(),
                    result: exec.result.clone(),
                    version: exec.version.clone(),
                    ..Default::default()
                };
                wire::serialize_get_durable_execution_response(&resp)
            }
            Err(_) => {
                // If not found, return a default response with the ARN and ACTIVE status
                // to preserve backward compatibility with callers that don't create executions first.
                let resp = model::GetDurableExecutionResponse {
                    durable_execution_arn: Some(exec_id.to_string()),
                    status: Some("ACTIVE".to_string()),
                    ..Default::default()
                };
                wire::serialize_get_durable_execution_response(&resp)
            }
        }
    }

    async fn handle_get_durable_execution_history(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_durable_execution_history_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state_guard = state.read().await;
        // Verify the execution exists (or return empty history for unknown executions)
        let _ = state_guard.get_durable_execution(&input.durable_execution_arn);
        let resp = model::GetDurableExecutionHistoryResponse {
            events: Some(vec![]),
            ..Default::default()
        };
        wire::serialize_get_durable_execution_history_response(&resp)
    }

    async fn handle_get_durable_execution_state(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_durable_execution_state_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state_guard = state.read().await;
        let _ = state_guard.get_durable_execution(&input.durable_execution_arn);
        let resp = model::GetDurableExecutionStateResponse {
            ..Default::default()
        };
        wire::serialize_get_durable_execution_state_response(&resp)
    }

    async fn handle_list_durable_executions_by_function(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_durable_executions_by_function_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state_guard = state.read().await;
        let executions: Vec<model::Execution> = state_guard
            .list_durable_executions_by_function(&input.function_name)
            .iter()
            .map(|e| model::Execution {
                durable_execution_arn: Some(e.durable_execution_arn.clone()),
                durable_execution_name: Some(e.durable_execution_name.clone()),
                function_arn: Some(e.function_arn.clone()),
                status: Some(e.status.clone()),
                start_timestamp: e.start_timestamp,
                end_timestamp: e.end_timestamp,
            })
            .collect();
        let resp = model::ListDurableExecutionsByFunctionResponse {
            durable_executions: Some(executions),
            ..Default::default()
        };
        wire::serialize_list_durable_executions_by_function_response(&resp)
    }

    async fn handle_checkpoint_durable_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_checkpoint_durable_execution_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state_guard = state.read().await;
        // Verify the execution exists; return default checkpoint response either way.
        let _ = state_guard.get_durable_execution(&input.durable_execution_arn);
        let resp = model::CheckpointDurableExecutionResponse {
            ..Default::default()
        };
        wire::serialize_checkpoint_durable_execution_response(&resp)
    }

    async fn handle_send_durable_execution_callback_failure(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_send_durable_execution_callback_failure_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state_guard = state.read().await;
        let _ = state_guard.get_durable_execution(&input.callback_id);
        let resp = model::SendDurableExecutionCallbackFailureResponse {};
        wire::serialize_send_durable_execution_callback_failure_response(&resp)
    }

    async fn handle_send_durable_execution_callback_heartbeat(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_send_durable_execution_callback_heartbeat_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state_guard = state.read().await;
        let _ = state_guard.get_durable_execution(&input.callback_id);
        let resp = model::SendDurableExecutionCallbackHeartbeatResponse {};
        wire::serialize_send_durable_execution_callback_heartbeat_response(&resp)
    }

    async fn handle_send_durable_execution_callback_success(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_send_durable_execution_callback_success_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state_guard = state.read().await;
        let _ = state_guard.get_durable_execution(&input.callback_id);
        let resp = model::SendDurableExecutionCallbackSuccessResponse {};
        wire::serialize_send_durable_execution_callback_success_response(&resp)
    }

    async fn handle_stop_durable_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<LambdaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_durable_execution_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state_guard = state.write().await;
        let stop_timestamp = state_guard
            .stop_durable_execution(&input.durable_execution_arn)
            .ok();
        let resp = model::StopDurableExecutionResponse { stop_timestamp };
        wire::serialize_stop_durable_execution_response(&resp)
    }
}

// ========== Helper functions ==========

fn function_configuration_from(
    func: &crate::types::LambdaFunction,
) -> model::FunctionConfiguration {
    let env = if func.environment.is_empty() {
        None
    } else {
        Some(model::EnvironmentResponse {
            variables: Some(func.environment.clone()),
            ..Default::default()
        })
    };

    model::FunctionConfiguration {
        function_name: Some(func.function_name.clone()),
        function_arn: Some(func.function_arn.clone()),
        runtime: Some(func.runtime.clone()),
        role: Some(func.role.clone()),
        handler: Some(func.handler.clone()),
        code_size: Some(func.code_size),
        code_sha256: Some(func.code_sha256.clone()),
        description: Some(func.description.clone()),
        timeout: Some(func.timeout),
        memory_size: Some(func.memory_size),
        last_modified: Some(
            func.last_modified
                .format("%Y-%m-%dT%H:%M:%S%.3f+0000")
                .to_string(),
        ),
        version: Some(func.version.clone()),
        state: Some(func.state.clone()),
        revision_id: Some(func.revision_id.clone()),
        package_type: Some("Zip".to_string()),
        architectures: Some(vec!["x86_64".to_string()]),
        ephemeral_storage: Some(model::EphemeralStorage { size: 512 }),
        tracing_config: Some(model::TracingConfigResponse {
            mode: Some("PassThrough".to_string()),
        }),
        layers: Some(vec![]),
        last_update_status: Some("Successful".to_string()),
        snap_start: Some(model::SnapStartResponse {
            apply_on: Some("None".to_string()),
            optimization_status: Some("Off".to_string()),
        }),
        environment: env,
        ..Default::default()
    }
}

fn alias_configuration_from(alias: &crate::types::Alias) -> model::AliasConfiguration {
    model::AliasConfiguration {
        alias_arn: Some(alias.alias_arn.clone()),
        name: Some(alias.name.clone()),
        function_version: Some(alias.function_version.clone()),
        description: Some(alias.description.clone()),
        revision_id: Some(alias.revision_id.clone()),
        routing_config: alias
            .routing_config
            .as_ref()
            .map(|rc| model::AliasRoutingConfiguration {
                additional_version_weights: Some(rc.clone()),
            }),
    }
}

fn esm_configuration_from(
    esm: &crate::types::EventSourceMapping,
) -> model::EventSourceMappingConfiguration {
    model::EventSourceMappingConfiguration {
        u_u_i_d: Some(esm.uuid.clone()),
        event_source_arn: esm.event_source_arn.clone(),
        function_arn: Some(esm.function_arn.clone()),
        batch_size: esm.batch_size,
        state: Some(esm.state.clone()),
        last_modified: Some(esm.last_modified.timestamp() as f64),
        starting_position: esm.starting_position.clone(),
        ..Default::default()
    }
}

fn layer_version_response_from(
    lv: &crate::types::LayerVersion,
) -> model::PublishLayerVersionResponse {
    model::PublishLayerVersionResponse {
        layer_arn: Some(lv.layer_arn.clone()),
        layer_version_arn: Some(lv.layer_version_arn.clone()),
        version: Some(lv.version),
        description: Some(lv.description.clone()),
        created_date: Some(lv.created_date.clone()),
        compatible_runtimes: if lv.compatible_runtimes.is_empty() {
            None
        } else {
            Some(lv.compatible_runtimes.clone())
        },
        compatible_architectures: if lv.compatible_architectures.is_empty() {
            None
        } else {
            Some(lv.compatible_architectures.clone())
        },
        license_info: lv.license_info.clone(),
        content: Some(model::LayerVersionContentOutput {
            code_sha256: Some(lv.code_sha256.clone()),
            code_size: Some(lv.code_size),
            ..Default::default()
        }),
    }
}

fn get_layer_version_response_from(
    lv: &crate::types::LayerVersion,
) -> model::GetLayerVersionResponse {
    model::GetLayerVersionResponse {
        layer_arn: Some(lv.layer_arn.clone()),
        layer_version_arn: Some(lv.layer_version_arn.clone()),
        version: Some(lv.version),
        description: Some(lv.description.clone()),
        created_date: Some(lv.created_date.clone()),
        compatible_runtimes: if lv.compatible_runtimes.is_empty() {
            None
        } else {
            Some(lv.compatible_runtimes.clone())
        },
        compatible_architectures: if lv.compatible_architectures.is_empty() {
            None
        } else {
            Some(lv.compatible_architectures.clone())
        },
        license_info: lv.license_info.clone(),
        content: Some(model::LayerVersionContentOutput {
            code_sha256: Some(lv.code_sha256.clone()),
            code_size: Some(lv.code_size),
            ..Default::default()
        }),
    }
}

fn event_invoke_config_to_model(
    config: &crate::types::FunctionEventInvokeConfig,
) -> model::FunctionEventInvokeConfig {
    let dest_config =
        if config.on_success_destination.is_some() || config.on_failure_destination.is_some() {
            Some(model::DestinationConfig {
                on_success: config
                    .on_success_destination
                    .as_ref()
                    .map(|d| model::OnSuccess {
                        destination: Some(d.clone()),
                    }),
                on_failure: config
                    .on_failure_destination
                    .as_ref()
                    .map(|d| model::OnFailure {
                        destination: Some(d.clone()),
                    }),
            })
        } else {
            None
        };

    model::FunctionEventInvokeConfig {
        function_arn: Some(config.function_arn.clone()),
        maximum_retry_attempts: config.maximum_retry_attempts,
        maximum_event_age_in_seconds: config.maximum_event_age_in_seconds,
        destination_config: dest_config,
        last_modified: Some(config.last_modified.timestamp() as f64),
    }
}

fn model_cors_to_types(cors: &model::Cors) -> crate::types::CorsConfig {
    crate::types::CorsConfig {
        allow_credentials: cors.allow_credentials,
        allow_headers: cors.allow_headers.clone(),
        allow_methods: cors.allow_methods.clone(),
        allow_origins: cors.allow_origins.clone(),
        expose_headers: cors.expose_headers.clone(),
        max_age: cors.max_age,
    }
}

fn cors_to_model(cors: &crate::types::CorsConfig) -> model::Cors {
    model::Cors {
        allow_credentials: cors.allow_credentials,
        allow_headers: cors.allow_headers.clone(),
        allow_methods: cors.allow_methods.clone(),
        allow_origins: cors.allow_origins.clone(),
        expose_headers: cors.expose_headers.clone(),
        max_age: cors.max_age,
    }
}

fn extract_path(uri: &str) -> String {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => path_and_query[..q].to_string(),
        None => path_and_query.to_string(),
    }
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

fn parse_tag_keys_from_query(query: &str) -> Vec<String> {
    query
        .split('&')
        .filter_map(|param| {
            let parts: Vec<&str> = param.splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == "tagKeys" {
                Some(percent_decode(parts[1]))
            } else {
                None
            }
        })
        .collect()
}

fn lambda_error_response(err: &LambdaError) -> MockResponse {
    let (status, error_type) = match err {
        LambdaError::FunctionAlreadyExists(_) => (409, "ResourceConflictException"),
        LambdaError::FunctionNotFoundByArn { .. } => (404, "ResourceNotFoundException"),
        LambdaError::FunctionNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::AliasAlreadyExists(_) => (409, "ConflictException"),
        LambdaError::AliasNotFoundByArn(_) => (404, "ResourceNotFoundException"),
        LambdaError::AliasNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::EventSourceMappingNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::LayerNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::LayerVersionNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::PermissionAlreadyExists(_) => (409, "ResourceConflictException"),
        LambdaError::PermissionNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::ResourceDoesNotExist => (404, "ResourceNotFoundException"),
        LambdaError::FunctionUrlConfigAlreadyExists(_) => (409, "ResourceConflictException"),
        LambdaError::FunctionUrlConfigNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::EventInvokeConfigNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::CodeSigningConfigNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::ProvisionedConcurrencyConfigNotFound(_) => {
            (404, "ProvisionedConcurrencyConfigNotFoundException")
        }
        LambdaError::CapacityProviderNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::DurableExecutionNotFound(_) => (404, "ResourceNotFoundException"),
        LambdaError::PreconditionFailed => (412, "PreconditionFailedException"),
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

fn percent_decode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(byte) =
                u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16)
        {
            result.push(byte);
            i += 3;
            continue;
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(result).unwrap_or_else(|_| s.to_string())
}

fn parse_query_param(query: &str, key: &str) -> Option<String> {
    for param in query.split('&') {
        let mut parts = param.splitn(2, '=');
        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
            if k == key {
                return Some(percent_decode(v));
            }
        }
    }
    None
}

fn csc_entry_to_model(entry: &crate::types::CodeSigningConfigEntry) -> model::CodeSigningConfig {
    model::CodeSigningConfig {
        code_signing_config_arn: Some(entry.code_signing_config_arn.clone()),
        code_signing_config_id: Some(entry.code_signing_config_id.clone()),
        description: entry.description.clone(),
        allowed_publishers: Some(model::AllowedPublishers {
            signing_profile_version_arns: entry.allowed_publishers.clone(),
        }),
        code_signing_policies: Some(model::CodeSigningPolicies {
            untrusted_artifact_on_deployment: Some(entry.untrusted_artifact_on_deployment.clone()),
        }),
        last_modified: Some(entry.last_modified.clone()),
    }
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
