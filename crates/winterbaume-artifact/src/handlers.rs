use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string, rest_json_error,
};

use crate::state::{ArtifactError, ArtifactState};
use crate::types::{CustomerAgreement, Report};
use crate::views::ArtifactStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ArtifactService {
    pub(crate) state: Arc<BackendState<ArtifactState>>,
    pub(crate) notifier: StateChangeNotifier<ArtifactStateView>,
}

impl ArtifactService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ArtifactService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ArtifactService {
    fn service_name(&self) -> &str {
        "artifact"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://artifact\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ArtifactService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let response = match (method.as_str(), path.as_str()) {
            ("GET", "/v1/account-settings/get") => self.handle_get_account_settings(&state).await,
            ("PUT", "/v1/account-settings/put") => {
                self.handle_put_account_settings(&state, &body).await
            }
            ("GET", "/v1/report/list") => self.handle_list_reports(&state).await,
            ("GET", "/v1/report/listVersions") => {
                self.handle_list_report_versions(&state, &request.uri).await
            }
            ("GET", "/v1/report/get") => self.handle_get_report(&state, &request.uri).await,
            ("GET", "/v1/report/getMetadata") => {
                self.handle_get_report_metadata(&state, &request.uri).await
            }
            ("GET", "/v1/report/getTermForReport") => {
                self.handle_get_term_for_report(&state, &request.uri).await
            }
            ("GET", "/v1/customer-agreement/list") => {
                self.handle_list_customer_agreements(&state).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "PUT" | "POST" | "PATCH" | "DELETE")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let response = wire::GetAccountSettingsResponse {
            account_settings: Some(wire::AccountSettings {
                notification_subscription_status: Some(
                    state
                        .account_settings
                        .notification_subscription_status
                        .clone()
                        .unwrap_or_else(|| "NOT_SUBSCRIBED".to_string()),
                ),
            }),
        };
        wire::serialize_get_account_settings_response(&response)
    }

    async fn handle_put_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
        body: &Value,
    ) -> MockResponse {
        let status = body
            .get("notificationSubscriptionStatus")
            .and_then(|v| v.as_str())
            .map(String::from);
        let mut state = state.write().await;
        state.put_account_settings(status.clone());
        let response = wire::PutAccountSettingsResponse {
            account_settings: Some(wire::AccountSettings {
                notification_subscription_status: state
                    .account_settings
                    .notification_subscription_status
                    .clone()
                    .or_else(|| Some("NOT_SUBSCRIBED".to_string())),
            }),
        };
        wire::serialize_put_account_settings_response(&response)
    }

    async fn handle_list_reports(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let reports: Vec<wire::ReportSummary> = state
            .list_reports()
            .iter()
            .map(|r| report_to_summary(r))
            .collect();
        let response = wire::ListReportsResponse {
            reports: if reports.is_empty() {
                None
            } else {
                Some(reports)
            },
            next_token: None,
        };
        wire::serialize_list_reports_response(&response)
    }

    async fn handle_list_report_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let report_id = match qs.get("reportId") {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => return rest_json_error(400, "ValidationException", "reportId is required"),
        };
        let state = state.read().await;
        let reports: Vec<wire::ReportSummary> = state
            .list_report_versions(&report_id)
            .iter()
            .map(|r| report_to_summary(r))
            .collect();
        let response = wire::ListReportVersionsResponse {
            reports: if reports.is_empty() {
                None
            } else {
                Some(reports)
            },
            next_token: None,
        };
        wire::serialize_list_report_versions_response(&response)
    }

    async fn handle_get_report(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let report_id = match qs.get("reportId") {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => return rest_json_error(400, "ValidationException", "reportId is required"),
        };
        let token = match qs.get("termToken") {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => return rest_json_error(400, "ValidationException", "termToken is required"),
        };
        let state = state.read().await;
        match state.get_report(&report_id, &token) {
            Ok(r) => {
                let response = wire::GetReportResponse {
                    document_presigned_url: Some(r.document_url.clone()),
                };
                wire::serialize_get_report_response(&response)
            }
            Err(e) => artifact_error_response(&e),
        }
    }

    async fn handle_get_report_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let report_id = match qs.get("reportId") {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => return rest_json_error(400, "ValidationException", "reportId is required"),
        };
        let state = state.read().await;
        match state.get_report_metadata(&report_id) {
            Ok(r) => {
                let response = wire::GetReportMetadataResponse {
                    report_details: Some(report_to_detail(r)),
                };
                wire::serialize_get_report_metadata_response(&response)
            }
            Err(e) => artifact_error_response(&e),
        }
    }

    async fn handle_get_term_for_report(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let report_id = match qs.get("reportId") {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => return rest_json_error(400, "ValidationException", "reportId is required"),
        };
        let mut state = state.write().await;
        match state.issue_term_token(&report_id) {
            Ok((url, token)) => {
                let response = wire::GetTermForReportResponse {
                    document_presigned_url: Some(url),
                    term_token: Some(token),
                };
                wire::serialize_get_term_for_report_response(&response)
            }
            Err(e) => artifact_error_response(&e),
        }
    }

    async fn handle_list_customer_agreements(
        &self,
        state: &Arc<tokio::sync::RwLock<ArtifactState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let agreements: Vec<wire::CustomerAgreementSummary> = state
            .list_customer_agreements()
            .iter()
            .map(|a| agreement_to_summary(a))
            .collect();
        let response = wire::ListCustomerAgreementsResponse {
            customer_agreements: Some(agreements),
            next_token: None,
        };
        wire::serialize_list_customer_agreements_response(&response)
    }
}

fn report_to_summary(r: &Report) -> wire::ReportSummary {
    wire::ReportSummary {
        id: Some(r.id.clone()),
        version: Some(r.version),
        name: Some(r.name.clone()),
        description: r.description.clone(),
        category: r.category.clone(),
        series: r.series.clone(),
        state: Some(r.state.clone()),
        arn: Some(r.arn.clone()),
        ..Default::default()
    }
}

fn report_to_detail(r: &Report) -> wire::ReportDetail {
    wire::ReportDetail {
        id: Some(r.id.clone()),
        version: Some(r.version),
        name: Some(r.name.clone()),
        description: r.description.clone(),
        category: r.category.clone(),
        series: r.series.clone(),
        state: Some(r.state.clone()),
        arn: Some(r.arn.clone()),
        ..Default::default()
    }
}

fn agreement_to_summary(a: &CustomerAgreement) -> wire::CustomerAgreementSummary {
    wire::CustomerAgreementSummary {
        id: Some(a.id.clone()),
        name: Some(a.name.clone()),
        arn: Some(a.arn.clone()),
        agreement_arn: Some(a.agreement_arn.clone()),
        aws_account_id: Some(a.aws_account_id.clone()),
        organization_arn: a.organization_arn.clone(),
        effective_start: a
            .effective_start
            .map(|t| chrono::DateTime::from_timestamp(t, 0).unwrap().to_rfc3339()),
        effective_end: a
            .effective_end
            .map(|t| chrono::DateTime::from_timestamp(t, 0).unwrap().to_rfc3339()),
        state: Some(a.state.clone()),
        ..Default::default()
    }
}

fn artifact_error_response(err: &ArtifactError) -> MockResponse {
    let (status, error_type) = match err {
        ArtifactError::ReportNotFound { .. } => (404, "ResourceNotFoundException"),
        ArtifactError::InvalidTermToken { .. } => (400, "ValidationException"),
        ArtifactError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
