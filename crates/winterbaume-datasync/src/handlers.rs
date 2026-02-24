use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{DataSyncError, DataSyncState};
use crate::views::DataSyncStateView;
use crate::wire;

/// Create an awsJson1.1 error response with `__type` and `message` fields.
fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "message": message}).to_string();
    MockResponse::json(status, body)
}

pub struct DataSyncService {
    pub(crate) state: Arc<BackendState<DataSyncState>>,
    pub(crate) notifier: StateChangeNotifier<DataSyncStateView>,
}

impl DataSyncService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DataSyncService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DataSyncService {
    fn service_name(&self) -> &str {
        "datasync"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://(.*\.)?(datasync)\..*\.amazonaws\.com",
            r"https?://datasync\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DataSyncService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

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

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateTask" => {
                self.handle_create_task(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeTask" => self.handle_describe_task(&state, body_bytes).await,
            "DeleteTask" => self.handle_delete_task(&state, body_bytes).await,
            "ListTasks" => self.handle_list_tasks(&state).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "CancelTaskExecution" => self.handle_cancel_task_execution(&state, body_bytes).await,
            "CreateAgent" => json_error_response(
                501,
                "NotImplementedError",
                "CreateAgent is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationAzureBlob" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationAzureBlob is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationEfs" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationEfs is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationFsxLustre" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationFsxLustre is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationFsxOntap" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationFsxOntap is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationFsxOpenZfs" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationFsxOpenZfs is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationFsxWindows" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationFsxWindows is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationHdfs" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationHdfs is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationNfs" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationNfs is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationObjectStorage" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationObjectStorage is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationS3" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationS3 is not yet implemented in winterbaume-datasync",
            ),
            "CreateLocationSmb" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLocationSmb is not yet implemented in winterbaume-datasync",
            ),
            "DeleteAgent" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteAgent is not yet implemented in winterbaume-datasync",
            ),
            "DeleteLocation" => self.handle_delete_location(&state, body_bytes).await,
            "DescribeAgent" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAgent is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationAzureBlob" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationAzureBlob is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationEfs" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationEfs is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationFsxLustre" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationFsxLustre is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationFsxOntap" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationFsxOntap is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationFsxOpenZfs" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationFsxOpenZfs is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationFsxWindows" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationFsxWindows is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationHdfs" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationHdfs is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationNfs" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationNfs is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationObjectStorage" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationObjectStorage is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationS3" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationS3 is not yet implemented in winterbaume-datasync",
            ),
            "DescribeLocationSmb" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLocationSmb is not yet implemented in winterbaume-datasync",
            ),
            "DescribeTaskExecution" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeTaskExecution is not yet implemented in winterbaume-datasync",
            ),
            "ListAgents" => json_error_response(
                501,
                "NotImplementedError",
                "ListAgents is not yet implemented in winterbaume-datasync",
            ),
            "ListLocations" => json_error_response(
                501,
                "NotImplementedError",
                "ListLocations is not yet implemented in winterbaume-datasync",
            ),
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-datasync",
            ),
            "ListTaskExecutions" => json_error_response(
                501,
                "NotImplementedError",
                "ListTaskExecutions is not yet implemented in winterbaume-datasync",
            ),
            "StartTaskExecution" => {
                self.handle_start_task_execution(&state, body_bytes, &region, account_id)
                    .await
            }
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-datasync",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-datasync",
            ),
            "UpdateAgent" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateAgent is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationAzureBlob" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationAzureBlob is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationEfs" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationEfs is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationFsxLustre" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationFsxLustre is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationFsxOntap" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationFsxOntap is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationFsxOpenZfs" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationFsxOpenZfs is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationFsxWindows" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationFsxWindows is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationHdfs" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationHdfs is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationNfs" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationNfs is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationObjectStorage" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationObjectStorage is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationS3" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationS3 is not yet implemented in winterbaume-datasync",
            ),
            "UpdateLocationSmb" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLocationSmb is not yet implemented in winterbaume-datasync",
            ),
            "UpdateTask" => self.handle_update_task(&state, body_bytes).await,
            "UpdateTaskExecution" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateTaskExecution is not yet implemented in winterbaume-datasync",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.source_location_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "SourceLocationArn is required",
            );
        }
        if input.destination_location_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "DestinationLocationArn is required",
            );
        }
        let source_location_arn = input.source_location_arn.as_str();
        let destination_location_arn = input.destination_location_arn.as_str();
        let cloud_watch_log_group_arn = input.cloud_watch_log_group_arn.as_deref();
        let name = input.name.as_deref();

        let mut state = state.write().await;
        match state.create_task(
            source_location_arn,
            destination_location_arn,
            cloud_watch_log_group_arn,
            name,
            region,
            account_id,
        ) {
            Ok(task) => {
                let resp = wire::CreateTaskResponse {
                    task_arn: Some(task.task_arn.clone()),
                };
                wire::serialize_create_task_response(&resp)
            }
            Err(e) => datasync_error_response(&e),
        }
    }

    async fn handle_describe_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "TaskArn is required");
        }
        let task_arn = input.task_arn.as_str();

        let state = state.read().await;
        match state.describe_task(task_arn) {
            Ok(task) => {
                let resp = wire::DescribeTaskResponse {
                    task_arn: Some(task.task_arn.clone()),
                    status: Some(task.status.clone()),
                    source_location_arn: Some(task.source_location_arn.clone()),
                    destination_location_arn: Some(task.destination_location_arn.clone()),
                    creation_time: Some(task.creation_time.timestamp() as f64),
                    name: task.name.clone(),
                    cloud_watch_log_group_arn: task.cloud_watch_log_group_arn.clone(),
                    ..Default::default()
                };
                wire::serialize_describe_task_response(&resp)
            }
            Err(e) => datasync_error_response(&e),
        }
    }

    async fn handle_delete_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "TaskArn is required");
        }
        let task_arn = input.task_arn.as_str();

        let mut state = state.write().await;
        match state.delete_task(task_arn) {
            Ok(()) => wire::serialize_delete_task_response(&wire::DeleteTaskResponse {}),
            Err(e) => datasync_error_response(&e),
        }
    }

    async fn handle_list_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let tasks = state.list_tasks();
        let entries: Vec<wire::TaskListEntry> = tasks
            .iter()
            .map(|task| wire::TaskListEntry {
                task_arn: Some(task.task_arn.clone()),
                status: Some(task.status.clone()),
                name: task.name.clone(),
                ..Default::default()
            })
            .collect();

        let resp = wire::ListTasksResponse {
            tasks: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_tasks_response(&resp)
    }
    async fn handle_cancel_task_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_task_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_execution_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "TaskExecutionArn is required",
            );
        }
        let task_execution_arn = input.task_execution_arn.as_str();
        let mut state = state.write().await;
        match state.cancel_task_execution(task_execution_arn) {
            Ok(()) => wire::serialize_cancel_task_execution_response(
                &wire::CancelTaskExecutionResponse {},
            ),
            Err(e) => datasync_error_response(&e),
        }
    }

    async fn handle_delete_location(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_location_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.location_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "LocationArn is required");
        }
        let location_arn = input.location_arn.as_str();
        let mut state = state.write().await;
        match state.delete_location(location_arn) {
            Ok(()) => wire::serialize_delete_location_response(&wire::DeleteLocationResponse {}),
            Err(e) => datasync_error_response(&e),
        }
    }

    async fn handle_start_task_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_task_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "TaskArn is required");
        }
        let task_arn = input.task_arn.as_str();
        let mut state = state.write().await;
        match state.start_task_execution(task_arn, region, account_id) {
            Ok(exec) => {
                let resp = wire::StartTaskExecutionResponse {
                    task_execution_arn: Some(exec.task_execution_arn.clone()),
                };
                wire::serialize_start_task_execution_response(&resp)
            }
            Err(e) => datasync_error_response(&e),
        }
    }

    async fn handle_update_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DataSyncState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "TaskArn is required");
        }
        let task_arn = input.task_arn.as_str();
        let name = input.name.as_deref();
        let mut state = state.write().await;
        match state.update_task(task_arn, name) {
            Ok(()) => wire::serialize_update_task_response(&wire::UpdateTaskResponse {}),
            Err(e) => datasync_error_response(&e),
        }
    }
}

fn datasync_error_response(err: &DataSyncError) -> MockResponse {
    let (status, error_type) = match err {
        DataSyncError::TaskNotFound { .. } => (400, "InvalidRequestException"),
        DataSyncError::LocationNotFound { .. } => (400, "InvalidRequestException"),
        DataSyncError::TaskExecutionNotFound { .. } => (400, "InvalidRequestException"),
    };
    let body = json!({"__type": error_type, "message": err.to_string()});
    MockResponse::json(status, body.to_string())
}
