use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{GreengrassError, GreengrassState};
use crate::types::{Definition, DefinitionType, DefinitionVersion, DeploymentInfo, Group};
use crate::views::GreengrassStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

fn extract_path(uri: &str) -> &str {
    uri.find("://")
        .and_then(|i| uri[i + 3..].find('/').map(|j| &uri[i + 3 + j..]))
        .and_then(|p| p.split('?').next())
        .unwrap_or("/")
}

pub struct GreengrassService {
    pub(crate) state: Arc<BackendState<GreengrassState>>,
    pub(crate) notifier: StateChangeNotifier<GreengrassStateView>,
}

impl GreengrassService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for GreengrassService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for GreengrassService {
    fn service_name(&self) -> &str {
        "greengrass"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://greengrass\.(.+)\.amazonaws\.com",
            r"https?://greengrass\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn parse_body(body: &[u8]) -> Result<Value, GreengrassError> {
    if body.is_empty() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    serde_json::from_slice(body).map_err(|_| GreengrassError::BadRequest)
}

impl GreengrassService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = match request.uri.find('?') {
            Some(idx) => request.uri[idx + 1..].to_string(),
            None => String::new(),
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        match (method, segments.as_slice()) {
            // ---- Groups ----
            ("POST", ["greengrass", "groups"]) => {
                self.handle_create_group(&state, &request, &[], &query_map, &region, account_id)
                    .await
            }
            ("GET", ["greengrass", "groups"]) => self.handle_list_groups(&state).await,
            ("GET", ["greengrass", "groups", group_id]) => {
                self.handle_get_group(&state, group_id).await
            }
            ("PUT", ["greengrass", "groups", group_id]) => {
                let labels: &[(&str, &str)] = &[("GroupId", group_id)];
                self.handle_update_group(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["greengrass", "groups", group_id]) => {
                self.handle_delete_group(&state, group_id).await
            }

            // ---- Group Versions ----
            ("POST", ["greengrass", "groups", group_id, "versions"]) => {
                let labels: &[(&str, &str)] = &[("GroupId", group_id)];
                self.handle_create_group_version(
                    &state, group_id, &request, labels, &query_map, &region, account_id,
                )
                .await
            }
            ("GET", ["greengrass", "groups", group_id, "versions"]) => {
                self.handle_list_group_versions(&state, group_id).await
            }
            ("GET", ["greengrass", "groups", group_id, "versions", version_id]) => {
                self.handle_get_group_version(&state, group_id, version_id)
                    .await
            }

            // ---- Group Role Association ----
            ("PUT", ["greengrass", "groups", group_id, "role"]) => {
                let labels: &[(&str, &str)] = &[("GroupId", group_id)];
                self.handle_associate_role_to_group(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["greengrass", "groups", group_id, "role"]) => {
                self.handle_disassociate_role_from_group(&state, group_id)
                    .await
            }
            ("GET", ["greengrass", "groups", group_id, "role"]) => {
                self.handle_get_associated_role(&state, group_id).await
            }

            // ---- Deployments ----
            ("POST", ["greengrass", "groups", group_id, "deployments"]) => {
                let labels: &[(&str, &str)] = &[("GroupId", group_id)];
                self.handle_create_deployment(
                    &state, group_id, &request, labels, &query_map, &region, account_id,
                )
                .await
            }
            ("GET", ["greengrass", "groups", group_id, "deployments"]) => {
                self.handle_list_deployments(&state, group_id).await
            }
            (
                "GET",
                [
                    "greengrass",
                    "groups",
                    group_id,
                    "deployments",
                    deployment_id,
                    "status",
                ],
            ) => {
                self.handle_get_deployment_status(&state, group_id, deployment_id)
                    .await
            }
            ("POST", ["greengrass", "groups", group_id, "deployments", "$reset"]) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_reset_deployments(&state, group_id, &region, account_id)
                    .await
            }

            // ---- Core Definitions ----
            ("POST", ["greengrass", "definition", "cores"]) => {
                let name = match wire::deserialize_create_core_definition_request(
                    &request,
                    &[],
                    &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_create_definition(
                    &state,
                    DefinitionType::Core,
                    &name,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "cores"]) => {
                self.handle_list_definitions(&state, DefinitionType::Core)
                    .await
            }
            ("GET", ["greengrass", "definition", "cores", def_id]) => {
                self.handle_get_definition(&state, DefinitionType::Core, def_id)
                    .await
            }
            ("PUT", ["greengrass", "definition", "cores", def_id]) => {
                let labels: &[(&str, &str)] = &[("CoreDefinitionId", def_id)];
                let name = match wire::deserialize_update_core_definition_request(
                    &request, labels, &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_update_definition(&state, DefinitionType::Core, def_id, &name)
                    .await
            }
            ("DELETE", ["greengrass", "definition", "cores", def_id]) => {
                self.handle_delete_definition(&state, DefinitionType::Core, def_id)
                    .await
            }
            ("POST", ["greengrass", "definition", "cores", def_id, "versions"]) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_create_definition_version(
                    &state,
                    DefinitionType::Core,
                    def_id,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "cores", def_id, "versions"]) => {
                self.handle_list_definition_versions(&state, DefinitionType::Core, def_id)
                    .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "cores",
                    def_id,
                    "versions",
                    ver_id,
                ],
            ) => {
                self.handle_get_definition_version(&state, DefinitionType::Core, def_id, ver_id)
                    .await
            }

            // ---- Device Definitions ----
            ("POST", ["greengrass", "definition", "devices"]) => {
                let name = match wire::deserialize_create_device_definition_request(
                    &request,
                    &[],
                    &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_create_definition(
                    &state,
                    DefinitionType::Device,
                    &name,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "devices"]) => {
                self.handle_list_definitions(&state, DefinitionType::Device)
                    .await
            }
            ("GET", ["greengrass", "definition", "devices", def_id]) => {
                self.handle_get_definition(&state, DefinitionType::Device, def_id)
                    .await
            }
            ("PUT", ["greengrass", "definition", "devices", def_id]) => {
                let labels: &[(&str, &str)] = &[("DeviceDefinitionId", def_id)];
                let name = match wire::deserialize_update_device_definition_request(
                    &request, labels, &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_update_definition(&state, DefinitionType::Device, def_id, &name)
                    .await
            }
            ("DELETE", ["greengrass", "definition", "devices", def_id]) => {
                self.handle_delete_definition(&state, DefinitionType::Device, def_id)
                    .await
            }
            ("POST", ["greengrass", "definition", "devices", def_id, "versions"]) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_create_definition_version(
                    &state,
                    DefinitionType::Device,
                    def_id,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "devices", def_id, "versions"]) => {
                self.handle_list_definition_versions(&state, DefinitionType::Device, def_id)
                    .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "devices",
                    def_id,
                    "versions",
                    ver_id,
                ],
            ) => {
                self.handle_get_definition_version(&state, DefinitionType::Device, def_id, ver_id)
                    .await
            }

            // ---- Function Definitions ----
            ("POST", ["greengrass", "definition", "functions"]) => {
                let name = match wire::deserialize_create_function_definition_request(
                    &request,
                    &[],
                    &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_create_definition(
                    &state,
                    DefinitionType::Function,
                    &name,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "functions"]) => {
                self.handle_list_definitions(&state, DefinitionType::Function)
                    .await
            }
            ("GET", ["greengrass", "definition", "functions", def_id]) => {
                self.handle_get_definition(&state, DefinitionType::Function, def_id)
                    .await
            }
            ("PUT", ["greengrass", "definition", "functions", def_id]) => {
                let labels: &[(&str, &str)] = &[("FunctionDefinitionId", def_id)];
                let name = match wire::deserialize_update_function_definition_request(
                    &request, labels, &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_update_definition(&state, DefinitionType::Function, def_id, &name)
                    .await
            }
            ("DELETE", ["greengrass", "definition", "functions", def_id]) => {
                self.handle_delete_definition(&state, DefinitionType::Function, def_id)
                    .await
            }
            ("POST", ["greengrass", "definition", "functions", def_id, "versions"]) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_create_definition_version(
                    &state,
                    DefinitionType::Function,
                    def_id,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "functions", def_id, "versions"]) => {
                self.handle_list_definition_versions(&state, DefinitionType::Function, def_id)
                    .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "functions",
                    def_id,
                    "versions",
                    ver_id,
                ],
            ) => {
                self.handle_get_definition_version(&state, DefinitionType::Function, def_id, ver_id)
                    .await
            }

            // ---- Resource Definitions ----
            ("POST", ["greengrass", "definition", "resources"]) => {
                let name = match wire::deserialize_create_resource_definition_request(
                    &request,
                    &[],
                    &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_create_definition(
                    &state,
                    DefinitionType::Resource,
                    &name,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "resources"]) => {
                self.handle_list_definitions(&state, DefinitionType::Resource)
                    .await
            }
            ("GET", ["greengrass", "definition", "resources", def_id]) => {
                self.handle_get_definition(&state, DefinitionType::Resource, def_id)
                    .await
            }
            ("PUT", ["greengrass", "definition", "resources", def_id]) => {
                let labels: &[(&str, &str)] = &[("ResourceDefinitionId", def_id)];
                let name = match wire::deserialize_update_resource_definition_request(
                    &request, labels, &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_update_definition(&state, DefinitionType::Resource, def_id, &name)
                    .await
            }
            ("DELETE", ["greengrass", "definition", "resources", def_id]) => {
                self.handle_delete_definition(&state, DefinitionType::Resource, def_id)
                    .await
            }
            ("POST", ["greengrass", "definition", "resources", def_id, "versions"]) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_create_definition_version(
                    &state,
                    DefinitionType::Resource,
                    def_id,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "resources", def_id, "versions"]) => {
                self.handle_list_definition_versions(&state, DefinitionType::Resource, def_id)
                    .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "resources",
                    def_id,
                    "versions",
                    ver_id,
                ],
            ) => {
                self.handle_get_definition_version(&state, DefinitionType::Resource, def_id, ver_id)
                    .await
            }

            // ---- Subscription Definitions ----
            ("POST", ["greengrass", "definition", "subscriptions"]) => {
                let name = match wire::deserialize_create_subscription_definition_request(
                    &request,
                    &[],
                    &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_create_definition(
                    &state,
                    DefinitionType::Subscription,
                    &name,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["greengrass", "definition", "subscriptions"]) => {
                self.handle_list_definitions(&state, DefinitionType::Subscription)
                    .await
            }
            ("GET", ["greengrass", "definition", "subscriptions", def_id]) => {
                self.handle_get_definition(&state, DefinitionType::Subscription, def_id)
                    .await
            }
            ("PUT", ["greengrass", "definition", "subscriptions", def_id]) => {
                let labels: &[(&str, &str)] = &[("SubscriptionDefinitionId", def_id)];
                let name = match wire::deserialize_update_subscription_definition_request(
                    &request, labels, &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_update_definition(&state, DefinitionType::Subscription, def_id, &name)
                    .await
            }
            ("DELETE", ["greengrass", "definition", "subscriptions", def_id]) => {
                self.handle_delete_definition(&state, DefinitionType::Subscription, def_id)
                    .await
            }
            (
                "POST",
                [
                    "greengrass",
                    "definition",
                    "subscriptions",
                    def_id,
                    "versions",
                ],
            ) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_create_definition_version(
                    &state,
                    DefinitionType::Subscription,
                    def_id,
                    &region,
                    account_id,
                )
                .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "subscriptions",
                    def_id,
                    "versions",
                ],
            ) => {
                self.handle_list_definition_versions(&state, DefinitionType::Subscription, def_id)
                    .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "subscriptions",
                    def_id,
                    "versions",
                    ver_id,
                ],
            ) => {
                self.handle_get_definition_version(
                    &state,
                    DefinitionType::Subscription,
                    def_id,
                    ver_id,
                )
                .await
            }

            // ---- Connector Definitions ----
            ("POST", ["greengrass", "definition", "connectors"]) => {
                let name = match wire::deserialize_create_connector_definition_request(
                    &request,
                    &[],
                    &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_create_connector_definition(&state, &name, &region, account_id)
                    .await
            }
            ("GET", ["greengrass", "definition", "connectors"]) => {
                self.handle_list_connector_definitions(&state).await
            }
            ("GET", ["greengrass", "definition", "connectors", def_id]) => {
                self.handle_get_connector_definition(&state, def_id).await
            }
            ("PUT", ["greengrass", "definition", "connectors", def_id]) => {
                let labels: &[(&str, &str)] = &[("ConnectorDefinitionId", def_id)];
                let name = match wire::deserialize_update_connector_definition_request(
                    &request, labels, &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_update_connector_definition(&state, def_id, &name)
                    .await
            }
            ("DELETE", ["greengrass", "definition", "connectors", def_id]) => {
                self.handle_delete_connector_definition(&state, def_id)
                    .await
            }
            ("POST", ["greengrass", "definition", "connectors", def_id, "versions"]) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_create_connector_definition_version(&state, def_id, &region, account_id)
                    .await
            }
            ("GET", ["greengrass", "definition", "connectors", def_id, "versions"]) => {
                self.handle_list_connector_definition_versions(&state, def_id)
                    .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "connectors",
                    def_id,
                    "versions",
                    ver_id,
                ],
            ) => {
                self.handle_get_connector_definition_version(&state, def_id, ver_id)
                    .await
            }

            // ---- Logger Definitions ----
            ("POST", ["greengrass", "definition", "loggers"]) => {
                let name = match wire::deserialize_create_logger_definition_request(
                    &request,
                    &[],
                    &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_create_logger_definition(&state, &name, &region, account_id)
                    .await
            }
            ("GET", ["greengrass", "definition", "loggers"]) => {
                self.handle_list_logger_definitions(&state).await
            }
            ("GET", ["greengrass", "definition", "loggers", def_id]) => {
                self.handle_get_logger_definition(&state, def_id).await
            }
            ("PUT", ["greengrass", "definition", "loggers", def_id]) => {
                let labels: &[(&str, &str)] = &[("LoggerDefinitionId", def_id)];
                let name = match wire::deserialize_update_logger_definition_request(
                    &request, labels, &query_map,
                ) {
                    Ok(v) => v.name.unwrap_or_default(),
                    Err(e) => {
                        return greengrass_error_response(&GreengrassError::InvalidInput(e));
                    }
                };
                self.handle_update_logger_definition(&state, def_id, &name)
                    .await
            }
            ("DELETE", ["greengrass", "definition", "loggers", def_id]) => {
                self.handle_delete_logger_definition(&state, def_id).await
            }
            ("POST", ["greengrass", "definition", "loggers", def_id, "versions"]) => {
                let body = match parse_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return greengrass_error_response(&e),
                };
                let _ = body;
                self.handle_create_logger_definition_version(&state, def_id, &region, account_id)
                    .await
            }
            ("GET", ["greengrass", "definition", "loggers", def_id, "versions"]) => {
                self.handle_list_logger_definition_versions(&state, def_id)
                    .await
            }
            (
                "GET",
                [
                    "greengrass",
                    "definition",
                    "loggers",
                    def_id,
                    "versions",
                    ver_id,
                ],
            ) => {
                self.handle_get_logger_definition_version(&state, def_id, ver_id)
                    .await
            }

            _ => greengrass_error_response(&GreengrassError::UnknownOperation),
        }
    }

