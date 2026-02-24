//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restXml protocol.
pub fn serialize_abort_multipart_upload_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(204, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_complete_multipart_upload_response(
    result: &CompleteMultipartUploadOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "x-amz-expiration": set by caller from expiration field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_copy_object_response(result: &CopyObjectResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "x-amz-copy-source-version-id": set by caller from copy_source_version_id field
    // Header "x-amz-expiration": set by caller from expiration field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-server-side-encryption-customer-algorithm": set by caller from s_s_e_customer_algorithm field
    // Header "x-amz-server-side-encryption-customer-key-md5": set by caller from s_s_e_customer_key_m_d5 field
    // Header "x-amz-server-side-encryption-context": set by caller from s_s_e_k_m_s_encryption_context field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_bucket_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-bucket-arn": set by caller from bucket_arn field
    // Header "location": set by caller from location field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_create_bucket_metadata_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_create_bucket_metadata_table_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_create_multipart_upload_response(
    result: &CreateMultipartUploadOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-abort-date": set by caller from abort_date field
    // Header "x-amz-abort-rule-id": set by caller from abort_rule_id field
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "x-amz-checksum-algorithm": set by caller from checksum_algorithm field
    // Header "x-amz-checksum-type": set by caller from checksum_type field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-server-side-encryption-customer-algorithm": set by caller from s_s_e_customer_algorithm field
    // Header "x-amz-server-side-encryption-customer-key-md5": set by caller from s_s_e_customer_key_m_d5 field
    // Header "x-amz-server-side-encryption-context": set by caller from s_s_e_k_m_s_encryption_context field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_session_response(result: &CreateSessionOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "x-amz-server-side-encryption-context": set by caller from s_s_e_k_m_s_encryption_context field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_analytics_configuration_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_cors_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_encryption_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_intelligent_tiering_configuration_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_inventory_configuration_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_lifecycle_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_metadata_configuration_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_metadata_table_configuration_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_metrics_configuration_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_ownership_controls_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_policy_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_replication_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_tagging_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_bucket_website_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_object_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(204, body);
    // Header "x-amz-delete-marker": set by caller from delete_marker field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_object_tagging_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(204, body);
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_objects_response(result: &DeleteObjectsOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_public_access_block_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_abac_response(result: &AbacStatus) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_accelerate_configuration_response(
    result: &GetBucketAccelerateConfigurationOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_acl_response(result: &GetBucketAclOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_analytics_configuration_response(
    result: &AnalyticsConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_cors_response(result: &GetBucketCorsOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_encryption_response(
    result: &ServerSideEncryptionConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_intelligent_tiering_configuration_response(
    result: &IntelligentTieringConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_inventory_configuration_response(
    result: &InventoryConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_lifecycle_configuration_response(
    result: &GetBucketLifecycleConfigurationOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-transition-default-minimum-object-size": set by caller from transition_default_minimum_object_size field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_location_response(result: &GetBucketLocationOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_logging_response(result: &GetBucketLoggingOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_metadata_configuration_response(
    result: &GetBucketMetadataConfigurationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_metadata_table_configuration_response(
    result: &GetBucketMetadataTableConfigurationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_metrics_configuration_response(
    result: &MetricsConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_notification_configuration_response(
    result: &NotificationConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_ownership_controls_response(
    result: &OwnershipControls,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_policy_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_policy_status_response(result: &PolicyStatus) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_replication_response(
    result: &ReplicationConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_request_payment_response(
    result: &GetBucketRequestPaymentOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_tagging_response(result: &GetBucketTaggingOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_versioning_response(
    result: &GetBucketVersioningOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_bucket_website_response(result: &GetBucketWebsiteOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "accept-ranges": set by caller from accept_ranges field
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "cache-control": set by caller from cache_control field
    // Header "x-amz-checksum-crc32": set by caller from checksum_c_r_c32 field
    // Header "x-amz-checksum-crc32c": set by caller from checksum_c_r_c32_c field
    // Header "x-amz-checksum-crc64nvme": set by caller from checksum_c_r_c64_n_v_m_e field
    // Header "x-amz-checksum-sha1": set by caller from checksum_s_h_a1 field
    // Header "x-amz-checksum-sha256": set by caller from checksum_s_h_a256 field
    // Header "x-amz-checksum-type": set by caller from checksum_type field
    // Header "content-disposition": set by caller from content_disposition field
    // Header "content-encoding": set by caller from content_encoding field
    // Header "content-language": set by caller from content_language field
    // Header "content-length": set by caller from content_length field
    // Header "content-range": set by caller from content_range field
    // Header "content-type": set by caller from content_type field
    // Header "x-amz-delete-marker": set by caller from delete_marker field
    // Header "etag": set by caller from e_tag field
    // Header "x-amz-expiration": set by caller from expiration field
    // Header "expires": set by caller from expires field
    // Header "last-modified": set by caller from last_modified field
    // Header "x-amz-missing-meta": set by caller from missing_meta field
    // Header "x-amz-object-lock-legal-hold": set by caller from object_lock_legal_hold_status field
    // Header "x-amz-object-lock-mode": set by caller from object_lock_mode field
    // Header "x-amz-object-lock-retain-until-date": set by caller from object_lock_retain_until_date field
    // Header "x-amz-mp-parts-count": set by caller from parts_count field
    // Header "x-amz-replication-status": set by caller from replication_status field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-restore": set by caller from restore field
    // Header "x-amz-server-side-encryption-customer-algorithm": set by caller from s_s_e_customer_algorithm field
    // Header "x-amz-server-side-encryption-customer-key-md5": set by caller from s_s_e_customer_key_m_d5 field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    // Header "x-amz-storage-class": set by caller from storage_class field
    // Header "x-amz-tagging-count": set by caller from tag_count field
    // Header "x-amz-version-id": set by caller from version_id field
    // Header "x-amz-website-redirect-location": set by caller from website_redirect_location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_acl_response(result: &GetObjectAclOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_attributes_response(
    result: &GetObjectAttributesOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-delete-marker": set by caller from delete_marker field
    // Header "last-modified": set by caller from last_modified field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_legal_hold_response(result: &ObjectLockLegalHold) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_lock_configuration_response(
    result: &ObjectLockConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_retention_response(result: &ObjectLockRetention) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_tagging_response(result: &GetObjectTaggingOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_object_torrent_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_public_access_block_response(
    result: &PublicAccessBlockConfiguration,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_head_bucket_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-access-point-alias": set by caller from access_point_alias field
    // Header "x-amz-bucket-arn": set by caller from bucket_arn field
    // Header "x-amz-bucket-location-name": set by caller from bucket_location_name field
    // Header "x-amz-bucket-location-type": set by caller from bucket_location_type field
    // Header "x-amz-bucket-region": set by caller from bucket_region field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_head_object_response(result: &HeadObjectOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "accept-ranges": set by caller from accept_ranges field
    // Header "x-amz-archive-status": set by caller from archive_status field
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "cache-control": set by caller from cache_control field
    // Header "x-amz-checksum-crc32": set by caller from checksum_c_r_c32 field
    // Header "x-amz-checksum-crc32c": set by caller from checksum_c_r_c32_c field
    // Header "x-amz-checksum-crc64nvme": set by caller from checksum_c_r_c64_n_v_m_e field
    // Header "x-amz-checksum-sha1": set by caller from checksum_s_h_a1 field
    // Header "x-amz-checksum-sha256": set by caller from checksum_s_h_a256 field
    // Header "x-amz-checksum-type": set by caller from checksum_type field
    // Header "content-disposition": set by caller from content_disposition field
    // Header "content-encoding": set by caller from content_encoding field
    // Header "content-language": set by caller from content_language field
    // Header "content-length": set by caller from content_length field
    // Header "content-range": set by caller from content_range field
    // Header "content-type": set by caller from content_type field
    // Header "x-amz-delete-marker": set by caller from delete_marker field
    // Header "etag": set by caller from e_tag field
    // Header "x-amz-expiration": set by caller from expiration field
    // Header "expires": set by caller from expires field
    // Header "last-modified": set by caller from last_modified field
    // Header "x-amz-missing-meta": set by caller from missing_meta field
    // Header "x-amz-object-lock-legal-hold": set by caller from object_lock_legal_hold_status field
    // Header "x-amz-object-lock-mode": set by caller from object_lock_mode field
    // Header "x-amz-object-lock-retain-until-date": set by caller from object_lock_retain_until_date field
    // Header "x-amz-mp-parts-count": set by caller from parts_count field
    // Header "x-amz-replication-status": set by caller from replication_status field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-restore": set by caller from restore field
    // Header "x-amz-server-side-encryption-customer-algorithm": set by caller from s_s_e_customer_algorithm field
    // Header "x-amz-server-side-encryption-customer-key-md5": set by caller from s_s_e_customer_key_m_d5 field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    // Header "x-amz-storage-class": set by caller from storage_class field
    // Header "x-amz-tagging-count": set by caller from tag_count field
    // Header "x-amz-version-id": set by caller from version_id field
    // Header "x-amz-website-redirect-location": set by caller from website_redirect_location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_list_bucket_analytics_configurations_response(
    result: &ListBucketAnalyticsConfigurationsOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_bucket_intelligent_tiering_configurations_response(
    result: &ListBucketIntelligentTieringConfigurationsOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_bucket_inventory_configurations_response(
    result: &ListBucketInventoryConfigurationsOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_bucket_metrics_configurations_response(
    result: &ListBucketMetricsConfigurationsOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_buckets_response(result: &ListBucketsOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_directory_buckets_response(
    result: &ListDirectoryBucketsOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_multipart_uploads_response(
    result: &ListMultipartUploadsOutput,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_list_object_versions_response(result: &ListObjectVersionsOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_list_objects_response(result: &ListObjectsOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_list_objects_v2_response(result: &ListObjectsV2Output) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_list_parts_response(result: &ListPartsOutput) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-abort-date": set by caller from abort_date field
    // Header "x-amz-abort-rule-id": set by caller from abort_rule_id field
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_abac_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_accelerate_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_acl_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_analytics_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_cors_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_encryption_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_intelligent_tiering_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_inventory_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_put_bucket_lifecycle_configuration_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-transition-default-minimum-object-size": set by caller from transition_default_minimum_object_size field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_logging_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_metrics_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_notification_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_ownership_controls_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_policy_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_replication_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_request_payment_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_tagging_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_versioning_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_bucket_website_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_put_object_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "x-amz-checksum-crc32": set by caller from checksum_c_r_c32 field
    // Header "x-amz-checksum-crc32c": set by caller from checksum_c_r_c32_c field
    // Header "x-amz-checksum-crc64nvme": set by caller from checksum_c_r_c64_n_v_m_e field
    // Header "x-amz-checksum-sha1": set by caller from checksum_s_h_a1 field
    // Header "x-amz-checksum-sha256": set by caller from checksum_s_h_a256 field
    // Header "x-amz-checksum-type": set by caller from checksum_type field
    // Header "etag": set by caller from e_tag field
    // Header "x-amz-expiration": set by caller from expiration field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-server-side-encryption-customer-algorithm": set by caller from s_s_e_customer_algorithm field
    // Header "x-amz-server-side-encryption-customer-key-md5": set by caller from s_s_e_customer_key_m_d5 field
    // Header "x-amz-server-side-encryption-context": set by caller from s_s_e_k_m_s_encryption_context field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    // Header "x-amz-object-size": set by caller from size field
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_put_object_acl_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_put_object_legal_hold_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_put_object_lock_configuration_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_put_object_retention_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_put_object_tagging_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-version-id": set by caller from version_id field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_put_public_access_block_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_rename_object_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_restore_object_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-restore-output-path": set by caller from restore_output_path field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_select_object_content_response(
    result: &SelectObjectContentEventStream,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_update_bucket_metadata_inventory_table_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_update_bucket_metadata_journal_table_configuration_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_update_object_encryption_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-request-charged": set by caller from request_charged field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_upload_part_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "x-amz-checksum-crc32": set by caller from checksum_c_r_c32 field
    // Header "x-amz-checksum-crc32c": set by caller from checksum_c_r_c32_c field
    // Header "x-amz-checksum-crc64nvme": set by caller from checksum_c_r_c64_n_v_m_e field
    // Header "x-amz-checksum-sha1": set by caller from checksum_s_h_a1 field
    // Header "x-amz-checksum-sha256": set by caller from checksum_s_h_a256 field
    // Header "etag": set by caller from e_tag field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-server-side-encryption-customer-algorithm": set by caller from s_s_e_customer_algorithm field
    // Header "x-amz-server-side-encryption-customer-key-md5": set by caller from s_s_e_customer_key_m_d5 field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_upload_part_copy_response(result: &CopyPartResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "x-amz-server-side-encryption-bucket-key-enabled": set by caller from bucket_key_enabled field
    // Header "x-amz-copy-source-version-id": set by caller from copy_source_version_id field
    // Header "x-amz-request-charged": set by caller from request_charged field
    // Header "x-amz-server-side-encryption-customer-algorithm": set by caller from s_s_e_customer_algorithm field
    // Header "x-amz-server-side-encryption-customer-key-md5": set by caller from s_s_e_customer_key_m_d5 field
    // Header "x-amz-server-side-encryption-aws-kms-key-id": set by caller from s_s_e_k_m_s_key_id field
    // Header "x-amz-server-side-encryption": set by caller from server_side_encryption field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_write_get_object_response_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Deserialize request for restXml protocol.
pub fn deserialize_abort_multipart_upload_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AbortMultipartUploadRequest, String> {
    let mut input = AbortMultipartUploadRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("uploadId") {
        input.upload_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-if-match-initiated-time")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match_initiated_time = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_complete_multipart_upload_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CompleteMultipartUploadRequest, String> {
    let mut input = CompleteMultipartUploadRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CompletedMultipartUpload>(body).map_err(|err| {
            format!("failed to deserialize CompleteMultipartUpload payload: {err}")
        })?;
        input.multipart_upload = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("uploadId") {
        input.upload_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc32")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc32c")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32_c = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc64nvme")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c64_n_v_m_e = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-sha1")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a1 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-sha256")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a256 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-type")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-None-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-mp-object-size")
        .and_then(|value| value.to_str().ok())
    {
        input.mpu_object_size = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_copy_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CopyObjectRequest, String> {
    let mut input = CopyObjectRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CopyObjectRequest>(body)
            .map_err(|err| format!("failed to deserialize CopyObject request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-acl")
        .and_then(|value| value.to_str().ok())
    {
        input.a_c_l = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-bucket-key-enabled")
        .and_then(|value| value.to_str().ok())
    {
        input.bucket_key_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("Cache-Control")
        .and_then(|value| value.to_str().ok())
    {
        input.cache_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Disposition")
        .and_then(|value| value.to_str().ok())
    {
        input.content_disposition = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Encoding")
        .and_then(|value| value.to_str().ok())
    {
        input.content_encoding = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Language")
        .and_then(|value| value.to_str().ok())
    {
        input.content_language = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.content_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-match")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-modified-since")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_modified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-none-match")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-unmodified-since")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_unmodified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-source-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_source_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Expires")
        .and_then(|value| value.to_str().ok())
    {
        input.expires = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-full-control")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_full_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-None-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-metadata-directive")
        .and_then(|value| value.to_str().ok())
    {
        input.metadata_directive = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-legal-hold")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_legal_hold_status = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-mode")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_mode = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-retain-until-date")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_retain_until_date = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-context")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_encryption_context = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-aws-kms-key-id")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption")
        .and_then(|value| value.to_str().ok())
    {
        input.server_side_encryption = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-storage-class")
        .and_then(|value| value.to_str().ok())
    {
        input.storage_class = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-tagging")
        .and_then(|value| value.to_str().ok())
    {
        input.tagging = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-tagging-directive")
        .and_then(|value| value.to_str().ok())
    {
        input.tagging_directive = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-website-redirect-location")
        .and_then(|value| value.to_str().ok())
    {
        input.website_redirect_location = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBucketRequest, String> {
    let mut input = CreateBucketRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CreateBucketConfiguration>(body)
            .map_err(|err| format!("failed to deserialize CreateBucket payload: {err}"))?;
        input.create_bucket_configuration = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-acl")
        .and_then(|value| value.to_str().ok())
    {
        input.a_c_l = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bucket-namespace")
        .and_then(|value| value.to_str().ok())
    {
        input.bucket_namespace = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-full-control")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_full_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bucket-object-lock-enabled")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_enabled_for_bucket = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-ownership")
        .and_then(|value| value.to_str().ok())
    {
        input.object_ownership = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_bucket_metadata_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBucketMetadataConfigurationRequest, String> {
    let mut input = CreateBucketMetadataConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<MetadataConfiguration>(body).map_err(|err| {
            format!("failed to deserialize CreateBucketMetadataConfiguration payload: {err}")
        })?;
        input.metadata_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_bucket_metadata_table_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBucketMetadataTableConfigurationRequest, String> {
    let mut input = CreateBucketMetadataTableConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<MetadataTableConfiguration>(body).map_err(|err| {
                format!(
                    "failed to deserialize CreateBucketMetadataTableConfiguration payload: {err}"
                )
            })?;
        input.metadata_table_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_multipart_upload_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMultipartUploadRequest, String> {
    let mut input = CreateMultipartUploadRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateMultipartUploadRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateMultipartUpload request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-acl")
        .and_then(|value| value.to_str().ok())
    {
        input.a_c_l = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-bucket-key-enabled")
        .and_then(|value| value.to_str().ok())
    {
        input.bucket_key_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("Cache-Control")
        .and_then(|value| value.to_str().ok())
    {
        input.cache_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-type")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Disposition")
        .and_then(|value| value.to_str().ok())
    {
        input.content_disposition = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Encoding")
        .and_then(|value| value.to_str().ok())
    {
        input.content_encoding = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Language")
        .and_then(|value| value.to_str().ok())
    {
        input.content_language = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.content_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Expires")
        .and_then(|value| value.to_str().ok())
    {
        input.expires = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-full-control")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_full_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-legal-hold")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_legal_hold_status = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-mode")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_mode = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-retain-until-date")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_retain_until_date = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-context")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_encryption_context = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-aws-kms-key-id")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption")
        .and_then(|value| value.to_str().ok())
    {
        input.server_side_encryption = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-storage-class")
        .and_then(|value| value.to_str().ok())
    {
        input.storage_class = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-tagging")
        .and_then(|value| value.to_str().ok())
    {
        input.tagging = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-website-redirect-location")
        .and_then(|value| value.to_str().ok())
    {
        input.website_redirect_location = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSessionRequest, String> {
    let mut input = CreateSessionRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-bucket-key-enabled")
        .and_then(|value| value.to_str().ok())
    {
        input.bucket_key_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-context")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_encryption_context = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-aws-kms-key-id")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption")
        .and_then(|value| value.to_str().ok())
    {
        input.server_side_encryption = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-create-session-mode")
        .and_then(|value| value.to_str().ok())
    {
        input.session_mode = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketRequest, String> {
    let mut input = DeleteBucketRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_analytics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketAnalyticsConfigurationRequest, String> {
    let mut input = DeleteBucketAnalyticsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_cors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketCorsRequest, String> {
    let mut input = DeleteBucketCorsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketEncryptionRequest, String> {
    let mut input = DeleteBucketEncryptionRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_intelligent_tiering_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketIntelligentTieringConfigurationRequest, String> {
    let mut input = DeleteBucketIntelligentTieringConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_inventory_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketInventoryConfigurationRequest, String> {
    let mut input = DeleteBucketInventoryConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_lifecycle_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketLifecycleRequest, String> {
    let mut input = DeleteBucketLifecycleRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_metadata_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketMetadataConfigurationRequest, String> {
    let mut input = DeleteBucketMetadataConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_metadata_table_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketMetadataTableConfigurationRequest, String> {
    let mut input = DeleteBucketMetadataTableConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_metrics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketMetricsConfigurationRequest, String> {
    let mut input = DeleteBucketMetricsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_ownership_controls_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketOwnershipControlsRequest, String> {
    let mut input = DeleteBucketOwnershipControlsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketPolicyRequest, String> {
    let mut input = DeleteBucketPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketReplicationRequest, String> {
    let mut input = DeleteBucketReplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketTaggingRequest, String> {
    let mut input = DeleteBucketTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_bucket_website_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBucketWebsiteRequest, String> {
    let mut input = DeleteBucketWebsiteRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteObjectRequest, String> {
    let mut input = DeleteObjectRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bypass-governance-retention")
        .and_then(|value| value.to_str().ok())
    {
        input.bypass_governance_retention = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-if-match-last-modified-time")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match_last_modified_time = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-if-match-size")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match_size = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-mfa")
        .and_then(|value| value.to_str().ok())
    {
        input.m_f_a = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_object_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteObjectTaggingRequest, String> {
    let mut input = DeleteObjectTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_objects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteObjectsRequest, String> {
    let mut input = DeleteObjectsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<Delete>(body)
            .map_err(|err| format!("failed to deserialize DeleteObjects payload: {err}"))?;
        input.delete = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bypass-governance-retention")
        .and_then(|value| value.to_str().ok())
    {
        input.bypass_governance_retention = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-mfa")
        .and_then(|value| value.to_str().ok())
    {
        input.m_f_a = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_public_access_block_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePublicAccessBlockRequest, String> {
    let mut input = DeletePublicAccessBlockRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_abac_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketAbacRequest, String> {
    let mut input = GetBucketAbacRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_accelerate_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketAccelerateConfigurationRequest, String> {
    let mut input = GetBucketAccelerateConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_acl_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketAclRequest, String> {
    let mut input = GetBucketAclRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_analytics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketAnalyticsConfigurationRequest, String> {
    let mut input = GetBucketAnalyticsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_cors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketCorsRequest, String> {
    let mut input = GetBucketCorsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketEncryptionRequest, String> {
    let mut input = GetBucketEncryptionRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_intelligent_tiering_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketIntelligentTieringConfigurationRequest, String> {
    let mut input = GetBucketIntelligentTieringConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_inventory_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketInventoryConfigurationRequest, String> {
    let mut input = GetBucketInventoryConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_lifecycle_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketLifecycleConfigurationRequest, String> {
    let mut input = GetBucketLifecycleConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketLocationRequest, String> {
    let mut input = GetBucketLocationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_logging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketLoggingRequest, String> {
    let mut input = GetBucketLoggingRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_metadata_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketMetadataConfigurationRequest, String> {
    let mut input = GetBucketMetadataConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_metadata_table_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketMetadataTableConfigurationRequest, String> {
    let mut input = GetBucketMetadataTableConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_metrics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketMetricsConfigurationRequest, String> {
    let mut input = GetBucketMetricsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_notification_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketNotificationConfigurationRequest, String> {
    let mut input = GetBucketNotificationConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_ownership_controls_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketOwnershipControlsRequest, String> {
    let mut input = GetBucketOwnershipControlsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketPolicyRequest, String> {
    let mut input = GetBucketPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_policy_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketPolicyStatusRequest, String> {
    let mut input = GetBucketPolicyStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketReplicationRequest, String> {
    let mut input = GetBucketReplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_request_payment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketRequestPaymentRequest, String> {
    let mut input = GetBucketRequestPaymentRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketTaggingRequest, String> {
    let mut input = GetBucketTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_versioning_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketVersioningRequest, String> {
    let mut input = GetBucketVersioningRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_bucket_website_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBucketWebsiteRequest, String> {
    let mut input = GetBucketWebsiteRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectRequest, String> {
    let mut input = GetObjectRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("partNumber") {
        input.part_number = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("response-cache-control") {
        input.response_cache_control = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-disposition") {
        input.response_content_disposition = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-encoding") {
        input.response_content_encoding = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-language") {
        input.response_content_language = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-type") {
        input.response_content_type = Some(value.to_string());
    }
    if let Some(value) = query.get("response-expires") {
        input.response_expires = Some(value.to_string());
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-mode")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_mode = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Modified-Since")
        .and_then(|value| value.to_str().ok())
    {
        input.if_modified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-None-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Unmodified-Since")
        .and_then(|value| value.to_str().ok())
    {
        input.if_unmodified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Range")
        .and_then(|value| value.to_str().ok())
    {
        input.range = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_acl_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectAclRequest, String> {
    let mut input = GetObjectAclRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectAttributesRequest, String> {
    let mut input = GetObjectAttributesRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-max-parts")
        .and_then(|value| value.to_str().ok())
    {
        input.max_parts = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-attributes")
        .and_then(|value| value.to_str().ok())
    {
        input.object_attributes = ObjectAttributesList {
            items: value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        };
    }
    if let Some(value) = request
        .headers
        .get("x-amz-part-number-marker")
        .and_then(|value| value.to_str().ok())
    {
        input.part_number_marker = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_legal_hold_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectLegalHoldRequest, String> {
    let mut input = GetObjectLegalHoldRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_lock_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectLockConfigurationRequest, String> {
    let mut input = GetObjectLockConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_retention_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectRetentionRequest, String> {
    let mut input = GetObjectRetentionRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectTaggingRequest, String> {
    let mut input = GetObjectTaggingRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_object_torrent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectTorrentRequest, String> {
    let mut input = GetObjectTorrentRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_public_access_block_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPublicAccessBlockRequest, String> {
    let mut input = GetPublicAccessBlockRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_head_bucket_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<HeadBucketRequest, String> {
    let mut input = HeadBucketRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_head_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<HeadObjectRequest, String> {
    let mut input = HeadObjectRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("partNumber") {
        input.part_number = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("response-cache-control") {
        input.response_cache_control = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-disposition") {
        input.response_content_disposition = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-encoding") {
        input.response_content_encoding = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-language") {
        input.response_content_language = Some(value.to_string());
    }
    if let Some(value) = query.get("response-content-type") {
        input.response_content_type = Some(value.to_string());
    }
    if let Some(value) = query.get("response-expires") {
        input.response_expires = Some(value.to_string());
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-mode")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_mode = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Modified-Since")
        .and_then(|value| value.to_str().ok())
    {
        input.if_modified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-None-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Unmodified-Since")
        .and_then(|value| value.to_str().ok())
    {
        input.if_unmodified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Range")
        .and_then(|value| value.to_str().ok())
    {
        input.range = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_bucket_analytics_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBucketAnalyticsConfigurationsRequest, String> {
    let mut input = ListBucketAnalyticsConfigurationsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("continuation-token") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_bucket_intelligent_tiering_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBucketIntelligentTieringConfigurationsRequest, String> {
    let mut input = ListBucketIntelligentTieringConfigurationsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("continuation-token") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_bucket_inventory_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBucketInventoryConfigurationsRequest, String> {
    let mut input = ListBucketInventoryConfigurationsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("continuation-token") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_bucket_metrics_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBucketMetricsConfigurationsRequest, String> {
    let mut input = ListBucketMetricsConfigurationsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("continuation-token") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBucketsRequest, String> {
    let mut input = ListBucketsRequest::default();
    if let Some(value) = query.get("bucket-region") {
        input.bucket_region = Some(value.to_string());
    }
    if let Some(value) = query.get("continuation-token") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = query.get("max-buckets") {
        input.max_buckets = Some(
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_directory_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDirectoryBucketsRequest, String> {
    let mut input = ListDirectoryBucketsRequest::default();
    if let Some(value) = query.get("continuation-token") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = query.get("max-directory-buckets") {
        input.max_directory_buckets = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_multipart_uploads_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMultipartUploadsRequest, String> {
    let mut input = ListMultipartUploadsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("delimiter") {
        input.delimiter = Some(value.to_string());
    }
    if let Some(value) = query.get("encoding-type") {
        input.encoding_type = Some(value.to_string());
    }
    if let Some(value) = query.get("key-marker") {
        input.key_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("max-uploads") {
        input.max_uploads = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("prefix") {
        input.prefix = Some(value.to_string());
    }
    if let Some(value) = query.get("upload-id-marker") {
        input.upload_id_marker = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_object_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectVersionsRequest, String> {
    let mut input = ListObjectVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("delimiter") {
        input.delimiter = Some(value.to_string());
    }
    if let Some(value) = query.get("encoding-type") {
        input.encoding_type = Some(value.to_string());
    }
    if let Some(value) = query.get("key-marker") {
        input.key_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("max-keys") {
        input.max_keys = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("prefix") {
        input.prefix = Some(value.to_string());
    }
    if let Some(value) = query.get("version-id-marker") {
        input.version_id_marker = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-optional-object-attributes")
        .and_then(|value| value.to_str().ok())
    {
        input.optional_object_attributes = Some(OptionalObjectAttributesList {
            items: value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        });
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_objects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectsRequest, String> {
    let mut input = ListObjectsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("delimiter") {
        input.delimiter = Some(value.to_string());
    }
    if let Some(value) = query.get("encoding-type") {
        input.encoding_type = Some(value.to_string());
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("max-keys") {
        input.max_keys = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("prefix") {
        input.prefix = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-optional-object-attributes")
        .and_then(|value| value.to_str().ok())
    {
        input.optional_object_attributes = Some(OptionalObjectAttributesList {
            items: value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        });
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_objects_v2_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectsV2Request, String> {
    let mut input = ListObjectsV2Request::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("continuation-token") {
        input.continuation_token = Some(value.to_string());
    }
    if let Some(value) = query.get("delimiter") {
        input.delimiter = Some(value.to_string());
    }
    if let Some(value) = query.get("encoding-type") {
        input.encoding_type = Some(value.to_string());
    }
    if let Some(value) = query.get("fetch-owner") {
        input.fetch_owner = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("max-keys") {
        input.max_keys = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("prefix") {
        input.prefix = Some(value.to_string());
    }
    if let Some(value) = query.get("start-after") {
        input.start_after = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-optional-object-attributes")
        .and_then(|value| value.to_str().ok())
    {
        input.optional_object_attributes = Some(OptionalObjectAttributesList {
            items: value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        });
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_parts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPartsRequest, String> {
    let mut input = ListPartsRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-parts") {
        input.max_parts = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("part-number-marker") {
        input.part_number_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("uploadId") {
        input.upload_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_abac_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketAbacRequest, String> {
    let mut input = PutBucketAbacRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<AbacStatus>(body)
            .map_err(|err| format!("failed to deserialize PutBucketAbac payload: {err}"))?;
        input.abac_status = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_accelerate_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketAccelerateConfigurationRequest, String> {
    let mut input = PutBucketAccelerateConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<AccelerateConfiguration>(body).map_err(|err| {
            format!("failed to deserialize PutBucketAccelerateConfiguration payload: {err}")
        })?;
        input.accelerate_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_acl_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketAclRequest, String> {
    let mut input = PutBucketAclRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<AccessControlPolicy>(body)
            .map_err(|err| format!("failed to deserialize PutBucketAcl payload: {err}"))?;
        input.access_control_policy = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-acl")
        .and_then(|value| value.to_str().ok())
    {
        input.a_c_l = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-full-control")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_full_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write_a_c_p = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_analytics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketAnalyticsConfigurationRequest, String> {
    let mut input = PutBucketAnalyticsConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<AnalyticsConfiguration>(body).map_err(|err| {
            format!("failed to deserialize PutBucketAnalyticsConfiguration payload: {err}")
        })?;
        input.analytics_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_cors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketCorsRequest, String> {
    let mut input = PutBucketCorsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CORSConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutBucketCors payload: {err}"))?;
        input.c_o_r_s_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketEncryptionRequest, String> {
    let mut input = PutBucketEncryptionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<ServerSideEncryptionConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutBucketEncryption payload: {err}"))?;
        input.server_side_encryption_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_intelligent_tiering_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketIntelligentTieringConfigurationRequest, String> {
    let mut input = PutBucketIntelligentTieringConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<IntelligentTieringConfiguration>(body).map_err(|err| {
                format!(
                    "failed to deserialize PutBucketIntelligentTieringConfiguration payload: {err}"
                )
            })?;
        input.intelligent_tiering_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_inventory_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketInventoryConfigurationRequest, String> {
    let mut input = PutBucketInventoryConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<InventoryConfiguration>(body).map_err(|err| {
            format!("failed to deserialize PutBucketInventoryConfiguration payload: {err}")
        })?;
        input.inventory_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_lifecycle_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketLifecycleConfigurationRequest, String> {
    let mut input = PutBucketLifecycleConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<BucketLifecycleConfiguration>(body).map_err(|err| {
                format!("failed to deserialize PutBucketLifecycleConfiguration payload: {err}")
            })?;
        input.lifecycle_configuration = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-transition-default-minimum-object-size")
        .and_then(|value| value.to_str().ok())
    {
        input.transition_default_minimum_object_size = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_logging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketLoggingRequest, String> {
    let mut input = PutBucketLoggingRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<BucketLoggingStatus>(body)
            .map_err(|err| format!("failed to deserialize PutBucketLogging payload: {err}"))?;
        input.bucket_logging_status = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_metrics_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketMetricsConfigurationRequest, String> {
    let mut input = PutBucketMetricsConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<MetricsConfiguration>(body).map_err(|err| {
            format!("failed to deserialize PutBucketMetricsConfiguration payload: {err}")
        })?;
        input.metrics_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("id") {
        input.id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_notification_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketNotificationConfigurationRequest, String> {
    let mut input = PutBucketNotificationConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<NotificationConfiguration>(body).map_err(|err| {
                format!("failed to deserialize PutBucketNotificationConfiguration payload: {err}")
            })?;
        input.notification_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-skip-destination-validation")
        .and_then(|value| value.to_str().ok())
    {
        input.skip_destination_validation = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_ownership_controls_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketOwnershipControlsRequest, String> {
    let mut input = PutBucketOwnershipControlsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<OwnershipControls>(body).map_err(|err| {
            format!("failed to deserialize PutBucketOwnershipControls payload: {err}")
        })?;
        input.ownership_controls = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketPolicyRequest, String> {
    let mut input = PutBucketPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.policy = body.to_string();
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-confirm-remove-self-bucket-access")
        .and_then(|value| value.to_str().ok())
    {
        input.confirm_remove_self_bucket_access = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_replication_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketReplicationRequest, String> {
    let mut input = PutBucketReplicationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<ReplicationConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutBucketReplication payload: {err}"))?;
        input.replication_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bucket-object-lock-token")
        .and_then(|value| value.to_str().ok())
    {
        input.token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_request_payment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketRequestPaymentRequest, String> {
    let mut input = PutBucketRequestPaymentRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<RequestPaymentConfiguration>(body).map_err(|err| {
                format!("failed to deserialize PutBucketRequestPayment payload: {err}")
            })?;
        input.request_payment_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketTaggingRequest, String> {
    let mut input = PutBucketTaggingRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<Tagging>(body)
            .map_err(|err| format!("failed to deserialize PutBucketTagging payload: {err}"))?;
        input.tagging = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_versioning_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketVersioningRequest, String> {
    let mut input = PutBucketVersioningRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<VersioningConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutBucketVersioning payload: {err}"))?;
        input.versioning_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-mfa")
        .and_then(|value| value.to_str().ok())
    {
        input.m_f_a = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_bucket_website_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutBucketWebsiteRequest, String> {
    let mut input = PutBucketWebsiteRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<WebsiteConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutBucketWebsite payload: {err}"))?;
        input.website_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutObjectRequest, String> {
    let mut input = PutObjectRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.body = Some(body.to_string());
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-acl")
        .and_then(|value| value.to_str().ok())
    {
        input.a_c_l = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-bucket-key-enabled")
        .and_then(|value| value.to_str().ok())
    {
        input.bucket_key_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("Cache-Control")
        .and_then(|value| value.to_str().ok())
    {
        input.cache_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc32")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc32c")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32_c = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc64nvme")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c64_n_v_m_e = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-sha1")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a1 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-sha256")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a256 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Disposition")
        .and_then(|value| value.to_str().ok())
    {
        input.content_disposition = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Encoding")
        .and_then(|value| value.to_str().ok())
    {
        input.content_encoding = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Language")
        .and_then(|value| value.to_str().ok())
    {
        input.content_language = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Length")
        .and_then(|value| value.to_str().ok())
    {
        input.content_length = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.content_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Expires")
        .and_then(|value| value.to_str().ok())
    {
        input.expires = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-full-control")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_full_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-None-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-legal-hold")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_legal_hold_status = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-mode")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_mode = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-object-lock-retain-until-date")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_retain_until_date = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-context")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_encryption_context = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-aws-kms-key-id")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption")
        .and_then(|value| value.to_str().ok())
    {
        input.server_side_encryption = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-storage-class")
        .and_then(|value| value.to_str().ok())
    {
        input.storage_class = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-tagging")
        .and_then(|value| value.to_str().ok())
    {
        input.tagging = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-website-redirect-location")
        .and_then(|value| value.to_str().ok())
    {
        input.website_redirect_location = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-write-offset-bytes")
        .and_then(|value| value.to_str().ok())
    {
        input.write_offset_bytes = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_object_acl_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutObjectAclRequest, String> {
    let mut input = PutObjectAclRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<AccessControlPolicy>(body)
            .map_err(|err| format!("failed to deserialize PutObjectAcl payload: {err}"))?;
        input.access_control_policy = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-acl")
        .and_then(|value| value.to_str().ok())
    {
        input.a_c_l = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-full-control")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_full_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-read-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_read_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-grant-write-acp")
        .and_then(|value| value.to_str().ok())
    {
        input.grant_write_a_c_p = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_object_legal_hold_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutObjectLegalHoldRequest, String> {
    let mut input = PutObjectLegalHoldRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<ObjectLockLegalHold>(body)
            .map_err(|err| format!("failed to deserialize PutObjectLegalHold payload: {err}"))?;
        input.legal_hold = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_object_lock_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutObjectLockConfigurationRequest, String> {
    let mut input = PutObjectLockConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<ObjectLockConfiguration>(body).map_err(|err| {
            format!("failed to deserialize PutObjectLockConfiguration payload: {err}")
        })?;
        input.object_lock_configuration = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bucket-object-lock-token")
        .and_then(|value| value.to_str().ok())
    {
        input.token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_object_retention_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutObjectRetentionRequest, String> {
    let mut input = PutObjectRetentionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<ObjectLockRetention>(body)
            .map_err(|err| format!("failed to deserialize PutObjectRetention payload: {err}"))?;
        input.retention = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-bypass-governance-retention")
        .and_then(|value| value.to_str().ok())
    {
        input.bypass_governance_retention = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_object_tagging_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutObjectTaggingRequest, String> {
    let mut input = PutObjectTaggingRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<Tagging>(body)
            .map_err(|err| format!("failed to deserialize PutObjectTagging payload: {err}"))?;
        input.tagging = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_public_access_block_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutPublicAccessBlockRequest, String> {
    let mut input = PutPublicAccessBlockRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<PublicAccessBlockConfiguration>(body)
            .map_err(|err| format!("failed to deserialize PutPublicAccessBlock payload: {err}"))?;
        input.public_access_block_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_rename_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RenameObjectRequest, String> {
    let mut input = RenameObjectRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-client-token")
        .and_then(|value| value.to_str().ok())
    {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.destination_if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Modified-Since")
        .and_then(|value| value.to_str().ok())
    {
        input.destination_if_modified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-None-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.destination_if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Unmodified-Since")
        .and_then(|value| value.to_str().ok())
    {
        input.destination_if_unmodified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-rename-source")
        .and_then(|value| value.to_str().ok())
    {
        input.rename_source = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-rename-source-if-match")
        .and_then(|value| value.to_str().ok())
    {
        input.source_if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-rename-source-if-modified-since")
        .and_then(|value| value.to_str().ok())
    {
        input.source_if_modified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-rename-source-if-none-match")
        .and_then(|value| value.to_str().ok())
    {
        input.source_if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-rename-source-if-unmodified-since")
        .and_then(|value| value.to_str().ok())
    {
        input.source_if_unmodified_since = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_restore_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RestoreObjectRequest, String> {
    let mut input = RestoreObjectRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<RestoreRequest>(body)
            .map_err(|err| format!("failed to deserialize RestoreObject payload: {err}"))?;
        input.restore_request = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_select_object_content_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SelectObjectContentRequest, String> {
    let mut input = SelectObjectContentRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<SelectObjectContentRequest>(body)
            .map_err(|err| format!("failed to deserialize SelectObjectContent request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_bucket_metadata_inventory_table_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBucketMetadataInventoryTableConfigurationRequest, String> {
    let mut input = UpdateBucketMetadataInventoryTableConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<InventoryTableConfigurationUpdates>(body)
            .map_err(|err| format!("failed to deserialize UpdateBucketMetadataInventoryTableConfiguration payload: {err}"))?;
        input.inventory_table_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_bucket_metadata_journal_table_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBucketMetadataJournalTableConfigurationRequest, String> {
    let mut input = UpdateBucketMetadataJournalTableConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<JournalTableConfigurationUpdates>(body)
            .map_err(|err| format!("failed to deserialize UpdateBucketMetadataJournalTableConfiguration payload: {err}"))?;
        input.journal_table_configuration = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_object_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateObjectEncryptionRequest, String> {
    let mut input = UpdateObjectEncryptionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<ObjectEncryption>(body).map_err(|err| {
            format!("failed to deserialize UpdateObjectEncryption payload: {err}")
        })?;
        input.object_encryption = payload;
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("versionId") {
        input.version_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_upload_part_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UploadPartRequest, String> {
    let mut input = UploadPartRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.body = Some(body.to_string());
    }
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("partNumber") {
        input.part_number = value
            .parse::<i32>()
            .map_err(|err| format!("failed to parse integer: {err}"))?;
    }
    if let Some(value) = query.get("uploadId") {
        input.upload_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-sdk-checksum-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc32")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc32c")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32_c = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-crc64nvme")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c64_n_v_m_e = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-sha1")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a1 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-checksum-sha256")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a256 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Length")
        .and_then(|value| value.to_str().ok())
    {
        input.content_length = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("Content-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.content_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_upload_part_copy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UploadPartCopyRequest, String> {
    let mut input = UploadPartCopyRequest::default();
    for (name, value) in labels {
        match *name {
            "Bucket" => {
                input.bucket = value.to_string();
            }
            "Key" => {
                input.key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("partNumber") {
        input.part_number = value
            .parse::<i32>()
            .map_err(|err| format!("failed to parse integer: {err}"))?;
    }
    if let Some(value) = query.get("uploadId") {
        input.upload_id = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-match")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-modified-since")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_modified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-none-match")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_none_match = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-if-unmodified-since")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_if_unmodified_since = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-range")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_range = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-copy-source-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.copy_source_s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-source-expected-bucket-owner")
        .and_then(|value| value.to_str().ok())
    {
        input.expected_source_bucket_owner = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-payer")
        .and_then(|value| value.to_str().ok())
    {
        input.request_payer = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_write_get_object_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<WriteGetObjectResponseRequest, String> {
    let mut input = WriteGetObjectResponseRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.body = Some(body.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-accept-ranges")
        .and_then(|value| value.to_str().ok())
    {
        input.accept_ranges = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-server-side-encryption-bucket-key-enabled")
        .and_then(|value| value.to_str().ok())
    {
        input.bucket_key_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Cache-Control")
        .and_then(|value| value.to_str().ok())
    {
        input.cache_control = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-checksum-crc32")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-checksum-crc32c")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c32_c = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-checksum-crc64nvme")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_c_r_c64_n_v_m_e = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-checksum-sha1")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a1 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-checksum-sha256")
        .and_then(|value| value.to_str().ok())
    {
        input.checksum_s_h_a256 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Content-Disposition")
        .and_then(|value| value.to_str().ok())
    {
        input.content_disposition = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Content-Encoding")
        .and_then(|value| value.to_str().ok())
    {
        input.content_encoding = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Content-Language")
        .and_then(|value| value.to_str().ok())
    {
        input.content_language = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Content-Length")
        .and_then(|value| value.to_str().ok())
    {
        input.content_length = Some(
            value
                .parse::<i64>()
                .map_err(|err| format!("failed to parse long: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Content-Range")
        .and_then(|value| value.to_str().ok())
    {
        input.content_range = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Content-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.content_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-delete-marker")
        .and_then(|value| value.to_str().ok())
    {
        input.delete_marker = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-ETag")
        .and_then(|value| value.to_str().ok())
    {
        input.e_tag = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-error-code")
        .and_then(|value| value.to_str().ok())
    {
        input.error_code = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-error-message")
        .and_then(|value| value.to_str().ok())
    {
        input.error_message = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-expiration")
        .and_then(|value| value.to_str().ok())
    {
        input.expiration = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Expires")
        .and_then(|value| value.to_str().ok())
    {
        input.expires = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-Last-Modified")
        .and_then(|value| value.to_str().ok())
    {
        input.last_modified = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-missing-meta")
        .and_then(|value| value.to_str().ok())
    {
        input.missing_meta = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-object-lock-legal-hold")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_legal_hold_status = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-object-lock-mode")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_mode = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-object-lock-retain-until-date")
        .and_then(|value| value.to_str().ok())
    {
        input.object_lock_retain_until_date = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-mp-parts-count")
        .and_then(|value| value.to_str().ok())
    {
        input.parts_count = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-replication-status")
        .and_then(|value| value.to_str().ok())
    {
        input.replication_status = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-request-charged")
        .and_then(|value| value.to_str().ok())
    {
        input.request_charged = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-route")
        .and_then(|value| value.to_str().ok())
    {
        input.request_route = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-request-token")
        .and_then(|value| value.to_str().ok())
    {
        input.request_token = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-restore")
        .and_then(|value| value.to_str().ok())
    {
        input.restore = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-server-side-encryption-customer-algorithm")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_algorithm = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-server-side-encryption-customer-key-MD5")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_customer_key_m_d5 = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-server-side-encryption-aws-kms-key-id")
        .and_then(|value| value.to_str().ok())
    {
        input.s_s_e_k_m_s_key_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-server-side-encryption")
        .and_then(|value| value.to_str().ok())
    {
        input.server_side_encryption = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-status")
        .and_then(|value| value.to_str().ok())
    {
        input.status_code = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-storage-class")
        .and_then(|value| value.to_str().ok())
    {
        input.storage_class = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-tagging-count")
        .and_then(|value| value.to_str().ok())
    {
        input.tag_count = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amz-fwd-header-x-amz-version-id")
        .and_then(|value| value.to_str().ok())
    {
        input.version_id = Some(value.to_string());
    }
    Ok(input)
}
