use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{ConnectError, ConnectState};
use crate::types::ConnectInstance;
use crate::views::ConnectStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ConnectService {
    pub(crate) state: Arc<BackendState<ConnectState>>,
    pub(crate) notifier: StateChangeNotifier<ConnectStateView>,
}

impl ConnectService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ConnectService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ConnectService {
    fn service_name(&self) -> &str {
        "connect"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://connect\..*\.amazonaws\.com",
            r"https?://connect\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ConnectService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let query_map: HashMap<String, String> =
            winterbaume_core::parse_query_string(&query_string);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        // Handle /tags/{resourceArn+} routes
        if segments[0] == "tags" && segments.len() >= 2 {
            let resource_arn = percent_decode(&segments[1..].join("/"));
            return match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &resource_arn)
                        .await
                }
                "POST" => {
                    let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    let tag_keys = extract_query_list(&query_string, "tagKeys");
                    self.handle_untag_resource(&state, &resource_arn, &tag_keys)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        // Handle /analytics-data/instance/{InstanceId}/association routes
        // PUT  /analytics-data/instance/{InstanceId}/association - AssociateAnalyticsDataSet
        // POST /analytics-data/instance/{InstanceId}/association - DisassociateAnalyticsDataSet
        // GET  /analytics-data/instance/{InstanceId}/association - ListAnalyticsDataAssociations
        if segments[0] == "analytics-data"
            && segments.len() >= 4
            && segments[1] == "instance"
            && segments[3] == "association"
        {
            let instance_id = segments[2];
            let labels: &[(&str, &str)] = &[("InstanceId", instance_id)];
            return match method {
                "PUT" => {
                    self.handle_associate_analytics_data_set(&state, &request, labels, &query_map)
                        .await
                }
                "POST" => {
                    self.handle_disassociate_analytics_data_set(
                        &state, &request, labels, &query_map,
                    )
                    .await
                }
                "GET" => {
                    self.handle_list_analytics_data_associations(&state, instance_id)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        // Route: PUT /instance - CreateInstance
        // Route: GET /instance - ListInstances
        // Route: GET /instance/{InstanceId} - DescribeInstance
        // Route: DELETE /instance/{InstanceId} - DeleteInstance

        if segments[0] != "instance" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match (method, segments.len()) {
            // PUT /instance - CreateInstance
            ("PUT", 1) => {
                self.handle_create_instance(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // GET /instance - ListInstances
            ("GET", 1) => self.handle_list_instances(&state).await,
            // GET /instance/{InstanceId} - DescribeInstance
            ("GET", 2) => {
                let instance_id = segments[1];
                self.handle_describe_instance(&state, instance_id).await
            }
            // DELETE /instance/{InstanceId} - DeleteInstance
            ("DELETE", 2) => {
                let instance_id = segments[1];
                self.handle_delete_instance(&state, instance_id).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_instance_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        if input.identity_management_type.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "Missing 'IdentityManagementType'",
            );
        }

        let tags = input.tags.clone().unwrap_or_default();
        let instance_id = uuid::Uuid::new_v4().to_string();

        let mut state = state.write().await;
        match state.create_instance(
            &instance_id,
            &input.identity_management_type,
            input.instance_alias.as_deref(),
            input.inbound_calls_enabled,
            input.outbound_calls_enabled,
            account_id,
            region,
            tags,
        ) {
            Ok(inst) => wire::serialize_create_instance_response(&wire::CreateInstanceResponse {
                id: Some(inst.id.clone()),
                arn: Some(inst.arn.clone()),
            }),
            Err(e) => connect_error_response(&e),
        }
    }

    async fn handle_describe_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        instance_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_instance(instance_id) {
            Ok(inst) => {
                wire::serialize_describe_instance_response(&wire::DescribeInstanceResponse {
                    instance: Some(to_wire_instance(inst)),
                    replication_configuration: None,
                })
            }
            Err(e) => connect_error_response(&e),
        }
    }

    async fn handle_delete_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        instance_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_instance(instance_id) {
            Ok(()) => wire::serialize_delete_instance_response(),
            Err(e) => connect_error_response(&e),
        }
    }

    async fn handle_list_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let instances = state.list_instances();
        let entries: Vec<wire::InstanceSummary> = instances
            .iter()
            .map(|i| to_wire_instance_summary(i))
            .collect();

        wire::serialize_list_instances_response(&wire::ListInstancesResponse {
            instance_summary_list: Some(entries),
            next_token: None,
        })
    }

    // --- Analytics Data Set Association handlers ---

    async fn handle_associate_analytics_data_set(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_analytics_data_set_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
            };
        if input.data_set_id.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'DataSetId'");
        }

        let mut state = state.write().await;
        match state.associate_analytics_data_set(
            &input.instance_id,
            &input.data_set_id,
            input.target_account_id.as_deref(),
        ) {
            Ok(assoc) => wire::serialize_associate_analytics_data_set_response(
                &wire::AssociateAnalyticsDataSetResponse {
                    data_set_id: Some(assoc.data_set_id.clone()),
                    resource_share_arn: Some(assoc.resource_share_arn.clone()),
                    resource_share_id: Some(assoc.resource_share_id.clone()),
                    target_account_id: assoc.target_account_id.clone(),
                    ..Default::default()
                },
            ),
            Err(e) => connect_error_response(&e),
        }
    }

