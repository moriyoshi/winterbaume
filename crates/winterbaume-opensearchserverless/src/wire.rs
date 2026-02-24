//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-opensearchserverless

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_collection_response(
    result: &BatchGetCollectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_collection_group_response(
    result: &BatchGetCollectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_effective_lifecycle_policy_response(
    result: &BatchGetEffectiveLifecyclePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_lifecycle_policy_response(
    result: &BatchGetLifecyclePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_get_vpc_endpoint_response(
    result: &BatchGetVpcEndpointResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_access_policy_response(
    result: &CreateAccessPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_collection_response(result: &CreateCollectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_collection_group_response(
    result: &CreateCollectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_index_response(result: &CreateIndexResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_lifecycle_policy_response(
    result: &CreateLifecyclePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_security_config_response(
    result: &CreateSecurityConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_security_policy_response(
    result: &CreateSecurityPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_vpc_endpoint_response(result: &CreateVpcEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_access_policy_response(
    result: &DeleteAccessPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_collection_response(result: &DeleteCollectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_collection_group_response(
    result: &DeleteCollectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_index_response(result: &DeleteIndexResponse) -> MockResponse {
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
pub fn serialize_delete_security_config_response(
    result: &DeleteSecurityConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_security_policy_response(
    result: &DeleteSecurityPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_vpc_endpoint_response(result: &DeleteVpcEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_access_policy_response(result: &GetAccessPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_account_settings_response(
    result: &GetAccountSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_index_response(result: &GetIndexResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_policies_stats_response(result: &GetPoliciesStatsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_security_config_response(result: &GetSecurityConfigResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_security_policy_response(result: &GetSecurityPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_access_policies_response(
    result: &ListAccessPoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_collection_groups_response(
    result: &ListCollectionGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_collections_response(result: &ListCollectionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_lifecycle_policies_response(
    result: &ListLifecyclePoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_security_configs_response(
    result: &ListSecurityConfigsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_security_policies_response(
    result: &ListSecurityPoliciesResponse,
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
pub fn serialize_list_vpc_endpoints_response(result: &ListVpcEndpointsResponse) -> MockResponse {
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
pub fn serialize_update_access_policy_response(
    result: &UpdateAccessPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_account_settings_response(
    result: &UpdateAccountSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_collection_response(result: &UpdateCollectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_collection_group_response(
    result: &UpdateCollectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_index_response(result: &UpdateIndexResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_lifecycle_policy_response(
    result: &UpdateLifecyclePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_security_config_response(
    result: &UpdateSecurityConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_security_policy_response(
    result: &UpdateSecurityPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_vpc_endpoint_response(result: &UpdateVpcEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_collection_request(
    body: &[u8],
) -> Result<BatchGetCollectionRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetCollectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetCollection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_collection_group_request(
    body: &[u8],
) -> Result<BatchGetCollectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetCollectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetCollectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_effective_lifecycle_policy_request(
    body: &[u8],
) -> Result<BatchGetEffectiveLifecyclePolicyRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetEffectiveLifecyclePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetEffectiveLifecyclePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_lifecycle_policy_request(
    body: &[u8],
) -> Result<BatchGetLifecyclePolicyRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetLifecyclePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetLifecyclePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_get_vpc_endpoint_request(
    body: &[u8],
) -> Result<BatchGetVpcEndpointRequest, String> {
    if body.is_empty() {
        return Ok(BatchGetVpcEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetVpcEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_access_policy_request(
    body: &[u8],
) -> Result<CreateAccessPolicyRequest, String> {
    if body.is_empty() {
        return Ok(CreateAccessPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAccessPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_collection_request(
    body: &[u8],
) -> Result<CreateCollectionRequest, String> {
    if body.is_empty() {
        return Ok(CreateCollectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCollection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_collection_group_request(
    body: &[u8],
) -> Result<CreateCollectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateCollectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCollectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_index_request(body: &[u8]) -> Result<CreateIndexRequest, String> {
    if body.is_empty() {
        return Ok(CreateIndexRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateIndex request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_lifecycle_policy_request(
    body: &[u8],
) -> Result<CreateLifecyclePolicyRequest, String> {
    if body.is_empty() {
        return Ok(CreateLifecyclePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLifecyclePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_security_config_request(
    body: &[u8],
) -> Result<CreateSecurityConfigRequest, String> {
    if body.is_empty() {
        return Ok(CreateSecurityConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSecurityConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_security_policy_request(
    body: &[u8],
) -> Result<CreateSecurityPolicyRequest, String> {
    if body.is_empty() {
        return Ok(CreateSecurityPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSecurityPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_vpc_endpoint_request(
    body: &[u8],
) -> Result<CreateVpcEndpointRequest, String> {
    if body.is_empty() {
        return Ok(CreateVpcEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateVpcEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_access_policy_request(
    body: &[u8],
) -> Result<DeleteAccessPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAccessPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAccessPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_collection_request(
    body: &[u8],
) -> Result<DeleteCollectionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCollectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCollection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_collection_group_request(
    body: &[u8],
) -> Result<DeleteCollectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCollectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCollectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_index_request(body: &[u8]) -> Result<DeleteIndexRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIndexRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIndex request: {e}"))
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
pub fn deserialize_delete_security_config_request(
    body: &[u8],
) -> Result<DeleteSecurityConfigRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSecurityConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSecurityConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_security_policy_request(
    body: &[u8],
) -> Result<DeleteSecurityPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSecurityPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSecurityPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_vpc_endpoint_request(
    body: &[u8],
) -> Result<DeleteVpcEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DeleteVpcEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteVpcEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_access_policy_request(
    body: &[u8],
) -> Result<GetAccessPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetAccessPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAccessPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_account_settings_request(
    body: &[u8],
) -> Result<GetAccountSettingsRequest, String> {
    if body.is_empty() {
        return Ok(GetAccountSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAccountSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_index_request(body: &[u8]) -> Result<GetIndexRequest, String> {
    if body.is_empty() {
        return Ok(GetIndexRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetIndex request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_policies_stats_request(
    body: &[u8],
) -> Result<GetPoliciesStatsRequest, String> {
    if body.is_empty() {
        return Ok(GetPoliciesStatsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPoliciesStats request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_security_config_request(
    body: &[u8],
) -> Result<GetSecurityConfigRequest, String> {
    if body.is_empty() {
        return Ok(GetSecurityConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSecurityConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_security_policy_request(
    body: &[u8],
) -> Result<GetSecurityPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetSecurityPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSecurityPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_access_policies_request(
    body: &[u8],
) -> Result<ListAccessPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListAccessPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAccessPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_collection_groups_request(
    body: &[u8],
) -> Result<ListCollectionGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListCollectionGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCollectionGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_collections_request(body: &[u8]) -> Result<ListCollectionsRequest, String> {
    if body.is_empty() {
        return Ok(ListCollectionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCollections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_lifecycle_policies_request(
    body: &[u8],
) -> Result<ListLifecyclePoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListLifecyclePoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLifecyclePolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_security_configs_request(
    body: &[u8],
) -> Result<ListSecurityConfigsRequest, String> {
    if body.is_empty() {
        return Ok(ListSecurityConfigsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSecurityConfigs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_security_policies_request(
    body: &[u8],
) -> Result<ListSecurityPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListSecurityPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSecurityPolicies request: {e}"))
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
pub fn deserialize_list_vpc_endpoints_request(
    body: &[u8],
) -> Result<ListVpcEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(ListVpcEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListVpcEndpoints request: {e}"))
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
pub fn deserialize_update_access_policy_request(
    body: &[u8],
) -> Result<UpdateAccessPolicyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAccessPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAccessPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_account_settings_request(
    body: &[u8],
) -> Result<UpdateAccountSettingsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAccountSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAccountSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_collection_request(
    body: &[u8],
) -> Result<UpdateCollectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCollectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCollection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_collection_group_request(
    body: &[u8],
) -> Result<UpdateCollectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCollectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCollectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_index_request(body: &[u8]) -> Result<UpdateIndexRequest, String> {
    if body.is_empty() {
        return Ok(UpdateIndexRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateIndex request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_lifecycle_policy_request(
    body: &[u8],
) -> Result<UpdateLifecyclePolicyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLifecyclePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLifecyclePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_security_config_request(
    body: &[u8],
) -> Result<UpdateSecurityConfigRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSecurityConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSecurityConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_security_policy_request(
    body: &[u8],
) -> Result<UpdateSecurityPolicyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSecurityPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSecurityPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_vpc_endpoint_request(
    body: &[u8],
) -> Result<UpdateVpcEndpointRequest, String> {
    if body.is_empty() {
        return Ok(UpdateVpcEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateVpcEndpoint request: {e}"))
}
