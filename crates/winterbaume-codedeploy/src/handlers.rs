use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::{CodeDeployError, CodeDeployState};
use crate::views::CodeDeployStateView;
use crate::wire;

pub struct CodeDeployService {
    pub(crate) state: Arc<BackendState<CodeDeployState>>,
    pub(crate) notifier: StateChangeNotifier<CodeDeployStateView>,
}

impl CodeDeployService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodeDeployService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodeDeployService {
    fn service_name(&self) -> &str {
        "codedeploy"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://codedeploy\..*\.amazonaws\.com",
            r"https?://codedeploy\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CodeDeployService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "CreateApplication" => self.handle_create_application(&state, body_bytes).await,
            "GetApplication" => self.handle_get_application(&state, body_bytes).await,
            "DeleteApplication" => self.handle_delete_application(&state, body_bytes).await,
            "ListApplications" => self.handle_list_applications(&state).await,
            "BatchGetApplications" => self.handle_batch_get_applications(&state, body_bytes).await,
            "CreateDeploymentGroup" => {
                self.handle_create_deployment_group(&state, body_bytes)
                    .await
            }
            "GetDeploymentGroup" => self.handle_get_deployment_group(&state, body_bytes).await,
            "ListDeploymentGroups" => self.handle_list_deployment_groups(&state, body_bytes).await,
            "CreateDeployment" => self.handle_create_deployment(&state, body_bytes).await,
            "GetDeployment" => self.handle_get_deployment(&state, body_bytes).await,
            "BatchGetDeployments" => self.handle_batch_get_deployments(&state, body_bytes).await,
            "ListDeployments" => self.handle_list_deployments(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AddTagsToOnPremisesInstances" => json_error_response(
                501,
                "NotImplementedError",
                "AddTagsToOnPremisesInstances is not yet implemented in winterbaume-codedeploy",
            ),
            "BatchGetApplicationRevisions" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetApplicationRevisions is not yet implemented in winterbaume-codedeploy",
            ),
            "BatchGetDeploymentGroups" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetDeploymentGroups is not yet implemented in winterbaume-codedeploy",
            ),
            "BatchGetDeploymentInstances" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetDeploymentInstances is not yet implemented in winterbaume-codedeploy",
            ),
            "BatchGetDeploymentTargets" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetDeploymentTargets is not yet implemented in winterbaume-codedeploy",
            ),
            "BatchGetOnPremisesInstances" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetOnPremisesInstances is not yet implemented in winterbaume-codedeploy",
            ),
            "ContinueDeployment" => json_error_response(
                501,
                "NotImplementedError",
                "ContinueDeployment is not yet implemented in winterbaume-codedeploy",
            ),
            "CreateDeploymentConfig" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDeploymentConfig is not yet implemented in winterbaume-codedeploy",
            ),
            "DeleteDeploymentConfig" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDeploymentConfig is not yet implemented in winterbaume-codedeploy",
            ),
            "DeleteDeploymentGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDeploymentGroup is not yet implemented in winterbaume-codedeploy",
            ),
            "DeleteGitHubAccountToken" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteGitHubAccountToken is not yet implemented in winterbaume-codedeploy",
            ),
            "DeleteResourcesByExternalId" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteResourcesByExternalId is not yet implemented in winterbaume-codedeploy",
            ),
            "DeregisterOnPremisesInstance" => json_error_response(
                501,
                "NotImplementedError",
                "DeregisterOnPremisesInstance is not yet implemented in winterbaume-codedeploy",
            ),
            "GetApplicationRevision" => json_error_response(
                501,
                "NotImplementedError",
                "GetApplicationRevision is not yet implemented in winterbaume-codedeploy",
            ),
            "GetDeploymentConfig" => json_error_response(
                501,
                "NotImplementedError",
                "GetDeploymentConfig is not yet implemented in winterbaume-codedeploy",
            ),
            "GetDeploymentInstance" => json_error_response(
                501,
                "NotImplementedError",
                "GetDeploymentInstance is not yet implemented in winterbaume-codedeploy",
            ),
            "GetDeploymentTarget" => json_error_response(
                501,
                "NotImplementedError",
                "GetDeploymentTarget is not yet implemented in winterbaume-codedeploy",
            ),
            "GetOnPremisesInstance" => json_error_response(
                501,
                "NotImplementedError",
                "GetOnPremisesInstance is not yet implemented in winterbaume-codedeploy",
            ),
            "ListApplicationRevisions" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationRevisions is not yet implemented in winterbaume-codedeploy",
            ),
            "ListDeploymentConfigs" => json_error_response(
                501,
                "NotImplementedError",
                "ListDeploymentConfigs is not yet implemented in winterbaume-codedeploy",
            ),
            "ListDeploymentInstances" => json_error_response(
                501,
                "NotImplementedError",
                "ListDeploymentInstances is not yet implemented in winterbaume-codedeploy",
            ),
            "ListDeploymentTargets" => json_error_response(
                501,
                "NotImplementedError",
                "ListDeploymentTargets is not yet implemented in winterbaume-codedeploy",
            ),
            "ListGitHubAccountTokenNames" => json_error_response(
                501,
                "NotImplementedError",
                "ListGitHubAccountTokenNames is not yet implemented in winterbaume-codedeploy",
            ),
            "ListOnPremisesInstances" => json_error_response(
                501,
                "NotImplementedError",
                "ListOnPremisesInstances is not yet implemented in winterbaume-codedeploy",
            ),
            "PutLifecycleEventHookExecutionStatus" => json_error_response(
                501,
                "NotImplementedError",
                "PutLifecycleEventHookExecutionStatus is not yet implemented in winterbaume-codedeploy",
            ),
            "RegisterApplicationRevision" => json_error_response(
                501,
                "NotImplementedError",
                "RegisterApplicationRevision is not yet implemented in winterbaume-codedeploy",
            ),
            "RegisterOnPremisesInstance" => json_error_response(
                501,
                "NotImplementedError",
                "RegisterOnPremisesInstance is not yet implemented in winterbaume-codedeploy",
            ),
            "RemoveTagsFromOnPremisesInstances" => json_error_response(
                501,
                "NotImplementedError",
                "RemoveTagsFromOnPremisesInstances is not yet implemented in winterbaume-codedeploy",
            ),
            "SkipWaitTimeForInstanceTermination" => json_error_response(
                501,
                "NotImplementedError",
                "SkipWaitTimeForInstanceTermination is not yet implemented in winterbaume-codedeploy",
            ),
            "StopDeployment" => json_error_response(
                501,
                "NotImplementedError",
                "StopDeployment is not yet implemented in winterbaume-codedeploy",
            ),
            "UpdateApplication" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateApplication is not yet implemented in winterbaume-codedeploy",
            ),
            "UpdateDeploymentGroup" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDeploymentGroup is not yet implemented in winterbaume-codedeploy",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        }
    }

    // ---- Application handlers ----

    async fn handle_create_application(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "applicationName is required",
            );
        }
        let application_name = input.application_name.as_str();
        let compute_platform = input.compute_platform.as_deref().unwrap_or("Server");

        let mut state = state.write().await;
        match state.create_application(application_name, compute_platform) {
            Ok(application_id) => {
                wire::serialize_create_application_response(&wire::CreateApplicationOutput {
                    application_id: Some(application_id),
                })
            }
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_get_application(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "applicationName is required",
            );
        }
        let application_name = input.application_name.as_str();

        let state = state.read().await;
        match state.get_application(application_name) {
            Ok(app) => wire::serialize_get_application_response(&wire::GetApplicationOutput {
                application: Some(application_to_wire(app)),
            }),
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_delete_application(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "applicationName is required",
            );
        }
        let application_name = input.application_name.as_str();

        let mut state = state.write().await;
        match state.delete_application(application_name) {
            Ok(()) => wire::serialize_delete_application_response(),
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_list_applications(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let apps: Vec<String> = state
            .list_applications()
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        wire::serialize_list_applications_response(&wire::ListApplicationsOutput {
            applications: Some(apps),
            next_token: None,
        })
    }

    async fn handle_batch_get_applications(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_applications_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let names = input.application_names;

        let state = state.read().await;
        let apps = state.batch_get_applications(&names);
        let infos: Vec<wire::ApplicationInfo> =
            apps.iter().map(|a| application_to_wire(a)).collect();

        wire::serialize_batch_get_applications_response(&wire::BatchGetApplicationsOutput {
            applications_info: Some(infos),
        })
    }

    // ---- DeploymentGroup handlers ----

    async fn handle_create_deployment_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_deployment_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "applicationName is required",
            );
        }
        if input.deployment_group_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "deploymentGroupName is required",
            );
        }
        if input.service_role_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "serviceRoleArn is required",
            );
        }
        let application_name = input.application_name.as_str();
        let deployment_group_name = input.deployment_group_name.as_str();
        let service_role_arn = input.service_role_arn.as_str();
        let deployment_config_name = input.deployment_config_name.as_deref();

        let mut state = state.write().await;
        match state.create_deployment_group(
            application_name,
            deployment_group_name,
            service_role_arn,
            deployment_config_name,
        ) {
            Ok(deployment_group_id) => wire::serialize_create_deployment_group_response(
                &wire::CreateDeploymentGroupOutput {
                    deployment_group_id: Some(deployment_group_id),
                },
            ),
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_get_deployment_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployment_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "applicationName is required",
            );
        }
        if input.deployment_group_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "deploymentGroupName is required",
            );
        }
        let application_name = input.application_name.as_str();
        let deployment_group_name = input.deployment_group_name.as_str();

        let state = state.read().await;
        match state.get_deployment_group(application_name, deployment_group_name) {
            Ok(dg) => {
                wire::serialize_get_deployment_group_response(&wire::GetDeploymentGroupOutput {
                    deployment_group_info: Some(deployment_group_to_wire(dg)),
                })
            }
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_list_deployment_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_deployment_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "applicationName is required",
            );
        }
        let application_name = input.application_name.as_str();

        let state = state.read().await;
        match state.list_deployment_groups(application_name) {
            Ok(groups) => {
                wire::serialize_list_deployment_groups_response(&wire::ListDeploymentGroupsOutput {
                    application_name: Some(application_name.to_string()),
                    deployment_groups: Some(groups.into_iter().map(|s| s.to_string()).collect()),
                    next_token: None,
                })
            }
            Err(e) => codedeploy_error_response(&e),
        }
    }

    // ---- Deployment handlers ----

    async fn handle_create_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_deployment_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "applicationName is required",
            );
        }
        let deployment_group_name = match input.deployment_group_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(
                    400,
                    "InvalidRequestException",
                    "deploymentGroupName is required",
                );
            }
        };
        let application_name = input.application_name.as_str();
        let description = input.description.as_deref();
        let file_exists_behavior = input.file_exists_behavior.as_deref();
        let ignore_application_stop_failures =
            input.ignore_application_stop_failures.unwrap_or(false);

        // Parse revision
        let revision_type = input
            .revision
            .as_ref()
            .and_then(|r| r.revision_type.as_deref());
        let revision_s3_bucket = input
            .revision
            .as_ref()
            .and_then(|r| r.s3_location.as_ref())
            .and_then(|s| s.bucket.as_deref());
        let revision_s3_key = input
            .revision
            .as_ref()
            .and_then(|r| r.s3_location.as_ref())
            .and_then(|s| s.key.as_deref());
        let revision_s3_bundle_type = input
            .revision
            .as_ref()
            .and_then(|r| r.s3_location.as_ref())
            .and_then(|s| s.bundle_type.as_deref());
        let revision_github_repository = input
            .revision
            .as_ref()
            .and_then(|r| r.git_hub_location.as_ref())
            .and_then(|g| g.repository.as_deref());
        let revision_github_commit_id = input
            .revision
            .as_ref()
            .and_then(|r| r.git_hub_location.as_ref())
            .and_then(|g| g.commit_id.as_deref());

        let mut state = state.write().await;
        match state.create_deployment(
            application_name,
            deployment_group_name,
            description,
            revision_type,
            revision_s3_bucket,
            revision_s3_key,
            revision_s3_bundle_type,
            revision_github_repository,
            revision_github_commit_id,
            file_exists_behavior,
            ignore_application_stop_failures,
        ) {
            Ok(deployment_id) => {
                wire::serialize_create_deployment_response(&wire::CreateDeploymentOutput {
                    deployment_id: Some(deployment_id),
                })
            }
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_get_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployment_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.deployment_id.is_empty() {
            return json_error_response(400, "InvalidRequestException", "deploymentId is required");
        }
        let deployment_id = input.deployment_id.as_str();

        let state = state.read().await;
        match state.get_deployment(deployment_id) {
            Ok(d) => wire::serialize_get_deployment_response(&wire::GetDeploymentOutput {
                deployment_info: Some(deployment_to_wire(d)),
            }),
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_batch_get_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_deployments_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let ids = input.deployment_ids;

        let state = state.read().await;
        let deployments = state.batch_get_deployments(&ids);
        let infos: Vec<wire::DeploymentInfo> =
            deployments.iter().map(|d| deployment_to_wire(d)).collect();

        wire::serialize_batch_get_deployments_response(&wire::BatchGetDeploymentsOutput {
            deployments_info: Some(infos),
        })
    }

    async fn handle_list_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_deployments_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let application_name = input.application_name.as_deref();
        let deployment_group_name = input.deployment_group_name.as_deref();

        let state = state.read().await;
        let ids: Vec<String> = state
            .list_deployments(application_name, deployment_group_name)
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        wire::serialize_list_deployments_response(&wire::ListDeploymentsOutput {
            deployments: Some(ids),
            next_token: None,
        })
    }

    // ---- Tag handlers ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceArn is required");
        }
        let resource_arn = input.resource_arn.as_str();

        let tags: Vec<(String, String)> = input
            .tags
            .into_iter()
            .filter_map(|t| Some((t.key?, t.value?)))
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(resource_arn, &tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceOutput {
                ..Default::default()
            }),
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceArn is required");
        }
        let resource_arn = input.resource_arn.as_str();
        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        match state.untag_resource(resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceOutput {
                ..Default::default()
            }),
            Err(e) => codedeploy_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeDeployState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceArn is required");
        }
        let resource_arn = input.resource_arn.as_str();

        let state = state.read().await;
        let tags = state.list_tags_for_resource(resource_arn);
        let wire_tags: Vec<wire::Tag> = tags
            .into_iter()
            .map(|(k, v)| wire::Tag {
                key: Some(k),
                value: Some(v),
            })
            .collect();

        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
            tags: Some(wire_tags),
            next_token: None,
        })
    }
}

