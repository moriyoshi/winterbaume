use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, json_error_response,
};

use crate::state::SwfState;
use crate::types::{Domain, RegistrationStatus};
use crate::views::SwfStateView;
use crate::wire;

/// SWF service handler that processes awsJson1.0 protocol requests.
pub struct SwfService {
    pub(crate) state: Arc<BackendState<SwfState>>,
    pub(crate) notifier: StateChangeNotifier<SwfStateView>,
}

impl SwfService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SwfService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SwfService {
    fn service_name(&self) -> &str {
        "swf"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://swf\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SwfService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "SimpleWorkflowService.RegisterDomain"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("SimpleWorkflowService."))
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(
                    400,
                    "com.amazonaws.swf#MissingAction",
                    "Missing or invalid X-Amz-Target header",
                );
            }
        };

        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "RegisterDomain" => {
                self.handle_register_domain(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeDomain" => self.handle_describe_domain(&state, body_bytes).await,
            "DeprecateDomain" => self.handle_deprecate_domain(&state, body_bytes).await,
            "ListDomains" => self.handle_list_domains(&state, body_bytes).await,
            "DeprecateActivityType" => {
                self.handle_deprecate_activity_type(&state, body_bytes)
                    .await
            }
            "DescribeActivityType" => self.handle_describe_activity_type(&state, body_bytes).await,
            "DeprecateWorkflowType" => {
                self.handle_deprecate_workflow_type(&state, body_bytes)
                    .await
            }
            "DescribeWorkflowType" => self.handle_describe_workflow_type(&state, body_bytes).await,
            "DescribeWorkflowExecution" => {
                self.handle_describe_workflow_execution(&state, body_bytes)
                    .await
            }
            "TerminateWorkflowExecution" => {
                self.handle_terminate_workflow_execution(&state, body_bytes)
                    .await
            }
            "GetWorkflowExecutionHistory" => {
                self.handle_get_workflow_execution_history(&state, body_bytes)
                    .await
            }
            "CountClosedWorkflowExecutions" => {
                self.handle_count_closed_workflow_executions(&state, body_bytes)
                    .await
            }
            "CountOpenWorkflowExecutions" => {
                self.handle_count_open_workflow_executions(&state, body_bytes)
                    .await
            }
            "CountPendingActivityTasks" => {
                self.handle_count_pending_activity_tasks(&state, body_bytes)
                    .await
            }
            "CountPendingDecisionTasks" => {
                self.handle_count_pending_decision_tasks(&state, body_bytes)
                    .await
            }
            "ListClosedWorkflowExecutions" => {
                self.handle_list_closed_workflow_executions(&state, body_bytes)
                    .await
            }
            "ListOpenWorkflowExecutions" => {
                self.handle_list_open_workflow_executions(&state, body_bytes)
                    .await
            }
            "PollForDecisionTask" => self.handle_poll_for_decision_task(&state, body_bytes).await,
            "PollForActivityTask" => self.handle_poll_for_activity_task(&state, body_bytes).await,
            "RecordActivityTaskHeartbeat" => {
                self.handle_record_activity_task_heartbeat(&state, body_bytes)
                    .await
            }
            "RespondActivityTaskCompleted" => {
                self.handle_respond_activity_task_completed(&state, body_bytes)
                    .await
            }
            "RespondActivityTaskFailed" => {
                self.handle_respond_activity_task_failed(&state, body_bytes)
                    .await
            }
            "RespondDecisionTaskCompleted" => {
                self.handle_respond_decision_task_completed(&state, body_bytes)
                    .await
            }
            "SignalWorkflowExecution" => {
                self.handle_signal_workflow_execution(&state, body_bytes)
                    .await
            }
            "UndeprecateDomain" => self.handle_undeprecate_domain(&state, body_bytes).await,
            "ListActivityTypes" => self.handle_list_activity_types(&state, body_bytes).await,
            "ListWorkflowTypes" => self.handle_list_workflow_types(&state, body_bytes).await,
            "RegisterActivityType" => self.handle_register_activity_type(&state, body_bytes).await,
            "RegisterWorkflowType" => self.handle_register_workflow_type(&state, body_bytes).await,
            "StartWorkflowExecution" => {
                self.handle_start_workflow_execution(&state, body_bytes)
                    .await
            }
            _ => json_error_response(
                400,
                "com.amazonaws.swf#UnknownOperationException",
                &format!("Unknown operation: {action}"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_register_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: name",
            );
        }
        if input.workflow_execution_retention_period_in_days.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowExecutionRetentionPeriodInDays",
            );
        }
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.register_domain(
            &input.name,
            description,
            &input.workflow_execution_retention_period_in_days,
            account_id,
            region,
        ) {
            Ok(()) => wire::serialize_register_domain_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_describe_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: name",
            );
        }

        let state = state.read().await;
        match state.describe_domain(&input.name) {
            Ok(domain) => {
                let detail = domain_to_detail(domain);
                wire::serialize_describe_domain_response(&detail)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_deprecate_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deprecate_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: name",
            );
        }

        let mut state = state.write().await;
        match state.deprecate_domain(&input.name) {
            Ok(()) => wire::serialize_deprecate_domain_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_list_domains(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_domains_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        let registration_status = match RegistrationStatus::from_str(&input.registration_status) {
            Some(s) => s,
            None => {
                return json_error_response(
                    400,
                    "com.amazonaws.swf#ValidationException",
                    "Missing or invalid required field: registrationStatus",
                );
            }
        };

        let state = state.read().await;
        let domains = state.list_domains(registration_status);

        let result = wire::DomainInfos {
            domain_infos: Some(domains.iter().map(|d| domain_to_info(d)).collect()),
            next_page_token: None,
        };
        wire::serialize_list_domains_response(&result)
    }

    async fn handle_register_activity_type(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_activity_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: name",
            );
        }
        if input.version.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: version",
            );
        }
        let description = input.description.as_deref();
        let default_task_list = input
            .default_task_list
            .as_ref()
            .map(|t| t.name.as_str())
            .filter(|s| !s.is_empty());
        let default_task_heartbeat_timeout = input.default_task_heartbeat_timeout.as_deref();
        let default_task_schedule_to_start_timeout =
            input.default_task_schedule_to_start_timeout.as_deref();
        let default_task_schedule_to_close_timeout =
            input.default_task_schedule_to_close_timeout.as_deref();
        let default_task_start_to_close_timeout =
            input.default_task_start_to_close_timeout.as_deref();
        let default_task_priority = input.default_task_priority.as_deref();

        let mut state = state.write().await;
        match state.register_activity_type(
            &input.domain,
            &input.name,
            &input.version,
            description,
            default_task_list,
            default_task_heartbeat_timeout,
            default_task_schedule_to_start_timeout,
            default_task_schedule_to_close_timeout,
            default_task_start_to_close_timeout,
            default_task_priority,
        ) {
            Ok(()) => wire::serialize_register_activity_type_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_deprecate_activity_type(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deprecate_activity_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.activity_type.name.is_empty() || input.activity_type.version.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: activityType",
            );
        }

        let mut state = state.write().await;
        match state.deprecate_activity_type(
            &input.domain,
            &input.activity_type.name,
            &input.activity_type.version,
        ) {
            Ok(()) => wire::serialize_deprecate_activity_type_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_describe_activity_type(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_activity_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.activity_type.name.is_empty() || input.activity_type.version.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: activityType",
            );
        }

        let state = state.read().await;
        match state.describe_activity_type(
            &input.domain,
            &input.activity_type.name,
            &input.activity_type.version,
        ) {
            Ok(at) => {
                let detail = wire::ActivityTypeDetail {
                    configuration: Some(wire::ActivityTypeConfiguration {
                        default_task_heartbeat_timeout: at.default_task_heartbeat_timeout.clone(),
                        default_task_list: at
                            .default_task_list
                            .as_ref()
                            .map(|n| wire::TaskList { name: n.clone() }),
                        default_task_priority: at.default_task_priority.clone(),
                        default_task_schedule_to_close_timeout: at
                            .default_task_schedule_to_close_timeout
                            .clone(),
                        default_task_schedule_to_start_timeout: at
                            .default_task_schedule_to_start_timeout
                            .clone(),
                        default_task_start_to_close_timeout: at
                            .default_task_start_to_close_timeout
                            .clone(),
                    }),
                    type_info: Some(wire::ActivityTypeInfo {
                        activity_type: Some(wire::ActivityType {
                            name: at.name.clone(),
                            version: at.version.clone(),
                        }),
                        creation_date: Some(at.creation_date),
                        deprecation_date: at.deprecation_date,
                        description: at.description.clone(),
                        status: Some(at.status.as_str().to_string()),
                    }),
                };
                wire::serialize_describe_activity_type_response(&detail)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_register_workflow_type(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_workflow_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: name",
            );
        }
        if input.version.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: version",
            );
        }
        let description = input.description.as_deref();
        let default_task_list = input
            .default_task_list
            .as_ref()
            .map(|t| t.name.as_str())
            .filter(|s| !s.is_empty());
        let default_execution_start_to_close_timeout =
            input.default_execution_start_to_close_timeout.as_deref();
        let default_task_start_to_close_timeout =
            input.default_task_start_to_close_timeout.as_deref();
        let default_child_policy = input.default_child_policy.as_deref();
        let default_lambda_role = input.default_lambda_role.as_deref();
        let default_task_priority = input.default_task_priority.as_deref();

        let mut state = state.write().await;
        match state.register_workflow_type(
            &input.domain,
            &input.name,
            &input.version,
            description,
            default_task_list,
            default_execution_start_to_close_timeout,
            default_task_start_to_close_timeout,
            default_child_policy,
            default_lambda_role,
            default_task_priority,
        ) {
            Ok(()) => wire::serialize_register_workflow_type_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_deprecate_workflow_type(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deprecate_workflow_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.workflow_type.name.is_empty() || input.workflow_type.version.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowType",
            );
        }

        let mut state = state.write().await;
        match state.deprecate_workflow_type(
            &input.domain,
            &input.workflow_type.name,
            &input.workflow_type.version,
        ) {
            Ok(()) => wire::serialize_deprecate_workflow_type_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_describe_workflow_type(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workflow_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.workflow_type.name.is_empty() || input.workflow_type.version.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowType",
            );
        }

        let state = state.read().await;
        match state.describe_workflow_type(
            &input.domain,
            &input.workflow_type.name,
            &input.workflow_type.version,
        ) {
            Ok(wt) => {
                let detail = wire::WorkflowTypeDetail {
                    configuration: Some(wire::WorkflowTypeConfiguration {
                        default_child_policy: wt.default_child_policy.clone(),
                        default_execution_start_to_close_timeout: wt
                            .default_execution_start_to_close_timeout
                            .clone(),
                        default_lambda_role: wt.default_lambda_role.clone(),
                        default_task_list: wt
                            .default_task_list
                            .as_ref()
                            .map(|n| wire::TaskList { name: n.clone() }),
                        default_task_priority: wt.default_task_priority.clone(),
                        default_task_start_to_close_timeout: wt
                            .default_task_start_to_close_timeout
                            .clone(),
                    }),
                    type_info: Some(wire::WorkflowTypeInfo {
                        creation_date: Some(wt.creation_date),
                        deprecation_date: wt.deprecation_date,
                        description: wt.description.clone(),
                        status: Some(wt.status.as_str().to_string()),
                        workflow_type: Some(wire::WorkflowType {
                            name: wt.name.clone(),
                            version: wt.version.clone(),
                        }),
                    }),
                };
                wire::serialize_describe_workflow_type_response(&detail)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_start_workflow_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_workflow_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.workflow_id.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowId",
            );
        }
        if input.workflow_type.name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowType.name",
            );
        }
        if input.workflow_type.version.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowType.version",
            );
        }
        let task_list = input
            .task_list
            .as_ref()
            .map(|t| t.name.as_str())
            .filter(|s| !s.is_empty());
        let execution_start_to_close_timeout = input.execution_start_to_close_timeout.as_deref();
        let task_start_to_close_timeout = input.task_start_to_close_timeout.as_deref();
        let child_policy = input.child_policy.as_deref();
        let tag_list = input.tag_list.clone();
        let lambda_role = input.lambda_role.as_deref();
        let task_priority = input.task_priority.as_deref();

        let mut state = state.write().await;
        match state.start_workflow_execution(
            &input.domain,
            &input.workflow_id,
            &input.workflow_type.name,
            &input.workflow_type.version,
            task_list,
            execution_start_to_close_timeout,
            task_start_to_close_timeout,
            child_policy,
            tag_list,
            lambda_role,
            task_priority,
        ) {
            Ok(run_id) => {
                let result = wire::Run {
                    run_id: Some(run_id),
                };
                wire::serialize_start_workflow_execution_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_describe_workflow_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workflow_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.execution.workflow_id.is_empty() || input.execution.run_id.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: execution",
            );
        }

        let state = state.read().await;
        match state.describe_workflow_execution(
            &input.domain,
            &input.execution.workflow_id,
            &input.execution.run_id,
        ) {
            Ok(exec) => {
                let detail = execution_to_detail(exec);
                wire::serialize_describe_workflow_execution_response(&detail)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_terminate_workflow_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_terminate_workflow_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.workflow_id.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowId",
            );
        }
        let run_id = input.run_id.as_deref();
        let reason = input.reason.as_deref();
        let details = input.details.as_deref();
        let child_policy = input.child_policy.as_deref();

        let mut state = state.write().await;
        match state.terminate_workflow_execution(
            &input.domain,
            &input.workflow_id,
            run_id,
            reason,
            details,
            child_policy,
        ) {
            Ok(()) => wire::serialize_terminate_workflow_execution_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_get_workflow_execution_history(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_workflow_execution_history_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.execution.workflow_id.is_empty() || input.execution.run_id.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: execution",
            );
        }

        let state = state.read().await;
        match state.get_workflow_execution_history(
            &input.domain,
            &input.execution.workflow_id,
            &input.execution.run_id,
        ) {
            Ok(exec) => {
                let events: Vec<wire::HistoryEvent> = exec
                    .history_events
                    .iter()
                    .map(|e| wire::HistoryEvent {
                        event_id: Some(e.event_id),
                        event_timestamp: Some(e.event_timestamp),
                        event_type: Some(e.event_type.clone()),
                        ..Default::default()
                    })
                    .collect();
                let result = wire::History {
                    events: Some(events),
                    next_page_token: None,
                };
                wire::serialize_get_workflow_execution_history_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_count_closed_workflow_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_count_closed_workflow_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }

        let state = state.read().await;
        match state.count_closed_workflow_executions(&input.domain) {
            Ok(count) => {
                let result = wire::WorkflowExecutionCount {
                    count: Some(count),
                    ..Default::default()
                };
                wire::serialize_count_closed_workflow_executions_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_count_open_workflow_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_count_open_workflow_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }

        let state = state.read().await;
        match state.count_open_workflow_executions(&input.domain) {
            Ok(count) => {
                let result = wire::WorkflowExecutionCount {
                    count: Some(count),
                    ..Default::default()
                };
                wire::serialize_count_open_workflow_executions_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_count_pending_activity_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_count_pending_activity_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        let task_list = if input.task_list.name.is_empty() {
            "default"
        } else {
            input.task_list.name.as_str()
        };

        let state = state.read().await;
        match state.count_pending_activity_tasks(&input.domain, task_list) {
            Ok(count) => {
                let result = wire::PendingTaskCount {
                    count: Some(count),
                    ..Default::default()
                };
                wire::serialize_count_pending_activity_tasks_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_count_pending_decision_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_count_pending_decision_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        let task_list = if input.task_list.name.is_empty() {
            "default"
        } else {
            input.task_list.name.as_str()
        };

        let state = state.read().await;
        match state.count_pending_decision_tasks(&input.domain, task_list) {
            Ok(count) => {
                let result = wire::PendingTaskCount {
                    count: Some(count),
                    ..Default::default()
                };
                wire::serialize_count_pending_decision_tasks_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_list_open_workflow_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_open_workflow_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }

        let state = state.read().await;
        match state.list_open_workflow_executions(&input.domain) {
            Ok(execs) => {
                let infos: Vec<wire::WorkflowExecutionInfo> =
                    execs.iter().map(|e| execution_to_info(e)).collect();
                let result = wire::WorkflowExecutionInfos {
                    execution_infos: Some(infos),
                    next_page_token: None,
                };
                wire::serialize_list_open_workflow_executions_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_list_closed_workflow_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_closed_workflow_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }

        let state = state.read().await;
        match state.list_closed_workflow_executions(&input.domain) {
            Ok(execs) => {
                let infos: Vec<wire::WorkflowExecutionInfo> =
                    execs.iter().map(|e| execution_to_info(e)).collect();
                let result = wire::WorkflowExecutionInfos {
                    execution_infos: Some(infos),
                    next_page_token: None,
                };
                wire::serialize_list_closed_workflow_executions_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_poll_for_decision_task(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_poll_for_decision_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        let task_list = if input.task_list.name.is_empty() {
            "default"
        } else {
            input.task_list.name.as_str()
        };

        let state = state.read().await;
        match state.poll_for_decision_task(&input.domain, task_list) {
            Ok(Some(exec)) => {
                let events: Vec<wire::HistoryEvent> = exec
                    .history_events
                    .iter()
                    .map(|e| wire::HistoryEvent {
                        event_id: Some(e.event_id),
                        event_timestamp: Some(e.event_timestamp),
                        event_type: Some(e.event_type.clone()),
                        ..Default::default()
                    })
                    .collect();
                let result = wire::DecisionTask {
                    events: Some(events),
                    started_event_id: Some(0),
                    task_token: Some(uuid::Uuid::new_v4().to_string()),
                    workflow_execution: Some(wire::WorkflowExecution {
                        run_id: exec.run_id.clone(),
                        workflow_id: exec.workflow_id.clone(),
                    }),
                    workflow_type: Some(wire::WorkflowType {
                        name: exec.workflow_type_name.clone(),
                        version: exec.workflow_type_version.clone(),
                    }),
                    ..Default::default()
                };
                wire::serialize_poll_for_decision_task_response(&result)
            }
            Ok(None) => {
                // Return empty decision task (no tasks available)
                let result = wire::DecisionTask {
                    ..Default::default()
                };
                wire::serialize_poll_for_decision_task_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_poll_for_activity_task(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_poll_for_activity_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        let task_list = if input.task_list.name.is_empty() {
            "default"
        } else {
            input.task_list.name.as_str()
        };

        let mut state = state.write().await;
        match state.poll_for_activity_task(&input.domain, task_list) {
            Ok(Some(task)) => {
                let result = wire::ActivityTask {
                    activity_id: Some(task.activity_id.clone()),
                    activity_type: Some(wire::ActivityType {
                        name: task.activity_type_name.clone(),
                        version: task.activity_type_version.clone(),
                    }),
                    input: task.input.clone(),
                    started_event_id: Some(1),
                    task_token: Some(task.task_token.clone()),
                    workflow_execution: Some(wire::WorkflowExecution {
                        run_id: task.run_id.clone(),
                        workflow_id: task.workflow_id.clone(),
                    }),
                };
                wire::serialize_poll_for_activity_task_response(&result)
            }
            Ok(None) => {
                // No tasks available - return empty ActivityTask (taskToken empty = no task)
                let result = wire::ActivityTask {
                    ..Default::default()
                };
                wire::serialize_poll_for_activity_task_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_record_activity_task_heartbeat(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_record_activity_task_heartbeat_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.task_token.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: taskToken",
            );
        }
        let details = input.details.as_deref();

        let state = state.read().await;
        match state.record_activity_task_heartbeat(&input.task_token, details) {
            Ok(cancel_requested) => {
                let result = wire::ActivityTaskStatus {
                    cancel_requested: Some(cancel_requested),
                };
                wire::serialize_record_activity_task_heartbeat_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_respond_activity_task_completed(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_respond_activity_task_completed_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.task_token.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: taskToken",
            );
        }
        let result = input.result.as_deref();

        let mut state = state.write().await;
        match state.respond_activity_task_completed(&input.task_token, result) {
            Ok(()) => wire::serialize_respond_activity_task_completed_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_respond_activity_task_failed(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_respond_activity_task_failed_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.task_token.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: taskToken",
            );
        }
        let reason = input.reason.as_deref();
        let details = input.details.as_deref();

        let mut state = state.write().await;
        match state.respond_activity_task_failed(&input.task_token, reason, details) {
            Ok(()) => wire::serialize_respond_activity_task_failed_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_respond_decision_task_completed(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_respond_decision_task_completed_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.task_token.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: taskToken",
            );
        }
        let decisions: Vec<serde_json::Value> = input
            .decisions
            .as_ref()
            .map(|arr| {
                arr.iter()
                    .filter_map(|d| serde_json::to_value(d).ok())
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.respond_decision_task_completed(&input.task_token, decisions) {
            Ok(()) => wire::serialize_respond_decision_task_completed_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_signal_workflow_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_signal_workflow_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        if input.workflow_id.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: workflowId",
            );
        }
        if input.signal_name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: signalName",
            );
        }
        let exec_input = input.input.as_deref();
        let run_id = input.run_id.as_deref();

        let mut state = state.write().await;
        match state.signal_workflow_execution(
            &input.domain,
            &input.workflow_id,
            &input.signal_name,
            exec_input,
            run_id,
        ) {
            Ok(()) => wire::serialize_signal_workflow_execution_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_undeprecate_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_undeprecate_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: name",
            );
        }

        let mut state = state.write().await;
        match state.undeprecate_domain(&input.name) {
            Ok(()) => wire::serialize_undeprecate_domain_response(),
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_list_activity_types(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_activity_types_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        let registration_status =
            match crate::types::RegistrationStatus::from_str(&input.registration_status) {
                Some(s) => s,
                None => {
                    return json_error_response(
                        400,
                        "com.amazonaws.swf#ValidationException",
                        "Missing or invalid required field: registrationStatus",
                    );
                }
            };

        let state = state.read().await;
        match state.list_activity_types(&input.domain, registration_status) {
            Ok(types) => {
                let infos: Vec<wire::ActivityTypeInfo> = types
                    .iter()
                    .map(|at| wire::ActivityTypeInfo {
                        activity_type: Some(wire::ActivityType {
                            name: at.name.clone(),
                            version: at.version.clone(),
                        }),
                        creation_date: Some(at.creation_date),
                        deprecation_date: at.deprecation_date,
                        description: at.description.clone(),
                        status: Some(at.status.as_str().to_string()),
                    })
                    .collect();
                let result = wire::ActivityTypeInfos {
                    type_infos: Some(infos),
                    next_page_token: None,
                };
                wire::serialize_list_activity_types_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }

    async fn handle_list_workflow_types(
        &self,
        state: &Arc<tokio::sync::RwLock<SwfState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_workflow_types_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "com.amazonaws.swf#ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(
                400,
                "com.amazonaws.swf#ValidationException",
                "Missing required field: domain",
            );
        }
        let registration_status =
            match crate::types::RegistrationStatus::from_str(&input.registration_status) {
                Some(s) => s,
                None => {
                    return json_error_response(
                        400,
                        "com.amazonaws.swf#ValidationException",
                        "Missing or invalid required field: registrationStatus",
                    );
                }
            };

        let state = state.read().await;
        match state.list_workflow_types(&input.domain, registration_status) {
            Ok(types) => {
                let infos: Vec<wire::WorkflowTypeInfo> = types
                    .iter()
                    .map(|wt| wire::WorkflowTypeInfo {
                        workflow_type: Some(wire::WorkflowType {
                            name: wt.name.clone(),
                            version: wt.version.clone(),
                        }),
                        creation_date: Some(wt.creation_date),
                        deprecation_date: wt.deprecation_date,
                        description: wt.description.clone(),
                        status: Some(wt.status.as_str().to_string()),
                    })
                    .collect();
                let result = wire::WorkflowTypeInfos {
                    type_infos: Some(infos),
                    next_page_token: None,
                };
                wire::serialize_list_workflow_types_response(&result)
            }
            Err(e) => swf_error_response(&e),
        }
    }
}

/// Convert a state Domain to a wire DomainInfo.
fn domain_to_info(domain: &Domain) -> wire::DomainInfo {
    wire::DomainInfo {
        name: Some(domain.name.clone()),
        status: Some(domain.status.as_str().to_string()),
        arn: Some(domain.arn.clone()),
        description: domain.description.clone(),
    }
}

/// Convert a state Domain to a wire DomainDetail.
fn domain_to_detail(domain: &Domain) -> wire::DomainDetail {
    wire::DomainDetail {
        domain_info: Some(domain_to_info(domain)),
        configuration: Some(wire::DomainConfiguration {
            workflow_execution_retention_period_in_days: Some(
                domain.workflow_execution_retention_period_in_days.clone(),
            ),
        }),
    }
}

/// Convert a workflow execution to wire WorkflowExecutionInfo.
fn execution_to_info(exec: &crate::types::WorkflowExecutionDef) -> wire::WorkflowExecutionInfo {
    wire::WorkflowExecutionInfo {
        cancel_requested: Some(exec.cancel_requested),
        close_status: exec.close_status.clone(),
        close_timestamp: exec.close_timestamp,
        execution: Some(wire::WorkflowExecution {
            run_id: exec.run_id.clone(),
            workflow_id: exec.workflow_id.clone(),
        }),
        execution_status: Some(exec.status.as_str().to_string()),
        parent: None,
        start_timestamp: Some(exec.start_timestamp),
        tag_list: exec.tag_list.clone(),
        workflow_type: Some(wire::WorkflowType {
            name: exec.workflow_type_name.clone(),
            version: exec.workflow_type_version.clone(),
        }),
    }
}

/// Convert a workflow execution to wire WorkflowExecutionDetail.
fn execution_to_detail(exec: &crate::types::WorkflowExecutionDef) -> wire::WorkflowExecutionDetail {
    wire::WorkflowExecutionDetail {
        execution_configuration: Some(wire::WorkflowExecutionConfiguration {
            child_policy: Some(exec.child_policy.clone()),
            execution_start_to_close_timeout: Some(exec.execution_start_to_close_timeout.clone()),
            lambda_role: exec.lambda_role.clone(),
            task_list: Some(wire::TaskList {
                name: exec.task_list.clone(),
            }),
            task_priority: exec.task_priority.clone(),
            task_start_to_close_timeout: Some(exec.task_start_to_close_timeout.clone()),
        }),
        execution_info: Some(execution_to_info(exec)),
        open_counts: Some(wire::WorkflowExecutionOpenCounts {
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn swf_error_response(err: &crate::state::SwfError) -> MockResponse {
    use crate::state::SwfError;
    let (status, error_type) = match err {
        SwfError::DomainAlreadyExists(_) => (400u16, "com.amazonaws.swf#DomainAlreadyExistsFault"),
        SwfError::DomainDeprecated(_) => (400, "com.amazonaws.swf#DomainDeprecatedFault"),
        SwfError::ActivityTypeAlreadyExists(_, _) => {
            (400, "com.amazonaws.swf#TypeAlreadyExistsFault")
        }
        SwfError::WorkflowTypeAlreadyExists(_, _) => {
            (400, "com.amazonaws.swf#TypeAlreadyExistsFault")
        }
        SwfError::ActivityTypeDeprecated(_, _) => (400, "com.amazonaws.swf#TypeDeprecatedFault"),
        SwfError::WorkflowTypeDeprecated(_, _) => (400, "com.amazonaws.swf#TypeDeprecatedFault"),
        SwfError::UnknownDomain(_) => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::UnknownType { .. } => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::UnknownExecution { .. } => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::NoOpenExecution(_) => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::ExecutionAlreadyClosed(_) => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::ExecutionClosed(_) => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::UnknownTaskToken(_) => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::TaskAlreadyCompleted => (400, "com.amazonaws.swf#UnknownResourceFault"),
        SwfError::WorkflowExecutionAlreadyStarted(_) => (
            400,
            "com.amazonaws.swf#WorkflowExecutionAlreadyStartedFault",
        ),
    };
    json_error_response(status, error_type, &err.to_string())
}
