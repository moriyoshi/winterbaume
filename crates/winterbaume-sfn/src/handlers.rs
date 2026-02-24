use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{SfnError, StepFunctionsState};
use crate::types::{
    CloudWatchLogsLogGroup, EncryptionConfiguration, LogDestination, LoggingConfiguration, Tag,
    TracingConfiguration,
};
use crate::views::StepfunctionsStateView;
use crate::wire;

/// Step Functions service handler that processes awsJson1.0 protocol requests.
pub struct SfnService {
    pub(crate) state: Arc<BackendState<StepFunctionsState>>,
    pub(crate) notifier: StateChangeNotifier<StepfunctionsStateView>,
}

impl SfnService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SfnService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SfnService {
    fn service_name(&self) -> &str {
        "states"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://states\..*\.amazonaws\.com",
            r"https?://states\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SfnService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "AWSStepFunctions.CreateStateMachine"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateStateMachine" => {
                self.handle_create_state_machine(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteStateMachine" => self.handle_delete_state_machine(&state, body_bytes).await,
            "DescribeStateMachine" => self.handle_describe_state_machine(&state, body_bytes).await,
            "ListStateMachines" => self.handle_list_state_machines(&state).await,
            "UpdateStateMachine" => self.handle_update_state_machine(&state, body_bytes).await,
            "StartExecution" => {
                self.handle_start_execution(&state, body_bytes, account_id, &region)
                    .await
            }
            "StopExecution" => self.handle_stop_execution(&state, body_bytes).await,
            "DescribeExecution" => self.handle_describe_execution(&state, body_bytes).await,
            "ListExecutions" => self.handle_list_executions(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "CreateActivity" => {
                self.handle_create_activity(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeActivity" => self.handle_describe_activity(&state, body_bytes).await,
            "DeleteActivity" => self.handle_delete_activity(&state, body_bytes).await,
            "ListActivities" => self.handle_list_activities(&state).await,
            "SendTaskSuccess" => self.handle_send_task_success(&state, body_bytes).await,
            "SendTaskFailure" => self.handle_send_task_failure(&state, body_bytes).await,
            "SendTaskHeartbeat" => self.handle_send_task_heartbeat(&state, body_bytes).await,
            "GetExecutionHistory" => self.handle_get_execution_history(&state, body_bytes).await,
            "DescribeMapRun" => self.handle_describe_map_run(&state, body_bytes).await,
            "ListMapRuns" => self.handle_list_map_runs(&state, body_bytes).await,
            "UpdateMapRun" => self.handle_update_map_run(&state, body_bytes).await,
            "DescribeStateMachineForExecution" => {
                self.handle_describe_state_machine_for_execution(&state, body_bytes)
                    .await
            }
            "ValidateStateMachineDefinition" => {
                self.handle_validate_state_machine_definition(body_bytes)
                    .await
            }
            "ListStateMachineVersions" => {
                self.handle_list_state_machine_versions(&state, body_bytes)
                    .await
            }
            "PublishStateMachineVersion" => {
                self.handle_publish_state_machine_version(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteStateMachineVersion" => {
                self.handle_delete_state_machine_version(&state, body_bytes)
                    .await
            }
            "CreateStateMachineAlias" => {
                self.handle_create_state_machine_alias(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeStateMachineAlias" => {
                self.handle_describe_state_machine_alias(&state, body_bytes)
                    .await
            }
            "DeleteStateMachineAlias" => {
                self.handle_delete_state_machine_alias(&state, body_bytes)
                    .await
            }
            "ListStateMachineAliases" => {
                self.handle_list_state_machine_aliases(&state, body_bytes)
                    .await
            }
            "UpdateStateMachineAlias" => {
                self.handle_update_state_machine_alias(&state, body_bytes)
                    .await
            }
            "GetActivityTask" => self.handle_get_activity_task(&state, body_bytes).await,
            "RedriveExecution" => self.handle_redrive_execution(&state, body_bytes).await,
            "StartSyncExecution" => {
                self.handle_start_sync_execution(&state, body_bytes, account_id, &region)
                    .await
            }
            "TestState" => self.handle_test_state(body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for StepFunctions"),
            ),
        };
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_state_machine(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_state_machine_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        if input.definition.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'definition'");
        }
        if input.role_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'roleArn'");
        }
        let sm_type = input.r#type.as_deref();
        // FIX(terraform-e2e): when publish=true, AWS auto-publishes a version and returns
        //   stateMachineVersionArn in the response. Without this, aws_sfn_alias resources
        //   using state_machine_version_arn get an empty string, causing a validation error.
        let publish = input.publish.unwrap_or(false);

        let tags = wire_tags_to_local(input.tags.as_deref());
        let logging_configuration = input
            .logging_configuration
            .as_ref()
            .map(wire_logging_to_local);
        let tracing_configuration = input
            .tracing_configuration
            .as_ref()
            .map(wire_tracing_to_local);
        let encryption_configuration = input
            .encryption_configuration
            .as_ref()
            .map(wire_encryption_to_local);

        let mut state = state.write().await;
        match state.create_state_machine(
            &input.name,
            &input.definition,
            &input.role_arn,
            account_id,
            region,
            sm_type,
            tags,
            logging_configuration,
            tracing_configuration,
            encryption_configuration,
        ) {
            Ok(sm) => {
                let sm_arn = sm.arn.clone();
                let creation_date = sm.creation_date.timestamp() as f64;
                // Auto-publish a version when publish=true
                let version_arn = if publish {
                    state
                        .publish_state_machine_version(&sm_arn, None, account_id, region)
                        .ok()
                        .map(|v| v.version_arn.clone())
                } else {
                    None
                };
                let output = wire::CreateStateMachineOutput {
                    state_machine_arn: Some(sm_arn),
                    creation_date: Some(creation_date),
                    state_machine_version_arn: version_arn,
                };
                wire::serialize_create_state_machine_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_delete_state_machine(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_state_machine_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }

        let mut state = state.write().await;
        match state.delete_state_machine(&input.state_machine_arn) {
            Ok(()) => {
                let output = wire::DeleteStateMachineOutput {};
                wire::serialize_delete_state_machine_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_describe_state_machine(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_state_machine_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }

        let state = state.read().await;
        match state.describe_state_machine(&input.state_machine_arn) {
            Ok(sm) => {
                let output = state_machine_to_describe_output(sm);
                wire::serialize_describe_state_machine_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_list_state_machines(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let machines = state.list_state_machines();
        let entries: Vec<wire::StateMachineListItem> = machines
            .iter()
            .map(|sm| wire::StateMachineListItem {
                state_machine_arn: Some(sm.arn.clone()),
                name: Some(sm.name.clone()),
                r#type: Some(sm.r#type.clone()),
                creation_date: Some(sm.creation_date.timestamp() as f64),
            })
            .collect();

        let output = wire::ListStateMachinesOutput {
            state_machines: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_state_machines_response(&output)
    }

    async fn handle_update_state_machine(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_state_machine_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }
        let definition = input.definition.as_deref();
        let role_arn = input.role_arn.as_deref();
        let logging_configuration = input
            .logging_configuration
            .as_ref()
            .map(wire_logging_to_local);
        let tracing_configuration = input
            .tracing_configuration
            .as_ref()
            .map(wire_tracing_to_local);
        let encryption_configuration = input
            .encryption_configuration
            .as_ref()
            .map(wire_encryption_to_local);

        let mut state = state.write().await;
        match state.update_state_machine(
            &input.state_machine_arn,
            definition,
            role_arn,
            logging_configuration,
            tracing_configuration,
            encryption_configuration,
        ) {
            Ok(sm) => {
                let output = wire::UpdateStateMachineOutput {
                    update_date: Some(sm.creation_date.timestamp() as f64),
                    ..Default::default()
                };
                wire::serialize_update_state_machine_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_start_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }
        let name = input.name.as_deref();
        let exec_input = input.input.as_deref();

        let mut state = state.write().await;
        match state.start_execution(
            &input.state_machine_arn,
            name,
            exec_input,
            account_id,
            region,
        ) {
            Ok(exec) => {
                let output = wire::StartExecutionOutput {
                    execution_arn: Some(exec.execution_arn.clone()),
                    start_date: Some(exec.start_date.timestamp() as f64),
                };
                wire::serialize_start_execution_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_stop_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.execution_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'executionArn'");
        }

        let mut state = state.write().await;
        match state.stop_execution(&input.execution_arn) {
            Ok(exec) => {
                let output = wire::StopExecutionOutput {
                    stop_date: Some(exec.stop_date.unwrap().timestamp() as f64),
                };
                wire::serialize_stop_execution_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_describe_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.execution_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'executionArn'");
        }

        let state = state.read().await;
        match state.describe_execution(&input.execution_arn) {
            Ok(exec) => {
                let output = execution_to_describe_output(exec);
                wire::serialize_describe_execution_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_list_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let state_machine_arn = match input.state_machine_arn.as_deref() {
            Some(a) if !a.is_empty() => a,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'stateMachineArn'",
                );
            }
        };

        let state = state.read().await;
        match state.list_executions(state_machine_arn) {
            Ok(execs) => {
                let entries: Vec<wire::ExecutionListItem> =
                    execs.iter().map(|e| execution_to_list_item(e)).collect();
                let output = wire::ListExecutionsOutput {
                    executions: Some(entries),
                    ..Default::default()
                };
                wire::serialize_list_executions_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }
        let tags = wire_tags_to_local(Some(input.tags.as_slice()));

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => {
                let output = wire::TagResourceOutput {};
                wire::serialize_tag_resource_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => {
                let output = wire::UntagResourceOutput {};
                wire::serialize_untag_resource_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }

        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        let tag_entries: Vec<wire::Tag> = tags
            .iter()
            .map(|t| wire::Tag {
                key: Some(t.key.clone()),
                value: Some(t.value.clone()),
            })
            .collect();

        let output = wire::ListTagsForResourceOutput {
            tags: if tag_entries.is_empty() {
                None
            } else {
                Some(tag_entries)
            },
        };
        wire::serialize_list_tags_for_resource_response(&output)
    }
    // --- Activity handlers ---

    async fn handle_create_activity(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_activity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }

        let tags = wire_tags_to_local(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_activity(&input.name, account_id, region, tags) {
            Ok(activity) => {
                let output = wire::CreateActivityOutput {
                    activity_arn: Some(activity.arn.clone()),
                    creation_date: Some(activity.creation_date.timestamp() as f64),
                };
                wire::serialize_create_activity_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_describe_activity(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_activity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.activity_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'activityArn'");
        }

        let state = state.read().await;
        match state.describe_activity(&input.activity_arn) {
            Ok(activity) => {
                let output = wire::DescribeActivityOutput {
                    activity_arn: Some(activity.arn.clone()),
                    creation_date: Some(activity.creation_date.timestamp() as f64),
                    name: Some(activity.name.clone()),
                    ..Default::default()
                };
                wire::serialize_describe_activity_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_delete_activity(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_activity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.activity_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'activityArn'");
        }

        let mut state = state.write().await;
        match state.delete_activity(&input.activity_arn) {
            Ok(()) => {
                let output = wire::DeleteActivityOutput {};
                wire::serialize_delete_activity_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_list_activities(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let activities = state.list_activities();
        let entries: Vec<wire::ActivityListItem> = activities
            .iter()
            .map(|a| wire::ActivityListItem {
                activity_arn: Some(a.arn.clone()),
                creation_date: Some(a.creation_date.timestamp() as f64),
                name: Some(a.name.clone()),
            })
            .collect();

        let output = wire::ListActivitiesOutput {
            activities: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_activities_response(&output)
    }

    // --- Task callback handlers ---

    async fn handle_send_task_success(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_send_task_success_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_token.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskToken'");
        }
        if input.output.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'output'");
        }

        let mut state = state.write().await;
        match state.send_task_success(&input.task_token, &input.output) {
            Ok(()) => {
                let output = wire::SendTaskSuccessOutput {};
                wire::serialize_send_task_success_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_send_task_failure(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_send_task_failure_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_token.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskToken'");
        }
        let error = input.error.as_deref();
        let cause = input.cause.as_deref();

        let mut state = state.write().await;
        match state.send_task_failure(&input.task_token, error, cause) {
            Ok(()) => {
                let output = wire::SendTaskFailureOutput {};
                wire::serialize_send_task_failure_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_send_task_heartbeat(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_send_task_heartbeat_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_token.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskToken'");
        }

        let mut state = state.write().await;
        match state.send_task_heartbeat(&input.task_token) {
            Ok(()) => {
                let output = wire::SendTaskHeartbeatOutput {};
                wire::serialize_send_task_heartbeat_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    // --- Execution history handler ---

    async fn handle_get_execution_history(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_execution_history_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.execution_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'executionArn'");
        }

        let state = state.read().await;
        match state.get_execution_history(&input.execution_arn) {
            Ok(events) => {
                let entries: Vec<wire::HistoryEvent> = events
                    .iter()
                    .map(|e| wire::HistoryEvent {
                        id: Some(e.id),
                        r#type: Some(e.event_type.clone()),
                        timestamp: Some(e.timestamp.timestamp() as f64),
                        previous_event_id: e.previous_event_id,
                        ..Default::default()
                    })
                    .collect();
                let output = wire::GetExecutionHistoryOutput {
                    events: Some(entries),
                    ..Default::default()
                };
                wire::serialize_get_execution_history_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    // --- Map run handlers ---

    async fn handle_describe_map_run(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_map_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.map_run_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'mapRunArn'");
        }

        let state = state.read().await;
        match state.describe_map_run(&input.map_run_arn) {
            Ok(mr) => {
                let output = wire::DescribeMapRunOutput {
                    map_run_arn: Some(mr.map_run_arn.clone()),
                    execution_arn: Some(mr.execution_arn.clone()),
                    status: Some(mr.status.as_str().to_string()),
                    start_date: Some(mr.start_date.timestamp() as f64),
                    stop_date: mr.stop_date.map(|d| d.timestamp() as f64),
                    max_concurrency: Some(mr.max_concurrency),
                    tolerated_failure_count: Some(mr.tolerated_failure_count),
                    tolerated_failure_percentage: Some(mr.tolerated_failure_percentage),
                    ..Default::default()
                };
                wire::serialize_describe_map_run_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_list_map_runs(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_map_runs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.execution_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'executionArn'");
        }

        let state = state.read().await;
        match state.list_map_runs(&input.execution_arn) {
            Ok(runs) => {
                let entries: Vec<wire::MapRunListItem> = runs
                    .iter()
                    .map(|mr| wire::MapRunListItem {
                        map_run_arn: Some(mr.map_run_arn.clone()),
                        execution_arn: Some(mr.execution_arn.clone()),
                        state_machine_arn: Some(mr.state_machine_arn.clone()),
                        start_date: Some(mr.start_date.timestamp() as f64),
                        stop_date: mr.stop_date.map(|d| d.timestamp() as f64),
                    })
                    .collect();
                let output = wire::ListMapRunsOutput {
                    map_runs: Some(entries),
                    ..Default::default()
                };
                wire::serialize_list_map_runs_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_update_map_run(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_map_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.map_run_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'mapRunArn'");
        }
        let max_concurrency = input.max_concurrency;
        let tolerated_failure_count = input.tolerated_failure_count;
        let tolerated_failure_percentage = input.tolerated_failure_percentage;

        let mut state = state.write().await;
        match state.update_map_run(
            &input.map_run_arn,
            max_concurrency,
            tolerated_failure_count,
            tolerated_failure_percentage,
        ) {
            Ok(()) => {
                let output = wire::UpdateMapRunOutput {};
                wire::serialize_update_map_run_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    // --- Describe state machine for execution handler ---

    async fn handle_describe_state_machine_for_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_state_machine_for_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.execution_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'executionArn'");
        }

        let state = state.read().await;
        match state.describe_state_machine_for_execution(&input.execution_arn) {
            Ok(sm) => {
                let output = wire::DescribeStateMachineForExecutionOutput {
                    state_machine_arn: Some(sm.arn.clone()),
                    name: Some(sm.name.clone()),
                    definition: Some(sm.definition.clone()),
                    role_arn: Some(sm.role_arn.clone()),
                    update_date: Some(sm.creation_date.timestamp() as f64),
                    ..Default::default()
                };
                wire::serialize_describe_state_machine_for_execution_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    // --- Validate / Version handlers ---

    async fn handle_validate_state_machine_definition(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_validate_state_machine_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let result = winterbaume_sfn_asl_eval::validate(
            &input.definition,
            &winterbaume_sfn_asl_eval::ValidateOptions::default(),
        );

        let diagnostics: Vec<wire::ValidateStateMachineDefinitionDiagnostic> = result
            .diagnostics
            .iter()
            .map(|d| wire::ValidateStateMachineDefinitionDiagnostic {
                code: Some(d.code.to_string()),
                message: Some(d.message.clone()),
                location: d.location.clone(),
                severity: Some(if result.valid { "WARNING" } else { "ERROR" }.to_string()),
            })
            .collect();

        let output = wire::ValidateStateMachineDefinitionOutput {
            result: Some(if result.valid { "OK" } else { "FAIL" }.to_string()),
            diagnostics: Some(diagnostics),
            truncated: Some(result.truncated),
        };
        wire::serialize_validate_state_machine_definition_response(&output)
    }

    async fn handle_list_state_machine_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_state_machine_versions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }

        let state = state.read().await;
        match state.list_state_machine_versions(&input.state_machine_arn) {
            Ok(versions) => {
                let entries: Vec<wire::StateMachineVersionListItem> = versions
                    .iter()
                    .map(|v| wire::StateMachineVersionListItem {
                        state_machine_version_arn: Some(v.version_arn.clone()),
                        creation_date: Some(v.creation_date.timestamp() as f64),
                    })
                    .collect();
                let output = wire::ListStateMachineVersionsOutput {
                    state_machine_versions: Some(entries),
                    ..Default::default()
                };
                wire::serialize_list_state_machine_versions_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_publish_state_machine_version(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_state_machine_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.publish_state_machine_version(
            &input.state_machine_arn,
            description,
            account_id,
            region,
        ) {
            Ok(version) => {
                let output = wire::PublishStateMachineVersionOutput {
                    state_machine_version_arn: Some(version.version_arn.clone()),
                    creation_date: Some(version.creation_date.timestamp() as f64),
                };
                wire::serialize_publish_state_machine_version_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_delete_state_machine_version(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_state_machine_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_version_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'stateMachineVersionArn'",
            );
        }

        let mut state = state.write().await;
        match state.delete_state_machine_version(&input.state_machine_version_arn) {
            Ok(()) => {
                let output = wire::DeleteStateMachineVersionOutput {};
                wire::serialize_delete_state_machine_version_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_create_state_machine_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_state_machine_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        let description = input.description.as_deref();

        let routing_configuration: Vec<(String, i32)> = input
            .routing_configuration
            .iter()
            .map(|r| {
                let weight = if r.weight == 0 { 100 } else { r.weight };
                (r.state_machine_version_arn.clone(), weight)
            })
            .collect();

        let mut state = state.write().await;
        match state.create_state_machine_alias(
            &input.name,
            description,
            routing_configuration,
            account_id,
            region,
        ) {
            Ok(alias) => {
                let output = wire::CreateStateMachineAliasOutput {
                    state_machine_alias_arn: Some(alias.alias_arn.clone()),
                    creation_date: Some(alias.creation_date.timestamp() as f64),
                };
                wire::serialize_create_state_machine_alias_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_describe_state_machine_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_state_machine_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_alias_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'stateMachineAliasArn'",
            );
        }

        let state = state.read().await;
        match state.describe_state_machine_alias(&input.state_machine_alias_arn) {
            Ok(alias) => {
                let routing: Vec<wire::RoutingConfigurationListItem> = alias
                    .routing_configuration
                    .iter()
                    .map(|(version_arn, weight)| wire::RoutingConfigurationListItem {
                        state_machine_version_arn: version_arn.clone(),
                        weight: *weight,
                    })
                    .collect();
                let output = wire::DescribeStateMachineAliasOutput {
                    state_machine_alias_arn: Some(alias.alias_arn.clone()),
                    name: Some(alias.name.clone()),
                    description: alias.description.clone(),
                    routing_configuration: if routing.is_empty() {
                        None
                    } else {
                        Some(routing)
                    },
                    creation_date: Some(alias.creation_date.timestamp() as f64),
                    update_date: Some(alias.update_date.timestamp() as f64),
                };
                wire::serialize_describe_state_machine_alias_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_delete_state_machine_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_state_machine_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_alias_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'stateMachineAliasArn'",
            );
        }

        let mut state = state.write().await;
        match state.delete_state_machine_alias(&input.state_machine_alias_arn) {
            Ok(()) => {
                let output = wire::DeleteStateMachineAliasOutput {};
                wire::serialize_delete_state_machine_alias_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_list_state_machine_aliases(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_state_machine_aliases_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }

        let state = state.read().await;
        match state.list_state_machine_aliases(&input.state_machine_arn) {
            Ok(aliases) => {
                let entries: Vec<wire::StateMachineAliasListItem> = aliases
                    .iter()
                    .map(|a| wire::StateMachineAliasListItem {
                        state_machine_alias_arn: Some(a.alias_arn.clone()),
                        creation_date: Some(a.creation_date.timestamp() as f64),
                    })
                    .collect();
                let output = wire::ListStateMachineAliasesOutput {
                    state_machine_aliases: Some(entries),
                    ..Default::default()
                };
                wire::serialize_list_state_machine_aliases_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_update_state_machine_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_state_machine_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_alias_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'stateMachineAliasArn'",
            );
        }
        let description = input.description.as_deref();
        let routing_configuration: Option<Vec<(String, i32)>> =
            input.routing_configuration.as_ref().map(|rc| {
                rc.iter()
                    .map(|r| {
                        let weight = if r.weight == 0 { 100 } else { r.weight };
                        (r.state_machine_version_arn.clone(), weight)
                    })
                    .collect()
            });

        let mut state = state.write().await;
        match state.update_state_machine_alias(
            &input.state_machine_alias_arn,
            description,
            routing_configuration,
        ) {
            Ok(alias) => {
                let output = wire::UpdateStateMachineAliasOutput {
                    update_date: Some(alias.update_date.timestamp() as f64),
                };
                wire::serialize_update_state_machine_alias_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_get_activity_task(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_activity_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.activity_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'activityArn'");
        }

        let state = state.read().await;
        match state.describe_activity(&input.activity_arn) {
            Ok(_) => {
                // In this mock, no tasks are queued — return empty result (null taskToken)
                let output = wire::GetActivityTaskOutput {
                    task_token: None,
                    input: None,
                };
                wire::serialize_get_activity_task_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    async fn handle_redrive_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_redrive_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.execution_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'executionArn'");
        }

        let mut state = state.write().await;
        match state.redrive_execution(&input.execution_arn) {
            Ok(()) => {
                let output = wire::RedriveExecutionOutput {
                    redrive_date: Some(Utc::now().timestamp() as f64),
                };
                wire::serialize_redrive_execution_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    // STUB[no-engine]: StartSyncExecution requires actually running the state machine definition
    //   through an ASL interpreter. The mock records the execution in state and immediately
    //   returns SUCCEEDED with empty output without executing any states.
    async fn handle_start_sync_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<StepFunctionsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_sync_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.state_machine_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'stateMachineArn'");
        }
        let name = input.name.as_deref();
        let exec_input = input.input.as_deref();

        let mut state = state.write().await;
        match state.start_execution(
            &input.state_machine_arn,
            name,
            exec_input,
            account_id,
            region,
        ) {
            Ok(exec) => {
                let now = Utc::now().timestamp() as f64;
                let output = wire::StartSyncExecutionOutput {
                    execution_arn: Some(exec.execution_arn.clone()),
                    name: Some(exec.name.clone()),
                    state_machine_arn: Some(exec.state_machine_arn.clone()),
                    status: Some("SUCCEEDED".to_string()),
                    start_date: Some(exec.start_date.timestamp() as f64),
                    stop_date: Some(now),
                    input: exec.input.clone(),
                    output: Some("{}".to_string()),
                    ..Default::default()
                };
                wire::serialize_start_sync_execution_response(&output)
            }
            Err(e) => sfn_error_response(&e),
        }
    }

    // STUB[no-engine]: TestState executes a single ASL state against sample input. This
    //   requires a full ASL interpreter; the mock always returns SUCCEEDED with empty output.
    async fn handle_test_state(&self, _body: &[u8]) -> MockResponse {
        // TestState validates/runs a single state — return a minimal OK response.
        let output = wire::TestStateOutput {
            status: Some("SUCCEEDED".to_string()),
            output: Some("{}".to_string()),
            ..Default::default()
        };
        wire::serialize_test_state_response(&output)
    }
}

// --- State-to-wire conversion helpers ---

fn state_machine_to_describe_output(
    sm: &crate::types::StateMachine,
) -> wire::DescribeStateMachineOutput {
    wire::DescribeStateMachineOutput {
        state_machine_arn: Some(sm.arn.clone()),
        name: Some(sm.name.clone()),
        definition: Some(sm.definition.clone()),
        role_arn: Some(sm.role_arn.clone()),
        status: Some(sm.status.as_str().to_string()),
        r#type: Some(sm.r#type.clone()),
        creation_date: Some(sm.creation_date.timestamp() as f64),
        ..Default::default()
    }
}

fn execution_to_describe_output(exec: &crate::types::Execution) -> wire::DescribeExecutionOutput {
    wire::DescribeExecutionOutput {
        execution_arn: Some(exec.execution_arn.clone()),
        name: Some(exec.name.clone()),
        state_machine_arn: Some(exec.state_machine_arn.clone()),
        status: Some(exec.status.as_str().to_string()),
        start_date: Some(exec.start_date.timestamp() as f64),
        stop_date: exec.stop_date.map(|d| d.timestamp() as f64),
        input: exec.input.clone(),
        output: exec.output.clone(),
        ..Default::default()
    }
}

fn execution_to_list_item(exec: &crate::types::Execution) -> wire::ExecutionListItem {
    wire::ExecutionListItem {
        execution_arn: Some(exec.execution_arn.clone()),
        name: Some(exec.name.clone()),
        state_machine_arn: Some(exec.state_machine_arn.clone()),
        status: Some(exec.status.as_str().to_string()),
        start_date: Some(exec.start_date.timestamp() as f64),
        stop_date: exec.stop_date.map(|d| d.timestamp() as f64),
        ..Default::default()
    }
}

// --- wire-to-local conversion helpers ---

fn wire_tags_to_local(tags: Option<&[wire::Tag]>) -> Vec<Tag> {
    tags.map(|arr| {
        arr.iter()
            .filter_map(|t| {
                let key = t.key.as_deref()?;
                let value = t.value.as_deref()?;
                Some(Tag {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            })
            .collect()
    })
    .unwrap_or_default()
}

fn wire_logging_to_local(lc: &wire::LoggingConfiguration) -> LoggingConfiguration {
    let level = lc.level.clone();
    let include_execution_data = lc.include_execution_data;
    let destinations = lc
        .destinations
        .as_ref()
        .map(|arr| {
            arr.iter()
                .map(|d| LogDestination {
                    cloud_watch_logs_log_group: d.cloud_watch_logs_log_group.as_ref().map(|cw| {
                        CloudWatchLogsLogGroup {
                            log_group_arn: cw.log_group_arn.clone(),
                        }
                    }),
                })
                .collect()
        })
        .unwrap_or_default();
    LoggingConfiguration {
        level,
        include_execution_data,
        destinations,
    }
}

fn wire_tracing_to_local(tc: &wire::TracingConfiguration) -> TracingConfiguration {
    TracingConfiguration {
        enabled: tc.enabled.unwrap_or(false),
    }
}

fn wire_encryption_to_local(ec: &wire::EncryptionConfiguration) -> EncryptionConfiguration {
    let r#type = if ec.r#type.is_empty() {
        "AWS_OWNED_KEY".to_string()
    } else {
        ec.r#type.clone()
    };
    EncryptionConfiguration {
        kms_key_id: ec.kms_key_id.clone(),
        kms_data_key_reuse_period_seconds: ec.kms_data_key_reuse_period_seconds.map(|v| v as i64),
        r#type,
    }
}

// --- Error helper ---

fn sfn_error_response(err: &SfnError) -> MockResponse {
    let (status, error_type) = match err {
        SfnError::StateMachineAlreadyExists(_) => (400, "StateMachineAlreadyExists"),
        SfnError::StateMachineDoesNotExist(_) => (400, "StateMachineDoesNotExist"),
        SfnError::ExecutionAlreadyExists(_) => (400, "ExecutionAlreadyExists"),
        SfnError::ExecutionDoesNotExist(_) => (400, "ExecutionDoesNotExist"),
        SfnError::ExecutionNotAbortedOrFailed(_) => (400, "ExecutionNotAbortedOrFailed"),
        SfnError::ActivityAlreadyExists(_) => (400, "ActivityAlreadyExists"),
        SfnError::ActivityDoesNotExist(_) => (400, "ActivityDoesNotExist"),
        SfnError::TaskDoesNotExist(_) => (400, "TaskDoesNotExist"),
        SfnError::ResourceNotFound(_) => (400, "ResourceNotFound"),
        SfnError::MapRunDoesNotExist(_) => (400, "ResourceNotFound"),
        SfnError::StateMachineAliasMustBeUnique(_) => (400, "StateMachineAliasMustBeUnique"),
        SfnError::StateMachineAliasDoesNotExist(_) => (400, "ResourceNotFound"),
    };
    json_error_response(status, error_type, &err.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