    async fn handle_disassociate_analytics_data_set(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_analytics_data_set_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
            };
        if input.data_set_id.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'DataSetId'");
        }

        let mut state = state.write().await;
        match state.disassociate_analytics_data_set(&input.instance_id, &input.data_set_id) {
            Ok(()) => wire::serialize_disassociate_analytics_data_set_response(),
            Err(e) => connect_error_response(&e),
        }
    }

    async fn handle_list_analytics_data_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        instance_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_analytics_data_associations(instance_id) {
            Ok(assocs) => {
                let results: Vec<wire::AnalyticsDataAssociationResult> = assocs
                    .iter()
                    .map(|a| wire::AnalyticsDataAssociationResult {
                        data_set_id: Some(a.data_set_id.clone()),
                        resource_share_arn: Some(a.resource_share_arn.clone()),
                        resource_share_id: Some(a.resource_share_id.clone()),
                        target_account_id: a.target_account_id.clone(),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_list_analytics_data_associations_response(
                    &wire::ListAnalyticsDataAssociationsResponse {
                        results: Some(results),
                        next_token: None,
                        ..Default::default()
                    },
                )
            }
            Err(e) => connect_error_response(&e),
        }
    }

    // --- Tag handlers ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags_for_resource(arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
            ..Default::default()
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectState>>,
        arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        state.untag_resource(arn, tag_keys);
        wire::serialize_untag_resource_response()
    }
}

fn to_wire_instance(inst: &ConnectInstance) -> wire::Instance {
    wire::Instance {
        id: Some(inst.id.clone()),
        arn: Some(inst.arn.clone()),
        identity_management_type: Some(inst.identity_management_type.clone()),
        instance_alias: inst.instance_alias.clone(),
        instance_status: Some(inst.instance_status.clone()),
        created_time: Some(inst.created_time.timestamp() as f64),
        inbound_calls_enabled: Some(inst.inbound_calls_enabled),
        outbound_calls_enabled: Some(inst.outbound_calls_enabled),
        ..Default::default()
    }
}

fn to_wire_instance_summary(inst: &ConnectInstance) -> wire::InstanceSummary {
    wire::InstanceSummary {
        id: Some(inst.id.clone()),
        arn: Some(inst.arn.clone()),
        identity_management_type: Some(inst.identity_management_type.clone()),
        instance_alias: inst.instance_alias.clone(),
        instance_status: Some(inst.instance_status.clone()),
        created_time: Some(inst.created_time.timestamp() as f64),
        inbound_calls_enabled: Some(inst.inbound_calls_enabled),
        outbound_calls_enabled: Some(inst.outbound_calls_enabled),
        ..Default::default()
    }
}

fn extract_path_and_query(uri: &str) -> (String, String) {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => (
            path_and_query[..q].to_string(),
            path_and_query[q + 1..].to_string(),
        ),
        None => (path_and_query.to_string(), String::new()),
    }
}

fn extract_query_list(query_string: &str, key: &str) -> Vec<String> {
    query_string
        .split('&')
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=')?;

            if k == key {
                Some(percent_decode(v))
            } else {
                None
            }
        })
        .collect()
}

fn percent_decode(s: &str) -> String {
    s.replace("%20", " ")
        .replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%3a", ":")
        .replace("%2f", "/")
        .replace('+', " ")
}

fn connect_error_response(err: &ConnectError) -> MockResponse {
    let (status, error_type) = match err {
        ConnectError::InstanceNotFound { .. } => (404, "ResourceNotFoundException"),
        ConnectError::InstanceAliasConflict { .. } => (409, "ResourceConflictException"),
        ConnectError::AnalyticsDataSetAlreadyAssociated { .. } => {
            (409, "ResourceConflictException")
        }
        ConnectError::AnalyticsDataSetNotAssociated { .. } => (404, "ResourceNotFoundException"),
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