    // ---- Group handlers ----

    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return greengrass_error_response(&GreengrassError::InvalidInput(e)),
        };
        let name = input.name.as_str();

        let mut state = state.write().await;
        match state.create_group(name, region, account_id) {
            Ok(group) => {
                wire::serialize_create_group_response(&group_to_create_group_response(&group))
            }
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_get_group(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_group(group_id) {
            Ok(group) => wire::serialize_get_group_response(&group_to_get_group_response(group)),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_group(group_id) {
            Ok(()) => wire::serialize_delete_group_response(&wire::DeleteGroupResponse {}),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_list_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let groups: Vec<wire::GroupInformation> = state
            .list_groups()
            .iter()
            .map(|g| group_to_group_information(g))
            .collect();
        wire::serialize_list_groups_response(&wire::ListGroupsResponse {
            groups: Some(groups),
            next_token: None,
        })
    }

    async fn handle_update_group(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return greengrass_error_response(&GreengrassError::InvalidInput(e)),
        };
        let name = input.name.as_deref().unwrap_or("");
        let mut state = state.write().await;
        match state.update_group(&input.group_id, name) {
            Ok(()) => wire::serialize_update_group_response(&wire::UpdateGroupResponse {}),
            Err(e) => greengrass_error_response(&e),
        }
    }

    // ---- Group Version handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_group_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return greengrass_error_response(&GreengrassError::InvalidInput(e)),
        };
        let core_def = input.core_definition_version_arn;
        let device_def = input.device_definition_version_arn;
        let function_def = input.function_definition_version_arn;
        let resource_def = input.resource_definition_version_arn;
        let subscription_def = input.subscription_definition_version_arn;

        let mut state = state.write().await;
        match state.create_group_version(
            group_id,
            region,
            account_id,
            core_def,
            device_def,
            function_def,
            resource_def,
            subscription_def,
        ) {
            Ok(gv) => {
                wire::serialize_create_group_version_response(&wire::CreateGroupVersionResponse {
                    arn: Some(gv.arn),
                    creation_timestamp: Some(gv.creation_timestamp),
                    id: Some(gv.id),
                    version: Some(gv.version),
                })
            }
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_get_group_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
        version_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_group_version(group_id, version_id) {
            Ok(gv) => wire::serialize_get_group_version_response(&wire::GetGroupVersionResponse {
                arn: Some(gv.arn.clone()),
                creation_timestamp: Some(gv.creation_timestamp.clone()),
                definition: Some(wire::GroupVersion {
                    core_definition_version_arn: gv.core_definition_version_arn.clone(),
                    device_definition_version_arn: gv.device_definition_version_arn.clone(),
                    function_definition_version_arn: gv.function_definition_version_arn.clone(),
                    resource_definition_version_arn: gv.resource_definition_version_arn.clone(),
                    subscription_definition_version_arn: gv
                        .subscription_definition_version_arn
                        .clone(),
                    ..Default::default()
                }),
                id: Some(gv.id.clone()),
                version: Some(gv.version.clone()),
            }),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_list_group_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_group_versions(group_id) {
            Ok(versions) => {
                let items: Vec<wire::VersionInformation> = versions
                    .iter()
                    .map(|gv| wire::VersionInformation {
                        arn: Some(gv.arn.clone()),
                        creation_timestamp: Some(gv.creation_timestamp.clone()),
                        id: Some(gv.id.clone()),
                        version: Some(gv.version.clone()),
                    })
                    .collect();
                wire::serialize_list_group_versions_response(&wire::ListGroupVersionsResponse {
                    versions: Some(items),
                    next_token: None,
                })
            }
            Err(e) => greengrass_error_response(&e),
        }
    }

    // ---- Role Association handlers ----

    async fn handle_associate_role_to_group(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_role_to_group_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return greengrass_error_response(&GreengrassError::InvalidInput(e)),
        };
        if input.role_arn.is_empty() {
            return greengrass_error_response(&GreengrassError::InvalidInput(
                "Missing 'RoleArn'".to_string(),
            ));
        }
        let role_arn = input.role_arn.as_str();
        let mut state = state.write().await;
        match state.associate_role_to_group(&input.group_id, role_arn) {
            Ok(associated_at) => wire::serialize_associate_role_to_group_response(
                &wire::AssociateRoleToGroupResponse {
                    associated_at: Some(associated_at),
                },
            ),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_disassociate_role_from_group(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.disassociate_role_from_group(group_id) {
            Ok(disassociated_at) => wire::serialize_disassociate_role_from_group_response(
                &wire::DisassociateRoleFromGroupResponse {
                    disassociated_at: Some(disassociated_at),
                },
            ),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_get_associated_role(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_associated_role(group_id) {
            Ok(assoc) => {
                wire::serialize_get_associated_role_response(&wire::GetAssociatedRoleResponse {
                    associated_at: Some(assoc.associated_at.clone()),
                    role_arn: Some(assoc.role_arn.clone()),
                })
            }
            Err(e) => greengrass_error_response(&e),
        }
    }

    // ---- Deployment handlers ----

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return greengrass_error_response(&GreengrassError::InvalidInput(e)),
        };
        let deployment_type = if input.deployment_type.is_empty() {
            "NewDeployment"
        } else {
            input.deployment_type.as_str()
        };
        let mut state = state.write().await;
        match state.create_deployment(group_id, deployment_type, region, account_id) {
            Ok(d) => wire::serialize_create_deployment_response(&wire::CreateDeploymentResponse {
                deployment_arn: Some(d.deployment_arn),
                deployment_id: Some(d.deployment_id),
            }),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_get_deployment_status(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
        deployment_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_deployment_status(group_id, deployment_id) {
            Ok(d) => {
                wire::serialize_get_deployment_status_response(&wire::GetDeploymentStatusResponse {
                    deployment_status: Some(d.deployment_status.clone()),
                    deployment_type: Some(d.deployment_type.clone()),
                    updated_at: Some(d.updated_at.clone()),
                    ..Default::default()
                })
            }
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_list_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_deployments(group_id) {
            Ok(deployments) => {
                let items: Vec<wire::Deployment> =
                    deployments.iter().map(|d| deployment_to_wire(d)).collect();
                wire::serialize_list_deployments_response(&wire::ListDeploymentsResponse {
                    deployments: Some(items),
                    next_token: None,
                })
            }
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_reset_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        group_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.reset_deployments(group_id, region, account_id) {
            Ok(d) => wire::serialize_reset_deployments_response(&wire::ResetDeploymentsResponse {
                deployment_arn: Some(d.deployment_arn),
                deployment_id: Some(d.deployment_id),
            }),
            Err(e) => greengrass_error_response(&e),
        }
    }

    // ---- Generic Definition handlers ----

    async fn handle_create_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_definition(def_type, name, region, account_id) {
            Ok(def) => serialize_create_definition_response(def_type, &def),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_get_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
        def_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_definition(def_type, def_id) {
            Ok(def) => serialize_get_definition_response(def_type, def),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_delete_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
        def_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_definition(def_type, def_id) {
            Ok(()) => serialize_delete_definition_response(def_type),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_update_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.update_definition(def_type, def_id, name) {
            Ok(()) => serialize_update_definition_response(def_type),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_list_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
    ) -> MockResponse {
        let state = state.read().await;
        let defs = state.list_definitions(def_type);
        serialize_list_definitions_response(def_type, &defs)
    }

    async fn handle_create_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_definition_version(def_type, def_id, region, account_id) {
            Ok(dv) => serialize_create_definition_version_response(def_type, &dv),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_get_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_definition_version(def_type, def_id, version_id) {
            Ok(dv) => serialize_get_definition_version_response(def_type, dv),
            Err(e) => greengrass_error_response(&e),
        }
    }

    async fn handle_list_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_type: DefinitionType,
        def_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_definition_versions(def_type, def_id) {
            Ok(versions) => serialize_list_definition_versions_response(def_type, &versions),
            Err(e) => greengrass_error_response(&e),
        }
    }

    // ---- Core Definition named handlers (for smithy-codegen detection) ----

    async fn handle_create_core_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition(state, DefinitionType::Core, name, region, account_id)
            .await
    }

    async fn handle_get_core_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_get_definition(state, DefinitionType::Core, def_id)
            .await
    }

    async fn handle_delete_core_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_delete_definition(state, DefinitionType::Core, def_id)
            .await
    }

    async fn handle_update_core_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        self.handle_update_definition(state, DefinitionType::Core, def_id, name)
            .await
    }

    async fn handle_list_core_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        self.handle_list_definitions(state, DefinitionType::Core)
            .await
    }

    async fn handle_create_core_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition_version(
            state,
            DefinitionType::Core,
            def_id,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_core_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        self.handle_get_definition_version(state, DefinitionType::Core, def_id, version_id)
            .await
    }

    async fn handle_list_core_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_list_definition_versions(state, DefinitionType::Core, def_id)
            .await
    }

    // ---- Device Definition named handlers (for smithy-codegen detection) ----

    async fn handle_create_device_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition(state, DefinitionType::Device, name, region, account_id)
            .await
    }

    async fn handle_get_device_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_get_definition(state, DefinitionType::Device, def_id)
            .await
    }

    async fn handle_delete_device_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_delete_definition(state, DefinitionType::Device, def_id)
            .await
    }

