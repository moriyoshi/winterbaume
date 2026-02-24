//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-efs

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
pub fn serialize_create_access_point_response(result: &AccessPointDescription) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_file_system_response(result: &FileSystemDescription) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_mount_target_response(result: &MountTargetDescription) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_replication_configuration_response(
    result: &ReplicationConfigurationDescription,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_create_tags_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_access_point_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_file_system_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_file_system_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_mount_target_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_replication_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_tags_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_access_points_response(
    result: &DescribeAccessPointsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_account_preferences_response(
    result: &DescribeAccountPreferencesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_backup_policy_response(result: &BackupPolicyDescription) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_file_system_policy_response(
    result: &FileSystemPolicyDescription,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_file_systems_response(
    result: &DescribeFileSystemsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_lifecycle_configuration_response(
    result: &LifecycleConfigurationDescription,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_mount_target_security_groups_response(
    result: &DescribeMountTargetSecurityGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_mount_targets_response(
    result: &DescribeMountTargetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_replication_configurations_response(
    result: &DescribeReplicationConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_tags_response(result: &DescribeTagsResponse) -> MockResponse {
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

/// Serialize void response for restJson protocol.
pub fn serialize_modify_mount_target_security_groups_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_put_account_preferences_response(
    result: &PutAccountPreferencesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_backup_policy_response(result: &BackupPolicyDescription) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_file_system_policy_response(
    result: &FileSystemPolicyDescription,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_lifecycle_configuration_response(
    result: &LifecycleConfigurationDescription,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_file_system_response(result: &FileSystemDescription) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_file_system_protection_response(
    result: &FileSystemProtectionDescription,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessPointRequest, String> {
    let mut input = CreateAccessPointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAccessPointRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAccessPoint request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_file_system_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFileSystemRequest, String> {
    let mut input = CreateFileSystemRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFileSystemRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFileSystem request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_mount_target_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMountTargetRequest, String> {
    let mut input = CreateMountTargetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMountTargetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateMountTarget request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_replication_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateReplicationConfigurationRequest, String> {
    let mut input = CreateReplicationConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateReplicationConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateReplicationConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "SourceFileSystemId" => {
                input.source_file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTagsRequest, String> {
    let mut input = CreateTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTags request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_access_point_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessPointRequest, String> {
    let mut input = DeleteAccessPointRequest::default();
    for (name, value) in labels {
        match *name {
            "AccessPointId" => {
                input.access_point_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_file_system_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFileSystemRequest, String> {
    let mut input = DeleteFileSystemRequest::default();
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_file_system_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFileSystemPolicyRequest, String> {
    let mut input = DeleteFileSystemPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_mount_target_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMountTargetRequest, String> {
    let mut input = DeleteMountTargetRequest::default();
    for (name, value) in labels {
        match *name {
            "MountTargetId" => {
                input.mount_target_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_replication_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteReplicationConfigurationRequest, String> {
    let mut input = DeleteReplicationConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "SourceFileSystemId" => {
                input.source_file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("deletionMode") {
        input.deletion_mode = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTagsRequest, String> {
    let mut input = DeleteTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteTags request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_access_points_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccessPointsRequest, String> {
    let mut input = DescribeAccessPointsRequest::default();
    if let Some(value) = query.get("AccessPointId") {
        input.access_point_id = Some(value.to_string());
    }
    if let Some(value) = query.get("FileSystemId") {
        input.file_system_id = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_account_preferences_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountPreferencesRequest, String> {
    let mut input = DescribeAccountPreferencesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeAccountPreferencesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeAccountPreferences request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_backup_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBackupPolicyRequest, String> {
    let mut input = DescribeBackupPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_file_system_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFileSystemPolicyRequest, String> {
    let mut input = DescribeFileSystemPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_file_systems_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFileSystemsRequest, String> {
    let mut input = DescribeFileSystemsRequest::default();
    if let Some(value) = query.get("CreationToken") {
        input.creation_token = Some(value.to_string());
    }
    if let Some(value) = query.get("FileSystemId") {
        input.file_system_id = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_lifecycle_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeLifecycleConfigurationRequest, String> {
    let mut input = DescribeLifecycleConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_mount_target_security_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMountTargetSecurityGroupsRequest, String> {
    let mut input = DescribeMountTargetSecurityGroupsRequest::default();
    for (name, value) in labels {
        match *name {
            "MountTargetId" => {
                input.mount_target_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_mount_targets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMountTargetsRequest, String> {
    let mut input = DescribeMountTargetsRequest::default();
    if let Some(value) = query.get("AccessPointId") {
        input.access_point_id = Some(value.to_string());
    }
    if let Some(value) = query.get("FileSystemId") {
        input.file_system_id = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("MountTargetId") {
        input.mount_target_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_replication_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeReplicationConfigurationsRequest, String> {
    let mut input = DescribeReplicationConfigurationsRequest::default();
    if let Some(value) = query.get("FileSystemId") {
        input.file_system_id = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTagsRequest, String> {
    let mut input = DescribeTagsRequest::default();
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
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
            "ResourceId" => {
                input.resource_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_modify_mount_target_security_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ModifyMountTargetSecurityGroupsRequest, String> {
    let mut input = ModifyMountTargetSecurityGroupsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ModifyMountTargetSecurityGroupsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ModifyMountTargetSecurityGroups request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "MountTargetId" => {
                input.mount_target_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_account_preferences_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccountPreferencesRequest, String> {
    let mut input = PutAccountPreferencesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAccountPreferencesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutAccountPreferences request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_backup_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBackupPolicyRequest, String> {
    let mut input = PutBackupPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutBackupPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutBackupPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_file_system_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutFileSystemPolicyRequest, String> {
    let mut input = PutFileSystemPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutFileSystemPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutFileSystemPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_lifecycle_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutLifecycleConfigurationRequest, String> {
    let mut input = PutLifecycleConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutLifecycleConfigurationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutLifecycleConfiguration request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
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
            "ResourceId" => {
                input.resource_id = value.to_string();
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
            "ResourceId" => {
                input.resource_id = value.to_string();
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
pub fn deserialize_update_file_system_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFileSystemRequest, String> {
    let mut input = UpdateFileSystemRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFileSystemRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFileSystem request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_file_system_protection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFileSystemProtectionRequest, String> {
    let mut input = UpdateFileSystemProtectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFileSystemProtectionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateFileSystemProtection request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FileSystemId" => {
                input.file_system_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
