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

use crate::state::{AppRegistryError, ServiceCatalogAppRegistryState};
use crate::types::Application as StateApplication;
use crate::types::AttributeGroup as StateAttributeGroup;
use crate::views::ServiceCatalogAppRegistryStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ServiceCatalogAppRegistryService {
    pub(crate) state: Arc<BackendState<ServiceCatalogAppRegistryState>>,
    pub(crate) notifier: StateChangeNotifier<ServiceCatalogAppRegistryStateView>,
}

impl ServiceCatalogAppRegistryService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ServiceCatalogAppRegistryService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ServiceCatalogAppRegistryService {
    fn service_name(&self) -> &str {
        "servicecatalog-appregistry"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://servicecatalog-appregistry\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ServiceCatalogAppRegistryService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        // Parse query string. UntagResource needs repeated tagKeys preserved, so
        // collect into both a single-value map (for the wire deserializer) and
        // a multi-value list (used by the untag handler directly).
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = parse_single_query(&raw_query);
        let multi_tag_keys: Vec<String> = parse_multi_tag_keys(&raw_query);

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Route dispatch based on path segments and HTTP method.
        //
        // Routes:
        //   POST   /applications                                                         CreateApplication
        //   GET    /applications                                                         ListApplications
        //   GET    /applications/{app}                                                   GetApplication
        //   PATCH  /applications/{app}                                                   UpdateApplication
        //   DELETE /applications/{app}                                                   DeleteApplication
        //   GET    /applications/{app}/attribute-groups                                  ListAssociatedAttributeGroups
        //   PUT    /applications/{app}/attribute-groups/{ag}                             AssociateAttributeGroup
        //   DELETE /applications/{app}/attribute-groups/{ag}                             DisassociateAttributeGroup
        //   GET    /applications/{app}/attribute-group-details                           ListAttributeGroupsForApplication
        //   GET    /applications/{app}/resources                                         ListAssociatedResources
        //   PUT    /applications/{app}/resources/{resourceType}/{resource}               AssociateResource
        //   DELETE /applications/{app}/resources/{resourceType}/{resource}               DisassociateResource
        //   GET    /applications/{app}/resources/{resourceType}/{resource}               GetAssociatedResource
        //   POST   /attribute-groups                                                     CreateAttributeGroup
        //   GET    /attribute-groups                                                     ListAttributeGroups
        //   GET    /attribute-groups/{ag}                                                GetAttributeGroup
        //   PATCH  /attribute-groups/{ag}                                                UpdateAttributeGroup
        //   DELETE /attribute-groups/{ag}                                                DeleteAttributeGroup
        //   GET    /tags/{resourceArn}                                                   ListTagsForResource
        //   POST   /tags/{resourceArn}                                                   TagResource
        //   DELETE /tags/{resourceArn}                                                   UntagResource
        //   GET    /configuration                                                        GetConfiguration
        //   PUT    /configuration                                                        PutConfiguration
        //   POST   /sync/{resourceType}/{resource}                                       SyncResource

        let response = match (method, segments.as_slice()) {
            // ----- Applications -----
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
            ("GET", ["applications", app]) => {
                let app = percent_decode(app);
                let labels: &[(&str, &str)] = &[("application", app.as_str())];
                self.handle_get_application(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["applications", app]) => {
                let app = percent_decode(app);
                let labels: &[(&str, &str)] = &[("application", app.as_str())];
                self.handle_update_application(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["applications", app]) => {
                let app = percent_decode(app);
                let labels: &[(&str, &str)] = &[("application", app.as_str())];
                self.handle_delete_application(&state, &request, labels, &query_map)
                    .await
            }

            // ----- Application / attribute-group associations -----
            ("GET", ["applications", app, "attribute-groups"]) => {
                let app = percent_decode(app);
                let labels: &[(&str, &str)] = &[("application", app.as_str())];
                self.handle_list_associated_attribute_groups(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["applications", app, "attribute-groups", ag]) => {
                let app = percent_decode(app);
                let ag = percent_decode(ag);
                let labels: &[(&str, &str)] = &[
                    ("application", app.as_str()),
                    ("attributeGroup", ag.as_str()),
                ];
                self.handle_associate_attribute_group(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["applications", app, "attribute-groups", ag]) => {
                let app = percent_decode(app);
                let ag = percent_decode(ag);
                let labels: &[(&str, &str)] = &[
                    ("application", app.as_str()),
                    ("attributeGroup", ag.as_str()),
                ];
                self.handle_disassociate_attribute_group(&state, &request, labels, &query_map)
                    .await
            }

            // ----- List attribute group details for application -----
            ("GET", ["applications", app, "attribute-group-details"]) => {
                let app = percent_decode(app);
                let labels: &[(&str, &str)] = &[("application", app.as_str())];
                self.handle_list_attribute_groups_for_application(
                    &state, &request, labels, &query_map,
                )
                .await
            }

            // ----- Application / resource associations -----
            ("GET", ["applications", app, "resources"]) => {
                let app = percent_decode(app);
                let labels: &[(&str, &str)] = &[("application", app.as_str())];
                self.handle_list_associated_resources(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["applications", app, "resources", resource_type, resource]) => {
                let app = percent_decode(app);
                let resource_type = percent_decode(resource_type);
                let resource = percent_decode(resource);
                let labels: &[(&str, &str)] = &[
                    ("application", app.as_str()),
                    ("resourceType", resource_type.as_str()),
                    ("resource", resource.as_str()),
                ];
                self.handle_associate_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["applications", app, "resources", resource_type, resource]) => {
                let app = percent_decode(app);
                let resource_type = percent_decode(resource_type);
                let resource = percent_decode(resource);
                let labels: &[(&str, &str)] = &[
                    ("application", app.as_str()),
                    ("resourceType", resource_type.as_str()),
                    ("resource", resource.as_str()),
                ];
                self.handle_disassociate_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["applications", app, "resources", resource_type, resource]) => {
                let app = percent_decode(app);
                let resource_type = percent_decode(resource_type);
                let resource = percent_decode(resource);
                let labels: &[(&str, &str)] = &[
                    ("application", app.as_str()),
                    ("resourceType", resource_type.as_str()),
                    ("resource", resource.as_str()),
                ];
                self.handle_get_associated_resource(&state, &request, labels, &query_map)
                    .await
            }

            // ----- Attribute Groups -----
            ("POST", ["attribute-groups"]) => {
                self.handle_create_attribute_group(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            ("GET", ["attribute-groups"]) => {
                self.handle_list_attribute_groups(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["attribute-groups", ag]) => {
                let ag = percent_decode(ag);
                let labels: &[(&str, &str)] = &[("attributeGroup", ag.as_str())];
                self.handle_get_attribute_group(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["attribute-groups", ag]) => {
                let ag = percent_decode(ag);
                let labels: &[(&str, &str)] = &[("attributeGroup", ag.as_str())];
                self.handle_update_attribute_group(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["attribute-groups", ag]) => {
                let ag = percent_decode(ag);
                let labels: &[(&str, &str)] = &[("attributeGroup", ag.as_str())];
                self.handle_delete_attribute_group(&state, &request, labels, &query_map)
                    .await
            }

            // ----- Tags -----
            ("GET", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                self.handle_untag_resource(&state, &resource_arn, &multi_tag_keys)
                    .await
            }

            // ----- Configuration -----
            ("GET", ["configuration"]) => self.handle_get_configuration(&state).await,
            ("PUT", ["configuration"]) => {
                self.handle_put_configuration(&state, &request, &[], &query_map)
                    .await
            }

            // ----- Sync -----
            ("POST", ["sync", resource_type, resource]) => {
                let resource_type = percent_decode(resource_type);
                let resource = percent_decode(resource);
                let labels: &[(&str, &str)] = &[
                    ("resourceType", resource_type.as_str()),
                    ("resource", resource.as_str()),
                ];
                self.handle_sync_resource(&state, &request, labels, &query_map)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ---- Application handlers ----

    async fn handle_create_application(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing required field 'name'");
        }
        let description = input.description.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_application(&input.name, description, tags, account_id, region) {
            Ok(app) => {
                let resp = wire::CreateApplicationResponse {
                    application: Some(state_app_to_wire(app)),
                };
                wire::serialize_create_application_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_get_application(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_application(&input.application) {
            Ok(app) => {
                let resp = state_app_to_get_response(app);
                wire::serialize_get_application_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_update_application(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let new_name = input.name.as_deref();
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_application(&input.application, new_name, description) {
            Ok(app) => {
                let resp = wire::UpdateApplicationResponse {
                    application: Some(state_app_to_wire(app)),
                };
                wire::serialize_update_application_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_delete_application(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_application(&input.application) {
            Ok(()) => {
                let resp = wire::DeleteApplicationResponse::default();
                wire::serialize_delete_application_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_list_applications(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_applications_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let apps = state.list_applications();
        let summaries: Vec<wire::ApplicationSummary> =
            apps.iter().map(|app| state_app_to_summary(app)).collect();
        let resp = wire::ListApplicationsResponse {
            applications: Some(summaries),
            next_token: None,
        };
        wire::serialize_list_applications_response(&resp)
    }

    // ---- Attribute Group handlers ----

    async fn handle_create_attribute_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_attribute_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing required field 'name'");
        }
        if input.attributes.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'attributes'",
            );
        }
        let description = input.description.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_attribute_group(
            &input.name,
            description,
            &input.attributes,
            tags,
            account_id,
            region,
        ) {
            Ok(ag) => {
                let resp = wire::CreateAttributeGroupResponse {
                    attribute_group: Some(state_ag_to_wire(ag)),
                };
                wire::serialize_create_attribute_group_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_get_attribute_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_attribute_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_attribute_group(&input.attribute_group) {
            Ok(ag) => {
                let resp = state_ag_to_get_response(ag);
                wire::serialize_get_attribute_group_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_update_attribute_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_attribute_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let new_name = input.name.as_deref();
        let description = input.description.as_deref();
        let attributes = input.attributes.as_deref();

        let mut state = state.write().await;
        match state.update_attribute_group(
            &input.attribute_group,
            new_name,
            description,
            attributes,
        ) {
            Ok(ag) => {
                let resp = wire::UpdateAttributeGroupResponse {
                    attribute_group: Some(state_ag_to_wire(ag)),
                };
                wire::serialize_update_attribute_group_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_delete_attribute_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_attribute_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_attribute_group(&input.attribute_group) {
            Ok(ag) => {
                let summary = state_ag_to_summary(&ag);
                let resp = wire::DeleteAttributeGroupResponse {
                    attribute_group: Some(summary),
                };
                wire::serialize_delete_attribute_group_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_list_attribute_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_attribute_groups_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let groups = state.list_attribute_groups();
        let summaries: Vec<wire::AttributeGroupSummary> =
            groups.iter().map(|ag| state_ag_to_summary(ag)).collect();
        let resp = wire::ListAttributeGroupsResponse {
            attribute_groups: Some(summaries),
            next_token: None,
        };
        wire::serialize_list_attribute_groups_response(&resp)
    }

    // ---- Association handlers ----

    async fn handle_associate_attribute_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_attribute_group_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.associate_attribute_group(&input.application, &input.attribute_group) {
            Ok((app_arn, ag_arn)) => {
                let resp = wire::AssociateAttributeGroupResponse {
                    application_arn: Some(app_arn),
                    attribute_group_arn: Some(ag_arn),
                };
                wire::serialize_associate_attribute_group_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_disassociate_attribute_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_attribute_group_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.disassociate_attribute_group(&input.application, &input.attribute_group) {
            Ok((app_arn, ag_arn)) => {
                let resp = wire::DisassociateAttributeGroupResponse {
                    application_arn: Some(app_arn),
                    attribute_group_arn: Some(ag_arn),
                };
                wire::serialize_disassociate_attribute_group_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_list_associated_attribute_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_associated_attribute_groups_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_associated_attribute_groups(&input.application) {
            Ok(arns) => {
                let resp = wire::ListAssociatedAttributeGroupsResponse {
                    attribute_groups: Some(arns),
                    next_token: None,
                };
                wire::serialize_list_associated_attribute_groups_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_list_attribute_groups_for_application(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_attribute_groups_for_application_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_attribute_groups_for_application(&input.application) {
            Ok(groups) => {
                let details: Vec<wire::AttributeGroupDetails> = groups
                    .iter()
                    .map(|ag| wire::AttributeGroupDetails {
                        id: Some(ag.id.clone()),
                        arn: Some(ag.arn.clone()),
                        name: Some(ag.name.clone()),
                        created_by: None,
                    })
                    .collect();
                let resp = wire::ListAttributeGroupsForApplicationResponse {
                    attribute_groups_details: Some(details),
                    next_token: None,
                };
                wire::serialize_list_attribute_groups_for_application_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_associate_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let options = input.options.unwrap_or_default();
        let mut state = state.write().await;
        match state.associate_resource(
            &input.application,
            &input.resource_type,
            &input.resource,
            options,
        ) {
            Ok((app_arn, resource_arn)) => {
                let resp = wire::AssociateResourceResponse {
                    application_arn: Some(app_arn),
                    resource_arn: Some(resource_arn),
                    options: None,
                };
                wire::serialize_associate_resource_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_disassociate_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.disassociate_resource(&input.application, &input.resource_type, &input.resource)
        {
            Ok((app_arn, resource_arn)) => {
                let resp = wire::DisassociateResourceResponse {
                    application_arn: Some(app_arn),
                    resource_arn: Some(resource_arn),
                };
                wire::serialize_disassociate_resource_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_get_associated_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_associated_resource_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_associated_resource(
            &input.application,
            &input.resource_type,
            &input.resource,
        ) {
            Ok(assoc) => {
                let resp = wire::GetAssociatedResourceResponse {
                    resource: Some(wire::Resource {
                        arn: assoc.resource_arn.clone(),
                        name: Some(assoc.resource.clone()),
                        association_time: None,
                        integrations: None,
                    }),
                    options: if assoc.options.is_empty() {
                        None
                    } else {
                        Some(assoc.options.clone())
                    },
                    application_tag_result: None,
                };
                wire::serialize_get_associated_resource_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_list_associated_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_associated_resources_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.list_associated_resources(&input.application) {
            Ok(assocs) => {
                let resources: Vec<wire::ResourceInfo> = assocs
                    .iter()
                    .map(|a| wire::ResourceInfo {
                        arn: a.resource_arn.clone(),
                        name: Some(a.resource.clone()),
                        resource_type: Some(a.resource_type.clone()),
                        options: if a.options.is_empty() {
                            None
                        } else {
                            Some(a.options.clone())
                        },
                        resource_details: None,
                    })
                    .collect();
                let resp = wire::ListAssociatedResourcesResponse {
                    resources: Some(resources),
                    next_token: None,
                };
                wire::serialize_list_associated_resources_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    // ---- Tag handlers ----

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let resp = wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
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
            Err(e) => app_registry_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => app_registry_error_response(&e),
        }
    }

    // ---- Configuration handlers ----

    async fn handle_get_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let config = state.get_configuration();
        let resp = wire::GetConfigurationResponse {
            configuration: Some(wire::AppRegistryConfiguration {
                tag_query_configuration: Some(wire::TagQueryConfiguration {
                    tag_key: config.tag_key.clone(),
                }),
            }),
        };
        wire::serialize_get_configuration_response(&resp)
    }

    async fn handle_put_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tag_key = input
            .configuration
            .tag_query_configuration
            .as_ref()
            .and_then(|tqc| tqc.tag_key.clone());

        let mut state = state.write().await;
        state.put_configuration(tag_key);
        wire::serialize_put_configuration_response()
    }

    // ---- SyncResource ----

    // STUB[no-telemetry]: SyncResource synchronises real AWS resource tags with the
    //   application; the mock has no tag-propagation engine, so it always returns
    //   NO_ACTION without inspecting live resource state.
    async fn handle_sync_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogAppRegistryState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_sync_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_arn = format!(
            "arn:aws:{}:::{}",
            input.resource_type.to_lowercase(),
            input.resource
        );
        let resp = wire::SyncResourceResponse {
            application_arn: None,
            resource_arn: Some(resource_arn),
            action_taken: Some("NO_ACTION".to_string()),
        };
        let _ = state; // state not needed for a sync stub
        wire::serialize_sync_resource_response(&resp)
    }
}

/// Convert a state Application to a wire Application.
fn state_app_to_wire(app: &StateApplication) -> wire::Application {
    wire::Application {
        id: Some(app.id.clone()),
        arn: Some(app.arn.clone()),
        name: Some(app.name.clone()),
        description: app.description.clone(),
        creation_time: Some(app.creation_time.to_rfc3339()),
        last_update_time: Some(app.last_update_time.to_rfc3339()),
        tags: if app.tags.is_empty() {
            None
        } else {
            Some(app.tags.clone())
        },
        application_tag: None,
    }
}

/// Convert a state Application to a wire GetApplicationResponse.
fn state_app_to_get_response(app: &StateApplication) -> wire::GetApplicationResponse {
    wire::GetApplicationResponse {
        id: Some(app.id.clone()),
        arn: Some(app.arn.clone()),
        name: Some(app.name.clone()),
        description: app.description.clone(),
        creation_time: Some(app.creation_time.to_rfc3339()),
        last_update_time: Some(app.last_update_time.to_rfc3339()),
        tags: if app.tags.is_empty() {
            None
        } else {
            Some(app.tags.clone())
        },
        application_tag: None,
        associated_resource_count: None,
        integrations: None,
    }
}

/// Convert a state Application to a wire ApplicationSummary.
fn state_app_to_summary(app: &StateApplication) -> wire::ApplicationSummary {
    wire::ApplicationSummary {
        id: Some(app.id.clone()),
        arn: Some(app.arn.clone()),
        name: Some(app.name.clone()),
        description: app.description.clone(),
        creation_time: Some(app.creation_time.to_rfc3339()),
        last_update_time: Some(app.last_update_time.to_rfc3339()),
    }
}

/// Convert a state AttributeGroup to a wire AttributeGroup.
fn state_ag_to_wire(ag: &StateAttributeGroup) -> wire::AttributeGroup {
    wire::AttributeGroup {
        id: Some(ag.id.clone()),
        arn: Some(ag.arn.clone()),
        name: Some(ag.name.clone()),
        description: ag.description.clone(),
        creation_time: Some(ag.creation_time.to_rfc3339()),
        last_update_time: Some(ag.last_update_time.to_rfc3339()),
        tags: if ag.tags.is_empty() {
            None
        } else {
            Some(ag.tags.clone())
        },
    }
}

/// Convert a state AttributeGroup to a wire GetAttributeGroupResponse.
fn state_ag_to_get_response(ag: &StateAttributeGroup) -> wire::GetAttributeGroupResponse {
    wire::GetAttributeGroupResponse {
        id: Some(ag.id.clone()),
        arn: Some(ag.arn.clone()),
        name: Some(ag.name.clone()),
        description: ag.description.clone(),
        attributes: Some(ag.attributes.clone()),
        creation_time: Some(ag.creation_time.to_rfc3339()),
        last_update_time: Some(ag.last_update_time.to_rfc3339()),
        tags: if ag.tags.is_empty() {
            None
        } else {
            Some(ag.tags.clone())
        },
        created_by: None,
    }
}

/// Convert a state AttributeGroup to a wire AttributeGroupSummary.
fn state_ag_to_summary(ag: &StateAttributeGroup) -> wire::AttributeGroupSummary {
    wire::AttributeGroupSummary {
        id: Some(ag.id.clone()),
        arn: Some(ag.arn.clone()),
        name: Some(ag.name.clone()),
        description: ag.description.clone(),
        creation_time: Some(ag.creation_time.to_rfc3339()),
        last_update_time: Some(ag.last_update_time.to_rfc3339()),
        created_by: None,
    }
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
    if let Some(q) = uri.find('?') {
        uri[q + 1..].to_string()
    } else {
        String::new()
    }
}

fn parse_single_query(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if query.is_empty() {
        return map;
    }
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            map.insert(percent_decode(k), percent_decode(v));
        }
    }
    map
}

fn parse_multi_tag_keys(query: &str) -> Vec<String> {
    query
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            if key == "tagKeys" {
                Some(percent_decode(value))
            } else {
                None
            }
        })
        .collect()
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

fn app_registry_error_response(err: &AppRegistryError) -> MockResponse {
    let (status, error_type) = match err {
        AppRegistryError::ApplicationAlreadyExists(_) => (409, "ConflictException"),
        AppRegistryError::ApplicationNotFound(_) => (404, "ResourceNotFoundException"),
        AppRegistryError::AttributeGroupAlreadyExists(_) => (409, "ConflictException"),
        AppRegistryError::AttributeGroupNotFound(_) => (404, "ResourceNotFoundException"),
        AppRegistryError::ResourceAlreadyAssociated(_) => (409, "ConflictException"),
        AppRegistryError::ResourceNotAssociated(_, _) => (404, "ResourceNotFoundException"),
        AppRegistryError::ResourceArnNotFound(_) => (404, "ResourceNotFoundException"),
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
