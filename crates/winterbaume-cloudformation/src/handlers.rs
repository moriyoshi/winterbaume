use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{CloudFormationError, CloudFormationState};
use crate::types::{StackParameter, StackTag};
use crate::views::CloudFormationStateView;
use crate::wire;

/// CloudFormation service handler (awsQuery protocol).
pub struct CloudFormationService {
    pub(crate) state: Arc<BackendState<CloudFormationState>>,
    pub(crate) notifier: StateChangeNotifier<CloudFormationStateView>,
}

impl CloudFormationService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudFormationService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudFormationService {
    fn service_name(&self) -> &str {
        "cloudformation"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://cloudformation\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

const MUTATING_ACTIONS: &[&str] = &[
    "CreateStack",
    "UpdateStack",
    "DeleteStack",
    "CreateChangeSet",
    "DeleteChangeSet",
    "ExecuteChangeSet",
    "CreateStackSet",
    "UpdateStackSet",
    "DeleteStackSet",
    "CreateStackInstances",
    "UpdateStackInstances",
    "DeleteStackInstances",
    "StopStackSetOperation",
    "SetStackPolicy",
    "UpdateTerminationProtection",
    "CancelUpdateStack",
    "RollbackStack",
];

impl CloudFormationService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.clone(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateStack" => {
                self.handle_create_stack(&state, &params, account_id, &region)
                    .await
            }
            "UpdateStack" => self.handle_update_stack(&state, &params).await,
            "DeleteStack" => self.handle_delete_stack(&state, &params).await,
            "DescribeStacks" => self.handle_describe_stacks(&state, &params).await,
            "ListStacks" => self.handle_list_stacks(&state, &params).await,
            "GetTemplate" => self.handle_get_template(&state, &params).await,
            "GetStackPolicy" => self.handle_get_stack_policy(&state, &params).await,
            "SetStackPolicy" => self.handle_set_stack_policy(&state, &params).await,
            "DescribeStackEvents" => self.handle_describe_stack_events(&state, &params).await,
            "DescribeStackResources" => self.handle_describe_stack_resources(&state, &params).await,
            "DescribeStackResource" => self.handle_describe_stack_resource(&state, &params).await,
            "ListStackResources" => self.handle_list_stack_resources(&state, &params).await,
            "ListExports" => self.handle_list_exports(&state).await,
            "CreateChangeSet" => {
                self.handle_create_change_set(&state, &params, account_id, &region)
                    .await
            }
            "DeleteChangeSet" => self.handle_delete_change_set(&state, &params).await,
            "DescribeChangeSet" => self.handle_describe_change_set(&state, &params).await,
            "ExecuteChangeSet" => self.handle_execute_change_set(&state, &params).await,
            "ListChangeSets" => self.handle_list_change_sets(&state, &params).await,
            "ValidateTemplate" => self.handle_validate_template(&params).await,
            "CreateStackSet" => {
                self.handle_create_stack_set(&state, &params, account_id, &region)
                    .await
            }
            "DescribeStackSet" => self.handle_describe_stack_set(&state, &params).await,
            "UpdateStackSet" => {
                self.handle_update_stack_set(&state, &params, account_id, &region)
                    .await
            }
            "DeleteStackSet" => self.handle_delete_stack_set(&state, &params).await,
            "ListStackSets" => self.handle_list_stack_sets(&state).await,
            "CreateStackInstances" => {
                self.handle_create_stack_instances(&state, &params, account_id)
                    .await
            }
            "DeleteStackInstances" => self.handle_delete_stack_instances(&state, &params).await,
            "DescribeStackInstance" => self.handle_describe_stack_instance(&state, &params).await,
            "ListStackInstances" => self.handle_list_stack_instances(&state, &params).await,
            "DescribeStackSetOperation" => {
                self.handle_describe_stack_set_operation(&state, &params)
                    .await
            }
            "ListStackSetOperations" => {
                self.handle_list_stack_set_operations(&state, &params).await
            }
            "ListStackSetOperationResults" => {
                self.handle_list_stack_set_operation_results(&state, &params)
                    .await
            }
            "StopStackSetOperation" => self.handle_stop_stack_set_operation(&state, &params).await,
            "UpdateTerminationProtection" => {
                self.handle_update_termination_protection(&state, &params)
                    .await
            }
            "DescribeAccountLimits" => self.handle_describe_account_limits().await,
            "ListImports" => self.handle_list_imports(&state, &params).await,
            "GetTemplateSummary" => self.handle_get_template_summary(&state, &params).await,
            "CancelUpdateStack" => self.handle_cancel_update_stack(&state, &params).await,
            "RollbackStack" => self.handle_rollback_stack(&state, &params).await,
            "SignalResource" => self.handle_signal_resource(&state, &params).await,
            "EstimateTemplateCost" => self.handle_estimate_template_cost().await,
            "ContinueUpdateRollback" => self.handle_continue_update_rollback(&state, &params).await,
            "ListTypes" => self.handle_list_types(&state).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "ActivateOrganizationsAccess" => MockResponse::error(
                501,
                "NotImplementedError",
                "ActivateOrganizationsAccess is not yet implemented in winterbaume-cloudformation",
            ),
            "ActivateType" => MockResponse::error(
                501,
                "NotImplementedError",
                "ActivateType is not yet implemented in winterbaume-cloudformation",
            ),
            "BatchDescribeTypeConfigurations" => MockResponse::error(
                501,
                "NotImplementedError",
                "BatchDescribeTypeConfigurations is not yet implemented in winterbaume-cloudformation",
            ),
            "CreateGeneratedTemplate" => MockResponse::error(
                501,
                "NotImplementedError",
                "CreateGeneratedTemplate is not yet implemented in winterbaume-cloudformation",
            ),
            "CreateStackRefactor" => MockResponse::error(
                501,
                "NotImplementedError",
                "CreateStackRefactor is not yet implemented in winterbaume-cloudformation",
            ),
            "DeactivateOrganizationsAccess" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeactivateOrganizationsAccess is not yet implemented in winterbaume-cloudformation",
            ),
            "DeactivateType" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeactivateType is not yet implemented in winterbaume-cloudformation",
            ),
            "DeleteGeneratedTemplate" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeleteGeneratedTemplate is not yet implemented in winterbaume-cloudformation",
            ),
            "DeregisterType" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeregisterType is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeChangeSetHooks" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeChangeSetHooks is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeEvents" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeEvents is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeGeneratedTemplate" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeGeneratedTemplate is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeOrganizationsAccess" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeOrganizationsAccess is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribePublisher" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribePublisher is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeResourceScan" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeResourceScan is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeStackDriftDetectionStatus" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeStackDriftDetectionStatus is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeStackRefactor" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeStackRefactor is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeStackResourceDrifts" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeStackResourceDrifts is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeType" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeType is not yet implemented in winterbaume-cloudformation",
            ),
            "DescribeTypeRegistration" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeTypeRegistration is not yet implemented in winterbaume-cloudformation",
            ),
            "DetectStackDrift" => MockResponse::error(
                501,
                "NotImplementedError",
                "DetectStackDrift is not yet implemented in winterbaume-cloudformation",
            ),
            "DetectStackResourceDrift" => MockResponse::error(
                501,
                "NotImplementedError",
                "DetectStackResourceDrift is not yet implemented in winterbaume-cloudformation",
            ),
            "DetectStackSetDrift" => MockResponse::error(
                501,
                "NotImplementedError",
                "DetectStackSetDrift is not yet implemented in winterbaume-cloudformation",
            ),
            "ExecuteStackRefactor" => MockResponse::error(
                501,
                "NotImplementedError",
                "ExecuteStackRefactor is not yet implemented in winterbaume-cloudformation",
            ),
            "GetGeneratedTemplate" => MockResponse::error(
                501,
                "NotImplementedError",
                "GetGeneratedTemplate is not yet implemented in winterbaume-cloudformation",
            ),
            "GetHookResult" => MockResponse::error(
                501,
                "NotImplementedError",
                "GetHookResult is not yet implemented in winterbaume-cloudformation",
            ),
            "ImportStacksToStackSet" => MockResponse::error(
                501,
                "NotImplementedError",
                "ImportStacksToStackSet is not yet implemented in winterbaume-cloudformation",
            ),
            "ListGeneratedTemplates" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListGeneratedTemplates is not yet implemented in winterbaume-cloudformation",
            ),
            "ListHookResults" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListHookResults is not yet implemented in winterbaume-cloudformation",
            ),
            "ListResourceScanRelatedResources" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListResourceScanRelatedResources is not yet implemented in winterbaume-cloudformation",
            ),
            "ListResourceScanResources" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListResourceScanResources is not yet implemented in winterbaume-cloudformation",
            ),
            "ListResourceScans" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListResourceScans is not yet implemented in winterbaume-cloudformation",
            ),
            "ListStackInstanceResourceDrifts" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListStackInstanceResourceDrifts is not yet implemented in winterbaume-cloudformation",
            ),
            "ListStackRefactorActions" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListStackRefactorActions is not yet implemented in winterbaume-cloudformation",
            ),
            "ListStackRefactors" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListStackRefactors is not yet implemented in winterbaume-cloudformation",
            ),
            "ListStackSetAutoDeploymentTargets" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListStackSetAutoDeploymentTargets is not yet implemented in winterbaume-cloudformation",
            ),
            "ListTypeRegistrations" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListTypeRegistrations is not yet implemented in winterbaume-cloudformation",
            ),
            "ListTypeVersions" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListTypeVersions is not yet implemented in winterbaume-cloudformation",
            ),
            "PublishType" => MockResponse::error(
                501,
                "NotImplementedError",
                "PublishType is not yet implemented in winterbaume-cloudformation",
            ),
            "RecordHandlerProgress" => MockResponse::error(
                501,
                "NotImplementedError",
                "RecordHandlerProgress is not yet implemented in winterbaume-cloudformation",
            ),
            "RegisterPublisher" => MockResponse::error(
                501,
                "NotImplementedError",
                "RegisterPublisher is not yet implemented in winterbaume-cloudformation",
            ),
            "RegisterType" => MockResponse::error(
                501,
                "NotImplementedError",
                "RegisterType is not yet implemented in winterbaume-cloudformation",
            ),
            "SetTypeConfiguration" => MockResponse::error(
                501,
                "NotImplementedError",
                "SetTypeConfiguration is not yet implemented in winterbaume-cloudformation",
            ),
            "SetTypeDefaultVersion" => MockResponse::error(
                501,
                "NotImplementedError",
                "SetTypeDefaultVersion is not yet implemented in winterbaume-cloudformation",
            ),
            "StartResourceScan" => MockResponse::error(
                501,
                "NotImplementedError",
                "StartResourceScan is not yet implemented in winterbaume-cloudformation",
            ),
            "TestType" => MockResponse::error(
                501,
                "NotImplementedError",
                "TestType is not yet implemented in winterbaume-cloudformation",
            ),
            "UpdateGeneratedTemplate" => MockResponse::error(
                501,
                "NotImplementedError",
                "UpdateGeneratedTemplate is not yet implemented in winterbaume-cloudformation",
            ),
            "UpdateStackInstances" => {
                self.handle_update_stack_instances(&state, &params, account_id)
                    .await
            }
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for CloudFormation"),
            ),
        };

        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // --- Stack handlers ---

    async fn handle_create_stack(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_stack_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let parameters = wire_parameters_to_domain(input.parameters);
        let tags = wire_tags_to_domain(input.tags);
        let capabilities = input.capabilities.map(|c| c.items).unwrap_or_default();

        let mut st = state.write().await;
        match st.create_stack(
            &input.stack_name,
            input.template_body,
            parameters,
            tags,
            capabilities,
            input.role_a_r_n,
            input.timeout_in_minutes,
            input.disable_rollback.unwrap_or(false),
            region,
            account_id,
        ) {
            Ok(stack_id) => wire::serialize_create_stack_response(&wire::CreateStackOutput {
                stack_id: Some(stack_id),
                ..Default::default()
            }),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_update_stack(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stack_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let parameters = wire_parameters_to_domain(input.parameters);
        let tags = wire_tags_to_domain(input.tags);
        let capabilities = input.capabilities.map(|c| c.items).unwrap_or_default();

        let mut st = state.write().await;
        match st.update_stack(
            &input.stack_name,
            input.template_body,
            parameters,
            tags,
            capabilities,
        ) {
            Ok(stack_id) => wire::serialize_update_stack_response(&wire::UpdateStackOutput {
                stack_id: Some(stack_id),
                ..Default::default()
            }),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_delete_stack(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_stack_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let mut st = state.write().await;
        match st.delete_stack(&input.stack_name) {
            Ok(()) => wire::serialize_delete_stack_response(),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_describe_stacks(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stacks_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_stacks(input.stack_name.as_deref()) {
            Ok(stacks) => {
                let wire_stacks: Vec<wire::Stack> =
                    stacks.iter().map(|s| stack_to_wire(s)).collect();
                wire::serialize_describe_stacks_response(&wire::DescribeStacksOutput {
                    stacks: Some(wire_stacks.into()),
                    ..Default::default()
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_stacks(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_stacks_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let status_filter = input
            .stack_status_filter
            .map(|f| f.items)
            .unwrap_or_default();
        let st = state.read().await;
        let stacks = st.list_stacks(&status_filter);
        let summaries: Vec<wire::StackSummary> =
            stacks.iter().map(|s| stack_to_summary(s)).collect();
        wire::serialize_list_stacks_response(&wire::ListStacksOutput {
            stack_summaries: Some(summaries.into()),
            ..Default::default()
        })
    }

    async fn handle_get_template(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_template_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let stack_name = match input.stack_name {
            Some(v) if !v.is_empty() => v,
            _ => return error_response(400, "ValidationError", "Missing 'StackName'"),
        };
        let st = state.read().await;
        match st.get_template(&stack_name) {
            Ok(body) => wire::serialize_get_template_response(&wire::GetTemplateOutput {
                template_body: body,
                ..Default::default()
            }),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_get_stack_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_stack_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let st = state.read().await;
        match st.get_stack_policy(&input.stack_name) {
            Ok(body) => wire::serialize_get_stack_policy_response(&wire::GetStackPolicyOutput {
                stack_policy_body: body,
            }),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_set_stack_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_stack_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let mut st = state.write().await;
        match st.set_stack_policy(&input.stack_name, input.stack_policy_body) {
            Ok(()) => wire::serialize_set_stack_policy_response(),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_describe_stack_events(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stack_events_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let st = state.read().await;
        match st.describe_stack_events(&input.stack_name) {
            Ok(events) => {
                let wire_events: Vec<wire::StackEvent> = events
                    .iter()
                    .map(|e| wire::StackEvent {
                        event_id: Some(e.event_id.clone()),
                        stack_id: Some(e.stack_id.clone()),
                        stack_name: Some(e.stack_name.clone()),
                        logical_resource_id: Some(e.logical_resource_id.clone()),
                        physical_resource_id: e.physical_resource_id.clone(),
                        resource_type: Some(e.resource_type.clone()),
                        timestamp: Some(e.timestamp.clone()),
                        resource_status: Some(e.resource_status.clone()),
                        resource_status_reason: e.resource_status_reason.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_stack_events_response(&wire::DescribeStackEventsOutput {
                    stack_events: Some(wire_events.into()),
                    ..Default::default()
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_describe_stack_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stack_resources_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let stack_name = match input.stack_name {
            Some(v) if !v.is_empty() => v,
            _ => return error_response(400, "ValidationError", "Missing 'StackName'"),
        };
        let st = state.read().await;
        match st.describe_stack_resources(&stack_name) {
            Ok(resources) => {
                let wire_resources: Vec<wire::StackResource> = resources
                    .iter()
                    .map(|r| resource_to_wire(r, &stack_name))
                    .collect();
                wire::serialize_describe_stack_resources_response(
                    &wire::DescribeStackResourcesOutput {
                        stack_resources: Some(wire_resources.into()),
                    },
                )
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_describe_stack_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stack_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        if input.logical_resource_id.is_empty() {
            return error_response(400, "ValidationError", "Missing 'LogicalResourceId'");
        }
        let st = state.read().await;
        match st.describe_stack_resource(&input.stack_name, &input.logical_resource_id) {
            Ok(Some(r)) => {
                let wire_detail = wire::StackResourceDetail {
                    logical_resource_id: Some(r.logical_resource_id.clone()),
                    physical_resource_id: r.physical_resource_id.clone(),
                    resource_type: Some(r.resource_type.clone()),
                    resource_status: Some(r.resource_status.clone()),
                    stack_id: Some(r.stack_id.clone()),
                    stack_name: Some(r.stack_name.clone()),
                    last_updated_timestamp: Some(r.timestamp.clone()),
                    ..Default::default()
                };
                wire::serialize_describe_stack_resource_response(
                    &wire::DescribeStackResourceOutput {
                        stack_resource_detail: Some(wire_detail),
                    },
                )
            }
            Ok(None) => error_response(
                400,
                "ValidationError",
                &format!(
                    "Resource '{}' does not exist for stack '{}'",
                    input.logical_resource_id, input.stack_name
                ),
            ),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_stack_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_stack_resources_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let st = state.read().await;
        match st.list_stack_resources(&input.stack_name) {
            Ok(resources) => {
                let wire_summaries: Vec<wire::StackResourceSummary> = resources
                    .iter()
                    .map(|r| wire::StackResourceSummary {
                        logical_resource_id: Some(r.logical_resource_id.clone()),
                        physical_resource_id: r.physical_resource_id.clone(),
                        resource_type: Some(r.resource_type.clone()),
                        resource_status: Some(r.resource_status.clone()),
                        last_updated_timestamp: Some(r.timestamp.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_stack_resources_response(&wire::ListStackResourcesOutput {
                    stack_resource_summaries: Some(wire_summaries.into()),
                    ..Default::default()
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_exports(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let exports = st.list_exports();
        let wire_exports: Vec<wire::Export> = exports
            .iter()
            .map(|e| wire::Export {
                name: Some(e.name.clone()),
                value: Some(e.value.clone()),
                exporting_stack_id: Some(e.exporting_stack_id.clone()),
            })
            .collect();
        wire::serialize_list_exports_response(&wire::ListExportsOutput {
            exports: Some(wire_exports.into()),
            ..Default::default()
        })
    }

    // --- ChangeSet handlers ---

    async fn handle_create_change_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_change_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        if input.change_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'ChangeSetName'");
        }
        let parameters = wire_parameters_to_domain(input.parameters);
        let tags = wire_tags_to_domain(input.tags);

        let mut st = state.write().await;
        match st.create_change_set(
            &input.stack_name,
            &input.change_set_name,
            input.description,
            parameters,
            tags,
            region,
            account_id,
        ) {
            Ok((change_set_id, stack_id)) => {
                wire::serialize_create_change_set_response(&wire::CreateChangeSetOutput {
                    id: Some(change_set_id),
                    stack_id: Some(stack_id),
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_delete_change_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_change_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let stack_name = match input.stack_name {
            Some(v) if !v.is_empty() => v,
            _ => return error_response(400, "ValidationError", "Missing 'StackName'"),
        };
        if input.change_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'ChangeSetName'");
        }
        let mut st = state.write().await;
        match st.delete_change_set(&stack_name, &input.change_set_name) {
            Ok(()) => wire::serialize_delete_change_set_response(&wire::DeleteChangeSetOutput {}),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_describe_change_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_change_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let stack_name = match input.stack_name {
            Some(v) if !v.is_empty() => v,
            _ => return error_response(400, "ValidationError", "Missing 'StackName'"),
        };
        if input.change_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'ChangeSetName'");
        }
        let st = state.read().await;
        match st.describe_change_set(&stack_name, &input.change_set_name) {
            Ok(cs) => {
                wire::serialize_describe_change_set_response(&wire::DescribeChangeSetOutput {
                    change_set_id: Some(cs.change_set_id.clone()),
                    change_set_name: Some(cs.change_set_name.clone()),
                    stack_id: Some(cs.stack_id.clone()),
                    stack_name: Some(cs.stack_name.clone()),
                    status: Some(cs.status.clone()),
                    status_reason: cs.status_reason.clone(),
                    execution_status: Some(cs.execution_status.clone()),
                    description: cs.description.clone(),
                    creation_time: Some(cs.creation_time.clone()),
                    parameters: Some(
                        cs.parameters
                            .iter()
                            .map(|p| wire::Parameter {
                                parameter_key: Some(p.parameter_key.clone()),
                                parameter_value: Some(p.parameter_value.clone()),
                                ..Default::default()
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                    ..Default::default()
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_execute_change_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_execute_change_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.change_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'ChangeSetName'");
        }
        let stack_name = match input.stack_name {
            Some(v) if !v.is_empty() => v,
            _ => return error_response(400, "ValidationError", "Missing 'StackName'"),
        };
        let mut st = state.write().await;
        match st.execute_change_set(&stack_name, &input.change_set_name) {
            Ok(()) => wire::serialize_execute_change_set_response(&wire::ExecuteChangeSetOutput {}),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_change_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_change_sets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let st = state.read().await;
        match st.list_change_sets(&input.stack_name) {
            Ok(change_sets) => {
                let summaries: Vec<wire::ChangeSetSummary> = change_sets
                    .iter()
                    .map(|cs| wire::ChangeSetSummary {
                        change_set_id: Some(cs.change_set_id.clone()),
                        change_set_name: Some(cs.change_set_name.clone()),
                        stack_id: Some(cs.stack_id.clone()),
                        stack_name: Some(cs.stack_name.clone()),
                        status: Some(cs.status.clone()),
                        status_reason: cs.status_reason.clone(),
                        execution_status: Some(cs.execution_status.clone()),
                        description: cs.description.clone(),
                        creation_time: Some(cs.creation_time.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_change_sets_response(&wire::ListChangeSetsOutput {
                    summaries: Some(summaries.into()),
                    ..Default::default()
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    // --- ValidateTemplate ---

    // STUB[no-engine]: ValidateTemplate requires full CloudFormation template parsing; always returns success.
    async fn handle_validate_template(&self, params: &HashMap<String, String>) -> MockResponse {
        // Always succeed; parse parameter keys from template if present
        let _input = match wire::deserialize_validate_template_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        wire::serialize_validate_template_response(&wire::ValidateTemplateOutput {
            description: Some("Template validated successfully".to_string()),
            ..Default::default()
        })
    }

    // --- StackSet handlers ---

    async fn handle_create_stack_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_stack_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let parameters = wire_parameters_to_domain(input.parameters);
        let tags = wire_tags_to_domain(input.tags);
        let capabilities = input.capabilities.map(|c| c.items).unwrap_or_default();

        let mut st = state.write().await;
        match st.create_stack_set(
            &input.stack_set_name,
            input.description,
            input.template_body,
            parameters,
            tags,
            capabilities,
            region,
            account_id,
        ) {
            Ok(stack_set_id) => {
                wire::serialize_create_stack_set_response(&wire::CreateStackSetOutput {
                    stack_set_id: Some(stack_set_id),
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_describe_stack_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stack_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let st = state.read().await;
        match st.describe_stack_set(&input.stack_set_name) {
            Ok(ss) => wire::serialize_describe_stack_set_response(&wire::DescribeStackSetOutput {
                stack_set: Some(stack_set_to_wire(ss)),
            }),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_update_stack_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stack_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let parameters = wire_parameters_to_domain(input.parameters);
        let tags = wire_tags_to_domain(input.tags);

        let mut st = state.write().await;
        match st.update_stack_set(
            &input.stack_set_name,
            input.description,
            input.template_body,
            parameters,
            tags,
            region,
            account_id,
        ) {
            Ok(operation_id) => {
                wire::serialize_update_stack_set_response(&wire::UpdateStackSetOutput {
                    operation_id: Some(operation_id),
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_delete_stack_set(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_stack_set_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let mut st = state.write().await;
        match st.delete_stack_set(&input.stack_set_name) {
            Ok(()) => wire::serialize_delete_stack_set_response(&wire::DeleteStackSetOutput {}),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_stack_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let stack_sets = st.list_stack_sets();
        let summaries: Vec<wire::StackSetSummary> = stack_sets
            .iter()
            .map(|ss| wire::StackSetSummary {
                stack_set_id: Some(ss.stack_set_id.clone()),
                stack_set_name: Some(ss.stack_set_name.clone()),
                status: Some(ss.status.clone()),
                description: ss.description.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_stack_sets_response(&wire::ListStackSetsOutput {
            summaries: Some(summaries.into()),
            ..Default::default()
        })
    }

    // --- StackInstance handlers ---

    async fn handle_create_stack_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_stack_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let accounts = input.accounts.map(|a| a.items).unwrap_or_default();
        let regions = input.regions.items;
        let parameter_overrides = wire_parameters_to_domain(input.parameter_overrides);

        let mut st = state.write().await;
        match st.create_stack_instances(
            &input.stack_set_name,
            &accounts,
            &regions,
            parameter_overrides,
            account_id,
        ) {
            Ok(operation_id) => {
                wire::serialize_create_stack_instances_response(&wire::CreateStackInstancesOutput {
                    operation_id: Some(operation_id),
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_update_stack_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stack_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let accounts = input.accounts.map(|a| a.items).unwrap_or_default();
        let regions = input.regions.items;
        let parameter_overrides = wire_parameters_to_domain(input.parameter_overrides);

        let final_accounts = if accounts.is_empty() {
            vec![account_id.to_string()]
        } else {
            accounts
        };

        let mut st = state.write().await;
        match st.update_stack_instances(
            &input.stack_set_name,
            &final_accounts,
            &regions,
            parameter_overrides,
        ) {
            Ok(operation_id) => {
                wire::serialize_update_stack_instances_response(&wire::UpdateStackInstancesOutput {
                    operation_id: Some(operation_id),
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_delete_stack_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_stack_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let accounts = input.accounts.map(|a| a.items).unwrap_or_default();
        let regions = input.regions.items;

        let mut st = state.write().await;
        match st.delete_stack_instances(&input.stack_set_name, &accounts, &regions) {
            Ok(operation_id) => {
                wire::serialize_delete_stack_instances_response(&wire::DeleteStackInstancesOutput {
                    operation_id: Some(operation_id),
                })
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_describe_stack_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stack_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        if input.stack_instance_account.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackInstanceAccount'");
        }
        if input.stack_instance_region.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackInstanceRegion'");
        }
        let st = state.read().await;
        match st.describe_stack_instance(
            &input.stack_set_name,
            &input.stack_instance_account,
            &input.stack_instance_region,
        ) {
            Ok(inst) => wire::serialize_describe_stack_instance_response(
                &wire::DescribeStackInstanceOutput {
                    stack_instance: Some(stack_instance_to_wire(inst)),
                },
            ),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_stack_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_stack_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let st = state.read().await;
        let instances = st.list_stack_instances(&input.stack_set_name);
        let summaries: Vec<wire::StackInstanceSummary> = instances
            .iter()
            .map(|i| wire::StackInstanceSummary {
                stack_set_id: Some(i.stack_set_id.clone()),
                account: Some(i.account.clone()),
                region: Some(i.region.clone()),
                stack_id: i.stack_id.clone(),
                status: Some(i.status.clone()),
                status_reason: i.status_reason.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_stack_instances_response(&wire::ListStackInstancesOutput {
            summaries: Some(summaries.into()),
            ..Default::default()
        })
    }

    // --- StackSetOperation handlers ---

    async fn handle_describe_stack_set_operation(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stack_set_operation_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        if input.operation_id.is_empty() {
            return error_response(400, "ValidationError", "Missing 'OperationId'");
        }
        let st = state.read().await;
        match st.describe_stack_set_operation(&input.stack_set_name, &input.operation_id) {
            Ok(op) => wire::serialize_describe_stack_set_operation_response(
                &wire::DescribeStackSetOperationOutput {
                    stack_set_operation: Some(wire::StackSetOperation {
                        operation_id: Some(op.operation_id.clone()),
                        action: Some(op.action.clone()),
                        status: Some(op.status.clone()),
                        creation_timestamp: Some(op.creation_timestamp.clone()),
                        end_timestamp: op.end_timestamp.clone(),
                        stack_set_id: Some(op.stack_set_id.clone()),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_stack_set_operations(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_stack_set_operations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        let st = state.read().await;
        match st.list_stack_set_operations(&input.stack_set_name) {
            Ok(ops) => {
                let summaries: Vec<wire::StackSetOperationSummary> = ops
                    .iter()
                    .map(|op| wire::StackSetOperationSummary {
                        operation_id: Some(op.operation_id.clone()),
                        action: Some(op.action.clone()),
                        status: Some(op.status.clone()),
                        creation_timestamp: Some(op.creation_timestamp.clone()),
                        end_timestamp: op.end_timestamp.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_stack_set_operations_response(
                    &wire::ListStackSetOperationsOutput {
                        summaries: Some(summaries.into()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_list_stack_set_operation_results(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_stack_set_operation_results_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        if input.operation_id.is_empty() {
            return error_response(400, "ValidationError", "Missing 'OperationId'");
        }
        let st = state.read().await;
        match st.list_stack_set_operation_results(&input.stack_set_name, &input.operation_id) {
            Ok(results) => {
                let wire_results: Vec<wire::StackSetOperationResultSummary> = results
                    .iter()
                    .map(|r| wire::StackSetOperationResultSummary {
                        account: Some(r.account.clone()),
                        region: Some(r.region.clone()),
                        status: Some(r.status.clone()),
                        status_reason: r.status_reason.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_stack_set_operation_results_response(
                    &wire::ListStackSetOperationResultsOutput {
                        summaries: Some(wire_results.into()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_stop_stack_set_operation(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_stack_set_operation_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_set_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackSetName'");
        }
        if input.operation_id.is_empty() {
            return error_response(400, "ValidationError", "Missing 'OperationId'");
        }
        let mut st = state.write().await;
        match st.stop_stack_set_operation(&input.stack_set_name, &input.operation_id) {
            Ok(()) => wire::serialize_stop_stack_set_operation_response(
                &wire::StopStackSetOperationOutput {},
            ),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_update_termination_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_termination_protection_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let mut st = state.write().await;
        match st
            .update_termination_protection(&input.stack_name, input.enable_termination_protection)
        {
            Ok(stack_id) => wire::serialize_update_termination_protection_response(
                &wire::UpdateTerminationProtectionOutput {
                    stack_id: Some(stack_id),
                },
            ),
            Err(e) => cf_error_response(&e),
        }
    }

    // STUB[no-telemetry]: DescribeAccountLimits returns real account quota data; returns hardcoded AWS defaults.
    async fn handle_describe_account_limits(&self) -> MockResponse {
        wire::serialize_describe_account_limits_response(&wire::DescribeAccountLimitsOutput {
            account_limits: Some(
                vec![
                    wire::AccountLimit {
                        name: Some("StackCount".to_string()),
                        value: Some(2000),
                    },
                    wire::AccountLimit {
                        name: Some("ConcurrentResourcesLimit".to_string()),
                        value: Some(2500),
                    },
                ]
                .into(),
            ),
            ..Default::default()
        })
    }

    async fn handle_list_imports(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_imports_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.export_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'ExportName'");
        }
        let st = state.read().await;
        let imports = st.list_imports(&input.export_name);
        wire::serialize_list_imports_response(&wire::ListImportsOutput {
            imports: if imports.is_empty() {
                None
            } else {
                Some(imports.into())
            },
            ..Default::default()
        })
    }

    async fn handle_get_template_summary(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_template_summary_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let description = if let Some(stack_name) = input.stack_name.as_deref() {
            st.stacks
                .get(stack_name)
                .and_then(|s| s.description.clone())
        } else {
            None
        };
        drop(st);
        wire::serialize_get_template_summary_response(&wire::GetTemplateSummaryOutput {
            description,
            version: Some("2010-09-09".to_string()),
            ..Default::default()
        })
    }

    async fn handle_cancel_update_stack(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_update_stack_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let mut st = state.write().await;
        match st.cancel_update_stack(&input.stack_name) {
            Ok(()) => wire::serialize_cancel_update_stack_response(),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_rollback_stack(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_rollback_stack_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.stack_name.is_empty() {
            return error_response(400, "ValidationError", "Missing 'StackName'");
        }
        let mut st = state.write().await;
        match st.rollback_stack(&input.stack_name) {
            Ok(stack_id) => wire::serialize_rollback_stack_response(&wire::RollbackStackOutput {
                stack_id: Some(stack_id),
                operation_id: None,
            }),
            Err(e) => cf_error_response(&e),
        }
    }

    async fn handle_signal_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_signal_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let stack_name = input.stack_name;
        let logical_resource_id = input.logical_resource_id;
        if stack_name.is_empty() || logical_resource_id.is_empty() {
            return MockResponse::error(
                400,
                "ValidationError",
                "StackName and LogicalResourceId are required",
            );
        }
        let state = state.read().await;
        if !state.stacks.contains_key(&stack_name) {
            return cf_error_response(&CloudFormationError::StackNotFound(stack_name));
        }
        wire::serialize_signal_resource_response()
    }

    // STUB[no-engine]: EstimateTemplateCost requires real AWS pricing integration; returns a fixed calculator URL.
    async fn handle_estimate_template_cost(&self) -> MockResponse {
        wire::serialize_estimate_template_cost_response(&wire::EstimateTemplateCostOutput {
            url: Some("https://aws.amazon.com/cloudformation/calculator/?key=estimate".to_string()),
        })
    }

    async fn handle_continue_update_rollback(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_continue_update_rollback_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let stack_name = input.stack_name;
        if stack_name.is_empty() {
            return MockResponse::error(400, "ValidationError", "StackName is required");
        }
        let state = state.read().await;
        // Find stack by name or ID
        let found = state
            .stacks
            .values()
            .any(|s| s.stack_name == stack_name || s.stack_id == stack_name);
        if !found {
            return cf_error_response(&CloudFormationError::StackNotFound(stack_name));
        }
        wire::serialize_continue_update_rollback_response(&wire::ContinueUpdateRollbackOutput {})
    }

    async fn handle_list_types(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFormationState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let type_summaries: Vec<wire::TypeSummary> = state
            .registered_types
            .iter()
            .map(|rt| wire::TypeSummary {
                type_name: Some(rt.type_name.clone()),
                r#type: Some(rt.type_kind.clone()),
                default_version_id: rt.default_version_id.clone(),
                type_arn: rt.type_arn.clone(),
                last_updated: rt.last_updated.clone(),
                description: rt.description.clone(),
                is_activated: Some(true),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_types_response(&wire::ListTypesOutput {
            type_summaries: if type_summaries.is_empty() {
                None
            } else {
                Some(wire::TypeSummaries::from(type_summaries))
            },
            ..Default::default()
        })
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urldecode(key);
            let value = urldecode(value);
            map.insert(key, value);
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().and_then(hex_val);
                let lo = chars.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
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

fn error_response(status: u16, code: &str, message: &str) -> MockResponse {
    MockResponse::error(status, code, message)
}

fn cf_error_response(e: &CloudFormationError) -> MockResponse {
    let (status, error_type) = match e {
        CloudFormationError::StackNotFound(_) => (400, "StackNotFoundException"),
        CloudFormationError::StackAlreadyExists(_) => (400, "AlreadyExistsException"),
        CloudFormationError::ChangeSetNotFound(_) => (400, "ChangeSetNotFoundException"),
        CloudFormationError::ChangeSetAlreadyExists(_) => (400, "AlreadyExistsException"),
        CloudFormationError::StackSetNotFound(_) => (400, "StackSetNotFoundException"),
        CloudFormationError::StackSetAlreadyExists(_) => (400, "AlreadyExistsException"),
        CloudFormationError::StackInstanceNotFound(_) => (400, "StackInstanceNotFoundException"),
        CloudFormationError::StackSetOperationNotFound(_) => {
            (400, "StackSetOperationNotFoundException")
        }
        CloudFormationError::ValidationError(_) => (400, "ValidationError"),
    };
    MockResponse::error(status, error_type, &e.to_string())
}

fn wire_parameters_to_domain(params: Option<wire::Parameters>) -> Vec<StackParameter> {
    params
        .map(|p| p.items)
        .unwrap_or_default()
        .into_iter()
        .map(|p| StackParameter {
            parameter_key: p.parameter_key.unwrap_or_default(),
            parameter_value: p.parameter_value.unwrap_or_default(),
        })
        .collect()
}

fn wire_tags_to_domain(tags: Option<wire::Tags>) -> Vec<StackTag> {
    tags.map(|t| t.items)
        .unwrap_or_default()
        .into_iter()
        .map(|t| StackTag {
            key: t.key,
            value: t.value,
        })
        .collect()
}

fn stack_to_wire(s: &crate::types::Stack) -> wire::Stack {
    wire::Stack {
        stack_id: Some(s.stack_id.clone()),
        stack_name: Some(s.stack_name.clone()),
        stack_status: Some(s.stack_status.clone()),
        creation_time: Some(s.creation_time.clone()),
        last_updated_time: s.last_updated_time.clone(),
        deletion_time: s.deletion_time.clone(),
        description: s.description.clone(),
        role_a_r_n: s.role_arn.clone(),
        timeout_in_minutes: s.timeout_in_minutes,
        disable_rollback: Some(s.disable_rollback),
        enable_termination_protection: Some(s.enable_termination_protection),
        parameters: if s.parameters.is_empty() {
            None
        } else {
            Some(
                s.parameters
                    .iter()
                    .map(|p| wire::Parameter {
                        parameter_key: Some(p.parameter_key.clone()),
                        parameter_value: Some(p.parameter_value.clone()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>()
                    .into(),
            )
        },
        tags: if s.tags.is_empty() {
            None
        } else {
            Some(
                s.tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect::<Vec<_>>()
                    .into(),
            )
        },
        ..Default::default()
    }
}

fn stack_to_summary(s: &crate::types::Stack) -> wire::StackSummary {
    wire::StackSummary {
        stack_id: Some(s.stack_id.clone()),
        stack_name: Some(s.stack_name.clone()),
        stack_status: Some(s.stack_status.clone()),
        creation_time: Some(s.creation_time.clone()),
        last_updated_time: s.last_updated_time.clone(),
        deletion_time: s.deletion_time.clone(),
        ..Default::default()
    }
}

fn resource_to_wire(r: &crate::types::StackResource, stack_name: &str) -> wire::StackResource {
    wire::StackResource {
        logical_resource_id: Some(r.logical_resource_id.clone()),
        physical_resource_id: r.physical_resource_id.clone(),
        resource_type: Some(r.resource_type.clone()),
        resource_status: Some(r.resource_status.clone()),
        timestamp: Some(r.timestamp.clone()),
        stack_id: Some(r.stack_id.clone()),
        stack_name: Some(stack_name.to_string()),
        ..Default::default()
    }
}

fn stack_set_to_wire(ss: &crate::types::StackSet) -> wire::StackSet {
    wire::StackSet {
        stack_set_id: Some(ss.stack_set_id.clone()),
        stack_set_name: Some(ss.stack_set_name.clone()),
        stack_set_a_r_n: Some(ss.stack_set_arn.clone()),
        status: Some(ss.status.clone()),
        description: ss.description.clone(),
        template_body: ss.template_body.clone(),
        parameters: if ss.parameters.is_empty() {
            None
        } else {
            Some(
                ss.parameters
                    .iter()
                    .map(|p| wire::Parameter {
                        parameter_key: Some(p.parameter_key.clone()),
                        parameter_value: Some(p.parameter_value.clone()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>()
                    .into(),
            )
        },
        ..Default::default()
    }
}

fn stack_instance_to_wire(i: &crate::types::StackInstance) -> wire::StackInstance {
    wire::StackInstance {
        stack_set_id: Some(i.stack_set_id.clone()),
        account: Some(i.account.clone()),
        region: Some(i.region.clone()),
        stack_id: i.stack_id.clone(),
        status: Some(i.status.clone()),
        status_reason: i.status_reason.clone(),
        ..Default::default()
    }
}
