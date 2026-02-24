//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-datasync

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_task_execution_response(
    result: &CancelTaskExecutionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_agent_response(result: &CreateAgentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_azure_blob_response(
    result: &CreateLocationAzureBlobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_efs_response(result: &CreateLocationEfsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_fsx_lustre_response(
    result: &CreateLocationFsxLustreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_fsx_ontap_response(
    result: &CreateLocationFsxOntapResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_fsx_open_zfs_response(
    result: &CreateLocationFsxOpenZfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_fsx_windows_response(
    result: &CreateLocationFsxWindowsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_hdfs_response(
    result: &CreateLocationHdfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_nfs_response(result: &CreateLocationNfsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_object_storage_response(
    result: &CreateLocationObjectStorageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_s3_response(result: &CreateLocationS3Response) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_location_smb_response(result: &CreateLocationSmbResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_task_response(result: &CreateTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_agent_response(result: &DeleteAgentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_location_response(result: &DeleteLocationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_task_response(result: &DeleteTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_agent_response(result: &DescribeAgentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_azure_blob_response(
    result: &DescribeLocationAzureBlobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_efs_response(
    result: &DescribeLocationEfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_fsx_lustre_response(
    result: &DescribeLocationFsxLustreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_fsx_ontap_response(
    result: &DescribeLocationFsxOntapResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_fsx_open_zfs_response(
    result: &DescribeLocationFsxOpenZfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_fsx_windows_response(
    result: &DescribeLocationFsxWindowsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_hdfs_response(
    result: &DescribeLocationHdfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_nfs_response(
    result: &DescribeLocationNfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_object_storage_response(
    result: &DescribeLocationObjectStorageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_s3_response(
    result: &DescribeLocationS3Response,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_location_smb_response(
    result: &DescribeLocationSmbResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_task_response(result: &DescribeTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_task_execution_response(
    result: &DescribeTaskExecutionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_agents_response(result: &ListAgentsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_locations_response(result: &ListLocationsResponse) -> MockResponse {
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
pub fn serialize_list_task_executions_response(
    result: &ListTaskExecutionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tasks_response(result: &ListTasksResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_task_execution_response(
    result: &StartTaskExecutionResponse,
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
pub fn serialize_update_agent_response(result: &UpdateAgentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_azure_blob_response(
    result: &UpdateLocationAzureBlobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_efs_response(result: &UpdateLocationEfsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_fsx_lustre_response(
    result: &UpdateLocationFsxLustreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_fsx_ontap_response(
    result: &UpdateLocationFsxOntapResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_fsx_open_zfs_response(
    result: &UpdateLocationFsxOpenZfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_fsx_windows_response(
    result: &UpdateLocationFsxWindowsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_hdfs_response(
    result: &UpdateLocationHdfsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_nfs_response(result: &UpdateLocationNfsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_object_storage_response(
    result: &UpdateLocationObjectStorageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_s3_response(result: &UpdateLocationS3Response) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_location_smb_response(result: &UpdateLocationSmbResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_task_response(result: &UpdateTaskResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_task_execution_response(
    result: &UpdateTaskExecutionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_task_execution_request(
    body: &[u8],
) -> Result<CancelTaskExecutionRequest, String> {
    if body.is_empty() {
        return Ok(CancelTaskExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelTaskExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_agent_request(body: &[u8]) -> Result<CreateAgentRequest, String> {
    if body.is_empty() {
        return Ok(CreateAgentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAgent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_azure_blob_request(
    body: &[u8],
) -> Result<CreateLocationAzureBlobRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationAzureBlobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationAzureBlob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_efs_request(
    body: &[u8],
) -> Result<CreateLocationEfsRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationEfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationEfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_fsx_lustre_request(
    body: &[u8],
) -> Result<CreateLocationFsxLustreRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationFsxLustreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationFsxLustre request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_fsx_ontap_request(
    body: &[u8],
) -> Result<CreateLocationFsxOntapRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationFsxOntapRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationFsxOntap request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_fsx_open_zfs_request(
    body: &[u8],
) -> Result<CreateLocationFsxOpenZfsRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationFsxOpenZfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationFsxOpenZfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_fsx_windows_request(
    body: &[u8],
) -> Result<CreateLocationFsxWindowsRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationFsxWindowsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationFsxWindows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_hdfs_request(
    body: &[u8],
) -> Result<CreateLocationHdfsRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationHdfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationHdfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_nfs_request(
    body: &[u8],
) -> Result<CreateLocationNfsRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationNfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationNfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_object_storage_request(
    body: &[u8],
) -> Result<CreateLocationObjectStorageRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationObjectStorageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationObjectStorage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_s3_request(
    body: &[u8],
) -> Result<CreateLocationS3Request, String> {
    if body.is_empty() {
        return Ok(CreateLocationS3Request::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationS3 request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_location_smb_request(
    body: &[u8],
) -> Result<CreateLocationSmbRequest, String> {
    if body.is_empty() {
        return Ok(CreateLocationSmbRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLocationSmb request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_task_request(body: &[u8]) -> Result<CreateTaskRequest, String> {
    if body.is_empty() {
        return Ok(CreateTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_agent_request(body: &[u8]) -> Result<DeleteAgentRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAgentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAgent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_location_request(body: &[u8]) -> Result<DeleteLocationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLocationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLocation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_task_request(body: &[u8]) -> Result<DeleteTaskRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_agent_request(body: &[u8]) -> Result<DescribeAgentRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAgentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAgent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_azure_blob_request(
    body: &[u8],
) -> Result<DescribeLocationAzureBlobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationAzureBlobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationAzureBlob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_efs_request(
    body: &[u8],
) -> Result<DescribeLocationEfsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationEfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationEfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_fsx_lustre_request(
    body: &[u8],
) -> Result<DescribeLocationFsxLustreRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationFsxLustreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationFsxLustre request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_fsx_ontap_request(
    body: &[u8],
) -> Result<DescribeLocationFsxOntapRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationFsxOntapRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationFsxOntap request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_fsx_open_zfs_request(
    body: &[u8],
) -> Result<DescribeLocationFsxOpenZfsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationFsxOpenZfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationFsxOpenZfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_fsx_windows_request(
    body: &[u8],
) -> Result<DescribeLocationFsxWindowsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationFsxWindowsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationFsxWindows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_hdfs_request(
    body: &[u8],
) -> Result<DescribeLocationHdfsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationHdfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationHdfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_nfs_request(
    body: &[u8],
) -> Result<DescribeLocationNfsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationNfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationNfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_object_storage_request(
    body: &[u8],
) -> Result<DescribeLocationObjectStorageRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationObjectStorageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationObjectStorage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_s3_request(
    body: &[u8],
) -> Result<DescribeLocationS3Request, String> {
    if body.is_empty() {
        return Ok(DescribeLocationS3Request::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationS3 request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_location_smb_request(
    body: &[u8],
) -> Result<DescribeLocationSmbRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLocationSmbRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLocationSmb request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_task_request(body: &[u8]) -> Result<DescribeTaskRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_task_execution_request(
    body: &[u8],
) -> Result<DescribeTaskExecutionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTaskExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTaskExecution request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_agents_request(body: &[u8]) -> Result<ListAgentsRequest, String> {
    if body.is_empty() {
        return Ok(ListAgentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAgents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_locations_request(body: &[u8]) -> Result<ListLocationsRequest, String> {
    if body.is_empty() {
        return Ok(ListLocationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLocations request: {e}"))
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
pub fn deserialize_list_task_executions_request(
    body: &[u8],
) -> Result<ListTaskExecutionsRequest, String> {
    if body.is_empty() {
        return Ok(ListTaskExecutionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTaskExecutions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tasks_request(body: &[u8]) -> Result<ListTasksRequest, String> {
    if body.is_empty() {
        return Ok(ListTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_task_execution_request(
    body: &[u8],
) -> Result<StartTaskExecutionRequest, String> {
    if body.is_empty() {
        return Ok(StartTaskExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartTaskExecution request: {e}"))
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
pub fn deserialize_update_agent_request(body: &[u8]) -> Result<UpdateAgentRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAgentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAgent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_azure_blob_request(
    body: &[u8],
) -> Result<UpdateLocationAzureBlobRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationAzureBlobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationAzureBlob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_efs_request(
    body: &[u8],
) -> Result<UpdateLocationEfsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationEfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationEfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_fsx_lustre_request(
    body: &[u8],
) -> Result<UpdateLocationFsxLustreRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationFsxLustreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationFsxLustre request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_fsx_ontap_request(
    body: &[u8],
) -> Result<UpdateLocationFsxOntapRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationFsxOntapRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationFsxOntap request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_fsx_open_zfs_request(
    body: &[u8],
) -> Result<UpdateLocationFsxOpenZfsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationFsxOpenZfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationFsxOpenZfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_fsx_windows_request(
    body: &[u8],
) -> Result<UpdateLocationFsxWindowsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationFsxWindowsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationFsxWindows request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_hdfs_request(
    body: &[u8],
) -> Result<UpdateLocationHdfsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationHdfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationHdfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_nfs_request(
    body: &[u8],
) -> Result<UpdateLocationNfsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationNfsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationNfs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_object_storage_request(
    body: &[u8],
) -> Result<UpdateLocationObjectStorageRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationObjectStorageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationObjectStorage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_s3_request(
    body: &[u8],
) -> Result<UpdateLocationS3Request, String> {
    if body.is_empty() {
        return Ok(UpdateLocationS3Request::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationS3 request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_location_smb_request(
    body: &[u8],
) -> Result<UpdateLocationSmbRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLocationSmbRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLocationSmb request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_task_request(body: &[u8]) -> Result<UpdateTaskRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_task_execution_request(
    body: &[u8],
) -> Result<UpdateTaskExecutionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTaskExecutionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTaskExecution request: {e}"))
}
