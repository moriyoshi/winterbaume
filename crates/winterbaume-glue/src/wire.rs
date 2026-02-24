//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-glue

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_create_partition_response(
    result: &BatchCreatePartitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_connection_response(
    result: &BatchDeleteConnectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_partition_response(
    result: &BatchDeletePartitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_table_response(result: &BatchDeleteTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_table_version_response(
    result: &BatchDeleteTableVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_blueprints_response(
    result: &BatchGetBlueprintsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_crawlers_response(result: &BatchGetCrawlersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_custom_entity_types_response(
    result: &BatchGetCustomEntityTypesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_data_quality_result_response(
    result: &BatchGetDataQualityResultResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_dev_endpoints_response(
    result: &BatchGetDevEndpointsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_jobs_response(result: &BatchGetJobsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_partition_response(result: &BatchGetPartitionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_table_optimizer_response(
    result: &BatchGetTableOptimizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_triggers_response(result: &BatchGetTriggersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_workflows_response(result: &BatchGetWorkflowsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_put_data_quality_statistic_annotation_response(
    result: &BatchPutDataQualityStatisticAnnotationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_stop_job_run_response(result: &BatchStopJobRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_update_partition_response(
    result: &BatchUpdatePartitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_data_quality_rule_recommendation_run_response(
    result: &CancelDataQualityRuleRecommendationRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_data_quality_ruleset_evaluation_run_response(
    result: &CancelDataQualityRulesetEvaluationRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_m_l_task_run_response(result: &CancelMLTaskRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_statement_response(result: &CancelStatementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_check_schema_version_validity_response(
    result: &CheckSchemaVersionValidityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_blueprint_response(result: &CreateBlueprintResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_catalog_response(result: &CreateCatalogResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_classifier_response(result: &CreateClassifierResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_column_statistics_task_settings_response(
    result: &CreateColumnStatisticsTaskSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_connection_response(result: &CreateConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_crawler_response(result: &CreateCrawlerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_custom_entity_type_response(
    result: &CreateCustomEntityTypeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_data_quality_ruleset_response(
    result: &CreateDataQualityRulesetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_database_response(result: &CreateDatabaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_dev_endpoint_response(result: &CreateDevEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_glue_identity_center_configuration_response(
    result: &CreateGlueIdentityCenterConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_integration_response(result: &CreateIntegrationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_integration_resource_property_response(
    result: &CreateIntegrationResourcePropertyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_integration_table_properties_response(
    result: &CreateIntegrationTablePropertiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_job_response(result: &CreateJobResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_m_l_transform_response(result: &CreateMLTransformResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_partition_response(result: &CreatePartitionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_partition_index_response(
    result: &CreatePartitionIndexResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_registry_response(result: &CreateRegistryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_schema_response(result: &CreateSchemaResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_script_response(result: &CreateScriptResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_security_configuration_response(
    result: &CreateSecurityConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_session_response(result: &CreateSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_table_response(result: &CreateTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_table_optimizer_response(
    result: &CreateTableOptimizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_trigger_response(result: &CreateTriggerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_usage_profile_response(
    result: &CreateUsageProfileResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_defined_function_response(
    result: &CreateUserDefinedFunctionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_workflow_response(result: &CreateWorkflowResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_blueprint_response(result: &DeleteBlueprintResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_catalog_response(result: &DeleteCatalogResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_classifier_response(result: &DeleteClassifierResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_column_statistics_for_partition_response(
    result: &DeleteColumnStatisticsForPartitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_column_statistics_for_table_response(
    result: &DeleteColumnStatisticsForTableResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_column_statistics_task_settings_response(
    result: &DeleteColumnStatisticsTaskSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_connection_response(result: &DeleteConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_connection_type_response(
    result: &DeleteConnectionTypeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_crawler_response(result: &DeleteCrawlerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_custom_entity_type_response(
    result: &DeleteCustomEntityTypeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_data_quality_ruleset_response(
    result: &DeleteDataQualityRulesetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_database_response(result: &DeleteDatabaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_dev_endpoint_response(result: &DeleteDevEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_glue_identity_center_configuration_response(
    result: &DeleteGlueIdentityCenterConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_integration_response(result: &DeleteIntegrationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_integration_resource_property_response(
    result: &DeleteIntegrationResourcePropertyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_integration_table_properties_response(
    result: &DeleteIntegrationTablePropertiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_job_response(result: &DeleteJobResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_m_l_transform_response(result: &DeleteMLTransformResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_partition_response(result: &DeletePartitionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_partition_index_response(
    result: &DeletePartitionIndexResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_registry_response(result: &DeleteRegistryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_schema_response(result: &DeleteSchemaResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_schema_versions_response(
    result: &DeleteSchemaVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_security_configuration_response(
    result: &DeleteSecurityConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_session_response(result: &DeleteSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_table_response(result: &DeleteTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_table_optimizer_response(
    result: &DeleteTableOptimizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_table_version_response(
    result: &DeleteTableVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_trigger_response(result: &DeleteTriggerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_usage_profile_response(
    result: &DeleteUsageProfileResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_user_defined_function_response(
    result: &DeleteUserDefinedFunctionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_workflow_response(result: &DeleteWorkflowResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connection_type_response(
    result: &DescribeConnectionTypeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_entity_response(result: &DescribeEntityResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_inbound_integrations_response(
    result: &DescribeInboundIntegrationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_integrations_response(
    result: &DescribeIntegrationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_blueprint_response(result: &GetBlueprintResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_blueprint_run_response(result: &GetBlueprintRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_blueprint_runs_response(result: &GetBlueprintRunsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_catalog_response(result: &GetCatalogResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_catalog_import_status_response(
    result: &GetCatalogImportStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_catalogs_response(result: &GetCatalogsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_classifier_response(result: &GetClassifierResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_classifiers_response(result: &GetClassifiersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_column_statistics_for_partition_response(
    result: &GetColumnStatisticsForPartitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_column_statistics_for_table_response(
    result: &GetColumnStatisticsForTableResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_column_statistics_task_run_response(
    result: &GetColumnStatisticsTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_column_statistics_task_runs_response(
    result: &GetColumnStatisticsTaskRunsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_column_statistics_task_settings_response(
    result: &GetColumnStatisticsTaskSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_connection_response(result: &GetConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_connections_response(result: &GetConnectionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_crawler_response(result: &GetCrawlerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_crawler_metrics_response(result: &GetCrawlerMetricsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_crawlers_response(result: &GetCrawlersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_custom_entity_type_response(
    result: &GetCustomEntityTypeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_catalog_encryption_settings_response(
    result: &GetDataCatalogEncryptionSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_quality_model_response(
    result: &GetDataQualityModelResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_quality_model_result_response(
    result: &GetDataQualityModelResultResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_quality_result_response(
    result: &GetDataQualityResultResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_quality_rule_recommendation_run_response(
    result: &GetDataQualityRuleRecommendationRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_quality_ruleset_response(
    result: &GetDataQualityRulesetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_quality_ruleset_evaluation_run_response(
    result: &GetDataQualityRulesetEvaluationRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_database_response(result: &GetDatabaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_databases_response(result: &GetDatabasesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dataflow_graph_response(result: &GetDataflowGraphResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dev_endpoint_response(result: &GetDevEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dev_endpoints_response(result: &GetDevEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_entity_records_response(result: &GetEntityRecordsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_glue_identity_center_configuration_response(
    result: &GetGlueIdentityCenterConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_integration_resource_property_response(
    result: &GetIntegrationResourcePropertyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_integration_table_properties_response(
    result: &GetIntegrationTablePropertiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_job_response(result: &GetJobResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_job_bookmark_response(result: &GetJobBookmarkResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_job_run_response(result: &GetJobRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_job_runs_response(result: &GetJobRunsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_jobs_response(result: &GetJobsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_m_l_task_run_response(result: &GetMLTaskRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_m_l_task_runs_response(result: &GetMLTaskRunsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_m_l_transform_response(result: &GetMLTransformResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_m_l_transforms_response(result: &GetMLTransformsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_mapping_response(result: &GetMappingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_materialized_view_refresh_task_run_response(
    result: &GetMaterializedViewRefreshTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_partition_response(result: &GetPartitionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_partition_indexes_response(
    result: &GetPartitionIndexesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_partitions_response(result: &GetPartitionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_plan_response(result: &GetPlanResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_registry_response(result: &GetRegistryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policies_response(
    result: &GetResourcePoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_schema_response(result: &GetSchemaResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_schema_by_definition_response(
    result: &GetSchemaByDefinitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_schema_version_response(result: &GetSchemaVersionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_schema_versions_diff_response(
    result: &GetSchemaVersionsDiffResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_security_configuration_response(
    result: &GetSecurityConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_security_configurations_response(
    result: &GetSecurityConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_session_response(result: &GetSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_statement_response(result: &GetStatementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_response(result: &GetTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_optimizer_response(result: &GetTableOptimizerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_version_response(result: &GetTableVersionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_versions_response(result: &GetTableVersionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_tables_response(result: &GetTablesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_tags_response(result: &GetTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_trigger_response(result: &GetTriggerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_triggers_response(result: &GetTriggersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_unfiltered_partition_metadata_response(
    result: &GetUnfilteredPartitionMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_unfiltered_partitions_metadata_response(
    result: &GetUnfilteredPartitionsMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_unfiltered_table_metadata_response(
    result: &GetUnfilteredTableMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_usage_profile_response(result: &GetUsageProfileResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_user_defined_function_response(
    result: &GetUserDefinedFunctionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_user_defined_functions_response(
    result: &GetUserDefinedFunctionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_workflow_response(result: &GetWorkflowResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_workflow_run_response(result: &GetWorkflowRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_workflow_run_properties_response(
    result: &GetWorkflowRunPropertiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_workflow_runs_response(result: &GetWorkflowRunsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_catalog_to_glue_response(
    result: &ImportCatalogToGlueResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_blueprints_response(result: &ListBlueprintsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_column_statistics_task_runs_response(
    result: &ListColumnStatisticsTaskRunsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_connection_types_response(
    result: &ListConnectionTypesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_crawlers_response(result: &ListCrawlersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_crawls_response(result: &ListCrawlsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_custom_entity_types_response(
    result: &ListCustomEntityTypesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_quality_results_response(
    result: &ListDataQualityResultsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_quality_rule_recommendation_runs_response(
    result: &ListDataQualityRuleRecommendationRunsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_quality_ruleset_evaluation_runs_response(
    result: &ListDataQualityRulesetEvaluationRunsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_quality_rulesets_response(
    result: &ListDataQualityRulesetsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_quality_statistic_annotations_response(
    result: &ListDataQualityStatisticAnnotationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_quality_statistics_response(
    result: &ListDataQualityStatisticsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dev_endpoints_response(result: &ListDevEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_entities_response(result: &ListEntitiesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_integration_resource_properties_response(
    result: &ListIntegrationResourcePropertiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_jobs_response(result: &ListJobsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_m_l_transforms_response(result: &ListMLTransformsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_materialized_view_refresh_task_runs_response(
    result: &ListMaterializedViewRefreshTaskRunsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_registries_response(result: &ListRegistriesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_schema_versions_response(
    result: &ListSchemaVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_schemas_response(result: &ListSchemasResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_sessions_response(result: &ListSessionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_statements_response(result: &ListStatementsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_table_optimizer_runs_response(
    result: &ListTableOptimizerRunsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_triggers_response(result: &ListTriggersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_usage_profiles_response(result: &ListUsageProfilesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_workflows_response(result: &ListWorkflowsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_integration_response(result: &ModifyIntegrationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_data_catalog_encryption_settings_response(
    result: &PutDataCatalogEncryptionSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_data_quality_profile_annotation_response(
    result: &PutDataQualityProfileAnnotationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_schema_version_metadata_response(
    result: &PutSchemaVersionMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_workflow_run_properties_response(
    result: &PutWorkflowRunPropertiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_query_schema_version_metadata_response(
    result: &QuerySchemaVersionMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_connection_type_response(
    result: &RegisterConnectionTypeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_schema_version_response(
    result: &RegisterSchemaVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_schema_version_metadata_response(
    result: &RemoveSchemaVersionMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reset_job_bookmark_response(result: &ResetJobBookmarkResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_resume_workflow_run_response(result: &ResumeWorkflowRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_run_statement_response(result: &RunStatementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_tables_response(result: &SearchTablesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_blueprint_run_response(result: &StartBlueprintRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_column_statistics_task_run_response(
    result: &StartColumnStatisticsTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_column_statistics_task_run_schedule_response(
    result: &StartColumnStatisticsTaskRunScheduleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_crawler_response(result: &StartCrawlerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_crawler_schedule_response(
    result: &StartCrawlerScheduleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_data_quality_rule_recommendation_run_response(
    result: &StartDataQualityRuleRecommendationRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_data_quality_ruleset_evaluation_run_response(
    result: &StartDataQualityRulesetEvaluationRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_export_labels_task_run_response(
    result: &StartExportLabelsTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_import_labels_task_run_response(
    result: &StartImportLabelsTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_job_run_response(result: &StartJobRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_m_l_evaluation_task_run_response(
    result: &StartMLEvaluationTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_m_l_labeling_set_generation_task_run_response(
    result: &StartMLLabelingSetGenerationTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_materialized_view_refresh_task_run_response(
    result: &StartMaterializedViewRefreshTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_trigger_response(result: &StartTriggerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_workflow_run_response(result: &StartWorkflowRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_column_statistics_task_run_response(
    result: &StopColumnStatisticsTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_column_statistics_task_run_schedule_response(
    result: &StopColumnStatisticsTaskRunScheduleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_crawler_response(result: &StopCrawlerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_crawler_schedule_response(
    result: &StopCrawlerScheduleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_materialized_view_refresh_task_run_response(
    result: &StopMaterializedViewRefreshTaskRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_session_response(result: &StopSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_trigger_response(result: &StopTriggerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_workflow_run_response(result: &StopWorkflowRunResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_connection_response(result: &TestConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_blueprint_response(result: &UpdateBlueprintResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_catalog_response(result: &UpdateCatalogResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_classifier_response(result: &UpdateClassifierResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_column_statistics_for_partition_response(
    result: &UpdateColumnStatisticsForPartitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_column_statistics_for_table_response(
    result: &UpdateColumnStatisticsForTableResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_column_statistics_task_settings_response(
    result: &UpdateColumnStatisticsTaskSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_connection_response(result: &UpdateConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_crawler_response(result: &UpdateCrawlerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_crawler_schedule_response(
    result: &UpdateCrawlerScheduleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_data_quality_ruleset_response(
    result: &UpdateDataQualityRulesetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_database_response(result: &UpdateDatabaseResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_dev_endpoint_response(result: &UpdateDevEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_glue_identity_center_configuration_response(
    result: &UpdateGlueIdentityCenterConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_integration_resource_property_response(
    result: &UpdateIntegrationResourcePropertyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_integration_table_properties_response(
    result: &UpdateIntegrationTablePropertiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_job_response(result: &UpdateJobResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_job_from_source_control_response(
    result: &UpdateJobFromSourceControlResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_m_l_transform_response(result: &UpdateMLTransformResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_partition_response(result: &UpdatePartitionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_registry_response(result: &UpdateRegistryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_schema_response(result: &UpdateSchemaResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_source_control_from_job_response(
    result: &UpdateSourceControlFromJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_table_response(result: &UpdateTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_table_optimizer_response(
    result: &UpdateTableOptimizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_trigger_response(result: &UpdateTriggerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_usage_profile_response(
    result: &UpdateUsageProfileResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_defined_function_response(
    result: &UpdateUserDefinedFunctionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_workflow_response(result: &UpdateWorkflowResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_create_partition_request(
    body: &[u8],
) -> Result<BatchCreatePartitionRequest, String> {
    if body.is_empty() {
        return Ok(BatchCreatePartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchCreatePartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_connection_request(
    body: &[u8],
) -> Result<BatchDeleteConnectionRequest, String> {
    if body.is_empty() {
        return Ok(BatchDeleteConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeleteConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_partition_request(
    body: &[u8],
) -> Result<BatchDeletePartitionRequest, String> {
    if body.is_empty() {
        return Ok(BatchDeletePartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeletePartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_table_request(
    body: &[u8],
) -> Result<BatchDeleteTableRequest, String> {
    if body.is_empty() {
        return Ok(BatchDeleteTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeleteTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_table_version_request(
    body: &[u8],
) -> Result<BatchDeleteTableVersionRequest, String> {
    if body.is_empty() {
        return Ok(BatchDeleteTableVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeleteTableVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_blueprints_request(
    body: &[u8],
) -> Result<BatchGetBlueprintsRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetBlueprintsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetBlueprints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_crawlers_request(
    body: &[u8],
) -> Result<BatchGetCrawlersRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetCrawlersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetCrawlers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_custom_entity_types_request(
    body: &[u8],
) -> Result<BatchGetCustomEntityTypesRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetCustomEntityTypesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetCustomEntityTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_data_quality_result_request(
    body: &[u8],
) -> Result<BatchGetDataQualityResultRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetDataQualityResultRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetDataQualityResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_dev_endpoints_request(
    body: &[u8],
) -> Result<BatchGetDevEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetDevEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetDevEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_jobs_request(body: &[u8]) -> Result<BatchGetJobsRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_partition_request(
    body: &[u8],
) -> Result<BatchGetPartitionRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetPartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetPartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_table_optimizer_request(
    body: &[u8],
) -> Result<BatchGetTableOptimizerRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetTableOptimizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetTableOptimizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_triggers_request(
    body: &[u8],
) -> Result<BatchGetTriggersRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetTriggersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetTriggers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_workflows_request(
    body: &[u8],
) -> Result<BatchGetWorkflowsRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetWorkflowsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetWorkflows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_put_data_quality_statistic_annotation_request(
    body: &[u8],
) -> Result<BatchPutDataQualityStatisticAnnotationRequest, String> {
    if body.is_empty() {
        return Ok(BatchPutDataQualityStatisticAnnotationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize BatchPutDataQualityStatisticAnnotation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_stop_job_run_request(
    body: &[u8],
) -> Result<BatchStopJobRunRequest, String> {
    if body.is_empty() {
        return Ok(BatchStopJobRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchStopJobRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_update_partition_request(
    body: &[u8],
) -> Result<BatchUpdatePartitionRequest, String> {
    if body.is_empty() {
        return Ok(BatchUpdatePartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchUpdatePartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_data_quality_rule_recommendation_run_request(
    body: &[u8],
) -> Result<CancelDataQualityRuleRecommendationRunRequest, String> {
    if body.is_empty() {
        return Ok(CancelDataQualityRuleRecommendationRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CancelDataQualityRuleRecommendationRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_data_quality_ruleset_evaluation_run_request(
    body: &[u8],
) -> Result<CancelDataQualityRulesetEvaluationRunRequest, String> {
    if body.is_empty() {
        return Ok(CancelDataQualityRulesetEvaluationRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CancelDataQualityRulesetEvaluationRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_m_l_task_run_request(
    body: &[u8],
) -> Result<CancelMLTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(CancelMLTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelMLTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_statement_request(body: &[u8]) -> Result<CancelStatementRequest, String> {
    if body.is_empty() {
        return Ok(CancelStatementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_check_schema_version_validity_request(
    body: &[u8],
) -> Result<CheckSchemaVersionValidityInput, String> {
    if body.is_empty() {
        return Ok(CheckSchemaVersionValidityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CheckSchemaVersionValidity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_blueprint_request(body: &[u8]) -> Result<CreateBlueprintRequest, String> {
    if body.is_empty() {
        return Ok(CreateBlueprintRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBlueprint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_catalog_request(body: &[u8]) -> Result<CreateCatalogRequest, String> {
    if body.is_empty() {
        return Ok(CreateCatalogRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_classifier_request(
    body: &[u8],
) -> Result<CreateClassifierRequest, String> {
    if body.is_empty() {
        return Ok(CreateClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_column_statistics_task_settings_request(
    body: &[u8],
) -> Result<CreateColumnStatisticsTaskSettingsRequest, String> {
    if body.is_empty() {
        return Ok(CreateColumnStatisticsTaskSettingsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CreateColumnStatisticsTaskSettings request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_connection_request(
    body: &[u8],
) -> Result<CreateConnectionRequest, String> {
    if body.is_empty() {
        return Ok(CreateConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_crawler_request(body: &[u8]) -> Result<CreateCrawlerRequest, String> {
    if body.is_empty() {
        return Ok(CreateCrawlerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCrawler request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_custom_entity_type_request(
    body: &[u8],
) -> Result<CreateCustomEntityTypeRequest, String> {
    if body.is_empty() {
        return Ok(CreateCustomEntityTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCustomEntityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_data_quality_ruleset_request(
    body: &[u8],
) -> Result<CreateDataQualityRulesetRequest, String> {
    if body.is_empty() {
        return Ok(CreateDataQualityRulesetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataQualityRuleset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_database_request(body: &[u8]) -> Result<CreateDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(CreateDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDatabase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_dev_endpoint_request(
    body: &[u8],
) -> Result<CreateDevEndpointRequest, String> {
    if body.is_empty() {
        return Ok(CreateDevEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDevEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_glue_identity_center_configuration_request(
    body: &[u8],
) -> Result<CreateGlueIdentityCenterConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(CreateGlueIdentityCenterConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CreateGlueIdentityCenterConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_integration_request(
    body: &[u8],
) -> Result<CreateIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(CreateIntegrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateIntegration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_integration_resource_property_request(
    body: &[u8],
) -> Result<CreateIntegrationResourcePropertyRequest, String> {
    if body.is_empty() {
        return Ok(CreateIntegrationResourcePropertyRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CreateIntegrationResourceProperty request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_integration_table_properties_request(
    body: &[u8],
) -> Result<CreateIntegrationTablePropertiesRequest, String> {
    if body.is_empty() {
        return Ok(CreateIntegrationTablePropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateIntegrationTableProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_job_request(body: &[u8]) -> Result<CreateJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_m_l_transform_request(
    body: &[u8],
) -> Result<CreateMLTransformRequest, String> {
    if body.is_empty() {
        return Ok(CreateMLTransformRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMLTransform request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_partition_request(body: &[u8]) -> Result<CreatePartitionRequest, String> {
    if body.is_empty() {
        return Ok(CreatePartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_partition_index_request(
    body: &[u8],
) -> Result<CreatePartitionIndexRequest, String> {
    if body.is_empty() {
        return Ok(CreatePartitionIndexRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePartitionIndex request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_registry_request(body: &[u8]) -> Result<CreateRegistryInput, String> {
    if body.is_empty() {
        return Ok(CreateRegistryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRegistry request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_schema_request(body: &[u8]) -> Result<CreateSchemaInput, String> {
    if body.is_empty() {
        return Ok(CreateSchemaInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_script_request(body: &[u8]) -> Result<CreateScriptRequest, String> {
    if body.is_empty() {
        return Ok(CreateScriptRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateScript request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_security_configuration_request(
    body: &[u8],
) -> Result<CreateSecurityConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(CreateSecurityConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSecurityConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_session_request(body: &[u8]) -> Result<CreateSessionRequest, String> {
    if body.is_empty() {
        return Ok(CreateSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_table_request(body: &[u8]) -> Result<CreateTableRequest, String> {
    if body.is_empty() {
        return Ok(CreateTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_table_optimizer_request(
    body: &[u8],
) -> Result<CreateTableOptimizerRequest, String> {
    if body.is_empty() {
        return Ok(CreateTableOptimizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTableOptimizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_trigger_request(body: &[u8]) -> Result<CreateTriggerRequest, String> {
    if body.is_empty() {
        return Ok(CreateTriggerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTrigger request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_usage_profile_request(
    body: &[u8],
) -> Result<CreateUsageProfileRequest, String> {
    if body.is_empty() {
        return Ok(CreateUsageProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUsageProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_user_defined_function_request(
    body: &[u8],
) -> Result<CreateUserDefinedFunctionRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserDefinedFunctionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUserDefinedFunction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_workflow_request(body: &[u8]) -> Result<CreateWorkflowRequest, String> {
    if body.is_empty() {
        return Ok(CreateWorkflowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWorkflow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_blueprint_request(body: &[u8]) -> Result<DeleteBlueprintRequest, String> {
    if body.is_empty() {
        return Ok(DeleteBlueprintRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBlueprint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_catalog_request(body: &[u8]) -> Result<DeleteCatalogRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCatalogRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_classifier_request(
    body: &[u8],
) -> Result<DeleteClassifierRequest, String> {
    if body.is_empty() {
        return Ok(DeleteClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_column_statistics_for_partition_request(
    body: &[u8],
) -> Result<DeleteColumnStatisticsForPartitionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteColumnStatisticsForPartitionRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteColumnStatisticsForPartition request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_column_statistics_for_table_request(
    body: &[u8],
) -> Result<DeleteColumnStatisticsForTableRequest, String> {
    if body.is_empty() {
        return Ok(DeleteColumnStatisticsForTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteColumnStatisticsForTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_column_statistics_task_settings_request(
    body: &[u8],
) -> Result<DeleteColumnStatisticsTaskSettingsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteColumnStatisticsTaskSettingsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteColumnStatisticsTaskSettings request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connection_request(
    body: &[u8],
) -> Result<DeleteConnectionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connection_type_request(
    body: &[u8],
) -> Result<DeleteConnectionTypeRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConnectionTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnectionType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_crawler_request(body: &[u8]) -> Result<DeleteCrawlerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCrawlerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCrawler request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_custom_entity_type_request(
    body: &[u8],
) -> Result<DeleteCustomEntityTypeRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCustomEntityTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCustomEntityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_data_quality_ruleset_request(
    body: &[u8],
) -> Result<DeleteDataQualityRulesetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDataQualityRulesetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataQualityRuleset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_database_request(body: &[u8]) -> Result<DeleteDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDatabase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_dev_endpoint_request(
    body: &[u8],
) -> Result<DeleteDevEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDevEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDevEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_glue_identity_center_configuration_request(
    body: &[u8],
) -> Result<DeleteGlueIdentityCenterConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteGlueIdentityCenterConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteGlueIdentityCenterConfiguration request: {e}")
    })
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
pub fn deserialize_delete_integration_resource_property_request(
    body: &[u8],
) -> Result<DeleteIntegrationResourcePropertyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIntegrationResourcePropertyRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteIntegrationResourceProperty request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_integration_table_properties_request(
    body: &[u8],
) -> Result<DeleteIntegrationTablePropertiesRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIntegrationTablePropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIntegrationTableProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_job_request(body: &[u8]) -> Result<DeleteJobRequest, String> {
    if body.is_empty() {
        return Ok(DeleteJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_m_l_transform_request(
    body: &[u8],
) -> Result<DeleteMLTransformRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMLTransformRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMLTransform request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_partition_request(body: &[u8]) -> Result<DeletePartitionRequest, String> {
    if body.is_empty() {
        return Ok(DeletePartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_partition_index_request(
    body: &[u8],
) -> Result<DeletePartitionIndexRequest, String> {
    if body.is_empty() {
        return Ok(DeletePartitionIndexRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePartitionIndex request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_registry_request(body: &[u8]) -> Result<DeleteRegistryInput, String> {
    if body.is_empty() {
        return Ok(DeleteRegistryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRegistry request: {e}"))
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
pub fn deserialize_delete_schema_request(body: &[u8]) -> Result<DeleteSchemaInput, String> {
    if body.is_empty() {
        return Ok(DeleteSchemaInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_schema_versions_request(
    body: &[u8],
) -> Result<DeleteSchemaVersionsInput, String> {
    if body.is_empty() {
        return Ok(DeleteSchemaVersionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSchemaVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_security_configuration_request(
    body: &[u8],
) -> Result<DeleteSecurityConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSecurityConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSecurityConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_session_request(body: &[u8]) -> Result<DeleteSessionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_table_request(body: &[u8]) -> Result<DeleteTableRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_table_optimizer_request(
    body: &[u8],
) -> Result<DeleteTableOptimizerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTableOptimizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTableOptimizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_table_version_request(
    body: &[u8],
) -> Result<DeleteTableVersionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTableVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTableVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_trigger_request(body: &[u8]) -> Result<DeleteTriggerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTriggerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTrigger request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_usage_profile_request(
    body: &[u8],
) -> Result<DeleteUsageProfileRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUsageProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUsageProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_defined_function_request(
    body: &[u8],
) -> Result<DeleteUserDefinedFunctionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserDefinedFunctionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUserDefinedFunction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_workflow_request(body: &[u8]) -> Result<DeleteWorkflowRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWorkflowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWorkflow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connection_type_request(
    body: &[u8],
) -> Result<DescribeConnectionTypeRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnectionType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_entity_request(body: &[u8]) -> Result<DescribeEntityRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEntityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEntity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_inbound_integrations_request(
    body: &[u8],
) -> Result<DescribeInboundIntegrationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInboundIntegrationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInboundIntegrations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_integrations_request(
    body: &[u8],
) -> Result<DescribeIntegrationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeIntegrationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeIntegrations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_blueprint_request(body: &[u8]) -> Result<GetBlueprintRequest, String> {
    if body.is_empty() {
        return Ok(GetBlueprintRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetBlueprint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_blueprint_run_request(
    body: &[u8],
) -> Result<GetBlueprintRunRequest, String> {
    if body.is_empty() {
        return Ok(GetBlueprintRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetBlueprintRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_blueprint_runs_request(
    body: &[u8],
) -> Result<GetBlueprintRunsRequest, String> {
    if body.is_empty() {
        return Ok(GetBlueprintRunsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetBlueprintRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_catalog_request(body: &[u8]) -> Result<GetCatalogRequest, String> {
    if body.is_empty() {
        return Ok(GetCatalogRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_catalog_import_status_request(
    body: &[u8],
) -> Result<GetCatalogImportStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetCatalogImportStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCatalogImportStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_catalogs_request(body: &[u8]) -> Result<GetCatalogsRequest, String> {
    if body.is_empty() {
        return Ok(GetCatalogsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCatalogs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_classifier_request(body: &[u8]) -> Result<GetClassifierRequest, String> {
    if body.is_empty() {
        return Ok(GetClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_classifiers_request(body: &[u8]) -> Result<GetClassifiersRequest, String> {
    if body.is_empty() {
        return Ok(GetClassifiersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetClassifiers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_column_statistics_for_partition_request(
    body: &[u8],
) -> Result<GetColumnStatisticsForPartitionRequest, String> {
    if body.is_empty() {
        return Ok(GetColumnStatisticsForPartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetColumnStatisticsForPartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_column_statistics_for_table_request(
    body: &[u8],
) -> Result<GetColumnStatisticsForTableRequest, String> {
    if body.is_empty() {
        return Ok(GetColumnStatisticsForTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetColumnStatisticsForTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_column_statistics_task_run_request(
    body: &[u8],
) -> Result<GetColumnStatisticsTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(GetColumnStatisticsTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetColumnStatisticsTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_column_statistics_task_runs_request(
    body: &[u8],
) -> Result<GetColumnStatisticsTaskRunsRequest, String> {
    if body.is_empty() {
        return Ok(GetColumnStatisticsTaskRunsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetColumnStatisticsTaskRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_column_statistics_task_settings_request(
    body: &[u8],
) -> Result<GetColumnStatisticsTaskSettingsRequest, String> {
    if body.is_empty() {
        return Ok(GetColumnStatisticsTaskSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetColumnStatisticsTaskSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_connection_request(body: &[u8]) -> Result<GetConnectionRequest, String> {
    if body.is_empty() {
        return Ok(GetConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_connections_request(body: &[u8]) -> Result<GetConnectionsRequest, String> {
    if body.is_empty() {
        return Ok(GetConnectionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetConnections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_crawler_request(body: &[u8]) -> Result<GetCrawlerRequest, String> {
    if body.is_empty() {
        return Ok(GetCrawlerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCrawler request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_crawler_metrics_request(
    body: &[u8],
) -> Result<GetCrawlerMetricsRequest, String> {
    if body.is_empty() {
        return Ok(GetCrawlerMetricsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCrawlerMetrics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_crawlers_request(body: &[u8]) -> Result<GetCrawlersRequest, String> {
    if body.is_empty() {
        return Ok(GetCrawlersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCrawlers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_custom_entity_type_request(
    body: &[u8],
) -> Result<GetCustomEntityTypeRequest, String> {
    if body.is_empty() {
        return Ok(GetCustomEntityTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCustomEntityType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_catalog_encryption_settings_request(
    body: &[u8],
) -> Result<GetDataCatalogEncryptionSettingsRequest, String> {
    if body.is_empty() {
        return Ok(GetDataCatalogEncryptionSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataCatalogEncryptionSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_quality_model_request(
    body: &[u8],
) -> Result<GetDataQualityModelRequest, String> {
    if body.is_empty() {
        return Ok(GetDataQualityModelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataQualityModel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_quality_model_result_request(
    body: &[u8],
) -> Result<GetDataQualityModelResultRequest, String> {
    if body.is_empty() {
        return Ok(GetDataQualityModelResultRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataQualityModelResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_quality_result_request(
    body: &[u8],
) -> Result<GetDataQualityResultRequest, String> {
    if body.is_empty() {
        return Ok(GetDataQualityResultRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataQualityResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_quality_rule_recommendation_run_request(
    body: &[u8],
) -> Result<GetDataQualityRuleRecommendationRunRequest, String> {
    if body.is_empty() {
        return Ok(GetDataQualityRuleRecommendationRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetDataQualityRuleRecommendationRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_quality_ruleset_request(
    body: &[u8],
) -> Result<GetDataQualityRulesetRequest, String> {
    if body.is_empty() {
        return Ok(GetDataQualityRulesetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataQualityRuleset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_quality_ruleset_evaluation_run_request(
    body: &[u8],
) -> Result<GetDataQualityRulesetEvaluationRunRequest, String> {
    if body.is_empty() {
        return Ok(GetDataQualityRulesetEvaluationRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetDataQualityRulesetEvaluationRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_database_request(body: &[u8]) -> Result<GetDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(GetDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDatabase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_databases_request(body: &[u8]) -> Result<GetDatabasesRequest, String> {
    if body.is_empty() {
        return Ok(GetDatabasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDatabases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_dataflow_graph_request(
    body: &[u8],
) -> Result<GetDataflowGraphRequest, String> {
    if body.is_empty() {
        return Ok(GetDataflowGraphRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataflowGraph request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_dev_endpoint_request(body: &[u8]) -> Result<GetDevEndpointRequest, String> {
    if body.is_empty() {
        return Ok(GetDevEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDevEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_dev_endpoints_request(
    body: &[u8],
) -> Result<GetDevEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(GetDevEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDevEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_entity_records_request(
    body: &[u8],
) -> Result<GetEntityRecordsRequest, String> {
    if body.is_empty() {
        return Ok(GetEntityRecordsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetEntityRecords request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_glue_identity_center_configuration_request(
    body: &[u8],
) -> Result<GetGlueIdentityCenterConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetGlueIdentityCenterConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetGlueIdentityCenterConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_integration_resource_property_request(
    body: &[u8],
) -> Result<GetIntegrationResourcePropertyRequest, String> {
    if body.is_empty() {
        return Ok(GetIntegrationResourcePropertyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetIntegrationResourceProperty request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_integration_table_properties_request(
    body: &[u8],
) -> Result<GetIntegrationTablePropertiesRequest, String> {
    if body.is_empty() {
        return Ok(GetIntegrationTablePropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetIntegrationTableProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_job_request(body: &[u8]) -> Result<GetJobRequest, String> {
    if body.is_empty() {
        return Ok(GetJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_job_bookmark_request(body: &[u8]) -> Result<GetJobBookmarkRequest, String> {
    if body.is_empty() {
        return Ok(GetJobBookmarkRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetJobBookmark request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_job_run_request(body: &[u8]) -> Result<GetJobRunRequest, String> {
    if body.is_empty() {
        return Ok(GetJobRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetJobRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_job_runs_request(body: &[u8]) -> Result<GetJobRunsRequest, String> {
    if body.is_empty() {
        return Ok(GetJobRunsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetJobRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_jobs_request(body: &[u8]) -> Result<GetJobsRequest, String> {
    if body.is_empty() {
        return Ok(GetJobsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_m_l_task_run_request(body: &[u8]) -> Result<GetMLTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(GetMLTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMLTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_m_l_task_runs_request(body: &[u8]) -> Result<GetMLTaskRunsRequest, String> {
    if body.is_empty() {
        return Ok(GetMLTaskRunsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMLTaskRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_m_l_transform_request(body: &[u8]) -> Result<GetMLTransformRequest, String> {
    if body.is_empty() {
        return Ok(GetMLTransformRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMLTransform request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_m_l_transforms_request(
    body: &[u8],
) -> Result<GetMLTransformsRequest, String> {
    if body.is_empty() {
        return Ok(GetMLTransformsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMLTransforms request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_mapping_request(body: &[u8]) -> Result<GetMappingRequest, String> {
    if body.is_empty() {
        return Ok(GetMappingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMapping request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_materialized_view_refresh_task_run_request(
    body: &[u8],
) -> Result<GetMaterializedViewRefreshTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(GetMaterializedViewRefreshTaskRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetMaterializedViewRefreshTaskRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_partition_request(body: &[u8]) -> Result<GetPartitionRequest, String> {
    if body.is_empty() {
        return Ok(GetPartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_partition_indexes_request(
    body: &[u8],
) -> Result<GetPartitionIndexesRequest, String> {
    if body.is_empty() {
        return Ok(GetPartitionIndexesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPartitionIndexes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_partitions_request(body: &[u8]) -> Result<GetPartitionsRequest, String> {
    if body.is_empty() {
        return Ok(GetPartitionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPartitions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_plan_request(body: &[u8]) -> Result<GetPlanRequest, String> {
    if body.is_empty() {
        return Ok(GetPlanRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetPlan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_registry_request(body: &[u8]) -> Result<GetRegistryInput, String> {
    if body.is_empty() {
        return Ok(GetRegistryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRegistry request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policies_request(
    body: &[u8],
) -> Result<GetResourcePoliciesRequest, String> {
    if body.is_empty() {
        return Ok(GetResourcePoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policy_request(
    body: &[u8],
) -> Result<GetResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_schema_request(body: &[u8]) -> Result<GetSchemaInput, String> {
    if body.is_empty() {
        return Ok(GetSchemaInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_schema_by_definition_request(
    body: &[u8],
) -> Result<GetSchemaByDefinitionInput, String> {
    if body.is_empty() {
        return Ok(GetSchemaByDefinitionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSchemaByDefinition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_schema_version_request(
    body: &[u8],
) -> Result<GetSchemaVersionInput, String> {
    if body.is_empty() {
        return Ok(GetSchemaVersionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSchemaVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_schema_versions_diff_request(
    body: &[u8],
) -> Result<GetSchemaVersionsDiffInput, String> {
    if body.is_empty() {
        return Ok(GetSchemaVersionsDiffInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSchemaVersionsDiff request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_security_configuration_request(
    body: &[u8],
) -> Result<GetSecurityConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetSecurityConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSecurityConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_security_configurations_request(
    body: &[u8],
) -> Result<GetSecurityConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(GetSecurityConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSecurityConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_session_request(body: &[u8]) -> Result<GetSessionRequest, String> {
    if body.is_empty() {
        return Ok(GetSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_statement_request(body: &[u8]) -> Result<GetStatementRequest, String> {
    if body.is_empty() {
        return Ok(GetStatementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_request(body: &[u8]) -> Result<GetTableRequest, String> {
    if body.is_empty() {
        return Ok(GetTableRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_optimizer_request(
    body: &[u8],
) -> Result<GetTableOptimizerRequest, String> {
    if body.is_empty() {
        return Ok(GetTableOptimizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTableOptimizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_version_request(
    body: &[u8],
) -> Result<GetTableVersionRequest, String> {
    if body.is_empty() {
        return Ok(GetTableVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTableVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_versions_request(
    body: &[u8],
) -> Result<GetTableVersionsRequest, String> {
    if body.is_empty() {
        return Ok(GetTableVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTableVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_tables_request(body: &[u8]) -> Result<GetTablesRequest, String> {
    if body.is_empty() {
        return Ok(GetTablesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTables request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_tags_request(body: &[u8]) -> Result<GetTagsRequest, String> {
    if body.is_empty() {
        return Ok(GetTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_trigger_request(body: &[u8]) -> Result<GetTriggerRequest, String> {
    if body.is_empty() {
        return Ok(GetTriggerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTrigger request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_triggers_request(body: &[u8]) -> Result<GetTriggersRequest, String> {
    if body.is_empty() {
        return Ok(GetTriggersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTriggers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_unfiltered_partition_metadata_request(
    body: &[u8],
) -> Result<GetUnfilteredPartitionMetadataRequest, String> {
    if body.is_empty() {
        return Ok(GetUnfilteredPartitionMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUnfilteredPartitionMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_unfiltered_partitions_metadata_request(
    body: &[u8],
) -> Result<GetUnfilteredPartitionsMetadataRequest, String> {
    if body.is_empty() {
        return Ok(GetUnfilteredPartitionsMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUnfilteredPartitionsMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_unfiltered_table_metadata_request(
    body: &[u8],
) -> Result<GetUnfilteredTableMetadataRequest, String> {
    if body.is_empty() {
        return Ok(GetUnfilteredTableMetadataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUnfilteredTableMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_usage_profile_request(
    body: &[u8],
) -> Result<GetUsageProfileRequest, String> {
    if body.is_empty() {
        return Ok(GetUsageProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUsageProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_user_defined_function_request(
    body: &[u8],
) -> Result<GetUserDefinedFunctionRequest, String> {
    if body.is_empty() {
        return Ok(GetUserDefinedFunctionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUserDefinedFunction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_user_defined_functions_request(
    body: &[u8],
) -> Result<GetUserDefinedFunctionsRequest, String> {
    if body.is_empty() {
        return Ok(GetUserDefinedFunctionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUserDefinedFunctions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_workflow_request(body: &[u8]) -> Result<GetWorkflowRequest, String> {
    if body.is_empty() {
        return Ok(GetWorkflowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWorkflow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_workflow_run_request(body: &[u8]) -> Result<GetWorkflowRunRequest, String> {
    if body.is_empty() {
        return Ok(GetWorkflowRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWorkflowRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_workflow_run_properties_request(
    body: &[u8],
) -> Result<GetWorkflowRunPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(GetWorkflowRunPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWorkflowRunProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_workflow_runs_request(
    body: &[u8],
) -> Result<GetWorkflowRunsRequest, String> {
    if body.is_empty() {
        return Ok(GetWorkflowRunsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWorkflowRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_catalog_to_glue_request(
    body: &[u8],
) -> Result<ImportCatalogToGlueRequest, String> {
    if body.is_empty() {
        return Ok(ImportCatalogToGlueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportCatalogToGlue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_blueprints_request(body: &[u8]) -> Result<ListBlueprintsRequest, String> {
    if body.is_empty() {
        return Ok(ListBlueprintsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBlueprints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_column_statistics_task_runs_request(
    body: &[u8],
) -> Result<ListColumnStatisticsTaskRunsRequest, String> {
    if body.is_empty() {
        return Ok(ListColumnStatisticsTaskRunsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListColumnStatisticsTaskRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_connection_types_request(
    body: &[u8],
) -> Result<ListConnectionTypesRequest, String> {
    if body.is_empty() {
        return Ok(ListConnectionTypesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListConnectionTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_crawlers_request(body: &[u8]) -> Result<ListCrawlersRequest, String> {
    if body.is_empty() {
        return Ok(ListCrawlersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCrawlers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_crawls_request(body: &[u8]) -> Result<ListCrawlsRequest, String> {
    if body.is_empty() {
        return Ok(ListCrawlsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCrawls request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_custom_entity_types_request(
    body: &[u8],
) -> Result<ListCustomEntityTypesRequest, String> {
    if body.is_empty() {
        return Ok(ListCustomEntityTypesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCustomEntityTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_quality_results_request(
    body: &[u8],
) -> Result<ListDataQualityResultsRequest, String> {
    if body.is_empty() {
        return Ok(ListDataQualityResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDataQualityResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_quality_rule_recommendation_runs_request(
    body: &[u8],
) -> Result<ListDataQualityRuleRecommendationRunsRequest, String> {
    if body.is_empty() {
        return Ok(ListDataQualityRuleRecommendationRunsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListDataQualityRuleRecommendationRuns request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_quality_ruleset_evaluation_runs_request(
    body: &[u8],
) -> Result<ListDataQualityRulesetEvaluationRunsRequest, String> {
    if body.is_empty() {
        return Ok(ListDataQualityRulesetEvaluationRunsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListDataQualityRulesetEvaluationRuns request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_quality_rulesets_request(
    body: &[u8],
) -> Result<ListDataQualityRulesetsRequest, String> {
    if body.is_empty() {
        return Ok(ListDataQualityRulesetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDataQualityRulesets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_quality_statistic_annotations_request(
    body: &[u8],
) -> Result<ListDataQualityStatisticAnnotationsRequest, String> {
    if body.is_empty() {
        return Ok(ListDataQualityStatisticAnnotationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListDataQualityStatisticAnnotations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_quality_statistics_request(
    body: &[u8],
) -> Result<ListDataQualityStatisticsRequest, String> {
    if body.is_empty() {
        return Ok(ListDataQualityStatisticsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDataQualityStatistics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dev_endpoints_request(
    body: &[u8],
) -> Result<ListDevEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(ListDevEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDevEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_entities_request(body: &[u8]) -> Result<ListEntitiesRequest, String> {
    if body.is_empty() {
        return Ok(ListEntitiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEntities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_integration_resource_properties_request(
    body: &[u8],
) -> Result<ListIntegrationResourcePropertiesRequest, String> {
    if body.is_empty() {
        return Ok(ListIntegrationResourcePropertiesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListIntegrationResourceProperties request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_jobs_request(body: &[u8]) -> Result<ListJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListJobsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize ListJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_m_l_transforms_request(
    body: &[u8],
) -> Result<ListMLTransformsRequest, String> {
    if body.is_empty() {
        return Ok(ListMLTransformsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMLTransforms request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_materialized_view_refresh_task_runs_request(
    body: &[u8],
) -> Result<ListMaterializedViewRefreshTaskRunsRequest, String> {
    if body.is_empty() {
        return Ok(ListMaterializedViewRefreshTaskRunsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListMaterializedViewRefreshTaskRuns request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_registries_request(body: &[u8]) -> Result<ListRegistriesInput, String> {
    if body.is_empty() {
        return Ok(ListRegistriesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRegistries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_schema_versions_request(
    body: &[u8],
) -> Result<ListSchemaVersionsInput, String> {
    if body.is_empty() {
        return Ok(ListSchemaVersionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSchemaVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_schemas_request(body: &[u8]) -> Result<ListSchemasInput, String> {
    if body.is_empty() {
        return Ok(ListSchemasInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSchemas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_sessions_request(body: &[u8]) -> Result<ListSessionsRequest, String> {
    if body.is_empty() {
        return Ok(ListSessionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSessions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_statements_request(body: &[u8]) -> Result<ListStatementsRequest, String> {
    if body.is_empty() {
        return Ok(ListStatementsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStatements request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_table_optimizer_runs_request(
    body: &[u8],
) -> Result<ListTableOptimizerRunsRequest, String> {
    if body.is_empty() {
        return Ok(ListTableOptimizerRunsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTableOptimizerRuns request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_triggers_request(body: &[u8]) -> Result<ListTriggersRequest, String> {
    if body.is_empty() {
        return Ok(ListTriggersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTriggers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_usage_profiles_request(
    body: &[u8],
) -> Result<ListUsageProfilesRequest, String> {
    if body.is_empty() {
        return Ok(ListUsageProfilesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUsageProfiles request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_workflows_request(body: &[u8]) -> Result<ListWorkflowsRequest, String> {
    if body.is_empty() {
        return Ok(ListWorkflowsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWorkflows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_integration_request(
    body: &[u8],
) -> Result<ModifyIntegrationRequest, String> {
    if body.is_empty() {
        return Ok(ModifyIntegrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyIntegration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_data_catalog_encryption_settings_request(
    body: &[u8],
) -> Result<PutDataCatalogEncryptionSettingsRequest, String> {
    if body.is_empty() {
        return Ok(PutDataCatalogEncryptionSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDataCatalogEncryptionSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_data_quality_profile_annotation_request(
    body: &[u8],
) -> Result<PutDataQualityProfileAnnotationRequest, String> {
    if body.is_empty() {
        return Ok(PutDataQualityProfileAnnotationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutDataQualityProfileAnnotation request: {e}"))
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
pub fn deserialize_put_schema_version_metadata_request(
    body: &[u8],
) -> Result<PutSchemaVersionMetadataInput, String> {
    if body.is_empty() {
        return Ok(PutSchemaVersionMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutSchemaVersionMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_workflow_run_properties_request(
    body: &[u8],
) -> Result<PutWorkflowRunPropertiesRequest, String> {
    if body.is_empty() {
        return Ok(PutWorkflowRunPropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutWorkflowRunProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_query_schema_version_metadata_request(
    body: &[u8],
) -> Result<QuerySchemaVersionMetadataInput, String> {
    if body.is_empty() {
        return Ok(QuerySchemaVersionMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize QuerySchemaVersionMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_connection_type_request(
    body: &[u8],
) -> Result<RegisterConnectionTypeRequest, String> {
    if body.is_empty() {
        return Ok(RegisterConnectionTypeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterConnectionType request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_schema_version_request(
    body: &[u8],
) -> Result<RegisterSchemaVersionInput, String> {
    if body.is_empty() {
        return Ok(RegisterSchemaVersionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterSchemaVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_schema_version_metadata_request(
    body: &[u8],
) -> Result<RemoveSchemaVersionMetadataInput, String> {
    if body.is_empty() {
        return Ok(RemoveSchemaVersionMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveSchemaVersionMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reset_job_bookmark_request(
    body: &[u8],
) -> Result<ResetJobBookmarkRequest, String> {
    if body.is_empty() {
        return Ok(ResetJobBookmarkRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResetJobBookmark request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resume_workflow_run_request(
    body: &[u8],
) -> Result<ResumeWorkflowRunRequest, String> {
    if body.is_empty() {
        return Ok(ResumeWorkflowRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResumeWorkflowRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_run_statement_request(body: &[u8]) -> Result<RunStatementRequest, String> {
    if body.is_empty() {
        return Ok(RunStatementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RunStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_tables_request(body: &[u8]) -> Result<SearchTablesRequest, String> {
    if body.is_empty() {
        return Ok(SearchTablesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchTables request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_blueprint_run_request(
    body: &[u8],
) -> Result<StartBlueprintRunRequest, String> {
    if body.is_empty() {
        return Ok(StartBlueprintRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartBlueprintRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_column_statistics_task_run_request(
    body: &[u8],
) -> Result<StartColumnStatisticsTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StartColumnStatisticsTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartColumnStatisticsTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_column_statistics_task_run_schedule_request(
    body: &[u8],
) -> Result<StartColumnStatisticsTaskRunScheduleRequest, String> {
    if body.is_empty() {
        return Ok(StartColumnStatisticsTaskRunScheduleRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartColumnStatisticsTaskRunSchedule request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_crawler_request(body: &[u8]) -> Result<StartCrawlerRequest, String> {
    if body.is_empty() {
        return Ok(StartCrawlerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCrawler request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_crawler_schedule_request(
    body: &[u8],
) -> Result<StartCrawlerScheduleRequest, String> {
    if body.is_empty() {
        return Ok(StartCrawlerScheduleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCrawlerSchedule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_data_quality_rule_recommendation_run_request(
    body: &[u8],
) -> Result<StartDataQualityRuleRecommendationRunRequest, String> {
    if body.is_empty() {
        return Ok(StartDataQualityRuleRecommendationRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartDataQualityRuleRecommendationRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_data_quality_ruleset_evaluation_run_request(
    body: &[u8],
) -> Result<StartDataQualityRulesetEvaluationRunRequest, String> {
    if body.is_empty() {
        return Ok(StartDataQualityRulesetEvaluationRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartDataQualityRulesetEvaluationRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_export_labels_task_run_request(
    body: &[u8],
) -> Result<StartExportLabelsTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StartExportLabelsTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartExportLabelsTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_import_labels_task_run_request(
    body: &[u8],
) -> Result<StartImportLabelsTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StartImportLabelsTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartImportLabelsTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_job_run_request(body: &[u8]) -> Result<StartJobRunRequest, String> {
    if body.is_empty() {
        return Ok(StartJobRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartJobRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_m_l_evaluation_task_run_request(
    body: &[u8],
) -> Result<StartMLEvaluationTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StartMLEvaluationTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMLEvaluationTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_m_l_labeling_set_generation_task_run_request(
    body: &[u8],
) -> Result<StartMLLabelingSetGenerationTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StartMLLabelingSetGenerationTaskRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartMLLabelingSetGenerationTaskRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_materialized_view_refresh_task_run_request(
    body: &[u8],
) -> Result<StartMaterializedViewRefreshTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StartMaterializedViewRefreshTaskRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartMaterializedViewRefreshTaskRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_trigger_request(body: &[u8]) -> Result<StartTriggerRequest, String> {
    if body.is_empty() {
        return Ok(StartTriggerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartTrigger request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_workflow_run_request(
    body: &[u8],
) -> Result<StartWorkflowRunRequest, String> {
    if body.is_empty() {
        return Ok(StartWorkflowRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartWorkflowRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_column_statistics_task_run_request(
    body: &[u8],
) -> Result<StopColumnStatisticsTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StopColumnStatisticsTaskRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopColumnStatisticsTaskRun request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_column_statistics_task_run_schedule_request(
    body: &[u8],
) -> Result<StopColumnStatisticsTaskRunScheduleRequest, String> {
    if body.is_empty() {
        return Ok(StopColumnStatisticsTaskRunScheduleRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StopColumnStatisticsTaskRunSchedule request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_crawler_request(body: &[u8]) -> Result<StopCrawlerRequest, String> {
    if body.is_empty() {
        return Ok(StopCrawlerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopCrawler request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_crawler_schedule_request(
    body: &[u8],
) -> Result<StopCrawlerScheduleRequest, String> {
    if body.is_empty() {
        return Ok(StopCrawlerScheduleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopCrawlerSchedule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_materialized_view_refresh_task_run_request(
    body: &[u8],
) -> Result<StopMaterializedViewRefreshTaskRunRequest, String> {
    if body.is_empty() {
        return Ok(StopMaterializedViewRefreshTaskRunRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StopMaterializedViewRefreshTaskRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_session_request(body: &[u8]) -> Result<StopSessionRequest, String> {
    if body.is_empty() {
        return Ok(StopSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_trigger_request(body: &[u8]) -> Result<StopTriggerRequest, String> {
    if body.is_empty() {
        return Ok(StopTriggerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopTrigger request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_workflow_run_request(
    body: &[u8],
) -> Result<StopWorkflowRunRequest, String> {
    if body.is_empty() {
        return Ok(StopWorkflowRunRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopWorkflowRun request: {e}"))
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
pub fn deserialize_test_connection_request(body: &[u8]) -> Result<TestConnectionRequest, String> {
    if body.is_empty() {
        return Ok(TestConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestConnection request: {e}"))
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
pub fn deserialize_update_blueprint_request(body: &[u8]) -> Result<UpdateBlueprintRequest, String> {
    if body.is_empty() {
        return Ok(UpdateBlueprintRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateBlueprint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_catalog_request(body: &[u8]) -> Result<UpdateCatalogRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCatalogRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_classifier_request(
    body: &[u8],
) -> Result<UpdateClassifierRequest, String> {
    if body.is_empty() {
        return Ok(UpdateClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_column_statistics_for_partition_request(
    body: &[u8],
) -> Result<UpdateColumnStatisticsForPartitionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateColumnStatisticsForPartitionRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateColumnStatisticsForPartition request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_column_statistics_for_table_request(
    body: &[u8],
) -> Result<UpdateColumnStatisticsForTableRequest, String> {
    if body.is_empty() {
        return Ok(UpdateColumnStatisticsForTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateColumnStatisticsForTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_column_statistics_task_settings_request(
    body: &[u8],
) -> Result<UpdateColumnStatisticsTaskSettingsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateColumnStatisticsTaskSettingsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateColumnStatisticsTaskSettings request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_connection_request(
    body: &[u8],
) -> Result<UpdateConnectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_crawler_request(body: &[u8]) -> Result<UpdateCrawlerRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCrawlerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCrawler request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_crawler_schedule_request(
    body: &[u8],
) -> Result<UpdateCrawlerScheduleRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCrawlerScheduleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCrawlerSchedule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_data_quality_ruleset_request(
    body: &[u8],
) -> Result<UpdateDataQualityRulesetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDataQualityRulesetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDataQualityRuleset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_database_request(body: &[u8]) -> Result<UpdateDatabaseRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDatabaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDatabase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_dev_endpoint_request(
    body: &[u8],
) -> Result<UpdateDevEndpointRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDevEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDevEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_glue_identity_center_configuration_request(
    body: &[u8],
) -> Result<UpdateGlueIdentityCenterConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateGlueIdentityCenterConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateGlueIdentityCenterConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_integration_resource_property_request(
    body: &[u8],
) -> Result<UpdateIntegrationResourcePropertyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateIntegrationResourcePropertyRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateIntegrationResourceProperty request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_integration_table_properties_request(
    body: &[u8],
) -> Result<UpdateIntegrationTablePropertiesRequest, String> {
    if body.is_empty() {
        return Ok(UpdateIntegrationTablePropertiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateIntegrationTableProperties request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_job_request(body: &[u8]) -> Result<UpdateJobRequest, String> {
    if body.is_empty() {
        return Ok(UpdateJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_job_from_source_control_request(
    body: &[u8],
) -> Result<UpdateJobFromSourceControlRequest, String> {
    if body.is_empty() {
        return Ok(UpdateJobFromSourceControlRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateJobFromSourceControl request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_m_l_transform_request(
    body: &[u8],
) -> Result<UpdateMLTransformRequest, String> {
    if body.is_empty() {
        return Ok(UpdateMLTransformRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMLTransform request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_partition_request(body: &[u8]) -> Result<UpdatePartitionRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePartitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePartition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_registry_request(body: &[u8]) -> Result<UpdateRegistryInput, String> {
    if body.is_empty() {
        return Ok(UpdateRegistryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRegistry request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_schema_request(body: &[u8]) -> Result<UpdateSchemaInput, String> {
    if body.is_empty() {
        return Ok(UpdateSchemaInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_source_control_from_job_request(
    body: &[u8],
) -> Result<UpdateSourceControlFromJobRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSourceControlFromJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSourceControlFromJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_table_request(body: &[u8]) -> Result<UpdateTableRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_table_optimizer_request(
    body: &[u8],
) -> Result<UpdateTableOptimizerRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTableOptimizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTableOptimizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_trigger_request(body: &[u8]) -> Result<UpdateTriggerRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTriggerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTrigger request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_usage_profile_request(
    body: &[u8],
) -> Result<UpdateUsageProfileRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUsageProfileRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUsageProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_defined_function_request(
    body: &[u8],
) -> Result<UpdateUserDefinedFunctionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserDefinedFunctionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUserDefinedFunction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_workflow_request(body: &[u8]) -> Result<UpdateWorkflowRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWorkflowRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWorkflow request: {e}"))
}