    async fn handle_update_device_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        self.handle_update_definition(state, DefinitionType::Device, def_id, name)
            .await
    }

    async fn handle_list_device_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        self.handle_list_definitions(state, DefinitionType::Device)
            .await
    }

    async fn handle_create_device_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition_version(
            state,
            DefinitionType::Device,
            def_id,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_device_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        self.handle_get_definition_version(state, DefinitionType::Device, def_id, version_id)
            .await
    }

    async fn handle_list_device_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_list_definition_versions(state, DefinitionType::Device, def_id)
            .await
    }

    // ---- Function Definition named handlers (for smithy-codegen detection) ----

    async fn handle_create_function_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition(state, DefinitionType::Function, name, region, account_id)
            .await
    }

    async fn handle_get_function_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_get_definition(state, DefinitionType::Function, def_id)
            .await
    }

    async fn handle_delete_function_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_delete_definition(state, DefinitionType::Function, def_id)
            .await
    }

    async fn handle_update_function_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        self.handle_update_definition(state, DefinitionType::Function, def_id, name)
            .await
    }

    async fn handle_list_function_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        self.handle_list_definitions(state, DefinitionType::Function)
            .await
    }

    async fn handle_create_function_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition_version(
            state,
            DefinitionType::Function,
            def_id,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_function_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        self.handle_get_definition_version(state, DefinitionType::Function, def_id, version_id)
            .await
    }

    async fn handle_list_function_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_list_definition_versions(state, DefinitionType::Function, def_id)
            .await
    }

    // ---- Resource Definition named handlers (for smithy-codegen detection) ----

    async fn handle_create_resource_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition(state, DefinitionType::Resource, name, region, account_id)
            .await
    }

    async fn handle_get_resource_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_get_definition(state, DefinitionType::Resource, def_id)
            .await
    }

    async fn handle_delete_resource_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_delete_definition(state, DefinitionType::Resource, def_id)
            .await
    }

    async fn handle_update_resource_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        self.handle_update_definition(state, DefinitionType::Resource, def_id, name)
            .await
    }

    async fn handle_list_resource_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        self.handle_list_definitions(state, DefinitionType::Resource)
            .await
    }

    async fn handle_create_resource_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition_version(
            state,
            DefinitionType::Resource,
            def_id,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_resource_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        self.handle_get_definition_version(state, DefinitionType::Resource, def_id, version_id)
            .await
    }

    async fn handle_list_resource_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_list_definition_versions(state, DefinitionType::Resource, def_id)
            .await
    }

    // ---- Subscription Definition named handlers (for smithy-codegen detection) ----

    async fn handle_create_subscription_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition(
            state,
            DefinitionType::Subscription,
            name,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_subscription_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_get_definition(state, DefinitionType::Subscription, def_id)
            .await
    }

    async fn handle_delete_subscription_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_delete_definition(state, DefinitionType::Subscription, def_id)
            .await
    }

    async fn handle_update_subscription_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        self.handle_update_definition(state, DefinitionType::Subscription, def_id, name)
            .await
    }

    async fn handle_list_subscription_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        self.handle_list_definitions(state, DefinitionType::Subscription)
            .await
    }

    async fn handle_create_subscription_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition_version(
            state,
            DefinitionType::Subscription,
            def_id,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_subscription_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        self.handle_get_definition_version(state, DefinitionType::Subscription, def_id, version_id)
            .await
    }

    async fn handle_list_subscription_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_list_definition_versions(state, DefinitionType::Subscription, def_id)
            .await
    }

    // ---- Connector Definition named handlers (for smithy-codegen detection) ----

    async fn handle_create_connector_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition(state, DefinitionType::Connector, name, region, account_id)
            .await
    }

    async fn handle_get_connector_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_get_definition(state, DefinitionType::Connector, def_id)
            .await
    }

    async fn handle_delete_connector_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_delete_definition(state, DefinitionType::Connector, def_id)
            .await
    }

    async fn handle_update_connector_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        self.handle_update_definition(state, DefinitionType::Connector, def_id, name)
            .await
    }

    async fn handle_list_connector_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        self.handle_list_definitions(state, DefinitionType::Connector)
            .await
    }

    async fn handle_create_connector_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition_version(
            state,
            DefinitionType::Connector,
            def_id,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_connector_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        self.handle_get_definition_version(state, DefinitionType::Connector, def_id, version_id)
            .await
    }

    async fn handle_list_connector_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_list_definition_versions(state, DefinitionType::Connector, def_id)
            .await
    }

    // ---- Logger Definition named handlers (for smithy-codegen detection) ----

    async fn handle_create_logger_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition(state, DefinitionType::Logger, name, region, account_id)
            .await
    }

    async fn handle_get_logger_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_get_definition(state, DefinitionType::Logger, def_id)
            .await
    }

    async fn handle_delete_logger_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_delete_definition(state, DefinitionType::Logger, def_id)
            .await
    }

    async fn handle_update_logger_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        name: &str,
    ) -> MockResponse {
        self.handle_update_definition(state, DefinitionType::Logger, def_id, name)
            .await
    }

    async fn handle_list_logger_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
    ) -> MockResponse {
        self.handle_list_definitions(state, DefinitionType::Logger)
            .await
    }

    async fn handle_create_logger_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        self.handle_create_definition_version(
            state,
            DefinitionType::Logger,
            def_id,
            region,
            account_id,
        )
        .await
    }

    async fn handle_get_logger_definition_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
        version_id: &str,
    ) -> MockResponse {
        self.handle_get_definition_version(state, DefinitionType::Logger, def_id, version_id)
            .await
    }

    async fn handle_list_logger_definition_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GreengrassState>>,
        def_id: &str,
    ) -> MockResponse {
        self.handle_list_definition_versions(state, DefinitionType::Logger, def_id)
            .await
    }
}

