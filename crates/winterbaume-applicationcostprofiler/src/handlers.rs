use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{ApplicationCostProfilerError, ApplicationCostProfilerState};
use crate::types::{ReportDefinition, S3Location};
use crate::views::ApplicationCostProfilerStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ApplicationCostProfilerService {
    pub(crate) state: Arc<BackendState<ApplicationCostProfilerState>>,
    pub(crate) notifier: StateChangeNotifier<ApplicationCostProfilerStateView>,
}

impl ApplicationCostProfilerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApplicationCostProfilerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApplicationCostProfilerService {
    fn service_name(&self) -> &str {
        "application-cost-profiler"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://application-cost-profiler\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ApplicationCostProfilerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = parse_query_string(raw_query);
        let method = request.method.as_str().to_uppercase();

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(String::as_str).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["reportDefinition"]) => {
                self.handle_put_report_definition(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["reportDefinition"]) => {
                self.handle_list_report_definitions(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["reportDefinition", id]) => {
                let labels: &[(&str, &str)] = &[("reportId", id)];
                self.handle_get_report_definition(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["reportDefinition", id]) => {
                let labels: &[(&str, &str)] = &[("reportId", id)];
                self.handle_update_report_definition(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["reportDefinition", id]) => {
                let labels: &[(&str, &str)] = &[("reportId", id)];
                self.handle_delete_report_definition(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["importApplicationUsage"]) => {
                self.handle_import_application_usage(&state, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "PUT" | "DELETE" | "PATCH")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put_report_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationCostProfilerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_report_definition_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.report_id.is_empty() {
            return rest_json_error(400, "ValidationException", "reportId is required");
        }
        if input.report_description.is_empty() {
            return rest_json_error(400, "ValidationException", "reportDescription is required");
        }
        if input.report_frequency.is_empty() {
            return rest_json_error(400, "ValidationException", "reportFrequency is required");
        }
        if input.format.is_empty() {
            return rest_json_error(400, "ValidationException", "format is required");
        }
        if input.destination_s3_location.bucket.is_empty()
            || input.destination_s3_location.prefix.is_empty()
        {
            return rest_json_error(
                400,
                "ValidationException",
                "destinationS3Location is required",
            );
        }

        let now = chrono::Utc::now().timestamp();
        let report = ReportDefinition {
            report_id: input.report_id.clone(),
            report_description: input.report_description.clone(),
            report_frequency: input.report_frequency.clone(),
            format: input.format.clone(),
            destination_s3_location: S3Location {
                bucket: input.destination_s3_location.bucket.clone(),
                prefix: input.destination_s3_location.prefix.clone(),
            },
            created_at: now,
            last_updated_at: now,
        };

        let mut state = state.write().await;
        match state.put_report(report) {
            Ok(()) => {
                let response = wire::PutReportDefinitionResult {
                    report_id: Some(input.report_id),
                };
                wire::serialize_put_report_definition_response(&response)
            }
            Err(e) => acp_error_response(&e),
        }
    }

    async fn handle_get_report_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationCostProfilerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_report_definition_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_report(&input.report_id) {
            Ok(r) => {
                let response = wire::GetReportDefinitionResult {
                    report_id: Some(r.report_id.clone()),
                    report_description: Some(r.report_description.clone()),
                    report_frequency: Some(r.report_frequency.clone()),
                    format: Some(r.format.clone()),
                    destination_s3_location: Some(s3_location_to_wire(&r.destination_s3_location)),
                    created_at: Some(r.created_at as f64),
                    last_updated: Some(r.last_updated_at as f64),
                };
                wire::serialize_get_report_definition_response(&response)
            }
            Err(e) => acp_error_response(&e),
        }
    }

    async fn handle_update_report_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationCostProfilerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_report_definition_request(request, labels, query)
        {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.report_description.is_empty() {
            return rest_json_error(400, "ValidationException", "reportDescription is required");
        }
        if input.report_frequency.is_empty() {
            return rest_json_error(400, "ValidationException", "reportFrequency is required");
        }
        if input.format.is_empty() {
            return rest_json_error(400, "ValidationException", "format is required");
        }
        if input.destination_s3_location.bucket.is_empty()
            || input.destination_s3_location.prefix.is_empty()
        {
            return rest_json_error(
                400,
                "ValidationException",
                "destinationS3Location is required",
            );
        }

        let dest = S3Location {
            bucket: input.destination_s3_location.bucket.clone(),
            prefix: input.destination_s3_location.prefix.clone(),
        };

        let mut state = state.write().await;
        match state.update_report(
            &input.report_id,
            input.report_description,
            input.report_frequency,
            input.format,
            dest,
        ) {
            Ok(()) => {
                let response = wire::UpdateReportDefinitionResult {
                    report_id: Some(input.report_id),
                };
                wire::serialize_update_report_definition_response(&response)
            }
            Err(e) => acp_error_response(&e),
        }
    }

    async fn handle_delete_report_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationCostProfilerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_report_definition_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_report(&input.report_id) {
            Ok(()) => {
                let response = wire::DeleteReportDefinitionResult {
                    report_id: Some(input.report_id),
                };
                wire::serialize_delete_report_definition_response(&response)
            }
            Err(e) => acp_error_response(&e),
        }
    }

    async fn handle_list_report_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationCostProfilerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_report_definitions_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let definitions: Vec<wire::ReportDefinition> = state
            .list_reports()
            .iter()
            .map(|r| wire::ReportDefinition {
                report_id: Some(r.report_id.clone()),
                report_description: Some(r.report_description.clone()),
                report_frequency: Some(r.report_frequency.clone()),
                format: Some(r.format.clone()),
                destination_s3_location: Some(s3_location_to_wire(&r.destination_s3_location)),
                created_at: Some(r.created_at as f64),
                last_updated_at: Some(r.last_updated_at as f64),
            })
            .collect();
        let response = wire::ListReportDefinitionsResult {
            report_definitions: if definitions.is_empty() {
                None
            } else {
                Some(definitions)
            },
            next_token: None,
        };
        wire::serialize_list_report_definitions_response(&response)
    }

    async fn handle_import_application_usage(
        &self,
        state: &Arc<tokio::sync::RwLock<ApplicationCostProfilerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_import_application_usage_request(request, labels, query)
        {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.source_s3_location.bucket.is_empty() {
            return rest_json_error(400, "ValidationException", "sourceS3Location is required");
        }
        if input.source_s3_location.key.is_empty() {
            return rest_json_error(400, "ValidationException", "key is required");
        }

        let mut state = state.write().await;
        let job = state.import_application_usage(
            &input.source_s3_location.bucket,
            &input.source_s3_location.key,
            input.source_s3_location.region,
        );
        let response = wire::ImportApplicationUsageResult {
            import_id: Some(job.import_id.clone()),
        };
        wire::serialize_import_application_usage_response(&response)
    }
}

fn s3_location_to_wire(s: &S3Location) -> wire::S3Location {
    wire::S3Location {
        bucket: s.bucket.clone(),
        prefix: s.prefix.clone(),
    }
}

fn acp_error_response(err: &ApplicationCostProfilerError) -> MockResponse {
    let (status, error_type) = match err {
        ApplicationCostProfilerError::ReportNotFound { .. } => (404, "ResourceNotFoundException"),
        ApplicationCostProfilerError::ReportAlreadyExists { .. } => (409, "ConflictException"),
        ApplicationCostProfilerError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
