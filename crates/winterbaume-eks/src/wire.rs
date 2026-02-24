//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-eks

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
pub fn serialize_associate_access_policy_response(
    result: &AssociateAccessPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_encryption_config_response(
    result: &AssociateEncryptionConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_identity_provider_config_response(
    result: &AssociateIdentityProviderConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_access_entry_response(result: &CreateAccessEntryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_addon_response(result: &CreateAddonResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_capability_response(result: &CreateCapabilityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_eks_anywhere_subscription_response(
    result: &CreateEksAnywhereSubscriptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_fargate_profile_response(
    result: &CreateFargateProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_nodegroup_response(result: &CreateNodegroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_pod_identity_association_response(
    result: &CreatePodIdentityAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_access_entry_response(result: &DeleteAccessEntryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_addon_response(result: &DeleteAddonResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_capability_response(result: &DeleteCapabilityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_cluster_response(result: &DeleteClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_eks_anywhere_subscription_response(
    result: &DeleteEksAnywhereSubscriptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_fargate_profile_response(
    result: &DeleteFargateProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_nodegroup_response(result: &DeleteNodegroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_pod_identity_association_response(
    result: &DeletePodIdentityAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_cluster_response(result: &DeregisterClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_access_entry_response(
    result: &DescribeAccessEntryResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_addon_response(result: &DescribeAddonResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_addon_configuration_response(
    result: &DescribeAddonConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_addon_versions_response(
    result: &DescribeAddonVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_capability_response(result: &DescribeCapabilityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_cluster_response(result: &DescribeClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_cluster_versions_response(
    result: &DescribeClusterVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_eks_anywhere_subscription_response(
    result: &DescribeEksAnywhereSubscriptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_fargate_profile_response(
    result: &DescribeFargateProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_identity_provider_config_response(
    result: &DescribeIdentityProviderConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_insight_response(result: &DescribeInsightResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_insights_refresh_response(
    result: &DescribeInsightsRefreshResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_nodegroup_response(result: &DescribeNodegroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_pod_identity_association_response(
    result: &DescribePodIdentityAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_update_response(result: &DescribeUpdateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_access_policy_response(
    result: &DisassociateAccessPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_identity_provider_config_response(
    result: &DisassociateIdentityProviderConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_access_entries_response(result: &ListAccessEntriesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_access_policies_response(
    result: &ListAccessPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_addons_response(result: &ListAddonsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_associated_access_policies_response(
    result: &ListAssociatedAccessPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_capabilities_response(result: &ListCapabilitiesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_clusters_response(result: &ListClustersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_eks_anywhere_subscriptions_response(
    result: &ListEksAnywhereSubscriptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_fargate_profiles_response(
    result: &ListFargateProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_identity_provider_configs_response(
    result: &ListIdentityProviderConfigsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_insights_response(result: &ListInsightsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_nodegroups_response(result: &ListNodegroupsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_pod_identity_associations_response(
    result: &ListPodIdentityAssociationsResponse,
) -> MockResponse {
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

/// Serialize response for restJson protocol.
pub fn serialize_list_updates_response(result: &ListUpdatesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_cluster_response(result: &RegisterClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_insights_refresh_response(
    result: &StartInsightsRefreshResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_access_entry_response(result: &UpdateAccessEntryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_addon_response(result: &UpdateAddonResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_capability_response(result: &UpdateCapabilityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_cluster_config_response(
    result: &UpdateClusterConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_cluster_version_response(
    result: &UpdateClusterVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_eks_anywhere_subscription_response(
    result: &UpdateEksAnywhereSubscriptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_nodegroup_config_response(
    result: &UpdateNodegroupConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_nodegroup_version_response(
    result: &UpdateNodegroupVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_pod_identity_association_response(
    result: &UpdatePodIdentityAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_access_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAccessPolicyRequest, String> {
    let mut input = AssociateAccessPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateAccessPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateAccessPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "principalArn" => {
                input.principal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_encryption_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateEncryptionConfigRequest, String> {
    let mut input = AssociateEncryptionConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateEncryptionConfigRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateEncryptionConfig request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_identity_provider_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateIdentityProviderConfigRequest, String> {
    let mut input = AssociateIdentityProviderConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateIdentityProviderConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateIdentityProviderConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_access_entry_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessEntryRequest, String> {
    let mut input = CreateAccessEntryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAccessEntryRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAccessEntry request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_addon_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAddonRequest, String> {
    let mut input = CreateAddonRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAddonRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAddon request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_capability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCapabilityRequest, String> {
    let mut input = CreateCapabilityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCapabilityRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCapability request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterRequest, String> {
    let mut input = CreateClusterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateClusterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCluster request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_eks_anywhere_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEksAnywhereSubscriptionRequest, String> {
    let mut input = CreateEksAnywhereSubscriptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEksAnywhereSubscriptionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateEksAnywhereSubscription request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_fargate_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFargateProfileRequest, String> {
    let mut input = CreateFargateProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFargateProfileRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFargateProfile request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_nodegroup_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNodegroupRequest, String> {
    let mut input = CreateNodegroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNodegroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateNodegroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_pod_identity_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePodIdentityAssociationRequest, String> {
    let mut input = CreatePodIdentityAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePodIdentityAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreatePodIdentityAssociation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_access_entry_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessEntryRequest, String> {
    let mut input = DeleteAccessEntryRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "principalArn" => {
                input.principal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_addon_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAddonRequest, String> {
    let mut input = DeleteAddonRequest::default();
    for (name, value) in labels {
        match *name {
            "addonName" => {
                input.addon_name = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("preserve") {
        input.preserve = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_capability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCapabilityRequest, String> {
    let mut input = DeleteCapabilityRequest::default();
    for (name, value) in labels {
        match *name {
            "capabilityName" => {
                input.capability_name = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterRequest, String> {
    let mut input = DeleteClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_eks_anywhere_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEksAnywhereSubscriptionRequest, String> {
    let mut input = DeleteEksAnywhereSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_fargate_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFargateProfileRequest, String> {
    let mut input = DeleteFargateProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "fargateProfileName" => {
                input.fargate_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_nodegroup_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteNodegroupRequest, String> {
    let mut input = DeleteNodegroupRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "nodegroupName" => {
                input.nodegroup_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_pod_identity_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePodIdentityAssociationRequest, String> {
    let mut input = DeletePodIdentityAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterClusterRequest, String> {
    let mut input = DeregisterClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_access_entry_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccessEntryRequest, String> {
    let mut input = DescribeAccessEntryRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "principalArn" => {
                input.principal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_addon_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAddonRequest, String> {
    let mut input = DescribeAddonRequest::default();
    for (name, value) in labels {
        match *name {
            "addonName" => {
                input.addon_name = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_addon_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAddonConfigurationRequest, String> {
    let mut input = DescribeAddonConfigurationRequest::default();
    if let Some(value) = query.get("addonName") {
        input.addon_name = value.to_string();
    }
    if let Some(value) = query.get("addonVersion") {
        input.addon_version = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_addon_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAddonVersionsRequest, String> {
    let mut input = DescribeAddonVersionsRequest::default();
    if let Some(value) = query.get("addonName") {
        input.addon_name = Some(value.to_string());
    }
    if let Some(value) = query.get("kubernetesVersion") {
        input.kubernetes_version = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("owners") {
        input.owners = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("publishers") {
        input.publishers = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("types") {
        input.types = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_capability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCapabilityRequest, String> {
    let mut input = DescribeCapabilityRequest::default();
    for (name, value) in labels {
        match *name {
            "capabilityName" => {
                input.capability_name = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterRequest, String> {
    let mut input = DescribeClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_cluster_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterVersionsRequest, String> {
    let mut input = DescribeClusterVersionsRequest::default();
    if let Some(value) = query.get("clusterType") {
        input.cluster_type = Some(value.to_string());
    }
    if let Some(value) = query.get("clusterVersions") {
        input.cluster_versions = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("defaultOnly") {
        input.default_only = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("includeAll") {
        input.include_all = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    if let Some(value) = query.get("versionStatus") {
        input.version_status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_eks_anywhere_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeEksAnywhereSubscriptionRequest, String> {
    let mut input = DescribeEksAnywhereSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_fargate_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFargateProfileRequest, String> {
    let mut input = DescribeFargateProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "fargateProfileName" => {
                input.fargate_profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_identity_provider_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeIdentityProviderConfigRequest, String> {
    let mut input = DescribeIdentityProviderConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeIdentityProviderConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeIdentityProviderConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_insight_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInsightRequest, String> {
    let mut input = DescribeInsightRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_insights_refresh_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInsightsRefreshRequest, String> {
    let mut input = DescribeInsightsRefreshRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_nodegroup_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeNodegroupRequest, String> {
    let mut input = DescribeNodegroupRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "nodegroupName" => {
                input.nodegroup_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_pod_identity_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribePodIdentityAssociationRequest, String> {
    let mut input = DescribePodIdentityAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeUpdateRequest, String> {
    let mut input = DescribeUpdateRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "updateId" => {
                input.update_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("addonName") {
        input.addon_name = Some(value.to_string());
    }
    if let Some(value) = query.get("capabilityName") {
        input.capability_name = Some(value.to_string());
    }
    if let Some(value) = query.get("nodegroupName") {
        input.nodegroup_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_access_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateAccessPolicyRequest, String> {
    let mut input = DisassociateAccessPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            "principalArn" => {
                input.principal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_identity_provider_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateIdentityProviderConfigRequest, String> {
    let mut input = DisassociateIdentityProviderConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateIdentityProviderConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DisassociateIdentityProviderConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_access_entries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessEntriesRequest, String> {
    let mut input = ListAccessEntriesRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("associatedPolicyArn") {
        input.associated_policy_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_access_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAccessPoliciesRequest, String> {
    let mut input = ListAccessPoliciesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_addons_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAddonsRequest, String> {
    let mut input = ListAddonsRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_associated_access_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssociatedAccessPoliciesRequest, String> {
    let mut input = ListAssociatedAccessPoliciesRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "principalArn" => {
                input.principal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_capabilities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCapabilitiesRequest, String> {
    let mut input = ListCapabilitiesRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_clusters_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClustersRequest, String> {
    let mut input = ListClustersRequest::default();
    if let Some(value) = query.get("include") {
        input.include = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_eks_anywhere_subscriptions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEksAnywhereSubscriptionsRequest, String> {
    let mut input = ListEksAnywhereSubscriptionsRequest::default();
    if let Some(value) = query.get("includeStatus") {
        input.include_status = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_fargate_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFargateProfilesRequest, String> {
    let mut input = ListFargateProfilesRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_identity_provider_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIdentityProviderConfigsRequest, String> {
    let mut input = ListIdentityProviderConfigsRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_insights_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInsightsRequest, String> {
    let mut input = ListInsightsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListInsightsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListInsights request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_nodegroups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNodegroupsRequest, String> {
    let mut input = ListNodegroupsRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_pod_identity_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPodIdentityAssociationsRequest, String> {
    let mut input = ListPodIdentityAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("serviceAccount") {
        input.service_account = Some(value.to_string());
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
pub fn deserialize_list_updates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUpdatesRequest, String> {
    let mut input = ListUpdatesRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("addonName") {
        input.addon_name = Some(value.to_string());
    }
    if let Some(value) = query.get("capabilityName") {
        input.capability_name = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("nodegroupName") {
        input.nodegroup_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterClusterRequest, String> {
    let mut input = RegisterClusterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterClusterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterCluster request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_insights_refresh_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartInsightsRefreshRequest, String> {
    let mut input = StartInsightsRefreshRequest::default();
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
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
pub fn deserialize_update_access_entry_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccessEntryRequest, String> {
    let mut input = UpdateAccessEntryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccessEntryRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAccessEntry request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "principalArn" => {
                input.principal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_addon_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAddonRequest, String> {
    let mut input = UpdateAddonRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAddonRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAddon request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "addonName" => {
                input.addon_name = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_capability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCapabilityRequest, String> {
    let mut input = UpdateCapabilityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCapabilityRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCapability request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "capabilityName" => {
                input.capability_name = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_cluster_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClusterConfigRequest, String> {
    let mut input = UpdateClusterConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClusterConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateClusterConfig request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_cluster_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClusterVersionRequest, String> {
    let mut input = UpdateClusterVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClusterVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateClusterVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_eks_anywhere_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEksAnywhereSubscriptionRequest, String> {
    let mut input = UpdateEksAnywhereSubscriptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEksAnywhereSubscriptionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateEksAnywhereSubscription request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_nodegroup_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNodegroupConfigRequest, String> {
    let mut input = UpdateNodegroupConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNodegroupConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateNodegroupConfig request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "nodegroupName" => {
                input.nodegroup_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_nodegroup_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNodegroupVersionRequest, String> {
    let mut input = UpdateNodegroupVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNodegroupVersionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateNodegroupVersion request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            "nodegroupName" => {
                input.nodegroup_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_pod_identity_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePodIdentityAssociationRequest, String> {
    let mut input = UpdatePodIdentityAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePodIdentityAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdatePodIdentityAssociation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "clusterName" => {
                input.cluster_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
