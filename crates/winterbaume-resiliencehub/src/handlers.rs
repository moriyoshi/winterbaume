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

use crate::state::{ResilienceHubError, ResilienceHubState};
use crate::types::FailurePolicyData;
use crate::views::ResilienceHubStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ResilienceHubService {
    pub(crate) state: Arc<BackendState<ResilienceHubState>>,
    pub(crate) notifier: StateChangeNotifier<ResilienceHubStateView>,
}

impl ResilienceHubService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ResilienceHubService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ResilienceHubService {
    fn service_name(&self) -> &str {
        "resiliencehub"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://resiliencehub\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ResilienceHubService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        // Handle /tags/{resourceArn} routes
        if let Some(rest) = path.strip_prefix("/tags/") {
            let resource_arn = percent_decode(rest);
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
                    self.handle_untag_resource(&state, &request, labels, &query_map)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
            if response.status / 100 == 2 {
                self.notify_state_changed(account_id, &region).await;
            }
            return response;
        }

        let response = match (method, path.as_str()) {
            // POST /create-app - CreateApp
            ("POST", "/create-app") => {
                self.handle_create_app(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // POST /describe-app - DescribeApp
            ("POST", "/describe-app") => {
                self.handle_describe_app(&state, &request, &[], &query_map)
                    .await
            }
            // POST /delete-app - DeleteApp
            ("POST", "/delete-app") => {
                self.handle_delete_app(&state, &request, &[], &query_map)
                    .await
            }
            // GET /list-apps - ListApps
            ("GET", "/list-apps") => {
                self.handle_list_apps(&state, &request, &[], &query_map)
                    .await
            }

            // POST /create-resiliency-policy
            ("POST", "/create-resiliency-policy") => {
                self.handle_create_resiliency_policy(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // POST /delete-resiliency-policy
            ("POST", "/delete-resiliency-policy") => {
                self.handle_delete_resiliency_policy(&state, &request, &[], &query_map)
                    .await
            }
            // POST /describe-resiliency-policy
            ("POST", "/describe-resiliency-policy") => {
                self.handle_describe_resiliency_policy(&state, &request, &[], &query_map)
                    .await
            }
            // GET /list-resiliency-policies
            ("GET", "/list-resiliency-policies") => {
                self.handle_list_resiliency_policies(&state).await
            }
            // GET /list-suggested-resiliency-policies
            ("GET", "/list-suggested-resiliency-policies") => {
                self.handle_list_suggested_resiliency_policies(&state).await
            }
            // POST /update-resiliency-policy
            ("POST", "/update-resiliency-policy") => {
                self.handle_update_resiliency_policy(&state, &request, &[], &query_map)
                    .await
            }
            // POST /publish-app-version
            ("POST", "/publish-app-version") => {
                self.handle_publish_app_version(&state, &request, &[], &query_map)
                    .await
            }
            // POST /list-app-versions
            ("POST", "/list-app-versions") => {
                self.handle_list_app_versions(&state, &request, &[], &query_map)
                    .await
            }
            // POST /describe-app-version-template
            ("POST", "/describe-app-version-template") => {
                self.handle_describe_app_version_template(&state, &request, &[], &query_map)
                    .await
            }
            // POST /create-app-version-resource
            ("POST", "/create-app-version-resource") => {
                self.handle_create_app_version_resource(&state, &request, &[], &query_map)
                    .await
            }
            // POST /list-app-version-resources
            ("POST", "/list-app-version-resources") => {
                self.handle_list_app_version_resources(&state, &request, &[], &query_map)
                    .await
            }
            // POST /create-app-version-app-component
            ("POST", "/create-app-version-app-component") => {
                self.handle_create_app_version_app_component(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                )
                .await
            }
            // POST /list-app-version-app-components
            ("POST", "/list-app-version-app-components") => {
                self.handle_list_app_version_app_components(&state, &request, &[], &query_map)
                    .await
            }
            // POST /import-resources-to-draft-app-version
            ("POST", "/import-resources-to-draft-app-version") => {
                self.handle_import_resources_to_draft_app_version(&state, &request, &[], &query_map)
                    .await
            }
            // GET /list-app-assessments
            ("GET", "/list-app-assessments") => {
                self.handle_list_app_assessments(&state, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_app(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_app_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing required field 'name'");
        }
        let description = input.description.as_deref().unwrap_or("");
        let policy_arn = input.policy_arn.as_deref().unwrap_or("");
        let assessment_schedule = input.assessment_schedule.as_deref().unwrap_or("Disabled");
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_app(
            &input.name,
            description,
            policy_arn,
            assessment_schedule,
            tags,
            account_id,
            region,
        ) {
            Ok(app) => wire::serialize_create_app_response(&wire::CreateAppResponse {
                app: Some(app_to_wire(app)),
            }),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_describe_app(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_app_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }

        let state = state.read().await;
        match state.describe_app(&input.app_arn) {
            Ok(app) => wire::serialize_describe_app_response(&wire::DescribeAppResponse {
                app: Some(app_to_wire(app)),
            }),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_delete_app(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_app_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }

        let mut state = state.write().await;
        match state.delete_app(&input.app_arn) {
            Ok(arn) => {
                wire::serialize_delete_app_response(&wire::DeleteAppResponse { app_arn: Some(arn) })
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_list_apps(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_apps_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name_filter = input.name.as_deref();
        let app_arn_filter = input.app_arn.as_deref();

        let state = state.read().await;
        let summaries = state.list_apps(name_filter, app_arn_filter);
        let wire_summaries: Vec<wire::AppSummary> =
            summaries.iter().map(app_summary_to_wire).collect();
        wire::serialize_list_apps_response(&wire::ListAppsResponse {
            app_summaries: Some(wire_summaries),
            next_token: None,
        })
    }

    // --- Resiliency Policy handlers ---

    async fn handle_create_resiliency_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resiliency_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.policy_name.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'policyName'",
            );
        }
        if input.tier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing required field 'tier'");
        }
        if input.policy.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'policy'",
            );
        }

        let policy_description = input.policy_description.as_deref().unwrap_or("");
        let data_location_constraint = input
            .data_location_constraint
            .as_deref()
            .unwrap_or("AnyLocation");

        let valid_disruption_types = ["Software", "Hardware", "AZ", "Region"];
        let mut failure_policies: HashMap<String, FailurePolicyData> = HashMap::new();
        for (key, val) in &input.policy {
            if !valid_disruption_types.contains(&key.as_str()) {
                return rest_json_error(
                    400,
                    "ValidationException",
                    &format!(
                        "Value '{}' at 'policy' failed to satisfy constraint: \
                         Member must satisfy enum value set: [Software, Hardware, Region, AZ]",
                        key
                    ),
                );
            }
            failure_policies.insert(
                key.clone(),
                FailurePolicyData {
                    rpo_in_secs: val.rpo_in_secs,
                    rto_in_secs: val.rto_in_secs,
                },
            );
        }
        // Validate that required disruption types are present (Software, Hardware, AZ)
        for required in &["Software", "Hardware", "AZ"] {
            if !failure_policies.contains_key(*required) {
                return rest_json_error(
                    400,
                    "ValidationException",
                    &format!("FailureType {} does not exist", required.to_uppercase()),
                );
            }
        }

        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_resiliency_policy(
            &input.policy_name,
            policy_description,
            data_location_constraint,
            &input.tier,
            failure_policies,
            tags,
            account_id,
            region,
        ) {
            Ok(policy) => wire::serialize_create_resiliency_policy_response(
                &wire::CreateResiliencyPolicyResponse {
                    policy: Some(policy_to_wire(policy)),
                },
            ),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_describe_resiliency_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_resiliency_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.policy_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'policyArn'",
            );
        }

        let state = state.read().await;
        match state.describe_resiliency_policy(&input.policy_arn) {
            Ok(policy) => wire::serialize_describe_resiliency_policy_response(
                &wire::DescribeResiliencyPolicyResponse {
                    policy: Some(policy_to_wire(policy)),
                },
            ),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_delete_resiliency_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resiliency_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.policy_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'policyArn'",
            );
        }

        let mut state = state.write().await;
        match state.delete_resiliency_policy(&input.policy_arn) {
            Ok(arn) => wire::serialize_delete_resiliency_policy_response(
                &wire::DeleteResiliencyPolicyResponse {
                    policy_arn: Some(arn),
                },
            ),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_update_resiliency_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_resiliency_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.policy_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'policyArn'",
            );
        }

        let policy_map: Option<HashMap<String, FailurePolicyData>> = input.policy.map(|m| {
            m.into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        FailurePolicyData {
                            rpo_in_secs: v.rpo_in_secs,
                            rto_in_secs: v.rto_in_secs,
                        },
                    )
                })
                .collect()
        });

        let mut state = state.write().await;
        match state.update_resiliency_policy(
            &input.policy_arn,
            input.policy_name.as_deref(),
            input.policy_description.as_deref(),
            input.data_location_constraint.as_deref(),
            input.tier.as_deref(),
            policy_map,
        ) {
            Ok(policy) => wire::serialize_update_resiliency_policy_response(
                &wire::UpdateResiliencyPolicyResponse {
                    policy: Some(policy_to_wire(policy)),
                },
            ),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_list_resiliency_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_resiliency_policies();
        let wire_policies: Vec<wire::ResiliencyPolicy> =
            policies.iter().map(|p| policy_to_wire(p)).collect();
        wire::serialize_list_resiliency_policies_response(&wire::ListResiliencyPoliciesResponse {
            resiliency_policies: Some(wire_policies),
            next_token: None,
        })
    }

    async fn handle_list_suggested_resiliency_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_suggested_resiliency_policies();
        let wire_policies: Vec<wire::ResiliencyPolicy> =
            policies.iter().map(policy_to_wire).collect();
        wire::serialize_list_suggested_resiliency_policies_response(
            &wire::ListSuggestedResiliencyPoliciesResponse {
                resiliency_policies: Some(wire_policies),
                next_token: None,
            },
        )
    }

