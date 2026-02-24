//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-personalize

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_batch_inference_job_response(
    result: &CreateBatchInferenceJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_batch_segment_job_response(
    result: &CreateBatchSegmentJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_campaign_response(result: &CreateCampaignResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_data_deletion_job_response(
    result: &CreateDataDeletionJobResponse,
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
pub fn serialize_create_dataset_export_job_response(
    result: &CreateDatasetExportJobResponse,
) -> MockResponse {
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
pub fn serialize_create_event_tracker_response(
    result: &CreateEventTrackerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_filter_response(result: &CreateFilterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_metric_attribution_response(
    result: &CreateMetricAttributionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_recommender_response(result: &CreateRecommenderResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_schema_response(result: &CreateSchemaResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_solution_response(result: &CreateSolutionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_solution_version_response(
    result: &CreateSolutionVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_campaign_response() -> MockResponse {
    MockResponse::json(200, "{}")
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
pub fn serialize_delete_event_tracker_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_filter_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_metric_attribution_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_recommender_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_schema_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_solution_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_algorithm_response(result: &DescribeAlgorithmResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_batch_inference_job_response(
    result: &DescribeBatchInferenceJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_batch_segment_job_response(
    result: &DescribeBatchSegmentJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_campaign_response(result: &DescribeCampaignResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_data_deletion_job_response(
    result: &DescribeDataDeletionJobResponse,
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
pub fn serialize_describe_dataset_export_job_response(
    result: &DescribeDatasetExportJobResponse,
) -> MockResponse {
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
pub fn serialize_describe_event_tracker_response(
    result: &DescribeEventTrackerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_feature_transformation_response(
    result: &DescribeFeatureTransformationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_filter_response(result: &DescribeFilterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metric_attribution_response(
    result: &DescribeMetricAttributionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_recipe_response(result: &DescribeRecipeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_recommender_response(
    result: &DescribeRecommenderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_schema_response(result: &DescribeSchemaResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_solution_response(result: &DescribeSolutionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_solution_version_response(
    result: &DescribeSolutionVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_solution_metrics_response(
    result: &GetSolutionMetricsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_batch_inference_jobs_response(
    result: &ListBatchInferenceJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_batch_segment_jobs_response(
    result: &ListBatchSegmentJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_campaigns_response(result: &ListCampaignsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_deletion_jobs_response(
    result: &ListDataDeletionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dataset_export_jobs_response(
    result: &ListDatasetExportJobsResponse,
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
pub fn serialize_list_event_trackers_response(result: &ListEventTrackersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_filters_response(result: &ListFiltersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_metric_attribution_metrics_response(
    result: &ListMetricAttributionMetricsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_metric_attributions_response(
    result: &ListMetricAttributionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_recipes_response(result: &ListRecipesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_recommenders_response(result: &ListRecommendersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_schemas_response(result: &ListSchemasResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_solution_versions_response(
    result: &ListSolutionVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_solutions_response(result: &ListSolutionsResponse) -> MockResponse {
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
pub fn serialize_start_recommender_response(result: &StartRecommenderResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_recommender_response(result: &StopRecommenderResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_stop_solution_version_creation_response() -> MockResponse {
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
pub fn serialize_update_campaign_response(result: &UpdateCampaignResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_dataset_response(result: &UpdateDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_metric_attribution_response(
    result: &UpdateMetricAttributionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_recommender_response(result: &UpdateRecommenderResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_solution_response(result: &UpdateSolutionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_batch_inference_job_request(
    body: &[u8],
) -> Result<CreateBatchInferenceJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateBatchInferenceJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBatchInferenceJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_batch_segment_job_request(
    body: &[u8],
) -> Result<CreateBatchSegmentJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateBatchSegmentJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBatchSegmentJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_campaign_request(body: &[u8]) -> Result<CreateCampaignRequest, String> {
    if body.is_empty() {
        return Ok(CreateCampaignRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCampaign request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_data_deletion_job_request(
    body: &[u8],
) -> Result<CreateDataDeletionJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateDataDeletionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataDeletionJob request: {e}"))
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
pub fn deserialize_create_dataset_export_job_request(
    body: &[u8],
) -> Result<CreateDatasetExportJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateDatasetExportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDatasetExportJob request: {e}"))
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
pub fn deserialize_create_event_tracker_request(
    body: &[u8],
) -> Result<CreateEventTrackerRequest, String> {
    if body.is_empty() {
        return Ok(CreateEventTrackerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEventTracker request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_filter_request(body: &[u8]) -> Result<CreateFilterRequest, String> {
    if body.is_empty() {
        return Ok(CreateFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_metric_attribution_request(
    body: &[u8],
) -> Result<CreateMetricAttributionRequest, String> {
    if body.is_empty() {
        return Ok(CreateMetricAttributionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMetricAttribution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_recommender_request(
    body: &[u8],
) -> Result<CreateRecommenderRequest, String> {
    if body.is_empty() {
        return Ok(CreateRecommenderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRecommender request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_schema_request(body: &[u8]) -> Result<CreateSchemaRequest, String> {
    if body.is_empty() {
        return Ok(CreateSchemaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_solution_request(body: &[u8]) -> Result<CreateSolutionRequest, String> {
    if body.is_empty() {
        return Ok(CreateSolutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSolution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_solution_version_request(
    body: &[u8],
) -> Result<CreateSolutionVersionRequest, String> {
    if body.is_empty() {
        return Ok(CreateSolutionVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSolutionVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_campaign_request(body: &[u8]) -> Result<DeleteCampaignRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCampaignRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCampaign request: {e}"))
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
pub fn deserialize_delete_event_tracker_request(
    body: &[u8],
) -> Result<DeleteEventTrackerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEventTrackerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEventTracker request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_filter_request(body: &[u8]) -> Result<DeleteFilterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_metric_attribution_request(
    body: &[u8],
) -> Result<DeleteMetricAttributionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMetricAttributionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMetricAttribution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_recommender_request(
    body: &[u8],
) -> Result<DeleteRecommenderRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRecommenderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRecommender request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_schema_request(body: &[u8]) -> Result<DeleteSchemaRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSchemaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_solution_request(body: &[u8]) -> Result<DeleteSolutionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSolutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSolution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_algorithm_request(
    body: &[u8],
) -> Result<DescribeAlgorithmRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAlgorithmRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAlgorithm request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_batch_inference_job_request(
    body: &[u8],
) -> Result<DescribeBatchInferenceJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBatchInferenceJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBatchInferenceJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_batch_segment_job_request(
    body: &[u8],
) -> Result<DescribeBatchSegmentJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBatchSegmentJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBatchSegmentJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_campaign_request(
    body: &[u8],
) -> Result<DescribeCampaignRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCampaignRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCampaign request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_data_deletion_job_request(
    body: &[u8],
) -> Result<DescribeDataDeletionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDataDeletionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDataDeletionJob request: {e}"))
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
pub fn deserialize_describe_dataset_export_job_request(
    body: &[u8],
) -> Result<DescribeDatasetExportJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDatasetExportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDatasetExportJob request: {e}"))
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
pub fn deserialize_describe_event_tracker_request(
    body: &[u8],
) -> Result<DescribeEventTrackerRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEventTrackerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEventTracker request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_feature_transformation_request(
    body: &[u8],
) -> Result<DescribeFeatureTransformationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFeatureTransformationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFeatureTransformation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_filter_request(body: &[u8]) -> Result<DescribeFilterRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metric_attribution_request(
    body: &[u8],
) -> Result<DescribeMetricAttributionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMetricAttributionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetricAttribution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_recipe_request(body: &[u8]) -> Result<DescribeRecipeRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRecipeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRecipe request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_recommender_request(
    body: &[u8],
) -> Result<DescribeRecommenderRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRecommenderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRecommender request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_schema_request(body: &[u8]) -> Result<DescribeSchemaRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSchemaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_solution_request(
    body: &[u8],
) -> Result<DescribeSolutionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSolutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSolution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_solution_version_request(
    body: &[u8],
) -> Result<DescribeSolutionVersionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSolutionVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSolutionVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_solution_metrics_request(
    body: &[u8],
) -> Result<GetSolutionMetricsRequest, String> {
    if body.is_empty() {
        return Ok(GetSolutionMetricsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSolutionMetrics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_batch_inference_jobs_request(
    body: &[u8],
) -> Result<ListBatchInferenceJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListBatchInferenceJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBatchInferenceJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_batch_segment_jobs_request(
    body: &[u8],
) -> Result<ListBatchSegmentJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListBatchSegmentJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBatchSegmentJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_campaigns_request(body: &[u8]) -> Result<ListCampaignsRequest, String> {
    if body.is_empty() {
        return Ok(ListCampaignsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCampaigns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_deletion_jobs_request(
    body: &[u8],
) -> Result<ListDataDeletionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListDataDeletionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDataDeletionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dataset_export_jobs_request(
    body: &[u8],
) -> Result<ListDatasetExportJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListDatasetExportJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatasetExportJobs request: {e}"))
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
pub fn deserialize_list_event_trackers_request(
    body: &[u8],
) -> Result<ListEventTrackersRequest, String> {
    if body.is_empty() {
        return Ok(ListEventTrackersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEventTrackers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_filters_request(body: &[u8]) -> Result<ListFiltersRequest, String> {
    if body.is_empty() {
        return Ok(ListFiltersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFilters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_metric_attribution_metrics_request(
    body: &[u8],
) -> Result<ListMetricAttributionMetricsRequest, String> {
    if body.is_empty() {
        return Ok(ListMetricAttributionMetricsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMetricAttributionMetrics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_metric_attributions_request(
    body: &[u8],
) -> Result<ListMetricAttributionsRequest, String> {
    if body.is_empty() {
        return Ok(ListMetricAttributionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMetricAttributions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_recipes_request(body: &[u8]) -> Result<ListRecipesRequest, String> {
    if body.is_empty() {
        return Ok(ListRecipesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRecipes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_recommenders_request(
    body: &[u8],
) -> Result<ListRecommendersRequest, String> {
    if body.is_empty() {
        return Ok(ListRecommendersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRecommenders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_schemas_request(body: &[u8]) -> Result<ListSchemasRequest, String> {
    if body.is_empty() {
        return Ok(ListSchemasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSchemas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_solution_versions_request(
    body: &[u8],
) -> Result<ListSolutionVersionsRequest, String> {
    if body.is_empty() {
        return Ok(ListSolutionVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSolutionVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_solutions_request(body: &[u8]) -> Result<ListSolutionsRequest, String> {
    if body.is_empty() {
        return Ok(ListSolutionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSolutions request: {e}"))
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
pub fn deserialize_start_recommender_request(
    body: &[u8],
) -> Result<StartRecommenderRequest, String> {
    if body.is_empty() {
        return Ok(StartRecommenderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartRecommender request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_recommender_request(body: &[u8]) -> Result<StopRecommenderRequest, String> {
    if body.is_empty() {
        return Ok(StopRecommenderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopRecommender request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_solution_version_creation_request(
    body: &[u8],
) -> Result<StopSolutionVersionCreationRequest, String> {
    if body.is_empty() {
        return Ok(StopSolutionVersionCreationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopSolutionVersionCreation request: {e}"))
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
pub fn deserialize_update_campaign_request(body: &[u8]) -> Result<UpdateCampaignRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCampaignRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCampaign request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_dataset_request(body: &[u8]) -> Result<UpdateDatasetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDatasetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDataset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_metric_attribution_request(
    body: &[u8],
) -> Result<UpdateMetricAttributionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateMetricAttributionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMetricAttribution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_recommender_request(
    body: &[u8],
) -> Result<UpdateRecommenderRequest, String> {
    if body.is_empty() {
        return Ok(UpdateRecommenderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRecommender request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_solution_request(body: &[u8]) -> Result<UpdateSolutionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSolutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSolution request: {e}"))
}
