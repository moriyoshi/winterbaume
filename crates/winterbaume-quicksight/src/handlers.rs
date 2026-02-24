use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{QuickSightError, QuickSightState};
use crate::views::QuickSightStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct QuickSightService {
    pub(crate) state: Arc<BackendState<QuickSightState>>,
    pub(crate) notifier: StateChangeNotifier<QuickSightStateView>,
}

impl QuickSightService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for QuickSightService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for QuickSightService {
    fn service_name(&self) -> &str {
        "quicksight"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://quicksight\..*\.amazonaws\.com",
            r"https?://quicksight\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl QuickSightService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query: HashMap<String, String> = extract_query(&request.uri);
        let method = request.method.as_str();
        let query_string: String = request
            .uri
            .find('?')
            .map(|i| request.uri[i + 1..].to_string())
            .unwrap_or_default();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Expected pattern: ["accounts", "{AwsAccountId}", ...]
        if segments.len() < 2 || segments[0] != "accounts" {
            // Check for tag operations: /resources/{Arn}/tags
            if segments.len() >= 2 && segments[0] == "resources" {
                let arn_encoded = segments[1..segments.len() - 1].join("/");
                let arn = urldecode(&arn_encoded);
                if segments.last() == Some(&"tags") {
                    let labels: &[(&str, &str)] = &[("ResourceArn", arn.as_str())];
                    return match method {
                        "GET" => {
                            self.handle_list_tags_for_resource(&state, &request, labels, &query)
                                .await
                        }
                        "POST" => {
                            self.handle_tag_resource(&state, &request, labels, &query)
                                .await
                        }
                        "DELETE" => {
                            self.handle_untag_resource(
                                &state,
                                &request,
                                labels,
                                &query,
                                &query_string,
                            )
                            .await
                        }
                        _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                    };
                }
            }
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let aws_account_id_label = segments[1]; // AwsAccountId from URL

        // CreateNamespace: POST /accounts/{id}
        if segments.len() == 2 && method == "POST" {
            let labels: &[(&str, &str)] = &[("AwsAccountId", aws_account_id_label)];
            return self
                .handle_create_namespace(&state, &request, labels, &query, account_id, &region)
                .await;
        }

        if segments.len() < 3 {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let resource_type = segments[2];

        match resource_type {
            "data-sets" => {
                self.dispatch_data_sets(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            "data-sources" => {
                self.dispatch_data_sources(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            "dashboards" => {
                self.dispatch_dashboards(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            "namespaces" => {
                self.dispatch_namespaces(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            "settings" => {
                self.dispatch_settings(method, &segments, &state, &request, &query)
                    .await
            }
            "public-sharing-settings" => {
                self.dispatch_public_sharing(method, &state, &request, &query)
                    .await
            }
            "analyses" => {
                self.dispatch_analyses(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            "folders" => {
                self.dispatch_folders(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            "templates" => {
                self.dispatch_templates(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            "themes" => {
                self.dispatch_themes(
                    method, &segments, &state, &request, &query, account_id, &region,
                )
                .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ---- DataSet routes ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_data_sets(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match (method, segments.len()) {
            // POST /accounts/{id}/data-sets
            ("POST", 3) => {
                let labels: &[(&str, &str)] = &[("AwsAccountId", aws_account_id_label)];
                self.handle_create_data_set(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /accounts/{id}/data-sets
            ("GET", 3) => self.handle_list_data_sets(state, account_id).await,
            // GET /accounts/{id}/data-sets/{dsId}
            ("GET", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("DataSetId", segments[3]),
                ];
                self.handle_describe_data_set(state, request, labels, query)
                    .await
            }
            // PUT /accounts/{id}/data-sets/{dsId}
            ("PUT", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("DataSetId", segments[3]),
                ];
                self.handle_update_data_set(state, request, labels, query)
                    .await
            }
            // DELETE /accounts/{id}/data-sets/{dsId}
            ("DELETE", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("DataSetId", segments[3]),
                ];
                self.handle_delete_data_set(state, request, labels, query)
                    .await
            }
            // PUT /accounts/{id}/data-sets/{dsId}/ingestions/{ingId}
            ("PUT", 6) if segments[4] == "ingestions" => {
                self.handle_create_ingestion(state, segments[3], segments[5], account_id, region)
                    .await
            }
            // GET /accounts/{id}/data-sets/{dsId}/ingestions/{ingId}
            ("GET", 6) if segments[4] == "ingestions" => {
                self.handle_describe_ingestion(state, segments[3], segments[5])
                    .await
            }
            // GET /accounts/{id}/data-sets/{dsId}/ingestions
            ("GET", 5) if segments[4] == "ingestions" => {
                self.handle_list_ingestions(state, segments[3]).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ---- DataSource routes ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_data_sources(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match (method, segments.len()) {
            // POST /accounts/{id}/data-sources
            ("POST", 3) => {
                let labels: &[(&str, &str)] = &[("AwsAccountId", aws_account_id_label)];
                self.handle_create_data_source(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /accounts/{id}/data-sources
            ("GET", 3) => self.handle_list_data_sources(state).await,
            // GET /accounts/{id}/data-sources/{dsId}
            ("GET", 4) => self.handle_describe_data_source(state, segments[3]).await,
            // PUT /accounts/{id}/data-sources/{dsId}
            ("PUT", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("DataSourceId", segments[3]),
                ];
                self.handle_update_data_source(state, request, labels, query)
                    .await
            }
            // DELETE /accounts/{id}/data-sources/{dsId}
            ("DELETE", 4) => self.handle_delete_data_source(state, segments[3]).await,
            // GET /accounts/{id}/data-sources/{dsId}/permissions
            ("GET", 5) if segments[4] == "permissions" => {
                self.handle_describe_data_source_permissions(state, segments[3])
                    .await
            }
            // POST /accounts/{id}/data-sources/{dsId}/permissions
            ("POST", 5) if segments[4] == "permissions" => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("DataSourceId", segments[3]),
                ];
                self.handle_update_data_source_permissions(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ---- Dashboard routes ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_dashboards(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match (method, segments.len()) {
            // POST /accounts/{id}/dashboards/{dashId}
            ("POST", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("DashboardId", segments[3]),
                ];
                self.handle_create_dashboard(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /accounts/{id}/dashboards
            ("GET", 3) => self.handle_list_dashboards(state).await,
            // GET /accounts/{id}/dashboards/{dashId}
            ("GET", 4) => self.handle_describe_dashboard(state, segments[3]).await,
            // PUT /accounts/{id}/dashboards/{dashId}
            ("PUT", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("DashboardId", segments[3]),
                ];
                self.handle_update_dashboard(state, request, labels, query)
                    .await
            }
            // DELETE /accounts/{id}/dashboards/{dashId}
            ("DELETE", 4) => self.handle_delete_dashboard(state, segments[3]).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ---- Namespace routes (groups, users, group-memberships) ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_namespaces(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        // /accounts/{id}/namespaces  (list namespaces)
        if segments.len() == 3 && method == "GET" {
            return self.handle_list_namespaces(state).await;
        }

        // /accounts/{id}/namespaces/{ns}/...
        if segments.len() < 4 {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }
        let namespace = segments[3];
        let aws_account_id_label = segments[1];

        // GET /accounts/{id}/namespaces/{ns}  (describe namespace)
        if segments.len() == 4 {
            return match method {
                "GET" => self.handle_describe_namespace(state, namespace).await,
                "DELETE" => self.handle_delete_namespace(state, namespace).await,
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        let sub_resource = segments[4];

        match sub_resource {
            "groups" => {
                self.dispatch_groups(
                    method,
                    segments,
                    state,
                    request,
                    query,
                    namespace,
                    aws_account_id_label,
                    account_id,
                    region,
                )
                .await
            }
            "groups-search" if method == "POST" && segments.len() == 5 => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("Namespace", namespace),
                ];
                self.handle_search_groups(state, request, labels, query, namespace)
                    .await
            }
            "users" => {
                self.dispatch_users(
                    method,
                    segments,
                    state,
                    request,
                    query,
                    namespace,
                    aws_account_id_label,
                    account_id,
                    region,
                )
                .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_groups(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        namespace: &str,
        aws_account_id_label: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            // POST /accounts/{id}/namespaces/{ns}/groups
            ("POST", 5) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("Namespace", namespace),
                ];
                self.handle_create_group(
                    state, request, labels, query, namespace, account_id, region,
                )
                .await
            }
            // GET /accounts/{id}/namespaces/{ns}/groups
            ("GET", 5) => self.handle_list_groups(state, namespace).await,
            // POST /accounts/{id}/namespaces/{ns}/groups/search  (SearchGroups)
            ("POST", 6) if segments[5] == "search" => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("Namespace", namespace),
                ];
                self.handle_search_groups(state, request, labels, query, namespace)
                    .await
            }
            // GET /accounts/{id}/namespaces/{ns}/groups/{groupName}
            ("GET", 6) => {
                self.handle_describe_group(state, namespace, segments[5])
                    .await
            }
            // PUT /accounts/{id}/namespaces/{ns}/groups/{groupName}
            ("PUT", 6) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("Namespace", namespace),
                    ("GroupName", segments[5]),
                ];
                self.handle_update_group(state, request, labels, query, namespace, segments[5])
                    .await
            }
            // DELETE /accounts/{id}/namespaces/{ns}/groups/{groupName}
            ("DELETE", 6) => {
                self.handle_delete_group(state, namespace, segments[5])
                    .await
            }
            // PUT /accounts/{id}/namespaces/{ns}/groups/{groupName}/members/{memberName}
            ("PUT", 8) if segments[6] == "members" => {
                self.handle_create_group_membership(
                    state,
                    namespace,
                    segments[5],
                    segments[7],
                    account_id,
                    region,
                )
                .await
            }
            // DELETE /accounts/{id}/namespaces/{ns}/groups/{groupName}/members/{memberName}
            ("DELETE", 8) if segments[6] == "members" => {
                self.handle_delete_group_membership(state, namespace, segments[5], segments[7])
                    .await
            }
            // GET /accounts/{id}/namespaces/{ns}/groups/{groupName}/members
            ("GET", 7) if segments[6] == "members" => {
                self.handle_list_group_memberships(
                    state,
                    namespace,
                    segments[5],
                    account_id,
                    region,
                )
                .await
            }
            // GET /accounts/{id}/namespaces/{ns}/groups/{groupName}/members/{memberName}
            ("GET", 8) if segments[6] == "members" => {
                self.handle_describe_group_membership(
                    state,
                    namespace,
                    segments[5],
                    segments[7],
                    account_id,
                    region,
                )
                .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_users(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        namespace: &str,
        aws_account_id_label: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            // POST /accounts/{id}/namespaces/{ns}/users
            ("POST", 5) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("Namespace", namespace),
                ];
                self.handle_register_user(
                    state, request, labels, query, namespace, account_id, region,
                )
                .await
            }
            // GET /accounts/{id}/namespaces/{ns}/users
            ("GET", 5) => self.handle_list_users(state, namespace).await,
            // GET /accounts/{id}/namespaces/{ns}/users/{userName}
            ("GET", 6) => {
                self.handle_describe_user(state, namespace, segments[5])
                    .await
            }
            // PUT /accounts/{id}/namespaces/{ns}/users/{userName}
            ("PUT", 6) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("Namespace", namespace),
                    ("UserName", segments[5]),
                ];
                self.handle_update_user(state, request, labels, query, namespace, segments[5])
                    .await
            }
            // DELETE /accounts/{id}/namespaces/{ns}/users/{userName}
            ("DELETE", 6) => self.handle_delete_user(state, namespace, segments[5]).await,
            // GET /accounts/{id}/namespaces/{ns}/users/{userName}/groups
            ("GET", 7) if segments[6] == "groups" => {
                self.handle_list_user_groups(state, namespace, segments[5], account_id, region)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ---- Settings routes ----

    async fn dispatch_settings(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match method {
            "GET" if segments.len() == 3 => self.handle_describe_account_settings(state).await,
            "PUT" if segments.len() == 3 => {
                let labels: &[(&str, &str)] = &[("AwsAccountId", aws_account_id_label)];
                self.handle_update_account_settings(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn dispatch_public_sharing(
        &self,
        method: &str,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        match method {
            "PUT" => {
                self.handle_update_public_sharing_settings(state, request, &[], query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ==== Handler implementations ====

    // ---- DataSet handlers ----

    async fn handle_create_data_set(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_data_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.data_set_id.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'DataSetId'");
        }
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'Name'");
        }
        let import_mode = if input.import_mode.is_empty() {
            "DIRECT_QUERY".to_string()
        } else {
            input.import_mode.clone()
        };

        let physical_table_map: HashMap<String, Value> = input
            .physical_table_map
            .into_iter()
            .filter_map(|(k, v)| serde_json::to_value(&v).ok().map(|val| (k, val)))
            .collect();

        let mut state = state.write().await;
        match state.create_data_set(
            &input.data_set_id,
            &input.name,
            &import_mode,
            physical_table_map,
            account_id,
            region,
        ) {
            Ok(ds) => {
                let wire_resp = wire::CreateDataSetResponse {
                    arn: Some(ds.arn.clone()),
                    data_set_id: Some(ds.data_set_id.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                let mut resp = wire::serialize_create_data_set_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_data_set(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_data_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let state = state.read().await;
        match state.describe_data_set(&input.data_set_id) {
            Ok(ds) => {
                let wire_resp = wire::DescribeDataSetResponse {
                    data_set: Some(data_set_to_wire(ds)),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_data_set_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_data_set(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_data_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = if input.name.is_empty() {
            None
        } else {
            Some(input.name.as_str())
        };
        let import_mode = if input.import_mode.is_empty() {
            None
        } else {
            Some(input.import_mode.as_str())
        };
        let physical_table_map = if input.physical_table_map.is_empty() {
            None
        } else {
            Some(
                input
                    .physical_table_map
                    .into_iter()
                    .filter_map(|(k, v)| serde_json::to_value(&v).ok().map(|val| (k, val)))
                    .collect::<HashMap<String, Value>>(),
            )
        };

        let mut state = state.write().await;
        match state.update_data_set(&input.data_set_id, name, import_mode, physical_table_map) {
            Ok(ds) => {
                let wire_resp = wire::UpdateDataSetResponse {
                    arn: Some(ds.arn.clone()),
                    data_set_id: Some(ds.data_set_id.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                wire::serialize_update_data_set_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_data_set(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_data_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let mut state = state.write().await;
        match state.delete_data_set(&input.data_set_id) {
            Ok(arn) => {
                let wire_resp = wire::DeleteDataSetResponse {
                    arn: Some(arn),
                    data_set_id: Some(input.data_set_id.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_data_set_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_data_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        _account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let data_sets = state.list_data_sets();
        let summaries: Vec<wire::DataSetSummary> =
            data_sets.iter().map(|ds| data_set_to_summary(ds)).collect();

        let wire_resp = wire::ListDataSetsResponse {
            data_set_summaries: Some(summaries),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_data_sets_response(&wire_resp)
    }

    // ---- DataSource handlers ----

    async fn handle_create_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_data_source_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.data_source_id.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'DataSourceId'",
            );
        }
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'Name'");
        }
        let ds_type = if input.r#type.is_empty() {
            "MANUAL".to_string()
        } else {
            input.r#type.clone()
        };

        let initial_perms = convert_resource_permissions(input.permissions);

        let mut state = state.write().await;
        match state.create_data_source(
            &input.data_source_id,
            &input.name,
            &ds_type,
            account_id,
            region,
        ) {
            Ok(ds) => {
                let arn = ds.arn.clone();
                let id = ds.data_source_id.clone();
                if !initial_perms.is_empty() {
                    let _ = state.update_data_source_permissions(
                        &input.data_source_id,
                        initial_perms,
                        Vec::new(),
                    );
                }
                let wire_resp = wire::CreateDataSourceResponse {
                    arn: Some(arn),
                    creation_status: Some("CREATION_SUCCESSFUL".to_string()),
                    data_source_id: Some(id),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                let mut resp = wire::serialize_create_data_source_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        data_source_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_data_source(data_source_id) {
            Ok(ds) => {
                let wire_resp = wire::DescribeDataSourceResponse {
                    data_source: Some(wire::DataSource {
                        arn: Some(ds.arn.clone()),
                        data_source_id: Some(ds.data_source_id.clone()),
                        name: Some(ds.name.clone()),
                        r#type: Some(ds.r#type.clone()),
                        status: Some(ds.status.clone()),
                        created_time: Some(ds.created_time.timestamp() as f64),
                        last_updated_time: Some(ds.last_updated_time.timestamp() as f64),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_data_source_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_data_source_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = if input.name.is_empty() {
            None
        } else {
            Some(input.name.as_str())
        };

        let mut state = state.write().await;
        match state.update_data_source(&input.data_source_id, name) {
            Ok(ds) => {
                let wire_resp = wire::UpdateDataSourceResponse {
                    arn: Some(ds.arn.clone()),
                    data_source_id: Some(ds.data_source_id.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    update_status: Some("UPDATE_SUCCESSFUL".to_string()),
                    ..Default::default()
                };
                wire::serialize_update_data_source_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        data_source_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_data_source(data_source_id) {
            Ok(arn) => {
                let wire_resp = wire::DeleteDataSourceResponse {
                    arn: Some(arn),
                    data_source_id: Some(data_source_id.to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_data_source_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_data_sources(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let sources = state.list_data_sources();
        let wire_sources: Vec<wire::DataSource> = sources
            .iter()
            .map(|ds| wire::DataSource {
                arn: Some(ds.arn.clone()),
                data_source_id: Some(ds.data_source_id.clone()),
                name: Some(ds.name.clone()),
                r#type: Some(ds.r#type.clone()),
                status: Some(ds.status.clone()),
                created_time: Some(ds.created_time.timestamp() as f64),
                last_updated_time: Some(ds.last_updated_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        let wire_resp = wire::ListDataSourcesResponse {
            data_sources: Some(wire_sources),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_data_sources_response(&wire_resp)
    }

    async fn handle_describe_data_source_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        data_source_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_data_source_permissions(data_source_id) {
            Ok((arn, perms)) => {
                let wire_perms = perms
                    .into_iter()
                    .map(|p| wire::ResourcePermission {
                        principal: p.principal,
                        actions: p.actions,
                    })
                    .collect();
                let wire_resp = wire::DescribeDataSourcePermissionsResponse {
                    data_source_arn: Some(arn),
                    data_source_id: Some(data_source_id.to_string()),
                    permissions: Some(wire_perms),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_data_source_permissions_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_data_source_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_data_source_permissions_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let grants = convert_resource_permissions(input.grant_permissions);
        let revokes = convert_resource_permissions(input.revoke_permissions);

        let mut state = state.write().await;
        match state.update_data_source_permissions(&input.data_source_id, grants, revokes) {
            Ok((arn, _perms)) => {
                let wire_resp = wire::UpdateDataSourcePermissionsResponse {
                    data_source_arn: Some(arn),
                    data_source_id: Some(input.data_source_id.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_update_data_source_permissions_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    // ---- Dashboard handlers ----

    async fn handle_create_dashboard(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dashboard_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.create_dashboard(&input.dashboard_id, &input.name, account_id, region) {
            Ok(db) => {
                let wire_resp = wire::CreateDashboardResponse {
                    arn: Some(db.arn.clone()),
                    creation_status: Some("CREATION_SUCCESSFUL".to_string()),
                    dashboard_id: Some(db.dashboard_id.clone()),
                    version_arn: Some(db.version_arn.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                let mut resp = wire::serialize_create_dashboard_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_dashboard(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        dashboard_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_dashboard(dashboard_id) {
            Ok(db) => {
                let wire_resp = wire::DescribeDashboardResponse {
                    dashboard: Some(wire::Dashboard {
                        arn: Some(db.arn.clone()),
                        dashboard_id: Some(db.dashboard_id.clone()),
                        name: Some(db.name.clone()),
                        created_time: Some(db.created_time.timestamp() as f64),
                        last_updated_time: Some(db.last_updated_time.timestamp() as f64),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_dashboard_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_dashboards(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let dashboards = state.list_dashboards();
        let summaries: Vec<wire::DashboardSummary> = dashboards
            .iter()
            .map(|db| wire::DashboardSummary {
                arn: Some(db.arn.clone()),
                dashboard_id: Some(db.dashboard_id.clone()),
                name: Some(db.name.clone()),
                created_time: Some(db.created_time.timestamp() as f64),
                last_updated_time: Some(db.last_updated_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        let wire_resp = wire::ListDashboardsResponse {
            dashboard_summary_list: Some(summaries),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_dashboards_response(&wire_resp)
    }

    // ---- Group handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        namespace: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.group_name.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'GroupName'");
        }
        let description = input.description.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.create_group(
            namespace,
            &input.group_name,
            description,
            account_id,
            region,
        ) {
            Ok(group) => {
                let wire_resp = wire::CreateGroupResponse {
                    group: Some(group_to_wire(&group)),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                let mut resp = wire::serialize_create_group_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_group(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        group_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_group(namespace, group_name) {
            Ok(group) => {
                let wire_resp = wire::DescribeGroupResponse {
                    group: Some(group_to_wire(group)),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_group_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_group(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        namespace: &str,
        group_name: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_group(namespace, group_name, description) {
            Ok(group) => {
                let wire_resp = wire::UpdateGroupResponse {
                    group: Some(group_to_wire(&group)),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_update_group_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        group_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_group(namespace, group_name) {
            Ok(()) => {
                let wire_resp = wire::DeleteGroupResponse {
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_group_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let groups = state.list_groups(namespace);
        let wire_groups: Vec<wire::Group> = groups.iter().map(|g| group_to_wire(g)).collect();

        let wire_resp = wire::ListGroupsResponse {
            group_list: Some(wire_groups),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_groups_response(&wire_resp)
    }

    async fn handle_search_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        namespace: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_search_groups_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        // Validate filter names - only GROUP_NAME is allowed
        for f in &input.filters {
            if f.name != "GROUP_NAME" {
                return rest_json_error(
                    400,
                    "ValidationException",
                    &format!(
                        "1 validation error detected: Value '{}' at 'filters.1.member.name' failed to satisfy constraint: Member must satisfy enum value set: [GROUP_NAME]",
                        f.name
                    ),
                );
            }
        }

        let filters: Vec<(String, String, String)> = input
            .filters
            .iter()
            .map(|f| (f.name.clone(), f.operator.clone(), f.value.clone()))
            .collect();

        let state = state.read().await;
        let groups = state.search_groups(namespace, &filters);
        let wire_groups: Vec<wire::Group> = groups.iter().map(|g| group_to_wire(g)).collect();

        let wire_resp = wire::SearchGroupsResponse {
            group_list: Some(wire_groups),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_search_groups_response(&wire_resp)
    }

    // ---- Group Membership handlers ----

    async fn handle_create_group_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        group_name: &str,
        member_name: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_group_membership(namespace, group_name, member_name, account_id, region)
        {
            Ok((arn, name)) => {
                let wire_resp = wire::CreateGroupMembershipResponse {
                    group_member: Some(wire::GroupMember {
                        arn: Some(arn),
                        member_name: Some(name),
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                wire::serialize_create_group_membership_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_group_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        group_name: &str,
        member_name: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_group_membership(
            namespace,
            group_name,
            member_name,
            account_id,
            region,
        ) {
            Ok((arn, name)) => {
                let wire_resp = wire::DescribeGroupMembershipResponse {
                    group_member: Some(wire::GroupMember {
                        arn: Some(arn),
                        member_name: Some(name),
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_group_membership_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_group_memberships(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        group_name: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_group_memberships(namespace, group_name, account_id, region) {
            Ok(members) => {
                let wire_members: Vec<wire::GroupMember> = members
                    .iter()
                    .map(|(arn, name)| wire::GroupMember {
                        arn: Some(arn.clone()),
                        member_name: Some(name.clone()),
                    })
                    .collect();

                let wire_resp = wire::ListGroupMembershipsResponse {
                    group_member_list: Some(wire_members),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                    ..Default::default()
                };
                wire::serialize_list_group_memberships_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    // ---- User handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn handle_register_user(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        namespace: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_user_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.email.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'Email'");
        }
        if input.identity_type.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'IdentityType'",
            );
        }
        if input.user_role.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'UserRole'");
        }
        let user_name = input.user_name.as_deref();

        let mut state = state.write().await;
        match state.register_user(
            namespace,
            &input.email,
            &input.identity_type,
            &input.user_role,
            user_name,
            account_id,
            region,
        ) {
            Ok(user) => {
                let wire_resp = wire::RegisterUserResponse {
                    user: Some(user_to_wire(&user)),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                let mut resp = wire::serialize_register_user_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_user(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        user_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_user(namespace, user_name) {
            Ok(user) => {
                let wire_resp = wire::DescribeUserResponse {
                    user: Some(user_to_wire(user)),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_user_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_user(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        namespace: &str,
        user_name: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let email = if input.email.is_empty() {
            None
        } else {
            Some(input.email.as_str())
        };
        let role = if input.role.is_empty() {
            None
        } else {
            Some(input.role.as_str())
        };

        let mut state = state.write().await;
        match state.update_user(namespace, user_name, email, role) {
            Ok(user) => {
                let wire_resp = wire::UpdateUserResponse {
                    user: Some(user_to_wire(&user)),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_update_user_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        user_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_user(namespace, user_name) {
            Ok(()) => {
                let wire_resp = wire::DeleteUserResponse {
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_user_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_users(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let users = state.list_users(namespace);
        let wire_users: Vec<wire::User> = users.iter().map(|u| user_to_wire(u)).collect();

        let wire_resp = wire::ListUsersResponse {
            user_list: Some(wire_users),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_users_response(&wire_resp)
    }

    async fn handle_list_user_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        user_name: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_user_groups(namespace, user_name, account_id, region) {
            Ok(groups) => {
                let wire_groups: Vec<wire::Group> = groups.iter().map(group_to_wire).collect();

                let wire_resp = wire::ListUserGroupsResponse {
                    group_list: Some(wire_groups),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                    ..Default::default()
                };
                wire::serialize_list_user_groups_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    // ---- Ingestion handler ----

    async fn handle_create_ingestion(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        data_set_id: &str,
        ingestion_id: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_ingestion(data_set_id, ingestion_id, account_id, region) {
            Ok(ing) => {
                let wire_resp = wire::CreateIngestionResponse {
                    arn: Some(ing.arn.clone()),
                    ingestion_id: Some(ing.ingestion_id.clone()),
                    ingestion_status: Some(ing.ingestion_status.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    ..Default::default()
                };
                let mut resp = wire::serialize_create_ingestion_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    // ---- Account Settings handlers ----

    async fn handle_describe_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let settings = state.describe_account_settings();

        let wire_resp = wire::DescribeAccountSettingsResponse {
            account_settings: Some(wire::AccountSettings {
                account_name: Some(settings.account_name.clone()),
                default_namespace: Some(settings.default_namespace.clone()),
                edition: Some(settings.edition.clone()),
                notification_email: Some(settings.notification_email.clone()),
                public_sharing_enabled: Some(settings.public_sharing_enabled),
                termination_protection_enabled: Some(settings.termination_protection_enabled),
            }),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
        };
        wire::serialize_describe_account_settings_response(&wire_resp)
    }

    async fn handle_update_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_account_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let default_namespace = if input.default_namespace.is_empty() {
            None
        } else {
            Some(input.default_namespace.as_str())
        };
        let notification_email = input.notification_email.as_deref();
        let termination_protection = input.termination_protection_enabled;

        let mut state = state.write().await;
        state.update_account_settings(
            default_namespace,
            notification_email,
            termination_protection,
        );

        let wire_resp = wire::UpdateAccountSettingsResponse {
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
        };
        wire::serialize_update_account_settings_response(&wire_resp)
    }

    async fn handle_update_public_sharing_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_public_sharing_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };

        let mut state = state.write().await;
        state.update_public_sharing_settings(input.public_sharing_enabled);

        let wire_resp = wire::UpdatePublicSharingSettingsResponse {
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
        };
        wire::serialize_update_public_sharing_settings_response(&wire_resp)
    }

    // ---- Tag handlers ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let tags: Vec<(String, String)> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();

        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, &tags);

        let wire_resp = wire::TagResourceResponse {
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
        };
        wire::serialize_tag_resource_response(&wire_resp)
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        query_string: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        // AWS SDKs send tagKeys as `?keys=k1&keys=k2`; the wire deserializer
        // splits a single comma-separated value, so re-parse for repeated keys.
        let mut tag_keys: Vec<String> = query_string
            .split('&')
            .filter_map(|part| part.strip_prefix("keys="))
            .map(urldecode)
            .collect();
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys;
        }

        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &tag_keys);

        let wire_resp = wire::UntagResourceResponse {
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
        };
        wire::serialize_untag_resource_response(&wire_resp)
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let resource_arn = input.resource_arn.as_str();
        let state = state.read().await;
        let tags = state.list_tags_for_resource(resource_arn);

        let wire_tags: Vec<wire::Tag> = tags
            .iter()
            .map(|(k, v)| wire::Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();

        let wire_resp = wire::ListTagsForResourceResponse {
            tags: Some(wire_tags),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
        };
        wire::serialize_list_tags_for_resource_response(&wire_resp)
    }

    // ---- Analysis dispatch and handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_analyses(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match (method, segments.len()) {
            // GET /accounts/{id}/analyses
            ("GET", 3) => self.handle_list_analyses(state).await,
            // POST /accounts/{id}/analyses/{analysisId}
            ("POST", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("AnalysisId", segments[3]),
                ];
                self.handle_create_analysis(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /accounts/{id}/analyses/{analysisId}
            ("GET", 4) => self.handle_describe_analysis(state, segments[3]).await,
            // PUT /accounts/{id}/analyses/{analysisId}
            ("PUT", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("AnalysisId", segments[3]),
                ];
                self.handle_update_analysis(state, request, labels, query)
                    .await
            }
            // DELETE /accounts/{id}/analyses/{analysisId}
            ("DELETE", 4) => self.handle_delete_analysis(state, segments[3]).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_analysis(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_analysis_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.create_analysis(&input.analysis_id, &input.name, account_id, region) {
            Ok(a) => {
                let wire_resp = wire::CreateAnalysisResponse {
                    analysis_id: Some(a.analysis_id.clone()),
                    arn: Some(a.arn.clone()),
                    creation_status: Some("CREATION_SUCCESSFUL".to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(201),
                };
                let mut resp = wire::serialize_create_analysis_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_analysis(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        analysis_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_analysis(analysis_id) {
            Ok(a) => {
                let wire_resp = wire::DescribeAnalysisResponse {
                    analysis: Some(wire::Analysis {
                        analysis_id: Some(a.analysis_id.clone()),
                        arn: Some(a.arn.clone()),
                        name: Some(a.name.clone()),
                        status: Some(a.status.clone()),
                        created_time: Some(a.created_time.timestamp() as f64),
                        last_updated_time: Some(a.last_updated_time.timestamp() as f64),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_analysis_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_analysis(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_analysis_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = if input.name.is_empty() {
            None
        } else {
            Some(input.name.as_str())
        };

        let mut state = state.write().await;
        match state.update_analysis(&input.analysis_id, name) {
            Ok(a) => {
                let wire_resp = wire::UpdateAnalysisResponse {
                    analysis_id: Some(a.analysis_id.clone()),
                    arn: Some(a.arn.clone()),
                    update_status: Some("UPDATE_SUCCESSFUL".to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_update_analysis_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_analysis(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        analysis_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_analysis(analysis_id) {
            Ok((arn, id)) => {
                let wire_resp = wire::DeleteAnalysisResponse {
                    analysis_id: Some(id),
                    arn: Some(arn),
                    deletion_time: Some(chrono::Utc::now().timestamp() as f64),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_analysis_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_analyses(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let analyses = state.list_analyses();
        let summaries: Vec<wire::AnalysisSummary> = analyses
            .iter()
            .map(|a| wire::AnalysisSummary {
                analysis_id: Some(a.analysis_id.clone()),
                arn: Some(a.arn.clone()),
                name: Some(a.name.clone()),
                status: Some(a.status.clone()),
                created_time: Some(a.created_time.timestamp() as f64),
                last_updated_time: Some(a.last_updated_time.timestamp() as f64),
            })
            .collect();

        let wire_resp = wire::ListAnalysesResponse {
            analysis_summary_list: Some(summaries),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_analyses_response(&wire_resp)
    }

    // ---- Dashboard (delete, update) handlers ----

    async fn handle_delete_dashboard(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        dashboard_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_dashboard(dashboard_id) {
            Ok((arn, id)) => {
                let wire_resp = wire::DeleteDashboardResponse {
                    arn: Some(arn),
                    dashboard_id: Some(id),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_dashboard_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_dashboard(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_dashboard_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = if input.name.is_empty() {
            None
        } else {
            Some(input.name.as_str())
        };

        let mut state = state.write().await;
        match state.update_dashboard(&input.dashboard_id, name) {
            Ok(db) => {
                let wire_resp = wire::UpdateDashboardResponse {
                    arn: Some(db.arn.clone()),
                    dashboard_id: Some(db.dashboard_id.clone()),
                    version_arn: Some(db.version_arn.clone()),
                    creation_status: Some("UPDATE_SUCCESSFUL".to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_update_dashboard_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    // ---- Group Membership (delete) handler ----

    async fn handle_delete_group_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
        group_name: &str,
        member_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_group_membership(namespace, group_name, member_name) {
            Ok(()) => {
                let wire_resp = wire::DeleteGroupMembershipResponse {
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_group_membership_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    // ---- Folder dispatch and handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_folders(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match (method, segments.len()) {
            // GET /accounts/{id}/folders
            ("GET", 3) => self.handle_list_folders(state).await,
            // POST /accounts/{id}/folders/{folderId}
            ("POST", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("FolderId", segments[3]),
                ];
                self.handle_create_folder(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /accounts/{id}/folders/{folderId}
            ("GET", 4) => self.handle_describe_folder(state, segments[3]).await,
            // DELETE /accounts/{id}/folders/{folderId}
            ("DELETE", 4) => self.handle_delete_folder(state, segments[3]).await,
            // PUT /accounts/{id}/folders/{folderId}/members/{memberType}/{memberId}
            ("PUT", 7) if segments[4] == "members" => {
                self.handle_create_folder_membership(state, segments[3], segments[6], segments[5])
                    .await
            }
            // DELETE /accounts/{id}/folders/{folderId}/members/{memberType}/{memberId}
            ("DELETE", 7) if segments[4] == "members" => {
                self.handle_delete_folder_membership(state, segments[3], segments[6], segments[5])
                    .await
            }
            // GET /accounts/{id}/folders/{folderId}/members
            ("GET", 5) if segments[4] == "members" => {
                self.handle_list_folder_members(state, segments[3]).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_folder(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_folder_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = match input.name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return rest_json_error(400, "InvalidParameterValueException", "Missing 'Name'");
            }
        };
        let folder_type = input.folder_type.as_deref().unwrap_or("SHARED");

        let mut state = state.write().await;
        match state.create_folder(&input.folder_id, name, folder_type, account_id, region) {
            Ok(f) => {
                let wire_resp = wire::CreateFolderResponse {
                    arn: Some(f.arn.clone()),
                    folder_id: Some(f.folder_id.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(201),
                };
                let mut resp = wire::serialize_create_folder_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_folder(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        folder_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_folder(folder_id) {
            Ok(f) => {
                let wire_resp = wire::DescribeFolderResponse {
                    folder: Some(wire::Folder {
                        folder_id: Some(f.folder_id.clone()),
                        arn: Some(f.arn.clone()),
                        name: Some(f.name.clone()),
                        folder_type: Some(f.folder_type.clone()),
                        created_time: Some(f.created_time.timestamp() as f64),
                        last_updated_time: Some(f.last_updated_time.timestamp() as f64),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_folder_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_folder(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        folder_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_folder(folder_id) {
            Ok((arn, id)) => {
                let wire_resp = wire::DeleteFolderResponse {
                    arn: Some(arn),
                    folder_id: Some(id),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_folder_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_folders(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let folders = state.list_folders();
        let summaries: Vec<wire::FolderSummary> = folders
            .iter()
            .map(|f| wire::FolderSummary {
                arn: Some(f.arn.clone()),
                folder_id: Some(f.folder_id.clone()),
                name: Some(f.name.clone()),
                folder_type: Some(f.folder_type.clone()),
                created_time: Some(f.created_time.timestamp() as f64),
                last_updated_time: Some(f.last_updated_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        let wire_resp = wire::ListFoldersResponse {
            folder_summary_list: Some(summaries),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_folders_response(&wire_resp)
    }

    async fn handle_create_folder_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        folder_id: &str,
        member_id: &str,
        member_type: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_folder_membership(folder_id, member_id, member_type) {
            Ok(entry) => {
                let wire_resp = wire::CreateFolderMembershipResponse {
                    folder_member: Some(wire::FolderMember {
                        member_id: Some(entry.member_id.clone()),
                        member_type: Some(entry.member_type.clone()),
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_create_folder_membership_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_folder_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        folder_id: &str,
        member_id: &str,
        member_type: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_folder_membership(folder_id, member_id, member_type) {
            Ok(()) => {
                let wire_resp = wire::DeleteFolderMembershipResponse {
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_folder_membership_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_folder_members(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        folder_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_folder_members(folder_id) {
            Ok(members) => {
                let wire_members: Vec<wire::MemberIdArnPair> = members
                    .iter()
                    .map(|m| wire::MemberIdArnPair {
                        member_id: Some(m.member_id.clone()),
                        ..Default::default()
                    })
                    .collect();

                let wire_resp = wire::ListFolderMembersResponse {
                    folder_member_list: Some(wire_members),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                    ..Default::default()
                };
                wire::serialize_list_folder_members_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    // ---- Namespace handlers ----

    async fn handle_create_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_namespace_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.namespace.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'Namespace'");
        }
        let identity_store = if input.identity_store.is_empty() {
            "QUICKSIGHT"
        } else {
            input.identity_store.as_str()
        };

        let mut state = state.write().await;
        match state.create_namespace(&input.namespace, identity_store, account_id, region) {
            Ok(ns) => {
                let wire_resp = wire::CreateNamespaceResponse {
                    arn: Some(ns.arn.clone()),
                    name: Some(ns.name.clone()),
                    capacity_region: Some(ns.capacity_region.clone()),
                    creation_status: Some(ns.creation_status.clone()),
                    identity_store: Some(ns.identity_store.clone()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(201),
                };
                let mut resp = wire::serialize_create_namespace_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_namespace(namespace) {
            Ok(ns) => {
                let wire_resp = wire::DescribeNamespaceResponse {
                    namespace: Some(wire::NamespaceInfoV2 {
                        arn: Some(ns.arn.clone()),
                        name: Some(ns.name.clone()),
                        capacity_region: Some(ns.capacity_region.clone()),
                        creation_status: Some(ns.creation_status.clone()),
                        identity_store: Some(ns.identity_store.clone()),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_namespace_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        namespace: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_namespace(namespace) {
            Ok(()) => {
                let wire_resp = wire::DeleteNamespaceResponse {
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_namespace_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_namespaces(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let namespaces = state.list_namespaces();
        let wire_ns: Vec<wire::NamespaceInfoV2> = namespaces
            .iter()
            .map(|ns| wire::NamespaceInfoV2 {
                arn: Some(ns.arn.clone()),
                name: Some(ns.name.clone()),
                capacity_region: Some(ns.capacity_region.clone()),
                creation_status: Some(ns.creation_status.clone()),
                identity_store: Some(ns.identity_store.clone()),
                ..Default::default()
            })
            .collect();

        let wire_resp = wire::ListNamespacesResponse {
            namespaces: Some(wire_ns),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_namespaces_response(&wire_resp)
    }

    // ---- Template dispatch and handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_templates(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match (method, segments.len()) {
            // GET /accounts/{id}/templates
            ("GET", 3) => self.handle_list_templates(state).await,
            // POST /accounts/{id}/templates/{templateId}
            ("POST", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("TemplateId", segments[3]),
                ];
                self.handle_create_template(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /accounts/{id}/templates/{templateId}
            ("GET", 4) => self.handle_describe_template(state, segments[3]).await,
            // PUT /accounts/{id}/templates/{templateId}
            ("PUT", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("TemplateId", segments[3]),
                ];
                self.handle_update_template(state, request, labels, query)
                    .await
            }
            // DELETE /accounts/{id}/templates/{templateId}
            ("DELETE", 4) => self.handle_delete_template(state, segments[3]).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_template(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = match input.name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return rest_json_error(400, "InvalidParameterValueException", "Missing 'Name'");
            }
        };

        let mut state = state.write().await;
        match state.create_template(&input.template_id, name, account_id, region) {
            Ok(t) => {
                let wire_resp = wire::CreateTemplateResponse {
                    arn: Some(t.arn.clone()),
                    template_id: Some(t.template_id.clone()),
                    version_arn: Some(t.version_arn.clone()),
                    creation_status: Some("CREATION_SUCCESSFUL".to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(201),
                };
                let mut resp = wire::serialize_create_template_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_template(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        template_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_template(template_id) {
            Ok(t) => {
                let wire_resp = wire::DescribeTemplateResponse {
                    template: Some(wire::Template {
                        arn: Some(t.arn.clone()),
                        template_id: Some(t.template_id.clone()),
                        name: Some(t.name.clone()),
                        created_time: Some(t.created_time.timestamp() as f64),
                        last_updated_time: Some(t.last_updated_time.timestamp() as f64),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_template_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_template(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = input.name.as_deref().filter(|s| !s.is_empty());

        let mut state = state.write().await;
        match state.update_template(&input.template_id, name) {
            Ok(t) => {
                let wire_resp = wire::UpdateTemplateResponse {
                    arn: Some(t.arn.clone()),
                    template_id: Some(t.template_id.clone()),
                    version_arn: Some(t.version_arn.clone()),
                    creation_status: Some("UPDATE_SUCCESSFUL".to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_update_template_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_template(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        template_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_template(template_id) {
            Ok((arn, id)) => {
                let wire_resp = wire::DeleteTemplateResponse {
                    arn: Some(arn),
                    template_id: Some(id),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_delete_template_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let templates = state.list_templates();
        let summaries: Vec<wire::TemplateSummary> = templates
            .iter()
            .map(|t| wire::TemplateSummary {
                arn: Some(t.arn.clone()),
                template_id: Some(t.template_id.clone()),
                name: Some(t.name.clone()),
                created_time: Some(t.created_time.timestamp() as f64),
                last_updated_time: Some(t.last_updated_time.timestamp() as f64),
                latest_version_number: Some(t.version_number),
            })
            .collect();

        let wire_resp = wire::ListTemplatesResponse {
            template_summary_list: Some(summaries),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_templates_response(&wire_resp)
    }

    // ---- Theme dispatch and handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_themes(
        &self,
        method: &str,
        segments: &[&str],
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let aws_account_id_label = segments[1];
        match (method, segments.len()) {
            // GET /accounts/{id}/themes
            ("GET", 3) => self.handle_list_themes(state).await,
            // POST /accounts/{id}/themes/{themeId}
            ("POST", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("ThemeId", segments[3]),
                ];
                self.handle_create_theme(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /accounts/{id}/themes/{themeId}
            ("GET", 4) => self.handle_describe_theme(state, segments[3]).await,
            // PUT /accounts/{id}/themes/{themeId}
            ("PUT", 4) => {
                let labels: &[(&str, &str)] = &[
                    ("AwsAccountId", aws_account_id_label),
                    ("ThemeId", segments[3]),
                ];
                self.handle_update_theme(state, request, labels, query)
                    .await
            }
            // DELETE /accounts/{id}/themes/{themeId}
            ("DELETE", 4) => self.handle_delete_theme(state, segments[3]).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_theme(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_theme_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.create_theme(&input.theme_id, &input.name, account_id, region) {
            Ok(t) => {
                let wire_resp = wire::CreateThemeResponse {
                    arn: Some(t.arn.clone()),
                    theme_id: Some(t.theme_id.clone()),
                    version_arn: Some(t.version_arn.clone()),
                    creation_status: Some("CREATION_SUCCESSFUL".to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(201),
                };
                let mut resp = wire::serialize_create_theme_response(&wire_resp);
                resp.status = 201;
                resp
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_describe_theme(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        theme_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_theme(theme_id) {
            Ok(t) => {
                let wire_resp = wire::DescribeThemeResponse {
                    theme: Some(wire::Theme {
                        arn: Some(t.arn.clone()),
                        theme_id: Some(t.theme_id.clone()),
                        name: Some(t.name.clone()),
                        created_time: Some(t.created_time.timestamp() as f64),
                        last_updated_time: Some(t.last_updated_time.timestamp() as f64),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_theme_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_update_theme(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_theme_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let name = input.name.as_deref().filter(|s| !s.is_empty());

        let mut state = state.write().await;
        match state.update_theme(&input.theme_id, name) {
            Ok(t) => {
                let wire_resp = wire::UpdateThemeResponse {
                    arn: Some(t.arn.clone()),
                    theme_id: Some(t.theme_id.clone()),
                    version_arn: Some(t.version_arn.clone()),
                    creation_status: Some("UPDATE_SUCCESSFUL".to_string()),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_update_theme_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_delete_theme(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        theme_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_theme(theme_id) {
            Ok((arn, id)) => {
                let wire_resp = wire::DeleteThemeResponse {
                    arn: Some(arn),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                    ..Default::default()
                };
                let _ = id;
                wire::serialize_delete_theme_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_themes(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let themes = state.list_themes();
        let summaries: Vec<wire::ThemeSummary> = themes
            .iter()
            .map(|t| wire::ThemeSummary {
                arn: Some(t.arn.clone()),
                theme_id: Some(t.theme_id.clone()),
                name: Some(t.name.clone()),
                created_time: Some(t.created_time.timestamp() as f64),
                last_updated_time: Some(t.last_updated_time.timestamp() as f64),
                latest_version_number: Some(t.version_number),
            })
            .collect();

        let wire_resp = wire::ListThemesResponse {
            theme_summary_list: Some(summaries),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_themes_response(&wire_resp)
    }

    // ---- Ingestion (describe, list) handlers ----

    async fn handle_describe_ingestion(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        data_set_id: &str,
        ingestion_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_ingestion(data_set_id, ingestion_id) {
            Ok(ing) => {
                let wire_resp = wire::DescribeIngestionResponse {
                    ingestion: Some(wire::Ingestion {
                        arn: Some(ing.arn.clone()),
                        ingestion_id: Some(ing.ingestion_id.clone()),
                        ingestion_status: Some(ing.ingestion_status.clone()),
                        ..Default::default()
                    }),
                    request_id: Some("mock-request-id".to_string()),
                    status: Some(200),
                };
                wire::serialize_describe_ingestion_response(&wire_resp)
            }
            Err(e) => quicksight_error_response(&e),
        }
    }

    async fn handle_list_ingestions(
        &self,
        state: &Arc<tokio::sync::RwLock<QuickSightState>>,
        data_set_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let ingestions = state.list_ingestions(data_set_id);
        let wire_ingestions: Vec<wire::Ingestion> = ingestions
            .iter()
            .map(|ing| wire::Ingestion {
                arn: Some(ing.arn.clone()),
                ingestion_id: Some(ing.ingestion_id.clone()),
                ingestion_status: Some(ing.ingestion_status.clone()),
                ..Default::default()
            })
            .collect();

        let wire_resp = wire::ListIngestionsResponse {
            ingestions: Some(wire_ingestions),
            request_id: Some("mock-request-id".to_string()),
            status: Some(200),
            ..Default::default()
        };
        wire::serialize_list_ingestions_response(&wire_resp)
    }
}

/// Convert a state DataSet to a wire model DataSet.
fn data_set_to_wire(ds: &crate::types::DataSet) -> wire::DataSet {
    wire::DataSet {
        arn: Some(ds.arn.clone()),
        data_set_id: Some(ds.data_set_id.clone()),
        name: Some(ds.name.clone()),
        import_mode: Some(ds.import_mode.clone()),
        created_time: Some(ds.created_time.timestamp() as f64),
        last_updated_time: Some(ds.last_updated_time.timestamp() as f64),
        ..Default::default()
    }
}

/// Convert a state DataSet to a wire model DataSetSummary.
fn data_set_to_summary(ds: &crate::types::DataSet) -> wire::DataSetSummary {
    wire::DataSetSummary {
        arn: Some(ds.arn.clone()),
        data_set_id: Some(ds.data_set_id.clone()),
        name: Some(ds.name.clone()),
        import_mode: Some(ds.import_mode.clone()),
        created_time: Some(ds.created_time.timestamp() as f64),
        last_updated_time: Some(ds.last_updated_time.timestamp() as f64),
        ..Default::default()
    }
}

fn group_to_wire(g: &crate::types::QuickSightGroup) -> wire::Group {
    wire::Group {
        arn: Some(g.arn.clone()),
        group_name: Some(g.group_name.clone()),
        description: Some(g.description.clone()),
        principal_id: Some(g.principal_id.clone()),
    }
}

fn user_to_wire(u: &crate::types::QuickSightUser) -> wire::User {
    wire::User {
        arn: Some(u.arn.clone()),
        user_name: Some(u.user_name.clone()),
        email: Some(u.email.clone()),
        role: Some(u.role.clone()),
        identity_type: Some(u.identity_type.clone()),
        active: Some(u.active),
        principal_id: Some(u.principal_id.clone()),
        ..Default::default()
    }
}

fn convert_resource_permissions(
    value: Option<Vec<wire::ResourcePermission>>,
) -> Vec<crate::types::DataSourceResourcePermission> {
    value
        .unwrap_or_default()
        .into_iter()
        .map(|p| crate::types::DataSourceResourcePermission {
            principal: p.principal,
            actions: p.actions,
        })
        .collect()
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

fn extract_query(uri: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(q) = uri.find('?') {
        let query_str = &uri[q + 1..];
        for pair in query_str.split('&') {
            if let Some(eq) = pair.find('=') {
                let key = urldecode(&pair[..eq]);
                let value = urldecode(&pair[eq + 1..]);
                map.insert(key, value);
            }
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().unwrap_or(b'0');
            let lo = chars.next().unwrap_or(b'0');
            let decoded =
                u8::from_str_radix(&format!("{}{}", hi as char, lo as char), 16).unwrap_or(b'?');
            result.push(decoded as char);
        } else if b == b'+' {
            result.push(' ');
        } else {
            result.push(b as char);
        }
    }
    result
}

fn quicksight_error_response(err: &QuickSightError) -> MockResponse {
    let (status, error_type) = match err {
        QuickSightError::ResourceExists { .. } => (409u16, "ResourceExistsException"),
        QuickSightError::GroupExists { .. } => (409, "ResourceExistsException"),
        QuickSightError::UserExists { .. } => (409, "ResourceExistsException"),
        QuickSightError::NamespaceExists { .. } => (409, "ResourceExistsException"),
        QuickSightError::DescribeDataSetNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::UpdateDataSetNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::DeleteDataSetNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::DataSourceNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::DashboardNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::GroupNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::MemberNotInGroup { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::UserNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::DataSetNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::AnalysisNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::FolderNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::MemberNotInFolder { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::NamespaceNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::TemplateNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::ThemeNotFound { .. } => (404, "ResourceNotFoundException"),
        QuickSightError::IngestionNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Message": err.to_string(),
        "RequestId": "mock-request-id",
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Message": message,
        "RequestId": "mock-request-id",
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