    // --- Tag handlers ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
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
                let tags_opt = if tags.is_empty() { None } else { Some(tags) };
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse { tags: tags_opt },
                )
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing required field 'tags'");
        }

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tag_keys.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'tagKeys'",
            );
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    // --- App version handlers ---

    async fn handle_publish_app_version(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_app_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }

        let mut state = state.write().await;
        match state.publish_app_version(&input.app_arn, input.version_name.as_deref()) {
            Ok((arn, ver, id)) => {
                wire::serialize_publish_app_version_response(&wire::PublishAppVersionResponse {
                    app_arn: Some(arn),
                    app_version: Some(ver),
                    identifier: Some(id),
                    ..Default::default()
                })
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_list_app_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_app_versions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }

        let state = state.read().await;
        match state.list_app_versions(&input.app_arn) {
            Ok(versions) => {
                let wire_versions: Vec<wire::AppVersionSummary> = versions
                    .iter()
                    .map(|v| wire::AppVersionSummary {
                        app_version: Some(v.app_version.clone()),
                        identifier: Some(v.identifier),
                        creation_time: Some(v.creation_time.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_app_versions_response(&wire::ListAppVersionsResponse {
                    app_versions: Some(wire_versions),
                    next_token: None,
                })
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_describe_app_version_template(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_app_version_template_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }
        if input.app_version.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appVersion'",
            );
        }

        let state = state.read().await;
        match state.describe_app_version_template(&input.app_arn, &input.app_version) {
            Ok((arn, ver, template_body)) => {
                wire::serialize_describe_app_version_template_response(
                    &wire::DescribeAppVersionTemplateResponse {
                        app_arn: Some(arn),
                        app_version: Some(ver),
                        app_template_body: Some(template_body),
                    },
                )
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_create_app_version_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_app_version_resource_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }
        if input.physical_resource_id.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'physicalResourceId'",
            );
        }
        if input.resource_type.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'resourceType'",
            );
        }
        let logical_id = input.logical_resource_id.identifier.as_str();
        let resource_name = input.resource_name.as_deref().unwrap_or("");
        let aws_region = input.aws_region.as_deref().unwrap_or("");
        let aws_account_id = input.aws_account_id.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.create_app_version_resource(
            &input.app_arn,
            resource_name,
            logical_id,
            &input.physical_resource_id,
            &input.resource_type,
            input.app_components.clone(),
            aws_region,
            aws_account_id,
        ) {
            Ok(resource) => wire::serialize_create_app_version_resource_response(
                &wire::CreateAppVersionResourceResponse {
                    app_arn: Some(input.app_arn.clone()),
                    app_version: Some("draft".to_string()),
                    physical_resource: Some(resource_to_wire(resource)),
                },
            ),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_list_app_version_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_app_version_resources_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }
        if input.app_version.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appVersion'",
            );
        }

        let state = state.read().await;
        match state.list_app_version_resources(&input.app_arn, &input.app_version) {
            Ok(resources) => {
                let wire_resources: Vec<wire::PhysicalResource> =
                    resources.iter().map(|r| resource_to_wire(r)).collect();
                wire::serialize_list_app_version_resources_response(
                    &wire::ListAppVersionResourcesResponse {
                        physical_resources: Some(wire_resources),
                        resolution_id: Some(uuid::Uuid::new_v4().to_string()),
                        next_token: None,
                    },
                )
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_create_app_version_app_component(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_app_version_app_component_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing required field 'name'");
        }
        let component_type = if input.r#type.is_empty() {
            "AWS::ResilienceHub::AppComponent"
        } else {
            input.r#type.as_str()
        };

        let mut state = state.write().await;
        match state.create_app_version_app_component(
            &input.app_arn,
            &input.name,
            component_type,
            account_id,
        ) {
            Ok(comp) => wire::serialize_create_app_version_app_component_response(
                &wire::CreateAppVersionAppComponentResponse {
                    app_arn: Some(input.app_arn.clone()),
                    app_version: Some("draft".to_string()),
                    app_component: Some(wire::AppComponent {
                        id: Some(comp.id.clone()),
                        name: Some(comp.name.clone()),
                        r#type: Some(comp.component_type.clone()),
                        additional_info: None,
                    }),
                },
            ),
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_list_app_version_app_components(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_app_version_app_components_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }
        let app_version = if input.app_version.is_empty() {
            "draft"
        } else {
            input.app_version.as_str()
        };

        let state = state.read().await;
        match state.list_app_version_app_components(&input.app_arn, app_version) {
            Ok(comps) => {
                let wire_comps: Vec<wire::AppComponent> = comps
                    .iter()
                    .map(|c| wire::AppComponent {
                        id: Some(c.id.clone()),
                        name: Some(c.name.clone()),
                        r#type: Some(c.component_type.clone()),
                        additional_info: None,
                    })
                    .collect();
                wire::serialize_list_app_version_app_components_response(
                    &wire::ListAppVersionAppComponentsResponse {
                        app_arn: Some(input.app_arn.clone()),
                        app_version: Some(app_version.to_string()),
                        app_components: Some(wire_comps),
                        next_token: None,
                    },
                )
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_import_resources_to_draft_app_version(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_import_resources_to_draft_app_version_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.app_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing required field 'appArn'",
            );
        }
        let source_arns = input.source_arns.unwrap_or_default();

        let mut state = state.write().await;
        match state.import_resources_to_draft_app_version(&input.app_arn, source_arns) {
            Ok((arn, version, arns)) => {
                wire::serialize_import_resources_to_draft_app_version_response(
                    &wire::ImportResourcesToDraftAppVersionResponse {
                        app_arn: Some(arn),
                        app_version: Some(version),
                        source_arns: if arns.is_empty() { None } else { Some(arns) },
                        status: Some("Success".to_string()),
                        eks_sources: None,
                        terraform_sources: None,
                    },
                )
            }
            Err(e) => resiliencehub_error_response(&e),
        }
    }

    async fn handle_list_app_assessments(
        &self,
        state: &Arc<tokio::sync::RwLock<ResilienceHubState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_app_assessments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let app_arn_filter = input.app_arn.as_deref();

        let state = state.read().await;
        let assessments = state.list_app_assessments(app_arn_filter);
        let summaries: Vec<wire::AppAssessmentSummary> = assessments
            .iter()
            .map(|a| wire::AppAssessmentSummary {
                assessment_arn: Some(a.assessment_arn.clone()),
                assessment_name: Some(a.assessment_name.clone()),
                assessment_status: Some(a.assessment_status.clone()),
                app_arn: Some(a.app_arn.clone()),
                app_version: Some(a.app_version.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_app_assessments_response(&wire::ListAppAssessmentsResponse {
            assessment_summaries: Some(summaries),
            next_token: None,
        })
    }
}

/// Convert a state `App` to the wire `App` model type.
fn app_to_wire(app: &crate::types::App) -> wire::App {
    wire::App {
        app_arn: Some(app.app_arn.clone()),
        name: Some(app.name.clone()),
        description: Some(app.description.clone()),
        status: Some(app.status.clone()),
        creation_time: Some(app.creation_time.timestamp() as f64),
        compliance_status: Some(app.compliance_status.clone()),
        policy_arn: Some(app.policy_arn.clone()),
        assessment_schedule: Some(app.assessment_schedule.clone()),
        tags: if app.tags.is_empty() {
            None
        } else {
            Some(app.tags.clone())
        },
        ..Default::default()
    }
}

/// Convert a state `AppSummary` to the wire `AppSummary` model type.
fn app_summary_to_wire(s: &crate::types::AppSummary) -> wire::AppSummary {
    wire::AppSummary {
        app_arn: Some(s.app_arn.clone()),
        name: Some(s.name.clone()),
        description: Some(s.description.clone()),
        status: Some(s.status.clone()),
        creation_time: Some(s.creation_time.timestamp() as f64),
        compliance_status: Some(s.compliance_status.clone()),
        assessment_schedule: Some(s.assessment_schedule.clone()),
        ..Default::default()
    }
}

/// Convert a state `ResiliencyPolicyData` to the wire `ResiliencyPolicy` model type.
fn policy_to_wire(p: &crate::types::ResiliencyPolicyData) -> wire::ResiliencyPolicy {
    let wire_policy: HashMap<String, wire::FailurePolicy> = p
        .policy
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                wire::FailurePolicy {
                    rpo_in_secs: v.rpo_in_secs,
                    rto_in_secs: v.rto_in_secs,
                },
            )
        })
        .collect();

    wire::ResiliencyPolicy {
        policy_arn: Some(p.policy_arn.clone()),
        policy_name: Some(p.policy_name.clone()),
        policy_description: Some(p.policy_description.clone()),
        data_location_constraint: Some(p.data_location_constraint.clone()),
        tier: Some(p.tier.clone()),
        policy: Some(wire_policy),
        creation_time: Some(p.creation_time.timestamp() as f64),
        tags: if p.tags.is_empty() {
            None
        } else {
            Some(p.tags.clone())
        },
        ..Default::default()
    }
}

/// Convert a state `AppVersionResource` to the wire `PhysicalResource` model type.
fn resource_to_wire(r: &crate::types::AppVersionResource) -> wire::PhysicalResource {
    wire::PhysicalResource {
        logical_resource_id: Some(wire::LogicalResourceId {
            identifier: r.logical_resource_id.clone(),
            ..Default::default()
        }),
        physical_resource_id: Some(wire::PhysicalResourceId {
            identifier: r.physical_resource_id.clone(),
            r#type: "Arn".to_string(),
            aws_account_id: if r.aws_account_id.is_empty() {
                None
            } else {
                Some(r.aws_account_id.clone())
            },
            aws_region: if r.aws_region.is_empty() {
                None
            } else {
                Some(r.aws_region.clone())
            },
        }),
        resource_type: Some(r.resource_type.clone()),
        resource_name: if r.resource_name.is_empty() {
            None
        } else {
            Some(r.resource_name.clone())
        },
        app_components: Some(
            r.app_components
                .iter()
                .map(|c| wire::AppComponent {
                    name: Some(c.clone()),
                    r#type: Some("AWS::ResilienceHub::AppComponent".to_string()),
                    ..Default::default()
                })
                .collect(),
        ),
        ..Default::default()
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
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(&s[i + 1..i + 3], 16)
        {
            result.push(byte as char);
            i += 3;
            continue;
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

fn resiliencehub_error_response(err: &ResilienceHubError) -> MockResponse {
    let (status, error_type) = match err {
        ResilienceHubError::AppConflict { .. } => (409, "ConflictException"),
        ResilienceHubError::AppNotFound { .. } => (404, "ResourceNotFoundException"),
        ResilienceHubError::PolicyNotFound { .. } => (404, "ResourceNotFoundException"),
        ResilienceHubError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