// ---- Wire conversion helpers ----

fn group_to_create_group_response(group: &Group) -> wire::CreateGroupResponse {
    wire::CreateGroupResponse {
        arn: Some(group.arn.clone()),
        creation_timestamp: Some(group.creation_timestamp.clone()),
        id: Some(group.id.clone()),
        last_updated_timestamp: Some(group.last_updated_timestamp.clone()),
        latest_version: Some(group.latest_version.clone()),
        latest_version_arn: Some(group.latest_version_arn.clone()),
        name: Some(group.name.clone()),
    }
}

fn group_to_get_group_response(group: &Group) -> wire::GetGroupResponse {
    wire::GetGroupResponse {
        arn: Some(group.arn.clone()),
        creation_timestamp: Some(group.creation_timestamp.clone()),
        id: Some(group.id.clone()),
        last_updated_timestamp: Some(group.last_updated_timestamp.clone()),
        latest_version: Some(group.latest_version.clone()),
        latest_version_arn: Some(group.latest_version_arn.clone()),
        name: Some(group.name.clone()),
        tags: None,
    }
}

fn group_to_group_information(group: &Group) -> wire::GroupInformation {
    wire::GroupInformation {
        arn: Some(group.arn.clone()),
        creation_timestamp: Some(group.creation_timestamp.clone()),
        id: Some(group.id.clone()),
        last_updated_timestamp: Some(group.last_updated_timestamp.clone()),
        latest_version: Some(group.latest_version.clone()),
        latest_version_arn: Some(group.latest_version_arn.clone()),
        name: Some(group.name.clone()),
    }
}

