//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesisanalyticsv2

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_application_cloud_watch_logging_option_response(
    result: &AddApplicationCloudWatchLoggingOptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_application_input_response(
    result: &AddApplicationInputResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_application_input_processing_configuration_response(
    result: &AddApplicationInputProcessingConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_application_output_response(
    result: &AddApplicationOutputResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_application_reference_data_source_response(
    result: &AddApplicationReferenceDataSourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_application_vpc_configuration_response(
    result: &AddApplicationVpcConfigurationResponse,
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
pub fn serialize_create_application_presigned_url_response(
    result: &CreateApplicationPresignedUrlResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_application_snapshot_response(
    result: &CreateApplicationSnapshotResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_response(result: &DeleteApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_cloud_watch_logging_option_response(
    result: &DeleteApplicationCloudWatchLoggingOptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_input_processing_configuration_response(
    result: &DeleteApplicationInputProcessingConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_output_response(
    result: &DeleteApplicationOutputResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_reference_data_source_response(
    result: &DeleteApplicationReferenceDataSourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_snapshot_response(
    result: &DeleteApplicationSnapshotResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_application_vpc_configuration_response(
    result: &DeleteApplicationVpcConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_response(
    result: &DescribeApplicationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_operation_response(
    result: &DescribeApplicationOperationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_snapshot_response(
    result: &DescribeApplicationSnapshotResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_application_version_response(
    result: &DescribeApplicationVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_discover_input_schema_response(
    result: &DiscoverInputSchemaResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_operations_response(
    result: &ListApplicationOperationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_snapshots_response(
    result: &ListApplicationSnapshotsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_application_versions_response(
    result: &ListApplicationVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_applications_response(result: &ListApplicationsResponse) -> MockResponse {
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
pub fn serialize_rollback_application_response(
    result: &RollbackApplicationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_application_response(result: &StartApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_application_response(result: &StopApplicationResponse) -> MockResponse {
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
pub fn serialize_update_application_response(result: &UpdateApplicationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_application_maintenance_configuration_response(
    result: &UpdateApplicationMaintenanceConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_application_cloud_watch_logging_option_request(
    body: &[u8],
) -> Result<AddApplicationCloudWatchLoggingOptionRequest, String> {
    if body.is_empty() {
        return Ok(AddApplicationCloudWatchLoggingOptionRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AddApplicationCloudWatchLoggingOption request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_application_input_request(
    body: &[u8],
) -> Result<AddApplicationInputRequest, String> {
    if body.is_empty() {
        return Ok(AddApplicationInputRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddApplicationInput request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_application_input_processing_configuration_request(
    body: &[u8],
) -> Result<AddApplicationInputProcessingConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(AddApplicationInputProcessingConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AddApplicationInputProcessingConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_application_output_request(
    body: &[u8],
) -> Result<AddApplicationOutputRequest, String> {
    if body.is_empty() {
        return Ok(AddApplicationOutputRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddApplicationOutput request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_application_reference_data_source_request(
    body: &[u8],
) -> Result<AddApplicationReferenceDataSourceRequest, String> {
    if body.is_empty() {
        return Ok(AddApplicationReferenceDataSourceRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AddApplicationReferenceDataSource request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_application_vpc_configuration_request(
    body: &[u8],
) -> Result<AddApplicationVpcConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(AddApplicationVpcConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddApplicationVpcConfiguration request: {e}"))
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
pub fn deserialize_create_application_presigned_url_request(
    body: &[u8],
) -> Result<CreateApplicationPresignedUrlRequest, String> {
    if body.is_empty() {
        return Ok(CreateApplicationPresignedUrlRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApplicationPresignedUrl request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_application_snapshot_request(
    body: &[u8],
) -> Result<CreateApplicationSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(CreateApplicationSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApplicationSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_request(
    body: &[u8],
) -> Result<DeleteApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_cloud_watch_logging_option_request(
    body: &[u8],
) -> Result<DeleteApplicationCloudWatchLoggingOptionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationCloudWatchLoggingOptionRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteApplicationCloudWatchLoggingOption request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_input_processing_configuration_request(
    body: &[u8],
) -> Result<DeleteApplicationInputProcessingConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationInputProcessingConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteApplicationInputProcessingConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_output_request(
    body: &[u8],
) -> Result<DeleteApplicationOutputRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationOutputRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplicationOutput request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_reference_data_source_request(
    body: &[u8],
) -> Result<DeleteApplicationReferenceDataSourceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationReferenceDataSourceRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteApplicationReferenceDataSource request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_snapshot_request(
    body: &[u8],
) -> Result<DeleteApplicationSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApplicationSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_application_vpc_configuration_request(
    body: &[u8],
) -> Result<DeleteApplicationVpcConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApplicationVpcConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteApplicationVpcConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_request(
    body: &[u8],
) -> Result<DescribeApplicationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_operation_request(
    body: &[u8],
) -> Result<DescribeApplicationOperationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationOperationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplicationOperation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_snapshot_request(
    body: &[u8],
) -> Result<DescribeApplicationSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplicationSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_application_version_request(
    body: &[u8],
) -> Result<DescribeApplicationVersionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApplicationVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApplicationVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_discover_input_schema_request(
    body: &[u8],
) -> Result<DiscoverInputSchemaRequest, String> {
    if body.is_empty() {
        return Ok(DiscoverInputSchemaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DiscoverInputSchema request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_operations_request(
    body: &[u8],
) -> Result<ListApplicationOperationsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationOperationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationOperations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_snapshots_request(
    body: &[u8],
) -> Result<ListApplicationSnapshotsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationSnapshotsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationSnapshots request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_application_versions_request(
    body: &[u8],
) -> Result<ListApplicationVersionsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplicationVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_applications_request(
    body: &[u8],
) -> Result<ListApplicationsRequest, String> {
    if body.is_empty() {
        return Ok(ListApplicationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApplications request: {e}"))
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
pub fn deserialize_rollback_application_request(
    body: &[u8],
) -> Result<RollbackApplicationRequest, String> {
    if body.is_empty() {
        return Ok(RollbackApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RollbackApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_application_request(
    body: &[u8],
) -> Result<StartApplicationRequest, String> {
    if body.is_empty() {
        return Ok(StartApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_application_request(body: &[u8]) -> Result<StopApplicationRequest, String> {
    if body.is_empty() {
        return Ok(StopApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopApplication request: {e}"))
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
pub fn deserialize_update_application_request(
    body: &[u8],
) -> Result<UpdateApplicationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateApplicationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateApplication request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_application_maintenance_configuration_request(
    body: &[u8],
) -> Result<UpdateApplicationMaintenanceConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateApplicationMaintenanceConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateApplicationMaintenanceConfiguration request: {e}")
    })
}
