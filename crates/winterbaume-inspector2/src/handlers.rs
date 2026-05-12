use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{Inspector2Error, Inspector2State};
use crate::views::Inspector2StateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct Inspector2Service {
    pub(crate) state: Arc<BackendState<Inspector2State>>,
    pub(crate) notifier: StateChangeNotifier<Inspector2StateView>,
}

impl Inspector2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for Inspector2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for Inspector2Service {
    fn service_name(&self) -> &str {
        "inspector2"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://inspector2\..*\.amazonaws\.com",
            r"https?://inspector2\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl Inspector2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = parse_query_multi(&raw_query);

        match (method, segments.as_slice()) {
            // POST /enable - Enable
            ("POST", ["enable"]) => {
                self.handle_enable(&state, &request, &[], &query_map, account_id)
                    .await
            }
            // POST /disable - Disable
            ("POST", ["disable"]) => {
                self.handle_disable(&state, &request, &[], &query_map, account_id)
                    .await
            }
            // POST /status/batch/get - BatchGetAccountStatus
            ("POST", ["status", "batch", "get"]) => {
                self.handle_batch_get_account_status(&state, &request, &[], &query_map, account_id)
                    .await
            }
            // POST /findings/list - ListFindings
            ("POST", ["findings", "list"]) => {
                self.handle_list_findings(&state, &request, &[], &query_map)
                    .await
            }
            // POST /members/associate => AssociateMember
            ("POST", ["members", "associate"]) => {
                self.handle_associate_member(&state, &request, &[], &query_map)
                    .await
            }
            // POST /members/disassociate => DisassociateMember
            ("POST", ["members", "disassociate"]) => {
                self.handle_disassociate_member(&state, &request, &[], &query_map)
                    .await
            }
            // POST /members/get => GetMember
            ("POST", ["members", "get"]) => {
                self.handle_get_member(&state, &request, &[], &query_map)
                    .await
            }
            // POST /filters/create => CreateFilter
            ("POST", ["filters", "create"]) => {
                self.handle_create_filter(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // POST /filters/delete => DeleteFilter
            ("POST", ["filters", "delete"]) => {
                self.handle_delete_filter(&state, &request, &[], &query_map)
                    .await
            }
            // POST /filters/list => ListFilters
            ("POST", ["filters", "list"]) => self.handle_list_filters(&state).await,
            // POST /organizationconfiguration/describe => DescribeOrganizationConfiguration
            ("POST", ["organizationconfiguration", "describe"]) => {
                self.handle_describe_organization_configuration(&state)
                    .await
            }
            // POST /delegatedadminaccounts/disable => DisableDelegatedAdminAccount
            ("POST", ["delegatedadminaccounts", "disable"]) => {
                self.handle_disable_delegated_admin_account(&state, &request, &[], &query_map)
                    .await
            }
            // POST /delegatedadminaccounts/enable => EnableDelegatedAdminAccount
            ("POST", ["delegatedadminaccounts", "enable"]) => {
                self.handle_enable_delegated_admin_account(&state, &request, &[], &query_map)
                    .await
            }
            // POST /delegatedadminaccounts/list => ListDelegatedAdminAccounts
            ("POST", ["delegatedadminaccounts", "list"]) => {
                self.handle_list_delegated_admin_accounts(&state).await
            }
            // POST /members/list => ListMembers
            ("POST", ["members", "list"]) => self.handle_list_members(&state).await,
            // GET /tags/{resourceArn} => ListTagsForResource
            ("GET", ["tags", ..]) => {
                let resource_arn = extract_tags_resource_arn(&request.uri);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            // POST /tags/{resourceArn} => TagResource
            ("POST", ["tags", ..]) => {
                let resource_arn = extract_tags_resource_arn(&request.uri);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /tags/{resourceArn} => UntagResource
            ("DELETE", ["tags", ..]) => {
                let resource_arn = extract_tags_resource_arn(&request.uri);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // POST /organizationconfiguration/update => UpdateOrganizationConfiguration
            ("POST", ["organizationconfiguration", "update"]) => {
                self.handle_update_organization_configuration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /codesecurity/scan-configuration/batch/associate => BatchAssociateCodeSecurityScanConfiguration
            ("POST", ["codesecurity", "scan-configuration", "batch", "associate"]) => {
                self.handle_batch_associate_code_security_scan_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // POST /codesecurity/scan-configuration/batch/disassociate => BatchDisassociateCodeSecurityScanConfiguration
            (
                "POST",
                [
                    "codesecurity",
                    "scan-configuration",
                    "batch",
                    "disassociate",
                ],
            ) => {
                self.handle_batch_disassociate_code_security_scan_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // POST /codesnippet/batchget => BatchGetCodeSnippet
            ("POST", ["codesnippet", "batchget"]) => {
                self.handle_batch_get_code_snippet(&state).await
            }
            // POST /findings/details/batch/get => BatchGetFindingDetails
            ("POST", ["findings", "details", "batch", "get"]) => {
                self.handle_batch_get_finding_details(&state).await
            }
            // POST /freetrialinfo/batchget => BatchGetFreeTrialInfo
            ("POST", ["freetrialinfo", "batchget"]) => {
                self.handle_batch_get_free_trial_info(&state).await
            }
            // POST /ec2deepinspectionstatus/member/batch/get => BatchGetMemberEc2DeepInspectionStatus
            ("POST", ["ec2deepinspectionstatus", "member", "batch", "get"]) => {
                self.handle_batch_get_member_ec2_deep_inspection_status(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // POST /ec2deepinspectionstatus/member/batch/update => BatchUpdateMemberEc2DeepInspectionStatus
            ("POST", ["ec2deepinspectionstatus", "member", "batch", "update"]) => {
                self.handle_batch_update_member_ec2_deep_inspection_status(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // POST /reporting/cancel => CancelFindingsReport
            ("POST", ["reporting", "cancel"]) => {
                self.handle_cancel_findings_report(&state, &request, &[], &query_map)
                    .await
            }
            // POST /sbomexport/cancel => CancelSbomExport
            ("POST", ["sbomexport", "cancel"]) => {
                self.handle_cancel_sbom_export(&state, &request, &[], &query_map)
                    .await
            }
            // POST /cis/scan-configuration/create => CreateCisScanConfiguration
            ("POST", ["cis", "scan-configuration", "create"]) => {
                self.handle_create_cis_scan_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // POST /codesecurity/integration/create => CreateCodeSecurityIntegration
            ("POST", ["codesecurity", "integration", "create"]) => {
                self.handle_create_code_security_integration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // POST /codesecurity/scan-configuration/create => CreateCodeSecurityScanConfiguration
            ("POST", ["codesecurity", "scan-configuration", "create"]) => {
                self.handle_create_code_security_scan_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // POST /reporting/create => CreateFindingsReport
            ("POST", ["reporting", "create"]) => {
                self.handle_create_findings_report(&state, &request, &[], &query_map)
                    .await
            }
            // POST /sbomexport/create => CreateSbomExport
            ("POST", ["sbomexport", "create"]) => {
                self.handle_create_sbom_export(&state, &request, &[], &query_map)
                    .await
            }
            // POST /cis/scan-configuration/delete => DeleteCisScanConfiguration
            ("POST", ["cis", "scan-configuration", "delete"]) => {
                self.handle_delete_cis_scan_configuration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /codesecurity/integration/delete => DeleteCodeSecurityIntegration
            ("POST", ["codesecurity", "integration", "delete"]) => {
                self.handle_delete_code_security_integration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /codesecurity/scan-configuration/delete => DeleteCodeSecurityScanConfiguration
            ("POST", ["codesecurity", "scan-configuration", "delete"]) => {
                self.handle_delete_code_security_scan_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // POST /cis/scan/report/get => GetCisScanReport
            ("POST", ["cis", "scan", "report", "get"]) => {
                self.handle_get_cis_scan_report(&state).await
            }
            // POST /cis/scan-result/details/get => GetCisScanResultDetails
            ("POST", ["cis", "scan-result", "details", "get"]) => {
                self.handle_get_cis_scan_result_details(&state).await
            }
            // POST /cluster/get => GetClustersForImage
            ("POST", ["cluster", "get"]) => self.handle_get_clusters_for_image(&state).await,
            // POST /codesecurity/integration/get => GetCodeSecurityIntegration
            ("POST", ["codesecurity", "integration", "get"]) => {
                self.handle_get_code_security_integration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /codesecurity/scan/get => GetCodeSecurityScan
            ("POST", ["codesecurity", "scan", "get"]) => {
                self.handle_get_code_security_scan(&state).await
            }
            // POST /codesecurity/scan-configuration/get => GetCodeSecurityScanConfiguration
            ("POST", ["codesecurity", "scan-configuration", "get"]) => {
                self.handle_get_code_security_scan_configuration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /configuration/get => GetConfiguration
            ("POST", ["configuration", "get"]) => self.handle_get_configuration(&state).await,
            // POST /delegatedadminaccounts/get => GetDelegatedAdminAccount
            ("POST", ["delegatedadminaccounts", "get"]) => {
                self.handle_get_delegated_admin_account(&state).await
            }
            // POST /ec2deepinspectionconfiguration/get => GetEc2DeepInspectionConfiguration
            ("POST", ["ec2deepinspectionconfiguration", "get"]) => {
                self.handle_get_ec2_deep_inspection_configuration(&state)
                    .await
            }
            // GET /encryptionkey/get => GetEncryptionKey
            ("GET", ["encryptionkey", "get"]) => self.handle_get_encryption_key(&state).await,
            // POST /reporting/status/get => GetFindingsReportStatus
            ("POST", ["reporting", "status", "get"]) => {
                self.handle_get_findings_report_status(&state, &request, &[], &query_map)
                    .await
            }
            // POST /sbomexport/get => GetSbomExport
            ("POST", ["sbomexport", "get"]) => {
                self.handle_get_sbom_export(&state, &request, &[], &query_map)
                    .await
            }
            // POST /accountpermissions/list => ListAccountPermissions
            ("POST", ["accountpermissions", "list"]) => {
                self.handle_list_account_permissions(&state).await
            }
            // POST /cis/scan-configuration/list => ListCisScanConfigurations
            ("POST", ["cis", "scan-configuration", "list"]) => {
                self.handle_list_cis_scan_configurations(&state).await
            }
            // POST /cis/scan-result/check/list => ListCisScanResultsAggregatedByChecks
            ("POST", ["cis", "scan-result", "check", "list"]) => {
                self.handle_list_cis_scan_results_aggregated_by_checks(&state)
                    .await
            }
            // POST /cis/scan-result/resource/list => ListCisScanResultsAggregatedByTargetResource
            ("POST", ["cis", "scan-result", "resource", "list"]) => {
                self.handle_list_cis_scan_results_aggregated_by_target_resource(&state)
                    .await
            }
            // POST /cis/scan/list => ListCisScans
            ("POST", ["cis", "scan", "list"]) => self.handle_list_cis_scans(&state).await,
            // POST /codesecurity/integration/list => ListCodeSecurityIntegrations
            ("POST", ["codesecurity", "integration", "list"]) => {
                self.handle_list_code_security_integrations(&state).await
            }
            // POST /codesecurity/scan-configuration/associations/list => ListCodeSecurityScanConfigurationAssociations
            ("POST", ["codesecurity", "scan-configuration", "associations", "list"]) => {
                self.handle_list_code_security_scan_configuration_associations(&state)
                    .await
            }
            // POST /codesecurity/scan-configuration/list => ListCodeSecurityScanConfigurations
            ("POST", ["codesecurity", "scan-configuration", "list"]) => {
                self.handle_list_code_security_scan_configurations(&state)
                    .await
            }
            // POST /coverage/list => ListCoverage
            ("POST", ["coverage", "list"]) => self.handle_list_coverage(&state).await,
            // POST /coverage/statistics/list => ListCoverageStatistics
            ("POST", ["coverage", "statistics", "list"]) => {
                self.handle_list_coverage_statistics(&state).await
            }
            // POST /findings/aggregation/list => ListFindingAggregations
            ("POST", ["findings", "aggregation", "list"]) => {
                self.handle_list_finding_aggregations(&state, &request, &[], &query_map)
                    .await
            }
            // POST /usage/list => ListUsageTotals
            ("POST", ["usage", "list"]) => self.handle_list_usage_totals(&state).await,
            // PUT /encryptionkey/reset => ResetEncryptionKey
            ("PUT", ["encryptionkey", "reset"]) => self.handle_reset_encryption_key(&state).await,
            // POST /vulnerabilities/search => SearchVulnerabilities
            ("POST", ["vulnerabilities", "search"]) => {
                self.handle_search_vulnerabilities(&state).await
            }
            // PUT /cissession/health/send => SendCisSessionHealth
            ("PUT", ["cissession", "health", "send"]) => {
                self.handle_send_cis_session_health(&state).await
            }
            // PUT /cissession/telemetry/send => SendCisSessionTelemetry
            ("PUT", ["cissession", "telemetry", "send"]) => {
                self.handle_send_cis_session_telemetry(&state).await
            }
            // PUT /cissession/start => StartCisSession
            ("PUT", ["cissession", "start"]) => self.handle_start_cis_session(&state).await,
            // POST /codesecurity/scan/start => StartCodeSecurityScan
            ("POST", ["codesecurity", "scan", "start"]) => {
                self.handle_start_code_security_scan(&state).await
            }
            // PUT /cissession/stop => StopCisSession
            ("PUT", ["cissession", "stop"]) => self.handle_stop_cis_session(&state).await,
            // POST /cis/scan-configuration/update => UpdateCisScanConfiguration
            ("POST", ["cis", "scan-configuration", "update"]) => {
                self.handle_update_cis_scan_configuration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /codesecurity/integration/update => UpdateCodeSecurityIntegration
            ("POST", ["codesecurity", "integration", "update"]) => {
                self.handle_update_code_security_integration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /codesecurity/scan-configuration/update => UpdateCodeSecurityScanConfiguration
            ("POST", ["codesecurity", "scan-configuration", "update"]) => {
                self.handle_update_code_security_scan_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // POST /configuration/update => UpdateConfiguration
            ("POST", ["configuration", "update"]) => {
                self.handle_update_configuration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /ec2deepinspectionconfiguration/update => UpdateEc2DeepInspectionConfiguration
            ("POST", ["ec2deepinspectionconfiguration", "update"]) => {
                self.handle_update_ec2_deep_inspection_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // PUT /encryptionkey/update => UpdateEncryptionKey
            ("PUT", ["encryptionkey", "update"]) => {
                self.handle_update_encryption_key(&state, &request, &[], &query_map)
                    .await
            }
            // POST /filters/update => UpdateFilter
            ("POST", ["filters", "update"]) => {
                self.handle_update_filter(&state, &request, &[], &query_map)
                    .await
            }
            // POST /ec2deepinspectionconfiguration/org/update => UpdateOrgEc2DeepInspectionConfiguration
            ("POST", ["ec2deepinspectionconfiguration", "org", "update"]) => {
                self.handle_update_org_ec2_deep_inspection_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_enable(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        default_account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_types = input.resource_types.clone();
        let mut account_ids = input.account_ids.clone().unwrap_or_default();
        if account_ids.is_empty() {
            account_ids.push(default_account_id.to_string());
        }

        let mut state = state.write().await;
        let mut accounts: Vec<wire::Account> = Vec::with_capacity(account_ids.len());
        for acct in &account_ids {
            match state.enable(&resource_types, acct) {
                Ok(result) => accounts.push(build_wire_account(&result)),
                Err(e) => return inspector2_error_response(&e),
            }
        }

        wire::serialize_enable_response(&wire::EnableResponse {
            accounts: Some(accounts),
            failed_accounts: Some(vec![]),
        })
    }

    async fn handle_disable(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        default_account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_types = input.resource_types.clone().unwrap_or_default();
        let mut account_ids = input.account_ids.clone().unwrap_or_default();
        if account_ids.is_empty() {
            account_ids.push(default_account_id.to_string());
        }

        let mut state = state.write().await;
        let mut accounts: Vec<wire::Account> = Vec::with_capacity(account_ids.len());
        for acct in &account_ids {
            match state.disable(&resource_types, acct) {
                Ok(result) => accounts.push(build_wire_account(&result)),
                Err(e) => return inspector2_error_response(&e),
            }
        }

        wire::serialize_disable_response(&wire::DisableResponse {
            accounts: Some(accounts),
            failed_accounts: Some(vec![]),
        })
    }

    async fn handle_batch_get_account_status(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        default_account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_account_status_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut account_ids = input.account_ids.clone().unwrap_or_default();
        if account_ids.is_empty() {
            account_ids.push(default_account_id.to_string());
        }

        let state = state.read().await;
        let mut accounts: Vec<wire::AccountState> = Vec::with_capacity(account_ids.len());
        for acct in &account_ids {
            let result = state.get_account_status(acct);
            let resource_state = build_wire_resource_state(&result.resource_status);
            accounts.push(wire::AccountState {
                account_id: Some(result.account_id.clone()),
                resource_state: Some(resource_state),
                state: Some(wire::State {
                    status: Some(result.status.clone()),
                    ..Default::default()
                }),
            });
        }

        wire::serialize_batch_get_account_status_response(&wire::BatchGetAccountStatusResponse {
            accounts: Some(accounts),
            failed_accounts: Some(vec![]),
        })
    }

    async fn handle_list_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (findings, _next) = state.list_findings(max_results, next_token);

        let wire_findings: Vec<wire::Finding> = findings
            .iter()
            .map(|f| wire::Finding {
                finding_arn: Some(f.finding_arn.clone()),
                aws_account_id: Some(f.aws_account_id.clone()),
                description: Some(f.description.clone()),
                first_observed_at: Some(f.first_observed_at.timestamp() as f64),
                last_observed_at: Some(f.last_observed_at.timestamp() as f64),
                updated_at: Some(f.updated_at.timestamp() as f64),
                severity: Some(f.severity.clone()),
                status: Some(f.status.clone()),
                title: Some(f.title.clone()),
                r#type: Some(f.finding_type.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_findings_response(&wire::ListFindingsResponse {
            findings: Some(wire_findings),
            next_token: _next,
        })
    }

    async fn handle_associate_member(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_member_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.account_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'accountId'");
        }

        let mut state = state.write().await;
        match state.associate_member(&input.account_id) {
            Ok(id) => wire::serialize_associate_member_response(&wire::AssociateMemberResponse {
                account_id: Some(id),
            }),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_create_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_filter_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.action.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'action'");
        }
        let description = input.description.as_deref();
        let tags = input.tags.clone();

        let mut state = state.write().await;
        match state.create_filter(
            &input.name,
            &input.action,
            description,
            account_id,
            region,
            tags,
        ) {
            Ok(filter) => wire::serialize_create_filter_response(&wire::CreateFilterResponse {
                arn: Some(filter.arn),
            }),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_delete_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_filter_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'arn'");
        }

        let mut state = state.write().await;
        match state.delete_filter(&input.arn) {
            Ok(arn) => wire::serialize_delete_filter_response(&wire::DeleteFilterResponse {
                arn: Some(arn),
            }),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_list_filters(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let filters = state.list_filters();
        let wire_filters: Vec<wire::Filter> = filters
            .iter()
            .map(|f| wire::Filter {
                arn: Some(f.arn.clone()),
                name: Some(f.name.clone()),
                action: Some(f.action.clone()),
                description: f.description.clone(),
                created_at: Some(f.created_at.timestamp() as f64),
                updated_at: Some(f.updated_at.timestamp() as f64),
                owner_id: Some(f.owner_id.clone()),
                tags: f.tags.clone(),
                criteria: Some(wire::FilterCriteria {
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_filters_response(&wire::ListFiltersResponse {
            filters: Some(wire_filters),
            ..Default::default()
        })
    }

    async fn handle_describe_organization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let config = state.describe_organization_configuration();

        let auto_enable = config.auto_enable.map(|ae| wire::AutoEnable {
            ec2: ae.ec2,
            ecr: ae.ecr,
            lambda: ae.lambda,
            lambda_code: ae.lambda_code,
            code_repository: ae.code_repository,
        });

        wire::serialize_describe_organization_configuration_response(
            &wire::DescribeOrganizationConfigurationResponse {
                auto_enable,
                max_account_limit_reached: Some(config.max_account_limit_reached),
            },
        )
    }

    async fn handle_disable_delegated_admin_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disable_delegated_admin_account_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.delegated_admin_account_id.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'delegatedAdminAccountId'",
            );
        }

        let mut state = state.write().await;
        match state.disable_delegated_admin(&input.delegated_admin_account_id) {
            Ok(id) => wire::serialize_disable_delegated_admin_account_response(
                &wire::DisableDelegatedAdminAccountResponse {
                    delegated_admin_account_id: Some(id),
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_enable_delegated_admin_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_delegated_admin_account_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.delegated_admin_account_id.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'delegatedAdminAccountId'",
            );
        }

        let mut state = state.write().await;
        match state.enable_delegated_admin(&input.delegated_admin_account_id) {
            Ok(id) => wire::serialize_enable_delegated_admin_account_response(
                &wire::EnableDelegatedAdminAccountResponse {
                    delegated_admin_account_id: Some(id),
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_list_delegated_admin_accounts(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let accounts = state.list_delegated_admin_accounts();
        let wire_accounts: Vec<wire::DelegatedAdminAccount> = accounts
            .iter()
            .map(|a| wire::DelegatedAdminAccount {
                account_id: Some(a.account_id.clone()),
                status: Some(a.status.clone()),
            })
            .collect();

        wire::serialize_list_delegated_admin_accounts_response(
            &wire::ListDelegatedAdminAccountsResponse {
                delegated_admin_accounts: Some(wire_accounts),
                ..Default::default()
            },
        )
    }

    async fn handle_disassociate_member(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_member_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.account_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'accountId'");
        }

        let mut state = state.write().await;
        match state.disassociate_member(&input.account_id) {
            Ok(id) => {
                wire::serialize_disassociate_member_response(&wire::DisassociateMemberResponse {
                    account_id: Some(id),
                })
            }
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_get_member(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_member_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.account_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'accountId'");
        }

        let state = state.read().await;
        match state.get_member(&input.account_id) {
            Ok(m) => wire::serialize_get_member_response(&wire::GetMemberResponse {
                member: Some(wire::Member {
                    account_id: Some(m.account_id.clone()),
                    relationship_status: Some(m.relationship_status.clone()),
                    updated_at: Some(m.updated_at.timestamp() as f64),
                    ..Default::default()
                }),
            }),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_list_members(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let members = state.list_members();
        let wire_members: Vec<wire::Member> = members
            .iter()
            .map(|m| wire::Member {
                account_id: Some(m.account_id.clone()),
                relationship_status: Some(m.relationship_status.clone()),
                updated_at: Some(m.updated_at.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_members_response(&wire::ListMembersResponse {
            members: Some(wire_members),
            ..Default::default()
        })
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        let tags_opt = if tags.is_empty() { None } else { Some(tags) };

        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: tags_opt,
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
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
            Ok(()) => {
                let body = "{}".to_string();
                MockResponse::rest_json(200, body)
            }
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => {
                let body = "{}".to_string();
                MockResponse::rest_json(200, body)
            }
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_update_organization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_organization_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let auto_enable = &input.auto_enable;

        let config = crate::types::AutoEnableConfig {
            ec2: auto_enable.ec2,
            ecr: auto_enable.ecr,
            lambda: auto_enable.lambda,
            lambda_code: auto_enable.lambda_code,
            code_repository: auto_enable.code_repository,
        };

        let mut state = state.write().await;
        let result = state.update_organization_configuration(config);

        wire::serialize_update_organization_configuration_response(
            &wire::UpdateOrganizationConfigurationResponse {
                auto_enable: Some(wire::AutoEnable {
                    ec2: result.ec2,
                    ecr: result.ecr,
                    lambda: result.lambda,
                    lambda_code: result.lambda_code,
                    code_repository: result.code_repository,
                }),
            },
        )
    }

    async fn handle_update_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_filter_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.filter_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'filterArn'");
        }
        let action = input.action.as_deref();
        let description = input.description.as_deref();
        let name = input.name.as_deref();

        let mut state = state.write().await;
        match state.update_filter(&input.filter_arn, action, description, name) {
            Ok(arn) => wire::serialize_update_filter_response(&wire::UpdateFilterResponse {
                arn: Some(arn),
            }),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_get_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let (ec2_scan_mode, ecr_rescan_duration) = state.get_configuration();
        wire::serialize_get_configuration_response(&wire::GetConfigurationResponse {
            ec2_configuration: Some(wire::Ec2ConfigurationState {
                scan_mode_state: Some(wire::Ec2ScanModeState {
                    scan_mode: if ec2_scan_mode.is_empty() {
                        None
                    } else {
                        Some(ec2_scan_mode.to_string())
                    },
                    ..Default::default()
                }),
            }),
            ecr_configuration: Some(wire::EcrConfigurationState {
                rescan_duration_state: Some(wire::EcrRescanDurationState {
                    rescan_duration: if ecr_rescan_duration.is_empty() {
                        None
                    } else {
                        Some(ecr_rescan_duration.to_string())
                    },
                    ..Default::default()
                }),
            }),
        })
    }

    async fn handle_update_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_configuration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let ec2_scan_mode = input.ec2_configuration.as_ref().and_then(|c| {
            if c.scan_mode.is_empty() {
                None
            } else {
                Some(c.scan_mode.clone())
            }
        });
        let ecr_rescan_duration = input.ecr_configuration.as_ref().and_then(|c| {
            if c.rescan_duration.is_empty() {
                None
            } else {
                Some(c.rescan_duration.clone())
            }
        });

        let mut state = state.write().await;
        state.update_configuration(ec2_scan_mode, ecr_rescan_duration);
        wire::serialize_update_configuration_response(&wire::UpdateConfigurationResponse {})
    }

    async fn handle_get_delegated_admin_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_delegated_admin_account() {
            Some((account_id, relationship_status)) => {
                wire::serialize_get_delegated_admin_account_response(
                    &wire::GetDelegatedAdminAccountResponse {
                        delegated_admin: Some(wire::DelegatedAdmin {
                            account_id: Some(account_id.to_string()),
                            relationship_status: Some(relationship_status.to_string()),
                        }),
                    },
                )
            }
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                "No delegated admin account found",
            ),
        }
    }

    async fn handle_get_ec2_deep_inspection_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let config = state.get_ec2_deep_inspection_configuration();
        let status = if config.activate_deep_inspection {
            "ACTIVATED"
        } else {
            "DEACTIVATED"
        };
        wire::serialize_get_ec2_deep_inspection_configuration_response(
            &wire::GetEc2DeepInspectionConfigurationResponse {
                package_paths: if config.package_paths.is_empty() {
                    None
                } else {
                    Some(config.package_paths.clone())
                },
                status: Some(status.to_string()),
                ..Default::default()
            },
        )
    }

    async fn handle_update_ec2_deep_inspection_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_ec2_deep_inspection_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let activate = input.activate_deep_inspection;
        let package_paths = input.package_paths.clone();

        let mut s = state.write().await;
        s.update_ec2_deep_inspection_configuration(activate, package_paths);
        let config = s.get_ec2_deep_inspection_configuration();
        let status = if config.activate_deep_inspection {
            "ACTIVATED"
        } else {
            "DEACTIVATED"
        };
        wire::serialize_update_ec2_deep_inspection_configuration_response(
            &wire::UpdateEc2DeepInspectionConfigurationResponse {
                package_paths: if config.package_paths.is_empty() {
                    None
                } else {
                    Some(config.package_paths.clone())
                },
                status: Some(status.to_string()),
                ..Default::default()
            },
        )
    }

    async fn handle_update_org_ec2_deep_inspection_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_org_ec2_deep_inspection_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let org_package_paths = input.org_package_paths.clone();

        let mut s = state.write().await;
        s.update_org_ec2_deep_inspection_configuration(org_package_paths);
        wire::serialize_update_org_ec2_deep_inspection_configuration_response(
            &wire::UpdateOrgEc2DeepInspectionConfigurationResponse {},
        )
    }

    async fn handle_get_encryption_key(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let key = state.get_encryption_key().to_string();
        wire::serialize_get_encryption_key_response(&wire::GetEncryptionKeyResponse {
            kms_key_id: Some(key),
        })
    }

    async fn handle_update_encryption_key(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_encryption_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.kms_key_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'kmsKeyId'");
        }
        let mut s = state.write().await;
        s.update_encryption_key(&input.kms_key_id);
        wire::serialize_update_encryption_key_response(&wire::UpdateEncryptionKeyResponse {})
    }

    async fn handle_create_findings_report(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_findings_report_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let report_format = if input.report_format.is_empty() {
            "JSON"
        } else {
            input.report_format.as_str()
        };
        let bucket_name = input.s3_destination.bucket_name.as_str();
        let key_prefix = input.s3_destination.key_prefix.as_deref().unwrap_or("");

        let mut s = state.write().await;
        let report_id = s.create_findings_report(report_format, bucket_name, key_prefix);
        wire::serialize_create_findings_report_response(&wire::CreateFindingsReportResponse {
            report_id: Some(report_id),
        })
    }

    async fn handle_get_findings_report_status(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_findings_report_status_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let report_id = input.report_id.as_deref();
        let state = state.read().await;
        match state.get_findings_report_status(report_id) {
            Some(r) => wire::serialize_get_findings_report_status_response(
                &wire::GetFindingsReportStatusResponse {
                    report_id: Some(r.report_id.clone()),
                    status: Some(r.status.clone()),
                    ..Default::default()
                },
            ),
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                "Findings report not found",
            ),
        }
    }

    async fn handle_cancel_findings_report(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_findings_report_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.report_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'reportId'");
        }
        let mut s = state.write().await;
        match s.cancel_findings_report(&input.report_id) {
            Ok(id) => wire::serialize_cancel_findings_report_response(
                &wire::CancelFindingsReportResponse {
                    report_id: Some(id),
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_create_sbom_export(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_sbom_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let report_format = if input.report_format.is_empty() {
            "CYCLONEDX_1_4"
        } else {
            input.report_format.as_str()
        };
        let bucket_name = input.s3_destination.bucket_name.as_str();
        let key_prefix = input.s3_destination.key_prefix.as_deref().unwrap_or("");

        let mut s = state.write().await;
        let report_id = s.create_sbom_export(report_format, bucket_name, key_prefix);
        wire::serialize_create_sbom_export_response(&wire::CreateSbomExportResponse {
            report_id: Some(report_id),
        })
    }

    async fn handle_get_sbom_export(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_sbom_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.report_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'reportId'");
        }
        let state = state.read().await;
        match state.get_sbom_export(&input.report_id) {
            Some(r) => wire::serialize_get_sbom_export_response(&wire::GetSbomExportResponse {
                report_id: Some(r.report_id.clone()),
                status: Some(r.status.clone()),
                format: Some(r.report_format.clone()),
                ..Default::default()
            }),
            None => rest_json_error(404, "ResourceNotFoundException", "SBOM export not found"),
        }
    }

    async fn handle_cancel_sbom_export(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_sbom_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.report_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'reportId'");
        }
        let mut s = state.write().await;
        match s.cancel_sbom_export(&input.report_id) {
            Ok(id) => {
                wire::serialize_cancel_sbom_export_response(&wire::CancelSbomExportResponse {
                    report_id: Some(id),
                })
            }
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_create_cis_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_cis_scan_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.scan_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'scanName'");
        }
        let tags = input.tags.clone();

        let mut s = state.write().await;
        let arn = s.create_cis_scan_configuration(&input.scan_name, account_id, region, tags);
        wire::serialize_create_cis_scan_configuration_response(
            &wire::CreateCisScanConfigurationResponse {
                scan_configuration_arn: Some(arn),
            },
        )
    }

    async fn handle_delete_cis_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_cis_scan_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.scan_configuration_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'scanConfigurationArn'");
        }
        let mut s = state.write().await;
        match s.delete_cis_scan_configuration(&input.scan_configuration_arn) {
            Ok(arn) => wire::serialize_delete_cis_scan_configuration_response(
                &wire::DeleteCisScanConfigurationResponse {
                    scan_configuration_arn: Some(arn),
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_list_cis_scan_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_cis_scan_configurations();
        let wire_configs: Vec<wire::CisScanConfiguration> = configs
            .iter()
            .map(|c| wire::CisScanConfiguration {
                scan_configuration_arn: Some(c.scan_configuration_arn.clone()),
                scan_name: Some(c.scan_name.clone()),
                owner_id: Some(c.owner_id.clone()),
                tags: c.tags.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_cis_scan_configurations_response(
            &wire::ListCisScanConfigurationsResponse {
                scan_configurations: Some(wire_configs),
                ..Default::default()
            },
        )
    }

    async fn handle_update_cis_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_cis_scan_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.scan_configuration_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'scanConfigurationArn'");
        }
        let scan_name = input.scan_name.as_deref();

        let mut s = state.write().await;
        match s.update_cis_scan_configuration(&input.scan_configuration_arn, scan_name) {
            Ok(arn) => wire::serialize_update_cis_scan_configuration_response(
                &wire::UpdateCisScanConfigurationResponse {
                    scan_configuration_arn: Some(arn),
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_batch_associate_code_security_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_associate_code_security_scan_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let scan_configuration_arn = input
            .associate_configuration_requests
            .first()
            .map(|r| r.scan_configuration_arn.as_str())
            .unwrap_or("");
        let resource_ids: Vec<String> = input
            .associate_configuration_requests
            .iter()
            .filter_map(|r| r.resource.project_id.clone())
            .collect();

        let mut s = state.write().await;
        let _failed = s.batch_associate_code_security_scan_configuration(
            scan_configuration_arn,
            &resource_ids,
        );
        wire::serialize_batch_associate_code_security_scan_configuration_response(
            &wire::BatchAssociateCodeSecurityScanConfigurationResponse {
                ..Default::default()
            },
        )
    }

    async fn handle_batch_disassociate_code_security_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_disassociate_code_security_scan_configuration_request(
                request, labels, query,
            ) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let scan_configuration_arn = input
            .disassociate_configuration_requests
            .first()
            .map(|r| r.scan_configuration_arn.as_str())
            .unwrap_or("");
        let resource_ids: Vec<String> = input
            .disassociate_configuration_requests
            .iter()
            .filter_map(|r| r.resource.project_id.clone())
            .collect();

        let mut s = state.write().await;
        let _failed = s.batch_disassociate_code_security_scan_configuration(
            scan_configuration_arn,
            &resource_ids,
        );
        wire::serialize_batch_disassociate_code_security_scan_configuration_response(
            &wire::BatchDisassociateCodeSecurityScanConfigurationResponse {
                ..Default::default()
            },
        )
    }

    // STUB[no-engine]: Code snippet retrieval requires a real scan engine; mock always returns empty.
    async fn handle_batch_get_code_snippet(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_batch_get_code_snippet_response(&wire::BatchGetCodeSnippetResponse {
            ..Default::default()
        })
    }

    // STUB[no-engine]: Finding details require a real scan engine; mock always returns empty.
    async fn handle_batch_get_finding_details(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_batch_get_finding_details_response(&wire::BatchGetFindingDetailsResponse {
            ..Default::default()
        })
    }

    // STUB[no-telemetry]: Free trial billing information is driven by real AWS account state; mock always returns empty.
    async fn handle_batch_get_free_trial_info(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_batch_get_free_trial_info_response(&wire::BatchGetFreeTrialInfoResponse {
            ..Default::default()
        })
    }

    async fn handle_batch_get_member_ec2_deep_inspection_status(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_member_ec2_deep_inspection_status_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let account_ids = input.account_ids.clone().unwrap_or_default();

        let s = state.read().await;
        let statuses = s.batch_get_member_ec2_deep_inspection_status(&account_ids);
        let wire_statuses: Vec<wire::MemberAccountEc2DeepInspectionStatusState> = statuses
            .iter()
            .map(|st| wire::MemberAccountEc2DeepInspectionStatusState {
                account_id: Some(st.account_id.clone()),
                status: Some(st.status.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_batch_get_member_ec2_deep_inspection_status_response(
            &wire::BatchGetMemberEc2DeepInspectionStatusResponse {
                account_ids: Some(wire_statuses),
                failed_account_ids: Some(vec![]),
            },
        )
    }

    async fn handle_batch_update_member_ec2_deep_inspection_status(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_member_ec2_deep_inspection_status_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let updates: Vec<(String, bool)> = input
            .account_ids
            .iter()
            .map(|m| (m.account_id.clone(), m.activate_deep_inspection))
            .collect();

        let mut s = state.write().await;
        let account_ids = s.batch_update_member_ec2_deep_inspection_status(&updates);
        let wire_statuses: Vec<wire::MemberAccountEc2DeepInspectionStatusState> = account_ids
            .iter()
            .map(|id| wire::MemberAccountEc2DeepInspectionStatusState {
                account_id: Some(id.clone()),
                status: Some("ACTIVATED".to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_batch_update_member_ec2_deep_inspection_status_response(
            &wire::BatchUpdateMemberEc2DeepInspectionStatusResponse {
                account_ids: Some(wire_statuses),
                failed_account_ids: Some(vec![]),
            },
        )
    }

    async fn handle_create_code_security_integration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_code_security_integration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            "default"
        } else {
            input.name.as_str()
        };

        let mut s = state.write().await;
        let integration = s.create_code_security_integration(name, account_id, region);
        wire::serialize_create_code_security_integration_response(
            &wire::CreateCodeSecurityIntegrationResponse {
                integration_arn: Some(integration.integration_arn),
                status: Some(integration.status),
                ..Default::default()
            },
        )
    }

    async fn handle_create_code_security_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_code_security_scan_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            "default"
        } else {
            input.name.as_str()
        };

        let mut s = state.write().await;
        let config = s.create_code_security_scan_configuration(name, account_id, region);
        wire::serialize_create_code_security_scan_configuration_response(
            &wire::CreateCodeSecurityScanConfigurationResponse {
                scan_configuration_arn: Some(config.scan_configuration_arn),
            },
        )
    }

    async fn handle_delete_code_security_integration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_code_security_integration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let integration_arn = input.integration_arn.as_str();

        let mut s = state.write().await;
        match s.delete_code_security_integration(integration_arn) {
            Ok(()) => wire::serialize_delete_code_security_integration_response(
                &wire::DeleteCodeSecurityIntegrationResponse {
                    ..Default::default()
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_delete_code_security_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_code_security_scan_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let scan_configuration_arn = input.scan_configuration_arn.as_str();

        let mut s = state.write().await;
        match s.delete_code_security_scan_configuration(scan_configuration_arn) {
            Ok(()) => wire::serialize_delete_code_security_scan_configuration_response(
                &wire::DeleteCodeSecurityScanConfigurationResponse {
                    ..Default::default()
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    // STUB[no-engine]: CIS scan reports require a real scan engine; mock always returns empty.
    async fn handle_get_cis_scan_report(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_get_cis_scan_report_response(&wire::GetCisScanReportResponse {
            ..Default::default()
        })
    }

    // STUB[no-engine]: CIS scan result details require a real scan engine; mock always returns empty.
    async fn handle_get_cis_scan_result_details(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_get_cis_scan_result_details_response(
            &wire::GetCisScanResultDetailsResponse {
                ..Default::default()
            },
        )
    }

    // STUB[no-engine]: Cluster-for-image lookup requires a real container infrastructure; mock always returns empty.
    async fn handle_get_clusters_for_image(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_get_clusters_for_image_response(&wire::GetClustersForImageResponse {
            ..Default::default()
        })
    }

    async fn handle_get_code_security_integration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_code_security_integration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let integration_arn = input.integration_arn.as_str();

        let s = state.read().await;
        match s.get_code_security_integration(integration_arn) {
            Ok(integration) => wire::serialize_get_code_security_integration_response(
                &wire::GetCodeSecurityIntegrationResponse {
                    integration_arn: Some(integration.integration_arn.clone()),
                    name: Some(integration.name.clone()),
                    status: Some(integration.status.clone()),
                    created_on: Some(integration.created_at.timestamp() as f64),
                    last_update_on: Some(integration.updated_at.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    // STUB[no-engine]: Code security scan results require a real scan engine; mock always returns empty.
    async fn handle_get_code_security_scan(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_get_code_security_scan_response(&wire::GetCodeSecurityScanResponse {
            ..Default::default()
        })
    }

    async fn handle_get_code_security_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_code_security_scan_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let scan_configuration_arn = input.scan_configuration_arn.as_str();

        let s = state.read().await;
        match s.get_code_security_scan_configuration(scan_configuration_arn) {
            Ok(config) => wire::serialize_get_code_security_scan_configuration_response(
                &wire::GetCodeSecurityScanConfigurationResponse {
                    scan_configuration_arn: Some(config.scan_configuration_arn.clone()),
                    name: Some(config.name.clone()),
                    created_at: Some(config.created_at.timestamp() as f64),
                    last_updated_at: Some(config.updated_at.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    // STUB[no-auth]: IAM permission listing requires a real auth engine; mock always returns empty.
    async fn handle_list_account_permissions(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_list_account_permissions_response(&wire::ListAccountPermissionsResponse {
            permissions: Some(vec![]),
            ..Default::default()
        })
    }

    // STUB[no-engine]: CIS scan check aggregation requires a real scan engine; mock always returns empty.
    async fn handle_list_cis_scan_results_aggregated_by_checks(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_list_cis_scan_results_aggregated_by_checks_response(
            &wire::ListCisScanResultsAggregatedByChecksResponse {
                ..Default::default()
            },
        )
    }

    // STUB[no-engine]: CIS scan resource aggregation requires a real scan engine; mock always returns empty.
    async fn handle_list_cis_scan_results_aggregated_by_target_resource(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_list_cis_scan_results_aggregated_by_target_resource_response(
            &wire::ListCisScanResultsAggregatedByTargetResourceResponse {
                ..Default::default()
            },
        )
    }

    // STUB[no-engine]: CIS scan history requires a real scan engine; mock always returns empty.
    async fn handle_list_cis_scans(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_list_cis_scans_response(&wire::ListCisScansResponse {
            ..Default::default()
        })
    }

    async fn handle_list_code_security_integrations(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let s = state.read().await;
        let integrations = s.list_code_security_integrations();
        let wire_integrations: Vec<wire::CodeSecurityIntegrationSummary> = integrations
            .iter()
            .map(|i| wire::CodeSecurityIntegrationSummary {
                integration_arn: Some(i.integration_arn.clone()),
                name: Some(i.name.clone()),
                status: Some(i.status.clone()),
                created_on: Some(i.created_at.timestamp() as f64),
                last_update_on: Some(i.updated_at.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_code_security_integrations_response(
            &wire::ListCodeSecurityIntegrationsResponse {
                integrations: if wire_integrations.is_empty() {
                    None
                } else {
                    Some(wire_integrations)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_code_security_scan_configuration_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let s = state.read().await;
        let associations = s.list_code_security_scan_configuration_associations();
        let wire_associations: Vec<wire::CodeSecurityScanConfigurationAssociationSummary> =
            associations
                .iter()
                .map(|a| wire::CodeSecurityScanConfigurationAssociationSummary {
                    resource: Some(wire::CodeSecurityResource {
                        project_id: Some(a.resource_id.clone()),
                    }),
                })
                .collect();
        wire::serialize_list_code_security_scan_configuration_associations_response(
            &wire::ListCodeSecurityScanConfigurationAssociationsResponse {
                associations: if wire_associations.is_empty() {
                    None
                } else {
                    Some(wire_associations)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_list_code_security_scan_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let s = state.read().await;
        let configs = s.list_code_security_scan_configurations();
        let wire_configs: Vec<wire::CodeSecurityScanConfigurationSummary> = configs
            .iter()
            .map(|c| wire::CodeSecurityScanConfigurationSummary {
                scan_configuration_arn: Some(c.scan_configuration_arn.clone()),
                name: Some(c.name.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_code_security_scan_configurations_response(
            &wire::ListCodeSecurityScanConfigurationsResponse {
                configurations: if wire_configs.is_empty() {
                    None
                } else {
                    Some(wire_configs)
                },
                ..Default::default()
            },
        )
    }

    // STUB[no-engine]: Resource coverage requires a real scan engine; mock always returns empty.
    async fn handle_list_coverage(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_list_coverage_response(&wire::ListCoverageResponse {
            covered_resources: Some(vec![]),
            ..Default::default()
        })
    }

    // STUB[no-engine]: Coverage statistics require a real scan engine; mock always returns zero.
    async fn handle_list_coverage_statistics(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_list_coverage_statistics_response(&wire::ListCoverageStatisticsResponse {
            total_counts: Some(0),
            ..Default::default()
        })
    }

    // STUB[no-engine]: Finding aggregations require a real scan engine; mock always returns empty.
    async fn handle_list_finding_aggregations(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_finding_aggregations_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let aggregation_type = input.aggregation_type.clone();
        wire::serialize_list_finding_aggregations_response(&wire::ListFindingAggregationsResponse {
            aggregation_type: if aggregation_type.is_empty() {
                None
            } else {
                Some(aggregation_type)
            },
            responses: Some(vec![]),
            ..Default::default()
        })
    }

    // STUB[no-telemetry]: Usage billing totals are driven by real AWS account data; mock always returns empty.
    async fn handle_list_usage_totals(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_list_usage_totals_response(&wire::ListUsageTotalsResponse {
            totals: Some(vec![]),
            ..Default::default()
        })
    }

    async fn handle_reset_encryption_key(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        s.reset_encryption_key();
        wire::serialize_reset_encryption_key_response(&wire::ResetEncryptionKeyResponse {})
    }

    // STUB[no-engine]: Vulnerability search requires a real vulnerability database; mock always returns empty.
    async fn handle_search_vulnerabilities(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_search_vulnerabilities_response(&wire::SearchVulnerabilitiesResponse {
            vulnerabilities: Some(vec![]),
            ..Default::default()
        })
    }

    // STUB[no-engine]: CIS session health reporting requires a real scan engine; mock accepts and discards.
    async fn handle_send_cis_session_health(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_send_cis_session_health_response(&wire::SendCisSessionHealthResponse {})
    }

    // STUB[no-engine]: CIS session telemetry requires a real scan engine; mock accepts and discards.
    async fn handle_send_cis_session_telemetry(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_send_cis_session_telemetry_response(
            &wire::SendCisSessionTelemetryResponse {},
        )
    }

    // STUB[no-engine]: CIS session lifecycle requires a real scan engine; mock accepts and discards.
    async fn handle_start_cis_session(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_start_cis_session_response(&wire::StartCisSessionResponse {})
    }

    // STUB[no-engine]: Code security scan execution requires a real scan engine; mock returns a generated scan ID.
    async fn handle_start_code_security_scan(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_start_code_security_scan_response(&wire::StartCodeSecurityScanResponse {
            scan_id: Some(uuid::Uuid::new_v4().to_string()),
            ..Default::default()
        })
    }

    // STUB[no-engine]: CIS session lifecycle requires a real scan engine; mock accepts and discards.
    async fn handle_stop_cis_session(
        &self,
        _state: &Arc<tokio::sync::RwLock<Inspector2State>>,
    ) -> MockResponse {
        wire::serialize_stop_cis_session_response(&wire::StopCisSessionResponse {})
    }

    async fn handle_update_code_security_integration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_code_security_integration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let integration_arn = input.integration_arn.as_str();

        let mut s = state.write().await;
        match s.update_code_security_integration(integration_arn) {
            Ok(_) => wire::serialize_update_code_security_integration_response(
                &wire::UpdateCodeSecurityIntegrationResponse {
                    ..Default::default()
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }

    async fn handle_update_code_security_scan_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Inspector2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_code_security_scan_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let scan_configuration_arn = input.scan_configuration_arn.as_str();

        let mut s = state.write().await;
        match s.update_code_security_scan_configuration(scan_configuration_arn) {
            Ok(_) => wire::serialize_update_code_security_scan_configuration_response(
                &wire::UpdateCodeSecurityScanConfigurationResponse {
                    ..Default::default()
                },
            ),
            Err(e) => inspector2_error_response(&e),
        }
    }
}

fn build_wire_account(result: &crate::state::AccountEnableResult) -> wire::Account {
    wire::Account {
        account_id: Some(result.account_id.clone()),
        resource_status: Some(wire::ResourceStatus {
            ec2: Some(
                result
                    .resource_status
                    .get("EC2")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            ecr: Some(
                result
                    .resource_status
                    .get("ECR")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            lambda: Some(
                result
                    .resource_status
                    .get("LAMBDA")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            lambda_code: Some(
                result
                    .resource_status
                    .get("LAMBDA_CODE")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            code_repository: Some(
                result
                    .resource_status
                    .get("CODE_REPOSITORY")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
        }),
        status: Some(result.status.clone()),
    }
}

fn build_wire_resource_state(
    resource_status: &std::collections::HashMap<String, String>,
) -> wire::ResourceState {
    wire::ResourceState {
        ec2: Some(wire::State {
            status: Some(
                resource_status
                    .get("EC2")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            ..Default::default()
        }),
        ecr: Some(wire::State {
            status: Some(
                resource_status
                    .get("ECR")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            ..Default::default()
        }),
        lambda: Some(wire::State {
            status: Some(
                resource_status
                    .get("LAMBDA")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            ..Default::default()
        }),
        lambda_code: Some(wire::State {
            status: Some(
                resource_status
                    .get("LAMBDA_CODE")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            ..Default::default()
        }),
        code_repository: Some(wire::State {
            status: Some(
                resource_status
                    .get("CODE_REPOSITORY")
                    .cloned()
                    .unwrap_or_else(|| "DISABLED".to_string()),
            ),
            ..Default::default()
        }),
    }
}

fn extract_path(uri: &str) -> String {
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))
        .unwrap_or(uri);
    let slash = after_scheme.find('/').unwrap_or(after_scheme.len());
    after_scheme[slash..]
        .split('?')
        .next()
        .unwrap_or("/")
        .to_string()
}

fn inspector2_error_response(err: &Inspector2Error) -> MockResponse {
    let (status, error_type) = match err {
        Inspector2Error::ValidationResourceTypesEmpty => (400u16, "ValidationException"),
        Inspector2Error::ValidationInvalidResourceType(_) => (400, "ValidationException"),
        Inspector2Error::ValidationAccountIdEmpty => (400, "ValidationException"),
        Inspector2Error::ValidationNameEmpty => (400, "ValidationException"),
        Inspector2Error::ValidationDelegatedAdminAccountIdEmpty => (400, "ValidationException"),
        Inspector2Error::FilterNotFound(_) => (404, "ResourceNotFoundException"),
        Inspector2Error::DelegatedAdminAccountNotFound(_) => (404, "ResourceNotFoundException"),
        Inspector2Error::MemberAccountNotFound(_) => (404, "ResourceNotFoundException"),
        Inspector2Error::ReportNotFound(_) => (404, "ResourceNotFoundException"),
        Inspector2Error::SbomExportNotFound(_) => (404, "ResourceNotFoundException"),
        Inspector2Error::CisScanConfigurationNotFound(_) => (404, "ResourceNotFoundException"),
        Inspector2Error::CodeSecurityIntegrationNotFound(_) => (404, "ResourceNotFoundException"),
        Inspector2Error::CodeSecurityScanConfigNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

fn extract_query(uri: &str) -> String {
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}

/// Parse a query string that may contain repeated keys, joining duplicate
/// values with commas. The wire deserialisers split list-typed query
/// parameters on commas, so multi-value query strings such as
/// `tagKeys=a&tagKeys=b` need to be normalised to `tagKeys=a,b`.
fn parse_query_multi(s: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    if s.is_empty() {
        return map;
    }
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urlencoding_decode(key);
            let value = urlencoding_decode(value);
            map.entry(key)
                .and_modify(|existing| {
                    existing.push(',');
                    existing.push_str(&value);
                })
                .or_insert(value);
        }
    }
    map
}

/// Extract the resource ARN from a /tags/{resourceArn} path.
/// The resourceArn is URL-encoded and occupies everything after /tags/.
fn extract_tags_resource_arn(uri: &str) -> String {
    let path = extract_path(uri);
    let prefix = "/tags/";
    if let Some(rest) = path.strip_prefix(prefix) {
        // URL-decode the ARN
        urlencoding_decode(rest)
    } else {
        path
    }
}

fn urlencoding_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().unwrap_or(b'0');
            let lo = chars.next().unwrap_or(b'0');
            let hex = [hi, lo];
            if let Ok(s) = std::str::from_utf8(&hex) {
                if let Ok(val) = u8::from_str_radix(s, 16) {
                    result.push(val as char);
                    continue;
                }
            }
            result.push('%');
            result.push(hi as char);
            result.push(lo as char);
        } else {
            result.push(b as char);
        }
    }
    result
}