fn application_to_wire(app: &crate::types::Application) -> wire::ApplicationInfo {
    wire::ApplicationInfo {
        application_id: Some(app.application_id.clone()),
        application_name: Some(app.application_name.clone()),
        compute_platform: Some(app.compute_platform.clone()),
        create_time: Some(app.create_time.timestamp() as f64),
        ..Default::default()
    }
}

fn deployment_group_to_wire(dg: &crate::types::DeploymentGroup) -> wire::DeploymentGroupInfo {
    wire::DeploymentGroupInfo {
        deployment_group_id: Some(dg.deployment_group_id.clone()),
        deployment_group_name: Some(dg.deployment_group_name.clone()),
        application_name: Some(dg.application_name.clone()),
        service_role_arn: Some(dg.service_role_arn.clone()),
        deployment_config_name: Some(dg.deployment_config_name.clone()),
        compute_platform: Some(dg.compute_platform.clone()),
        ..Default::default()
    }
}

fn deployment_to_wire(d: &crate::types::Deployment) -> wire::DeploymentInfo {
    let revision = if d.revision_type.is_some() {
        let s3_location = if d.revision_s3_bucket.is_some() {
            Some(wire::S3Location {
                bucket: d.revision_s3_bucket.clone(),
                key: d.revision_s3_key.clone(),
                bundle_type: d.revision_s3_bundle_type.clone(),
                ..Default::default()
            })
        } else {
            None
        };
        let github_location = if d.revision_github_repository.is_some() {
            Some(wire::GitHubLocation {
                repository: d.revision_github_repository.clone(),
                commit_id: d.revision_github_commit_id.clone(),
            })
        } else {
            None
        };
        Some(wire::RevisionLocation {
            revision_type: d.revision_type.clone(),
            s3_location,
            git_hub_location: github_location,
            ..Default::default()
        })
    } else {
        None
    };

    wire::DeploymentInfo {
        deployment_id: Some(d.deployment_id.clone()),
        application_name: Some(d.application_name.clone()),
        deployment_group_name: Some(d.deployment_group_name.clone()),
        deployment_config_name: Some(d.deployment_config_name.clone()),
        description: if d.description.is_empty() {
            None
        } else {
            Some(d.description.clone())
        },
        status: Some(d.status.clone()),
        create_time: Some(d.create_time.timestamp() as f64),
        revision,
        file_exists_behavior: d.file_exists_behavior.clone(),
        ignore_application_stop_failures: Some(d.ignore_application_stop_failures),
        ..Default::default()
    }
}

fn codedeploy_error_response(err: &CodeDeployError) -> MockResponse {
    let (error_type, status) = match err {
        CodeDeployError::ApplicationAlreadyExists { .. } => {
            ("ApplicationAlreadyExistsException", 400)
        }
        CodeDeployError::ApplicationDoesNotExist { .. } => {
            ("ApplicationDoesNotExistException", 400)
        }
        CodeDeployError::DeploymentGroupAlreadyExists { .. } => {
            ("DeploymentGroupAlreadyExistsException", 400)
        }
        CodeDeployError::DeploymentGroupDoesNotExist { .. } => {
            ("DeploymentGroupDoesNotExistException", 400)
        }
        CodeDeployError::DeploymentDoesNotExist { .. } => ("DeploymentDoesNotExistException", 400),
    };
    let message = err.to_string();
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": message}).to_string(),
    )
}
