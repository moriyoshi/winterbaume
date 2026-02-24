//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-config

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_resource_types_response(
    result: &AssociateResourceTypesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_aggregate_resource_config_response(
    result: &BatchGetAggregateResourceConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_resource_config_response(
    result: &BatchGetResourceConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_aggregation_authorization_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_config_rule_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_configuration_aggregator_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_configuration_recorder_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_conformance_pack_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_delivery_channel_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_evaluation_results_response(
    result: &DeleteEvaluationResultsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_organization_config_rule_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_organization_conformance_pack_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_pending_aggregation_request_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_remediation_configuration_response(
    result: &DeleteRemediationConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_remediation_exceptions_response(
    result: &DeleteRemediationExceptionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_resource_config_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_retention_configuration_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_service_linked_configuration_recorder_response(
    result: &DeleteServiceLinkedConfigurationRecorderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_stored_query_response(result: &DeleteStoredQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deliver_config_snapshot_response(
    result: &DeliverConfigSnapshotResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_aggregate_compliance_by_config_rules_response(
    result: &DescribeAggregateComplianceByConfigRulesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_aggregate_compliance_by_conformance_packs_response(
    result: &DescribeAggregateComplianceByConformancePacksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_aggregation_authorizations_response(
    result: &DescribeAggregationAuthorizationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_compliance_by_config_rule_response(
    result: &DescribeComplianceByConfigRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_compliance_by_resource_response(
    result: &DescribeComplianceByResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_config_rule_evaluation_status_response(
    result: &DescribeConfigRuleEvaluationStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_config_rules_response(
    result: &DescribeConfigRulesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_configuration_aggregator_sources_status_response(
    result: &DescribeConfigurationAggregatorSourcesStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_configuration_aggregators_response(
    result: &DescribeConfigurationAggregatorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_configuration_recorder_status_response(
    result: &DescribeConfigurationRecorderStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_configuration_recorders_response(
    result: &DescribeConfigurationRecordersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_conformance_pack_compliance_response(
    result: &DescribeConformancePackComplianceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_conformance_pack_status_response(
    result: &DescribeConformancePackStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_conformance_packs_response(
    result: &DescribeConformancePacksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_delivery_channel_status_response(
    result: &DescribeDeliveryChannelStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_delivery_channels_response(
    result: &DescribeDeliveryChannelsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_organization_config_rule_statuses_response(
    result: &DescribeOrganizationConfigRuleStatusesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_organization_config_rules_response(
    result: &DescribeOrganizationConfigRulesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_organization_conformance_pack_statuses_response(
    result: &DescribeOrganizationConformancePackStatusesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_organization_conformance_packs_response(
    result: &DescribeOrganizationConformancePacksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_pending_aggregation_requests_response(
    result: &DescribePendingAggregationRequestsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_remediation_configurations_response(
    result: &DescribeRemediationConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_remediation_exceptions_response(
    result: &DescribeRemediationExceptionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_remediation_execution_status_response(
    result: &DescribeRemediationExecutionStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_retention_configurations_response(
    result: &DescribeRetentionConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_resource_types_response(
    result: &DisassociateResourceTypesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_aggregate_compliance_details_by_config_rule_response(
    result: &GetAggregateComplianceDetailsByConfigRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_aggregate_config_rule_compliance_summary_response(
    result: &GetAggregateConfigRuleComplianceSummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_aggregate_conformance_pack_compliance_summary_response(
    result: &GetAggregateConformancePackComplianceSummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_aggregate_discovered_resource_counts_response(
    result: &GetAggregateDiscoveredResourceCountsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_aggregate_resource_config_response(
    result: &GetAggregateResourceConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_compliance_details_by_config_rule_response(
    result: &GetComplianceDetailsByConfigRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_compliance_details_by_resource_response(
    result: &GetComplianceDetailsByResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_compliance_summary_by_config_rule_response(
    result: &GetComplianceSummaryByConfigRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_compliance_summary_by_resource_type_response(
    result: &GetComplianceSummaryByResourceTypeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_conformance_pack_compliance_details_response(
    result: &GetConformancePackComplianceDetailsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_conformance_pack_compliance_summary_response(
    result: &GetConformancePackComplianceSummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_custom_rule_policy_response(
    result: &GetCustomRulePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_discovered_resource_counts_response(
    result: &GetDiscoveredResourceCountsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_organization_config_rule_detailed_status_response(
    result: &GetOrganizationConfigRuleDetailedStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_organization_conformance_pack_detailed_status_response(
    result: &GetOrganizationConformancePackDetailedStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_organization_custom_rule_policy_response(
    result: &GetOrganizationCustomRulePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_config_history_response(
    result: &GetResourceConfigHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_evaluation_summary_response(
    result: &GetResourceEvaluationSummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_stored_query_response(result: &GetStoredQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_aggregate_discovered_resources_response(
    result: &ListAggregateDiscoveredResourcesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_configuration_recorders_response(
    result: &ListConfigurationRecordersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_conformance_pack_compliance_scores_response(
    result: &ListConformancePackComplianceScoresResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_discovered_resources_response(
    result: &ListDiscoveredResourcesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resource_evaluations_response(
    result: &ListResourceEvaluationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_stored_queries_response(result: &ListStoredQueriesResponse) -> MockResponse {
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
pub fn serialize_put_aggregation_authorization_response(
    result: &PutAggregationAuthorizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_config_rule_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_configuration_aggregator_response(
    result: &PutConfigurationAggregatorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_configuration_recorder_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_conformance_pack_response(
    result: &PutConformancePackResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_delivery_channel_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_evaluations_response(result: &PutEvaluationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_external_evaluation_response(
    result: &PutExternalEvaluationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_organization_config_rule_response(
    result: &PutOrganizationConfigRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_organization_conformance_pack_response(
    result: &PutOrganizationConformancePackResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_remediation_configurations_response(
    result: &PutRemediationConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_remediation_exceptions_response(
    result: &PutRemediationExceptionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_resource_config_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_retention_configuration_response(
    result: &PutRetentionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_service_linked_configuration_recorder_response(
    result: &PutServiceLinkedConfigurationRecorderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_stored_query_response(result: &PutStoredQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_select_aggregate_resource_config_response(
    result: &SelectAggregateResourceConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_select_resource_config_response(
    result: &SelectResourceConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_config_rules_evaluation_response(
    result: &StartConfigRulesEvaluationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_start_configuration_recorder_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_remediation_execution_response(
    result: &StartRemediationExecutionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_resource_evaluation_response(
    result: &StartResourceEvaluationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_stop_configuration_recorder_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_resource_types_request(
    body: &[u8],
) -> Result<AssociateResourceTypesRequest, String> {
    if body.is_empty() {
        return Ok(AssociateResourceTypesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateResourceTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_aggregate_resource_config_request(
    body: &[u8],
) -> Result<BatchGetAggregateResourceConfigRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetAggregateResourceConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetAggregateResourceConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_resource_config_request(
    body: &[u8],
) -> Result<BatchGetResourceConfigRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetResourceConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetResourceConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_aggregation_authorization_request(
    body: &[u8],
) -> Result<DeleteAggregationAuthorizationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAggregationAuthorizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAggregationAuthorization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_config_rule_request(
    body: &[u8],
) -> Result<DeleteConfigRuleRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConfigRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConfigRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_configuration_aggregator_request(
    body: &[u8],
) -> Result<DeleteConfigurationAggregatorRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConfigurationAggregatorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConfigurationAggregator request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_configuration_recorder_request(
    body: &[u8],
) -> Result<DeleteConfigurationRecorderRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConfigurationRecorderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConfigurationRecorder request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_conformance_pack_request(
    body: &[u8],
) -> Result<DeleteConformancePackRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConformancePackRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConformancePack request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_delivery_channel_request(
    body: &[u8],
) -> Result<DeleteDeliveryChannelRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDeliveryChannelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDeliveryChannel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_evaluation_results_request(
    body: &[u8],
) -> Result<DeleteEvaluationResultsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEvaluationResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEvaluationResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_organization_config_rule_request(
    body: &[u8],
) -> Result<DeleteOrganizationConfigRuleRequest, String> {
    if body.is_empty() {
        return Ok(DeleteOrganizationConfigRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteOrganizationConfigRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_organization_conformance_pack_request(
    body: &[u8],
) -> Result<DeleteOrganizationConformancePackRequest, String> {
    if body.is_empty() {
        return Ok(DeleteOrganizationConformancePackRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteOrganizationConformancePack request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_pending_aggregation_request_request(
    body: &[u8],
) -> Result<DeletePendingAggregationRequestRequest, String> {
    if body.is_empty() {
        return Ok(DeletePendingAggregationRequestRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePendingAggregationRequest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_remediation_configuration_request(
    body: &[u8],
) -> Result<DeleteRemediationConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRemediationConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRemediationConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_remediation_exceptions_request(
    body: &[u8],
) -> Result<DeleteRemediationExceptionsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRemediationExceptionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRemediationExceptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_config_request(
    body: &[u8],
) -> Result<DeleteResourceConfigRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourceConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourceConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_retention_configuration_request(
    body: &[u8],
) -> Result<DeleteRetentionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRetentionConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRetentionConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_service_linked_configuration_recorder_request(
    body: &[u8],
) -> Result<DeleteServiceLinkedConfigurationRecorderRequest, String> {
    if body.is_empty() {
        return Ok(DeleteServiceLinkedConfigurationRecorderRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteServiceLinkedConfigurationRecorder request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_stored_query_request(
    body: &[u8],
) -> Result<DeleteStoredQueryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteStoredQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStoredQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deliver_config_snapshot_request(
    body: &[u8],
) -> Result<DeliverConfigSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(DeliverConfigSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeliverConfigSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_aggregate_compliance_by_config_rules_request(
    body: &[u8],
) -> Result<DescribeAggregateComplianceByConfigRulesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAggregateComplianceByConfigRulesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeAggregateComplianceByConfigRules request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_aggregate_compliance_by_conformance_packs_request(
    body: &[u8],
) -> Result<DescribeAggregateComplianceByConformancePacksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAggregateComplianceByConformancePacksRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeAggregateComplianceByConformancePacks request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_aggregation_authorizations_request(
    body: &[u8],
) -> Result<DescribeAggregationAuthorizationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAggregationAuthorizationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeAggregationAuthorizations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_compliance_by_config_rule_request(
    body: &[u8],
) -> Result<DescribeComplianceByConfigRuleRequest, String> {
    if body.is_empty() {
        return Ok(DescribeComplianceByConfigRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeComplianceByConfigRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_compliance_by_resource_request(
    body: &[u8],
) -> Result<DescribeComplianceByResourceRequest, String> {
    if body.is_empty() {
        return Ok(DescribeComplianceByResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeComplianceByResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_config_rule_evaluation_status_request(
    body: &[u8],
) -> Result<DescribeConfigRuleEvaluationStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigRuleEvaluationStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeConfigRuleEvaluationStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_config_rules_request(
    body: &[u8],
) -> Result<DescribeConfigRulesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConfigRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_configuration_aggregator_sources_status_request(
    body: &[u8],
) -> Result<DescribeConfigurationAggregatorSourcesStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigurationAggregatorSourcesStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeConfigurationAggregatorSourcesStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_configuration_aggregators_request(
    body: &[u8],
) -> Result<DescribeConfigurationAggregatorsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigurationAggregatorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConfigurationAggregators request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_configuration_recorder_status_request(
    body: &[u8],
) -> Result<DescribeConfigurationRecorderStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigurationRecorderStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeConfigurationRecorderStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_configuration_recorders_request(
    body: &[u8],
) -> Result<DescribeConfigurationRecordersRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigurationRecordersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConfigurationRecorders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_conformance_pack_compliance_request(
    body: &[u8],
) -> Result<DescribeConformancePackComplianceRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConformancePackComplianceRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeConformancePackCompliance request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_conformance_pack_status_request(
    body: &[u8],
) -> Result<DescribeConformancePackStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConformancePackStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConformancePackStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_conformance_packs_request(
    body: &[u8],
) -> Result<DescribeConformancePacksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConformancePacksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConformancePacks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_delivery_channel_status_request(
    body: &[u8],
) -> Result<DescribeDeliveryChannelStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDeliveryChannelStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDeliveryChannelStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_delivery_channels_request(
    body: &[u8],
) -> Result<DescribeDeliveryChannelsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDeliveryChannelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDeliveryChannels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_organization_config_rule_statuses_request(
    body: &[u8],
) -> Result<DescribeOrganizationConfigRuleStatusesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeOrganizationConfigRuleStatusesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeOrganizationConfigRuleStatuses request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_organization_config_rules_request(
    body: &[u8],
) -> Result<DescribeOrganizationConfigRulesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeOrganizationConfigRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeOrganizationConfigRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_organization_conformance_pack_statuses_request(
    body: &[u8],
) -> Result<DescribeOrganizationConformancePackStatusesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeOrganizationConformancePackStatusesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeOrganizationConformancePackStatuses request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_organization_conformance_packs_request(
    body: &[u8],
) -> Result<DescribeOrganizationConformancePacksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeOrganizationConformancePacksRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeOrganizationConformancePacks request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_pending_aggregation_requests_request(
    body: &[u8],
) -> Result<DescribePendingAggregationRequestsRequest, String> {
    if body.is_empty() {
        return Ok(DescribePendingAggregationRequestsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribePendingAggregationRequests request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_remediation_configurations_request(
    body: &[u8],
) -> Result<DescribeRemediationConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRemediationConfigurationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeRemediationConfigurations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_remediation_exceptions_request(
    body: &[u8],
) -> Result<DescribeRemediationExceptionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRemediationExceptionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRemediationExceptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_remediation_execution_status_request(
    body: &[u8],
) -> Result<DescribeRemediationExecutionStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRemediationExecutionStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeRemediationExecutionStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_retention_configurations_request(
    body: &[u8],
) -> Result<DescribeRetentionConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRetentionConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRetentionConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_resource_types_request(
    body: &[u8],
) -> Result<DisassociateResourceTypesRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateResourceTypesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateResourceTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_aggregate_compliance_details_by_config_rule_request(
    body: &[u8],
) -> Result<GetAggregateComplianceDetailsByConfigRuleRequest, String> {
    if body.is_empty() {
        return Ok(GetAggregateComplianceDetailsByConfigRuleRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetAggregateComplianceDetailsByConfigRule request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_aggregate_config_rule_compliance_summary_request(
    body: &[u8],
) -> Result<GetAggregateConfigRuleComplianceSummaryRequest, String> {
    if body.is_empty() {
        return Ok(GetAggregateConfigRuleComplianceSummaryRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetAggregateConfigRuleComplianceSummary request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_aggregate_conformance_pack_compliance_summary_request(
    body: &[u8],
) -> Result<GetAggregateConformancePackComplianceSummaryRequest, String> {
    if body.is_empty() {
        return Ok(GetAggregateConformancePackComplianceSummaryRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetAggregateConformancePackComplianceSummary request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_aggregate_discovered_resource_counts_request(
    body: &[u8],
) -> Result<GetAggregateDiscoveredResourceCountsRequest, String> {
    if body.is_empty() {
        return Ok(GetAggregateDiscoveredResourceCountsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetAggregateDiscoveredResourceCounts request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_aggregate_resource_config_request(
    body: &[u8],
) -> Result<GetAggregateResourceConfigRequest, String> {
    if body.is_empty() {
        return Ok(GetAggregateResourceConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAggregateResourceConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_compliance_details_by_config_rule_request(
    body: &[u8],
) -> Result<GetComplianceDetailsByConfigRuleRequest, String> {
    if body.is_empty() {
        return Ok(GetComplianceDetailsByConfigRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetComplianceDetailsByConfigRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_compliance_details_by_resource_request(
    body: &[u8],
) -> Result<GetComplianceDetailsByResourceRequest, String> {
    if body.is_empty() {
        return Ok(GetComplianceDetailsByResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetComplianceDetailsByResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_compliance_summary_by_resource_type_request(
    body: &[u8],
) -> Result<GetComplianceSummaryByResourceTypeRequest, String> {
    if body.is_empty() {
        return Ok(GetComplianceSummaryByResourceTypeRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetComplianceSummaryByResourceType request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_conformance_pack_compliance_details_request(
    body: &[u8],
) -> Result<GetConformancePackComplianceDetailsRequest, String> {
    if body.is_empty() {
        return Ok(GetConformancePackComplianceDetailsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetConformancePackComplianceDetails request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_conformance_pack_compliance_summary_request(
    body: &[u8],
) -> Result<GetConformancePackComplianceSummaryRequest, String> {
    if body.is_empty() {
        return Ok(GetConformancePackComplianceSummaryRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetConformancePackComplianceSummary request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_custom_rule_policy_request(
    body: &[u8],
) -> Result<GetCustomRulePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetCustomRulePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCustomRulePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_discovered_resource_counts_request(
    body: &[u8],
) -> Result<GetDiscoveredResourceCountsRequest, String> {
    if body.is_empty() {
        return Ok(GetDiscoveredResourceCountsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDiscoveredResourceCounts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_organization_config_rule_detailed_status_request(
    body: &[u8],
) -> Result<GetOrganizationConfigRuleDetailedStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetOrganizationConfigRuleDetailedStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetOrganizationConfigRuleDetailedStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_organization_conformance_pack_detailed_status_request(
    body: &[u8],
) -> Result<GetOrganizationConformancePackDetailedStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetOrganizationConformancePackDetailedStatusRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetOrganizationConformancePackDetailedStatus request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_organization_custom_rule_policy_request(
    body: &[u8],
) -> Result<GetOrganizationCustomRulePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetOrganizationCustomRulePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOrganizationCustomRulePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_config_history_request(
    body: &[u8],
) -> Result<GetResourceConfigHistoryRequest, String> {
    if body.is_empty() {
        return Ok(GetResourceConfigHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourceConfigHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_evaluation_summary_request(
    body: &[u8],
) -> Result<GetResourceEvaluationSummaryRequest, String> {
    if body.is_empty() {
        return Ok(GetResourceEvaluationSummaryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourceEvaluationSummary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_stored_query_request(body: &[u8]) -> Result<GetStoredQueryRequest, String> {
    if body.is_empty() {
        return Ok(GetStoredQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetStoredQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_aggregate_discovered_resources_request(
    body: &[u8],
) -> Result<ListAggregateDiscoveredResourcesRequest, String> {
    if body.is_empty() {
        return Ok(ListAggregateDiscoveredResourcesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAggregateDiscoveredResources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_configuration_recorders_request(
    body: &[u8],
) -> Result<ListConfigurationRecordersRequest, String> {
    if body.is_empty() {
        return Ok(ListConfigurationRecordersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListConfigurationRecorders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_conformance_pack_compliance_scores_request(
    body: &[u8],
) -> Result<ListConformancePackComplianceScoresRequest, String> {
    if body.is_empty() {
        return Ok(ListConformancePackComplianceScoresRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListConformancePackComplianceScores request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_discovered_resources_request(
    body: &[u8],
) -> Result<ListDiscoveredResourcesRequest, String> {
    if body.is_empty() {
        return Ok(ListDiscoveredResourcesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDiscoveredResources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resource_evaluations_request(
    body: &[u8],
) -> Result<ListResourceEvaluationsRequest, String> {
    if body.is_empty() {
        return Ok(ListResourceEvaluationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourceEvaluations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_stored_queries_request(
    body: &[u8],
) -> Result<ListStoredQueriesRequest, String> {
    if body.is_empty() {
        return Ok(ListStoredQueriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStoredQueries request: {e}"))
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
pub fn deserialize_put_aggregation_authorization_request(
    body: &[u8],
) -> Result<PutAggregationAuthorizationRequest, String> {
    if body.is_empty() {
        return Ok(PutAggregationAuthorizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAggregationAuthorization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_config_rule_request(body: &[u8]) -> Result<PutConfigRuleRequest, String> {
    if body.is_empty() {
        return Ok(PutConfigRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutConfigRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_configuration_aggregator_request(
    body: &[u8],
) -> Result<PutConfigurationAggregatorRequest, String> {
    if body.is_empty() {
        return Ok(PutConfigurationAggregatorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutConfigurationAggregator request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_configuration_recorder_request(
    body: &[u8],
) -> Result<PutConfigurationRecorderRequest, String> {
    if body.is_empty() {
        return Ok(PutConfigurationRecorderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutConfigurationRecorder request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_conformance_pack_request(
    body: &[u8],
) -> Result<PutConformancePackRequest, String> {
    if body.is_empty() {
        return Ok(PutConformancePackRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutConformancePack request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_delivery_channel_request(
    body: &[u8],
) -> Result<PutDeliveryChannelRequest, String> {
    if body.is_empty() {
        return Ok(PutDeliveryChannelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDeliveryChannel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_evaluations_request(body: &[u8]) -> Result<PutEvaluationsRequest, String> {
    if body.is_empty() {
        return Ok(PutEvaluationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutEvaluations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_external_evaluation_request(
    body: &[u8],
) -> Result<PutExternalEvaluationRequest, String> {
    if body.is_empty() {
        return Ok(PutExternalEvaluationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutExternalEvaluation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_organization_config_rule_request(
    body: &[u8],
) -> Result<PutOrganizationConfigRuleRequest, String> {
    if body.is_empty() {
        return Ok(PutOrganizationConfigRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutOrganizationConfigRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_organization_conformance_pack_request(
    body: &[u8],
) -> Result<PutOrganizationConformancePackRequest, String> {
    if body.is_empty() {
        return Ok(PutOrganizationConformancePackRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutOrganizationConformancePack request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_remediation_configurations_request(
    body: &[u8],
) -> Result<PutRemediationConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(PutRemediationConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRemediationConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_remediation_exceptions_request(
    body: &[u8],
) -> Result<PutRemediationExceptionsRequest, String> {
    if body.is_empty() {
        return Ok(PutRemediationExceptionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRemediationExceptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_config_request(
    body: &[u8],
) -> Result<PutResourceConfigRequest, String> {
    if body.is_empty() {
        return Ok(PutResourceConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourceConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_retention_configuration_request(
    body: &[u8],
) -> Result<PutRetentionConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutRetentionConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRetentionConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_service_linked_configuration_recorder_request(
    body: &[u8],
) -> Result<PutServiceLinkedConfigurationRecorderRequest, String> {
    if body.is_empty() {
        return Ok(PutServiceLinkedConfigurationRecorderRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutServiceLinkedConfigurationRecorder request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_stored_query_request(body: &[u8]) -> Result<PutStoredQueryRequest, String> {
    if body.is_empty() {
        return Ok(PutStoredQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutStoredQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_select_aggregate_resource_config_request(
    body: &[u8],
) -> Result<SelectAggregateResourceConfigRequest, String> {
    if body.is_empty() {
        return Ok(SelectAggregateResourceConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SelectAggregateResourceConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_select_resource_config_request(
    body: &[u8],
) -> Result<SelectResourceConfigRequest, String> {
    if body.is_empty() {
        return Ok(SelectResourceConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SelectResourceConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_config_rules_evaluation_request(
    body: &[u8],
) -> Result<StartConfigRulesEvaluationRequest, String> {
    if body.is_empty() {
        return Ok(StartConfigRulesEvaluationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartConfigRulesEvaluation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_configuration_recorder_request(
    body: &[u8],
) -> Result<StartConfigurationRecorderRequest, String> {
    if body.is_empty() {
        return Ok(StartConfigurationRecorderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartConfigurationRecorder request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_remediation_execution_request(
    body: &[u8],
) -> Result<StartRemediationExecutionRequest, String> {
    if body.is_empty() {
        return Ok(StartRemediationExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartRemediationExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_resource_evaluation_request(
    body: &[u8],
) -> Result<StartResourceEvaluationRequest, String> {
    if body.is_empty() {
        return Ok(StartResourceEvaluationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartResourceEvaluation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_configuration_recorder_request(
    body: &[u8],
) -> Result<StopConfigurationRecorderRequest, String> {
    if body.is_empty() {
        return Ok(StopConfigurationRecorderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopConfigurationRecorder request: {e}"))
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
