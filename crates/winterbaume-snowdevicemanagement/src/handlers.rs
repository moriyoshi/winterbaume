use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{
    DeviceRecord, ExecutionRecord, SnowDeviceManagementError, SnowDeviceManagementState, TaskRecord,
};
use crate::views::SnowDeviceManagementStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SnowDeviceManagementService {
    pub(crate) state: Arc<BackendState<SnowDeviceManagementState>>,
    pub(crate) notifier: StateChangeNotifier<SnowDeviceManagementStateView>,
}

impl SnowDeviceManagementService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SnowDeviceManagementService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SnowDeviceManagementService {
    fn service_name(&self) -> &str {
        "snow-device-management"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://snow-device-management\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<SnowDeviceManagementState>>;

impl SnowDeviceManagementService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["task"]) => {
                self.handle_create_task(&state, account_id, &region, &body)
                    .await
            }
            ("POST", ["task", id, "cancel"]) => self.handle_cancel_task(&state, id).await,
            ("POST", ["task", id]) => self.handle_describe_task(&state, id).await,
            ("GET", ["tasks"]) => self.handle_list_tasks(&state, &request.uri).await,
            ("POST", ["managed-device", id, "describe"]) => {
                self.handle_describe_device(&state, account_id, &region, id)
                    .await
            }
            ("POST", ["managed-device", id, "resources", "ec2", "describe"]) => {
                self.handle_describe_device_ec2(&state, account_id, &region, id, &body)
                    .await
            }
            ("POST", ["task", task_id, "execution", device_id]) => {
                self.handle_describe_execution(&state, task_id, device_id)
                    .await
            }
            ("GET", ["managed-device", id, "resources"]) => {
                self.handle_list_device_resources(&state, account_id, &region, id)
                    .await
            }
            ("GET", ["managed-devices"]) => {
                self.handle_list_devices(&state, account_id, &region, &request.uri)
                    .await
            }
            ("GET", ["executions"]) => self.handle_list_executions(&state, &request.uri).await,
            ("GET", ["tags", arn]) => self.handle_list_tags(&state, arn).await,
            ("POST", ["tags", arn]) => self.handle_tag_resource(&state, arn, &body).await,
            ("DELETE", ["tags", arn]) => {
                self.handle_untag_resource(&state, arn, &request.uri).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        let mutating = matches!(
            (method.as_str(), segs.as_slice()),
            ("POST", ["task"])
                | ("POST", ["task", _, "cancel"])
                | ("POST", ["tags", _])
                | ("DELETE", ["tags", _])
        );
        if response.status / 100 == 2 && mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_task(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let command = body.get("command").cloned().unwrap_or(json!({}));
        if command.as_object().map(|o| o.is_empty()).unwrap_or(true) {
            return rest_json_error(400, "ValidationException", "command is required");
        }
        let targets: Vec<String> = body
            .get("targets")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        if targets.is_empty() {
            return rest_json_error(400, "ValidationException", "targets is required");
        }
        let id = format!("task-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:snow-device-management:{region}:{account_id}:task/{id}");
        let now = chrono::Utc::now().timestamp() as f64;
        let tags = parse_object_string_map(body.get("tags"));
        let task = TaskRecord {
            task_id: id.clone(),
            task_arn: arn.clone(),
            description: body
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from),
            state: "QUEUED".to_string(),
            created_at: now,
            last_updated_at: now,
            completed_at: None,
            command,
            targets,
            tags,
        };
        let mut state = state.write().await;
        state.create_task(task);
        wire::serialize_create_task_response(&wire::CreateTaskOutput {
            task_arn: Some(arn),
            task_id: Some(id),
        })
    }

