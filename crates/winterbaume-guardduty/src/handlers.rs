use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{GuardDutyError, GuardDutyState};
use crate::types::*;
use crate::views::GuardDutyStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct GuardDutyService {
    pub(crate) state: Arc<BackendState<GuardDutyState>>,
    pub(crate) notifier: StateChangeNotifier<GuardDutyStateView>,
}

impl GuardDutyService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for GuardDutyService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for GuardDutyService {
    fn service_name(&self) -> &str {
        "guardduty"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://guardduty\..*\.amazonaws\.com",
            r"https?://guardduty\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl GuardDutyService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str();

        let raw_segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s: &&str| !s.is_empty())
            .collect();
        let decoded_segments: Vec<String> = raw_segments
            .iter()
            .map(|s| {
                urlencoding::decode(s)
                    .unwrap_or(std::borrow::Cow::Borrowed(s))
                    .into_owned()
            })
            .collect();
        let segments: Vec<&str> = decoded_segments.iter().map(|s| s.as_str()).collect();

        let response = self
            .dispatch_route(
                &state,
                method,
                segments.as_slice(),
                &request,
                &query_map,
                account_id,
                &region,
            )
            .await;

        if matches!(method, "POST" | "PUT" | "PATCH" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn dispatch_route(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments) {
            // POST /detector - CreateDetector
            ("POST", ["detector"]) => {
                self.handle_create_detector(state, request, &[], query)
                    .await
            }
            // GET /detector - ListDetectors
            ("GET", ["detector"]) => self.handle_list_detectors(state, request, &[], query).await,
            // GET /detector/{detectorId} - GetDetector
            ("GET", ["detector", detector_id]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_detector(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId} - UpdateDetector
            ("POST", ["detector", detector_id]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_update_detector(state, request, labels, query)
                    .await
            }
            // DELETE /detector/{detectorId} - DeleteDetector
            ("DELETE", ["detector", detector_id]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_delete_detector(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/filter - CreateFilter
            ("POST", ["detector", detector_id, "filter"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_filter(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/filter - ListFilters
            ("GET", ["detector", detector_id, "filter"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_filters(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/filter/{filterName} - GetFilter
            ("GET", ["detector", detector_id, "filter", filter_name]) => {
                let labels: &[(&str, &str)] =
                    &[("DetectorId", detector_id), ("FilterName", filter_name)];
                self.handle_get_filter(state, request, labels, query).await
            }
            // POST /detector/{detectorId}/filter/{filterName} - UpdateFilter
            ("POST", ["detector", detector_id, "filter", filter_name]) => {
                let labels: &[(&str, &str)] =
                    &[("DetectorId", detector_id), ("FilterName", filter_name)];
                self.handle_update_filter(state, request, labels, query)
                    .await
            }
            // DELETE /detector/{detectorId}/filter/{filterName} - DeleteFilter
            ("DELETE", ["detector", detector_id, "filter", filter_name]) => {
                let labels: &[(&str, &str)] =
                    &[("DetectorId", detector_id), ("FilterName", filter_name)];
                self.handle_delete_filter(state, request, labels, query)
                    .await
            }
            // POST /admin/enable - EnableOrganizationAdminAccount
            ("POST", ["admin", "enable"]) => {
                self.handle_enable_organization_admin_account(state, request, &[], query)
                    .await
            }
            // POST /admin/disable - DisableOrganizationAdminAccount
            ("POST", ["admin", "disable"]) => {
                self.handle_disable_organization_admin_account(state, request, &[], query)
                    .await
            }
            // GET /admin - ListOrganizationAdminAccounts
            ("GET", ["admin"]) => {
                self.handle_list_organization_admin_accounts(state, request, &[], query)
                    .await
            }
            // GET /detector/{detectorId}/administrator - GetAdministratorAccount
            ("GET", ["detector", detector_id, "administrator"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_administrator_account(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/administrator - AcceptAdministratorInvitation
            ("POST", ["detector", detector_id, "administrator"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_accept_administrator_invitation(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/administrator/disassociate - DisassociateFromAdministratorAccount
            ("POST", ["detector", detector_id, "administrator", "disassociate"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_disassociate_from_administrator_account(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/master - GetMasterAccount
            ("GET", ["detector", detector_id, "master"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_master_account(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/master - AcceptInvitation
            ("POST", ["detector", detector_id, "master"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_accept_invitation(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/master/disassociate - DisassociateFromMasterAccount
            ("POST", ["detector", detector_id, "master", "disassociate"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_disassociate_from_master_account(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/findings/archive - ArchiveFindings
            ("POST", ["detector", detector_id, "findings", "archive"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_archive_findings(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/findings/unarchive - UnarchiveFindings
            ("POST", ["detector", detector_id, "findings", "unarchive"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_unarchive_findings(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/findings/create - CreateSampleFindings
            ("POST", ["detector", detector_id, "findings", "create"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_sample_findings(
                    state, request, labels, query, account_id, region,
                )
                .await
            }
            // POST /detector/{detectorId}/findings/get - GetFindings
            ("POST", ["detector", detector_id, "findings", "get"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_findings(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/findings/statistics - GetFindingsStatistics
            ("POST", ["detector", detector_id, "findings", "statistics"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_findings_statistics(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/findings/feedback - UpdateFindingsFeedback
            ("POST", ["detector", detector_id, "findings", "feedback"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_update_findings_feedback(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/findings - ListFindings
            ("POST", ["detector", detector_id, "findings"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_findings(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/ipset - CreateIPSet
            ("POST", ["detector", detector_id, "ipset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_i_p_set(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/ipset - ListIPSets
            ("GET", ["detector", detector_id, "ipset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_i_p_sets(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/ipset/{ipSetId} - GetIPSet
            ("GET", ["detector", detector_id, "ipset", ip_set_id]) => {
                let labels: &[(&str, &str)] =
                    &[("DetectorId", detector_id), ("IpSetId", ip_set_id)];
                self.handle_get_i_p_set(state, request, labels, query).await
            }
            // POST /detector/{detectorId}/ipset/{ipSetId} - UpdateIPSet
            ("POST", ["detector", detector_id, "ipset", ip_set_id]) => {
                let labels: &[(&str, &str)] =
                    &[("DetectorId", detector_id), ("IpSetId", ip_set_id)];
                self.handle_update_i_p_set(state, request, labels, query)
                    .await
            }
            // DELETE /detector/{detectorId}/ipset/{ipSetId} - DeleteIPSet
            ("DELETE", ["detector", detector_id, "ipset", ip_set_id]) => {
                let labels: &[(&str, &str)] =
                    &[("DetectorId", detector_id), ("IpSetId", ip_set_id)];
                self.handle_delete_i_p_set(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member - CreateMembers
            ("POST", ["detector", detector_id, "member"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_members(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/member - ListMembers
            ("GET", ["detector", detector_id, "member"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_members(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member/delete - DeleteMembers
            ("POST", ["detector", detector_id, "member", "delete"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_delete_members(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member/disassociate - DisassociateMembers
            ("POST", ["detector", detector_id, "member", "disassociate"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_disassociate_members(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member/invite - InviteMembers
            ("POST", ["detector", detector_id, "member", "invite"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_invite_members(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member/start - StartMonitoringMembers
            ("POST", ["detector", detector_id, "member", "start"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_start_monitoring_members(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member/stop - StopMonitoringMembers
            ("POST", ["detector", detector_id, "member", "stop"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_stop_monitoring_members(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member/get - GetMembers
            ("POST", ["detector", detector_id, "member", "get"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_members(state, request, labels, query).await
            }
            // POST /detector/{detectorId}/member/detector/get - GetMemberDetectors
            ("POST", ["detector", detector_id, "member", "detector", "get"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_member_detectors(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/member/detector/update - UpdateMemberDetectors
            ("POST", ["detector", detector_id, "member", "detector", "update"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_update_member_detectors(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/publishingDestination - CreatePublishingDestination
            ("POST", ["detector", detector_id, "publishingDestination"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_publishing_destination(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/publishingDestination - ListPublishingDestinations
            ("GET", ["detector", detector_id, "publishingDestination"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_publishing_destinations(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/publishingDestination/{destinationId} - DescribePublishingDestination
            (
                "GET",
                [
                    "detector",
                    detector_id,
                    "publishingDestination",
                    destination_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("DestinationId", destination_id),
                ];
                self.handle_describe_publishing_destination(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/publishingDestination/{destinationId} - UpdatePublishingDestination
            (
                "POST",
                [
                    "detector",
                    detector_id,
                    "publishingDestination",
                    destination_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("DestinationId", destination_id),
                ];
                self.handle_update_publishing_destination(state, request, labels, query)
                    .await
            }
            // DELETE /detector/{detectorId}/publishingDestination/{destinationId} - DeletePublishingDestination
            (
                "DELETE",
                [
                    "detector",
                    detector_id,
                    "publishingDestination",
                    destination_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("DestinationId", destination_id),
                ];
                self.handle_delete_publishing_destination(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/threatintelset - CreateThreatIntelSet
            ("POST", ["detector", detector_id, "threatintelset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_threat_intel_set(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/threatintelset - ListThreatIntelSets
            ("GET", ["detector", detector_id, "threatintelset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_threat_intel_sets(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/threatintelset/{threatIntelSetId} - GetThreatIntelSet
            (
                "GET",
                [
                    "detector",
                    detector_id,
                    "threatintelset",
                    threat_intel_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("ThreatIntelSetId", threat_intel_set_id),
                ];
                self.handle_get_threat_intel_set(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/threatintelset/{threatIntelSetId} - UpdateThreatIntelSet
            (
                "POST",
                [
                    "detector",
                    detector_id,
                    "threatintelset",
                    threat_intel_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("ThreatIntelSetId", threat_intel_set_id),
                ];
                self.handle_update_threat_intel_set(state, request, labels, query)
                    .await
            }
            // DELETE /detector/{detectorId}/threatintelset/{threatIntelSetId} - DeleteThreatIntelSet
            (
                "DELETE",
                [
                    "detector",
                    detector_id,
                    "threatintelset",
                    threat_intel_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("ThreatIntelSetId", threat_intel_set_id),
                ];
                self.handle_delete_threat_intel_set(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/threatentityset - CreateThreatEntitySet
            ("POST", ["detector", detector_id, "threatentityset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_threat_entity_set(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/threatentityset - ListThreatEntitySets
            ("GET", ["detector", detector_id, "threatentityset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_threat_entity_sets(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/threatentityset/{threatEntitySetId} - GetThreatEntitySet
            (
                "GET",
                [
                    "detector",
                    detector_id,
                    "threatentityset",
                    threat_entity_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("ThreatEntitySetId", threat_entity_set_id),
                ];
                self.handle_get_threat_entity_set(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/threatentityset/{threatEntitySetId} - UpdateThreatEntitySet
            (
                "POST",
                [
                    "detector",
                    detector_id,
                    "threatentityset",
                    threat_entity_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("ThreatEntitySetId", threat_entity_set_id),
                ];
                self.handle_update_threat_entity_set(state, request, labels, query)
                    .await
            }
            // DELETE /detector/{detectorId}/threatentityset/{threatEntitySetId} - DeleteThreatEntitySet
            (
                "DELETE",
                [
                    "detector",
                    detector_id,
                    "threatentityset",
                    threat_entity_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("ThreatEntitySetId", threat_entity_set_id),
                ];
                self.handle_delete_threat_entity_set(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/trustedentityset - CreateTrustedEntitySet
            ("POST", ["detector", detector_id, "trustedentityset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_create_trusted_entity_set(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/trustedentityset - ListTrustedEntitySets
            ("GET", ["detector", detector_id, "trustedentityset"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_trusted_entity_sets(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/trustedentityset/{trustedEntitySetId} - GetTrustedEntitySet
            (
                "GET",
                [
                    "detector",
                    detector_id,
                    "trustedentityset",
                    trusted_entity_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("TrustedEntitySetId", trusted_entity_set_id),
                ];
                self.handle_get_trusted_entity_set(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/trustedentityset/{trustedEntitySetId} - UpdateTrustedEntitySet
            (
                "POST",
                [
                    "detector",
                    detector_id,
                    "trustedentityset",
                    trusted_entity_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("TrustedEntitySetId", trusted_entity_set_id),
                ];
                self.handle_update_trusted_entity_set(state, request, labels, query)
                    .await
            }
            // DELETE /detector/{detectorId}/trustedentityset/{trustedEntitySetId} - DeleteTrustedEntitySet
            (
                "DELETE",
                [
                    "detector",
                    detector_id,
                    "trustedentityset",
                    trusted_entity_set_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("DetectorId", detector_id),
                    ("TrustedEntitySetId", trusted_entity_set_id),
                ];
                self.handle_delete_trusted_entity_set(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/admin - UpdateOrganizationConfiguration
            ("POST", ["detector", detector_id, "admin"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_update_organization_configuration(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/admin - DescribeOrganizationConfiguration
            ("GET", ["detector", detector_id, "admin"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_describe_organization_configuration(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/coverage/statistics - GetCoverageStatistics
            ("POST", ["detector", detector_id, "coverage", "statistics"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_coverage_statistics(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/coverage - ListCoverage
            ("POST", ["detector", detector_id, "coverage"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_list_coverage(state, request, labels, query)
                    .await
            }
            // GET /detector/{detectorId}/malware-scan-settings - GetMalwareScanSettings
            ("GET", ["detector", detector_id, "malware-scan-settings"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_malware_scan_settings(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/malware-scan-settings - UpdateMalwareScanSettings
            ("POST", ["detector", detector_id, "malware-scan-settings"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_update_malware_scan_settings(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/malware-scans - DescribeMalwareScans
            ("POST", ["detector", detector_id, "malware-scans"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_describe_malware_scans(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/freeTrial/daysRemaining - GetRemainingFreeTrialDays
            ("POST", ["detector", detector_id, "freeTrial", "daysRemaining"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_remaining_free_trial_days(state, request, labels, query)
                    .await
            }
            // POST /detector/{detectorId}/usage/statistics - GetUsageStatistics
            ("POST", ["detector", detector_id, "usage", "statistics"]) => {
                let labels: &[(&str, &str)] = &[("DetectorId", detector_id)];
                self.handle_get_usage_statistics(state, request, labels, query)
                    .await
            }
            // POST /invitation/decline - DeclineInvitations
            ("POST", ["invitation", "decline"]) => {
                self.handle_decline_invitations(state, request, &[], query)
                    .await
            }
            // POST /invitation/delete - DeleteInvitations
            ("POST", ["invitation", "delete"]) => {
                self.handle_delete_invitations(state, request, &[], query)
                    .await
            }
            // GET /invitation/count - GetInvitationsCount
            ("GET", ["invitation", "count"]) => {
                self.handle_get_invitations_count(state, request, &[], query)
                    .await
            }
            // GET /invitation - ListInvitations
            ("GET", ["invitation"]) => {
                self.handle_list_invitations(state, request, &[], query)
                    .await
            }
            // POST /malware-protection-plan - CreateMalwareProtectionPlan
            ("POST", ["malware-protection-plan"]) => {
                self.handle_create_malware_protection_plan(
                    state,
                    request,
                    &[],
                    query,
                    account_id,
                    region,
                )
                .await
            }
            // GET /malware-protection-plan - ListMalwareProtectionPlans
            ("GET", ["malware-protection-plan"]) => {
                self.handle_list_malware_protection_plans(state, request, &[], query)
                    .await
            }
            // GET /malware-protection-plan/{malwareProtectionPlanId} - GetMalwareProtectionPlan
            ("GET", ["malware-protection-plan", plan_id]) => {
                let labels: &[(&str, &str)] = &[("MalwareProtectionPlanId", plan_id)];
                self.handle_get_malware_protection_plan(state, request, labels, query)
                    .await
            }
            // PATCH /malware-protection-plan/{malwareProtectionPlanId} - UpdateMalwareProtectionPlan
            ("PATCH", ["malware-protection-plan", plan_id]) => {
                let labels: &[(&str, &str)] = &[("MalwareProtectionPlanId", plan_id)];
                self.handle_update_malware_protection_plan(state, request, labels, query)
                    .await
            }
            // DELETE /malware-protection-plan/{malwareProtectionPlanId} - DeleteMalwareProtectionPlan
            ("DELETE", ["malware-protection-plan", plan_id]) => {
                let labels: &[(&str, &str)] = &[("MalwareProtectionPlanId", plan_id)];
                self.handle_delete_malware_protection_plan(state, request, labels, query)
                    .await
            }
            // GET /malware-scan/{scanId} - GetMalwareScan
            ("GET", ["malware-scan", scan_id]) => {
                let labels: &[(&str, &str)] = &[("ScanId", scan_id)];
                self.handle_get_malware_scan(state, request, labels, query)
                    .await
            }
            // POST /malware-scan - ListMalwareScans
            ("POST", ["malware-scan"]) => {
                self.handle_list_malware_scans(state, request, &[], query)
                    .await
            }
            // POST /malware-scan/start - StartMalwareScan
            ("POST", ["malware-scan", "start"]) => {
                self.handle_start_malware_scan(state, request, &[], query)
                    .await
            }
            // POST /object-malware-scan/send - SendObjectMalwareScan
            ("POST", ["object-malware-scan", "send"]) => {
                self.handle_send_object_malware_scan(state, request, &[], query)
                    .await
            }
            // GET /organization/statistics - GetOrganizationStatistics
            ("GET", ["organization", "statistics"]) => {
                self.handle_get_organization_statistics(state).await
            }
            // GET /tags/{resourceArn} - ListTagsForResource
            ("GET", ["tags", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn)];
                self.handle_list_tags_for_resource(state, request, labels, query)
                    .await
            }
            // POST /tags/{resourceArn} - TagResource
            ("POST", ["tags", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn)];
                self.handle_tag_resource(state, request, labels, query)
                    .await
            }
            // DELETE /tags/{resourceArn} - UntagResource
            ("DELETE", ["tags", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn)];
                self.handle_untag_resource(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_detector_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let enable = input.enable;
        let frequency = input
            .finding_publishing_frequency
            .as_deref()
            .unwrap_or("SIX_HOURS");
        let tags = input.tags.unwrap_or_default();
        let data_sources = data_sources_from_wire(input.data_sources.as_ref());
        let features = features_from_wire(input.features.as_deref());

        let mut state = state.write().await;
        match state.create_detector(enable, frequency, tags, data_sources, features) {
            Ok(detector_id) => {
                wire::serialize_create_detector_response(&wire::CreateDetectorResponse {
                    detector_id: Some(detector_id),
                    unprocessed_data_sources: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_detector_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_detector(&input.detector_id) {
            Ok(detector) => {
                let data_sources =
                    detector
                        .data_sources
                        .as_ref()
                        .map(|ds| wire::DataSourceConfigurationsResult {
                            cloud_trail: Some(wire::CloudTrailConfigurationResult {
                                status: Some("ENABLED".to_string()),
                            }),
                            d_n_s_logs: Some(wire::DNSLogsConfigurationResult {
                                status: Some("ENABLED".to_string()),
                            }),
                            flow_logs: Some(wire::FlowLogsConfigurationResult {
                                status: Some("ENABLED".to_string()),
                            }),
                            s3_logs: Some(wire::S3LogsConfigurationResult {
                                status: Some(if ds.s3_logs_enabled.unwrap_or(false) {
                                    "ENABLED".to_string()
                                } else {
                                    "DISABLED".to_string()
                                }),
                            }),
                            kubernetes: ds.kubernetes_audit_logs_enabled.map(|enabled| {
                                wire::KubernetesConfigurationResult {
                                    audit_logs: Some(
                                        wire::KubernetesAuditLogsConfigurationResult {
                                            status: Some(if enabled {
                                                "ENABLED".to_string()
                                            } else {
                                                "DISABLED".to_string()
                                            }),
                                        },
                                    ),
                                }
                            }),
                            malware_protection: None,
                        });

                let features = detector.features.as_ref().map(|feats| {
                    feats
                        .iter()
                        .map(|f| wire::DetectorFeatureConfigurationResult {
                            name: Some(f.name.clone()),
                            status: Some(f.status.clone()),
                            additional_configuration: if f.additional_configuration.is_empty() {
                                None
                            } else {
                                Some(
                                    f.additional_configuration
                                        .iter()
                                        .map(|ac| wire::DetectorAdditionalConfigurationResult {
                                            name: Some(ac.name.clone()),
                                            status: Some(ac.status.clone()),
                                            updated_at: None,
                                        })
                                        .collect(),
                                )
                            },
                            updated_at: None,
                        })
                        .collect()
                });

                wire::serialize_get_detector_response(&wire::GetDetectorResponse {
                    status: Some(detector.status.clone()),
                    finding_publishing_frequency: Some(
                        detector.finding_publishing_frequency.clone(),
                    ),
                    created_at: Some(detector.created_at.clone()),
                    service_role: Some(String::new()),
                    data_sources,
                    features,
                    tags: if detector.tags.is_empty() {
                        None
                    } else {
                        Some(detector.tags.clone())
                    },
                    updated_at: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_detector_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let enable = input.enable;
        let frequency = input.finding_publishing_frequency.as_deref();
        let data_sources = data_sources_from_wire(input.data_sources.as_ref());
        let features = features_from_wire(input.features.as_deref());

        let mut state = state.write().await;
        match state.update_detector(
            &input.detector_id,
            enable,
            frequency,
            data_sources,
            features,
        ) {
            Ok(()) => wire::serialize_update_detector_response(&wire::UpdateDetectorResponse {}),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_detector_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_detector(&input.detector_id) {
            Ok(()) => wire::serialize_delete_detector_response(&wire::DeleteDetectorResponse {}),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_detectors(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_detectors_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let ids: Vec<&str> = state.list_detector_ids();
        wire::serialize_list_detectors_response(&wire::ListDetectorsResponse {
            detector_ids: Some(ids.into_iter().map(|s| s.to_string()).collect()),
            next_token: None,
        })
    }

    async fn handle_create_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_filter_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let description = input.description.as_deref().unwrap_or("");
        let action = input.action.as_deref().unwrap_or("NOOP");
        let rank = input.rank.unwrap_or(1);
        let finding_criteria = finding_criteria_from_wire(Some(&input.finding_criteria));
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_filter(
            &input.detector_id,
            &input.name,
            description,
            action,
            rank,
            finding_criteria,
            tags,
        ) {
            Ok(filter_name) => {
                wire::serialize_create_filter_response(&wire::CreateFilterResponse {
                    name: Some(filter_name.to_string()),
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_filter_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_filter(&input.detector_id, &input.filter_name) {
            Ok(filter) => wire::serialize_get_filter_response(&wire::GetFilterResponse {
                name: Some(filter.name.clone()),
                description: Some(filter.description.clone()),
                action: Some(filter.action.clone()),
                rank: Some(filter.rank),
                finding_criteria: Some(convert_finding_criteria_to_wire(&filter.finding_criteria)),
                tags: Some(filter.tags.clone()),
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_filter_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();
        let action = input.action.as_deref();
        let rank = input.rank;
        let finding_criteria = input
            .finding_criteria
            .as_ref()
            .map(|fc| finding_criteria_from_wire(Some(fc)));

        let mut state = state.write().await;
        match state.update_filter(
            &input.detector_id,
            &input.filter_name,
            description,
            action,
            rank,
            finding_criteria,
        ) {
            Ok(name) => wire::serialize_update_filter_response(&wire::UpdateFilterResponse {
                name: Some(name.to_string()),
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_filter_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_filter(&input.detector_id, &input.filter_name) {
            Ok(()) => wire::serialize_delete_filter_response(&wire::DeleteFilterResponse {}),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_filters(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_filters_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_filter_names(&input.detector_id) {
            Ok(names) => wire::serialize_list_filters_response(&wire::ListFiltersResponse {
                filter_names: Some(names),
                next_token: None,
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_enable_organization_admin_account(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_organization_admin_account_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.enable_organization_admin_account(&input.admin_account_id) {
            Ok(()) => wire::serialize_enable_organization_admin_account_response(
                &wire::EnableOrganizationAdminAccountResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_organization_admin_accounts(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_organization_admin_accounts_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let accounts = state.list_organization_admin_accounts();
        let admin_accounts: Vec<wire::AdminAccount> = accounts
            .iter()
            .map(|a| wire::AdminAccount {
                admin_account_id: Some(a.admin_account_id.clone()),
                admin_status: Some(a.admin_status.clone()),
            })
            .collect();

        wire::serialize_list_organization_admin_accounts_response(
            &wire::ListOrganizationAdminAccountsResponse {
                admin_accounts: Some(admin_accounts),
                next_token: None,
            },
        )
    }

    async fn handle_get_administrator_account(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_administrator_account_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        // Verify detector exists
        if let Err(e) = state.get_detector(&input.detector_id) {
            return guardduty_error_response(&e);
        }

        match state.get_administrator_account() {
            Some(admin) => wire::serialize_get_administrator_account_response(
                &wire::GetAdministratorAccountResponse {
                    administrator: Some(wire::Administrator {
                        account_id: Some(admin.admin_account_id.clone()),
                        relationship_status: Some("Enabled".to_string()),
                        invitation_id: None,
                        invited_at: None,
                    }),
                },
            ),
            None => wire::serialize_get_administrator_account_response(
                &wire::GetAdministratorAccountResponse {
                    administrator: Some(wire::Administrator::default()),
                },
            ),
        }
    }

    async fn handle_accept_administrator_invitation(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_accept_administrator_invitation_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.accept_administrator_invitation(
            &input.detector_id,
            &input.administrator_id,
            &input.invitation_id,
        ) {
            Ok(()) => wire::serialize_accept_administrator_invitation_response(
                &wire::AcceptAdministratorInvitationResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_accept_invitation(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_accept_invitation_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.accept_invitation(&input.detector_id, &input.master_id, &input.invitation_id) {
            Ok(()) => {
                wire::serialize_accept_invitation_response(&wire::AcceptInvitationResponse {})
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_archive_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_archive_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.archive_findings(&input.detector_id, &input.finding_ids) {
            Ok(()) => wire::serialize_archive_findings_response(&wire::ArchiveFindingsResponse {}),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_unarchive_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_unarchive_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.unarchive_findings(&input.detector_id, &input.finding_ids) {
            Ok(()) => {
                wire::serialize_unarchive_findings_response(&wire::UnarchiveFindingsResponse {})
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_create_sample_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_sample_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let finding_types = input.finding_types.unwrap_or_default();
        let mut state = state.write().await;
        match state.create_sample_findings(&input.detector_id, account_id, region, &finding_types) {
            Ok(()) => wire::serialize_create_sample_findings_response(
                &wire::CreateSampleFindingsResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_findings(&input.detector_id, &input.finding_ids) {
            Ok(findings) => {
                let wire_findings: Vec<wire::Finding> = findings
                    .iter()
                    .map(|f| wire::Finding {
                        id: Some(f.id.clone()),
                        arn: Some(f.arn.clone()),
                        account_id: Some(f.account_id.clone()),
                        region: Some(f.region.clone()),
                        r#type: Some(f.r#type.clone()),
                        severity: Some(f.severity),
                        title: Some(f.title.clone()),
                        description: Some(f.description.clone()),
                        created_at: Some(f.created_at.clone()),
                        updated_at: Some(f.updated_at.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_findings_response(&wire::GetFindingsResponse {
                    findings: Some(wire_findings),
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_findings_statistics(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_findings_statistics_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_findings_statistics(&input.detector_id) {
            Ok(count_by_severity) => {
                let grouped_by_severity: Vec<wire::SeverityStatistics> = count_by_severity
                    .iter()
                    .map(|(severity_str, &count)| wire::SeverityStatistics {
                        severity: severity_str.parse::<f64>().ok(),
                        total_findings: Some(count),
                        last_generated_at: None,
                    })
                    .collect();
                wire::serialize_get_findings_statistics_response(
                    &wire::GetFindingsStatisticsResponse {
                        finding_statistics: Some(wire::FindingStatistics {
                            count_by_severity: Some(count_by_severity),
                            grouped_by_severity: Some(grouped_by_severity),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                )
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_findings_feedback(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_findings_feedback_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Verify detector exists; feedback metadata is not persisted in the mock.
        let state = state.read().await;
        match state.get_detector(&input.detector_id) {
            Ok(_) => wire::serialize_update_findings_feedback_response(
                &wire::UpdateFindingsFeedbackResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Check whether the caller wants archived findings included.
        // The `findingCriteria` filter supports `service.archived` eq "true"/"false".
        // AWS default: exclude archived findings unless `service.archived` eq ["true"] is
        // explicitly set in the criteria.
        let include_archived = input
            .finding_criteria
            .as_ref()
            .and_then(|fc| fc.criterion.as_ref())
            .and_then(|c| c.get("service.archived"))
            .map(|cond| {
                let arr = cond.eq.as_ref().or(cond.equals.as_ref());
                arr.map(|a| a.iter().any(|v| v == "true")).unwrap_or(false)
            })
            .unwrap_or(false);

        let state = state.read().await;
        match state.list_findings(&input.detector_id, include_archived) {
            Ok(ids) => wire::serialize_list_findings_response(&wire::ListFindingsResponse {
                finding_ids: Some(ids),
                ..Default::default()
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_create_i_p_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_i_p_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let format = if input.format.is_empty() {
            "TXT"
        } else {
            input.format.as_str()
        };
        let location = input.location.as_str();
        let activate = input.activate;
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_ip_set(
            &input.detector_id,
            &input.name,
            format,
            location,
            activate,
            tags,
        ) {
            Ok(ip_set_id) => wire::serialize_create_i_p_set_response(&wire::CreateIPSetResponse {
                ip_set_id: Some(ip_set_id),
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_i_p_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_i_p_sets_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_ip_set_ids(&input.detector_id) {
            Ok(ids) => wire::serialize_list_i_p_sets_response(&wire::ListIPSetsResponse {
                ip_set_ids: Some(ids),
                next_token: None,
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_i_p_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_i_p_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_ip_set(&input.detector_id, &input.ip_set_id) {
            Ok(ip_set) => wire::serialize_get_i_p_set_response(&wire::GetIPSetResponse {
                name: Some(ip_set.name.clone()),
                format: Some(ip_set.format.clone()),
                location: Some(ip_set.location.clone()),
                status: Some(ip_set.status.clone()),
                tags: if ip_set.tags.is_empty() {
                    None
                } else {
                    Some(ip_set.tags.clone())
                },
                expected_bucket_owner: None,
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_i_p_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_i_p_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name.as_deref();
        let location = input.location.as_deref();
        let activate = input.activate;

        let mut state = state.write().await;
        match state.update_ip_set(
            &input.detector_id,
            &input.ip_set_id,
            name,
            location,
            activate,
        ) {
            Ok(()) => wire::serialize_update_i_p_set_response(&wire::UpdateIPSetResponse {}),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_i_p_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_i_p_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_ip_set(&input.detector_id, &input.ip_set_id) {
            Ok(()) => wire::serialize_delete_i_p_set_response(&wire::DeleteIPSetResponse {}),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_create_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let accounts: Vec<(String, String)> = input
            .account_details
            .iter()
            .map(|d| (d.account_id.clone(), d.email.clone()))
            .collect();

        let mut state = state.write().await;
        match state.create_members(&input.detector_id, accounts) {
            Ok(()) => wire::serialize_create_members_response(&wire::CreateMembersResponse {
                unprocessed_accounts: None,
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_members(&input.detector_id) {
            Ok(members) => {
                let wire_members: Vec<wire::Member> = members
                    .iter()
                    .map(|m| wire::Member {
                        account_id: Some(m.account_id.clone()),
                        email: Some(m.email.clone()),
                        relationship_status: Some(m.relationship_status.clone()),
                        invited_at: m.invited_at.clone(),
                        updated_at: Some(m.updated_at.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_members_response(&wire::ListMembersResponse {
                    members: if wire_members.is_empty() {
                        None
                    } else {
                        Some(wire_members)
                    },
                    next_token: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_members(&input.detector_id, &input.account_ids) {
            Ok(()) => wire::serialize_delete_members_response(&wire::DeleteMembersResponse {
                unprocessed_accounts: None,
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_disassociate_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.disassociate_members(&input.detector_id, &input.account_ids) {
            Ok(()) => {
                wire::serialize_disassociate_members_response(&wire::DisassociateMembersResponse {
                    unprocessed_accounts: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_invite_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_invite_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.invite_members(&input.detector_id, &input.account_ids) {
            Ok(()) => wire::serialize_invite_members_response(&wire::InviteMembersResponse {
                unprocessed_accounts: None,
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_start_monitoring_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_monitoring_members_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.start_monitoring_members(&input.detector_id, &input.account_ids) {
            Ok(()) => wire::serialize_start_monitoring_members_response(
                &wire::StartMonitoringMembersResponse {
                    unprocessed_accounts: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_stop_monitoring_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_monitoring_members_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.stop_monitoring_members(&input.detector_id, &input.account_ids) {
            Ok(()) => wire::serialize_stop_monitoring_members_response(
                &wire::StopMonitoringMembersResponse {
                    unprocessed_accounts: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_members(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_members(&input.detector_id, &input.account_ids) {
            Ok(members) => {
                let wire_members: Vec<wire::Member> = members
                    .iter()
                    .map(|m| wire::Member {
                        account_id: Some(m.account_id.clone()),
                        email: Some(m.email.clone()),
                        relationship_status: Some(m.relationship_status.clone()),
                        invited_at: m.invited_at.clone(),
                        updated_at: Some(m.updated_at.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_members_response(&wire::GetMembersResponse {
                    members: if wire_members.is_empty() {
                        None
                    } else {
                        Some(wire_members)
                    },
                    unprocessed_accounts: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_member_detectors(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_member_detectors_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_member_detectors(&input.detector_id, &input.account_ids) {
            Ok(members) => {
                let configs: Vec<wire::MemberDataSourceConfiguration> = members
                    .iter()
                    .map(|m| wire::MemberDataSourceConfiguration {
                        account_id: Some(m.account_id.clone()),
                        features: if m.detector_features.is_empty() {
                            None
                        } else {
                            Some(
                                m.detector_features
                                    .iter()
                                    .map(|f| wire::MemberFeaturesConfigurationResult {
                                        name: Some(f.name.clone()),
                                        status: Some(f.status.clone()),
                                        updated_at: None,
                                        additional_configuration: if f
                                            .additional_configuration
                                            .is_empty()
                                        {
                                            None
                                        } else {
                                            Some(
                                                f.additional_configuration
                                                    .iter()
                                                    .map(|ac| {
                                                        wire::MemberAdditionalConfigurationResult {
                                                            name: Some(ac.name.clone()),
                                                            status: Some(ac.status.clone()),
                                                            updated_at: None,
                                                        }
                                                    })
                                                    .collect(),
                                            )
                                        },
                                    })
                                    .collect(),
                            )
                        },
                        data_sources: None,
                    })
                    .collect();
                wire::serialize_get_member_detectors_response(&wire::GetMemberDetectorsResponse {
                    member_data_source_configurations: if configs.is_empty() {
                        None
                    } else {
                        Some(configs)
                    },
                    unprocessed_accounts: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_member_detectors(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_member_detectors_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let features: Vec<MemberDetectorFeature> = input
            .features
            .as_deref()
            .map(|arr| {
                arr.iter()
                    .filter_map(|f| {
                        let name = f.name.clone()?;
                        let status = f.status.clone().unwrap_or_else(|| "DISABLED".to_string());
                        let additional_configuration: Vec<MemberDetectorAdditionalConfig> = f
                            .additional_configuration
                            .as_deref()
                            .map(|ac_arr| {
                                ac_arr
                                    .iter()
                                    .filter_map(|ac| {
                                        let ac_name = ac.name.clone()?;
                                        let ac_status = ac
                                            .status
                                            .clone()
                                            .unwrap_or_else(|| "DISABLED".to_string());
                                        Some(MemberDetectorAdditionalConfig {
                                            name: ac_name,
                                            status: ac_status,
                                        })
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        Some(MemberDetectorFeature {
                            name,
                            status,
                            additional_configuration,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        match state.update_member_detectors(&input.detector_id, &input.account_ids, features) {
            Ok(()) => wire::serialize_update_member_detectors_response(
                &wire::UpdateMemberDetectorsResponse {
                    unprocessed_accounts: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_create_publishing_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_publishing_destination_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let destination_type = if input.destination_type.is_empty() {
            "S3"
        } else {
            input.destination_type.as_str()
        };
        let destination_arn = input.destination_properties.destination_arn.as_deref();
        let kms_key_arn = input.destination_properties.kms_key_arn.as_deref();
        let tags = input.tags.unwrap_or_default();
        let mut state = state.write().await;
        match state.create_publishing_destination(
            &input.detector_id,
            destination_type,
            destination_arn,
            kms_key_arn,
            tags,
        ) {
            Ok(destination_id) => wire::serialize_create_publishing_destination_response(
                &wire::CreatePublishingDestinationResponse {
                    destination_id: Some(destination_id),
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_publishing_destinations(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_publishing_destinations_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.list_publishing_destinations(&input.detector_id) {
            Ok(dests) => {
                let destinations: Vec<wire::Destination> = dests
                    .into_iter()
                    .map(|d| wire::Destination {
                        destination_id: Some(d.destination_id.clone()),
                        destination_type: Some(d.destination_type.clone()),
                        status: Some(d.status.clone()),
                    })
                    .collect();
                wire::serialize_list_publishing_destinations_response(
                    &wire::ListPublishingDestinationsResponse {
                        destinations: if destinations.is_empty() {
                            None
                        } else {
                            Some(destinations)
                        },
                        next_token: None,
                    },
                )
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_describe_publishing_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_publishing_destination_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_publishing_destination(&input.detector_id, &input.destination_id) {
            Ok(d) => wire::serialize_describe_publishing_destination_response(
                &wire::DescribePublishingDestinationResponse {
                    destination_id: Some(d.destination_id.clone()),
                    destination_type: Some(d.destination_type.clone()),
                    status: Some(d.status.clone()),
                    destination_properties: Some(wire::DestinationProperties {
                        destination_arn: d.destination_arn.clone(),
                        kms_key_arn: d.kms_key_arn.clone(),
                    }),
                    tags: if d.tags.is_empty() {
                        None
                    } else {
                        Some(d.tags.clone())
                    },
                    publishing_failure_start_timestamp: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_publishing_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_publishing_destination_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let destination_arn = input
            .destination_properties
            .as_ref()
            .and_then(|p| p.destination_arn.as_deref());
        let kms_key_arn = input
            .destination_properties
            .as_ref()
            .and_then(|p| p.kms_key_arn.as_deref());
        let mut state = state.write().await;
        match state.update_publishing_destination(
            &input.detector_id,
            &input.destination_id,
            destination_arn,
            kms_key_arn,
        ) {
            Ok(()) => wire::serialize_update_publishing_destination_response(
                &wire::UpdatePublishingDestinationResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_publishing_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_publishing_destination_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_publishing_destination(&input.detector_id, &input.destination_id) {
            Ok(()) => wire::serialize_delete_publishing_destination_response(
                &wire::DeletePublishingDestinationResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_create_threat_intel_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_threat_intel_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let format = if input.format.is_empty() {
            "TXT"
        } else {
            input.format.as_str()
        };
        let location = input.location.as_str();
        let activate = input.activate;
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_threat_intel_set(
            &input.detector_id,
            &input.name,
            format,
            location,
            activate,
            tags,
        ) {
            Ok(set_id) => wire::serialize_create_threat_intel_set_response(
                &wire::CreateThreatIntelSetResponse {
                    threat_intel_set_id: Some(set_id),
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_threat_intel_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_threat_intel_sets_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_threat_intel_set_ids(&input.detector_id) {
            Ok(ids) => wire::serialize_list_threat_intel_sets_response(
                &wire::ListThreatIntelSetsResponse {
                    threat_intel_set_ids: Some(ids),
                    next_token: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_threat_intel_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_threat_intel_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_threat_intel_set(&input.detector_id, &input.threat_intel_set_id) {
            Ok(set) => {
                wire::serialize_get_threat_intel_set_response(&wire::GetThreatIntelSetResponse {
                    name: Some(set.name.clone()),
                    format: Some(set.format.clone()),
                    location: Some(set.location.clone()),
                    status: Some(set.status.clone()),
                    tags: if set.tags.is_empty() {
                        None
                    } else {
                        Some(set.tags.clone())
                    },
                    expected_bucket_owner: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_threat_intel_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_threat_intel_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name.as_deref();
        let location = input.location.as_deref();
        let activate = input.activate;

        let mut state = state.write().await;
        match state.update_threat_intel_set(
            &input.detector_id,
            &input.threat_intel_set_id,
            name,
            location,
            activate,
        ) {
            Ok(()) => wire::serialize_update_threat_intel_set_response(
                &wire::UpdateThreatIntelSetResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_threat_intel_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_threat_intel_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_threat_intel_set(&input.detector_id, &input.threat_intel_set_id) {
            Ok(()) => wire::serialize_delete_threat_intel_set_response(
                &wire::DeleteThreatIntelSetResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_create_threat_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_threat_entity_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let format = if input.format.is_empty() {
            "TXT"
        } else {
            input.format.as_str()
        };
        let location = input.location.as_str();
        let activate = input.activate;
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_threat_entity_set(
            &input.detector_id,
            &input.name,
            format,
            location,
            activate,
            tags,
        ) {
            Ok(set_id) => wire::serialize_create_threat_entity_set_response(
                &wire::CreateThreatEntitySetResponse {
                    threat_entity_set_id: Some(set_id),
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_threat_entity_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_threat_entity_sets_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_threat_entity_set_ids(&input.detector_id) {
            Ok(ids) => wire::serialize_list_threat_entity_sets_response(
                &wire::ListThreatEntitySetsResponse {
                    threat_entity_set_ids: Some(ids),
                    next_token: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_threat_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_threat_entity_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_threat_entity_set(&input.detector_id, &input.threat_entity_set_id) {
            Ok(set) => {
                wire::serialize_get_threat_entity_set_response(&wire::GetThreatEntitySetResponse {
                    name: Some(set.name.clone()),
                    format: Some(set.format.clone()),
                    location: Some(set.location.clone()),
                    status: Some(set.status.clone()),
                    tags: if set.tags.is_empty() {
                        None
                    } else {
                        Some(set.tags.clone())
                    },
                    created_at: None,
                    error_details: None,
                    expected_bucket_owner: None,
                    updated_at: None,
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_threat_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_threat_entity_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name.as_deref();
        let location = input.location.as_deref();
        let activate = input.activate;

        let mut state = state.write().await;
        match state.update_threat_entity_set(
            &input.detector_id,
            &input.threat_entity_set_id,
            name,
            location,
            activate,
        ) {
            Ok(()) => wire::serialize_update_threat_entity_set_response(
                &wire::UpdateThreatEntitySetResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_threat_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_threat_entity_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_threat_entity_set(&input.detector_id, &input.threat_entity_set_id) {
            Ok(()) => wire::serialize_delete_threat_entity_set_response(
                &wire::DeleteThreatEntitySetResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_create_trusted_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_trusted_entity_set_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let format = if input.format.is_empty() {
            "TXT"
        } else {
            input.format.as_str()
        };
        let location = input.location.as_str();
        let activate = input.activate;
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_trusted_entity_set(
            &input.detector_id,
            &input.name,
            format,
            location,
            activate,
            tags,
        ) {
            Ok(set_id) => wire::serialize_create_trusted_entity_set_response(
                &wire::CreateTrustedEntitySetResponse {
                    trusted_entity_set_id: Some(set_id),
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_trusted_entity_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_trusted_entity_sets_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_trusted_entity_set_ids(&input.detector_id) {
            Ok(ids) => wire::serialize_list_trusted_entity_sets_response(
                &wire::ListTrustedEntitySetsResponse {
                    trusted_entity_set_ids: Some(ids),
                    next_token: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_trusted_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_trusted_entity_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_trusted_entity_set(&input.detector_id, &input.trusted_entity_set_id) {
            Ok(set) => wire::serialize_get_trusted_entity_set_response(
                &wire::GetTrustedEntitySetResponse {
                    name: Some(set.name.clone()),
                    format: Some(set.format.clone()),
                    location: Some(set.location.clone()),
                    status: Some(set.status.clone()),
                    tags: if set.tags.is_empty() {
                        None
                    } else {
                        Some(set.tags.clone())
                    },
                    created_at: None,
                    error_details: None,
                    expected_bucket_owner: None,
                    updated_at: None,
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_trusted_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_trusted_entity_set_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let name = input.name.as_deref();
        let location = input.location.as_deref();
        let activate = input.activate;

        let mut state = state.write().await;
        match state.update_trusted_entity_set(
            &input.detector_id,
            &input.trusted_entity_set_id,
            name,
            location,
            activate,
        ) {
            Ok(()) => wire::serialize_update_trusted_entity_set_response(
                &wire::UpdateTrustedEntitySetResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_trusted_entity_set(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_trusted_entity_set_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_trusted_entity_set(&input.detector_id, &input.trusted_entity_set_id) {
            Ok(()) => wire::serialize_delete_trusted_entity_set_response(
                &wire::DeleteTrustedEntitySetResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_organization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
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
        let auto_enable = input.auto_enable;
        let auto_enable_members = input.auto_enable_organization_members.as_deref();
        let mut state = state.write().await;
        match state.update_org_config(&input.detector_id, auto_enable, auto_enable_members) {
            Ok(()) => wire::serialize_update_organization_configuration_response(
                &wire::UpdateOrganizationConfigurationResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_describe_organization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_organization_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_org_config(&input.detector_id) {
            Ok(cfg) => wire::serialize_describe_organization_configuration_response(
                &wire::DescribeOrganizationConfigurationResponse {
                    auto_enable: Some(cfg.auto_enable),
                    auto_enable_organization_members: cfg.auto_enable_organization_members.clone(),
                    ..Default::default()
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_coverage_statistics(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_coverage_statistics_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        // Verify detector exists.
        if let Err(e) = state.get_detector(&input.detector_id) {
            return guardduty_error_response(&e);
        }

        // Return coverage statistics with zero counts from any existing records.
        let records = state
            .list_coverage_records(&input.detector_id)
            .unwrap_or_default();
        let mut count_by_status: HashMap<String, i64> = HashMap::new();
        for r in records {
            *count_by_status
                .entry(r.coverage_status.clone())
                .or_insert(0) += 1;
        }

        wire::serialize_get_coverage_statistics_response(&wire::GetCoverageStatisticsResponse {
            coverage_statistics: Some(wire::CoverageStatistics {
                count_by_coverage_status: if count_by_status.is_empty() {
                    None
                } else {
                    Some(count_by_status)
                },
                count_by_resource_type: None,
            }),
        })
    }

    async fn handle_list_coverage(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_coverage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        // Verify detector exists.
        if let Err(e) = state.get_detector(&input.detector_id) {
            return guardduty_error_response(&e);
        }

        let records = state
            .list_coverage_records(&input.detector_id)
            .unwrap_or_default();
        let resources: Vec<wire::CoverageResource> = records
            .iter()
            .map(|r| wire::CoverageResource {
                account_id: Some(r.account_id.clone()),
                coverage_status: Some(r.coverage_status.clone()),
                detector_id: Some(r.detector_id.clone()),
                issue: None,
                resource_details: None,
                resource_id: Some(r.resource_id.clone()),
                updated_at: None,
            })
            .collect();

        wire::serialize_list_coverage_response(&wire::ListCoverageResponse {
            next_token: None,
            resources: if resources.is_empty() {
                None
            } else {
                Some(resources)
            },
        })
    }

    async fn handle_get_malware_scan_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_malware_scan_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_malware_scan_settings(&input.detector_id) {
            Ok(settings) => wire::serialize_get_malware_scan_settings_response(
                &wire::GetMalwareScanSettingsResponse {
                    ebs_snapshot_preservation: settings.ebs_snapshot_preservation.clone(),
                    ..Default::default()
                },
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_malware_scan_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_malware_scan_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let ebs_snapshot_preservation = input.ebs_snapshot_preservation.as_deref();
        let mut state = state.write().await;
        match state.update_malware_scan_settings(&input.detector_id, ebs_snapshot_preservation) {
            Ok(()) => wire::serialize_update_malware_scan_settings_response(
                &wire::UpdateMalwareScanSettingsResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_describe_malware_scans(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_malware_scans_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_malware_scans(&input.detector_id) {
            Ok(scans) => {
                let wire_scans: Vec<wire::Scan> = scans
                    .iter()
                    .map(|s| wire::Scan {
                        scan_id: Some(s.scan_id.clone()),
                        detector_id: Some(s.detector_id.clone()),
                        scan_status: Some(s.scan_status.clone()),
                        scan_start_time: Some(s.scan_started_at),
                        scan_end_time: s.scan_completed_at,
                        scan_type: Some(s.scan_type.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_malware_scans_response(
                    &wire::DescribeMalwareScansResponse {
                        scans: if wire_scans.is_empty() {
                            None
                        } else {
                            Some(wire_scans)
                        },
                        next_token: None,
                    },
                )
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    // STUB[no-telemetry]: Free trial days remaining is driven by real AWS billing state; mock always returns empty.
    async fn handle_get_remaining_free_trial_days(
        &self,
        _state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_get_remaining_free_trial_days_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_get_remaining_free_trial_days_response(
            &wire::GetRemainingFreeTrialDaysResponse::default(),
        )
    }

    // STUB[no-telemetry]: Usage statistics are driven by real AWS billing data; mock always returns empty.
    async fn handle_get_usage_statistics(
        &self,
        _state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_usage_statistics_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_get_usage_statistics_response(&wire::GetUsageStatisticsResponse::default())
    }

    async fn handle_decline_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_decline_invitations_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.decline_invitations(&input.account_ids);
        wire::serialize_decline_invitations_response(&wire::DeclineInvitationsResponse {
            unprocessed_accounts: None,
        })
    }

    async fn handle_delete_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_invitations_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.delete_invitations(&input.account_ids);
        wire::serialize_delete_invitations_response(&wire::DeleteInvitationsResponse {
            unprocessed_accounts: None,
        })
    }

    async fn handle_get_invitations_count(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_invitations_count_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let count = state.get_invitations_count() as i32;
        wire::serialize_get_invitations_count_response(&wire::GetInvitationsCountResponse {
            invitations_count: Some(count),
        })
    }

    async fn handle_list_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_invitations_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let invitations = state.list_invitations();
        let wire_invitations: Vec<wire::Invitation> = invitations
            .iter()
            .map(|inv| wire::Invitation {
                account_id: Some(inv.account_id.clone()),
                invitation_id: Some(inv.invitation_id.clone()),
                invited_at: Some(inv.invited_at.clone()),
                relationship_status: Some(inv.relationship_status.clone()),
            })
            .collect();
        wire::serialize_list_invitations_response(&wire::ListInvitationsResponse {
            invitations: if wire_invitations.is_empty() {
                None
            } else {
                Some(wire_invitations)
            },
            next_token: None,
        })
    }

    async fn handle_create_malware_protection_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_malware_protection_plan_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let role = input.role.as_str();
        let s3_bucket = input.protected_resource.s3_bucket.as_ref();
        let s3_bucket_name = s3_bucket.and_then(|b| b.bucket_name.as_deref());
        let s3_object_prefixes: Vec<String> = s3_bucket
            .and_then(|b| b.object_prefixes.clone())
            .unwrap_or_default();
        let tagging_status = input
            .actions
            .as_ref()
            .and_then(|a| a.tagging.as_ref())
            .and_then(|t| t.status.as_deref());
        let actions = tagging_status.map(|ts| crate::types::MalwareProtectionPlanActions {
            tagging_status: Some(ts.to_string()),
        });
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        let plan_id = state.create_malware_protection_plan(
            account_id,
            region,
            role,
            s3_bucket_name,
            s3_object_prefixes,
            actions,
            tags,
        );
        wire::serialize_create_malware_protection_plan_response(
            &wire::CreateMalwareProtectionPlanResponse {
                malware_protection_plan_id: Some(plan_id),
            },
        )
    }

    async fn handle_list_malware_protection_plans(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_malware_protection_plans_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let plans = state.list_malware_protection_plans();
        let summaries: Vec<wire::MalwareProtectionPlanSummary> = plans
            .iter()
            .map(|p| wire::MalwareProtectionPlanSummary {
                malware_protection_plan_id: Some(p.plan_id.clone()),
            })
            .collect();
        wire::serialize_list_malware_protection_plans_response(
            &wire::ListMalwareProtectionPlansResponse {
                malware_protection_plans: if summaries.is_empty() {
                    None
                } else {
                    Some(summaries)
                },
                next_token: None,
            },
        )
    }

    async fn handle_get_malware_protection_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_malware_protection_plan_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_malware_protection_plan(&input.malware_protection_plan_id) {
            Ok(plan) => {
                let protected_resource = Some(wire::CreateProtectedResource {
                    s3_bucket: plan.protected_resource.s3_bucket_name.as_ref().map(|name| {
                        wire::CreateS3BucketResource {
                            bucket_name: Some(name.clone()),
                            object_prefixes: if plan
                                .protected_resource
                                .s3_object_prefixes
                                .is_empty()
                            {
                                None
                            } else {
                                Some(plan.protected_resource.s3_object_prefixes.clone())
                            },
                        }
                    }),
                });
                let actions = plan
                    .actions
                    .as_ref()
                    .map(|a| wire::MalwareProtectionPlanActions {
                        tagging: a.tagging_status.as_ref().map(|s| {
                            wire::MalwareProtectionPlanTaggingAction {
                                status: Some(s.clone()),
                            }
                        }),
                    });
                wire::serialize_get_malware_protection_plan_response(
                    &wire::GetMalwareProtectionPlanResponse {
                        arn: Some(plan.arn.clone()),
                        role: Some(plan.role.clone()),
                        protected_resource,
                        actions,
                        status: Some(plan.status.clone()),
                        created_at: Some(plan.created_at),
                        tags: if plan.tags.is_empty() {
                            None
                        } else {
                            Some(plan.tags.clone())
                        },
                        ..Default::default()
                    },
                )
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_update_malware_protection_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_malware_protection_plan_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let role = input.role.as_deref();
        let tagging_status = input
            .actions
            .as_ref()
            .and_then(|a| a.tagging.as_ref())
            .and_then(|t| t.status.as_deref());
        let mut state = state.write().await;
        match state.update_malware_protection_plan(
            &input.malware_protection_plan_id,
            role,
            tagging_status,
        ) {
            Ok(()) => wire::serialize_update_malware_protection_plan_response(),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_delete_malware_protection_plan(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_malware_protection_plan_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_malware_protection_plan(&input.malware_protection_plan_id) {
            Ok(()) => wire::serialize_delete_malware_protection_plan_response(),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_malware_scan(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_malware_scan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_malware_scan(&input.scan_id) {
            Ok(scan) => wire::serialize_get_malware_scan_response(&wire::GetMalwareScanResponse {
                scan_id: Some(scan.scan_id.clone()),
                detector_id: Some(scan.detector_id.clone()),
                resource_arn: scan.resource_arn.clone(),
                resource_type: scan.resource_type.clone(),
                scan_status: Some(scan.scan_status.clone()),
                scan_started_at: Some(scan.scan_started_at),
                scan_completed_at: scan.scan_completed_at,
                ..Default::default()
            }),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_list_malware_scans(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_malware_scans_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        // ListMalwareScans is a global scan list — collect from all detectors
        let state = state.read().await;
        let all_scans: Vec<wire::MalwareScan> = state
            .detectors
            .values()
            .flat_map(|d| {
                d.malware_scans.values().map(|s| wire::MalwareScan {
                    scan_id: Some(s.scan_id.clone()),
                    resource_arn: s.resource_arn.clone(),
                    resource_type: s.resource_type.clone(),
                    scan_status: Some(s.scan_status.clone()),
                    scan_started_at: Some(s.scan_started_at),
                    scan_completed_at: s.scan_completed_at,
                    ..Default::default()
                })
            })
            .collect();
        wire::serialize_list_malware_scans_response(&wire::ListMalwareScansResponse {
            scans: if all_scans.is_empty() {
                None
            } else {
                Some(all_scans)
            },
            next_token: None,
        })
    }

    async fn handle_start_malware_scan(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_malware_scan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_arn = if input.resource_arn.is_empty() {
            None
        } else {
            Some(input.resource_arn.as_str())
        };
        // StartMalwareScan requires at least one detector to exist; use the first one
        let mut state = state.write().await;
        let detector_id = match state.detectors.keys().next().cloned() {
            Some(id) => id,
            None => {
                return rest_json_error(
                    400,
                    "BadRequestException",
                    "No detector exists in this account",
                );
            }
        };
        match state.start_malware_scan(&detector_id, resource_arn) {
            Ok(scan_id) => {
                wire::serialize_start_malware_scan_response(&wire::StartMalwareScanResponse {
                    scan_id: Some(scan_id),
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_send_object_malware_scan(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_send_object_malware_scan_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let mut state = state.write().await;
        let _scan_id = state.create_malware_scan_record();
        // The wire response is a unit struct (no fields) per the model.
        wire::serialize_send_object_malware_scan_response(&wire::SendObjectMalwareScanResponse {})
    }

    async fn handle_get_organization_statistics(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let enabled_count = state
            .admin_accounts
            .iter()
            .filter(|a| a.admin_status == "ENABLED")
            .count() as i32;

        wire::serialize_get_organization_statistics_response(
            &wire::GetOrganizationStatisticsResponse {
                organization_details: Some(wire::OrganizationDetails {
                    organization_statistics: Some(wire::OrganizationStatistics {
                        active_accounts_count: Some(enabled_count),
                        enabled_accounts_count: Some(enabled_count),
                        count_by_feature: None,
                        member_accounts_count: Some(0),
                        total_accounts_count: Some(enabled_count),
                    }),
                    updated_at: None,
                }),
            },
        )
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
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
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // GuardDuty's UntagResource sends `tagKeys` as repeated query params
        // (?tagKeys=key1&tagKeys=key2). The auto-generated deserializer reads
        // the (single) value from `query` and splits on commas, which produces
        // the wrong result for multiple repeated keys. Re-parse from the raw
        // URI query so multi-key requests are not silently truncated.
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let multi_keys = parse_tag_keys_from_uri(&request.uri);
        let tag_keys = if multi_keys.is_empty() {
            input.tag_keys
        } else {
            multi_keys
        };
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_disable_organization_admin_account(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_organization_admin_account_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.disable_organization_admin_account(&input.admin_account_id);
        wire::serialize_disable_organization_admin_account_response(
            &wire::DisableOrganizationAdminAccountResponse {},
        )
    }

    async fn handle_disassociate_from_administrator_account(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_from_administrator_account_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.disassociate_from_administrator_account(&input.detector_id) {
            Ok(()) => wire::serialize_disassociate_from_administrator_account_response(
                &wire::DisassociateFromAdministratorAccountResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_get_master_account(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_master_account_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_master_account_for_detector(&input.detector_id) {
            Ok(Some(master_id)) => {
                wire::serialize_get_master_account_response(&wire::GetMasterAccountResponse {
                    master: Some(wire::Master {
                        account_id: Some(master_id.to_string()),
                        relationship_status: Some("Enabled".to_string()),
                        invitation_id: None,
                        invited_at: None,
                    }),
                })
            }
            Ok(None) => {
                wire::serialize_get_master_account_response(&wire::GetMasterAccountResponse {
                    master: Some(wire::Master::default()),
                })
            }
            Err(e) => guardduty_error_response(&e),
        }
    }

    async fn handle_disassociate_from_master_account(
        &self,
        state: &Arc<tokio::sync::RwLock<GuardDutyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_from_master_account_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.disassociate_from_master_account(&input.detector_id) {
            Ok(()) => wire::serialize_disassociate_from_master_account_response(
                &wire::DisassociateFromMasterAccountResponse {},
            ),
            Err(e) => guardduty_error_response(&e),
        }
    }
}

fn data_sources_from_wire(
    value: Option<&crate::model::DataSourceConfigurations>,
) -> Option<DataSourcesConfig> {
    let val = value?;
    let mut ds = DataSourcesConfig::default();
    let mut has_any = false;

    if let Some(s3) = val.s3_logs.as_ref() {
        ds.s3_logs_enabled = Some(s3.enable);
        has_any = true;
    }

    if let Some(k8s) = val.kubernetes.as_ref() {
        ds.kubernetes_audit_logs_enabled = Some(k8s.audit_logs.enable);
        has_any = true;
    }

    if has_any { Some(ds) } else { None }
}

fn features_from_wire(
    value: Option<&[crate::model::DetectorFeatureConfiguration]>,
) -> Option<Vec<DetectorFeature>> {
    let arr = value?;
    let features: Vec<DetectorFeature> = arr
        .iter()
        .filter_map(|item| {
            let name = item.name.clone()?;
            let status = item.status.clone().unwrap_or_else(|| "ENABLED".to_string());
            let additional: Vec<DetectorAdditionalConfiguration> = item
                .additional_configuration
                .as_deref()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|ac| {
                            let ac_name = ac.name.clone()?;
                            let ac_status =
                                ac.status.clone().unwrap_or_else(|| "ENABLED".to_string());
                            Some(DetectorAdditionalConfiguration {
                                name: ac_name,
                                status: ac_status,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            Some(DetectorFeature {
                name,
                status,
                additional_configuration: additional,
            })
        })
        .collect();

    if features.is_empty() {
        None
    } else {
        Some(features)
    }
}

fn finding_criteria_from_wire(value: Option<&crate::model::FindingCriteria>) -> FindingCriteria {
    let mut fc = FindingCriteria::default();
    if let Some(val) = value {
        if let Some(criterion) = val.criterion.as_ref() {
            for (key, cond_val) in criterion {
                let mut condition = Condition::default();
                if let Some(eq) = cond_val.eq.as_ref().or(cond_val.equals.as_ref()) {
                    condition.eq = Some(eq.clone());
                }
                if let Some(neq) = cond_val.neq.as_ref().or(cond_val.not_equals.as_ref()) {
                    condition.neq = Some(neq.clone());
                }
                if let Some(gt) = cond_val
                    .greater_than
                    .or_else(|| cond_val.gt.map(|v| v as i64))
                {
                    condition.gt = Some(gt);
                }
                if let Some(gte) = cond_val
                    .greater_than_or_equal
                    .or_else(|| cond_val.gte.map(|v| v as i64))
                {
                    condition.gte = Some(gte);
                }
                if let Some(lt) = cond_val.less_than.or_else(|| cond_val.lt.map(|v| v as i64)) {
                    condition.lt = Some(lt);
                }
                if let Some(lte) = cond_val
                    .less_than_or_equal
                    .or_else(|| cond_val.lte.map(|v| v as i64))
                {
                    condition.lte = Some(lte);
                }
                fc.criterion.insert(key.clone(), condition);
            }
        }
    }
    fc
}

fn convert_finding_criteria_to_wire(fc: &FindingCriteria) -> wire::FindingCriteria {
    let criterion: HashMap<String, wire::Condition> = fc
        .criterion
        .iter()
        .map(|(key, cond)| {
            let wire_cond = wire::Condition {
                eq: cond.eq.clone(),
                neq: cond.neq.clone(),
                greater_than: cond.gt,
                greater_than_or_equal: cond.gte,
                less_than: cond.lt,
                less_than_or_equal: cond.lte,
                ..Default::default()
            };
            (key.clone(), wire_cond)
        })
        .collect();
    wire::FindingCriteria {
        criterion: if criterion.is_empty() {
            None
        } else {
            Some(criterion)
        },
    }
}

fn parse_tag_keys_from_uri(uri: &str) -> Vec<String> {
    uri.split('?')
        .nth(1)
        .unwrap_or("")
        .split('&')
        .filter_map(|p| {
            let (k, v) = p.split_once('=')?;
            if k == "tagKeys" {
                Some(
                    urlencoding::decode(v)
                        .unwrap_or(std::borrow::Cow::Borrowed(v))
                        .into_owned(),
                )
            } else {
                None
            }
        })
        .collect()
}

fn extract_path(uri: &str) -> String {
    // Delegate to the shared core helper, which correctly strips the scheme
    // and host (including custom-endpoint hostnames like `127.0.0.1:PORT`)
    // before returning the path. The previous implementation only matched on
    // `amazonaws.com` and returned the entire URI for non-AWS endpoints,
    // causing dispatch to fail with 404 against the in-process mock server.
    winterbaume_core::extract_path(uri)
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

fn guardduty_error_response(err: &GuardDutyError) -> MockResponse {
    let (status, error_type) = match err {
        GuardDutyError::DetectorNotFound => (400u16, "BadRequestException"),
        GuardDutyError::FilterAlreadyExists { .. } => (400, "BadRequestException"),
        GuardDutyError::FilterNotFound { .. } => (400, "BadRequestException"),
        GuardDutyError::IpSetNotFound { .. } => (400, "BadRequestException"),
        GuardDutyError::ThreatIntelSetNotFound { .. } => (400, "BadRequestException"),
        GuardDutyError::ThreatEntitySetNotFound { .. } => (400, "BadRequestException"),
        GuardDutyError::TrustedEntitySetNotFound { .. } => (400, "BadRequestException"),
        GuardDutyError::AdminAccountAlreadyEnabled => (400, "BadRequestException"),
        GuardDutyError::PublishingDestinationNotFound { .. } => (400, "BadRequestException"),
        GuardDutyError::MalwareProtectionPlanNotFound { .. } => (404, "ResourceNotFoundException"),
        GuardDutyError::MalwareScanNotFound { .. } => (404, "ResourceNotFoundException"),
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
