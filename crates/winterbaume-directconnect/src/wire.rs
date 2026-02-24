//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-directconnect

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_accept_direct_connect_gateway_association_proposal_response(
    result: &AcceptDirectConnectGatewayAssociationProposalResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_allocate_connection_on_interconnect_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_allocate_hosted_connection_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_allocate_private_virtual_interface_response(
    result: &VirtualInterface,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_allocate_public_virtual_interface_response(
    result: &VirtualInterface,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_allocate_transit_virtual_interface_response(
    result: &AllocateTransitVirtualInterfaceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_connection_with_lag_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_hosted_connection_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_mac_sec_key_response(
    result: &AssociateMacSecKeyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_virtual_interface_response(result: &VirtualInterface) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_connection_response(result: &ConfirmConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_customer_agreement_response(
    result: &ConfirmCustomerAgreementResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_private_virtual_interface_response(
    result: &ConfirmPrivateVirtualInterfaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_public_virtual_interface_response(
    result: &ConfirmPublicVirtualInterfaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_transit_virtual_interface_response(
    result: &ConfirmTransitVirtualInterfaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_b_g_p_peer_response(result: &CreateBGPPeerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_connection_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_direct_connect_gateway_response(
    result: &CreateDirectConnectGatewayResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_direct_connect_gateway_association_response(
    result: &CreateDirectConnectGatewayAssociationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_direct_connect_gateway_association_proposal_response(
    result: &CreateDirectConnectGatewayAssociationProposalResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_interconnect_response(result: &Interconnect) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_lag_response(result: &Lag) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_private_virtual_interface_response(
    result: &VirtualInterface,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_public_virtual_interface_response(
    result: &VirtualInterface,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_transit_virtual_interface_response(
    result: &CreateTransitVirtualInterfaceResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_b_g_p_peer_response(result: &DeleteBGPPeerResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_connection_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_direct_connect_gateway_response(
    result: &DeleteDirectConnectGatewayResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_direct_connect_gateway_association_response(
    result: &DeleteDirectConnectGatewayAssociationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_direct_connect_gateway_association_proposal_response(
    result: &DeleteDirectConnectGatewayAssociationProposalResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_interconnect_response(result: &DeleteInterconnectResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_lag_response(result: &Lag) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_virtual_interface_response(
    result: &DeleteVirtualInterfaceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connection_loa_response(
    result: &DescribeConnectionLoaResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connections_response(result: &Connections) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connections_on_interconnect_response(
    result: &Connections,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_customer_metadata_response(
    result: &DescribeCustomerMetadataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_direct_connect_gateway_association_proposals_response(
    result: &DescribeDirectConnectGatewayAssociationProposalsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_direct_connect_gateway_associations_response(
    result: &DescribeDirectConnectGatewayAssociationsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_direct_connect_gateway_attachments_response(
    result: &DescribeDirectConnectGatewayAttachmentsResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_direct_connect_gateways_response(
    result: &DescribeDirectConnectGatewaysResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_hosted_connections_response(result: &Connections) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_interconnect_loa_response(
    result: &DescribeInterconnectLoaResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_interconnects_response(result: &Interconnects) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_lags_response(result: &Lags) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_loa_response(result: &Loa) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_locations_response(result: &Locations) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_router_configuration_response(
    result: &DescribeRouterConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_tags_response(result: &DescribeTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_virtual_gateways_response(result: &VirtualGateways) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_virtual_interfaces_response(result: &VirtualInterfaces) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_connection_from_lag_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_mac_sec_key_response(
    result: &DisassociateMacSecKeyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_virtual_interface_test_history_response(
    result: &ListVirtualInterfaceTestHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_bgp_failover_test_response(
    result: &StartBgpFailoverTestResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_bgp_failover_test_response(
    result: &StopBgpFailoverTestResponse,
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
pub fn serialize_update_connection_response(result: &Connection) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_direct_connect_gateway_response(
    result: &UpdateDirectConnectGatewayResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_direct_connect_gateway_association_response(
    result: &UpdateDirectConnectGatewayAssociationResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_lag_response(result: &Lag) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_virtual_interface_attributes_response(
    result: &VirtualInterface,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_accept_direct_connect_gateway_association_proposal_request(
    body: &[u8],
) -> Result<AcceptDirectConnectGatewayAssociationProposalRequest, String> {
    if body.is_empty() {
        return Ok(AcceptDirectConnectGatewayAssociationProposalRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AcceptDirectConnectGatewayAssociationProposal request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_allocate_connection_on_interconnect_request(
    body: &[u8],
) -> Result<AllocateConnectionOnInterconnectRequest, String> {
    if body.is_empty() {
        return Ok(AllocateConnectionOnInterconnectRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AllocateConnectionOnInterconnect request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_allocate_hosted_connection_request(
    body: &[u8],
) -> Result<AllocateHostedConnectionRequest, String> {
    if body.is_empty() {
        return Ok(AllocateHostedConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AllocateHostedConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_allocate_private_virtual_interface_request(
    body: &[u8],
) -> Result<AllocatePrivateVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(AllocatePrivateVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AllocatePrivateVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_allocate_public_virtual_interface_request(
    body: &[u8],
) -> Result<AllocatePublicVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(AllocatePublicVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AllocatePublicVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_allocate_transit_virtual_interface_request(
    body: &[u8],
) -> Result<AllocateTransitVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(AllocateTransitVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AllocateTransitVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_connection_with_lag_request(
    body: &[u8],
) -> Result<AssociateConnectionWithLagRequest, String> {
    if body.is_empty() {
        return Ok(AssociateConnectionWithLagRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateConnectionWithLag request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_hosted_connection_request(
    body: &[u8],
) -> Result<AssociateHostedConnectionRequest, String> {
    if body.is_empty() {
        return Ok(AssociateHostedConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateHostedConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_mac_sec_key_request(
    body: &[u8],
) -> Result<AssociateMacSecKeyRequest, String> {
    if body.is_empty() {
        return Ok(AssociateMacSecKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateMacSecKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_virtual_interface_request(
    body: &[u8],
) -> Result<AssociateVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(AssociateVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_connection_request(
    body: &[u8],
) -> Result<ConfirmConnectionRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_customer_agreement_request(
    body: &[u8],
) -> Result<ConfirmCustomerAgreementRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmCustomerAgreementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmCustomerAgreement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_private_virtual_interface_request(
    body: &[u8],
) -> Result<ConfirmPrivateVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmPrivateVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmPrivateVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_public_virtual_interface_request(
    body: &[u8],
) -> Result<ConfirmPublicVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmPublicVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmPublicVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_transit_virtual_interface_request(
    body: &[u8],
) -> Result<ConfirmTransitVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmTransitVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmTransitVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_b_g_p_peer_request(body: &[u8]) -> Result<CreateBGPPeerRequest, String> {
    if body.is_empty() {
        return Ok(CreateBGPPeerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBGPPeer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_connection_request(
    body: &[u8],
) -> Result<CreateConnectionRequest, String> {
    if body.is_empty() {
        return Ok(CreateConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_direct_connect_gateway_request(
    body: &[u8],
) -> Result<CreateDirectConnectGatewayRequest, String> {
    if body.is_empty() {
        return Ok(CreateDirectConnectGatewayRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDirectConnectGateway request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_direct_connect_gateway_association_request(
    body: &[u8],
) -> Result<CreateDirectConnectGatewayAssociationRequest, String> {
    if body.is_empty() {
        return Ok(CreateDirectConnectGatewayAssociationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CreateDirectConnectGatewayAssociation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_direct_connect_gateway_association_proposal_request(
    body: &[u8],
) -> Result<CreateDirectConnectGatewayAssociationProposalRequest, String> {
    if body.is_empty() {
        return Ok(CreateDirectConnectGatewayAssociationProposalRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CreateDirectConnectGatewayAssociationProposal request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_interconnect_request(
    body: &[u8],
) -> Result<CreateInterconnectRequest, String> {
    if body.is_empty() {
        return Ok(CreateInterconnectRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateInterconnect request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_lag_request(body: &[u8]) -> Result<CreateLagRequest, String> {
    if body.is_empty() {
        return Ok(CreateLagRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLag request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_private_virtual_interface_request(
    body: &[u8],
) -> Result<CreatePrivateVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(CreatePrivateVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePrivateVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_public_virtual_interface_request(
    body: &[u8],
) -> Result<CreatePublicVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(CreatePublicVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePublicVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_transit_virtual_interface_request(
    body: &[u8],
) -> Result<CreateTransitVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(CreateTransitVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTransitVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_b_g_p_peer_request(body: &[u8]) -> Result<DeleteBGPPeerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteBGPPeerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBGPPeer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connection_request(
    body: &[u8],
) -> Result<DeleteConnectionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_direct_connect_gateway_request(
    body: &[u8],
) -> Result<DeleteDirectConnectGatewayRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDirectConnectGatewayRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDirectConnectGateway request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_direct_connect_gateway_association_request(
    body: &[u8],
) -> Result<DeleteDirectConnectGatewayAssociationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDirectConnectGatewayAssociationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteDirectConnectGatewayAssociation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_direct_connect_gateway_association_proposal_request(
    body: &[u8],
) -> Result<DeleteDirectConnectGatewayAssociationProposalRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDirectConnectGatewayAssociationProposalRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeleteDirectConnectGatewayAssociationProposal request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_interconnect_request(
    body: &[u8],
) -> Result<DeleteInterconnectRequest, String> {
    if body.is_empty() {
        return Ok(DeleteInterconnectRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteInterconnect request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_lag_request(body: &[u8]) -> Result<DeleteLagRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLagRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLag request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_virtual_interface_request(
    body: &[u8],
) -> Result<DeleteVirtualInterfaceRequest, String> {
    if body.is_empty() {
        return Ok(DeleteVirtualInterfaceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteVirtualInterface request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connection_loa_request(
    body: &[u8],
) -> Result<DescribeConnectionLoaRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionLoaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnectionLoa request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connections_request(
    body: &[u8],
) -> Result<DescribeConnectionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connections_on_interconnect_request(
    body: &[u8],
) -> Result<DescribeConnectionsOnInterconnectRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionsOnInterconnectRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeConnectionsOnInterconnect request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_direct_connect_gateway_association_proposals_request(
    body: &[u8],
) -> Result<DescribeDirectConnectGatewayAssociationProposalsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDirectConnectGatewayAssociationProposalsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize DescribeDirectConnectGatewayAssociationProposals request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_direct_connect_gateway_associations_request(
    body: &[u8],
) -> Result<DescribeDirectConnectGatewayAssociationsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDirectConnectGatewayAssociationsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeDirectConnectGatewayAssociations request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_direct_connect_gateway_attachments_request(
    body: &[u8],
) -> Result<DescribeDirectConnectGatewayAttachmentsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDirectConnectGatewayAttachmentsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeDirectConnectGatewayAttachments request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_direct_connect_gateways_request(
    body: &[u8],
) -> Result<DescribeDirectConnectGatewaysRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDirectConnectGatewaysRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDirectConnectGateways request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_hosted_connections_request(
    body: &[u8],
) -> Result<DescribeHostedConnectionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeHostedConnectionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeHostedConnections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_interconnect_loa_request(
    body: &[u8],
) -> Result<DescribeInterconnectLoaRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInterconnectLoaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInterconnectLoa request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_interconnects_request(
    body: &[u8],
) -> Result<DescribeInterconnectsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeInterconnectsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeInterconnects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_lags_request(body: &[u8]) -> Result<DescribeLagsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_loa_request(body: &[u8]) -> Result<DescribeLoaRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLoaRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLoa request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_router_configuration_request(
    body: &[u8],
) -> Result<DescribeRouterConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRouterConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRouterConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_tags_request(body: &[u8]) -> Result<DescribeTagsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_virtual_interfaces_request(
    body: &[u8],
) -> Result<DescribeVirtualInterfacesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeVirtualInterfacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeVirtualInterfaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_connection_from_lag_request(
    body: &[u8],
) -> Result<DisassociateConnectionFromLagRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateConnectionFromLagRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateConnectionFromLag request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_mac_sec_key_request(
    body: &[u8],
) -> Result<DisassociateMacSecKeyRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateMacSecKeyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateMacSecKey request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_virtual_interface_test_history_request(
    body: &[u8],
) -> Result<ListVirtualInterfaceTestHistoryRequest, String> {
    if body.is_empty() {
        return Ok(ListVirtualInterfaceTestHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListVirtualInterfaceTestHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_bgp_failover_test_request(
    body: &[u8],
) -> Result<StartBgpFailoverTestRequest, String> {
    if body.is_empty() {
        return Ok(StartBgpFailoverTestRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartBgpFailoverTest request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_bgp_failover_test_request(
    body: &[u8],
) -> Result<StopBgpFailoverTestRequest, String> {
    if body.is_empty() {
        return Ok(StopBgpFailoverTestRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopBgpFailoverTest request: {e}"))
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
pub fn deserialize_update_connection_request(
    body: &[u8],
) -> Result<UpdateConnectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_direct_connect_gateway_request(
    body: &[u8],
) -> Result<UpdateDirectConnectGatewayRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDirectConnectGatewayRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDirectConnectGateway request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_direct_connect_gateway_association_request(
    body: &[u8],
) -> Result<UpdateDirectConnectGatewayAssociationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDirectConnectGatewayAssociationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateDirectConnectGatewayAssociation request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_lag_request(body: &[u8]) -> Result<UpdateLagRequest, String> {
    if body.is_empty() {
        return Ok(UpdateLagRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateLag request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_virtual_interface_attributes_request(
    body: &[u8],
) -> Result<UpdateVirtualInterfaceAttributesRequest, String> {
    if body.is_empty() {
        return Ok(UpdateVirtualInterfaceAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateVirtualInterfaceAttributes request: {e}"))
}
