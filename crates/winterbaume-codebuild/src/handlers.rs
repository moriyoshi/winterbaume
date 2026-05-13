use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, json_error_response,
};

use crate::state::{CodeBuildError, CodeBuildState};
use crate::views::CodeBuildStateView;
use crate::wire;

pub struct CodeBuildService {
    pub(crate) state: Arc<BackendState<CodeBuildState>>,
    pub(crate) notifier: StateChangeNotifier<CodeBuildStateView>,
}

impl CodeBuildService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodeBuildService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodeBuildService {
    fn service_name(&self) -> &str {
        "codebuild"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://codebuild\..*\.amazonaws\.com",
            r"https?://codebuild\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CodeBuildService {
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
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateProject" => {
                self.handle_create_project(&state, body_bytes, account_id, &region)
                    .await
            }
            "BatchGetProjects" => self.handle_batch_get_projects(&state, body_bytes).await,
            "DeleteProject" => self.handle_delete_project(&state, body_bytes).await,
            "ListProjects" => self.handle_list_projects(&state).await,
            "BatchDeleteBuilds" => self.handle_batch_delete_builds(&state, body_bytes).await,
            "BatchGetBuilds" => self.handle_batch_get_builds(&state, body_bytes).await,
            "UpdateProject" => {
                self.handle_update_project(&state, body_bytes, account_id)
                    .await
            }
            "RetryBuild" => {
                self.handle_retry_build(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateWebhook" => {
                self.handle_create_webhook(&state, body_bytes, account_id, &region)
                    .await
            }
            "UpdateWebhook" => self.handle_update_webhook(&state, body_bytes).await,
            "DeleteWebhook" => self.handle_delete_webhook(&state, body_bytes).await,
            "ImportSourceCredentials" => {
                self.handle_import_source_credentials(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListSourceCredentials" => self.handle_list_source_credentials(&state).await,
            "DeleteSourceCredentials" => {
                self.handle_delete_source_credentials(&state, body_bytes)
                    .await
            }
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "GetResourcePolicy" => self.handle_get_resource_policy(&state, body_bytes).await,
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, body_bytes).await,
            "InvalidateProjectCache" => {
                self.handle_invalidate_project_cache(&state, body_bytes)
                    .await
            }
            // STUB[no-engine]: DescribeTestCases requires real build execution and test result
            //   collection; the mock has no test-report engine to populate case results.
            "DescribeTestCases" => {
                wire::serialize_describe_test_cases_response(&wire::DescribeTestCasesOutput {
                    ..Default::default()
                })
            }
            "ListReportGroups" => self.handle_list_report_groups(&state).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "BatchGetBuildBatches" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetBuildBatches is not yet implemented in winterbaume-codebuild",
            ),
            "BatchGetCommandExecutions" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetCommandExecutions is not yet implemented in winterbaume-codebuild",
            ),
            "BatchGetFleets" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetFleets is not yet implemented in winterbaume-codebuild",
            ),
            "BatchGetReportGroups" => {
                self.handle_batch_get_report_groups(&state, body_bytes)
                    .await
            }
            "BatchGetReports" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetReports is not yet implemented in winterbaume-codebuild",
            ),
            "BatchGetSandboxes" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetSandboxes is not yet implemented in winterbaume-codebuild",
            ),
            "CreateFleet" => json_error_response(
                501,
                "NotImplementedError",
                "CreateFleet is not yet implemented in winterbaume-codebuild",
            ),
            "CreateReportGroup" => {
                self.handle_create_report_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteBuildBatch" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteBuildBatch is not yet implemented in winterbaume-codebuild",
            ),
            "DeleteFleet" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteFleet is not yet implemented in winterbaume-codebuild",
            ),
            "DeleteReport" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteReport is not yet implemented in winterbaume-codebuild",
            ),
            "DeleteReportGroup" => self.handle_delete_report_group(&state, body_bytes).await,
            "DescribeCodeCoverages" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeCodeCoverages is not yet implemented in winterbaume-codebuild",
            ),
            "GetReportGroupTrend" => json_error_response(
                501,
                "NotImplementedError",
                "GetReportGroupTrend is not yet implemented in winterbaume-codebuild",
            ),
            "ListBuildBatches" => json_error_response(
                501,
                "NotImplementedError",
                "ListBuildBatches is not yet implemented in winterbaume-codebuild",
            ),
            "ListBuildBatchesForProject" => json_error_response(
                501,
                "NotImplementedError",
                "ListBuildBatchesForProject is not yet implemented in winterbaume-codebuild",
            ),
            "ListBuilds" => self.handle_list_builds(&state).await,
            "ListBuildsForProject" => {
                self.handle_list_builds_for_project(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListCommandExecutionsForSandbox" => json_error_response(
                501,
                "NotImplementedError",
                "ListCommandExecutionsForSandbox is not yet implemented in winterbaume-codebuild",
            ),
            "ListCuratedEnvironmentImages" => json_error_response(
                501,
                "NotImplementedError",
                "ListCuratedEnvironmentImages is not yet implemented in winterbaume-codebuild",
            ),
            "ListFleets" => json_error_response(
                501,
                "NotImplementedError",
                "ListFleets is not yet implemented in winterbaume-codebuild",
            ),
            "ListReports" => json_error_response(
                501,
                "NotImplementedError",
                "ListReports is not yet implemented in winterbaume-codebuild",
            ),
            "ListReportsForReportGroup" => {
                self.handle_list_reports_for_report_group(&state, body_bytes)
                    .await
            }
            "ListSandboxes" => json_error_response(
                501,
                "NotImplementedError",
                "ListSandboxes is not yet implemented in winterbaume-codebuild",
            ),
            "ListSandboxesForProject" => json_error_response(
                501,
                "NotImplementedError",
                "ListSandboxesForProject is not yet implemented in winterbaume-codebuild",
            ),
            "ListSharedProjects" => json_error_response(
                501,
                "NotImplementedError",
                "ListSharedProjects is not yet implemented in winterbaume-codebuild",
            ),
            "ListSharedReportGroups" => json_error_response(
                501,
                "NotImplementedError",
                "ListSharedReportGroups is not yet implemented in winterbaume-codebuild",
            ),
            "RetryBuildBatch" => json_error_response(
                501,
                "NotImplementedError",
                "RetryBuildBatch is not yet implemented in winterbaume-codebuild",
            ),
            "StartBuild" => {
                self.handle_start_build(&state, body_bytes, account_id, &region)
                    .await
            }
            "StartBuildBatch" => json_error_response(
                501,
                "NotImplementedError",
                "StartBuildBatch is not yet implemented in winterbaume-codebuild",
            ),
            "StartCommandExecution" => json_error_response(
                501,
                "NotImplementedError",
                "StartCommandExecution is not yet implemented in winterbaume-codebuild",
            ),
            "StartSandbox" => json_error_response(
                501,
                "NotImplementedError",
                "StartSandbox is not yet implemented in winterbaume-codebuild",
            ),
            "StartSandboxConnection" => json_error_response(
                501,
                "NotImplementedError",
                "StartSandboxConnection is not yet implemented in winterbaume-codebuild",
            ),
            "StopBuild" => self.handle_stop_build(&state, body_bytes).await,
            "StopBuildBatch" => json_error_response(
                501,
                "NotImplementedError",
                "StopBuildBatch is not yet implemented in winterbaume-codebuild",
            ),
            "StopSandbox" => json_error_response(
                501,
                "NotImplementedError",
                "StopSandbox is not yet implemented in winterbaume-codebuild",
            ),
            "UpdateFleet" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateFleet is not yet implemented in winterbaume-codebuild",
            ),
            "UpdateProjectVisibility" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateProjectVisibility is not yet implemented in winterbaume-codebuild",
            ),
            "UpdateReportGroup" => self.handle_update_report_group(&state, body_bytes).await,
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_project(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_project_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "name is required");
        }

        let description = input.description.unwrap_or_default();
        let source_type = if input.source.r#type.is_empty() {
            "NO_SOURCE".to_string()
        } else {
            input.source.r#type.clone()
        };
        let source_location = input.source.location.unwrap_or_default();
        let artifact_type = if input.artifacts.r#type.is_empty() {
            "NO_ARTIFACTS".to_string()
        } else {
            input.artifacts.r#type.clone()
        };
        let artifact_location = input.artifacts.location;
        let env_type = if input.environment.r#type.is_empty() {
            "LINUX_CONTAINER".to_string()
        } else {
            input.environment.r#type.clone()
        };
        let env_image = if input.environment.image.is_empty() {
            "aws/codebuild/standard:5.0".to_string()
        } else {
            input.environment.image.clone()
        };
        let env_compute = if input.environment.compute_type.is_empty() {
            "BUILD_GENERAL1_SMALL".to_string()
        } else {
            input.environment.compute_type.clone()
        };
        let service_role = input.service_role;
        let tags = tags_from_wire(input.tags.unwrap_or_default());

        let mut state = state.write().await;
        match state.create_project(
            &input.name,
            &description,
            &source_type,
            &source_location,
            &artifact_type,
            artifact_location.as_deref(),
            &env_type,
            &env_image,
            &env_compute,
            &service_role,
            tags,
            account_id,
            region,
        ) {
            Ok(project) => wire::serialize_create_project_response(&wire::CreateProjectOutput {
                project: Some(project_to_wire(project)),
            }),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_batch_get_projects(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_projects_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let names = input.names;

        let state = state.read().await;
        let projects = state.batch_get_projects(&names);
        let found: Vec<wire::Project> = projects.iter().map(|p| project_to_wire(p)).collect();
        let found_names: Vec<&str> = projects.iter().map(|p| p.name.as_str()).collect();
        let found_arns: Vec<&str> = projects.iter().map(|p| p.arn.as_str()).collect();
        let not_found: Vec<String> = names
            .iter()
            .filter(|n| !found_names.contains(&n.as_str()) && !found_arns.contains(&n.as_str()))
            .cloned()
            .collect();

        wire::serialize_batch_get_projects_response(&wire::BatchGetProjectsOutput {
            projects: Some(found),
            projects_not_found: Some(not_found),
        })
    }

    async fn handle_delete_project(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_project_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "name is required");
        }

        let mut state = state.write().await;
        match state.delete_project(&input.name) {
            Ok(()) => wire::serialize_delete_project_response(&wire::DeleteProjectOutput {}),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_batch_get_builds(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_builds_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let ids = input.ids;

        let state = state.read().await;
        let builds = match state.batch_get_builds(&ids) {
            Ok(b) => b,
            Err(e) => return codebuild_error_response(&e),
        };
        let found_ids: Vec<&str> = builds.iter().map(|b| b.id.as_str()).collect();
        let not_found: Vec<String> = ids
            .iter()
            .filter(|id| !found_ids.contains(&id.as_str()))
            .cloned()
            .collect();

        let found: Vec<wire::Build> = builds.iter().map(build_to_wire).collect();

        wire::serialize_batch_get_builds_response(&wire::BatchGetBuildsOutput {
            builds: Some(found),
            builds_not_found: Some(not_found),
        })
    }

    async fn handle_list_projects(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let names: Vec<String> = state
            .list_projects()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        wire::serialize_list_projects_response(&wire::ListProjectsOutput {
            next_token: None,
            projects: Some(names),
        })
    }

    async fn handle_start_build(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_build_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.project_name.is_empty() {
            return json_error_response(400, "InvalidInputException", "projectName is required");
        }

        let mut state = state.write().await;
        match state.start_build(
            &input.project_name,
            input.source_version.as_deref(),
            account_id,
            region,
        ) {
            Ok(build) => wire::serialize_start_build_response(&wire::StartBuildOutput {
                build: Some(build_to_wire(build)),
            }),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_stop_build(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_build_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInputException", "id is required");
        }

        let mut state = state.write().await;
        match state.stop_build(&input.id) {
            Ok(build) => wire::serialize_stop_build_response(&wire::StopBuildOutput {
                build: Some(build_to_wire(build)),
            }),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_list_builds(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let ids: Vec<String> = state
            .list_builds()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        wire::serialize_list_builds_response(&wire::ListBuildsOutput {
            ids: Some(ids),
            next_token: None,
        })
    }

    async fn handle_list_builds_for_project(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_builds_for_project_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.project_name.is_empty() {
            return json_error_response(400, "InvalidInputException", "projectName is required");
        }

        let state = state.read().await;
        if !state.projects.contains_key(&input.project_name) {
            let project_name = &input.project_name;
            return json_error_response(
                400,
                "ResourceNotFoundException",
                &format!(
                    "The provided project arn:aws:codebuild:{region}:{account_id}:project/{project_name} does not exist"
                ),
            );
        }
        let ids: Vec<String> = state
            .list_builds_for_project(&input.project_name)
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        wire::serialize_list_builds_for_project_response(&wire::ListBuildsForProjectOutput {
            ids: Some(ids),
            next_token: None,
        })
    }

    async fn handle_batch_delete_builds(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_builds_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let mut state = state.write().await;
        let deleted = state.batch_delete_builds(&input.ids);

        wire::serialize_batch_delete_builds_response(&wire::BatchDeleteBuildsOutput {
            builds_deleted: Some(deleted),
            builds_not_deleted: None,
        })
    }

    async fn handle_update_project(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_project_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "name is required");
        }

        let description = input.description;
        let source_type = input
            .source
            .as_ref()
            .map(|s| s.r#type.clone())
            .filter(|t| !t.is_empty());
        let source_location = input.source.as_ref().and_then(|s| s.location.clone());
        let artifact_type = input
            .artifacts
            .as_ref()
            .map(|a| a.r#type.clone())
            .filter(|t| !t.is_empty());
        // Detect "artifacts key was sent" via outer Option of artifacts field.
        let artifact_location: Option<Option<String>> =
            input.artifacts.as_ref().map(|a| a.location.clone());
        let env_type = input
            .environment
            .as_ref()
            .map(|e| e.r#type.clone())
            .filter(|t| !t.is_empty());
        let env_image = input
            .environment
            .as_ref()
            .map(|e| e.image.clone())
            .filter(|t| !t.is_empty());
        let env_compute = input
            .environment
            .as_ref()
            .map(|e| e.compute_type.clone())
            .filter(|t| !t.is_empty());
        let service_role = input.service_role;
        let tags = input.tags.map(tags_from_wire);

        let mut state = state.write().await;
        match state.update_project(
            &input.name,
            description.as_deref(),
            source_type.as_deref(),
            source_location.as_deref(),
            artifact_type.as_deref(),
            artifact_location.as_ref().map(|loc| loc.as_deref()),
            env_type.as_deref(),
            env_image.as_deref(),
            env_compute.as_deref(),
            service_role.as_deref(),
            tags,
            account_id,
        ) {
            Ok(project) => wire::serialize_update_project_response(&wire::UpdateProjectOutput {
                project: Some(project_to_wire(project)),
            }),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_retry_build(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_retry_build_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let build_id = match input.id.as_deref() {
            Some(id) if !id.is_empty() => id,
            _ => return json_error_response(400, "InvalidInputException", "id is required"),
        };

        let mut state = state.write().await;
        match state.retry_build(build_id, account_id, region) {
            Ok(build) => wire::serialize_retry_build_response(&wire::RetryBuildOutput {
                build: Some(build_to_wire(build)),
            }),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_create_webhook(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_webhook_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.project_name.is_empty() {
            return json_error_response(400, "InvalidInputException", "projectName is required");
        }

        let mut state = state.write().await;
        match state.create_webhook(
            &input.project_name,
            input.branch_filter.as_deref(),
            input.build_type.as_deref(),
            account_id,
            region,
        ) {
            Ok(wh) => wire::serialize_create_webhook_response(&wire::CreateWebhookOutput {
                webhook: Some(webhook_to_wire(wh)),
            }),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_update_webhook(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_webhook_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.project_name.is_empty() {
            return json_error_response(400, "InvalidInputException", "projectName is required");
        }

        let mut state = state.write().await;
        match state.update_webhook(
            &input.project_name,
            input.branch_filter.as_deref(),
            input.build_type.as_deref(),
        ) {
            Ok(wh) => wire::serialize_update_webhook_response(&wire::UpdateWebhookOutput {
                webhook: Some(webhook_to_wire(wh)),
            }),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_delete_webhook(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_webhook_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.project_name.is_empty() {
            return json_error_response(400, "InvalidInputException", "projectName is required");
        }

        let mut state = state.write().await;
        match state.delete_webhook(&input.project_name) {
            Ok(()) => wire::serialize_delete_webhook_response(&wire::DeleteWebhookOutput {}),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_import_source_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_import_source_credentials_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.token.is_empty() {
            return json_error_response(400, "InvalidInputException", "token is required");
        }
        if input.server_type.is_empty() {
            return json_error_response(400, "InvalidInputException", "serverType is required");
        }
        if input.auth_type.is_empty() {
            return json_error_response(400, "InvalidInputException", "authType is required");
        }

        let mut state = state.write().await;
        match state.import_source_credentials(
            &input.token,
            &input.server_type,
            &input.auth_type,
            input.username.as_deref(),
            account_id,
            region,
        ) {
            Ok(cred) => wire::serialize_import_source_credentials_response(
                &wire::ImportSourceCredentialsOutput {
                    arn: Some(cred.arn.clone()),
                },
            ),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_list_source_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let creds: Vec<wire::SourceCredentialsInfo> = state
            .list_source_credentials()
            .iter()
            .map(|c| wire::SourceCredentialsInfo {
                arn: Some(c.arn.clone()),
                server_type: Some(c.server_type.clone()),
                auth_type: Some(c.auth_type.clone()),
                resource: c.resource.clone(),
            })
            .collect();

        wire::serialize_list_source_credentials_response(&wire::ListSourceCredentialsOutput {
            source_credentials_infos: Some(creds),
        })
    }

    async fn handle_delete_source_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_source_credentials_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return json_error_response(400, "InvalidInputException", "arn is required");
        }

        let mut state = state.write().await;
        match state.delete_source_credentials(&input.arn) {
            Ok(()) => wire::serialize_delete_source_credentials_response(
                &wire::DeleteSourceCredentialsOutput {
                    arn: Some(input.arn.clone()),
                },
            ),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidInputException", "resourceArn is required");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "InvalidInputException", "policy is required");
        }

        let mut state = state.write().await;
        match state.put_resource_policy(&input.resource_arn, &input.policy) {
            Ok(arn) => {
                wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyOutput {
                    resource_arn: Some(arn.to_string()),
                })
            }
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidInputException", "resourceArn is required");
        }

        let state = state.read().await;
        match state.get_resource_policy(&input.resource_arn) {
            Ok(policy) => {
                wire::serialize_get_resource_policy_response(&wire::GetResourcePolicyOutput {
                    policy: Some(policy),
                })
            }
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidInputException", "resourceArn is required");
        }

        let mut state = state.write().await;
        match state.delete_resource_policy(&input.resource_arn) {
            Ok(()) => wire::serialize_delete_resource_policy_response(
                &wire::DeleteResourcePolicyOutput {},
            ),
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_invalidate_project_cache(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_invalidate_project_cache_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Cache invalidation is a no-op in the mock but validates the project exists.
        if !input.project_name.is_empty() {
            let s = state.read().await;
            if !s.projects.contains_key(&input.project_name) {
                let project_name = input.project_name;
                return json_error_response(
                    400,
                    "ResourceNotFoundException",
                    &format!("Project {project_name} not found"),
                );
            }
        }
        wire::serialize_invalidate_project_cache_response(&wire::InvalidateProjectCacheOutput {})
    }

    // ── Report Group handlers ──

    async fn handle_create_report_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_report_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "name is required");
        }
        let report_type = if input.r#type.is_empty() {
            "TEST"
        } else {
            input.r#type.as_str()
        };
        let export_config_type = input.export_config.export_config_type.as_deref();
        let tags = tags_from_wire(input.tags.unwrap_or_default());

        let mut state = state.write().await;
        match state.create_report_group(
            &input.name,
            report_type,
            export_config_type,
            tags,
            account_id,
            region,
        ) {
            Ok(rg) => {
                wire::serialize_create_report_group_response(&wire::CreateReportGroupOutput {
                    report_group: Some(report_group_to_wire(rg)),
                })
            }
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_batch_get_report_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_report_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let arns = input.report_group_arns;

        let state = state.read().await;
        let found = state.batch_get_report_groups(&arns);
        let found_arns: Vec<&str> = found.iter().map(|rg| rg.arn.as_str()).collect();
        let not_found: Vec<String> = arns
            .iter()
            .filter(|arn| !found_arns.contains(&arn.as_str()))
            .cloned()
            .collect();

        wire::serialize_batch_get_report_groups_response(&wire::BatchGetReportGroupsOutput {
            report_groups: Some(found.iter().map(|rg| report_group_to_wire(rg)).collect()),
            report_groups_not_found: Some(not_found),
        })
    }

    async fn handle_list_report_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let arns: Vec<String> = state
            .list_report_groups()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        wire::serialize_list_report_groups_response(&wire::ListReportGroupsOutput {
            report_groups: Some(arns),
            next_token: None,
        })
    }

    async fn handle_delete_report_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_report_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return json_error_response(400, "InvalidInputException", "arn is required");
        }

        let mut state = state.write().await;
        match state.delete_report_group(&input.arn) {
            Ok(()) => {
                wire::serialize_delete_report_group_response(&wire::DeleteReportGroupOutput {})
            }
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_update_report_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_report_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return json_error_response(400, "InvalidInputException", "arn is required");
        }
        let export_config_type = input
            .export_config
            .as_ref()
            .and_then(|c| c.export_config_type.as_deref());
        let tags = input.tags.map(tags_from_wire);

        let mut state = state.write().await;
        match state.update_report_group(&input.arn, export_config_type, tags) {
            Ok(rg) => {
                wire::serialize_update_report_group_response(&wire::UpdateReportGroupOutput {
                    report_group: Some(report_group_to_wire(rg)),
                })
            }
            Err(e) => codebuild_error_response(&e),
        }
    }

    async fn handle_list_reports_for_report_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CodeBuildState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_reports_for_report_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.report_group_arn.is_empty() {
            return json_error_response(400, "InvalidInputException", "reportGroupArn is required");
        }

        let state = state.read().await;
        if !state.report_groups.contains_key(&input.report_group_arn) {
            let arn = input.report_group_arn;
            return json_error_response(
                400,
                "ResourceNotFoundException",
                &format!("Report group {arn} does not exist"),
            );
        }

        wire::serialize_list_reports_for_report_group_response(
            &wire::ListReportsForReportGroupOutput {
                reports: Some(vec![]),
                next_token: None,
            },
        )
    }
}

fn tags_from_wire(tags: Vec<wire::Tag>) -> Vec<crate::types::Tag> {
    tags.into_iter()
        .map(|t| crate::types::Tag {
            key: t.key.unwrap_or_default(),
            value: t.value.unwrap_or_default(),
        })
        .collect()
}

fn report_group_to_wire(rg: &crate::types::ReportGroup) -> wire::ReportGroup {
    let tags: Option<Vec<wire::Tag>> = if rg.tags.is_empty() {
        None
    } else {
        Some(
            rg.tags
                .iter()
                .map(|t| wire::Tag {
                    key: Some(t.key.clone()),
                    value: Some(t.value.clone()),
                })
                .collect(),
        )
    };

    wire::ReportGroup {
        arn: Some(rg.arn.clone()),
        name: Some(rg.name.clone()),
        r#type: Some(rg.r#type.clone()),
        status: Some(rg.status.clone()),
        created: Some(rg.created.timestamp() as f64),
        last_modified: Some(rg.last_modified.timestamp() as f64),
        tags,
        export_config: rg
            .export_config_type
            .as_ref()
            .map(|ect| wire::ReportExportConfig {
                export_config_type: Some(ect.clone()),
                ..Default::default()
            }),
    }
}

fn webhook_to_wire(wh: &crate::types::Webhook) -> wire::Webhook {
    wire::Webhook {
        url: Some(wh.url.clone()),
        branch_filter: wh.branch_filter.clone(),
        build_type: wh.build_type.clone(),
        secret: wh.secret.clone(),
        ..Default::default()
    }
}

fn project_to_wire(p: &crate::types::Project) -> wire::Project {
    let tags: Option<Vec<wire::Tag>> = if p.tags.is_empty() {
        None
    } else {
        Some(
            p.tags
                .iter()
                .map(|t| wire::Tag {
                    key: Some(t.key.clone()),
                    value: Some(t.value.clone()),
                })
                .collect(),
        )
    };

    let artifacts = Some(wire::ProjectArtifacts {
        r#type: p.artifact_type.clone(),
        location: p.artifact_location.clone(),
        ..Default::default()
    });

    wire::Project {
        name: Some(p.name.clone()),
        arn: Some(p.arn.clone()),
        description: if p.description.is_empty() {
            None
        } else {
            Some(p.description.clone())
        },
        source: Some(wire::ProjectSource {
            r#type: p.source_type.clone(),
            location: if p.source_location.is_empty() {
                None
            } else {
                Some(p.source_location.clone())
            },
            ..Default::default()
        }),
        artifacts,
        environment: Some(wire::ProjectEnvironment {
            r#type: p.environment_type.clone(),
            image: p.environment_image.clone(),
            compute_type: p.environment_compute_type.clone(),
            ..Default::default()
        }),
        service_role: Some(p.service_role.clone()),
        tags,
        created: Some(p.created.timestamp() as f64),
        last_modified: Some(p.last_modified.timestamp() as f64),
        ..Default::default()
    }
}

fn build_to_wire(b: &crate::types::Build) -> wire::Build {
    let phases: Vec<wire::BuildPhase> = b
        .phases
        .iter()
        .map(|p| wire::BuildPhase {
            phase_type: Some(p.phase_type.clone()),
            phase_status: p.phase_status.clone(),
            start_time: Some(p.start_time),
            end_time: p.end_time,
            duration_in_seconds: p.duration_in_seconds,
            ..Default::default()
        })
        .collect();

    wire::Build {
        id: Some(b.id.clone()),
        arn: Some(b.arn.clone()),
        project_name: Some(b.project_name.clone()),
        build_status: Some(b.build_status.clone()),
        current_phase: Some(b.current_phase.clone()),
        source_version: Some(b.source_version.clone()),
        source: Some(wire::ProjectSource {
            r#type: b.source_type.clone(),
            location: if b.source_location.is_empty() {
                None
            } else {
                Some(b.source_location.clone())
            },
            ..Default::default()
        }),
        artifacts: if b.artifact_type == "NO_ARTIFACTS" {
            None
        } else {
            Some(wire::BuildArtifacts {
                location: b.artifact_location.clone(),
                ..Default::default()
            })
        },
        environment: Some(wire::ProjectEnvironment {
            r#type: b.environment_type.clone(),
            image: b.environment_image.clone(),
            compute_type: b.environment_compute_type.clone(),
            ..Default::default()
        }),
        service_role: Some(b.service_role.clone()),
        start_time: Some(b.start_time.timestamp() as f64),
        end_time: b.end_time.map(|t| t.timestamp() as f64),
        build_number: Some(b.build_number),
        phases: Some(phases),
        ..Default::default()
    }
}

fn codebuild_error_response(err: &CodeBuildError) -> MockResponse {
    use CodeBuildError::*;
    let (status, error_type) = match err {
        InvalidProjectName => (400, "InvalidInputException"),
        InvalidServiceRole => (400, "InvalidInputException"),
        InvalidBuildId => (400, "InvalidInputException"),
        ProjectAlreadyExists { .. } => (400, "ResourceAlreadyExistsException"),
        ProjectNotFound { .. } => (400, "ResourceNotFoundException"),
        BuildNotFound { .. } => (400, "ResourceNotFoundException"),
        ProjectDoesNotExist { .. } => (400, "ResourceNotFoundException"),
        WebhookAlreadyExists { .. } => (400, "ResourceAlreadyExistsException"),
        WebhookNotFound { .. } => (400, "ResourceNotFoundException"),
        SourceCredentialsNotFound { .. } => (400, "ResourceNotFoundException"),
        ResourcePolicyNotFound { .. } => (400, "ResourceNotFoundException"),
        ReportGroupAlreadyExists { .. } => (400, "ResourceAlreadyExistsException"),
        ReportGroupNotFound { .. } => (400, "ResourceNotFoundException"),
    };
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": err.to_string()}).to_string(),
    )
}
