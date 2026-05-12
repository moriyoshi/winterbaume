use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    urldecode,
};

use crate::state::{EmrServerlessError, EmrServerlessState};
use crate::types::*;
use crate::views::EmrServerlessStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct EmrServerlessService {
    pub(crate) state: Arc<BackendState<EmrServerlessState>>,
    pub(crate) notifier: StateChangeNotifier<EmrServerlessStateView>,
}

impl EmrServerlessService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for EmrServerlessService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EmrServerlessService {
    fn service_name(&self) -> &str {
        "emr-serverless"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://emr-serverless\..*\.amazonaws\.com",
            r"https?://emr-serverless\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EmrServerlessService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();
        let query_map: HashMap<String, String> = parse_query_string(&query_string);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Routes:
        // POST   /applications                                          - CreateApplication
        // GET    /applications                                          - ListApplications
        // GET    /applications/{applicationId}                          - GetApplication
        // PATCH  /applications/{applicationId}                          - UpdateApplication
        // DELETE /applications/{applicationId}                          - DeleteApplication
        // PUT    /applications/{applicationId}/start                    - StartApplication
        // PUT    /applications/{applicationId}/stop                     - StopApplication
        // POST   /applications/{applicationId}/jobruns                  - StartJobRun
        // GET    /applications/{applicationId}/jobruns                  - ListJobRuns
        // GET    /applications/{applicationId}/jobruns/{jobRunId}       - GetJobRun
        // PATCH  /applications/{applicationId}/jobruns/{jobRunId}/cancel - CancelJobRun

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        use winterbaume_core::StatefulService;
        let response = if segments[0] == "tags" {
            // Tag operations: /tags/{resourceArn}
            let resource_arn = if segments.len() >= 2 {
                // The ARN may have been split on '/' - rejoin everything after "tags/"
                // and URL-decode (the SDK percent-encodes ARN colons in path params).
                urldecode(&segments[1..].join("/"))
            } else {
                return rest_json_error(400, "ValidationException", "Missing resourceArn");
            };
            let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
            match method {
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
                _ => rest_json_error(405, "UnknownOperationException", "Method not allowed"),
            }
        } else if segments[0] == "applications" {
            match (method, segments.len()) {
                // POST /applications - CreateApplication
                ("POST", 1) => {
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
                // GET /applications - ListApplications
                ("GET", 1) => {
                    self.handle_list_applications(&state, &request, &[], &query_map)
                        .await
                }
                // GET /applications/{id} - GetApplication
                ("GET", 2) => {
                    let app_id = segments[1];
                    let labels: &[(&str, &str)] = &[("applicationId", app_id)];
                    self.handle_get_application(&state, &request, labels, &query_map)
                        .await
                }
                // PATCH /applications/{id} - UpdateApplication
                ("PATCH", 2) => {
                    let app_id = segments[1];
                    let labels: &[(&str, &str)] = &[("applicationId", app_id)];
                    self.handle_update_application(&state, &request, labels, &query_map)
                        .await
                }
                // DELETE /applications/{id} - DeleteApplication
                ("DELETE", 2) => {
                    let app_id = segments[1];
                    let labels: &[(&str, &str)] = &[("applicationId", app_id)];
                    self.handle_delete_application(&state, &request, labels, &query_map)
                        .await
                }
                // POST/PUT /applications/{id}/start or /applications/{id}/stop
                ("POST" | "PUT", 3) if segments[2] == "start" => {
                    let app_id = segments[1];
                    let labels: &[(&str, &str)] = &[("applicationId", app_id)];
                    self.handle_start_application(&state, &request, labels, &query_map)
                        .await
                }
                ("POST" | "PUT", 3) if segments[2] == "stop" => {
                    let app_id = segments[1];
                    let labels: &[(&str, &str)] = &[("applicationId", app_id)];
                    self.handle_stop_application(&state, &request, labels, &query_map)
                        .await
                }
                // POST /applications/{id}/jobruns - StartJobRun
                ("POST", 3) if segments[2] == "jobruns" => {
                    let app_id = segments[1];
                    let labels: &[(&str, &str)] = &[("applicationId", app_id)];
                    self.handle_start_job_run(
                        &state, &request, labels, &query_map, account_id, &region,
                    )
                    .await
                }
                // GET /applications/{id}/jobruns - ListJobRuns
                ("GET", 3) if segments[2] == "jobruns" => {
                    let app_id = segments[1];
                    let labels: &[(&str, &str)] = &[("applicationId", app_id)];
                    self.handle_list_job_runs(&state, &request, labels, &query_map)
                        .await
                }
                // GET /applications/{id}/jobruns/{jobRunId} - GetJobRun
                ("GET", 4) if segments[2] == "jobruns" => {
                    let app_id = segments[1];
                    let job_run_id = segments[3];
                    let labels: &[(&str, &str)] =
                        &[("applicationId", app_id), ("jobRunId", job_run_id)];
                    self.handle_get_job_run(&state, &request, labels, &query_map)
                        .await
                }
                // DELETE /applications/{id}/jobruns/{jobRunId} - CancelJobRun
                ("DELETE", 4) if segments[2] == "jobruns" => {
                    let app_id = segments[1];
                    let job_run_id = segments[3];
                    let labels: &[(&str, &str)] =
                        &[("applicationId", app_id), ("jobRunId", job_run_id)];
                    self.handle_cancel_job_run(&state, &request, labels, &query_map)
                        .await
                }
                // PATCH /applications/{id}/jobruns/{jobRunId}/cancel - CancelJobRun (alt)
                ("PATCH", 5) if segments[2] == "jobruns" && segments[4] == "cancel" => {
                    let app_id = segments[1];
                    let job_run_id = segments[3];
                    let labels: &[(&str, &str)] =
                        &[("applicationId", app_id), ("jobRunId", job_run_id)];
                    self.handle_cancel_job_run(&state, &request, labels, &query_map)
                        .await
                }
                // GET /applications/{id}/jobruns/{jobRunId}/dashboard - GetDashboardForJobRun
                ("GET", 5) if segments[2] == "jobruns" && segments[4] == "dashboard" => {
                    let app_id = segments[1];
                    let job_run_id = segments[3];
                    let labels: &[(&str, &str)] =
                        &[("applicationId", app_id), ("jobRunId", job_run_id)];
                    self.handle_get_dashboard_for_job_run(&state, &request, labels, &query_map)
                        .await
                }
                // GET /applications/{id}/jobruns/{jobRunId}/attempts - ListJobRunAttempts
                ("GET", 5) if segments[2] == "jobruns" && segments[4] == "attempts" => {
                    let app_id = segments[1];
                    let job_run_id = segments[3];
                    let labels: &[(&str, &str)] =
                        &[("applicationId", app_id), ("jobRunId", job_run_id)];
                    self.handle_list_job_run_attempts(&state, &request, labels, &query_map)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            rest_json_error(404, "UnknownOperationException", "Not found")
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_application(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON body"),
        };

        let name = input.name.as_deref().unwrap_or("");
        let app_type = input.r#type.as_str();
        let release_label = if input.release_label.is_empty() {
            DEFAULT_RELEASE_LABEL
        } else {
            input.release_label.as_str()
        };

        let initial_capacity = wire_initial_capacity_to_internal(input.initial_capacity.as_ref());
        let maximum_capacity = wire_maximum_capacity_to_internal(input.maximum_capacity.as_ref());
        let network_configuration =
            wire_network_configuration_to_internal(input.network_configuration.as_ref());
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_application(
            name,
            app_type,
            release_label,
            account_id,
            region,
            initial_capacity,
            maximum_capacity,
            network_configuration,
            tags,
        ) {
            Ok(app) => {
                let resp = wire::CreateApplicationResponse {
                    application_id: Some(app.application_id.clone()),
                    name: Some(app.name.clone()),
                    arn: Some(app.arn.clone()),
                };
                wire::serialize_create_application_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_get_application(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_application(&input.application_id) {
            Ok(app) => {
                let resp = wire::GetApplicationResponse {
                    application: Some(application_to_wire(app)),
                };
                wire::serialize_get_application_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_delete_application(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_application(&input.application_id) {
            Ok(()) => {
                let resp = wire::DeleteApplicationResponse {};
                wire::serialize_delete_application_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_update_application(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_application_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON body"),
        };

        let initial_capacity = wire_initial_capacity_to_internal(input.initial_capacity.as_ref());
        let maximum_capacity = wire_maximum_capacity_to_internal(input.maximum_capacity.as_ref());
        let auto_start = input
            .auto_start_configuration
            .as_ref()
            .map(|v| AutoStartConfig {
                enabled: v.enabled.unwrap_or(true),
            });
        let auto_stop = input
            .auto_stop_configuration
            .as_ref()
            .map(|v| AutoStopConfig {
                enabled: v.enabled.unwrap_or(true),
                idle_timeout_minutes: v.idle_timeout_minutes.map(|n| n as i64).unwrap_or(15),
            });
        let network_configuration =
            wire_network_configuration_to_internal(input.network_configuration.as_ref());

        let mut state = state.write().await;
        match state.update_application(
            &input.application_id,
            initial_capacity,
            maximum_capacity,
            auto_start,
            auto_stop,
            network_configuration,
        ) {
            Ok(app) => {
                let resp = wire::UpdateApplicationResponse {
                    application: Some(application_to_wire(app)),
                };
                wire::serialize_update_application_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_list_applications(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_applications_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let states: Option<Vec<String>> = input.states.clone();
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (apps, token) = state.list_applications(states.as_deref(), max_results, next_token);

        let entries: Vec<wire::ApplicationSummary> = apps
            .iter()
            .map(|a| application_summary_to_wire(a))
            .collect();

        let resp = wire::ListApplicationsResponse {
            applications: Some(entries),
            next_token: token,
        };
        wire::serialize_list_applications_response(&resp)
    }

    async fn handle_start_application(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.start_application(&input.application_id) {
            Ok(()) => {
                let resp = wire::StartApplicationResponse {};
                wire::serialize_start_application_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_stop_application(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_application_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.stop_application(&input.application_id) {
            Ok(()) => {
                let resp = wire::StopApplicationResponse {};
                wire::serialize_stop_application_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_start_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_job_run_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON body"),
        };

        let name = input.name.as_deref();
        let execution_role_arn = input.execution_role_arn.as_str();
        let execution_timeout_minutes = input.execution_timeout_minutes;

        let job_driver = wire_job_driver_to_internal(input.job_driver.as_ref());
        let configuration_overrides =
            wire_configuration_overrides_to_internal(input.configuration_overrides.as_ref());
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.start_job_run(
            &input.application_id,
            name,
            execution_role_arn,
            job_driver,
            configuration_overrides,
            tags,
            execution_timeout_minutes,
            account_id,
            region,
        ) {
            Ok(job_run) => {
                let resp = wire::StartJobRunResponse {
                    application_id: Some(job_run.application_id.clone()),
                    job_run_id: Some(job_run.job_run_id.clone()),
                    arn: Some(job_run.arn.clone()),
                };
                wire::serialize_start_job_run_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_get_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_job_run_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_job_run(&input.application_id, &input.job_run_id) {
            Ok(run) => {
                let resp = wire::GetJobRunResponse {
                    job_run: Some(job_run_to_wire(run)),
                };
                wire::serialize_get_job_run_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_list_job_runs(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_job_runs_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let states: Option<Vec<String>> = input.states.clone();
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        match state.list_job_runs(
            &input.application_id,
            states.as_deref(),
            max_results,
            next_token,
        ) {
            Ok((runs, token)) => {
                let entries: Vec<wire::JobRunSummary> =
                    runs.iter().map(|r| job_run_summary_to_wire(r)).collect();

                let resp = wire::ListJobRunsResponse {
                    job_runs: Some(entries),
                    next_token: token,
                };
                wire::serialize_list_job_runs_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_cancel_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_job_run_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.cancel_job_run(&input.application_id, &input.job_run_id) {
            Ok(()) => {
                let resp = wire::CancelJobRunResponse {
                    application_id: Some(input.application_id.clone()),
                    job_run_id: Some(input.job_run_id.clone()),
                };
                wire::serialize_cancel_job_run_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_get_dashboard_for_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_dashboard_for_job_run_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        // Verify the job run exists
        let state = state.read().await;
        match state.get_job_run(&input.application_id, &input.job_run_id) {
            Ok(_) => {
                let resp = wire::GetDashboardForJobRunResponse {
                    url: Some(format!(
                        "https://mock-dashboard.emrserverless.local/{}/{}",
                        input.application_id, input.job_run_id
                    )),
                };
                wire::serialize_get_dashboard_for_job_run_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_list_job_run_attempts(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_job_run_attempts_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Verify the job run exists
        let state = state.read().await;
        match state.get_job_run(&input.application_id, &input.job_run_id) {
            Ok(_) => {
                let resp = wire::ListJobRunAttemptsResponse {
                    job_run_attempts: Some(vec![]),
                    next_token: None,
                };
                wire::serialize_list_job_run_attempts_response(&resp)
            }
            Err(e) => emr_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        let resp = wire::ListTagsForResourceResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
        };
        wire::serialize_list_tags_for_resource_response(&resp)
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        let resp = wire::TagResourceResponse {};
        wire::serialize_tag_resource_response(&resp)
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrServerlessState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &input.tag_keys);
        let resp = wire::UntagResourceResponse {};
        wire::serialize_untag_resource_response(&resp)
    }
}

// --- Wire-to-internal conversions ---

fn wire_initial_capacity_to_internal(
    ic: Option<&HashMap<String, wire::InitialCapacityConfig>>,
) -> Option<HashMap<String, InitialCapacityConfig>> {
    ic.map(|m| {
        m.iter()
            .map(|(k, v)| {
                let worker_configuration =
                    v.worker_configuration
                        .as_ref()
                        .map(|wc| WorkerConfiguration {
                            cpu: wc.cpu.clone(),
                            memory: wc.memory.clone(),
                            disk: wc.disk.clone().unwrap_or_default(),
                        });
                (
                    k.clone(),
                    InitialCapacityConfig {
                        worker_count: v.worker_count,
                        worker_configuration,
                    },
                )
            })
            .collect()
    })
}

fn wire_maximum_capacity_to_internal(
    mc: Option<&wire::MaximumAllowedResources>,
) -> Option<MaximumCapacity> {
    mc.map(|m| MaximumCapacity {
        cpu: m.cpu.clone(),
        memory: m.memory.clone(),
        disk: m.disk.clone().unwrap_or_default(),
    })
}

fn wire_network_configuration_to_internal(
    nc: Option<&wire::NetworkConfiguration>,
) -> Option<NetworkConfiguration> {
    nc.map(|n| NetworkConfiguration {
        subnet_ids: n.subnet_ids.clone().unwrap_or_default(),
        security_group_ids: n.security_group_ids.clone().unwrap_or_default(),
    })
}

fn wire_job_driver_to_internal(jd: Option<&wire::JobDriver>) -> JobDriver {
    let spark_submit = jd
        .and_then(|j| j.spark_submit.as_ref())
        .map(|ss| SparkSubmit {
            entry_point: ss.entry_point.clone(),
            entry_point_arguments: ss.entry_point_arguments.clone().unwrap_or_default(),
            spark_submit_parameters: ss.spark_submit_parameters.clone(),
        });
    JobDriver { spark_submit }
}

fn wire_configuration_overrides_to_internal(
    co: Option<&wire::ConfigurationOverrides>,
) -> Option<ConfigurationOverrides> {
    co.map(|c| ConfigurationOverrides {
        monitoring_configuration: c.monitoring_configuration.as_ref().map(|mc| {
            MonitoringConfiguration {
                s3_monitoring_configuration: mc.s3_monitoring_configuration.as_ref().map(|s3| {
                    S3MonitoringConfiguration {
                        log_uri: s3.log_uri.clone().unwrap_or_default(),
                    }
                }),
            }
        }),
    })
}

// --- State-to-wire conversion helpers ---

fn parse_epoch(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or(0.0)
}

fn application_to_wire(app: &Application) -> wire::Application {
    let tags = if app.tags.is_empty() {
        None
    } else {
        Some(app.tags.clone())
    };

    let initial_capacity = app.initial_capacity.as_ref().map(|ic| {
        ic.iter()
            .map(|(key, config)| {
                let wc =
                    config
                        .worker_configuration
                        .as_ref()
                        .map(|wc| wire::WorkerResourceConfig {
                            cpu: wc.cpu.clone(),
                            memory: wc.memory.clone(),
                            disk: if wc.disk.is_empty() {
                                None
                            } else {
                                Some(wc.disk.clone())
                            },
                            disk_type: None,
                        });
                (
                    key.clone(),
                    wire::InitialCapacityConfig {
                        worker_count: config.worker_count,
                        worker_configuration: wc,
                    },
                )
            })
            .collect()
    });

    let maximum_capacity = app
        .maximum_capacity
        .as_ref()
        .map(|mc| wire::MaximumAllowedResources {
            cpu: mc.cpu.clone(),
            memory: mc.memory.clone(),
            disk: if mc.disk.is_empty() {
                None
            } else {
                Some(mc.disk.clone())
            },
        });

    let network_configuration =
        app.network_configuration
            .as_ref()
            .map(|nc| wire::NetworkConfiguration {
                subnet_ids: if nc.subnet_ids.is_empty() {
                    None
                } else {
                    Some(nc.subnet_ids.clone())
                },
                security_group_ids: if nc.security_group_ids.is_empty() {
                    None
                } else {
                    Some(nc.security_group_ids.clone())
                },
            });

    wire::Application {
        application_id: Some(app.application_id.clone()),
        name: Some(app.name.clone()),
        arn: Some(app.arn.clone()),
        release_label: Some(app.release_label.clone()),
        r#type: Some(app.application_type.clone()),
        state: Some(app.state.clone()),
        state_details: if app.state_details.is_empty() {
            None
        } else {
            Some(app.state_details.clone())
        },
        created_at: Some(parse_epoch(&app.created_at)),
        updated_at: Some(parse_epoch(&app.updated_at)),
        tags,
        auto_start_configuration: Some(wire::AutoStartConfig {
            enabled: Some(app.auto_start_configuration.enabled),
        }),
        auto_stop_configuration: Some(wire::AutoStopConfig {
            enabled: Some(app.auto_stop_configuration.enabled),
            idle_timeout_minutes: Some(app.auto_stop_configuration.idle_timeout_minutes as i32),
        }),
        initial_capacity,
        maximum_capacity,
        network_configuration,
        ..Default::default()
    }
}

fn application_summary_to_wire(app: &Application) -> wire::ApplicationSummary {
    wire::ApplicationSummary {
        id: Some(app.application_id.clone()),
        name: Some(app.name.clone()),
        arn: Some(app.arn.clone()),
        release_label: Some(app.release_label.clone()),
        r#type: Some(app.application_type.clone()),
        state: Some(app.state.clone()),
        state_details: if app.state_details.is_empty() {
            None
        } else {
            Some(app.state_details.clone())
        },
        created_at: Some(parse_epoch(&app.created_at)),
        updated_at: Some(parse_epoch(&app.updated_at)),
        ..Default::default()
    }
}

fn job_run_to_wire(run: &JobRun) -> wire::JobRun {
    let job_driver = run
        .job_driver
        .spark_submit
        .as_ref()
        .map(|ss| wire::JobDriver {
            spark_submit: Some(wire::SparkSubmit {
                entry_point: ss.entry_point.clone(),
                entry_point_arguments: if ss.entry_point_arguments.is_empty() {
                    None
                } else {
                    Some(ss.entry_point_arguments.clone())
                },
                spark_submit_parameters: ss.spark_submit_parameters.clone(),
            }),
            hive: None,
        });

    let configuration_overrides = run.configuration_overrides.as_ref().and_then(|co| {
        co.monitoring_configuration.as_ref().and_then(|mc| {
            mc.s3_monitoring_configuration
                .as_ref()
                .map(|s3| wire::ConfigurationOverrides {
                    monitoring_configuration: Some(wire::MonitoringConfiguration {
                        s3_monitoring_configuration: Some(wire::S3MonitoringConfiguration {
                            log_uri: Some(s3.log_uri.clone()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
        })
    });

    let tags = if run.tags.is_empty() {
        None
    } else {
        Some(run.tags.clone())
    };

    wire::JobRun {
        application_id: Some(run.application_id.clone()),
        job_run_id: Some(run.job_run_id.clone()),
        name: if run.name.is_empty() {
            None
        } else {
            Some(run.name.clone())
        },
        arn: Some(run.arn.clone()),
        created_by: Some(String::new()),
        created_at: Some(parse_epoch(&run.created_at)),
        updated_at: Some(parse_epoch(&run.updated_at)),
        execution_role: Some(run.execution_role_arn.clone()),
        state: Some(run.state.clone()),
        state_details: if run.state_details.is_empty() {
            None
        } else {
            Some(run.state_details.clone())
        },
        release_label: Some(String::new()),
        job_driver,
        configuration_overrides,
        tags,
        mode: None,
        execution_timeout_minutes: run.execution_timeout_minutes,
        ..Default::default()
    }
}

fn job_run_summary_to_wire(run: &JobRun) -> wire::JobRunSummary {
    wire::JobRunSummary {
        application_id: Some(run.application_id.clone()),
        id: Some(run.job_run_id.clone()),
        name: if run.name.is_empty() {
            None
        } else {
            Some(run.name.clone())
        },
        arn: Some(run.arn.clone()),
        created_by: Some(String::new()),
        created_at: Some(parse_epoch(&run.created_at)),
        updated_at: Some(parse_epoch(&run.updated_at)),
        execution_role: Some(run.execution_role_arn.clone()),
        state: Some(run.state.clone()),
        state_details: if run.state_details.is_empty() {
            None
        } else {
            Some(run.state_details.clone())
        },
        release_label: Some(String::new()),
        r#type: Some(String::new()),
        ..Default::default()
    }
}

// --- Utility functions ---

fn extract_path_and_query(uri: &str) -> (String, String) {
    let path_and_query = if let Some(pos) = uri.find("://") {
        let rest = &uri[pos + 3..];
        if let Some(pos) = rest.find('/') {
            &rest[pos..]
        } else {
            "/"
        }
    } else {
        uri
    };

    if let Some(pos) = path_and_query.find('?') {
        (
            path_and_query[..pos].to_string(),
            path_and_query[pos + 1..].to_string(),
        )
    } else {
        (path_and_query.to_string(), String::new())
    }
}

fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if query.is_empty() {
        return map;
    }
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let k = urlencoding::decode(k).unwrap_or_default().to_string();
            let v = urlencoding::decode(v).unwrap_or_default().to_string();
            map.insert(k, v);
        }
    }
    map
}

fn emr_error_response(err: &EmrServerlessError) -> MockResponse {
    let (status, error_type) = match err {
        EmrServerlessError::UnsupportedEngine { .. } => (400, "ValidationException"),
        EmrServerlessError::UnsupportedReleaseLabel { .. } => (400, "ValidationException"),
        EmrServerlessError::ApplicationNotFound { .. } => (404, "ResourceNotFoundException"),
        EmrServerlessError::InvalidApplicationState { .. } => (400, "ValidationException"),
        EmrServerlessError::JobRunNotFound { .. } => (404, "ResourceNotFoundException"),
        EmrServerlessError::CrossAccountPassRole => (403, "AccessDeniedException"),
        EmrServerlessError::InvalidExecutionTimeout => (400, "ValidationException"),
    };
    let body = json!({
        "Message": err.to_string(),
    });
    let mut response = MockResponse::rest_json(status, body.to_string());
    response
        .headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    response
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Message": message,
    });
    let mut response = MockResponse::rest_json(status, body.to_string());
    response
        .headers
        .insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    response
}