fn def_to_definition_information(def: &Definition) -> wire::DefinitionInformation {
    wire::DefinitionInformation {
        arn: Some(def.arn.clone()),
        creation_timestamp: Some(def.creation_timestamp.clone()),
        id: Some(def.id.clone()),
        last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
        latest_version: Some(def.latest_version.clone()),
        latest_version_arn: Some(def.latest_version_arn.clone()),
        name: Some(def.name.clone()),
        tags: None,
    }
}

fn dv_to_version_information(dv: &DefinitionVersion) -> wire::VersionInformation {
    wire::VersionInformation {
        arn: Some(dv.arn.clone()),
        creation_timestamp: Some(dv.creation_timestamp.clone()),
        id: Some(dv.id.clone()),
        version: Some(dv.version.clone()),
    }
}

fn deployment_to_wire(d: &DeploymentInfo) -> wire::Deployment {
    wire::Deployment {
        created_at: Some(d.created_at.clone()),
        deployment_arn: Some(d.deployment_arn.clone()),
        deployment_id: Some(d.deployment_id.clone()),
        deployment_type: Some(d.deployment_type.clone()),
        group_arn: Some(d.group_arn.clone()),
    }
}

// ---- Per-type serialization dispatch ----

fn serialize_create_definition_response(
    def_type: DefinitionType,
    def: &Definition,
) -> MockResponse {
    match def_type {
        DefinitionType::Connector => wire::serialize_create_connector_definition_response(
            &wire::CreateConnectorDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
            },
        ),
        DefinitionType::Core => {
            wire::serialize_create_core_definition_response(&wire::CreateCoreDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
            })
        }
        DefinitionType::Device => wire::serialize_create_device_definition_response(
            &wire::CreateDeviceDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
            },
        ),
        DefinitionType::Function => wire::serialize_create_function_definition_response(
            &wire::CreateFunctionDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
            },
        ),
        DefinitionType::Logger => wire::serialize_create_logger_definition_response(
            &wire::CreateLoggerDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
            },
        ),
        DefinitionType::Resource => wire::serialize_create_resource_definition_response(
            &wire::CreateResourceDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
            },
        ),
        DefinitionType::Subscription => wire::serialize_create_subscription_definition_response(
            &wire::CreateSubscriptionDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
            },
        ),
    }
}

