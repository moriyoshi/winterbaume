use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::model;
use crate::state::{AuditManagerError, AuditManagerState};
use crate::views::AuditManagerStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AuditManagerService {
    pub(crate) state: Arc<BackendState<AuditManagerState>>,
    pub(crate) notifier: StateChangeNotifier<AuditManagerStateView>,
}

impl AuditManagerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AuditManagerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AuditManagerService {
    fn service_name(&self) -> &str {
        "auditmanager"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://auditmanager\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AuditManagerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = match request.uri.find('?') {
            Some(idx) => request.uri[idx + 1..].to_string(),
            None => String::new(),
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str();
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let response = match (method, segments.as_slice()) {
            // Account operations
            ("GET", ["account", "status"]) => self.handle_get_account_status(&state).await,
            ("POST", ["account", "registerAccount"]) => self.handle_register_account(&state).await,
            ("POST", ["account", "deregisterAccount"]) => {
                self.handle_deregister_account(&state).await
            }
            // Control operations
            ("POST", ["controls"]) => {
                self.handle_create_control(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["controls"]) => self.handle_list_controls(&state).await,
            ("GET", ["controls", id]) => {
                let id = percent_decode(id);
                self.handle_get_control(&state, &id).await
            }
            ("DELETE", ["controls", id]) => {
                let id = percent_decode(id);
                self.handle_delete_control(&state, &id).await
            }
            // Framework operations
            ("POST", ["assessmentFrameworks"]) => {
                self.handle_create_assessment_framework(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["assessmentFrameworks"]) => {
                self.handle_list_assessment_frameworks(&state).await
            }
            ("GET", ["assessmentFrameworks", id]) => {
                let id = percent_decode(id);
                self.handle_get_assessment_framework(&state, &id).await
            }
            ("DELETE", ["assessmentFrameworks", id]) => {
                let id = percent_decode(id);
                self.handle_delete_assessment_framework(&state, &id).await
            }
            // Assessment operations
            ("POST", ["assessments"]) => {
                self.handle_create_assessment(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["assessments"]) => self.handle_list_assessments(&state).await,
            ("GET", ["assessments", id]) => {
                let id = percent_decode(id);
                self.handle_get_assessment(&state, &id).await
            }
            ("DELETE", ["assessments", id]) => {
                let id = percent_decode(id);
                self.handle_delete_assessment(&state, &id).await
            }
            // --- Unimplemented operations ---
            // --- Unimplemented operations (auto-generated stubs) ---
            // PUT /assessments/{assessmentId}/associateToAssessmentReport => AssociateAssessmentReportEvidenceFolder (not implemented)
            // PUT /assessments/{assessmentId}/batchAssociateToAssessmentReport => BatchAssociateAssessmentReportEvidence (not implemented)
            // POST /assessments/{assessmentId}/delegations => BatchCreateDelegationByAssessment (not implemented)
            // PUT /assessments/{assessmentId}/delegations => BatchDeleteDelegationByAssessment (not implemented)
            // PUT /assessments/{assessmentId}/batchDisassociateFromAssessmentReport => BatchDisassociateAssessmentReportEvidence (not implemented)
            // POST /assessments/{assessmentId}/controlSets/{controlSetId}/controls/{controlId}/evidence => BatchImportEvidenceToAssessmentControl (not implemented)
            // POST /assessments/{assessmentId}/reports => CreateAssessmentReport (not implemented)
            // DELETE /assessmentFrameworkShareRequests/{requestId} => DeleteAssessmentFrameworkShare (not implemented)
            // DELETE /assessments/{assessmentId}/reports/{assessmentReportId} => DeleteAssessmentReport (not implemented)
            // POST /account/deregisterOrganizationAdminAccount => DeregisterOrganizationAdminAccount (not implemented)
            // PUT /assessments/{assessmentId}/disassociateFromAssessmentReport => DisassociateAssessmentReportEvidenceFolder (not implemented)
            // GET /assessments/{assessmentId}/reports/{assessmentReportId}/url => GetAssessmentReportUrl (not implemented)
            // GET /assessments/{assessmentId}/changelogs => GetChangeLogs (not implemented)
            // GET /delegations => GetDelegations (not implemented)
            // GET /assessments/{assessmentId}/controlSets/{controlSetId}/evidenceFolders/{evidenceFolderId}/evidence/{evidenceId} => GetEvidence (not implemented)
            // GET /assessments/{assessmentId}/controlSets/{controlSetId}/evidenceFolders/{evidenceFolderId}/evidence => GetEvidenceByEvidenceFolder (not implemented)
            // GET /evidenceFileUploadUrl => GetEvidenceFileUploadUrl (not implemented)
            // GET /assessments/{assessmentId}/controlSets/{controlSetId}/evidenceFolders/{evidenceFolderId} => GetEvidenceFolder (not implemented)
            // GET /assessments/{assessmentId}/evidenceFolders => GetEvidenceFoldersByAssessment (not implemented)
            // GET /assessments/{assessmentId}/evidenceFolders-by-assessment-control/{controlSetId}/{controlId} => GetEvidenceFoldersByAssessmentControl (not implemented)
            // GET /insights => GetInsights (not implemented)
            // GET /insights/assessments/{assessmentId} => GetInsightsByAssessment (not implemented)
            // GET /account/organizationAdminAccount => GetOrganizationAdminAccount (not implemented)
            // GET /services => GetServicesInScope (not implemented)
            // GET /settings/{attribute} => GetSettings (not implemented)
            // GET /insights/controls-by-assessment => ListAssessmentControlInsightsByControlDomain (not implemented)
            // GET /assessmentFrameworkShareRequests => ListAssessmentFrameworkShareRequests (not implemented)
            // GET /assessmentReports => ListAssessmentReports (not implemented)
            // GET /insights/control-domains => ListControlDomainInsights (not implemented)
            // GET /insights/control-domains-by-assessment => ListControlDomainInsightsByAssessment (not implemented)
            // GET /insights/controls => ListControlInsightsByControlDomain (not implemented)
            // GET /dataSourceKeywords => ListKeywordsForDataSource (not implemented)
            // GET /notifications => ListNotifications (not implemented)
            // GET /tags/{resourceArn} => ListTagsForResource (not implemented)
            // POST /account/registerOrganizationAdminAccount => RegisterOrganizationAdminAccount (not implemented)
            // POST /assessmentFrameworks/{frameworkId}/shareRequests => StartAssessmentFrameworkShare (not implemented)
            // POST /tags/{resourceArn} => TagResource (not implemented)
            // DELETE /tags/{resourceArn} => UntagResource (not implemented)
            // PUT /assessments/{assessmentId} => UpdateAssessment (not implemented)
            // PUT /assessments/{assessmentId}/controlSets/{controlSetId}/controls/{controlId} => UpdateAssessmentControl (not implemented)
            // PUT /assessments/{assessmentId}/controlSets/{controlSetId}/status => UpdateAssessmentControlSetStatus (not implemented)
            // PUT /assessmentFrameworks/{frameworkId} => UpdateAssessmentFramework (not implemented)
            // PUT /assessmentFrameworkShareRequests/{requestId} => UpdateAssessmentFrameworkShare (not implemented)
            // PUT /assessments/{assessmentId}/status => UpdateAssessmentStatus (not implemented)
            // PUT /controls/{controlId} => UpdateControl (not implemented)
            // PUT /settings => UpdateSettings (not implemented)
            // POST /assessmentReports/integrity => ValidateAssessmentReportIntegrity (not implemented)

            // 47 unimplemented operations above
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        let is_mutating =
            matches!(method, "POST" | "PUT" | "PATCH" | "DELETE") && response.status / 100 == 2;
        if is_mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_account_status(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let status = state.get_account_status().to_string();
        wire::serialize_get_account_status_response(&model::GetAccountStatusResponse {
            status: Some(status),
        })
    }

    async fn handle_register_account(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.register_account() {
            Ok(status) => {
                wire::serialize_register_account_response(&model::RegisterAccountResponse {
                    status: Some(status),
                })
            }
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_deregister_account(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.deregister_account() {
            Ok(status) => {
                wire::serialize_deregister_account_response(&model::DeregisterAccountResponse {
                    status: Some(status),
                })
            }
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_create_control(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_control_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let tags = input.tags.clone().unwrap_or_default();

        let now = chrono_now();
        let mut state = state.write().await;
        match state.create_control(&input.name, input.description.clone(), tags, now) {
            Ok(control) => wire::serialize_create_control_response(&model::CreateControlResponse {
                control: Some(control_to_model(&control)),
            }),
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_list_controls(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let controls: Vec<model::ControlMetadata> = state
            .list_controls()
            .into_iter()
            .map(|c| model::ControlMetadata {
                arn: Some(format!(
                    "arn:aws:auditmanager:us-east-1:123456789012:control/{}",
                    c.id
                )),
                id: Some(c.id.clone()),
                name: Some(c.name.clone()),
                created_at: Some(c.created_at),
                last_updated_at: Some(c.created_at),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_controls_response(&model::ListControlsResponse {
            control_metadata_list: Some(controls),
            ..Default::default()
        })
    }

    async fn handle_get_control(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_control(id) {
            Ok(control) => wire::serialize_get_control_response(&model::GetControlResponse {
                control: Some(control_to_model(control)),
            }),
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_delete_control(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_control(id) {
            Ok(()) => wire::serialize_delete_control_response(&model::DeleteControlResponse {}),
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_create_assessment_framework(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_assessment_framework_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let tags = input.tags.clone().unwrap_or_default();

        let now = chrono_now();
        let mut state = state.write().await;
        match state.create_assessment_framework(
            &input.name,
            input.description.clone(),
            input.compliance_type.clone(),
            tags,
            now,
        ) {
            Ok(framework) => wire::serialize_create_assessment_framework_response(
                &model::CreateAssessmentFrameworkResponse {
                    framework: Some(framework_to_model(&framework)),
                },
            ),
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_list_assessment_frameworks(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let frameworks: Vec<model::AssessmentFrameworkMetadata> = state
            .list_assessment_frameworks()
            .into_iter()
            .map(|f| model::AssessmentFrameworkMetadata {
                arn: Some(format!(
                    "arn:aws:auditmanager:us-east-1:123456789012:assessmentFramework/{}",
                    f.id
                )),
                id: Some(f.id.clone()),
                name: Some(f.name.clone()),
                compliance_type: f.compliance_type.clone(),
                r#type: Some(f.framework_type.clone()),
                created_at: Some(f.created_at),
                last_updated_at: Some(f.created_at),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_assessment_frameworks_response(
            &model::ListAssessmentFrameworksResponse {
                framework_metadata_list: Some(frameworks),
                ..Default::default()
            },
        )
    }

    async fn handle_get_assessment_framework(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_assessment_framework(id) {
            Ok(framework) => wire::serialize_get_assessment_framework_response(
                &model::GetAssessmentFrameworkResponse {
                    framework: Some(framework_to_model(framework)),
                },
            ),
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_delete_assessment_framework(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_assessment_framework(id) {
            Ok(()) => wire::serialize_delete_assessment_framework_response(
                &model::DeleteAssessmentFrameworkResponse {},
            ),
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_create_assessment(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_assessment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.framework_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'frameworkId'");
        }
        let tags = input.tags.clone().unwrap_or_default();

        let now = chrono_now();
        let mut state = state.write().await;
        match state.create_assessment(
            &input.name,
            input.description.clone(),
            &input.framework_id,
            tags,
            now,
        ) {
            Ok(assessment) => {
                wire::serialize_create_assessment_response(&model::CreateAssessmentResponse {
                    assessment: Some(assessment_to_model(&assessment)),
                })
            }
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_list_assessments(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let assessments: Vec<model::AssessmentMetadataItem> = state
            .list_assessments()
            .into_iter()
            .map(|a| model::AssessmentMetadataItem {
                id: Some(a.id.clone()),
                name: Some(a.name.clone()),
                status: Some(a.status.clone()),
                creation_time: Some(a.created_at),
                last_updated: Some(a.created_at),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_assessments_response(&model::ListAssessmentsResponse {
            assessment_metadata: Some(assessments),
            ..Default::default()
        })
    }

    async fn handle_get_assessment(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_assessment(id) {
            Ok(assessment) => {
                wire::serialize_get_assessment_response(&model::GetAssessmentResponse {
                    assessment: Some(assessment_to_model(assessment)),
                    ..Default::default()
                })
            }
            Err(e) => audit_manager_error_response(&e),
        }
    }

    async fn handle_delete_assessment(
        &self,
        state: &Arc<tokio::sync::RwLock<AuditManagerState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_assessment(id) {
            Ok(()) => {
                wire::serialize_delete_assessment_response(&model::DeleteAssessmentResponse {})
            }
            Err(e) => audit_manager_error_response(&e),
        }
    }
}

fn control_to_model(c: &crate::types::Control) -> model::Control {
    model::Control {
        arn: Some(format!(
            "arn:aws:auditmanager:us-east-1:123456789012:control/{}",
            c.id
        )),
        id: Some(c.id.clone()),
        name: Some(c.name.clone()),
        description: c.description.clone(),
        r#type: Some(c.control_type.clone()),
        created_at: Some(c.created_at),
        last_updated_at: Some(c.created_at),
        ..Default::default()
    }
}

fn framework_to_model(f: &crate::types::Framework) -> model::Framework {
    model::Framework {
        arn: Some(format!(
            "arn:aws:auditmanager:us-east-1:123456789012:assessmentFramework/{}",
            f.id
        )),
        id: Some(f.id.clone()),
        name: Some(f.name.clone()),
        description: f.description.clone(),
        compliance_type: f.compliance_type.clone(),
        r#type: Some(f.framework_type.clone()),
        created_at: Some(f.created_at),
        last_updated_at: Some(f.created_at),
        ..Default::default()
    }
}

fn assessment_to_model(a: &crate::types::Assessment) -> model::Assessment {
    model::Assessment {
        metadata: Some(model::AssessmentMetadata {
            id: Some(a.id.clone()),
            name: Some(a.name.clone()),
            description: a.description.clone(),
            status: Some(a.status.clone()),
            creation_time: Some(a.created_at),
            last_updated: Some(a.created_at),
            ..Default::default()
        }),
        framework: Some(model::AssessmentFramework {
            id: Some(a.framework_id.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn chrono_now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

fn audit_manager_error_response(err: &AuditManagerError) -> MockResponse {
    let (status, error_type) = match err {
        AuditManagerError::ControlNotFound { .. } => (404, "ResourceNotFoundException"),
        AuditManagerError::FrameworkNotFound { .. } => (404, "ResourceNotFoundException"),
        AuditManagerError::AssessmentNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
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

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
