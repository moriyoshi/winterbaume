//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-wafv2

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_web_a_c_l_response(result: &AssociateWebACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_check_capacity_response(result: &CheckCapacityResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_a_p_i_key_response(result: &CreateAPIKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_i_p_set_response(result: &CreateIPSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_regex_pattern_set_response(
    result: &CreateRegexPatternSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_rule_group_response(result: &CreateRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_web_a_c_l_response(result: &CreateWebACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_a_p_i_key_response(result: &DeleteAPIKeyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_firewall_manager_rule_groups_response(
    result: &DeleteFirewallManagerRuleGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_i_p_set_response(result: &DeleteIPSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_logging_configuration_response(
    result: &DeleteLoggingConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_permission_policy_response(
    result: &DeletePermissionPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_regex_pattern_set_response(
    result: &DeleteRegexPatternSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_rule_group_response(result: &DeleteRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_web_a_c_l_response(result: &DeleteWebACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_all_managed_products_response(
    result: &DescribeAllManagedProductsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_managed_products_by_vendor_response(
    result: &DescribeManagedProductsByVendorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_managed_rule_group_response(
    result: &DescribeManagedRuleGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_web_a_c_l_response(
    result: &DisassociateWebACLResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_mobile_sdk_release_url_response(
    result: &GenerateMobileSdkReleaseUrlResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_decrypted_a_p_i_key_response(
    result: &GetDecryptedAPIKeyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_i_p_set_response(result: &GetIPSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_logging_configuration_response(
    result: &GetLoggingConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_managed_rule_set_response(result: &GetManagedRuleSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_mobile_sdk_release_response(
    result: &GetMobileSdkReleaseResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_permission_policy_response(
    result: &GetPermissionPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_rate_based_statement_managed_keys_response(
    result: &GetRateBasedStatementManagedKeysResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_regex_pattern_set_response(
    result: &GetRegexPatternSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_rule_group_response(result: &GetRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_sampled_requests_response(
    result: &GetSampledRequestsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_top_path_statistics_by_traffic_response(
    result: &GetTopPathStatisticsByTrafficResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_web_a_c_l_response(result: &GetWebACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_web_a_c_l_for_resource_response(
    result: &GetWebACLForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_a_p_i_keys_response(result: &ListAPIKeysResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_available_managed_rule_group_versions_response(
    result: &ListAvailableManagedRuleGroupVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_available_managed_rule_groups_response(
    result: &ListAvailableManagedRuleGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_i_p_sets_response(result: &ListIPSetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_logging_configurations_response(
    result: &ListLoggingConfigurationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_managed_rule_sets_response(
    result: &ListManagedRuleSetsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_mobile_sdk_releases_response(
    result: &ListMobileSdkReleasesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_regex_pattern_sets_response(
    result: &ListRegexPatternSetsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resources_for_web_a_c_l_response(
    result: &ListResourcesForWebACLResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_rule_groups_response(result: &ListRuleGroupsResponse) -> MockResponse {
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
pub fn serialize_list_web_a_c_ls_response(result: &ListWebACLsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_logging_configuration_response(
    result: &PutLoggingConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_managed_rule_set_versions_response(
    result: &PutManagedRuleSetVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_permission_policy_response(
    result: &PutPermissionPolicyResponse,
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
pub fn serialize_update_i_p_set_response(result: &UpdateIPSetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_managed_rule_set_version_expiry_date_response(
    result: &UpdateManagedRuleSetVersionExpiryDateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_regex_pattern_set_response(
    result: &UpdateRegexPatternSetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_rule_group_response(result: &UpdateRuleGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_web_a_c_l_response(result: &UpdateWebACLResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_web_a_c_l_request(
    body: &[u8],
) -> Result<AssociateWebACLRequest, String> {
    if body.is_empty() {
        return Ok(AssociateWebACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateWebACL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_check_capacity_request(body: &[u8]) -> Result<CheckCapacityRequest, String> {
    if body.is_empty() {
        return Ok(CheckCapacityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CheckCapacity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_a_p_i_key_request(body: &[u8]) -> Result<CreateAPIKeyRequest, String> {
    if body.is_empty() {
        return Ok(CreateAPIKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateAPIKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_i_p_set_request(body: &[u8]) -> Result<CreateIPSetRequest, String> {
    if body.is_empty() {
        return Ok(CreateIPSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateIPSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_regex_pattern_set_request(
    body: &[u8],
) -> Result<CreateRegexPatternSetRequest, String> {
    if body.is_empty() {
        return Ok(CreateRegexPatternSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRegexPatternSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_rule_group_request(
    body: &[u8],
) -> Result<CreateRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_web_a_c_l_request(body: &[u8]) -> Result<CreateWebACLRequest, String> {
    if body.is_empty() {
        return Ok(CreateWebACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateWebACL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_a_p_i_key_request(body: &[u8]) -> Result<DeleteAPIKeyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteAPIKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteAPIKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_firewall_manager_rule_groups_request(
    body: &[u8],
) -> Result<DeleteFirewallManagerRuleGroupsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFirewallManagerRuleGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFirewallManagerRuleGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_i_p_set_request(body: &[u8]) -> Result<DeleteIPSetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIPSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIPSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_logging_configuration_request(
    body: &[u8],
) -> Result<DeleteLoggingConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLoggingConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLoggingConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_permission_policy_request(
    body: &[u8],
) -> Result<DeletePermissionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeletePermissionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePermissionPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_regex_pattern_set_request(
    body: &[u8],
) -> Result<DeleteRegexPatternSetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRegexPatternSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRegexPatternSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_rule_group_request(
    body: &[u8],
) -> Result<DeleteRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_web_a_c_l_request(body: &[u8]) -> Result<DeleteWebACLRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWebACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWebACL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_all_managed_products_request(
    body: &[u8],
) -> Result<DescribeAllManagedProductsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAllManagedProductsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAllManagedProducts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_managed_products_by_vendor_request(
    body: &[u8],
) -> Result<DescribeManagedProductsByVendorRequest, String> {
    if body.is_empty() {
        return Ok(DescribeManagedProductsByVendorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeManagedProductsByVendor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_managed_rule_group_request(
    body: &[u8],
) -> Result<DescribeManagedRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(DescribeManagedRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeManagedRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_web_a_c_l_request(
    body: &[u8],
) -> Result<DisassociateWebACLRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateWebACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateWebACL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_mobile_sdk_release_url_request(
    body: &[u8],
) -> Result<GenerateMobileSdkReleaseUrlRequest, String> {
    if body.is_empty() {
        return Ok(GenerateMobileSdkReleaseUrlRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GenerateMobileSdkReleaseUrl request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_decrypted_a_p_i_key_request(
    body: &[u8],
) -> Result<GetDecryptedAPIKeyRequest, String> {
    if body.is_empty() {
        return Ok(GetDecryptedAPIKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDecryptedAPIKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_i_p_set_request(body: &[u8]) -> Result<GetIPSetRequest, String> {
    if body.is_empty() {
        return Ok(GetIPSetRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetIPSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_logging_configuration_request(
    body: &[u8],
) -> Result<GetLoggingConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetLoggingConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLoggingConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_managed_rule_set_request(
    body: &[u8],
) -> Result<GetManagedRuleSetRequest, String> {
    if body.is_empty() {
        return Ok(GetManagedRuleSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetManagedRuleSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_mobile_sdk_release_request(
    body: &[u8],
) -> Result<GetMobileSdkReleaseRequest, String> {
    if body.is_empty() {
        return Ok(GetMobileSdkReleaseRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMobileSdkRelease request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_permission_policy_request(
    body: &[u8],
) -> Result<GetPermissionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetPermissionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPermissionPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_rate_based_statement_managed_keys_request(
    body: &[u8],
) -> Result<GetRateBasedStatementManagedKeysRequest, String> {
    if body.is_empty() {
        return Ok(GetRateBasedStatementManagedKeysRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRateBasedStatementManagedKeys request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_regex_pattern_set_request(
    body: &[u8],
) -> Result<GetRegexPatternSetRequest, String> {
    if body.is_empty() {
        return Ok(GetRegexPatternSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRegexPatternSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_rule_group_request(body: &[u8]) -> Result<GetRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(GetRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_sampled_requests_request(
    body: &[u8],
) -> Result<GetSampledRequestsRequest, String> {
    if body.is_empty() {
        return Ok(GetSampledRequestsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSampledRequests request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_top_path_statistics_by_traffic_request(
    body: &[u8],
) -> Result<GetTopPathStatisticsByTrafficRequest, String> {
    if body.is_empty() {
        return Ok(GetTopPathStatisticsByTrafficRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTopPathStatisticsByTraffic request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_web_a_c_l_request(body: &[u8]) -> Result<GetWebACLRequest, String> {
    if body.is_empty() {
        return Ok(GetWebACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWebACL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_web_a_c_l_for_resource_request(
    body: &[u8],
) -> Result<GetWebACLForResourceRequest, String> {
    if body.is_empty() {
        return Ok(GetWebACLForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetWebACLForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_a_p_i_keys_request(body: &[u8]) -> Result<ListAPIKeysRequest, String> {
    if body.is_empty() {
        return Ok(ListAPIKeysRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAPIKeys request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_available_managed_rule_group_versions_request(
    body: &[u8],
) -> Result<ListAvailableManagedRuleGroupVersionsRequest, String> {
    if body.is_empty() {
        return Ok(ListAvailableManagedRuleGroupVersionsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListAvailableManagedRuleGroupVersions request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_available_managed_rule_groups_request(
    body: &[u8],
) -> Result<ListAvailableManagedRuleGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListAvailableManagedRuleGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAvailableManagedRuleGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_i_p_sets_request(body: &[u8]) -> Result<ListIPSetsRequest, String> {
    if body.is_empty() {
        return Ok(ListIPSetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListIPSets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_logging_configurations_request(
    body: &[u8],
) -> Result<ListLoggingConfigurationsRequest, String> {
    if body.is_empty() {
        return Ok(ListLoggingConfigurationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLoggingConfigurations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_managed_rule_sets_request(
    body: &[u8],
) -> Result<ListManagedRuleSetsRequest, String> {
    if body.is_empty() {
        return Ok(ListManagedRuleSetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListManagedRuleSets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_mobile_sdk_releases_request(
    body: &[u8],
) -> Result<ListMobileSdkReleasesRequest, String> {
    if body.is_empty() {
        return Ok(ListMobileSdkReleasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMobileSdkReleases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_regex_pattern_sets_request(
    body: &[u8],
) -> Result<ListRegexPatternSetsRequest, String> {
    if body.is_empty() {
        return Ok(ListRegexPatternSetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRegexPatternSets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resources_for_web_a_c_l_request(
    body: &[u8],
) -> Result<ListResourcesForWebACLRequest, String> {
    if body.is_empty() {
        return Ok(ListResourcesForWebACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourcesForWebACL request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_rule_groups_request(body: &[u8]) -> Result<ListRuleGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListRuleGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRuleGroups request: {e}"))
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
pub fn deserialize_list_web_a_c_ls_request(body: &[u8]) -> Result<ListWebACLsRequest, String> {
    if body.is_empty() {
        return Ok(ListWebACLsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWebACLs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_logging_configuration_request(
    body: &[u8],
) -> Result<PutLoggingConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutLoggingConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutLoggingConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_managed_rule_set_versions_request(
    body: &[u8],
) -> Result<PutManagedRuleSetVersionsRequest, String> {
    if body.is_empty() {
        return Ok(PutManagedRuleSetVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutManagedRuleSetVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_permission_policy_request(
    body: &[u8],
) -> Result<PutPermissionPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutPermissionPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutPermissionPolicy request: {e}"))
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
pub fn deserialize_update_i_p_set_request(body: &[u8]) -> Result<UpdateIPSetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateIPSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateIPSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_managed_rule_set_version_expiry_date_request(
    body: &[u8],
) -> Result<UpdateManagedRuleSetVersionExpiryDateRequest, String> {
    if body.is_empty() {
        return Ok(UpdateManagedRuleSetVersionExpiryDateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateManagedRuleSetVersionExpiryDate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_regex_pattern_set_request(
    body: &[u8],
) -> Result<UpdateRegexPatternSetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateRegexPatternSetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRegexPatternSet request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_rule_group_request(
    body: &[u8],
) -> Result<UpdateRuleGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateRuleGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateRuleGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_web_a_c_l_request(body: &[u8]) -> Result<UpdateWebACLRequest, String> {
    if body.is_empty() {
        return Ok(UpdateWebACLRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateWebACL request: {e}"))
}
