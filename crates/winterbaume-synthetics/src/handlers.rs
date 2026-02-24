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

use crate::state::{SyntheticsError, SyntheticsState};
use crate::views::SyntheticsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SyntheticsService {
    pub(crate) state: Arc<BackendState<SyntheticsState>>,
    pub(crate) notifier: StateChangeNotifier<SyntheticsStateView>,
}

impl SyntheticsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SyntheticsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SyntheticsService {
    fn service_name(&self) -> &str {
        "synthetics"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://synthetics\..*\.amazonaws\.com",
            r"https?://synthetics\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn group_to_wire(g: &crate::types::Group) -> wire::Group {
    let tags = if g.tags.is_empty() {
        None
    } else {
        Some(g.tags.clone())
    };
    wire::Group {
        name: Some(g.name.clone()),
        id: Some(g.id.clone()),
        arn: Some(g.arn.clone()),
        created_time: Some(g.created_time.timestamp() as f64),
        last_modified_time: Some(g.last_modified_time.timestamp() as f64),
        tags,
    }
}

async fn canary_to_wire(c: &crate::types::Canary) -> wire::Canary {
    let tags = if c.tags.is_empty() {
        None
    } else {
        Some(c.tags.clone())
    };
    wire::Canary {
        name: Some(c.name.clone()),
        id: Some(c.id.clone()),
        execution_role_arn: Some(c.execution_role_arn.clone()),
        artifact_s3_location: Some(c.artifact_s3_location.clone()),
        runtime_version: Some(c.runtime_version.clone()),
        success_retention_period_in_days: Some(c.success_retention_period_in_days),
        failure_retention_period_in_days: Some(c.failure_retention_period_in_days),
        status: Some(wire::CanaryStatus {
            state: Some(c.status.state.clone()),
            state_reason: c.status.state_reason.clone(),
            state_reason_code: c.status.state_reason_code.clone(),
        }),
        schedule: Some(wire::CanaryScheduleOutput {
            expression: Some(c.schedule_expression.clone()),
            duration_in_seconds: c.schedule_duration_in_seconds,
            ..Default::default()
        }),
        code: Some(wire::CanaryCodeOutput {
            handler: Some(c.handler.clone()),
            ..Default::default()
        }),
        tags,
        ..Default::default()
    }
}

impl SyntheticsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let response = match (method, segments.as_slice()) {
            // POST /canary - CreateCanary
            ("POST", ["canary"]) => {
                self.handle_create_canary(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // GET /canary/{name} - GetCanary
            ("GET", ["canary", name]) => {
                let decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", decoded.as_str())];
                self.handle_get_canary(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /canary/{name} - DeleteCanary
            ("DELETE", ["canary", name]) => {
                let decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", decoded.as_str())];
                self.handle_delete_canary(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /canary/{name} - UpdateCanary
            ("PATCH", ["canary", name]) => {
                let decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", decoded.as_str())];
                self.handle_update_canary(&state, &request, labels, &query_map)
                    .await
            }
            // POST /canary/{name}/start - StartCanary
            ("POST", ["canary", name, "start"]) => {
                let decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", decoded.as_str())];
                self.handle_start_canary(&state, &request, labels, &query_map)
                    .await
            }
            // POST /canary/{name}/stop - StopCanary
            ("POST", ["canary", name, "stop"]) => {
                let decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", decoded.as_str())];
                self.handle_stop_canary(&state, &request, labels, &query_map)
                    .await
            }
            // POST /canary/{name}/dry-run/start - StartCanaryDryRun
            ("POST", ["canary", name, "dry-run", "start"]) => {
                let decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", decoded.as_str())];
                self.handle_start_canary_dry_run(&state, &request, labels, &query_map)
                    .await
            }
            // POST /canary/{name}/runs - GetCanaryRuns
            ("POST", ["canary", name, "runs"]) => {
                let decoded = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", decoded.as_str())];
                self.handle_get_canary_runs(&state, &request, labels, &query_map)
                    .await
            }
            // POST /canaries - DescribeCanaries
            ("POST", ["canaries"]) => {
                self.handle_describe_canaries(&state, &request, &[], &query_map)
                    .await
            }
            // POST /canaries/last-run - DescribeCanariesLastRun
            ("POST", ["canaries", "last-run"]) => {
                self.handle_describe_canaries_last_run(&state, &request, &[], &query_map)
                    .await
            }
            // POST /runtime-versions - DescribeRuntimeVersions
            ("POST", ["runtime-versions"]) => {
                self.handle_describe_runtime_versions(&request, &[], &query_map)
                    .await
            }
            // GET /tags/{resource_arn} - ListTagsForResource
            ("GET", ["tags", resource_arn]) => {
                let decoded = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("ResourceArn", decoded.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            // POST /tags/{resource_arn} - TagResource
            ("POST", ["tags", resource_arn]) => {
                let decoded = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("ResourceArn", decoded.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /tags/{resource_arn} - UntagResource
            ("DELETE", ["tags", resource_arn]) => {
                let decoded = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("ResourceArn", decoded.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map, &raw_query)
                    .await
            }
            // POST /group - CreateGroup
            ("POST", ["group"]) => {
                self.handle_create_group(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // GET /group/{identifier} - GetGroup
            ("GET", ["group", identifier]) => {
                let decoded = percent_decode(identifier);
                let labels: &[(&str, &str)] = &[("GroupIdentifier", decoded.as_str())];
                self.handle_get_group(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /group/{identifier} - DeleteGroup
            ("DELETE", ["group", identifier]) => {
                let decoded = percent_decode(identifier);
                let labels: &[(&str, &str)] = &[("GroupIdentifier", decoded.as_str())];
                self.handle_delete_group(&state, &request, labels, &query_map)
                    .await
            }
            // POST /groups - ListGroups
            ("POST", ["groups"]) => {
                self.handle_list_groups(&state, &request, &[], &query_map)
                    .await
            }
            // PATCH /group/{identifier}/associate - AssociateResource
            ("PATCH", ["group", identifier, "associate"]) => {
                let decoded = percent_decode(identifier);
                let labels: &[(&str, &str)] = &[("GroupIdentifier", decoded.as_str())];
                self.handle_associate_resource(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /group/{identifier}/disassociate - DisassociateResource
            ("PATCH", ["group", identifier, "disassociate"]) => {
                let decoded = percent_decode(identifier);
                let labels: &[(&str, &str)] = &[("GroupIdentifier", decoded.as_str())];
                self.handle_disassociate_resource(&state, &request, labels, &query_map)
                    .await
            }
            // POST /group/{identifier}/resources - ListGroupResources
            ("POST", ["group", identifier, "resources"]) => {
                let decoded = percent_decode(identifier);
                let labels: &[(&str, &str)] = &[("GroupIdentifier", decoded.as_str())];
                self.handle_list_group_resources(&state, &request, labels, &query_map)
                    .await
            }
            // POST /resource/{resource_arn}/groups - ListAssociatedGroups
            ("POST", ["resource", resource_arn, "groups"]) => {
                let decoded = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("ResourceArn", decoded.as_str())];
                self.handle_list_associated_groups(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_canary(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        use chrono::Utc;

        use crate::types::{Canary, CanaryStatus};
        let input = match wire::deserialize_create_canary_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = &input.name;
        if name.is_empty() {
            return rest_json_error(400, "ValidationException", "Name is required");
        }
        let handler = match input.code.handler.as_deref() {
            Some(h) if !h.is_empty() => h.to_string(),
            _ => return rest_json_error(400, "ValidationException", "Code.Handler is required"),
        };
        let artifact_s3_location = &input.artifact_s3_location;
        let execution_role_arn = &input.execution_role_arn;
        let schedule_expression = &input.schedule.expression;
        let runtime_version = &input.runtime_version;
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:synthetics:{region}:{account_id}:canary:{name}");
        let now = Utc::now();
        let canary = Canary {
            name: name.clone(),
            id,
            arn,
            artifact_s3_location: artifact_s3_location.clone(),
            runtime_version: runtime_version.clone(),
            handler,
            schedule_expression: schedule_expression.clone(),
            schedule_duration_in_seconds: input.schedule.duration_in_seconds,
            success_retention_period_in_days: input.success_retention_period_in_days.unwrap_or(31),
            failure_retention_period_in_days: input.failure_retention_period_in_days.unwrap_or(31),
            status: CanaryStatus {
                state: "CREATING".to_string(),
                state_reason: None,
                state_reason_code: None,
            },
            created_at: now,
            last_modified: now,
            execution_role_arn: execution_role_arn.clone(),
            s3_encryption_mode: None,
            tags: input.tags.clone().unwrap_or_default(),
        };
        let mut guard = state.write().await;
        if guard.canaries.contains_key(name) {
            return synthetics_error_response(&SyntheticsError::CanaryAlreadyExists {
                name: name.clone(),
                account_id: account_id.to_string(),
                region: region.to_string(),
            });
        }
        guard.canaries.insert(name.clone(), canary);
        let wire_canary = canary_to_wire(guard.canaries.get(name).unwrap()).await;
        wire::serialize_create_canary_response(&wire::CreateCanaryResponse {
            canary: Some(wire_canary),
        })
    }

    async fn handle_get_canary(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_canary_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_canary(&input.name) {
            Ok(canary) => wire::serialize_get_canary_response(&wire::GetCanaryResponse {
                canary: Some(canary_to_wire(canary).await),
            }),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_delete_canary(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_canary_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_canary(&input.name) {
            Ok(()) => wire::serialize_delete_canary_response(&wire::DeleteCanaryResponse {}),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_describe_canaries(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_canaries_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let canaries = state.describe_canaries();
        let mut entries = Vec::new();
        for c in canaries.iter() {
            entries.push(canary_to_wire(c).await);
        }
        wire::serialize_describe_canaries_response(&wire::DescribeCanariesResponse {
            canaries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse { tags: Some(tags) },
            ),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
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
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        raw_query: &str,
    ) -> MockResponse {
        // Run wire deserializer to get resource_arn from labels.
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The AWS SDK sends tag keys as repeated query params (?tagKeys=k1&tagKeys=k2),
        // but parse_query_string keeps only the last value, so parse them from the raw
        // query string here. Fall back to whatever the wire deserializer parsed (which
        // assumes a comma-separated single param) if no repeated params were found.
        let mut tag_keys: Vec<String> = raw_query
            .split('&')
            .filter_map(|kv| {
                let (key, val) = kv.split_once('=')?;
                if key == "tagKeys" {
                    Some(percent_decode(val))
                } else {
                    None
                }
            })
            .collect();
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys;
        }
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_update_canary(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_canary_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let runtime_version = input.runtime_version.as_deref();
        let execution_role_arn = input.execution_role_arn.as_deref();
        let schedule_expression = input.schedule.as_ref().and_then(|s| {
            if s.expression.is_empty() {
                None
            } else {
                Some(s.expression.as_str())
            }
        });
        let schedule_duration = input.schedule.as_ref().and_then(|s| s.duration_in_seconds);
        let success_retention = input.success_retention_period_in_days;
        let failure_retention = input.failure_retention_period_in_days;
        let handler = input.code.as_ref().and_then(|c| c.handler.as_deref());
        let artifact_s3_location = input.artifact_s3_location.as_deref();
        let mut state = state.write().await;
        match state.update_canary(
            &input.name,
            runtime_version,
            execution_role_arn,
            schedule_expression,
            schedule_duration,
            success_retention,
            failure_retention,
            handler,
            artifact_s3_location,
        ) {
            Ok(()) => wire::serialize_update_canary_response(&wire::UpdateCanaryResponse {}),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_start_canary(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_canary_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.start_canary(&input.name) {
            Ok(()) => wire::serialize_start_canary_response(&wire::StartCanaryResponse {}),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_stop_canary(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_canary_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.stop_canary(&input.name) {
            Ok(()) => wire::serialize_stop_canary_response(&wire::StopCanaryResponse {}),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_start_canary_dry_run(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_canary_dry_run_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        if !state.canaries.contains_key(&input.name) {
            return synthetics_error_response(&SyntheticsError::CanaryNotFound(input.name));
        }
        let dry_run_id = uuid::Uuid::new_v4().to_string();
        wire::serialize_start_canary_dry_run_response(&wire::StartCanaryDryRunResponse {
            dry_run_config: Some(wire::DryRunConfigOutput {
                dry_run_id: Some(dry_run_id),
                last_dry_run_execution_status: Some("RUNNING".to_string()),
            }),
        })
    }

    async fn handle_get_canary_runs(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_canary_runs_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_canary_runs(&input.name) {
            Ok(runs) => {
                let wire_runs: Vec<wire::CanaryRun> = runs
                    .iter()
                    .map(|r| wire::CanaryRun {
                        id: Some(r.id.clone()),
                        name: Some(r.name.clone()),
                        artifact_s3_location: Some(r.artifact_s3_location.clone()),
                        status: Some(wire::CanaryRunStatus {
                            state: Some(r.status.state.clone()),
                            state_reason: r.status.state_reason.clone(),
                            state_reason_code: r.status.state_reason_code.clone(),
                            test_result: r.status.test_result.clone(),
                        }),
                        timeline: Some(wire::CanaryRunTimeline {
                            started: Some(r.started_at.timestamp() as f64),
                            completed: r.completed_at.map(|t| t.timestamp() as f64),
                            metric_timestamp_for_run_and_retries: None,
                        }),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_canary_runs_response(&wire::GetCanaryRunsResponse {
                    canary_runs: Some(wire_runs),
                    ..Default::default()
                })
            }
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_describe_canaries_last_run(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_canaries_last_run_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let entries: Vec<wire::CanaryLastRun> = state
            .canaries
            .keys()
            .map(|name| wire::CanaryLastRun {
                canary_name: Some(name.clone()),
                last_run: None,
            })
            .collect();
        wire::serialize_describe_canaries_last_run_response(
            &wire::DescribeCanariesLastRunResponse {
                canaries_last_run: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_describe_runtime_versions(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_runtime_versions_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let versions = vec![
            wire::RuntimeVersion {
                version_name: Some("syn-nodejs-puppeteer-7.0".to_string()),
                description: Some("Node.js Puppeteer 7.0".to_string()),
                ..Default::default()
            },
            wire::RuntimeVersion {
                version_name: Some("syn-nodejs-puppeteer-6.2".to_string()),
                description: Some("Node.js Puppeteer 6.2".to_string()),
                ..Default::default()
            },
            wire::RuntimeVersion {
                version_name: Some("syn-python-selenium-4.0".to_string()),
                description: Some("Python Selenium 4.0".to_string()),
                ..Default::default()
            },
        ];
        wire::serialize_describe_runtime_versions_response(&wire::DescribeRuntimeVersionsResponse {
            runtime_versions: Some(versions),
            ..Default::default()
        })
    }

    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Name is required");
        }
        let tags = input.tags;
        let mut state = state.write().await;
        match state.create_group(&input.name, tags, account_id, region) {
            Ok(group) => wire::serialize_create_group_response(&wire::CreateGroupResponse {
                group: Some(group_to_wire(group)),
            }),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_get_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_group(&input.group_identifier) {
            Ok(group) => wire::serialize_get_group_response(&wire::GetGroupResponse {
                group: Some(group_to_wire(group)),
            }),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_group(&input.group_identifier) {
            Ok(()) => wire::serialize_delete_group_response(&wire::DeleteGroupResponse {}),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_list_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_groups_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let groups: Vec<wire::GroupSummary> = state
            .list_groups()
            .iter()
            .map(|g| wire::GroupSummary {
                name: Some(g.name.clone()),
                id: Some(g.id.clone()),
                arn: Some(g.arn.clone()),
            })
            .collect();
        wire::serialize_list_groups_response(&wire::ListGroupsResponse {
            groups: Some(groups),
            ..Default::default()
        })
    }

    async fn handle_associate_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "ResourceArn is required");
        }
        let mut state = state.write().await;
        match state.associate_resource(&input.group_identifier, &input.resource_arn) {
            Ok(()) => {
                wire::serialize_associate_resource_response(&wire::AssociateResourceResponse {})
            }
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_disassociate_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "ResourceArn is required");
        }
        let mut state = state.write().await;
        match state.disassociate_resource(&input.group_identifier, &input.resource_arn) {
            Ok(()) => wire::serialize_disassociate_resource_response(
                &wire::DisassociateResourceResponse {},
            ),
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_list_group_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_group_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_group_resources(&input.group_identifier) {
            Ok(resources) => {
                wire::serialize_list_group_resources_response(&wire::ListGroupResourcesResponse {
                    resources: Some(resources),
                    ..Default::default()
                })
            }
            Err(e) => synthetics_error_response(&e),
        }
    }

    async fn handle_list_associated_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<SyntheticsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_associated_groups_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let groups: Vec<wire::GroupSummary> = state
            .list_associated_groups(&input.resource_arn)
            .iter()
            .map(|g| wire::GroupSummary {
                name: Some(g.name.clone()),
                id: Some(g.id.clone()),
                arn: Some(g.arn.clone()),
            })
            .collect();
        wire::serialize_list_associated_groups_response(&wire::ListAssociatedGroupsResponse {
            groups: Some(groups),
            ..Default::default()
        })
    }
}

fn extract_path(uri: &str) -> String {
    // Delegate to the shared core helper, which correctly strips the scheme
    // and host (including custom-endpoint hostnames like `127.0.0.1:PORT`)
    // before returning the path. The previous implementation only matched on
    // `amazonaws.com` and returned the entire URI for non-AWS endpoints,
    // causing dispatch to fail with 404 against the in-process mock server.
    winterbaume_core::extract_path(uri)
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
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

fn synthetics_error_response(err: &SyntheticsError) -> MockResponse {
    let (status, error_type) = match err {
        SyntheticsError::FieldRequired(_) => (400, "ValidationException"),
        SyntheticsError::CanaryAlreadyExists { .. } => (409, "ConflictException"),
        SyntheticsError::GroupAlreadyExists(_) => (409, "ConflictException"),
        SyntheticsError::CanaryNotFound(_) => (404, "ResourceNotFoundException"),
        SyntheticsError::GroupNotFound(_) => (404, "ResourceNotFoundException"),
        SyntheticsError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
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
