//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationdiscoveryservice

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_configuration_items_to_application_response(
    result: &AssociateConfigurationItemsToApplicationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_agents_response(result: &BatchDeleteAgentsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_import_data_response(
    result: &BatchDeleteImportDataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_application_response(result: &CreateApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_tags_response(result: &CreateTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_applications_response(result: &DeleteApplicationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_tags_response(result: &DeleteTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_agents_response(result: &DescribeAgentsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_batch_delete_configuration_task_response(
    result: &DescribeBatchDeleteConfigurationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_configurations_response(
    result: &DescribeConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_continuous_exports_response(
    result: &DescribeContinuousExportsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_export_configurations_response(
    result: &DescribeExportConfigurationsResponse,
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
pub fn serialize_describe_import_tasks_response(
    result: &DescribeImportTasksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_tags_response(result: &DescribeTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_configuration_items_from_application_response(
    result: &DisassociateConfigurationItemsFromApplicationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_export_configurations_response(
    result: &ExportConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_discovery_summary_response(
    result: &GetDiscoverySummaryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_configurations_response(result: &ListConfigurationsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_server_neighbors_response(
    result: &ListServerNeighborsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_batch_delete_configuration_task_response(
    result: &StartBatchDeleteConfigurationTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_continuous_export_response(
    result: &StartContinuousExportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_data_collection_by_agent_ids_response(
    result: &StartDataCollectionByAgentIdsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_export_task_response(result: &StartExportTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_import_task_response(result: &StartImportTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_continuous_export_response(
    result: &StopContinuousExportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_data_collection_by_agent_ids_response(
    result: &StopDataCollectionByAgentIdsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_application_response(result: &UpdateApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_configuration_items_to_application_request(
    body: &[u8],
) -> Result<AssociateConfigurationItemsToApplicationRequest, String> {
    if body.is_empty() {
        return Ok(AssociateConfigurationItemsToApplicationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AssociateConfigurationItemsToApplication request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_agents_request(
    body: &[u8],
) -> Result<BatchDeleteAgentsRequest, String> {
    if body.is_empty() {
        return Ok(BatchDeleteAgentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeleteAgents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_import_data_request(
    body: &[u8],
) -> Result<BatchDeleteImportDataRequest, String> {
    if body.is_empty() {
        return Ok(BatchDeleteImportDataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeleteImportData request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_application_request(
    body: &[u8],
) -> Result<CreateApplicationRequest, String> {
    if body.is_empty() {
        return Ok(CreateApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_tags_request(body: &[u8]) -> Result<CreateTagsRequest, String> {
    if body.is_empty() {
        return Ok(CreateTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_applications_request(
    body: &[u8],
) -> Result<DeleteApplicationsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplications request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_tags_request(body: &[u8]) -> Result<DeleteTagsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_agents_request(body: &[u8]) -> Result<DescribeAgentsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAgentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAgents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_batch_delete_configuration_task_request(
    body: &[u8],
) -> Result<DescribeBatchDeleteConfigurationTaskRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBatchDeleteConfigurationTaskRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeBatchDeleteConfigurationTask request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_configurations_request(
    body: &[u8],
) -> Result<DescribeConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_continuous_exports_request(
    body: &[u8],
) -> Result<DescribeContinuousExportsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeContinuousExportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeContinuousExports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_export_configurations_request(
    body: &[u8],
) -> Result<DescribeExportConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeExportConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExportConfigurations request: {e}"))
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
pub fn deserialize_describe_tags_request(body: &[u8]) -> Result<DescribeTagsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_configuration_items_from_application_request(
    body: &[u8],
) -> Result<DisassociateConfigurationItemsFromApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateConfigurationItemsFromApplicationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociateConfigurationItemsFromApplication request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_discovery_summary_request(
    body: &[u8],
) -> Result<GetDiscoverySummaryRequest, String> {
    if body.is_empty() {
        return Ok(GetDiscoverySummaryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDiscoverySummary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_configurations_request(
    body: &[u8],
) -> Result<ListConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(ListConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_server_neighbors_request(
    body: &[u8],
) -> Result<ListServerNeighborsRequest, String> {
    if body.is_empty() {
        return Ok(ListServerNeighborsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServerNeighbors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_batch_delete_configuration_task_request(
    body: &[u8],
) -> Result<StartBatchDeleteConfigurationTaskRequest, String> {
    if body.is_empty() {
        return Ok(StartBatchDeleteConfigurationTaskRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartBatchDeleteConfigurationTask request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_continuous_export_request(
    body: &[u8],
) -> Result<StartContinuousExportRequest, String> {
    if body.is_empty() {
        return Ok(StartContinuousExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartContinuousExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_data_collection_by_agent_ids_request(
    body: &[u8],
) -> Result<StartDataCollectionByAgentIdsRequest, String> {
    if body.is_empty() {
        return Ok(StartDataCollectionByAgentIdsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDataCollectionByAgentIds request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_export_task_request(
    body: &[u8],
) -> Result<StartExportTaskRequest, String> {
    if body.is_empty() {
        return Ok(StartExportTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartExportTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_import_task_request(
    body: &[u8],
) -> Result<StartImportTaskRequest, String> {
    if body.is_empty() {
        return Ok(StartImportTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartImportTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_continuous_export_request(
    body: &[u8],
) -> Result<StopContinuousExportRequest, String> {
    if body.is_empty() {
        return Ok(StopContinuousExportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopContinuousExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_data_collection_by_agent_ids_request(
    body: &[u8],
) -> Result<StopDataCollectionByAgentIdsRequest, String> {
    if body.is_empty() {
        return Ok(StopDataCollectionByAgentIdsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopDataCollectionByAgentIds request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_application_request(
    body: &[u8],
) -> Result<UpdateApplicationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateApplication request: {e}"))
}
