use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{SsmError, SsmState};
use crate::views::SsmStateView;
use crate::wire;

pub struct SsmService {
    pub(crate) state: Arc<BackendState<SsmState>>,
    pub(crate) notifier: StateChangeNotifier<SsmStateView>,
}

impl SsmService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SsmService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SsmService {
    fn service_name(&self) -> &str {
        "ssm"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://ssm\..*\.amazonaws\.com",
            r"https?://ssm\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SsmService {
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

        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // Parameter operations
            "PutParameter" => {
                self.handle_put_parameter(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetParameter" => self.handle_get_parameter(&state, body_bytes).await,
            "GetParameters" => self.handle_get_parameters(&state, body_bytes).await,
            "GetParametersByPath" => self.handle_get_parameters_by_path(&state, body_bytes).await,
            "GetParameterHistory" => self.handle_get_parameter_history(&state, body_bytes).await,
            "DeleteParameter" => self.handle_delete_parameter(&state, body_bytes).await,
            "DeleteParameters" => self.handle_delete_parameters(&state, body_bytes).await,
            "DescribeParameters" => self.handle_describe_parameters(&state, body_bytes).await,
            "LabelParameterVersion" => {
                self.handle_label_parameter_version(&state, body_bytes)
                    .await
            }
            "UnlabelParameterVersion" => {
                self.handle_unlabel_parameter_version(&state, body_bytes)
                    .await
            }

            // Tag operations
            "AddTagsToResource" => self.handle_add_tags_to_resource(&state, body_bytes).await,
            "RemoveTagsFromResource" => {
                self.handle_remove_tags_from_resource(&state, body_bytes)
                    .await
            }
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,

            // Document operations
            "CreateDocument" => {
                self.handle_create_document(&state, body_bytes, account_id)
                    .await
            }
            "GetDocument" => self.handle_get_document(&state, body_bytes).await,
            "DescribeDocument" => self.handle_describe_document(&state, body_bytes).await,
            "DeleteDocument" => self.handle_delete_document(&state, body_bytes).await,
            "UpdateDocument" => self.handle_update_document(&state, body_bytes).await,
            "UpdateDocumentDefaultVersion" => {
                self.handle_update_document_default_version(&state, body_bytes)
                    .await
            }
            "ListDocuments" => self.handle_list_documents(&state).await,
            "DescribeDocumentPermission" => {
                self.handle_describe_document_permission(&state, body_bytes)
                    .await
            }
            "ModifyDocumentPermission" => {
                self.handle_modify_document_permission(&state, body_bytes)
                    .await
            }

            // Maintenance Window operations
            "CreateMaintenanceWindow" => {
                self.handle_create_maintenance_window(&state, body_bytes)
                    .await
            }
            "GetMaintenanceWindow" => self.handle_get_maintenance_window(&state, body_bytes).await,
            "DeleteMaintenanceWindow" => {
                self.handle_delete_maintenance_window(&state, body_bytes)
                    .await
            }
            "DescribeMaintenanceWindows" => self.handle_describe_maintenance_windows(&state).await,
            "RegisterTargetWithMaintenanceWindow" => {
                self.handle_register_target_with_maintenance_window(&state, body_bytes)
                    .await
            }
            "DeregisterTargetFromMaintenanceWindow" => {
                self.handle_deregister_target_from_maintenance_window(&state, body_bytes)
                    .await
            }
            "DescribeMaintenanceWindowTargets" => {
                self.handle_describe_maintenance_window_targets(&state, body_bytes)
                    .await
            }
            "RegisterTaskWithMaintenanceWindow" => {
                self.handle_register_task_with_maintenance_window(&state, body_bytes)
                    .await
            }
            "DeregisterTaskFromMaintenanceWindow" => {
                self.handle_deregister_task_from_maintenance_window(&state, body_bytes)
                    .await
            }
            "DescribeMaintenanceWindowTasks" => {
                self.handle_describe_maintenance_window_tasks(&state, body_bytes)
                    .await
            }

            // Patch Baseline operations
            "CreatePatchBaseline" => self.handle_create_patch_baseline(&state, body_bytes).await,
            "DeletePatchBaseline" => self.handle_delete_patch_baseline(&state, body_bytes).await,
            "DescribePatchBaselines" => self.handle_describe_patch_baselines(&state).await,
            "RegisterPatchBaselineForPatchGroup" => {
                self.handle_register_patch_baseline_for_patch_group(&state, body_bytes)
                    .await
            }
            "DeregisterPatchBaselineForPatchGroup" => {
                self.handle_deregister_patch_baseline_for_patch_group(&state, body_bytes)
                    .await
            }
            "GetPatchBaselineForPatchGroup" => {
                self.handle_get_patch_baseline_for_patch_group(&state, body_bytes)
                    .await
            }

            // Command operations
            "SendCommand" => self.handle_send_command(&state, body_bytes).await,
            "ListCommands" => self.handle_list_commands(&state).await,
            "GetCommandInvocation" => self.handle_get_command_invocation(&state, body_bytes).await,

            // Stub operations for full API coverage
            "AssociateOpsItemRelatedItem" => {
                self.handle_associate_ops_item_related_item(&state, body_bytes)
                    .await
            }
            "CancelCommand" => self.handle_cancel_command(&state, body_bytes).await,
            "CancelMaintenanceWindowExecution" => {
                self.handle_cancel_maintenance_window_execution(&state, body_bytes)
                    .await
            }
            "CreateActivation" => self.handle_create_activation(&state, body_bytes).await,
            "CreateAssociation" => self.handle_create_association(&state, body_bytes).await,
            "CreateAssociationBatch" => {
                self.handle_create_association_batch(&state, body_bytes)
                    .await
            }
            "CreateOpsItem" => self.handle_create_ops_item(&state, body_bytes).await,
            "CreateOpsMetadata" => {
                self.handle_create_ops_metadata(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateResourceDataSync" => {
                self.handle_create_resource_data_sync(&state, body_bytes)
                    .await
            }
            "DeleteActivation" => self.handle_delete_activation(&state, body_bytes).await,
            "DeleteAssociation" => self.handle_delete_association(&state, body_bytes).await,
            "DeleteInventory" => self.handle_delete_inventory(&state, body_bytes).await,
            "DeleteOpsItem" => self.handle_delete_ops_item(&state, body_bytes).await,
            "DeleteOpsMetadata" => self.handle_delete_ops_metadata(&state, body_bytes).await,
            "DeleteResourceDataSync" => {
                self.handle_delete_resource_data_sync(&state, body_bytes)
                    .await
            }
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, body_bytes).await,
            "DeregisterManagedInstance" => {
                self.handle_deregister_managed_instance(&state, body_bytes)
                    .await
            }
            "DescribeActivations" => self.handle_describe_activations(&state, body_bytes).await,
            "DescribeAssociation" => self.handle_describe_association(&state, body_bytes).await,
            "DescribeAssociationExecutionTargets" => {
                self.handle_describe_association_execution_targets(&state, body_bytes)
                    .await
            }
            "DescribeAssociationExecutions" => {
                self.handle_describe_association_executions(&state, body_bytes)
                    .await
            }
            "DescribeAutomationExecutions" => {
                self.handle_describe_automation_executions(&state, body_bytes)
                    .await
            }
            "DescribeAutomationStepExecutions" => {
                self.handle_describe_automation_step_executions(&state, body_bytes)
                    .await
            }
            "DescribeAvailablePatches" => {
                self.handle_describe_available_patches(&state, body_bytes)
                    .await
            }
            "DescribeEffectiveInstanceAssociations" => {
                self.handle_describe_effective_instance_associations(&state, body_bytes)
                    .await
            }
            "DescribeEffectivePatchesForPatchBaseline" => {
                self.handle_describe_effective_patches_for_patch_baseline(&state, body_bytes)
                    .await
            }
            "DescribeInstanceAssociationsStatus" => {
                self.handle_describe_instance_associations_status(&state, body_bytes)
                    .await
            }
            "DescribeInstanceInformation" => {
                self.handle_describe_instance_information(&state, body_bytes)
                    .await
            }
            "DescribeInstancePatchStates" => {
                self.handle_describe_instance_patch_states(&state, body_bytes)
                    .await
            }
            "DescribeInstancePatchStatesForPatchGroup" => {
                self.handle_describe_instance_patch_states_for_patch_group(&state, body_bytes)
                    .await
            }
            "DescribeInstancePatches" => {
                self.handle_describe_instance_patches(&state, body_bytes)
                    .await
            }
            "DescribeInstanceProperties" => {
                self.handle_describe_instance_properties(&state, body_bytes)
                    .await
            }
            "DescribeInventoryDeletions" => {
                self.handle_describe_inventory_deletions(&state, body_bytes)
                    .await
            }
            "DescribeMaintenanceWindowExecutionTaskInvocations" => {
                self.handle_describe_maintenance_window_execution_task_invocations(
                    &state, body_bytes,
                )
                .await
            }
            "DescribeMaintenanceWindowExecutionTasks" => {
                self.handle_describe_maintenance_window_execution_tasks(&state, body_bytes)
                    .await
            }
            "DescribeMaintenanceWindowExecutions" => {
                self.handle_describe_maintenance_window_executions(&state, body_bytes)
                    .await
            }
            "DescribeMaintenanceWindowSchedule" => {
                self.handle_describe_maintenance_window_schedule(&state, body_bytes)
                    .await
            }
            "DescribeMaintenanceWindowsForTarget" => {
                self.handle_describe_maintenance_windows_for_target(&state, body_bytes)
                    .await
            }
            "DescribeOpsItems" => self.handle_describe_ops_items(&state, body_bytes).await,
            "DescribePatchGroupState" => {
                self.handle_describe_patch_group_state(&state, body_bytes)
                    .await
            }
            "DescribePatchGroups" => self.handle_describe_patch_groups(&state, body_bytes).await,
            "DescribePatchProperties" => {
                self.handle_describe_patch_properties(&state, body_bytes)
                    .await
            }
            "DescribeSessions" => self.handle_describe_sessions(&state, body_bytes).await,
            "DisassociateOpsItemRelatedItem" => {
                self.handle_disassociate_ops_item_related_item(&state, body_bytes)
                    .await
            }
            "GetAccessToken" => self.handle_get_access_token(&state, body_bytes).await,
            "GetAutomationExecution" => {
                self.handle_get_automation_execution(&state, body_bytes)
                    .await
            }
            "GetCalendarState" => self.handle_get_calendar_state(&state, body_bytes).await,
            "GetConnectionStatus" => self.handle_get_connection_status(&state, body_bytes).await,
            "GetDefaultPatchBaseline" => {
                self.handle_get_default_patch_baseline(&state, body_bytes)
                    .await
            }
            "GetDeployablePatchSnapshotForInstance" => {
                self.handle_get_deployable_patch_snapshot_for_instance(&state, body_bytes)
                    .await
            }
            "GetExecutionPreview" => self.handle_get_execution_preview(&state, body_bytes).await,
            "GetInventory" => self.handle_get_inventory(&state, body_bytes).await,
            "GetInventorySchema" => self.handle_get_inventory_schema(&state, body_bytes).await,
            "GetMaintenanceWindowExecution" => {
                self.handle_get_maintenance_window_execution(&state, body_bytes)
                    .await
            }
            "GetMaintenanceWindowExecutionTask" => {
                self.handle_get_maintenance_window_execution_task(&state, body_bytes)
                    .await
            }
            "GetMaintenanceWindowExecutionTaskInvocation" => {
                self.handle_get_maintenance_window_execution_task_invocation(&state, body_bytes)
                    .await
            }
            "GetMaintenanceWindowTask" => {
                self.handle_get_maintenance_window_task(&state, body_bytes)
                    .await
            }
            "GetOpsItem" => self.handle_get_ops_item(&state, body_bytes).await,
            "GetOpsMetadata" => self.handle_get_ops_metadata(&state, body_bytes).await,
            "GetOpsSummary" => self.handle_get_ops_summary(&state, body_bytes).await,
            "GetPatchBaseline" => self.handle_get_patch_baseline(&state, body_bytes).await,
            "GetResourcePolicies" => self.handle_get_resource_policies(&state, body_bytes).await,
            "GetServiceSetting" => self.handle_get_service_setting(&state, body_bytes).await,
            "ListAssociationVersions" => {
                self.handle_list_association_versions(&state, body_bytes)
                    .await
            }
            "ListAssociations" => self.handle_list_associations(&state, body_bytes).await,
            "ListCommandInvocations" => {
                self.handle_list_command_invocations(&state, body_bytes)
                    .await
            }
            "ListComplianceItems" => self.handle_list_compliance_items(&state, body_bytes).await,
            "ListComplianceSummaries" => {
                self.handle_list_compliance_summaries(&state, body_bytes)
                    .await
            }
            "ListDocumentMetadataHistory" => {
                self.handle_list_document_metadata_history(&state, body_bytes)
                    .await
            }
            "ListDocumentVersions" => self.handle_list_document_versions(&state, body_bytes).await,
            "ListInventoryEntries" => self.handle_list_inventory_entries(&state, body_bytes).await,
            "ListNodes" => self.handle_list_nodes(&state, body_bytes).await,
            "ListNodesSummary" => self.handle_list_nodes_summary(&state, body_bytes).await,
            "ListOpsItemEvents" => self.handle_list_ops_item_events(&state, body_bytes).await,
            "ListOpsItemRelatedItems" => {
                self.handle_list_ops_item_related_items(&state, body_bytes)
                    .await
            }
            "ListOpsMetadata" => self.handle_list_ops_metadata(&state, body_bytes).await,
            "ListResourceComplianceSummaries" => {
                self.handle_list_resource_compliance_summaries(&state, body_bytes)
                    .await
            }
            "ListResourceDataSync" => {
                self.handle_list_resource_data_sync(&state, body_bytes)
                    .await
            }
            "PutComplianceItems" => self.handle_put_compliance_items(&state, body_bytes).await,
            "PutInventory" => self.handle_put_inventory(&state, body_bytes).await,
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "RegisterDefaultPatchBaseline" => {
                self.handle_register_default_patch_baseline(&state, body_bytes)
                    .await
            }
            "ResetServiceSetting" => self.handle_reset_service_setting(&state, body_bytes).await,
            "ResumeSession" => self.handle_resume_session(&state, body_bytes).await,
            "SendAutomationSignal" => self.handle_send_automation_signal(&state, body_bytes).await,
            "StartAccessRequest" => self.handle_start_access_request(&state, body_bytes).await,
            "StartAssociationsOnce" => {
                self.handle_start_associations_once(&state, body_bytes)
                    .await
            }
            "StartAutomationExecution" => {
                self.handle_start_automation_execution(&state, body_bytes)
                    .await
            }
            "StartChangeRequestExecution" => {
                self.handle_start_change_request_execution(&state, body_bytes)
                    .await
            }
            "StartExecutionPreview" => {
                self.handle_start_execution_preview(&state, body_bytes)
                    .await
            }
            "StartSession" => self.handle_start_session(&state, body_bytes).await,
            "StopAutomationExecution" => {
                self.handle_stop_automation_execution(&state, body_bytes)
                    .await
            }
            "TerminateSession" => self.handle_terminate_session(&state, body_bytes).await,
            "UpdateAssociation" => self.handle_update_association(&state, body_bytes).await,
            "UpdateAssociationStatus" => {
                self.handle_update_association_status(&state, body_bytes)
                    .await
            }
            "UpdateDocumentMetadata" => {
                self.handle_update_document_metadata(&state, body_bytes)
                    .await
            }
            "UpdateMaintenanceWindow" => {
                self.handle_update_maintenance_window(&state, body_bytes)
                    .await
            }
            "UpdateMaintenanceWindowTarget" => {
                self.handle_update_maintenance_window_target(&state, body_bytes)
                    .await
            }
            "UpdateMaintenanceWindowTask" => {
                self.handle_update_maintenance_window_task(&state, body_bytes)
                    .await
            }
            "UpdateManagedInstanceRole" => {
                self.handle_update_managed_instance_role(&state, body_bytes)
                    .await
            }
            "UpdateOpsItem" => self.handle_update_ops_item(&state, body_bytes).await,
            "UpdateOpsMetadata" => self.handle_update_ops_metadata(&state, body_bytes).await,
            "UpdatePatchBaseline" => self.handle_update_patch_baseline(&state, body_bytes).await,
            "UpdateResourceDataSync" => {
                self.handle_update_resource_data_sync(&state, body_bytes)
                    .await
            }
            "UpdateServiceSetting" => self.handle_update_service_setting(&state, body_bytes).await,

            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for SSM"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ── Parameter operations ──────────────────────────────────────────

    async fn handle_put_parameter(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_parameter_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.value.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Value'");
        }
        let name = input.name.as_str();
        let value = input.value.as_str();
        let param_type = input.r#type.as_deref().unwrap_or("String");
        let overwrite = input.overwrite.unwrap_or(false);
        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();

        let mut state = state.write().await;
        match state.put_parameter(name, value, param_type, account_id, region, overwrite, tags) {
            Ok(version) => wire::serialize_put_parameter_response(&wire::PutParameterResult {
                version: Some(version),
                tier: Some("Standard".to_string()),
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_get_parameter(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_parameter_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let state = state.read().await;
        match state.get_parameter(name) {
            Ok(param) => wire::serialize_get_parameter_response(&wire::GetParameterResult {
                parameter: Some(to_wire_parameter(param)),
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_get_parameters(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_parameters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let names: Vec<&str> = input.names.iter().map(|s| s.as_str()).collect();

        let state = state.read().await;
        let (valid, invalid) = state.get_parameters(&names);

        let params: Vec<wire::Parameter> = valid.iter().map(|p| to_wire_parameter(p)).collect();

        wire::serialize_get_parameters_response(&wire::GetParametersResult {
            parameters: Some(params),
            invalid_parameters: Some(invalid),
        })
    }

    async fn handle_get_parameters_by_path(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_parameters_by_path_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.path.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Path'");
        }
        let path = input.path.as_str();
        let recursive = input.recursive.unwrap_or(false);

        let state = state.read().await;
        let params = state.get_parameters_by_path(path, recursive);
        let entries: Vec<wire::Parameter> = params.iter().map(|p| to_wire_parameter(p)).collect();

        wire::serialize_get_parameters_by_path_response(&wire::GetParametersByPathResult {
            parameters: Some(entries),
            next_token: None,
        })
    }

    async fn handle_get_parameter_history(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_parameter_history_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let state = state.read().await;
        match state.get_parameter_history(name) {
            Ok(versions) => {
                let entries: Vec<wire::ParameterHistory> = versions
                    .iter()
                    .map(|pv| wire::ParameterHistory {
                        name: Some(pv.name.clone()),
                        r#type: Some(pv.r#type.clone()),
                        value: Some(pv.value.clone()),
                        version: Some(pv.version),
                        last_modified_date: Some(pv.last_modified_date.timestamp() as f64),
                        labels: if pv.labels.is_empty() {
                            None
                        } else {
                            Some(pv.labels.clone())
                        },
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_parameter_history_response(&wire::GetParameterHistoryResult {
                    parameters: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_parameter(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_parameter_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let mut state = state.write().await;
        match state.delete_parameter(name) {
            Ok(()) => wire::serialize_delete_parameter_response(&wire::DeleteParameterResult {}),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_parameters(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_parameters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let names: Vec<&str> = input.names.iter().map(|s| s.as_str()).collect();

        let mut state = state.write().await;
        let (deleted, invalid) = state.delete_parameters(&names);

        wire::serialize_delete_parameters_response(&wire::DeleteParametersResult {
            deleted_parameters: Some(deleted),
            invalid_parameters: Some(invalid),
        })
    }

    async fn handle_describe_parameters(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_parameters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;

        // Extract Name/Equals filters from ParameterFilters (used by the AWS provider
        // to look up a specific parameter by exact name).
        let name_filter: Option<Vec<String>> = input
            .parameter_filters
            .unwrap_or_default()
            .into_iter()
            .find(|f| f.key == "Name" && f.option.as_deref() == Some("Equals"))
            .and_then(|f| f.values);

        let all_params = state.describe_parameters();
        let params: Vec<_> = match &name_filter {
            Some(names) => all_params
                .into_iter()
                .filter(|p| names.iter().any(|n| n == &p.name))
                .collect(),
            None => all_params,
        };

        let entries: Vec<wire::ParameterMetadata> = params
            .iter()
            .map(|p| wire::ParameterMetadata {
                name: Some(p.name.clone()),
                r#type: Some(p.r#type.clone()),
                last_modified_date: Some(p.last_modified_date.timestamp() as f64),
                version: Some(p.version),
                data_type: Some(p.data_type.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_parameters_response(&wire::DescribeParametersResult {
            parameters: Some(entries),
            next_token: None,
        })
    }

    async fn handle_label_parameter_version(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_label_parameter_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let labels = input.labels;
        let version = input.parameter_version;

        let mut state = state.write().await;
        match state.label_parameter_version(name, version, labels) {
            Ok(invalid_labels) => wire::serialize_label_parameter_version_response(
                &wire::LabelParameterVersionResult {
                    invalid_labels: if invalid_labels.is_empty() {
                        None
                    } else {
                        Some(invalid_labels)
                    },
                    parameter_version: version,
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_unlabel_parameter_version(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_unlabel_parameter_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let version = input.parameter_version;
        let labels = input.labels;

        let mut state = state.write().await;
        match state.unlabel_parameter_version(name, version, labels) {
            Ok(invalid_labels) => wire::serialize_unlabel_parameter_version_response(
                &wire::UnlabelParameterVersionResult {
                    invalid_labels: if invalid_labels.is_empty() {
                        None
                    } else {
                        Some(invalid_labels)
                    },
                    removed_labels: None,
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    // ── Tag operations ────────────────────────────────────────────────

    async fn handle_add_tags_to_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_to_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        let resource_id = input.resource_id.as_str();
        let tags: HashMap<String, String> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();

        let mut state = state.write().await;
        match state.add_tags_to_resource(resource_id, tags) {
            Ok(()) => {
                wire::serialize_add_tags_to_resource_response(&wire::AddTagsToResourceResult {})
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_remove_tags_from_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_from_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        let resource_id = input.resource_id.as_str();
        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        match state.remove_tags_from_resource(resource_id, &tag_keys) {
            Ok(()) => wire::serialize_remove_tags_from_resource_response(
                &wire::RemoveTagsFromResourceResult {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        let resource_id = input.resource_id.as_str();

        let state = state.read().await;
        let tags = state.list_tags_for_resource(resource_id);
        let tag_list: Vec<wire::Tag> = tags
            .iter()
            .map(|(k, v)| wire::Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();

        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResult {
            tag_list: if tag_list.is_empty() {
                None
            } else {
                Some(tag_list)
            },
        })
    }

    // ── Document operations ───────────────────────────────────────────

    async fn handle_create_document(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_document_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.content.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Content'");
        }
        let name = input.name.as_str();
        let content = input.content.as_str();
        let document_type = input.document_type.as_deref().unwrap_or("Command");
        let document_format = input.document_format.as_deref().unwrap_or("JSON");

        let mut state = state.write().await;
        match state.create_document(name, content, document_type, document_format, account_id) {
            Ok(doc) => wire::serialize_create_document_response(&wire::CreateDocumentResult {
                document_description: Some(to_wire_document_description(doc)),
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_get_document(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_document_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let version = input.document_version.as_deref();

        let state = state.read().await;
        match state.get_document(name, version) {
            Ok(doc) => {
                // For GetDocument, find the right version content
                let (content, doc_version) = if let Some(ver) = version {
                    doc.versions
                        .iter()
                        .find(|v| v.document_version == ver)
                        .map(|v| (v.content.clone(), v.document_version.clone()))
                        .unwrap_or((doc.content.clone(), doc.default_version.clone()))
                } else {
                    (doc.content.clone(), doc.default_version.clone())
                };
                wire::serialize_get_document_response(&wire::GetDocumentResult {
                    name: Some(doc.name.clone()),
                    content: Some(content),
                    document_type: Some(doc.document_type.clone()),
                    document_format: Some(doc.document_format.clone()),
                    document_version: Some(doc_version),
                    status: Some(doc.status.clone()),
                    created_date: Some(doc.created_date.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_document(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_document_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let state = state.read().await;
        match state.describe_document(name) {
            Ok(doc) => wire::serialize_describe_document_response(&wire::DescribeDocumentResult {
                document: Some(to_wire_document_description(doc)),
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_document(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_document_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let mut state = state.write().await;
        match state.delete_document(name) {
            Ok(()) => wire::serialize_delete_document_response(&wire::DeleteDocumentResult {}),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_document(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_document_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.content.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Content'");
        }
        let name = input.name.as_str();
        let content = input.content.as_str();
        let document_version = input.document_version.as_deref();

        let mut state = state.write().await;
        match state.update_document(name, content, document_version) {
            Ok(doc) => wire::serialize_update_document_response(&wire::UpdateDocumentResult {
                document_description: Some(to_wire_document_description(doc)),
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_document_default_version(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_document_default_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.document_version.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DocumentVersion'");
        }
        let name = input.name.as_str();
        let document_version = input.document_version.as_str();

        let mut state = state.write().await;
        match state.update_document_default_version(name, document_version) {
            Ok(doc) => wire::serialize_update_document_default_version_response(
                &wire::UpdateDocumentDefaultVersionResult {
                    description: Some(wire::DocumentDefaultVersionDescription {
                        name: Some(doc.name.clone()),
                        default_version: Some(doc.default_version.clone()),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_list_documents(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let docs = state.list_documents();
        let identifiers: Vec<wire::DocumentIdentifier> = docs
            .iter()
            .map(|doc| wire::DocumentIdentifier {
                name: Some(doc.name.clone()),
                owner: Some(doc.owner.clone()),
                document_type: Some(doc.document_type.clone()),
                document_format: Some(doc.document_format.clone()),
                document_version: Some(doc.latest_version.clone()),
                created_date: Some(doc.created_date.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_documents_response(&wire::ListDocumentsResult {
            document_identifiers: Some(identifiers),
            next_token: None,
        })
    }

    async fn handle_describe_document_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_document_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let state = state.read().await;
        match state.describe_document_permission(name) {
            Ok(doc) => wire::serialize_describe_document_permission_response(
                &wire::DescribeDocumentPermissionResponse {
                    account_ids: Some(doc.account_permissions.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_modify_document_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_document_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let permission_type = if input.permission_type.is_empty() {
            "Share"
        } else {
            input.permission_type.as_str()
        };
        let account_ids_to_add = input.account_ids_to_add.unwrap_or_default();
        let account_ids_to_remove = input.account_ids_to_remove.unwrap_or_default();

        let mut state = state.write().await;
        match state.modify_document_permission(
            name,
            permission_type,
            account_ids_to_add,
            account_ids_to_remove,
        ) {
            Ok(()) => wire::serialize_modify_document_permission_response(
                &wire::ModifyDocumentPermissionResponse {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    // ── Maintenance Window operations ─────────────────────────────────

    async fn handle_create_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_maintenance_window_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.schedule.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Schedule'");
        }
        let name = input.name.as_str();
        let schedule = input.schedule.as_str();
        let duration = if input.duration == 0 {
            1
        } else {
            input.duration as i64
        };
        let cutoff = input.cutoff as i64;
        // CreateMaintenanceWindow has no `Enabled` input field per Smithy; AWS
        // creates new maintenance windows in the enabled state.
        let enabled = true;

        let mut state = state.write().await;
        let window_id = state.create_maintenance_window(name, schedule, duration, cutoff, enabled);

        wire::serialize_create_maintenance_window_response(&wire::CreateMaintenanceWindowResult {
            window_id: Some(window_id),
        })
    }

    async fn handle_get_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_maintenance_window_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        let window_id = input.window_id.as_str();

        let state = state.read().await;
        match state.get_maintenance_window(window_id) {
            Ok(window) => {
                wire::serialize_get_maintenance_window_response(&wire::GetMaintenanceWindowResult {
                    window_id: Some(window.window_id.clone()),
                    name: Some(window.name.clone()),
                    schedule: Some(window.schedule.clone()),
                    duration: Some(window.duration as i32),
                    cutoff: Some(window.cutoff as i32),
                    enabled: Some(window.enabled),
                    ..Default::default()
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_maintenance_window_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        let window_id = input.window_id.as_str();

        let mut state = state.write().await;
        match state.delete_maintenance_window(window_id) {
            Ok(()) => wire::serialize_delete_maintenance_window_response(
                &wire::DeleteMaintenanceWindowResult {
                    window_id: Some(window_id.to_string()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_maintenance_windows(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let windows = state.describe_maintenance_windows();
        let identities: Vec<wire::MaintenanceWindowIdentity> = windows
            .iter()
            .map(|w| wire::MaintenanceWindowIdentity {
                window_id: Some(w.window_id.clone()),
                name: Some(w.name.clone()),
                schedule: Some(w.schedule.clone()),
                duration: Some(w.duration as i32),
                cutoff: Some(w.cutoff as i32),
                enabled: Some(w.enabled),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_maintenance_windows_response(
            &wire::DescribeMaintenanceWindowsResult {
                window_identities: Some(identities),
                next_token: None,
            },
        )
    }

    async fn handle_register_target_with_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_target_with_maintenance_window_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        let window_id = input.window_id.as_str();
        let resource_type = if input.resource_type.is_empty() {
            "INSTANCE"
        } else {
            input.resource_type.as_str()
        };
        let targets = wire_targets_to_types(Some(input.targets));

        let mut state = state.write().await;
        match state.register_target_with_maintenance_window(window_id, resource_type, targets) {
            Ok(target_id) => wire::serialize_register_target_with_maintenance_window_response(
                &wire::RegisterTargetWithMaintenanceWindowResult {
                    window_target_id: Some(target_id),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_deregister_target_from_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_target_from_maintenance_window_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        if input.window_target_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowTargetId'");
        }
        let window_id = input.window_id.as_str();
        let window_target_id = input.window_target_id.as_str();

        let mut state = state.write().await;
        match state.deregister_target_from_maintenance_window(window_id, window_target_id) {
            Ok(()) => wire::serialize_deregister_target_from_maintenance_window_response(
                &wire::DeregisterTargetFromMaintenanceWindowResult {
                    window_id: Some(window_id.to_string()),
                    window_target_id: Some(window_target_id.to_string()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_maintenance_window_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_maintenance_window_targets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        let window_id = input.window_id.as_str();

        let state = state.read().await;
        match state.describe_maintenance_window_targets(window_id) {
            Ok(targets) => {
                let wire_targets: Vec<wire::MaintenanceWindowTarget> = targets
                    .iter()
                    .map(|t| wire::MaintenanceWindowTarget {
                        window_id: Some(t.window_id.clone()),
                        window_target_id: Some(t.window_target_id.clone()),
                        resource_type: Some(t.resource_type.clone()),
                        targets: Some(
                            t.targets
                                .iter()
                                .map(|tgt| wire::Target {
                                    key: Some(tgt.key.clone()),
                                    values: Some(tgt.values.clone()),
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_maintenance_window_targets_response(
                    &wire::DescribeMaintenanceWindowTargetsResult {
                        targets: Some(wire_targets),
                        next_token: None,
                    },
                )
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_register_task_with_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_task_with_maintenance_window_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        if input.task_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TaskArn'");
        }
        if input.task_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TaskType'");
        }
        let window_id = input.window_id.as_str();
        let task_arn = input.task_arn.as_str();
        let task_type = input.task_type.as_str();
        let targets = wire_targets_to_types(input.targets);

        let mut state = state.write().await;
        match state.register_task_with_maintenance_window(window_id, task_arn, task_type, targets) {
            Ok(task_id) => wire::serialize_register_task_with_maintenance_window_response(
                &wire::RegisterTaskWithMaintenanceWindowResult {
                    window_task_id: Some(task_id),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_deregister_task_from_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_task_from_maintenance_window_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        if input.window_task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowTaskId'");
        }
        let window_id = input.window_id.as_str();
        let window_task_id = input.window_task_id.as_str();

        let mut state = state.write().await;
        match state.deregister_task_from_maintenance_window(window_id, window_task_id) {
            Ok(()) => wire::serialize_deregister_task_from_maintenance_window_response(
                &wire::DeregisterTaskFromMaintenanceWindowResult {
                    window_id: Some(window_id.to_string()),
                    window_task_id: Some(window_task_id.to_string()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_maintenance_window_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_maintenance_window_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        let window_id = input.window_id.as_str();

        let state = state.read().await;
        match state.describe_maintenance_window_tasks(window_id) {
            Ok(tasks) => {
                let wire_tasks: Vec<wire::MaintenanceWindowTask> = tasks
                    .iter()
                    .map(|t| wire::MaintenanceWindowTask {
                        window_id: Some(t.window_id.clone()),
                        window_task_id: Some(t.window_task_id.clone()),
                        task_arn: Some(t.task_arn.clone()),
                        r#type: Some(t.task_type.clone()),
                        targets: Some(
                            t.targets
                                .iter()
                                .map(|tgt| wire::Target {
                                    key: Some(tgt.key.clone()),
                                    values: Some(tgt.values.clone()),
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_maintenance_window_tasks_response(
                    &wire::DescribeMaintenanceWindowTasksResult {
                        tasks: Some(wire_tasks),
                        next_token: None,
                    },
                )
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    // ── Patch Baseline operations ─────────────────────────────────────

    async fn handle_create_patch_baseline(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_patch_baseline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let operating_system = input.operating_system.as_deref().unwrap_or("WINDOWS");
        let description = input.description.as_deref();

        let mut state = state.write().await;
        let baseline_id = state.create_patch_baseline(name, operating_system, description);

        wire::serialize_create_patch_baseline_response(&wire::CreatePatchBaselineResult {
            baseline_id: Some(baseline_id),
        })
    }

    async fn handle_delete_patch_baseline(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_patch_baseline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BaselineId'");
        }
        let baseline_id = input.baseline_id.as_str();

        let mut state = state.write().await;
        match state.delete_patch_baseline(baseline_id) {
            Ok(id) => {
                wire::serialize_delete_patch_baseline_response(&wire::DeletePatchBaselineResult {
                    baseline_id: Some(id),
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_patch_baselines(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let baselines = state.describe_patch_baselines();
        let identities: Vec<wire::PatchBaselineIdentity> = baselines
            .iter()
            .map(|b| wire::PatchBaselineIdentity {
                baseline_id: Some(b.baseline_id.clone()),
                baseline_name: Some(b.name.clone()),
                operating_system: Some(b.operating_system.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_patch_baselines_response(&wire::DescribePatchBaselinesResult {
            baseline_identities: Some(identities),
            next_token: None,
        })
    }

    async fn handle_register_patch_baseline_for_patch_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_patch_baseline_for_patch_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BaselineId'");
        }
        if input.patch_group.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PatchGroup'");
        }
        let baseline_id = input.baseline_id.as_str();
        let patch_group = input.patch_group.as_str();

        let mut state = state.write().await;
        match state.register_patch_baseline_for_patch_group(baseline_id, patch_group) {
            Ok(()) => wire::serialize_register_patch_baseline_for_patch_group_response(
                &wire::RegisterPatchBaselineForPatchGroupResult {
                    baseline_id: Some(baseline_id.to_string()),
                    patch_group: Some(patch_group.to_string()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_deregister_patch_baseline_for_patch_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_patch_baseline_for_patch_group_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BaselineId'");
        }
        if input.patch_group.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PatchGroup'");
        }
        let baseline_id = input.baseline_id.as_str();
        let patch_group = input.patch_group.as_str();

        let mut state = state.write().await;
        match state.deregister_patch_baseline_for_patch_group(baseline_id, patch_group) {
            Ok(()) => wire::serialize_deregister_patch_baseline_for_patch_group_response(
                &wire::DeregisterPatchBaselineForPatchGroupResult {
                    baseline_id: Some(baseline_id.to_string()),
                    patch_group: Some(patch_group.to_string()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_get_patch_baseline_for_patch_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_patch_baseline_for_patch_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.patch_group.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PatchGroup'");
        }
        let patch_group = input.patch_group.as_str();

        let state = state.read().await;
        match state.get_patch_baseline_for_patch_group(patch_group) {
            Some(baseline) => wire::serialize_get_patch_baseline_for_patch_group_response(
                &wire::GetPatchBaselineForPatchGroupResult {
                    baseline_id: Some(baseline.baseline_id.clone()),
                    operating_system: Some(baseline.operating_system.clone()),
                    patch_group: Some(patch_group.to_string()),
                },
            ),
            None => wire::serialize_get_patch_baseline_for_patch_group_response(
                &wire::GetPatchBaselineForPatchGroupResult {
                    baseline_id: None,
                    operating_system: None,
                    patch_group: Some(patch_group.to_string()),
                },
            ),
        }
    }

    // ── Command operations ────────────────────────────────────────────

    async fn handle_send_command(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_send_command_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.document_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DocumentName'");
        }
        let document_name = input.document_name.as_str();
        let instance_ids = input.instance_ids.unwrap_or_default();
        let parameters = input.parameters.unwrap_or_default();

        let mut state = state.write().await;
        let cmd = state.send_command(document_name, instance_ids, parameters);

        wire::serialize_send_command_response(&wire::SendCommandResult {
            command: Some(to_wire_command(cmd)),
        })
    }

    async fn handle_list_commands(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let commands = state.list_commands();
        let wire_commands: Vec<wire::Command> =
            commands.iter().map(|c| to_wire_command(c)).collect();

        wire::serialize_list_commands_response(&wire::ListCommandsResult {
            commands: Some(wire_commands),
            next_token: None,
        })
    }

    async fn handle_get_command_invocation(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_command_invocation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.command_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CommandId'");
        }
        if input.instance_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceId'");
        }
        let command_id = input.command_id.as_str();
        let instance_id = input.instance_id.as_str();

        let state = state.read().await;
        match state.get_command_invocation(command_id, instance_id) {
            Ok(cmd) => {
                wire::serialize_get_command_invocation_response(&wire::GetCommandInvocationResult {
                    command_id: Some(cmd.command_id.clone()),
                    instance_id: Some(instance_id.to_string()),
                    document_name: Some(cmd.document_name.clone()),
                    status: Some(cmd.status.clone()),
                    status_details: Some(cmd.status.clone()),
                    ..Default::default()
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    // ── Stub operations ───────────────────────────────────────────────

    async fn handle_associate_ops_item_related_item(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_ops_item_related_item_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_item_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsItemId'");
        }
        let ops_item_id = input.ops_item_id.as_str();
        let association_type = if input.association_type.is_empty() {
            "RelatedItem"
        } else {
            input.association_type.as_str()
        };
        let resource_type = input.resource_type.as_str();
        let resource_uri = input.resource_uri.as_str();

        let mut state = state.write().await;
        match state.associate_ops_item_related_item(
            ops_item_id,
            association_type,
            resource_type,
            resource_uri,
        ) {
            Ok(item) => wire::serialize_associate_ops_item_related_item_response(
                &wire::AssociateOpsItemRelatedItemResponse {
                    association_id: Some(item.association_id.clone()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_cancel_command(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_command_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.command_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CommandId'");
        }
        let command_id = input.command_id.as_str();
        let instance_ids = input.instance_ids;

        let mut state = state.write().await;
        match state.cancel_command(command_id, instance_ids.as_deref()) {
            Ok(()) => wire::serialize_cancel_command_response(&wire::CancelCommandResult {}),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_cancel_maintenance_window_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_maintenance_window_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_execution_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowExecutionId'");
        }
        let window_execution_id = input.window_execution_id.as_str();
        let mut state = state.write().await;
        match state.cancel_maintenance_window_execution(window_execution_id) {
            Ok(()) => wire::serialize_cancel_maintenance_window_execution_response(
                &wire::CancelMaintenanceWindowExecutionResult {
                    window_execution_id: Some(window_execution_id.to_string()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_create_activation(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_activation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.iam_role.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IamRole'");
        }
        let iam_role = input.iam_role.as_str();
        let description = input.description.as_deref();
        let default_instance_name = input.default_instance_name.as_deref();
        let registration_limit = input.registration_limit;
        let expiration_date = input.expiration_date.map(|ts| {
            chrono::DateTime::from_timestamp(ts as i64, 0).unwrap_or_else(chrono::Utc::now)
        });

        let mut state = state.write().await;
        let activation = state.create_activation(
            iam_role,
            description,
            default_instance_name,
            registration_limit,
            expiration_date,
        );
        wire::serialize_create_activation_response(&wire::CreateActivationResult {
            activation_id: Some(activation.activation_id.clone()),
            activation_code: Some(activation.activation_code.clone()),
        })
    }

    async fn handle_create_association(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let association_name = input.association_name.as_deref();
        let document_version = input.document_version.as_deref();
        let schedule_expression = input.schedule_expression.as_deref();
        let targets = wire_targets_to_types(input.targets);
        let parameters = input.parameters.unwrap_or_default();

        let mut state = state.write().await;
        let assoc = state.create_association(
            name,
            association_name,
            targets,
            schedule_expression,
            parameters,
            document_version,
        );
        wire::serialize_create_association_response(&wire::CreateAssociationResult {
            association_description: Some(wire::AssociationDescription {
                association_id: Some(assoc.association_id.clone()),
                association_name: assoc.association_name.clone(),
                association_version: Some(assoc.association_version.clone()),
                name: Some(assoc.name.clone()),
                document_version: assoc.document_version.clone(),
                schedule_expression: assoc.schedule_expression.clone(),
                targets: if assoc.targets.is_empty() {
                    None
                } else {
                    Some(
                        assoc
                            .targets
                            .iter()
                            .map(|t| wire::Target {
                                key: Some(t.key.clone()),
                                values: Some(t.values.clone()),
                            })
                            .collect(),
                    )
                },
                // FIX(terraform-e2e): include Overview.Status so waitAssociationSuccess
                //   immediately resolves to "Success" without polling.
                overview: Some(wire::AssociationOverview {
                    status: Some("Success".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        })
    }

    async fn handle_create_association_batch(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_association_batch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let mut state = state.write().await;
        let mut successful = Vec::new();
        for entry in input.entries {
            if entry.name.is_empty() {
                continue;
            }
            let name = entry.name.as_str();
            let association_name = entry.association_name.as_deref();
            let document_version = entry.document_version.as_deref();
            let schedule_expression = entry.schedule_expression.as_deref();
            let targets = wire_targets_to_types(entry.targets);
            let parameters = entry.parameters.unwrap_or_default();

            let assoc = state.create_association(
                name,
                association_name,
                targets,
                schedule_expression,
                parameters,
                document_version,
            );
            successful.push(wire::AssociationDescription {
                association_id: Some(assoc.association_id.clone()),
                association_name: assoc.association_name.clone(),
                association_version: Some(assoc.association_version.clone()),
                name: Some(assoc.name.clone()),
                document_version: assoc.document_version.clone(),
                schedule_expression: assoc.schedule_expression.clone(),
                overview: Some(wire::AssociationOverview {
                    status: Some("Success".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            });
        }
        wire::serialize_create_association_batch_response(&wire::CreateAssociationBatchResult {
            successful: if successful.is_empty() {
                None
            } else {
                Some(successful)
            },
            failed: None,
        })
    }

    async fn handle_create_ops_item(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_ops_item_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.title.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Title'");
        }
        if input.source.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Source'");
        }
        let title = input.title.as_str();
        let source = input.source.as_str();
        let description = if input.description.is_empty() {
            None
        } else {
            Some(input.description.as_str())
        };
        let priority = input.priority;
        let category = input.category.as_deref();
        let severity = input.severity.as_deref();
        let ops_item_type = input.ops_item_type.as_deref();

        let mut state = state.write().await;
        let item = state.create_ops_item(
            title,
            source,
            description,
            priority,
            category,
            severity,
            ops_item_type,
        );
        wire::serialize_create_ops_item_response(&wire::CreateOpsItemResponse {
            ops_item_id: Some(item.ops_item_id.clone()),
            ..Default::default()
        })
    }

    async fn handle_create_ops_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_ops_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        let resource_id = input.resource_id.as_str();
        let metadata: HashMap<String, String> = input
            .metadata
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(k, v)| v.value.map(|s| (k, s)))
            .collect();

        let mut state = state.write().await;
        let entry = state.create_ops_metadata(resource_id, metadata, account_id, region);
        wire::serialize_create_ops_metadata_response(&wire::CreateOpsMetadataResult {
            ops_metadata_arn: Some(entry.ops_metadata_arn.clone()),
        })
    }

    async fn handle_create_resource_data_sync(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_resource_data_sync_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.sync_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SyncName'");
        }
        let sync_name = input.sync_name.as_str();
        let sync_type = input.sync_type.as_deref().unwrap_or("SyncToDestination");
        let bucket = input
            .s3_destination
            .as_ref()
            .map(|d| d.bucket_name.as_str())
            .unwrap_or("");
        let region = input
            .s3_destination
            .as_ref()
            .map(|d| d.region.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("us-east-1");
        let prefix = input
            .s3_destination
            .as_ref()
            .and_then(|d| d.prefix.as_deref());

        let mut state = state.write().await;
        match state.create_resource_data_sync(sync_name, sync_type, bucket, region, prefix) {
            Ok(()) => wire::serialize_create_resource_data_sync_response(
                &wire::CreateResourceDataSyncResult {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_activation(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_activation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.activation_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ActivationId'");
        }
        let activation_id = input.activation_id.as_str();
        let mut state = state.write().await;
        match state.delete_activation(activation_id) {
            Ok(()) => wire::serialize_delete_activation_response(&wire::DeleteActivationResult {}),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_association(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let association_id = input.association_id.unwrap_or_default();
        if association_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AssociationId'");
        }

        let mut state = state.write().await;
        match state.delete_association(&association_id) {
            Ok(()) => {
                wire::serialize_delete_association_response(&wire::DeleteAssociationResult {})
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_inventory(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_inventory_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.type_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TypeName'");
        }
        let type_name = input.type_name.as_str();
        let mut state = state.write().await;
        let deletion = state.delete_inventory(type_name);
        wire::serialize_delete_inventory_response(&wire::DeleteInventoryResult {
            deletion_id: Some(deletion.deletion_id.clone()),
            ..Default::default()
        })
    }

    async fn handle_delete_ops_item(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_ops_item_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_item_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsItemId'");
        }
        let ops_item_id = input.ops_item_id.as_str();
        let mut state = state.write().await;
        match state.delete_ops_item(ops_item_id) {
            Ok(()) => wire::serialize_delete_ops_item_response(&wire::DeleteOpsItemResponse {}),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_ops_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_ops_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_metadata_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsMetadataArn'");
        }
        let ops_metadata_arn = input.ops_metadata_arn.as_str();
        let mut state = state.write().await;
        match state.delete_ops_metadata(ops_metadata_arn) {
            Ok(()) => {
                wire::serialize_delete_ops_metadata_response(&wire::DeleteOpsMetadataResult {})
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_resource_data_sync(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_data_sync_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.sync_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SyncName'");
        }
        let sync_name = input.sync_name.as_str();
        let mut state = state.write().await;
        match state.delete_resource_data_sync(sync_name) {
            Ok(()) => wire::serialize_delete_resource_data_sync_response(
                &wire::DeleteResourceDataSyncResult {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        if input.policy_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PolicyId'");
        }
        let resource_arn = input.resource_arn.as_str();
        let policy_id = input.policy_id.as_str();
        let mut state = state.write().await;
        match state.delete_resource_policy(resource_arn, policy_id) {
            Ok(()) => wire::serialize_delete_resource_policy_response(
                &wire::DeleteResourcePolicyResponse {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_deregister_managed_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_managed_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.instance_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceId'");
        }
        let instance_id = input.instance_id.as_str();
        let mut state = state.write().await;
        match state.deregister_managed_instance(instance_id) {
            Ok(()) => wire::serialize_deregister_managed_instance_response(
                &wire::DeregisterManagedInstanceResult {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_activations(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let activations = state.describe_activations();
        let wire_activations: Vec<wire::Activation> = activations
            .iter()
            .map(|a| wire::Activation {
                activation_id: Some(a.activation_id.clone()),
                description: a.description.clone(),
                iam_role: Some(a.iam_role.clone()),
                registration_limit: a.registration_limit,
                registrations_count: Some(a.registrations_count),
                expiration_date: a.expiration_date.map(|dt| dt.timestamp() as f64),
                expired: Some(a.expired),
                created_date: Some(a.created_date.timestamp() as f64),
                default_instance_name: a.default_instance_name.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_activations_response(&wire::DescribeActivationsResult {
            activation_list: if wire_activations.is_empty() {
                None
            } else {
                Some(wire_activations)
            },
            next_token: None,
        })
    }

    async fn handle_describe_association(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let association_id = input.association_id.unwrap_or_default();
        if association_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AssociationId'");
        }

        let state = state.read().await;
        match state.get_association(&association_id) {
            Ok(assoc) => {
                // FIX(terraform-e2e): provider's findAssociationByID filters associations by
                //   Overview != nil; without Overview.Status the filter rejects the result and
                //   returns "empty result", causing resourceAssociationRead to error.
                wire::serialize_describe_association_response(&wire::DescribeAssociationResult {
                    association_description: Some(wire::AssociationDescription {
                        association_id: Some(assoc.association_id.clone()),
                        association_name: assoc.association_name.clone(),
                        association_version: Some(assoc.association_version.clone()),
                        name: Some(assoc.name.clone()),
                        document_version: assoc.document_version.clone(),
                        schedule_expression: assoc.schedule_expression.clone(),
                        targets: if assoc.targets.is_empty() {
                            None
                        } else {
                            Some(
                                assoc
                                    .targets
                                    .iter()
                                    .map(|t| wire::Target {
                                        key: Some(t.key.clone()),
                                        values: Some(t.values.clone()),
                                    })
                                    .collect(),
                            )
                        },
                        overview: Some(wire::AssociationOverview {
                            status: Some("Success".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_association_execution_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_association_execution_targets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.association_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AssociationId'");
        }
        let association_id = input.association_id.as_str();
        let state = state.read().await;
        match state.get_association(association_id) {
            Ok(assoc) => {
                // Return targets from the association as execution targets
                let targets: Vec<wire::AssociationExecutionTarget> = assoc
                    .targets
                    .iter()
                    .flat_map(|t| {
                        t.values
                            .iter()
                            .map(move |v| wire::AssociationExecutionTarget {
                                association_id: Some(assoc.association_id.clone()),
                                association_version: Some(assoc.association_version.clone()),
                                resource_id: Some(v.clone()),
                                resource_type: Some(t.key.clone()),
                                status: Some("Success".to_string()),
                                ..Default::default()
                            })
                    })
                    .collect();
                wire::serialize_describe_association_execution_targets_response(
                    &wire::DescribeAssociationExecutionTargetsResult {
                        association_execution_targets: if targets.is_empty() {
                            None
                        } else {
                            Some(targets)
                        },
                        next_token: None,
                    },
                )
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_describe_association_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_association_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.association_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AssociationId'");
        }
        let association_id = input.association_id.as_str();
        let state = state.read().await;
        match state.get_association(association_id) {
            Ok(assoc) => {
                let executions = vec![wire::AssociationExecution {
                    association_id: Some(assoc.association_id.clone()),
                    association_version: Some(assoc.association_version.clone()),
                    execution_id: Some(assoc.association_id.clone()),
                    status: Some(assoc.overview.status.clone()),
                    detailed_status: Some(assoc.overview.detailed_status.clone()),
                    created_time: Some(assoc.created_date.timestamp() as f64),
                    ..Default::default()
                }];
                wire::serialize_describe_association_executions_response(
                    &wire::DescribeAssociationExecutionsResult {
                        association_executions: Some(executions),
                        next_token: None,
                    },
                )
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    // STUB[no-engine]: Automation execution requires a real execution engine.
    async fn handle_describe_automation_executions(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_describe_automation_executions_response(
            &wire::DescribeAutomationExecutionsResult::default(),
        )
    }

    // STUB[no-engine]: Automation execution requires a real execution engine.
    async fn handle_describe_automation_step_executions(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_describe_automation_step_executions_response(
            &wire::DescribeAutomationStepExecutionsResult::default(),
        )
    }

    async fn handle_describe_available_patches(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        // Available patches are external AWS data; return empty list from mock.
        wire::serialize_describe_available_patches_response(
            &wire::DescribeAvailablePatchesResult::default(),
        )
    }

    // STUB[no-telemetry]: Effective instance association status depends on real infrastructure telemetry.
    async fn handle_describe_effective_instance_associations(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_describe_effective_instance_associations_response(
            &wire::DescribeEffectiveInstanceAssociationsResult::default(),
        )
    }

    async fn handle_describe_effective_patches_for_patch_baseline(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_effective_patches_for_patch_baseline_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.baseline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BaselineId'");
        }
        let baseline_id = input.baseline_id.as_str();
        let state = state.read().await;
        if !state.patch_baselines.contains_key(baseline_id) {
            return json_error_response(
                400,
                "DoesNotExistException",
                &format!("Patch baseline {baseline_id} does not exist."),
            );
        }
        wire::serialize_describe_effective_patches_for_patch_baseline_response(
            &wire::DescribeEffectivePatchesForPatchBaselineResult::default(),
        )
    }

    // STUB[no-telemetry]: Instance association status depends on real infrastructure telemetry.
    async fn handle_describe_instance_associations_status(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_describe_instance_associations_status_response(
            &wire::DescribeInstanceAssociationsStatusResult::default(),
        )
    }

    async fn handle_describe_instance_information(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let instances = state.describe_instance_information();
        let wire_instances: Vec<wire::InstanceInformation> = instances
            .iter()
            .map(|mi| wire::InstanceInformation {
                instance_id: Some(mi.instance_id.clone()),
                ping_status: Some(mi.ping_status.clone()),
                platform_type: Some(mi.platform_type.clone()),
                platform_name: Some(mi.platform_name.clone()),
                platform_version: Some(mi.platform_version.clone()),
                activation_id: mi.activation_id.clone(),
                iam_role: mi.iam_role.clone(),
                registration_date: Some(mi.registration_date.timestamp() as f64),
                resource_type: Some(mi.resource_type.clone()),
                i_p_address: Some(mi.ip_address.clone()),
                computer_name: Some(mi.computer_name.clone()),
                is_latest_version: Some(mi.is_latest_version),
                last_ping_date_time: Some(mi.last_ping_date_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_instance_information_response(
            &wire::DescribeInstanceInformationResult {
                instance_information_list: if wire_instances.is_empty() {
                    None
                } else {
                    Some(wire_instances)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_instance_patch_states(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        // Patch state data requires real agent telemetry; return empty list from mock.
        wire::serialize_describe_instance_patch_states_response(
            &wire::DescribeInstancePatchStatesResult::default(),
        )
    }

    async fn handle_describe_instance_patch_states_for_patch_group(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        // Patch state data requires real agent telemetry; return empty list from mock.
        wire::serialize_describe_instance_patch_states_for_patch_group_response(
            &wire::DescribeInstancePatchStatesForPatchGroupResult::default(),
        )
    }

    async fn handle_describe_instance_patches(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        // Patch data requires real agent telemetry; return empty list from mock.
        wire::serialize_describe_instance_patches_response(
            &wire::DescribeInstancePatchesResult::default(),
        )
    }

    async fn handle_describe_instance_properties(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let instances = state.describe_instance_information();
        let wire_props: Vec<wire::InstanceProperty> = instances
            .iter()
            .map(|mi| wire::InstanceProperty {
                instance_id: Some(mi.instance_id.clone()),
                ping_status: Some(mi.ping_status.clone()),
                platform_type: Some(mi.platform_type.clone()),
                platform_name: Some(mi.platform_name.clone()),
                platform_version: Some(mi.platform_version.clone()),
                activation_id: mi.activation_id.clone(),
                i_p_address: Some(mi.ip_address.clone()),
                computer_name: Some(mi.computer_name.clone()),
                resource_type: Some(mi.resource_type.clone()),
                registration_date: Some(mi.registration_date.timestamp() as f64),
                last_ping_date_time: Some(mi.last_ping_date_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_instance_properties_response(
            &wire::DescribeInstancePropertiesResult {
                instance_properties: if wire_props.is_empty() {
                    None
                } else {
                    Some(wire_props)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_inventory_deletions(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let deletions = state.describe_inventory_deletions();
        let wire_deletions: Vec<wire::InventoryDeletionStatusItem> = deletions
            .iter()
            .map(|d| wire::InventoryDeletionStatusItem {
                deletion_id: Some(d.deletion_id.clone()),
                type_name: Some(d.type_name.clone()),
                deletion_start_time: Some(d.deletion_start_time.timestamp() as f64),
                last_status: Some(d.last_status.clone()),
                last_status_message: Some(d.last_status_message.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_inventory_deletions_response(
            &wire::DescribeInventoryDeletionsResult {
                inventory_deletions: if wire_deletions.is_empty() {
                    None
                } else {
                    Some(wire_deletions)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_maintenance_window_execution_task_invocations(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_maintenance_window_execution_task_invocations_request(
                body,
            ) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.window_execution_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowExecutionId'");
        }
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TaskId'");
        }
        let window_execution_id = input.window_execution_id.as_str();
        let task_id = input.task_id.as_str();
        let state = state.read().await;
        let invocations = state
            .describe_maintenance_window_execution_task_invocations(window_execution_id, task_id);
        let wire_invocations: Vec<wire::MaintenanceWindowExecutionTaskInvocationIdentity> =
            invocations
                .iter()
                .map(
                    |inv| wire::MaintenanceWindowExecutionTaskInvocationIdentity {
                        window_execution_id: Some(inv.window_execution_id.clone()),
                        task_execution_id: Some(inv.task_execution_id.clone()),
                        invocation_id: Some(inv.invocation_id.clone()),
                        execution_id: Some(inv.execution_id.clone()),
                        status: Some(inv.status.clone()),
                        start_time: Some(inv.start_time.timestamp() as f64),
                        end_time: inv.end_time.map(|dt| dt.timestamp() as f64),
                        ..Default::default()
                    },
                )
                .collect();
        wire::serialize_describe_maintenance_window_execution_task_invocations_response(
            &wire::DescribeMaintenanceWindowExecutionTaskInvocationsResult {
                window_execution_task_invocation_identities: if wire_invocations.is_empty() {
                    None
                } else {
                    Some(wire_invocations)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_maintenance_window_execution_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_maintenance_window_execution_tasks_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.window_execution_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowExecutionId'");
        }
        let window_execution_id = input.window_execution_id.as_str();
        let state = state.read().await;
        let tasks = state.describe_maintenance_window_execution_tasks(window_execution_id);
        let wire_tasks: Vec<wire::MaintenanceWindowExecutionTaskIdentity> = tasks
            .iter()
            .map(|t| wire::MaintenanceWindowExecutionTaskIdentity {
                window_execution_id: Some(t.window_execution_id.clone()),
                task_execution_id: Some(t.task_execution_id.clone()),
                task_arn: Some(t.task_arn.clone()),
                task_type: Some(t.task_type.clone()),
                status: Some(t.status.clone()),
                start_time: Some(t.start_time.timestamp() as f64),
                end_time: t.end_time.map(|dt| dt.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_maintenance_window_execution_tasks_response(
            &wire::DescribeMaintenanceWindowExecutionTasksResult {
                window_execution_task_identities: if wire_tasks.is_empty() {
                    None
                } else {
                    Some(wire_tasks)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_maintenance_window_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_maintenance_window_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        let window_id = input.window_id.as_str();
        let state = state.read().await;
        let executions = state.describe_maintenance_window_executions(window_id);
        let wire_execs: Vec<wire::MaintenanceWindowExecution> = executions
            .iter()
            .map(|e| wire::MaintenanceWindowExecution {
                window_execution_id: Some(e.window_execution_id.clone()),
                window_id: Some(e.window_id.clone()),
                status: Some(e.status.clone()),
                start_time: Some(e.start_time.timestamp() as f64),
                end_time: e.end_time.map(|dt| dt.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_maintenance_window_executions_response(
            &wire::DescribeMaintenanceWindowExecutionsResult {
                window_executions: if wire_execs.is_empty() {
                    None
                } else {
                    Some(wire_execs)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_maintenance_window_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_maintenance_window_schedule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let window_id = input.window_id.as_deref();
        let state = state.read().await;
        let schedule_entries = state.describe_maintenance_window_schedule(window_id);
        let wire_entries: Vec<wire::ScheduledWindowExecution> = schedule_entries
            .iter()
            .map(|(w, _)| wire::ScheduledWindowExecution {
                window_id: Some(w.window_id.clone()),
                name: Some(w.name.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_maintenance_window_schedule_response(
            &wire::DescribeMaintenanceWindowScheduleResult {
                scheduled_window_executions: if wire_entries.is_empty() {
                    None
                } else {
                    Some(wire_entries)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_maintenance_windows_for_target(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_maintenance_windows_for_target_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let targets = wire_targets_to_types(Some(input.targets));
        let state = state.read().await;
        let windows = state.describe_maintenance_windows_for_target(&targets);
        let wire_windows: Vec<wire::MaintenanceWindowIdentityForTarget> = windows
            .iter()
            .map(|w| wire::MaintenanceWindowIdentityForTarget {
                window_id: Some(w.window_id.clone()),
                name: Some(w.name.clone()),
            })
            .collect();
        wire::serialize_describe_maintenance_windows_for_target_response(
            &wire::DescribeMaintenanceWindowsForTargetResult {
                window_identities: if wire_windows.is_empty() {
                    None
                } else {
                    Some(wire_windows)
                },
                next_token: None,
            },
        )
    }

    async fn handle_describe_ops_items(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let items = state.describe_ops_items();
        let wire_items: Vec<wire::OpsItemSummary> = items
            .iter()
            .map(|o| wire::OpsItemSummary {
                ops_item_id: Some(o.ops_item_id.clone()),
                title: Some(o.title.clone()),
                source: Some(o.source.clone()),
                status: Some(o.status.clone()),
                priority: o.priority,
                category: o.category.clone(),
                severity: o.severity.clone(),
                ops_item_type: o.ops_item_type.clone(),
                created_time: Some(o.created_time.timestamp() as f64),
                last_modified_time: Some(o.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_ops_items_response(&wire::DescribeOpsItemsResponse {
            ops_item_summaries: if wire_items.is_empty() {
                None
            } else {
                Some(wire_items)
            },
            next_token: None,
        })
    }

    async fn handle_describe_patch_group_state(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_patch_group_state_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.patch_group.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PatchGroup'");
        }
        let patch_group = input.patch_group.as_str();
        let state = state.read().await;
        let instance_count = state.managed_instances.values().count() as i32;
        let _ = patch_group; // We just return instance count from managed instances
        wire::serialize_describe_patch_group_state_response(&wire::DescribePatchGroupStateResult {
            instances: Some(instance_count),
            ..Default::default()
        })
    }

    async fn handle_describe_patch_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let mut mappings: Vec<wire::PatchGroupPatchBaselineMapping> = Vec::new();
        for baseline in state.describe_patch_baselines() {
            for pg in &baseline.patch_groups {
                mappings.push(wire::PatchGroupPatchBaselineMapping {
                    patch_group: Some(pg.clone()),
                    baseline_identity: Some(wire::PatchBaselineIdentity {
                        baseline_id: Some(baseline.baseline_id.clone()),
                        baseline_name: Some(baseline.name.clone()),
                        baseline_description: baseline.description.clone(),
                        operating_system: Some(baseline.operating_system.clone()),
                        default_baseline: Some(
                            state
                                .default_patch_baselines
                                .get(&baseline.operating_system)
                                .map(|id| id == &baseline.baseline_id)
                                .unwrap_or(false),
                        ),
                    }),
                });
            }
        }
        wire::serialize_describe_patch_groups_response(&wire::DescribePatchGroupsResult {
            mappings: if mappings.is_empty() {
                None
            } else {
                Some(mappings)
            },
            next_token: None,
        })
    }

    async fn handle_describe_patch_properties(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        // Patch properties are external AWS catalogue data; return empty list from mock.
        wire::serialize_describe_patch_properties_response(
            &wire::DescribePatchPropertiesResult::default(),
        )
    }

    async fn handle_describe_sessions(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_sessions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let state_filter = if input.state.is_empty() {
            None
        } else {
            Some(input.state.as_str())
        };
        let state = state.read().await;
        let sessions = state.describe_sessions(state_filter);
        let wire_sessions: Vec<wire::Session> = sessions
            .iter()
            .map(|s| wire::Session {
                session_id: Some(s.session_id.clone()),
                target: Some(s.target.clone()),
                status: Some(s.status.clone()),
                start_date: Some(s.start_date.timestamp() as f64),
                end_date: s.end_date.map(|dt| dt.timestamp() as f64),
                document_name: s.document_name.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_sessions_response(&wire::DescribeSessionsResponse {
            sessions: if wire_sessions.is_empty() {
                None
            } else {
                Some(wire_sessions)
            },
            next_token: None,
        })
    }

    async fn handle_disassociate_ops_item_related_item(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_ops_item_related_item_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_item_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsItemId'");
        }
        if input.association_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AssociationId'");
        }
        let ops_item_id = input.ops_item_id.as_str();
        let association_id = input.association_id.as_str();
        let mut state = state.write().await;
        match state.disassociate_ops_item_related_item(ops_item_id, association_id) {
            Ok(()) => wire::serialize_disassociate_ops_item_related_item_response(
                &wire::DisassociateOpsItemRelatedItemResponse {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    // STUB[org-integration]: Access token issuance requires real AWS Organizations or IAM Identity Center integration.
    async fn handle_get_access_token(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_get_access_token_response(&wire::GetAccessTokenResponse::default())
    }

    // STUB[no-engine]: Automation execution requires a real execution engine.
    async fn handle_get_automation_execution(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_get_automation_execution_response(
            &wire::GetAutomationExecutionResult::default(),
        )
    }

    // STUB[no-engine]: Change Calendar state evaluation requires a real execution engine.
    async fn handle_get_calendar_state(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_get_calendar_state_response(&wire::GetCalendarStateResponse::default())
    }

    async fn handle_get_connection_status(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_connection_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.target.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Target'");
        }
        let target = input.target.as_str();
        let state = state.read().await;
        let status = state.get_connection_status(target);
        wire::serialize_get_connection_status_response(&wire::GetConnectionStatusResponse {
            target: Some(target.to_string()),
            status: Some(status),
        })
    }

    async fn handle_get_default_patch_baseline(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_default_patch_baseline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let operating_system = input.operating_system.as_deref().unwrap_or("WINDOWS");
        let state = state.read().await;
        let result = if let Some(baseline) = state.get_default_patch_baseline(operating_system) {
            wire::GetDefaultPatchBaselineResult {
                baseline_id: Some(baseline.baseline_id.clone()),
                operating_system: Some(baseline.operating_system.clone()),
            }
        } else {
            wire::GetDefaultPatchBaselineResult {
                operating_system: Some(operating_system.to_string()),
                ..Default::default()
            }
        };
        wire::serialize_get_default_patch_baseline_response(&result)
    }

    async fn handle_get_deployable_patch_snapshot_for_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployable_patch_snapshot_for_instance_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.instance_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceId'");
        }
        let instance_id = input.instance_id.as_str();
        let snapshot_id = input.snapshot_id.as_str();
        let _ = state; // State is read but snapshot data is synthetic
        wire::serialize_get_deployable_patch_snapshot_for_instance_response(
            &wire::GetDeployablePatchSnapshotForInstanceResult {
                instance_id: Some(instance_id.to_string()),
                snapshot_id: Some(snapshot_id.to_string()),
                ..Default::default()
            },
        )
    }

    // STUB[no-engine]: Automation execution preview requires a real execution engine.
    async fn handle_get_execution_preview(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_get_execution_preview_response(&wire::GetExecutionPreviewResponse::default())
    }

    async fn handle_get_inventory(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        // Group inventory by instance_id
        let mut by_instance: HashMap<String, Vec<&crate::types::InventoryData>> = HashMap::new();
        for ((instance_id, _), data) in &state.inventory {
            by_instance
                .entry(instance_id.clone())
                .or_default()
                .push(data);
        }
        let entities: Vec<wire::InventoryResultEntity> = by_instance
            .into_iter()
            .map(|(instance_id, items)| {
                let data: HashMap<String, wire::InventoryResultItem> = items
                    .iter()
                    .map(|item| {
                        (
                            item.type_name.clone(),
                            wire::InventoryResultItem {
                                type_name: Some(item.type_name.clone()),
                                capture_time: Some(item.capture_time.clone()),
                                schema_version: Some(item.schema_version.clone()),
                                content: if item.content.is_empty() {
                                    None
                                } else {
                                    Some(item.content.clone())
                                },
                                ..Default::default()
                            },
                        )
                    })
                    .collect();
                wire::InventoryResultEntity {
                    id: Some(instance_id),
                    data: if data.is_empty() { None } else { Some(data) },
                }
            })
            .collect();
        wire::serialize_get_inventory_response(&wire::GetInventoryResult {
            entities: if entities.is_empty() {
                None
            } else {
                Some(entities)
            },
            next_token: None,
        })
    }

    async fn handle_get_inventory_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let type_names = state.get_inventory_schema();
        let schemas: Vec<wire::InventoryItemSchema> = type_names
            .iter()
            .map(|tn| wire::InventoryItemSchema {
                type_name: Some(tn.clone()),
                version: Some("1.0".to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_get_inventory_schema_response(&wire::GetInventorySchemaResult {
            schemas: if schemas.is_empty() {
                None
            } else {
                Some(schemas)
            },
            next_token: None,
        })
    }

    async fn handle_get_maintenance_window_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_maintenance_window_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_execution_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowExecutionId'");
        }
        let window_execution_id = input.window_execution_id.as_str();
        let state = state.read().await;
        match state.get_maintenance_window_execution(window_execution_id) {
            Some(exec) => wire::serialize_get_maintenance_window_execution_response(
                &wire::GetMaintenanceWindowExecutionResult {
                    window_execution_id: Some(exec.window_execution_id.clone()),
                    status: Some(exec.status.clone()),
                    start_time: Some(exec.start_time.timestamp() as f64),
                    end_time: exec.end_time.map(|dt| dt.timestamp() as f64),
                    ..Default::default()
                },
            ),
            None => wire::serialize_get_maintenance_window_execution_response(
                &wire::GetMaintenanceWindowExecutionResult {
                    window_execution_id: Some(window_execution_id.to_string()),
                    ..Default::default()
                },
            ),
        }
    }

    async fn handle_get_maintenance_window_execution_task(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_maintenance_window_execution_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_execution_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowExecutionId'");
        }
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TaskId'");
        }
        let window_execution_id = input.window_execution_id.as_str();
        let task_id = input.task_id.as_str();
        let state = state.read().await;
        match state.get_maintenance_window_execution_task(window_execution_id, task_id) {
            Some(task) => wire::serialize_get_maintenance_window_execution_task_response(
                &wire::GetMaintenanceWindowExecutionTaskResult {
                    task_execution_id: Some(task.task_execution_id.clone()),
                    task_arn: Some(task.task_arn.clone()),
                    status: Some(task.status.clone()),
                    start_time: Some(task.start_time.timestamp() as f64),
                    end_time: task.end_time.map(|dt| dt.timestamp() as f64),
                    ..Default::default()
                },
            ),
            None => wire::serialize_get_maintenance_window_execution_task_response(
                &wire::GetMaintenanceWindowExecutionTaskResult::default(),
            ),
        }
    }

    async fn handle_get_maintenance_window_execution_task_invocation(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_maintenance_window_execution_task_invocation_request(
            body,
        ) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_execution_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowExecutionId'");
        }
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TaskId'");
        }
        if input.invocation_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InvocationId'");
        }
        let window_execution_id = input.window_execution_id.as_str();
        let task_id = input.task_id.as_str();
        let invocation_id = input.invocation_id.as_str();
        let state = state.read().await;
        match state.get_maintenance_window_execution_task_invocation(
            window_execution_id,
            task_id,
            invocation_id,
        ) {
            Some(inv) => wire::serialize_get_maintenance_window_execution_task_invocation_response(
                &wire::GetMaintenanceWindowExecutionTaskInvocationResult {
                    window_execution_id: Some(inv.window_execution_id.clone()),
                    task_execution_id: Some(inv.task_execution_id.clone()),
                    invocation_id: Some(inv.invocation_id.clone()),
                    execution_id: Some(inv.execution_id.clone()),
                    status: Some(inv.status.clone()),
                    start_time: Some(inv.start_time.timestamp() as f64),
                    end_time: inv.end_time.map(|dt| dt.timestamp() as f64),
                    ..Default::default()
                },
            ),
            None => wire::serialize_get_maintenance_window_execution_task_invocation_response(
                &wire::GetMaintenanceWindowExecutionTaskInvocationResult::default(),
            ),
        }
    }

    async fn handle_get_maintenance_window_task(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_maintenance_window_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        if input.window_task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowTaskId'");
        }
        let window_id = input.window_id.as_str();
        let window_task_id = input.window_task_id.as_str();

        let state = state.read().await;
        match state.get_maintenance_window_task(window_id, window_task_id) {
            Ok(task) => wire::serialize_get_maintenance_window_task_response(
                &wire::GetMaintenanceWindowTaskResult {
                    window_id: Some(task.window_id.clone()),
                    window_task_id: Some(task.window_task_id.clone()),
                    task_arn: Some(task.task_arn.clone()),
                    task_type: Some(task.task_type.clone()),
                    targets: Some(
                        task.targets
                            .iter()
                            .map(|tgt| wire::Target {
                                key: Some(tgt.key.clone()),
                                values: Some(tgt.values.clone()),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_get_ops_item(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_ops_item_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_item_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsItemId'");
        }
        let ops_item_id = input.ops_item_id.as_str();
        let state = state.read().await;
        match state.get_ops_item(ops_item_id) {
            Ok(item) => wire::serialize_get_ops_item_response(&wire::GetOpsItemResponse {
                ops_item: Some(wire::OpsItem {
                    ops_item_id: Some(item.ops_item_id.clone()),
                    title: Some(item.title.clone()),
                    description: item.description.clone(),
                    source: Some(item.source.clone()),
                    status: Some(item.status.clone()),
                    priority: item.priority,
                    category: item.category.clone(),
                    severity: item.severity.clone(),
                    ops_item_type: item.ops_item_type.clone(),
                    created_time: Some(item.created_time.timestamp() as f64),
                    last_modified_time: Some(item.last_modified_time.timestamp() as f64),
                    ..Default::default()
                }),
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_get_ops_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_ops_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_metadata_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsMetadataArn'");
        }
        let ops_metadata_arn = input.ops_metadata_arn.as_str();
        let state = state.read().await;
        match state.get_ops_metadata(ops_metadata_arn) {
            Some(entry) => {
                let metadata: HashMap<String, wire::MetadataValue> = entry
                    .metadata
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            wire::MetadataValue {
                                value: Some(v.clone()),
                            },
                        )
                    })
                    .collect();
                wire::serialize_get_ops_metadata_response(&wire::GetOpsMetadataResult {
                    resource_id: Some(entry.resource_id.clone()),
                    metadata: if metadata.is_empty() {
                        None
                    } else {
                        Some(metadata)
                    },
                    next_token: None,
                })
            }
            None => json_error_response(
                400,
                "OpsMetadataNotFoundException",
                &format!("OpsMetadata {ops_metadata_arn} not found."),
            ),
        }
    }

    async fn handle_get_ops_summary(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let items = state.get_ops_summary();
        let entities: Vec<wire::OpsEntity> = items
            .iter()
            .map(|item| {
                let mut data: HashMap<String, wire::OpsEntityItem> = HashMap::new();
                data.insert(
                    "AWS:OpsItem".to_string(),
                    wire::OpsEntityItem {
                        capture_time: Some(
                            item.created_time.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                        ..Default::default()
                    },
                );
                wire::OpsEntity {
                    id: Some(item.ops_item_id.clone()),
                    data: Some(data),
                }
            })
            .collect();
        wire::serialize_get_ops_summary_response(&wire::GetOpsSummaryResult {
            entities: if entities.is_empty() {
                None
            } else {
                Some(entities)
            },
            next_token: None,
        })
    }

    async fn handle_get_patch_baseline(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_patch_baseline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BaselineId'");
        }
        let baseline_id = input.baseline_id.as_str();
        let state = state.read().await;
        match state.patch_baselines.get(baseline_id) {
            Some(baseline) => {
                // FIX(terraform-e2e): provider crashes in flattenPatchRuleGroup with a nil
                //   pointer dereference when ApprovalRules is absent from the response.
                //   Always return an empty PatchRuleGroup and PatchFilterGroup so the provider
                //   can read the baseline back after create without panicking.
                wire::serialize_get_patch_baseline_response(&wire::GetPatchBaselineResult {
                    baseline_id: Some(baseline.baseline_id.clone()),
                    name: Some(baseline.name.clone()),
                    operating_system: Some(baseline.operating_system.clone()),
                    description: baseline.description.clone(),
                    approval_rules: Some(wire::PatchRuleGroup::default()),
                    global_filters: Some(wire::PatchFilterGroup::default()),
                    ..Default::default()
                })
            }
            None => json_error_response(
                400,
                "DoesNotExistException",
                &format!("Patch baseline {baseline_id} does not exist."),
            ),
        }
    }

    async fn handle_get_resource_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();
        let state = state.read().await;
        let policies = state.get_resource_policies(resource_arn);
        let wire_policies: Vec<wire::GetResourcePoliciesResponseEntry> = policies
            .iter()
            .map(|p| wire::GetResourcePoliciesResponseEntry {
                policy_id: Some(p.policy_id.clone()),
                policy_hash: Some(p.policy_hash.clone()),
                policy: Some(p.policy.clone()),
            })
            .collect();
        wire::serialize_get_resource_policies_response(&wire::GetResourcePoliciesResponse {
            policies: if wire_policies.is_empty() {
                None
            } else {
                Some(wire_policies)
            },
            next_token: None,
        })
    }

    async fn handle_get_service_setting(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_service_setting_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.setting_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SettingId'");
        }
        let setting_id = input.setting_id.as_str();
        let state = state.read().await;
        let setting = state.get_service_setting(setting_id);
        let wire_setting = setting.map(|s| wire::ServiceSetting {
            setting_id: Some(s.setting_id.clone()),
            setting_value: Some(s.setting_value.clone()),
            last_modified_date: Some(s.last_modified_time.timestamp() as f64),
            a_r_n: Some(s.arn.clone()),
            ..Default::default()
        });
        wire::serialize_get_service_setting_response(&wire::GetServiceSettingResult {
            service_setting: wire_setting,
        })
    }

    async fn handle_list_association_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_association_versions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.association_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AssociationId'");
        }
        let association_id = input.association_id.as_str();
        let state = state.read().await;
        match state.list_association_versions(association_id) {
            Ok(versions) => {
                let wire_versions: Vec<wire::AssociationVersionInfo> = versions
                    .iter()
                    .map(|a| wire::AssociationVersionInfo {
                        association_id: Some(a.association_id.clone()),
                        association_name: a.association_name.clone(),
                        association_version: Some(a.association_version.clone()),
                        name: Some(a.name.clone()),
                        document_version: a.document_version.clone(),
                        schedule_expression: a.schedule_expression.clone(),
                        created_date: Some(a.created_date.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_association_versions_response(
                    &wire::ListAssociationVersionsResult {
                        association_versions: if wire_versions.is_empty() {
                            None
                        } else {
                            Some(wire_versions)
                        },
                        next_token: None,
                    },
                )
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_list_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let assocs = state.list_associations();
        let wire_assocs: Vec<wire::Association> = assocs
            .iter()
            .map(|a| wire::Association {
                association_id: Some(a.association_id.clone()),
                association_name: a.association_name.clone(),
                association_version: Some(a.association_version.clone()),
                name: Some(a.name.clone()),
                document_version: a.document_version.clone(),
                schedule_expression: a.schedule_expression.clone(),
                targets: if a.targets.is_empty() {
                    None
                } else {
                    Some(
                        a.targets
                            .iter()
                            .map(|t| wire::Target {
                                key: Some(t.key.clone()),
                                values: Some(t.values.clone()),
                            })
                            .collect(),
                    )
                },
                ..Default::default()
            })
            .collect();
        wire::serialize_list_associations_response(&wire::ListAssociationsResult {
            associations: Some(wire_assocs),
            next_token: None,
        })
    }

    async fn handle_list_command_invocations(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_command_invocations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let filter_command_id = input.command_id.as_deref();
        let filter_instance_id = input.instance_id.as_deref();

        let state = state.read().await;
        let mut invocations: Vec<wire::CommandInvocation> = Vec::new();
        for cmd in state.list_commands() {
            if let Some(cid) = filter_command_id {
                if cmd.command_id != cid {
                    continue;
                }
            }
            for instance_id in &cmd.instance_ids {
                if let Some(iid) = filter_instance_id {
                    if instance_id != iid {
                        continue;
                    }
                }
                invocations.push(wire::CommandInvocation {
                    command_id: Some(cmd.command_id.clone()),
                    instance_id: Some(instance_id.clone()),
                    document_name: Some(cmd.document_name.clone()),
                    status: Some(cmd.status.clone()),
                    requested_date_time: Some(cmd.requested_date_time.timestamp() as f64),
                    ..Default::default()
                });
            }
        }
        wire::serialize_list_command_invocations_response(&wire::ListCommandInvocationsResult {
            command_invocations: if invocations.is_empty() {
                None
            } else {
                Some(invocations)
            },
            next_token: None,
        })
    }

    async fn handle_list_compliance_items(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_compliance_items_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_ids = input.resource_ids.unwrap_or_default();
        // Extract ComplianceType filter values from the typed Smithy filters list,
        // matching the legacy semantics that filtered records by `ComplianceType`.
        let compliance_types: Vec<String> = input
            .filters
            .unwrap_or_default()
            .into_iter()
            .filter(|f| f.key.as_deref() == Some("ComplianceType"))
            .filter_map(|f| f.values)
            .flatten()
            .collect();

        let state = state.read().await;
        let records = state.list_compliance_items(&resource_ids, &compliance_types);
        let wire_items: Vec<wire::ComplianceItem> = records
            .iter()
            .map(|r| wire::ComplianceItem {
                compliance_type: Some(r.compliance_type.clone()),
                resource_id: Some(r.resource_id.clone()),
                resource_type: Some(r.resource_type.clone()),
                status: Some(r.status.clone()),
                severity: Some(r.severity.clone()),
                id: Some(r.item_id.clone()),
                title: Some(r.title.clone()),
                details: if r.details.is_empty() {
                    None
                } else {
                    Some(r.details.clone())
                },
                execution_summary: Some(wire::ComplianceExecutionSummary {
                    execution_type: Some(r.execution_type.clone()),
                    execution_id: Some(r.execution_id.clone()),
                    execution_time: r.execution_time.timestamp() as f64,
                }),
            })
            .collect();
        wire::serialize_list_compliance_items_response(&wire::ListComplianceItemsResult {
            compliance_items: if wire_items.is_empty() {
                None
            } else {
                Some(wire_items)
            },
            next_token: None,
        })
    }

    async fn handle_list_compliance_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_compliance_summaries_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let compliance_types: Vec<String> = input
            .filters
            .unwrap_or_default()
            .into_iter()
            .filter(|f| f.key.as_deref() == Some("ComplianceType"))
            .filter_map(|f| f.values)
            .flatten()
            .collect();

        let state = state.read().await;
        let summaries = state.list_compliance_summaries(&compliance_types);
        let wire_items: Vec<wire::ComplianceSummaryItem> = summaries
            .iter()
            .map(
                |(ct, compliant, non_compliant)| wire::ComplianceSummaryItem {
                    compliance_type: Some(ct.clone()),
                    compliant_summary: Some(wire::CompliantSummary {
                        compliant_count: Some(*compliant as i32),
                        severity_summary: None,
                    }),
                    non_compliant_summary: Some(wire::NonCompliantSummary {
                        non_compliant_count: Some(*non_compliant as i32),
                        severity_summary: None,
                    }),
                },
            )
            .collect();
        wire::serialize_list_compliance_summaries_response(&wire::ListComplianceSummariesResult {
            compliance_summary_items: if wire_items.is_empty() {
                None
            } else {
                Some(wire_items)
            },
            next_token: None,
        })
    }

    async fn handle_list_document_metadata_history(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_document_metadata_history_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let state = state.read().await;
        match state.list_document_metadata_history(name) {
            Ok(entries) => {
                let reviews: Vec<wire::DocumentReviewerResponseSource> = entries
                    .iter()
                    .map(|e| wire::DocumentReviewerResponseSource {
                        reviewer: Some(e.reviewer.clone()),
                        review_status: Some(e.status.clone()),
                        comment: Some(vec![wire::DocumentReviewCommentSource {
                            content: Some(e.comment.clone()),
                            r#type: Some("Comment".to_string()),
                        }]),
                        updated_time: Some(e.updated_date.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_document_metadata_history_response(
                    &wire::ListDocumentMetadataHistoryResponse {
                        metadata: Some(wire::DocumentMetadataResponseInfo {
                            reviewer_response: if reviews.is_empty() {
                                None
                            } else {
                                Some(reviews)
                            },
                        }),
                        name: Some(name.to_string()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_list_document_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_document_versions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let state = state.read().await;
        match state.documents.get(name) {
            Some(doc) => {
                let versions: Vec<wire::DocumentVersionInfo> = doc
                    .versions
                    .iter()
                    .map(|v| wire::DocumentVersionInfo {
                        name: Some(doc.name.clone()),
                        document_version: Some(v.document_version.clone()),
                        version_name: if v.version_name.is_empty() {
                            None
                        } else {
                            Some(v.version_name.clone())
                        },
                        created_date: Some(v.created_date.timestamp() as f64),
                        is_default_version: Some(v.document_version == doc.default_version),
                        document_format: Some(doc.document_format.clone()),
                        status: Some(v.status.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_document_versions_response(&wire::ListDocumentVersionsResult {
                    document_versions: if versions.is_empty() {
                        None
                    } else {
                        Some(versions)
                    },
                    next_token: None,
                })
            }
            None => json_error_response(
                400,
                "InvalidDocument",
                &format!("Document {name} does not exist."),
            ),
        }
    }

    async fn handle_list_inventory_entries(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_inventory_entries_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.instance_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceId'");
        }
        if input.type_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TypeName'");
        }
        let instance_id = input.instance_id.as_str();
        let type_name = input.type_name.as_str();
        let state = state.read().await;
        match state.get_inventory_entries(instance_id, type_name) {
            Some(data) => {
                wire::serialize_list_inventory_entries_response(&wire::ListInventoryEntriesResult {
                    instance_id: Some(instance_id.to_string()),
                    type_name: Some(type_name.to_string()),
                    capture_time: Some(data.capture_time.clone()),
                    schema_version: Some(data.schema_version.clone()),
                    entries: if data.content.is_empty() {
                        None
                    } else {
                        Some(data.content.clone())
                    },
                    next_token: None,
                })
            }
            None => {
                wire::serialize_list_inventory_entries_response(&wire::ListInventoryEntriesResult {
                    instance_id: Some(instance_id.to_string()),
                    type_name: Some(type_name.to_string()),
                    ..Default::default()
                })
            }
        }
    }

    async fn handle_list_nodes(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let nodes: Vec<wire::Node> = state
            .managed_instances
            .values()
            .map(|mi| wire::Node {
                capture_time: Some(mi.last_ping_date_time.timestamp() as f64),
                id: Some(mi.instance_id.clone()),
                owner: mi.iam_role.as_ref().map(|role| wire::NodeOwnerInfo {
                    account_id: Some("123456789012".to_string()),
                    organizational_unit_id: None,
                    organizational_unit_path: None,
                }),
                region: Some("us-east-1".to_string()),
                node_type: Some(wire::NodeType {
                    instance: Some(wire::InstanceInfo {
                        resource_type: Some(mi.resource_type.clone()),
                        ..Default::default()
                    }),
                }),
            })
            .collect();
        wire::serialize_list_nodes_response(&wire::ListNodesResult {
            nodes: if nodes.is_empty() { None } else { Some(nodes) },
            next_token: None,
        })
    }

    async fn handle_list_nodes_summary(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let mut summary: Vec<HashMap<String, String>> = Vec::new();
        if !state.managed_instances.is_empty() {
            let mut entry = HashMap::new();
            entry.insert(
                "Count".to_string(),
                state.managed_instances.len().to_string(),
            );
            entry.insert("ResourceType".to_string(), "ManagedInstance".to_string());
            summary.push(entry);
        }
        wire::serialize_list_nodes_summary_response(&wire::ListNodesSummaryResult {
            summary: if summary.is_empty() {
                None
            } else {
                Some(summary)
            },
            next_token: None,
        })
    }

    async fn handle_list_ops_item_events(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_ops_item_events_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let ops_item_id = input
            .filters
            .unwrap_or_default()
            .into_iter()
            .find(|f| f.key == "OpsItemId")
            .and_then(|mut f| f.values.drain(..).next());
        let state = state.read().await;
        let items = state.list_ops_item_events(ops_item_id.as_deref());
        let wire_events: Vec<wire::OpsItemEventSummary> = items
            .iter()
            .map(|item| wire::OpsItemEventSummary {
                ops_item_id: Some(item.ops_item_id.clone()),
                source: Some(item.source.clone()),
                created_time: Some(item.created_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_ops_item_events_response(&wire::ListOpsItemEventsResponse {
            summaries: if wire_events.is_empty() {
                None
            } else {
                Some(wire_events)
            },
            next_token: None,
        })
    }

    async fn handle_list_ops_item_related_items(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_ops_item_related_items_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let ops_item_id = input.ops_item_id.as_deref();
        let state = state.read().await;
        let items = state.list_ops_item_related_items(ops_item_id);
        let wire_items: Vec<wire::OpsItemRelatedItemSummary> = items
            .iter()
            .map(|item| wire::OpsItemRelatedItemSummary {
                association_id: Some(item.association_id.clone()),
                association_type: Some(item.association_type.clone()),
                ops_item_id: Some(item.ops_item_id.clone()),
                resource_type: Some(item.resource_type.clone()),
                resource_uri: Some(item.resource_uri.clone()),
                created_time: Some(item.created_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_ops_item_related_items_response(
            &wire::ListOpsItemRelatedItemsResponse {
                summaries: if wire_items.is_empty() {
                    None
                } else {
                    Some(wire_items)
                },
                next_token: None,
            },
        )
    }

    async fn handle_list_ops_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let entries = state.list_ops_metadata();
        let wire_entries: Vec<wire::OpsMetadata> = entries
            .iter()
            .map(|e| wire::OpsMetadata {
                ops_metadata_arn: Some(e.ops_metadata_arn.clone()),
                resource_id: Some(e.resource_id.clone()),
                creation_date: Some(e.created_date.timestamp() as f64),
                last_modified_date: Some(e.last_modified_date.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_ops_metadata_response(&wire::ListOpsMetadataResult {
            ops_metadata_list: if wire_entries.is_empty() {
                None
            } else {
                Some(wire_entries)
            },
            next_token: None,
        })
    }

    async fn handle_list_resource_compliance_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_resource_compliance_summaries_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let compliance_types: Vec<String> = input
            .filters
            .unwrap_or_default()
            .into_iter()
            .filter(|f| f.key.as_deref() == Some("ComplianceType"))
            .filter_map(|f| f.values)
            .flatten()
            .collect();

        let state = state.read().await;
        // Build per-resource summaries
        let mut by_resource: HashMap<String, HashMap<String, (usize, usize)>> = HashMap::new();
        for record in state.compliance_records.values() {
            if !compliance_types.is_empty()
                && !compliance_types
                    .iter()
                    .any(|ct| ct == &record.compliance_type)
            {
                continue;
            }
            let resource_entry = by_resource.entry(record.resource_id.clone()).or_default();
            let type_entry = resource_entry
                .entry(record.compliance_type.clone())
                .or_default();
            if record.status.to_lowercase() == "compliant" {
                type_entry.0 += 1;
            } else {
                type_entry.1 += 1;
            }
        }

        let wire_items: Vec<wire::ResourceComplianceSummaryItem> = by_resource
            .into_iter()
            .map(|(resource_id, types)| {
                let total_compliant: usize = types.values().map(|(c, _)| c).sum();
                let total_non_compliant: usize = types.values().map(|(_, nc)| nc).sum();
                wire::ResourceComplianceSummaryItem {
                    resource_id: Some(resource_id),
                    compliant_summary: Some(wire::CompliantSummary {
                        compliant_count: Some(total_compliant as i32),
                        severity_summary: None,
                    }),
                    non_compliant_summary: Some(wire::NonCompliantSummary {
                        non_compliant_count: Some(total_non_compliant as i32),
                        severity_summary: None,
                    }),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_resource_compliance_summaries_response(
            &wire::ListResourceComplianceSummariesResult {
                resource_compliance_summary_items: if wire_items.is_empty() {
                    None
                } else {
                    Some(wire_items)
                },
                next_token: None,
            },
        )
    }

    async fn handle_list_resource_data_sync(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        let state = state.read().await;
        let syncs = state.list_resource_data_syncs();
        let wire_items: Vec<wire::ResourceDataSyncItem> = syncs
            .iter()
            .map(|s| wire::ResourceDataSyncItem {
                sync_name: Some(s.sync_name.clone()),
                sync_type: Some(s.sync_type.clone()),
                s3_destination: Some(wire::ResourceDataSyncS3Destination {
                    bucket_name: s.s3_destination_bucket.clone(),
                    region: s.s3_destination_region.clone(),
                    prefix: s.s3_destination_prefix.clone(),
                    ..Default::default()
                }),
                last_status: Some(s.last_status.clone()),
                sync_created_time: Some(s.created_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_resource_data_sync_response(&wire::ListResourceDataSyncResult {
            resource_data_sync_items: if wire_items.is_empty() {
                None
            } else {
                Some(wire_items)
            },
            next_token: None,
        })
    }

    async fn handle_put_compliance_items(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_compliance_items_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId'");
        }
        if input.compliance_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ComplianceType'");
        }
        let resource_id = input.resource_id.as_str();
        let resource_type = if input.resource_type.is_empty() {
            "ManagedInstance"
        } else {
            input.resource_type.as_str()
        };
        let compliance_type = input.compliance_type.as_str();
        let execution_type = input
            .execution_summary
            .execution_type
            .clone()
            .unwrap_or_else(|| "Command".to_string());
        let execution_id = input
            .execution_summary
            .execution_id
            .clone()
            .unwrap_or_default();
        let items: Vec<(String, String, String, String, HashMap<String, String>)> = input
            .items
            .into_iter()
            .filter_map(|item| {
                let item_id = item.id?;
                let title = item.title.unwrap_or_default();
                let status = if item.status.is_empty() {
                    "COMPLIANT".to_string()
                } else {
                    item.status
                };
                let severity = if item.severity.is_empty() {
                    "UNSPECIFIED".to_string()
                } else {
                    item.severity
                };
                let details = item.details.unwrap_or_default();
                Some((item_id, title, status, severity, details))
            })
            .collect();

        let mut state = state.write().await;
        state.put_compliance_items(
            resource_id,
            resource_type,
            compliance_type,
            &execution_type,
            &execution_id,
            items,
        );
        wire::serialize_put_compliance_items_response(&wire::PutComplianceItemsResult {})
    }

    async fn handle_put_inventory(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_inventory_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.instance_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceId'");
        }
        let instance_id = input.instance_id.as_str();

        let mut state = state.write().await;
        for item in input.items {
            let type_name = if item.type_name.is_empty() {
                "AWS:Application"
            } else {
                item.type_name.as_str()
            };
            let capture_time = item.capture_time.as_str();
            let schema_version = if item.schema_version.is_empty() {
                "1.0"
            } else {
                item.schema_version.as_str()
            };
            let content = item.content.unwrap_or_default();
            state.put_inventory(
                instance_id,
                type_name,
                capture_time,
                schema_version,
                content,
            );
        }
        wire::serialize_put_inventory_response(&wire::PutInventoryResult::default())
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Policy'");
        }
        let resource_arn = input.resource_arn.as_str();
        let policy = input.policy.as_str();
        let policy_id = input.policy_id.as_deref();

        let mut state = state.write().await;
        let entry = state.put_resource_policy(resource_arn, policy, policy_id);
        wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResponse {
            policy_id: Some(entry.policy_id.clone()),
            policy_hash: Some(entry.policy_hash.clone()),
        })
    }

    async fn handle_register_default_patch_baseline(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_default_patch_baseline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BaselineId'");
        }
        let baseline_id = input.baseline_id.as_str();
        let mut state = state.write().await;
        match state.set_default_patch_baseline(baseline_id) {
            Ok(()) => wire::serialize_register_default_patch_baseline_response(
                &wire::RegisterDefaultPatchBaselineResult {
                    baseline_id: Some(baseline_id.to_string()),
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_reset_service_setting(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_reset_service_setting_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.setting_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SettingId'");
        }
        let setting_id = input.setting_id.as_str();
        let mut state = state.write().await;
        state.reset_service_setting(setting_id);
        wire::serialize_reset_service_setting_response(&wire::ResetServiceSettingResult::default())
    }

    async fn handle_resume_session(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_resume_session_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.session_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SessionId'");
        }
        let session_id = input.session_id.as_str();
        let state = state.read().await;
        match state.resume_session(session_id) {
            Ok(session) => wire::serialize_resume_session_response(&wire::ResumeSessionResponse {
                session_id: Some(session.session_id.clone()),
                ..Default::default()
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    // STUB[no-engine]: Automation execution requires a real execution engine.
    async fn handle_send_automation_signal(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_send_automation_signal_response(&wire::SendAutomationSignalResult {})
    }

    // STUB[org-integration]: Access request initiation requires real AWS Organizations or IAM Identity Center integration.
    async fn handle_start_access_request(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_start_access_request_response(&wire::StartAccessRequestResponse::default())
    }

    async fn handle_start_associations_once(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_associations_once_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let association_ids = input.association_ids;
        // Verify all associations exist
        let state = state.read().await;
        for id in &association_ids {
            if !state.associations.contains_key(id) {
                return ssm_error_response(&SsmError::AssociationDoesNotExist(id.clone()));
            }
        }
        wire::serialize_start_associations_once_response(&wire::StartAssociationsOnceResult {})
    }

    // STUB[no-engine]: Automation execution requires a real execution engine.
    async fn handle_start_automation_execution(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_start_automation_execution_response(
            &wire::StartAutomationExecutionResult::default(),
        )
    }

    // STUB[no-engine]: Change request execution requires a real execution engine.
    async fn handle_start_change_request_execution(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_start_change_request_execution_response(
            &wire::StartChangeRequestExecutionResult::default(),
        )
    }

    // STUB[no-engine]: Automation execution preview requires a real execution engine.
    async fn handle_start_execution_preview(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_start_execution_preview_response(
            &wire::StartExecutionPreviewResponse::default(),
        )
    }

    async fn handle_start_session(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_session_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.target.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Target'");
        }
        let target = input.target.as_str();
        let document_name = input.document_name.as_deref();

        let mut state = state.write().await;
        let session = state.start_session(target, document_name);
        wire::serialize_start_session_response(&wire::StartSessionResponse {
            session_id: Some(session.session_id.clone()),
            ..Default::default()
        })
    }

    // STUB[no-engine]: Automation execution requires a real execution engine.
    async fn handle_stop_automation_execution(
        &self,
        _state: &Arc<tokio::sync::RwLock<SsmState>>,
        _body: &[u8],
    ) -> MockResponse {
        wire::serialize_stop_automation_execution_response(&wire::StopAutomationExecutionResult {})
    }

    async fn handle_terminate_session(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_terminate_session_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.session_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SessionId'");
        }
        let session_id = input.session_id.as_str();
        let mut state = state.write().await;
        match state.terminate_session(session_id) {
            Ok(()) => wire::serialize_terminate_session_response(&wire::TerminateSessionResponse {
                session_id: Some(session_id.to_string()),
            }),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_association(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.association_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AssociationId'");
        }
        let association_id = input.association_id.as_str();
        let schedule_expression = input.schedule_expression.as_deref();
        let document_version = input.document_version.as_deref();

        let mut state = state.write().await;
        match state.update_association(association_id, schedule_expression, document_version) {
            Ok(assoc) => {
                wire::serialize_update_association_response(&wire::UpdateAssociationResult {
                    association_description: Some(wire::AssociationDescription {
                        association_id: Some(assoc.association_id.clone()),
                        association_name: assoc.association_name.clone(),
                        association_version: Some(assoc.association_version.clone()),
                        name: Some(assoc.name.clone()),
                        document_version: assoc.document_version.clone(),
                        schedule_expression: assoc.schedule_expression.clone(),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_association_status(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_association_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.instance_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceId'");
        }
        let name = input.name.as_str();
        let instance_id = input.instance_id.as_str();
        let status = if input.association_status.name.is_empty() {
            "Success"
        } else {
            input.association_status.name.as_str()
        };
        let detailed_status = input.association_status.message.as_str();

        let mut state = state.write().await;
        match state.update_association_status(name, instance_id, status, detailed_status) {
            Ok(Some(assoc)) => wire::serialize_update_association_status_response(
                &wire::UpdateAssociationStatusResult {
                    association_description: Some(wire::AssociationDescription {
                        association_id: Some(assoc.association_id.clone()),
                        association_name: assoc.association_name.clone(),
                        name: Some(assoc.name.clone()),
                        overview: Some(wire::AssociationOverview {
                            status: Some(assoc.overview.status.clone()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                },
            ),
            Ok(None) => wire::serialize_update_association_status_response(
                &wire::UpdateAssociationStatusResult::default(),
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_document_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_document_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let status = if input.document_reviews.action.is_empty() {
            "Approve"
        } else {
            input.document_reviews.action.as_str()
        };
        let comment = input
            .document_reviews
            .comment
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|c| c.content.as_deref())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.update_document_metadata(name, status, comment) {
            Ok(()) => wire::serialize_update_document_metadata_response(
                &wire::UpdateDocumentMetadataResponse {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_maintenance_window(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_maintenance_window_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        let window_id = input.window_id.as_str();
        let name = input.name.as_deref();
        let schedule = input.schedule.as_deref();
        let duration = input.duration.map(|d| d as i64);
        let cutoff = input.cutoff.map(|c| c as i64);
        let enabled = input.enabled;

        let mut state = state.write().await;
        match state.update_maintenance_window(window_id, name, schedule, duration, cutoff, enabled)
        {
            Ok(window) => wire::serialize_update_maintenance_window_response(
                &wire::UpdateMaintenanceWindowResult {
                    window_id: Some(window.window_id.clone()),
                    name: Some(window.name.clone()),
                    schedule: Some(window.schedule.clone()),
                    duration: Some(window.duration as i32),
                    cutoff: Some(window.cutoff as i32),
                    enabled: Some(window.enabled),
                    ..Default::default()
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_maintenance_window_target(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_maintenance_window_target_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        if input.window_target_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowTargetId'");
        }
        let window_id = input.window_id.as_str();
        let window_target_id = input.window_target_id.as_str();
        let name = input.name.as_deref();
        let description = input.description.as_deref();
        let targets = input.targets.map(|t| wire_targets_to_types(Some(t)));

        let mut state = state.write().await;
        match state.update_maintenance_window_target(
            window_id,
            window_target_id,
            name,
            description,
            targets,
        ) {
            Ok(target) => wire::serialize_update_maintenance_window_target_response(
                &wire::UpdateMaintenanceWindowTargetResult {
                    window_id: Some(target.window_id.clone()),
                    window_target_id: Some(target.window_target_id.clone()),
                    targets: Some(
                        target
                            .targets
                            .iter()
                            .map(|t| wire::Target {
                                key: Some(t.key.clone()),
                                values: Some(t.values.clone()),
                            })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_maintenance_window_task(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_maintenance_window_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.window_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowId'");
        }
        if input.window_task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WindowTaskId'");
        }
        let window_id = input.window_id.as_str();
        let window_task_id = input.window_task_id.as_str();
        let task_arn = input.task_arn.as_deref();

        let mut state = state.write().await;
        match state.update_maintenance_window_task(window_id, window_task_id, task_arn) {
            Ok(task) => wire::serialize_update_maintenance_window_task_response(
                &wire::UpdateMaintenanceWindowTaskResult {
                    window_id: Some(task.window_id.clone()),
                    window_task_id: Some(task.window_task_id.clone()),
                    task_arn: Some(task.task_arn.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_managed_instance_role(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_managed_instance_role_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.instance_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InstanceId'");
        }
        if input.iam_role.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IamRole'");
        }
        let instance_id = input.instance_id.as_str();
        let iam_role = input.iam_role.as_str();
        let mut state = state.write().await;
        match state.update_managed_instance_role(instance_id, iam_role) {
            Ok(()) => wire::serialize_update_managed_instance_role_response(
                &wire::UpdateManagedInstanceRoleResult {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_ops_item(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_ops_item_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_item_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsItemId'");
        }
        let ops_item_id = input.ops_item_id.as_str();
        let title = input.title.as_deref();
        let status = input.status.as_deref();
        let priority = input.priority;
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_ops_item(ops_item_id, title, status, priority, description) {
            Ok(()) => wire::serialize_update_ops_item_response(&wire::UpdateOpsItemResponse {}),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_ops_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_ops_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ops_metadata_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'OpsMetadataArn'");
        }
        let ops_metadata_arn = input.ops_metadata_arn.as_str();
        let metadata_to_update: HashMap<String, String> = input
            .metadata_to_update
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(k, v)| v.value.map(|s| (k, s)))
            .collect();
        let keys_to_delete = input.keys_to_delete.unwrap_or_default();

        let mut state = state.write().await;
        match state.update_ops_metadata(ops_metadata_arn, metadata_to_update, keys_to_delete) {
            Ok(()) => {
                wire::serialize_update_ops_metadata_response(&wire::UpdateOpsMetadataResult {
                    ops_metadata_arn: Some(ops_metadata_arn.to_string()),
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_patch_baseline(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_patch_baseline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BaselineId'");
        }
        let baseline_id = input.baseline_id.as_str();
        let name = input.name.as_deref();
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_patch_baseline(baseline_id, name, description) {
            Ok(baseline) => {
                wire::serialize_update_patch_baseline_response(&wire::UpdatePatchBaselineResult {
                    baseline_id: Some(baseline.baseline_id.clone()),
                    name: Some(baseline.name.clone()),
                    operating_system: Some(baseline.operating_system.clone()),
                    description: baseline.description.clone(),
                    ..Default::default()
                })
            }
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_resource_data_sync(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_resource_data_sync_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.sync_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SyncName'");
        }
        let sync_name = input.sync_name.as_str();
        let sync_type = if input.sync_type.is_empty() {
            "SyncToDestination"
        } else {
            input.sync_type.as_str()
        };

        let mut state = state.write().await;
        match state.update_resource_data_sync(sync_name, sync_type) {
            Ok(()) => wire::serialize_update_resource_data_sync_response(
                &wire::UpdateResourceDataSyncResult {},
            ),
            Err(e) => ssm_error_response(&e),
        }
    }

    async fn handle_update_service_setting(
        &self,
        state: &Arc<tokio::sync::RwLock<SsmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_setting_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.setting_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SettingId'");
        }
        if input.setting_value.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SettingValue'");
        }
        let setting_id = input.setting_id.as_str();
        let setting_value = input.setting_value.as_str();
        let arn = format!("arn:aws:ssm:us-east-1:123456789012:servicesetting/{setting_id}");

        let mut state = state.write().await;
        state.update_service_setting(setting_id, setting_value, &arn);
        wire::serialize_update_service_setting_response(&wire::UpdateServiceSettingResult {})
    }
}

// ── Helper functions ──────────────────────────────────────────────────

fn to_wire_parameter(param: &crate::types::Parameter) -> wire::Parameter {
    wire::Parameter {
        name: Some(param.name.clone()),
        r#type: Some(param.r#type.clone()),
        value: Some(param.value.clone()),
        version: Some(param.version),
        last_modified_date: Some(param.last_modified_date.timestamp() as f64),
        a_r_n: Some(param.arn.clone()),
        data_type: Some(param.data_type.clone()),
        ..Default::default()
    }
}

fn to_wire_document_description(doc: &crate::types::Document) -> wire::DocumentDescription {
    wire::DocumentDescription {
        name: Some(doc.name.clone()),
        document_type: Some(doc.document_type.clone()),
        document_format: Some(doc.document_format.clone()),
        status: Some(doc.status.clone()),
        owner: Some(doc.owner.clone()),
        default_version: Some(doc.default_version.clone()),
        latest_version: Some(doc.latest_version.clone()),
        created_date: Some(doc.created_date.timestamp() as f64),
        document_version: Some(doc.latest_version.clone()),
        ..Default::default()
    }
}

fn to_wire_command(cmd: &crate::types::Command) -> wire::Command {
    wire::Command {
        command_id: Some(cmd.command_id.clone()),
        document_name: Some(cmd.document_name.clone()),
        status: Some(cmd.status.clone()),
        instance_ids: Some(cmd.instance_ids.clone()),
        requested_date_time: Some(cmd.requested_date_time.timestamp() as f64),
        parameters: if cmd.parameters.is_empty() {
            None
        } else {
            Some(cmd.parameters.clone())
        },
        ..Default::default()
    }
}

fn wire_targets_to_types(targets: Option<Vec<wire::Target>>) -> Vec<crate::types::Target> {
    targets
        .unwrap_or_default()
        .into_iter()
        .filter_map(|t| {
            let key = t.key?;
            let values = t.values.unwrap_or_default();
            Some(crate::types::Target { key, values })
        })
        .collect()
}

fn ssm_error_response(err: &SsmError) -> MockResponse {
    let (status, error_type) = match err {
        SsmError::ParameterAlreadyExists => (400, "ParameterAlreadyExists"),
        SsmError::ParameterNotFound(_) => (400, "ParameterNotFound"),
        SsmError::ParameterVersionNotFound(_, _) => (400, "ParameterVersionNotFound"),
        SsmError::DocumentAlreadyExists(_) => (400, "DocumentAlreadyExists"),
        SsmError::InvalidDocument(_) => (400, "InvalidDocument"),
        SsmError::InvalidDocumentVersion(_) => (400, "InvalidDocumentVersion"),
        SsmError::MaintenanceWindowDoesNotExist(_) => (400, "DoesNotExistException"),
        SsmError::MaintenanceWindowTaskDoesNotExist(_, _) => (400, "DoesNotExistException"),
        SsmError::PatchBaselineDoesNotExist(_) => (400, "DoesNotExistException"),
        SsmError::SessionDoesNotExist(_) => (400, "DoesNotExistException"),
        SsmError::AssociationDoesNotExist(_) => (400, "AssociationDoesNotExist"),
        SsmError::InvalidCommandId(_) => (400, "InvalidCommandId"),
        SsmError::InvocationDoesNotExist(_) => (400, "InvocationDoesNotExist"),
        SsmError::OpsItemNotFound(_) => (400, "OpsItemNotFoundException"),
        SsmError::InvalidActivationId(_) => (400, "InvalidActivationId"),
        SsmError::OpsMetadataNotFound(_) => (400, "OpsMetadataNotFoundException"),
        SsmError::ResourceDataSyncAlreadyExists(_) => {
            (400, "ResourceDataSyncAlreadyExistsException")
        }
        SsmError::ResourceDataSyncNotFound(_) => (400, "ResourceDataSyncNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
