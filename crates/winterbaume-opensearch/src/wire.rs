//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-opensearch

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
pub fn serialize_accept_inbound_connection_response(
    result: &AcceptInboundConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_data_source_response(result: &AddDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_direct_query_data_source_response(
    result: &AddDirectQueryDataSourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_add_tags_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_package_response(result: &AssociatePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_packages_response(result: &AssociatePackagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_authorize_vpc_endpoint_access_response(
    result: &AuthorizeVpcEndpointAccessResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_domain_config_change_response(
    result: &CancelDomainConfigChangeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_service_software_update_response(
    result: &CancelServiceSoftwareUpdateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_application_response(result: &CreateApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_domain_response(result: &CreateDomainResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_index_response(result: &CreateIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_outbound_connection_response(
    result: &CreateOutboundConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_package_response(result: &CreatePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vpc_endpoint_response(result: &CreateVpcEndpointResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_application_response(result: &DeleteApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_source_response(result: &DeleteDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_direct_query_data_source_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_domain_response(result: &DeleteDomainResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_inbound_connection_response(
    result: &DeleteInboundConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_index_response(result: &DeleteIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_outbound_connection_response(
    result: &DeleteOutboundConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_package_response(result: &DeletePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_vpc_endpoint_response(result: &DeleteVpcEndpointResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_capability_response(
    result: &DeregisterCapabilityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domain_response(result: &DescribeDomainResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domain_auto_tunes_response(
    result: &DescribeDomainAutoTunesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domain_change_progress_response(
    result: &DescribeDomainChangeProgressResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domain_config_response(
    result: &DescribeDomainConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domain_health_response(
    result: &DescribeDomainHealthResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domain_nodes_response(
    result: &DescribeDomainNodesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_domains_response(result: &DescribeDomainsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_dry_run_progress_response(
    result: &DescribeDryRunProgressResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_inbound_connections_response(
    result: &DescribeInboundConnectionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_insight_details_response(
    result: &DescribeInsightDetailsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_instance_type_limits_response(
    result: &DescribeInstanceTypeLimitsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_outbound_connections_response(
    result: &DescribeOutboundConnectionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_packages_response(result: &DescribePackagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_reserved_instance_offerings_response(
    result: &DescribeReservedInstanceOfferingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_reserved_instances_response(
    result: &DescribeReservedInstancesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_vpc_endpoints_response(
    result: &DescribeVpcEndpointsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_dissociate_package_response(result: &DissociatePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_dissociate_packages_response(result: &DissociatePackagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_application_response(result: &GetApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_capability_response(result: &GetCapabilityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_compatible_versions_response(
    result: &GetCompatibleVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_source_response(result: &GetDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_default_application_setting_response(
    result: &GetDefaultApplicationSettingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_direct_query_data_source_response(
    result: &GetDirectQueryDataSourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_maintenance_status_response(
    result: &GetDomainMaintenanceStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_index_response(result: &GetIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_package_version_history_response(
    result: &GetPackageVersionHistoryResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_upgrade_history_response(result: &GetUpgradeHistoryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_upgrade_status_response(result: &GetUpgradeStatusResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_applications_response(result: &ListApplicationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_sources_response(result: &ListDataSourcesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_direct_query_data_sources_response(
    result: &ListDirectQueryDataSourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domain_maintenances_response(
    result: &ListDomainMaintenancesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domain_names_response(result: &ListDomainNamesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domains_for_package_response(
    result: &ListDomainsForPackageResponse,
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
pub fn serialize_list_instance_type_details_response(
    result: &ListInstanceTypeDetailsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_packages_for_domain_response(
    result: &ListPackagesForDomainResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_scheduled_actions_response(
    result: &ListScheduledActionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_versions_response(result: &ListVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_vpc_endpoint_access_response(
    result: &ListVpcEndpointAccessResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_vpc_endpoints_response(result: &ListVpcEndpointsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_vpc_endpoints_for_domain_response(
    result: &ListVpcEndpointsForDomainResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_purchase_reserved_instance_offering_response(
    result: &PurchaseReservedInstanceOfferingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_default_application_setting_response(
    result: &PutDefaultApplicationSettingResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_capability_response(result: &RegisterCapabilityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reject_inbound_connection_response(
    result: &RejectInboundConnectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_remove_tags_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_revoke_vpc_endpoint_access_response(
    result: &RevokeVpcEndpointAccessResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_rollback_service_software_update_response(
    result: &RollbackServiceSoftwareUpdateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_domain_maintenance_response(
    result: &StartDomainMaintenanceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_service_software_update_response(
    result: &StartServiceSoftwareUpdateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_application_response(result: &UpdateApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_source_response(result: &UpdateDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_direct_query_data_source_response(
    result: &UpdateDirectQueryDataSourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_domain_config_response(
    result: &UpdateDomainConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_index_response(result: &UpdateIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_package_response(result: &UpdatePackageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_package_scope_response(
    result: &UpdatePackageScopeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_scheduled_action_response(
    result: &UpdateScheduledActionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_vpc_endpoint_response(result: &UpdateVpcEndpointResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_upgrade_domain_response(result: &UpgradeDomainResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_inbound_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptInboundConnectionRequest, String> {
    let mut input = AcceptInboundConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectionId" => {
                input.connection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddDataSourceRequest, String> {
    let mut input = AddDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_direct_query_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddDirectQueryDataSourceRequest, String> {
    let mut input = AddDirectQueryDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddDirectQueryDataSourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AddDirectQueryDataSource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddTagsRequest, String> {
    let mut input = AddTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddTags request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociatePackageRequest, String> {
    let mut input = AssociatePackageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociatePackageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociatePackage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "PackageID" => {
                input.package_i_d = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_packages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociatePackagesRequest, String> {
    let mut input = AssociatePackagesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociatePackagesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociatePackages request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_authorize_vpc_endpoint_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AuthorizeVpcEndpointAccessRequest, String> {
    let mut input = AuthorizeVpcEndpointAccessRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AuthorizeVpcEndpointAccessRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AuthorizeVpcEndpointAccess request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_domain_config_change_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelDomainConfigChangeRequest, String> {
    let mut input = CancelDomainConfigChangeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelDomainConfigChangeRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CancelDomainConfigChange request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_service_software_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelServiceSoftwareUpdateRequest, String> {
    let mut input = CancelServiceSoftwareUpdateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelServiceSoftwareUpdateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CancelServiceSoftwareUpdate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApplicationRequest, String> {
    let mut input = CreateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApplication request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDomainRequest, String> {
    let mut input = CreateDomainRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDomainRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDomain request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIndexRequest, String> {
    let mut input = CreateIndexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIndexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateIndex request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_outbound_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateOutboundConnectionRequest, String> {
    let mut input = CreateOutboundConnectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateOutboundConnectionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateOutboundConnection request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePackageRequest, String> {
    let mut input = CreatePackageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePackageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePackage request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_vpc_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVpcEndpointRequest, String> {
    let mut input = CreateVpcEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVpcEndpointRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVpcEndpoint request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApplicationRequest, String> {
    let mut input = DeleteApplicationRequest::default();
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
pub fn deserialize_delete_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataSourceRequest, String> {
    let mut input = DeleteDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_direct_query_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDirectQueryDataSourceRequest, String> {
    let mut input = DeleteDirectQueryDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "DataSourceName" => {
                input.data_source_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDomainRequest, String> {
    let mut input = DeleteDomainRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_inbound_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInboundConnectionRequest, String> {
    let mut input = DeleteInboundConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectionId" => {
                input.connection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIndexRequest, String> {
    let mut input = DeleteIndexRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "IndexName" => {
                input.index_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_outbound_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteOutboundConnectionRequest, String> {
    let mut input = DeleteOutboundConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectionId" => {
                input.connection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePackageRequest, String> {
    let mut input = DeletePackageRequest::default();
    for (name, value) in labels {
        match *name {
            "PackageID" => {
                input.package_i_d = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_vpc_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVpcEndpointRequest, String> {
    let mut input = DeleteVpcEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "VpcEndpointId" => {
                input.vpc_endpoint_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_capability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterCapabilityRequest, String> {
    let mut input = DeregisterCapabilityRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "capabilityName" => {
                input.capability_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainRequest, String> {
    let mut input = DescribeDomainRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_domain_auto_tunes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainAutoTunesRequest, String> {
    let mut input = DescribeDomainAutoTunesRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
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
pub fn deserialize_describe_domain_change_progress_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainChangeProgressRequest, String> {
    let mut input = DescribeDomainChangeProgressRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("changeid") {
        input.change_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_domain_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainConfigRequest, String> {
    let mut input = DescribeDomainConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_domain_health_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainHealthRequest, String> {
    let mut input = DescribeDomainHealthRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_domain_nodes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainNodesRequest, String> {
    let mut input = DescribeDomainNodesRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_domains_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDomainsRequest, String> {
    let mut input = DescribeDomainsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeDomainsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeDomains request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_dry_run_progress_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeDryRunProgressRequest, String> {
    let mut input = DescribeDryRunProgressRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("dryRunId") {
        input.dry_run_id = Some(value.to_string());
    }
    if let Some(value) = query.get("loadDryRunConfig") {
        input.load_dry_run_config = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_inbound_connections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInboundConnectionsRequest, String> {
    let mut input = DescribeInboundConnectionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeInboundConnectionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeInboundConnections request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_insight_details_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInsightDetailsRequest, String> {
    let mut input = DescribeInsightDetailsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeInsightDetailsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeInsightDetails request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_instance_type_limits_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInstanceTypeLimitsRequest, String> {
    let mut input = DescribeInstanceTypeLimitsRequest::default();
    for (name, value) in labels {
        match *name {
            "EngineVersion" => {
                input.engine_version = value.to_string();
            }
            "InstanceType" => {
                input.instance_type = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainName") {
        input.domain_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_outbound_connections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeOutboundConnectionsRequest, String> {
    let mut input = DescribeOutboundConnectionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeOutboundConnectionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeOutboundConnections request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_packages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribePackagesRequest, String> {
    let mut input = DescribePackagesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribePackagesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribePackages request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_reserved_instance_offerings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedInstanceOfferingsRequest, String> {
    let mut input = DescribeReservedInstanceOfferingsRequest::default();
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
    if let Some(value) = query.get("offeringId") {
        input.reserved_instance_offering_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_reserved_instances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservedInstancesRequest, String> {
    let mut input = DescribeReservedInstancesRequest::default();
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
    if let Some(value) = query.get("reservationId") {
        input.reserved_instance_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_vpc_endpoints_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVpcEndpointsRequest, String> {
    let mut input = DescribeVpcEndpointsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeVpcEndpointsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeVpcEndpoints request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_dissociate_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DissociatePackageRequest, String> {
    let mut input = DissociatePackageRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "PackageID" => {
                input.package_i_d = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_dissociate_packages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DissociatePackagesRequest, String> {
    let mut input = DissociatePackagesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DissociatePackagesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DissociatePackages request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApplicationRequest, String> {
    let mut input = GetApplicationRequest::default();
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
pub fn deserialize_get_capability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCapabilityRequest, String> {
    let mut input = GetCapabilityRequest::default();
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            "capabilityName" => {
                input.capability_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_compatible_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCompatibleVersionsRequest, String> {
    let mut input = GetCompatibleVersionsRequest::default();
    if let Some(value) = query.get("domainName") {
        input.domain_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataSourceRequest, String> {
    let mut input = GetDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_default_application_setting_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDefaultApplicationSettingRequest, String> {
    let input = GetDefaultApplicationSettingRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_direct_query_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDirectQueryDataSourceRequest, String> {
    let mut input = GetDirectQueryDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "DataSourceName" => {
                input.data_source_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_maintenance_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainMaintenanceStatusRequest, String> {
    let mut input = GetDomainMaintenanceStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maintenanceId") {
        input.maintenance_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIndexRequest, String> {
    let mut input = GetIndexRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "IndexName" => {
                input.index_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_package_version_history_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPackageVersionHistoryRequest, String> {
    let mut input = GetPackageVersionHistoryRequest::default();
    for (name, value) in labels {
        match *name {
            "PackageID" => {
                input.package_i_d = value.to_string();
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
pub fn deserialize_get_upgrade_history_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUpgradeHistoryRequest, String> {
    let mut input = GetUpgradeHistoryRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
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
pub fn deserialize_get_upgrade_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUpgradeStatusRequest, String> {
    let mut input = GetUpgradeStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_applications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListApplicationsRequest, String> {
    let mut input = ListApplicationsRequest::default();
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
    if let Some(value) = query.get("statuses") {
        input.statuses = Some(
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
pub fn deserialize_list_data_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataSourcesRequest, String> {
    let mut input = ListDataSourcesRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_direct_query_data_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDirectQueryDataSourcesRequest, String> {
    let mut input = ListDirectQueryDataSourcesRequest::default();
    if let Some(value) = query.get("nexttoken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_domain_maintenances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainMaintenancesRequest, String> {
    let mut input = ListDomainMaintenancesRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("action") {
        input.action = Some(value.to_string());
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_domain_names_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainNamesRequest, String> {
    let mut input = ListDomainNamesRequest::default();
    if let Some(value) = query.get("engineType") {
        input.engine_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_domains_for_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainsForPackageRequest, String> {
    let mut input = ListDomainsForPackageRequest::default();
    for (name, value) in labels {
        match *name {
            "PackageID" => {
                input.package_i_d = value.to_string();
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_instance_type_details_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInstanceTypeDetailsRequest, String> {
    let mut input = ListInstanceTypeDetailsRequest::default();
    for (name, value) in labels {
        match *name {
            "EngineVersion" => {
                input.engine_version = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainName") {
        input.domain_name = Some(value.to_string());
    }
    if let Some(value) = query.get("instanceType") {
        input.instance_type = Some(value.to_string());
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
    if let Some(value) = query.get("retrieveAZs") {
        input.retrieve_a_zs = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_packages_for_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPackagesForDomainRequest, String> {
    let mut input = ListPackagesForDomainRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
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
pub fn deserialize_list_scheduled_actions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListScheduledActionsRequest, String> {
    let mut input = ListScheduledActionsRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
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
pub fn deserialize_list_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsRequest, String> {
    let mut input = ListTagsRequest::default();
    if let Some(value) = query.get("arn") {
        input.a_r_n = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVersionsRequest, String> {
    let mut input = ListVersionsRequest::default();
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
pub fn deserialize_list_vpc_endpoint_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVpcEndpointAccessRequest, String> {
    let mut input = ListVpcEndpointAccessRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_vpc_endpoints_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVpcEndpointsRequest, String> {
    let mut input = ListVpcEndpointsRequest::default();
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_vpc_endpoints_for_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVpcEndpointsForDomainRequest, String> {
    let mut input = ListVpcEndpointsForDomainRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_purchase_reserved_instance_offering_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PurchaseReservedInstanceOfferingRequest, String> {
    let mut input = PurchaseReservedInstanceOfferingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PurchaseReservedInstanceOfferingRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PurchaseReservedInstanceOffering request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_default_application_setting_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDefaultApplicationSettingRequest, String> {
    let mut input = PutDefaultApplicationSettingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDefaultApplicationSettingRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutDefaultApplicationSetting request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_capability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterCapabilityRequest, String> {
    let mut input = RegisterCapabilityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterCapabilityRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterCapability request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "applicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_inbound_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectInboundConnectionRequest, String> {
    let mut input = RejectInboundConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectionId" => {
                input.connection_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveTagsRequest, String> {
    let mut input = RemoveTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RemoveTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RemoveTags request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_revoke_vpc_endpoint_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RevokeVpcEndpointAccessRequest, String> {
    let mut input = RevokeVpcEndpointAccessRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RevokeVpcEndpointAccessRequest>(&request.body).map_err(
            |err| format!("failed to deserialize RevokeVpcEndpointAccess request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_rollback_service_software_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RollbackServiceSoftwareUpdateRequest, String> {
    let mut input = RollbackServiceSoftwareUpdateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RollbackServiceSoftwareUpdateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize RollbackServiceSoftwareUpdate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_domain_maintenance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDomainMaintenanceRequest, String> {
    let mut input = StartDomainMaintenanceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDomainMaintenanceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartDomainMaintenance request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_service_software_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartServiceSoftwareUpdateRequest, String> {
    let mut input = StartServiceSoftwareUpdateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartServiceSoftwareUpdateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartServiceSoftwareUpdate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationRequest, String> {
    let mut input = UpdateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApplication request: {err}"))?;
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
pub fn deserialize_update_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataSourceRequest, String> {
    let mut input = UpdateDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_direct_query_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDirectQueryDataSourceRequest, String> {
    let mut input = UpdateDirectQueryDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDirectQueryDataSourceRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateDirectQueryDataSource request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DataSourceName" => {
                input.data_source_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_domain_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDomainConfigRequest, String> {
    let mut input = UpdateDomainConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDomainConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDomainConfig request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIndexRequest, String> {
    let mut input = UpdateIndexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIndexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateIndex request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "IndexName" => {
                input.index_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_package_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePackageRequest, String> {
    let mut input = UpdatePackageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePackageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePackage request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_package_scope_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePackageScopeRequest, String> {
    let mut input = UpdatePackageScopeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePackageScopeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePackageScope request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_scheduled_action_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateScheduledActionRequest, String> {
    let mut input = UpdateScheduledActionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateScheduledActionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateScheduledAction request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_vpc_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVpcEndpointRequest, String> {
    let mut input = UpdateVpcEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVpcEndpointRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVpcEndpoint request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_upgrade_domain_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpgradeDomainRequest, String> {
    let mut input = UpgradeDomainRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpgradeDomainRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpgradeDomain request: {err}"))?;
    }
    Ok(input)
}
