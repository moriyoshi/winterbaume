//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-lexmodelsv2

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_batch_create_custom_vocabulary_item_response(
    result: &BatchCreateCustomVocabularyItemResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_delete_custom_vocabulary_item_response(
    result: &BatchDeleteCustomVocabularyItemResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_custom_vocabulary_item_response(
    result: &BatchUpdateCustomVocabularyItemResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_build_bot_locale_response(result: &BuildBotLocaleResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_bot_response(result: &CreateBotResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_bot_alias_response(result: &CreateBotAliasResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_bot_locale_response(result: &CreateBotLocaleResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_bot_replica_response(result: &CreateBotReplicaResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_bot_version_response(result: &CreateBotVersionResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_export_response(result: &CreateExportResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_intent_response(result: &CreateIntentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_policy_response(
    result: &CreateResourcePolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_policy_statement_response(
    result: &CreateResourcePolicyStatementResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_slot_response(result: &CreateSlotResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_slot_type_response(result: &CreateSlotTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_test_set_discrepancy_report_response(
    result: &CreateTestSetDiscrepancyReportResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_upload_url_response(result: &CreateUploadUrlResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_bot_response(result: &DeleteBotResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_bot_alias_response(result: &DeleteBotAliasResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_bot_analyzer_recommendation_response(
    result: &DeleteBotAnalyzerRecommendationResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_bot_locale_response(result: &DeleteBotLocaleResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_bot_replica_response(result: &DeleteBotReplicaResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_bot_version_response(result: &DeleteBotVersionResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_custom_vocabulary_response(
    result: &DeleteCustomVocabularyResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_export_response(result: &DeleteExportResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_import_response(result: &DeleteImportResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_intent_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_policy_statement_response(
    result: &DeleteResourcePolicyStatementResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_slot_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_slot_type_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_test_set_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_utterances_response(result: &DeleteUtterancesResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_response(result: &DescribeBotResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_alias_response(result: &DescribeBotAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_analyzer_recommendation_response(
    result: &DescribeBotAnalyzerRecommendationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_locale_response(result: &DescribeBotLocaleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_recommendation_response(
    result: &DescribeBotRecommendationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_replica_response(
    result: &DescribeBotReplicaResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_resource_generation_response(
    result: &DescribeBotResourceGenerationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bot_version_response(
    result: &DescribeBotVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_custom_vocabulary_metadata_response(
    result: &DescribeCustomVocabularyMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_export_response(result: &DescribeExportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_import_response(result: &DescribeImportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_intent_response(result: &DescribeIntentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_resource_policy_response(
    result: &DescribeResourcePolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_slot_response(result: &DescribeSlotResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_slot_type_response(result: &DescribeSlotTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_test_execution_response(
    result: &DescribeTestExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_test_set_response(result: &DescribeTestSetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_test_set_discrepancy_report_response(
    result: &DescribeTestSetDiscrepancyReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_test_set_generation_response(
    result: &DescribeTestSetGenerationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_generate_bot_element_response(
    result: &GenerateBotElementResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_test_execution_artifacts_url_response(
    result: &GetTestExecutionArtifactsUrlResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_aggregated_utterances_response(
    result: &ListAggregatedUtterancesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_alias_replicas_response(
    result: &ListBotAliasReplicasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_aliases_response(result: &ListBotAliasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_analyzer_history_response(
    result: &ListBotAnalyzerHistoryResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_locales_response(result: &ListBotLocalesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_recommendations_response(
    result: &ListBotRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_replicas_response(result: &ListBotReplicasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_resource_generations_response(
    result: &ListBotResourceGenerationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_version_replicas_response(
    result: &ListBotVersionReplicasResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bot_versions_response(result: &ListBotVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bots_response(result: &ListBotsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_built_in_intents_response(
    result: &ListBuiltInIntentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_built_in_slot_types_response(
    result: &ListBuiltInSlotTypesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_custom_vocabulary_items_response(
    result: &ListCustomVocabularyItemsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_exports_response(result: &ListExportsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_imports_response(result: &ListImportsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_intent_metrics_response(result: &ListIntentMetricsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_intent_paths_response(result: &ListIntentPathsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_intent_stage_metrics_response(
    result: &ListIntentStageMetricsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_intents_response(result: &ListIntentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_recommended_intents_response(
    result: &ListRecommendedIntentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_session_analytics_data_response(
    result: &ListSessionAnalyticsDataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_session_metrics_response(
    result: &ListSessionMetricsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_slot_types_response(result: &ListSlotTypesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_slots_response(result: &ListSlotsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_test_execution_result_items_response(
    result: &ListTestExecutionResultItemsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_test_executions_response(
    result: &ListTestExecutionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_test_set_records_response(
    result: &ListTestSetRecordsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_test_sets_response(result: &ListTestSetsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_utterance_analytics_data_response(
    result: &ListUtteranceAnalyticsDataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_utterance_metrics_response(
    result: &ListUtteranceMetricsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_associated_transcripts_response(
    result: &SearchAssociatedTranscriptsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_bot_analyzer_response(result: &StartBotAnalyzerResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_bot_recommendation_response(
    result: &StartBotRecommendationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_bot_resource_generation_response(
    result: &StartBotResourceGenerationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_import_response(result: &StartImportResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_test_execution_response(
    result: &StartTestExecutionResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_test_set_generation_response(
    result: &StartTestSetGenerationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_bot_analyzer_response(result: &StopBotAnalyzerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_bot_recommendation_response(
    result: &StopBotRecommendationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bot_response(result: &UpdateBotResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bot_alias_response(result: &UpdateBotAliasResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bot_locale_response(result: &UpdateBotLocaleResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bot_recommendation_response(
    result: &UpdateBotRecommendationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_export_response(result: &UpdateExportResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_intent_response(result: &UpdateIntentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_policy_response(
    result: &UpdateResourcePolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_slot_response(result: &UpdateSlotResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_slot_type_response(result: &UpdateSlotTypeResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_test_set_response(result: &UpdateTestSetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_create_custom_vocabulary_item_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchCreateCustomVocabularyItemRequest, String> {
    let mut input = BatchCreateCustomVocabularyItemRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchCreateCustomVocabularyItemRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchCreateCustomVocabularyItem request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_custom_vocabulary_item_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteCustomVocabularyItemRequest, String> {
    let mut input = BatchDeleteCustomVocabularyItemRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteCustomVocabularyItemRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchDeleteCustomVocabularyItem request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_custom_vocabulary_item_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateCustomVocabularyItemRequest, String> {
    let mut input = BatchUpdateCustomVocabularyItemRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateCustomVocabularyItemRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchUpdateCustomVocabularyItem request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_build_bot_locale_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BuildBotLocaleRequest, String> {
    let mut input = BuildBotLocaleRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBotRequest, String> {
    let mut input = CreateBotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBot request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_bot_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBotAliasRequest, String> {
    let mut input = CreateBotAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBotAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBotAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_bot_locale_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBotLocaleRequest, String> {
    let mut input = CreateBotLocaleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBotLocaleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBotLocale request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_bot_replica_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBotReplicaRequest, String> {
    let mut input = CreateBotReplicaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBotReplicaRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBotReplica request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_bot_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBotVersionRequest, String> {
    let mut input = CreateBotVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBotVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBotVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateExportRequest, String> {
    let mut input = CreateExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateExport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_intent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIntentRequest, String> {
    let mut input = CreateIntentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIntentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateIntent request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourcePolicyRequest, String> {
    let mut input = CreateResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateResourcePolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_policy_statement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourcePolicyStatementRequest, String> {
    let mut input = CreateResourcePolicyStatementRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourcePolicyStatementRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateResourcePolicyStatement request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedRevisionId") {
        input.expected_revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_slot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSlotRequest, String> {
    let mut input = CreateSlotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSlotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSlot request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_slot_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSlotTypeRequest, String> {
    let mut input = CreateSlotTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSlotTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSlotType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_test_set_discrepancy_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTestSetDiscrepancyReportRequest, String> {
    let mut input = CreateTestSetDiscrepancyReportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTestSetDiscrepancyReportRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateTestSetDiscrepancyReport request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "testSetId" => {
                input.test_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_upload_url_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUploadUrlRequest, String> {
    let input = CreateUploadUrlRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBotRequest, String> {
    let mut input = DeleteBotRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_bot_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBotAliasRequest, String> {
    let mut input = DeleteBotAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "botAliasId" => {
                input.bot_alias_id = value.to_string();
            }
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_bot_analyzer_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBotAnalyzerRecommendationRequest, String> {
    let mut input = DeleteBotAnalyzerRecommendationRequest::default();
    for (name, value) in labels {
        match *name {
            "botAnalyzerRequestId" => {
                input.bot_analyzer_request_id = value.to_string();
            }
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_bot_locale_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBotLocaleRequest, String> {
    let mut input = DeleteBotLocaleRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_bot_replica_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBotReplicaRequest, String> {
    let mut input = DeleteBotReplicaRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "replicaRegion" => {
                input.replica_region = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_bot_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBotVersionRequest, String> {
    let mut input = DeleteBotVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_custom_vocabulary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomVocabularyRequest, String> {
    let mut input = DeleteCustomVocabularyRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteExportRequest, String> {
    let mut input = DeleteExportRequest::default();
    for (name, value) in labels {
        match *name {
            "exportId" => {
                input.export_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_import_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteImportRequest, String> {
    let mut input = DeleteImportRequest::default();
    for (name, value) in labels {
        match *name {
            "importId" => {
                input.import_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_intent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIntentRequest, String> {
    let mut input = DeleteIntentRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyRequest, String> {
    let mut input = DeleteResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedRevisionId") {
        input.expected_revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_policy_statement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyStatementRequest, String> {
    let mut input = DeleteResourcePolicyStatementRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            "statementId" => {
                input.statement_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedRevisionId") {
        input.expected_revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_slot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSlotRequest, String> {
    let mut input = DeleteSlotRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            "slotId" => {
                input.slot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_slot_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSlotTypeRequest, String> {
    let mut input = DeleteSlotTypeRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            "slotTypeId" => {
                input.slot_type_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_test_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTestSetRequest, String> {
    let mut input = DeleteTestSetRequest::default();
    for (name, value) in labels {
        match *name {
            "testSetId" => {
                input.test_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_utterances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUtterancesRequest, String> {
    let mut input = DeleteUtterancesRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("localeId") {
        input.locale_id = Some(value.to_string());
    }
    if let Some(value) = query.get("sessionId") {
        input.session_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotRequest, String> {
    let mut input = DescribeBotRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotAliasRequest, String> {
    let mut input = DescribeBotAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "botAliasId" => {
                input.bot_alias_id = value.to_string();
            }
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_analyzer_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotAnalyzerRecommendationRequest, String> {
    let mut input = DescribeBotAnalyzerRecommendationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeBotAnalyzerRecommendationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize DescribeBotAnalyzerRecommendation request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "botAnalyzerRequestId" => {
                input.bot_analyzer_request_id = value.to_string();
            }
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_locale_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotLocaleRequest, String> {
    let mut input = DescribeBotLocaleRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotRecommendationRequest, String> {
    let mut input = DescribeBotRecommendationRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botRecommendationId" => {
                input.bot_recommendation_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_replica_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotReplicaRequest, String> {
    let mut input = DescribeBotReplicaRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "replicaRegion" => {
                input.replica_region = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_resource_generation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotResourceGenerationRequest, String> {
    let mut input = DescribeBotResourceGenerationRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "generationId" => {
                input.generation_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bot_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBotVersionRequest, String> {
    let mut input = DescribeBotVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_custom_vocabulary_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCustomVocabularyMetadataRequest, String> {
    let mut input = DescribeCustomVocabularyMetadataRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeExportRequest, String> {
    let mut input = DescribeExportRequest::default();
    for (name, value) in labels {
        match *name {
            "exportId" => {
                input.export_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_import_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeImportRequest, String> {
    let mut input = DescribeImportRequest::default();
    for (name, value) in labels {
        match *name {
            "importId" => {
                input.import_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_intent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeIntentRequest, String> {
    let mut input = DescribeIntentRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeResourcePolicyRequest, String> {
    let mut input = DescribeResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_slot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSlotRequest, String> {
    let mut input = DescribeSlotRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            "slotId" => {
                input.slot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_slot_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSlotTypeRequest, String> {
    let mut input = DescribeSlotTypeRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            "slotTypeId" => {
                input.slot_type_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_test_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTestExecutionRequest, String> {
    let mut input = DescribeTestExecutionRequest::default();
    for (name, value) in labels {
        match *name {
            "testExecutionId" => {
                input.test_execution_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_test_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTestSetRequest, String> {
    let mut input = DescribeTestSetRequest::default();
    for (name, value) in labels {
        match *name {
            "testSetId" => {
                input.test_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_test_set_discrepancy_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTestSetDiscrepancyReportRequest, String> {
    let mut input = DescribeTestSetDiscrepancyReportRequest::default();
    for (name, value) in labels {
        match *name {
            "testSetDiscrepancyReportId" => {
                input.test_set_discrepancy_report_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_test_set_generation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTestSetGenerationRequest, String> {
    let mut input = DescribeTestSetGenerationRequest::default();
    for (name, value) in labels {
        match *name {
            "testSetGenerationId" => {
                input.test_set_generation_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_bot_element_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateBotElementRequest, String> {
    let mut input = GenerateBotElementRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GenerateBotElementRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GenerateBotElement request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_test_execution_artifacts_url_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTestExecutionArtifactsUrlRequest, String> {
    let mut input = GetTestExecutionArtifactsUrlRequest::default();
    for (name, value) in labels {
        match *name {
            "testExecutionId" => {
                input.test_execution_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_aggregated_utterances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAggregatedUtterancesRequest, String> {
    let mut input = ListAggregatedUtterancesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAggregatedUtterancesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListAggregatedUtterances request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_alias_replicas_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotAliasReplicasRequest, String> {
    let mut input = ListBotAliasReplicasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotAliasReplicasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBotAliasReplicas request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "replicaRegion" => {
                input.replica_region = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotAliasesRequest, String> {
    let mut input = ListBotAliasesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotAliasesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBotAliases request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_analyzer_history_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotAnalyzerHistoryRequest, String> {
    let mut input = ListBotAnalyzerHistoryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotAnalyzerHistoryRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListBotAnalyzerHistory request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_locales_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotLocalesRequest, String> {
    let mut input = ListBotLocalesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotLocalesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBotLocales request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotRecommendationsRequest, String> {
    let mut input = ListBotRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotRecommendationsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListBotRecommendations request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_replicas_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotReplicasRequest, String> {
    let mut input = ListBotReplicasRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_resource_generations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotResourceGenerationsRequest, String> {
    let mut input = ListBotResourceGenerationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotResourceGenerationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListBotResourceGenerations request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_version_replicas_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotVersionReplicasRequest, String> {
    let mut input = ListBotVersionReplicasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotVersionReplicasRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListBotVersionReplicas request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "replicaRegion" => {
                input.replica_region = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bot_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotVersionsRequest, String> {
    let mut input = ListBotVersionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotVersionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBotVersions request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bots_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBotsRequest, String> {
    let mut input = ListBotsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBotsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBots request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_built_in_intents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBuiltInIntentsRequest, String> {
    let mut input = ListBuiltInIntentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBuiltInIntentsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBuiltInIntents request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_built_in_slot_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBuiltInSlotTypesRequest, String> {
    let mut input = ListBuiltInSlotTypesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBuiltInSlotTypesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBuiltInSlotTypes request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_custom_vocabulary_items_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCustomVocabularyItemsRequest, String> {
    let mut input = ListCustomVocabularyItemsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCustomVocabularyItemsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListCustomVocabularyItems request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_exports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListExportsRequest, String> {
    let mut input = ListExportsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListExportsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListExports request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_imports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListImportsRequest, String> {
    let mut input = ListImportsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListImportsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListImports request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_intent_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIntentMetricsRequest, String> {
    let mut input = ListIntentMetricsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIntentMetricsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListIntentMetrics request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_intent_paths_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIntentPathsRequest, String> {
    let mut input = ListIntentPathsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIntentPathsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListIntentPaths request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_intent_stage_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIntentStageMetricsRequest, String> {
    let mut input = ListIntentStageMetricsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIntentStageMetricsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListIntentStageMetrics request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_intents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIntentsRequest, String> {
    let mut input = ListIntentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIntentsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListIntents request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_recommended_intents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRecommendedIntentsRequest, String> {
    let mut input = ListRecommendedIntentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRecommendedIntentsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListRecommendedIntents request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botRecommendationId" => {
                input.bot_recommendation_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_session_analytics_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSessionAnalyticsDataRequest, String> {
    let mut input = ListSessionAnalyticsDataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSessionAnalyticsDataRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListSessionAnalyticsData request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_session_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSessionMetricsRequest, String> {
    let mut input = ListSessionMetricsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSessionMetricsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListSessionMetrics request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_slot_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSlotTypesRequest, String> {
    let mut input = ListSlotTypesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSlotTypesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListSlotTypes request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_slots_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSlotsRequest, String> {
    let mut input = ListSlotsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSlotsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListSlots request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceARN" => {
                input.resource_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_execution_result_items_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestExecutionResultItemsRequest, String> {
    let mut input = ListTestExecutionResultItemsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTestExecutionResultItemsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListTestExecutionResultItems request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "testExecutionId" => {
                input.test_execution_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_executions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestExecutionsRequest, String> {
    let mut input = ListTestExecutionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTestExecutionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTestExecutions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_set_records_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestSetRecordsRequest, String> {
    let mut input = ListTestSetRecordsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTestSetRecordsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTestSetRecords request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "testSetId" => {
                input.test_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_test_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTestSetsRequest, String> {
    let mut input = ListTestSetsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTestSetsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTestSets request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_utterance_analytics_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUtteranceAnalyticsDataRequest, String> {
    let mut input = ListUtteranceAnalyticsDataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListUtteranceAnalyticsDataRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListUtteranceAnalyticsData request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_utterance_metrics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUtteranceMetricsRequest, String> {
    let mut input = ListUtteranceMetricsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListUtteranceMetricsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListUtteranceMetrics request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_associated_transcripts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchAssociatedTranscriptsRequest, String> {
    let mut input = SearchAssociatedTranscriptsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchAssociatedTranscriptsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize SearchAssociatedTranscripts request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botRecommendationId" => {
                input.bot_recommendation_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_bot_analyzer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartBotAnalyzerRequest, String> {
    let mut input = StartBotAnalyzerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartBotAnalyzerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartBotAnalyzer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_bot_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartBotRecommendationRequest, String> {
    let mut input = StartBotRecommendationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartBotRecommendationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartBotRecommendation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_bot_resource_generation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartBotResourceGenerationRequest, String> {
    let mut input = StartBotResourceGenerationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartBotResourceGenerationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartBotResourceGeneration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_import_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartImportRequest, String> {
    let mut input = StartImportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartImportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartImport request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_test_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartTestExecutionRequest, String> {
    let mut input = StartTestExecutionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartTestExecutionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartTestExecution request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "testSetId" => {
                input.test_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_test_set_generation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartTestSetGenerationRequest, String> {
    let mut input = StartTestSetGenerationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartTestSetGenerationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartTestSetGeneration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_bot_analyzer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopBotAnalyzerRequest, String> {
    let mut input = StopBotAnalyzerRequest::default();
    for (name, value) in labels {
        match *name {
            "botAnalyzerRequestId" => {
                input.bot_analyzer_request_id = value.to_string();
            }
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_bot_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopBotRecommendationRequest, String> {
    let mut input = StopBotRecommendationRequest::default();
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botRecommendationId" => {
                input.bot_recommendation_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceARN" => {
                input.resource_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceARN" => {
                input.resource_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_bot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBotRequest, String> {
    let mut input = UpdateBotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBot request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_bot_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBotAliasRequest, String> {
    let mut input = UpdateBotAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBotAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBotAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botAliasId" => {
                input.bot_alias_id = value.to_string();
            }
            "botId" => {
                input.bot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_bot_locale_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBotLocaleRequest, String> {
    let mut input = UpdateBotLocaleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBotLocaleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBotLocale request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_bot_recommendation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBotRecommendationRequest, String> {
    let mut input = UpdateBotRecommendationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBotRecommendationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateBotRecommendation request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botRecommendationId" => {
                input.bot_recommendation_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateExportRequest, String> {
    let mut input = UpdateExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateExport request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "exportId" => {
                input.export_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_intent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIntentRequest, String> {
    let mut input = UpdateIntentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIntentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateIntent request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourcePolicyRequest, String> {
    let mut input = UpdateResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateResourcePolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("expectedRevisionId") {
        input.expected_revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_slot_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSlotRequest, String> {
    let mut input = UpdateSlotRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSlotRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSlot request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "intentId" => {
                input.intent_id = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            "slotId" => {
                input.slot_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_slot_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSlotTypeRequest, String> {
    let mut input = UpdateSlotTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSlotTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSlotType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "botId" => {
                input.bot_id = value.to_string();
            }
            "botVersion" => {
                input.bot_version = value.to_string();
            }
            "localeId" => {
                input.locale_id = value.to_string();
            }
            "slotTypeId" => {
                input.slot_type_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_test_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTestSetRequest, String> {
    let mut input = UpdateTestSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTestSetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTestSet request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "testSetId" => {
                input.test_set_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
