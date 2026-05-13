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

use crate::state::{AmpError, AmpState};
use crate::views::AmpStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AmpService {
    pub(crate) state: Arc<BackendState<AmpState>>,
    pub(crate) notifier: StateChangeNotifier<AmpStateView>,
}

impl AmpService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AmpService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AmpService {
    fn service_name(&self) -> &str {
        "amp"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://aps\..*\.amazonaws\.com",
            r"https?://aps\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AmpService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        // Routes for /tags/{resourceArn}
        if segments[0] == "tags" && segments.len() >= 2 {
            let resource_arn = percent_decode(&segments[1..].join("/"));
            let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
            let response = match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                        .await
                }
                "POST" => {
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    self.handle_untag_resource(&state, &request, labels, &query_map, &raw_query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
            if matches!(method, "POST" | "DELETE") && response.status / 100 == 2 {
                self.notify_state_changed(account_id, &region).await;
            }
            return response;
        }

        if segments[0] != "workspaces" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let response = match (method, segments.as_slice()) {
            // POST /workspaces - CreateWorkspace
            ("POST", ["workspaces"]) => {
                self.handle_create_workspace(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // GET /workspaces - ListWorkspaces
            ("GET", ["workspaces"]) => {
                self.handle_list_workspaces(&state, &request, &[], &query_map)
                    .await
            }
            // GET /workspaces/{workspaceId} - DescribeWorkspace
            ("GET", ["workspaces", workspace_id]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_describe_workspace(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /workspaces/{workspaceId} - DeleteWorkspace
            ("DELETE", ["workspaces", workspace_id]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_delete_workspace(&state, &request, labels, &query_map)
                    .await
            }
            // POST /workspaces/{workspaceId}/alias - UpdateWorkspaceAlias
            ("POST", ["workspaces", workspace_id, "alias"]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_update_workspace_alias(&state, &request, labels, &query_map)
                    .await
            }
            // POST /workspaces/{workspaceId}/logging - CreateLoggingConfiguration
            ("POST", ["workspaces", workspace_id, "logging"]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_create_logging_configuration(&state, &request, labels, &query_map)
                    .await
            }
            // GET /workspaces/{workspaceId}/logging - DescribeLoggingConfiguration
            ("GET", ["workspaces", workspace_id, "logging"]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_describe_logging_configuration(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /workspaces/{workspaceId}/logging - UpdateLoggingConfiguration
            ("PUT", ["workspaces", workspace_id, "logging"]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_update_logging_configuration(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /workspaces/{workspaceId}/logging - DeleteLoggingConfiguration
            ("DELETE", ["workspaces", workspace_id, "logging"]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_delete_logging_configuration(&state, &request, labels, &query_map)
                    .await
            }
            // POST /workspaces/{workspaceId}/rulegroupsnamespaces - CreateRuleGroupsNamespace
            ("POST", ["workspaces", workspace_id, "rulegroupsnamespaces"]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_create_rule_groups_namespace(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // GET /workspaces/{workspaceId}/rulegroupsnamespaces - ListRuleGroupsNamespaces
            ("GET", ["workspaces", workspace_id, "rulegroupsnamespaces"]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id)];
                self.handle_list_rule_groups_namespaces(&state, &request, labels, &query_map)
                    .await
            }
            // GET /workspaces/{workspaceId}/rulegroupsnamespaces/{name} - DescribeRuleGroupsNamespace
            ("GET", ["workspaces", workspace_id, "rulegroupsnamespaces", name]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id), ("name", name)];
                self.handle_describe_rule_groups_namespace(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /workspaces/{workspaceId}/rulegroupsnamespaces/{name} - PutRuleGroupsNamespace
            ("PUT", ["workspaces", workspace_id, "rulegroupsnamespaces", name]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id), ("name", name)];
                self.handle_put_rule_groups_namespace(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /workspaces/{workspaceId}/rulegroupsnamespaces/{name} - DeleteRuleGroupsNamespace
            ("DELETE", ["workspaces", workspace_id, "rulegroupsnamespaces", name]) => {
                let labels: &[(&str, &str)] = &[("workspaceId", workspace_id), ("name", name)];
                self.handle_delete_rule_groups_namespace(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if matches!(method, "POST" | "PUT" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_workspace(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_workspace_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let alias = input.alias.as_deref();
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_workspace(alias, tags, account_id, region) {
            Ok(ws) => wire::serialize_create_workspace_response(&wire::CreateWorkspaceResponse {
                workspace_id: Some(ws.workspace_id.clone()),
                arn: Some(ws.arn.clone()),
                status: Some(workspace_status_wire(&ws.status)),
                tags: Some(ws.tags.clone()),
                ..Default::default()
            }),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_describe_workspace(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspace_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_workspace(&input.workspace_id) {
            Ok(ws) => {
                wire::serialize_describe_workspace_response(&wire::DescribeWorkspaceResponse {
                    workspace: Some(workspace_description_wire(ws)),
                })
            }
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_delete_workspace(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_workspace_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_workspace(&input.workspace_id) {
            Ok(()) => wire::serialize_delete_workspace_response(),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_list_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_workspaces_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let workspaces = state.list_workspaces(input.alias.as_deref());

        wire::serialize_list_workspaces_response(&wire::ListWorkspacesResponse {
            workspaces: Some(workspaces.iter().map(workspace_summary_wire).collect()),
            next_token: None,
        })
    }

    async fn handle_update_workspace_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_workspace_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_workspace_alias(&input.workspace_id, input.alias.as_deref()) {
            Ok(()) => wire::serialize_update_workspace_alias_response(),
            Err(e) => amp_error_response(&e),
        }
    }

    // --- Logging Configuration ---

    async fn handle_create_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_logging_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.log_group_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'logGroupArn'");
        }
        let mut state = state.write().await;
        match state.create_logging_configuration(&input.workspace_id, &input.log_group_arn) {
            Ok(config) => wire::serialize_create_logging_configuration_response(
                &wire::CreateLoggingConfigurationResponse {
                    status: Some(logging_configuration_status_wire(&config.status)),
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_describe_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_logging_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_logging_configuration(&input.workspace_id) {
            Ok(Some(config)) => wire::serialize_describe_logging_configuration_response(
                &wire::DescribeLoggingConfigurationResponse {
                    logging_configuration: Some(logging_configuration_metadata_wire(config)),
                },
            ),
            Ok(None) => wire::serialize_describe_logging_configuration_response(
                &wire::DescribeLoggingConfigurationResponse {
                    logging_configuration: Some(Default::default()),
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_update_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_logging_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.log_group_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'logGroupArn'");
        }
        let mut state = state.write().await;
        match state.update_logging_configuration(&input.workspace_id, &input.log_group_arn) {
            Ok(config) => wire::serialize_update_logging_configuration_response(
                &wire::UpdateLoggingConfigurationResponse {
                    status: Some(logging_configuration_status_wire(&config.status)),
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_delete_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_logging_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_logging_configuration(&input.workspace_id) {
            Ok(()) => wire::serialize_delete_logging_configuration_response(),
            Err(e) => amp_error_response(&e),
        }
    }

    // --- Rule Groups Namespaces ---

    async fn handle_create_rule_groups_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_rule_groups_namespace_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.data.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'data'");
        }
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_rule_groups_namespace(
            &input.workspace_id,
            &input.name,
            &input.data,
            tags,
            account_id,
            region,
        ) {
            Ok(ns) => wire::serialize_create_rule_groups_namespace_response(
                &wire::CreateRuleGroupsNamespaceResponse {
                    name: Some(ns.name.clone()),
                    arn: Some(ns.arn.clone()),
                    status: Some(rule_groups_namespace_status_wire(&ns.status)),
                    tags: Some(ns.tags.clone()),
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_describe_rule_groups_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_rule_groups_namespace_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_rule_groups_namespace(&input.workspace_id, &input.name) {
            Ok(ns) => wire::serialize_describe_rule_groups_namespace_response(
                &wire::DescribeRuleGroupsNamespaceResponse {
                    rule_groups_namespace: Some(rule_groups_namespace_description_wire(ns)),
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_put_rule_groups_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_rule_groups_namespace_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.data.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'data'");
        }
        let mut state = state.write().await;
        match state.put_rule_groups_namespace(&input.workspace_id, &input.name, &input.data) {
            Ok(ns) => wire::serialize_put_rule_groups_namespace_response(
                &wire::PutRuleGroupsNamespaceResponse {
                    name: Some(ns.name.clone()),
                    arn: Some(ns.arn.clone()),
                    status: Some(rule_groups_namespace_status_wire(&ns.status)),
                    tags: Some(ns.tags.clone()),
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_delete_rule_groups_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_rule_groups_namespace_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_rule_groups_namespace(&input.workspace_id, &input.name) {
            Ok(()) => wire::serialize_delete_rule_groups_namespace_response(),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_list_rule_groups_namespaces(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_rule_groups_namespaces_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.list_rule_groups_namespaces(&input.workspace_id, input.name.as_deref()) {
            Ok(namespaces) => wire::serialize_list_rule_groups_namespaces_response(
                &wire::ListRuleGroupsNamespacesResponse {
                    rule_groups_namespaces: Some(
                        namespaces
                            .iter()
                            .map(|ns| rule_groups_namespace_summary_wire(ns))
                            .collect(),
                    ),
                    next_token: None,
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    // --- Tags ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.find_resource_tags(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: Some(tags.clone()),
                },
            ),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => amp_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AmpState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        raw_query: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The AWS SDK sends tag keys as repeated query params (?tagKeys=k1&tagKeys=k2),
        // but parse_query_string keeps only the last value, so parse them from the raw
        // query string here. Fall back to whatever the wire deserializer parsed (which
        // assumes a comma-separated single param) if no repeated params were found.
        let mut tag_keys = extract_query_param_list(raw_query, "tagKeys");
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys;
        }
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => amp_error_response(&e),
        }
    }
}

fn logging_configuration_status_wire(
    status: &crate::types::LoggingConfigurationStatus,
) -> wire::LoggingConfigurationStatus {
    wire::LoggingConfigurationStatus {
        status_code: Some(status.status_code.clone()),
        ..Default::default()
    }
}

fn logging_configuration_metadata_wire(
    config: &crate::types::LoggingConfiguration,
) -> wire::LoggingConfigurationMetadata {
    wire::LoggingConfigurationMetadata {
        log_group_arn: Some(config.log_group_arn.clone()),
        workspace: Some(config.workspace_id.clone()),
        status: Some(logging_configuration_status_wire(&config.status)),
        created_at: Some(config.created_at.timestamp() as f64),
        modified_at: Some(config.modified_at.timestamp() as f64),
    }
}

fn rule_groups_namespace_status_wire(
    status: &crate::types::RuleGroupsNamespaceStatus,
) -> wire::RuleGroupsNamespaceStatus {
    wire::RuleGroupsNamespaceStatus {
        status_code: Some(status.status_code.clone()),
        ..Default::default()
    }
}

fn rule_groups_namespace_description_wire(
    ns: &crate::types::RuleGroupsNamespace,
) -> wire::RuleGroupsNamespaceDescription {
    wire::RuleGroupsNamespaceDescription {
        name: Some(ns.name.clone()),
        arn: Some(ns.arn.clone()),
        data: Some(ns.data.clone()),
        status: Some(rule_groups_namespace_status_wire(&ns.status)),
        created_at: Some(ns.created_at.timestamp() as f64),
        modified_at: Some(ns.modified_at.timestamp() as f64),
        tags: Some(ns.tags.clone()),
    }
}

fn rule_groups_namespace_summary_wire(
    ns: &crate::types::RuleGroupsNamespace,
) -> wire::RuleGroupsNamespaceSummary {
    wire::RuleGroupsNamespaceSummary {
        name: Some(ns.name.clone()),
        arn: Some(ns.arn.clone()),
        status: Some(rule_groups_namespace_status_wire(&ns.status)),
        created_at: Some(ns.created_at.timestamp() as f64),
        modified_at: Some(ns.modified_at.timestamp() as f64),
        tags: Some(ns.tags.clone()),
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().unwrap_or(b'0');
            let lo = chars.next().unwrap_or(b'0');
            let hex = [hi, lo];
            if let Ok(s) = std::str::from_utf8(&hex)
                && let Ok(val) = u8::from_str_radix(s, 16)
            {
                result.push(val as char);
                continue;
            }
            result.push('%');
            result.push(hi as char);
            result.push(lo as char);
        } else {
            result.push(b as char);
        }
    }
    result
}

fn extract_query_param_list(query: &str, key: &str) -> Vec<String> {
    let mut results = vec![];
    if query.is_empty() {
        return results;
    }
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=')
            && k == key
        {
            results.push(percent_decode(v));
        }
    }
    results
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
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

fn workspace_status_wire(status: &crate::types::WorkspaceStatus) -> wire::WorkspaceStatus {
    wire::WorkspaceStatus {
        status_code: Some(status.status_code.clone()),
    }
}

fn workspace_description_wire(ws: &crate::types::Workspace) -> wire::WorkspaceDescription {
    wire::WorkspaceDescription {
        workspace_id: Some(ws.workspace_id.clone()),
        arn: Some(ws.arn.clone()),
        alias: ws.alias.clone(),
        status: Some(workspace_status_wire(&ws.status)),
        prometheus_endpoint: Some(ws.prometheus_endpoint.clone()),
        created_at: Some(ws.created_at.timestamp() as f64),
        tags: Some(ws.tags.clone()),
        ..Default::default()
    }
}

fn workspace_summary_wire(ws: &crate::types::WorkspaceSummary) -> wire::WorkspaceSummary {
    wire::WorkspaceSummary {
        workspace_id: Some(ws.workspace_id.clone()),
        arn: Some(ws.arn.clone()),
        alias: ws.alias.clone(),
        status: Some(workspace_status_wire(&ws.status)),
        created_at: Some(ws.created_at.timestamp() as f64),
        tags: Some(ws.tags.clone()),
        ..Default::default()
    }
}

fn amp_error_response(err: &AmpError) -> MockResponse {
    let (status, error_type) = match err {
        AmpError::WorkspaceNotFound { .. }
        | AmpError::LoggingConfigNotFound { .. }
        | AmpError::RuleGroupsNamespaceNotFound { .. }
        | AmpError::ResourceNotFound { .. } => (404u16, "ResourceNotFoundException"),
        AmpError::LoggingConfigAlreadyExists { .. }
        | AmpError::RuleGroupsNamespaceAlreadyExists { .. } => (409u16, "ConflictException"),
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
