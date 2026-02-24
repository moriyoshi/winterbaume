//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-route53

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restXml protocol.
pub fn serialize_activate_key_signing_key_response(
    result: &ActivateKeySigningKeyResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_associate_v_p_c_with_hosted_zone_response(
    result: &AssociateVPCWithHostedZoneResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_change_cidr_collection_response(
    result: &ChangeCidrCollectionResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_change_resource_record_sets_response(
    result: &ChangeResourceRecordSetsResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_change_tags_for_resource_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_create_cidr_collection_response(
    result: &CreateCidrCollectionResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_health_check_response(result: &CreateHealthCheckResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_hosted_zone_response(result: &CreateHostedZoneResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_key_signing_key_response(
    result: &CreateKeySigningKeyResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_query_logging_config_response(
    result: &CreateQueryLoggingConfigResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_reusable_delegation_set_response(
    result: &CreateReusableDelegationSetResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_traffic_policy_response(
    result: &CreateTrafficPolicyResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_traffic_policy_instance_response(
    result: &CreateTrafficPolicyInstanceResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_traffic_policy_version_response(
    result: &CreateTrafficPolicyVersionResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    let resp = MockResponse::xml(201, body);
    // Header "location": set by caller from location field
    resp
}

/// Serialize response for restXml protocol.
pub fn serialize_create_v_p_c_association_authorization_response(
    result: &CreateVPCAssociationAuthorizationResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_deactivate_key_signing_key_response(
    result: &DeactivateKeySigningKeyResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_cidr_collection_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_health_check_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_hosted_zone_response(result: &DeleteHostedZoneResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_key_signing_key_response(
    result: &DeleteKeySigningKeyResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_query_logging_config_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_reusable_delegation_set_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_traffic_policy_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_traffic_policy_instance_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_delete_v_p_c_association_authorization_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_disable_hosted_zone_d_n_s_s_e_c_response(
    result: &DisableHostedZoneDNSSECResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_disassociate_v_p_c_from_hosted_zone_response(
    result: &DisassociateVPCFromHostedZoneResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_enable_hosted_zone_d_n_s_s_e_c_response(
    result: &EnableHostedZoneDNSSECResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_account_limit_response(result: &GetAccountLimitResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_change_response(result: &GetChangeResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_checker_ip_ranges_response(
    result: &GetCheckerIpRangesResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_d_n_s_s_e_c_response(result: &GetDNSSECResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_geo_location_response(result: &GetGeoLocationResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_health_check_response(result: &GetHealthCheckResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_health_check_count_response(
    result: &GetHealthCheckCountResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_health_check_last_failure_reason_response(
    result: &GetHealthCheckLastFailureReasonResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_health_check_status_response(
    result: &GetHealthCheckStatusResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_hosted_zone_response(result: &GetHostedZoneResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_hosted_zone_count_response(
    result: &GetHostedZoneCountResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_hosted_zone_limit_response(
    result: &GetHostedZoneLimitResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_query_logging_config_response(
    result: &GetQueryLoggingConfigResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_reusable_delegation_set_response(
    result: &GetReusableDelegationSetResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_reusable_delegation_set_limit_response(
    result: &GetReusableDelegationSetLimitResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_traffic_policy_response(result: &GetTrafficPolicyResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_traffic_policy_instance_response(
    result: &GetTrafficPolicyInstanceResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_get_traffic_policy_instance_count_response(
    result: &GetTrafficPolicyInstanceCountResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_cidr_blocks_response(result: &ListCidrBlocksResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_cidr_collections_response(
    result: &ListCidrCollectionsResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_cidr_locations_response(result: &ListCidrLocationsResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_geo_locations_response(result: &ListGeoLocationsResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_health_checks_response(result: &ListHealthChecksResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_hosted_zones_response(result: &ListHostedZonesResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_hosted_zones_by_name_response(
    result: &ListHostedZonesByNameResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_hosted_zones_by_v_p_c_response(
    result: &ListHostedZonesByVPCResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_query_logging_configs_response(
    result: &ListQueryLoggingConfigsResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_resource_record_sets_response(
    result: &ListResourceRecordSetsResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_reusable_delegation_sets_response(
    result: &ListReusableDelegationSetsResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_tags_for_resources_response(
    result: &ListTagsForResourcesResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_traffic_policies_response(
    result: &ListTrafficPoliciesResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_traffic_policy_instances_response(
    result: &ListTrafficPolicyInstancesResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_traffic_policy_instances_by_hosted_zone_response(
    result: &ListTrafficPolicyInstancesByHostedZoneResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_traffic_policy_instances_by_policy_response(
    result: &ListTrafficPolicyInstancesByPolicyResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_traffic_policy_versions_response(
    result: &ListTrafficPolicyVersionsResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_list_v_p_c_association_authorizations_response(
    result: &ListVPCAssociationAuthorizationsResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_test_d_n_s_answer_response(result: &TestDNSAnswerResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_health_check_response(result: &UpdateHealthCheckResponse) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_hosted_zone_comment_response(
    result: &UpdateHostedZoneCommentResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_hosted_zone_features_response() -> MockResponse {
    let body = String::new();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_traffic_policy_comment_response(
    result: &UpdateTrafficPolicyCommentResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Serialize response for restXml protocol.
pub fn serialize_update_traffic_policy_instance_response(
    result: &UpdateTrafficPolicyInstanceResponse,
) -> MockResponse {
    let body = quick_xml::se::to_string(result).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_activate_key_signing_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ActivateKeySigningKeyRequest, String> {
    let mut input = ActivateKeySigningKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_associate_v_p_c_with_hosted_zone_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateVPCWithHostedZoneRequest, String> {
    let mut input = AssociateVPCWithHostedZoneRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<AssociateVPCWithHostedZoneRequest>(body).map_err(|err| {
                format!("failed to deserialize AssociateVPCWithHostedZone request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_change_cidr_collection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ChangeCidrCollectionRequest, String> {
    let mut input = ChangeCidrCollectionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ChangeCidrCollectionRequest>(body)
            .map_err(|err| format!("failed to deserialize ChangeCidrCollection request: {err}"))?;
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
pub fn deserialize_change_resource_record_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ChangeResourceRecordSetsRequest, String> {
    let mut input = ChangeResourceRecordSetsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<ChangeResourceRecordSetsRequest>(body).map_err(|err| {
                format!("failed to deserialize ChangeResourceRecordSets request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_change_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ChangeTagsForResourceRequest, String> {
    let mut input = ChangeTagsForResourceRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ChangeTagsForResourceRequest>(body)
            .map_err(|err| format!("failed to deserialize ChangeTagsForResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceId" => {
                input.resource_id = value.to_string();
            }
            "ResourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_cidr_collection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCidrCollectionRequest, String> {
    let mut input = CreateCidrCollectionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateCidrCollectionRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateCidrCollection request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_health_check_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateHealthCheckRequest, String> {
    let mut input = CreateHealthCheckRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateHealthCheckRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateHealthCheck request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_hosted_zone_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateHostedZoneRequest, String> {
    let mut input = CreateHostedZoneRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateHostedZoneRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateHostedZone request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_key_signing_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateKeySigningKeyRequest, String> {
    let mut input = CreateKeySigningKeyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateKeySigningKeyRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateKeySigningKey request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_query_logging_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateQueryLoggingConfigRequest, String> {
    let mut input = CreateQueryLoggingConfigRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateQueryLoggingConfigRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateQueryLoggingConfig request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_reusable_delegation_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateReusableDelegationSetRequest, String> {
    let mut input = CreateReusableDelegationSetRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateReusableDelegationSetRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateReusableDelegationSet request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_traffic_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTrafficPolicyRequest, String> {
    let mut input = CreateTrafficPolicyRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateTrafficPolicyRequest>(body)
            .map_err(|err| format!("failed to deserialize CreateTrafficPolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_traffic_policy_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTrafficPolicyInstanceRequest, String> {
    let mut input = CreateTrafficPolicyInstanceRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateTrafficPolicyInstanceRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateTrafficPolicyInstance request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_traffic_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTrafficPolicyVersionRequest, String> {
    let mut input = CreateTrafficPolicyVersionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<CreateTrafficPolicyVersionRequest>(body).map_err(|err| {
                format!("failed to deserialize CreateTrafficPolicyVersion request: {err}")
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
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_create_v_p_c_association_authorization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVPCAssociationAuthorizationRequest, String> {
    let mut input = CreateVPCAssociationAuthorizationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<CreateVPCAssociationAuthorizationRequest>(body).map_err(
            |err| format!("failed to deserialize CreateVPCAssociationAuthorization request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_deactivate_key_signing_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeactivateKeySigningKeyRequest, String> {
    let mut input = DeactivateKeySigningKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_cidr_collection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCidrCollectionRequest, String> {
    let mut input = DeleteCidrCollectionRequest::default();
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
pub fn deserialize_delete_health_check_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteHealthCheckRequest, String> {
    let mut input = DeleteHealthCheckRequest::default();
    for (name, value) in labels {
        match *name {
            "HealthCheckId" => {
                input.health_check_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_hosted_zone_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteHostedZoneRequest, String> {
    let mut input = DeleteHostedZoneRequest::default();
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
pub fn deserialize_delete_key_signing_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteKeySigningKeyRequest, String> {
    let mut input = DeleteKeySigningKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_query_logging_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteQueryLoggingConfigRequest, String> {
    let mut input = DeleteQueryLoggingConfigRequest::default();
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
pub fn deserialize_delete_reusable_delegation_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteReusableDelegationSetRequest, String> {
    let mut input = DeleteReusableDelegationSetRequest::default();
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
pub fn deserialize_delete_traffic_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTrafficPolicyRequest, String> {
    let mut input = DeleteTrafficPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            "Version" => {
                input.version = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_delete_traffic_policy_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTrafficPolicyInstanceRequest, String> {
    let mut input = DeleteTrafficPolicyInstanceRequest::default();
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
pub fn deserialize_delete_v_p_c_association_authorization_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVPCAssociationAuthorizationRequest, String> {
    let mut input = DeleteVPCAssociationAuthorizationRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<DeleteVPCAssociationAuthorizationRequest>(body).map_err(
            |err| format!("failed to deserialize DeleteVPCAssociationAuthorization request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_disable_hosted_zone_d_n_s_s_e_c_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableHostedZoneDNSSECRequest, String> {
    let mut input = DisableHostedZoneDNSSECRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_disassociate_v_p_c_from_hosted_zone_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateVPCFromHostedZoneRequest, String> {
    let mut input = DisassociateVPCFromHostedZoneRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<DisassociateVPCFromHostedZoneRequest>(body).map_err(
            |err| format!("failed to deserialize DisassociateVPCFromHostedZone request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_enable_hosted_zone_d_n_s_s_e_c_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableHostedZoneDNSSECRequest, String> {
    let mut input = EnableHostedZoneDNSSECRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_account_limit_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccountLimitRequest, String> {
    let mut input = GetAccountLimitRequest::default();
    for (name, value) in labels {
        match *name {
            "Type" => {
                input.r#type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_change_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetChangeRequest, String> {
    let mut input = GetChangeRequest::default();
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
pub fn deserialize_get_checker_ip_ranges_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCheckerIpRangesRequest, String> {
    let input = GetCheckerIpRangesRequest {};
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_d_n_s_s_e_c_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDNSSECRequest, String> {
    let mut input = GetDNSSECRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_geo_location_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGeoLocationRequest, String> {
    let mut input = GetGeoLocationRequest::default();
    if let Some(value) = query.get("continentcode") {
        input.continent_code = Some(value.to_string());
    }
    if let Some(value) = query.get("countrycode") {
        input.country_code = Some(value.to_string());
    }
    if let Some(value) = query.get("subdivisioncode") {
        input.subdivision_code = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_health_check_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHealthCheckRequest, String> {
    let mut input = GetHealthCheckRequest::default();
    for (name, value) in labels {
        match *name {
            "HealthCheckId" => {
                input.health_check_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_health_check_count_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHealthCheckCountRequest, String> {
    let input = GetHealthCheckCountRequest {};
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_health_check_last_failure_reason_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHealthCheckLastFailureReasonRequest, String> {
    let mut input = GetHealthCheckLastFailureReasonRequest::default();
    for (name, value) in labels {
        match *name {
            "HealthCheckId" => {
                input.health_check_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_health_check_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHealthCheckStatusRequest, String> {
    let mut input = GetHealthCheckStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "HealthCheckId" => {
                input.health_check_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_hosted_zone_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHostedZoneRequest, String> {
    let mut input = GetHostedZoneRequest::default();
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
pub fn deserialize_get_hosted_zone_count_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHostedZoneCountRequest, String> {
    let input = GetHostedZoneCountRequest {};
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_hosted_zone_limit_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHostedZoneLimitRequest, String> {
    let mut input = GetHostedZoneLimitRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            "Type" => {
                input.r#type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_query_logging_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetQueryLoggingConfigRequest, String> {
    let mut input = GetQueryLoggingConfigRequest::default();
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
pub fn deserialize_get_reusable_delegation_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetReusableDelegationSetRequest, String> {
    let mut input = GetReusableDelegationSetRequest::default();
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
pub fn deserialize_get_reusable_delegation_set_limit_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetReusableDelegationSetLimitRequest, String> {
    let mut input = GetReusableDelegationSetLimitRequest::default();
    for (name, value) in labels {
        match *name {
            "DelegationSetId" => {
                input.delegation_set_id = value.to_string();
            }
            "Type" => {
                input.r#type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_traffic_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrafficPolicyRequest, String> {
    let mut input = GetTrafficPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            "Version" => {
                input.version = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_get_traffic_policy_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrafficPolicyInstanceRequest, String> {
    let mut input = GetTrafficPolicyInstanceRequest::default();
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
pub fn deserialize_get_traffic_policy_instance_count_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrafficPolicyInstanceCountRequest, String> {
    let input = GetTrafficPolicyInstanceCountRequest {};
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_cidr_blocks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCidrBlocksRequest, String> {
    let mut input = ListCidrBlocksRequest::default();
    for (name, value) in labels {
        match *name {
            "CollectionId" => {
                input.collection_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("location") {
        input.location_name = Some(value.to_string());
    }
    if let Some(value) = query.get("maxresults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nexttoken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_cidr_collections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCidrCollectionsRequest, String> {
    let mut input = ListCidrCollectionsRequest::default();
    if let Some(value) = query.get("maxresults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nexttoken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_cidr_locations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCidrLocationsRequest, String> {
    let mut input = ListCidrLocationsRequest::default();
    for (name, value) in labels {
        match *name {
            "CollectionId" => {
                input.collection_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxresults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nexttoken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_geo_locations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGeoLocationsRequest, String> {
    let mut input = ListGeoLocationsRequest::default();
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("startcontinentcode") {
        input.start_continent_code = Some(value.to_string());
    }
    if let Some(value) = query.get("startcountrycode") {
        input.start_country_code = Some(value.to_string());
    }
    if let Some(value) = query.get("startsubdivisioncode") {
        input.start_subdivision_code = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_health_checks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHealthChecksRequest, String> {
    let mut input = ListHealthChecksRequest::default();
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_hosted_zones_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHostedZonesRequest, String> {
    let mut input = ListHostedZonesRequest::default();
    if let Some(value) = query.get("delegationsetid") {
        input.delegation_set_id = Some(value.to_string());
    }
    if let Some(value) = query.get("hostedzonetype") {
        input.hosted_zone_type = Some(value.to_string());
    }
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_hosted_zones_by_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHostedZonesByNameRequest, String> {
    let mut input = ListHostedZonesByNameRequest::default();
    if let Some(value) = query.get("dnsname") {
        input.d_n_s_name = Some(value.to_string());
    }
    if let Some(value) = query.get("hostedzoneid") {
        input.hosted_zone_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_hosted_zones_by_v_p_c_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHostedZonesByVPCRequest, String> {
    let mut input = ListHostedZonesByVPCRequest::default();
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nexttoken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("vpcid") {
        input.v_p_c_id = value.to_string();
    }
    if let Some(value) = query.get("vpcregion") {
        input.v_p_c_region = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_query_logging_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListQueryLoggingConfigsRequest, String> {
    let mut input = ListQueryLoggingConfigsRequest::default();
    if let Some(value) = query.get("hostedzoneid") {
        input.hosted_zone_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxresults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nexttoken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_resource_record_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceRecordSetsRequest, String> {
    let mut input = ListResourceRecordSetsRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("identifier") {
        input.start_record_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("name") {
        input.start_record_name = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.start_record_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_reusable_delegation_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReusableDelegationSetsRequest, String> {
    let mut input = ListReusableDelegationSetsRequest::default();
    if let Some(value) = query.get("marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("maxitems") {
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
    for (name, value) in labels {
        match *name {
            "ResourceId" => {
                input.resource_id = value.to_string();
            }
            "ResourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_tags_for_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourcesRequest, String> {
    let mut input = ListTagsForResourcesRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<ListTagsForResourcesRequest>(body)
            .map_err(|err| format!("failed to deserialize ListTagsForResources request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_traffic_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrafficPoliciesRequest, String> {
    let mut input = ListTrafficPoliciesRequest::default();
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("trafficpolicyid") {
        input.traffic_policy_id_marker = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_traffic_policy_instances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrafficPolicyInstancesRequest, String> {
    let mut input = ListTrafficPolicyInstancesRequest::default();
    if let Some(value) = query.get("hostedzoneid") {
        input.hosted_zone_id_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("trafficpolicyinstancename") {
        input.traffic_policy_instance_name_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("trafficpolicyinstancetype") {
        input.traffic_policy_instance_type_marker = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_traffic_policy_instances_by_hosted_zone_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrafficPolicyInstancesByHostedZoneRequest, String> {
    let mut input = ListTrafficPolicyInstancesByHostedZoneRequest::default();
    if let Some(value) = query.get("id") {
        input.hosted_zone_id = value.to_string();
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("trafficpolicyinstancename") {
        input.traffic_policy_instance_name_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("trafficpolicyinstancetype") {
        input.traffic_policy_instance_type_marker = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_traffic_policy_instances_by_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrafficPolicyInstancesByPolicyRequest, String> {
    let mut input = ListTrafficPolicyInstancesByPolicyRequest::default();
    if let Some(value) = query.get("hostedzoneid") {
        input.hosted_zone_id_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("id") {
        input.traffic_policy_id = value.to_string();
    }
    if let Some(value) = query.get("trafficpolicyinstancename") {
        input.traffic_policy_instance_name_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("trafficpolicyinstancetype") {
        input.traffic_policy_instance_type_marker = Some(value.to_string());
    }
    if let Some(value) = query.get("version") {
        input.traffic_policy_version = value
            .parse::<i32>()
            .map_err(|err| format!("failed to parse integer: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_traffic_policy_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrafficPolicyVersionsRequest, String> {
    let mut input = ListTrafficPolicyVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxitems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("trafficpolicyversion") {
        input.traffic_policy_version_marker = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_list_v_p_c_association_authorizations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVPCAssociationAuthorizationsRequest, String> {
    let mut input = ListVPCAssociationAuthorizationsRequest::default();
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxresults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nexttoken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_test_d_n_s_answer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestDNSAnswerRequest, String> {
    let mut input = TestDNSAnswerRequest::default();
    if let Some(value) = query.get("edns0clientsubnetip") {
        input.e_d_n_s0_client_subnet_i_p = Some(value.to_string());
    }
    if let Some(value) = query.get("edns0clientsubnetmask") {
        input.e_d_n_s0_client_subnet_mask = Some(value.to_string());
    }
    if let Some(value) = query.get("hostedzoneid") {
        input.hosted_zone_id = value.to_string();
    }
    if let Some(value) = query.get("recordname") {
        input.record_name = value.to_string();
    }
    if let Some(value) = query.get("recordtype") {
        input.record_type = value.to_string();
    }
    if let Some(value) = query.get("resolverip") {
        input.resolver_i_p = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_health_check_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateHealthCheckRequest, String> {
    let mut input = UpdateHealthCheckRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateHealthCheckRequest>(body)
            .map_err(|err| format!("failed to deserialize UpdateHealthCheck request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "HealthCheckId" => {
                input.health_check_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_hosted_zone_comment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateHostedZoneCommentRequest, String> {
    let mut input = UpdateHostedZoneCommentRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input = quick_xml::de::from_str::<UpdateHostedZoneCommentRequest>(body).map_err(|err| {
            format!("failed to deserialize UpdateHostedZoneComment request: {err}")
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
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_hosted_zone_features_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateHostedZoneFeaturesRequest, String> {
    let mut input = UpdateHostedZoneFeaturesRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<UpdateHostedZoneFeaturesRequest>(body).map_err(|err| {
                format!("failed to deserialize UpdateHostedZoneFeatures request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "HostedZoneId" => {
                input.hosted_zone_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_traffic_policy_comment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTrafficPolicyCommentRequest, String> {
    let mut input = UpdateTrafficPolicyCommentRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<UpdateTrafficPolicyCommentRequest>(body).map_err(|err| {
                format!("failed to deserialize UpdateTrafficPolicyComment request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            "Version" => {
                input.version = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restXml protocol.
pub fn deserialize_update_traffic_policy_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTrafficPolicyInstanceRequest, String> {
    let mut input = UpdateTrafficPolicyInstanceRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input =
            quick_xml::de::from_str::<UpdateTrafficPolicyInstanceRequest>(body).map_err(|err| {
                format!("failed to deserialize UpdateTrafficPolicyInstance request: {err}")
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
    Ok(input)
}
