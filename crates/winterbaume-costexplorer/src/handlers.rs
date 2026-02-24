use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{CostExplorerError, CostExplorerState};
use crate::types::{CostCategoryRule, SubscriberRecord};
use crate::views::CostExplorerStateView;
use crate::wire;

/// Cost Explorer service handler that processes awsJson1.1 protocol requests.
pub struct CostExplorerService {
    pub(crate) state: Arc<BackendState<CostExplorerState>>,
    pub(crate) notifier: StateChangeNotifier<CostExplorerStateView>,
}

impl CostExplorerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CostExplorerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CostExplorerService {
    fn service_name(&self) -> &str {
        "ce"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://ce\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CostExplorerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "AWSInsightsIndexService.GetCostAndUsage"
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
        // `wire` re-parse the bytes per operation. An empty body is treated as `{}`.
        let body_bytes: &[u8] = if request.body.is_empty() {
            b"{}"
        } else {
            if serde_json::from_slice::<Value>(&request.body).is_err() {
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }
            &request.body
        };

        let state = self.state.get(account_id, &region);

        use winterbaume_core::StatefulService;

        // Mutating actions that should trigger notify_state_changed
        let is_mutating = matches!(
            action.as_str(),
            "CreateAnomalyMonitor"
                | "CreateAnomalySubscription"
                | "CreateCostCategoryDefinition"
                | "DeleteAnomalyMonitor"
                | "DeleteAnomalySubscription"
                | "DeleteCostCategoryDefinition"
                | "ProvideAnomalyFeedback"
                | "StartCommitmentPurchaseAnalysis"
                | "StartCostAllocationTagBackfill"
                | "StartSavingsPlansPurchaseRecommendationGeneration"
                | "TagResource"
                | "UntagResource"
                | "UpdateAnomalyMonitor"
                | "UpdateAnomalySubscription"
                | "UpdateCostAllocationTagsStatus"
                | "UpdateCostCategoryDefinition"
        );

        let response = match action.as_str() {
            // Cost and usage queries (stateless)
            "GetCostAndUsage" => self.handle_get_cost_and_usage(body_bytes).await,
            "GetCostAndUsageWithResources" => {
                self.handle_get_cost_and_usage_with_resources(body_bytes)
                    .await
            }
            "GetCostAndUsageComparisons" => {
                self.handle_get_cost_and_usage_comparisons(body_bytes).await
            }
            "GetCostForecast" => self.handle_get_cost_forecast(body_bytes).await,
            "GetUsageForecast" => self.handle_get_usage_forecast(body_bytes).await,
            "GetDimensionValues" => self.handle_get_dimension_values(body_bytes).await,
            "GetTags" => self.handle_get_tags(body_bytes).await,
            "GetCostCategories" => self.handle_get_cost_categories(body_bytes).await,
            "GetCostComparisonDrivers" => self.handle_get_cost_comparison_drivers(body_bytes).await,

            // Reservation queries (stateless)
            "GetReservationCoverage" => self.handle_get_reservation_coverage(body_bytes).await,
            "GetReservationPurchaseRecommendation" => {
                self.handle_get_reservation_purchase_recommendation(body_bytes)
                    .await
            }
            "GetReservationUtilization" => {
                self.handle_get_reservation_utilization(body_bytes).await
            }

            // Savings Plans queries (stateless)
            "GetRightsizingRecommendation" => {
                self.handle_get_rightsizing_recommendation(body_bytes).await
            }
            "GetSavingsPlanPurchaseRecommendationDetails" => {
                self.handle_get_savings_plan_purchase_recommendation_details(body_bytes)
                    .await
            }
            "GetSavingsPlansCoverage" => self.handle_get_savings_plans_coverage(body_bytes).await,
            "GetSavingsPlansPurchaseRecommendation" => {
                self.handle_get_savings_plans_purchase_recommendation(body_bytes)
                    .await
            }
            "GetSavingsPlansUtilization" => {
                self.handle_get_savings_plans_utilization(body_bytes).await
            }
            "GetSavingsPlansUtilizationDetails" => {
                self.handle_get_savings_plans_utilization_details(body_bytes)
                    .await
            }
            "GetApproximateUsageRecords" => {
                self.handle_get_approximate_usage_records(body_bytes).await
            }
            "GetCommitmentPurchaseAnalysis" => {
                self.handle_get_commitment_purchase_analysis(body_bytes)
                    .await
            }

            // Cost Category CRUD (stateful)
            "CreateCostCategoryDefinition" => {
                self.handle_create_cost_category_definition(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteCostCategoryDefinition" => {
                self.handle_delete_cost_category_definition(&state, body_bytes)
                    .await
            }
            "DescribeCostCategoryDefinition" => {
                self.handle_describe_cost_category_definition(&state, body_bytes)
                    .await
            }
            "UpdateCostCategoryDefinition" => {
                self.handle_update_cost_category_definition(&state, body_bytes)
                    .await
            }
            "ListCostCategoryDefinitions" => {
                self.handle_list_cost_category_definitions(&state).await
            }
            "ListCostCategoryResourceAssociations" => {
                self.handle_list_cost_category_resource_associations(&state, body_bytes)
                    .await
            }

            // Anomaly Monitor CRUD (stateful)
            "CreateAnomalyMonitor" => {
                self.handle_create_anomaly_monitor(&state, body_bytes, account_id)
                    .await
            }
            "DeleteAnomalyMonitor" => self.handle_delete_anomaly_monitor(&state, body_bytes).await,
            "GetAnomalyMonitors" => self.handle_get_anomaly_monitors(&state, body_bytes).await,
            "UpdateAnomalyMonitor" => self.handle_update_anomaly_monitor(&state, body_bytes).await,

            // Anomaly Subscription CRUD (stateful)
            "CreateAnomalySubscription" => {
                self.handle_create_anomaly_subscription(&state, body_bytes, account_id)
                    .await
            }
            "DeleteAnomalySubscription" => {
                self.handle_delete_anomaly_subscription(&state, body_bytes)
                    .await
            }
            "GetAnomalySubscriptions" => {
                self.handle_get_anomaly_subscriptions(&state, body_bytes)
                    .await
            }
            "UpdateAnomalySubscription" => {
                self.handle_update_anomaly_subscription(&state, body_bytes)
                    .await
            }

            // Anomaly detection (stateless mock)
            "GetAnomalies" => self.handle_get_anomalies(body_bytes).await,
            "ProvideAnomalyFeedback" => {
                self.handle_provide_anomaly_feedback(&state, body_bytes)
                    .await
            }

            // Resource tags (stateful)
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,

            // Cost allocation tags (stateless mock)
            "ListCostAllocationTags" => {
                self.handle_list_cost_allocation_tags(&state, body_bytes)
                    .await
            }
            "ListCostAllocationTagBackfillHistory" => {
                self.handle_list_cost_allocation_tag_backfill_history(&state, body_bytes)
                    .await
            }
            "UpdateCostAllocationTagsStatus" => {
                self.handle_update_cost_allocation_tags_status(&state, body_bytes)
                    .await
            }
            "StartCostAllocationTagBackfill" => {
                self.handle_start_cost_allocation_tag_backfill(&state, body_bytes)
                    .await
            }

            // Commitment / savings generation (stateless mock)
            "ListCommitmentPurchaseAnalyses" => {
                self.handle_list_commitment_purchase_analyses(body_bytes)
                    .await
            }
            "ListSavingsPlansPurchaseRecommendationGeneration" => {
                self.handle_list_savings_plans_purchase_recommendation_generation(body_bytes)
                    .await
            }
            "StartCommitmentPurchaseAnalysis" => {
                self.handle_start_commitment_purchase_analysis(body_bytes)
                    .await
            }
            "StartSavingsPlansPurchaseRecommendationGeneration" => {
                self.handle_start_savings_plans_purchase_recommendation_generation()
                    .await
            }

            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for CostExplorer"),
            ),
        };

        if is_mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // =========================================================================
    // Cost & Usage (stateless)
    // =========================================================================

    // STUB[no-telemetry]: Cost and usage data requires real billing history; returns synthetic empty-total results.
    async fn handle_get_cost_and_usage(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_cost_and_usage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }
        if input.granularity.is_empty() {
            return json_error_response(400, "ValidationException", "Granularity is required");
        }
        if input.metrics.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Metrics is required and must not be empty",
            );
        }

        let start = if input.time_period.start.is_empty() {
            "2024-01-01".to_string()
        } else {
            input.time_period.start
        };
        let end = if input.time_period.end.is_empty() {
            "2024-01-31".to_string()
        } else {
            input.time_period.end
        };

        let response = wire::GetCostAndUsageResponse {
            results_by_time: Some(vec![wire::ResultByTime {
                time_period: Some(wire::DateInterval { start, end }),
                total: Some(std::collections::HashMap::new()),
                groups: Some(vec![]),
                estimated: Some(true),
            }]),
            dimension_value_attributes: Some(vec![]),
            ..Default::default()
        };

        wire::serialize_get_cost_and_usage_response(&response)
    }

    // STUB[no-telemetry]: Cost and usage with resources requires real billing and resource history; returns synthetic empty-total results.
    async fn handle_get_cost_and_usage_with_resources(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_cost_and_usage_with_resources_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }
        if input.granularity.is_empty() {
            return json_error_response(400, "ValidationException", "Granularity is required");
        }

        let start = if input.time_period.start.is_empty() {
            "2024-01-01".to_string()
        } else {
            input.time_period.start
        };
        let end = if input.time_period.end.is_empty() {
            "2024-01-31".to_string()
        } else {
            input.time_period.end
        };

        let response = wire::GetCostAndUsageWithResourcesResponse {
            results_by_time: Some(vec![wire::ResultByTime {
                time_period: Some(wire::DateInterval { start, end }),
                total: Some(std::collections::HashMap::new()),
                groups: Some(vec![]),
                estimated: Some(true),
            }]),
            dimension_value_attributes: Some(vec![]),
            ..Default::default()
        };

        wire::serialize_get_cost_and_usage_with_resources_response(&response)
    }

    // STUB[no-telemetry]: Cost and usage comparisons require real billing data; always returns empty results.
    async fn handle_get_cost_and_usage_comparisons(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_cost_and_usage_comparisons_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_time_period.start.is_empty() && input.baseline_time_period.end.is_empty()
        {
            return json_error_response(
                400,
                "ValidationException",
                "BaselineTimePeriod is required",
            );
        }
        if input.comparison_time_period.start.is_empty()
            && input.comparison_time_period.end.is_empty()
        {
            return json_error_response(
                400,
                "ValidationException",
                "ComparisonTimePeriod is required",
            );
        }
        if input.metric_for_comparison.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "MetricForComparison is required",
            );
        }

        let response = wire::GetCostAndUsageComparisonsResponse {
            ..Default::default()
        };
        wire::serialize_get_cost_and_usage_comparisons_response(&response)
    }

    // STUB[no-telemetry]: Cost forecasts require real billing history and ML models; returns zeroed forecast data.
    async fn handle_get_cost_forecast(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_cost_forecast_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }
        if input.metric.is_empty() {
            return json_error_response(400, "ValidationException", "Metric is required");
        }
        if input.granularity.is_empty() {
            return json_error_response(400, "ValidationException", "Granularity is required");
        }

        let start = if input.time_period.start.is_empty() {
            "2024-01-01".to_string()
        } else {
            input.time_period.start
        };
        let end = if input.time_period.end.is_empty() {
            "2024-12-31".to_string()
        } else {
            input.time_period.end
        };

        let response = wire::GetCostForecastResponse {
            total: Some(wire::MetricValue {
                amount: Some("0.0".to_string()),
                unit: Some("USD".to_string()),
            }),
            forecast_results_by_time: Some(vec![wire::ForecastResult {
                time_period: Some(wire::DateInterval { start, end }),
                mean_value: Some("0.0".to_string()),
                prediction_interval_lower_bound: Some("0.0".to_string()),
                prediction_interval_upper_bound: Some("0.0".to_string()),
            }]),
        };

        wire::serialize_get_cost_forecast_response(&response)
    }

    // STUB[no-telemetry]: Usage forecasts require real usage history and ML models; returns zeroed forecast data.
    async fn handle_get_usage_forecast(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_usage_forecast_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }
        if input.metric.is_empty() {
            return json_error_response(400, "ValidationException", "Metric is required");
        }
        if input.granularity.is_empty() {
            return json_error_response(400, "ValidationException", "Granularity is required");
        }

        let start = if input.time_period.start.is_empty() {
            "2024-01-01".to_string()
        } else {
            input.time_period.start
        };
        let end = if input.time_period.end.is_empty() {
            "2024-12-31".to_string()
        } else {
            input.time_period.end
        };

        let response = wire::GetUsageForecastResponse {
            total: Some(wire::MetricValue {
                amount: Some("0.0".to_string()),
                unit: Some("Hrs".to_string()),
            }),
            forecast_results_by_time: Some(vec![wire::ForecastResult {
                time_period: Some(wire::DateInterval { start, end }),
                mean_value: Some("0.0".to_string()),
                prediction_interval_lower_bound: Some("0.0".to_string()),
                prediction_interval_upper_bound: Some("0.0".to_string()),
            }]),
        };

        wire::serialize_get_usage_forecast_response(&response)
    }

    // STUB[no-telemetry]: Dimension values are derived from real billing data; always returns empty results.
    async fn handle_get_dimension_values(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_dimension_values_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }
        if input.dimension.is_empty() {
            return json_error_response(400, "ValidationException", "Dimension is required");
        }

        let response = wire::GetDimensionValuesResponse {
            dimension_values: Some(vec![]),
            return_size: Some(0),
            total_size: Some(0),
            next_page_token: None,
        };

        wire::serialize_get_dimension_values_response(&response)
    }

    // STUB[no-telemetry]: Tag values are derived from real resource billing data; always returns empty results.
    async fn handle_get_tags(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }

        let response = wire::GetTagsResponse {
            tags: Some(vec![]),
            return_size: Some(0),
            total_size: Some(0),
            next_page_token: None,
        };

        wire::serialize_get_tags_response(&response)
    }

    // STUB[no-telemetry]: Cost category values are derived from real billing data; always returns empty results.
    async fn handle_get_cost_categories(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_cost_categories_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }

        let response = wire::GetCostCategoriesResponse {
            cost_category_names: Some(vec![]),
            cost_category_values: Some(vec![]),
            return_size: Some(0),
            total_size: Some(0),
            next_page_token: None,
        };

        wire::serialize_get_cost_categories_response(&response)
    }

    // STUB[no-telemetry]: Cost comparison drivers require real billing data; always returns empty results.
    async fn handle_get_cost_comparison_drivers(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_cost_comparison_drivers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.baseline_time_period.start.is_empty() && input.baseline_time_period.end.is_empty()
        {
            return json_error_response(
                400,
                "ValidationException",
                "BaselineTimePeriod is required",
            );
        }
        if input.comparison_time_period.start.is_empty()
            && input.comparison_time_period.end.is_empty()
        {
            return json_error_response(
                400,
                "ValidationException",
                "ComparisonTimePeriod is required",
            );
        }
        if input.metric_for_comparison.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "MetricForComparison is required",
            );
        }

        let response = wire::GetCostComparisonDriversResponse {
            ..Default::default()
        };
        wire::serialize_get_cost_comparison_drivers_response(&response)
    }

    // =========================================================================
    // Reservation (stateless)
    // =========================================================================

    // STUB[no-telemetry]: Reservation coverage requires real Reserved Instance purchase history; always returns empty results.
    async fn handle_get_reservation_coverage(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_reservation_coverage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }

        let response = wire::GetReservationCoverageResponse {
            coverages_by_time: Some(vec![]),
            next_page_token: None,
            total: None,
        };

        wire::serialize_get_reservation_coverage_response(&response)
    }

    // STUB[no-telemetry]: Reservation purchase recommendations require real usage and billing history; always returns empty results.
    async fn handle_get_reservation_purchase_recommendation(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_reservation_purchase_recommendation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service.is_empty() {
            return json_error_response(400, "ValidationException", "Service is required");
        }

        let response = wire::GetReservationPurchaseRecommendationResponse {
            recommendations: Some(vec![]),
            next_page_token: None,
            metadata: None,
        };

        wire::serialize_get_reservation_purchase_recommendation_response(&response)
    }

    // STUB[no-telemetry]: Reservation utilisation requires real RI usage data; always returns empty results.
    async fn handle_get_reservation_utilization(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_reservation_utilization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }

        let response = wire::GetReservationUtilizationResponse {
            utilizations_by_time: Some(vec![]),
            next_page_token: None,
            total: None,
        };

        wire::serialize_get_reservation_utilization_response(&response)
    }

    // =========================================================================
    // Savings Plans / Rightsizing (stateless)
    // =========================================================================

    // STUB[no-telemetry]: Rightsizing recommendations require real instance usage and cost history; always returns empty results.
    async fn handle_get_rightsizing_recommendation(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_rightsizing_recommendation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service.is_empty() {
            return json_error_response(400, "ValidationException", "Service is required");
        }

        let response = wire::GetRightsizingRecommendationResponse {
            rightsizing_recommendations: Some(vec![]),
            next_page_token: None,
            ..Default::default()
        };

        wire::serialize_get_rightsizing_recommendation_response(&response)
    }

    // STUB[no-telemetry]: Savings plan purchase recommendation details require real usage analysis; returns no detail data.
    async fn handle_get_savings_plan_purchase_recommendation_details(
        &self,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_savings_plan_purchase_recommendation_details_request(
            body,
        ) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.recommendation_detail_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "RecommendationDetailId is required",
            );
        }

        let response = wire::GetSavingsPlanPurchaseRecommendationDetailsResponse {
            recommendation_detail_id: Some(input.recommendation_detail_id),
            recommendation_detail_data: None,
        };

        wire::serialize_get_savings_plan_purchase_recommendation_details_response(&response)
    }

    // STUB[no-telemetry]: Savings Plans coverage requires real Savings Plans purchase and usage history; always returns empty results.
    async fn handle_get_savings_plans_coverage(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_savings_plans_coverage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }

        let response = wire::GetSavingsPlansCoverageResponse {
            savings_plans_coverages: Some(vec![]),
            next_token: None,
        };

        wire::serialize_get_savings_plans_coverage_response(&response)
    }

    // STUB[no-telemetry]: Savings Plans purchase recommendations require real usage and cost history; always returns empty results.
    async fn handle_get_savings_plans_purchase_recommendation(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_savings_plans_purchase_recommendation_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.savings_plans_type.is_empty() {
            return json_error_response(400, "ValidationException", "SavingsPlansType is required");
        }
        if input.term_in_years.is_empty() {
            return json_error_response(400, "ValidationException", "TermInYears is required");
        }
        if input.payment_option.is_empty() {
            return json_error_response(400, "ValidationException", "PaymentOption is required");
        }
        if input.lookback_period_in_days.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "LookbackPeriodInDays is required",
            );
        }

        let response = wire::GetSavingsPlansPurchaseRecommendationResponse {
            savings_plans_purchase_recommendation: None,
            next_page_token: None,
            metadata: None,
        };

        wire::serialize_get_savings_plans_purchase_recommendation_response(&response)
    }

    // STUB[no-telemetry]: Savings Plans utilisation requires real Savings Plans usage data; always returns empty results.
    async fn handle_get_savings_plans_utilization(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_savings_plans_utilization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }

        let response = wire::GetSavingsPlansUtilizationResponse {
            savings_plans_utilizations_by_time: Some(vec![]),
            total: None,
        };

        wire::serialize_get_savings_plans_utilization_response(&response)
    }

    // STUB[no-telemetry]: Savings Plans utilisation details require real Savings Plans usage data; always returns empty results.
    async fn handle_get_savings_plans_utilization_details(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_savings_plans_utilization_details_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.time_period.start.is_empty() && input.time_period.end.is_empty() {
            return json_error_response(400, "ValidationException", "TimePeriod is required");
        }

        let response = wire::GetSavingsPlansUtilizationDetailsResponse {
            savings_plans_utilization_details: Some(vec![]),
            next_token: None,
            time_period: None,
            total: None,
        };

        wire::serialize_get_savings_plans_utilization_details_response(&response)
    }

    // STUB[no-telemetry]: Approximate usage records require real CloudWatch usage data; always returns zero totals.
    async fn handle_get_approximate_usage_records(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_approximate_usage_records_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.granularity.is_empty() {
            return json_error_response(400, "ValidationException", "Granularity is required");
        }
        if input.approximation_dimension.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "ApproximationDimension is required",
            );
        }

        let response = wire::GetApproximateUsageRecordsResponse {
            services: Some(std::collections::HashMap::new()),
            total_records: Some(0),
            lookback_period: None,
        };

        wire::serialize_get_approximate_usage_records_response(&response)
    }

    // STUB[no-telemetry]: Commitment purchase analysis requires real usage and purchase data; returns a synthetic SUCCEEDED status.
    async fn handle_get_commitment_purchase_analysis(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_commitment_purchase_analysis_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.analysis_id.is_empty() {
            return json_error_response(400, "ValidationException", "AnalysisId is required");
        }

        let response = wire::GetCommitmentPurchaseAnalysisResponse {
            analysis_id: Some(input.analysis_id),
            analysis_status: Some("SUCCEEDED".to_string()),
            ..Default::default()
        };

        wire::serialize_get_commitment_purchase_analysis_response(&response)
    }

    // =========================================================================
    // Cost Category CRUD (stateful)
    // =========================================================================

    async fn handle_create_cost_category_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cost_category_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Name is required");
        }
        let name = input.name;

        let rule_version = if input.rule_version.is_empty() {
            "CostCategoryExpression.v1".to_string()
        } else {
            input.rule_version
        };

        let rules: Vec<CostCategoryRule> = input
            .rules
            .iter()
            .map(|r| CostCategoryRule {
                value: r.value.clone(),
                rule: r.rule.as_ref().and_then(|v| serde_json::to_value(v).ok()),
            })
            .collect();

        if rules.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Rules is required and must not be empty",
            );
        }

        let effective_start = input.effective_start.as_deref();

        let mut state = state.write().await;
        match state.create_cost_category(
            &name,
            &rule_version,
            rules,
            effective_start,
            account_id,
            region,
        ) {
            Ok(definition) => {
                let response = wire::CreateCostCategoryDefinitionResponse {
                    cost_category_arn: Some(definition.cost_category_arn.clone()),
                    effective_start: Some(definition.effective_start.clone()),
                };
                wire::serialize_create_cost_category_definition_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    async fn handle_delete_cost_category_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cost_category_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cost_category_arn.is_empty() {
            return json_error_response(400, "ValidationException", "CostCategoryArn is required");
        }
        let arn = input.cost_category_arn;

        let mut state = state.write().await;
        match state.delete_cost_category(&arn) {
            Ok(d) => {
                let response = wire::DeleteCostCategoryDefinitionResponse {
                    cost_category_arn: Some(d.cost_category_arn),
                    effective_end: Some("2024-12-31".to_string()),
                };
                wire::serialize_delete_cost_category_definition_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    async fn handle_describe_cost_category_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cost_category_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cost_category_arn.is_empty() {
            return json_error_response(400, "ValidationException", "CostCategoryArn is required");
        }
        let arn = input.cost_category_arn;

        let state = state.read().await;
        match state.describe_cost_category(&arn) {
            Ok(d) => {
                let cost_category = wire::CostCategory {
                    cost_category_arn: Some(d.cost_category_arn.clone()),
                    name: Some(d.name.clone()),
                    rule_version: Some(d.rule_version.clone()),
                    effective_start: Some(d.effective_start.clone()),
                    effective_end: None,
                    rules: None,
                    split_charge_rules: None,
                    processing_status: None,
                    default_value: None,
                };
                let response = wire::DescribeCostCategoryDefinitionResponse {
                    cost_category: Some(cost_category),
                };
                wire::serialize_describe_cost_category_definition_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    async fn handle_update_cost_category_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_cost_category_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cost_category_arn.is_empty() {
            return json_error_response(400, "ValidationException", "CostCategoryArn is required");
        }
        let arn = input.cost_category_arn;

        let rule_version = if input.rule_version.is_empty() {
            "CostCategoryExpression.v1".to_string()
        } else {
            input.rule_version
        };

        let rules: Vec<CostCategoryRule> = input
            .rules
            .iter()
            .map(|r| CostCategoryRule {
                value: r.value.clone(),
                rule: r.rule.as_ref().and_then(|v| serde_json::to_value(v).ok()),
            })
            .collect();

        if rules.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Rules is required and must not be empty",
            );
        }

        let effective_start = input.effective_start.as_deref();

        let mut state = state.write().await;
        match state.update_cost_category(&arn, &rule_version, rules, effective_start) {
            Ok((cost_category_arn, effective_start)) => {
                let response = wire::UpdateCostCategoryDefinitionResponse {
                    cost_category_arn: Some(cost_category_arn),
                    effective_start: Some(effective_start),
                };
                wire::serialize_update_cost_category_definition_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    async fn handle_list_cost_category_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let categories = state.list_cost_categories();

        let refs: Vec<wire::CostCategoryReference> = categories
            .iter()
            .map(|d| wire::CostCategoryReference {
                cost_category_arn: Some(d.cost_category_arn.clone()),
                name: Some(d.name.clone()),
                effective_start: Some(d.effective_start.clone()),
                effective_end: None,
                number_of_rules: Some(d.rules.len() as i32),
                processing_status: None,
                default_value: None,
                values: None,
                supported_resource_types: None,
            })
            .collect();

        let response = wire::ListCostCategoryDefinitionsResponse {
            cost_category_references: Some(refs),
            next_token: None,
        };

        wire::serialize_list_cost_category_definitions_response(&response)
    }

    // Resource associations for cost categories: returns associations from cost
    // category definitions (derived from the names of existing cost categories).
    async fn handle_list_cost_category_resource_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_cost_category_resource_associations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cost_category_arn = input.cost_category_arn.as_deref();
        let guard = state.read().await;

        let associations: Vec<wire::CostCategoryResourceAssociation> =
            if let Some(arn) = cost_category_arn {
                if let Some(cat) = guard.cost_categories.get(arn) {
                    vec![wire::CostCategoryResourceAssociation {
                        cost_category_arn: Some(cat.cost_category_arn.clone()),
                        cost_category_name: Some(cat.name.clone()),
                        resource_arn: None,
                    }]
                } else {
                    vec![]
                }
            } else {
                guard
                    .cost_categories
                    .values()
                    .map(|cat| wire::CostCategoryResourceAssociation {
                        cost_category_arn: Some(cat.cost_category_arn.clone()),
                        cost_category_name: Some(cat.name.clone()),
                        resource_arn: None,
                    })
                    .collect()
            };

        let response = wire::ListCostCategoryResourceAssociationsResponse {
            cost_category_resource_associations: Some(associations),
            next_token: None,
        };
        wire::serialize_list_cost_category_resource_associations_response(&response)
    }

    // =========================================================================
    // Anomaly Monitor CRUD (stateful)
    // =========================================================================

    async fn handle_create_anomaly_monitor(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_anomaly_monitor_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let anomaly_monitor = input.anomaly_monitor;

        if anomaly_monitor.monitor_name.is_empty() {
            return json_error_response(400, "ValidationException", "MonitorName is required");
        }
        let monitor_name = anomaly_monitor.monitor_name;

        if anomaly_monitor.monitor_type.is_empty() {
            return json_error_response(400, "ValidationException", "MonitorType is required");
        }
        let monitor_type = anomaly_monitor.monitor_type;

        let monitor_dimension = anomaly_monitor.monitor_dimension.as_deref();

        let mut state = state.write().await;
        let arn = state.create_anomaly_monitor(
            &monitor_name,
            &monitor_type,
            monitor_dimension,
            account_id,
        );

        let response = wire::CreateAnomalyMonitorResponse {
            monitor_arn: Some(arn),
        };
        wire::serialize_create_anomaly_monitor_response(&response)
    }

    async fn handle_delete_anomaly_monitor(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_anomaly_monitor_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.monitor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "MonitorArn is required");
        }
        let monitor_arn = input.monitor_arn;

        let mut state = state.write().await;
        match state.delete_anomaly_monitor(&monitor_arn) {
            Ok(()) => {
                let response = wire::DeleteAnomalyMonitorResponse {};
                wire::serialize_delete_anomaly_monitor_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    async fn handle_get_anomaly_monitors(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_anomaly_monitors_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let monitor_arn_list = input.monitor_arn_list;

        let state = state.read().await;
        let monitors = state.get_anomaly_monitors(monitor_arn_list.as_ref());

        let wire_monitors: Vec<wire::AnomalyMonitor> = monitors
            .iter()
            .map(|m| wire::AnomalyMonitor {
                monitor_arn: Some(m.monitor_arn.clone()),
                monitor_name: m.monitor_name.clone(),
                monitor_type: m.monitor_type.clone(),
                monitor_dimension: m.monitor_dimension.clone(),
                creation_date: Some(m.creation_date.clone()),
                last_updated_date: Some(m.last_updated_date.clone()),
                last_evaluated_date: m.last_evaluated_date.clone(),
                dimensional_value_count: None,
                monitor_specification: None,
            })
            .collect();

        let response = wire::GetAnomalyMonitorsResponse {
            anomaly_monitors: Some(wire_monitors),
            next_page_token: None,
        };
        wire::serialize_get_anomaly_monitors_response(&response)
    }

    async fn handle_update_anomaly_monitor(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_anomaly_monitor_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.monitor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "MonitorArn is required");
        }
        let monitor_arn = input.monitor_arn;
        let monitor_name = input.monitor_name.as_deref();

        let mut state = state.write().await;
        match state.update_anomaly_monitor(&monitor_arn, monitor_name) {
            Ok(arn) => {
                let response = wire::UpdateAnomalyMonitorResponse {
                    monitor_arn: Some(arn),
                };
                wire::serialize_update_anomaly_monitor_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    // =========================================================================
    // Anomaly Subscription CRUD (stateful)
    // =========================================================================

    async fn handle_create_anomaly_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_anomaly_subscription_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let anomaly_subscription = input.anomaly_subscription;

        if anomaly_subscription.subscription_name.is_empty() {
            return json_error_response(400, "ValidationException", "SubscriptionName is required");
        }
        let subscription_name = anomaly_subscription.subscription_name;

        if anomaly_subscription.frequency.is_empty() {
            return json_error_response(400, "ValidationException", "Frequency is required");
        }
        let frequency = anomaly_subscription.frequency;

        let monitor_arn_list = anomaly_subscription.monitor_arn_list;

        let subscribers: Vec<SubscriberRecord> = anomaly_subscription
            .subscribers
            .into_iter()
            .map(|s| SubscriberRecord {
                address: s.address,
                status: s.status,
                subscriber_type: s.r#type,
            })
            .collect();

        let threshold = anomaly_subscription.threshold;

        let mut state = state.write().await;
        let arn = state.create_anomaly_subscription(
            &subscription_name,
            monitor_arn_list,
            subscribers,
            &frequency,
            threshold,
            account_id,
        );

        let response = wire::CreateAnomalySubscriptionResponse {
            subscription_arn: Some(arn),
        };
        wire::serialize_create_anomaly_subscription_response(&response)
    }

    async fn handle_delete_anomaly_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_anomaly_subscription_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.subscription_arn.is_empty() {
            return json_error_response(400, "ValidationException", "SubscriptionArn is required");
        }
        let subscription_arn = input.subscription_arn;

        let mut state = state.write().await;
        match state.delete_anomaly_subscription(&subscription_arn) {
            Ok(()) => {
                let response = wire::DeleteAnomalySubscriptionResponse {};
                wire::serialize_delete_anomaly_subscription_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    async fn handle_get_anomaly_subscriptions(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_anomaly_subscriptions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let subscription_arn_list = input.subscription_arn_list;
        let monitor_arn = input.monitor_arn.as_deref();

        let state = state.read().await;
        let subscriptions =
            state.get_anomaly_subscriptions(subscription_arn_list.as_ref(), monitor_arn);

        let wire_subs: Vec<wire::AnomalySubscription> = subscriptions
            .iter()
            .map(|s| wire::AnomalySubscription {
                subscription_arn: Some(s.subscription_arn.clone()),
                subscription_name: s.subscription_name.clone(),
                account_id: Some(s.account_id.clone()),
                monitor_arn_list: s.monitor_arn_list.clone(),
                subscribers: s
                    .subscribers
                    .iter()
                    .map(|sub| wire::Subscriber {
                        address: sub.address.clone(),
                        status: sub.status.clone(),
                        r#type: sub.subscriber_type.clone(),
                    })
                    .collect(),
                frequency: s.frequency.clone(),
                threshold: s.threshold,
                threshold_expression: None,
            })
            .collect();

        let response = wire::GetAnomalySubscriptionsResponse {
            anomaly_subscriptions: Some(wire_subs),
            next_page_token: None,
        };
        wire::serialize_get_anomaly_subscriptions_response(&response)
    }

    async fn handle_update_anomaly_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_anomaly_subscription_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.subscription_arn.is_empty() {
            return json_error_response(400, "ValidationException", "SubscriptionArn is required");
        }
        let subscription_arn = input.subscription_arn;
        let frequency = input.frequency.as_deref();
        let subscription_name = input.subscription_name.as_deref();
        let threshold = input.threshold;
        let monitor_arn_list = input.monitor_arn_list;

        let subscribers: Option<Vec<SubscriberRecord>> = input.subscribers.map(|arr| {
            arr.into_iter()
                .map(|s| SubscriberRecord {
                    address: s.address,
                    status: s.status,
                    subscriber_type: s.r#type,
                })
                .collect()
        });

        let mut state = state.write().await;
        match state.update_anomaly_subscription(
            &subscription_arn,
            frequency,
            monitor_arn_list,
            subscribers,
            subscription_name,
            threshold,
        ) {
            Ok(arn) => {
                let response = wire::UpdateAnomalySubscriptionResponse {
                    subscription_arn: Some(arn),
                };
                wire::serialize_update_anomaly_subscription_response(&response)
            }
            Err(e) => ce_error_response(&e),
        }
    }

    // =========================================================================
    // Anomaly detection (stateless mock)
    // =========================================================================

    // STUB[no-telemetry]: Anomaly detection requires real billing data and ML anomaly detection; always returns empty results.
    async fn handle_get_anomalies(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_anomalies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.date_interval.start_date.is_empty() && input.date_interval.end_date.is_none() {
            return json_error_response(400, "ValidationException", "DateInterval is required");
        }

        let response = wire::GetAnomaliesResponse {
            anomalies: Some(vec![]),
            next_page_token: None,
        };
        wire::serialize_get_anomalies_response(&response)
    }

    async fn handle_provide_anomaly_feedback(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_provide_anomaly_feedback_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.anomaly_id.is_empty() {
            return json_error_response(400, "ValidationException", "AnomalyId is required");
        }
        let anomaly_id = input.anomaly_id;

        if input.feedback.is_empty() {
            return json_error_response(400, "ValidationException", "Feedback is required");
        }
        let feedback = input.feedback;

        let mut guard = state.write().await;
        guard.provide_anomaly_feedback(&anomaly_id, &feedback);

        let response = wire::ProvideAnomalyFeedbackResponse {
            anomaly_id: Some(anomaly_id),
        };
        wire::serialize_provide_anomaly_feedback_response(&response)
    }

    // =========================================================================
    // Resource Tags (stateful)
    // =========================================================================

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceArn is required");
        }
        let resource_arn = input.resource_arn;

        let tags: Vec<(String, String)> = input
            .resource_tags
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();

        let mut state = state.write().await;
        state.tag_resource(&resource_arn, tags);

        let response = wire::TagResourceResponse {};
        wire::serialize_tag_resource_response(&response)
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceArn is required");
        }
        let resource_arn = input.resource_arn;
        let tag_keys = input.resource_tag_keys;

        let mut state = state.write().await;
        state.untag_resource(&resource_arn, &tag_keys);

        let response = wire::UntagResourceResponse {};
        wire::serialize_untag_resource_response(&response)
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceArn is required");
        }
        let resource_arn = input.resource_arn;

        let state = state.read().await;
        let tags = state.list_tags_for_resource(&resource_arn);

        let wire_tags: Vec<wire::ResourceTag> = tags
            .iter()
            .map(|(k, v)| wire::ResourceTag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();

        let response = wire::ListTagsForResourceResponse {
            resource_tags: Some(wire_tags),
        };
        wire::serialize_list_tags_for_resource_response(&response)
    }

    // =========================================================================
    // Cost allocation tags (stateless mock)
    // =========================================================================

    async fn handle_list_cost_allocation_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        _body: &[u8],
    ) -> MockResponse {
        let guard = state.read().await;
        let tags: Vec<wire::CostAllocationTag> = guard
            .list_cost_allocation_tags()
            .iter()
            .map(|t| wire::CostAllocationTag {
                tag_key: Some(t.tag_key.clone()),
                status: Some(t.status.clone()),
                r#type: Some(t.tag_type.clone()),
                last_updated_date: t.last_updated_date.clone(),
                last_used_date: t.last_used_date.clone(),
            })
            .collect();
        let response = wire::ListCostAllocationTagsResponse {
            cost_allocation_tags: Some(tags),
            next_token: None,
        };
        wire::serialize_list_cost_allocation_tags_response(&response)
    }

    async fn handle_list_cost_allocation_tag_backfill_history(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        _body: &[u8],
    ) -> MockResponse {
        let guard = state.read().await;
        let requests: Vec<wire::CostAllocationTagBackfillRequest> = guard
            .list_cost_allocation_tag_backfill_history()
            .iter()
            .map(|r| wire::CostAllocationTagBackfillRequest {
                backfill_from: Some(r.backfill_from.clone()),
                backfill_status: Some(r.backfill_status.clone()),
                requested_at: Some(r.requested_at.clone()),
                completed_at: r.completed_at.clone(),
                last_updated_at: r.last_updated_at.clone(),
            })
            .collect();
        let response = wire::ListCostAllocationTagBackfillHistoryResponse {
            backfill_requests: Some(requests),
            next_token: None,
        };
        wire::serialize_list_cost_allocation_tag_backfill_history_response(&response)
    }

    async fn handle_update_cost_allocation_tags_status(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_cost_allocation_tags_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let updates: Vec<(String, String)> = input
            .cost_allocation_tags_status
            .into_iter()
            .map(|entry| (entry.tag_key, entry.status))
            .collect();

        let mut guard = state.write().await;
        let _errors = guard.update_cost_allocation_tags_status(&updates);

        let response = wire::UpdateCostAllocationTagsStatusResponse {
            errors: Some(vec![]),
        };
        wire::serialize_update_cost_allocation_tags_status_response(&response)
    }

    async fn handle_start_cost_allocation_tag_backfill(
        &self,
        state: &Arc<tokio::sync::RwLock<CostExplorerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_cost_allocation_tag_backfill_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.backfill_from.is_empty() {
            return json_error_response(400, "ValidationException", "BackfillFrom is required");
        }
        let backfill_from = input.backfill_from;

        let mut guard = state.write().await;
        let record = guard.start_cost_allocation_tag_backfill(&backfill_from);
        let response = wire::StartCostAllocationTagBackfillResponse {
            backfill_request: Some(wire::CostAllocationTagBackfillRequest {
                backfill_from: Some(record.backfill_from.clone()),
                backfill_status: Some(record.backfill_status.clone()),
                requested_at: Some(record.requested_at.clone()),
                completed_at: record.completed_at.clone(),
                last_updated_at: record.last_updated_at.clone(),
            }),
        };
        wire::serialize_start_cost_allocation_tag_backfill_response(&response)
    }

    // =========================================================================
    // Commitment / savings generation (stateless mock)
    // =========================================================================

    // STUB[no-telemetry]: Commitment purchase analyses require real usage and commitment data; always returns empty list.
    async fn handle_list_commitment_purchase_analyses(&self, _body: &[u8]) -> MockResponse {
        let response = wire::ListCommitmentPurchaseAnalysesResponse {
            analysis_summary_list: Some(vec![]),
            next_page_token: None,
        };
        wire::serialize_list_commitment_purchase_analyses_response(&response)
    }

    // STUB[no-telemetry]: Savings Plans purchase recommendation generation history requires real usage data; always returns empty list.
    async fn handle_list_savings_plans_purchase_recommendation_generation(
        &self,
        _body: &[u8],
    ) -> MockResponse {
        let response = wire::ListSavingsPlansPurchaseRecommendationGenerationResponse {
            generation_summary_list: Some(vec![]),
            next_page_token: None,
        };
        wire::serialize_list_savings_plans_purchase_recommendation_generation_response(&response)
    }

    // STUB[no-telemetry]: Commitment purchase analysis requires real commitment and usage data; returns a synthetic analysis ID.
    async fn handle_start_commitment_purchase_analysis(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_start_commitment_purchase_analysis_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input
            .commitment_purchase_analysis_configuration
            .savings_plans_purchase_analysis_configuration
            .is_none()
        {
            return json_error_response(
                400,
                "ValidationException",
                "CommitmentPurchaseAnalysisConfiguration is required",
            );
        }

        let analysis_id = format!(
            "analysis-mock-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        );
        let response = wire::StartCommitmentPurchaseAnalysisResponse {
            analysis_id: Some(analysis_id),
            analysis_started_time: Some("2024-01-01T00:00:00Z".to_string()),
            estimated_completion_time: Some("2024-01-01T01:00:00Z".to_string()),
        };
        wire::serialize_start_commitment_purchase_analysis_response(&response)
    }

    // STUB[no-telemetry]: Savings Plans purchase recommendation generation requires real usage data; returns empty job status.
    async fn handle_start_savings_plans_purchase_recommendation_generation(&self) -> MockResponse {
        let response = wire::StartSavingsPlansPurchaseRecommendationGenerationResponse {
            ..Default::default()
        };
        wire::serialize_start_savings_plans_purchase_recommendation_generation_response(&response)
    }
}

// --- Utility functions ---

fn ce_error_response(err: &CostExplorerError) -> MockResponse {
    let (status, error_type) = match err {
        CostExplorerError::CostCategoryAlreadyExists(_) => (400, "ResourceExistsException"),
        CostExplorerError::CostCategoryNotFound(_) => (404, "ResourceNotFoundException"),
        CostExplorerError::UnknownMonitor(_) => (404, "UnknownMonitorException"),
        CostExplorerError::UnknownSubscription(_) => (404, "UnknownSubscriptionException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
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
