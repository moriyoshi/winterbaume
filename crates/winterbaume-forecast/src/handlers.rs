use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{ForecastError, ForecastState};
use crate::views::ForecastStateView;
use crate::wire;

pub struct ForecastService {
    pub(crate) state: Arc<BackendState<ForecastState>>,
    pub(crate) notifier: StateChangeNotifier<ForecastStateView>,
}

impl ForecastService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ForecastService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ForecastService {
    fn service_name(&self) -> &str {
        "forecast"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://forecast\..*\.amazonaws\.com",
            r"https?://forecast\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ForecastService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "AmazonForecast.CreateDatasetGroup"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        use winterbaume_core::StatefulService;
        let response = match action.as_str() {
            "CreateDatasetGroup" => {
                self.handle_create_dataset_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeDatasetGroup" => self.handle_describe_dataset_group(&state, body_bytes).await,
            "DeleteDatasetGroup" => self.handle_delete_dataset_group(&state, body_bytes).await,
            "ListDatasetGroups" => self.handle_list_dataset_groups(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "CreateAutoPredictor" => json_error_response(
                501,
                "NotImplementedError",
                "CreateAutoPredictor is not yet implemented in winterbaume-forecast",
            ),
            "CreateDataset" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDataset is not yet implemented in winterbaume-forecast",
            ),
            "CreateDatasetImportJob" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDatasetImportJob is not yet implemented in winterbaume-forecast",
            ),
            "CreateExplainability" => json_error_response(
                501,
                "NotImplementedError",
                "CreateExplainability is not yet implemented in winterbaume-forecast",
            ),
            "CreateExplainabilityExport" => json_error_response(
                501,
                "NotImplementedError",
                "CreateExplainabilityExport is not yet implemented in winterbaume-forecast",
            ),
            "CreateForecast" => json_error_response(
                501,
                "NotImplementedError",
                "CreateForecast is not yet implemented in winterbaume-forecast",
            ),
            "CreateForecastExportJob" => json_error_response(
                501,
                "NotImplementedError",
                "CreateForecastExportJob is not yet implemented in winterbaume-forecast",
            ),
            "CreateMonitor" => json_error_response(
                501,
                "NotImplementedError",
                "CreateMonitor is not yet implemented in winterbaume-forecast",
            ),
            "CreatePredictor" => json_error_response(
                501,
                "NotImplementedError",
                "CreatePredictor is not yet implemented in winterbaume-forecast",
            ),
            "CreatePredictorBacktestExportJob" => json_error_response(
                501,
                "NotImplementedError",
                "CreatePredictorBacktestExportJob is not yet implemented in winterbaume-forecast",
            ),
            "CreateWhatIfAnalysis" => json_error_response(
                501,
                "NotImplementedError",
                "CreateWhatIfAnalysis is not yet implemented in winterbaume-forecast",
            ),
            "CreateWhatIfForecast" => json_error_response(
                501,
                "NotImplementedError",
                "CreateWhatIfForecast is not yet implemented in winterbaume-forecast",
            ),
            "CreateWhatIfForecastExport" => json_error_response(
                501,
                "NotImplementedError",
                "CreateWhatIfForecastExport is not yet implemented in winterbaume-forecast",
            ),
            "DeleteDataset" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDataset is not yet implemented in winterbaume-forecast",
            ),
            "DeleteDatasetImportJob" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDatasetImportJob is not yet implemented in winterbaume-forecast",
            ),
            "DeleteExplainability" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteExplainability is not yet implemented in winterbaume-forecast",
            ),
            "DeleteExplainabilityExport" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteExplainabilityExport is not yet implemented in winterbaume-forecast",
            ),
            "DeleteForecast" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteForecast is not yet implemented in winterbaume-forecast",
            ),
            "DeleteForecastExportJob" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteForecastExportJob is not yet implemented in winterbaume-forecast",
            ),
            "DeleteMonitor" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteMonitor is not yet implemented in winterbaume-forecast",
            ),
            "DeletePredictor" => json_error_response(
                501,
                "NotImplementedError",
                "DeletePredictor is not yet implemented in winterbaume-forecast",
            ),
            "DeletePredictorBacktestExportJob" => json_error_response(
                501,
                "NotImplementedError",
                "DeletePredictorBacktestExportJob is not yet implemented in winterbaume-forecast",
            ),
            "DeleteResourceTree" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteResourceTree is not yet implemented in winterbaume-forecast",
            ),
            "DeleteWhatIfAnalysis" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteWhatIfAnalysis is not yet implemented in winterbaume-forecast",
            ),
            "DeleteWhatIfForecast" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteWhatIfForecast is not yet implemented in winterbaume-forecast",
            ),
            "DeleteWhatIfForecastExport" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteWhatIfForecastExport is not yet implemented in winterbaume-forecast",
            ),
            "DescribeAutoPredictor" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAutoPredictor is not yet implemented in winterbaume-forecast",
            ),
            "DescribeDataset" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDataset is not yet implemented in winterbaume-forecast",
            ),
            "DescribeDatasetImportJob" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDatasetImportJob is not yet implemented in winterbaume-forecast",
            ),
            "DescribeExplainability" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeExplainability is not yet implemented in winterbaume-forecast",
            ),
            "DescribeExplainabilityExport" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeExplainabilityExport is not yet implemented in winterbaume-forecast",
            ),
            "DescribeForecast" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeForecast is not yet implemented in winterbaume-forecast",
            ),
            "DescribeForecastExportJob" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeForecastExportJob is not yet implemented in winterbaume-forecast",
            ),
            "DescribeMonitor" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeMonitor is not yet implemented in winterbaume-forecast",
            ),
            "DescribePredictor" => json_error_response(
                501,
                "NotImplementedError",
                "DescribePredictor is not yet implemented in winterbaume-forecast",
            ),
            "DescribePredictorBacktestExportJob" => json_error_response(
                501,
                "NotImplementedError",
                "DescribePredictorBacktestExportJob is not yet implemented in winterbaume-forecast",
            ),
            "DescribeWhatIfAnalysis" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeWhatIfAnalysis is not yet implemented in winterbaume-forecast",
            ),
            "DescribeWhatIfForecast" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeWhatIfForecast is not yet implemented in winterbaume-forecast",
            ),
            "DescribeWhatIfForecastExport" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeWhatIfForecastExport is not yet implemented in winterbaume-forecast",
            ),
            "GetAccuracyMetrics" => json_error_response(
                501,
                "NotImplementedError",
                "GetAccuracyMetrics is not yet implemented in winterbaume-forecast",
            ),
            "ListDatasetImportJobs" => json_error_response(
                501,
                "NotImplementedError",
                "ListDatasetImportJobs is not yet implemented in winterbaume-forecast",
            ),
            "ListDatasets" => json_error_response(
                501,
                "NotImplementedError",
                "ListDatasets is not yet implemented in winterbaume-forecast",
            ),
            "ListExplainabilities" => json_error_response(
                501,
                "NotImplementedError",
                "ListExplainabilities is not yet implemented in winterbaume-forecast",
            ),
            "ListExplainabilityExports" => json_error_response(
                501,
                "NotImplementedError",
                "ListExplainabilityExports is not yet implemented in winterbaume-forecast",
            ),
            "ListForecastExportJobs" => json_error_response(
                501,
                "NotImplementedError",
                "ListForecastExportJobs is not yet implemented in winterbaume-forecast",
            ),
            "ListForecasts" => json_error_response(
                501,
                "NotImplementedError",
                "ListForecasts is not yet implemented in winterbaume-forecast",
            ),
            "ListMonitorEvaluations" => json_error_response(
                501,
                "NotImplementedError",
                "ListMonitorEvaluations is not yet implemented in winterbaume-forecast",
            ),
            "ListMonitors" => json_error_response(
                501,
                "NotImplementedError",
                "ListMonitors is not yet implemented in winterbaume-forecast",
            ),
            "ListPredictorBacktestExportJobs" => json_error_response(
                501,
                "NotImplementedError",
                "ListPredictorBacktestExportJobs is not yet implemented in winterbaume-forecast",
            ),
            "ListPredictors" => json_error_response(
                501,
                "NotImplementedError",
                "ListPredictors is not yet implemented in winterbaume-forecast",
            ),
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-forecast",
            ),
            "ListWhatIfAnalyses" => json_error_response(
                501,
                "NotImplementedError",
                "ListWhatIfAnalyses is not yet implemented in winterbaume-forecast",
            ),
            "ListWhatIfForecastExports" => json_error_response(
                501,
                "NotImplementedError",
                "ListWhatIfForecastExports is not yet implemented in winterbaume-forecast",
            ),
            "ListWhatIfForecasts" => json_error_response(
                501,
                "NotImplementedError",
                "ListWhatIfForecasts is not yet implemented in winterbaume-forecast",
            ),
            "ResumeResource" => json_error_response(
                501,
                "NotImplementedError",
                "ResumeResource is not yet implemented in winterbaume-forecast",
            ),
            "StopResource" => json_error_response(
                501,
                "NotImplementedError",
                "StopResource is not yet implemented in winterbaume-forecast",
            ),
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-forecast",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-forecast",
            ),
            "UpdateDatasetGroup" => self.handle_update_dataset_group(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Forecast"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_dataset_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ForecastState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dataset_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.dataset_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DatasetGroupName'");
        }
        if input.domain.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Domain'");
        }
        let dataset_group_name = input.dataset_group_name.as_str();
        let domain = input.domain.as_str();
        let dataset_arns = input.dataset_arns;

        let mut state = state.write().await;
        match state.create_dataset_group(
            dataset_group_name,
            domain,
            dataset_arns,
            account_id,
            region,
        ) {
            Ok(arn) => {
                let resp = wire::CreateDatasetGroupResponse {
                    dataset_group_arn: Some(arn),
                };
                wire::serialize_create_dataset_group_response(&resp)
            }
            Err(e) => forecast_error_response(&e),
        }
    }

    async fn handle_describe_dataset_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ForecastState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_dataset_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.dataset_group_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DatasetGroupArn'");
        }
        let dataset_group_arn = input.dataset_group_arn.as_str();

        let state = state.read().await;
        match state.describe_dataset_group(dataset_group_arn) {
            Ok(dg) => {
                let resp = wire::DescribeDatasetGroupResponse {
                    dataset_group_name: Some(dg.dataset_group_name.clone()),
                    dataset_group_arn: Some(dg.dataset_group_arn.clone()),
                    domain: Some(dg.domain.clone()),
                    dataset_arns: Some(dg.dataset_arns.clone()),
                    status: Some(dg.status.clone()),
                    creation_time: Some(dg.creation_time.timestamp() as f64),
                    last_modification_time: Some(dg.last_modification_time.timestamp() as f64),
                };
                wire::serialize_describe_dataset_group_response(&resp)
            }
            Err(e) => forecast_error_response(&e),
        }
    }

    async fn handle_delete_dataset_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ForecastState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_dataset_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.dataset_group_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DatasetGroupArn'");
        }
        let dataset_group_arn = input.dataset_group_arn.as_str();

        let mut state = state.write().await;
        match state.delete_dataset_group(dataset_group_arn) {
            Ok(()) => wire::serialize_delete_dataset_group_response(),
            Err(e) => forecast_error_response(&e),
        }
    }

    async fn handle_list_dataset_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ForecastState>>,
        body: &[u8],
    ) -> MockResponse {
        let _input = match wire::deserialize_list_dataset_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let state = state.read().await;
        let dataset_groups = state.list_dataset_groups();

        let groups: Vec<wire::DatasetGroupSummary> = dataset_groups
            .iter()
            .map(|dg| wire::DatasetGroupSummary {
                dataset_group_name: Some(dg.dataset_group_name.clone()),
                dataset_group_arn: Some(dg.dataset_group_arn.clone()),
                creation_time: Some(dg.creation_time.timestamp() as f64),
                last_modification_time: Some(dg.last_modification_time.timestamp() as f64),
            })
            .collect();

        let resp = wire::ListDatasetGroupsResponse {
            dataset_groups: Some(groups),
            next_token: None,
        };
        wire::serialize_list_dataset_groups_response(&resp)
    }

    async fn handle_update_dataset_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ForecastState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_dataset_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.dataset_group_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DatasetGroupArn'");
        }
        let dataset_group_arn = input.dataset_group_arn.as_str();
        let dataset_arns = input.dataset_arns;

        let mut state = state.write().await;
        match state.update_dataset_group(dataset_group_arn, dataset_arns) {
            Ok(()) => {
                let resp = wire::UpdateDatasetGroupResponse {};
                wire::serialize_update_dataset_group_response(&resp)
            }
            Err(e) => forecast_error_response(&e),
        }
    }
}

fn forecast_error_response(err: &ForecastError) -> MockResponse {
    let (status, error_type, message) = match err {
        ForecastError::ResourceAlreadyExists { .. } => {
            (400, "ResourceAlreadyExistsException", err.to_string())
        }
        ForecastError::ResourceNotFound { .. } => {
            (400, "ResourceNotFoundException", err.to_string())
        }
    };
    let body = json!({
        "__type": error_type,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
