use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{Macie2Error, Macie2State};
use crate::types::{MacieAllowListCriteria, MacieS3WordsList, MacieSeverityLevel};
use crate::views::Macie2StateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct Macie2Service {
    pub(crate) state: Arc<BackendState<Macie2State>>,
    pub(crate) notifier: StateChangeNotifier<Macie2StateView>,
}

impl Macie2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for Macie2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for Macie2Service {
    fn service_name(&self) -> &str {
        "macie2"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://macie2\..*\.amazonaws\.com",
            r"https?://macie2\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl Macie2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        match (method, segments.as_slice()) {
            // POST /macie - EnableMacie
            ("POST", ["macie"]) => {
                self.handle_enable_macie(&state, &request, account_id, &region)
                    .await
            }
            // GET /macie - GetMacieSession
            ("GET", ["macie"]) => self.handle_get_macie_session(&state).await,
            // DELETE /macie - DisableMacie
            ("DELETE", ["macie"]) => self.handle_disable_macie(&state).await,
            // PATCH /macie => UpdateMacieSession
            ("PATCH", ["macie"]) => self.handle_update_macie_session(&state, &request).await,
            // POST /findings - ListFindings
            ("POST", ["findings"]) => self.handle_list_findings(&state).await,
            // POST /findings/sample => CreateSampleFindings
            ("POST", ["findings", "sample"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_create_sample_findings(&state, &body).await
            }
            // POST /findings/statistics => GetFindingStatistics
            ("POST", ["findings", "statistics"]) => {
                self.handle_get_finding_statistics(&state).await
            }
            // POST /findings/describe => GetFindings
            ("POST", ["findings", "describe"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_get_findings(&state, &body).await
            }
            // GET /findings/{findingId}/reveal/availability => GetSensitiveDataOccurrencesAvailability
            ("GET", ["findings", _finding_id, "reveal", "availability"]) => {
                self.handle_get_sensitive_data_occurrences_availability(&state)
                    .await
            }
            // GET /findings/{findingId}/reveal => GetSensitiveDataOccurrences
            ("GET", ["findings", _finding_id, "reveal"]) => {
                self.handle_get_sensitive_data_occurrences(&state).await
            }
            // POST /invitations/accept => AcceptInvitation
            ("POST", ["invitations", "accept"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_accept_invitation(&state, &body).await
            }
            // POST /invitations/decline => DeclineInvitations
            ("POST", ["invitations", "decline"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_decline_invitations(&state, &body).await
            }
            // POST /invitations/delete => DeleteInvitations
            ("POST", ["invitations", "delete"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_delete_invitations(&state, &body).await
            }
            // POST /invitations => CreateInvitations
            ("POST", ["invitations"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_create_invitations(&state, &body).await
            }
            // GET /invitations/count => GetInvitationsCount
            ("GET", ["invitations", "count"]) => self.handle_get_invitations_count(&state).await,
            // GET /invitations => ListInvitations
            ("GET", ["invitations"]) => self.handle_list_invitations(&state).await,
            // DELETE /members/{id} => DeleteMember
            ("DELETE", ["members", id]) => self.handle_delete_member(&state, id).await,
            // POST /members/disassociate/{id} => DisassociateMember
            ("POST", ["members", "disassociate", id]) => {
                self.handle_disassociate_member(&state, id).await
            }
            // GET /members/{id} => GetMember
            ("GET", ["members", id]) => self.handle_get_member(&state, id).await,
            // POST /members => CreateMember
            ("POST", ["members"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_create_member(&state, &body, account_id, &region)
                    .await
            }
            // GET /members => ListMembers
            ("GET", ["members"]) => self.handle_list_members(&state, account_id).await,
            // PATCH /macie/members/{id} => UpdateMemberSession
            ("PATCH", ["macie", "members", id]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_update_member_session(&state, id, &body).await
            }
            // POST /admin => EnableOrganizationAdminAccount
            ("POST", ["admin"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_enable_organization_admin_account(&state, &body)
                    .await
            }
            // DELETE /admin => DisableOrganizationAdminAccount
            ("DELETE", ["admin"]) => {
                let query = extract_query(&request.uri);
                self.handle_disable_organization_admin_account(&state, &query)
                    .await
            }
            // GET /admin/configuration => DescribeOrganizationConfiguration
            ("GET", ["admin", "configuration"]) => {
                self.handle_describe_organization_configuration(&state)
                    .await
            }
            // PATCH /admin/configuration => UpdateOrganizationConfiguration
            ("PATCH", ["admin", "configuration"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_update_organization_configuration(&state, &body)
                    .await
            }
            // GET /admin => ListOrganizationAdminAccounts
            ("GET", ["admin"]) => self.handle_list_organization_admin_accounts(&state).await,
            // GET /administrator => GetAdministratorAccount
            ("GET", ["administrator"]) => self.handle_get_administrator_account(&state).await,
            // POST /administrator/disassociate => DisassociateFromAdministratorAccount
            ("POST", ["administrator", "disassociate"]) => {
                self.handle_disassociate_from_administrator_account(&state)
                    .await
            }
            // GET /master => GetMasterAccount
            ("GET", ["master"]) => self.handle_get_master_account(&state).await,
            // POST /master/disassociate => DisassociateFromMasterAccount
            ("POST", ["master", "disassociate"]) => {
                self.handle_disassociate_from_master_account(&state).await
            }
            // POST /custom-data-identifiers/get => BatchGetCustomDataIdentifiers
            ("POST", ["custom-data-identifiers", "get"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_batch_get_custom_data_identifiers(&state, &body)
                    .await
            }
            // POST /custom-data-identifiers/list => ListCustomDataIdentifiers
            ("POST", ["custom-data-identifiers", "list"]) => {
                self.handle_list_custom_data_identifiers(&state).await
            }
            // POST /custom-data-identifiers/test => TestCustomDataIdentifier
            ("POST", ["custom-data-identifiers", "test"]) => {
                self.handle_test_custom_data_identifier(&state).await
            }
            // DELETE /custom-data-identifiers/{id} => DeleteCustomDataIdentifier
            ("DELETE", ["custom-data-identifiers", id]) => {
                self.handle_delete_custom_data_identifier(&state, id).await
            }
            // GET /custom-data-identifiers/{id} => GetCustomDataIdentifier
            ("GET", ["custom-data-identifiers", id]) => {
                self.handle_get_custom_data_identifier(&state, id).await
            }
            // POST /custom-data-identifiers => CreateCustomDataIdentifier
            ("POST", ["custom-data-identifiers"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_create_custom_data_identifier(&state, &body, account_id, &region)
                    .await
            }
            // POST /findingsfilters => CreateFindingsFilter
            ("POST", ["findingsfilters"]) => {
                self.handle_create_findings_filter(&state, &request, account_id, &region)
                    .await
            }
            // DELETE /findingsfilters/{id} => DeleteFindingsFilter
            ("DELETE", ["findingsfilters", id]) => {
                self.handle_delete_findings_filter(&state, id).await
            }
            // GET /findingsfilters/{id} => GetFindingsFilter
            ("GET", ["findingsfilters", id]) => self.handle_get_findings_filter(&state, id).await,
            // PATCH /findingsfilters/{id} => UpdateFindingsFilter
            ("PATCH", ["findingsfilters", id]) => {
                self.handle_update_findings_filter(&state, id, &request)
                    .await
            }
            // GET /findingsfilters => ListFindingsFilters
            ("GET", ["findingsfilters"]) => self.handle_list_findings_filters(&state).await,
            // POST /allow-lists => CreateAllowList
            ("POST", ["allow-lists"]) => {
                self.handle_create_allow_list(&state, &request, account_id, &region)
                    .await
            }
            // DELETE /allow-lists/{id} => DeleteAllowList
            ("DELETE", ["allow-lists", id]) => self.handle_delete_allow_list(&state, id).await,
            // GET /allow-lists/{id} => GetAllowList
            ("GET", ["allow-lists", id]) => self.handle_get_allow_list(&state, id).await,
            // PUT /allow-lists/{id} => UpdateAllowList
            ("PUT", ["allow-lists", id]) => {
                self.handle_update_allow_list(&state, id, &request).await
            }
            // GET /allow-lists => ListAllowLists
            ("GET", ["allow-lists"]) => self.handle_list_allow_lists(&state).await,
            // POST /jobs => CreateClassificationJob
            ("POST", ["jobs"]) => {
                self.handle_create_classification_job(&state, &request, account_id, &region)
                    .await
            }
            // GET /jobs/{jobId} => DescribeClassificationJob
            ("GET", ["jobs", job_id]) => {
                self.handle_describe_classification_job(&state, job_id)
                    .await
            }
            // PATCH /jobs/{jobId} => UpdateClassificationJob
            ("PATCH", ["jobs", job_id]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_update_classification_job(&state, job_id, &body)
                    .await
            }
            // POST /jobs/list => ListClassificationJobs
            ("POST", ["jobs", "list"]) => self.handle_list_classification_jobs(&state).await,
            // GET /classification-export-configuration => GetClassificationExportConfiguration
            ("GET", ["classification-export-configuration"]) => {
                self.handle_get_classification_export_configuration(&state)
                    .await
            }
            // PUT /classification-export-configuration => PutClassificationExportConfiguration
            ("PUT", ["classification-export-configuration"]) => {
                self.handle_put_classification_export_configuration(&state, &request)
                    .await
            }
            // GET /classification-scopes/{id} => GetClassificationScope
            ("GET", ["classification-scopes", id]) => {
                self.handle_get_classification_scope(&state, id).await
            }
            // PATCH /classification-scopes/{id} => UpdateClassificationScope
            ("PATCH", ["classification-scopes", id]) => {
                self.handle_update_classification_scope(&state, id).await
            }
            // GET /classification-scopes => ListClassificationScopes
            ("GET", ["classification-scopes"]) => {
                self.handle_list_classification_scopes(&state).await
            }
            // GET /findings-publication-configuration => GetFindingsPublicationConfiguration
            ("GET", ["findings-publication-configuration"]) => {
                self.handle_get_findings_publication_configuration(&state)
                    .await
            }
            // PUT /findings-publication-configuration => PutFindingsPublicationConfiguration
            ("PUT", ["findings-publication-configuration"]) => {
                self.handle_put_findings_publication_configuration(&state, &request)
                    .await
            }
            // GET /resource-profiles/artifacts => ListResourceProfileArtifacts
            ("GET", ["resource-profiles", "artifacts"]) => {
                self.handle_list_resource_profile_artifacts(&state).await
            }
            // GET /resource-profiles/detections => ListResourceProfileDetections
            ("GET", ["resource-profiles", "detections"]) => {
                self.handle_list_resource_profile_detections(&state).await
            }
            // PATCH /resource-profiles/detections => UpdateResourceProfileDetections
            ("PATCH", ["resource-profiles", "detections"]) => {
                self.handle_update_resource_profile_detections(&state).await
            }
            // GET /resource-profiles => GetResourceProfile
            ("GET", ["resource-profiles"]) => self.handle_get_resource_profile(&state).await,
            // PATCH /resource-profiles => UpdateResourceProfile
            ("PATCH", ["resource-profiles"]) => self.handle_update_resource_profile(&state).await,
            // GET /reveal-configuration => GetRevealConfiguration
            ("GET", ["reveal-configuration"]) => self.handle_get_reveal_configuration(&state).await,
            // PUT /reveal-configuration => UpdateRevealConfiguration
            ("PUT", ["reveal-configuration"]) => {
                self.handle_update_reveal_configuration(&state, &request)
                    .await
            }
            // GET /templates/sensitivity-inspections/{id} => GetSensitivityInspectionTemplate
            ("GET", ["templates", "sensitivity-inspections", id]) => {
                self.handle_get_sensitivity_inspection_template(&state, id)
                    .await
            }
            // PUT /templates/sensitivity-inspections/{id} => UpdateSensitivityInspectionTemplate
            ("PUT", ["templates", "sensitivity-inspections", id]) => {
                self.handle_update_sensitivity_inspection_template(&state, id, &request, account_id)
                    .await
            }
            // GET /templates/sensitivity-inspections => ListSensitivityInspectionTemplates
            ("GET", ["templates", "sensitivity-inspections"]) => {
                self.handle_list_sensitivity_inspection_templates(&state)
                    .await
            }
            // GET /automated-discovery/configuration => GetAutomatedDiscoveryConfiguration
            ("GET", ["automated-discovery", "configuration"]) => {
                self.handle_get_automated_discovery_configuration(&state)
                    .await
            }
            // PUT /automated-discovery/configuration => UpdateAutomatedDiscoveryConfiguration
            ("PUT", ["automated-discovery", "configuration"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_update_automated_discovery_configuration(&state, &body, account_id)
                    .await
            }
            // GET /automated-discovery/accounts => ListAutomatedDiscoveryAccounts
            ("GET", ["automated-discovery", "accounts"]) => {
                self.handle_list_automated_discovery_accounts(&state).await
            }
            // PATCH /automated-discovery/accounts => BatchUpdateAutomatedDiscoveryAccounts
            ("PATCH", ["automated-discovery", "accounts"]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_batch_update_automated_discovery_accounts(&state, &body)
                    .await
            }
            // POST /datasources/s3 => DescribeBuckets
            ("POST", ["datasources", "s3"]) => self.handle_describe_buckets(&state).await,
            // POST /datasources/s3/statistics => GetBucketStatistics
            ("POST", ["datasources", "s3", "statistics"]) => {
                self.handle_get_bucket_statistics(&state).await
            }
            // POST /datasources/search-resources => SearchResources
            ("POST", ["datasources", "search-resources"]) => {
                self.handle_search_resources(&state).await
            }
            // POST /managed-data-identifiers/list => ListManagedDataIdentifiers
            ("POST", ["managed-data-identifiers", "list"]) => {
                self.handle_list_managed_data_identifiers(&state).await
            }
            // POST /usage/statistics => GetUsageStatistics
            ("POST", ["usage", "statistics"]) => self.handle_get_usage_statistics(&state).await,
            // GET /usage => GetUsageTotals
            ("GET", ["usage"]) => self.handle_get_usage_totals(&state).await,
            // GET /tags/{resourceArn} => ListTagsForResource
            ("GET", ["tags", resource_arn]) => {
                self.handle_list_tags_for_resource(&state, resource_arn)
                    .await
            }
            // POST /tags/{resourceArn} => TagResource
            ("POST", ["tags", resource_arn]) => {
                let body: Value = serde_json::from_slice(&request.body).unwrap_or(json!({}));
                self.handle_tag_resource(&state, resource_arn, &body).await
            }
            // DELETE /tags/{resourceArn} => UntagResource
            ("DELETE", ["tags", resource_arn]) => {
                let query = extract_query(&request.uri);
                self.handle_untag_resource(&state, resource_arn, &query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ── Session handlers ──────────────────────────────────────────────────────

    async fn handle_enable_macie(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_macie_request(request, &[], &HashMap::new()) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let status = input.status.as_deref();
        let frequency = input.finding_publishing_frequency.as_deref();

        let mut state = state.write().await;
        match state.enable_macie(status, frequency, account_id, region) {
            Ok(()) => wire::serialize_enable_macie_response(&wire::EnableMacieResponse {}),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_macie_session(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_macie_session() {
            Ok(session) => {
                wire::serialize_get_macie_session_response(&wire::GetMacieSessionResponse {
                    created_at: Some(session.created_at.to_rfc3339()),
                    finding_publishing_frequency: Some(
                        session.finding_publishing_frequency.clone(),
                    ),
                    service_role: Some(session.service_role.clone()),
                    status: Some(session.status.clone()),
                    updated_at: Some(session.updated_at.to_rfc3339()),
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_disable_macie(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.disable_macie() {
            Ok(()) => wire::serialize_disable_macie_response(&wire::DisableMacieResponse {}),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_macie_session(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_macie_session_request(request, &[], &HashMap::new()) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let status = input.status.as_deref();
        let frequency = input.finding_publishing_frequency.as_deref();
        let mut state = state.write().await;
        match state.update_macie_session(status, frequency) {
            Ok(()) => {
                wire::serialize_update_macie_session_response(&wire::UpdateMacieSessionResponse {})
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Findings handlers ─────────────────────────────────────────────────────

    async fn handle_list_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_findings() {
            Ok(finding_ids) => {
                wire::serialize_list_findings_response(&wire::ListFindingsResponse {
                    finding_ids: Some(finding_ids),
                    ..Default::default()
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_create_sample_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let finding_types: Vec<String> = body
            .get("findingTypes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let mut guard = state.write().await;
        match guard.create_sample_findings(&finding_types) {
            Ok(()) => wire::serialize_create_sample_findings_response(
                &wire::CreateSampleFindingsResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_finding_statistics(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_finding_statistics() {
            Ok(count) => {
                let counts_by_group = if count > 0 {
                    Some(vec![wire::GroupCount {
                        count: Some(count as i64),
                        group_key: Some("ALL".to_string()),
                    }])
                } else {
                    Some(vec![])
                };
                wire::serialize_get_finding_statistics_response(
                    &wire::GetFindingStatisticsResponse { counts_by_group },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let finding_ids: Vec<String> = body
            .get("findingIds")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let guard = state.read().await;
        match guard.get_findings(&finding_ids) {
            Ok(findings) => {
                let wire_findings: Vec<wire::Finding> = findings
                    .iter()
                    .map(|f| wire::Finding {
                        id: Some(f.id.clone()),
                        r#type: Some(f.finding_type.clone()),
                        title: Some(f.title.clone()),
                        description: Some(f.description.clone()),
                        category: Some(f.category.clone()),
                        sample: Some(f.sample),
                        created_at: Some(f.created_at.to_rfc3339()),
                        updated_at: Some(f.updated_at.to_rfc3339()),
                        severity: Some(wire::Severity {
                            description: Some(f.severity.clone()),
                            score: Some(3),
                        }),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_findings_response(&wire::GetFindingsResponse {
                    findings: Some(wire_findings),
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // STUB[no-engine]: Requires real object content scanning to determine sensitive data availability.
    async fn handle_get_sensitive_data_occurrences_availability(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_get_sensitive_data_occurrences_availability_response(
            &wire::GetSensitiveDataOccurrencesAvailabilityResponse {
                ..Default::default()
            },
        )
    }

    // STUB[no-engine]: Requires real object content scanning to retrieve sensitive data occurrences.
    async fn handle_get_sensitive_data_occurrences(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_get_sensitive_data_occurrences_response(
            &wire::GetSensitiveDataOccurrencesResponse {
                ..Default::default()
            },
        )
    }

    // ── Invitation handlers ───────────────────────────────────────────────────

    async fn handle_accept_invitation(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let administrator_account_id = body
            .get("administratorAccountId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let invitation_id = body
            .get("invitationId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.accept_invitation(administrator_account_id, invitation_id) {
            Ok(()) => {
                wire::serialize_accept_invitation_response(&wire::AcceptInvitationResponse {})
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_create_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let account_ids: Vec<String> = body
            .get("accountIds")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_invitations(&account_ids) {
            Ok(unprocessed_ids) => {
                let unprocessed = if unprocessed_ids.is_empty() {
                    None
                } else {
                    Some(
                        unprocessed_ids
                            .into_iter()
                            .map(|id| wire::UnprocessedAccount {
                                account_id: Some(id),
                                error_code: Some("ClientError".to_string()),
                                error_message: Some("Account is not a member.".to_string()),
                            })
                            .collect(),
                    )
                };
                wire::serialize_create_invitations_response(&wire::CreateInvitationsResponse {
                    unprocessed_accounts: unprocessed,
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_decline_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let account_ids: Vec<String> = body
            .get("accountIds")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.decline_invitations(&account_ids) {
            Ok(unprocessed_ids) => {
                let unprocessed = if unprocessed_ids.is_empty() {
                    None
                } else {
                    Some(
                        unprocessed_ids
                            .into_iter()
                            .map(|id| wire::UnprocessedAccount {
                                account_id: Some(id),
                                error_code: Some("ClientError".to_string()),
                                error_message: Some(
                                    "No invitation found from this account.".to_string(),
                                ),
                            })
                            .collect(),
                    )
                };
                wire::serialize_decline_invitations_response(&wire::DeclineInvitationsResponse {
                    unprocessed_accounts: unprocessed,
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_delete_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let account_ids: Vec<String> = body
            .get("accountIds")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.delete_invitations(&account_ids) {
            Ok(unprocessed_ids) => {
                let unprocessed = if unprocessed_ids.is_empty() {
                    None
                } else {
                    Some(
                        unprocessed_ids
                            .into_iter()
                            .map(|id| wire::UnprocessedAccount {
                                account_id: Some(id),
                                error_code: Some("ClientError".to_string()),
                                error_message: Some(
                                    "No invitation found from this account.".to_string(),
                                ),
                            })
                            .collect(),
                    )
                };
                wire::serialize_delete_invitations_response(&wire::DeleteInvitationsResponse {
                    unprocessed_accounts: unprocessed,
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_invitations_count(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let count = state.invitations.len() as i64;
        wire::serialize_get_invitations_count_response(&wire::GetInvitationsCountResponse {
            invitations_count: Some(count),
        })
    }

    async fn handle_list_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_invitations() {
            Ok(invitations) => {
                let items: Vec<wire::Invitation> = invitations
                    .iter()
                    .map(|inv| wire::Invitation {
                        account_id: Some(inv.account_id.clone()),
                        invitation_id: Some(inv.invitation_id.clone()),
                        invited_at: Some(inv.invited_at.to_rfc3339()),
                        relationship_status: Some(inv.relationship_status.clone()),
                    })
                    .collect();
                wire::serialize_list_invitations_response(&wire::ListInvitationsResponse {
                    invitations: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Member handlers ───────────────────────────────────────────────────────

    async fn handle_delete_member(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        member_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_member(member_id) {
            Ok(()) => wire::serialize_delete_member_response(&wire::DeleteMemberResponse {}),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_disassociate_member(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        member_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.disassociate_member(member_id) {
            Ok(()) => {
                wire::serialize_disassociate_member_response(&wire::DisassociateMemberResponse {})
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_member(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        member_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.members.get(member_id) {
            Some(m) => wire::serialize_get_member_response(&wire::GetMemberResponse {
                account_id: Some(m.account_id.clone()),
                email: Some(m.email.clone()),
                relationship_status: Some(m.relationship_status.clone()),
                updated_at: Some(m.updated_at.to_rfc3339()),
                ..Default::default()
            }),
            None => rest_json_error(404, "ResourceNotFoundException", "Member not found"),
        }
    }

    async fn handle_create_member(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
        account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let member_account_id = body
            .get("account")
            .and_then(|a| a.get("accountId"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let email = body
            .get("account")
            .and_then(|a| a.get("email"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.add_member(member_account_id, email) {
            Ok(arn) => wire::serialize_create_member_response(&wire::CreateMemberResponse {
                arn: Some(arn),
            }),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_members(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_members() {
            Ok(members) => {
                let items: Vec<wire::Member> = members
                    .iter()
                    .map(|m| wire::Member {
                        account_id: Some(m.account_id.clone()),
                        administrator_account_id: Some(account_id.to_string()),
                        email: Some(m.email.clone()),
                        invited_at: m.invited_at.map(|t| t.to_rfc3339()),
                        relationship_status: Some(m.relationship_status.clone()),
                        updated_at: Some(m.updated_at.to_rfc3339()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_members_response(&wire::ListMembersResponse {
                    members: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_member_session(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        member_id: &str,
        body: &Value,
    ) -> MockResponse {
        let status = body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("ENABLED");
        let mut state = state.write().await;
        match state.update_member_session(member_id, status) {
            Ok(()) => wire::serialize_update_member_session_response(
                &wire::UpdateMemberSessionResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Admin / organisation handlers ─────────────────────────────────────────

    async fn handle_enable_organization_admin_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let admin_account_id = body
            .get("adminAccountId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.enable_organization_admin_account(admin_account_id) {
            Ok(()) => wire::serialize_enable_organization_admin_account_response(
                &wire::EnableOrganizationAdminAccountResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_disable_organization_admin_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        query: &str,
    ) -> MockResponse {
        // adminAccountId comes as a query param
        let admin_account_id = query
            .split('&')
            .find_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                if k == "adminAccountId" {
                    Some(v.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.disable_organization_admin_account(&admin_account_id) {
            Ok(()) => wire::serialize_disable_organization_admin_account_response(
                &wire::DisableOrganizationAdminAccountResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_describe_organization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_org_config() {
            Ok(cfg) => wire::serialize_describe_organization_configuration_response(
                &wire::DescribeOrganizationConfigurationResponse {
                    auto_enable: cfg.map(|c| c.auto_enable),
                    max_account_limit_reached: cfg.map(|c| c.max_account_limit_reached),
                },
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_organization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let auto_enable = body
            .get("autoEnable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let mut state = state.write().await;
        match state.update_org_config(auto_enable) {
            Ok(()) => wire::serialize_update_organization_configuration_response(
                &wire::UpdateOrganizationConfigurationResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_organization_admin_accounts(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_organization_admin_accounts() {
            Ok(accounts) => {
                let items: Vec<wire::AdminAccount> = accounts
                    .iter()
                    .map(|a| wire::AdminAccount {
                        account_id: Some(a.account_id.clone()),
                        status: Some(a.status.clone()),
                    })
                    .collect();
                wire::serialize_list_organization_admin_accounts_response(
                    &wire::ListOrganizationAdminAccountsResponse {
                        admin_accounts: Some(items),
                        ..Default::default()
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_administrator_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_administrator_account() {
            Ok(admin) => {
                let administrator = admin.map(|a| wire::Invitation {
                    account_id: Some(a.account_id.clone()),
                    invitation_id: Some(a.invitation_id.clone()),
                    invited_at: Some(a.invited_at.to_rfc3339()),
                    relationship_status: Some(a.relationship_status.clone()),
                });
                wire::serialize_get_administrator_account_response(
                    &wire::GetAdministratorAccountResponse { administrator },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_disassociate_from_administrator_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.disassociate_from_administrator_account() {
            Ok(()) => wire::serialize_disassociate_from_administrator_account_response(
                &wire::DisassociateFromAdministratorAccountResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    // Legacy master account endpoints delegate to the same administrator state.
    async fn handle_get_master_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let master = state.administrator.as_ref().map(|a| wire::Invitation {
            account_id: Some(a.account_id.clone()),
            invitation_id: Some(a.invitation_id.clone()),
            invited_at: Some(a.invited_at.to_rfc3339()),
            relationship_status: Some(a.relationship_status.clone()),
        });
        wire::serialize_get_master_account_response(&wire::GetMasterAccountResponse { master })
    }

    async fn handle_disassociate_from_master_account(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        // Clears the same administrator field for the legacy endpoint.
        let _ = state.disassociate_from_administrator_account();
        wire::serialize_disassociate_from_master_account_response(
            &wire::DisassociateFromMasterAccountResponse {},
        )
    }

    // ── Custom data identifier handlers ───────────────────────────────────────

    async fn handle_create_custom_data_identifier(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = body
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let description = body
            .get("description")
            .and_then(|v| v.as_str())
            .map(String::from);
        let regex = body
            .get("regex")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let keywords: Vec<String> = body
            .get("keywords")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let ignore_words: Vec<String> = body
            .get("ignoreWords")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let maximum_match_distance = body
            .get("maximumMatchDistance")
            .and_then(|v| v.as_i64())
            .unwrap_or(50) as i32;
        let severity_levels: Vec<MacieSeverityLevel> = body
            .get("severityLevels")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|sl| MacieSeverityLevel {
                        occurrences_threshold: sl
                            .get("occurrencesThreshold")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(0),
                        severity: sl
                            .get("severity")
                            .and_then(|v| v.as_str())
                            .unwrap_or("LOW")
                            .to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        let tags: HashMap<String, String> = body
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_custom_data_identifier(
            account_id,
            region,
            name,
            description,
            regex,
            keywords,
            ignore_words,
            maximum_match_distance,
            severity_levels,
            tags,
        ) {
            Ok(id) => wire::serialize_create_custom_data_identifier_response(
                &wire::CreateCustomDataIdentifierResponse {
                    custom_data_identifier_id: Some(id),
                },
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_custom_data_identifier(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_custom_data_identifier(id) {
            Ok(cdi) => wire::serialize_get_custom_data_identifier_response(
                &wire::GetCustomDataIdentifierResponse {
                    arn: Some(cdi.arn.clone()),
                    created_at: Some(cdi.created_at.to_rfc3339()),
                    deleted: Some(cdi.deleted),
                    description: cdi.description.clone(),
                    id: Some(cdi.id.clone()),
                    ignore_words: if cdi.ignore_words.is_empty() {
                        None
                    } else {
                        Some(cdi.ignore_words.clone())
                    },
                    keywords: if cdi.keywords.is_empty() {
                        None
                    } else {
                        Some(cdi.keywords.clone())
                    },
                    maximum_match_distance: Some(cdi.maximum_match_distance),
                    name: Some(cdi.name.clone()),
                    regex: Some(cdi.regex.clone()),
                    severity_levels: if cdi.severity_levels.is_empty() {
                        None
                    } else {
                        Some(
                            cdi.severity_levels
                                .iter()
                                .map(|sl| wire::SeverityLevel {
                                    occurrences_threshold: sl.occurrences_threshold,
                                    severity: sl.severity.clone(),
                                })
                                .collect(),
                        )
                    },
                    tags: if cdi.tags.is_empty() {
                        None
                    } else {
                        Some(cdi.tags.clone())
                    },
                },
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_delete_custom_data_identifier(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_custom_data_identifier(id) {
            Ok(()) => wire::serialize_delete_custom_data_identifier_response(
                &wire::DeleteCustomDataIdentifierResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_custom_data_identifiers(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_custom_data_identifiers() {
            Ok(items) => {
                let summaries: Vec<wire::CustomDataIdentifierSummary> = items
                    .iter()
                    .map(|c| wire::CustomDataIdentifierSummary {
                        arn: Some(c.arn.clone()),
                        created_at: Some(c.created_at.to_rfc3339()),
                        description: c.description.clone(),
                        id: Some(c.id.clone()),
                        name: Some(c.name.clone()),
                    })
                    .collect();
                wire::serialize_list_custom_data_identifiers_response(
                    &wire::ListCustomDataIdentifiersResponse {
                        items: if summaries.is_empty() {
                            None
                        } else {
                            Some(summaries)
                        },
                        ..Default::default()
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_batch_get_custom_data_identifiers(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let ids: Vec<String> = body
            .get("ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let state = state.read().await;
        match state.batch_get_custom_data_identifiers(&ids) {
            Ok((found, not_found)) => {
                let summaries: Vec<wire::BatchGetCustomDataIdentifierSummary> = found
                    .iter()
                    .map(|c| wire::BatchGetCustomDataIdentifierSummary {
                        arn: Some(c.arn.clone()),
                        created_at: Some(c.created_at.to_rfc3339()),
                        deleted: Some(c.deleted),
                        description: c.description.clone(),
                        id: Some(c.id.clone()),
                        name: Some(c.name.clone()),
                    })
                    .collect();
                wire::serialize_batch_get_custom_data_identifiers_response(
                    &wire::BatchGetCustomDataIdentifiersResponse {
                        custom_data_identifiers: if summaries.is_empty() {
                            None
                        } else {
                            Some(summaries)
                        },
                        not_found_identifier_ids: if not_found.is_empty() {
                            None
                        } else {
                            Some(not_found)
                        },
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // STUB[no-engine]: Testing custom regex-based identifiers requires a real classification engine.
    async fn handle_test_custom_data_identifier(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_test_custom_data_identifier_response(
            &wire::TestCustomDataIdentifierResponse {
                match_count: Some(0),
            },
        )
    }

    // ── Allow list handlers ───────────────────────────────────────────────────

    async fn handle_create_allow_list(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_allow_list_request(request, &[], &HashMap::new())
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name;
        let description = input.description;
        let criteria = parse_allow_list_criteria(&input.criteria);
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_allow_list(account_id, region, name, description, criteria, tags) {
            Ok(id) => {
                let arn = state
                    .allow_lists
                    .get(&id)
                    .map(|al| al.arn.clone())
                    .unwrap_or_default();
                wire::serialize_create_allow_list_response(&wire::CreateAllowListResponse {
                    arn: Some(arn),
                    id: Some(id),
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_allow_list(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_allow_list(id) {
            Ok(al) => {
                wire::serialize_get_allow_list_response(&wire::GetAllowListResponse {
                    arn: Some(al.arn.clone()),
                    created_at: Some(al.created_at.to_rfc3339()),
                    criteria: Some(wire::AllowListCriteria {
                        regex: al.criteria.regex.clone(),
                        s3_words_list: al.criteria.s3_words_list.as_ref().map(|s| {
                            wire::S3WordsList {
                                bucket_name: s.bucket_name.clone(),
                                object_key: s.object_key.clone(),
                            }
                        }),
                    }),
                    description: al.description.clone(),
                    id: Some(al.id.clone()),
                    name: Some(al.name.clone()),
                    status: Some(wire::AllowListStatus {
                        code: Some(al.status_code.clone()),
                        description: None,
                    }),
                    tags: if al.tags.is_empty() {
                        None
                    } else {
                        Some(al.tags.clone())
                    },
                    updated_at: Some(al.updated_at.to_rfc3339()),
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_allow_list(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("id", id)];
        let input =
            match wire::deserialize_update_allow_list_request(request, labels, &HashMap::new()) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let name = input.name;
        let description = input.description;
        let criteria = parse_allow_list_criteria(&input.criteria);

        let mut state = state.write().await;
        match state.update_allow_list(id, name, description, criteria) {
            Ok(arn) => wire::serialize_update_allow_list_response(&wire::UpdateAllowListResponse {
                arn: Some(arn),
                id: Some(id.to_string()),
            }),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_delete_allow_list(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_allow_list(id) {
            Ok(()) => wire::serialize_delete_allow_list_response(&wire::DeleteAllowListResponse {}),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_allow_lists(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_allow_lists() {
            Ok(items) => {
                let summaries: Vec<wire::AllowListSummary> = items
                    .iter()
                    .map(|al| wire::AllowListSummary {
                        arn: Some(al.arn.clone()),
                        created_at: Some(al.created_at.to_rfc3339()),
                        description: al.description.clone(),
                        id: Some(al.id.clone()),
                        name: Some(al.name.clone()),
                        updated_at: Some(al.updated_at.to_rfc3339()),
                    })
                    .collect();
                wire::serialize_list_allow_lists_response(&wire::ListAllowListsResponse {
                    allow_lists: if summaries.is_empty() {
                        None
                    } else {
                        Some(summaries)
                    },
                    ..Default::default()
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Findings filter handlers ───────────────────────────────────────────────

    async fn handle_create_findings_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_findings_filter_request(request, &[], &HashMap::new()) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let name = input.name;
        let description = input.description;
        let action = if input.action.is_empty() {
            "ARCHIVE".to_string()
        } else {
            input.action
        };
        let position = input.position;
        let finding_criteria = serde_json::to_value(&input.finding_criteria).unwrap_or(json!({}));
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_findings_filter(
            account_id,
            region,
            name,
            description,
            action,
            position,
            finding_criteria,
            tags,
        ) {
            Ok(id) => {
                let arn = state
                    .findings_filters
                    .get(&id)
                    .map(|ff| ff.arn.clone())
                    .unwrap_or_default();
                wire::serialize_create_findings_filter_response(
                    &wire::CreateFindingsFilterResponse {
                        arn: Some(arn),
                        id: Some(id),
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_get_findings_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_findings_filter(id) {
            Ok(ff) => {
                let finding_criteria: Option<wire::FindingCriteria> =
                    serde_json::from_value(ff.finding_criteria.clone()).ok();
                wire::serialize_get_findings_filter_response(&wire::GetFindingsFilterResponse {
                    action: Some(ff.action.clone()),
                    arn: Some(ff.arn.clone()),
                    description: ff.description.clone(),
                    finding_criteria,
                    id: Some(ff.id.clone()),
                    name: Some(ff.name.clone()),
                    position: Some(ff.position),
                    tags: if ff.tags.is_empty() {
                        None
                    } else {
                        Some(ff.tags.clone())
                    },
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_findings_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("id", id)];
        let input = match wire::deserialize_update_findings_filter_request(
            request,
            labels,
            &HashMap::new(),
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name;
        let description = input.description;
        let action = input.action;
        let position = input.position;
        let finding_criteria = input
            .finding_criteria
            .as_ref()
            .and_then(|fc| serde_json::to_value(fc).ok());

        let mut state = state.write().await;
        match state.update_findings_filter(
            id,
            name,
            description,
            action,
            position,
            finding_criteria,
        ) {
            Ok(arn) => wire::serialize_update_findings_filter_response(
                &wire::UpdateFindingsFilterResponse {
                    arn: Some(arn),
                    id: Some(id.to_string()),
                },
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_delete_findings_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_findings_filter(id) {
            Ok(()) => wire::serialize_delete_findings_filter_response(
                &wire::DeleteFindingsFilterResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_findings_filters(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_findings_filters() {
            Ok(items) => {
                let list_items: Vec<wire::FindingsFilterListItem> = items
                    .iter()
                    .map(|ff| wire::FindingsFilterListItem {
                        action: Some(ff.action.clone()),
                        arn: Some(ff.arn.clone()),
                        id: Some(ff.id.clone()),
                        name: Some(ff.name.clone()),
                        tags: if ff.tags.is_empty() {
                            None
                        } else {
                            Some(ff.tags.clone())
                        },
                    })
                    .collect();
                wire::serialize_list_findings_filters_response(&wire::ListFindingsFiltersResponse {
                    findings_filter_list_items: if list_items.is_empty() {
                        None
                    } else {
                        Some(list_items)
                    },
                    ..Default::default()
                })
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Classification job handlers ───────────────────────────────────────────

    async fn handle_create_classification_job(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_classification_job_request(
            request,
            &[],
            &HashMap::new(),
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name;
        let description = input.description;
        let job_type = if input.job_type.is_empty() {
            "ONE_TIME".to_string()
        } else {
            input.job_type
        };
        let client_token = input.client_token;
        let s3_job_definition = serde_json::to_value(&input.s3_job_definition).unwrap_or(json!({}));
        let allow_list_ids = input.allow_list_ids.unwrap_or_default();
        let custom_data_identifier_ids = input.custom_data_identifier_ids.unwrap_or_default();
        let managed_data_identifier_ids = input.managed_data_identifier_ids.unwrap_or_default();
        let managed_data_identifier_selector = input.managed_data_identifier_selector;
        let sampling_percentage = input.sampling_percentage;
        let schedule_frequency = input
            .schedule_frequency
            .as_ref()
            .and_then(|sf| serde_json::to_value(sf).ok());
        let initial_run = input.initial_run.unwrap_or(false);
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_classification_job(
            account_id,
            region,
            name,
            description,
            job_type,
            client_token,
            s3_job_definition,
            allow_list_ids,
            custom_data_identifier_ids,
            managed_data_identifier_ids,
            managed_data_identifier_selector,
            sampling_percentage,
            schedule_frequency,
            initial_run,
            tags,
        ) {
            Ok(id) => {
                let arn = state
                    .classification_jobs
                    .get(&id)
                    .map(|j| j.job_arn.clone())
                    .unwrap_or_default();
                wire::serialize_create_classification_job_response(
                    &wire::CreateClassificationJobResponse {
                        job_arn: Some(arn),
                        job_id: Some(id),
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_describe_classification_job(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        job_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_classification_job(job_id) {
            Ok(job) => {
                let s3_def: Option<wire::S3JobDefinition> =
                    serde_json::from_value(job.s3_job_definition.clone()).ok();
                let schedule: Option<wire::JobScheduleFrequency> = job
                    .schedule_frequency
                    .as_ref()
                    .and_then(|v| serde_json::from_value(v.clone()).ok());
                wire::serialize_describe_classification_job_response(
                    &wire::DescribeClassificationJobResponse {
                        allow_list_ids: if job.allow_list_ids.is_empty() {
                            None
                        } else {
                            Some(job.allow_list_ids.clone())
                        },
                        client_token: Some(job.client_token.clone()),
                        created_at: Some(job.created_at.to_rfc3339()),
                        custom_data_identifier_ids: if job.custom_data_identifier_ids.is_empty() {
                            None
                        } else {
                            Some(job.custom_data_identifier_ids.clone())
                        },
                        description: job.description.clone(),
                        initial_run: Some(job.initial_run),
                        job_arn: Some(job.job_arn.clone()),
                        job_id: Some(job.job_id.clone()),
                        job_status: Some(job.job_status.clone()),
                        job_type: Some(job.job_type.clone()),
                        last_run_error_status: None,
                        last_run_time: job.last_run_time.map(|t| t.to_rfc3339()),
                        managed_data_identifier_ids: if job.managed_data_identifier_ids.is_empty() {
                            None
                        } else {
                            Some(job.managed_data_identifier_ids.clone())
                        },
                        managed_data_identifier_selector: job
                            .managed_data_identifier_selector
                            .clone(),
                        name: Some(job.name.clone()),
                        s3_job_definition: s3_def,
                        sampling_percentage: job.sampling_percentage,
                        schedule_frequency: schedule,
                        statistics: None,
                        tags: if job.tags.is_empty() {
                            None
                        } else {
                            Some(job.tags.clone())
                        },
                        user_paused_details: None,
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_classification_job(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        job_id: &str,
        body: &Value,
    ) -> MockResponse {
        let job_status = body
            .get("jobStatus")
            .and_then(|v| v.as_str())
            .unwrap_or("CANCELLED");
        let mut state = state.write().await;
        match state.update_classification_job(job_id, job_status) {
            Ok(()) => wire::serialize_update_classification_job_response(
                &wire::UpdateClassificationJobResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_classification_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_classification_jobs() {
            Ok(jobs) => {
                let items: Vec<wire::JobSummary> = jobs
                    .iter()
                    .map(|j| wire::JobSummary {
                        created_at: Some(j.created_at.to_rfc3339()),
                        job_id: Some(j.job_id.clone()),
                        job_status: Some(j.job_status.clone()),
                        job_type: Some(j.job_type.clone()),
                        name: Some(j.name.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_classification_jobs_response(
                    &wire::ListClassificationJobsResponse {
                        items: if items.is_empty() { None } else { Some(items) },
                        ..Default::default()
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Classification export configuration handlers ───────────────────────────

    async fn handle_get_classification_export_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_classification_export_config() {
            Ok(cfg) => {
                let configuration: Option<wire::ClassificationExportConfiguration> =
                    cfg.and_then(|c| serde_json::from_value(c.raw.clone()).ok());
                wire::serialize_get_classification_export_configuration_response(
                    &wire::GetClassificationExportConfigurationResponse { configuration },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_put_classification_export_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match wire::deserialize_put_classification_export_configuration_request(
            request,
            &[],
            &HashMap::new(),
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let raw = serde_json::to_value(&input.configuration).unwrap_or(json!({}));
        let mut state = state.write().await;
        match state.put_classification_export_config(raw) {
            Ok(raw_out) => {
                let configuration: Option<wire::ClassificationExportConfiguration> =
                    serde_json::from_value(raw_out).ok();
                wire::serialize_put_classification_export_configuration_response(
                    &wire::PutClassificationExportConfigurationResponse { configuration },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Classification scope handlers (no-state — no real engine) ─────────────

    // STUB[no-engine]: Classification scopes are tied to automated discovery which has no real engine.
    async fn handle_get_classification_scope(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        wire::serialize_get_classification_scope_response(&wire::GetClassificationScopeResponse {
            id: Some(id.to_string()),
            name: Some("default".to_string()),
            ..Default::default()
        })
    }

    // STUB[no-engine]: Classification scope updates require a real automated-discovery engine.
    async fn handle_update_classification_scope(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
        _id: &str,
    ) -> MockResponse {
        wire::serialize_update_classification_scope_response(
            &wire::UpdateClassificationScopeResponse {},
        )
    }

    // STUB[no-engine]: Classification scope list requires a real automated-discovery engine.
    async fn handle_list_classification_scopes(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_list_classification_scopes_response(
            &wire::ListClassificationScopesResponse {
                ..Default::default()
            },
        )
    }

    // ── Findings publication configuration handlers ───────────────────────────

    async fn handle_get_findings_publication_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_findings_publication_config() {
            Ok(cfg) => {
                let security_hub_configuration = cfg.map(|c| wire::SecurityHubConfiguration {
                    publish_classification_findings: c.publish_classification_findings,
                    publish_policy_findings: c.publish_policy_findings,
                });
                wire::serialize_get_findings_publication_configuration_response(
                    &wire::GetFindingsPublicationConfigurationResponse {
                        security_hub_configuration,
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_put_findings_publication_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match wire::deserialize_put_findings_publication_configuration_request(
            request,
            &[],
            &HashMap::new(),
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let publish_classification = input
            .security_hub_configuration
            .as_ref()
            .map(|h| h.publish_classification_findings)
            .unwrap_or(false);
        let publish_policy = input
            .security_hub_configuration
            .as_ref()
            .map(|h| h.publish_policy_findings)
            .unwrap_or(true);

        let mut state = state.write().await;
        match state.put_findings_publication_config(publish_classification, publish_policy) {
            Ok(()) => wire::serialize_put_findings_publication_configuration_response(
                &wire::PutFindingsPublicationConfigurationResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Resource profile handlers (s3-integration stubs) ─────────────────────

    // STUB[s3-integration]: Requires real S3 bucket scanning to list profile artifacts.
    async fn handle_list_resource_profile_artifacts(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_list_resource_profile_artifacts_response(
            &wire::ListResourceProfileArtifactsResponse {
                ..Default::default()
            },
        )
    }

    // STUB[s3-integration]: Requires real S3 object scanning to list sensitive data detections.
    async fn handle_list_resource_profile_detections(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_list_resource_profile_detections_response(
            &wire::ListResourceProfileDetectionsResponse {
                ..Default::default()
            },
        )
    }

    // STUB[s3-integration]: Requires real S3 bucket metadata to update detection suppression.
    async fn handle_update_resource_profile_detections(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_update_resource_profile_detections_response(
            &wire::UpdateResourceProfileDetectionsResponse {},
        )
    }

    // STUB[s3-integration]: Requires real S3 bucket metadata and object scanning for resource profiles.
    async fn handle_get_resource_profile(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_get_resource_profile_response(&wire::GetResourceProfileResponse {
            ..Default::default()
        })
    }

    // STUB[s3-integration]: Requires real S3 bucket metadata to update sensitivity scoring.
    async fn handle_update_resource_profile(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_update_resource_profile_response(&wire::UpdateResourceProfileResponse {})
    }

    // ── Reveal configuration handlers ─────────────────────────────────────────

    async fn handle_get_reveal_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_reveal_config() {
            Ok(cfg) => {
                let configuration = cfg.map(|c| wire::RevealConfiguration {
                    kms_key_id: c.kms_key_id.clone(),
                    status: c.status.clone(),
                });
                let retrieval_configuration = cfg.map(|c| wire::RetrievalConfiguration {
                    retrieval_mode: Some(c.retrieval_mode.clone()),
                    role_name: c.role_name.clone(),
                    external_id: None,
                });
                wire::serialize_get_reveal_configuration_response(
                    &wire::GetRevealConfigurationResponse {
                        configuration,
                        retrieval_configuration,
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_reveal_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let input = match wire::deserialize_update_reveal_configuration_request(
            request,
            &[],
            &HashMap::new(),
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let status = if input.configuration.status.is_empty() {
            "DISABLED".to_string()
        } else {
            input.configuration.status
        };
        let kms_key_id = input.configuration.kms_key_id;
        let (retrieval_mode, role_name): (String, Option<String>) =
            match input.retrieval_configuration {
                Some(r) => {
                    let mode = if r.retrieval_mode.is_empty() {
                        "CALLER_CREDENTIALS".to_string()
                    } else {
                        r.retrieval_mode
                    };
                    (mode, r.role_name)
                }
                None => ("CALLER_CREDENTIALS".to_string(), None),
            };

        let mut state = state.write().await;
        match state.update_reveal_config(
            status.clone(),
            kms_key_id.clone(),
            retrieval_mode.clone(),
            role_name.clone(),
        ) {
            Ok(()) => wire::serialize_update_reveal_configuration_response(
                &wire::UpdateRevealConfigurationResponse {
                    configuration: Some(wire::RevealConfiguration { status, kms_key_id }),
                    retrieval_configuration: Some(wire::RetrievalConfiguration {
                        retrieval_mode: Some(retrieval_mode),
                        role_name,
                        external_id: None,
                    }),
                },
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Sensitivity inspection template handlers ──────────────────────────────

    async fn handle_get_sensitivity_inspection_template(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_sensitivity_inspection_template(id) {
            Ok(tmpl) => wire::serialize_get_sensitivity_inspection_template_response(
                &wire::GetSensitivityInspectionTemplateResponse {
                    description: tmpl.description.clone(),
                    excludes: if tmpl.excludes_managed_data_identifier_ids.is_empty() {
                        None
                    } else {
                        Some(wire::SensitivityInspectionTemplateExcludes {
                            managed_data_identifier_ids: Some(
                                tmpl.excludes_managed_data_identifier_ids.clone(),
                            ),
                        })
                    },
                    includes: if tmpl.includes_allow_list_ids.is_empty()
                        && tmpl.includes_custom_data_identifier_ids.is_empty()
                        && tmpl.includes_managed_data_identifier_ids.is_empty()
                    {
                        None
                    } else {
                        Some(wire::SensitivityInspectionTemplateIncludes {
                            allow_list_ids: if tmpl.includes_allow_list_ids.is_empty() {
                                None
                            } else {
                                Some(tmpl.includes_allow_list_ids.clone())
                            },
                            custom_data_identifier_ids: if tmpl
                                .includes_custom_data_identifier_ids
                                .is_empty()
                            {
                                None
                            } else {
                                Some(tmpl.includes_custom_data_identifier_ids.clone())
                            },
                            managed_data_identifier_ids: if tmpl
                                .includes_managed_data_identifier_ids
                                .is_empty()
                            {
                                None
                            } else {
                                Some(tmpl.includes_managed_data_identifier_ids.clone())
                            },
                        })
                    },
                    name: Some(tmpl.name.clone()),
                    sensitivity_inspection_template_id: Some(tmpl.id.clone()),
                },
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_sensitivity_inspection_template(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        id: &str,
        request: &MockRequest,
        account_id: &str,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("id", id)];
        let input = match wire::deserialize_update_sensitivity_inspection_template_request(
            request,
            labels,
            &HashMap::new(),
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description;
        let excludes_managed: Vec<String> = input
            .excludes
            .and_then(|e| e.managed_data_identifier_ids)
            .unwrap_or_default();
        let (includes_allow_list, includes_custom, includes_managed): (
            Vec<String>,
            Vec<String>,
            Vec<String>,
        ) = match input.includes {
            Some(i) => (
                i.allow_list_ids.unwrap_or_default(),
                i.custom_data_identifier_ids.unwrap_or_default(),
                i.managed_data_identifier_ids.unwrap_or_default(),
            ),
            None => (Vec::new(), Vec::new(), Vec::new()),
        };

        let mut state = state.write().await;
        // Derive a stable name: either from existing template or default.
        let name = state
            .sensitivity_inspection_templates
            .get(id)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| format!("default-{}", account_id));

        match state.upsert_sensitivity_inspection_template(
            account_id,
            Some(id.to_string()),
            name,
            description,
            excludes_managed,
            includes_allow_list,
            includes_custom,
            includes_managed,
        ) {
            Ok(_) => wire::serialize_update_sensitivity_inspection_template_response(
                &wire::UpdateSensitivityInspectionTemplateResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_sensitivity_inspection_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_sensitivity_inspection_templates() {
            Ok(items) => {
                let entries: Vec<wire::SensitivityInspectionTemplatesEntry> = items
                    .iter()
                    .map(|t| wire::SensitivityInspectionTemplatesEntry {
                        id: Some(t.id.clone()),
                        name: Some(t.name.clone()),
                    })
                    .collect();
                wire::serialize_list_sensitivity_inspection_templates_response(
                    &wire::ListSensitivityInspectionTemplatesResponse {
                        sensitivity_inspection_templates: if entries.is_empty() {
                            None
                        } else {
                            Some(entries)
                        },
                        ..Default::default()
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── Automated discovery configuration handlers ────────────────────────────

    async fn handle_get_automated_discovery_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_automated_discovery_config() {
            Ok(cfg) => wire::serialize_get_automated_discovery_configuration_response(
                &wire::GetAutomatedDiscoveryConfigurationResponse {
                    auto_enable_organization_members: cfg
                        .and_then(|c| c.auto_enable_organization_members.clone()),
                    classification_scope_id: cfg.map(|c| c.classification_scope_id.clone()),
                    disabled_at: cfg.and_then(|c| c.disabled_at.map(|t| t.to_rfc3339())),
                    first_enabled_at: cfg.and_then(|c| c.first_enabled_at.map(|t| t.to_rfc3339())),
                    last_updated_at: cfg.map(|c| c.last_updated_at.to_rfc3339()),
                    sensitivity_inspection_template_id: cfg
                        .map(|c| c.sensitivity_inspection_template_id.clone()),
                    status: cfg.map(|c| c.status.clone()),
                },
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_update_automated_discovery_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let status = body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("ENABLED")
            .to_string();
        let auto_enable = body
            .get("autoEnableOrganizationMembers")
            .and_then(|v| v.as_str())
            .map(String::from);

        let mut state = state.write().await;
        match state.update_automated_discovery_config(account_id, status, auto_enable) {
            Ok(()) => wire::serialize_update_automated_discovery_configuration_response(
                &wire::UpdateAutomatedDiscoveryConfigurationResponse {},
            ),
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_list_automated_discovery_accounts(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_automated_discovery_accounts() {
            Ok(accounts) => {
                let items: Vec<wire::AutomatedDiscoveryAccount> = accounts
                    .iter()
                    .map(|a| wire::AutomatedDiscoveryAccount {
                        account_id: Some(a.account_id.clone()),
                        status: Some(a.status.clone()),
                    })
                    .collect();
                wire::serialize_list_automated_discovery_accounts_response(
                    &wire::ListAutomatedDiscoveryAccountsResponse {
                        items: if items.is_empty() { None } else { Some(items) },
                        ..Default::default()
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    async fn handle_batch_update_automated_discovery_accounts(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        body: &Value,
    ) -> MockResponse {
        let updates: Vec<(String, String)> = body
            .get("accounts")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let account_id = item.get("accountId")?.as_str()?.to_string();
                        let status = item.get("status")?.as_str()?.to_string();
                        Some((account_id, status))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.batch_update_automated_discovery_accounts(updates) {
            Ok(error_ids) => {
                let errors: Vec<wire::AutomatedDiscoveryAccountUpdateError> = error_ids
                    .into_iter()
                    .map(|id| wire::AutomatedDiscoveryAccountUpdateError {
                        account_id: Some(id),
                        error_code: Some("InvalidParameter".to_string()),
                    })
                    .collect();
                wire::serialize_batch_update_automated_discovery_accounts_response(
                    &wire::BatchUpdateAutomatedDiscoveryAccountsResponse {
                        errors: if errors.is_empty() {
                            None
                        } else {
                            Some(errors)
                        },
                    },
                )
            }
            Err(e) => macie2_error_response(&e),
        }
    }

    // ── S3-integration stubs ──────────────────────────────────────────────────

    // STUB[s3-integration]: Requires real S3 bucket metadata to enumerate monitored buckets.
    async fn handle_describe_buckets(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_describe_buckets_response(&wire::DescribeBucketsResponse {
            ..Default::default()
        })
    }

    // STUB[s3-integration]: Requires real S3 bucket metadata and object counts for statistics.
    async fn handle_get_bucket_statistics(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_get_bucket_statistics_response(&wire::GetBucketStatisticsResponse {
            ..Default::default()
        })
    }

    // STUB[s3-integration]: Requires real S3 bucket metadata to search monitored resources.
    async fn handle_search_resources(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        wire::serialize_search_resources_response(&wire::SearchResourcesResponse {
            ..Default::default()
        })
    }

    // ── Miscellaneous handlers ────────────────────────────────────────────────

    // Returns a static catalogue of well-known managed data identifier categories.
    async fn handle_list_managed_data_identifiers(
        &self,
        _state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let items = vec![
            wire::ManagedDataIdentifierSummary {
                category: Some("CREDENTIALS".to_string()),
                id: Some("AWS_CREDENTIALS".to_string()),
            },
            wire::ManagedDataIdentifierSummary {
                category: Some("FINANCIAL_INFORMATION".to_string()),
                id: Some("CREDIT_CARD_NUMBER".to_string()),
            },
            wire::ManagedDataIdentifierSummary {
                category: Some("PERSONAL_INFORMATION".to_string()),
                id: Some("EMAIL_ADDRESS".to_string()),
            },
            wire::ManagedDataIdentifierSummary {
                category: Some("PERSONAL_INFORMATION".to_string()),
                id: Some("PHONE_NUMBER".to_string()),
            },
        ];
        wire::serialize_list_managed_data_identifiers_response(
            &wire::ListManagedDataIdentifiersResponse {
                items: Some(items),
                next_token: None,
            },
        )
    }

    // Returns synthetic usage statistics based on job count.
    async fn handle_get_usage_statistics(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let job_count = guard.classification_jobs.len();
        let records = if guard.session.is_some() {
            vec![wire::UsageRecord {
                account_id: None,
                automated_discovery_free_trial_start_date: None,
                free_trial_start_date: None,
                usage: Some(vec![wire::UsageByAccount {
                    currency: Some("USD".to_string()),
                    estimated_cost: Some(format!("{:.2}", job_count as f64 * 0.10)),
                    service_limit: None,
                    r#type: Some("DATA_INVENTORY_EVALUATION".to_string()),
                }]),
            }]
        } else {
            vec![]
        };
        wire::serialize_get_usage_statistics_response(&wire::GetUsageStatisticsResponse {
            records: Some(records),
            next_token: None,
            time_range: Some("MONTH_TO_DATE".to_string()),
        })
    }

    // Returns synthetic usage totals.
    async fn handle_get_usage_totals(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let job_count = guard.classification_jobs.len();
        let totals = if guard.session.is_some() {
            vec![wire::UsageTotal {
                currency: Some("USD".to_string()),
                estimated_cost: Some(format!("{:.2}", job_count as f64 * 0.10)),
                r#type: Some("DATA_INVENTORY_EVALUATION".to_string()),
            }]
        } else {
            vec![]
        };
        wire::serialize_get_usage_totals_response(&wire::GetUsageTotalsResponse {
            time_range: Some("MONTH_TO_DATE".to_string()),
            usage_totals: Some(totals),
        })
    }

    // ── Tag resource handlers ─────────────────────────────────────────────────

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags_for_resource(resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        resource_arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags: HashMap<String, String> = body
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let mut state = state.write().await;
        state.tag_resource(resource_arn, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Macie2State>>,
        resource_arn: &str,
        query: &str,
    ) -> MockResponse {
        // Tag keys come as repeated query params: ?tagKeys=key1&tagKeys=key2
        let tag_keys: Vec<String> = query
            .split('&')
            .filter_map(|pair| {
                let (key, value) = pair.split_once('=')?;
                if key == "tagKeys" {
                    Some(
                        urlencoding::decode(value)
                            .unwrap_or(std::borrow::Cow::Borrowed(value))
                            .into_owned(),
                    )
                } else {
                    None
                }
            })
            .collect();
        let mut state = state.write().await;
        state.untag_resource(resource_arn, &tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

// ── Helper functions ──────────────────────────────────────────────────────────

fn parse_allow_list_criteria(criteria: &wire::AllowListCriteria) -> MacieAllowListCriteria {
    MacieAllowListCriteria {
        regex: criteria.regex.clone(),
        s3_words_list: criteria.s3_words_list.as_ref().map(|s| MacieS3WordsList {
            bucket_name: s.bucket_name.clone(),
            object_key: s.object_key.clone(),
        }),
    }
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
    if let Some(q) = uri.find('?') {
        uri[q + 1..].to_string()
    } else {
        String::new()
    }
}

fn macie2_error_shape(err: &Macie2Error) -> (u16, &'static str) {
    match err {
        Macie2Error::MacieAlreadyEnabled => (409, "ConflictException"),
        Macie2Error::MacieNotEnabled => (403, "AccessDeniedException"),
        Macie2Error::InvitationNotFound => (404, "ResourceNotFoundException"),
        Macie2Error::MemberNotFound(_) => (404, "ResourceNotFoundException"),
        Macie2Error::MemberAlreadyExists => (409, "ConflictException"),
        Macie2Error::AdminAlreadyDelegated => (409, "ConflictException"),
        Macie2Error::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
    }
}

fn macie2_error_response(err: &Macie2Error) -> MockResponse {
    let (status, error_type) = macie2_error_shape(err);
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
