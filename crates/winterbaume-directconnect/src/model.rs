//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-directconnect

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptDirectConnectGatewayAssociationProposalRequest {
    #[serde(rename = "associatedGatewayOwnerAccount")]
    #[serde(default)]
    pub associated_gateway_owner_account: String,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    pub direct_connect_gateway_id: String,
    #[serde(rename = "overrideAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "proposalId")]
    #[serde(default)]
    pub proposal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteFilterPrefix {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptDirectConnectGatewayAssociationProposalResult {
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectConnectGatewayAssociation {
    #[serde(rename = "allowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "associatedCoreNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_core_network: Option<AssociatedCoreNetwork>,
    #[serde(rename = "associatedGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway: Option<AssociatedGateway>,
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "associationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_state: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "directConnectGatewayOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_owner_account: Option<String>,
    #[serde(rename = "stateChangeError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualGatewayOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_owner_account: Option<String>,
    #[serde(rename = "virtualGatewayRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedCoreNetwork {
    #[serde(rename = "attachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedGateway {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllocateConnectionOnInterconnectRequest {
    #[serde(default)]
    pub bandwidth: String,
    #[serde(rename = "connectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "interconnectId")]
    #[serde(default)]
    pub interconnect_id: String,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    pub owner_account: String,
    #[serde(default)]
    pub vlan: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllocateHostedConnectionRequest {
    #[serde(default)]
    pub bandwidth: String,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "connectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    pub owner_account: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub vlan: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllocatePrivateVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "newPrivateVirtualInterfaceAllocation")]
    #[serde(default)]
    pub new_private_virtual_interface_allocation: NewPrivateVirtualInterfaceAllocation,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    pub owner_account: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewPrivateVirtualInterfaceAllocation {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    pub virtual_interface_name: String,
    #[serde(default)]
    pub vlan: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllocatePublicVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "newPublicVirtualInterfaceAllocation")]
    #[serde(default)]
    pub new_public_virtual_interface_allocation: NewPublicVirtualInterfaceAllocation,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    pub owner_account: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewPublicVirtualInterfaceAllocation {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    pub virtual_interface_name: String,
    #[serde(default)]
    pub vlan: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllocateTransitVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "newTransitVirtualInterfaceAllocation")]
    #[serde(default)]
    pub new_transit_virtual_interface_allocation: NewTransitVirtualInterfaceAllocation,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    pub owner_account: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewTransitVirtualInterfaceAllocation {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllocateTransitVirtualInterfaceResult {
    #[serde(rename = "virtualInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualInterface {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename = "amazonSideAsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "awsDeviceV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "awsLogicalDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_logical_device_id: Option<String>,
    #[serde(rename = "bgpPeers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peers: Option<Vec<BGPPeer>>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "customerRouterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_router_config: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "jumboFrameCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "siteLinkEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_link_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
    #[serde(rename = "virtualInterfaceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
    #[serde(rename = "virtualInterfaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BGPPeer {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "awsDeviceV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "awsLogicalDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_logical_device_id: Option<String>,
    #[serde(rename = "bgpPeerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peer_id: Option<String>,
    #[serde(rename = "bgpPeerState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peer_state: Option<String>,
    #[serde(rename = "bgpStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_status: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConnectionWithLagRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "lagId")]
    #[serde(default)]
    pub lag_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateHostedConnectionRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "parentConnectionId")]
    #[serde(default)]
    pub parent_connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateMacSecKeyRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cak: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ckn: Option<String>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "secretARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateMacSecKeyResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "macSecKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_keys: Option<Vec<MacSecKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MacSecKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ckn: Option<String>,
    #[serde(rename = "secretARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_a_r_n: Option<String>,
    #[serde(rename = "startOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmConnectionRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmConnectionResponse {
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmCustomerAgreementRequest {
    #[serde(rename = "agreementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmCustomerAgreementResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmPrivateVirtualInterfaceRequest {
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmPrivateVirtualInterfaceResponse {
    #[serde(rename = "virtualInterfaceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmPublicVirtualInterfaceRequest {
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmPublicVirtualInterfaceResponse {
    #[serde(rename = "virtualInterfaceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmTransitVirtualInterfaceRequest {
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    pub direct_connect_gateway_id: String,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmTransitVirtualInterfaceResponse {
    #[serde(rename = "virtualInterfaceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "awsDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    #[serde(rename = "awsDeviceV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "awsLogicalDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_logical_device_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "encryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "hasLogicalRedundancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logical_redundancy: Option<String>,
    #[serde(rename = "jumboFrameCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    #[serde(rename = "lagId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(rename = "loaIssueTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_issue_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "macSecCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_capable: Option<bool>,
    #[serde(rename = "macSecKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_keys: Option<Vec<MacSecKey>>,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "partnerInterconnectMacSecCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_interconnect_mac_sec_capable: Option<bool>,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[serde(rename = "portEncryptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_encryption_status: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connections {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBGPPeerRequest {
    #[serde(rename = "newBGPPeer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_b_g_p_peer: Option<NewBGPPeer>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewBGPPeer {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBGPPeerResponse {
    #[serde(rename = "virtualInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionRequest {
    #[serde(default)]
    pub bandwidth: String,
    #[serde(rename = "connectionName")]
    #[serde(default)]
    pub connection_name: String,
    #[serde(rename = "lagId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(default)]
    pub location: String,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "requestMACSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_m_a_c_sec: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayAssociationProposalRequest {
    #[serde(rename = "addAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    pub direct_connect_gateway_id: String,
    #[serde(rename = "directConnectGatewayOwnerAccount")]
    #[serde(default)]
    pub direct_connect_gateway_owner_account: String,
    #[serde(rename = "gatewayId")]
    #[serde(default)]
    pub gateway_id: String,
    #[serde(rename = "removeAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayAssociationProposalResult {
    #[serde(rename = "directConnectGatewayAssociationProposal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association_proposal:
        Option<DirectConnectGatewayAssociationProposal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectConnectGatewayAssociationProposal {
    #[serde(rename = "associatedGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway: Option<AssociatedGateway>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "directConnectGatewayOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_owner_account: Option<String>,
    #[serde(rename = "existingAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "proposalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    #[serde(rename = "proposalState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_state: Option<String>,
    #[serde(rename = "requestedAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayAssociationRequest {
    #[serde(rename = "addAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    pub direct_connect_gateway_id: String,
    #[serde(rename = "gatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayAssociationResult {
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayRequest {
    #[serde(rename = "amazonSideAsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    #[serde(rename = "directConnectGatewayName")]
    #[serde(default)]
    pub direct_connect_gateway_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayResult {
    #[serde(rename = "directConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectConnectGateway {
    #[serde(rename = "amazonSideAsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "directConnectGatewayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_name: Option<String>,
    #[serde(rename = "directConnectGatewayState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_state: Option<String>,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "stateChangeError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInterconnectRequest {
    #[serde(default)]
    pub bandwidth: String,
    #[serde(rename = "interconnectName")]
    #[serde(default)]
    pub interconnect_name: String,
    #[serde(rename = "lagId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(default)]
    pub location: String,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "requestMACSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_m_a_c_sec: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLagRequest {
    #[serde(rename = "childConnectionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_connection_tags: Option<Vec<Tag>>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionsBandwidth")]
    #[serde(default)]
    pub connections_bandwidth: String,
    #[serde(rename = "lagName")]
    #[serde(default)]
    pub lag_name: String,
    #[serde(default)]
    pub location: String,
    #[serde(rename = "numberOfConnections")]
    #[serde(default)]
    pub number_of_connections: i32,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "requestMACSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_m_a_c_sec: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePrivateVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "newPrivateVirtualInterface")]
    #[serde(default)]
    pub new_private_virtual_interface: NewPrivateVirtualInterface,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewPrivateVirtualInterface {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "enableSiteLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_site_link: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    pub virtual_interface_name: String,
    #[serde(default)]
    pub vlan: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePublicVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "newPublicVirtualInterface")]
    #[serde(default)]
    pub new_public_virtual_interface: NewPublicVirtualInterface,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewPublicVirtualInterface {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    pub virtual_interface_name: String,
    #[serde(default)]
    pub vlan: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTransitVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "newTransitVirtualInterface")]
    #[serde(default)]
    pub new_transit_virtual_interface: NewTransitVirtualInterface,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewTransitVirtualInterface {
    #[serde(rename = "addressFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "enableSiteLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_site_link: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTransitVirtualInterfaceResult {
    #[serde(rename = "virtualInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBGPPeerRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "asnLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_long: Option<i64>,
    #[serde(rename = "bgpPeerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peer_id: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBGPPeerResponse {
    #[serde(rename = "virtualInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectConnectGatewayAssociationProposalRequest {
    #[serde(rename = "proposalId")]
    #[serde(default)]
    pub proposal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectConnectGatewayAssociationProposalResult {
    #[serde(rename = "directConnectGatewayAssociationProposal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association_proposal:
        Option<DirectConnectGatewayAssociationProposal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectConnectGatewayAssociationRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectConnectGatewayAssociationResult {
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectConnectGatewayRequest {
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    pub direct_connect_gateway_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectConnectGatewayResult {
    #[serde(rename = "directConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInterconnectRequest {
    #[serde(rename = "interconnectId")]
    #[serde(default)]
    pub interconnect_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInterconnectResponse {
    #[serde(rename = "interconnectState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLagRequest {
    #[serde(rename = "lagId")]
    #[serde(default)]
    pub lag_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualInterfaceRequest {
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVirtualInterfaceResponse {
    #[serde(rename = "virtualInterfaceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionLoaRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "loaContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionLoaResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Loa {
    #[serde(rename = "loaContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content: Option<String>,
    #[serde(rename = "loaContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionsOnInterconnectRequest {
    #[serde(rename = "interconnectId")]
    #[serde(default)]
    pub interconnect_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionsRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomerMetadataResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreements: Option<Vec<CustomerAgreement>>,
    #[serde(rename = "nniPartnerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nni_partner_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerAgreement {
    #[serde(rename = "agreementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewayAssociationProposalsRequest {
    #[serde(rename = "associatedGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway_id: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "proposalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewayAssociationProposalsResult {
    #[serde(rename = "directConnectGatewayAssociationProposals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association_proposals:
        Option<Vec<DirectConnectGatewayAssociationProposal>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewayAssociationsRequest {
    #[serde(rename = "associatedGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_gateway_id: Option<String>,
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewayAssociationsResult {
    #[serde(rename = "directConnectGatewayAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_associations: Option<Vec<DirectConnectGatewayAssociation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewayAttachmentsRequest {
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewayAttachmentsResult {
    #[serde(rename = "directConnectGatewayAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_attachments: Option<Vec<DirectConnectGatewayAttachment>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectConnectGatewayAttachment {
    #[serde(rename = "attachmentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_state: Option<String>,
    #[serde(rename = "attachmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "stateChangeError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
    #[serde(rename = "virtualInterfaceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_owner_account: Option<String>,
    #[serde(rename = "virtualInterfaceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewaysRequest {
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDirectConnectGatewaysResult {
    #[serde(rename = "directConnectGateways")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateways: Option<Vec<DirectConnectGateway>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHostedConnectionsRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInterconnectLoaRequest {
    #[serde(rename = "interconnectId")]
    #[serde(default)]
    pub interconnect_id: String,
    #[serde(rename = "loaContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInterconnectLoaResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInterconnectsRequest {
    #[serde(rename = "interconnectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLagsRequest {
    #[serde(rename = "lagId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLoaRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "loaContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRouterConfigurationRequest {
    #[serde(rename = "routerTypeIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_type_identifier: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRouterConfigurationResponse {
    #[serde(rename = "customerRouterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_router_config: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<RouterType>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "routerTypeIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_type_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "xsltTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xslt_template_name: Option<String>,
    #[serde(rename = "xsltTemplateNameForMacSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xslt_template_name_for_mac_sec: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsRequest {
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    pub resource_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagsResponse {
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVirtualInterfacesRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConnectionFromLagRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "lagId")]
    #[serde(default)]
    pub lag_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMacSecKeyRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "secretARN")]
    #[serde(default)]
    pub secret_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMacSecKeyResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "macSecKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_keys: Option<Vec<MacSecKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Interconnect {
    #[serde(rename = "awsDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    #[serde(rename = "awsDeviceV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "awsLogicalDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_logical_device_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    #[serde(rename = "encryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "hasLogicalRedundancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logical_redundancy: Option<String>,
    #[serde(rename = "interconnectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_id: Option<String>,
    #[serde(rename = "interconnectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_name: Option<String>,
    #[serde(rename = "interconnectState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_state: Option<String>,
    #[serde(rename = "jumboFrameCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    #[serde(rename = "lagId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(rename = "loaIssueTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_issue_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "macSecCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_capable: Option<bool>,
    #[serde(rename = "macSecKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_keys: Option<Vec<MacSecKey>>,
    #[serde(rename = "portEncryptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_encryption_status: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Interconnects {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnects: Option<Vec<Interconnect>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Lag {
    #[serde(rename = "allowsHostedConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_hosted_connections: Option<bool>,
    #[serde(rename = "awsDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    #[serde(rename = "awsDeviceV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "awsLogicalDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_logical_device_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    #[serde(rename = "connectionsBandwidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections_bandwidth: Option<String>,
    #[serde(rename = "encryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "hasLogicalRedundancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logical_redundancy: Option<String>,
    #[serde(rename = "jumboFrameCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumbo_frame_capable: Option<bool>,
    #[serde(rename = "lagId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(rename = "lagName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_name: Option<String>,
    #[serde(rename = "lagState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "macSecCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_capable: Option<bool>,
    #[serde(rename = "macSecKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sec_keys: Option<Vec<MacSecKey>>,
    #[serde(rename = "minimumLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_links: Option<i32>,
    #[serde(rename = "numberOfConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_connections: Option<i32>,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Lags {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lags: Option<Vec<Lag>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualInterfaceTestHistoryRequest {
    #[serde(rename = "bgpPeers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peers: Option<Vec<String>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "testId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVirtualInterfaceTestHistoryResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualInterfaceTestHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_test_history: Option<Vec<VirtualInterfaceTestHistory>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualInterfaceTestHistory {
    #[serde(rename = "bgpPeers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peers: Option<Vec<String>>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ownerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "testDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_duration_in_minutes: Option<i32>,
    #[serde(rename = "testId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Locations {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Location {
    #[serde(rename = "availableMacSecPortSpeeds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_mac_sec_port_speeds: Option<Vec<String>>,
    #[serde(rename = "availablePortSpeeds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_port_speeds: Option<Vec<String>>,
    #[serde(rename = "availableProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_providers: Option<Vec<String>>,
    #[serde(rename = "locationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    #[serde(rename = "locationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBgpFailoverTestRequest {
    #[serde(rename = "bgpPeers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peers: Option<Vec<String>>,
    #[serde(rename = "testDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_duration_in_minutes: Option<i32>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBgpFailoverTestResponse {
    #[serde(rename = "virtualInterfaceTest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_test: Option<VirtualInterfaceTestHistory>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBgpFailoverTestRequest {
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBgpFailoverTestResponse {
    #[serde(rename = "virtualInterfaceTest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_test: Option<VirtualInterfaceTestHistory>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "connectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "encryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectConnectGatewayAssociationRequest {
    #[serde(rename = "addAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "removeAllowedPrefixesToDirectConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_allowed_prefixes_to_direct_connect_gateway: Option<Vec<RouteFilterPrefix>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectConnectGatewayAssociationResult {
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectConnectGatewayRequest {
    #[serde(rename = "directConnectGatewayId")]
    #[serde(default)]
    pub direct_connect_gateway_id: String,
    #[serde(rename = "newDirectConnectGatewayName")]
    #[serde(default)]
    pub new_direct_connect_gateway_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectConnectGatewayResponse {
    #[serde(rename = "directConnectGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLagRequest {
    #[serde(rename = "encryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "lagId")]
    #[serde(default)]
    pub lag_id: String,
    #[serde(rename = "lagName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_name: Option<String>,
    #[serde(rename = "minimumLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_links: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVirtualInterfaceAttributesRequest {
    #[serde(rename = "enableSiteLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_site_link: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(default)]
    pub virtual_interface_id: String,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGateways {
    #[serde(rename = "virtualGateways")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateways: Option<Vec<VirtualGateway>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualGateway {
    #[serde(rename = "virtualGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualGatewayState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualInterfaces {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "virtualInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interfaces: Option<Vec<VirtualInterface>>,
}
