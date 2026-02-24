//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-networkmanager

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
pub fn serialize_accept_attachment_response(result: &AcceptAttachmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_connect_peer_response(
    result: &AssociateConnectPeerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_customer_gateway_response(
    result: &AssociateCustomerGatewayResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_link_response(result: &AssociateLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_transit_gateway_connect_peer_response(
    result: &AssociateTransitGatewayConnectPeerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_connect_attachment_response(
    result: &CreateConnectAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_connect_peer_response(result: &CreateConnectPeerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_connection_response(result: &CreateConnectionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_core_network_response(result: &CreateCoreNetworkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_core_network_prefix_list_association_response(
    result: &CreateCoreNetworkPrefixListAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_device_response(result: &CreateDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_direct_connect_gateway_attachment_response(
    result: &CreateDirectConnectGatewayAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_global_network_response(
    result: &CreateGlobalNetworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_link_response(result: &CreateLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_site_response(result: &CreateSiteResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_site_to_site_vpn_attachment_response(
    result: &CreateSiteToSiteVpnAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_transit_gateway_peering_response(
    result: &CreateTransitGatewayPeeringResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_transit_gateway_route_table_attachment_response(
    result: &CreateTransitGatewayRouteTableAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vpc_attachment_response(
    result: &CreateVpcAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_attachment_response(result: &DeleteAttachmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_connect_peer_response(result: &DeleteConnectPeerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_connection_response(result: &DeleteConnectionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_core_network_response(result: &DeleteCoreNetworkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_core_network_policy_version_response(
    result: &DeleteCoreNetworkPolicyVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_core_network_prefix_list_association_response(
    result: &DeleteCoreNetworkPrefixListAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_device_response(result: &DeleteDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_global_network_response(
    result: &DeleteGlobalNetworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_link_response(result: &DeleteLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_peering_response(result: &DeletePeeringResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_site_response(result: &DeleteSiteResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_transit_gateway_response(
    result: &DeregisterTransitGatewayResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_global_networks_response(
    result: &DescribeGlobalNetworksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_connect_peer_response(
    result: &DisassociateConnectPeerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_customer_gateway_response(
    result: &DisassociateCustomerGatewayResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_link_response(result: &DisassociateLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_transit_gateway_connect_peer_response(
    result: &DisassociateTransitGatewayConnectPeerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_execute_core_network_change_set_response(
    result: &ExecuteCoreNetworkChangeSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connect_attachment_response(
    result: &GetConnectAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connect_peer_response(result: &GetConnectPeerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connect_peer_associations_response(
    result: &GetConnectPeerAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connections_response(result: &GetConnectionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_core_network_response(result: &GetCoreNetworkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_core_network_change_events_response(
    result: &GetCoreNetworkChangeEventsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_core_network_change_set_response(
    result: &GetCoreNetworkChangeSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_core_network_policy_response(
    result: &GetCoreNetworkPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_customer_gateway_associations_response(
    result: &GetCustomerGatewayAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_devices_response(result: &GetDevicesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_direct_connect_gateway_attachment_response(
    result: &GetDirectConnectGatewayAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_link_associations_response(
    result: &GetLinkAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_links_response(result: &GetLinksResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_network_resource_counts_response(
    result: &GetNetworkResourceCountsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_network_resource_relationships_response(
    result: &GetNetworkResourceRelationshipsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_network_resources_response(
    result: &GetNetworkResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_network_routes_response(result: &GetNetworkRoutesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_network_telemetry_response(
    result: &GetNetworkTelemetryResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_route_analysis_response(result: &GetRouteAnalysisResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_site_to_site_vpn_attachment_response(
    result: &GetSiteToSiteVpnAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sites_response(result: &GetSitesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_transit_gateway_connect_peer_associations_response(
    result: &GetTransitGatewayConnectPeerAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_transit_gateway_peering_response(
    result: &GetTransitGatewayPeeringResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_transit_gateway_registrations_response(
    result: &GetTransitGatewayRegistrationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_transit_gateway_route_table_attachment_response(
    result: &GetTransitGatewayRouteTableAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vpc_attachment_response(result: &GetVpcAttachmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attachment_routing_policy_associations_response(
    result: &ListAttachmentRoutingPolicyAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attachments_response(result: &ListAttachmentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_connect_peers_response(result: &ListConnectPeersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_core_network_policy_versions_response(
    result: &ListCoreNetworkPolicyVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_core_network_prefix_list_associations_response(
    result: &ListCoreNetworkPrefixListAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_core_network_routing_information_response(
    result: &ListCoreNetworkRoutingInformationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_core_networks_response(result: &ListCoreNetworksResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_organization_service_access_status_response(
    result: &ListOrganizationServiceAccessStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_peerings_response(result: &ListPeeringsResponse) -> MockResponse {
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
pub fn serialize_put_attachment_routing_policy_label_response(
    result: &PutAttachmentRoutingPolicyLabelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_core_network_policy_response(
    result: &PutCoreNetworkPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_transit_gateway_response(
    result: &RegisterTransitGatewayResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reject_attachment_response(result: &RejectAttachmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_attachment_routing_policy_label_response(
    result: &RemoveAttachmentRoutingPolicyLabelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_restore_core_network_policy_version_response(
    result: &RestoreCoreNetworkPolicyVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_organization_service_access_update_response(
    result: &StartOrganizationServiceAccessUpdateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_route_analysis_response(
    result: &StartRouteAnalysisResponse,
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
pub fn serialize_update_connection_response(result: &UpdateConnectionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_core_network_response(result: &UpdateCoreNetworkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_device_response(result: &UpdateDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_direct_connect_gateway_attachment_response(
    result: &UpdateDirectConnectGatewayAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_global_network_response(
    result: &UpdateGlobalNetworkResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_link_response(result: &UpdateLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_network_resource_metadata_response(
    result: &UpdateNetworkResourceMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_site_response(result: &UpdateSiteResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_vpc_attachment_response(
    result: &UpdateVpcAttachmentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptAttachmentRequest, String> {
    let mut input = AcceptAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_connect_peer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateConnectPeerRequest, String> {
    let mut input = AssociateConnectPeerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateConnectPeerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateConnectPeer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_customer_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateCustomerGatewayRequest, String> {
    let mut input = AssociateCustomerGatewayRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateCustomerGatewayRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateCustomerGateway request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateLinkRequest, String> {
    let mut input = AssociateLinkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateLinkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateLink request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_transit_gateway_connect_peer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateTransitGatewayConnectPeerRequest, String> {
    let mut input = AssociateTransitGatewayConnectPeerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateTransitGatewayConnectPeerRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateTransitGatewayConnectPeer request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_connect_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectAttachmentRequest, String> {
    let mut input = CreateConnectAttachmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConnectAttachmentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateConnectAttachment request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_connect_peer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectPeerRequest, String> {
    let mut input = CreateConnectPeerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConnectPeerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateConnectPeer request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectionRequest, String> {
    let mut input = CreateConnectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConnectionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateConnection request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_core_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCoreNetworkRequest, String> {
    let mut input = CreateCoreNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCoreNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCoreNetwork request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_core_network_prefix_list_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCoreNetworkPrefixListAssociationRequest, String> {
    let mut input = CreateCoreNetworkPrefixListAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCoreNetworkPrefixListAssociationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize CreateCoreNetworkPrefixListAssociation request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeviceRequest, String> {
    let mut input = CreateDeviceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeviceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDevice request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_direct_connect_gateway_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDirectConnectGatewayAttachmentRequest, String> {
    let mut input = CreateDirectConnectGatewayAttachmentRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<CreateDirectConnectGatewayAttachmentRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize CreateDirectConnectGatewayAttachment request: {err}"
                    )
                })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_global_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGlobalNetworkRequest, String> {
    let mut input = CreateGlobalNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGlobalNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGlobalNetwork request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLinkRequest, String> {
    let mut input = CreateLinkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLinkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateLink request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_site_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSiteRequest, String> {
    let mut input = CreateSiteRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSiteRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSite request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_site_to_site_vpn_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSiteToSiteVpnAttachmentRequest, String> {
    let mut input = CreateSiteToSiteVpnAttachmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSiteToSiteVpnAttachmentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateSiteToSiteVpnAttachment request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_transit_gateway_peering_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTransitGatewayPeeringRequest, String> {
    let mut input = CreateTransitGatewayPeeringRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTransitGatewayPeeringRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateTransitGatewayPeering request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_transit_gateway_route_table_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTransitGatewayRouteTableAttachmentRequest, String> {
    let mut input = CreateTransitGatewayRouteTableAttachmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTransitGatewayRouteTableAttachmentRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize CreateTransitGatewayRouteTableAttachment request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_vpc_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVpcAttachmentRequest, String> {
    let mut input = CreateVpcAttachmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVpcAttachmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVpcAttachment request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAttachmentRequest, String> {
    let mut input = DeleteAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_connect_peer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectPeerRequest, String> {
    let mut input = DeleteConnectPeerRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectPeerId" => {
                input.connect_peer_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectionRequest, String> {
    let mut input = DeleteConnectionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectionId" => {
                input.connection_id = value.to_string();
            }
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_core_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCoreNetworkRequest, String> {
    let mut input = DeleteCoreNetworkRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_core_network_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCoreNetworkPolicyVersionRequest, String> {
    let mut input = DeleteCoreNetworkPolicyVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            "PolicyVersionId" => {
                input.policy_version_id = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_core_network_prefix_list_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCoreNetworkPrefixListAssociationRequest, String> {
    let mut input = DeleteCoreNetworkPrefixListAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            "PrefixListArn" => {
                input.prefix_list_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDeviceRequest, String> {
    let mut input = DeleteDeviceRequest::default();
    for (name, value) in labels {
        match *name {
            "DeviceId" => {
                input.device_id = value.to_string();
            }
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_global_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGlobalNetworkRequest, String> {
    let mut input = DeleteGlobalNetworkRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLinkRequest, String> {
    let mut input = DeleteLinkRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "LinkId" => {
                input.link_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_peering_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePeeringRequest, String> {
    let mut input = DeletePeeringRequest::default();
    for (name, value) in labels {
        match *name {
            "PeeringId" => {
                input.peering_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyRequest, String> {
    let mut input = DeleteResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_site_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSiteRequest, String> {
    let mut input = DeleteSiteRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "SiteId" => {
                input.site_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_transit_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterTransitGatewayRequest, String> {
    let mut input = DeregisterTransitGatewayRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "TransitGatewayArn" => {
                input.transit_gateway_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_global_networks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeGlobalNetworksRequest, String> {
    let mut input = DescribeGlobalNetworksRequest::default();
    if let Some(value) = query.get("globalNetworkIds") {
        input.global_network_ids = Some(
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
pub fn deserialize_disassociate_connect_peer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateConnectPeerRequest, String> {
    let mut input = DisassociateConnectPeerRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectPeerId" => {
                input.connect_peer_id = value.to_string();
            }
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_customer_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateCustomerGatewayRequest, String> {
    let mut input = DisassociateCustomerGatewayRequest::default();
    for (name, value) in labels {
        match *name {
            "CustomerGatewayArn" => {
                input.customer_gateway_arn = value.to_string();
            }
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateLinkRequest, String> {
    let mut input = DisassociateLinkRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("deviceId") {
        input.device_id = value.to_string();
    }
    if let Some(value) = query.get("linkId") {
        input.link_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_transit_gateway_connect_peer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateTransitGatewayConnectPeerRequest, String> {
    let mut input = DisassociateTransitGatewayConnectPeerRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "TransitGatewayConnectPeerArn" => {
                input.transit_gateway_connect_peer_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_execute_core_network_change_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExecuteCoreNetworkChangeSetRequest, String> {
    let mut input = ExecuteCoreNetworkChangeSetRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            "PolicyVersionId" => {
                input.policy_version_id = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connect_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectAttachmentRequest, String> {
    let mut input = GetConnectAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connect_peer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectPeerRequest, String> {
    let mut input = GetConnectPeerRequest::default();
    for (name, value) in labels {
        match *name {
            "ConnectPeerId" => {
                input.connect_peer_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connect_peer_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectPeerAssociationsRequest, String> {
    let mut input = GetConnectPeerAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("connectPeerIds") {
        input.connect_peer_ids = Some(
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
pub fn deserialize_get_connections_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectionsRequest, String> {
    let mut input = GetConnectionsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("connectionIds") {
        input.connection_ids = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("deviceId") {
        input.device_id = Some(value.to_string());
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
pub fn deserialize_get_core_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCoreNetworkRequest, String> {
    let mut input = GetCoreNetworkRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_core_network_change_events_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCoreNetworkChangeEventsRequest, String> {
    let mut input = GetCoreNetworkChangeEventsRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            "PolicyVersionId" => {
                input.policy_version_id = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
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
pub fn deserialize_get_core_network_change_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCoreNetworkChangeSetRequest, String> {
    let mut input = GetCoreNetworkChangeSetRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            "PolicyVersionId" => {
                input.policy_version_id = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
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
pub fn deserialize_get_core_network_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCoreNetworkPolicyRequest, String> {
    let mut input = GetCoreNetworkPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("alias") {
        input.alias = Some(value.to_string());
    }
    if let Some(value) = query.get("policyVersionId") {
        input.policy_version_id = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_customer_gateway_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCustomerGatewayAssociationsRequest, String> {
    let mut input = GetCustomerGatewayAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("customerGatewayArns") {
        input.customer_gateway_arns = Some(
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
pub fn deserialize_get_devices_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDevicesRequest, String> {
    let mut input = GetDevicesRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("deviceIds") {
        input.device_ids = Some(
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
    if let Some(value) = query.get("siteId") {
        input.site_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_direct_connect_gateway_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDirectConnectGatewayAttachmentRequest, String> {
    let mut input = GetDirectConnectGatewayAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_link_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLinkAssociationsRequest, String> {
    let mut input = GetLinkAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("deviceId") {
        input.device_id = Some(value.to_string());
    }
    if let Some(value) = query.get("linkId") {
        input.link_id = Some(value.to_string());
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
pub fn deserialize_get_links_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLinksRequest, String> {
    let mut input = GetLinksRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("linkIds") {
        input.link_ids = Some(
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
    if let Some(value) = query.get("provider") {
        input.provider = Some(value.to_string());
    }
    if let Some(value) = query.get("siteId") {
        input.site_id = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_network_resource_counts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetNetworkResourceCountsRequest, String> {
    let mut input = GetNetworkResourceCountsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
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
    if let Some(value) = query.get("resourceType") {
        input.resource_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_network_resource_relationships_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetNetworkResourceRelationshipsRequest, String> {
    let mut input = GetNetworkResourceRelationshipsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("accountId") {
        input.account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("awsRegion") {
        input.aws_region = Some(value.to_string());
    }
    if let Some(value) = query.get("coreNetworkId") {
        input.core_network_id = Some(value.to_string());
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
    if let Some(value) = query.get("registeredGatewayArn") {
        input.registered_gateway_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_network_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetNetworkResourcesRequest, String> {
    let mut input = GetNetworkResourcesRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("accountId") {
        input.account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("awsRegion") {
        input.aws_region = Some(value.to_string());
    }
    if let Some(value) = query.get("coreNetworkId") {
        input.core_network_id = Some(value.to_string());
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
    if let Some(value) = query.get("registeredGatewayArn") {
        input.registered_gateway_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_network_routes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetNetworkRoutesRequest, String> {
    let mut input = GetNetworkRoutesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetNetworkRoutesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetNetworkRoutes request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_network_telemetry_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetNetworkTelemetryRequest, String> {
    let mut input = GetNetworkTelemetryRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("accountId") {
        input.account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("awsRegion") {
        input.aws_region = Some(value.to_string());
    }
    if let Some(value) = query.get("coreNetworkId") {
        input.core_network_id = Some(value.to_string());
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
    if let Some(value) = query.get("registeredGatewayArn") {
        input.registered_gateway_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcePolicyRequest, String> {
    let mut input = GetResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_route_analysis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouteAnalysisRequest, String> {
    let mut input = GetRouteAnalysisRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "RouteAnalysisId" => {
                input.route_analysis_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_site_to_site_vpn_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSiteToSiteVpnAttachmentRequest, String> {
    let mut input = GetSiteToSiteVpnAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sites_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSitesRequest, String> {
    let mut input = GetSitesRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
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
    if let Some(value) = query.get("siteIds") {
        input.site_ids = Some(
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
pub fn deserialize_get_transit_gateway_connect_peer_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTransitGatewayConnectPeerAssociationsRequest, String> {
    let mut input = GetTransitGatewayConnectPeerAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
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
    if let Some(value) = query.get("transitGatewayConnectPeerArns") {
        input.transit_gateway_connect_peer_arns = Some(
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
pub fn deserialize_get_transit_gateway_peering_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTransitGatewayPeeringRequest, String> {
    let mut input = GetTransitGatewayPeeringRequest::default();
    for (name, value) in labels {
        match *name {
            "PeeringId" => {
                input.peering_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_transit_gateway_registrations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTransitGatewayRegistrationsRequest, String> {
    let mut input = GetTransitGatewayRegistrationsRequest::default();
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
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
    if let Some(value) = query.get("transitGatewayArns") {
        input.transit_gateway_arns = Some(
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
pub fn deserialize_get_transit_gateway_route_table_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTransitGatewayRouteTableAttachmentRequest, String> {
    let mut input = GetTransitGatewayRouteTableAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_vpc_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetVpcAttachmentRequest, String> {
    let mut input = GetVpcAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_attachment_routing_policy_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttachmentRoutingPolicyAssociationsRequest, String> {
    let mut input = ListAttachmentRoutingPolicyAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("attachmentId") {
        input.attachment_id = Some(value.to_string());
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
pub fn deserialize_list_attachments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttachmentsRequest, String> {
    let mut input = ListAttachmentsRequest::default();
    if let Some(value) = query.get("attachmentType") {
        input.attachment_type = Some(value.to_string());
    }
    if let Some(value) = query.get("coreNetworkId") {
        input.core_network_id = Some(value.to_string());
    }
    if let Some(value) = query.get("edgeLocation") {
        input.edge_location = Some(value.to_string());
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
    if let Some(value) = query.get("state") {
        input.state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_connect_peers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectPeersRequest, String> {
    let mut input = ListConnectPeersRequest::default();
    if let Some(value) = query.get("connectAttachmentId") {
        input.connect_attachment_id = Some(value.to_string());
    }
    if let Some(value) = query.get("coreNetworkId") {
        input.core_network_id = Some(value.to_string());
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
pub fn deserialize_list_core_network_policy_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoreNetworkPolicyVersionsRequest, String> {
    let mut input = ListCoreNetworkPolicyVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
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
pub fn deserialize_list_core_network_prefix_list_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoreNetworkPrefixListAssociationsRequest, String> {
    let mut input = ListCoreNetworkPrefixListAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
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
    if let Some(value) = query.get("prefixListArn") {
        input.prefix_list_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_core_network_routing_information_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoreNetworkRoutingInformationRequest, String> {
    let mut input = ListCoreNetworkRoutingInformationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCoreNetworkRoutingInformationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize ListCoreNetworkRoutingInformation request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
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
pub fn deserialize_list_core_networks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCoreNetworksRequest, String> {
    let mut input = ListCoreNetworksRequest::default();
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
pub fn deserialize_list_organization_service_access_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOrganizationServiceAccessStatusRequest, String> {
    let mut input = ListOrganizationServiceAccessStatusRequest::default();
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
pub fn deserialize_list_peerings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPeeringsRequest, String> {
    let mut input = ListPeeringsRequest::default();
    if let Some(value) = query.get("coreNetworkId") {
        input.core_network_id = Some(value.to_string());
    }
    if let Some(value) = query.get("edgeLocation") {
        input.edge_location = Some(value.to_string());
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
    if let Some(value) = query.get("peeringType") {
        input.peering_type = Some(value.to_string());
    }
    if let Some(value) = query.get("state") {
        input.state = Some(value.to_string());
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_attachment_routing_policy_label_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAttachmentRoutingPolicyLabelRequest, String> {
    let mut input = PutAttachmentRoutingPolicyLabelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAttachmentRoutingPolicyLabelRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutAttachmentRoutingPolicyLabel request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_core_network_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutCoreNetworkPolicyRequest, String> {
    let mut input = PutCoreNetworkPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutCoreNetworkPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutCoreNetworkPolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutResourcePolicyRequest, String> {
    let mut input = PutResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutResourcePolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_transit_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterTransitGatewayRequest, String> {
    let mut input = RegisterTransitGatewayRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterTransitGatewayRequest>(&request.body).map_err(
            |err| format!("failed to deserialize RegisterTransitGateway request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectAttachmentRequest, String> {
    let mut input = RejectAttachmentRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_attachment_routing_policy_label_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveAttachmentRoutingPolicyLabelRequest, String> {
    let mut input = RemoveAttachmentRoutingPolicyLabelRequest::default();
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_restore_core_network_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RestoreCoreNetworkPolicyVersionRequest, String> {
    let mut input = RestoreCoreNetworkPolicyVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            "PolicyVersionId" => {
                input.policy_version_id = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_organization_service_access_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartOrganizationServiceAccessUpdateRequest, String> {
    let mut input = StartOrganizationServiceAccessUpdateRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<StartOrganizationServiceAccessUpdateRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize StartOrganizationServiceAccessUpdate request: {err}"
                    )
                })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_route_analysis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartRouteAnalysisRequest, String> {
    let mut input = StartRouteAnalysisRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartRouteAnalysisRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartRouteAnalysis request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
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
            "ResourceArn" => {
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
            "ResourceArn" => {
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
pub fn deserialize_update_connection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectionRequest, String> {
    let mut input = UpdateConnectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConnectionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateConnection request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ConnectionId" => {
                input.connection_id = value.to_string();
            }
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_core_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCoreNetworkRequest, String> {
    let mut input = UpdateCoreNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCoreNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCoreNetwork request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "CoreNetworkId" => {
                input.core_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDeviceRequest, String> {
    let mut input = UpdateDeviceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDeviceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDevice request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DeviceId" => {
                input.device_id = value.to_string();
            }
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_direct_connect_gateway_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDirectConnectGatewayAttachmentRequest, String> {
    let mut input = UpdateDirectConnectGatewayAttachmentRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<UpdateDirectConnectGatewayAttachmentRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize UpdateDirectConnectGatewayAttachment request: {err}"
                    )
                })?;
    }
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_global_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGlobalNetworkRequest, String> {
    let mut input = UpdateGlobalNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGlobalNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGlobalNetwork request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateLinkRequest, String> {
    let mut input = UpdateLinkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateLinkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateLink request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "LinkId" => {
                input.link_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_network_resource_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNetworkResourceMetadataRequest, String> {
    let mut input = UpdateNetworkResourceMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNetworkResourceMetadataRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateNetworkResourceMetadata request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_site_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSiteRequest, String> {
    let mut input = UpdateSiteRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSiteRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSite request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GlobalNetworkId" => {
                input.global_network_id = value.to_string();
            }
            "SiteId" => {
                input.site_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_vpc_attachment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVpcAttachmentRequest, String> {
    let mut input = UpdateVpcAttachmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVpcAttachmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVpcAttachment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AttachmentId" => {
                input.attachment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
