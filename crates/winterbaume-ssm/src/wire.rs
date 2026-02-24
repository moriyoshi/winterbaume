//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ssm

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_tags_to_resource_response(result: &AddTagsToResourceResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_ops_item_related_item_response(
    result: &AssociateOpsItemRelatedItemResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_command_response(result: &CancelCommandResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_maintenance_window_execution_response(
    result: &CancelMaintenanceWindowExecutionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_activation_response(result: &CreateActivationResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_association_response(result: &CreateAssociationResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_association_batch_response(
    result: &CreateAssociationBatchResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_document_response(result: &CreateDocumentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_maintenance_window_response(
    result: &CreateMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_ops_item_response(result: &CreateOpsItemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_ops_metadata_response(result: &CreateOpsMetadataResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_patch_baseline_response(
    result: &CreatePatchBaselineResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_resource_data_sync_response(
    result: &CreateResourceDataSyncResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_activation_response(result: &DeleteActivationResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_association_response(result: &DeleteAssociationResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_document_response(result: &DeleteDocumentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_inventory_response(result: &DeleteInventoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_maintenance_window_response(
    result: &DeleteMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_ops_item_response(result: &DeleteOpsItemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_ops_metadata_response(result: &DeleteOpsMetadataResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_parameter_response(result: &DeleteParameterResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_parameters_response(result: &DeleteParametersResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_patch_baseline_response(
    result: &DeletePatchBaselineResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_data_sync_response(
    result: &DeleteResourceDataSyncResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_managed_instance_response(
    result: &DeregisterManagedInstanceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_patch_baseline_for_patch_group_response(
    result: &DeregisterPatchBaselineForPatchGroupResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_target_from_maintenance_window_response(
    result: &DeregisterTargetFromMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_task_from_maintenance_window_response(
    result: &DeregisterTaskFromMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_activations_response(result: &DescribeActivationsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_association_response(result: &DescribeAssociationResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_association_execution_targets_response(
    result: &DescribeAssociationExecutionTargetsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_association_executions_response(
    result: &DescribeAssociationExecutionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_automation_executions_response(
    result: &DescribeAutomationExecutionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_automation_step_executions_response(
    result: &DescribeAutomationStepExecutionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_available_patches_response(
    result: &DescribeAvailablePatchesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_document_response(result: &DescribeDocumentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_document_permission_response(
    result: &DescribeDocumentPermissionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_effective_instance_associations_response(
    result: &DescribeEffectiveInstanceAssociationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_effective_patches_for_patch_baseline_response(
    result: &DescribeEffectivePatchesForPatchBaselineResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_associations_status_response(
    result: &DescribeInstanceAssociationsStatusResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_information_response(
    result: &DescribeInstanceInformationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_patch_states_response(
    result: &DescribeInstancePatchStatesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_patch_states_for_patch_group_response(
    result: &DescribeInstancePatchStatesForPatchGroupResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_patches_response(
    result: &DescribeInstancePatchesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_properties_response(
    result: &DescribeInstancePropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_inventory_deletions_response(
    result: &DescribeInventoryDeletionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_window_execution_task_invocations_response(
    result: &DescribeMaintenanceWindowExecutionTaskInvocationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_window_execution_tasks_response(
    result: &DescribeMaintenanceWindowExecutionTasksResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_window_executions_response(
    result: &DescribeMaintenanceWindowExecutionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_window_schedule_response(
    result: &DescribeMaintenanceWindowScheduleResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_window_targets_response(
    result: &DescribeMaintenanceWindowTargetsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_window_tasks_response(
    result: &DescribeMaintenanceWindowTasksResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_windows_response(
    result: &DescribeMaintenanceWindowsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_maintenance_windows_for_target_response(
    result: &DescribeMaintenanceWindowsForTargetResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_ops_items_response(result: &DescribeOpsItemsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_parameters_response(result: &DescribeParametersResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_patch_baselines_response(
    result: &DescribePatchBaselinesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_patch_group_state_response(
    result: &DescribePatchGroupStateResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_patch_groups_response(
    result: &DescribePatchGroupsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_patch_properties_response(
    result: &DescribePatchPropertiesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_sessions_response(result: &DescribeSessionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_ops_item_related_item_response(
    result: &DisassociateOpsItemRelatedItemResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_access_token_response(result: &GetAccessTokenResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_automation_execution_response(
    result: &GetAutomationExecutionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_calendar_state_response(result: &GetCalendarStateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_command_invocation_response(
    result: &GetCommandInvocationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_connection_status_response(
    result: &GetConnectionStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_default_patch_baseline_response(
    result: &GetDefaultPatchBaselineResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_deployable_patch_snapshot_for_instance_response(
    result: &GetDeployablePatchSnapshotForInstanceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_document_response(result: &GetDocumentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_execution_preview_response(
    result: &GetExecutionPreviewResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_inventory_response(result: &GetInventoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_inventory_schema_response(result: &GetInventorySchemaResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_maintenance_window_response(
    result: &GetMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_maintenance_window_execution_response(
    result: &GetMaintenanceWindowExecutionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_maintenance_window_execution_task_response(
    result: &GetMaintenanceWindowExecutionTaskResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_maintenance_window_execution_task_invocation_response(
    result: &GetMaintenanceWindowExecutionTaskInvocationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_maintenance_window_task_response(
    result: &GetMaintenanceWindowTaskResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_ops_item_response(result: &GetOpsItemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_ops_metadata_response(result: &GetOpsMetadataResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_ops_summary_response(result: &GetOpsSummaryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_parameter_response(result: &GetParameterResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_parameter_history_response(
    result: &GetParameterHistoryResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_parameters_response(result: &GetParametersResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_parameters_by_path_response(
    result: &GetParametersByPathResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_patch_baseline_response(result: &GetPatchBaselineResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_patch_baseline_for_patch_group_response(
    result: &GetPatchBaselineForPatchGroupResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policies_response(
    result: &GetResourcePoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_service_setting_response(result: &GetServiceSettingResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_label_parameter_version_response(
    result: &LabelParameterVersionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_association_versions_response(
    result: &ListAssociationVersionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_associations_response(result: &ListAssociationsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_command_invocations_response(
    result: &ListCommandInvocationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_commands_response(result: &ListCommandsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_compliance_items_response(
    result: &ListComplianceItemsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_compliance_summaries_response(
    result: &ListComplianceSummariesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_document_metadata_history_response(
    result: &ListDocumentMetadataHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_document_versions_response(
    result: &ListDocumentVersionsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_documents_response(result: &ListDocumentsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_inventory_entries_response(
    result: &ListInventoryEntriesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_nodes_response(result: &ListNodesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_nodes_summary_response(result: &ListNodesSummaryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_ops_item_events_response(result: &ListOpsItemEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_ops_item_related_items_response(
    result: &ListOpsItemRelatedItemsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_ops_metadata_response(result: &ListOpsMetadataResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resource_compliance_summaries_response(
    result: &ListResourceComplianceSummariesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resource_data_sync_response(
    result: &ListResourceDataSyncResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_document_permission_response(
    result: &ModifyDocumentPermissionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_compliance_items_response(result: &PutComplianceItemsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_inventory_response(result: &PutInventoryResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_parameter_response(result: &PutParameterResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_default_patch_baseline_response(
    result: &RegisterDefaultPatchBaselineResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_patch_baseline_for_patch_group_response(
    result: &RegisterPatchBaselineForPatchGroupResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_target_with_maintenance_window_response(
    result: &RegisterTargetWithMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_task_with_maintenance_window_response(
    result: &RegisterTaskWithMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_tags_from_resource_response(
    result: &RemoveTagsFromResourceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reset_service_setting_response(
    result: &ResetServiceSettingResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_resume_session_response(result: &ResumeSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_automation_signal_response(
    result: &SendAutomationSignalResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_command_response(result: &SendCommandResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_access_request_response(
    result: &StartAccessRequestResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_associations_once_response(
    result: &StartAssociationsOnceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_automation_execution_response(
    result: &StartAutomationExecutionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_change_request_execution_response(
    result: &StartChangeRequestExecutionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_execution_preview_response(
    result: &StartExecutionPreviewResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_session_response(result: &StartSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_automation_execution_response(
    result: &StopAutomationExecutionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_terminate_session_response(result: &TerminateSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_unlabel_parameter_version_response(
    result: &UnlabelParameterVersionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_association_response(result: &UpdateAssociationResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_association_status_response(
    result: &UpdateAssociationStatusResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_document_response(result: &UpdateDocumentResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_document_default_version_response(
    result: &UpdateDocumentDefaultVersionResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_document_metadata_response(
    result: &UpdateDocumentMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_maintenance_window_response(
    result: &UpdateMaintenanceWindowResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_maintenance_window_target_response(
    result: &UpdateMaintenanceWindowTargetResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_maintenance_window_task_response(
    result: &UpdateMaintenanceWindowTaskResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_managed_instance_role_response(
    result: &UpdateManagedInstanceRoleResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_ops_item_response(result: &UpdateOpsItemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_ops_metadata_response(result: &UpdateOpsMetadataResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_patch_baseline_response(
    result: &UpdatePatchBaselineResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_resource_data_sync_response(
    result: &UpdateResourceDataSyncResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_service_setting_response(
    result: &UpdateServiceSettingResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_to_resource_request(
    body: &[u8],
) -> Result<AddTagsToResourceRequest, String> {
    if body.is_empty() {
        return Ok(AddTagsToResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddTagsToResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_ops_item_related_item_request(
    body: &[u8],
) -> Result<AssociateOpsItemRelatedItemRequest, String> {
    if body.is_empty() {
        return Ok(AssociateOpsItemRelatedItemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateOpsItemRelatedItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_command_request(body: &[u8]) -> Result<CancelCommandRequest, String> {
    if body.is_empty() {
        return Ok(CancelCommandRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelCommand request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_maintenance_window_execution_request(
    body: &[u8],
) -> Result<CancelMaintenanceWindowExecutionRequest, String> {
    if body.is_empty() {
        return Ok(CancelMaintenanceWindowExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelMaintenanceWindowExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_activation_request(
    body: &[u8],
) -> Result<CreateActivationRequest, String> {
    if body.is_empty() {
        return Ok(CreateActivationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateActivation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_association_request(
    body: &[u8],
) -> Result<CreateAssociationRequest, String> {
    if body.is_empty() {
        return Ok(CreateAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_association_batch_request(
    body: &[u8],
) -> Result<CreateAssociationBatchRequest, String> {
    if body.is_empty() {
        return Ok(CreateAssociationBatchRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAssociationBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_document_request(body: &[u8]) -> Result<CreateDocumentRequest, String> {
    if body.is_empty() {
        return Ok(CreateDocumentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDocument request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_maintenance_window_request(
    body: &[u8],
) -> Result<CreateMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(CreateMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMaintenanceWindow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_ops_item_request(body: &[u8]) -> Result<CreateOpsItemRequest, String> {
    if body.is_empty() {
        return Ok(CreateOpsItemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateOpsItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_ops_metadata_request(
    body: &[u8],
) -> Result<CreateOpsMetadataRequest, String> {
    if body.is_empty() {
        return Ok(CreateOpsMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateOpsMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_patch_baseline_request(
    body: &[u8],
) -> Result<CreatePatchBaselineRequest, String> {
    if body.is_empty() {
        return Ok(CreatePatchBaselineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePatchBaseline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_resource_data_sync_request(
    body: &[u8],
) -> Result<CreateResourceDataSyncRequest, String> {
    if body.is_empty() {
        return Ok(CreateResourceDataSyncRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateResourceDataSync request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_activation_request(
    body: &[u8],
) -> Result<DeleteActivationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteActivationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteActivation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_association_request(
    body: &[u8],
) -> Result<DeleteAssociationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_document_request(body: &[u8]) -> Result<DeleteDocumentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDocumentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDocument request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_inventory_request(body: &[u8]) -> Result<DeleteInventoryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteInventoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteInventory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_maintenance_window_request(
    body: &[u8],
) -> Result<DeleteMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMaintenanceWindow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_ops_item_request(body: &[u8]) -> Result<DeleteOpsItemRequest, String> {
    if body.is_empty() {
        return Ok(DeleteOpsItemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteOpsItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_ops_metadata_request(
    body: &[u8],
) -> Result<DeleteOpsMetadataRequest, String> {
    if body.is_empty() {
        return Ok(DeleteOpsMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteOpsMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_parameter_request(body: &[u8]) -> Result<DeleteParameterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteParameterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteParameter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_parameters_request(
    body: &[u8],
) -> Result<DeleteParametersRequest, String> {
    if body.is_empty() {
        return Ok(DeleteParametersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteParameters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_patch_baseline_request(
    body: &[u8],
) -> Result<DeletePatchBaselineRequest, String> {
    if body.is_empty() {
        return Ok(DeletePatchBaselineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePatchBaseline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_data_sync_request(
    body: &[u8],
) -> Result<DeleteResourceDataSyncRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourceDataSyncRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourceDataSync request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_managed_instance_request(
    body: &[u8],
) -> Result<DeregisterManagedInstanceRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterManagedInstanceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterManagedInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_patch_baseline_for_patch_group_request(
    body: &[u8],
) -> Result<DeregisterPatchBaselineForPatchGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterPatchBaselineForPatchGroupRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeregisterPatchBaselineForPatchGroup request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_target_from_maintenance_window_request(
    body: &[u8],
) -> Result<DeregisterTargetFromMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterTargetFromMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeregisterTargetFromMaintenanceWindow request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_task_from_maintenance_window_request(
    body: &[u8],
) -> Result<DeregisterTaskFromMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterTaskFromMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeregisterTaskFromMaintenanceWindow request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_activations_request(
    body: &[u8],
) -> Result<DescribeActivationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeActivationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeActivations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_association_request(
    body: &[u8],
) -> Result<DescribeAssociationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_association_execution_targets_request(
    body: &[u8],
) -> Result<DescribeAssociationExecutionTargetsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAssociationExecutionTargetsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeAssociationExecutionTargets request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_association_executions_request(
    body: &[u8],
) -> Result<DescribeAssociationExecutionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAssociationExecutionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAssociationExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_automation_executions_request(
    body: &[u8],
) -> Result<DescribeAutomationExecutionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAutomationExecutionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAutomationExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_automation_step_executions_request(
    body: &[u8],
) -> Result<DescribeAutomationStepExecutionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAutomationStepExecutionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAutomationStepExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_available_patches_request(
    body: &[u8],
) -> Result<DescribeAvailablePatchesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAvailablePatchesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAvailablePatches request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_document_request(
    body: &[u8],
) -> Result<DescribeDocumentRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDocumentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDocument request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_document_permission_request(
    body: &[u8],
) -> Result<DescribeDocumentPermissionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDocumentPermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDocumentPermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_effective_instance_associations_request(
    body: &[u8],
) -> Result<DescribeEffectiveInstanceAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEffectiveInstanceAssociationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeEffectiveInstanceAssociations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_effective_patches_for_patch_baseline_request(
    body: &[u8],
) -> Result<DescribeEffectivePatchesForPatchBaselineRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEffectivePatchesForPatchBaselineRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeEffectivePatchesForPatchBaseline request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_associations_status_request(
    body: &[u8],
) -> Result<DescribeInstanceAssociationsStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstanceAssociationsStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeInstanceAssociationsStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_information_request(
    body: &[u8],
) -> Result<DescribeInstanceInformationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstanceInformationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInstanceInformation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_patch_states_request(
    body: &[u8],
) -> Result<DescribeInstancePatchStatesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstancePatchStatesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInstancePatchStates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_patch_states_for_patch_group_request(
    body: &[u8],
) -> Result<DescribeInstancePatchStatesForPatchGroupRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstancePatchStatesForPatchGroupRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeInstancePatchStatesForPatchGroup request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_patches_request(
    body: &[u8],
) -> Result<DescribeInstancePatchesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstancePatchesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInstancePatches request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_properties_request(
    body: &[u8],
) -> Result<DescribeInstancePropertiesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInstancePropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInstanceProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_inventory_deletions_request(
    body: &[u8],
) -> Result<DescribeInventoryDeletionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInventoryDeletionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInventoryDeletions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_window_execution_task_invocations_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowExecutionTaskInvocationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowExecutionTaskInvocationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize DescribeMaintenanceWindowExecutionTaskInvocations request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_window_execution_tasks_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowExecutionTasksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowExecutionTasksRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeMaintenanceWindowExecutionTasks request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_window_executions_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowExecutionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowExecutionsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeMaintenanceWindowExecutions request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_window_schedule_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowScheduleRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowScheduleRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeMaintenanceWindowSchedule request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_window_targets_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowTargetsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowTargetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMaintenanceWindowTargets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_window_tasks_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowTasksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMaintenanceWindowTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_windows_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMaintenanceWindows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_maintenance_windows_for_target_request(
    body: &[u8],
) -> Result<DescribeMaintenanceWindowsForTargetRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMaintenanceWindowsForTargetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeMaintenanceWindowsForTarget request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_ops_items_request(
    body: &[u8],
) -> Result<DescribeOpsItemsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeOpsItemsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeOpsItems request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_parameters_request(
    body: &[u8],
) -> Result<DescribeParametersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeParametersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeParameters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_patch_baselines_request(
    body: &[u8],
) -> Result<DescribePatchBaselinesRequest, String> {
    if body.is_empty() {
        return Ok(DescribePatchBaselinesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePatchBaselines request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_patch_group_state_request(
    body: &[u8],
) -> Result<DescribePatchGroupStateRequest, String> {
    if body.is_empty() {
        return Ok(DescribePatchGroupStateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePatchGroupState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_patch_groups_request(
    body: &[u8],
) -> Result<DescribePatchGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribePatchGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePatchGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_patch_properties_request(
    body: &[u8],
) -> Result<DescribePatchPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(DescribePatchPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePatchProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_sessions_request(
    body: &[u8],
) -> Result<DescribeSessionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSessionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSessions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_ops_item_related_item_request(
    body: &[u8],
) -> Result<DisassociateOpsItemRelatedItemRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateOpsItemRelatedItemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateOpsItemRelatedItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_access_token_request(body: &[u8]) -> Result<GetAccessTokenRequest, String> {
    if body.is_empty() {
        return Ok(GetAccessTokenRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAccessToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_automation_execution_request(
    body: &[u8],
) -> Result<GetAutomationExecutionRequest, String> {
    if body.is_empty() {
        return Ok(GetAutomationExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAutomationExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_calendar_state_request(
    body: &[u8],
) -> Result<GetCalendarStateRequest, String> {
    if body.is_empty() {
        return Ok(GetCalendarStateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCalendarState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_command_invocation_request(
    body: &[u8],
) -> Result<GetCommandInvocationRequest, String> {
    if body.is_empty() {
        return Ok(GetCommandInvocationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCommandInvocation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_connection_status_request(
    body: &[u8],
) -> Result<GetConnectionStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetConnectionStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetConnectionStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_default_patch_baseline_request(
    body: &[u8],
) -> Result<GetDefaultPatchBaselineRequest, String> {
    if body.is_empty() {
        return Ok(GetDefaultPatchBaselineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDefaultPatchBaseline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_deployable_patch_snapshot_for_instance_request(
    body: &[u8],
) -> Result<GetDeployablePatchSnapshotForInstanceRequest, String> {
    if body.is_empty() {
        return Ok(GetDeployablePatchSnapshotForInstanceRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetDeployablePatchSnapshotForInstance request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_document_request(body: &[u8]) -> Result<GetDocumentRequest, String> {
    if body.is_empty() {
        return Ok(GetDocumentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDocument request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_execution_preview_request(
    body: &[u8],
) -> Result<GetExecutionPreviewRequest, String> {
    if body.is_empty() {
        return Ok(GetExecutionPreviewRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetExecutionPreview request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_inventory_request(body: &[u8]) -> Result<GetInventoryRequest, String> {
    if body.is_empty() {
        return Ok(GetInventoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetInventory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_inventory_schema_request(
    body: &[u8],
) -> Result<GetInventorySchemaRequest, String> {
    if body.is_empty() {
        return Ok(GetInventorySchemaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetInventorySchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_maintenance_window_request(
    body: &[u8],
) -> Result<GetMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(GetMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMaintenanceWindow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_maintenance_window_execution_request(
    body: &[u8],
) -> Result<GetMaintenanceWindowExecutionRequest, String> {
    if body.is_empty() {
        return Ok(GetMaintenanceWindowExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMaintenanceWindowExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_maintenance_window_execution_task_request(
    body: &[u8],
) -> Result<GetMaintenanceWindowExecutionTaskRequest, String> {
    if body.is_empty() {
        return Ok(GetMaintenanceWindowExecutionTaskRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetMaintenanceWindowExecutionTask request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_maintenance_window_execution_task_invocation_request(
    body: &[u8],
) -> Result<GetMaintenanceWindowExecutionTaskInvocationRequest, String> {
    if body.is_empty() {
        return Ok(GetMaintenanceWindowExecutionTaskInvocationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetMaintenanceWindowExecutionTaskInvocation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_maintenance_window_task_request(
    body: &[u8],
) -> Result<GetMaintenanceWindowTaskRequest, String> {
    if body.is_empty() {
        return Ok(GetMaintenanceWindowTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMaintenanceWindowTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_ops_item_request(body: &[u8]) -> Result<GetOpsItemRequest, String> {
    if body.is_empty() {
        return Ok(GetOpsItemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOpsItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_ops_metadata_request(body: &[u8]) -> Result<GetOpsMetadataRequest, String> {
    if body.is_empty() {
        return Ok(GetOpsMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOpsMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_ops_summary_request(body: &[u8]) -> Result<GetOpsSummaryRequest, String> {
    if body.is_empty() {
        return Ok(GetOpsSummaryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOpsSummary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_parameter_request(body: &[u8]) -> Result<GetParameterRequest, String> {
    if body.is_empty() {
        return Ok(GetParameterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetParameter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_parameter_history_request(
    body: &[u8],
) -> Result<GetParameterHistoryRequest, String> {
    if body.is_empty() {
        return Ok(GetParameterHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetParameterHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_parameters_request(body: &[u8]) -> Result<GetParametersRequest, String> {
    if body.is_empty() {
        return Ok(GetParametersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetParameters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_parameters_by_path_request(
    body: &[u8],
) -> Result<GetParametersByPathRequest, String> {
    if body.is_empty() {
        return Ok(GetParametersByPathRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetParametersByPath request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_patch_baseline_request(
    body: &[u8],
) -> Result<GetPatchBaselineRequest, String> {
    if body.is_empty() {
        return Ok(GetPatchBaselineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPatchBaseline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_patch_baseline_for_patch_group_request(
    body: &[u8],
) -> Result<GetPatchBaselineForPatchGroupRequest, String> {
    if body.is_empty() {
        return Ok(GetPatchBaselineForPatchGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPatchBaselineForPatchGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policies_request(
    body: &[u8],
) -> Result<GetResourcePoliciesRequest, String> {
    if body.is_empty() {
        return Ok(GetResourcePoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_service_setting_request(
    body: &[u8],
) -> Result<GetServiceSettingRequest, String> {
    if body.is_empty() {
        return Ok(GetServiceSettingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetServiceSetting request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_label_parameter_version_request(
    body: &[u8],
) -> Result<LabelParameterVersionRequest, String> {
    if body.is_empty() {
        return Ok(LabelParameterVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize LabelParameterVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_association_versions_request(
    body: &[u8],
) -> Result<ListAssociationVersionsRequest, String> {
    if body.is_empty() {
        return Ok(ListAssociationVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAssociationVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_associations_request(
    body: &[u8],
) -> Result<ListAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(ListAssociationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAssociations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_command_invocations_request(
    body: &[u8],
) -> Result<ListCommandInvocationsRequest, String> {
    if body.is_empty() {
        return Ok(ListCommandInvocationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCommandInvocations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_commands_request(body: &[u8]) -> Result<ListCommandsRequest, String> {
    if body.is_empty() {
        return Ok(ListCommandsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCommands request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_compliance_items_request(
    body: &[u8],
) -> Result<ListComplianceItemsRequest, String> {
    if body.is_empty() {
        return Ok(ListComplianceItemsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListComplianceItems request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_compliance_summaries_request(
    body: &[u8],
) -> Result<ListComplianceSummariesRequest, String> {
    if body.is_empty() {
        return Ok(ListComplianceSummariesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListComplianceSummaries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_document_metadata_history_request(
    body: &[u8],
) -> Result<ListDocumentMetadataHistoryRequest, String> {
    if body.is_empty() {
        return Ok(ListDocumentMetadataHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDocumentMetadataHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_document_versions_request(
    body: &[u8],
) -> Result<ListDocumentVersionsRequest, String> {
    if body.is_empty() {
        return Ok(ListDocumentVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDocumentVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_documents_request(body: &[u8]) -> Result<ListDocumentsRequest, String> {
    if body.is_empty() {
        return Ok(ListDocumentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDocuments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_inventory_entries_request(
    body: &[u8],
) -> Result<ListInventoryEntriesRequest, String> {
    if body.is_empty() {
        return Ok(ListInventoryEntriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInventoryEntries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_nodes_request(body: &[u8]) -> Result<ListNodesRequest, String> {
    if body.is_empty() {
        return Ok(ListNodesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListNodes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_nodes_summary_request(
    body: &[u8],
) -> Result<ListNodesSummaryRequest, String> {
    if body.is_empty() {
        return Ok(ListNodesSummaryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListNodesSummary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_ops_item_events_request(
    body: &[u8],
) -> Result<ListOpsItemEventsRequest, String> {
    if body.is_empty() {
        return Ok(ListOpsItemEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOpsItemEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_ops_item_related_items_request(
    body: &[u8],
) -> Result<ListOpsItemRelatedItemsRequest, String> {
    if body.is_empty() {
        return Ok(ListOpsItemRelatedItemsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOpsItemRelatedItems request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_ops_metadata_request(
    body: &[u8],
) -> Result<ListOpsMetadataRequest, String> {
    if body.is_empty() {
        return Ok(ListOpsMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOpsMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resource_compliance_summaries_request(
    body: &[u8],
) -> Result<ListResourceComplianceSummariesRequest, String> {
    if body.is_empty() {
        return Ok(ListResourceComplianceSummariesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourceComplianceSummaries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resource_data_sync_request(
    body: &[u8],
) -> Result<ListResourceDataSyncRequest, String> {
    if body.is_empty() {
        return Ok(ListResourceDataSyncRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourceDataSync request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_document_permission_request(
    body: &[u8],
) -> Result<ModifyDocumentPermissionRequest, String> {
    if body.is_empty() {
        return Ok(ModifyDocumentPermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyDocumentPermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_compliance_items_request(
    body: &[u8],
) -> Result<PutComplianceItemsRequest, String> {
    if body.is_empty() {
        return Ok(PutComplianceItemsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutComplianceItems request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_inventory_request(body: &[u8]) -> Result<PutInventoryRequest, String> {
    if body.is_empty() {
        return Ok(PutInventoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutInventory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_parameter_request(body: &[u8]) -> Result<PutParameterRequest, String> {
    if body.is_empty() {
        return Ok(PutParameterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutParameter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_default_patch_baseline_request(
    body: &[u8],
) -> Result<RegisterDefaultPatchBaselineRequest, String> {
    if body.is_empty() {
        return Ok(RegisterDefaultPatchBaselineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterDefaultPatchBaseline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_patch_baseline_for_patch_group_request(
    body: &[u8],
) -> Result<RegisterPatchBaselineForPatchGroupRequest, String> {
    if body.is_empty() {
        return Ok(RegisterPatchBaselineForPatchGroupRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize RegisterPatchBaselineForPatchGroup request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_target_with_maintenance_window_request(
    body: &[u8],
) -> Result<RegisterTargetWithMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(RegisterTargetWithMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize RegisterTargetWithMaintenanceWindow request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_task_with_maintenance_window_request(
    body: &[u8],
) -> Result<RegisterTaskWithMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(RegisterTaskWithMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize RegisterTaskWithMaintenanceWindow request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_tags_from_resource_request(
    body: &[u8],
) -> Result<RemoveTagsFromResourceRequest, String> {
    if body.is_empty() {
        return Ok(RemoveTagsFromResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTagsFromResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reset_service_setting_request(
    body: &[u8],
) -> Result<ResetServiceSettingRequest, String> {
    if body.is_empty() {
        return Ok(ResetServiceSettingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResetServiceSetting request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resume_session_request(body: &[u8]) -> Result<ResumeSessionRequest, String> {
    if body.is_empty() {
        return Ok(ResumeSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResumeSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_automation_signal_request(
    body: &[u8],
) -> Result<SendAutomationSignalRequest, String> {
    if body.is_empty() {
        return Ok(SendAutomationSignalRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendAutomationSignal request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_command_request(body: &[u8]) -> Result<SendCommandRequest, String> {
    if body.is_empty() {
        return Ok(SendCommandRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendCommand request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_access_request_request(
    body: &[u8],
) -> Result<StartAccessRequestRequest, String> {
    if body.is_empty() {
        return Ok(StartAccessRequestRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartAccessRequest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_associations_once_request(
    body: &[u8],
) -> Result<StartAssociationsOnceRequest, String> {
    if body.is_empty() {
        return Ok(StartAssociationsOnceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartAssociationsOnce request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_automation_execution_request(
    body: &[u8],
) -> Result<StartAutomationExecutionRequest, String> {
    if body.is_empty() {
        return Ok(StartAutomationExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartAutomationExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_change_request_execution_request(
    body: &[u8],
) -> Result<StartChangeRequestExecutionRequest, String> {
    if body.is_empty() {
        return Ok(StartChangeRequestExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartChangeRequestExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_execution_preview_request(
    body: &[u8],
) -> Result<StartExecutionPreviewRequest, String> {
    if body.is_empty() {
        return Ok(StartExecutionPreviewRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartExecutionPreview request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_session_request(body: &[u8]) -> Result<StartSessionRequest, String> {
    if body.is_empty() {
        return Ok(StartSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_automation_execution_request(
    body: &[u8],
) -> Result<StopAutomationExecutionRequest, String> {
    if body.is_empty() {
        return Ok(StopAutomationExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopAutomationExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_terminate_session_request(
    body: &[u8],
) -> Result<TerminateSessionRequest, String> {
    if body.is_empty() {
        return Ok(TerminateSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_unlabel_parameter_version_request(
    body: &[u8],
) -> Result<UnlabelParameterVersionRequest, String> {
    if body.is_empty() {
        return Ok(UnlabelParameterVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UnlabelParameterVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_association_request(
    body: &[u8],
) -> Result<UpdateAssociationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_association_status_request(
    body: &[u8],
) -> Result<UpdateAssociationStatusRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAssociationStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAssociationStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_document_request(body: &[u8]) -> Result<UpdateDocumentRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDocumentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDocument request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_document_default_version_request(
    body: &[u8],
) -> Result<UpdateDocumentDefaultVersionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDocumentDefaultVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDocumentDefaultVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_document_metadata_request(
    body: &[u8],
) -> Result<UpdateDocumentMetadataRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDocumentMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDocumentMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_maintenance_window_request(
    body: &[u8],
) -> Result<UpdateMaintenanceWindowRequest, String> {
    if body.is_empty() {
        return Ok(UpdateMaintenanceWindowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMaintenanceWindow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_maintenance_window_target_request(
    body: &[u8],
) -> Result<UpdateMaintenanceWindowTargetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateMaintenanceWindowTargetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMaintenanceWindowTarget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_maintenance_window_task_request(
    body: &[u8],
) -> Result<UpdateMaintenanceWindowTaskRequest, String> {
    if body.is_empty() {
        return Ok(UpdateMaintenanceWindowTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMaintenanceWindowTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_managed_instance_role_request(
    body: &[u8],
) -> Result<UpdateManagedInstanceRoleRequest, String> {
    if body.is_empty() {
        return Ok(UpdateManagedInstanceRoleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateManagedInstanceRole request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_ops_item_request(body: &[u8]) -> Result<UpdateOpsItemRequest, String> {
    if body.is_empty() {
        return Ok(UpdateOpsItemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateOpsItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_ops_metadata_request(
    body: &[u8],
) -> Result<UpdateOpsMetadataRequest, String> {
    if body.is_empty() {
        return Ok(UpdateOpsMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateOpsMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_patch_baseline_request(
    body: &[u8],
) -> Result<UpdatePatchBaselineRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePatchBaselineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePatchBaseline request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_resource_data_sync_request(
    body: &[u8],
) -> Result<UpdateResourceDataSyncRequest, String> {
    if body.is_empty() {
        return Ok(UpdateResourceDataSyncRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResourceDataSync request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_service_setting_request(
    body: &[u8],
) -> Result<UpdateServiceSettingRequest, String> {
    if body.is_empty() {
        return Ok(UpdateServiceSettingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateServiceSetting request: {e}"))
}
