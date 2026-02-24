//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudfront

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for restXml protocol.
pub fn serialize_associate_alias_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_associate_distribution_tenant_web_a_c_l_response(
    result: &AssociateDistributionTenantWebACLResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_associate_distribution_web_a_c_l_response(
    result: &AssociateDistributionWebACLResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_copy_distribution_response(result: &Distribution) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_anycast_ip_list_response(result: &AnycastIpList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(202, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_cache_policy_response(result: &CachePolicy) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_cloud_front_origin_access_identity_response(
    result: &CloudFrontOriginAccessIdentity,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_connection_function_response(
    result: &ConnectionFunctionSummary,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_connection_group_response(result: &ConnectionGroup) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_continuous_deployment_policy_response(
    result: &ContinuousDeploymentPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_distribution_response(result: &Distribution) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_distribution_tenant_response(result: &DistributionTenant) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_distribution_with_tags_response(result: &Distribution) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_field_level_encryption_config_response(
    result: &FieldLevelEncryption,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_field_level_encryption_profile_response(
    result: &FieldLevelEncryptionProfile,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_function_response(result: &FunctionSummary) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_invalidation_response(result: &Invalidation) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_invalidation_for_distribution_tenant_response(
    result: &Invalidation,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_key_group_response(result: &KeyGroup) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_key_value_store_response(result: &KeyValueStore) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_monitoring_subscription_response(
    result: &MonitoringSubscription,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_origin_access_control_response(
    result: &OriginAccessControl,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_origin_request_policy_response(
    result: &OriginRequestPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_public_key_response(result: &PublicKey) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_realtime_log_config_response(
    result: &CreateRealtimeLogConfigResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(201, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_response_headers_policy_response(
    result: &ResponseHeadersPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_streaming_distribution_response(
    result: &StreamingDistribution,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_streaming_distribution_with_tags_response(
    result: &StreamingDistribution,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_trust_store_response(result: &TrustStore) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_vpc_origin_response(result: &VpcOrigin) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(202, body);
    // Header "etag": set by caller from e_tag field
    // Header "location": set by caller from location field
    resp
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_anycast_ip_list_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_cache_policy_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_cloud_front_origin_access_identity_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_connection_function_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_connection_group_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_continuous_deployment_policy_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_distribution_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_distribution_tenant_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_field_level_encryption_config_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_field_level_encryption_profile_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_function_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_key_group_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_key_value_store_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_monitoring_subscription_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_origin_access_control_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_origin_request_policy_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_public_key_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_realtime_log_config_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_resource_policy_response() -> MockResponse {
    MockResponse::xml(200, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_response_headers_policy_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_streaming_distribution_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize void response for restXml protocol.
pub fn serialize_delete_trust_store_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_vpc_origin_response(result: &VpcOrigin) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(202, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_describe_connection_function_response(
    result: &ConnectionFunctionSummary,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_describe_function_response(result: &FunctionSummary) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_describe_key_value_store_response(result: &KeyValueStore) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_disassociate_distribution_tenant_web_a_c_l_response(
    result: &DisassociateDistributionTenantWebACLResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_disassociate_distribution_web_a_c_l_response(
    result: &DisassociateDistributionWebACLResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_anycast_ip_list_response(result: &AnycastIpList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_cache_policy_response(result: &CachePolicy) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_cache_policy_config_response(result: &CachePolicyConfig) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_cloud_front_origin_access_identity_response(
    result: &CloudFrontOriginAccessIdentity,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_cloud_front_origin_access_identity_config_response(
    result: &CloudFrontOriginAccessIdentityConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_connection_function_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "content-type": set by caller from content_type field
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_connection_group_response(result: &ConnectionGroup) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_connection_group_by_routing_endpoint_response(
    result: &ConnectionGroup,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_continuous_deployment_policy_response(
    result: &ContinuousDeploymentPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_continuous_deployment_policy_config_response(
    result: &ContinuousDeploymentPolicyConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_distribution_response(result: &Distribution) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_distribution_config_response(result: &DistributionConfig) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_distribution_tenant_response(result: &DistributionTenant) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_distribution_tenant_by_domain_response(
    result: &DistributionTenant,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_field_level_encryption_response(
    result: &FieldLevelEncryption,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_field_level_encryption_config_response(
    result: &FieldLevelEncryptionConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_field_level_encryption_profile_response(
    result: &FieldLevelEncryptionProfile,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_field_level_encryption_profile_config_response(
    result: &FieldLevelEncryptionProfileConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_function_response() -> MockResponse {
    let body = String::new();
    let resp = MockResponse::xml(200, body);
    // Header "content-type": set by caller from content_type field
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_invalidation_response(result: &Invalidation) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_invalidation_for_distribution_tenant_response(
    result: &Invalidation,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_key_group_response(result: &KeyGroup) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_key_group_config_response(result: &KeyGroupConfig) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_managed_certificate_details_response(
    result: &ManagedCertificateDetails,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_monitoring_subscription_response(
    result: &MonitoringSubscription,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_origin_access_control_response(result: &OriginAccessControl) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_origin_access_control_config_response(
    result: &OriginAccessControlConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_origin_request_policy_response(result: &OriginRequestPolicy) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_origin_request_policy_config_response(
    result: &OriginRequestPolicyConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_public_key_response(result: &PublicKey) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_public_key_config_response(result: &PublicKeyConfig) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_realtime_log_config_response(
    result: &GetRealtimeLogConfigResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_response_headers_policy_response(
    result: &ResponseHeadersPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_response_headers_policy_config_response(
    result: &ResponseHeadersPolicyConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_streaming_distribution_response(
    result: &StreamingDistribution,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_streaming_distribution_config_response(
    result: &StreamingDistributionConfig,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_trust_store_response(result: &TrustStore) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_get_vpc_origin_response(result: &VpcOrigin) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_list_anycast_ip_lists_response(result: &AnycastIpListCollection) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_cache_policies_response(result: &CachePolicyList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_cloud_front_origin_access_identities_response(
    result: &CloudFrontOriginAccessIdentityList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_conflicting_aliases_response(
    result: &ConflictingAliasesList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_connection_functions_response(
    result: &ListConnectionFunctionsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_connection_groups_response(
    result: &ListConnectionGroupsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_continuous_deployment_policies_response(
    result: &ContinuousDeploymentPolicyList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distribution_tenants_response(
    result: &ListDistributionTenantsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distribution_tenants_by_customization_response(
    result: &ListDistributionTenantsByCustomizationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_response(result: &DistributionList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_anycast_ip_list_id_response(
    result: &DistributionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_cache_policy_id_response(
    result: &DistributionIdList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_connection_function_response(
    result: &DistributionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_connection_mode_response(
    result: &DistributionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_key_group_response(
    result: &DistributionIdList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_origin_request_policy_id_response(
    result: &DistributionIdList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_owned_resource_response(
    result: &DistributionIdOwnerList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_realtime_log_config_response(
    result: &DistributionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_response_headers_policy_id_response(
    result: &DistributionIdList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_trust_store_response(
    result: &DistributionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_vpc_origin_id_response(
    result: &DistributionIdList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_distributions_by_web_a_c_l_id_response(
    result: &DistributionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_domain_conflicts_response(
    result: &ListDomainConflictsResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_field_level_encryption_configs_response(
    result: &FieldLevelEncryptionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_field_level_encryption_profiles_response(
    result: &FieldLevelEncryptionProfileList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_functions_response(result: &FunctionList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_invalidations_response(result: &InvalidationList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_invalidations_for_distribution_tenant_response(
    result: &InvalidationList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_key_groups_response(result: &KeyGroupList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_key_value_stores_response(result: &KeyValueStoreList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_origin_access_controls_response(
    result: &OriginAccessControlList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_origin_request_policies_response(
    result: &OriginRequestPolicyList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_public_keys_response(result: &PublicKeyList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_realtime_log_configs_response(result: &RealtimeLogConfigs) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_response_headers_policies_response(
    result: &ResponseHeadersPolicyList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_streaming_distributions_response(
    result: &StreamingDistributionList,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_tags_for_resource_response(result: &Tags) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_trust_stores_response(result: &ListTrustStoresResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_vpc_origins_response(result: &VpcOriginList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_publish_connection_function_response(
    result: &ConnectionFunctionSummary,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_publish_function_response(result: &FunctionSummary) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_test_connection_function_response(
    result: &ConnectionFunctionTestResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_test_function_response(result: &TestResult) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize void response for restXml protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::xml(204, "")
}

/// Serialize response for restXml protocol.
pub fn serialize_update_anycast_ip_list_response(result: &AnycastIpList) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_cache_policy_response(result: &CachePolicy) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_cloud_front_origin_access_identity_response(
    result: &CloudFrontOriginAccessIdentity,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_connection_function_response(
    result: &ConnectionFunctionSummary,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_connection_group_response(result: &ConnectionGroup) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_continuous_deployment_policy_response(
    result: &ContinuousDeploymentPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_distribution_response(result: &Distribution) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_distribution_tenant_response(result: &DistributionTenant) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_distribution_with_staging_config_response(
    result: &Distribution,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_domain_association_response(
    result: &UpdateDomainAssociationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_field_level_encryption_config_response(
    result: &FieldLevelEncryption,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_field_level_encryption_profile_response(
    result: &FieldLevelEncryptionProfile,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_function_response(result: &FunctionSummary) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "ettag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_key_group_response(result: &KeyGroup) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_key_value_store_response(result: &KeyValueStore) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_origin_access_control_response(
    result: &OriginAccessControl,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_origin_request_policy_response(
    result: &OriginRequestPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_public_key_response(result: &PublicKey) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_realtime_log_config_response(
    result: &UpdateRealtimeLogConfigResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_response_headers_policy_response(
    result: &ResponseHeadersPolicy,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_streaming_distribution_response(
    result: &StreamingDistribution,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_trust_store_response(result: &TrustStore) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(200, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_update_vpc_origin_response(result: &VpcOrigin) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(202, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_verify_dns_configuration_response(
    result: &VerifyDnsConfigurationResult,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_associate_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAliasRequest, String> {
    let mut input = AssociateAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "TargetDistributionId" => {
                input.target_distribution_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Alias") {
        input.alias = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_associate_distribution_tenant_web_a_c_l_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateDistributionTenantWebACLRequest, String> {
    let mut input = AssociateDistributionTenantWebACLRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<AssociateDistributionTenantWebACLRequest>(body).map_err(
            |err| format!("failed to deserialize AssociateDistributionTenantWebACL request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_associate_distribution_web_a_c_l_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateDistributionWebACLRequest, String> {
    let mut input = AssociateDistributionWebACLRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<AssociateDistributionWebACLRequest>(body).map_err(|err| {
                format!("failed to deserialize AssociateDistributionWebACL request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_copy_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CopyDistributionRequest, String> {
    let mut input = CopyDistributionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CopyDistributionRequest>(body)
            .map_err(|err| format!("failed to deserialize CopyDistribution request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "PrimaryDistributionId" => {
                input.primary_distribution_id = value.to_string();
            }
            _ => {}
        }
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
        .get("Staging")
        .and_then(|value| value.to_str().ok())
    {
        input.staging = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_anycast_ip_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAnycastIpListRequest, String> {
    let mut input = CreateAnycastIpListRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateAnycastIpListRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateAnycastIpList request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_cache_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCachePolicyRequest, String> {
    let mut input = CreateCachePolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CachePolicyConfig>(body)
            .map_err(|err| format!("failed to deserialize CreateCachePolicy payload: {err}"))?;
        input.cache_policy_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_cloud_front_origin_access_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCloudFrontOriginAccessIdentityRequest, String> {
    let mut input = CreateCloudFrontOriginAccessIdentityRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CloudFrontOriginAccessIdentityConfig>(body)
            .map_err(|err| {
                format!("failed to deserialize CreateCloudFrontOriginAccessIdentity payload: {err}")
            })?;
        input.cloud_front_origin_access_identity_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectionFunctionRequest, String> {
    let mut input = CreateConnectionFunctionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateConnectionFunctionRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateConnectionFunction request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_connection_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectionGroupRequest, String> {
    let mut input = CreateConnectionGroupRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateConnectionGroupRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateConnectionGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_continuous_deployment_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContinuousDeploymentPolicyRequest, String> {
    let mut input = CreateContinuousDeploymentPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<ContinuousDeploymentPolicyConfig>(body).map_err(|err| {
                format!("failed to deserialize CreateContinuousDeploymentPolicy payload: {err}")
            })?;
        input.continuous_deployment_policy_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDistributionRequest, String> {
    let mut input = CreateDistributionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<DistributionConfig>(body)
            .map_err(|err| format!("failed to deserialize CreateDistribution payload: {err}"))?;
        input.distribution_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_distribution_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDistributionTenantRequest, String> {
    let mut input = CreateDistributionTenantRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateDistributionTenantRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateDistributionTenant request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_distribution_with_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDistributionWithTagsRequest, String> {
    let mut input = CreateDistributionWithTagsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<DistributionConfigWithTags>(body).map_err(|err| {
                format!("failed to deserialize CreateDistributionWithTags payload: {err}")
            })?;
        input.distribution_config_with_tags = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_field_level_encryption_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFieldLevelEncryptionConfigRequest, String> {
    let mut input = CreateFieldLevelEncryptionConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<FieldLevelEncryptionConfig>(body).map_err(|err| {
                format!("failed to deserialize CreateFieldLevelEncryptionConfig payload: {err}")
            })?;
        input.field_level_encryption_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_field_level_encryption_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFieldLevelEncryptionProfileRequest, String> {
    let mut input = CreateFieldLevelEncryptionProfileRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<FieldLevelEncryptionProfileConfig>(body).map_err(|err| {
                format!("failed to deserialize CreateFieldLevelEncryptionProfile payload: {err}")
            })?;
        input.field_level_encryption_profile_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFunctionRequest, String> {
    let mut input = CreateFunctionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateFunctionRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateFunction request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_invalidation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInvalidationRequest, String> {
    let mut input = CreateInvalidationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<InvalidationBatch>(body)
            .map_err(|err| format!("failed to deserialize CreateInvalidation payload: {err}"))?;
        input.invalidation_batch = payload;
    }
    for (name, value) in labels {
        match *name {
            "DistributionId" => {
                input.distribution_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_invalidation_for_distribution_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInvalidationForDistributionTenantRequest, String> {
    let mut input = CreateInvalidationForDistributionTenantRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<InvalidationBatch>(body).map_err(|err| {
            format!("failed to deserialize CreateInvalidationForDistributionTenant payload: {err}")
        })?;
        input.invalidation_batch = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_key_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateKeyGroupRequest, String> {
    let mut input = CreateKeyGroupRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<KeyGroupConfig>(body)
            .map_err(|err| format!("failed to deserialize CreateKeyGroup payload: {err}"))?;
        input.key_group_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_key_value_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateKeyValueStoreRequest, String> {
    let mut input = CreateKeyValueStoreRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateKeyValueStoreRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateKeyValueStore request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_monitoring_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMonitoringSubscriptionRequest, String> {
    let mut input = CreateMonitoringSubscriptionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<MonitoringSubscription>(body).map_err(|err| {
            format!("failed to deserialize CreateMonitoringSubscription payload: {err}")
        })?;
        input.monitoring_subscription = payload;
    }
    for (name, value) in labels {
        match *name {
            "DistributionId" => {
                input.distribution_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_origin_access_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateOriginAccessControlRequest, String> {
    let mut input = CreateOriginAccessControlRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<OriginAccessControlConfig>(body).map_err(|err| {
                format!("failed to deserialize CreateOriginAccessControl payload: {err}")
            })?;
        input.origin_access_control_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_origin_request_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateOriginRequestPolicyRequest, String> {
    let mut input = CreateOriginRequestPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<OriginRequestPolicyConfig>(body).map_err(|err| {
                format!("failed to deserialize CreateOriginRequestPolicy payload: {err}")
            })?;
        input.origin_request_policy_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_public_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePublicKeyRequest, String> {
    let mut input = CreatePublicKeyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<PublicKeyConfig>(body)
            .map_err(|err| format!("failed to deserialize CreatePublicKey payload: {err}"))?;
        input.public_key_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_realtime_log_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRealtimeLogConfigRequest, String> {
    let mut input = CreateRealtimeLogConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateRealtimeLogConfigRequest>(body).map_err(|err| {
            format!("failed to deserialize CreateRealtimeLogConfig request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_response_headers_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResponseHeadersPolicyRequest, String> {
    let mut input = CreateResponseHeadersPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<ResponseHeadersPolicyConfig>(body).map_err(|err| {
                format!("failed to deserialize CreateResponseHeadersPolicy payload: {err}")
            })?;
        input.response_headers_policy_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_streaming_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateStreamingDistributionRequest, String> {
    let mut input = CreateStreamingDistributionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<StreamingDistributionConfig>(body).map_err(|err| {
                format!("failed to deserialize CreateStreamingDistribution payload: {err}")
            })?;
        input.streaming_distribution_config = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_streaming_distribution_with_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateStreamingDistributionWithTagsRequest, String> {
    let mut input = CreateStreamingDistributionWithTagsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<StreamingDistributionConfigWithTags>(body)
            .map_err(|err| {
                format!("failed to deserialize CreateStreamingDistributionWithTags payload: {err}")
            })?;
        input.streaming_distribution_config_with_tags = payload;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTrustStoreRequest, String> {
    let mut input = CreateTrustStoreRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateTrustStoreRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateTrustStore request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_vpc_origin_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVpcOriginRequest, String> {
    let mut input = CreateVpcOriginRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateVpcOriginRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateVpcOrigin request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_anycast_ip_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAnycastIpListRequest, String> {
    let mut input = DeleteAnycastIpListRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_cache_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCachePolicyRequest, String> {
    let mut input = DeleteCachePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_cloud_front_origin_access_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCloudFrontOriginAccessIdentityRequest, String> {
    let mut input = DeleteCloudFrontOriginAccessIdentityRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectionFunctionRequest, String> {
    let mut input = DeleteConnectionFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_connection_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectionGroupRequest, String> {
    let mut input = DeleteConnectionGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_continuous_deployment_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContinuousDeploymentPolicyRequest, String> {
    let mut input = DeleteContinuousDeploymentPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDistributionRequest, String> {
    let mut input = DeleteDistributionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_distribution_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDistributionTenantRequest, String> {
    let mut input = DeleteDistributionTenantRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_field_level_encryption_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFieldLevelEncryptionConfigRequest, String> {
    let mut input = DeleteFieldLevelEncryptionConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_field_level_encryption_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFieldLevelEncryptionProfileRequest, String> {
    let mut input = DeleteFieldLevelEncryptionProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionRequest, String> {
    let mut input = DeleteFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_key_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteKeyGroupRequest, String> {
    let mut input = DeleteKeyGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_key_value_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteKeyValueStoreRequest, String> {
    let mut input = DeleteKeyValueStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_monitoring_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMonitoringSubscriptionRequest, String> {
    let mut input = DeleteMonitoringSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "DistributionId" => {
                input.distribution_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_origin_access_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteOriginAccessControlRequest, String> {
    let mut input = DeleteOriginAccessControlRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_origin_request_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteOriginRequestPolicyRequest, String> {
    let mut input = DeleteOriginRequestPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_public_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePublicKeyRequest, String> {
    let mut input = DeletePublicKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_realtime_log_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRealtimeLogConfigRequest, String> {
    let mut input = DeleteRealtimeLogConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<DeleteRealtimeLogConfigRequest>(body).map_err(|err| {
            format!("failed to deserialize DeleteRealtimeLogConfig request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyRequest, String> {
    let mut input = DeleteResourcePolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<DeleteResourcePolicyRequest>(body)
            .map_err(|err| format!("failed to deserialize DeleteResourcePolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_response_headers_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResponseHeadersPolicyRequest, String> {
    let mut input = DeleteResponseHeadersPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_streaming_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStreamingDistributionRequest, String> {
    let mut input = DeleteStreamingDistributionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTrustStoreRequest, String> {
    let mut input = DeleteTrustStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_vpc_origin_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVpcOriginRequest, String> {
    let mut input = DeleteVpcOriginRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_describe_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConnectionFunctionRequest, String> {
    let mut input = DescribeConnectionFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Stage") {
        input.stage = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_describe_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFunctionRequest, String> {
    let mut input = DescribeFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Stage") {
        input.stage = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_describe_key_value_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeKeyValueStoreRequest, String> {
    let mut input = DescribeKeyValueStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_disassociate_distribution_tenant_web_a_c_l_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateDistributionTenantWebACLRequest, String> {
    let mut input = DisassociateDistributionTenantWebACLRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_disassociate_distribution_web_a_c_l_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateDistributionWebACLRequest, String> {
    let mut input = DisassociateDistributionWebACLRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_anycast_ip_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAnycastIpListRequest, String> {
    let mut input = GetAnycastIpListRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_cache_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCachePolicyRequest, String> {
    let mut input = GetCachePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_cache_policy_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCachePolicyConfigRequest, String> {
    let mut input = GetCachePolicyConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_cloud_front_origin_access_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCloudFrontOriginAccessIdentityRequest, String> {
    let mut input = GetCloudFrontOriginAccessIdentityRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_cloud_front_origin_access_identity_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCloudFrontOriginAccessIdentityConfigRequest, String> {
    let mut input = GetCloudFrontOriginAccessIdentityConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectionFunctionRequest, String> {
    let mut input = GetConnectionFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Stage") {
        input.stage = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_connection_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectionGroupRequest, String> {
    let mut input = GetConnectionGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_connection_group_by_routing_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectionGroupByRoutingEndpointRequest, String> {
    let mut input = GetConnectionGroupByRoutingEndpointRequest::default();
    if let Some(value) = query.get("RoutingEndpoint") {
        input.routing_endpoint = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_continuous_deployment_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetContinuousDeploymentPolicyRequest, String> {
    let mut input = GetContinuousDeploymentPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_continuous_deployment_policy_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetContinuousDeploymentPolicyConfigRequest, String> {
    let mut input = GetContinuousDeploymentPolicyConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDistributionRequest, String> {
    let mut input = GetDistributionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_distribution_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDistributionConfigRequest, String> {
    let mut input = GetDistributionConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_distribution_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDistributionTenantRequest, String> {
    let mut input = GetDistributionTenantRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_distribution_tenant_by_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDistributionTenantByDomainRequest, String> {
    let mut input = GetDistributionTenantByDomainRequest::default();
    if let Some(value) = query.get("domain") {
        input.domain = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_field_level_encryption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFieldLevelEncryptionRequest, String> {
    let mut input = GetFieldLevelEncryptionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_field_level_encryption_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFieldLevelEncryptionConfigRequest, String> {
    let mut input = GetFieldLevelEncryptionConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_field_level_encryption_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFieldLevelEncryptionProfileRequest, String> {
    let mut input = GetFieldLevelEncryptionProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_field_level_encryption_profile_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFieldLevelEncryptionProfileConfigRequest, String> {
    let mut input = GetFieldLevelEncryptionProfileConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionRequest, String> {
    let mut input = GetFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Stage") {
        input.stage = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_invalidation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInvalidationRequest, String> {
    let mut input = GetInvalidationRequest::default();
    for (name, value) in labels {
        match *name {
            "DistributionId" => {
                input.distribution_id = value.to_string();
            }
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_invalidation_for_distribution_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInvalidationForDistributionTenantRequest, String> {
    let mut input = GetInvalidationForDistributionTenantRequest::default();
    for (name, value) in labels {
        match *name {
            "DistributionTenantId" => {
                input.distribution_tenant_id = value.to_string();
            }
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_key_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetKeyGroupRequest, String> {
    let mut input = GetKeyGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_key_group_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetKeyGroupConfigRequest, String> {
    let mut input = GetKeyGroupConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_managed_certificate_details_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetManagedCertificateDetailsRequest, String> {
    let mut input = GetManagedCertificateDetailsRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_monitoring_subscription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMonitoringSubscriptionRequest, String> {
    let mut input = GetMonitoringSubscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "DistributionId" => {
                input.distribution_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_origin_access_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetOriginAccessControlRequest, String> {
    let mut input = GetOriginAccessControlRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_origin_access_control_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetOriginAccessControlConfigRequest, String> {
    let mut input = GetOriginAccessControlConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_origin_request_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetOriginRequestPolicyRequest, String> {
    let mut input = GetOriginRequestPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_origin_request_policy_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetOriginRequestPolicyConfigRequest, String> {
    let mut input = GetOriginRequestPolicyConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_public_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPublicKeyRequest, String> {
    let mut input = GetPublicKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_public_key_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPublicKeyConfigRequest, String> {
    let mut input = GetPublicKeyConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_realtime_log_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRealtimeLogConfigRequest, String> {
    let mut input = GetRealtimeLogConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<GetRealtimeLogConfigRequest>(body)
            .map_err(|err| format!("failed to deserialize GetRealtimeLogConfig request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcePolicyRequest, String> {
    let mut input = GetResourcePolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<GetResourcePolicyRequest>(body)
            .map_err(|err| format!("failed to deserialize GetResourcePolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_response_headers_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResponseHeadersPolicyRequest, String> {
    let mut input = GetResponseHeadersPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_response_headers_policy_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResponseHeadersPolicyConfigRequest, String> {
    let mut input = GetResponseHeadersPolicyConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_streaming_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStreamingDistributionRequest, String> {
    let mut input = GetStreamingDistributionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_streaming_distribution_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStreamingDistributionConfigRequest, String> {
    let mut input = GetStreamingDistributionConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrustStoreRequest, String> {
    let mut input = GetTrustStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_vpc_origin_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetVpcOriginRequest, String> {
    let mut input = GetVpcOriginRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_anycast_ip_lists_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAnycastIpListsRequest, String> {
    let mut input = ListAnycastIpListsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_cache_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCachePoliciesRequest, String> {
    let mut input = ListCachePoliciesRequest::default();
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
    if let Some(value) = query.get("Type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_cloud_front_origin_access_identities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCloudFrontOriginAccessIdentitiesRequest, String> {
    let mut input = ListCloudFrontOriginAccessIdentitiesRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_conflicting_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConflictingAliasesRequest, String> {
    let mut input = ListConflictingAliasesRequest::default();
    if let Some(value) = query.get("Alias") {
        input.alias = value.to_string();
    }
    if let Some(value) = query.get("DistributionId") {
        input.distribution_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_connection_functions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectionFunctionsRequest, String> {
    let mut input = ListConnectionFunctionsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListConnectionFunctionsRequest>(body).map_err(|err| {
            format!("failed to deserialize ListConnectionFunctions request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_connection_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectionGroupsRequest, String> {
    let mut input = ListConnectionGroupsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListConnectionGroupsRequest>(body)
            .map_err(|err| format!("failed to deserialize ListConnectionGroups request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_continuous_deployment_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContinuousDeploymentPoliciesRequest, String> {
    let mut input = ListContinuousDeploymentPoliciesRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distribution_tenants_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionTenantsRequest, String> {
    let mut input = ListDistributionTenantsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListDistributionTenantsRequest>(body).map_err(|err| {
            format!("failed to deserialize ListDistributionTenants request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distribution_tenants_by_customization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionTenantsByCustomizationRequest, String> {
    let mut input = ListDistributionTenantsByCustomizationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListDistributionTenantsByCustomizationRequest>(body)
            .map_err(|err| {
                format!(
                    "failed to deserialize ListDistributionTenantsByCustomization request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsRequest, String> {
    let mut input = ListDistributionsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_anycast_ip_list_id_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByAnycastIpListIdRequest, String> {
    let mut input = ListDistributionsByAnycastIpListIdRequest::default();
    for (name, value) in labels {
        match *name {
            "AnycastIpListId" => {
                input.anycast_ip_list_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_cache_policy_id_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByCachePolicyIdRequest, String> {
    let mut input = ListDistributionsByCachePolicyIdRequest::default();
    for (name, value) in labels {
        match *name {
            "CachePolicyId" => {
                input.cache_policy_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByConnectionFunctionRequest, String> {
    let mut input = ListDistributionsByConnectionFunctionRequest::default();
    if let Some(value) = query.get("ConnectionFunctionIdentifier") {
        input.connection_function_identifier = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_connection_mode_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByConnectionModeRequest, String> {
    let mut input = ListDistributionsByConnectionModeRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectionMode" => {
                input.connection_mode = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_key_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByKeyGroupRequest, String> {
    let mut input = ListDistributionsByKeyGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "KeyGroupId" => {
                input.key_group_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_origin_request_policy_id_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByOriginRequestPolicyIdRequest, String> {
    let mut input = ListDistributionsByOriginRequestPolicyIdRequest::default();
    for (name, value) in labels {
        match *name {
            "OriginRequestPolicyId" => {
                input.origin_request_policy_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_owned_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByOwnedResourceRequest, String> {
    let mut input = ListDistributionsByOwnedResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_realtime_log_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByRealtimeLogConfigRequest, String> {
    let mut input = ListDistributionsByRealtimeLogConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListDistributionsByRealtimeLogConfigRequest>(body)
            .map_err(|err| {
                format!("failed to deserialize ListDistributionsByRealtimeLogConfig request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_response_headers_policy_id_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByResponseHeadersPolicyIdRequest, String> {
    let mut input = ListDistributionsByResponseHeadersPolicyIdRequest::default();
    for (name, value) in labels {
        match *name {
            "ResponseHeadersPolicyId" => {
                input.response_headers_policy_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByTrustStoreRequest, String> {
    let mut input = ListDistributionsByTrustStoreRequest::default();
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
    if let Some(value) = query.get("TrustStoreIdentifier") {
        input.trust_store_identifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_vpc_origin_id_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByVpcOriginIdRequest, String> {
    let mut input = ListDistributionsByVpcOriginIdRequest::default();
    for (name, value) in labels {
        match *name {
            "VpcOriginId" => {
                input.vpc_origin_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_distributions_by_web_a_c_l_id_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDistributionsByWebACLIdRequest, String> {
    let mut input = ListDistributionsByWebACLIdRequest::default();
    for (name, value) in labels {
        match *name {
            "WebACLId" => {
                input.web_a_c_l_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_domain_conflicts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainConflictsRequest, String> {
    let mut input = ListDomainConflictsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListDomainConflictsRequest>(body)
            .map_err(|err| format!("failed to deserialize ListDomainConflicts request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_field_level_encryption_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFieldLevelEncryptionConfigsRequest, String> {
    let mut input = ListFieldLevelEncryptionConfigsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_field_level_encryption_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFieldLevelEncryptionProfilesRequest, String> {
    let mut input = ListFieldLevelEncryptionProfilesRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_functions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionsRequest, String> {
    let mut input = ListFunctionsRequest::default();
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
    if let Some(value) = query.get("Stage") {
        input.stage = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_invalidations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInvalidationsRequest, String> {
    let mut input = ListInvalidationsRequest::default();
    for (name, value) in labels {
        match *name {
            "DistributionId" => {
                input.distribution_id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_invalidations_for_distribution_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInvalidationsForDistributionTenantRequest, String> {
    let mut input = ListInvalidationsForDistributionTenantRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_key_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListKeyGroupsRequest, String> {
    let mut input = ListKeyGroupsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_key_value_stores_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListKeyValueStoresRequest, String> {
    let mut input = ListKeyValueStoresRequest::default();
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
    if let Some(value) = query.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_origin_access_controls_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOriginAccessControlsRequest, String> {
    let mut input = ListOriginAccessControlsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_origin_request_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOriginRequestPoliciesRequest, String> {
    let mut input = ListOriginRequestPoliciesRequest::default();
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
    if let Some(value) = query.get("Type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_public_keys_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPublicKeysRequest, String> {
    let mut input = ListPublicKeysRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_realtime_log_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRealtimeLogConfigsRequest, String> {
    let mut input = ListRealtimeLogConfigsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_response_headers_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResponseHeadersPoliciesRequest, String> {
    let mut input = ListResponseHeadersPoliciesRequest::default();
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
    if let Some(value) = query.get("Type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_streaming_distributions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStreamingDistributionsRequest, String> {
    let mut input = ListStreamingDistributionsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if let Some(value) = query.get("Resource") {
        input.resource = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_trust_stores_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrustStoresRequest, String> {
    let mut input = ListTrustStoresRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListTrustStoresRequest>(body)
            .map_err(|err| format!("failed to deserialize ListTrustStores request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_vpc_origins_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVpcOriginsRequest, String> {
    let mut input = ListVpcOriginsRequest::default();
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

/// Deserialize request for restXml protocol.
pub fn deserialize_publish_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PublishConnectionFunctionRequest, String> {
    let mut input = PublishConnectionFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_publish_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PublishFunctionRequest, String> {
    let mut input = PublishFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_put_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutResourcePolicyRequest, String> {
    let mut input = PutResourcePolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<PutResourcePolicyRequest>(body)
            .map_err(|err| format!("failed to deserialize PutResourcePolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<Tags>(body)
            .map_err(|err| format!("failed to deserialize TagResource payload: {err}"))?;
        input.tags = payload;
    }
    if let Some(value) = query.get("Resource") {
        input.resource = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_test_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestConnectionFunctionRequest, String> {
    let mut input = TestConnectionFunctionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<TestConnectionFunctionRequest>(body).map_err(|err| {
            format!("failed to deserialize TestConnectionFunction request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_test_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestFunctionRequest, String> {
    let mut input = TestFunctionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<TestFunctionRequest>(body)
            .map_err(|err| format!("failed to deserialize TestFunction request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<TagKeys>(body)
            .map_err(|err| format!("failed to deserialize UntagResource payload: {err}"))?;
        input.tag_keys = payload;
    }
    if let Some(value) = query.get("Resource") {
        input.resource = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_anycast_ip_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAnycastIpListRequest, String> {
    let mut input = UpdateAnycastIpListRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateAnycastIpListRequest>(body)
            .map_err(|err| format!("failed to deserialize UpdateAnycastIpList request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_cache_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCachePolicyRequest, String> {
    let mut input = UpdateCachePolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CachePolicyConfig>(body)
            .map_err(|err| format!("failed to deserialize UpdateCachePolicy payload: {err}"))?;
        input.cache_policy_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_cloud_front_origin_access_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCloudFrontOriginAccessIdentityRequest, String> {
    let mut input = UpdateCloudFrontOriginAccessIdentityRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CloudFrontOriginAccessIdentityConfig>(body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCloudFrontOriginAccessIdentity payload: {err}")
            })?;
        input.cloud_front_origin_access_identity_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_connection_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectionFunctionRequest, String> {
    let mut input = UpdateConnectionFunctionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<UpdateConnectionFunctionRequest>(body).map_err(|err| {
                format!("failed to deserialize UpdateConnectionFunction request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_connection_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectionGroupRequest, String> {
    let mut input = UpdateConnectionGroupRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateConnectionGroupRequest>(body)
            .map_err(|err| format!("failed to deserialize UpdateConnectionGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_continuous_deployment_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContinuousDeploymentPolicyRequest, String> {
    let mut input = UpdateContinuousDeploymentPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<ContinuousDeploymentPolicyConfig>(body).map_err(|err| {
                format!("failed to deserialize UpdateContinuousDeploymentPolicy payload: {err}")
            })?;
        input.continuous_deployment_policy_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDistributionRequest, String> {
    let mut input = UpdateDistributionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<DistributionConfig>(body)
            .map_err(|err| format!("failed to deserialize UpdateDistribution payload: {err}"))?;
        input.distribution_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_distribution_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDistributionTenantRequest, String> {
    let mut input = UpdateDistributionTenantRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<UpdateDistributionTenantRequest>(body).map_err(|err| {
                format!("failed to deserialize UpdateDistributionTenant request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_distribution_with_staging_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDistributionWithStagingConfigRequest, String> {
    let mut input = UpdateDistributionWithStagingConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("StagingDistributionId") {
        input.staging_distribution_id = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_domain_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDomainAssociationRequest, String> {
    let mut input = UpdateDomainAssociationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateDomainAssociationRequest>(body).map_err(|err| {
            format!("failed to deserialize UpdateDomainAssociation request: {err}")
        })?;
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_field_level_encryption_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFieldLevelEncryptionConfigRequest, String> {
    let mut input = UpdateFieldLevelEncryptionConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<FieldLevelEncryptionConfig>(body).map_err(|err| {
                format!("failed to deserialize UpdateFieldLevelEncryptionConfig payload: {err}")
            })?;
        input.field_level_encryption_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_field_level_encryption_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFieldLevelEncryptionProfileRequest, String> {
    let mut input = UpdateFieldLevelEncryptionProfileRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<FieldLevelEncryptionProfileConfig>(body).map_err(|err| {
                format!("failed to deserialize UpdateFieldLevelEncryptionProfile payload: {err}")
            })?;
        input.field_level_encryption_profile_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFunctionRequest, String> {
    let mut input = UpdateFunctionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateFunctionRequest>(body)
            .map_err(|err| format!("failed to deserialize UpdateFunction request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_key_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateKeyGroupRequest, String> {
    let mut input = UpdateKeyGroupRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<KeyGroupConfig>(body)
            .map_err(|err| format!("failed to deserialize UpdateKeyGroup payload: {err}"))?;
        input.key_group_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_key_value_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateKeyValueStoreRequest, String> {
    let mut input = UpdateKeyValueStoreRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateKeyValueStoreRequest>(body)
            .map_err(|err| format!("failed to deserialize UpdateKeyValueStore request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_origin_access_control_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateOriginAccessControlRequest, String> {
    let mut input = UpdateOriginAccessControlRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<OriginAccessControlConfig>(body).map_err(|err| {
                format!("failed to deserialize UpdateOriginAccessControl payload: {err}")
            })?;
        input.origin_access_control_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_origin_request_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateOriginRequestPolicyRequest, String> {
    let mut input = UpdateOriginRequestPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<OriginRequestPolicyConfig>(body).map_err(|err| {
                format!("failed to deserialize UpdateOriginRequestPolicy payload: {err}")
            })?;
        input.origin_request_policy_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_public_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePublicKeyRequest, String> {
    let mut input = UpdatePublicKeyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<PublicKeyConfig>(body)
            .map_err(|err| format!("failed to deserialize UpdatePublicKey payload: {err}"))?;
        input.public_key_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_realtime_log_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRealtimeLogConfigRequest, String> {
    let mut input = UpdateRealtimeLogConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateRealtimeLogConfigRequest>(body).map_err(|err| {
            format!("failed to deserialize UpdateRealtimeLogConfig request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_response_headers_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResponseHeadersPolicyRequest, String> {
    let mut input = UpdateResponseHeadersPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<ResponseHeadersPolicyConfig>(body).map_err(|err| {
                format!("failed to deserialize UpdateResponseHeadersPolicy payload: {err}")
            })?;
        input.response_headers_policy_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_streaming_distribution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStreamingDistributionRequest, String> {
    let mut input = UpdateStreamingDistributionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload =
            quick_xml::de::from_str::<StreamingDistributionConfig>(body).map_err(|err| {
                format!("failed to deserialize UpdateStreamingDistribution payload: {err}")
            })?;
        input.streaming_distribution_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTrustStoreRequest, String> {
    let mut input = UpdateTrustStoreRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<CaCertificatesBundleSource>(body)
            .map_err(|err| format!("failed to deserialize UpdateTrustStore payload: {err}"))?;
        input.ca_certificates_bundle_source = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_vpc_origin_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVpcOriginRequest, String> {
    let mut input = UpdateVpcOriginRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        let payload = quick_xml::de::from_str::<VpcOriginEndpointConfig>(body)
            .map_err(|err| format!("failed to deserialize UpdateVpcOrigin payload: {err}"))?;
        input.vpc_origin_endpoint_config = payload;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("If-Match")
        .and_then(|value| value.to_str().ok())
    {
        input.if_match = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_verify_dns_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<VerifyDnsConfigurationRequest, String> {
    let mut input = VerifyDnsConfigurationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<VerifyDnsConfigurationRequest>(body).map_err(|err| {
            format!("failed to deserialize VerifyDnsConfiguration request: {err}")
        })?;
    }
    Ok(input)
}
