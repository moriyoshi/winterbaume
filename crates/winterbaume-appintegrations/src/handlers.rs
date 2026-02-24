use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{AppIntegrationsError, AppIntegrationsState};
use crate::types::{
    Application, DataIntegration, DataIntegrationAssociation, EventIntegration,
    EventIntegrationAssociationRecord,
};
use crate::views::AppIntegrationsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

/// Convert a serializable value to a `Value`, returning `None` if it serialises to null.
fn to_value_opt<T: serde::Serialize>(v: &T) -> Option<Value> {
    let val = serde_json::to_value(v).ok()?;
    if val.is_null() { None } else { Some(val) }
}

pub struct AppIntegrationsService {
    pub(crate) state: Arc<BackendState<AppIntegrationsState>>,
    pub(crate) notifier: StateChangeNotifier<AppIntegrationsStateView>,
}

impl AppIntegrationsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppIntegrationsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppIntegrationsService {
    fn service_name(&self) -> &str {
        "app-integrations"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://app-integrations\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<AppIntegrationsState>>;

impl AppIntegrationsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "BadRequestException", "Invalid JSON body");
        }

        let qs = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = parse_query_string(qs);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let response = match (method.as_str(), segments.as_slice()) {
            ("POST", ["applications"]) => {
                self.handle_create_application(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            ("GET", ["applications"]) => {
                self.handle_list_applications(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["applications", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("Arn", arn_decoded.as_str())];
                self.handle_get_application(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["applications", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("Arn", arn_decoded.as_str())];
                self.handle_update_application(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["applications", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("Arn", arn_decoded.as_str())];
                self.handle_delete_application(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["applications", id, "associations"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ApplicationId", id_decoded.as_str())];
                self.handle_list_application_associations(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["dataIntegrations"]) => {
                self.handle_create_data_integration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            ("GET", ["dataIntegrations"]) => {
                self.handle_list_data_integrations(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["dataIntegrations", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("Identifier", id_decoded.as_str())];
                self.handle_get_data_integration(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["dataIntegrations", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("Identifier", id_decoded.as_str())];
                self.handle_update_data_integration(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["dataIntegrations", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("DataIntegrationIdentifier", id_decoded.as_str())];
                self.handle_delete_data_integration(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["dataIntegrations", id, "associations"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("DataIntegrationIdentifier", id_decoded.as_str())];
                self.handle_create_data_integration_association(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["dataIntegrations", id, "associations"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("DataIntegrationIdentifier", id_decoded.as_str())];
                self.handle_list_data_integration_associations(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["dataIntegrations", id, "associations", aid]) => {
                let id_decoded = percent_decode(id);
                let aid_decoded = percent_decode(aid);
                let labels: &[(&str, &str)] = &[
                    ("DataIntegrationIdentifier", id_decoded.as_str()),
                    ("DataIntegrationAssociationIdentifier", aid_decoded.as_str()),
                ];
                self.handle_update_data_integration_association(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            ("POST", ["eventIntegrations"]) => {
                self.handle_create_event_integration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            ("GET", ["eventIntegrations"]) => {
                self.handle_list_event_integrations(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["eventIntegrations", name]) => {
                let name_decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", name_decoded.as_str())];
                self.handle_get_event_integration(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["eventIntegrations", name]) => {
                let name_decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", name_decoded.as_str())];
                self.handle_update_event_integration(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["eventIntegrations", name]) => {
                let name_decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", name_decoded.as_str())];
                self.handle_delete_event_integration(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["eventIntegrations", name, "associations"]) => {
                let name_decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("EventIntegrationName", name_decoded.as_str())];
                self.handle_list_event_integration_associations(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            ("POST", ["tags", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("resourceArn", arn_decoded.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["tags", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("resourceArn", arn_decoded.as_str())];
                self.handle_untag_resource(&state, &request, labels, &request.uri)
                    .await
            }
            ("GET", ["tags", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("resourceArn", arn_decoded.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "PATCH" | "DELETE" | "PUT")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_application(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Name is required");
        }
        if input.namespace.is_empty() {
            return rest_json_error(400, "ValidationException", "Namespace is required");
        }

        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:app-integrations:{}:{}:application/{}",
            region, account_id, id
        );
        let now = chrono::Utc::now().timestamp();
        let app = Application {
            id: id.clone(),
            arn: arn.clone(),
            name: input.name.clone(),
            namespace: input.namespace.clone(),
            description: input.description.clone(),
            application_type: input.application_type.clone(),
            client_token: input.client_token.clone(),
            initialization_timeout: input.initialization_timeout,
            is_service: input.is_service,
            permissions: input.permissions.unwrap_or_default(),
            publications: input
                .publications
                .unwrap_or_default()
                .iter()
                .filter_map(to_value_opt)
                .collect(),
            subscriptions: input
                .subscriptions
                .unwrap_or_default()
                .iter()
                .filter_map(to_value_opt)
                .collect(),
            application_config: input.application_config.as_ref().and_then(to_value_opt),
            application_source_config: to_value_opt(&input.application_source_config),
            iframe_config: input.iframe_config.as_ref().and_then(to_value_opt),
            created_time: now,
            last_modified_time: now,
            tags: input.tags.unwrap_or_default(),
        };

        let mut state = state.write().await;
        match state.create_application(app) {
            Ok(_) => {
                wire::serialize_create_application_response(&wire::CreateApplicationResponse {
                    arn: Some(arn),
                    id: Some(id),
                })
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_get_application(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_application(&input.arn) {
            Ok(a) => wire::serialize_get_application_response(&application_to_get_response(a)),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_update_application(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        let result = state.update_application(&input.arn, |a| {
            if let Some(v) = input.description.clone() {
                a.description = Some(v);
            }
            if let Some(v) = input.permissions.clone() {
                a.permissions = v;
            }
            if let Some(v) = input.publications.as_ref() {
                a.publications = v.iter().filter_map(to_value_opt).collect();
            }
            if let Some(v) = input.subscriptions.as_ref() {
                a.subscriptions = v.iter().filter_map(to_value_opt).collect();
            }
            if let Some(v) = input.application_config.as_ref().and_then(to_value_opt) {
                a.application_config = Some(v);
            }
            if let Some(v) = input
                .application_source_config
                .as_ref()
                .and_then(to_value_opt)
            {
                a.application_source_config = Some(v);
            }
            if let Some(v) = input.iframe_config.as_ref().and_then(to_value_opt) {
                a.iframe_config = Some(v);
            }
            if let Some(v) = input.initialization_timeout {
                a.initialization_timeout = Some(v);
            }
        });
        match result {
            Ok(_) => {
                wire::serialize_update_application_response(&wire::UpdateApplicationResponse {})
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_delete_application(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_application(&input.arn) {
            Ok(()) => {
                wire::serialize_delete_application_response(&wire::DeleteApplicationResponse {})
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_list_applications(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_applications_request(request, labels, query) {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let items: Vec<wire::ApplicationSummary> = state
            .list_applications()
            .into_iter()
            .map(|a| wire::ApplicationSummary {
                application_type: a.application_type.clone(),
                arn: Some(a.arn.clone()),
                id: Some(a.id.clone()),
                name: Some(a.name.clone()),
                namespace: Some(a.namespace.clone()),
                created_time: Some(a.created_time as f64),
                last_modified_time: Some(a.last_modified_time as f64),
                is_service: a.is_service,
            })
            .collect();
        wire::serialize_list_applications_response(&wire::ListApplicationsResponse {
            applications: Some(items),
            next_token: None,
        })
    }

    async fn handle_list_application_associations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_application_associations_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let state = state.read().await;
        match state.list_application_associations(&input.application_id) {
            Ok(items) => {
                let summaries: Vec<wire::ApplicationAssociationSummary> = items
                    .into_iter()
                    .map(|a| wire::ApplicationAssociationSummary {
                        application_association_arn: Some(a.application_association_arn.clone()),
                        application_arn: Some(a.application_arn.clone()),
                        client_id: Some(a.client_id.clone()),
                    })
                    .collect();
                wire::serialize_list_application_associations_response(
                    &wire::ListApplicationAssociationsResponse {
                        application_associations: Some(summaries),
                        next_token: None,
                    },
                )
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_create_data_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_data_integration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Name is required");
        }
        if input.kms_key.is_empty() {
            return rest_json_error(400, "ValidationException", "KmsKey is required");
        }
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:app-integrations:{}:{}:data-integration/{}",
            region, account_id, id
        );
        let di = DataIntegration {
            id: id.clone(),
            arn: arn.clone(),
            name: input.name.clone(),
            description: input.description.clone(),
            kms_key: input.kms_key.clone(),
            source_uri: input.source_u_r_i.clone(),
            file_configuration: input.file_configuration.as_ref().and_then(to_value_opt),
            object_configuration: input.object_configuration.as_ref().and_then(to_value_opt),
            schedule_config: input.schedule_config.as_ref().and_then(to_value_opt),
            client_token: input.client_token.clone(),
            tags: input.tags.unwrap_or_default(),
        };

        let mut state = state.write().await;
        match state.create_data_integration(di) {
            Ok(d) => {
                let response = wire::CreateDataIntegrationResponse {
                    arn: Some(d.arn.clone()),
                    client_token: d.client_token.clone(),
                    description: d.description.clone(),
                    file_configuration: parse_file_config(d.file_configuration.as_ref()),
                    id: Some(d.id.clone()),
                    kms_key: Some(d.kms_key.clone()),
                    name: Some(d.name.clone()),
                    object_configuration: d
                        .object_configuration
                        .as_ref()
                        .and_then(parse_object_configuration),
                    schedule_configuration: parse_schedule_config(d.schedule_config.as_ref()),
                    source_u_r_i: d.source_uri.clone(),
                    tags: if d.tags.is_empty() {
                        None
                    } else {
                        Some(d.tags.clone())
                    },
                };
                wire::serialize_create_data_integration_response(&response)
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_get_data_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_data_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_data_integration(&input.identifier) {
            Ok(d) => {
                wire::serialize_get_data_integration_response(&data_integration_to_response(d))
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_update_data_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_data_integration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        let result = state.update_data_integration(&input.identifier, |d| {
            if let Some(v) = input.name.clone() {
                d.name = v;
            }
            if let Some(v) = input.description.clone() {
                d.description = Some(v);
            }
        });
        match result {
            Ok(_) => wire::serialize_update_data_integration_response(
                &wire::UpdateDataIntegrationResponse {},
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_delete_data_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_data_integration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_data_integration(&input.data_integration_identifier) {
            Ok(()) => wire::serialize_delete_data_integration_response(
                &wire::DeleteDataIntegrationResponse {},
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_list_data_integrations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_data_integrations_request(request, labels, query) {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let items: Vec<wire::DataIntegrationSummary> = state
            .list_data_integrations()
            .into_iter()
            .map(|d| wire::DataIntegrationSummary {
                arn: Some(d.arn.clone()),
                name: Some(d.name.clone()),
                source_u_r_i: d.source_uri.clone(),
            })
            .collect();
        wire::serialize_list_data_integrations_response(&wire::ListDataIntegrationsResponse {
            data_integrations: Some(items),
            next_token: None,
        })
    }

    async fn handle_create_data_integration_association(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_data_integration_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let identifier = input.data_integration_identifier.as_str();
        let association_id = uuid::Uuid::new_v4().to_string();
        let association_arn = format!(
            "arn:aws:app-integrations:{}:{}:data-integration-association/{}",
            region, account_id, association_id
        );
        let now = chrono::Utc::now().timestamp();

        // Resolve DI ARN/ID up front.
        let (di_id, di_arn) = {
            let guard = state.read().await;
            match guard.get_data_integration(identifier) {
                Ok(d) => (d.id.clone(), d.arn.clone()),
                Err(e) => return app_error_response(&e),
            }
        };

        let association = DataIntegrationAssociation {
            id: association_id.clone(),
            arn: association_arn,
            data_integration_id: di_id,
            data_integration_arn: di_arn.clone(),
            client_id: input.client_id.clone(),
            destination_uri: input.destination_u_r_i.clone(),
            last_execution_status: None,
            execution_configuration: input
                .execution_configuration
                .as_ref()
                .and_then(to_value_opt),
            client_association_metadata: input.client_association_metadata.unwrap_or_default(),
            created_time: now,
            last_modified_time: now,
        };

        let mut state = state.write().await;
        match state.create_data_integration_association(identifier, association) {
            Ok(a) => {
                let response = wire::CreateDataIntegrationAssociationResponse {
                    data_integration_arn: Some(a.data_integration_arn.clone()),
                    data_integration_association_id: Some(a.id.clone()),
                };
                wire::serialize_create_data_integration_association_response(&response)
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_update_data_integration_association(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // ExecutionStatus.Status is not part of the modelled UpdateDataIntegrationAssociation
        // request shape, so parse it manually from the JSON body for backward compatibility.
        let last_execution_status: Option<String> = if request.body.is_empty() {
            None
        } else {
            serde_json::from_slice::<Value>(&request.body)
                .ok()
                .and_then(|v| {
                    v.get("ExecutionStatus")
                        .and_then(|s| s.get("Status"))
                        .and_then(|s| s.as_str())
                        .map(String::from)
                })
        };
        let input = match wire::deserialize_update_data_integration_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_data_integration_association(
            &input.data_integration_identifier,
            &input.data_integration_association_identifier,
            last_execution_status,
        ) {
            Ok(_) => wire::serialize_update_data_integration_association_response(
                &wire::UpdateDataIntegrationAssociationResponse {},
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_list_data_integration_associations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_data_integration_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.list_data_integration_associations(&input.data_integration_identifier) {
            Ok(items) => {
                let summaries: Vec<wire::DataIntegrationAssociationSummary> = items
                    .into_iter()
                    .map(|a| wire::DataIntegrationAssociationSummary {
                        client_id: a.client_id.clone(),
                        data_integration_arn: Some(a.data_integration_arn.clone()),
                        data_integration_association_arn: Some(a.arn.clone()),
                        destination_u_r_i: a.destination_uri.clone(),
                        execution_configuration: a
                            .execution_configuration
                            .as_ref()
                            .and_then(parse_execution_configuration),
                        last_execution_status: parse_last_execution_status(
                            a.last_execution_status.as_deref(),
                        ),
                    })
                    .collect();
                wire::serialize_list_data_integration_associations_response(
                    &wire::ListDataIntegrationAssociationsResponse {
                        data_integration_associations: Some(summaries),
                        next_token: None,
                    },
                )
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_create_event_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_integration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Name is required");
        }
        if input.event_filter.source.is_empty() {
            return rest_json_error(400, "ValidationException", "EventFilter.Source is required");
        }
        if input.event_bridge_bus.is_empty() {
            return rest_json_error(400, "ValidationException", "EventBridgeBus is required");
        }

        let arn = format!(
            "arn:aws:app-integrations:{}:{}:event-integration/{}",
            region, account_id, input.name
        );
        let ei = EventIntegration {
            arn: arn.clone(),
            name: input.name.clone(),
            description: input.description.clone(),
            event_filter_source: input.event_filter.source.clone(),
            event_bridge_bus: input.event_bridge_bus.clone(),
            tags: input.tags.unwrap_or_default(),
        };

        let mut state = state.write().await;
        match state.create_event_integration(ei) {
            Ok(_) => wire::serialize_create_event_integration_response(
                &wire::CreateEventIntegrationResponse {
                    event_integration_arn: Some(arn),
                },
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_get_event_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_event_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_event_integration(&input.name) {
            Ok(e) => {
                wire::serialize_get_event_integration_response(&event_integration_to_response(e))
            }
            Err(err) => app_error_response(&err),
        }
    }

    async fn handle_update_event_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_event_integration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_event_integration(&input.name, input.description.clone()) {
            Ok(_) => wire::serialize_update_event_integration_response(
                &wire::UpdateEventIntegrationResponse {},
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_delete_event_integration(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_integration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_event_integration(&input.name) {
            Ok(()) => wire::serialize_delete_event_integration_response(
                &wire::DeleteEventIntegrationResponse {},
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_list_event_integrations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_event_integrations_request(request, labels, query) {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let items: Vec<wire::EventIntegration> = state
            .list_event_integrations()
            .into_iter()
            .map(event_integration_to_summary)
            .collect();
        wire::serialize_list_event_integrations_response(&wire::ListEventIntegrationsResponse {
            event_integrations: Some(items),
            next_token: None,
        })
    }

    async fn handle_list_event_integration_associations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_event_integration_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.list_event_integration_associations(&input.event_integration_name) {
            Ok(items) => {
                let summaries: Vec<wire::EventIntegrationAssociation> = items
                    .into_iter()
                    .map(event_integration_association_to_wire)
                    .collect();
                wire::serialize_list_event_integration_associations_response(
                    &wire::ListEventIntegrationAssociationsResponse {
                        event_integration_associations: Some(summaries),
                        next_token: None,
                    },
                )
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "tags is required");
        }
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        uri: &str,
    ) -> MockResponse {
        // tagKeys is repeated as `tagKeys=k1&tagKeys=k2` on the wire; the generated
        // deserialiser collapses to a single value, so collect each occurrence here.
        let qs = extract_query_string(uri);
        let mut keys: Vec<String> = Vec::new();
        for pair in qs.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                if k == "tagKeys" {
                    keys.push(percent_decode(v));
                }
            }
        }
        if keys.is_empty() {
            return rest_json_error(400, "ValidationException", "tagKeys is required");
        }
        // Resolve resourceArn through the label-only path of the deserialiser.
        let input = match wire::deserialize_untag_resource_request(request, labels, &HashMap::new())
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.list_tags(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                },
            ),
            Err(e) => app_error_response(&e),
        }
    }
}

fn application_to_get_response(a: &Application) -> wire::GetApplicationResponse {
    wire::GetApplicationResponse {
        application_config: a
            .application_config
            .as_ref()
            .and_then(parse_application_config),
        application_source_config: a
            .application_source_config
            .as_ref()
            .and_then(parse_application_source_config),
        application_type: a.application_type.clone(),
        arn: Some(a.arn.clone()),
        created_time: Some(a.created_time as f64),
        description: a.description.clone(),
        iframe_config: a.iframe_config.as_ref().and_then(parse_iframe_config),
        id: Some(a.id.clone()),
        initialization_timeout: a.initialization_timeout,
        is_service: a.is_service,
        last_modified_time: Some(a.last_modified_time as f64),
        name: Some(a.name.clone()),
        namespace: Some(a.namespace.clone()),
        permissions: Some(a.permissions.clone()),
        publications: parse_publications(&a.publications),
        subscriptions: parse_subscriptions(&a.subscriptions),
        tags: if a.tags.is_empty() {
            None
        } else {
            Some(a.tags.clone())
        },
    }
}

fn data_integration_to_response(d: &DataIntegration) -> wire::GetDataIntegrationResponse {
    wire::GetDataIntegrationResponse {
        arn: Some(d.arn.clone()),
        description: d.description.clone(),
        file_configuration: parse_file_config(d.file_configuration.as_ref()),
        id: Some(d.id.clone()),
        kms_key: Some(d.kms_key.clone()),
        name: Some(d.name.clone()),
        object_configuration: d
            .object_configuration
            .as_ref()
            .and_then(parse_object_configuration),
        schedule_configuration: parse_schedule_config(d.schedule_config.as_ref()),
        source_u_r_i: d.source_uri.clone(),
        tags: if d.tags.is_empty() {
            None
        } else {
            Some(d.tags.clone())
        },
    }
}

fn event_integration_to_response(e: &EventIntegration) -> wire::GetEventIntegrationResponse {
    wire::GetEventIntegrationResponse {
        description: e.description.clone(),
        event_bridge_bus: Some(e.event_bridge_bus.clone()),
        event_filter: Some(wire::EventFilter {
            source: e.event_filter_source.clone(),
        }),
        event_integration_arn: Some(e.arn.clone()),
        name: Some(e.name.clone()),
        tags: if e.tags.is_empty() {
            None
        } else {
            Some(e.tags.clone())
        },
    }
}

fn event_integration_to_summary(e: &EventIntegration) -> wire::EventIntegration {
    wire::EventIntegration {
        description: e.description.clone(),
        event_bridge_bus: Some(e.event_bridge_bus.clone()),
        event_filter: Some(wire::EventFilter {
            source: e.event_filter_source.clone(),
        }),
        event_integration_arn: Some(e.arn.clone()),
        name: Some(e.name.clone()),
        tags: if e.tags.is_empty() {
            None
        } else {
            Some(e.tags.clone())
        },
    }
}

fn event_integration_association_to_wire(
    a: &EventIntegrationAssociationRecord,
) -> wire::EventIntegrationAssociation {
    wire::EventIntegrationAssociation {
        client_association_metadata: if a.client_association_metadata.is_empty() {
            None
        } else {
            Some(a.client_association_metadata.clone())
        },
        client_id: Some(a.client_id.clone()),
        event_bridge_rule_name: Some(a.event_bridge_rule_name.clone()),
        event_integration_association_arn: Some(a.event_integration_association_arn.clone()),
        event_integration_association_id: Some(a.event_integration_association_id.clone()),
        event_integration_name: Some(a.event_integration_name.clone()),
    }
}

fn parse_application_config(v: &Value) -> Option<wire::ApplicationConfig> {
    serde_json::from_value(v.clone()).ok()
}

fn parse_application_source_config(v: &Value) -> Option<wire::ApplicationSourceConfig> {
    serde_json::from_value(v.clone()).ok()
}

fn parse_iframe_config(v: &Value) -> Option<wire::IframeConfig> {
    serde_json::from_value(v.clone()).ok()
}

fn parse_publications(items: &[Value]) -> Option<Vec<wire::Publication>> {
    let pubs: Vec<wire::Publication> = items
        .iter()
        .filter_map(|v| serde_json::from_value(v.clone()).ok())
        .collect();
    if pubs.is_empty() { None } else { Some(pubs) }
}

fn parse_subscriptions(items: &[Value]) -> Option<Vec<wire::Subscription>> {
    let subs: Vec<wire::Subscription> = items
        .iter()
        .filter_map(|v| serde_json::from_value(v.clone()).ok())
        .collect();
    if subs.is_empty() { None } else { Some(subs) }
}

fn parse_file_config(v: Option<&Value>) -> Option<wire::FileConfiguration> {
    v.and_then(|v| serde_json::from_value(v.clone()).ok())
}

fn parse_schedule_config(v: Option<&Value>) -> Option<wire::ScheduleConfiguration> {
    v.and_then(|v| serde_json::from_value(v.clone()).ok())
}

fn parse_object_configuration(v: &Value) -> Option<HashMap<String, HashMap<String, Vec<String>>>> {
    serde_json::from_value(v.clone()).ok()
}

fn parse_execution_configuration(v: &Value) -> Option<wire::ExecutionConfiguration> {
    serde_json::from_value(v.clone()).ok()
}

fn parse_last_execution_status(s: Option<&str>) -> Option<wire::LastExecutionStatus> {
    s.map(|status| wire::LastExecutionStatus {
        execution_status: Some(status.to_string()),
        status_message: None,
    })
}

fn app_error_response(err: &AppIntegrationsError) -> MockResponse {
    let (status, error_type) = match err {
        AppIntegrationsError::NotFound { .. } => (404, "ResourceNotFoundException"),
        AppIntegrationsError::AlreadyExists { .. } => (409, "DuplicateResourceException"),
        AppIntegrationsError::InUse { .. } => (409, "ResourceQuotaExceededException"),
        AppIntegrationsError::Validation { .. } => (400, "InvalidRequestException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
