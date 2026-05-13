use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path, extract_query_string, percent_decode, rest_json_error,
};

use crate::state::{AppConfigError, AppConfigState};
use crate::views::AppconfigStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AppConfigService {
    pub(crate) state: Arc<BackendState<AppConfigState>>,
    pub(crate) notifier: StateChangeNotifier<AppconfigStateView>,
}

impl AppConfigService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppConfigService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppConfigService {
    fn service_name(&self) -> &str {
        "appconfig"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://appconfig\..*\.amazonaws\.com",
            r"https?://appconfig\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AppConfigService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let query_string = extract_query_string(&request.uri);
        let query: HashMap<String, String> = winterbaume_core::parse_query_string(query_string);

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Routes:
        // POST   /applications                                                                          - CreateApplication
        // GET    /applications/{id}                                                                     - GetApplication
        // DELETE /applications/{id}                                                                     - DeleteApplication
        // PATCH  /applications/{id}                                                                     - UpdateApplication
        // GET    /applications                                                                          - ListApplications
        // POST   /applications/{AppId}/configurationprofiles                                            - CreateConfigurationProfile
        // GET    /applications/{AppId}/configurationprofiles/{ProfileId}                                - GetConfigurationProfile
        // DELETE /applications/{AppId}/configurationprofiles/{ProfileId}                                - DeleteConfigurationProfile
        // PATCH  /applications/{AppId}/configurationprofiles/{ProfileId}                                - UpdateConfigurationProfile
        // GET    /applications/{AppId}/configurationprofiles                                            - ListConfigurationProfiles
        // POST   /applications/{AppId}/configurationprofiles/{ProfileId}/hostedconfigurationversions    - CreateHostedConfigurationVersion
        // GET    /applications/{AppId}/configurationprofiles/{ProfileId}/hostedconfigurationversions/{V} - GetHostedConfigurationVersion
        // DELETE /applications/{AppId}/configurationprofiles/{ProfileId}/hostedconfigurationversions/{V} - DeleteHostedConfigurationVersion
        // GET    /tags/{ResourceArn}                                                                    - ListTagsForResource
        // POST   /tags/{ResourceArn}                                                                    - TagResource
        // DELETE /tags/{ResourceArn}                                                                    - UntagResource

        let response = match (method, segments.as_slice()) {
            // GET /settings => GetAccountSettings
            ("GET", ["settings"]) => self.handle_get_account_settings(&state).await,
            // PATCH /settings => UpdateAccountSettings
            ("PATCH", ["settings"]) => {
                self.handle_update_account_settings(&state, &request, &[], &query)
                    .await
            }
            // POST /deploymentstrategies => CreateDeploymentStrategy
            ("POST", ["deploymentstrategies"]) => {
                self.handle_create_deployment_strategy(&state, &request, &[], &query)
                    .await
            }
            // GET /deploymentstrategies => ListDeploymentStrategies
            ("GET", ["deploymentstrategies"]) => {
                self.handle_list_deployment_strategies(&state).await
            }
            // GET /deploymentstrategies/{id} => GetDeploymentStrategy
            ("GET", ["deploymentstrategies", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("DeploymentStrategyId", id.as_str())];
                self.handle_get_deployment_strategy(&state, &request, labels, &query)
                    .await
            }
            // DELETE /deployementstrategies/{id} => DeleteDeploymentStrategy (note typo in route)
            ("DELETE", ["deployementstrategies", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("DeploymentStrategyId", id.as_str())];
                self.handle_delete_deployment_strategy(&state, &request, labels, &query)
                    .await
            }
            // PATCH /deploymentstrategies/{id} => UpdateDeploymentStrategy
            ("PATCH", ["deploymentstrategies", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("DeploymentStrategyId", id.as_str())];
                self.handle_update_deployment_strategy(&state, &request, labels, &query)
                    .await
            }
            // POST /extensions => CreateExtension
            ("POST", ["extensions"]) => {
                self.handle_create_extension(&state, &request, &[], &query, &region, account_id)
                    .await
            }
            // GET /extensions => ListExtensions
            ("GET", ["extensions"]) => self.handle_list_extensions(&state).await,
            // GET /extensions/{id} => GetExtension
            ("GET", ["extensions", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ExtensionIdentifier", id.as_str())];
                self.handle_get_extension(&state, &request, labels, &query)
                    .await
            }
            // DELETE /extensions/{id} => DeleteExtension
            ("DELETE", ["extensions", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ExtensionIdentifier", id.as_str())];
                self.handle_delete_extension(&state, &request, labels, &query)
                    .await
            }
            // PATCH /extensions/{id} => UpdateExtension
            ("PATCH", ["extensions", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ExtensionIdentifier", id.as_str())];
                self.handle_update_extension(&state, &request, labels, &query)
                    .await
            }
            // POST /extensionassociations => CreateExtensionAssociation
            ("POST", ["extensionassociations"]) => {
                self.handle_create_extension_association(
                    &state,
                    &request,
                    &[],
                    &query,
                    &region,
                    account_id,
                )
                .await
            }
            // GET /extensionassociations => ListExtensionAssociations
            ("GET", ["extensionassociations"]) => {
                self.handle_list_extension_associations(&state).await
            }
            // GET /extensionassociations/{id} => GetExtensionAssociation
            ("GET", ["extensionassociations", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ExtensionAssociationId", id.as_str())];
                self.handle_get_extension_association(&state, &request, labels, &query)
                    .await
            }
            // DELETE /extensionassociations/{id} => DeleteExtensionAssociation
            ("DELETE", ["extensionassociations", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ExtensionAssociationId", id.as_str())];
                self.handle_delete_extension_association(&state, &request, labels, &query)
                    .await
            }
            // PATCH /extensionassociations/{id} => UpdateExtensionAssociation
            ("PATCH", ["extensionassociations", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ExtensionAssociationId", id.as_str())];
                self.handle_update_extension_association(&state, &request, labels, &query)
                    .await
            }
            // POST /applications => CreateApplication
            ("POST", ["applications"]) => {
                self.handle_create_application(&state, &request, &[], &query, &region, account_id)
                    .await
            }
            // GET /applications => ListApplications
            ("GET", ["applications"]) => self.handle_list_applications(&state).await,
            // GET /applications/{id} => GetApplication
            ("GET", ["applications", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ApplicationId", id.as_str())];
                self.handle_get_application(&state, &request, labels, &query)
                    .await
            }
            // DELETE /applications/{id} => DeleteApplication
            ("DELETE", ["applications", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ApplicationId", id.as_str())];
                self.handle_delete_application(&state, &request, labels, &query)
                    .await
            }
            // PATCH /applications/{id} => UpdateApplication
            ("PATCH", ["applications", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ApplicationId", id.as_str())];
                self.handle_update_application(&state, &request, labels, &query)
                    .await
            }
            // POST /applications/{AppId}/environments => CreateEnvironment
            ("POST", ["applications", app_id, "environments"]) => {
                let app_id = percent_decode(app_id);
                let labels: &[(&str, &str)] = &[("ApplicationId", app_id.as_str())];
                self.handle_create_environment(&state, &request, labels, &query)
                    .await
            }
            // GET /applications/{AppId}/environments => ListEnvironments
            ("GET", ["applications", app_id, "environments"]) => {
                let app_id = percent_decode(app_id);
                let labels: &[(&str, &str)] = &[("ApplicationId", app_id.as_str())];
                self.handle_list_environments(&state, &request, labels, &query)
                    .await
            }
            // GET /applications/{AppId}/environments/{EnvId} => GetEnvironment
            ("GET", ["applications", app_id, "environments", env_id]) => {
                let app_id = percent_decode(app_id);
                let env_id = percent_decode(env_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("EnvironmentId", env_id.as_str()),
                ];
                self.handle_get_environment(&state, &request, labels, &query)
                    .await
            }
            // DELETE /applications/{AppId}/environments/{EnvId} => DeleteEnvironment
            ("DELETE", ["applications", app_id, "environments", env_id]) => {
                let app_id = percent_decode(app_id);
                let env_id = percent_decode(env_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("EnvironmentId", env_id.as_str()),
                ];
                self.handle_delete_environment(&state, &request, labels, &query)
                    .await
            }
            // PATCH /applications/{AppId}/environments/{EnvId} => UpdateEnvironment
            ("PATCH", ["applications", app_id, "environments", env_id]) => {
                let app_id = percent_decode(app_id);
                let env_id = percent_decode(env_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("EnvironmentId", env_id.as_str()),
                ];
                self.handle_update_environment(&state, &request, labels, &query)
                    .await
            }
            // POST /applications/{AppId}/environments/{EnvId}/deployments => StartDeployment
            (
                "POST",
                [
                    "applications",
                    app_id,
                    "environments",
                    env_id,
                    "deployments",
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let env_id = percent_decode(env_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("EnvironmentId", env_id.as_str()),
                ];
                self.handle_start_deployment(&state, &request, labels, &query)
                    .await
            }
            // GET /applications/{AppId}/environments/{EnvId}/deployments => ListDeployments
            (
                "GET",
                [
                    "applications",
                    app_id,
                    "environments",
                    env_id,
                    "deployments",
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let env_id = percent_decode(env_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("EnvironmentId", env_id.as_str()),
                ];
                self.handle_list_deployments(&state, &request, labels, &query)
                    .await
            }
            // GET /applications/{AppId}/environments/{EnvId}/deployments/{DepNum} => GetDeployment
            (
                "GET",
                [
                    "applications",
                    app_id,
                    "environments",
                    env_id,
                    "deployments",
                    dep_num,
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let env_id = percent_decode(env_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("EnvironmentId", env_id.as_str()),
                    ("DeploymentNumber", dep_num),
                ];
                self.handle_get_deployment(&state, &request, labels, &query)
                    .await
            }
            // DELETE /applications/{AppId}/environments/{EnvId}/deployments/{DepNum} => StopDeployment
            (
                "DELETE",
                [
                    "applications",
                    app_id,
                    "environments",
                    env_id,
                    "deployments",
                    dep_num,
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let env_id = percent_decode(env_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("EnvironmentId", env_id.as_str()),
                    ("DeploymentNumber", dep_num),
                ];
                self.handle_stop_deployment(&state, &request, labels, &query)
                    .await
            }
            // POST /applications/{AppId}/configurationprofiles => CreateConfigurationProfile
            ("POST", ["applications", app_id, "configurationprofiles"]) => {
                let app_id = percent_decode(app_id);
                let labels: &[(&str, &str)] = &[("ApplicationId", app_id.as_str())];
                self.handle_create_configuration_profile(
                    &state, &request, labels, &query, &region, account_id,
                )
                .await
            }
            // GET /applications/{AppId}/configurationprofiles => ListConfigurationProfiles
            ("GET", ["applications", app_id, "configurationprofiles"]) => {
                let app_id = percent_decode(app_id);
                let labels: &[(&str, &str)] = &[("ApplicationId", app_id.as_str())];
                self.handle_list_configuration_profiles(&state, &request, labels, &query)
                    .await
            }
            // GET /applications/{AppId}/configurationprofiles/{ProfileId} => GetConfigurationProfile
            ("GET", ["applications", app_id, "configurationprofiles", profile_id]) => {
                let app_id = percent_decode(app_id);
                let profile_id = percent_decode(profile_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("ConfigurationProfileId", profile_id.as_str()),
                ];
                self.handle_get_configuration_profile(&state, &request, labels, &query)
                    .await
            }
            // DELETE /applications/{AppId}/configurationprofiles/{ProfileId} => DeleteConfigurationProfile
            ("DELETE", ["applications", app_id, "configurationprofiles", profile_id]) => {
                let app_id = percent_decode(app_id);
                let profile_id = percent_decode(profile_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("ConfigurationProfileId", profile_id.as_str()),
                ];
                self.handle_delete_configuration_profile(&state, &request, labels, &query)
                    .await
            }
            // PATCH /applications/{AppId}/configurationprofiles/{ProfileId} => UpdateConfigurationProfile
            ("PATCH", ["applications", app_id, "configurationprofiles", profile_id]) => {
                let app_id = percent_decode(app_id);
                let profile_id = percent_decode(profile_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("ConfigurationProfileId", profile_id.as_str()),
                ];
                self.handle_update_configuration_profile(&state, &request, labels, &query)
                    .await
            }
            // GET /applications/{App}/environments/{Env}/configurations/{Config} => GetConfiguration (deprecated)
            (
                "GET",
                [
                    "applications",
                    application,
                    "environments",
                    environment,
                    "configurations",
                    _configuration,
                ],
            ) => {
                let application = percent_decode(application);
                let environment = percent_decode(environment);
                self.handle_get_configuration(&state, &application, &environment)
                    .await
            }
            // GET /applications/{AppId}/configurationprofiles/{ProfileId}/hostedconfigurationversions => ListHostedConfigurationVersions
            (
                "GET",
                [
                    "applications",
                    app_id,
                    "configurationprofiles",
                    profile_id,
                    "hostedconfigurationversions",
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let profile_id = percent_decode(profile_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("ConfigurationProfileId", profile_id.as_str()),
                ];
                self.handle_list_hosted_configuration_versions(&state, &request, labels, &query)
                    .await
            }
            // POST /applications/{AppId}/configurationprofiles/{ProfileId}/validators => ValidateConfiguration
            (
                "POST",
                [
                    "applications",
                    _app_id,
                    "configurationprofiles",
                    _profile_id,
                    "validators",
                ],
            ) => self.handle_validate_configuration().await,
            // POST /applications/{AppId}/configurationprofiles/{ProfileId}/hostedconfigurationversions => CreateHostedConfigurationVersion
            (
                "POST",
                [
                    "applications",
                    app_id,
                    "configurationprofiles",
                    profile_id,
                    "hostedconfigurationversions",
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let profile_id = percent_decode(profile_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("ConfigurationProfileId", profile_id.as_str()),
                ];
                self.handle_create_hosted_configuration_version(&state, &request, labels, &query)
                    .await
            }
            // GET /applications/{AppId}/configurationprofiles/{ProfileId}/hostedconfigurationversions/{V} => GetHostedConfigurationVersion
            (
                "GET",
                [
                    "applications",
                    app_id,
                    "configurationprofiles",
                    profile_id,
                    "hostedconfigurationversions",
                    version,
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let profile_id = percent_decode(profile_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("ConfigurationProfileId", profile_id.as_str()),
                    ("VersionNumber", version),
                ];
                self.handle_get_hosted_configuration_version(&state, &request, labels, &query)
                    .await
            }
            // DELETE /applications/{AppId}/configurationprofiles/{ProfileId}/hostedconfigurationversions/{V} => DeleteHostedConfigurationVersion
            (
                "DELETE",
                [
                    "applications",
                    app_id,
                    "configurationprofiles",
                    profile_id,
                    "hostedconfigurationversions",
                    version,
                ],
            ) => {
                let app_id = percent_decode(app_id);
                let profile_id = percent_decode(profile_id);
                let labels: &[(&str, &str)] = &[
                    ("ApplicationId", app_id.as_str()),
                    ("ConfigurationProfileId", profile_id.as_str()),
                    ("VersionNumber", version),
                ];
                self.handle_delete_hosted_configuration_version(&state, &request, labels, &query)
                    .await
            }
            // POST /tags/{ResourceArn+} => TagResource
            ("POST", ["tags", ..]) if segments.len() >= 2 => {
                let resource_arn = percent_decode(&segments[1..].join("/"));
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query)
                    .await
            }
            // DELETE /tags/{ResourceArn+} => UntagResource (tag keys come from query string)
            ("DELETE", ["tags", ..]) if segments.len() >= 2 => {
                let resource_arn = percent_decode(&segments[1..].join("/"));
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query, query_string)
                    .await
            }
            // GET /tags/{ResourceArn+} => ListTagsForResource
            ("GET", ["tags", ..]) if segments.len() >= 2 => {
                let resource_arn = percent_decode(&segments[1..].join("/"));
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status >= 200 && response.status < 300 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ---- Application handlers ----

    async fn handle_create_application(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Name is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_application(&input.name, description) {
            Ok(app) => {
                let app_id = app.id.clone();
                let app_name = app.name.clone();
                let app_desc = app.description.clone();
                if !tags.is_empty() {
                    let arn =
                        format!("arn:aws:appconfig:{region}:{account_id}:application/{app_id}");
                    state.tag_resource(&arn, tags);
                }
                wire::serialize_create_application_response(&wire::Application {
                    id: Some(app_id),
                    name: Some(app_name),
                    description: Some(app_desc),
                    ..Default::default()
                })
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_application(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_application(&input.application_id) {
            Ok(app) => wire::serialize_get_application_response(&wire::Application {
                id: Some(app.id.clone()),
                name: Some(app.name.clone()),
                description: Some(app.description.clone()),
                ..Default::default()
            }),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_delete_application(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_application(&input.application_id) {
            Ok(()) => wire::serialize_delete_application_response(),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_list_applications(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let apps = state.list_applications();
        let entries: Vec<wire::Application> = apps
            .iter()
            .map(|a| wire::Application {
                id: Some(a.id.clone()),
                name: Some(a.name.clone()),
                description: Some(a.description.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_applications_response(&wire::Applications {
            items: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_update_application(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };

        let mut state = state.write().await;
        match state.update_application(
            &input.application_id,
            input.name.as_deref(),
            input.description.as_deref(),
        ) {
            Ok(app) => wire::serialize_update_application_response(&wire::Application {
                id: Some(app.id.clone()),
                name: Some(app.name.clone()),
                description: Some(app.description.clone()),
                ..Default::default()
            }),
            Err(e) => appconfig_error_response(&e),
        }
    }

    // ---- ConfigurationProfile handlers ----

    async fn handle_create_configuration_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_configuration_profile_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Name is required");
        }
        if input.location_uri.is_empty() {
            return rest_json_error(400, "BadRequestException", "LocationUri is required");
        }
        let application_id = input.application_id.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let profile_type = input.r#type.as_deref().unwrap_or("AWS.Freeform");
        let retrieval_role_arn = input.retrieval_role_arn.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_configuration_profile(
            application_id,
            &input.name,
            &input.location_uri,
            description,
            profile_type,
            retrieval_role_arn,
        ) {
            Ok(profile) => {
                let profile_id = profile.id.clone();
                let app_id = profile.application_id.clone();
                let resp = wire::serialize_create_configuration_profile_response(
                    &wire::ConfigurationProfile {
                        application_id: Some(app_id.clone()),
                        id: Some(profile_id.clone()),
                        name: Some(profile.name.clone()),
                        description: Some(profile.description.clone()),
                        location_uri: Some(profile.location_uri.clone()),
                        r#type: Some(profile.r#type.clone()),
                        retrieval_role_arn: profile.retrieval_role_arn.clone(),
                        ..Default::default()
                    },
                );
                // Store tags at creation time keyed by ARN
                if !tags.is_empty() {
                    let arn = format!(
                        "arn:aws:appconfig:{region}:{account_id}:application/{app_id}/configurationprofile/{profile_id}"
                    );
                    state.tag_resource(&arn, tags);
                }
                resp
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_configuration_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_configuration_profile_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let state = state.read().await;
        match state
            .get_configuration_profile(&input.application_id, &input.configuration_profile_id)
        {
            Ok(profile) => {
                wire::serialize_get_configuration_profile_response(&wire::ConfigurationProfile {
                    application_id: Some(profile.application_id.clone()),
                    id: Some(profile.id.clone()),
                    name: Some(profile.name.clone()),
                    description: Some(profile.description.clone()),
                    location_uri: Some(profile.location_uri.clone()),
                    r#type: Some(profile.r#type.clone()),
                    retrieval_role_arn: profile.retrieval_role_arn.clone(),
                    ..Default::default()
                })
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_delete_configuration_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_configuration_profile_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let mut state = state.write().await;
        match state
            .delete_configuration_profile(&input.application_id, &input.configuration_profile_id)
        {
            Ok(()) => wire::serialize_delete_configuration_profile_response(),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_list_configuration_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_configuration_profiles_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let state = state.read().await;
        let profiles = state.list_configuration_profiles(&input.application_id);
        let entries: Vec<wire::ConfigurationProfileSummary> = profiles
            .iter()
            .map(|p| wire::ConfigurationProfileSummary {
                application_id: Some(p.application_id.clone()),
                id: Some(p.id.clone()),
                name: Some(p.name.clone()),
                location_uri: Some(p.location_uri.clone()),
                r#type: Some(p.r#type.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_configuration_profiles_response(&wire::ConfigurationProfiles {
            items: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_update_configuration_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_configuration_profile_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let mut state = state.write().await;
        match state.update_configuration_profile(
            &input.application_id,
            &input.configuration_profile_id,
            input.name.as_deref(),
            input.description.as_deref(),
            input.retrieval_role_arn.as_deref(),
        ) {
            Ok(profile) => {
                wire::serialize_update_configuration_profile_response(&wire::ConfigurationProfile {
                    application_id: Some(profile.application_id.clone()),
                    id: Some(profile.id.clone()),
                    name: Some(profile.name.clone()),
                    description: Some(profile.description.clone()),
                    location_uri: Some(profile.location_uri.clone()),
                    r#type: Some(profile.r#type.clone()),
                    retrieval_role_arn: profile.retrieval_role_arn.clone(),
                    ..Default::default()
                })
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    // ---- HostedConfigurationVersion handlers ----

    async fn handle_create_hosted_configuration_version(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_hosted_configuration_version_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let content_type = if input.content_type.is_empty() {
            "application/json".to_string()
        } else {
            input.content_type.clone()
        };
        let description = input.description.unwrap_or_default();
        let mut state = state.write().await;
        match state.create_hosted_configuration_version(
            &input.application_id,
            &input.configuration_profile_id,
            &content_type,
            &description,
        ) {
            Ok(version) => {
                let mut resp = wire::serialize_create_hosted_configuration_version_response(
                    &wire::HostedConfigurationVersion {
                        application_id: Some(version.application_id.clone()),
                        configuration_profile_id: Some(version.configuration_profile_id.clone()),
                        version_number: Some(version.version_number),
                        content_type: Some(version.content_type.clone()),
                        description: Some(version.description.clone()),
                        ..Default::default()
                    },
                );
                set_hosted_version_headers(&mut resp, version);
                resp
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_hosted_configuration_version(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_hosted_configuration_version_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_hosted_configuration_version(
            &input.application_id,
            &input.configuration_profile_id,
            input.version_number,
        ) {
            Ok(version) => {
                let mut resp = wire::serialize_get_hosted_configuration_version_response(
                    &wire::HostedConfigurationVersion {
                        application_id: Some(version.application_id.clone()),
                        configuration_profile_id: Some(version.configuration_profile_id.clone()),
                        version_number: Some(version.version_number),
                        content_type: Some(version.content_type.clone()),
                        description: Some(version.description.clone()),
                        ..Default::default()
                    },
                );
                set_hosted_version_headers(&mut resp, version);
                resp
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_delete_hosted_configuration_version(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_hosted_configuration_version_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_hosted_configuration_version(
            &input.application_id,
            &input.configuration_profile_id,
            input.version_number,
        ) {
            Ok(()) => wire::serialize_delete_hosted_configuration_version_response(),
            Err(e) => appconfig_error_response(&e),
        }
    }

    // ---- Tag handlers ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "BadRequestException", "Tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        query_string: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        // AWS SDKs serialise the `tagKeys` httpQuery as repeated params
        // (`?tagKeys=a&tagKeys=b`), but the wire deserializer expects a
        // comma-separated single value. Re-parse from the raw query string so
        // multi-key untag requests are not silently truncated.
        let mut tag_keys: Vec<String> = query_string
            .split('&')
            .filter_map(|pair| {
                let (key, value) = pair.split_once('=')?;
                if key == "tagKeys" {
                    Some(percent_decode(value))
                } else {
                    None
                }
            })
            .collect();
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys;
        }
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &tag_keys);
        wire::serialize_untag_resource_response()
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ResourceTags {
            tags: Some(tags),
            ..Default::default()
        })
    }

    // ---- AccountSettings handlers ----

    async fn handle_get_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let settings = state.get_account_settings();
        wire::serialize_get_account_settings_response(&wire::AccountSettings {
            deletion_protection: settings.deletion_protection_enabled.map(|enabled| {
                wire::DeletionProtectionSettings {
                    enabled: Some(enabled),
                    protection_period_in_minutes: settings.deletion_protection_period_in_minutes,
                }
            }),
        })
    }

    async fn handle_update_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_account_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let enabled = input.deletion_protection.as_ref().and_then(|d| d.enabled);
        let period = input
            .deletion_protection
            .as_ref()
            .and_then(|d| d.protection_period_in_minutes);
        let mut state = state.write().await;
        let settings = state.update_account_settings(enabled, period);
        wire::serialize_update_account_settings_response(&wire::AccountSettings {
            deletion_protection: settings.deletion_protection_enabled.map(|e| {
                wire::DeletionProtectionSettings {
                    enabled: Some(e),
                    protection_period_in_minutes: settings.deletion_protection_period_in_minutes,
                }
            }),
        })
    }

    // ---- DeploymentStrategy handlers ----

    async fn handle_create_deployment_strategy(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_deployment_strategy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Name is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let final_bake = input.final_bake_time_in_minutes.unwrap_or(0);
        let growth_type = input.growth_type.as_deref().unwrap_or("LINEAR");
        let replicate_to = input.replicate_to.as_deref().unwrap_or("NONE");
        let mut state = state.write().await;
        match state.create_deployment_strategy(
            &input.name,
            description,
            input.deployment_duration_in_minutes,
            final_bake,
            input.growth_factor,
            growth_type,
            replicate_to,
        ) {
            Ok(ds) => {
                wire::serialize_create_deployment_strategy_response(&wire::DeploymentStrategy {
                    id: Some(ds.id.clone()),
                    name: Some(ds.name.clone()),
                    description: Some(ds.description.clone()),
                    deployment_duration_in_minutes: Some(ds.deployment_duration_in_minutes),
                    final_bake_time_in_minutes: Some(ds.final_bake_time_in_minutes),
                    growth_factor: Some(ds.growth_factor),
                    growth_type: Some(ds.growth_type.clone()),
                    replicate_to: Some(ds.replicate_to.clone()),
                })
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_deployment_strategy(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployment_strategy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_deployment_strategy(&input.deployment_strategy_id) {
            Ok(ds) => wire::serialize_get_deployment_strategy_response(&wire::DeploymentStrategy {
                id: Some(ds.id.clone()),
                name: Some(ds.name.clone()),
                description: Some(ds.description.clone()),
                deployment_duration_in_minutes: Some(ds.deployment_duration_in_minutes),
                final_bake_time_in_minutes: Some(ds.final_bake_time_in_minutes),
                growth_factor: Some(ds.growth_factor),
                growth_type: Some(ds.growth_type.clone()),
                replicate_to: Some(ds.replicate_to.clone()),
            }),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_delete_deployment_strategy(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_deployment_strategy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let mut state = state.write().await;
        match state.delete_deployment_strategy(&input.deployment_strategy_id) {
            Ok(()) => wire::serialize_delete_deployment_strategy_response(),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_list_deployment_strategies(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::DeploymentStrategy> = state
            .list_deployment_strategies()
            .iter()
            .map(|ds| wire::DeploymentStrategy {
                id: Some(ds.id.clone()),
                name: Some(ds.name.clone()),
                description: Some(ds.description.clone()),
                deployment_duration_in_minutes: Some(ds.deployment_duration_in_minutes),
                final_bake_time_in_minutes: Some(ds.final_bake_time_in_minutes),
                growth_factor: Some(ds.growth_factor),
                growth_type: Some(ds.growth_type.clone()),
                replicate_to: Some(ds.replicate_to.clone()),
            })
            .collect();
        wire::serialize_list_deployment_strategies_response(&wire::DeploymentStrategies {
            items: Some(items),
            ..Default::default()
        })
    }

    async fn handle_update_deployment_strategy(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_deployment_strategy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let mut state = state.write().await;
        match state.update_deployment_strategy(
            &input.deployment_strategy_id,
            None, // Name is not a field of UpdateDeploymentStrategyRequest in Smithy
            input.description.as_deref(),
            input.deployment_duration_in_minutes,
            input.final_bake_time_in_minutes,
            input.growth_factor,
            input.growth_type.as_deref(),
        ) {
            Ok(ds) => {
                wire::serialize_update_deployment_strategy_response(&wire::DeploymentStrategy {
                    id: Some(ds.id.clone()),
                    name: Some(ds.name.clone()),
                    description: Some(ds.description.clone()),
                    deployment_duration_in_minutes: Some(ds.deployment_duration_in_minutes),
                    final_bake_time_in_minutes: Some(ds.final_bake_time_in_minutes),
                    growth_factor: Some(ds.growth_factor),
                    growth_type: Some(ds.growth_type.clone()),
                    replicate_to: Some(ds.replicate_to.clone()),
                })
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    // ---- Environment handlers ----

    async fn handle_create_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_environment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Name is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let monitors: Vec<crate::types::MonitorData> = input
            .monitors
            .unwrap_or_default()
            .into_iter()
            .map(|m| crate::types::MonitorData {
                alarm_arn: m.alarm_arn,
                alarm_role_arn: m.alarm_role_arn,
            })
            .collect();
        let mut state = state.write().await;
        match state.create_environment(&input.application_id, &input.name, description, monitors) {
            Ok(env) => wire::serialize_create_environment_response(&env_to_wire(env)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_environment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_environment(&input.application_id, &input.environment_id) {
            Ok(env) => wire::serialize_get_environment_response(&env_to_wire(env)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_delete_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_environment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_environment(&input.application_id, &input.environment_id) {
            Ok(()) => wire::serialize_delete_environment_response(),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_list_environments(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_environments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        let items: Vec<wire::Environment> = state
            .list_environments(&input.application_id)
            .iter()
            .map(|e| env_to_wire(e))
            .collect();
        wire::serialize_list_environments_response(&wire::Environments {
            items: Some(items),
            ..Default::default()
        })
    }

    async fn handle_update_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_environment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let monitors = input.monitors.map(|ms| {
            ms.into_iter()
                .map(|m| crate::types::MonitorData {
                    alarm_arn: m.alarm_arn,
                    alarm_role_arn: m.alarm_role_arn,
                })
                .collect::<Vec<_>>()
        });
        let mut state = state.write().await;
        match state.update_environment(
            &input.application_id,
            &input.environment_id,
            input.name.as_deref(),
            input.description.as_deref(),
            monitors,
        ) {
            Ok(env) => wire::serialize_update_environment_response(&env_to_wire(env)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    // ---- Deployment handlers ----

    async fn handle_start_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.deployment_strategy_id.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "DeploymentStrategyId is required",
            );
        }
        if input.configuration_profile_id.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "ConfigurationProfileId is required",
            );
        }
        if input.configuration_version.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "ConfigurationVersion is required",
            );
        }
        let description = input.description.as_deref().unwrap_or("");
        let mut state = state.write().await;
        match state.start_deployment(
            &input.application_id,
            &input.environment_id,
            &input.deployment_strategy_id,
            &input.configuration_profile_id,
            &input.configuration_version,
            description,
        ) {
            Ok(dep) => wire::serialize_start_deployment_response(&dep_to_wire(dep)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_deployment(
            &input.application_id,
            &input.environment_id,
            input.deployment_number,
        ) {
            Ok(dep) => wire::serialize_get_deployment_response(&dep_to_wire(dep)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_stop_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.stop_deployment(
            &input.application_id,
            &input.environment_id,
            input.deployment_number,
        ) {
            Ok(dep) => wire::serialize_stop_deployment_response(&dep_to_wire(dep)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_list_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_deployments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        let items: Vec<wire::DeploymentSummary> = state
            .list_deployments(&input.application_id, &input.environment_id)
            .iter()
            .map(|d| wire::DeploymentSummary {
                deployment_number: Some(d.deployment_number),
                configuration_version: Some(d.configuration_version.clone()),
                state: Some(d.state.clone()),
                started_at: Some(d.started_at.clone()),
                completed_at: Some(d.completed_at.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_deployments_response(&wire::Deployments {
            items: Some(items),
            ..Default::default()
        })
    }

    // ---- Extension handlers ----

    async fn handle_create_extension(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_extension_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Name is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let actions: HashMap<String, Vec<crate::types::ActionData>> = input
            .actions
            .into_iter()
            .map(|(k, acts)| {
                let v: Vec<crate::types::ActionData> = acts
                    .into_iter()
                    .map(|a| crate::types::ActionData {
                        name: a.name,
                        description: a.description,
                        uri: a.uri,
                        role_arn: a.role_arn,
                    })
                    .collect();
                (k, v)
            })
            .collect();
        let parameters: HashMap<String, crate::types::ParameterData> = input
            .parameters
            .unwrap_or_default()
            .into_iter()
            .map(|(k, p)| {
                (
                    k,
                    crate::types::ParameterData {
                        description: p.description,
                        required: p.required,
                        dynamic: p.dynamic,
                    },
                )
            })
            .collect();
        let mut state = state.write().await;
        match state.create_extension(
            &input.name,
            description,
            actions,
            parameters,
            region,
            account_id,
        ) {
            Ok(ext) => wire::serialize_create_extension_response(&ext_to_wire(ext)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_extension(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_extension_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.get_extension(&input.extension_identifier) {
            Ok(ext) => wire::serialize_get_extension_response(&ext_to_wire(ext)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_delete_extension(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_extension_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_extension(&input.extension_identifier) {
            Ok(()) => wire::serialize_delete_extension_response(),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_list_extensions(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::ExtensionSummary> = state
            .list_extensions()
            .iter()
            .map(|e| wire::ExtensionSummary {
                id: Some(e.id.clone()),
                name: Some(e.name.clone()),
                description: Some(e.description.clone()),
                version_number: Some(e.version_number),
                arn: Some(e.arn.clone()),
            })
            .collect();
        wire::serialize_list_extensions_response(&wire::Extensions {
            items: Some(items),
            ..Default::default()
        })
    }

    async fn handle_update_extension(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_extension_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let actions = input.actions.map(|m| {
            m.into_iter()
                .map(|(k, acts)| {
                    let v: Vec<crate::types::ActionData> = acts
                        .into_iter()
                        .map(|a| crate::types::ActionData {
                            name: a.name,
                            description: a.description,
                            uri: a.uri,
                            role_arn: a.role_arn,
                        })
                        .collect();
                    (k, v)
                })
                .collect::<HashMap<String, Vec<crate::types::ActionData>>>()
        });
        let parameters = input.parameters.map(|m| {
            m.into_iter()
                .map(|(k, p)| {
                    (
                        k,
                        crate::types::ParameterData {
                            description: p.description,
                            required: p.required,
                            dynamic: p.dynamic,
                        },
                    )
                })
                .collect::<HashMap<String, crate::types::ParameterData>>()
        });
        let mut state = state.write().await;
        match state.update_extension(
            &input.extension_identifier,
            input.description.as_deref(),
            actions,
            parameters,
        ) {
            Ok(ext) => wire::serialize_update_extension_response(&ext_to_wire(ext)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    // ---- ExtensionAssociation handlers ----

    async fn handle_create_extension_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_extension_association_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        if input.extension_identifier.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "ExtensionIdentifier is required",
            );
        }
        if input.resource_identifier.is_empty() {
            return rest_json_error(400, "BadRequestException", "ResourceIdentifier is required");
        }
        let ext_version = input.extension_version_number.unwrap_or(1);
        let parameters = input.parameters.unwrap_or_default();
        // Resolve extension ARN
        let ext_arn = {
            let state_r = state.read().await;
            match state_r.get_extension(&input.extension_identifier) {
                Ok(ext) => ext.arn.clone(),
                Err(e) => return appconfig_error_response(&e),
            }
        };
        let mut state = state.write().await;
        match state.create_extension_association(
            &ext_arn,
            &input.resource_identifier,
            ext_version,
            parameters,
            region,
            account_id,
        ) {
            Ok(assoc) => {
                wire::serialize_create_extension_association_response(&assoc_to_wire(assoc))
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_get_extension_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_extension_association_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let state = state.read().await;
        match state.get_extension_association(&input.extension_association_id) {
            Ok(assoc) => wire::serialize_get_extension_association_response(&assoc_to_wire(assoc)),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_delete_extension_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_extension_association_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let mut state = state.write().await;
        match state.delete_extension_association(&input.extension_association_id) {
            Ok(()) => wire::serialize_delete_extension_association_response(),
            Err(e) => appconfig_error_response(&e),
        }
    }

    async fn handle_list_extension_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::ExtensionAssociationSummary> = state
            .list_extension_associations()
            .iter()
            .map(|a| wire::ExtensionAssociationSummary {
                id: Some(a.id.clone()),
                extension_arn: Some(a.extension_arn.clone()),
                resource_arn: Some(a.resource_arn.clone()),
            })
            .collect();
        wire::serialize_list_extension_associations_response(&wire::ExtensionAssociations {
            items: Some(items),
            ..Default::default()
        })
    }

    async fn handle_update_extension_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_extension_association_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let mut state = state.write().await;
        match state.update_extension_association(&input.extension_association_id, input.parameters)
        {
            Ok(assoc) => {
                wire::serialize_update_extension_association_response(&assoc_to_wire(assoc))
            }
            Err(e) => appconfig_error_response(&e),
        }
    }

    // ---- GetConfiguration (deprecated) handler ----

    async fn handle_get_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        _application: &str,
        _environment: &str,
    ) -> MockResponse {
        // Deprecated API - return empty configuration
        let mut resp = wire::serialize_get_configuration_response(&wire::Configuration {
            configuration_version: Some("1".to_string()),
            content: Some("".to_string()),
            content_type: Some("application/json".to_string()),
        });
        resp.headers.insert(
            HeaderName::from_static("configuration-version"),
            "1".parse().unwrap(),
        );
        resp.headers.insert(
            http::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        resp
    }

    // ---- ListHostedConfigurationVersions handler ----

    async fn handle_list_hosted_configuration_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<AppConfigState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_hosted_configuration_versions_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        let versions = state.list_hosted_configuration_versions(
            &input.application_id,
            &input.configuration_profile_id,
        );
        let items: Vec<wire::HostedConfigurationVersionSummary> = versions
            .iter()
            .map(|v| wire::HostedConfigurationVersionSummary {
                application_id: Some(v.application_id.clone()),
                configuration_profile_id: Some(v.configuration_profile_id.clone()),
                version_number: Some(v.version_number),
                content_type: Some(v.content_type.clone()),
                description: Some(v.description.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_hosted_configuration_versions_response(
            &wire::HostedConfigurationVersions {
                items: Some(items),
                ..Default::default()
            },
        )
    }

    async fn handle_validate_configuration(&self) -> MockResponse {
        wire::serialize_validate_configuration_response()
    }
}

fn set_hosted_version_headers(
    resp: &mut MockResponse,
    version: &crate::types::HostedConfigurationVersionData,
) {
    resp.headers.insert(
        HeaderName::from_static("application-id"),
        version.application_id.parse().unwrap(),
    );
    resp.headers.insert(
        HeaderName::from_static("configuration-profile-id"),
        version.configuration_profile_id.parse().unwrap(),
    );
    resp.headers.insert(
        HeaderName::from_static("version-number"),
        version.version_number.to_string().parse().unwrap(),
    );
    resp.headers.insert(
        http::header::CONTENT_TYPE,
        version.content_type.parse().unwrap(),
    );
    if !version.description.is_empty() {
        resp.headers.insert(
            HeaderName::from_static("description"),
            version.description.parse().unwrap(),
        );
    }
}

fn appconfig_error_response(err: &AppConfigError) -> MockResponse {
    let (status, error_type) = match err {
        AppConfigError::ApplicationNotFound(_) => (404u16, "ResourceNotFoundException"),
        AppConfigError::ConfigurationProfileNotFound(_) => (404, "ResourceNotFoundException"),
        AppConfigError::HostedConfigurationVersionNotFound(_) => (404, "ResourceNotFoundException"),
        AppConfigError::DeploymentStrategyNotFound(_) => (404, "ResourceNotFoundException"),
        AppConfigError::EnvironmentNotFound(_) => (404, "ResourceNotFoundException"),
        AppConfigError::DeploymentNotFound(_) => (404, "ResourceNotFoundException"),
        AppConfigError::ExtensionNotFound(_) => (404, "ResourceNotFoundException"),
        AppConfigError::ExtensionAssociationNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let body = json!({ "Message": err.to_string() });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn env_to_wire(env: &crate::types::EnvironmentData) -> wire::Environment {
    wire::Environment {
        id: Some(env.id.clone()),
        application_id: Some(env.application_id.clone()),
        name: Some(env.name.clone()),
        description: Some(env.description.clone()),
        state: Some(env.state.clone()),
        monitors: if env.monitors.is_empty() {
            None
        } else {
            Some(
                env.monitors
                    .iter()
                    .map(|m| wire::Monitor {
                        alarm_arn: m.alarm_arn.clone(),
                        alarm_role_arn: m.alarm_role_arn.clone(),
                    })
                    .collect(),
            )
        },
    }
}

fn dep_to_wire(dep: &crate::types::DeploymentData) -> wire::Deployment {
    wire::Deployment {
        application_id: Some(dep.application_id.clone()),
        environment_id: Some(dep.environment_id.clone()),
        deployment_strategy_id: Some(dep.deployment_strategy_id.clone()),
        configuration_profile_id: Some(dep.configuration_profile_id.clone()),
        configuration_version: Some(dep.configuration_version.clone()),
        deployment_number: Some(dep.deployment_number),
        state: Some(dep.state.clone()),
        started_at: Some(dep.started_at.clone()),
        completed_at: Some(dep.completed_at.clone()),
        ..Default::default()
    }
}

fn ext_to_wire(ext: &crate::types::ExtensionData) -> wire::Extension {
    wire::Extension {
        id: Some(ext.id.clone()),
        name: Some(ext.name.clone()),
        description: Some(ext.description.clone()),
        version_number: Some(ext.version_number),
        arn: Some(ext.arn.clone()),
        actions: if ext.actions.is_empty() {
            None
        } else {
            Some(
                ext.actions
                    .iter()
                    .map(|(k, acts)| {
                        (
                            k.clone(),
                            acts.iter()
                                .map(|a| wire::Action {
                                    name: a.name.clone(),
                                    description: a.description.clone(),
                                    uri: a.uri.clone(),
                                    role_arn: a.role_arn.clone(),
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            )
        },
        parameters: if ext.parameters.is_empty() {
            None
        } else {
            Some(
                ext.parameters
                    .iter()
                    .map(|(k, p)| {
                        (
                            k.clone(),
                            wire::Parameter {
                                description: p.description.clone(),
                                required: p.required,
                                dynamic: p.dynamic,
                            },
                        )
                    })
                    .collect(),
            )
        },
    }
}

fn assoc_to_wire(assoc: &crate::types::ExtensionAssociationData) -> wire::ExtensionAssociation {
    wire::ExtensionAssociation {
        id: Some(assoc.id.clone()),
        arn: Some(assoc.arn.clone()),
        extension_arn: Some(assoc.extension_arn.clone()),
        resource_arn: Some(assoc.resource_arn.clone()),
        extension_version_number: Some(assoc.extension_version_number),
        parameters: if assoc.parameters.is_empty() {
            None
        } else {
            Some(assoc.parameters.clone())
        },
    }
}
