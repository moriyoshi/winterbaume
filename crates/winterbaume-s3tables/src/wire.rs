//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3tables

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
pub fn serialize_create_namespace_response(result: &CreateNamespaceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_table_response(result: &CreateTableResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_table_bucket_response(result: &CreateTableBucketResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_namespace_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_bucket_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_bucket_encryption_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_bucket_metrics_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_bucket_policy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_bucket_replication_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_policy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_table_replication_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_namespace_response(result: &GetNamespaceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_response(result: &GetTableResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_bucket_response(result: &GetTableBucketResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_bucket_encryption_response(
    result: &GetTableBucketEncryptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_bucket_maintenance_configuration_response(
    result: &GetTableBucketMaintenanceConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_bucket_metrics_configuration_response(
    result: &GetTableBucketMetricsConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_bucket_policy_response(
    result: &GetTableBucketPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_bucket_replication_response(
    result: &GetTableBucketReplicationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_bucket_storage_class_response(
    result: &GetTableBucketStorageClassResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_encryption_response(
    result: &GetTableEncryptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_maintenance_configuration_response(
    result: &GetTableMaintenanceConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_maintenance_job_status_response(
    result: &GetTableMaintenanceJobStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_metadata_location_response(
    result: &GetTableMetadataLocationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_policy_response(result: &GetTablePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_record_expiration_configuration_response(
    result: &GetTableRecordExpirationConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_record_expiration_job_status_response(
    result: &GetTableRecordExpirationJobStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_replication_response(
    result: &GetTableReplicationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_replication_status_response(
    result: &GetTableReplicationStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_storage_class_response(
    result: &GetTableStorageClassResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_namespaces_response(result: &ListNamespacesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_table_buckets_response(result: &ListTableBucketsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tables_response(result: &ListTablesResponse) -> MockResponse {
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
pub fn serialize_put_table_bucket_encryption_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_table_bucket_maintenance_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_table_bucket_metrics_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_table_bucket_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_put_table_bucket_replication_response(
    result: &PutTableBucketReplicationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_table_bucket_storage_class_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_table_maintenance_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_table_policy_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_table_record_expiration_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_put_table_replication_response(
    result: &PutTableReplicationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_rename_table_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_table_metadata_location_response(
    result: &UpdateTableMetadataLocationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNamespaceRequest, String> {
    let mut input = CreateNamespaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNamespaceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateNamespace request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_table_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTableRequest, String> {
    let mut input = CreateTableRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTableRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTable request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_table_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTableBucketRequest, String> {
    let mut input = CreateTableBucketRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTableBucketRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTableBucket request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteNamespaceRequest, String> {
    let mut input = DeleteNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTableRequest, String> {
    let mut input = DeleteTableRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionToken") {
        input.version_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTableBucketRequest, String> {
    let mut input = DeleteTableBucketRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_bucket_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTableBucketEncryptionRequest, String> {
    let mut input = DeleteTableBucketEncryptionRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_bucket_metrics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTableBucketMetricsConfigurationRequest, String> {
    let mut input = DeleteTableBucketMetricsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTableBucketPolicyRequest, String> {
    let mut input = DeleteTableBucketPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTableBucketReplicationRequest, String> {
    let mut input = DeleteTableBucketReplicationRequest::default();
    if let Some(value) = query.get("tableBucketARN") {
        input.table_bucket_a_r_n = value.to_string();
    }
    if let Some(value) = query.get("versionToken") {
        input.version_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTablePolicyRequest, String> {
    let mut input = DeleteTablePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_table_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTableReplicationRequest, String> {
    let mut input = DeleteTableReplicationRequest::default();
    if let Some(value) = query.get("tableArn") {
        input.table_arn = value.to_string();
    }
    if let Some(value) = query.get("versionToken") {
        input.version_token = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetNamespaceRequest, String> {
    let mut input = GetNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableRequest, String> {
    let mut input = GetTableRequest::default();
    if let Some(value) = query.get("name") {
        input.name = Some(value.to_string());
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("tableArn") {
        input.table_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("tableBucketARN") {
        input.table_bucket_a_r_n = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableBucketRequest, String> {
    let mut input = GetTableBucketRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_bucket_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableBucketEncryptionRequest, String> {
    let mut input = GetTableBucketEncryptionRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_bucket_maintenance_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableBucketMaintenanceConfigurationRequest, String> {
    let mut input = GetTableBucketMaintenanceConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_bucket_metrics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableBucketMetricsConfigurationRequest, String> {
    let mut input = GetTableBucketMetricsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableBucketPolicyRequest, String> {
    let mut input = GetTableBucketPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableBucketReplicationRequest, String> {
    let mut input = GetTableBucketReplicationRequest::default();
    if let Some(value) = query.get("tableBucketARN") {
        input.table_bucket_a_r_n = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_bucket_storage_class_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableBucketStorageClassRequest, String> {
    let mut input = GetTableBucketStorageClassRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableEncryptionRequest, String> {
    let mut input = GetTableEncryptionRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_maintenance_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableMaintenanceConfigurationRequest, String> {
    let mut input = GetTableMaintenanceConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_maintenance_job_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableMaintenanceJobStatusRequest, String> {
    let mut input = GetTableMaintenanceJobStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_metadata_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableMetadataLocationRequest, String> {
    let mut input = GetTableMetadataLocationRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTablePolicyRequest, String> {
    let mut input = GetTablePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_record_expiration_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableRecordExpirationConfigurationRequest, String> {
    let mut input = GetTableRecordExpirationConfigurationRequest::default();
    if let Some(value) = query.get("tableArn") {
        input.table_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_record_expiration_job_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableRecordExpirationJobStatusRequest, String> {
    let mut input = GetTableRecordExpirationJobStatusRequest::default();
    if let Some(value) = query.get("tableArn") {
        input.table_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableReplicationRequest, String> {
    let mut input = GetTableReplicationRequest::default();
    if let Some(value) = query.get("tableArn") {
        input.table_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_replication_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableReplicationStatusRequest, String> {
    let mut input = GetTableReplicationStatusRequest::default();
    if let Some(value) = query.get("tableArn") {
        input.table_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_storage_class_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableStorageClassRequest, String> {
    let mut input = GetTableStorageClassRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_namespaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNamespacesRequest, String> {
    let mut input = ListNamespacesRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("continuationToken") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = query.get("maxNamespaces") {
        input.max_namespaces = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("prefix") {
        input.prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_table_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTableBucketsRequest, String> {
    let mut input = ListTableBucketsRequest::default();
    if let Some(value) = query.get("continuationToken") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = query.get("maxBuckets") {
        input.max_buckets = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("prefix") {
        input.prefix = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tables_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTablesRequest, String> {
    let mut input = ListTablesRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("continuationToken") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = query.get("maxTables") {
        input.max_tables = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("prefix") {
        input.prefix = Some(value.to_string());
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
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_bucket_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableBucketEncryptionRequest, String> {
    let mut input = PutTableBucketEncryptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTableBucketEncryptionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutTableBucketEncryption request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_bucket_maintenance_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableBucketMaintenanceConfigurationRequest, String> {
    let mut input = PutTableBucketMaintenanceConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTableBucketMaintenanceConfigurationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize PutTableBucketMaintenanceConfiguration request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            "type" => {
                input.r#type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_bucket_metrics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableBucketMetricsConfigurationRequest, String> {
    let mut input = PutTableBucketMetricsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableBucketPolicyRequest, String> {
    let mut input = PutTableBucketPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTableBucketPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTableBucketPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableBucketReplicationRequest, String> {
    let mut input = PutTableBucketReplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTableBucketReplicationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutTableBucketReplication request: {err}"),
        )?;
    }
    if let Some(value) = query.get("tableBucketARN") {
        input.table_bucket_a_r_n = value.to_string();
    }
    if let Some(value) = query.get("versionToken") {
        input.version_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_bucket_storage_class_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableBucketStorageClassRequest, String> {
    let mut input = PutTableBucketStorageClassRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTableBucketStorageClassRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutTableBucketStorageClass request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_maintenance_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableMaintenanceConfigurationRequest, String> {
    let mut input = PutTableMaintenanceConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTableMaintenanceConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutTableMaintenanceConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            "type" => {
                input.r#type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTablePolicyRequest, String> {
    let mut input = PutTablePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTablePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTablePolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_record_expiration_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableRecordExpirationConfigurationRequest, String> {
    let mut input = PutTableRecordExpirationConfigurationRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<PutTableRecordExpirationConfigurationRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize PutTableRecordExpirationConfiguration request: {err}"
                )
            })?;
    }
    if let Some(value) = query.get("tableArn") {
        input.table_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_table_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTableReplicationRequest, String> {
    let mut input = PutTableReplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTableReplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTableReplication request: {err}"))?;
    }
    if let Some(value) = query.get("tableArn") {
        input.table_arn = value.to_string();
    }
    if let Some(value) = query.get("versionToken") {
        input.version_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_rename_table_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RenameTableRequest, String> {
    let mut input = RenameTableRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RenameTableRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RenameTable request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
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
            "resourceArn" => {
                input.resource_arn = value.to_string();
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
            "resourceArn" => {
                input.resource_arn = value.to_string();
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
pub fn deserialize_update_table_metadata_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTableMetadataLocationRequest, String> {
    let mut input = UpdateTableMetadataLocationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTableMetadataLocationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateTableMetadataLocation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "namespace" => {
                input.namespace = value.to_string();
            }
            "tableBucketARN" => {
                input.table_bucket_a_r_n = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
