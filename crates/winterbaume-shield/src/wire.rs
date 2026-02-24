//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-shield

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_d_r_t_log_bucket_response(
    result: &AssociateDRTLogBucketResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_d_r_t_role_response(result: &AssociateDRTRoleResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_health_check_response(
    result: &AssociateHealthCheckResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_proactive_engagement_details_response(
    result: &AssociateProactiveEngagementDetailsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_protection_response(result: &CreateProtectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_protection_group_response(
    result: &CreateProtectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_subscription_response(result: &CreateSubscriptionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_protection_response(result: &DeleteProtectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_protection_group_response(
    result: &DeleteProtectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_subscription_response(result: &DeleteSubscriptionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_attack_response(result: &DescribeAttackResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_attack_statistics_response(
    result: &DescribeAttackStatisticsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_d_r_t_access_response(
    result: &DescribeDRTAccessResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_emergency_contact_settings_response(
    result: &DescribeEmergencyContactSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_protection_response(result: &DescribeProtectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_protection_group_response(
    result: &DescribeProtectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_subscription_response(
    result: &DescribeSubscriptionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_application_layer_automatic_response_response(
    result: &DisableApplicationLayerAutomaticResponseResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_proactive_engagement_response(
    result: &DisableProactiveEngagementResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_d_r_t_log_bucket_response(
    result: &DisassociateDRTLogBucketResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_d_r_t_role_response(
    result: &DisassociateDRTRoleResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_health_check_response(
    result: &DisassociateHealthCheckResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_application_layer_automatic_response_response(
    result: &EnableApplicationLayerAutomaticResponseResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_proactive_engagement_response(
    result: &EnableProactiveEngagementResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_subscription_state_response(
    result: &GetSubscriptionStateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_attacks_response(result: &ListAttacksResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_protection_groups_response(
    result: &ListProtectionGroupsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_protections_response(result: &ListProtectionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resources_in_protection_group_response(
    result: &ListResourcesInProtectionGroupResponse,
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
pub fn serialize_update_application_layer_automatic_response_response(
    result: &UpdateApplicationLayerAutomaticResponseResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_emergency_contact_settings_response(
    result: &UpdateEmergencyContactSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_protection_group_response(
    result: &UpdateProtectionGroupResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_subscription_response(result: &UpdateSubscriptionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_d_r_t_log_bucket_request(
    body: &[u8],
) -> Result<AssociateDRTLogBucketRequest, String> {
    if body.is_empty() {
        return Ok(AssociateDRTLogBucketRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateDRTLogBucket request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_d_r_t_role_request(
    body: &[u8],
) -> Result<AssociateDRTRoleRequest, String> {
    if body.is_empty() {
        return Ok(AssociateDRTRoleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateDRTRole request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_health_check_request(
    body: &[u8],
) -> Result<AssociateHealthCheckRequest, String> {
    if body.is_empty() {
        return Ok(AssociateHealthCheckRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateHealthCheck request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_proactive_engagement_details_request(
    body: &[u8],
) -> Result<AssociateProactiveEngagementDetailsRequest, String> {
    if body.is_empty() {
        return Ok(AssociateProactiveEngagementDetailsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AssociateProactiveEngagementDetails request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_protection_request(
    body: &[u8],
) -> Result<CreateProtectionRequest, String> {
    if body.is_empty() {
        return Ok(CreateProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_protection_group_request(
    body: &[u8],
) -> Result<CreateProtectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateProtectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProtectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_subscription_request(
    body: &[u8],
) -> Result<CreateSubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(CreateSubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_protection_request(
    body: &[u8],
) -> Result<DeleteProtectionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_protection_group_request(
    body: &[u8],
) -> Result<DeleteProtectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProtectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProtectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_subscription_request(
    body: &[u8],
) -> Result<DeleteSubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_attack_request(body: &[u8]) -> Result<DescribeAttackRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAttackRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAttack request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_attack_statistics_request(
    body: &[u8],
) -> Result<DescribeAttackStatisticsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeAttackStatisticsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAttackStatistics request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_d_r_t_access_request(
    body: &[u8],
) -> Result<DescribeDRTAccessRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDRTAccessRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDRTAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_emergency_contact_settings_request(
    body: &[u8],
) -> Result<DescribeEmergencyContactSettingsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEmergencyContactSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEmergencyContactSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_protection_request(
    body: &[u8],
) -> Result<DescribeProtectionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProtectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProtection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_protection_group_request(
    body: &[u8],
) -> Result<DescribeProtectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProtectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProtectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_subscription_request(
    body: &[u8],
) -> Result<DescribeSubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSubscription request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_application_layer_automatic_response_request(
    body: &[u8],
) -> Result<DisableApplicationLayerAutomaticResponseRequest, String> {
    if body.is_empty() {
        return Ok(DisableApplicationLayerAutomaticResponseRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisableApplicationLayerAutomaticResponse request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_proactive_engagement_request(
    body: &[u8],
) -> Result<DisableProactiveEngagementRequest, String> {
    if body.is_empty() {
        return Ok(DisableProactiveEngagementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableProactiveEngagement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_d_r_t_log_bucket_request(
    body: &[u8],
) -> Result<DisassociateDRTLogBucketRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateDRTLogBucketRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateDRTLogBucket request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_d_r_t_role_request(
    body: &[u8],
) -> Result<DisassociateDRTRoleRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateDRTRoleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateDRTRole request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_health_check_request(
    body: &[u8],
) -> Result<DisassociateHealthCheckRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateHealthCheckRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateHealthCheck request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_application_layer_automatic_response_request(
    body: &[u8],
) -> Result<EnableApplicationLayerAutomaticResponseRequest, String> {
    if body.is_empty() {
        return Ok(EnableApplicationLayerAutomaticResponseRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize EnableApplicationLayerAutomaticResponse request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_proactive_engagement_request(
    body: &[u8],
) -> Result<EnableProactiveEngagementRequest, String> {
    if body.is_empty() {
        return Ok(EnableProactiveEngagementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableProactiveEngagement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_subscription_state_request(
    body: &[u8],
) -> Result<GetSubscriptionStateRequest, String> {
    if body.is_empty() {
        return Ok(GetSubscriptionStateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSubscriptionState request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_attacks_request(body: &[u8]) -> Result<ListAttacksRequest, String> {
    if body.is_empty() {
        return Ok(ListAttacksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAttacks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_protection_groups_request(
    body: &[u8],
) -> Result<ListProtectionGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListProtectionGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProtectionGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_protections_request(body: &[u8]) -> Result<ListProtectionsRequest, String> {
    if body.is_empty() {
        return Ok(ListProtectionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProtections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resources_in_protection_group_request(
    body: &[u8],
) -> Result<ListResourcesInProtectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(ListResourcesInProtectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourcesInProtectionGroup request: {e}"))
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
pub fn deserialize_update_application_layer_automatic_response_request(
    body: &[u8],
) -> Result<UpdateApplicationLayerAutomaticResponseRequest, String> {
    if body.is_empty() {
        return Ok(UpdateApplicationLayerAutomaticResponseRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateApplicationLayerAutomaticResponse request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_emergency_contact_settings_request(
    body: &[u8],
) -> Result<UpdateEmergencyContactSettingsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateEmergencyContactSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateEmergencyContactSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_protection_group_request(
    body: &[u8],
) -> Result<UpdateProtectionGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateProtectionGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProtectionGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_subscription_request(
    body: &[u8],
) -> Result<UpdateSubscriptionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSubscriptionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSubscription request: {e}"))
}
