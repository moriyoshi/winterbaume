use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{PiError, PiState, ReportRecord};
use crate::views::PiStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct PiService {
    pub(crate) state: Arc<BackendState<PiState>>,
    pub(crate) notifier: StateChangeNotifier<PiStateView>,
}

impl PiService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PiService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PiService {
    fn service_name(&self) -> &str {
        "pi"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://pi\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<PiState>>;

const MUTATING_ACTIONS: &[&str] = &[
    "CreatePerformanceAnalysisReport",
    "DeletePerformanceAnalysisReport",
    "TagResource",
    "UntagResource",
];

impl PiService {
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
            None => return aws_json_error(400, "MissingAction", "Missing X-Amz-Target"),
        };

        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return aws_json_error(400, "SerializationException", "Invalid JSON body");
        }
        let body_owned: Vec<u8> = if request.body.is_empty() {
            b"{}".to_vec()
        } else {
            request.body.to_vec()
        };
        let body_bytes: &[u8] = &body_owned;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreatePerformanceAnalysisReport" => {
                self.handle_create_report(&state, body_bytes).await
            }
            "DeletePerformanceAnalysisReport" => {
                self.handle_delete_report(&state, body_bytes).await
            }
            "GetPerformanceAnalysisReport" => self.handle_get_report(&state, body_bytes).await,
            "ListPerformanceAnalysisReports" => self.handle_list_reports(&state, body_bytes).await,
            "DescribeDimensionKeys" => self.handle_describe_dimension_keys(body_bytes).await,
            "GetDimensionKeyDetails" => self.handle_get_dimension_key_details(body_bytes).await,
            "GetResourceMetadata" => self.handle_get_resource_metadata(body_bytes).await,
            "GetResourceMetrics" => self.handle_get_resource_metrics(body_bytes).await,
            "ListAvailableResourceDimensions" => {
                self.handle_list_available_dimensions(body_bytes).await
            }
            "ListAvailableResourceMetrics" => self.handle_list_available_metrics(body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            other => aws_json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_report(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_create_performance_analysis_report_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return aws_json_error(400, "ValidationException", "Identifier is required");
        }
        let identifier = input.identifier.clone();
        if input.service_type.is_empty() {
            return aws_json_error(400, "ValidationException", "ServiceType is required");
        }
        let service_type = input.service_type.clone();
        let start_time = input.start_time;
        let end_time = input.end_time;
        let id = format!("report-{}", uuid::Uuid::new_v4().simple());
        let tags: HashMap<String, String> = input
            .tags
            .as_deref()
            .map(|arr| {
                arr.iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let report = ReportRecord {
            analysis_report_id: id.clone(),
            identifier,
            service_type,
            start_time,
            end_time,
            status: "SUCCEEDED".to_string(),
            create_time: chrono::Utc::now().timestamp() as f64,
            tags,
        };
        let mut state = state.write().await;
        state.create_report(report);
        wire::serialize_create_performance_analysis_report_response(
            &wire::CreatePerformanceAnalysisReportResponse {
                analysis_report_id: Some(id),
            },
        )
    }

    async fn handle_delete_report(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_performance_analysis_report_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.analysis_report_id.is_empty() {
            return aws_json_error(400, "ValidationException", "AnalysisReportId is required");
        }
        let id = input.analysis_report_id.clone();
        let mut state = state.write().await;
        match state.delete_report(&id) {
            Ok(()) => wire::serialize_delete_performance_analysis_report_response(
                &wire::DeletePerformanceAnalysisReportResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_get_report(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_performance_analysis_report_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.analysis_report_id.is_empty() {
            return aws_json_error(400, "ValidationException", "AnalysisReportId is required");
        }
        let id = input.analysis_report_id.clone();
        let state = state.read().await;
        match state.get_report(&id) {
            Ok(r) => {
                let report = wire::AnalysisReport {
                    analysis_report_id: Some(r.analysis_report_id.clone()),
                    create_time: Some(r.create_time),
                    end_time: Some(r.end_time),
                    identifier: Some(r.identifier.clone()),
                    insights: None,
                    service_type: Some(r.service_type.clone()),
                    start_time: Some(r.start_time),
                    status: Some(r.status.clone()),
                };
                wire::serialize_get_performance_analysis_report_response(
                    &wire::GetPerformanceAnalysisReportResponse {
                        analysis_report: Some(report),
                    },
                )
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_reports(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_performance_analysis_reports_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        let identifier = input.identifier.as_str();
        let service_type = input.service_type.as_str();
        let state = state.read().await;
        let summaries: Vec<wire::AnalysisReportSummary> = state
            .list_reports(identifier, service_type)
            .into_iter()
            .map(|r| wire::AnalysisReportSummary {
                analysis_report_id: Some(r.analysis_report_id.clone()),
                create_time: Some(r.create_time),
                end_time: Some(r.end_time),
                start_time: Some(r.start_time),
                status: Some(r.status.clone()),
                tags: if r.tags.is_empty() {
                    None
                } else {
                    Some(tags_to_wire(&r.tags))
                },
            })
            .collect();
        wire::serialize_list_performance_analysis_reports_response(
            &wire::ListPerformanceAnalysisReportsResponse {
                analysis_reports: Some(summaries),
                next_token: None,
            },
        )
    }

    // STUB[no-telemetry]: DescribeDimensionKeys requires real database load metrics; mock has no such data.
    async fn handle_describe_dimension_keys(&self, _body: &[u8]) -> MockResponse {
        wire::serialize_describe_dimension_keys_response(&wire::DescribeDimensionKeysResponse {
            aligned_end_time: None,
            aligned_start_time: None,
            keys: Some(vec![]),
            next_token: None,
            partition_keys: None,
        })
    }

    // STUB[no-telemetry]: GetDimensionKeyDetails requires real database dimension data; mock has no such data.
    async fn handle_get_dimension_key_details(&self, _body: &[u8]) -> MockResponse {
        wire::serialize_get_dimension_key_details_response(&wire::GetDimensionKeyDetailsResponse {
            dimensions: Some(vec![]),
        })
    }

    // STUB[no-telemetry]: GetResourceMetadata returns live PI feature availability from real infrastructure.
    async fn handle_get_resource_metadata(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_resource_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        let identifier = input.identifier.clone();
        wire::serialize_get_resource_metadata_response(&wire::GetResourceMetadataResponse {
            features: None,
            identifier: Some(identifier),
        })
    }

    // STUB[no-telemetry]: GetResourceMetrics returns real time-series database metrics; mock has no such data.
    async fn handle_get_resource_metrics(&self, body: &[u8]) -> MockResponse {
        // Parse raw to detect "field absent" so we can preserve the previous
        // None-vs-Some mapping for AlignedStartTime/AlignedEndTime when start
        // and end times are absent from the request.
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_get_resource_metrics_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        let identifier = input.identifier.clone();
        let start = if raw.get("StartTime").is_some() {
            Some(input.start_time)
        } else {
            None
        };
        let end = if raw.get("EndTime").is_some() {
            Some(input.end_time)
        } else {
            None
        };
        wire::serialize_get_resource_metrics_response(&wire::GetResourceMetricsResponse {
            aligned_end_time: end,
            aligned_start_time: start,
            identifier: Some(identifier),
            metric_list: Some(vec![]),
            next_token: None,
        })
    }

    // STUB[no-telemetry]: ListAvailableResourceDimensions reflects live engine capabilities; mock returns empty.
    async fn handle_list_available_dimensions(&self, _body: &[u8]) -> MockResponse {
        wire::serialize_list_available_resource_dimensions_response(
            &wire::ListAvailableResourceDimensionsResponse {
                metric_dimensions: Some(vec![]),
                next_token: None,
            },
        )
    }

    // STUB[no-telemetry]: ListAvailableResourceMetrics reflects live engine capabilities; mock returns empty.
    async fn handle_list_available_metrics(&self, _body: &[u8]) -> MockResponse {
        wire::serialize_list_available_resource_metrics_response(
            &wire::ListAvailableResourceMetricsResponse {
                metrics: Some(vec![]),
                next_token: None,
            },
        )
    }

    async fn handle_list_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return aws_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n.clone();
        let state = state.read().await;
        let tags_map = state.list_tags(&arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags_map.is_empty() {
                None
            } else {
                Some(tags_to_wire(&tags_map))
            },
        })
    }

    async fn handle_tag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return aws_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n.clone();
        let tags: HashMap<String, String> = input
            .tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        if tags.is_empty() {
            return aws_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(&arn, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return aws_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n.clone();
        let keys = input.tag_keys.clone();
        let mut state = state.write().await;
        state.untag_resource(&arn, &keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

fn tags_to_wire(tags: &HashMap<String, String>) -> Vec<wire::Tag> {
    let mut v: Vec<wire::Tag> = tags
        .iter()
        .map(|(k, val)| wire::Tag {
            key: k.clone(),
            value: val.clone(),
        })
        .collect();
    v.sort_by(|a, b| a.key.cmp(&b.key));
    v
}

fn aws_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &PiError) -> MockResponse {
    let (status, error_type) = match err {
        PiError::ReportNotFound { .. } => (404, "NotAuthorizedException"),
        PiError::Validation { .. } => (400, "InvalidArgumentException"),
    };
    aws_json_error(status, error_type, &err.to_string())
}