fn serialize_get_definition_response(def_type: DefinitionType, def: &Definition) -> MockResponse {
    match def_type {
        DefinitionType::Connector => wire::serialize_get_connector_definition_response(
            &wire::GetConnectorDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
                tags: None,
            },
        ),
        DefinitionType::Core => {
            wire::serialize_get_core_definition_response(&wire::GetCoreDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
                tags: None,
            })
        }
        DefinitionType::Device => {
            wire::serialize_get_device_definition_response(&wire::GetDeviceDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
                tags: None,
            })
        }
        DefinitionType::Function => {
            wire::serialize_get_function_definition_response(&wire::GetFunctionDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
                tags: None,
            })
        }
        DefinitionType::Logger => {
            wire::serialize_get_logger_definition_response(&wire::GetLoggerDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
                tags: None,
            })
        }
        DefinitionType::Resource => {
            wire::serialize_get_resource_definition_response(&wire::GetResourceDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
                tags: None,
            })
        }
        DefinitionType::Subscription => wire::serialize_get_subscription_definition_response(
            &wire::GetSubscriptionDefinitionResponse {
                arn: Some(def.arn.clone()),
                creation_timestamp: Some(def.creation_timestamp.clone()),
                id: Some(def.id.clone()),
                last_updated_timestamp: Some(def.last_updated_timestamp.clone()),
                latest_version: Some(def.latest_version.clone()),
                latest_version_arn: Some(def.latest_version_arn.clone()),
                name: Some(def.name.clone()),
                tags: None,
            },
        ),
    }
}

