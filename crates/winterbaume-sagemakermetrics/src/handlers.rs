use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{SageMakerMetricsError, SageMakerMetricsState};
use crate::types::*;
use crate::views::SageMakerMetricsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SageMakerMetricsService {
    pub(crate) state: Arc<BackendState<SageMakerMetricsState>>,
    pub(crate) notifier: StateChangeNotifier<SageMakerMetricsStateView>,
}

impl SageMakerMetricsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SageMakerMetricsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SageMakerMetricsService {
    fn service_name(&self) -> &str {
        "sagemaker-metrics"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://metrics\.sagemaker\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SageMakerMetricsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let response = match (method, path.as_str()) {
            // PUT /BatchPutMetrics
            ("PUT", "/BatchPutMetrics") => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_batch_put_metrics(&state, &body).await
            }
            // POST /BatchGetMetrics
            ("POST", "/BatchGetMetrics") => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_batch_get_metrics(&state, &body).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_batch_put_metrics(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerMetricsState>>,
        body: &Value,
    ) -> MockResponse {
        let trial_component_name = match body.get("TrialComponentName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ValidationException", "Missing 'TrialComponentName'");
            }
        };

        let metric_data: Vec<RawMetricData> = match body.get("MetricData") {
            Some(v) => match serde_json::from_value(v.clone()) {
                Ok(d) => d,
                Err(_) => {
                    return rest_json_error(
                        400,
                        "ValidationException",
                        "Invalid 'MetricData' format",
                    );
                }
            },
            None => return rest_json_error(400, "ValidationException", "Missing 'MetricData'"),
        };

        if metric_data.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "MetricData must contain at least one entry",
            );
        }

        let mut state = state.write().await;
        match state.batch_put_metrics(trial_component_name, &metric_data) {
            Ok(errors) => {
                let error_entries: Option<Vec<wire::BatchPutMetricsError>> = if errors.is_empty() {
                    Some(vec![])
                } else {
                    Some(
                        errors
                            .iter()
                            .map(|idx| wire::BatchPutMetricsError {
                                code: Some("METRIC_LIMIT_EXCEEDED".to_string()),
                                metric_index: Some(*idx as i32),
                            })
                            .collect(),
                    )
                };
                wire::serialize_batch_put_metrics_response(&wire::BatchPutMetricsResponse {
                    errors: error_entries,
                })
            }
            Err(e) => sagemaker_metrics_error_response(&e),
        }
    }

    async fn handle_batch_get_metrics(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerMetricsState>>,
        body: &Value,
    ) -> MockResponse {
        let queries: Vec<MetricQuery> = match body.get("MetricQueries") {
            Some(v) => match serde_json::from_value(v.clone()) {
                Ok(q) => q,
                Err(_) => {
                    return rest_json_error(
                        400,
                        "ValidationException",
                        "Invalid 'MetricQueries' format",
                    );
                }
            },
            None => return rest_json_error(400, "ValidationException", "Missing 'MetricQueries'"),
        };

        let state = state.read().await;
        let results = state.batch_get_metrics(&queries);

        let wire_results: Vec<wire::MetricQueryResult> = results
            .into_iter()
            .map(|r| metric_query_result_to_wire(&r))
            .collect();

        wire::serialize_batch_get_metrics_response(&wire::BatchGetMetricsResponse {
            metric_query_results: Some(wire_results),
        })
    }
}

fn metric_query_result_to_wire(r: &crate::types::MetricQueryResult) -> wire::MetricQueryResult {
    wire::MetricQueryResult {
        status: Some(r.status.clone()),
        x_axis_values: Some(r.x_axis_values.iter().map(|v| *v as i64).collect()),
        metric_values: Some(r.metric_values.clone()),
        message: r.message.clone(),
    }
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

fn sagemaker_metrics_error_response(err: &SageMakerMetricsError) -> MockResponse {
    match err {
        SageMakerMetricsError::EmptyTrialComponentName => {
            rest_json_error(400, "ValidationException", &err.to_string())
        }
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
