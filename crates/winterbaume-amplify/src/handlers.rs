use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{AmplifyError, AmplifyState};
use crate::types::SubDomain;
use crate::views::AmplifyStateView;
use crate::wire;

pub struct AmplifyService {
    pub(crate) state: Arc<BackendState<AmplifyState>>,
    pub(crate) notifier: StateChangeNotifier<AmplifyStateView>,
}

impl AmplifyService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AmplifyService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AmplifyService {
    fn service_name(&self) -> &str {
        "amplify"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://amplify\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    MockResponse::error(status, error_type, message)
}

fn service_error_response(e: &AmplifyError) -> MockResponse {
    let (status, error_type) = match e {
        AmplifyError::AppNotFound(_) => (404, "NotFoundException"),
        AmplifyError::BranchNotFound(_, _) => (404, "NotFoundException"),
        AmplifyError::BranchAlreadyExists(_, _) => (400, "BadRequestException"),
        AmplifyError::DomainNotFound(_, _) => (404, "NotFoundException"),
        AmplifyError::DomainAlreadyExists(_, _) => (400, "BadRequestException"),
        AmplifyError::JobNotFound(_) => (404, "NotFoundException"),
    };
    error_response(status, error_type, &e.to_string())
}

fn extract_path(uri: &str) -> &str {
    let path = if let Some(idx) = uri.find('?') {
        &uri[..idx]
    } else {
        uri
    };
    // Strip scheme+host if present
    if let Some(idx) = path.find("://") {
        let after = &path[idx + 3..];
        if let Some(slash) = after.find('/') {
            return &after[slash..];
        }
        return "/";
    }
    path
}

fn extract_query(uri: &str) -> &str {
    uri.split_once('?').map(|(_, q)| q).unwrap_or("")
}

fn percent_decode(s: &str) -> String {
    // Minimal percent-decoding for path segments
    let mut result = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Ok(h), Ok(l)) = (
                std::str::from_utf8(&bytes[i + 1..i + 2]),
                std::str::from_utf8(&bytes[i + 2..i + 3]),
            ) {
                if let Ok(b) = u8::from_str_radix(&format!("{h}{l}"), 16) {
                    result.push(b as char);
                    i += 3;
                    continue;
                }
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

fn app_to_model(a: &crate::types::AmplifyApp) -> model::App {
    model::App {
        app_id: Some(a.app_id.clone()),
        app_arn: Some(a.app_arn.clone()),
        name: Some(a.name.clone()),
        description: a.description.clone(),
        repository: a.repository.clone(),
        platform: a.platform.clone(),
        create_time: Some(a.create_time),
        update_time: Some(a.update_time),
        iam_service_role_arn: a.iam_service_role_arn.clone(),
        environment_variables: Some(a.environment_variables.clone()),
        default_domain: Some(a.default_domain.clone()),
        enable_branch_auto_build: Some(a.enable_branch_auto_build),
        enable_branch_auto_deletion: Some(a.enable_branch_auto_deletion),
        enable_basic_auth: Some(a.enable_basic_auth),
        build_spec: a.build_spec.clone(),
        custom_headers: a.custom_headers.clone(),
        tags: Some(a.tags.clone()),
        ..Default::default()
    }
}

fn branch_to_model(b: &crate::types::AmplifyBranch) -> model::Branch {
    model::Branch {
        branch_arn: Some(b.branch_arn.clone()),
        branch_name: Some(b.branch_name.clone()),
        description: b.description.clone(),
        stage: b.stage.clone(),
        display_name: b.display_name.clone(),
        enable_auto_build: Some(b.enable_auto_build),
        enable_basic_auth: Some(b.enable_basic_auth),
        enable_notification: Some(b.enable_notification),
        enable_performance_mode: Some(b.enable_performance_mode),
        enable_pull_request_preview: Some(b.enable_pull_request_preview),
        environment_variables: Some(b.environment_variables.clone()),
        framework: b.framework.clone(),
        ttl: b.ttl.clone(),
        create_time: Some(b.create_time),
        update_time: Some(b.update_time),
        total_number_of_jobs: Some(b.total_number_of_jobs.clone()),
        active_job_id: b.active_job_id.clone(),
        tags: Some(b.tags.clone()),
        ..Default::default()
    }
}

fn domain_to_model(d: &crate::types::AmplifyDomainAssociation) -> model::DomainAssociation {
    model::DomainAssociation {
        domain_association_arn: Some(d.domain_association_arn.clone()),
        domain_name: Some(d.domain_name.clone()),
        enable_auto_sub_domain: Some(d.enable_auto_sub_domain),
        domain_status: Some(d.domain_status.clone()),
        status_reason: Some(d.status_reason.clone()),
        sub_domains: Some(
            d.sub_domains
                .iter()
                .map(|s| model::SubDomain {
                    sub_domain_setting: Some(model::SubDomainSetting {
                        prefix: s.prefix.clone(),
                        branch_name: s.branch_name.clone(),
                    }),
                    dns_record: s.dns_record.clone(),
                    verified: Some(s.verified),
                })
                .collect(),
        ),
        ..Default::default()
    }
}

async fn job_to_summary(j: &crate::types::AmplifyJob) -> model::JobSummary {
    model::JobSummary {
        job_id: Some(j.job_id.clone()),
        job_arn: Some(j.job_arn.clone()),
        job_type: Some(j.job_type.clone()),
        status: Some(j.status.clone()),
        start_time: Some(j.start_time),
        end_time: j.end_time,
        commit_id: j.commit_id.clone(),
        commit_message: j.commit_message.clone(),
        commit_time: j.commit_time,
        ..Default::default()
    }
}

impl AmplifyService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let account_id = default_account_id();
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(raw_query);
        let method = request.method.as_str();
        let segments_owned: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .map(percent_decode)
            .collect();
        let segments: Vec<&str> = segments_owned.iter().map(|s| s.as_str()).collect();

        let is_mutating = matches!(method, "POST" | "PUT" | "DELETE");

        let response = self
            .route(
                method, &segments, &request, &query_map, &state, account_id, &region,
            )
            .await;

        if is_mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn route(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments) {
            // POST /apps
            ("POST", ["apps"]) => {
                self.handle_create_app(state, request, &[], query, account_id, region)
                    .await
            }
            // GET /apps
            ("GET", ["apps"]) => self.handle_list_apps(state, request, &[], query).await,
            // GET /apps/{appId}
            ("GET", ["apps", app_id]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id)];
                self.handle_get_app(state, request, labels, query).await
            }
            // POST /apps/{appId}
            ("POST", ["apps", app_id]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id)];
                self.handle_update_app(state, request, labels, query).await
            }
            // DELETE /apps/{appId}
            ("DELETE", ["apps", app_id]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id)];
                self.handle_delete_app(state, request, labels, query).await
            }
            // POST /apps/{appId}/branches
            ("POST", ["apps", app_id, "branches"]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id)];
                self.handle_create_branch(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /apps/{appId}/branches
            ("GET", ["apps", app_id, "branches"]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id)];
                self.handle_list_branches(state, request, labels, query)
                    .await
            }
            // GET /apps/{appId}/branches/{branchName}
            ("GET", ["apps", app_id, "branches", branch_name]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("branchName", branch_name)];
                self.handle_get_branch(state, request, labels, query).await
            }
            // POST /apps/{appId}/branches/{branchName}
            ("POST", ["apps", app_id, "branches", branch_name]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("branchName", branch_name)];
                self.handle_update_branch(state, request, labels, query)
                    .await
            }
            // DELETE /apps/{appId}/branches/{branchName}
            ("DELETE", ["apps", app_id, "branches", branch_name]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("branchName", branch_name)];
                self.handle_delete_branch(state, request, labels, query)
                    .await
            }
            // POST /apps/{appId}/domains
            ("POST", ["apps", app_id, "domains"]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id)];
                self.handle_create_domain_association(
                    state, request, labels, query, account_id, region,
                )
                .await
            }
            // GET /apps/{appId}/domains
            ("GET", ["apps", app_id, "domains"]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id)];
                self.handle_list_domain_associations(state, request, labels, query)
                    .await
            }
            // GET /apps/{appId}/domains/{domainName}
            ("GET", ["apps", app_id, "domains", domain_name]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("domainName", domain_name)];
                self.handle_get_domain_association(state, request, labels, query)
                    .await
            }
            // POST /apps/{appId}/domains/{domainName}
            ("POST", ["apps", app_id, "domains", domain_name]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("domainName", domain_name)];
                self.handle_update_domain_association(state, request, labels, query)
                    .await
            }
            // DELETE /apps/{appId}/domains/{domainName}
            ("DELETE", ["apps", app_id, "domains", domain_name]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("domainName", domain_name)];
                self.handle_delete_domain_association(state, request, labels, query)
                    .await
            }
            // POST /apps/{appId}/branches/{branchName}/jobs
            ("POST", ["apps", app_id, "branches", branch_name, "jobs"]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("branchName", branch_name)];
                self.handle_start_job(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /apps/{appId}/branches/{branchName}/jobs
            ("GET", ["apps", app_id, "branches", branch_name, "jobs"]) => {
                let labels: &[(&str, &str)] = &[("appId", app_id), ("branchName", branch_name)];
                self.handle_list_jobs(state, request, labels, query).await
            }
            // GET /apps/{appId}/branches/{branchName}/jobs/{jobId}
            ("GET", ["apps", app_id, "branches", branch_name, "jobs", job_id]) => {
                let labels: &[(&str, &str)] = &[
                    ("appId", app_id),
                    ("branchName", branch_name),
                    ("jobId", job_id),
                ];
                self.handle_get_job(state, request, labels, query).await
            }
            // DELETE /apps/{appId}/branches/{branchName}/jobs/{jobId}/stop
            (
                "DELETE",
                [
                    "apps",
                    app_id,
                    "branches",
                    branch_name,
                    "jobs",
                    job_id,
                    "stop",
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("appId", app_id),
                    ("branchName", branch_name),
                    ("jobId", job_id),
                ];
                self.handle_stop_job(state, request, labels, query).await
            }
            // DELETE /apps/{appId}/branches/{branchName}/jobs/{jobId}
            ("DELETE", ["apps", app_id, "branches", branch_name, "jobs", job_id]) => {
                let labels: &[(&str, &str)] = &[
                    ("appId", app_id),
                    ("branchName", branch_name),
                    ("jobId", job_id),
                ];
                self.handle_delete_job(state, request, labels, query).await
            }
            // GET /tags/{resourceArn}
            ("GET", ["tags", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn)];
                self.handle_list_tags_for_resource(state, request, labels, query)
                    .await
            }
            // POST /tags/{resourceArn}
            ("POST", ["tags", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn)];
                self.handle_tag_resource(state, request, labels, query)
                    .await
            }
            // DELETE /tags/{resourceArn}
            ("DELETE", ["tags", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn)];
                self.handle_untag_resource(state, request, labels, query)
                    .await
            }
            // 501 stubs for unimplemented operations
            ("POST", ["apps", _, "backendenvironments"]) => error_response(
                501,
                "NotImplemented",
                "CreateBackendEnvironment not implemented",
            ),
            ("GET", ["apps", _, "backendenvironments"]) => error_response(
                501,
                "NotImplemented",
                "ListBackendEnvironments not implemented",
            ),
            ("GET", ["apps", _, "backendenvironments", _]) => error_response(
                501,
                "NotImplemented",
                "GetBackendEnvironment not implemented",
            ),
            ("DELETE", ["apps", _, "backendenvironments", _]) => error_response(
                501,
                "NotImplemented",
                "DeleteBackendEnvironment not implemented",
            ),
            ("POST", ["apps", _, "branches", _, "deployments"]) => {
                error_response(501, "NotImplemented", "CreateDeployment not implemented")
            }
            ("POST", ["apps", _, "branches", _, "deployments", "start"]) => {
                error_response(501, "NotImplemented", "StartDeployment not implemented")
            }
            ("POST", ["apps", _, "accesslogs"]) => {
                error_response(501, "NotImplemented", "GenerateAccessLogs not implemented")
            }
            ("GET", ["artifacts", _]) => {
                error_response(501, "NotImplemented", "GetArtifactUrl not implemented")
            }
            ("GET", ["apps", _, "branches", _, "jobs", _, "artifacts"]) => {
                error_response(501, "NotImplemented", "ListArtifacts not implemented")
            }
            ("POST", ["apps", _, "webhooks"]) => {
                error_response(501, "NotImplemented", "CreateWebhook not implemented")
            }
            ("GET", ["apps", _, "webhooks"]) => {
                error_response(501, "NotImplemented", "ListWebhooks not implemented")
            }
            ("GET", ["webhooks", _]) => {
                error_response(501, "NotImplemented", "GetWebhook not implemented")
            }
            ("DELETE", ["webhooks", _]) => {
                error_response(501, "NotImplemented", "DeleteWebhook not implemented")
            }
            ("POST", ["webhooks", _]) => {
                error_response(501, "NotImplemented", "UpdateWebhook not implemented")
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            // POST /apps/{appId}/backendenvironments => CreateBackendEnvironment (not implemented)
            // POST /apps/{appId}/branches/{branchName}/deployments => CreateDeployment (not implemented)
            // POST /apps/{appId}/webhooks => CreateWebhook (not implemented)
            // DELETE /apps/{appId}/backendenvironments/{environmentName} => DeleteBackendEnvironment (not implemented)
            // DELETE /webhooks/{webhookId} => DeleteWebhook (not implemented)
            // POST /apps/{appId}/accesslogs => GenerateAccessLogs (not implemented)
            // GET /artifacts/{artifactId} => GetArtifactUrl (not implemented)
            // GET /apps/{appId}/backendenvironments/{environmentName} => GetBackendEnvironment (not implemented)
            // GET /webhooks/{webhookId} => GetWebhook (not implemented)
            // GET /apps/{appId}/branches/{branchName}/jobs/{jobId}/artifacts => ListArtifacts (not implemented)
            // GET /apps/{appId}/backendenvironments => ListBackendEnvironments (not implemented)
            // GET /apps/{appId}/webhooks => ListWebhooks (not implemented)
            // POST /apps/{appId}/branches/{branchName}/deployments/start => StartDeployment (not implemented)
            // POST /webhooks/{webhookId} => UpdateWebhook (not implemented)

            // 14 unimplemented operations above
            _ => error_response(
                404,
                "NotFoundException",
                &format!("Unknown route: {method} /{}", segments.join("/")),
            ),
        }
    }

    // ---- App handlers ----

    async fn handle_create_app(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_app_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return error_response(400, "BadRequestException", "Missing 'name'");
        }
        let mut state = state.write().await;
        match state.create_app(
            &input.name,
            input.description,
            input.repository,
            input.platform,
            input.iam_service_role_arn,
            input.environment_variables.unwrap_or_default(),
            input.enable_branch_auto_build.unwrap_or(false),
            input.enable_branch_auto_deletion.unwrap_or(false),
            input.enable_basic_auth.unwrap_or(false),
            input.build_spec,
            input.custom_headers,
            input.tags.unwrap_or_default(),
            account_id,
            region,
        ) {
            Ok(app) => wire::serialize_create_app_response(&wire::CreateAppResult {
                app: Some(app_to_model(app)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_app(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_app_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_app(&input.app_id) {
            Ok(app) => wire::serialize_get_app_response(&wire::GetAppResult {
                app: Some(app_to_model(app)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_apps(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_apps_request(request, labels, query) {
            return error_response(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let apps = state.list_apps().into_iter().map(app_to_model).collect();
        wire::serialize_list_apps_response(&wire::ListAppsResult {
            apps: Some(apps),
            next_token: None,
        })
    }

    async fn handle_update_app(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_app_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_app(
            &input.app_id,
            input.name,
            input.description,
            input.repository,
            input.platform,
            input.iam_service_role_arn,
            input.environment_variables,
            input.enable_branch_auto_build,
            input.enable_branch_auto_deletion,
            input.enable_basic_auth,
            input.build_spec,
            input.custom_headers,
        ) {
            Ok(app) => wire::serialize_update_app_response(&wire::UpdateAppResult {
                app: Some(app_to_model(app)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_app(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_app_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_app(&input.app_id) {
            Ok(app) => wire::serialize_delete_app_response(&wire::DeleteAppResult {
                app: Some(app_to_model(&app)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Branch handlers ----

    async fn handle_create_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_branch_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        if input.branch_name.is_empty() {
            return error_response(400, "BadRequestException", "Missing 'branchName'");
        }

        let mut state = state.write().await;
        match state.create_branch(
            &input.app_id,
            &input.branch_name,
            input.description,
            input.stage,
            input.display_name,
            input.enable_auto_build.unwrap_or(true),
            input.enable_basic_auth.unwrap_or(false),
            input.enable_notification.unwrap_or(false),
            input.enable_performance_mode.unwrap_or(false),
            input.enable_pull_request_preview.unwrap_or(false),
            input.environment_variables.unwrap_or_default(),
            input.framework,
            input.ttl,
            input.tags.unwrap_or_default(),
            account_id,
            region,
        ) {
            Ok(branch) => wire::serialize_create_branch_response(&wire::CreateBranchResult {
                branch: Some(branch_to_model(branch)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_branch_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_branch(&input.app_id, &input.branch_name) {
            Ok(branch) => wire::serialize_get_branch_response(&wire::GetBranchResult {
                branch: Some(branch_to_model(branch)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_branches(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_branches_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let branches = state
            .list_branches(&input.app_id)
            .into_iter()
            .map(branch_to_model)
            .collect();
        wire::serialize_list_branches_response(&wire::ListBranchesResult {
            branches: Some(branches),
            next_token: None,
        })
    }

    async fn handle_update_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_branch_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_branch(
            &input.app_id,
            &input.branch_name,
            input.description,
            input.stage,
            input.framework,
            input.enable_auto_build,
            input.enable_notification,
            input.enable_performance_mode,
            input.enable_pull_request_preview,
            input.environment_variables,
            input.ttl,
        ) {
            Ok(branch) => wire::serialize_update_branch_response(&wire::UpdateBranchResult {
                branch: Some(branch_to_model(branch)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_branch(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_branch_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_branch(&input.app_id, &input.branch_name) {
            Ok(branch) => wire::serialize_delete_branch_response(&wire::DeleteBranchResult {
                branch: Some(branch_to_model(&branch)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Domain Association handlers ----

    async fn handle_create_domain_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_domain_association_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return error_response(400, "ValidationException", &e),
            };
        if input.domain_name.is_empty() {
            return error_response(400, "BadRequestException", "Missing 'domainName'");
        }
        let sub_domains: Vec<SubDomain> = input
            .sub_domain_settings
            .into_iter()
            .map(|s| SubDomain {
                prefix: s.prefix,
                branch_name: s.branch_name,
                dns_record: None,
                verified: false,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_domain_association(
            &input.app_id,
            &input.domain_name,
            input.enable_auto_sub_domain.unwrap_or(false),
            sub_domains,
            account_id,
            region,
        ) {
            Ok(domain) => wire::serialize_create_domain_association_response(
                &wire::CreateDomainAssociationResult {
                    domain_association: Some(domain_to_model(domain)),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_domain_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_domain_association_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_domain_association(&input.app_id, &input.domain_name) {
            Ok(domain) => {
                wire::serialize_get_domain_association_response(&wire::GetDomainAssociationResult {
                    domain_association: Some(domain_to_model(domain)),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_domain_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_domain_associations_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let domains = state
            .list_domain_associations(&input.app_id)
            .into_iter()
            .map(domain_to_model)
            .collect();
        wire::serialize_list_domain_associations_response(&wire::ListDomainAssociationsResult {
            domain_associations: Some(domains),
            next_token: None,
        })
    }

    async fn handle_update_domain_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_domain_association_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return error_response(400, "ValidationException", &e),
            };
        let sub_domains: Option<Vec<SubDomain>> = input.sub_domain_settings.map(|arr| {
            arr.into_iter()
                .map(|s| SubDomain {
                    prefix: s.prefix,
                    branch_name: s.branch_name,
                    dns_record: None,
                    verified: false,
                })
                .collect()
        });

        let mut state_w = state.write().await;
        match state_w.update_domain_association(
            &input.app_id,
            &input.domain_name,
            input.enable_auto_sub_domain,
            sub_domains,
        ) {
            Ok(domain) => wire::serialize_update_domain_association_response(
                &wire::UpdateDomainAssociationResult {
                    domain_association: Some(domain_to_model(domain)),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_domain_association(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_domain_association_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return error_response(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_domain_association(&input.app_id, &input.domain_name) {
            Ok(domain) => wire::serialize_delete_domain_association_response(
                &wire::DeleteDomainAssociationResult {
                    domain_association: Some(domain_to_model(&domain)),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Job handlers ----

    async fn handle_start_job(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        if input.job_type.is_empty() {
            return error_response(400, "BadRequestException", "Missing 'jobType'");
        }
        let mut state = state.write().await;
        match state.start_job(
            &input.app_id,
            &input.branch_name,
            &input.job_type,
            input.commit_id,
            input.commit_message,
            input.commit_time,
            account_id,
            region,
        ) {
            Ok(job) => wire::serialize_start_job_response(&wire::StartJobResult {
                job_summary: Some(job_to_summary(job).await),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_job(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_job(&input.app_id, &input.branch_name, &input.job_id) {
            Ok(job) => wire::serialize_get_job_response(&wire::GetJobResult {
                job: Some(model::Job {
                    summary: Some(job_to_summary(job).await),
                    steps: Some(vec![]),
                }),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_jobs_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let mut summaries = Vec::new();
        for job in state.list_jobs(&input.app_id, &input.branch_name) {
            summaries.push(job_to_summary(job).await);
        }
        wire::serialize_list_jobs_response(&wire::ListJobsResult {
            job_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_stop_job(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.stop_job(&input.app_id, &input.branch_name, &input.job_id) {
            Ok(job) => wire::serialize_stop_job_response(&wire::StopJobResult {
                job_summary: Some(job_to_summary(job).await),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_job(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        // Get and mark as deleted (we treat delete as stop + remove from state here)
        let mut state = state.write().await;
        let key = (
            input.app_id.clone(),
            input.branch_name.clone(),
            input.job_id.clone(),
        );
        match state.jobs.remove(&key) {
            Some(job) => wire::serialize_delete_job_response(&wire::DeleteJobResult {
                job_summary: Some(job_to_summary(&job).await),
            }),
            None => service_error_response(&AmplifyError::JobNotFound(input.job_id.clone())),
        }
    }

    // ---- Tag handlers ----

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse { tags: Some(tags) },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AmplifyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }
}