fn serialize_delete_definition_response(def_type: DefinitionType) -> MockResponse {
    match def_type {
        DefinitionType::Connector => wire::serialize_delete_connector_definition_response(
            &wire::DeleteConnectorDefinitionResponse {},
        ),
        DefinitionType::Core => {
            wire::serialize_delete_core_definition_response(&wire::DeleteCoreDefinitionResponse {})
        }
        DefinitionType::Device => wire::serialize_delete_device_definition_response(
            &wire::DeleteDeviceDefinitionResponse {},
        ),
        DefinitionType::Function => wire::serialize_delete_function_definition_response(
            &wire::DeleteFunctionDefinitionResponse {},
        ),
        DefinitionType::Logger => wire::serialize_delete_logger_definition_response(
            &wire::DeleteLoggerDefinitionResponse {},
        ),
        DefinitionType::Resource => wire::serialize_delete_resource_definition_response(
            &wire::DeleteResourceDefinitionResponse {},
        ),
        DefinitionType::Subscription => wire::serialize_delete_subscription_definition_response(
            &wire::DeleteSubscriptionDefinitionResponse {},
        ),
    }
}

fn serialize_update_definition_response(def_type: DefinitionType) -> MockResponse {
    match def_type {
        DefinitionType::Connector => wire::serialize_update_connector_definition_response(
            &wire::UpdateConnectorDefinitionResponse {},
        ),
        DefinitionType::Core => {
            wire::serialize_update_core_definition_response(&wire::UpdateCoreDefinitionResponse {})
        }
        DefinitionType::Device => wire::serialize_update_device_definition_response(
            &wire::UpdateDeviceDefinitionResponse {},
        ),
        DefinitionType::Function => wire::serialize_update_function_definition_response(
            &wire::UpdateFunctionDefinitionResponse {},
        ),
        DefinitionType::Logger => wire::serialize_update_logger_definition_response(
            &wire::UpdateLoggerDefinitionResponse {},
        ),
        DefinitionType::Resource => wire::serialize_update_resource_definition_response(
            &wire::UpdateResourceDefinitionResponse {},
        ),
        DefinitionType::Subscription => wire::serialize_update_subscription_definition_response(
            &wire::UpdateSubscriptionDefinitionResponse {},
        ),
    }
}

