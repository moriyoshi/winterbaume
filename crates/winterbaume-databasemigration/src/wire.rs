//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-databasemigration

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_tags_to_resource_response(result: &AddTagsToResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_apply_pending_maintenance_action_response(
    result: &ApplyPendingMaintenanceActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_start_recommendations_response(
    result: &BatchStartRecommendationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_metadata_model_conversion_response(
    result: &CancelMetadataModelConversionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_metadata_model_creation_response(
    result: &CancelMetadataModelCreationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_replication_task_assessment_run_response(
    result: &CancelReplicationTaskAssessmentRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_data_migration_response(
    result: &CreateDataMigrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_data_provider_response(
    result: &CreateDataProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_endpoint_response(result: &CreateEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_event_subscription_response(
    result: &CreateEventSubscriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_fleet_advisor_collector_response(
    result: &CreateFleetAdvisorCollectorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_instance_profile_response(
    result: &CreateInstanceProfileResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_migration_project_response(
    result: &CreateMigrationProjectResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_replication_config_response(
    result: &CreateReplicationConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_replication_instance_response(
    result: &CreateReplicationInstanceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_replication_subnet_group_response(
    result: &CreateReplicationSubnetGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_replication_task_response(
    result: &CreateReplicationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_certificate_response(result: &DeleteCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_connection_response(result: &DeleteConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_data_migration_response(
    result: &DeleteDataMigrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_data_provider_response(
    result: &DeleteDataProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_endpoint_response(result: &DeleteEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_event_subscription_response(
    result: &DeleteEventSubscriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_fleet_advisor_collector_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_fleet_advisor_databases_response(
    result: &DeleteFleetAdvisorDatabasesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_instance_profile_response(
    result: &DeleteInstanceProfileResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_migration_project_response(
    result: &DeleteMigrationProjectResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_replication_config_response(
    result: &DeleteReplicationConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_replication_instance_response(
    result: &DeleteReplicationInstanceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_replication_subnet_group_response(
    result: &DeleteReplicationSubnetGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_replication_task_response(
    result: &DeleteReplicationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_replication_task_assessment_run_response(
    result: &DeleteReplicationTaskAssessmentRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_attributes_response(
    result: &DescribeAccountAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_applicable_individual_assessments_response(
    result: &DescribeApplicableIndividualAssessmentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_certificates_response(
    result: &DescribeCertificatesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connections_response(
    result: &DescribeConnectionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_conversion_configuration_response(
    result: &DescribeConversionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_data_migrations_response(
    result: &DescribeDataMigrationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_data_providers_response(
    result: &DescribeDataProvidersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoint_settings_response(
    result: &DescribeEndpointSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoint_types_response(
    result: &DescribeEndpointTypesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoints_response(result: &DescribeEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_engine_versions_response(
    result: &DescribeEngineVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_event_categories_response(
    result: &DescribeEventCategoriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_event_subscriptions_response(
    result: &DescribeEventSubscriptionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_events_response(result: &DescribeEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_extension_pack_associations_response(
    result: &DescribeExtensionPackAssociationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_fleet_advisor_collectors_response(
    result: &DescribeFleetAdvisorCollectorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_fleet_advisor_databases_response(
    result: &DescribeFleetAdvisorDatabasesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_fleet_advisor_lsa_analysis_response(
    result: &DescribeFleetAdvisorLsaAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_fleet_advisor_schema_object_summary_response(
    result: &DescribeFleetAdvisorSchemaObjectSummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_fleet_advisor_schemas_response(
    result: &DescribeFleetAdvisorSchemasResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_instance_profiles_response(
    result: &DescribeInstanceProfilesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_response(
    result: &DescribeMetadataModelResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_assessments_response(
    result: &DescribeMetadataModelAssessmentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_children_response(
    result: &DescribeMetadataModelChildrenResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_conversions_response(
    result: &DescribeMetadataModelConversionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_creations_response(
    result: &DescribeMetadataModelCreationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_exports_as_script_response(
    result: &DescribeMetadataModelExportsAsScriptResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_exports_to_target_response(
    result: &DescribeMetadataModelExportsToTargetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_metadata_model_imports_response(
    result: &DescribeMetadataModelImportsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_migration_projects_response(
    result: &DescribeMigrationProjectsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_orderable_replication_instances_response(
    result: &DescribeOrderableReplicationInstancesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_pending_maintenance_actions_response(
    result: &DescribePendingMaintenanceActionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_recommendation_limitations_response(
    result: &DescribeRecommendationLimitationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_recommendations_response(
    result: &DescribeRecommendationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_refresh_schemas_status_response(
    result: &DescribeRefreshSchemasStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_configs_response(
    result: &DescribeReplicationConfigsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_instance_task_logs_response(
    result: &DescribeReplicationInstanceTaskLogsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_instances_response(
    result: &DescribeReplicationInstancesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_subnet_groups_response(
    result: &DescribeReplicationSubnetGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_table_statistics_response(
    result: &DescribeReplicationTableStatisticsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_task_assessment_results_response(
    result: &DescribeReplicationTaskAssessmentResultsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_task_assessment_runs_response(
    result: &DescribeReplicationTaskAssessmentRunsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_task_individual_assessments_response(
    result: &DescribeReplicationTaskIndividualAssessmentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replication_tasks_response(
    result: &DescribeReplicationTasksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replications_response(
    result: &DescribeReplicationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_schemas_response(result: &DescribeSchemasResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_table_statistics_response(
    result: &DescribeTableStatisticsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_export_metadata_model_assessment_response(
    result: &ExportMetadataModelAssessmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_target_selection_rules_response(
    result: &GetTargetSelectionRulesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_certificate_response(result: &ImportCertificateResponse) -> MockResponse {
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
pub fn serialize_modify_conversion_configuration_response(
    result: &ModifyConversionConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_data_migration_response(
    result: &ModifyDataMigrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_data_provider_response(
    result: &ModifyDataProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_endpoint_response(result: &ModifyEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_event_subscription_response(
    result: &ModifyEventSubscriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_instance_profile_response(
    result: &ModifyInstanceProfileResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_migration_project_response(
    result: &ModifyMigrationProjectResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_replication_config_response(
    result: &ModifyReplicationConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_replication_instance_response(
    result: &ModifyReplicationInstanceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_replication_subnet_group_response(
    result: &ModifyReplicationSubnetGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_modify_replication_task_response(
    result: &ModifyReplicationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_move_replication_task_response(
    result: &MoveReplicationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reboot_replication_instance_response(
    result: &RebootReplicationInstanceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_refresh_schemas_response(result: &RefreshSchemasResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reload_replication_tables_response(
    result: &ReloadReplicationTablesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reload_tables_response(result: &ReloadTablesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_tags_from_resource_response(
    result: &RemoveTagsFromResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_run_fleet_advisor_lsa_analysis_response(
    result: &RunFleetAdvisorLsaAnalysisResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_data_migration_response(
    result: &StartDataMigrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_extension_pack_association_response(
    result: &StartExtensionPackAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_metadata_model_assessment_response(
    result: &StartMetadataModelAssessmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_metadata_model_conversion_response(
    result: &StartMetadataModelConversionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_metadata_model_creation_response(
    result: &StartMetadataModelCreationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_metadata_model_export_as_script_response(
    result: &StartMetadataModelExportAsScriptResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_metadata_model_export_to_target_response(
    result: &StartMetadataModelExportToTargetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_metadata_model_import_response(
    result: &StartMetadataModelImportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_start_recommendations_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_replication_response(result: &StartReplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_replication_task_response(
    result: &StartReplicationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_replication_task_assessment_response(
    result: &StartReplicationTaskAssessmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_replication_task_assessment_run_response(
    result: &StartReplicationTaskAssessmentRunResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_data_migration_response(result: &StopDataMigrationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_replication_response(result: &StopReplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_replication_task_response(
    result: &StopReplicationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_connection_response(result: &TestConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_subscriptions_to_event_bridge_response(
    result: &UpdateSubscriptionsToEventBridgeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_to_resource_request(
    body: &[u8],
) -> Result<AddTagsToResourceMessage, String> {
    if body.is_empty() {
        return Ok(AddTagsToResourceMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddTagsToResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_apply_pending_maintenance_action_request(
    body: &[u8],
) -> Result<ApplyPendingMaintenanceActionMessage, String> {
    if body.is_empty() {
        return Ok(ApplyPendingMaintenanceActionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ApplyPendingMaintenanceAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_start_recommendations_request(
    body: &[u8],
) -> Result<BatchStartRecommendationsRequest, String> {
    if body.is_empty() {
        return Ok(BatchStartRecommendationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchStartRecommendations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_metadata_model_conversion_request(
    body: &[u8],
) -> Result<CancelMetadataModelConversionMessage, String> {
    if body.is_empty() {
        return Ok(CancelMetadataModelConversionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelMetadataModelConversion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_metadata_model_creation_request(
    body: &[u8],
) -> Result<CancelMetadataModelCreationMessage, String> {
    if body.is_empty() {
        return Ok(CancelMetadataModelCreationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelMetadataModelCreation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_replication_task_assessment_run_request(
    body: &[u8],
) -> Result<CancelReplicationTaskAssessmentRunMessage, String> {
    if body.is_empty() {
        return Ok(CancelReplicationTaskAssessmentRunMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CancelReplicationTaskAssessmentRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_data_migration_request(
    body: &[u8],
) -> Result<CreateDataMigrationMessage, String> {
    if body.is_empty() {
        return Ok(CreateDataMigrationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataMigration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_data_provider_request(
    body: &[u8],
) -> Result<CreateDataProviderMessage, String> {
    if body.is_empty() {
        return Ok(CreateDataProviderMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_endpoint_request(body: &[u8]) -> Result<CreateEndpointMessage, String> {
    if body.is_empty() {
        return Ok(CreateEndpointMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_event_subscription_request(
    body: &[u8],
) -> Result<CreateEventSubscriptionMessage, String> {
    if body.is_empty() {
        return Ok(CreateEventSubscriptionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEventSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_fleet_advisor_collector_request(
    body: &[u8],
) -> Result<CreateFleetAdvisorCollectorRequest, String> {
    if body.is_empty() {
        return Ok(CreateFleetAdvisorCollectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFleetAdvisorCollector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_instance_profile_request(
    body: &[u8],
) -> Result<CreateInstanceProfileMessage, String> {
    if body.is_empty() {
        return Ok(CreateInstanceProfileMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateInstanceProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_migration_project_request(
    body: &[u8],
) -> Result<CreateMigrationProjectMessage, String> {
    if body.is_empty() {
        return Ok(CreateMigrationProjectMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMigrationProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_replication_config_request(
    body: &[u8],
) -> Result<CreateReplicationConfigMessage, String> {
    if body.is_empty() {
        return Ok(CreateReplicationConfigMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateReplicationConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_replication_instance_request(
    body: &[u8],
) -> Result<CreateReplicationInstanceMessage, String> {
    if body.is_empty() {
        return Ok(CreateReplicationInstanceMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateReplicationInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_replication_subnet_group_request(
    body: &[u8],
) -> Result<CreateReplicationSubnetGroupMessage, String> {
    if body.is_empty() {
        return Ok(CreateReplicationSubnetGroupMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateReplicationSubnetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_replication_task_request(
    body: &[u8],
) -> Result<CreateReplicationTaskMessage, String> {
    if body.is_empty() {
        return Ok(CreateReplicationTaskMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateReplicationTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_certificate_request(
    body: &[u8],
) -> Result<DeleteCertificateMessage, String> {
    if body.is_empty() {
        return Ok(DeleteCertificateMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connection_request(
    body: &[u8],
) -> Result<DeleteConnectionMessage, String> {
    if body.is_empty() {
        return Ok(DeleteConnectionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_data_migration_request(
    body: &[u8],
) -> Result<DeleteDataMigrationMessage, String> {
    if body.is_empty() {
        return Ok(DeleteDataMigrationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataMigration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_data_provider_request(
    body: &[u8],
) -> Result<DeleteDataProviderMessage, String> {
    if body.is_empty() {
        return Ok(DeleteDataProviderMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_endpoint_request(body: &[u8]) -> Result<DeleteEndpointMessage, String> {
    if body.is_empty() {
        return Ok(DeleteEndpointMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_event_subscription_request(
    body: &[u8],
) -> Result<DeleteEventSubscriptionMessage, String> {
    if body.is_empty() {
        return Ok(DeleteEventSubscriptionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEventSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_fleet_advisor_collector_request(
    body: &[u8],
) -> Result<DeleteCollectorRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCollectorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFleetAdvisorCollector request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_fleet_advisor_databases_request(
    body: &[u8],
) -> Result<DeleteFleetAdvisorDatabasesRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFleetAdvisorDatabasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFleetAdvisorDatabases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_instance_profile_request(
    body: &[u8],
) -> Result<DeleteInstanceProfileMessage, String> {
    if body.is_empty() {
        return Ok(DeleteInstanceProfileMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteInstanceProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_migration_project_request(
    body: &[u8],
) -> Result<DeleteMigrationProjectMessage, String> {
    if body.is_empty() {
        return Ok(DeleteMigrationProjectMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMigrationProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_replication_config_request(
    body: &[u8],
) -> Result<DeleteReplicationConfigMessage, String> {
    if body.is_empty() {
        return Ok(DeleteReplicationConfigMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteReplicationConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_replication_instance_request(
    body: &[u8],
) -> Result<DeleteReplicationInstanceMessage, String> {
    if body.is_empty() {
        return Ok(DeleteReplicationInstanceMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteReplicationInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_replication_subnet_group_request(
    body: &[u8],
) -> Result<DeleteReplicationSubnetGroupMessage, String> {
    if body.is_empty() {
        return Ok(DeleteReplicationSubnetGroupMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteReplicationSubnetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_replication_task_request(
    body: &[u8],
) -> Result<DeleteReplicationTaskMessage, String> {
    if body.is_empty() {
        return Ok(DeleteReplicationTaskMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteReplicationTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_replication_task_assessment_run_request(
    body: &[u8],
) -> Result<DeleteReplicationTaskAssessmentRunMessage, String> {
    if body.is_empty() {
        return Ok(DeleteReplicationTaskAssessmentRunMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteReplicationTaskAssessmentRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_attributes_request(
    body: &[u8],
) -> Result<DescribeAccountAttributesMessage, String> {
    if body.is_empty() {
        return Ok(DescribeAccountAttributesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccountAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_applicable_individual_assessments_request(
    body: &[u8],
) -> Result<DescribeApplicableIndividualAssessmentsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeApplicableIndividualAssessmentsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeApplicableIndividualAssessments request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_certificates_request(
    body: &[u8],
) -> Result<DescribeCertificatesMessage, String> {
    if body.is_empty() {
        return Ok(DescribeCertificatesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCertificates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connections_request(
    body: &[u8],
) -> Result<DescribeConnectionsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_conversion_configuration_request(
    body: &[u8],
) -> Result<DescribeConversionConfigurationMessage, String> {
    if body.is_empty() {
        return Ok(DescribeConversionConfigurationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConversionConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_data_migrations_request(
    body: &[u8],
) -> Result<DescribeDataMigrationsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeDataMigrationsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDataMigrations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_data_providers_request(
    body: &[u8],
) -> Result<DescribeDataProvidersMessage, String> {
    if body.is_empty() {
        return Ok(DescribeDataProvidersMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDataProviders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_endpoint_settings_request(
    body: &[u8],
) -> Result<DescribeEndpointSettingsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeEndpointSettingsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEndpointSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_endpoint_types_request(
    body: &[u8],
) -> Result<DescribeEndpointTypesMessage, String> {
    if body.is_empty() {
        return Ok(DescribeEndpointTypesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEndpointTypes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_endpoints_request(
    body: &[u8],
) -> Result<DescribeEndpointsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeEndpointsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_engine_versions_request(
    body: &[u8],
) -> Result<DescribeEngineVersionsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeEngineVersionsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEngineVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_event_categories_request(
    body: &[u8],
) -> Result<DescribeEventCategoriesMessage, String> {
    if body.is_empty() {
        return Ok(DescribeEventCategoriesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEventCategories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_event_subscriptions_request(
    body: &[u8],
) -> Result<DescribeEventSubscriptionsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeEventSubscriptionsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEventSubscriptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_events_request(body: &[u8]) -> Result<DescribeEventsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeEventsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_extension_pack_associations_request(
    body: &[u8],
) -> Result<DescribeExtensionPackAssociationsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeExtensionPackAssociationsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeExtensionPackAssociations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_fleet_advisor_collectors_request(
    body: &[u8],
) -> Result<DescribeFleetAdvisorCollectorsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFleetAdvisorCollectorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFleetAdvisorCollectors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_fleet_advisor_databases_request(
    body: &[u8],
) -> Result<DescribeFleetAdvisorDatabasesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFleetAdvisorDatabasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFleetAdvisorDatabases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_fleet_advisor_lsa_analysis_request(
    body: &[u8],
) -> Result<DescribeFleetAdvisorLsaAnalysisRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFleetAdvisorLsaAnalysisRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFleetAdvisorLsaAnalysis request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_fleet_advisor_schema_object_summary_request(
    body: &[u8],
) -> Result<DescribeFleetAdvisorSchemaObjectSummaryRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFleetAdvisorSchemaObjectSummaryRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeFleetAdvisorSchemaObjectSummary request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_fleet_advisor_schemas_request(
    body: &[u8],
) -> Result<DescribeFleetAdvisorSchemasRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFleetAdvisorSchemasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFleetAdvisorSchemas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_instance_profiles_request(
    body: &[u8],
) -> Result<DescribeInstanceProfilesMessage, String> {
    if body.is_empty() {
        return Ok(DescribeInstanceProfilesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInstanceProfiles request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_request(
    body: &[u8],
) -> Result<DescribeMetadataModelMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetadataModel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_assessments_request(
    body: &[u8],
) -> Result<DescribeMetadataModelAssessmentsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelAssessmentsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetadataModelAssessments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_children_request(
    body: &[u8],
) -> Result<DescribeMetadataModelChildrenMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelChildrenMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetadataModelChildren request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_conversions_request(
    body: &[u8],
) -> Result<DescribeMetadataModelConversionsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelConversionsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetadataModelConversions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_creations_request(
    body: &[u8],
) -> Result<DescribeMetadataModelCreationsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelCreationsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetadataModelCreations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_exports_as_script_request(
    body: &[u8],
) -> Result<DescribeMetadataModelExportsAsScriptMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelExportsAsScriptMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeMetadataModelExportsAsScript request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_exports_to_target_request(
    body: &[u8],
) -> Result<DescribeMetadataModelExportsToTargetMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelExportsToTargetMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeMetadataModelExportsToTarget request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_metadata_model_imports_request(
    body: &[u8],
) -> Result<DescribeMetadataModelImportsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMetadataModelImportsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMetadataModelImports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_migration_projects_request(
    body: &[u8],
) -> Result<DescribeMigrationProjectsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeMigrationProjectsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeMigrationProjects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_orderable_replication_instances_request(
    body: &[u8],
) -> Result<DescribeOrderableReplicationInstancesMessage, String> {
    if body.is_empty() {
        return Ok(DescribeOrderableReplicationInstancesMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeOrderableReplicationInstances request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_pending_maintenance_actions_request(
    body: &[u8],
) -> Result<DescribePendingMaintenanceActionsMessage, String> {
    if body.is_empty() {
        return Ok(DescribePendingMaintenanceActionsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribePendingMaintenanceActions request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_recommendation_limitations_request(
    body: &[u8],
) -> Result<DescribeRecommendationLimitationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRecommendationLimitationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeRecommendationLimitations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_recommendations_request(
    body: &[u8],
) -> Result<DescribeRecommendationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRecommendationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRecommendations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_refresh_schemas_status_request(
    body: &[u8],
) -> Result<DescribeRefreshSchemasStatusMessage, String> {
    if body.is_empty() {
        return Ok(DescribeRefreshSchemasStatusMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRefreshSchemasStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_configs_request(
    body: &[u8],
) -> Result<DescribeReplicationConfigsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationConfigsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReplicationConfigs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_instance_task_logs_request(
    body: &[u8],
) -> Result<DescribeReplicationInstanceTaskLogsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationInstanceTaskLogsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeReplicationInstanceTaskLogs request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_instances_request(
    body: &[u8],
) -> Result<DescribeReplicationInstancesMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationInstancesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReplicationInstances request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_subnet_groups_request(
    body: &[u8],
) -> Result<DescribeReplicationSubnetGroupsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationSubnetGroupsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReplicationSubnetGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_table_statistics_request(
    body: &[u8],
) -> Result<DescribeReplicationTableStatisticsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationTableStatisticsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeReplicationTableStatistics request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_task_assessment_results_request(
    body: &[u8],
) -> Result<DescribeReplicationTaskAssessmentResultsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationTaskAssessmentResultsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeReplicationTaskAssessmentResults request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_task_assessment_runs_request(
    body: &[u8],
) -> Result<DescribeReplicationTaskAssessmentRunsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationTaskAssessmentRunsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeReplicationTaskAssessmentRuns request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_task_individual_assessments_request(
    body: &[u8],
) -> Result<DescribeReplicationTaskIndividualAssessmentsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationTaskIndividualAssessmentsMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeReplicationTaskIndividualAssessments request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replication_tasks_request(
    body: &[u8],
) -> Result<DescribeReplicationTasksMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationTasksMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReplicationTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replications_request(
    body: &[u8],
) -> Result<DescribeReplicationsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeReplicationsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_schemas_request(body: &[u8]) -> Result<DescribeSchemasMessage, String> {
    if body.is_empty() {
        return Ok(DescribeSchemasMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSchemas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_table_statistics_request(
    body: &[u8],
) -> Result<DescribeTableStatisticsMessage, String> {
    if body.is_empty() {
        return Ok(DescribeTableStatisticsMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTableStatistics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_export_metadata_model_assessment_request(
    body: &[u8],
) -> Result<ExportMetadataModelAssessmentMessage, String> {
    if body.is_empty() {
        return Ok(ExportMetadataModelAssessmentMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExportMetadataModelAssessment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_target_selection_rules_request(
    body: &[u8],
) -> Result<GetTargetSelectionRulesMessage, String> {
    if body.is_empty() {
        return Ok(GetTargetSelectionRulesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTargetSelectionRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_certificate_request(
    body: &[u8],
) -> Result<ImportCertificateMessage, String> {
    if body.is_empty() {
        return Ok(ImportCertificateMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceMessage, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_conversion_configuration_request(
    body: &[u8],
) -> Result<ModifyConversionConfigurationMessage, String> {
    if body.is_empty() {
        return Ok(ModifyConversionConfigurationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyConversionConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_data_migration_request(
    body: &[u8],
) -> Result<ModifyDataMigrationMessage, String> {
    if body.is_empty() {
        return Ok(ModifyDataMigrationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyDataMigration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_data_provider_request(
    body: &[u8],
) -> Result<ModifyDataProviderMessage, String> {
    if body.is_empty() {
        return Ok(ModifyDataProviderMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyDataProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_endpoint_request(body: &[u8]) -> Result<ModifyEndpointMessage, String> {
    if body.is_empty() {
        return Ok(ModifyEndpointMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_event_subscription_request(
    body: &[u8],
) -> Result<ModifyEventSubscriptionMessage, String> {
    if body.is_empty() {
        return Ok(ModifyEventSubscriptionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyEventSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_instance_profile_request(
    body: &[u8],
) -> Result<ModifyInstanceProfileMessage, String> {
    if body.is_empty() {
        return Ok(ModifyInstanceProfileMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyInstanceProfile request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_migration_project_request(
    body: &[u8],
) -> Result<ModifyMigrationProjectMessage, String> {
    if body.is_empty() {
        return Ok(ModifyMigrationProjectMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyMigrationProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_replication_config_request(
    body: &[u8],
) -> Result<ModifyReplicationConfigMessage, String> {
    if body.is_empty() {
        return Ok(ModifyReplicationConfigMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyReplicationConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_replication_instance_request(
    body: &[u8],
) -> Result<ModifyReplicationInstanceMessage, String> {
    if body.is_empty() {
        return Ok(ModifyReplicationInstanceMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyReplicationInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_replication_subnet_group_request(
    body: &[u8],
) -> Result<ModifyReplicationSubnetGroupMessage, String> {
    if body.is_empty() {
        return Ok(ModifyReplicationSubnetGroupMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyReplicationSubnetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_modify_replication_task_request(
    body: &[u8],
) -> Result<ModifyReplicationTaskMessage, String> {
    if body.is_empty() {
        return Ok(ModifyReplicationTaskMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ModifyReplicationTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_move_replication_task_request(
    body: &[u8],
) -> Result<MoveReplicationTaskMessage, String> {
    if body.is_empty() {
        return Ok(MoveReplicationTaskMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MoveReplicationTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reboot_replication_instance_request(
    body: &[u8],
) -> Result<RebootReplicationInstanceMessage, String> {
    if body.is_empty() {
        return Ok(RebootReplicationInstanceMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RebootReplicationInstance request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_refresh_schemas_request(body: &[u8]) -> Result<RefreshSchemasMessage, String> {
    if body.is_empty() {
        return Ok(RefreshSchemasMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RefreshSchemas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reload_replication_tables_request(
    body: &[u8],
) -> Result<ReloadReplicationTablesMessage, String> {
    if body.is_empty() {
        return Ok(ReloadReplicationTablesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReloadReplicationTables request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reload_tables_request(body: &[u8]) -> Result<ReloadTablesMessage, String> {
    if body.is_empty() {
        return Ok(ReloadTablesMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReloadTables request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_tags_from_resource_request(
    body: &[u8],
) -> Result<RemoveTagsFromResourceMessage, String> {
    if body.is_empty() {
        return Ok(RemoveTagsFromResourceMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTagsFromResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_data_migration_request(
    body: &[u8],
) -> Result<StartDataMigrationMessage, String> {
    if body.is_empty() {
        return Ok(StartDataMigrationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDataMigration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_extension_pack_association_request(
    body: &[u8],
) -> Result<StartExtensionPackAssociationMessage, String> {
    if body.is_empty() {
        return Ok(StartExtensionPackAssociationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartExtensionPackAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_metadata_model_assessment_request(
    body: &[u8],
) -> Result<StartMetadataModelAssessmentMessage, String> {
    if body.is_empty() {
        return Ok(StartMetadataModelAssessmentMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMetadataModelAssessment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_metadata_model_conversion_request(
    body: &[u8],
) -> Result<StartMetadataModelConversionMessage, String> {
    if body.is_empty() {
        return Ok(StartMetadataModelConversionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMetadataModelConversion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_metadata_model_creation_request(
    body: &[u8],
) -> Result<StartMetadataModelCreationMessage, String> {
    if body.is_empty() {
        return Ok(StartMetadataModelCreationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMetadataModelCreation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_metadata_model_export_as_script_request(
    body: &[u8],
) -> Result<StartMetadataModelExportAsScriptMessage, String> {
    if body.is_empty() {
        return Ok(StartMetadataModelExportAsScriptMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMetadataModelExportAsScript request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_metadata_model_export_to_target_request(
    body: &[u8],
) -> Result<StartMetadataModelExportToTargetMessage, String> {
    if body.is_empty() {
        return Ok(StartMetadataModelExportToTargetMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMetadataModelExportToTarget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_metadata_model_import_request(
    body: &[u8],
) -> Result<StartMetadataModelImportMessage, String> {
    if body.is_empty() {
        return Ok(StartMetadataModelImportMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMetadataModelImport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_recommendations_request(
    body: &[u8],
) -> Result<StartRecommendationsRequest, String> {
    if body.is_empty() {
        return Ok(StartRecommendationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartRecommendations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_replication_request(
    body: &[u8],
) -> Result<StartReplicationMessage, String> {
    if body.is_empty() {
        return Ok(StartReplicationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartReplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_replication_task_request(
    body: &[u8],
) -> Result<StartReplicationTaskMessage, String> {
    if body.is_empty() {
        return Ok(StartReplicationTaskMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartReplicationTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_replication_task_assessment_request(
    body: &[u8],
) -> Result<StartReplicationTaskAssessmentMessage, String> {
    if body.is_empty() {
        return Ok(StartReplicationTaskAssessmentMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartReplicationTaskAssessment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_replication_task_assessment_run_request(
    body: &[u8],
) -> Result<StartReplicationTaskAssessmentRunMessage, String> {
    if body.is_empty() {
        return Ok(StartReplicationTaskAssessmentRunMessage::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartReplicationTaskAssessmentRun request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_data_migration_request(
    body: &[u8],
) -> Result<StopDataMigrationMessage, String> {
    if body.is_empty() {
        return Ok(StopDataMigrationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopDataMigration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_replication_request(body: &[u8]) -> Result<StopReplicationMessage, String> {
    if body.is_empty() {
        return Ok(StopReplicationMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopReplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_replication_task_request(
    body: &[u8],
) -> Result<StopReplicationTaskMessage, String> {
    if body.is_empty() {
        return Ok(StopReplicationTaskMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopReplicationTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_test_connection_request(body: &[u8]) -> Result<TestConnectionMessage, String> {
    if body.is_empty() {
        return Ok(TestConnectionMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_subscriptions_to_event_bridge_request(
    body: &[u8],
) -> Result<UpdateSubscriptionsToEventBridgeMessage, String> {
    if body.is_empty() {
        return Ok(UpdateSubscriptionsToEventBridgeMessage::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSubscriptionsToEventBridge request: {e}"))
}
