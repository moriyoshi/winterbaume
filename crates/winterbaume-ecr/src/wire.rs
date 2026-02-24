//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ecr

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_check_layer_availability_response(
    result: &BatchCheckLayerAvailabilityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_delete_image_response(result: &BatchDeleteImageResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_image_response(result: &BatchGetImageResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_repository_scanning_configuration_response(
    result: &BatchGetRepositoryScanningConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_complete_layer_upload_response(
    result: &CompleteLayerUploadResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_pull_through_cache_rule_response(
    result: &CreatePullThroughCacheRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_repository_response(result: &CreateRepositoryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_repository_creation_template_response(
    result: &CreateRepositoryCreationTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_lifecycle_policy_response(
    result: &DeleteLifecyclePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_pull_through_cache_rule_response(
    result: &DeletePullThroughCacheRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_registry_policy_response(
    result: &DeleteRegistryPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_repository_response(result: &DeleteRepositoryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_repository_creation_template_response(
    result: &DeleteRepositoryCreationTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_repository_policy_response(
    result: &DeleteRepositoryPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_signing_configuration_response(
    result: &DeleteSigningConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_pull_time_update_exclusion_response(
    result: &DeregisterPullTimeUpdateExclusionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_image_replication_status_response(
    result: &DescribeImageReplicationStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_image_scan_findings_response(
    result: &DescribeImageScanFindingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_image_signing_status_response(
    result: &DescribeImageSigningStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_images_response(result: &DescribeImagesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_pull_through_cache_rules_response(
    result: &DescribePullThroughCacheRulesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_registry_response(result: &DescribeRegistryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_repositories_response(
    result: &DescribeRepositoriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_repository_creation_templates_response(
    result: &DescribeRepositoryCreationTemplatesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_account_setting_response(result: &GetAccountSettingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_authorization_token_response(
    result: &GetAuthorizationTokenResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_download_url_for_layer_response(
    result: &GetDownloadUrlForLayerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_lifecycle_policy_response(
    result: &GetLifecyclePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_lifecycle_policy_preview_response(
    result: &GetLifecyclePolicyPreviewResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_registry_policy_response(result: &GetRegistryPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_registry_scanning_configuration_response(
    result: &GetRegistryScanningConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_repository_policy_response(
    result: &GetRepositoryPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_signing_configuration_response(
    result: &GetSigningConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_initiate_layer_upload_response(
    result: &InitiateLayerUploadResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_image_referrers_response(
    result: &ListImageReferrersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_images_response(result: &ListImagesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_pull_time_update_exclusions_response(
    result: &ListPullTimeUpdateExclusionsResponse,
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
pub fn serialize_put_account_setting_response(result: &PutAccountSettingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_image_response(result: &PutImageResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_image_scanning_configuration_response(
    result: &PutImageScanningConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_image_tag_mutability_response(
    result: &PutImageTagMutabilityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_lifecycle_policy_response(
    result: &PutLifecyclePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_registry_policy_response(result: &PutRegistryPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_registry_scanning_configuration_response(
    result: &PutRegistryScanningConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_replication_configuration_response(
    result: &PutReplicationConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_signing_configuration_response(
    result: &PutSigningConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_pull_time_update_exclusion_response(
    result: &RegisterPullTimeUpdateExclusionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_repository_policy_response(
    result: &SetRepositoryPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_image_scan_response(result: &StartImageScanResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_lifecycle_policy_preview_response(
    result: &StartLifecyclePolicyPreviewResponse,
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
pub fn serialize_update_image_storage_class_response(
    result: &UpdateImageStorageClassResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_pull_through_cache_rule_response(
    result: &UpdatePullThroughCacheRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_repository_creation_template_response(
    result: &UpdateRepositoryCreationTemplateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_upload_layer_part_response(result: &UploadLayerPartResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_validate_pull_through_cache_rule_response(
    result: &ValidatePullThroughCacheRuleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_check_layer_availability_request(
    body: &[u8],
) -> Result<BatchCheckLayerAvailabilityRequest, String> {
    if body.is_empty() {
        return Ok(BatchCheckLayerAvailabilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchCheckLayerAvailability request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_delete_image_request(
    body: &[u8],
) -> Result<BatchDeleteImageRequest, String> {
    if body.is_empty() {
        return Ok(BatchDeleteImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDeleteImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_image_request(body: &[u8]) -> Result<BatchGetImageRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_repository_scanning_configuration_request(
    body: &[u8],
) -> Result<BatchGetRepositoryScanningConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetRepositoryScanningConfigurationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize BatchGetRepositoryScanningConfiguration request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_complete_layer_upload_request(
    body: &[u8],
) -> Result<CompleteLayerUploadRequest, String> {
    if body.is_empty() {
        return Ok(CompleteLayerUploadRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CompleteLayerUpload request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_pull_through_cache_rule_request(
    body: &[u8],
) -> Result<CreatePullThroughCacheRuleRequest, String> {
    if body.is_empty() {
        return Ok(CreatePullThroughCacheRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePullThroughCacheRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_repository_request(
    body: &[u8],
) -> Result<CreateRepositoryRequest, String> {
    if body.is_empty() {
        return Ok(CreateRepositoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRepository request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_repository_creation_template_request(
    body: &[u8],
) -> Result<CreateRepositoryCreationTemplateRequest, String> {
    if body.is_empty() {
        return Ok(CreateRepositoryCreationTemplateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRepositoryCreationTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_lifecycle_policy_request(
    body: &[u8],
) -> Result<DeleteLifecyclePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLifecyclePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLifecyclePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_pull_through_cache_rule_request(
    body: &[u8],
) -> Result<DeletePullThroughCacheRuleRequest, String> {
    if body.is_empty() {
        return Ok(DeletePullThroughCacheRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePullThroughCacheRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_registry_policy_request(
    body: &[u8],
) -> Result<DeleteRegistryPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRegistryPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRegistryPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_repository_request(
    body: &[u8],
) -> Result<DeleteRepositoryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRepositoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRepository request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_repository_creation_template_request(
    body: &[u8],
) -> Result<DeleteRepositoryCreationTemplateRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRepositoryCreationTemplateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRepositoryCreationTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_repository_policy_request(
    body: &[u8],
) -> Result<DeleteRepositoryPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRepositoryPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRepositoryPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_signing_configuration_request(
    body: &[u8],
) -> Result<DeleteSigningConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSigningConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSigningConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_pull_time_update_exclusion_request(
    body: &[u8],
) -> Result<DeregisterPullTimeUpdateExclusionRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterPullTimeUpdateExclusionRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeregisterPullTimeUpdateExclusion request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_image_replication_status_request(
    body: &[u8],
) -> Result<DescribeImageReplicationStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeImageReplicationStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImageReplicationStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_image_scan_findings_request(
    body: &[u8],
) -> Result<DescribeImageScanFindingsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeImageScanFindingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImageScanFindings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_image_signing_status_request(
    body: &[u8],
) -> Result<DescribeImageSigningStatusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeImageSigningStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImageSigningStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_images_request(body: &[u8]) -> Result<DescribeImagesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeImagesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImages request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_pull_through_cache_rules_request(
    body: &[u8],
) -> Result<DescribePullThroughCacheRulesRequest, String> {
    if body.is_empty() {
        return Ok(DescribePullThroughCacheRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePullThroughCacheRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_registry_request(
    body: &[u8],
) -> Result<DescribeRegistryRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRegistryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRegistry request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_repositories_request(
    body: &[u8],
) -> Result<DescribeRepositoriesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRepositoriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRepositories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_repository_creation_templates_request(
    body: &[u8],
) -> Result<DescribeRepositoryCreationTemplatesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRepositoryCreationTemplatesRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeRepositoryCreationTemplates request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_account_setting_request(
    body: &[u8],
) -> Result<GetAccountSettingRequest, String> {
    if body.is_empty() {
        return Ok(GetAccountSettingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAccountSetting request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_authorization_token_request(
    body: &[u8],
) -> Result<GetAuthorizationTokenRequest, String> {
    if body.is_empty() {
        return Ok(GetAuthorizationTokenRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAuthorizationToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_download_url_for_layer_request(
    body: &[u8],
) -> Result<GetDownloadUrlForLayerRequest, String> {
    if body.is_empty() {
        return Ok(GetDownloadUrlForLayerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDownloadUrlForLayer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_lifecycle_policy_request(
    body: &[u8],
) -> Result<GetLifecyclePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetLifecyclePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLifecyclePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_lifecycle_policy_preview_request(
    body: &[u8],
) -> Result<GetLifecyclePolicyPreviewRequest, String> {
    if body.is_empty() {
        return Ok(GetLifecyclePolicyPreviewRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLifecyclePolicyPreview request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_registry_policy_request(
    body: &[u8],
) -> Result<GetRegistryPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetRegistryPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRegistryPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_registry_scanning_configuration_request(
    body: &[u8],
) -> Result<GetRegistryScanningConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetRegistryScanningConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRegistryScanningConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_repository_policy_request(
    body: &[u8],
) -> Result<GetRepositoryPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetRepositoryPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRepositoryPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_signing_configuration_request(
    body: &[u8],
) -> Result<GetSigningConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetSigningConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSigningConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_initiate_layer_upload_request(
    body: &[u8],
) -> Result<InitiateLayerUploadRequest, String> {
    if body.is_empty() {
        return Ok(InitiateLayerUploadRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize InitiateLayerUpload request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_image_referrers_request(
    body: &[u8],
) -> Result<ListImageReferrersRequest, String> {
    if body.is_empty() {
        return Ok(ListImageReferrersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListImageReferrers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_images_request(body: &[u8]) -> Result<ListImagesRequest, String> {
    if body.is_empty() {
        return Ok(ListImagesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListImages request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_pull_time_update_exclusions_request(
    body: &[u8],
) -> Result<ListPullTimeUpdateExclusionsRequest, String> {
    if body.is_empty() {
        return Ok(ListPullTimeUpdateExclusionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPullTimeUpdateExclusions request: {e}"))
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
pub fn deserialize_put_account_setting_request(
    body: &[u8],
) -> Result<PutAccountSettingRequest, String> {
    if body.is_empty() {
        return Ok(PutAccountSettingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAccountSetting request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_image_request(body: &[u8]) -> Result<PutImageRequest, String> {
    if body.is_empty() {
        return Ok(PutImageRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize PutImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_image_scanning_configuration_request(
    body: &[u8],
) -> Result<PutImageScanningConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutImageScanningConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutImageScanningConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_image_tag_mutability_request(
    body: &[u8],
) -> Result<PutImageTagMutabilityRequest, String> {
    if body.is_empty() {
        return Ok(PutImageTagMutabilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutImageTagMutability request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_lifecycle_policy_request(
    body: &[u8],
) -> Result<PutLifecyclePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutLifecyclePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutLifecyclePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_registry_policy_request(
    body: &[u8],
) -> Result<PutRegistryPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutRegistryPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRegistryPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_registry_scanning_configuration_request(
    body: &[u8],
) -> Result<PutRegistryScanningConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutRegistryScanningConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRegistryScanningConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_replication_configuration_request(
    body: &[u8],
) -> Result<PutReplicationConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutReplicationConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutReplicationConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_signing_configuration_request(
    body: &[u8],
) -> Result<PutSigningConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutSigningConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutSigningConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_pull_time_update_exclusion_request(
    body: &[u8],
) -> Result<RegisterPullTimeUpdateExclusionRequest, String> {
    if body.is_empty() {
        return Ok(RegisterPullTimeUpdateExclusionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterPullTimeUpdateExclusion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_repository_policy_request(
    body: &[u8],
) -> Result<SetRepositoryPolicyRequest, String> {
    if body.is_empty() {
        return Ok(SetRepositoryPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetRepositoryPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_image_scan_request(body: &[u8]) -> Result<StartImageScanRequest, String> {
    if body.is_empty() {
        return Ok(StartImageScanRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartImageScan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_lifecycle_policy_preview_request(
    body: &[u8],
) -> Result<StartLifecyclePolicyPreviewRequest, String> {
    if body.is_empty() {
        return Ok(StartLifecyclePolicyPreviewRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartLifecyclePolicyPreview request: {e}"))
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
pub fn deserialize_update_image_storage_class_request(
    body: &[u8],
) -> Result<UpdateImageStorageClassRequest, String> {
    if body.is_empty() {
        return Ok(UpdateImageStorageClassRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateImageStorageClass request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_pull_through_cache_rule_request(
    body: &[u8],
) -> Result<UpdatePullThroughCacheRuleRequest, String> {
    if body.is_empty() {
        return Ok(UpdatePullThroughCacheRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePullThroughCacheRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_repository_creation_template_request(
    body: &[u8],
) -> Result<UpdateRepositoryCreationTemplateRequest, String> {
    if body.is_empty() {
        return Ok(UpdateRepositoryCreationTemplateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRepositoryCreationTemplate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_upload_layer_part_request(
    body: &[u8],
) -> Result<UploadLayerPartRequest, String> {
    if body.is_empty() {
        return Ok(UploadLayerPartRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UploadLayerPart request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_validate_pull_through_cache_rule_request(
    body: &[u8],
) -> Result<ValidatePullThroughCacheRuleRequest, String> {
    if body.is_empty() {
        return Ok(ValidatePullThroughCacheRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ValidatePullThroughCacheRule request: {e}"))
}