    async fn handle_cancel_task(&self, state: &SharedState, id: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.cancel_task(id) {
            Ok(()) => wire::serialize_cancel_task_response(&wire::CancelTaskOutput {
                task_id: Some(id.to_string()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_describe_task(&self, state: &SharedState, id: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_task(id) {
            Ok(t) => {
                let tags_map = if t.tags.is_empty() {
                    None
                } else {
                    Some(t.tags.clone())
                };
                wire::serialize_describe_task_response(&wire::DescribeTaskOutput {
                    completed_at: t.completed_at,
                    created_at: Some(t.created_at),
                    description: t.description.clone(),
                    last_updated_at: Some(t.last_updated_at),
                    state: Some(t.state.clone()),
                    tags: tags_map,
                    targets: Some(t.targets.clone()),
                    task_arn: Some(t.task_arn.clone()),
                    task_id: Some(t.task_id.clone()),
                })
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_tasks(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let state_filter = qs
            .iter()
            .find(|(k, _)| k.as_str() == "state")
            .map(|(_, v)| v.clone());
        let state = state.read().await;
        let tasks: Vec<wire::TaskSummary> = state
            .list_tasks(state_filter.as_deref())
            .into_iter()
            .map(|t| wire::TaskSummary {
                state: Some(t.state.clone()),
                tags: if t.tags.is_empty() {
                    None
                } else {
                    Some(t.tags.clone())
                },
                task_arn: Some(t.task_arn.clone()),
                task_id: Some(t.task_id.clone()),
            })
            .collect();
        wire::serialize_list_tasks_response(&wire::ListTasksOutput {
            next_token: None,
            tasks: Some(tasks),
        })
    }

    async fn handle_describe_device(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_seeded(account_id, region);
        match state.get_device(id) {
            Ok(d) => wire::serialize_describe_device_response(&wire::DescribeDeviceOutput {
                associated_with_job: d.job_id.clone(),
                device_capacities: Some(vec![]),
                device_state: Some(d.state.clone()),
                device_type: Some(d.device_type.clone()),
                last_reached_out_at: d.last_reached_out_at,
                last_updated_at: d.last_updated_at,
                managed_device_arn: Some(d.managed_device_arn.clone()),
                managed_device_id: Some(d.managed_device_id.clone()),
                physical_network_interfaces: Some(vec![]),
                software: None,
                tags: None,
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_describe_device_ec2(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        id: &str,
        _body: &Value,
    ) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_seeded(account_id, region);
        match state.get_device(id) {
            Ok(_) => wire::serialize_describe_device_ec2_instances_response(
                &wire::DescribeDeviceEc2Output {
                    instances: Some(vec![]),
                },
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_describe_execution(
        &self,
        state: &SharedState,
        task_id: &str,
        device_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_execution(task_id, device_id) {
            Ok(e) => wire::serialize_describe_execution_response(&wire::DescribeExecutionOutput {
                execution_id: Some(e.execution_id.clone()),
                last_updated_at: Some(e.last_updated_at),
                managed_device_id: Some(e.managed_device_id.clone()),
                started_at: None,
                state: Some(e.state.clone()),
                task_id: Some(e.task_id.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_device_resources(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_seeded(account_id, region);
        if state.get_device(id).is_err() {
            return err_response(&SnowDeviceManagementError::DeviceNotFound { id: id.to_string() });
        }
        wire::serialize_list_device_resources_response(&wire::ListDeviceResourcesOutput {
            next_token: None,
            resources: Some(vec![]),
        })
    }

    async fn handle_list_devices(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let job_id = qs
            .iter()
            .find(|(k, _)| k.as_str() == "jobId")
            .map(|(_, v)| v.clone());
        let mut state = state.write().await;
        state.ensure_seeded(account_id, region);
        let devices: Vec<wire::DeviceSummary> = state
            .list_devices(job_id.as_deref())
            .into_iter()
            .map(|d: &DeviceRecord| wire::DeviceSummary {
                associated_with_job: d.job_id.clone(),
                managed_device_arn: Some(d.managed_device_arn.clone()),
                managed_device_id: Some(d.managed_device_id.clone()),
                tags: None,
            })
            .collect();
        wire::serialize_list_devices_response(&wire::ListDevicesOutput {
            devices: Some(devices),
            next_token: None,
        })
    }

    async fn handle_list_executions(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let task_id = qs
            .iter()
            .find(|(k, _)| k.as_str() == "taskId")
            .map(|(_, v)| v.clone());
        let state_filter = qs
            .iter()
            .find(|(k, _)| k.as_str() == "state")
            .map(|(_, v)| v.clone());
        let state = state.read().await;
        let executions: Vec<wire::ExecutionSummary> = state
            .list_executions_by_task(task_id.as_deref(), state_filter.as_deref())
            .into_iter()
            .map(|e: &ExecutionRecord| wire::ExecutionSummary {
                execution_id: Some(e.execution_id.clone()),
                managed_device_id: Some(e.managed_device_id.clone()),
                state: Some(e.state.clone()),
                task_id: Some(e.task_id.clone()),
            })
            .collect();
        wire::serialize_list_executions_response(&wire::ListExecutionsOutput {
            executions: Some(executions),
            next_token: None,
        })
    }

    async fn handle_list_tags(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags_map = state.list_tags(arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
            tags: if tags_map.is_empty() {
                None
            } else {
                Some(tags_map)
            },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags = parse_object_string_map(body.get("tags"));
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(arn, tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let keys: Vec<String> = qs
            .iter()
            .filter(|(k, _)| k.as_str() == "tagKeys")
            .map(|(_, v)| v.clone())
            .collect();
        let mut state = state.write().await;
        state.untag_resource(arn, &keys);
        wire::serialize_untag_resource_response()
    }
}

fn parse_object_string_map(v: Option<&Value>) -> HashMap<String, String> {
    v.and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn err_response(err: &SnowDeviceManagementError) -> MockResponse {
    let (status, error_type) = match err {
        SnowDeviceManagementError::TaskNotFound { .. }
        | SnowDeviceManagementError::DeviceNotFound { .. }
        | SnowDeviceManagementError::ExecutionNotFound { .. } => (404, "ResourceNotFoundException"),
        SnowDeviceManagementError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
