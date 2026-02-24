//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-costexplorer

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_anomaly_monitor_response(
    result: &CreateAnomalyMonitorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_anomaly_subscription_response(
    result: &CreateAnomalySubscriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_cost_category_definition_response(
    result: &CreateCostCategoryDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_anomaly_monitor_response(
    result: &DeleteAnomalyMonitorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_anomaly_subscription_response(
    result: &DeleteAnomalySubscriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_cost_category_definition_response(
    result: &DeleteCostCategoryDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_cost_category_definition_response(
    result: &DescribeCostCategoryDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_anomalies_response(result: &GetAnomaliesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_anomaly_monitors_response(
    result: &GetAnomalyMonitorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_anomaly_subscriptions_response(
    result: &GetAnomalySubscriptionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_approximate_usage_records_response(
    result: &GetApproximateUsageRecordsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_commitment_purchase_analysis_response(
    result: &GetCommitmentPurchaseAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_cost_and_usage_response(result: &GetCostAndUsageResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_cost_and_usage_comparisons_response(
    result: &GetCostAndUsageComparisonsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_cost_and_usage_with_resources_response(
    result: &GetCostAndUsageWithResourcesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_cost_categories_response(result: &GetCostCategoriesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_cost_comparison_drivers_response(
    result: &GetCostComparisonDriversResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_cost_forecast_response(result: &GetCostForecastResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dimension_values_response(
    result: &GetDimensionValuesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_reservation_coverage_response(
    result: &GetReservationCoverageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_reservation_purchase_recommendation_response(
    result: &GetReservationPurchaseRecommendationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_reservation_utilization_response(
    result: &GetReservationUtilizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_rightsizing_recommendation_response(
    result: &GetRightsizingRecommendationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_savings_plan_purchase_recommendation_details_response(
    result: &GetSavingsPlanPurchaseRecommendationDetailsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_savings_plans_coverage_response(
    result: &GetSavingsPlansCoverageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_savings_plans_purchase_recommendation_response(
    result: &GetSavingsPlansPurchaseRecommendationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_savings_plans_utilization_response(
    result: &GetSavingsPlansUtilizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_savings_plans_utilization_details_response(
    result: &GetSavingsPlansUtilizationDetailsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_tags_response(result: &GetTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_usage_forecast_response(result: &GetUsageForecastResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_commitment_purchase_analyses_response(
    result: &ListCommitmentPurchaseAnalysesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_cost_allocation_tag_backfill_history_response(
    result: &ListCostAllocationTagBackfillHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_cost_allocation_tags_response(
    result: &ListCostAllocationTagsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_cost_category_definitions_response(
    result: &ListCostCategoryDefinitionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_cost_category_resource_associations_response(
    result: &ListCostCategoryResourceAssociationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_savings_plans_purchase_recommendation_generation_response(
    result: &ListSavingsPlansPurchaseRecommendationGenerationResponse,
) -> MockResponse {
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
pub fn serialize_provide_anomaly_feedback_response(
    result: &ProvideAnomalyFeedbackResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_commitment_purchase_analysis_response(
    result: &StartCommitmentPurchaseAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_cost_allocation_tag_backfill_response(
    result: &StartCostAllocationTagBackfillResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_savings_plans_purchase_recommendation_generation_response(
    result: &StartSavingsPlansPurchaseRecommendationGenerationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
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
pub fn serialize_update_anomaly_monitor_response(
    result: &UpdateAnomalyMonitorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_anomaly_subscription_response(
    result: &UpdateAnomalySubscriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_cost_allocation_tags_status_response(
    result: &UpdateCostAllocationTagsStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_cost_category_definition_response(
    result: &UpdateCostCategoryDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_anomaly_monitor_request(
    body: &[u8],
) -> Result<CreateAnomalyMonitorRequest, String> {
    if body.is_empty() {
        return Ok(CreateAnomalyMonitorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAnomalyMonitor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_anomaly_subscription_request(
    body: &[u8],
) -> Result<CreateAnomalySubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(CreateAnomalySubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAnomalySubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_cost_category_definition_request(
    body: &[u8],
) -> Result<CreateCostCategoryDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(CreateCostCategoryDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCostCategoryDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_anomaly_monitor_request(
    body: &[u8],
) -> Result<DeleteAnomalyMonitorRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAnomalyMonitorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAnomalyMonitor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_anomaly_subscription_request(
    body: &[u8],
) -> Result<DeleteAnomalySubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAnomalySubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAnomalySubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_cost_category_definition_request(
    body: &[u8],
) -> Result<DeleteCostCategoryDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCostCategoryDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCostCategoryDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_cost_category_definition_request(
    body: &[u8],
) -> Result<DescribeCostCategoryDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCostCategoryDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCostCategoryDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_anomalies_request(body: &[u8]) -> Result<GetAnomaliesRequest, String> {
    if body.is_empty() {
        return Ok(GetAnomaliesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAnomalies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_anomaly_monitors_request(
    body: &[u8],
) -> Result<GetAnomalyMonitorsRequest, String> {
    if body.is_empty() {
        return Ok(GetAnomalyMonitorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAnomalyMonitors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_anomaly_subscriptions_request(
    body: &[u8],
) -> Result<GetAnomalySubscriptionsRequest, String> {
    if body.is_empty() {
        return Ok(GetAnomalySubscriptionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAnomalySubscriptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_approximate_usage_records_request(
    body: &[u8],
) -> Result<GetApproximateUsageRecordsRequest, String> {
    if body.is_empty() {
        return Ok(GetApproximateUsageRecordsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetApproximateUsageRecords request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_commitment_purchase_analysis_request(
    body: &[u8],
) -> Result<GetCommitmentPurchaseAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(GetCommitmentPurchaseAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCommitmentPurchaseAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_cost_and_usage_request(
    body: &[u8],
) -> Result<GetCostAndUsageRequest, String> {
    if body.is_empty() {
        return Ok(GetCostAndUsageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCostAndUsage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_cost_and_usage_comparisons_request(
    body: &[u8],
) -> Result<GetCostAndUsageComparisonsRequest, String> {
    if body.is_empty() {
        return Ok(GetCostAndUsageComparisonsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCostAndUsageComparisons request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_cost_and_usage_with_resources_request(
    body: &[u8],
) -> Result<GetCostAndUsageWithResourcesRequest, String> {
    if body.is_empty() {
        return Ok(GetCostAndUsageWithResourcesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCostAndUsageWithResources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_cost_categories_request(
    body: &[u8],
) -> Result<GetCostCategoriesRequest, String> {
    if body.is_empty() {
        return Ok(GetCostCategoriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCostCategories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_cost_comparison_drivers_request(
    body: &[u8],
) -> Result<GetCostComparisonDriversRequest, String> {
    if body.is_empty() {
        return Ok(GetCostComparisonDriversRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCostComparisonDrivers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_cost_forecast_request(
    body: &[u8],
) -> Result<GetCostForecastRequest, String> {
    if body.is_empty() {
        return Ok(GetCostForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCostForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_dimension_values_request(
    body: &[u8],
) -> Result<GetDimensionValuesRequest, String> {
    if body.is_empty() {
        return Ok(GetDimensionValuesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDimensionValues request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_reservation_coverage_request(
    body: &[u8],
) -> Result<GetReservationCoverageRequest, String> {
    if body.is_empty() {
        return Ok(GetReservationCoverageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetReservationCoverage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_reservation_purchase_recommendation_request(
    body: &[u8],
) -> Result<GetReservationPurchaseRecommendationRequest, String> {
    if body.is_empty() {
        return Ok(GetReservationPurchaseRecommendationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetReservationPurchaseRecommendation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_reservation_utilization_request(
    body: &[u8],
) -> Result<GetReservationUtilizationRequest, String> {
    if body.is_empty() {
        return Ok(GetReservationUtilizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetReservationUtilization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_rightsizing_recommendation_request(
    body: &[u8],
) -> Result<GetRightsizingRecommendationRequest, String> {
    if body.is_empty() {
        return Ok(GetRightsizingRecommendationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRightsizingRecommendation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_savings_plan_purchase_recommendation_details_request(
    body: &[u8],
) -> Result<GetSavingsPlanPurchaseRecommendationDetailsRequest, String> {
    if body.is_empty() {
        return Ok(GetSavingsPlanPurchaseRecommendationDetailsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetSavingsPlanPurchaseRecommendationDetails request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_savings_plans_coverage_request(
    body: &[u8],
) -> Result<GetSavingsPlansCoverageRequest, String> {
    if body.is_empty() {
        return Ok(GetSavingsPlansCoverageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSavingsPlansCoverage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_savings_plans_purchase_recommendation_request(
    body: &[u8],
) -> Result<GetSavingsPlansPurchaseRecommendationRequest, String> {
    if body.is_empty() {
        return Ok(GetSavingsPlansPurchaseRecommendationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetSavingsPlansPurchaseRecommendation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_savings_plans_utilization_request(
    body: &[u8],
) -> Result<GetSavingsPlansUtilizationRequest, String> {
    if body.is_empty() {
        return Ok(GetSavingsPlansUtilizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSavingsPlansUtilization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_savings_plans_utilization_details_request(
    body: &[u8],
) -> Result<GetSavingsPlansUtilizationDetailsRequest, String> {
    if body.is_empty() {
        return Ok(GetSavingsPlansUtilizationDetailsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetSavingsPlansUtilizationDetails request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_tags_request(body: &[u8]) -> Result<GetTagsRequest, String> {
    if body.is_empty() {
        return Ok(GetTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_usage_forecast_request(
    body: &[u8],
) -> Result<GetUsageForecastRequest, String> {
    if body.is_empty() {
        return Ok(GetUsageForecastRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUsageForecast request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_commitment_purchase_analyses_request(
    body: &[u8],
) -> Result<ListCommitmentPurchaseAnalysesRequest, String> {
    if body.is_empty() {
        return Ok(ListCommitmentPurchaseAnalysesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCommitmentPurchaseAnalyses request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_cost_allocation_tag_backfill_history_request(
    body: &[u8],
) -> Result<ListCostAllocationTagBackfillHistoryRequest, String> {
    if body.is_empty() {
        return Ok(ListCostAllocationTagBackfillHistoryRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListCostAllocationTagBackfillHistory request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_cost_allocation_tags_request(
    body: &[u8],
) -> Result<ListCostAllocationTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListCostAllocationTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCostAllocationTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_cost_category_definitions_request(
    body: &[u8],
) -> Result<ListCostCategoryDefinitionsRequest, String> {
    if body.is_empty() {
        return Ok(ListCostCategoryDefinitionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCostCategoryDefinitions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_cost_category_resource_associations_request(
    body: &[u8],
) -> Result<ListCostCategoryResourceAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(ListCostCategoryResourceAssociationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListCostCategoryResourceAssociations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_savings_plans_purchase_recommendation_generation_request(
    body: &[u8],
) -> Result<ListSavingsPlansPurchaseRecommendationGenerationRequest, String> {
    if body.is_empty() {
        return Ok(ListSavingsPlansPurchaseRecommendationGenerationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize ListSavingsPlansPurchaseRecommendationGeneration request: {e}"
        )
    })
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
pub fn deserialize_provide_anomaly_feedback_request(
    body: &[u8],
) -> Result<ProvideAnomalyFeedbackRequest, String> {
    if body.is_empty() {
        return Ok(ProvideAnomalyFeedbackRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ProvideAnomalyFeedback request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_commitment_purchase_analysis_request(
    body: &[u8],
) -> Result<StartCommitmentPurchaseAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(StartCommitmentPurchaseAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCommitmentPurchaseAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_cost_allocation_tag_backfill_request(
    body: &[u8],
) -> Result<StartCostAllocationTagBackfillRequest, String> {
    if body.is_empty() {
        return Ok(StartCostAllocationTagBackfillRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCostAllocationTagBackfill request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_savings_plans_purchase_recommendation_generation_request(
    body: &[u8],
) -> Result<StartSavingsPlansPurchaseRecommendationGenerationRequest, String> {
    if body.is_empty() {
        return Ok(StartSavingsPlansPurchaseRecommendationGenerationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize StartSavingsPlansPurchaseRecommendationGeneration request: {e}"
        )
    })
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
pub fn deserialize_update_anomaly_monitor_request(
    body: &[u8],
) -> Result<UpdateAnomalyMonitorRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAnomalyMonitorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAnomalyMonitor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_anomaly_subscription_request(
    body: &[u8],
) -> Result<UpdateAnomalySubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAnomalySubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAnomalySubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_cost_allocation_tags_status_request(
    body: &[u8],
) -> Result<UpdateCostAllocationTagsStatusRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCostAllocationTagsStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCostAllocationTagsStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_cost_category_definition_request(
    body: &[u8],
) -> Result<UpdateCostCategoryDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCostCategoryDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCostCategoryDefinition request: {e}"))
}
