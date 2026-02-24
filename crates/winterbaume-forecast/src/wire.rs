//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-forecast

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_auto_predictor_response(
    result: &CreateAutoPredictorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_dataset_response(result: &CreateDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_dataset_group_response(
    result: &CreateDatasetGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_dataset_import_job_response(
    result: &CreateDatasetImportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_explainability_response(
    result: &CreateExplainabilityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_explainability_export_response(
    result: &CreateExplainabilityExportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_forecast_response(result: &CreateForecastResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_forecast_export_job_response(
    result: &CreateForecastExportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_monitor_response(result: &CreateMonitorResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_predictor_response(result: &CreatePredictorResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_predictor_backtest_export_job_response(
    result: &CreatePredictorBacktestExportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_what_if_analysis_response(
    result: &CreateWhatIfAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_what_if_forecast_response(
    result: &CreateWhatIfForecastResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_what_if_forecast_export_response(
    result: &CreateWhatIfForecastExportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_dataset_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_dataset_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_dataset_import_job_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_explainability_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_explainability_export_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_forecast_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_forecast_export_job_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_monitor_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_predictor_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_predictor_backtest_export_job_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_resource_tree_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_what_if_analysis_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_what_if_forecast_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_what_if_forecast_export_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_auto_predictor_response(
    result: &DescribeAutoPredictorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_dataset_response(result: &DescribeDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_dataset_group_response(
    result: &DescribeDatasetGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_dataset_import_job_response(
    result: &DescribeDatasetImportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_explainability_response(
    result: &DescribeExplainabilityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_explainability_export_response(
    result: &DescribeExplainabilityExportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_forecast_response(result: &DescribeForecastResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_forecast_export_job_response(
    result: &DescribeForecastExportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_monitor_response(result: &DescribeMonitorResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_predictor_response(result: &DescribePredictorResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_predictor_backtest_export_job_response(
    result: &DescribePredictorBacktestExportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_what_if_analysis_response(
    result: &DescribeWhatIfAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_what_if_forecast_response(
    result: &DescribeWhatIfForecastResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_what_if_forecast_export_response(
    result: &DescribeWhatIfForecastExportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_accuracy_metrics_response(
    result: &GetAccuracyMetricsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dataset_groups_response(result: &ListDatasetGroupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dataset_import_jobs_response(
    result: &ListDatasetImportJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_datasets_response(result: &ListDatasetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_explainabilities_response(
    result: &ListExplainabilitiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_explainability_exports_response(
    result: &ListExplainabilityExportsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_forecast_export_jobs_response(
    result: &ListForecastExportJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_forecasts_response(result: &ListForecastsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_monitor_evaluations_response(
    result: &ListMonitorEvaluationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_monitors_response(result: &ListMonitorsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_predictor_backtest_export_jobs_response(
    result: &ListPredictorBacktestExportJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_predictors_response(result: &ListPredictorsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_what_if_analyses_response(
    result: &ListWhatIfAnalysesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_what_if_forecast_exports_response(
    result: &ListWhatIfForecastExportsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_what_if_forecasts_response(
    result: &ListWhatIfForecastsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_resume_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_stop_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_dataset_group_response(
    result: &UpdateDatasetGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_auto_predictor_request(
    body: &[u8],
) -> Result<CreateAutoPredictorRequest, String> {
    if body.is_empty() {
        return Ok(CreateAutoPredictorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAutoPredictor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_dataset_request(body: &[u8]) -> Result<CreateDatasetRequest, String> {
    if body.is_empty() {
        return Ok(CreateDatasetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_dataset_group_request(
    body: &[u8],
) -> Result<CreateDatasetGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateDatasetGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDatasetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_dataset_import_job_request(
    body: &[u8],
) -> Result<CreateDatasetImportJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateDatasetImportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDatasetImportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_explainability_request(
    body: &[u8],
) -> Result<CreateExplainabilityRequest, String> {
    if body.is_empty() {
        return Ok(CreateExplainabilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateExplainability request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_explainability_export_request(
    body: &[u8],
) -> Result<CreateExplainabilityExportRequest, String> {
    if body.is_empty() {
        return Ok(CreateExplainabilityExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateExplainabilityExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_forecast_request(body: &[u8]) -> Result<CreateForecastRequest, String> {
    if body.is_empty() {
        return Ok(CreateForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_forecast_export_job_request(
    body: &[u8],
) -> Result<CreateForecastExportJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateForecastExportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateForecastExportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_monitor_request(body: &[u8]) -> Result<CreateMonitorRequest, String> {
    if body.is_empty() {
        return Ok(CreateMonitorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMonitor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_predictor_request(body: &[u8]) -> Result<CreatePredictorRequest, String> {
    if body.is_empty() {
        return Ok(CreatePredictorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePredictor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_predictor_backtest_export_job_request(
    body: &[u8],
) -> Result<CreatePredictorBacktestExportJobRequest, String> {
    if body.is_empty() {
        return Ok(CreatePredictorBacktestExportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePredictorBacktestExportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_what_if_analysis_request(
    body: &[u8],
) -> Result<CreateWhatIfAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(CreateWhatIfAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWhatIfAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_what_if_forecast_request(
    body: &[u8],
) -> Result<CreateWhatIfForecastRequest, String> {
    if body.is_empty() {
        return Ok(CreateWhatIfForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWhatIfForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_what_if_forecast_export_request(
    body: &[u8],
) -> Result<CreateWhatIfForecastExportRequest, String> {
    if body.is_empty() {
        return Ok(CreateWhatIfForecastExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWhatIfForecastExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_dataset_request(body: &[u8]) -> Result<DeleteDatasetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDatasetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_dataset_group_request(
    body: &[u8],
) -> Result<DeleteDatasetGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDatasetGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDatasetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_dataset_import_job_request(
    body: &[u8],
) -> Result<DeleteDatasetImportJobRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDatasetImportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDatasetImportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_explainability_request(
    body: &[u8],
) -> Result<DeleteExplainabilityRequest, String> {
    if body.is_empty() {
        return Ok(DeleteExplainabilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteExplainability request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_explainability_export_request(
    body: &[u8],
) -> Result<DeleteExplainabilityExportRequest, String> {
    if body.is_empty() {
        return Ok(DeleteExplainabilityExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteExplainabilityExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_forecast_request(body: &[u8]) -> Result<DeleteForecastRequest, String> {
    if body.is_empty() {
        return Ok(DeleteForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_forecast_export_job_request(
    body: &[u8],
) -> Result<DeleteForecastExportJobRequest, String> {
    if body.is_empty() {
        return Ok(DeleteForecastExportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteForecastExportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_monitor_request(body: &[u8]) -> Result<DeleteMonitorRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMonitorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMonitor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_predictor_request(body: &[u8]) -> Result<DeletePredictorRequest, String> {
    if body.is_empty() {
        return Ok(DeletePredictorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePredictor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_predictor_backtest_export_job_request(
    body: &[u8],
) -> Result<DeletePredictorBacktestExportJobRequest, String> {
    if body.is_empty() {
        return Ok(DeletePredictorBacktestExportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePredictorBacktestExportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_tree_request(
    body: &[u8],
) -> Result<DeleteResourceTreeRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourceTreeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourceTree request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_what_if_analysis_request(
    body: &[u8],
) -> Result<DeleteWhatIfAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWhatIfAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWhatIfAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_what_if_forecast_request(
    body: &[u8],
) -> Result<DeleteWhatIfForecastRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWhatIfForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWhatIfForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_what_if_forecast_export_request(
    body: &[u8],
) -> Result<DeleteWhatIfForecastExportRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWhatIfForecastExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWhatIfForecastExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_auto_predictor_request(
    body: &[u8],
) -> Result<DescribeAutoPredictorRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAutoPredictorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAutoPredictor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_dataset_request(body: &[u8]) -> Result<DescribeDatasetRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDatasetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDataset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_dataset_group_request(
    body: &[u8],
) -> Result<DescribeDatasetGroupRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDatasetGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDatasetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_dataset_import_job_request(
    body: &[u8],
) -> Result<DescribeDatasetImportJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDatasetImportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDatasetImportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_explainability_request(
    body: &[u8],
) -> Result<DescribeExplainabilityRequest, String> {
    if body.is_empty() {
        return Ok(DescribeExplainabilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExplainability request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_explainability_export_request(
    body: &[u8],
) -> Result<DescribeExplainabilityExportRequest, String> {
    if body.is_empty() {
        return Ok(DescribeExplainabilityExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExplainabilityExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_forecast_request(
    body: &[u8],
) -> Result<DescribeForecastRequest, String> {
    if body.is_empty() {
        return Ok(DescribeForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_forecast_export_job_request(
    body: &[u8],
) -> Result<DescribeForecastExportJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeForecastExportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeForecastExportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_monitor_request(body: &[u8]) -> Result<DescribeMonitorRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMonitorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMonitor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_predictor_request(
    body: &[u8],
) -> Result<DescribePredictorRequest, String> {
    if body.is_empty() {
        return Ok(DescribePredictorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePredictor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_predictor_backtest_export_job_request(
    body: &[u8],
) -> Result<DescribePredictorBacktestExportJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribePredictorBacktestExportJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribePredictorBacktestExportJob request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_what_if_analysis_request(
    body: &[u8],
) -> Result<DescribeWhatIfAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWhatIfAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWhatIfAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_what_if_forecast_request(
    body: &[u8],
) -> Result<DescribeWhatIfForecastRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWhatIfForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWhatIfForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_what_if_forecast_export_request(
    body: &[u8],
) -> Result<DescribeWhatIfForecastExportRequest, String> {
    if body.is_empty() {
        return Ok(DescribeWhatIfForecastExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeWhatIfForecastExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_accuracy_metrics_request(
    body: &[u8],
) -> Result<GetAccuracyMetricsRequest, String> {
    if body.is_empty() {
        return Ok(GetAccuracyMetricsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAccuracyMetrics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dataset_groups_request(
    body: &[u8],
) -> Result<ListDatasetGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListDatasetGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatasetGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dataset_import_jobs_request(
    body: &[u8],
) -> Result<ListDatasetImportJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListDatasetImportJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatasetImportJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_datasets_request(body: &[u8]) -> Result<ListDatasetsRequest, String> {
    if body.is_empty() {
        return Ok(ListDatasetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatasets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_explainabilities_request(
    body: &[u8],
) -> Result<ListExplainabilitiesRequest, String> {
    if body.is_empty() {
        return Ok(ListExplainabilitiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListExplainabilities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_explainability_exports_request(
    body: &[u8],
) -> Result<ListExplainabilityExportsRequest, String> {
    if body.is_empty() {
        return Ok(ListExplainabilityExportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListExplainabilityExports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_forecast_export_jobs_request(
    body: &[u8],
) -> Result<ListForecastExportJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListForecastExportJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListForecastExportJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_forecasts_request(body: &[u8]) -> Result<ListForecastsRequest, String> {
    if body.is_empty() {
        return Ok(ListForecastsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListForecasts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_monitor_evaluations_request(
    body: &[u8],
) -> Result<ListMonitorEvaluationsRequest, String> {
    if body.is_empty() {
        return Ok(ListMonitorEvaluationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMonitorEvaluations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_monitors_request(body: &[u8]) -> Result<ListMonitorsRequest, String> {
    if body.is_empty() {
        return Ok(ListMonitorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMonitors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_predictor_backtest_export_jobs_request(
    body: &[u8],
) -> Result<ListPredictorBacktestExportJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListPredictorBacktestExportJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPredictorBacktestExportJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_predictors_request(body: &[u8]) -> Result<ListPredictorsRequest, String> {
    if body.is_empty() {
        return Ok(ListPredictorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPredictors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_what_if_analyses_request(
    body: &[u8],
) -> Result<ListWhatIfAnalysesRequest, String> {
    if body.is_empty() {
        return Ok(ListWhatIfAnalysesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWhatIfAnalyses request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_what_if_forecast_exports_request(
    body: &[u8],
) -> Result<ListWhatIfForecastExportsRequest, String> {
    if body.is_empty() {
        return Ok(ListWhatIfForecastExportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWhatIfForecastExports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_what_if_forecasts_request(
    body: &[u8],
) -> Result<ListWhatIfForecastsRequest, String> {
    if body.is_empty() {
        return Ok(ListWhatIfForecastsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWhatIfForecasts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resume_resource_request(body: &[u8]) -> Result<ResumeResourceRequest, String> {
    if body.is_empty() {
        return Ok(ResumeResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResumeResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_resource_request(body: &[u8]) -> Result<StopResourceRequest, String> {
    if body.is_empty() {
        return Ok(StopResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceRequest, String> {
    if body.is_empty() {
        return Ok(TagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_dataset_group_request(
    body: &[u8],
) -> Result<UpdateDatasetGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDatasetGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDatasetGroup request: {e}"))
}
