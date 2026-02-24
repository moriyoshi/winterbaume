//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-athena

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_named_query_response(result: &BatchGetNamedQueryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_prepared_statement_response(
    result: &BatchGetPreparedStatementOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_query_execution_response(
    result: &BatchGetQueryExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_capacity_reservation_response(
    result: &CancelCapacityReservationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_capacity_reservation_response(
    result: &CreateCapacityReservationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_data_catalog_response(result: &CreateDataCatalogOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_named_query_response(result: &CreateNamedQueryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_notebook_response(result: &CreateNotebookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_prepared_statement_response(
    result: &CreatePreparedStatementOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_presigned_notebook_url_response(
    result: &CreatePresignedNotebookUrlResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_work_group_response(result: &CreateWorkGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_capacity_reservation_response(
    result: &DeleteCapacityReservationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_data_catalog_response(result: &DeleteDataCatalogOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_named_query_response(result: &DeleteNamedQueryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_notebook_response(result: &DeleteNotebookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_prepared_statement_response(
    result: &DeletePreparedStatementOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_work_group_response(result: &DeleteWorkGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_export_notebook_response(result: &ExportNotebookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_calculation_execution_response(
    result: &GetCalculationExecutionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_calculation_execution_code_response(
    result: &GetCalculationExecutionCodeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_calculation_execution_status_response(
    result: &GetCalculationExecutionStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_capacity_assignment_configuration_response(
    result: &GetCapacityAssignmentConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_capacity_reservation_response(
    result: &GetCapacityReservationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_data_catalog_response(result: &GetDataCatalogOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_database_response(result: &GetDatabaseOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_named_query_response(result: &GetNamedQueryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_notebook_metadata_response(
    result: &GetNotebookMetadataOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_prepared_statement_response(
    result: &GetPreparedStatementOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_query_execution_response(result: &GetQueryExecutionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_query_results_response(result: &GetQueryResultsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_query_runtime_statistics_response(
    result: &GetQueryRuntimeStatisticsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_dashboard_response(
    result: &GetResourceDashboardResponse,
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
pub fn serialize_get_session_endpoint_response(
    result: &GetSessionEndpointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_session_status_response(result: &GetSessionStatusResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_table_metadata_response(result: &GetTableMetadataOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_work_group_response(result: &GetWorkGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_notebook_response(result: &ImportNotebookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_d_p_u_sizes_response(
    result: &ListApplicationDPUSizesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_calculation_executions_response(
    result: &ListCalculationExecutionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_capacity_reservations_response(
    result: &ListCapacityReservationsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_data_catalogs_response(result: &ListDataCatalogsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_databases_response(result: &ListDatabasesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_engine_versions_response(result: &ListEngineVersionsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_executors_response(result: &ListExecutorsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_named_queries_response(result: &ListNamedQueriesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_notebook_metadata_response(
    result: &ListNotebookMetadataOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_notebook_sessions_response(
    result: &ListNotebookSessionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_prepared_statements_response(
    result: &ListPreparedStatementsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_query_executions_response(
    result: &ListQueryExecutionsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_sessions_response(result: &ListSessionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_table_metadata_response(result: &ListTableMetadataOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_work_groups_response(result: &ListWorkGroupsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_capacity_assignment_configuration_response(
    result: &PutCapacityAssignmentConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_calculation_execution_response(
    result: &StartCalculationExecutionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_query_execution_response(
    result: &StartQueryExecutionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_session_response(result: &StartSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_calculation_execution_response(
    result: &StopCalculationExecutionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_query_execution_response(result: &StopQueryExecutionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_terminate_session_response(result: &TerminateSessionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_capacity_reservation_response(
    result: &UpdateCapacityReservationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_data_catalog_response(result: &UpdateDataCatalogOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_named_query_response(result: &UpdateNamedQueryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_notebook_response(result: &UpdateNotebookOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_notebook_metadata_response(
    result: &UpdateNotebookMetadataOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_prepared_statement_response(
    result: &UpdatePreparedStatementOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_work_group_response(result: &UpdateWorkGroupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_named_query_request(
    body: &[u8],
) -> Result<BatchGetNamedQueryInput, String> {
    if body.is_empty() {
        return Ok(BatchGetNamedQueryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetNamedQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_prepared_statement_request(
    body: &[u8],
) -> Result<BatchGetPreparedStatementInput, String> {
    if body.is_empty() {
        return Ok(BatchGetPreparedStatementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetPreparedStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_query_execution_request(
    body: &[u8],
) -> Result<BatchGetQueryExecutionInput, String> {
    if body.is_empty() {
        return Ok(BatchGetQueryExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetQueryExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_capacity_reservation_request(
    body: &[u8],
) -> Result<CancelCapacityReservationInput, String> {
    if body.is_empty() {
        return Ok(CancelCapacityReservationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelCapacityReservation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_capacity_reservation_request(
    body: &[u8],
) -> Result<CreateCapacityReservationInput, String> {
    if body.is_empty() {
        return Ok(CreateCapacityReservationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCapacityReservation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_data_catalog_request(
    body: &[u8],
) -> Result<CreateDataCatalogInput, String> {
    if body.is_empty() {
        return Ok(CreateDataCatalogInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_named_query_request(
    body: &[u8],
) -> Result<CreateNamedQueryInput, String> {
    if body.is_empty() {
        return Ok(CreateNamedQueryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateNamedQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_notebook_request(body: &[u8]) -> Result<CreateNotebookInput, String> {
    if body.is_empty() {
        return Ok(CreateNotebookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateNotebook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_prepared_statement_request(
    body: &[u8],
) -> Result<CreatePreparedStatementInput, String> {
    if body.is_empty() {
        return Ok(CreatePreparedStatementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePreparedStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_presigned_notebook_url_request(
    body: &[u8],
) -> Result<CreatePresignedNotebookUrlRequest, String> {
    if body.is_empty() {
        return Ok(CreatePresignedNotebookUrlRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePresignedNotebookUrl request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_work_group_request(body: &[u8]) -> Result<CreateWorkGroupInput, String> {
    if body.is_empty() {
        return Ok(CreateWorkGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWorkGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_capacity_reservation_request(
    body: &[u8],
) -> Result<DeleteCapacityReservationInput, String> {
    if body.is_empty() {
        return Ok(DeleteCapacityReservationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCapacityReservation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_data_catalog_request(
    body: &[u8],
) -> Result<DeleteDataCatalogInput, String> {
    if body.is_empty() {
        return Ok(DeleteDataCatalogInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_named_query_request(
    body: &[u8],
) -> Result<DeleteNamedQueryInput, String> {
    if body.is_empty() {
        return Ok(DeleteNamedQueryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteNamedQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_notebook_request(body: &[u8]) -> Result<DeleteNotebookInput, String> {
    if body.is_empty() {
        return Ok(DeleteNotebookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteNotebook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_prepared_statement_request(
    body: &[u8],
) -> Result<DeletePreparedStatementInput, String> {
    if body.is_empty() {
        return Ok(DeletePreparedStatementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePreparedStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_work_group_request(body: &[u8]) -> Result<DeleteWorkGroupInput, String> {
    if body.is_empty() {
        return Ok(DeleteWorkGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWorkGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_export_notebook_request(body: &[u8]) -> Result<ExportNotebookInput, String> {
    if body.is_empty() {
        return Ok(ExportNotebookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExportNotebook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_calculation_execution_request(
    body: &[u8],
) -> Result<GetCalculationExecutionRequest, String> {
    if body.is_empty() {
        return Ok(GetCalculationExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCalculationExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_calculation_execution_code_request(
    body: &[u8],
) -> Result<GetCalculationExecutionCodeRequest, String> {
    if body.is_empty() {
        return Ok(GetCalculationExecutionCodeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCalculationExecutionCode request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_calculation_execution_status_request(
    body: &[u8],
) -> Result<GetCalculationExecutionStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetCalculationExecutionStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCalculationExecutionStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_capacity_assignment_configuration_request(
    body: &[u8],
) -> Result<GetCapacityAssignmentConfigurationInput, String> {
    if body.is_empty() {
        return Ok(GetCapacityAssignmentConfigurationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetCapacityAssignmentConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_capacity_reservation_request(
    body: &[u8],
) -> Result<GetCapacityReservationInput, String> {
    if body.is_empty() {
        return Ok(GetCapacityReservationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCapacityReservation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_data_catalog_request(body: &[u8]) -> Result<GetDataCatalogInput, String> {
    if body.is_empty() {
        return Ok(GetDataCatalogInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDataCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_database_request(body: &[u8]) -> Result<GetDatabaseInput, String> {
    if body.is_empty() {
        return Ok(GetDatabaseInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDatabase request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_named_query_request(body: &[u8]) -> Result<GetNamedQueryInput, String> {
    if body.is_empty() {
        return Ok(GetNamedQueryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetNamedQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_notebook_metadata_request(
    body: &[u8],
) -> Result<GetNotebookMetadataInput, String> {
    if body.is_empty() {
        return Ok(GetNotebookMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetNotebookMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_prepared_statement_request(
    body: &[u8],
) -> Result<GetPreparedStatementInput, String> {
    if body.is_empty() {
        return Ok(GetPreparedStatementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPreparedStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_query_execution_request(
    body: &[u8],
) -> Result<GetQueryExecutionInput, String> {
    if body.is_empty() {
        return Ok(GetQueryExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQueryExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_query_results_request(body: &[u8]) -> Result<GetQueryResultsInput, String> {
    if body.is_empty() {
        return Ok(GetQueryResultsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQueryResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_query_runtime_statistics_request(
    body: &[u8],
) -> Result<GetQueryRuntimeStatisticsInput, String> {
    if body.is_empty() {
        return Ok(GetQueryRuntimeStatisticsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQueryRuntimeStatistics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_dashboard_request(
    body: &[u8],
) -> Result<GetResourceDashboardRequest, String> {
    if body.is_empty() {
        return Ok(GetResourceDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourceDashboard request: {e}"))
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
pub fn deserialize_get_session_endpoint_request(
    body: &[u8],
) -> Result<GetSessionEndpointRequest, String> {
    if body.is_empty() {
        return Ok(GetSessionEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSessionEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_session_status_request(
    body: &[u8],
) -> Result<GetSessionStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetSessionStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSessionStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_table_metadata_request(
    body: &[u8],
) -> Result<GetTableMetadataInput, String> {
    if body.is_empty() {
        return Ok(GetTableMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTableMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_work_group_request(body: &[u8]) -> Result<GetWorkGroupInput, String> {
    if body.is_empty() {
        return Ok(GetWorkGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWorkGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_notebook_request(body: &[u8]) -> Result<ImportNotebookInput, String> {
    if body.is_empty() {
        return Ok(ImportNotebookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportNotebook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_d_p_u_sizes_request(
    body: &[u8],
) -> Result<ListApplicationDPUSizesInput, String> {
    if body.is_empty() {
        return Ok(ListApplicationDPUSizesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationDPUSizes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_calculation_executions_request(
    body: &[u8],
) -> Result<ListCalculationExecutionsRequest, String> {
    if body.is_empty() {
        return Ok(ListCalculationExecutionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCalculationExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_capacity_reservations_request(
    body: &[u8],
) -> Result<ListCapacityReservationsInput, String> {
    if body.is_empty() {
        return Ok(ListCapacityReservationsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCapacityReservations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_data_catalogs_request(
    body: &[u8],
) -> Result<ListDataCatalogsInput, String> {
    if body.is_empty() {
        return Ok(ListDataCatalogsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDataCatalogs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_databases_request(body: &[u8]) -> Result<ListDatabasesInput, String> {
    if body.is_empty() {
        return Ok(ListDatabasesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatabases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_engine_versions_request(
    body: &[u8],
) -> Result<ListEngineVersionsInput, String> {
    if body.is_empty() {
        return Ok(ListEngineVersionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEngineVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_executors_request(body: &[u8]) -> Result<ListExecutorsRequest, String> {
    if body.is_empty() {
        return Ok(ListExecutorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListExecutors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_named_queries_request(
    body: &[u8],
) -> Result<ListNamedQueriesInput, String> {
    if body.is_empty() {
        return Ok(ListNamedQueriesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListNamedQueries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_notebook_metadata_request(
    body: &[u8],
) -> Result<ListNotebookMetadataInput, String> {
    if body.is_empty() {
        return Ok(ListNotebookMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListNotebookMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_notebook_sessions_request(
    body: &[u8],
) -> Result<ListNotebookSessionsRequest, String> {
    if body.is_empty() {
        return Ok(ListNotebookSessionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListNotebookSessions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_prepared_statements_request(
    body: &[u8],
) -> Result<ListPreparedStatementsInput, String> {
    if body.is_empty() {
        return Ok(ListPreparedStatementsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPreparedStatements request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_query_executions_request(
    body: &[u8],
) -> Result<ListQueryExecutionsInput, String> {
    if body.is_empty() {
        return Ok(ListQueryExecutionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListQueryExecutions request: {e}"))
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
pub fn deserialize_list_table_metadata_request(
    body: &[u8],
) -> Result<ListTableMetadataInput, String> {
    if body.is_empty() {
        return Ok(ListTableMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTableMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_work_groups_request(body: &[u8]) -> Result<ListWorkGroupsInput, String> {
    if body.is_empty() {
        return Ok(ListWorkGroupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWorkGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_capacity_assignment_configuration_request(
    body: &[u8],
) -> Result<PutCapacityAssignmentConfigurationInput, String> {
    if body.is_empty() {
        return Ok(PutCapacityAssignmentConfigurationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize PutCapacityAssignmentConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_calculation_execution_request(
    body: &[u8],
) -> Result<StartCalculationExecutionRequest, String> {
    if body.is_empty() {
        return Ok(StartCalculationExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCalculationExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_query_execution_request(
    body: &[u8],
) -> Result<StartQueryExecutionInput, String> {
    if body.is_empty() {
        return Ok(StartQueryExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartQueryExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_session_request(body: &[u8]) -> Result<StartSessionRequest, String> {
    if body.is_empty() {
        return Ok(StartSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_calculation_execution_request(
    body: &[u8],
) -> Result<StopCalculationExecutionRequest, String> {
    if body.is_empty() {
        return Ok(StopCalculationExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopCalculationExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_query_execution_request(
    body: &[u8],
) -> Result<StopQueryExecutionInput, String> {
    if body.is_empty() {
        return Ok(StopQueryExecutionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopQueryExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_terminate_session_request(
    body: &[u8],
) -> Result<TerminateSessionRequest, String> {
    if body.is_empty() {
        return Ok(TerminateSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_capacity_reservation_request(
    body: &[u8],
) -> Result<UpdateCapacityReservationInput, String> {
    if body.is_empty() {
        return Ok(UpdateCapacityReservationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCapacityReservation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_data_catalog_request(
    body: &[u8],
) -> Result<UpdateDataCatalogInput, String> {
    if body.is_empty() {
        return Ok(UpdateDataCatalogInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDataCatalog request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_named_query_request(
    body: &[u8],
) -> Result<UpdateNamedQueryInput, String> {
    if body.is_empty() {
        return Ok(UpdateNamedQueryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateNamedQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_notebook_request(body: &[u8]) -> Result<UpdateNotebookInput, String> {
    if body.is_empty() {
        return Ok(UpdateNotebookInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateNotebook request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_notebook_metadata_request(
    body: &[u8],
) -> Result<UpdateNotebookMetadataInput, String> {
    if body.is_empty() {
        return Ok(UpdateNotebookMetadataInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateNotebookMetadata request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_prepared_statement_request(
    body: &[u8],
) -> Result<UpdatePreparedStatementInput, String> {
    if body.is_empty() {
        return Ok(UpdatePreparedStatementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePreparedStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_work_group_request(body: &[u8]) -> Result<UpdateWorkGroupInput, String> {
    if body.is_empty() {
        return Ok(UpdateWorkGroupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWorkGroup request: {e}"))
}
