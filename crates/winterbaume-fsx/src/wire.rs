//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-fsx

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_file_system_aliases_response(
    result: &AssociateFileSystemAliasesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_data_repository_task_response(
    result: &CancelDataRepositoryTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_copy_backup_response(result: &CopyBackupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_copy_snapshot_and_update_volume_response(
    result: &CopySnapshotAndUpdateVolumeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_and_attach_s3_access_point_response(
    result: &CreateAndAttachS3AccessPointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_backup_response(result: &CreateBackupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_data_repository_association_response(
    result: &CreateDataRepositoryAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_data_repository_task_response(
    result: &CreateDataRepositoryTaskResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_file_cache_response(result: &CreateFileCacheResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_file_system_response(result: &CreateFileSystemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_file_system_from_backup_response(
    result: &CreateFileSystemFromBackupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_snapshot_response(result: &CreateSnapshotResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_storage_virtual_machine_response(
    result: &CreateStorageVirtualMachineResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_volume_response(result: &CreateVolumeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_volume_from_backup_response(
    result: &CreateVolumeFromBackupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_backup_response(result: &DeleteBackupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_data_repository_association_response(
    result: &DeleteDataRepositoryAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_file_cache_response(result: &DeleteFileCacheResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_file_system_response(result: &DeleteFileSystemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_snapshot_response(result: &DeleteSnapshotResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_storage_virtual_machine_response(
    result: &DeleteStorageVirtualMachineResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_volume_response(result: &DeleteVolumeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_backups_response(result: &DescribeBackupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_data_repository_associations_response(
    result: &DescribeDataRepositoryAssociationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_data_repository_tasks_response(
    result: &DescribeDataRepositoryTasksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_file_caches_response(
    result: &DescribeFileCachesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_file_system_aliases_response(
    result: &DescribeFileSystemAliasesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_file_systems_response(
    result: &DescribeFileSystemsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_s3_access_point_attachments_response(
    result: &DescribeS3AccessPointAttachmentsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_shared_vpc_configuration_response(
    result: &DescribeSharedVpcConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_snapshots_response(result: &DescribeSnapshotsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_storage_virtual_machines_response(
    result: &DescribeStorageVirtualMachinesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_volumes_response(result: &DescribeVolumesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detach_and_delete_s3_access_point_response(
    result: &DetachAndDeleteS3AccessPointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_file_system_aliases_response(
    result: &DisassociateFileSystemAliasesResponse,
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
pub fn serialize_release_file_system_nfs_v3_locks_response(
    result: &ReleaseFileSystemNfsV3LocksResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_volume_from_snapshot_response(
    result: &RestoreVolumeFromSnapshotResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_misconfigured_state_recovery_response(
    result: &StartMisconfiguredStateRecoveryResponse,
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
pub fn serialize_update_data_repository_association_response(
    result: &UpdateDataRepositoryAssociationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_file_cache_response(result: &UpdateFileCacheResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_file_system_response(result: &UpdateFileSystemResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_shared_vpc_configuration_response(
    result: &UpdateSharedVpcConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_snapshot_response(result: &UpdateSnapshotResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_storage_virtual_machine_response(
    result: &UpdateStorageVirtualMachineResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_volume_response(result: &UpdateVolumeResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_file_system_aliases_request(
    body: &[u8],
) -> Result<AssociateFileSystemAliasesRequest, String> {
    if body.is_empty() {
        return Ok(AssociateFileSystemAliasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateFileSystemAliases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_data_repository_task_request(
    body: &[u8],
) -> Result<CancelDataRepositoryTaskRequest, String> {
    if body.is_empty() {
        return Ok(CancelDataRepositoryTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelDataRepositoryTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_copy_backup_request(body: &[u8]) -> Result<CopyBackupRequest, String> {
    if body.is_empty() {
        return Ok(CopyBackupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CopyBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_copy_snapshot_and_update_volume_request(
    body: &[u8],
) -> Result<CopySnapshotAndUpdateVolumeRequest, String> {
    if body.is_empty() {
        return Ok(CopySnapshotAndUpdateVolumeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CopySnapshotAndUpdateVolume request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_and_attach_s3_access_point_request(
    body: &[u8],
) -> Result<CreateAndAttachS3AccessPointRequest, String> {
    if body.is_empty() {
        return Ok(CreateAndAttachS3AccessPointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAndAttachS3AccessPoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_backup_request(body: &[u8]) -> Result<CreateBackupRequest, String> {
    if body.is_empty() {
        return Ok(CreateBackupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_data_repository_association_request(
    body: &[u8],
) -> Result<CreateDataRepositoryAssociationRequest, String> {
    if body.is_empty() {
        return Ok(CreateDataRepositoryAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataRepositoryAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_data_repository_task_request(
    body: &[u8],
) -> Result<CreateDataRepositoryTaskRequest, String> {
    if body.is_empty() {
        return Ok(CreateDataRepositoryTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataRepositoryTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_file_cache_request(
    body: &[u8],
) -> Result<CreateFileCacheRequest, String> {
    if body.is_empty() {
        return Ok(CreateFileCacheRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFileCache request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_file_system_request(
    body: &[u8],
) -> Result<CreateFileSystemRequest, String> {
    if body.is_empty() {
        return Ok(CreateFileSystemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFileSystem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_file_system_from_backup_request(
    body: &[u8],
) -> Result<CreateFileSystemFromBackupRequest, String> {
    if body.is_empty() {
        return Ok(CreateFileSystemFromBackupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFileSystemFromBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_snapshot_request(body: &[u8]) -> Result<CreateSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(CreateSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_storage_virtual_machine_request(
    body: &[u8],
) -> Result<CreateStorageVirtualMachineRequest, String> {
    if body.is_empty() {
        return Ok(CreateStorageVirtualMachineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStorageVirtualMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_volume_request(body: &[u8]) -> Result<CreateVolumeRequest, String> {
    if body.is_empty() {
        return Ok(CreateVolumeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateVolume request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_volume_from_backup_request(
    body: &[u8],
) -> Result<CreateVolumeFromBackupRequest, String> {
    if body.is_empty() {
        return Ok(CreateVolumeFromBackupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateVolumeFromBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_backup_request(body: &[u8]) -> Result<DeleteBackupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteBackupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_data_repository_association_request(
    body: &[u8],
) -> Result<DeleteDataRepositoryAssociationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDataRepositoryAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataRepositoryAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_file_cache_request(
    body: &[u8],
) -> Result<DeleteFileCacheRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFileCacheRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFileCache request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_file_system_request(
    body: &[u8],
) -> Result<DeleteFileSystemRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFileSystemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFileSystem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_snapshot_request(body: &[u8]) -> Result<DeleteSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_storage_virtual_machine_request(
    body: &[u8],
) -> Result<DeleteStorageVirtualMachineRequest, String> {
    if body.is_empty() {
        return Ok(DeleteStorageVirtualMachineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStorageVirtualMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_volume_request(body: &[u8]) -> Result<DeleteVolumeRequest, String> {
    if body.is_empty() {
        return Ok(DeleteVolumeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteVolume request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_backups_request(body: &[u8]) -> Result<DescribeBackupsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBackupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBackups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_data_repository_associations_request(
    body: &[u8],
) -> Result<DescribeDataRepositoryAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDataRepositoryAssociationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeDataRepositoryAssociations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_data_repository_tasks_request(
    body: &[u8],
) -> Result<DescribeDataRepositoryTasksRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDataRepositoryTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDataRepositoryTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_file_caches_request(
    body: &[u8],
) -> Result<DescribeFileCachesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFileCachesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFileCaches request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_file_system_aliases_request(
    body: &[u8],
) -> Result<DescribeFileSystemAliasesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFileSystemAliasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFileSystemAliases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_file_systems_request(
    body: &[u8],
) -> Result<DescribeFileSystemsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFileSystemsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFileSystems request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_s3_access_point_attachments_request(
    body: &[u8],
) -> Result<DescribeS3AccessPointAttachmentsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeS3AccessPointAttachmentsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeS3AccessPointAttachments request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_shared_vpc_configuration_request(
    body: &[u8],
) -> Result<DescribeSharedVpcConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSharedVpcConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSharedVpcConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_snapshots_request(
    body: &[u8],
) -> Result<DescribeSnapshotsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSnapshotsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSnapshots request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_storage_virtual_machines_request(
    body: &[u8],
) -> Result<DescribeStorageVirtualMachinesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeStorageVirtualMachinesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStorageVirtualMachines request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_volumes_request(body: &[u8]) -> Result<DescribeVolumesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeVolumesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeVolumes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detach_and_delete_s3_access_point_request(
    body: &[u8],
) -> Result<DetachAndDeleteS3AccessPointRequest, String> {
    if body.is_empty() {
        return Ok(DetachAndDeleteS3AccessPointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetachAndDeleteS3AccessPoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_file_system_aliases_request(
    body: &[u8],
) -> Result<DisassociateFileSystemAliasesRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateFileSystemAliasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateFileSystemAliases request: {e}"))
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
pub fn deserialize_release_file_system_nfs_v3_locks_request(
    body: &[u8],
) -> Result<ReleaseFileSystemNfsV3LocksRequest, String> {
    if body.is_empty() {
        return Ok(ReleaseFileSystemNfsV3LocksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReleaseFileSystemNfsV3Locks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_volume_from_snapshot_request(
    body: &[u8],
) -> Result<RestoreVolumeFromSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(RestoreVolumeFromSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreVolumeFromSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_misconfigured_state_recovery_request(
    body: &[u8],
) -> Result<StartMisconfiguredStateRecoveryRequest, String> {
    if body.is_empty() {
        return Ok(StartMisconfiguredStateRecoveryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMisconfiguredStateRecovery request: {e}"))
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
pub fn deserialize_update_data_repository_association_request(
    body: &[u8],
) -> Result<UpdateDataRepositoryAssociationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDataRepositoryAssociationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDataRepositoryAssociation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_file_cache_request(
    body: &[u8],
) -> Result<UpdateFileCacheRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFileCacheRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFileCache request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_file_system_request(
    body: &[u8],
) -> Result<UpdateFileSystemRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFileSystemRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFileSystem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_shared_vpc_configuration_request(
    body: &[u8],
) -> Result<UpdateSharedVpcConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSharedVpcConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSharedVpcConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_snapshot_request(body: &[u8]) -> Result<UpdateSnapshotRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSnapshotRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSnapshot request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_storage_virtual_machine_request(
    body: &[u8],
) -> Result<UpdateStorageVirtualMachineRequest, String> {
    if body.is_empty() {
        return Ok(UpdateStorageVirtualMachineRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStorageVirtualMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_volume_request(body: &[u8]) -> Result<UpdateVolumeRequest, String> {
    if body.is_empty() {
        return Ok(UpdateVolumeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateVolume request: {e}"))
}
