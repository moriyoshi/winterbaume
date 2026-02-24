//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudwatchlogs

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for awsJson protocol.
pub fn serialize_associate_kms_key_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_source_to_s3_table_integration_response(
    result: &AssociateSourceToS3TableIntegrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_cancel_export_task_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_import_task_response(result: &CancelImportTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_delivery_response(result: &CreateDeliveryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_export_task_response(result: &CreateExportTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_import_task_response(result: &CreateImportTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_log_anomaly_detector_response(
    result: &CreateLogAnomalyDetectorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_create_log_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_create_log_stream_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_lookup_table_response(result: &CreateLookupTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_scheduled_query_response(
    result: &CreateScheduledQueryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_account_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_data_protection_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_delivery_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_delivery_destination_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_delivery_destination_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_delivery_source_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_destination_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_index_policy_response(result: &DeleteIndexPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_integration_response(result: &DeleteIntegrationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_log_anomaly_detector_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_log_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_log_stream_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_lookup_table_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_metric_filter_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_query_definition_response(
    result: &DeleteQueryDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_resource_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_retention_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_scheduled_query_response(
    result: &DeleteScheduledQueryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_subscription_filter_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_transformer_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_policies_response(
    result: &DescribeAccountPoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_configuration_templates_response(
    result: &DescribeConfigurationTemplatesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_deliveries_response(result: &DescribeDeliveriesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_delivery_destinations_response(
    result: &DescribeDeliveryDestinationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_delivery_sources_response(
    result: &DescribeDeliverySourcesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_destinations_response(
    result: &DescribeDestinationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_export_tasks_response(
    result: &DescribeExportTasksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_field_indexes_response(
    result: &DescribeFieldIndexesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_import_task_batches_response(
    result: &DescribeImportTaskBatchesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_import_tasks_response(
    result: &DescribeImportTasksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_index_policies_response(
    result: &DescribeIndexPoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_log_groups_response(result: &DescribeLogGroupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_log_streams_response(
    result: &DescribeLogStreamsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_lookup_tables_response(
    result: &DescribeLookupTablesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metric_filters_response(
    result: &DescribeMetricFiltersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_queries_response(result: &DescribeQueriesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_query_definitions_response(
    result: &DescribeQueryDefinitionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_resource_policies_response(
    result: &DescribeResourcePoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_subscription_filters_response(
    result: &DescribeSubscriptionFiltersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disassociate_kms_key_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_source_from_s3_table_integration_response(
    result: &DisassociateSourceFromS3TableIntegrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_filter_log_events_response(result: &FilterLogEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_protection_policy_response(
    result: &GetDataProtectionPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_delivery_response(result: &GetDeliveryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_delivery_destination_response(
    result: &GetDeliveryDestinationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_delivery_destination_policy_response(
    result: &GetDeliveryDestinationPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_delivery_source_response(result: &GetDeliverySourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_integration_response(result: &GetIntegrationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_log_anomaly_detector_response(
    result: &GetLogAnomalyDetectorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_log_events_response(result: &GetLogEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_log_fields_response(result: &GetLogFieldsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_log_group_fields_response(result: &GetLogGroupFieldsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_log_object_response(result: &GetLogObjectResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_log_record_response(result: &GetLogRecordResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_lookup_table_response(result: &GetLookupTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_query_results_response(result: &GetQueryResultsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_scheduled_query_response(result: &GetScheduledQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_scheduled_query_history_response(
    result: &GetScheduledQueryHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_transformer_response(result: &GetTransformerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_aggregate_log_group_summaries_response(
    result: &ListAggregateLogGroupSummariesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_anomalies_response(result: &ListAnomaliesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_integrations_response(result: &ListIntegrationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_log_anomaly_detectors_response(
    result: &ListLogAnomalyDetectorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_log_groups_response(result: &ListLogGroupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_log_groups_for_query_response(
    result: &ListLogGroupsForQueryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_scheduled_queries_response(
    result: &ListScheduledQueriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_sources_for_s3_table_integration_response(
    result: &ListSourcesForS3TableIntegrationResponse,
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
pub fn serialize_list_tags_log_group_response(result: &ListTagsLogGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_account_policy_response(result: &PutAccountPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_bearer_token_authentication_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_data_protection_policy_response(
    result: &PutDataProtectionPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_delivery_destination_response(
    result: &PutDeliveryDestinationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_delivery_destination_policy_response(
    result: &PutDeliveryDestinationPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_delivery_source_response(result: &PutDeliverySourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_destination_response(result: &PutDestinationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_destination_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_index_policy_response(result: &PutIndexPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_integration_response(result: &PutIntegrationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_log_events_response(result: &PutLogEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_log_group_deletion_protection_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_metric_filter_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_query_definition_response(
    result: &PutQueryDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_retention_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_subscription_filter_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_transformer_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_live_tail_response(result: &StartLiveTailResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_query_response(result: &StartQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_query_response(result: &StopQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_log_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_metric_filter_response(result: &TestMetricFilterResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_transformer_response(result: &TestTransformerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_log_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_anomaly_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_delivery_configuration_response(
    result: &UpdateDeliveryConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_log_anomaly_detector_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_lookup_table_response(result: &UpdateLookupTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_scheduled_query_response(
    result: &UpdateScheduledQueryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_kms_key_request(
    body: &[u8],
) -> Result<AssociateKmsKeyRequest, String> {
    if body.is_empty() {
        return Ok(AssociateKmsKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateKmsKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_source_to_s3_table_integration_request(
    body: &[u8],
) -> Result<AssociateSourceToS3TableIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(AssociateSourceToS3TableIntegrationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AssociateSourceToS3TableIntegration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_export_task_request(
    body: &[u8],
) -> Result<CancelExportTaskRequest, String> {
    if body.is_empty() {
        return Ok(CancelExportTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelExportTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_import_task_request(
    body: &[u8],
) -> Result<CancelImportTaskRequest, String> {
    if body.is_empty() {
        return Ok(CancelImportTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelImportTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_delivery_request(body: &[u8]) -> Result<CreateDeliveryRequest, String> {
    if body.is_empty() {
        return Ok(CreateDeliveryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDelivery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_export_task_request(
    body: &[u8],
) -> Result<CreateExportTaskRequest, String> {
    if body.is_empty() {
        return Ok(CreateExportTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateExportTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_import_task_request(
    body: &[u8],
) -> Result<CreateImportTaskRequest, String> {
    if body.is_empty() {
        return Ok(CreateImportTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateImportTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_log_anomaly_detector_request(
    body: &[u8],
) -> Result<CreateLogAnomalyDetectorRequest, String> {
    if body.is_empty() {
        return Ok(CreateLogAnomalyDetectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLogAnomalyDetector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_log_group_request(body: &[u8]) -> Result<CreateLogGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateLogGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLogGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_log_stream_request(
    body: &[u8],
) -> Result<CreateLogStreamRequest, String> {
    if body.is_empty() {
        return Ok(CreateLogStreamRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLogStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_lookup_table_request(
    body: &[u8],
) -> Result<CreateLookupTableRequest, String> {
    if body.is_empty() {
        return Ok(CreateLookupTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLookupTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_scheduled_query_request(
    body: &[u8],
) -> Result<CreateScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(CreateScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateScheduledQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_account_policy_request(
    body: &[u8],
) -> Result<DeleteAccountPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAccountPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAccountPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_data_protection_policy_request(
    body: &[u8],
) -> Result<DeleteDataProtectionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDataProtectionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataProtectionPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_delivery_request(body: &[u8]) -> Result<DeleteDeliveryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDeliveryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDelivery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_delivery_destination_request(
    body: &[u8],
) -> Result<DeleteDeliveryDestinationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDeliveryDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDeliveryDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_delivery_destination_policy_request(
    body: &[u8],
) -> Result<DeleteDeliveryDestinationPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDeliveryDestinationPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDeliveryDestinationPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_delivery_source_request(
    body: &[u8],
) -> Result<DeleteDeliverySourceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDeliverySourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDeliverySource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_destination_request(
    body: &[u8],
) -> Result<DeleteDestinationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_index_policy_request(
    body: &[u8],
) -> Result<DeleteIndexPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIndexPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIndexPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_integration_request(
    body: &[u8],
) -> Result<DeleteIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIntegrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIntegration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_log_anomaly_detector_request(
    body: &[u8],
) -> Result<DeleteLogAnomalyDetectorRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLogAnomalyDetectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLogAnomalyDetector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_log_group_request(body: &[u8]) -> Result<DeleteLogGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLogGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLogGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_log_stream_request(
    body: &[u8],
) -> Result<DeleteLogStreamRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLogStreamRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLogStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_lookup_table_request(
    body: &[u8],
) -> Result<DeleteLookupTableRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLookupTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLookupTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_metric_filter_request(
    body: &[u8],
) -> Result<DeleteMetricFilterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMetricFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMetricFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_query_definition_request(
    body: &[u8],
) -> Result<DeleteQueryDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteQueryDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteQueryDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_retention_policy_request(
    body: &[u8],
) -> Result<DeleteRetentionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRetentionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRetentionPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_scheduled_query_request(
    body: &[u8],
) -> Result<DeleteScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteScheduledQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_subscription_filter_request(
    body: &[u8],
) -> Result<DeleteSubscriptionFilterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSubscriptionFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSubscriptionFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_transformer_request(
    body: &[u8],
) -> Result<DeleteTransformerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTransformerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTransformer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_policies_request(
    body: &[u8],
) -> Result<DescribeAccountPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAccountPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccountPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_configuration_templates_request(
    body: &[u8],
) -> Result<DescribeConfigurationTemplatesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigurationTemplatesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConfigurationTemplates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_deliveries_request(
    body: &[u8],
) -> Result<DescribeDeliveriesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDeliveriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDeliveries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_delivery_destinations_request(
    body: &[u8],
) -> Result<DescribeDeliveryDestinationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDeliveryDestinationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDeliveryDestinations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_delivery_sources_request(
    body: &[u8],
) -> Result<DescribeDeliverySourcesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDeliverySourcesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDeliverySources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_destinations_request(
    body: &[u8],
) -> Result<DescribeDestinationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDestinationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDestinations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_export_tasks_request(
    body: &[u8],
) -> Result<DescribeExportTasksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeExportTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExportTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_field_indexes_request(
    body: &[u8],
) -> Result<DescribeFieldIndexesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFieldIndexesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFieldIndexes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_import_task_batches_request(
    body: &[u8],
) -> Result<DescribeImportTaskBatchesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeImportTaskBatchesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImportTaskBatches request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_import_tasks_request(
    body: &[u8],
) -> Result<DescribeImportTasksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeImportTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImportTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_index_policies_request(
    body: &[u8],
) -> Result<DescribeIndexPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeIndexPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeIndexPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_log_groups_request(
    body: &[u8],
) -> Result<DescribeLogGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLogGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLogGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_log_streams_request(
    body: &[u8],
) -> Result<DescribeLogStreamsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLogStreamsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLogStreams request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_lookup_tables_request(
    body: &[u8],
) -> Result<DescribeLookupTablesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLookupTablesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLookupTables request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metric_filters_request(
    body: &[u8],
) -> Result<DescribeMetricFiltersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeMetricFiltersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetricFilters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_queries_request(body: &[u8]) -> Result<DescribeQueriesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeQueriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeQueries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_query_definitions_request(
    body: &[u8],
) -> Result<DescribeQueryDefinitionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeQueryDefinitionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeQueryDefinitions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_resource_policies_request(
    body: &[u8],
) -> Result<DescribeResourcePoliciesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeResourcePoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeResourcePolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_subscription_filters_request(
    body: &[u8],
) -> Result<DescribeSubscriptionFiltersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSubscriptionFiltersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSubscriptionFilters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_kms_key_request(
    body: &[u8],
) -> Result<DisassociateKmsKeyRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateKmsKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateKmsKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_source_from_s3_table_integration_request(
    body: &[u8],
) -> Result<DisassociateSourceFromS3TableIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateSourceFromS3TableIntegrationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociateSourceFromS3TableIntegration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_filter_log_events_request(
    body: &[u8],
) -> Result<FilterLogEventsRequest, String> {
    if body.is_empty() {
        return Ok(FilterLogEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize FilterLogEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_protection_policy_request(
    body: &[u8],
) -> Result<GetDataProtectionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetDataProtectionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataProtectionPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_delivery_request(body: &[u8]) -> Result<GetDeliveryRequest, String> {
    if body.is_empty() {
        return Ok(GetDeliveryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDelivery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_delivery_destination_request(
    body: &[u8],
) -> Result<GetDeliveryDestinationRequest, String> {
    if body.is_empty() {
        return Ok(GetDeliveryDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeliveryDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_delivery_destination_policy_request(
    body: &[u8],
) -> Result<GetDeliveryDestinationPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetDeliveryDestinationPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeliveryDestinationPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_delivery_source_request(
    body: &[u8],
) -> Result<GetDeliverySourceRequest, String> {
    if body.is_empty() {
        return Ok(GetDeliverySourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDeliverySource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_integration_request(body: &[u8]) -> Result<GetIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(GetIntegrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetIntegration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_log_anomaly_detector_request(
    body: &[u8],
) -> Result<GetLogAnomalyDetectorRequest, String> {
    if body.is_empty() {
        return Ok(GetLogAnomalyDetectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLogAnomalyDetector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_log_events_request(body: &[u8]) -> Result<GetLogEventsRequest, String> {
    if body.is_empty() {
        return Ok(GetLogEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLogEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_log_fields_request(body: &[u8]) -> Result<GetLogFieldsRequest, String> {
    if body.is_empty() {
        return Ok(GetLogFieldsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLogFields request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_log_group_fields_request(
    body: &[u8],
) -> Result<GetLogGroupFieldsRequest, String> {
    if body.is_empty() {
        return Ok(GetLogGroupFieldsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLogGroupFields request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_log_object_request(body: &[u8]) -> Result<GetLogObjectRequest, String> {
    if body.is_empty() {
        return Ok(GetLogObjectRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLogObject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_log_record_request(body: &[u8]) -> Result<GetLogRecordRequest, String> {
    if body.is_empty() {
        return Ok(GetLogRecordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLogRecord request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_lookup_table_request(body: &[u8]) -> Result<GetLookupTableRequest, String> {
    if body.is_empty() {
        return Ok(GetLookupTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLookupTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_query_results_request(
    body: &[u8],
) -> Result<GetQueryResultsRequest, String> {
    if body.is_empty() {
        return Ok(GetQueryResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQueryResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_scheduled_query_request(
    body: &[u8],
) -> Result<GetScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(GetScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetScheduledQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_scheduled_query_history_request(
    body: &[u8],
) -> Result<GetScheduledQueryHistoryRequest, String> {
    if body.is_empty() {
        return Ok(GetScheduledQueryHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetScheduledQueryHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_transformer_request(body: &[u8]) -> Result<GetTransformerRequest, String> {
    if body.is_empty() {
        return Ok(GetTransformerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTransformer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_aggregate_log_group_summaries_request(
    body: &[u8],
) -> Result<ListAggregateLogGroupSummariesRequest, String> {
    if body.is_empty() {
        return Ok(ListAggregateLogGroupSummariesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAggregateLogGroupSummaries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_anomalies_request(body: &[u8]) -> Result<ListAnomaliesRequest, String> {
    if body.is_empty() {
        return Ok(ListAnomaliesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAnomalies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_integrations_request(
    body: &[u8],
) -> Result<ListIntegrationsRequest, String> {
    if body.is_empty() {
        return Ok(ListIntegrationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListIntegrations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_log_anomaly_detectors_request(
    body: &[u8],
) -> Result<ListLogAnomalyDetectorsRequest, String> {
    if body.is_empty() {
        return Ok(ListLogAnomalyDetectorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLogAnomalyDetectors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_log_groups_request(body: &[u8]) -> Result<ListLogGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListLogGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLogGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_log_groups_for_query_request(
    body: &[u8],
) -> Result<ListLogGroupsForQueryRequest, String> {
    if body.is_empty() {
        return Ok(ListLogGroupsForQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLogGroupsForQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_scheduled_queries_request(
    body: &[u8],
) -> Result<ListScheduledQueriesRequest, String> {
    if body.is_empty() {
        return Ok(ListScheduledQueriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListScheduledQueries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_sources_for_s3_table_integration_request(
    body: &[u8],
) -> Result<ListSourcesForS3TableIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(ListSourcesForS3TableIntegrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSourcesForS3TableIntegration request: {e}"))
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
pub fn deserialize_list_tags_log_group_request(
    body: &[u8],
) -> Result<ListTagsLogGroupRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsLogGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsLogGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_account_policy_request(
    body: &[u8],
) -> Result<PutAccountPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutAccountPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAccountPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_bearer_token_authentication_request(
    body: &[u8],
) -> Result<PutBearerTokenAuthenticationRequest, String> {
    if body.is_empty() {
        return Ok(PutBearerTokenAuthenticationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutBearerTokenAuthentication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_data_protection_policy_request(
    body: &[u8],
) -> Result<PutDataProtectionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutDataProtectionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDataProtectionPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_delivery_destination_request(
    body: &[u8],
) -> Result<PutDeliveryDestinationRequest, String> {
    if body.is_empty() {
        return Ok(PutDeliveryDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDeliveryDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_delivery_destination_policy_request(
    body: &[u8],
) -> Result<PutDeliveryDestinationPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutDeliveryDestinationPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDeliveryDestinationPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_delivery_source_request(
    body: &[u8],
) -> Result<PutDeliverySourceRequest, String> {
    if body.is_empty() {
        return Ok(PutDeliverySourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDeliverySource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_destination_request(body: &[u8]) -> Result<PutDestinationRequest, String> {
    if body.is_empty() {
        return Ok(PutDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_destination_policy_request(
    body: &[u8],
) -> Result<PutDestinationPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutDestinationPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDestinationPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_index_policy_request(body: &[u8]) -> Result<PutIndexPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutIndexPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutIndexPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_integration_request(body: &[u8]) -> Result<PutIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(PutIntegrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutIntegration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_log_events_request(body: &[u8]) -> Result<PutLogEventsRequest, String> {
    if body.is_empty() {
        return Ok(PutLogEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutLogEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_log_group_deletion_protection_request(
    body: &[u8],
) -> Result<PutLogGroupDeletionProtectionRequest, String> {
    if body.is_empty() {
        return Ok(PutLogGroupDeletionProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutLogGroupDeletionProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_metric_filter_request(
    body: &[u8],
) -> Result<PutMetricFilterRequest, String> {
    if body.is_empty() {
        return Ok(PutMetricFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutMetricFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_query_definition_request(
    body: &[u8],
) -> Result<PutQueryDefinitionRequest, String> {
    if body.is_empty() {
        return Ok(PutQueryDefinitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutQueryDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_retention_policy_request(
    body: &[u8],
) -> Result<PutRetentionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutRetentionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRetentionPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_subscription_filter_request(
    body: &[u8],
) -> Result<PutSubscriptionFilterRequest, String> {
    if body.is_empty() {
        return Ok(PutSubscriptionFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutSubscriptionFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_transformer_request(body: &[u8]) -> Result<PutTransformerRequest, String> {
    if body.is_empty() {
        return Ok(PutTransformerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutTransformer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_live_tail_request(body: &[u8]) -> Result<StartLiveTailRequest, String> {
    if body.is_empty() {
        return Ok(StartLiveTailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartLiveTail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_query_request(body: &[u8]) -> Result<StartQueryRequest, String> {
    if body.is_empty() {
        return Ok(StartQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_query_request(body: &[u8]) -> Result<StopQueryRequest, String> {
    if body.is_empty() {
        return Ok(StopQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_log_group_request(body: &[u8]) -> Result<TagLogGroupRequest, String> {
    if body.is_empty() {
        return Ok(TagLogGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagLogGroup request: {e}"))
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
pub fn deserialize_test_metric_filter_request(
    body: &[u8],
) -> Result<TestMetricFilterRequest, String> {
    if body.is_empty() {
        return Ok(TestMetricFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestMetricFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_test_transformer_request(body: &[u8]) -> Result<TestTransformerRequest, String> {
    if body.is_empty() {
        return Ok(TestTransformerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestTransformer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_log_group_request(body: &[u8]) -> Result<UntagLogGroupRequest, String> {
    if body.is_empty() {
        return Ok(UntagLogGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagLogGroup request: {e}"))
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
pub fn deserialize_update_anomaly_request(body: &[u8]) -> Result<UpdateAnomalyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAnomalyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAnomaly request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_delivery_configuration_request(
    body: &[u8],
) -> Result<UpdateDeliveryConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDeliveryConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDeliveryConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_log_anomaly_detector_request(
    body: &[u8],
) -> Result<UpdateLogAnomalyDetectorRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLogAnomalyDetectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLogAnomalyDetector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_lookup_table_request(
    body: &[u8],
) -> Result<UpdateLookupTableRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLookupTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLookupTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_scheduled_query_request(
    body: &[u8],
) -> Result<UpdateScheduledQueryRequest, String> {
    if body.is_empty() {
        return Ok(UpdateScheduledQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateScheduledQuery request: {e}"))
}
