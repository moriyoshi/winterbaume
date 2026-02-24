//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-glacier

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

/// Serialize void response for restJson protocol.
pub fn serialize_abort_multipart_upload_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_abort_vault_lock_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_add_tags_to_vault_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_complete_multipart_upload_response(
    result: &ArchiveCreationOutput,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-archive-id": set by caller from archive_id field
    // Header "x-amz-sha256-tree-hash": set by caller from checksum field
    // Header "location": set by caller from location field
    resp
}

/// Serialize void response for restJson protocol.
pub fn serialize_complete_vault_lock_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vault_response(result: &CreateVaultOutput) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_archive_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_vault_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_vault_access_policy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_vault_notifications_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_job_response(result: &GlacierJobDescription) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_vault_response(result: &DescribeVaultOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_retrieval_policy_response(
    result: &GetDataRetrievalPolicyOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_job_output_response(result: &GetJobOutputOutput) -> MockResponse {
    let status = result.status.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "accept-ranges": set by caller from accept_ranges field
    // Header "x-amz-archive-description": set by caller from archive_description field
    // Header "x-amz-sha256-tree-hash": set by caller from checksum field
    // Header "content-range": set by caller from content_range field
    // Header "content-type": set by caller from content_type field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vault_access_policy_response(
    result: &GetVaultAccessPolicyOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = match &result.policy {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vault_lock_response(result: &GetVaultLockOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vault_notifications_response(
    result: &GetVaultNotificationsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = match &result.vault_notification_config {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_initiate_job_response(result: &InitiateJobOutput) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-job-id": set by caller from job_id field
    // Header "x-amz-job-output-path": set by caller from job_output_path field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_initiate_multipart_upload_response(
    result: &InitiateMultipartUploadOutput,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "location": set by caller from location field
    // Header "x-amz-multipart-upload-id": set by caller from upload_id field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_initiate_vault_lock_response(result: &InitiateVaultLockOutput) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-lock-id": set by caller from lock_id field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_list_jobs_response(result: &ListJobsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_multipart_uploads_response(
    result: &ListMultipartUploadsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_parts_response(result: &ListPartsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_provisioned_capacity_response(
    result: &ListProvisionedCapacityOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_vault_response(result: &ListTagsForVaultOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_vaults_response(result: &ListVaultsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_purchase_provisioned_capacity_response(
    result: &PurchaseProvisionedCapacityOutput,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-capacity-id": set by caller from capacity_id field
    resp
}

/// Serialize void response for restJson protocol.
pub fn serialize_remove_tags_from_vault_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_set_data_retrieval_policy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_set_vault_access_policy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_set_vault_notifications_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_upload_archive_response(result: &ArchiveCreationOutput) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-archive-id": set by caller from archive_id field
    // Header "x-amz-sha256-tree-hash": set by caller from checksum field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_upload_multipart_part_response(
    result: &UploadMultipartPartOutput,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-sha256-tree-hash": set by caller from checksum field
    resp
}