fn serialize_list_definitions_response(
    def_type: DefinitionType,
    defs: &[&Definition],
) -> MockResponse {
    let items: Vec<wire::DefinitionInformation> = defs
        .iter()
        .map(|d| def_to_definition_information(d))
        .collect();
    match def_type {
        DefinitionType::Connector => wire::serialize_list_connector_definitions_response(
            &wire::ListConnectorDefinitionsResponse {
                definitions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Core => {
            wire::serialize_list_core_definitions_response(&wire::ListCoreDefinitionsResponse {
                definitions: Some(items),
                next_token: None,
            })
        }
        DefinitionType::Device => {
            wire::serialize_list_device_definitions_response(&wire::ListDeviceDefinitionsResponse {
                definitions: Some(items),
                next_token: None,
            })
        }
        DefinitionType::Function => wire::serialize_list_function_definitions_response(
            &wire::ListFunctionDefinitionsResponse {
                definitions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Logger => {
            wire::serialize_list_logger_definitions_response(&wire::ListLoggerDefinitionsResponse {
                definitions: Some(items),
                next_token: None,
            })
        }
        DefinitionType::Resource => wire::serialize_list_resource_definitions_response(
            &wire::ListResourceDefinitionsResponse {
                definitions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Subscription => wire::serialize_list_subscription_definitions_response(
            &wire::ListSubscriptionDefinitionsResponse {
                definitions: Some(items),
                next_token: None,
            },
        ),
    }
}

fn serialize_create_definition_version_response(
    def_type: DefinitionType,
    dv: &DefinitionVersion,
) -> MockResponse {
    match def_type {
        DefinitionType::Connector => wire::serialize_create_connector_definition_version_response(
            &wire::CreateConnectorDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
            },
        ),
        DefinitionType::Core => wire::serialize_create_core_definition_version_response(
            &wire::CreateCoreDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
            },
        ),
        DefinitionType::Device => wire::serialize_create_device_definition_version_response(
            &wire::CreateDeviceDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
            },
        ),
        DefinitionType::Function => wire::serialize_create_function_definition_version_response(
            &wire::CreateFunctionDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
            },
        ),
        DefinitionType::Logger => wire::serialize_create_logger_definition_version_response(
            &wire::CreateLoggerDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
            },
        ),
        DefinitionType::Resource => wire::serialize_create_resource_definition_version_response(
            &wire::CreateResourceDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
            },
        ),
        DefinitionType::Subscription => {
            wire::serialize_create_subscription_definition_version_response(
                &wire::CreateSubscriptionDefinitionVersionResponse {
                    arn: Some(dv.arn.clone()),
                    creation_timestamp: Some(dv.creation_timestamp.clone()),
                    id: Some(dv.id.clone()),
                    version: Some(dv.version.clone()),
                },
            )
        }
    }
}

fn serialize_get_definition_version_response(
    def_type: DefinitionType,
    dv: &DefinitionVersion,
) -> MockResponse {
    match def_type {
        DefinitionType::Connector => wire::serialize_get_connector_definition_version_response(
            &wire::GetConnectorDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
                ..Default::default()
            },
        ),
        DefinitionType::Core => wire::serialize_get_core_definition_version_response(
            &wire::GetCoreDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
                ..Default::default()
            },
        ),
        DefinitionType::Device => wire::serialize_get_device_definition_version_response(
            &wire::GetDeviceDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
                ..Default::default()
            },
        ),
        DefinitionType::Function => wire::serialize_get_function_definition_version_response(
            &wire::GetFunctionDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
                ..Default::default()
            },
        ),
        DefinitionType::Logger => wire::serialize_get_logger_definition_version_response(
            &wire::GetLoggerDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
                ..Default::default()
            },
        ),
        DefinitionType::Resource => wire::serialize_get_resource_definition_version_response(
            &wire::GetResourceDefinitionVersionResponse {
                arn: Some(dv.arn.clone()),
                creation_timestamp: Some(dv.creation_timestamp.clone()),
                id: Some(dv.id.clone()),
                version: Some(dv.version.clone()),
                ..Default::default()
            },
        ),
        DefinitionType::Subscription => {
            wire::serialize_get_subscription_definition_version_response(
                &wire::GetSubscriptionDefinitionVersionResponse {
                    arn: Some(dv.arn.clone()),
                    creation_timestamp: Some(dv.creation_timestamp.clone()),
                    id: Some(dv.id.clone()),
                    version: Some(dv.version.clone()),
                    ..Default::default()
                },
            )
        }
    }
}

fn serialize_list_definition_versions_response(
    def_type: DefinitionType,
    versions: &[&DefinitionVersion],
) -> MockResponse {
    let items: Vec<wire::VersionInformation> = versions
        .iter()
        .map(|dv| dv_to_version_information(dv))
        .collect();
    match def_type {
        DefinitionType::Connector => wire::serialize_list_connector_definition_versions_response(
            &wire::ListConnectorDefinitionVersionsResponse {
                versions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Core => wire::serialize_list_core_definition_versions_response(
            &wire::ListCoreDefinitionVersionsResponse {
                versions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Device => wire::serialize_list_device_definition_versions_response(
            &wire::ListDeviceDefinitionVersionsResponse {
                versions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Function => wire::serialize_list_function_definition_versions_response(
            &wire::ListFunctionDefinitionVersionsResponse {
                versions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Logger => wire::serialize_list_logger_definition_versions_response(
            &wire::ListLoggerDefinitionVersionsResponse {
                versions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Resource => wire::serialize_list_resource_definition_versions_response(
            &wire::ListResourceDefinitionVersionsResponse {
                versions: Some(items),
                next_token: None,
            },
        ),
        DefinitionType::Subscription => {
            wire::serialize_list_subscription_definition_versions_response(
                &wire::ListSubscriptionDefinitionVersionsResponse {
                    versions: Some(items),
                    next_token: None,
                },
            )
        }
    }
}

fn greengrass_error_response(err: &GreengrassError) -> MockResponse {
    let (status, error_type) = match err {
        GreengrassError::IdNotFound(_) => (404u16, "IdNotFoundException"),
        GreengrassError::VersionNotFound(_) => (404, "VersionNotFoundException"),
        GreengrassError::InvalidContainerDefinition => (400, "InvalidContainerDefinitionException"),
        GreengrassError::InvalidInput(_) => (400, "InvalidInputException"),
        GreengrassError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
        GreengrassError::RoleNotAssociated => (404, "404"),
        GreengrassError::BadRequest => (400, "BadRequestException"),
        GreengrassError::UnknownOperation => (404, "UnknownOperationException"),
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
