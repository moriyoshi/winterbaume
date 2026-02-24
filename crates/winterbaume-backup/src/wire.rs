//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-backup

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for restJson protocol.
pub fn serialize_associate_backup_vault_mpa_approval_team_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_legal_hold_response(result: &CancelLegalHoldOutput) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backup_plan_response(result: &CreateBackupPlanOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backup_selection_response(
    result: &CreateBackupSelectionOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backup_vault_response(result: &CreateBackupVaultOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_framework_response(result: &CreateFrameworkOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_legal_hold_response(result: &CreateLegalHoldOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_logically_air_gapped_backup_vault_response(
    result: &CreateLogicallyAirGappedBackupVaultOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_report_plan_response(result: &CreateReportPlanOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_restore_access_backup_vault_response(
    result: &CreateRestoreAccessBackupVaultOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_restore_testing_plan_response(
    result: &CreateRestoreTestingPlanOutput,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_restore_testing_selection_response(
    result: &CreateRestoreTestingSelectionOutput,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_tiering_configuration_response(
    result: &CreateTieringConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_backup_plan_response(result: &DeleteBackupPlanOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_backup_selection_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_backup_vault_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_backup_vault_access_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_backup_vault_lock_configuration_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_backup_vault_notifications_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_framework_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_recovery_point_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_report_plan_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_restore_testing_plan_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_restore_testing_selection_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_tiering_configuration_response(
    result: &DeleteTieringConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_backup_job_response(result: &DescribeBackupJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_backup_vault_response(
    result: &DescribeBackupVaultOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_copy_job_response(result: &DescribeCopyJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_framework_response(result: &DescribeFrameworkOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_global_settings_response(
    result: &DescribeGlobalSettingsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_protected_resource_response(
    result: &DescribeProtectedResourceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_recovery_point_response(
    result: &DescribeRecoveryPointOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_region_settings_response(
    result: &DescribeRegionSettingsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_report_job_response(result: &DescribeReportJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_report_plan_response(result: &DescribeReportPlanOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_restore_job_response(result: &DescribeRestoreJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_scan_job_response(result: &DescribeScanJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_backup_vault_mpa_approval_team_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_recovery_point_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disassociate_recovery_point_from_parent_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_export_backup_plan_template_response(
    result: &ExportBackupPlanTemplateOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backup_plan_response(result: &GetBackupPlanOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backup_plan_from_j_s_o_n_response(
    result: &GetBackupPlanFromJSONOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backup_plan_from_template_response(
    result: &GetBackupPlanFromTemplateOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backup_selection_response(result: &GetBackupSelectionOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backup_vault_access_policy_response(
    result: &GetBackupVaultAccessPolicyOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backup_vault_notifications_response(
    result: &GetBackupVaultNotificationsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_legal_hold_response(result: &GetLegalHoldOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_recovery_point_index_details_response(
    result: &GetRecoveryPointIndexDetailsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_recovery_point_restore_metadata_response(
    result: &GetRecoveryPointRestoreMetadataOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_restore_job_metadata_response(
    result: &GetRestoreJobMetadataOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_restore_testing_inferred_metadata_response(
    result: &GetRestoreTestingInferredMetadataOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_restore_testing_plan_response(
    result: &GetRestoreTestingPlanOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_restore_testing_selection_response(
    result: &GetRestoreTestingSelectionOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_supported_resource_types_response(
    result: &GetSupportedResourceTypesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tiering_configuration_response(
    result: &GetTieringConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backup_job_summaries_response(
    result: &ListBackupJobSummariesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backup_jobs_response(result: &ListBackupJobsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backup_plan_templates_response(
    result: &ListBackupPlanTemplatesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backup_plan_versions_response(
    result: &ListBackupPlanVersionsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backup_plans_response(result: &ListBackupPlansOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backup_selections_response(
    result: &ListBackupSelectionsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backup_vaults_response(result: &ListBackupVaultsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_copy_job_summaries_response(
    result: &ListCopyJobSummariesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_copy_jobs_response(result: &ListCopyJobsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_frameworks_response(result: &ListFrameworksOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_indexed_recovery_points_response(
    result: &ListIndexedRecoveryPointsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_legal_holds_response(result: &ListLegalHoldsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_protected_resources_response(
    result: &ListProtectedResourcesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_protected_resources_by_backup_vault_response(
    result: &ListProtectedResourcesByBackupVaultOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_recovery_points_by_backup_vault_response(
    result: &ListRecoveryPointsByBackupVaultOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_recovery_points_by_legal_hold_response(
    result: &ListRecoveryPointsByLegalHoldOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_recovery_points_by_resource_response(
    result: &ListRecoveryPointsByResourceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_report_jobs_response(result: &ListReportJobsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_report_plans_response(result: &ListReportPlansOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_restore_access_backup_vaults_response(
    result: &ListRestoreAccessBackupVaultsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_restore_job_summaries_response(
    result: &ListRestoreJobSummariesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_restore_jobs_response(result: &ListRestoreJobsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_restore_jobs_by_protected_resource_response(
    result: &ListRestoreJobsByProtectedResourceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_restore_testing_plans_response(
    result: &ListRestoreTestingPlansOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_restore_testing_selections_response(
    result: &ListRestoreTestingSelectionsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_scan_job_summaries_response(
    result: &ListScanJobSummariesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_scan_jobs_response(result: &ListScanJobsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tiering_configurations_response(
    result: &ListTieringConfigurationsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_backup_vault_access_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_backup_vault_lock_configuration_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_backup_vault_notifications_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_restore_validation_result_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_revoke_restore_access_backup_vault_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_start_backup_job_response(result: &StartBackupJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_copy_job_response(result: &StartCopyJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_report_job_response(result: &StartReportJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_restore_job_response(result: &StartRestoreJobOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_scan_job_response(result: &StartScanJobOutput) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_stop_backup_job_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_backup_plan_response(result: &UpdateBackupPlanOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_framework_response(result: &UpdateFrameworkOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_global_settings_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_recovery_point_index_settings_response(
    result: &UpdateRecoveryPointIndexSettingsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_recovery_point_lifecycle_response(
    result: &UpdateRecoveryPointLifecycleOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_region_settings_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_report_plan_response(result: &UpdateReportPlanOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_restore_testing_plan_response(
    result: &UpdateRestoreTestingPlanOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_restore_testing_selection_response(
    result: &UpdateRestoreTestingSelectionOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_tiering_configuration_response(
    result: &UpdateTieringConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_backup_vault_mpa_approval_team_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateBackupVaultMpaApprovalTeamInput, String> {
    let mut input = AssociateBackupVaultMpaApprovalTeamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateBackupVaultMpaApprovalTeamInput>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize AssociateBackupVaultMpaApprovalTeam request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_legal_hold_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelLegalHoldInput, String> {
    let mut input = CancelLegalHoldInput::default();
    for (name, value) in labels {
        match *name {
            "LegalHoldId" => {
                input.legal_hold_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("cancelDescription") {
        input.cancel_description = value.to_string();
    }
    if let Some(value) = query.get("retainRecordInDays") {
        input.retain_record_in_days = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backup_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackupPlanInput, String> {
    let mut input = CreateBackupPlanInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackupPlanInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackupPlan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backup_selection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackupSelectionInput, String> {
    let mut input = CreateBackupSelectionInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackupSelectionInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackupSelection request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackupVaultInput, String> {
    let mut input = CreateBackupVaultInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackupVaultInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackupVault request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFrameworkInput, String> {
    let mut input = CreateFrameworkInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFrameworkInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFramework request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_legal_hold_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLegalHoldInput, String> {
    let mut input = CreateLegalHoldInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLegalHoldInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateLegalHold request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_logically_air_gapped_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLogicallyAirGappedBackupVaultInput, String> {
    let mut input = CreateLogicallyAirGappedBackupVaultInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLogicallyAirGappedBackupVaultInput>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize CreateLogicallyAirGappedBackupVault request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_report_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateReportPlanInput, String> {
    let mut input = CreateReportPlanInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateReportPlanInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateReportPlan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_restore_access_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRestoreAccessBackupVaultInput, String> {
    let mut input = CreateRestoreAccessBackupVaultInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRestoreAccessBackupVaultInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateRestoreAccessBackupVault request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_restore_testing_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRestoreTestingPlanInput, String> {
    let mut input = CreateRestoreTestingPlanInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRestoreTestingPlanInput>(&request.body).map_err(
            |err| format!("failed to deserialize CreateRestoreTestingPlan request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_restore_testing_selection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRestoreTestingSelectionInput, String> {
    let mut input = CreateRestoreTestingSelectionInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRestoreTestingSelectionInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateRestoreTestingSelection request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_tiering_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTieringConfigurationInput, String> {
    let mut input = CreateTieringConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTieringConfigurationInput>(&request.body).map_err(
            |err| format!("failed to deserialize CreateTieringConfiguration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backup_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackupPlanInput, String> {
    let mut input = DeleteBackupPlanInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backup_selection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackupSelectionInput, String> {
    let mut input = DeleteBackupSelectionInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            "SelectionId" => {
                input.selection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackupVaultInput, String> {
    let mut input = DeleteBackupVaultInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backup_vault_access_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackupVaultAccessPolicyInput, String> {
    let mut input = DeleteBackupVaultAccessPolicyInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backup_vault_lock_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackupVaultLockConfigurationInput, String> {
    let mut input = DeleteBackupVaultLockConfigurationInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backup_vault_notifications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackupVaultNotificationsInput, String> {
    let mut input = DeleteBackupVaultNotificationsInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFrameworkInput, String> {
    let mut input = DeleteFrameworkInput::default();
    for (name, value) in labels {
        match *name {
            "FrameworkName" => {
                input.framework_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_recovery_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRecoveryPointInput, String> {
    let mut input = DeleteRecoveryPointInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_report_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteReportPlanInput, String> {
    let mut input = DeleteReportPlanInput::default();
    for (name, value) in labels {
        match *name {
            "ReportPlanName" => {
                input.report_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_restore_testing_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRestoreTestingPlanInput, String> {
    let mut input = DeleteRestoreTestingPlanInput::default();
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_restore_testing_selection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRestoreTestingSelectionInput, String> {
    let mut input = DeleteRestoreTestingSelectionInput::default();
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            "RestoreTestingSelectionName" => {
                input.restore_testing_selection_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_tiering_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTieringConfigurationInput, String> {
    let mut input = DeleteTieringConfigurationInput::default();
    for (name, value) in labels {
        match *name {
            "TieringConfigurationName" => {
                input.tiering_configuration_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_backup_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBackupJobInput, String> {
    let mut input = DescribeBackupJobInput::default();
    for (name, value) in labels {
        match *name {
            "BackupJobId" => {
                input.backup_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBackupVaultInput, String> {
    let mut input = DescribeBackupVaultInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("backupVaultAccountId") {
        input.backup_vault_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_copy_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCopyJobInput, String> {
    let mut input = DescribeCopyJobInput::default();
    for (name, value) in labels {
        match *name {
            "CopyJobId" => {
                input.copy_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFrameworkInput, String> {
    let mut input = DescribeFrameworkInput::default();
    for (name, value) in labels {
        match *name {
            "FrameworkName" => {
                input.framework_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_global_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeGlobalSettingsInput, String> {
    let input = DescribeGlobalSettingsInput {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_protected_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeProtectedResourceInput, String> {
    let mut input = DescribeProtectedResourceInput::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_recovery_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRecoveryPointInput, String> {
    let mut input = DescribeRecoveryPointInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("backupVaultAccountId") {
        input.backup_vault_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_region_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRegionSettingsInput, String> {
    let input = DescribeRegionSettingsInput {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_report_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeReportJobInput, String> {
    let mut input = DescribeReportJobInput::default();
    for (name, value) in labels {
        match *name {
            "ReportJobId" => {
                input.report_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_report_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeReportPlanInput, String> {
    let mut input = DescribeReportPlanInput::default();
    for (name, value) in labels {
        match *name {
            "ReportPlanName" => {
                input.report_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_restore_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRestoreJobInput, String> {
    let mut input = DescribeRestoreJobInput::default();
    for (name, value) in labels {
        match *name {
            "RestoreJobId" => {
                input.restore_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_scan_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeScanJobInput, String> {
    let mut input = DescribeScanJobInput::default();
    for (name, value) in labels {
        match *name {
            "ScanJobId" => {
                input.scan_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_backup_vault_mpa_approval_team_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateBackupVaultMpaApprovalTeamInput, String> {
    let mut input = DisassociateBackupVaultMpaApprovalTeamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateBackupVaultMpaApprovalTeamInput>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize DisassociateBackupVaultMpaApprovalTeam request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_recovery_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateRecoveryPointInput, String> {
    let mut input = DisassociateRecoveryPointInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_recovery_point_from_parent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateRecoveryPointFromParentInput, String> {
    let mut input = DisassociateRecoveryPointFromParentInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_export_backup_plan_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExportBackupPlanTemplateInput, String> {
    let mut input = ExportBackupPlanTemplateInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backup_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackupPlanInput, String> {
    let mut input = GetBackupPlanInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxScheduledRunsPreview") {
        input.max_scheduled_runs_preview = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backup_plan_from_j_s_o_n_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackupPlanFromJSONInput, String> {
    let mut input = GetBackupPlanFromJSONInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBackupPlanFromJSONInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBackupPlanFromJSON request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backup_plan_from_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackupPlanFromTemplateInput, String> {
    let mut input = GetBackupPlanFromTemplateInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanTemplateId" => {
                input.backup_plan_template_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backup_selection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackupSelectionInput, String> {
    let mut input = GetBackupSelectionInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            "SelectionId" => {
                input.selection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backup_vault_access_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackupVaultAccessPolicyInput, String> {
    let mut input = GetBackupVaultAccessPolicyInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backup_vault_notifications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackupVaultNotificationsInput, String> {
    let mut input = GetBackupVaultNotificationsInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_legal_hold_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLegalHoldInput, String> {
    let mut input = GetLegalHoldInput::default();
    for (name, value) in labels {
        match *name {
            "LegalHoldId" => {
                input.legal_hold_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_recovery_point_index_details_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRecoveryPointIndexDetailsInput, String> {
    let mut input = GetRecoveryPointIndexDetailsInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_recovery_point_restore_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRecoveryPointRestoreMetadataInput, String> {
    let mut input = GetRecoveryPointRestoreMetadataInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("backupVaultAccountId") {
        input.backup_vault_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_restore_job_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRestoreJobMetadataInput, String> {
    let mut input = GetRestoreJobMetadataInput::default();
    for (name, value) in labels {
        match *name {
            "RestoreJobId" => {
                input.restore_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_restore_testing_inferred_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRestoreTestingInferredMetadataInput, String> {
    let mut input = GetRestoreTestingInferredMetadataInput::default();
    if let Some(value) = query.get("BackupVaultAccountId") {
        input.backup_vault_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("BackupVaultName") {
        input.backup_vault_name = value.to_string();
    }
    if let Some(value) = query.get("RecoveryPointArn") {
        input.recovery_point_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_restore_testing_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRestoreTestingPlanInput, String> {
    let mut input = GetRestoreTestingPlanInput::default();
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_restore_testing_selection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRestoreTestingSelectionInput, String> {
    let mut input = GetRestoreTestingSelectionInput::default();
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            "RestoreTestingSelectionName" => {
                input.restore_testing_selection_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tiering_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTieringConfigurationInput, String> {
    let mut input = GetTieringConfigurationInput::default();
    for (name, value) in labels {
        match *name {
            "TieringConfigurationName" => {
                input.tiering_configuration_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backup_job_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackupJobSummariesInput, String> {
    let mut input = ListBackupJobSummariesInput::default();
    if let Some(value) = query.get("AccountId") {
        input.account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("AggregationPeriod") {
        input.aggregation_period = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("MessageCategory") {
        input.message_category = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("ResourceType") {
        input.resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("State") {
        input.state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backup_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackupJobsInput, String> {
    let mut input = ListBackupJobsInput::default();
    if let Some(value) = query.get("accountId") {
        input.by_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("backupVaultName") {
        input.by_backup_vault_name = Some(value.to_string());
    }
    if let Some(value) = query.get("completeAfter") {
        input.by_complete_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("completeBefore") {
        input.by_complete_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdAfter") {
        input.by_created_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdBefore") {
        input.by_created_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("messageCategory") {
        input.by_message_category = Some(value.to_string());
    }
    if let Some(value) = query.get("parentJobId") {
        input.by_parent_job_id = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.by_resource_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.by_resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("state") {
        input.by_state = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backup_plan_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackupPlanTemplatesInput, String> {
    let mut input = ListBackupPlanTemplatesInput::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backup_plan_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackupPlanVersionsInput, String> {
    let mut input = ListBackupPlanVersionsInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backup_plans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackupPlansInput, String> {
    let mut input = ListBackupPlansInput::default();
    if let Some(value) = query.get("includeDeleted") {
        input.include_deleted = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backup_selections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackupSelectionsInput, String> {
    let mut input = ListBackupSelectionsInput::default();
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backup_vaults_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackupVaultsInput, String> {
    let mut input = ListBackupVaultsInput::default();
    if let Some(value) = query.get("shared") {
        input.by_shared = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("vaultType") {
        input.by_vault_type = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_copy_job_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCopyJobSummariesInput, String> {
    let mut input = ListCopyJobSummariesInput::default();
    if let Some(value) = query.get("AccountId") {
        input.account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("AggregationPeriod") {
        input.aggregation_period = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("MessageCategory") {
        input.message_category = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("ResourceType") {
        input.resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("State") {
        input.state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_copy_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCopyJobsInput, String> {
    let mut input = ListCopyJobsInput::default();
    if let Some(value) = query.get("accountId") {
        input.by_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("completeAfter") {
        input.by_complete_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("completeBefore") {
        input.by_complete_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdAfter") {
        input.by_created_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdBefore") {
        input.by_created_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("destinationVaultArn") {
        input.by_destination_vault_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("messageCategory") {
        input.by_message_category = Some(value.to_string());
    }
    if let Some(value) = query.get("parentJobId") {
        input.by_parent_job_id = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.by_resource_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.by_resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("sourceRecoveryPointArn") {
        input.by_source_recovery_point_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("state") {
        input.by_state = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_frameworks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFrameworksInput, String> {
    let mut input = ListFrameworksInput::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_indexed_recovery_points_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIndexedRecoveryPointsInput, String> {
    let mut input = ListIndexedRecoveryPointsInput::default();
    if let Some(value) = query.get("createdAfter") {
        input.created_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdBefore") {
        input.created_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("indexStatus") {
        input.index_status = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("sourceResourceArn") {
        input.source_resource_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_legal_holds_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLegalHoldsInput, String> {
    let mut input = ListLegalHoldsInput::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_protected_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProtectedResourcesInput, String> {
    let mut input = ListProtectedResourcesInput::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_protected_resources_by_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProtectedResourcesByBackupVaultInput, String> {
    let mut input = ListProtectedResourcesByBackupVaultInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("backupVaultAccountId") {
        input.backup_vault_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_recovery_points_by_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRecoveryPointsByBackupVaultInput, String> {
    let mut input = ListRecoveryPointsByBackupVaultInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("backupVaultAccountId") {
        input.backup_vault_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("backupPlanId") {
        input.by_backup_plan_id = Some(value.to_string());
    }
    if let Some(value) = query.get("createdAfter") {
        input.by_created_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdBefore") {
        input.by_created_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("parentRecoveryPointArn") {
        input.by_parent_recovery_point_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.by_resource_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.by_resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_recovery_points_by_legal_hold_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRecoveryPointsByLegalHoldInput, String> {
    let mut input = ListRecoveryPointsByLegalHoldInput::default();
    for (name, value) in labels {
        match *name {
            "LegalHoldId" => {
                input.legal_hold_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_recovery_points_by_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRecoveryPointsByResourceInput, String> {
    let mut input = ListRecoveryPointsByResourceInput::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("managedByAWSBackupOnly") {
        input.managed_by_a_w_s_backup_only = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_report_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReportJobsInput, String> {
    let mut input = ListReportJobsInput::default();
    if let Some(value) = query.get("CreationAfter") {
        input.by_creation_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("CreationBefore") {
        input.by_creation_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("ReportPlanName") {
        input.by_report_plan_name = Some(value.to_string());
    }
    if let Some(value) = query.get("Status") {
        input.by_status = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_report_plans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReportPlansInput, String> {
    let mut input = ListReportPlansInput::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_restore_access_backup_vaults_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRestoreAccessBackupVaultsInput, String> {
    let mut input = ListRestoreAccessBackupVaultsInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_restore_job_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRestoreJobSummariesInput, String> {
    let mut input = ListRestoreJobSummariesInput::default();
    if let Some(value) = query.get("AccountId") {
        input.account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("AggregationPeriod") {
        input.aggregation_period = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("ResourceType") {
        input.resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("State") {
        input.state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_restore_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRestoreJobsInput, String> {
    let mut input = ListRestoreJobsInput::default();
    if let Some(value) = query.get("accountId") {
        input.by_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("completeAfter") {
        input.by_complete_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("completeBefore") {
        input.by_complete_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdAfter") {
        input.by_created_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("createdBefore") {
        input.by_created_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("parentJobId") {
        input.by_parent_job_id = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.by_resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("restoreTestingPlanArn") {
        input.by_restore_testing_plan_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("status") {
        input.by_status = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_restore_jobs_by_protected_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRestoreJobsByProtectedResourceInput, String> {
    let mut input = ListRestoreJobsByProtectedResourceInput::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("recoveryPointCreationDateAfter") {
        input.by_recovery_point_creation_date_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("recoveryPointCreationDateBefore") {
        input.by_recovery_point_creation_date_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("status") {
        input.by_status = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_restore_testing_plans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRestoreTestingPlansInput, String> {
    let mut input = ListRestoreTestingPlansInput::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_restore_testing_selections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRestoreTestingSelectionsInput, String> {
    let mut input = ListRestoreTestingSelectionsInput::default();
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_scan_job_summaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListScanJobSummariesInput, String> {
    let mut input = ListScanJobSummariesInput::default();
    if let Some(value) = query.get("AccountId") {
        input.account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("AggregationPeriod") {
        input.aggregation_period = Some(value.to_string());
    }
    if let Some(value) = query.get("MalwareScanner") {
        input.malware_scanner = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("ResourceType") {
        input.resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("ScanResultStatus") {
        input.scan_result_status = Some(value.to_string());
    }
    if let Some(value) = query.get("State") {
        input.state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_scan_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListScanJobsInput, String> {
    let mut input = ListScanJobsInput::default();
    if let Some(value) = query.get("ByAccountId") {
        input.by_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("ByBackupVaultName") {
        input.by_backup_vault_name = Some(value.to_string());
    }
    if let Some(value) = query.get("ByCompleteAfter") {
        input.by_complete_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("ByCompleteBefore") {
        input.by_complete_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("ByMalwareScanner") {
        input.by_malware_scanner = Some(value.to_string());
    }
    if let Some(value) = query.get("ByRecoveryPointArn") {
        input.by_recovery_point_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("ByResourceArn") {
        input.by_resource_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("ByResourceType") {
        input.by_resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("ByScanResultStatus") {
        input.by_scan_result_status = Some(value.to_string());
    }
    if let Some(value) = query.get("ByState") {
        input.by_state = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsInput, String> {
    let mut input = ListTagsInput::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tiering_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTieringConfigurationsInput, String> {
    let mut input = ListTieringConfigurationsInput::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_backup_vault_access_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBackupVaultAccessPolicyInput, String> {
    let mut input = PutBackupVaultAccessPolicyInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutBackupVaultAccessPolicyInput>(&request.body).map_err(
            |err| format!("failed to deserialize PutBackupVaultAccessPolicy request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_backup_vault_lock_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBackupVaultLockConfigurationInput, String> {
    let mut input = PutBackupVaultLockConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutBackupVaultLockConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutBackupVaultLockConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_backup_vault_notifications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBackupVaultNotificationsInput, String> {
    let mut input = PutBackupVaultNotificationsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutBackupVaultNotificationsInput>(&request.body).map_err(
            |err| format!("failed to deserialize PutBackupVaultNotifications request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_restore_validation_result_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutRestoreValidationResultInput, String> {
    let mut input = PutRestoreValidationResultInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutRestoreValidationResultInput>(&request.body).map_err(
            |err| format!("failed to deserialize PutRestoreValidationResult request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "RestoreJobId" => {
                input.restore_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_revoke_restore_access_backup_vault_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RevokeRestoreAccessBackupVaultInput, String> {
    let mut input = RevokeRestoreAccessBackupVaultInput::default();
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RestoreAccessBackupVaultArn" => {
                input.restore_access_backup_vault_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("requesterComment") {
        input.requester_comment = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_backup_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartBackupJobInput, String> {
    let mut input = StartBackupJobInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartBackupJobInput>(&request.body)
            .map_err(|err| format!("failed to deserialize StartBackupJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_copy_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartCopyJobInput, String> {
    let mut input = StartCopyJobInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartCopyJobInput>(&request.body)
            .map_err(|err| format!("failed to deserialize StartCopyJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_report_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartReportJobInput, String> {
    let mut input = StartReportJobInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartReportJobInput>(&request.body)
            .map_err(|err| format!("failed to deserialize StartReportJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ReportPlanName" => {
                input.report_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_restore_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartRestoreJobInput, String> {
    let mut input = StartRestoreJobInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartRestoreJobInput>(&request.body)
            .map_err(|err| format!("failed to deserialize StartRestoreJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_scan_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartScanJobInput, String> {
    let mut input = StartScanJobInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartScanJobInput>(&request.body)
            .map_err(|err| format!("failed to deserialize StartScanJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_backup_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopBackupJobInput, String> {
    let mut input = StopBackupJobInput::default();
    for (name, value) in labels {
        match *name {
            "BackupJobId" => {
                input.backup_job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceInput, String> {
    let mut input = TagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceInput, String> {
    let mut input = UntagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_backup_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBackupPlanInput, String> {
    let mut input = UpdateBackupPlanInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBackupPlanInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBackupPlan request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BackupPlanId" => {
                input.backup_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_framework_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFrameworkInput, String> {
    let mut input = UpdateFrameworkInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFrameworkInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFramework request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FrameworkName" => {
                input.framework_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_global_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGlobalSettingsInput, String> {
    let mut input = UpdateGlobalSettingsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGlobalSettingsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGlobalSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_recovery_point_index_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRecoveryPointIndexSettingsInput, String> {
    let mut input = UpdateRecoveryPointIndexSettingsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRecoveryPointIndexSettingsInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateRecoveryPointIndexSettings request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_recovery_point_lifecycle_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRecoveryPointLifecycleInput, String> {
    let mut input = UpdateRecoveryPointLifecycleInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRecoveryPointLifecycleInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateRecoveryPointLifecycle request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "BackupVaultName" => {
                input.backup_vault_name = value.to_string();
            }
            "RecoveryPointArn" => {
                input.recovery_point_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_region_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRegionSettingsInput, String> {
    let mut input = UpdateRegionSettingsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRegionSettingsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRegionSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_report_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateReportPlanInput, String> {
    let mut input = UpdateReportPlanInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateReportPlanInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateReportPlan request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ReportPlanName" => {
                input.report_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_restore_testing_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRestoreTestingPlanInput, String> {
    let mut input = UpdateRestoreTestingPlanInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRestoreTestingPlanInput>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateRestoreTestingPlan request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_restore_testing_selection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRestoreTestingSelectionInput, String> {
    let mut input = UpdateRestoreTestingSelectionInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRestoreTestingSelectionInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateRestoreTestingSelection request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "RestoreTestingPlanName" => {
                input.restore_testing_plan_name = value.to_string();
            }
            "RestoreTestingSelectionName" => {
                input.restore_testing_selection_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_tiering_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTieringConfigurationInput, String> {
    let mut input = UpdateTieringConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTieringConfigurationInput>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateTieringConfiguration request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "TieringConfigurationName" => {
                input.tiering_configuration_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
