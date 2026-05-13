use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path, extract_query_string, parse_query_string, percent_decode,
    rest_json_error,
};

use crate::state::{BackupError, BackupState};
use crate::views::BackupStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct BackupService {
    pub(crate) state: Arc<BackendState<BackupState>>,
    pub(crate) notifier: StateChangeNotifier<BackupStateView>,
}

impl BackupService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BackupService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BackupService {
    fn service_name(&self) -> &str {
        "backup"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://backup\..*\.amazonaws\.com",
            r"https?://backup\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl BackupService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        // Validate JSON body up-front; typed deserialisers re-parse per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "BadRequestException", "Invalid JSON body");
        }
        let query = parse_query_string(extract_query_string(&request.uri));

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let response = match (method, segments.as_slice()) {
            // PUT /backup-vaults/{name} - CreateBackupVault
            ("PUT", ["backup-vaults", name]) if !name.is_empty() => {
                let vault_name = percent_decode(name);
                self.handle_create_backup_vault(
                    &state,
                    &vault_name,
                    &region,
                    account_id,
                    &request,
                    &query,
                )
                .await
            }
            // GET /backup-vaults/{name} - DescribeBackupVault
            ("GET", ["backup-vaults", name]) if !name.is_empty() => {
                let vault_name = percent_decode(name);
                self.handle_describe_backup_vault(&state, &vault_name).await
            }
            // DELETE /backup-vaults/{name} - DeleteBackupVault
            ("DELETE", ["backup-vaults", name]) if !name.is_empty() => {
                let vault_name = percent_decode(name);
                self.handle_delete_backup_vault(&state, &vault_name).await
            }
            // GET /backup-vaults - ListBackupVaults
            ("GET", ["backup-vaults"]) => self.handle_list_backup_vaults(&state).await,

            // PUT /backup/plans - CreateBackupPlan
            ("PUT", ["backup", "plans"]) => {
                self.handle_create_backup_plan(&state, &request, &query, &region, account_id)
                    .await
            }
            // GET /backup/plans/{BackupPlanId} - GetBackupPlan
            ("GET", ["backup", "plans", plan_id]) if !plan_id.is_empty() => {
                let plan_id = percent_decode(plan_id);
                self.handle_get_backup_plan(&state, &plan_id).await
            }
            // DELETE /backup/plans/{BackupPlanId} - DeleteBackupPlan
            ("DELETE", ["backup", "plans", plan_id]) if !plan_id.is_empty() => {
                let plan_id = percent_decode(plan_id);
                self.handle_delete_backup_plan(&state, &plan_id).await
            }
            // GET /backup/plans - ListBackupPlans
            ("GET", ["backup", "plans"]) => self.handle_list_backup_plans(&state).await,

            // POST /audit/report-plans - CreateReportPlan
            ("POST", ["audit", "report-plans"]) => {
                self.handle_create_report_plan(&state, &request, &query, &region, account_id)
                    .await
            }
            // GET /audit/report-plans/{ReportPlanName} - DescribeReportPlan
            ("GET", ["audit", "report-plans", name]) if !name.is_empty() => {
                let name = percent_decode(name);
                self.handle_describe_report_plan(&state, &name).await
            }
            // DELETE /audit/report-plans/{ReportPlanName} - DeleteReportPlan
            ("DELETE", ["audit", "report-plans", name]) if !name.is_empty() => {
                let name = percent_decode(name);
                self.handle_delete_report_plan(&state, &name).await
            }
            // GET /audit/report-plans - ListReportPlans
            ("GET", ["audit", "report-plans"]) => self.handle_list_report_plans(&state).await,

            // PUT /backup-vaults/{BackupVaultName}/vault-lock - PutBackupVaultLockConfiguration
            ("PUT", ["backup-vaults", name, "vault-lock"]) if !name.is_empty() => {
                let vault_name = percent_decode(name);
                self.handle_put_backup_vault_lock_configuration(
                    &state,
                    &vault_name,
                    &request,
                    &query,
                )
                .await
            }
            // DELETE /backup-vaults/{BackupVaultName}/vault-lock - DeleteBackupVaultLockConfiguration
            ("DELETE", ["backup-vaults", name, "vault-lock"]) if !name.is_empty() => {
                let vault_name = percent_decode(name);
                self.handle_delete_backup_vault_lock_configuration(&state, &vault_name)
                    .await
            }

            // GET /tags/{ResourceArn} - ListTags
            ("GET", ["tags", rest @ ..]) if !rest.is_empty() => {
                // The resource ARN is URL-encoded and may contain slashes
                let resource_arn = percent_decode(&segments[1..].join("/"));
                self.handle_list_tags(&state, &resource_arn).await
            }
            // POST /tags/{ResourceArn} - TagResource
            ("POST", ["tags", rest @ ..]) if !rest.is_empty() => {
                let resource_arn = percent_decode(&segments[1..].join("/"));
                self.handle_tag_resource(&state, &resource_arn, &request, &query)
                    .await
            }
            // POST /untag/{ResourceArn} - UntagResource
            ("POST", ["untag", rest @ ..]) if !rest.is_empty() => {
                let resource_arn = percent_decode(&segments[1..].join("/"));
                self.handle_untag_resource(&state, &resource_arn, &request, &query)
                    .await
            }

            // PUT /backup-vaults/{BackupVaultName}/mpaApprovalTeam => AssociateBackupVaultMpaApprovalTeam
            ("PUT", ["backup-vaults", name, "mpaApprovalTeam"]) if !name.is_empty() => {
                self.handle_associate_backup_vault_mpa_approval_team().await
            }
            // DELETE /legal-holds/{LegalHoldId} => CancelLegalHold
            ("DELETE", ["legal-holds", id]) => {
                let hold_id = percent_decode(id);
                self.handle_cancel_legal_hold(&state, &hold_id).await
            }
            // PUT /backup/plans/{BackupPlanId}/selections => CreateBackupSelection
            ("PUT", ["backup", "plans", plan_id, "selections"]) => {
                let plan_id = percent_decode(plan_id);
                self.handle_create_backup_selection(&state, &plan_id, &request, &query)
                    .await
            }
            // POST /audit/frameworks => CreateFramework
            ("POST", ["audit", "frameworks"]) => {
                self.handle_create_framework(&state, &request, &query, &region, account_id)
                    .await
            }
            // POST /legal-holds => CreateLegalHold
            ("POST", ["legal-holds"]) => {
                self.handle_create_legal_hold(&state, &request, &query, &region, account_id)
                    .await
            }
            // PUT /logically-air-gapped-backup-vaults/{BackupVaultName} => CreateLogicallyAirGappedBackupVault
            ("PUT", ["logically-air-gapped-backup-vaults", name]) => {
                let vault_name = percent_decode(name);
                self.handle_create_logically_air_gapped_backup_vault(
                    &state,
                    &vault_name,
                    &region,
                    account_id,
                    &request,
                    &query,
                )
                .await
            }
            // PUT /restore-access-backup-vaults => CreateRestoreAccessBackupVault
            ("PUT", ["restore-access-backup-vaults"]) => {
                self.handle_create_restore_access_backup_vault(
                    &state, &request, &query, &region, account_id,
                )
                .await
            }
            // PUT /restore-testing/plans => CreateRestoreTestingPlan
            ("PUT", ["restore-testing", "plans"]) => {
                self.handle_create_restore_testing_plan(
                    &state, &request, &query, &region, account_id,
                )
                .await
            }
            // PUT /restore-testing/plans/{RestoreTestingPlanName}/selections => CreateRestoreTestingSelection
            ("PUT", ["restore-testing", "plans", plan_name, "selections"]) => {
                let plan_name = percent_decode(plan_name);
                self.handle_create_restore_testing_selection(&state, &plan_name, &request, &query)
                    .await
            }
            // PUT /tiering-configurations => CreateTieringConfiguration
            ("PUT", ["tiering-configurations"]) => {
                self.handle_create_tiering_configuration(
                    &state, &request, &query, &region, account_id,
                )
                .await
            }
            // DELETE /backup/plans/{BackupPlanId}/selections/{SelectionId} => DeleteBackupSelection
            ("DELETE", ["backup", "plans", plan_id, "selections", sel_id]) => {
                let plan_id = percent_decode(plan_id);
                let sel_id = percent_decode(sel_id);
                self.handle_delete_backup_selection(&state, &plan_id, &sel_id)
                    .await
            }
            // DELETE /backup-vaults/{BackupVaultName}/access-policy => DeleteBackupVaultAccessPolicy
            ("DELETE", ["backup-vaults", name, "access-policy"]) => {
                let vault_name = percent_decode(name);
                self.handle_delete_backup_vault_access_policy(&state, &vault_name)
                    .await
            }
            // DELETE /backup-vaults/{BackupVaultName}/notification-configuration => DeleteBackupVaultNotifications
            ("DELETE", ["backup-vaults", name, "notification-configuration"]) => {
                let vault_name = percent_decode(name);
                self.handle_delete_backup_vault_notifications(&state, &vault_name)
                    .await
            }
            // DELETE /audit/frameworks/{FrameworkName} => DeleteFramework
            ("DELETE", ["audit", "frameworks", name]) => {
                let fw_name = percent_decode(name);
                self.handle_delete_framework(&state, &fw_name).await
            }
            // DELETE /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn} => DeleteRecoveryPoint
            ("DELETE", ["backup-vaults", vault, "recovery-points", rp]) => {
                let vault_name = percent_decode(vault);
                let rp_arn = percent_decode(rp);
                self.handle_delete_recovery_point(&state, &vault_name, &rp_arn)
                    .await
            }
            // DELETE /restore-testing/plans/{RestoreTestingPlanName} => DeleteRestoreTestingPlan
            ("DELETE", ["restore-testing", "plans", plan_name]) => {
                let plan_name = percent_decode(plan_name);
                self.handle_delete_restore_testing_plan(&state, &plan_name)
                    .await
            }
            // DELETE /restore-testing/plans/{RestoreTestingPlanName}/selections/{RestoreTestingSelectionName} => DeleteRestoreTestingSelection
            (
                "DELETE",
                [
                    "restore-testing",
                    "plans",
                    plan_name,
                    "selections",
                    sel_name,
                ],
            ) => {
                let plan_name = percent_decode(plan_name);
                let sel_name = percent_decode(sel_name);
                self.handle_delete_restore_testing_selection(&state, &plan_name, &sel_name)
                    .await
            }
            // DELETE /tiering-configurations/{TieringConfigurationName} => DeleteTieringConfiguration
            ("DELETE", ["tiering-configurations", name]) => {
                let config_name = percent_decode(name);
                self.handle_delete_tiering_configuration(&state, &config_name)
                    .await
            }
            // GET /backup-jobs/{BackupJobId} => DescribeBackupJob
            ("GET", ["backup-jobs", id]) => {
                let job_id = percent_decode(id);
                self.handle_describe_backup_job(&state, &job_id).await
            }
            // GET /copy-jobs/{CopyJobId} => DescribeCopyJob
            ("GET", ["copy-jobs", id]) => {
                let copy_job_id = percent_decode(id);
                self.handle_describe_copy_job(&state, &copy_job_id).await
            }
            // GET /audit/frameworks/{FrameworkName} => DescribeFramework
            ("GET", ["audit", "frameworks", name]) => {
                let fw_name = percent_decode(name);
                self.handle_describe_framework(&state, &fw_name).await
            }
            // GET /global-settings => DescribeGlobalSettings
            ("GET", ["global-settings"]) => self.handle_describe_global_settings(&state).await,
            // GET /resources/{ResourceArn} => DescribeProtectedResource
            ("GET", ["resources", rest @ ..]) if rest.len() == 1 => {
                let resource_arn = percent_decode(&segments[1..].join("/"));
                self.handle_describe_protected_resource(&state, &resource_arn)
                    .await
            }
            // GET /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn} => DescribeRecoveryPoint
            ("GET", ["backup-vaults", vault, "recovery-points", rp]) => {
                let vault_name = percent_decode(vault);
                let rp_arn = percent_decode(rp);
                self.handle_describe_recovery_point(&state, &vault_name, &rp_arn)
                    .await
            }
            // GET /account-settings => DescribeRegionSettings
            ("GET", ["account-settings"]) => self.handle_describe_region_settings(&state).await,
            // GET /audit/report-jobs/{ReportJobId} => DescribeReportJob
            ("GET", ["audit", "report-jobs", id]) => {
                let job_id = percent_decode(id);
                self.handle_describe_report_job(&state, &job_id).await
            }
            // GET /restore-jobs/{RestoreJobId} => DescribeRestoreJob
            ("GET", ["restore-jobs", id]) => {
                let job_id = percent_decode(id);
                self.handle_describe_restore_job(&state, &job_id).await
            }
            // GET /scan/jobs/{ScanJobId} => DescribeScanJob
            ("GET", ["scan", "jobs", id]) => {
                let scan_job_id = percent_decode(id);
                self.handle_describe_scan_job(&state, &scan_job_id).await
            }
            // POST /backup-vaults/{BackupVaultName}/mpaApprovalTeam?delete => DisassociateBackupVaultMpaApprovalTeam
            ("POST", ["backup-vaults", _name, "mpaApprovalTeam"]) => {
                self.handle_disassociate_backup_vault_mpa_approval_team()
                    .await
            }
            // POST /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/disassociate => DisassociateRecoveryPoint
            (
                "POST",
                [
                    "backup-vaults",
                    _vault,
                    "recovery-points",
                    _rp,
                    "disassociate",
                ],
            ) => self.handle_disassociate_recovery_point().await,
            // DELETE /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/parentAssociation => DisassociateRecoveryPointFromParent
            (
                "DELETE",
                [
                    "backup-vaults",
                    _vault,
                    "recovery-points",
                    _rp,
                    "parentAssociation",
                ],
            ) => self.handle_disassociate_recovery_point_from_parent().await,
            // GET /backup/plans/{BackupPlanId}/toTemplate => ExportBackupPlanTemplate
            ("GET", ["backup", "plans", id, "toTemplate"]) => {
                let plan_id = percent_decode(id);
                self.handle_export_backup_plan_template(&state, &plan_id)
                    .await
            }
            // POST /backup/template/json/toPlan => GetBackupPlanFromJSON
            ("POST", ["backup", "template", "json", "toPlan"]) => {
                self.handle_get_backup_plan_from_j_s_o_n(&request, &query)
                    .await
            }
            // GET /backup/template/plans/{BackupPlanTemplateId}/toPlan => GetBackupPlanFromTemplate
            ("GET", ["backup", "template", "plans", _id, "toPlan"]) => {
                self.handle_get_backup_plan_from_template().await
            }
            // GET /backup/plans/{BackupPlanId}/selections/{SelectionId} => GetBackupSelection
            ("GET", ["backup", "plans", plan_id, "selections", sel_id]) => {
                let plan_id = percent_decode(plan_id);
                let sel_id = percent_decode(sel_id);
                self.handle_get_backup_selection(&state, &plan_id, &sel_id)
                    .await
            }
            // GET /backup-vaults/{BackupVaultName}/access-policy => GetBackupVaultAccessPolicy
            ("GET", ["backup-vaults", name, "access-policy"]) => {
                let vault_name = percent_decode(name);
                self.handle_get_backup_vault_access_policy(&state, &vault_name)
                    .await
            }
            // GET /backup-vaults/{BackupVaultName}/notification-configuration => GetBackupVaultNotifications
            ("GET", ["backup-vaults", name, "notification-configuration"]) => {
                let vault_name = percent_decode(name);
                self.handle_get_backup_vault_notifications(&state, &vault_name)
                    .await
            }
            // GET /legal-holds/{LegalHoldId} => GetLegalHold
            ("GET", ["legal-holds", id]) => {
                let hold_id = percent_decode(id);
                self.handle_get_legal_hold(&state, &hold_id).await
            }
            // GET /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/index => GetRecoveryPointIndexDetails
            ("GET", ["backup-vaults", vault, "recovery-points", rp, "index"]) => {
                let vault_name = percent_decode(vault);
                let rp_arn = percent_decode(rp);
                self.handle_get_recovery_point_index_details(&state, &vault_name, &rp_arn)
                    .await
            }
            // GET /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/restore-metadata => GetRecoveryPointRestoreMetadata
            (
                "GET",
                [
                    "backup-vaults",
                    vault,
                    "recovery-points",
                    rp,
                    "restore-metadata",
                ],
            ) => {
                let vault_name = percent_decode(vault);
                let rp_arn = percent_decode(rp);
                self.handle_get_recovery_point_restore_metadata(&state, &vault_name, &rp_arn)
                    .await
            }
            // GET /restore-jobs/{RestoreJobId}/metadata => GetRestoreJobMetadata
            ("GET", ["restore-jobs", id, "metadata"]) => {
                let job_id = percent_decode(id);
                self.handle_get_restore_job_metadata(&state, &job_id).await
            }
            // GET /restore-testing/inferred-metadata => GetRestoreTestingInferredMetadata
            ("GET", ["restore-testing", "inferred-metadata"]) => {
                self.handle_get_restore_testing_inferred_metadata().await
            }
            // GET /restore-testing/plans/{RestoreTestingPlanName} => GetRestoreTestingPlan
            ("GET", ["restore-testing", "plans", name]) => {
                let plan_name = percent_decode(name);
                self.handle_get_restore_testing_plan(&state, &plan_name)
                    .await
            }
            // GET /restore-testing/plans/{RestoreTestingPlanName}/selections/{RestoreTestingSelectionName} => GetRestoreTestingSelection
            (
                "GET",
                [
                    "restore-testing",
                    "plans",
                    plan_name,
                    "selections",
                    sel_name,
                ],
            ) => {
                let plan_name = percent_decode(plan_name);
                let sel_name = percent_decode(sel_name);
                self.handle_get_restore_testing_selection(&state, &plan_name, &sel_name)
                    .await
            }
            // GET /supported-resource-types => GetSupportedResourceTypes
            ("GET", ["supported-resource-types"]) => {
                self.handle_get_supported_resource_types().await
            }
            // GET /tiering-configurations/{TieringConfigurationName} => GetTieringConfiguration
            ("GET", ["tiering-configurations", name]) => {
                let config_name = percent_decode(name);
                self.handle_get_tiering_configuration(&state, &config_name)
                    .await
            }
            // GET /audit/backup-job-summaries => ListBackupJobSummaries
            ("GET", ["audit", "backup-job-summaries"]) => {
                self.handle_list_backup_job_summaries(&state, account_id, &region)
                    .await
            }
            // GET /backup-jobs => ListBackupJobs
            ("GET", ["backup-jobs"]) => self.handle_list_backup_jobs(&state).await,
            // GET /backup/template/plans => ListBackupPlanTemplates
            ("GET", ["backup", "template", "plans"]) => {
                self.handle_list_backup_plan_templates(&state).await
            }
            // GET /backup/plans/{BackupPlanId}/versions => ListBackupPlanVersions
            ("GET", ["backup", "plans", id, "versions"]) => {
                let plan_id = percent_decode(id);
                self.handle_list_backup_plan_versions(&state, &plan_id)
                    .await
            }
            // GET /backup/plans/{BackupPlanId}/selections => ListBackupSelections
            ("GET", ["backup", "plans", plan_id, "selections"]) => {
                let plan_id = percent_decode(plan_id);
                self.handle_list_backup_selections(&state, &plan_id).await
            }
            // GET /audit/copy-job-summaries => ListCopyJobSummaries
            ("GET", ["audit", "copy-job-summaries"]) => {
                self.handle_list_copy_job_summaries(&state).await
            }
            // GET /copy-jobs => ListCopyJobs
            ("GET", ["copy-jobs"]) => self.handle_list_copy_jobs(&state).await,
            // GET /audit/frameworks => ListFrameworks
            ("GET", ["audit", "frameworks"]) => self.handle_list_frameworks(&state).await,
            // GET /indexes/recovery-point => ListIndexedRecoveryPoints
            ("GET", ["indexes", "recovery-point"]) => {
                self.handle_list_indexed_recovery_points(&state).await
            }
            // GET /legal-holds => ListLegalHolds
            ("GET", ["legal-holds"]) => self.handle_list_legal_holds(&state).await,
            // GET /resources => ListProtectedResources
            ("GET", ["resources"]) => self.handle_list_protected_resources(&state).await,
            // GET /backup-vaults/{BackupVaultName}/resources => ListProtectedResourcesByBackupVault
            ("GET", ["backup-vaults", name, "resources"]) => {
                let vault_name = percent_decode(name);
                self.handle_list_protected_resources_by_backup_vault(&state, &vault_name)
                    .await
            }
            // GET /backup-vaults/{BackupVaultName}/recovery-points => ListRecoveryPointsByBackupVault
            ("GET", ["backup-vaults", name, "recovery-points"]) => {
                let vault_name = percent_decode(name);
                self.handle_list_recovery_points_by_backup_vault(&state, &vault_name)
                    .await
            }
            // GET /legal-holds/{LegalHoldId}/recovery-points => ListRecoveryPointsByLegalHold
            ("GET", ["legal-holds", id, "recovery-points"]) => {
                let hold_id = percent_decode(id);
                self.handle_list_recovery_points_by_legal_hold(&state, &hold_id)
                    .await
            }
            // GET /resources/{ResourceArn}/recovery-points => ListRecoveryPointsByResource
            ("GET", ["resources", rest @ ..])
                if rest.len() >= 2 && rest.last() == Some(&"recovery-points") =>
            {
                // Resource ARN is everything between /resources/ and /recovery-points
                let resource_segments = &segments[1..segments.len() - 1];
                let resource_arn = percent_decode(&resource_segments.join("/"));
                self.handle_list_recovery_points_by_resource(&state, &resource_arn)
                    .await
            }
            // GET /audit/report-jobs => ListReportJobs
            ("GET", ["audit", "report-jobs"]) => {
                self.handle_list_report_jobs(&state, &request.uri).await
            }
            // GET /logically-air-gapped-backup-vaults/{BackupVaultName}/restore-access-backup-vaults => ListRestoreAccessBackupVaults
            (
                "GET",
                [
                    "logically-air-gapped-backup-vaults",
                    _name,
                    "restore-access-backup-vaults",
                ],
            ) => self.handle_list_restore_access_backup_vaults().await,
            // GET /audit/restore-job-summaries => ListRestoreJobSummaries
            ("GET", ["audit", "restore-job-summaries"]) => {
                self.handle_list_restore_job_summaries(&state, account_id, &region)
                    .await
            }
            // GET /restore-jobs => ListRestoreJobs
            ("GET", ["restore-jobs"]) => self.handle_list_restore_jobs(&state).await,
            // GET /resources/{ResourceArn}/restore-jobs => ListRestoreJobsByProtectedResource
            ("GET", ["resources", rest @ ..])
                if rest.len() >= 2 && rest.last() == Some(&"restore-jobs") =>
            {
                let resource_segments = &segments[1..segments.len() - 1];
                let resource_arn = percent_decode(&resource_segments.join("/"));
                self.handle_list_restore_jobs_by_protected_resource(&state, &resource_arn)
                    .await
            }
            // GET /restore-testing/plans => ListRestoreTestingPlans
            ("GET", ["restore-testing", "plans"]) => {
                self.handle_list_restore_testing_plans(&state).await
            }
            // GET /restore-testing/plans/{RestoreTestingPlanName}/selections => ListRestoreTestingSelections
            ("GET", ["restore-testing", "plans", plan_name, "selections"]) => {
                let plan_name = percent_decode(plan_name);
                self.handle_list_restore_testing_selections(&state, &plan_name)
                    .await
            }
            // GET /audit/scan-job-summaries => ListScanJobSummaries
            ("GET", ["audit", "scan-job-summaries"]) => {
                self.handle_list_scan_job_summaries(&state).await
            }
            // GET /scan/jobs => ListScanJobs
            ("GET", ["scan", "jobs"]) => self.handle_list_scan_jobs(&state).await,
            // GET /tiering-configurations => ListTieringConfigurations
            ("GET", ["tiering-configurations"]) => {
                self.handle_list_tiering_configurations(&state).await
            }
            // PUT /backup-vaults/{BackupVaultName}/access-policy => PutBackupVaultAccessPolicy
            ("PUT", ["backup-vaults", name, "access-policy"]) => {
                let vault_name = percent_decode(name);
                self.handle_put_backup_vault_access_policy(
                    &state,
                    &vault_name,
                    &region,
                    account_id,
                    &request,
                    &query,
                )
                .await
            }
            // PUT /backup-vaults/{BackupVaultName}/notification-configuration => PutBackupVaultNotifications
            ("PUT", ["backup-vaults", name, "notification-configuration"]) => {
                let vault_name = percent_decode(name);
                self.handle_put_backup_vault_notifications(
                    &state,
                    &vault_name,
                    &region,
                    account_id,
                    &request,
                    &query,
                )
                .await
            }
            // PUT /restore-jobs/{RestoreJobId}/validations => PutRestoreValidationResult
            ("PUT", ["restore-jobs", id, "validations"]) => {
                let job_id = percent_decode(id);
                self.handle_put_restore_validation_result(&state, &job_id, &request, &query)
                    .await
            }
            // DELETE /logically-air-gapped-backup-vaults/{BackupVaultName}/restore-access-backup-vaults/{RestoreAccessBackupVaultArn} => RevokeRestoreAccessBackupVault
            (
                "DELETE",
                [
                    "logically-air-gapped-backup-vaults",
                    _vault,
                    "restore-access-backup-vaults",
                    _arn,
                ],
            ) => self.handle_revoke_restore_access_backup_vault().await, // No state needed - passthrough
            // PUT /backup-jobs => StartBackupJob
            ("PUT", ["backup-jobs"]) => {
                self.handle_start_backup_job(&state, &request, &query, &region, account_id)
                    .await
            }
            // PUT /copy-jobs => StartCopyJob
            ("PUT", ["copy-jobs"]) => {
                self.handle_start_copy_job(&state, &request, &query, &region, account_id)
                    .await
            }
            // POST /audit/report-jobs/{ReportPlanName} => StartReportJob
            ("POST", ["audit", "report-jobs", name]) => {
                let plan_name = percent_decode(name);
                self.handle_start_report_job(&state, &plan_name, &region, account_id)
                    .await
            }
            // PUT /restore-jobs => StartRestoreJob
            ("PUT", ["restore-jobs"]) => {
                self.handle_start_restore_job(&state, &request, &query, account_id)
                    .await
            }
            // PUT /scan/job => StartScanJob
            ("PUT", ["scan", "job"]) => {
                self.handle_start_scan_job(&state, &request, &query, &region, account_id)
                    .await
            }
            // POST /backup-jobs/{BackupJobId} => StopBackupJob
            ("POST", ["backup-jobs", id]) => {
                let job_id = percent_decode(id);
                self.handle_stop_backup_job(&state, &job_id).await
            }
            // POST /backup/plans/{BackupPlanId} => UpdateBackupPlan
            ("POST", ["backup", "plans", id]) => {
                let plan_id = percent_decode(id);
                self.handle_update_backup_plan(&state, &plan_id, &request, &query)
                    .await
            }
            // PUT /audit/frameworks/{FrameworkName} => UpdateFramework
            ("PUT", ["audit", "frameworks", name]) => {
                let fw_name = percent_decode(name);
                self.handle_update_framework(&state, &fw_name, &request, &query)
                    .await
            }
            // PUT /global-settings => UpdateGlobalSettings
            ("PUT", ["global-settings"]) => {
                self.handle_update_global_settings(&state, &request, &query)
                    .await
            }
            // POST /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/index => UpdateRecoveryPointIndexSettings
            ("POST", ["backup-vaults", vault, "recovery-points", rp, "index"]) => {
                let vault_name = percent_decode(vault);
                let rp_arn = percent_decode(rp);
                self.handle_update_recovery_point_index_settings(
                    &state,
                    &vault_name,
                    &rp_arn,
                    &request,
                    &query,
                )
                .await
            }
            // POST /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn} => UpdateRecoveryPointLifecycle
            ("POST", ["backup-vaults", vault, "recovery-points", rp]) => {
                let vault_name = percent_decode(vault);
                let rp_arn = percent_decode(rp);
                self.handle_update_recovery_point_lifecycle(
                    &state,
                    &vault_name,
                    &rp_arn,
                    &request,
                    &query,
                )
                .await
            }
            // PUT /account-settings => UpdateRegionSettings
            ("PUT", ["account-settings"]) => {
                self.handle_update_region_settings(&state, &request, &query)
                    .await
            }
            // PUT /audit/report-plans/{ReportPlanName} => UpdateReportPlan
            ("PUT", ["audit", "report-plans", name]) => {
                let plan_name = percent_decode(name);
                self.handle_update_report_plan(&state, &plan_name, &request, &query)
                    .await
            }
            // PUT /restore-testing/plans/{RestoreTestingPlanName} => UpdateRestoreTestingPlan
            ("PUT", ["restore-testing", "plans", name]) => {
                let plan_name = percent_decode(name);
                self.handle_update_restore_testing_plan(&state, &plan_name, &request, &query)
                    .await
            }
            // PUT /restore-testing/plans/{RestoreTestingPlanName}/selections/{RestoreTestingSelectionName} => UpdateRestoreTestingSelection
            (
                "PUT",
                [
                    "restore-testing",
                    "plans",
                    plan_name,
                    "selections",
                    sel_name,
                ],
            ) => {
                let plan_name = percent_decode(plan_name);
                let sel_name = percent_decode(sel_name);
                self.handle_update_restore_testing_selection(
                    &state, &plan_name, &sel_name, &request, &query,
                )
                .await
            }
            // PUT /tiering-configurations/{TieringConfigurationName} => UpdateTieringConfiguration
            ("PUT", ["tiering-configurations", name]) => {
                let config_name = percent_decode(name);
                self.handle_update_tiering_configuration(&state, &config_name, &request, &query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_backup_vault(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        region: &str,
        account_id: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("BackupVaultName", vault_name)];
        let input = match wire::deserialize_create_backup_vault_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let arn = format!("arn:aws:backup:{region}:{account_id}:backup-vault:{vault_name}");

        let tags: HashMap<String, String> = input.backup_vault_tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_backup_vault(vault_name, &arn, tags) {
            Ok(vault) => {
                let creation_epoch = vault.creation_date.timestamp() as f64
                    + (vault.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_backup_vault_response(&wire::CreateBackupVaultOutput {
                    backup_vault_name: Some(vault.backup_vault_name.clone()),
                    backup_vault_arn: Some(vault.backup_vault_arn.clone()),
                    creation_date: Some(creation_epoch),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_backup_vault(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_backup_vault(vault_name) {
            Ok(vault) => {
                let creation_epoch = vault.creation_date.timestamp() as f64
                    + (vault.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_describe_backup_vault_response(&wire::DescribeBackupVaultOutput {
                    backup_vault_name: Some(vault.backup_vault_name.clone()),
                    backup_vault_arn: Some(vault.backup_vault_arn.clone()),
                    creation_date: Some(creation_epoch),
                    number_of_recovery_points: Some(vault.number_of_recovery_points),
                    locked: Some(vault.locked),
                    min_retention_days: vault.min_retention_days,
                    max_retention_days: vault.max_retention_days,
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_backup_vault(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_backup_vault(vault_name) {
            Ok(()) => wire::serialize_delete_backup_vault_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_list_backup_vaults(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let vaults = state.list_backup_vaults();
        let entries: Vec<wire::BackupVaultListMember> = vaults
            .iter()
            .map(|v| {
                let creation_epoch = v.creation_date.timestamp() as f64
                    + (v.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let lock_date_epoch = v.lock_date.map(|ld| {
                    ld.timestamp() as f64 + (ld.timestamp_subsec_millis() as f64 / 1000.0)
                });
                wire::BackupVaultListMember {
                    backup_vault_name: Some(v.backup_vault_name.clone()),
                    backup_vault_arn: Some(v.backup_vault_arn.clone()),
                    creation_date: Some(creation_epoch),
                    number_of_recovery_points: Some(v.number_of_recovery_points),
                    locked: if v.locked { Some(true) } else { None },
                    min_retention_days: v.min_retention_days,
                    max_retention_days: v.max_retention_days,
                    lock_date: lock_date_epoch,
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_backup_vaults_response(&wire::ListBackupVaultsOutput {
            backup_vault_list: Some(entries),
            ..Default::default()
        })
    }

    // --- Backup Plan handlers ---

    async fn handle_create_backup_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        // Inspect raw body to preserve original "missing field" error messages.
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        if raw.get("BackupPlan").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'BackupPlan'",
            );
        }
        let input = match wire::deserialize_create_backup_plan_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.backup_plan.backup_plan_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'BackupPlanName'",
            );
        }
        let plan_name = input.backup_plan.backup_plan_name.clone();
        let backup_plan_value = serde_json::to_value(&input.backup_plan).unwrap_or(Value::Null);

        let tags: HashMap<String, String> = input.backup_plan_tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_backup_plan(&plan_name, &backup_plan_value, region, account_id, tags) {
            Ok(plan) => {
                let creation_epoch = plan.creation_date.timestamp() as f64
                    + (plan.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_backup_plan_response(&wire::CreateBackupPlanOutput {
                    backup_plan_id: Some(plan.backup_plan_id.clone()),
                    backup_plan_arn: Some(plan.backup_plan_arn.clone()),
                    creation_date: Some(creation_epoch),
                    version_id: Some(plan.version_id.clone()),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_backup_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_backup_plan(plan_id) {
            Ok(plan) => {
                let creation_epoch = plan.creation_date.timestamp() as f64
                    + (plan.creation_date.timestamp_subsec_millis() as f64 / 1000.0);

                // Build BackupPlan wire type from stored JSON
                let rules_json = plan
                    .backup_plan_json
                    .get("Rules")
                    .cloned()
                    .unwrap_or(Value::Array(vec![]));
                let rules: Vec<wire::BackupRule> = if let Value::Array(arr) = &rules_json {
                    arr.iter()
                        .map(|r| wire::BackupRule {
                            rule_name: r
                                .get("RuleName")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            target_backup_vault_name: r
                                .get("TargetBackupVaultName")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            schedule_expression: r
                                .get("ScheduleExpression")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            rule_id: r
                                .get("RuleId")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            start_window_minutes: r
                                .get("StartWindowMinutes")
                                .and_then(|v| v.as_i64()),
                            completion_window_minutes: r
                                .get("CompletionWindowMinutes")
                                .and_then(|v| v.as_i64()),
                            ..Default::default()
                        })
                        .collect()
                } else {
                    vec![]
                };

                let backup_plan_wire = wire::BackupPlan {
                    backup_plan_name: Some(plan.backup_plan_name.clone()),
                    rules: Some(rules),
                    ..Default::default()
                };

                wire::serialize_get_backup_plan_response(&wire::GetBackupPlanOutput {
                    backup_plan: Some(backup_plan_wire),
                    backup_plan_id: Some(plan.backup_plan_id.clone()),
                    backup_plan_arn: Some(plan.backup_plan_arn.clone()),
                    creation_date: Some(creation_epoch),
                    version_id: Some(plan.version_id.clone()),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_backup_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_backup_plan(plan_id) {
            Ok(plan) => {
                let deletion_epoch = chrono::Utc::now().timestamp() as f64;
                wire::serialize_delete_backup_plan_response(&wire::DeleteBackupPlanOutput {
                    backup_plan_id: Some(plan.backup_plan_id),
                    backup_plan_arn: Some(plan.backup_plan_arn),
                    deletion_date: Some(deletion_epoch),
                    version_id: Some(plan.version_id),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_list_backup_plans(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let plans = state.list_backup_plans();
        let entries: Vec<wire::BackupPlansListMember> = plans
            .iter()
            .map(|p| {
                let creation_epoch = p.creation_date.timestamp() as f64
                    + (p.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::BackupPlansListMember {
                    backup_plan_id: Some(p.backup_plan_id.clone()),
                    backup_plan_arn: Some(p.backup_plan_arn.clone()),
                    backup_plan_name: Some(p.backup_plan_name.clone()),
                    creation_date: Some(creation_epoch),
                    version_id: Some(p.version_id.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_backup_plans_response(&wire::ListBackupPlansOutput {
            backup_plans_list: Some(entries),
            ..Default::default()
        })
    }

    // --- Report Plan handlers ---

    async fn handle_create_report_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        if raw.get("ReportPlanName").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'ReportPlanName'",
            );
        }
        if raw.get("ReportDeliveryChannel").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'ReportDeliveryChannel'",
            );
        }
        if raw.get("ReportSetting").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'ReportSetting'",
            );
        }
        let input = match wire::deserialize_create_report_plan_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };

        let report_plan_name = input.report_plan_name.as_str();
        let report_delivery_channel =
            serde_json::to_value(&input.report_delivery_channel).unwrap_or(Value::Null);
        let report_setting = serde_json::to_value(&input.report_setting).unwrap_or(Value::Null);
        let description = input.report_plan_description.as_deref().unwrap_or("");
        let tags: HashMap<String, String> = input.report_plan_tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_report_plan(
            report_plan_name,
            description,
            &report_delivery_channel,
            &report_setting,
            region,
            account_id,
            tags,
        ) {
            Ok(plan) => {
                let creation_epoch = plan.creation_time.timestamp() as f64
                    + (plan.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_report_plan_response(&wire::CreateReportPlanOutput {
                    report_plan_name: Some(plan.report_plan_name.clone()),
                    report_plan_arn: Some(plan.report_plan_arn.clone()),
                    creation_time: Some(creation_epoch),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_report_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_report_plan(name) {
            Ok(plan) => {
                let creation_epoch = plan.creation_time.timestamp() as f64
                    + (plan.creation_time.timestamp_subsec_millis() as f64 / 1000.0);

                let dc = &plan.report_delivery_channel;
                let delivery_channel = wire::ReportDeliveryChannel {
                    s3_bucket_name: dc
                        .get("S3BucketName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    s3_key_prefix: dc
                        .get("S3KeyPrefix")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    formats: dc.get("Formats").and_then(|v| {
                        v.as_array().map(|arr| {
                            arr.iter()
                                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                    }),
                };

                let rs = &plan.report_setting;
                let report_setting = wire::ReportSetting {
                    report_template: rs
                        .get("ReportTemplate")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    ..Default::default()
                };

                wire::serialize_describe_report_plan_response(&wire::DescribeReportPlanOutput {
                    report_plan: Some(wire::ReportPlan {
                        report_plan_name: Some(plan.report_plan_name.clone()),
                        report_plan_arn: Some(plan.report_plan_arn.clone()),
                        report_plan_description: Some(plan.report_plan_description.clone()),
                        report_delivery_channel: Some(delivery_channel),
                        report_setting: Some(report_setting),
                        creation_time: Some(creation_epoch),
                        deployment_status: Some(plan.deployment_status.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_report_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_report_plan(name) {
            Ok(()) => wire::serialize_delete_report_plan_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_list_report_plans(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let plans = state.list_report_plans();
        let entries: Vec<wire::ReportPlan> = plans
            .iter()
            .map(|p| {
                let creation_epoch = p.creation_time.timestamp() as f64
                    + (p.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::ReportPlan {
                    report_plan_name: Some(p.report_plan_name.clone()),
                    report_plan_arn: Some(p.report_plan_arn.clone()),
                    report_plan_description: Some(p.report_plan_description.clone()),
                    creation_time: Some(creation_epoch),
                    deployment_status: Some(p.deployment_status.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_report_plans_response(&wire::ListReportPlansOutput {
            report_plans: Some(entries),
            ..Default::default()
        })
    }

    // --- Vault Lock handlers ---

    async fn handle_put_backup_vault_lock_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("BackupVaultName", vault_name)];
        let input = match wire::deserialize_put_backup_vault_lock_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let min_retention_days = input.min_retention_days;
        let max_retention_days = input.max_retention_days;

        let mut state = state.write().await;
        match state.put_backup_vault_lock_configuration(
            vault_name,
            min_retention_days,
            max_retention_days,
        ) {
            Ok(()) => wire::serialize_put_backup_vault_lock_configuration_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_backup_vault_lock_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_backup_vault_lock_configuration(vault_name) {
            Ok(()) => wire::serialize_delete_backup_vault_lock_configuration_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    // --- Tag handlers ---

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags(resource_arn);
        wire::serialize_list_tags_response(&wire::ListTagsOutput {
            tags: if tags.is_empty() { None } else { Some(tags) },
            ..Default::default()
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        resource_arn: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn)];
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let tags: HashMap<String, String> = input.tags;

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        resource_arn: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn)];
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let tag_keys: Vec<String> = input.tag_key_list;

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    // --- Stub handlers for remaining operations ---

    // MPA approval team association is a no-op in mock (approval workflow)
    async fn handle_associate_backup_vault_mpa_approval_team(&self) -> MockResponse {
        wire::serialize_associate_backup_vault_mpa_approval_team_response()
    }

    async fn handle_cancel_legal_hold(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        legal_hold_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.cancel_legal_hold(legal_hold_id) {
            Ok(()) => wire::serialize_cancel_legal_hold_response(&wire::CancelLegalHoldOutput {}),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_backup_selection(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        if raw.get("BackupSelection").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'BackupSelection'",
            );
        }
        let labels: &[(&str, &str)] = &[("BackupPlanId", plan_id)];
        let input = match wire::deserialize_create_backup_selection_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.backup_selection.selection_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'SelectionName'",
            );
        }
        let selection_name = input.backup_selection.selection_name.clone();
        let iam_role_arn = input.backup_selection.iam_role_arn.clone();
        let resources: Vec<String> = input.backup_selection.resources.clone().unwrap_or_default();
        let backup_selection_value =
            serde_json::to_value(&input.backup_selection).unwrap_or(Value::Null);

        let mut state = state.write().await;
        match state.create_backup_selection(
            plan_id,
            &selection_name,
            &iam_role_arn,
            resources,
            backup_selection_value,
        ) {
            Ok(sel) => {
                let creation_epoch = sel.creation_date.timestamp() as f64
                    + (sel.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_backup_selection_response(
                    &wire::CreateBackupSelectionOutput {
                        backup_plan_id: Some(sel.backup_plan_id.clone()),
                        selection_id: Some(sel.selection_id.clone()),
                        creation_date: Some(creation_epoch),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_framework(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_framework_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.framework_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'FrameworkName'",
            );
        }
        let name = input.framework_name.clone();
        let description = input.framework_description.clone().unwrap_or_default();
        let controls =
            serde_json::to_value(&input.framework_controls).unwrap_or(Value::Array(vec![]));
        let tags: HashMap<String, String> = input.framework_tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_framework(&name, &description, controls, region, account_id, tags) {
            Ok(fw) => wire::serialize_create_framework_response(&wire::CreateFrameworkOutput {
                framework_name: Some(fw.framework_name.clone()),
                framework_arn: Some(fw.framework_arn.clone()),
            }),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_legal_hold(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_legal_hold_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let title = input.title.clone();
        let description = input.description.clone();
        let recovery_point_selection = input
            .recovery_point_selection
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null))
            .unwrap_or(Value::Null);
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_legal_hold(
            &title,
            &description,
            recovery_point_selection,
            region,
            account_id,
            tags,
        ) {
            Ok(hold) => {
                let creation_epoch = hold.creation_date.timestamp() as f64
                    + (hold.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let rps: Option<wire::RecoveryPointSelection> =
                    serde_json::from_value(hold.recovery_point_selection.clone()).ok();
                wire::serialize_create_legal_hold_response(&wire::CreateLegalHoldOutput {
                    legal_hold_id: Some(hold.legal_hold_id.clone()),
                    legal_hold_arn: Some(hold.legal_hold_arn.clone()),
                    title: Some(hold.title.clone()),
                    description: Some(hold.description.clone()),
                    status: Some(hold.status.clone()),
                    creation_date: Some(creation_epoch),
                    recovery_point_selection: rps,
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_logically_air_gapped_backup_vault(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        region: &str,
        account_id: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("BackupVaultName", vault_name)];
        let input = match wire::deserialize_create_logically_air_gapped_backup_vault_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        // Logically air-gapped vaults are stored as regular vaults with lock config
        let arn = format!("arn:aws:backup:{region}:{account_id}:backup-vault:{vault_name}");
        let tags: HashMap<String, String> = input.backup_vault_tags.unwrap_or_default();
        let min_retention_days = input.min_retention_days;
        let max_retention_days = input.max_retention_days;

        let mut state = state.write().await;
        match state.create_backup_vault(vault_name, &arn, tags) {
            Ok(_) => {
                // Apply lock configuration
                let _ = state.put_backup_vault_lock_configuration(
                    vault_name,
                    Some(min_retention_days),
                    Some(max_retention_days),
                );
                let vault = state.describe_backup_vault(vault_name).unwrap();
                let creation_epoch = vault.creation_date.timestamp() as f64
                    + (vault.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_logically_air_gapped_backup_vault_response(
                    &wire::CreateLogicallyAirGappedBackupVaultOutput {
                        backup_vault_name: Some(vault_name.to_string()),
                        backup_vault_arn: Some(vault.backup_vault_arn.clone()),
                        creation_date: Some(creation_epoch),
                        vault_state: Some("AVAILABLE".to_string()),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_restore_access_backup_vault(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_restore_access_backup_vault_request(request, &[], query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
            };
        // Create a restore access backup vault (backed by a regular vault entry)
        let source_vault_arn = input.source_backup_vault_arn.as_str();
        let derived_name = input.backup_vault_name.clone().unwrap_or_else(|| {
            source_vault_arn
                .rsplit(':')
                .next()
                .unwrap_or("restore-access")
                .to_string()
        });
        let vault_name = derived_name.as_str();
        let tags: HashMap<String, String> = input.backup_vault_tags.unwrap_or_default();
        let arn = format!(
            "arn:aws:backup:{region}:{account_id}:restore-access-backup-vault:{vault_name}"
        );

        let mut state = state.write().await;
        match state.create_backup_vault(vault_name, &arn, tags) {
            Ok(vault) => {
                let creation_epoch = vault.creation_date.timestamp() as f64
                    + (vault.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_restore_access_backup_vault_response(
                    &wire::CreateRestoreAccessBackupVaultOutput {
                        restore_access_backup_vault_name: Some(vault_name.to_string()),
                        restore_access_backup_vault_arn: Some(arn),
                        creation_date: Some(creation_epoch),
                        vault_state: Some("AVAILABLE".to_string()),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_restore_testing_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        if raw.get("RestoreTestingPlan").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'RestoreTestingPlan'",
            );
        }
        let input = match wire::deserialize_create_restore_testing_plan_request(request, &[], query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input
            .restore_testing_plan
            .restore_testing_plan_name
            .is_empty()
        {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'RestoreTestingPlanName'",
            );
        }
        let name = input.restore_testing_plan.restore_testing_plan_name.clone();
        let schedule_expression = input.restore_testing_plan.schedule_expression.clone();
        let schedule_expression_timezone = input
            .restore_testing_plan
            .schedule_expression_timezone
            .clone();
        let start_window_hours = input.restore_testing_plan.start_window_hours;
        let recovery_point_selection =
            serde_json::to_value(&input.restore_testing_plan.recovery_point_selection)
                .unwrap_or(Value::Null);
        let creator_request_id = input.creator_request_id.clone();
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_restore_testing_plan(
            &name,
            &schedule_expression,
            schedule_expression_timezone,
            start_window_hours,
            recovery_point_selection,
            creator_request_id,
            region,
            account_id,
            tags,
        ) {
            Ok(plan) => {
                let creation_epoch = plan.creation_time.timestamp() as f64
                    + (plan.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_restore_testing_plan_response(
                    &wire::CreateRestoreTestingPlanOutput {
                        restore_testing_plan_name: Some(plan.restore_testing_plan_name.clone()),
                        restore_testing_plan_arn: Some(plan.restore_testing_plan_arn.clone()),
                        creation_time: Some(creation_epoch),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_restore_testing_selection(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        if raw.get("RestoreTestingSelection").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'RestoreTestingSelection'",
            );
        }
        let labels: &[(&str, &str)] = &[("RestoreTestingPlanName", plan_name)];
        let input = match wire::deserialize_create_restore_testing_selection_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let sel = &input.restore_testing_selection;
        if sel.restore_testing_selection_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'RestoreTestingSelectionName'",
            );
        }
        let selection_name = sel.restore_testing_selection_name.clone();
        let iam_role_arn = sel.iam_role_arn.clone();
        let protected_resource_type = sel.protected_resource_type.clone();
        let protected_resource_arns: Vec<String> =
            sel.protected_resource_arns.clone().unwrap_or_default();
        let protected_resource_conditions = sel
            .protected_resource_conditions
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null))
            .unwrap_or(Value::Null);
        let restore_metadata_overrides: HashMap<String, String> =
            sel.restore_metadata_overrides.clone().unwrap_or_default();
        let validation_window_hours = sel.validation_window_hours;
        let creator_request_id = input.creator_request_id.clone();

        let mut state = state.write().await;
        match state.create_restore_testing_selection(
            plan_name,
            &selection_name,
            &iam_role_arn,
            &protected_resource_type,
            protected_resource_arns,
            protected_resource_conditions,
            restore_metadata_overrides,
            validation_window_hours,
            creator_request_id,
        ) {
            Ok(sel) => {
                let creation_epoch = sel.creation_time.timestamp() as f64
                    + (sel.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_restore_testing_selection_response(
                    &wire::CreateRestoreTestingSelectionOutput {
                        restore_testing_selection_name: Some(
                            sel.restore_testing_selection_name.clone(),
                        ),
                        restore_testing_plan_name: Some(sel.restore_testing_plan_name.clone()),
                        restore_testing_plan_arn: Some(sel.restore_testing_plan_arn.clone()),
                        creation_time: Some(creation_epoch),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_create_tiering_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        if raw.get("TieringConfiguration").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'TieringConfiguration'",
            );
        }
        let input =
            match wire::deserialize_create_tiering_configuration_request(request, &[], query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
            };
        let cfg = &input.tiering_configuration;
        if cfg.tiering_configuration_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'TieringConfigurationName'",
            );
        }
        let name = cfg.tiering_configuration_name.clone();
        let vault_name = cfg.backup_vault_name.clone();
        let resource_selection =
            serde_json::to_value(&cfg.resource_selection).unwrap_or(Value::Array(vec![]));
        let creator_request_id = input.creator_request_id.clone();
        let tags: HashMap<String, String> = input.tiering_configuration_tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_tiering_configuration(
            &name,
            &vault_name,
            resource_selection,
            creator_request_id,
            region,
            account_id,
            tags,
        ) {
            Ok(config) => {
                let creation_epoch = config.creation_time.timestamp() as f64
                    + (config.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_create_tiering_configuration_response(
                    &wire::CreateTieringConfigurationOutput {
                        tiering_configuration_name: Some(config.tiering_configuration_name.clone()),
                        tiering_configuration_arn: Some(config.tiering_configuration_arn.clone()),
                        creation_time: Some(creation_epoch),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_backup_selection(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
        selection_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_backup_selection(plan_id, selection_id) {
            Ok(()) => wire::serialize_delete_backup_selection_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_backup_vault_access_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_backup_vault_access_policy(vault_name) {
            Ok(()) => wire::serialize_delete_backup_vault_access_policy_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_backup_vault_notifications(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_backup_vault_notifications(vault_name) {
            Ok(()) => wire::serialize_delete_backup_vault_notifications_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_framework(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_framework(name) {
            Ok(()) => wire::serialize_delete_framework_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_recovery_point(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        recovery_point_arn: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_recovery_point(vault_name, recovery_point_arn) {
            Ok(()) => wire::serialize_delete_recovery_point_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_restore_testing_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_restore_testing_plan(plan_name) {
            Ok(()) => wire::serialize_delete_restore_testing_plan_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_restore_testing_selection(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
        selection_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_restore_testing_selection(plan_name, selection_name) {
            Ok(()) => wire::serialize_delete_restore_testing_selection_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_delete_tiering_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_tiering_configuration(name) {
            Ok(()) => wire::serialize_delete_tiering_configuration_response(
                &wire::DeleteTieringConfigurationOutput {},
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_backup_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_backup_job(job_id) {
            Ok(job) => {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::serialize_describe_backup_job_response(&wire::DescribeBackupJobOutput {
                    backup_job_id: Some(job.backup_job_id.clone()),
                    backup_vault_name: Some(job.backup_vault_name.clone()),
                    backup_vault_arn: Some(job.backup_vault_arn.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    resource_arn: Some(job.resource_arn.clone()),
                    resource_type: Some(job.resource_type.clone()),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    state: Some(job.state.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    account_id: Some(job.account_id.clone()),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_copy_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        copy_job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_copy_job(copy_job_id) {
            Ok(job) => {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::serialize_describe_copy_job_response(&wire::DescribeCopyJobOutput {
                    copy_job: Some(wire::CopyJob {
                        copy_job_id: Some(job.copy_job_id.clone()),
                        source_backup_vault_arn: Some(job.source_backup_vault_arn.clone()),
                        source_recovery_point_arn: Some(job.source_recovery_point_arn.clone()),
                        destination_backup_vault_arn: Some(
                            job.destination_backup_vault_arn.clone(),
                        ),
                        destination_recovery_point_arn: Some(
                            job.destination_recovery_point_arn.clone(),
                        ),
                        iam_role_arn: Some(job.iam_role_arn.clone()),
                        state: Some(job.state.clone()),
                        creation_date: Some(creation_epoch),
                        completion_date: completion_epoch,
                        account_id: Some(job.account_id.clone()),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_framework(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_framework(name) {
            Ok(fw) => {
                let creation_epoch = fw.creation_time.timestamp() as f64
                    + (fw.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let controls: Vec<wire::FrameworkControl> =
                    if let Some(arr) = fw.framework_controls.as_array() {
                        arr.iter()
                            .filter_map(|c| serde_json::from_value(c.clone()).ok())
                            .collect()
                    } else {
                        vec![]
                    };
                wire::serialize_describe_framework_response(&wire::DescribeFrameworkOutput {
                    framework_name: Some(fw.framework_name.clone()),
                    framework_arn: Some(fw.framework_arn.clone()),
                    framework_description: Some(fw.framework_description.clone()),
                    framework_controls: Some(controls),
                    creation_time: Some(creation_epoch),
                    deployment_status: Some(fw.deployment_status.clone()),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_global_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let gs = state.describe_global_settings();
        wire::serialize_describe_global_settings_response(&wire::DescribeGlobalSettingsOutput {
            global_settings: if gs.global_settings.is_empty() {
                None
            } else {
                Some(gs.global_settings.clone())
            },
            ..Default::default()
        })
    }

    async fn handle_describe_protected_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        resource_arn: &str,
    ) -> MockResponse {
        // Derive protected resource info from recovery points
        let state = state.read().await;
        let matching_rp = state
            .recovery_points
            .values()
            .filter(|rp| rp.resource_arn == resource_arn)
            .max_by_key(|rp| rp.creation_date);
        match matching_rp {
            Some(rp) => {
                let last_backup_epoch = rp.creation_date.timestamp() as f64
                    + (rp.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_describe_protected_resource_response(
                    &wire::DescribeProtectedResourceOutput {
                        resource_arn: Some(rp.resource_arn.clone()),
                        resource_type: Some(rp.resource_type.clone()),
                        last_backup_time: Some(last_backup_epoch),
                        last_backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                        last_recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                        ..Default::default()
                    },
                )
            }
            None => wire::serialize_describe_protected_resource_response(
                &wire::DescribeProtectedResourceOutput::default(),
            ),
        }
    }

    async fn handle_describe_recovery_point(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        recovery_point_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_recovery_point(vault_name, recovery_point_arn) {
            Ok(rp) => {
                let creation_epoch = rp.creation_date.timestamp() as f64
                    + (rp.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_describe_recovery_point_response(
                    &wire::DescribeRecoveryPointOutput {
                        recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                        backup_vault_name: Some(rp.backup_vault_name.clone()),
                        backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                        resource_arn: Some(rp.resource_arn.clone()),
                        resource_type: Some(rp.resource_type.clone()),
                        iam_role_arn: Some(rp.iam_role_arn.clone()),
                        status: Some(rp.status.clone()),
                        creation_date: Some(creation_epoch),
                        backup_size_in_bytes: Some(rp.backup_size_bytes),
                        ..Default::default()
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_region_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let rs = state.describe_region_settings();
        wire::serialize_describe_region_settings_response(&wire::DescribeRegionSettingsOutput {
            resource_type_opt_in_preference: if rs.resource_type_opt_in_preference.is_empty() {
                None
            } else {
                Some(rs.resource_type_opt_in_preference.clone())
            },
            resource_type_management_preference: if rs
                .resource_type_management_preference
                .is_empty()
            {
                None
            } else {
                Some(rs.resource_type_management_preference.clone())
            },
        })
    }

    async fn handle_describe_report_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_report_job(job_id) {
            Ok(job) => {
                let creation_epoch = job.creation_time.timestamp() as f64
                    + (job.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_time
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::serialize_describe_report_job_response(&wire::DescribeReportJobOutput {
                    report_job: Some(wire::ReportJob {
                        report_job_id: Some(job.report_job_id.clone()),
                        report_plan_arn: Some(job.report_plan_arn.clone()),
                        report_template: Some(job.report_template.clone()),
                        creation_time: Some(creation_epoch),
                        completion_time: completion_epoch,
                        status: Some(job.status.clone()),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_restore_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        restore_job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_restore_job(restore_job_id) {
            Ok(job) => {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::serialize_describe_restore_job_response(&wire::DescribeRestoreJobOutput {
                    restore_job_id: Some(job.restore_job_id.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    resource_type: Some(job.resource_type.clone()),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    status: Some(job.status.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    backup_size_in_bytes: Some(job.backup_size_in_bytes),
                    account_id: Some(job.account_id.clone()),
                    validation_status: job.validation_status.clone(),
                    validation_status_message: job.validation_status_message.clone(),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_describe_scan_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        scan_job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_scan_job(scan_job_id) {
            Ok(job) => {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::serialize_describe_scan_job_response(&wire::DescribeScanJobOutput {
                    scan_job_id: Some(job.scan_job_id.clone()),
                    backup_vault_name: Some(job.backup_vault_name.clone()),
                    backup_vault_arn: Some(job.backup_vault_arn.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    malware_scanner: Some(job.malware_scanner.clone()),
                    scan_mode: Some(job.scan_mode.clone()),
                    scanner_role_arn: Some(job.scanner_role_arn.clone()),
                    scan_base_recovery_point_arn: job.scan_base_recovery_point_arn.clone(),
                    state: Some(job.state.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    account_id: Some(job.account_id.clone()),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    // MPA approval team operations are no-ops in mock (no persistent state needed)
    async fn handle_disassociate_backup_vault_mpa_approval_team(&self) -> MockResponse {
        wire::serialize_disassociate_backup_vault_mpa_approval_team_response()
    }

    // Recovery point disassociation is a no-op in mock
    async fn handle_disassociate_recovery_point(&self) -> MockResponse {
        wire::serialize_disassociate_recovery_point_response()
    }

    // Recovery point parent disassociation is a no-op in mock
    async fn handle_disassociate_recovery_point_from_parent(&self) -> MockResponse {
        wire::serialize_disassociate_recovery_point_from_parent_response()
    }

    async fn handle_export_backup_plan_template(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_backup_plan(plan_id) {
            Ok(plan) => {
                let template_json = serde_json::to_string(&plan.backup_plan_json)
                    .unwrap_or_else(|_| "{}".to_string());
                wire::serialize_export_backup_plan_template_response(
                    &wire::ExportBackupPlanTemplateOutput {
                        backup_plan_template_json: Some(template_json),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_backup_plan_from_j_s_o_n(
        &self,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_backup_plan_from_j_s_o_n_request(request, &[], query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
            };
        // Parse the BackupPlanTemplateJson string and return a plan structure
        let template_json_str = if input.backup_plan_template_json.is_empty() {
            "{}".to_string()
        } else {
            input.backup_plan_template_json.clone()
        };
        let parsed: Value =
            serde_json::from_str(&template_json_str).unwrap_or(Value::Object(Default::default()));

        let plan_name = parsed
            .get("BackupPlanName")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let rules_json = parsed.get("Rules").cloned().unwrap_or(Value::Array(vec![]));
        let rules: Vec<wire::BackupRule> = if let Value::Array(arr) = &rules_json {
            arr.iter()
                .map(|r| wire::BackupRule {
                    rule_name: r
                        .get("RuleName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    target_backup_vault_name: r
                        .get("TargetBackupVaultName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    schedule_expression: r
                        .get("ScheduleExpression")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    ..Default::default()
                })
                .collect()
        } else {
            vec![]
        };

        wire::serialize_get_backup_plan_from_j_s_o_n_response(&wire::GetBackupPlanFromJSONOutput {
            backup_plan: Some(wire::BackupPlan {
                backup_plan_name: Some(plan_name),
                rules: Some(rules),
                ..Default::default()
            }),
        })
    }

    // Plan templates are static/built-in - return empty list in mock
    async fn handle_get_backup_plan_from_template(&self) -> MockResponse {
        wire::serialize_get_backup_plan_from_template_response(
            &wire::GetBackupPlanFromTemplateOutput::default(),
        )
    }

    async fn handle_get_backup_selection(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
        selection_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_backup_selection(plan_id, selection_id) {
            Ok(sel) => {
                let creation_epoch = sel.creation_date.timestamp() as f64
                    + (sel.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let backup_selection_wire: wire::BackupSelection =
                    serde_json::from_value(sel.selection_json.clone()).unwrap_or_else(|_| {
                        wire::BackupSelection {
                            selection_name: sel.selection_name.clone(),
                            iam_role_arn: sel.iam_role_arn.clone(),
                            resources: if sel.resources.is_empty() {
                                None
                            } else {
                                Some(sel.resources.clone())
                            },
                            ..Default::default()
                        }
                    });
                wire::serialize_get_backup_selection_response(&wire::GetBackupSelectionOutput {
                    backup_plan_id: Some(sel.backup_plan_id.clone()),
                    selection_id: Some(sel.selection_id.clone()),
                    creation_date: Some(creation_epoch),
                    backup_selection: Some(backup_selection_wire),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_backup_vault_access_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_backup_vault_access_policy(vault_name) {
            Ok(policy) => wire::serialize_get_backup_vault_access_policy_response(
                &wire::GetBackupVaultAccessPolicyOutput {
                    backup_vault_name: Some(policy.backup_vault_name.clone()),
                    backup_vault_arn: Some(policy.backup_vault_arn.clone()),
                    policy: Some(policy.policy.clone()),
                },
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_backup_vault_notifications(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_backup_vault_notifications(vault_name) {
            Ok(config) => wire::serialize_get_backup_vault_notifications_response(
                &wire::GetBackupVaultNotificationsOutput {
                    backup_vault_name: Some(config.backup_vault_name.clone()),
                    backup_vault_arn: Some(config.backup_vault_arn.clone()),
                    s_n_s_topic_arn: Some(config.sns_topic_arn.clone()),
                    backup_vault_events: Some(config.backup_vault_events.clone()),
                },
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_legal_hold(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        legal_hold_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_legal_hold(legal_hold_id) {
            Ok(hold) => {
                let creation_epoch = hold.creation_date.timestamp() as f64
                    + (hold.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let cancellation_epoch = hold
                    .cancellation_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                let rps: Option<wire::RecoveryPointSelection> =
                    serde_json::from_value(hold.recovery_point_selection.clone()).ok();
                wire::serialize_get_legal_hold_response(&wire::GetLegalHoldOutput {
                    legal_hold_id: Some(hold.legal_hold_id.clone()),
                    legal_hold_arn: Some(hold.legal_hold_arn.clone()),
                    title: Some(hold.title.clone()),
                    description: Some(hold.description.clone()),
                    status: Some(hold.status.clone()),
                    creation_date: Some(creation_epoch),
                    cancellation_date: cancellation_epoch,
                    recovery_point_selection: rps,
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_recovery_point_index_details(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        recovery_point_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_recovery_point(vault_name, recovery_point_arn) {
            Ok(rp) => wire::serialize_get_recovery_point_index_details_response(
                &wire::GetRecoveryPointIndexDetailsOutput {
                    recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                    source_resource_arn: Some(rp.resource_arn.clone()),
                    index_status: Some("ACTIVE".to_string()),
                    ..Default::default()
                },
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_recovery_point_restore_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        recovery_point_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_recovery_point(vault_name, recovery_point_arn) {
            Ok(rp) => wire::serialize_get_recovery_point_restore_metadata_response(
                &wire::GetRecoveryPointRestoreMetadataOutput {
                    recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                    resource_type: Some(rp.resource_type.clone()),
                    restore_metadata: Some(HashMap::new()),
                },
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_restore_job_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        restore_job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_restore_job_metadata(restore_job_id) {
            Ok(job) => wire::serialize_get_restore_job_metadata_response(
                &wire::GetRestoreJobMetadataOutput {
                    restore_job_id: Some(job.restore_job_id.clone()),
                    metadata: if job.metadata.is_empty() {
                        None
                    } else {
                        Some(job.metadata.clone())
                    },
                },
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    // Inferred metadata is a static/computed operation - return empty map in mock
    async fn handle_get_restore_testing_inferred_metadata(&self) -> MockResponse {
        wire::serialize_get_restore_testing_inferred_metadata_response(
            &wire::GetRestoreTestingInferredMetadataOutput {
                inferred_metadata: Some(HashMap::new()),
            },
        )
    }

    async fn handle_get_restore_testing_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_restore_testing_plan(plan_name) {
            Ok(plan) => {
                let creation_epoch = plan.creation_time.timestamp() as f64
                    + (plan.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let update_epoch = plan.last_update_time.timestamp() as f64
                    + (plan.last_update_time.timestamp_subsec_millis() as f64 / 1000.0);
                let rps: Option<wire::RestoreTestingRecoveryPointSelection> =
                    serde_json::from_value(plan.recovery_point_selection.clone()).ok();
                wire::serialize_get_restore_testing_plan_response(
                    &wire::GetRestoreTestingPlanOutput {
                        restore_testing_plan: Some(wire::RestoreTestingPlanForGet {
                            restore_testing_plan_name: Some(plan.restore_testing_plan_name.clone()),
                            restore_testing_plan_arn: Some(plan.restore_testing_plan_arn.clone()),
                            schedule_expression: Some(plan.schedule_expression.clone()),
                            schedule_expression_timezone: plan.schedule_expression_timezone.clone(),
                            start_window_hours: plan.start_window_hours,
                            recovery_point_selection: rps,
                            creator_request_id: plan.creator_request_id.clone(),
                            creation_time: Some(creation_epoch),
                            last_update_time: Some(update_epoch),
                            last_execution_time: None,
                        }),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_get_restore_testing_selection(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
        selection_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_restore_testing_selection(plan_name, selection_name) {
            Ok(sel) => {
                let creation_epoch = sel.creation_time.timestamp() as f64
                    + (sel.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let prc: Option<wire::ProtectedResourceConditions> =
                    serde_json::from_value(sel.protected_resource_conditions.clone()).ok();
                wire::serialize_get_restore_testing_selection_response(
                    &wire::GetRestoreTestingSelectionOutput {
                        restore_testing_selection: Some(wire::RestoreTestingSelectionForGet {
                            restore_testing_selection_name: Some(
                                sel.restore_testing_selection_name.clone(),
                            ),
                            restore_testing_plan_name: Some(sel.restore_testing_plan_name.clone()),
                            iam_role_arn: Some(sel.iam_role_arn.clone()),
                            protected_resource_type: Some(sel.protected_resource_type.clone()),
                            protected_resource_arns: if sel.protected_resource_arns.is_empty() {
                                None
                            } else {
                                Some(sel.protected_resource_arns.clone())
                            },
                            protected_resource_conditions: prc,
                            restore_metadata_overrides: if sel.restore_metadata_overrides.is_empty()
                            {
                                None
                            } else {
                                Some(sel.restore_metadata_overrides.clone())
                            },
                            validation_window_hours: sel.validation_window_hours,
                            creator_request_id: sel.creator_request_id.clone(),
                            creation_time: Some(creation_epoch),
                        }),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    // Supported resource types is a static list
    async fn handle_get_supported_resource_types(&self) -> MockResponse {
        wire::serialize_get_supported_resource_types_response(
            &wire::GetSupportedResourceTypesOutput {
                resource_types: Some(vec![
                    "EC2".to_string(),
                    "EBS".to_string(),
                    "RDS".to_string(),
                    "DynamoDB".to_string(),
                    "EFS".to_string(),
                    "S3".to_string(),
                ]),
            },
        )
    }

    async fn handle_get_tiering_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_tiering_configuration(name) {
            Ok(config) => {
                let creation_epoch = config.creation_time.timestamp() as f64
                    + (config.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let updated_epoch = config.last_updated_time.timestamp() as f64
                    + (config.last_updated_time.timestamp_subsec_millis() as f64 / 1000.0);
                let resource_selection: Option<Vec<wire::ResourceSelection>> =
                    serde_json::from_value(config.resource_selection.clone()).ok();
                wire::serialize_get_tiering_configuration_response(
                    &wire::GetTieringConfigurationOutput {
                        tiering_configuration: Some(wire::TieringConfiguration {
                            tiering_configuration_name: Some(
                                config.tiering_configuration_name.clone(),
                            ),
                            tiering_configuration_arn: Some(
                                config.tiering_configuration_arn.clone(),
                            ),
                            backup_vault_name: Some(config.backup_vault_name.clone()),
                            resource_selection,
                            creation_time: Some(creation_epoch),
                            last_updated_time: Some(updated_epoch),
                            creator_request_id: config.creator_request_id.clone(),
                        }),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_list_backup_job_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_backup_jobs();
        let mut counts: HashMap<String, i32> = HashMap::new();
        for job in &jobs {
            *counts.entry(job.state.clone()).or_insert(0) += 1;
        }
        let summaries: Vec<wire::BackupJobSummary> = counts
            .into_iter()
            .map(|(state_val, count)| wire::BackupJobSummary {
                state: Some(state_val),
                count: Some(count),
                account_id: Some(account_id.to_string()),
                region: Some(region.to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_backup_job_summaries_response(&wire::ListBackupJobSummariesOutput {
            backup_job_summaries: if summaries.is_empty() {
                None
            } else {
                Some(summaries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_backup_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_backup_jobs();
        let entries: Vec<wire::BackupJob> = jobs
            .iter()
            .map(|job| {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::BackupJob {
                    backup_job_id: Some(job.backup_job_id.clone()),
                    backup_vault_name: Some(job.backup_vault_name.clone()),
                    backup_vault_arn: Some(job.backup_vault_arn.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    resource_arn: Some(job.resource_arn.clone()),
                    resource_type: Some(job.resource_type.clone()),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    state: Some(job.state.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    account_id: Some(job.account_id.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_backup_jobs_response(&wire::ListBackupJobsOutput {
            backup_jobs: Some(entries),
            ..Default::default()
        })
    }

    // Plan templates are built-in AWS templates - return empty in mock
    async fn handle_list_backup_plan_templates(
        &self,
        _state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        wire::serialize_list_backup_plan_templates_response(&wire::ListBackupPlanTemplatesOutput {
            backup_plan_templates_list: Some(vec![]),
            ..Default::default()
        })
    }

    async fn handle_list_backup_plan_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_backup_plan(plan_id) {
            Ok(plan) => {
                let creation_epoch = plan.creation_date.timestamp() as f64
                    + (plan.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                // Return the current version as the only version
                let entry = wire::BackupPlansListMember {
                    backup_plan_id: Some(plan.backup_plan_id.clone()),
                    backup_plan_arn: Some(plan.backup_plan_arn.clone()),
                    backup_plan_name: Some(plan.backup_plan_name.clone()),
                    creation_date: Some(creation_epoch),
                    version_id: Some(plan.version_id.clone()),
                    ..Default::default()
                };
                wire::serialize_list_backup_plan_versions_response(
                    &wire::ListBackupPlanVersionsOutput {
                        backup_plan_versions_list: Some(vec![entry]),
                        ..Default::default()
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_list_backup_selections(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let selections = state.list_backup_selections(plan_id);
        let entries: Vec<wire::BackupSelectionsListMember> = selections
            .iter()
            .map(|sel| {
                let creation_epoch = sel.creation_date.timestamp() as f64
                    + (sel.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::BackupSelectionsListMember {
                    backup_plan_id: Some(sel.backup_plan_id.clone()),
                    selection_id: Some(sel.selection_id.clone()),
                    selection_name: Some(sel.selection_name.clone()),
                    iam_role_arn: Some(sel.iam_role_arn.clone()),
                    creation_date: Some(creation_epoch),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_backup_selections_response(&wire::ListBackupSelectionsOutput {
            backup_selections_list: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_copy_job_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_copy_jobs();
        let mut counts: HashMap<String, i32> = HashMap::new();
        for job in &jobs {
            *counts.entry(job.state.clone()).or_insert(0) += 1;
        }
        let summaries: Vec<wire::CopyJobSummary> = counts
            .into_iter()
            .map(|(state_val, count)| wire::CopyJobSummary {
                state: Some(state_val),
                count: Some(count),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_copy_job_summaries_response(&wire::ListCopyJobSummariesOutput {
            copy_job_summaries: if summaries.is_empty() {
                None
            } else {
                Some(summaries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_copy_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_copy_jobs();
        let entries: Vec<wire::CopyJob> = jobs
            .iter()
            .map(|job| {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::CopyJob {
                    copy_job_id: Some(job.copy_job_id.clone()),
                    source_backup_vault_arn: Some(job.source_backup_vault_arn.clone()),
                    source_recovery_point_arn: Some(job.source_recovery_point_arn.clone()),
                    destination_backup_vault_arn: Some(job.destination_backup_vault_arn.clone()),
                    destination_recovery_point_arn: Some(
                        job.destination_recovery_point_arn.clone(),
                    ),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    state: Some(job.state.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    account_id: Some(job.account_id.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_copy_jobs_response(&wire::ListCopyJobsOutput {
            copy_jobs: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_frameworks(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let frameworks = state.list_frameworks();
        let entries: Vec<wire::Framework> = frameworks
            .iter()
            .map(|fw| {
                let creation_epoch = fw.creation_time.timestamp() as f64
                    + (fw.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::Framework {
                    framework_name: Some(fw.framework_name.clone()),
                    framework_arn: Some(fw.framework_arn.clone()),
                    framework_description: Some(fw.framework_description.clone()),
                    number_of_controls: Some(fw.number_of_controls),
                    creation_time: Some(creation_epoch),
                    deployment_status: Some(fw.deployment_status.clone()),
                }
            })
            .collect();
        wire::serialize_list_frameworks_response(&wire::ListFrameworksOutput {
            frameworks: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_indexed_recovery_points(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let entries: Vec<wire::IndexedRecoveryPoint> = state
            .recovery_points
            .values()
            .map(|rp| {
                let creation_epoch = rp.creation_date.timestamp() as f64
                    + (rp.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::IndexedRecoveryPoint {
                    recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                    source_resource_arn: Some(rp.resource_arn.clone()),
                    resource_type: Some(rp.resource_type.clone()),
                    iam_role_arn: Some(rp.iam_role_arn.clone()),
                    backup_creation_date: Some(creation_epoch),
                    index_status: Some("ACTIVE".to_string()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_indexed_recovery_points_response(
            &wire::ListIndexedRecoveryPointsOutput {
                indexed_recovery_points: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_legal_holds(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let holds = state.list_legal_holds();
        let entries: Vec<wire::LegalHold> = holds
            .iter()
            .map(|h| {
                let creation_epoch = h.creation_date.timestamp() as f64
                    + (h.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let cancellation_epoch = h
                    .cancellation_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::LegalHold {
                    legal_hold_id: Some(h.legal_hold_id.clone()),
                    legal_hold_arn: Some(h.legal_hold_arn.clone()),
                    title: Some(h.title.clone()),
                    description: Some(h.description.clone()),
                    status: Some(h.status.clone()),
                    creation_date: Some(creation_epoch),
                    cancellation_date: cancellation_epoch,
                }
            })
            .collect();
        wire::serialize_list_legal_holds_response(&wire::ListLegalHoldsOutput {
            legal_holds: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_protected_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        // Derive protected resources from recovery points
        let state = state.read().await;
        let mut resource_map: HashMap<String, &crate::types::RecoveryPointData> = HashMap::new();
        for rp in state.recovery_points.values() {
            let entry = resource_map.get(&rp.resource_arn);
            if entry.is_none() || entry.unwrap().creation_date < rp.creation_date {
                resource_map.insert(rp.resource_arn.clone(), rp);
            }
        }
        let entries: Vec<wire::ProtectedResource> = resource_map
            .values()
            .map(|rp| {
                let last_backup_epoch = rp.creation_date.timestamp() as f64
                    + (rp.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::ProtectedResource {
                    resource_arn: Some(rp.resource_arn.clone()),
                    resource_type: Some(rp.resource_type.clone()),
                    last_backup_time: Some(last_backup_epoch),
                    last_backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                    last_recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_protected_resources_response(&wire::ListProtectedResourcesOutput {
            results: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_protected_resources_by_backup_vault(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let mut resource_map: HashMap<String, &crate::types::RecoveryPointData> = HashMap::new();
        for rp in state
            .recovery_points
            .values()
            .filter(|rp| rp.backup_vault_name == vault_name)
        {
            let entry = resource_map.get(&rp.resource_arn);
            if entry.is_none() || entry.unwrap().creation_date < rp.creation_date {
                resource_map.insert(rp.resource_arn.clone(), rp);
            }
        }
        let entries: Vec<wire::ProtectedResource> = resource_map
            .values()
            .map(|rp| {
                let last_backup_epoch = rp.creation_date.timestamp() as f64
                    + (rp.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::ProtectedResource {
                    resource_arn: Some(rp.resource_arn.clone()),
                    resource_type: Some(rp.resource_type.clone()),
                    last_backup_time: Some(last_backup_epoch),
                    last_backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                    last_recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_protected_resources_by_backup_vault_response(
            &wire::ListProtectedResourcesByBackupVaultOutput {
                results: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_recovery_points_by_backup_vault(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let rps = state.list_recovery_points_by_backup_vault(vault_name);
        let entries: Vec<wire::RecoveryPointByBackupVault> = rps
            .iter()
            .map(|rp| {
                let creation_epoch = rp.creation_date.timestamp() as f64
                    + (rp.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::RecoveryPointByBackupVault {
                    recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    backup_vault_name: Some(rp.backup_vault_name.clone()),
                    backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                    resource_arn: Some(rp.resource_arn.clone()),
                    resource_type: Some(rp.resource_type.clone()),
                    iam_role_arn: Some(rp.iam_role_arn.clone()),
                    status: Some(rp.status.clone()),
                    creation_date: Some(creation_epoch),
                    backup_size_in_bytes: Some(rp.backup_size_bytes),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_recovery_points_by_backup_vault_response(
            &wire::ListRecoveryPointsByBackupVaultOutput {
                recovery_points: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_list_recovery_points_by_legal_hold(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        _legal_hold_id: &str,
    ) -> MockResponse {
        // In mock, legal holds don't actually filter recovery points - return all
        let state = state.read().await;
        let entries: Vec<wire::RecoveryPointMember> = state
            .recovery_points
            .values()
            .map(|rp| wire::RecoveryPointMember {
                recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                backup_vault_name: Some(rp.backup_vault_name.clone()),
                resource_arn: Some(rp.resource_arn.clone()),
                resource_type: Some(rp.resource_type.clone()),
            })
            .collect();
        wire::serialize_list_recovery_points_by_legal_hold_response(
            &wire::ListRecoveryPointsByLegalHoldOutput {
                recovery_points: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_recovery_points_by_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let entries: Vec<wire::RecoveryPointByResource> = state
            .recovery_points
            .values()
            .filter(|rp| rp.resource_arn == resource_arn)
            .map(|rp| {
                let creation_epoch = rp.creation_date.timestamp() as f64
                    + (rp.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::RecoveryPointByResource {
                    recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    backup_vault_name: Some(rp.backup_vault_name.clone()),
                    backup_size_bytes: Some(rp.backup_size_bytes),
                    creation_date: Some(creation_epoch),
                    status: Some(rp.status.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_recovery_points_by_resource_response(
            &wire::ListRecoveryPointsByResourceOutput {
                recovery_points: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_report_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        uri: &str,
    ) -> MockResponse {
        // Extract optional ReportPlanName query param
        let report_plan_name: Option<String> =
            uri.find('?').map(|i| &uri[i + 1..]).and_then(|qs| {
                qs.split('&').find_map(|kv| {
                    let (k, v) = kv.split_once('=')?;
                    if k == "ReportPlanName" {
                        Some(percent_decode(v))
                    } else {
                        None
                    }
                })
            });

        let state = state.read().await;
        let jobs = state.list_report_jobs(report_plan_name.as_deref());
        let entries: Vec<wire::ReportJob> = jobs
            .iter()
            .map(|j| {
                let creation_epoch = j.creation_time.timestamp() as f64
                    + (j.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = j
                    .completion_time
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::ReportJob {
                    report_job_id: Some(j.report_job_id.clone()),
                    report_plan_arn: Some(j.report_plan_arn.clone()),
                    report_template: Some(j.report_template.clone()),
                    creation_time: Some(creation_epoch),
                    completion_time: completion_epoch,
                    status: Some(j.status.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_report_jobs_response(&wire::ListReportJobsOutput {
            report_jobs: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            ..Default::default()
        })
    }

    // Restore access backup vaults are a subset feature - return empty list in mock
    async fn handle_list_restore_access_backup_vaults(&self) -> MockResponse {
        wire::serialize_list_restore_access_backup_vaults_response(
            &wire::ListRestoreAccessBackupVaultsOutput {
                restore_access_backup_vaults: Some(vec![]),
                ..Default::default()
            },
        )
    }

    async fn handle_list_restore_job_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_restore_jobs();
        let mut counts: HashMap<String, i32> = HashMap::new();
        for job in &jobs {
            *counts.entry(job.status.clone()).or_insert(0) += 1;
        }
        let summaries: Vec<wire::RestoreJobSummary> = counts
            .into_iter()
            .map(|(state_val, count)| wire::RestoreJobSummary {
                state: Some(state_val),
                count: Some(count),
                account_id: Some(account_id.to_string()),
                region: Some(region.to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_restore_job_summaries_response(&wire::ListRestoreJobSummariesOutput {
            restore_job_summaries: if summaries.is_empty() {
                None
            } else {
                Some(summaries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_restore_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_restore_jobs();
        let entries: Vec<wire::RestoreJobsListMember> = jobs
            .iter()
            .map(|job| {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::RestoreJobsListMember {
                    restore_job_id: Some(job.restore_job_id.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    resource_type: Some(job.resource_type.clone()),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    status: Some(job.status.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    backup_size_in_bytes: Some(job.backup_size_in_bytes),
                    account_id: Some(job.account_id.clone()),
                    validation_status: job.validation_status.clone(),
                    validation_status_message: job.validation_status_message.clone(),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_restore_jobs_response(&wire::ListRestoreJobsOutput {
            restore_jobs: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_restore_jobs_by_protected_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_restore_jobs_by_recovery_point(resource_arn);
        let entries: Vec<wire::RestoreJobsListMember> = jobs
            .iter()
            .map(|job| {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::RestoreJobsListMember {
                    restore_job_id: Some(job.restore_job_id.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    resource_type: Some(job.resource_type.clone()),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    status: Some(job.status.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    backup_size_in_bytes: Some(job.backup_size_in_bytes),
                    account_id: Some(job.account_id.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_restore_jobs_by_protected_resource_response(
            &wire::ListRestoreJobsByProtectedResourceOutput {
                restore_jobs: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_restore_testing_plans(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let plans = state.list_restore_testing_plans();
        let entries: Vec<wire::RestoreTestingPlanForList> = plans
            .iter()
            .map(|p| {
                let creation_epoch = p.creation_time.timestamp() as f64
                    + (p.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let update_epoch = p.last_update_time.timestamp() as f64
                    + (p.last_update_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::RestoreTestingPlanForList {
                    restore_testing_plan_name: Some(p.restore_testing_plan_name.clone()),
                    restore_testing_plan_arn: Some(p.restore_testing_plan_arn.clone()),
                    schedule_expression: Some(p.schedule_expression.clone()),
                    creation_time: Some(creation_epoch),
                    last_update_time: Some(update_epoch),
                    last_execution_time: None,
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_restore_testing_plans_response(&wire::ListRestoreTestingPlansOutput {
            restore_testing_plans: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_restore_testing_selections(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let selections = state.list_restore_testing_selections(plan_name);
        let entries: Vec<wire::RestoreTestingSelectionForList> = selections
            .iter()
            .map(|s| {
                let creation_epoch = s.creation_time.timestamp() as f64
                    + (s.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::RestoreTestingSelectionForList {
                    restore_testing_selection_name: Some(s.restore_testing_selection_name.clone()),
                    restore_testing_plan_name: Some(s.restore_testing_plan_name.clone()),
                    iam_role_arn: Some(s.iam_role_arn.clone()),
                    protected_resource_type: Some(s.protected_resource_type.clone()),
                    creation_time: Some(creation_epoch),
                    validation_window_hours: s.validation_window_hours,
                }
            })
            .collect();
        wire::serialize_list_restore_testing_selections_response(
            &wire::ListRestoreTestingSelectionsOutput {
                restore_testing_selections: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_scan_job_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        // Return count summary per state from stored scan jobs.
        let state = state.read().await;
        let jobs = state.list_scan_jobs();
        let mut counts: HashMap<String, i32> = HashMap::new();
        for job in &jobs {
            *counts.entry(job.state.clone()).or_insert(0) += 1;
        }
        let summaries: Vec<wire::ScanJobSummary> = counts
            .into_iter()
            .map(|(state_val, count)| wire::ScanJobSummary {
                state: Some(state_val),
                count: Some(count),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_scan_job_summaries_response(&wire::ListScanJobSummariesOutput {
            scan_job_summaries: if summaries.is_empty() {
                None
            } else {
                Some(summaries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_scan_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_scan_jobs();
        let entries: Vec<wire::ScanJob> = jobs
            .iter()
            .map(|job| {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                let completion_epoch = job
                    .completion_date
                    .map(|d| d.timestamp() as f64 + (d.timestamp_subsec_millis() as f64 / 1000.0));
                wire::ScanJob {
                    scan_job_id: Some(job.scan_job_id.clone()),
                    backup_vault_name: Some(job.backup_vault_name.clone()),
                    backup_vault_arn: Some(job.backup_vault_arn.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    iam_role_arn: Some(job.iam_role_arn.clone()),
                    malware_scanner: Some(job.malware_scanner.clone()),
                    scan_mode: Some(job.scan_mode.clone()),
                    scanner_role_arn: Some(job.scanner_role_arn.clone()),
                    scan_base_recovery_point_arn: job.scan_base_recovery_point_arn.clone(),
                    state: Some(job.state.clone()),
                    creation_date: Some(creation_epoch),
                    completion_date: completion_epoch,
                    account_id: Some(job.account_id.clone()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_scan_jobs_response(&wire::ListScanJobsOutput {
            scan_jobs: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_tiering_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_tiering_configurations();
        let entries: Vec<wire::TieringConfigurationsListMember> = configs
            .iter()
            .map(|c| {
                let creation_epoch = c.creation_time.timestamp() as f64
                    + (c.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::TieringConfigurationsListMember {
                    tiering_configuration_name: Some(c.tiering_configuration_name.clone()),
                    tiering_configuration_arn: Some(c.tiering_configuration_arn.clone()),
                    backup_vault_name: Some(c.backup_vault_name.clone()),
                    creation_time: Some(creation_epoch),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_tiering_configurations_response(
            &wire::ListTieringConfigurationsOutput {
                tiering_configurations: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_put_backup_vault_access_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        region: &str,
        account_id: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("BackupVaultName", vault_name)];
        let input = match wire::deserialize_put_backup_vault_access_policy_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let policy = input.policy.clone().unwrap_or_default();
        let vault_arn = format!("arn:aws:backup:{region}:{account_id}:backup-vault:{vault_name}");
        let mut state = state.write().await;
        match state.put_backup_vault_access_policy(vault_name, &vault_arn, &policy) {
            Ok(()) => wire::serialize_put_backup_vault_access_policy_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_put_backup_vault_notifications(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        region: &str,
        account_id: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("BackupVaultName", vault_name)];
        let input = match wire::deserialize_put_backup_vault_notifications_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let sns_topic_arn = input.s_n_s_topic_arn.clone();
        let backup_vault_events: Vec<String> = input.backup_vault_events.clone();
        let vault_arn = format!("arn:aws:backup:{region}:{account_id}:backup-vault:{vault_name}");
        let mut state = state.write().await;
        match state.put_backup_vault_notifications(
            vault_name,
            &vault_arn,
            &sns_topic_arn,
            backup_vault_events,
        ) {
            Ok(()) => wire::serialize_put_backup_vault_notifications_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_put_restore_validation_result(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        restore_job_id: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("RestoreJobId", restore_job_id)];
        let input =
            match wire::deserialize_put_restore_validation_result_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
            };
        let validation_status = input.validation_status.clone();
        let validation_status_message = input.validation_status_message.clone();
        let mut state = state.write().await;
        match state.put_restore_validation_result(
            restore_job_id,
            &validation_status,
            validation_status_message.as_deref(),
        ) {
            Ok(()) => wire::serialize_put_restore_validation_result_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    // Revoke restore access is a no-op in mock
    async fn handle_revoke_restore_access_backup_vault(&self) -> MockResponse {
        wire::serialize_revoke_restore_access_backup_vault_response()
    }

    async fn handle_start_backup_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_backup_job_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        if input.backup_vault_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'BackupVaultName'",
            );
        }
        let vault_name = input.backup_vault_name.clone();
        let resource_arn = input.resource_arn.clone();
        let iam_role_arn = input.iam_role_arn.clone();
        // ResourceType is not part of the StartBackupJob Smithy input shape — fall
        // back to inspecting the raw body for backwards compatibility.
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        let resource_type = raw
            .get("ResourceType")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let vault_arn = format!("arn:aws:backup:{region}:{account_id}:backup-vault:{vault_name}");

        let mut state = state.write().await;
        match state.start_backup_job(
            &vault_name,
            &vault_arn,
            &resource_arn,
            &resource_type,
            &iam_role_arn,
            account_id,
            region,
        ) {
            Ok(job) => {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_start_backup_job_response(&wire::StartBackupJobOutput {
                    backup_job_id: Some(job.backup_job_id.clone()),
                    recovery_point_arn: Some(job.recovery_point_arn.clone()),
                    creation_date: Some(creation_epoch),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_start_copy_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_copy_job_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let source_backup_vault_name = input.source_backup_vault_name.clone();
        let recovery_point_arn = input.recovery_point_arn.clone();
        let destination_backup_vault_arn = input.destination_backup_vault_arn.clone();
        let iam_role_arn = input.iam_role_arn.clone();

        let mut state = state.write().await;
        match state.start_copy_job(
            &source_backup_vault_name,
            &recovery_point_arn,
            &destination_backup_vault_arn,
            &iam_role_arn,
            account_id,
            region,
        ) {
            Ok(job) => {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_start_copy_job_response(&wire::StartCopyJobOutput {
                    copy_job_id: Some(job.copy_job_id.clone()),
                    creation_date: Some(creation_epoch),
                    is_parent: Some(false),
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_start_report_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        report_plan_name: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.start_report_job(report_plan_name, region, account_id) {
            Ok(job) => wire::serialize_start_report_job_response(&wire::StartReportJobOutput {
                report_job_id: Some(job.report_job_id.clone()),
            }),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_start_restore_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_restore_job_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let recovery_point_arn = input.recovery_point_arn.clone();
        let iam_role_arn = input.iam_role_arn.clone().unwrap_or_default();
        let resource_type = input.resource_type.clone().unwrap_or_default();
        let metadata: HashMap<String, String> = input.metadata.clone();

        let mut state = state.write().await;
        match state.start_restore_job(
            &recovery_point_arn,
            &iam_role_arn,
            &resource_type,
            metadata,
            account_id,
        ) {
            Ok(job) => wire::serialize_start_restore_job_response(&wire::StartRestoreJobOutput {
                restore_job_id: Some(job.restore_job_id.clone()),
            }),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_start_scan_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_scan_job_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let vault_name = input.backup_vault_name.clone();
        let vault_arn = format!("arn:aws:backup:{region}:{account_id}:backup-vault:{vault_name}");
        let recovery_point_arn = input.recovery_point_arn.clone();
        let iam_role_arn = input.iam_role_arn.clone();
        let malware_scanner = input.malware_scanner.clone();
        let scan_mode = input.scan_mode.clone();
        let scanner_role_arn = input.scanner_role_arn.clone();
        let scan_base_recovery_point_arn = input.scan_base_recovery_point_arn.clone();

        let mut state = state.write().await;
        match state.start_scan_job(
            &vault_name,
            &vault_arn,
            &recovery_point_arn,
            &iam_role_arn,
            &malware_scanner,
            &scan_mode,
            &scanner_role_arn,
            scan_base_recovery_point_arn,
            account_id,
            region,
        ) {
            Ok(job) => {
                let creation_epoch = job.creation_date.timestamp() as f64
                    + (job.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_start_scan_job_response(&wire::StartScanJobOutput {
                    scan_job_id: Some(job.scan_job_id.clone()),
                    creation_date: Some(creation_epoch),
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_stop_backup_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        job_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.stop_backup_job(job_id) {
            Ok(()) => wire::serialize_stop_backup_job_response(),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_backup_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_id: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let raw: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(Value::Null)
        };
        if raw.get("BackupPlan").is_none() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'BackupPlan'",
            );
        }
        let labels: &[(&str, &str)] = &[("BackupPlanId", plan_id)];
        let input = match wire::deserialize_update_backup_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let backup_plan = serde_json::to_value(&input.backup_plan).unwrap_or(Value::Null);

        let mut state = state.write().await;
        match state.update_backup_plan(plan_id, &backup_plan) {
            Ok(plan) => {
                let creation_epoch = plan.creation_date.timestamp() as f64
                    + (plan.creation_date.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_update_backup_plan_response(&wire::UpdateBackupPlanOutput {
                    backup_plan_id: Some(plan.backup_plan_id.clone()),
                    backup_plan_arn: Some(plan.backup_plan_arn.clone()),
                    creation_date: Some(creation_epoch),
                    version_id: Some(plan.version_id.clone()),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_framework(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("FrameworkName", name)];
        let input = match wire::deserialize_update_framework_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let description = input.framework_description.clone();
        let controls = input
            .framework_controls
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null));
        let mut state = state.write().await;
        match state.update_framework(name, description.as_deref(), controls) {
            Ok(fw) => {
                let creation_epoch = fw.creation_time.timestamp() as f64
                    + (fw.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_update_framework_response(&wire::UpdateFrameworkOutput {
                    framework_name: Some(fw.framework_name.clone()),
                    framework_arn: Some(fw.framework_arn.clone()),
                    creation_time: Some(creation_epoch),
                    ..Default::default()
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_global_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_global_settings_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let settings: HashMap<String, String> = input.global_settings.unwrap_or_default();
        let mut state = state.write().await;
        state.update_global_settings(settings);
        wire::serialize_update_global_settings_response()
    }

    async fn handle_update_recovery_point_index_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        recovery_point_arn: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[
            ("BackupVaultName", vault_name),
            ("RecoveryPointArn", recovery_point_arn),
        ];
        let input = match wire::deserialize_update_recovery_point_index_settings_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let index = if input.index.is_empty() {
            "ENABLED".to_string()
        } else {
            input.index.clone()
        };
        let state = state.read().await;
        match state.describe_recovery_point(vault_name, recovery_point_arn) {
            Ok(rp) => wire::serialize_update_recovery_point_index_settings_response(
                &wire::UpdateRecoveryPointIndexSettingsOutput {
                    backup_vault_name: Some(rp.backup_vault_name.clone()),
                    recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    index: Some(index),
                    index_status: Some("ACTIVE".to_string()),
                },
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_recovery_point_lifecycle(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        vault_name: &str,
        recovery_point_arn: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[
            ("BackupVaultName", vault_name),
            ("RecoveryPointArn", recovery_point_arn),
        ];
        let input =
            match wire::deserialize_update_recovery_point_lifecycle_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
            };
        let lifecycle_wire = input.lifecycle.clone();
        let state = state.read().await;
        match state.describe_recovery_point(vault_name, recovery_point_arn) {
            Ok(rp) => wire::serialize_update_recovery_point_lifecycle_response(
                &wire::UpdateRecoveryPointLifecycleOutput {
                    backup_vault_arn: Some(rp.backup_vault_arn.clone()),
                    recovery_point_arn: Some(rp.recovery_point_arn.clone()),
                    lifecycle: lifecycle_wire,
                    ..Default::default()
                },
            ),
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_region_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_region_settings_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let opt_in = input.resource_type_opt_in_preference.clone();
        let management = input.resource_type_management_preference.clone();
        let mut state = state.write().await;
        state.update_region_settings(opt_in, management);
        wire::serialize_update_region_settings_response()
    }

    async fn handle_update_report_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("ReportPlanName", name)];
        let input = match wire::deserialize_update_report_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let description = input.report_plan_description.clone();
        let delivery_channel = input
            .report_delivery_channel
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null));
        let report_setting = input
            .report_setting
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null));
        let mut state = state.write().await;
        match state.update_report_plan(
            name,
            description.as_deref(),
            delivery_channel,
            report_setting,
        ) {
            Ok(plan) => {
                let creation_epoch = plan.creation_time.timestamp() as f64
                    + (plan.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_update_report_plan_response(&wire::UpdateReportPlanOutput {
                    report_plan_name: Some(plan.report_plan_name.clone()),
                    report_plan_arn: Some(plan.report_plan_arn.clone()),
                    creation_time: Some(creation_epoch),
                })
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_restore_testing_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("RestoreTestingPlanName", plan_name)];
        let input =
            match wire::deserialize_update_restore_testing_plan_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
            };
        let plan_in = &input.restore_testing_plan;
        let schedule_expression = plan_in.schedule_expression.clone();
        let schedule_expression_timezone = plan_in.schedule_expression_timezone.clone();
        let start_window_hours = plan_in.start_window_hours;
        let recovery_point_selection = plan_in
            .recovery_point_selection
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null));

        let mut state = state.write().await;
        match state.update_restore_testing_plan(
            plan_name,
            schedule_expression.as_deref(),
            schedule_expression_timezone,
            start_window_hours,
            recovery_point_selection,
        ) {
            Ok(plan) => {
                let creation_epoch = plan.creation_time.timestamp() as f64
                    + (plan.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let update_epoch = plan.last_update_time.timestamp() as f64
                    + (plan.last_update_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_update_restore_testing_plan_response(
                    &wire::UpdateRestoreTestingPlanOutput {
                        restore_testing_plan_name: Some(plan.restore_testing_plan_name.clone()),
                        restore_testing_plan_arn: Some(plan.restore_testing_plan_arn.clone()),
                        creation_time: Some(creation_epoch),
                        update_time: Some(update_epoch),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_restore_testing_selection(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        plan_name: &str,
        selection_name: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[
            ("RestoreTestingPlanName", plan_name),
            ("RestoreTestingSelectionName", selection_name),
        ];
        let input = match wire::deserialize_update_restore_testing_selection_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
        };
        let sel_in = &input.restore_testing_selection;
        let iam_role_arn = sel_in.iam_role_arn.clone();
        let protected_resource_arns = sel_in.protected_resource_arns.clone();
        let protected_resource_conditions = sel_in
            .protected_resource_conditions
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null));
        let restore_metadata_overrides = sel_in.restore_metadata_overrides.clone();
        let validation_window_hours = sel_in.validation_window_hours;

        let mut state = state.write().await;
        match state.update_restore_testing_selection(
            plan_name,
            selection_name,
            iam_role_arn.as_deref(),
            protected_resource_arns,
            protected_resource_conditions,
            restore_metadata_overrides,
            validation_window_hours,
        ) {
            Ok(sel) => {
                let creation_epoch = sel.creation_time.timestamp() as f64
                    + (sel.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let update_epoch = sel.last_update_time.timestamp() as f64
                    + (sel.last_update_time.timestamp_subsec_millis() as f64 / 1000.0);
                // Need to get the plan ARN
                wire::serialize_update_restore_testing_selection_response(
                    &wire::UpdateRestoreTestingSelectionOutput {
                        restore_testing_selection_name: Some(
                            sel.restore_testing_selection_name.clone(),
                        ),
                        restore_testing_plan_name: Some(sel.restore_testing_plan_name.clone()),
                        restore_testing_plan_arn: Some(sel.restore_testing_plan_arn.clone()),
                        creation_time: Some(creation_epoch),
                        update_time: Some(update_epoch),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }

    async fn handle_update_tiering_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BackupState>>,
        name: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("TieringConfigurationName", name)];
        let input =
            match wire::deserialize_update_tiering_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterValueException", &e),
            };
        let vault_name = if input.tiering_configuration.backup_vault_name.is_empty() {
            None
        } else {
            Some(input.tiering_configuration.backup_vault_name.clone())
        };
        let resource_selection = if input.tiering_configuration.resource_selection.is_empty() {
            None
        } else {
            serde_json::to_value(&input.tiering_configuration.resource_selection).ok()
        };

        let mut state = state.write().await;
        match state.update_tiering_configuration(name, vault_name.as_deref(), resource_selection) {
            Ok(config) => {
                let creation_epoch = config.creation_time.timestamp() as f64
                    + (config.creation_time.timestamp_subsec_millis() as f64 / 1000.0);
                let updated_epoch = config.last_updated_time.timestamp() as f64
                    + (config.last_updated_time.timestamp_subsec_millis() as f64 / 1000.0);
                wire::serialize_update_tiering_configuration_response(
                    &wire::UpdateTieringConfigurationOutput {
                        tiering_configuration_name: Some(config.tiering_configuration_name.clone()),
                        tiering_configuration_arn: Some(config.tiering_configuration_arn.clone()),
                        creation_time: Some(creation_epoch),
                        last_updated_time: Some(updated_epoch),
                    },
                )
            }
            Err(e) => backup_error_response(&e),
        }
    }
}

fn backup_error_response(err: &BackupError) -> MockResponse {
    let (status, error_type) = match err {
        BackupError::VaultAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::VaultNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::PlanAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::PlanNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::ReportPlanAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::ReportPlanNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::FrameworkAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::FrameworkNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::SelectionNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::RecoveryPointNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::VaultAccessPolicyNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::VaultNotificationsNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::BackupJobNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::ReportJobNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::ScanJobNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::TieringConfigAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::TieringConfigNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::VaultNotLocked(_) => (400, "InvalidParameterValueException"),
        BackupError::BackupJobNotCancellable(_) => (400, "InvalidRequestException"),
        BackupError::LegalHoldAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::LegalHoldNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::CopyJobNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::RestoreJobNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::RestoreTestingPlanAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::RestoreTestingPlanNotFound(_) => (404, "ResourceNotFoundException"),
        BackupError::RestoreTestingSelectionAlreadyExists(_) => (400, "AlreadyExistsException"),
        BackupError::RestoreTestingSelectionNotFound(_) => (404, "ResourceNotFoundException"),
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
