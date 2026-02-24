//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-networkmanager

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAttachmentResponse {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_policy_rule_number: Option<i32>,
    #[serde(rename = "AttachmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<String>,
    #[serde(rename = "CoreNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_arn: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "EdgeLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_locations: Option<Vec<String>>,
    #[serde(rename = "LastModificationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_errors: Option<Vec<AttachmentError>>,
    #[serde(rename = "NetworkFunctionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_group_name: Option<String>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "ProposedNetworkFunctionGroupChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_network_function_group_change: Option<ProposedNetworkFunctionGroupChange>,
    #[serde(rename = "ProposedSegmentChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_segment_change: Option<ProposedSegmentChange>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProposedNetworkFunctionGroupChange {
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_policy_rule_number: Option<i32>,
    #[serde(rename = "NetworkFunctionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_group_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProposedSegmentChange {
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_policy_rule_number: Option<i32>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConnectPeerRequest {
    #[serde(rename = "ConnectPeerId")]
    #[serde(default)]
    pub connect_peer_id: String,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConnectPeerResponse {
    #[serde(rename = "ConnectPeerAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_association: Option<ConnectPeerAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectPeerAssociation {
    #[serde(rename = "ConnectPeerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_id: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateCustomerGatewayRequest {
    #[serde(rename = "CustomerGatewayArn")]
    #[serde(default)]
    pub customer_gateway_arn: String,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateCustomerGatewayResponse {
    #[serde(rename = "CustomerGatewayAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_association: Option<CustomerGatewayAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerGatewayAssociation {
    #[serde(rename = "CustomerGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_arn: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateLinkRequest {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    pub link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateLinkResponse {
    #[serde(rename = "LinkAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_association: Option<LinkAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinkAssociation {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "LinkAssociationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_association_state: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTransitGatewayConnectPeerRequest {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "TransitGatewayConnectPeerArn")]
    #[serde(default)]
    pub transit_gateway_connect_peer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTransitGatewayConnectPeerResponse {
    #[serde(rename = "TransitGatewayConnectPeerAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_association: Option<TransitGatewayConnectPeerAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitGatewayConnectPeerAssociation {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TransitGatewayConnectPeerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectAttachmentRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    pub edge_location: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    pub options: ConnectAttachmentOptions,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TransportAttachmentId")]
    #[serde(default)]
    pub transport_attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectAttachmentOptions {
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectAttachmentResponse {
    #[serde(rename = "ConnectAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_attachment: Option<ConnectAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectAttachment {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ConnectAttachmentOptions>,
    #[serde(rename = "TransportAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_attachment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectPeerRequest {
    #[serde(rename = "BgpOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_options: Option<BgpOptions>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ConnectAttachmentId")]
    #[serde(default)]
    pub connect_attachment_id: String,
    #[serde(rename = "CoreNetworkAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_address: Option<String>,
    #[serde(rename = "InsideCidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_cidr_blocks: Option<Vec<String>>,
    #[serde(rename = "PeerAddress")]
    #[serde(default)]
    pub peer_address: String,
    #[serde(rename = "SubnetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BgpOptions {
    #[serde(rename = "PeerAsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectPeerResponse {
    #[serde(rename = "ConnectPeer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer: Option<ConnectPeer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectPeer {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConnectPeerConfiguration>,
    #[serde(rename = "ConnectAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_attachment_id: Option<String>,
    #[serde(rename = "ConnectPeerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_id: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "LastModificationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_errors: Option<Vec<ConnectPeerError>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "SubnetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectPeerConfiguration {
    #[serde(rename = "BgpConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_configurations: Option<Vec<ConnectPeerBgpConfiguration>>,
    #[serde(rename = "CoreNetworkAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_address: Option<String>,
    #[serde(rename = "InsideCidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_cidr_blocks: Option<Vec<String>>,
    #[serde(rename = "PeerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_address: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectPeerBgpConfiguration {
    #[serde(rename = "CoreNetworkAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_address: Option<String>,
    #[serde(rename = "CoreNetworkAsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_asn: Option<i64>,
    #[serde(rename = "PeerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_address: Option<String>,
    #[serde(rename = "PeerAsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectPeerError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionRequest {
    #[serde(rename = "ConnectedDeviceId")]
    #[serde(default)]
    pub connected_device_id: String,
    #[serde(rename = "ConnectedLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_link_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "ConnectedDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_device_id: Option<String>,
    #[serde(rename = "ConnectedLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_link_id: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreNetworkPrefixListAssociationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "PrefixListAlias")]
    #[serde(default)]
    pub prefix_list_alias: String,
    #[serde(rename = "PrefixListArn")]
    #[serde(default)]
    pub prefix_list_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreNetworkPrefixListAssociationResponse {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "PrefixListAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_alias: Option<String>,
    #[serde(rename = "PrefixListArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreNetworkRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCoreNetworkResponse {
    #[serde(rename = "CoreNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network: Option<CoreNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetwork {
    #[serde(rename = "CoreNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_arn: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Edges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<CoreNetworkEdge>>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "NetworkFunctionGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_groups: Option<Vec<CoreNetworkNetworkFunctionGroup>>,
    #[serde(rename = "Segments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<CoreNetworkSegment>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkEdge {
    #[serde(rename = "Asn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "InsideCidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_cidr_blocks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkNetworkFunctionGroup {
    #[serde(rename = "EdgeLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_locations: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Segments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<ServiceInsertionSegments>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceInsertionSegments {
    #[serde(rename = "SendTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to: Option<Vec<String>>,
    #[serde(rename = "SendVia")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_via: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkSegment {
    #[serde(rename = "EdgeLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_locations: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SharedSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_segments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeviceRequest {
    #[serde(rename = "AWSLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_location: Option<AWSLocation>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "Model")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Vendor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSLocation {
    #[serde(rename = "SubnetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arn: Option<String>,
    #[serde(rename = "Zone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Location {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Latitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(rename = "Longitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeviceResponse {
    #[serde(rename = "Device")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "AWSLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_location: Option<AWSLocation>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DeviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "Model")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Vendor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayAttachmentRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "DirectConnectGatewayArn")]
    #[serde(default)]
    pub direct_connect_gateway_arn: String,
    #[serde(rename = "EdgeLocations")]
    #[serde(default)]
    pub edge_locations: Vec<String>,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectConnectGatewayAttachmentResponse {
    #[serde(rename = "DirectConnectGatewayAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_attachment: Option<DirectConnectGatewayAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectConnectGatewayAttachment {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
    #[serde(rename = "DirectConnectGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlobalNetworkRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGlobalNetworkResponse {
    #[serde(rename = "GlobalNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network: Option<GlobalNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalNetwork {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_arn: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLinkRequest {
    #[serde(rename = "Bandwidth")]
    #[serde(default)]
    pub bandwidth: Bandwidth,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Bandwidth {
    #[serde(rename = "DownloadSpeed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_speed: Option<i32>,
    #[serde(rename = "UploadSpeed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_speed: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLinkResponse {
    #[serde(rename = "Link")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "Bandwidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Bandwidth>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "LinkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_arn: Option<String>,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSiteRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSiteResponse {
    #[serde(rename = "Site")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Site {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "SiteArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_arn: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSiteToSiteVpnAttachmentRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpnConnectionArn")]
    #[serde(default)]
    pub vpn_connection_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSiteToSiteVpnAttachmentResponse {
    #[serde(rename = "SiteToSiteVpnAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_to_site_vpn_attachment: Option<SiteToSiteVpnAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SiteToSiteVpnAttachment {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
    #[serde(rename = "VpnConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTransitGatewayPeeringRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TransitGatewayArn")]
    #[serde(default)]
    pub transit_gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTransitGatewayPeeringResponse {
    #[serde(rename = "TransitGatewayPeering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_peering: Option<TransitGatewayPeering>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitGatewayPeering {
    #[serde(rename = "Peering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering: Option<Peering>,
    #[serde(rename = "TransitGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_arn: Option<String>,
    #[serde(rename = "TransitGatewayPeeringAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_peering_attachment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Peering {
    #[serde(rename = "CoreNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_arn: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "LastModificationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_errors: Option<Vec<PeeringError>>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "PeeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_id: Option<String>,
    #[serde(rename = "PeeringType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_type: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PeeringError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "MissingPermissionsContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_permissions_context: Option<PermissionsErrorContext>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionsErrorContext {
    #[serde(rename = "MissingPermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTransitGatewayRouteTableAttachmentRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "PeeringId")]
    #[serde(default)]
    pub peering_id: String,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TransitGatewayRouteTableArn")]
    #[serde(default)]
    pub transit_gateway_route_table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTransitGatewayRouteTableAttachmentResponse {
    #[serde(rename = "TransitGatewayRouteTableAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_route_table_attachment: Option<TransitGatewayRouteTableAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitGatewayRouteTableAttachment {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
    #[serde(rename = "PeeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_id: Option<String>,
    #[serde(rename = "TransitGatewayRouteTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_route_table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcAttachmentRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<VpcOptions>,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
    #[serde(rename = "SubnetArns")]
    #[serde(default)]
    pub subnet_arns: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcArn")]
    #[serde(default)]
    pub vpc_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcOptions {
    #[serde(rename = "ApplianceModeSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appliance_mode_support: Option<bool>,
    #[serde(rename = "DnsSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_support: Option<bool>,
    #[serde(rename = "Ipv6Support")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_support: Option<bool>,
    #[serde(rename = "SecurityGroupReferencingSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_referencing_support: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcAttachmentResponse {
    #[serde(rename = "VpcAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_attachment: Option<VpcAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcAttachment {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<VpcOptions>,
    #[serde(rename = "SubnetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttachmentResponse {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectPeerRequest {
    #[serde(rename = "ConnectPeerId")]
    #[serde(default)]
    pub connect_peer_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectPeerResponse {
    #[serde(rename = "ConnectPeer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer: Option<ConnectPeer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionRequest {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreNetworkPolicyVersionRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    pub policy_version_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreNetworkPolicyVersionResponse {
    #[serde(rename = "CoreNetworkPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_policy: Option<CoreNetworkPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkPolicy {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "ChangeSetState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_state: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "PolicyErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_errors: Option<Vec<CoreNetworkPolicyError>>,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkPolicyError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreNetworkPrefixListAssociationRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "PrefixListArn")]
    #[serde(default)]
    pub prefix_list_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreNetworkPrefixListAssociationResponse {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "PrefixListArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreNetworkRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCoreNetworkResponse {
    #[serde(rename = "CoreNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network: Option<CoreNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeviceRequest {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeviceResponse {
    #[serde(rename = "Device")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGlobalNetworkRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGlobalNetworkResponse {
    #[serde(rename = "GlobalNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network: Option<GlobalNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLinkRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    pub link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLinkResponse {
    #[serde(rename = "Link")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePeeringRequest {
    #[serde(rename = "PeeringId")]
    #[serde(default)]
    pub peering_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePeeringResponse {
    #[serde(rename = "Peering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering: Option<Peering>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSiteRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSiteResponse {
    #[serde(rename = "Site")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTransitGatewayRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "TransitGatewayArn")]
    #[serde(default)]
    pub transit_gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTransitGatewayResponse {
    #[serde(rename = "TransitGatewayRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_registration: Option<TransitGatewayRegistration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitGatewayRegistration {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TransitGatewayRegistrationStateReason>,
    #[serde(rename = "TransitGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitGatewayRegistrationStateReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalNetworksRequest {
    #[serde(rename = "GlobalNetworkIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_ids: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGlobalNetworksResponse {
    #[serde(rename = "GlobalNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_networks: Option<Vec<GlobalNetwork>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConnectPeerRequest {
    #[serde(rename = "ConnectPeerId")]
    #[serde(default)]
    pub connect_peer_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConnectPeerResponse {
    #[serde(rename = "ConnectPeerAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_association: Option<ConnectPeerAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateCustomerGatewayRequest {
    #[serde(rename = "CustomerGatewayArn")]
    #[serde(default)]
    pub customer_gateway_arn: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateCustomerGatewayResponse {
    #[serde(rename = "CustomerGatewayAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_association: Option<CustomerGatewayAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateLinkRequest {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    pub link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateLinkResponse {
    #[serde(rename = "LinkAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_association: Option<LinkAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTransitGatewayConnectPeerRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "TransitGatewayConnectPeerArn")]
    #[serde(default)]
    pub transit_gateway_connect_peer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTransitGatewayConnectPeerResponse {
    #[serde(rename = "TransitGatewayConnectPeerAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_association: Option<TransitGatewayConnectPeerAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteCoreNetworkChangeSetRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    pub policy_version_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteCoreNetworkChangeSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectAttachmentResponse {
    #[serde(rename = "ConnectAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_attachment: Option<ConnectAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectPeerAssociationsRequest {
    #[serde(rename = "ConnectPeerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_ids: Option<Vec<String>>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectPeerAssociationsResponse {
    #[serde(rename = "ConnectPeerAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_associations: Option<Vec<ConnectPeerAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectPeerRequest {
    #[serde(rename = "ConnectPeerId")]
    #[serde(default)]
    pub connect_peer_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectPeerResponse {
    #[serde(rename = "ConnectPeer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer: Option<ConnectPeer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionsRequest {
    #[serde(rename = "ConnectionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_ids: Option<Vec<String>>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionsResponse {
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkChangeEventsRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    pub policy_version_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkChangeEventsResponse {
    #[serde(rename = "CoreNetworkChangeEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_change_events: Option<Vec<CoreNetworkChangeEvent>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkChangeEvent {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "EventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    #[serde(rename = "IdentifierPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_path: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<CoreNetworkChangeEventValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkChangeEventValues {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "Cidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "NetworkFunctionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_group_name: Option<String>,
    #[serde(rename = "PeerEdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_edge_location: Option<String>,
    #[serde(rename = "RoutingPolicyAssociationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_association_details: Option<Vec<RoutingPolicyAssociationDetail>>,
    #[serde(rename = "RoutingPolicyDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_direction: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingPolicyAssociationDetail {
    #[serde(rename = "RoutingPolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_names: Option<Vec<String>>,
    #[serde(rename = "SharedSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_segments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkChangeSetRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    pub policy_version_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkChangeSetResponse {
    #[serde(rename = "CoreNetworkChanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_changes: Option<Vec<CoreNetworkChange>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkChange {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "IdentifierPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_path: Option<String>,
    #[serde(rename = "NewValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_values: Option<CoreNetworkChangeValues>,
    #[serde(rename = "PreviousValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_values: Option<CoreNetworkChangeValues>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkChangeValues {
    #[serde(rename = "Asn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "Cidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(rename = "DestinationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_identifier: Option<String>,
    #[serde(rename = "DnsSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_support: Option<bool>,
    #[serde(rename = "EdgeLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_locations: Option<Vec<String>>,
    #[serde(rename = "InsideCidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_cidr_blocks: Option<Vec<String>>,
    #[serde(rename = "NetworkFunctionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_group_name: Option<String>,
    #[serde(rename = "PeerEdgeLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_edge_locations: Option<Vec<String>>,
    #[serde(rename = "RoutingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy: Option<String>,
    #[serde(rename = "RoutingPolicyAssociationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_association_details: Option<Vec<RoutingPolicyAssociationDetail>>,
    #[serde(rename = "RoutingPolicyDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_direction: Option<String>,
    #[serde(rename = "SecurityGroupReferencingSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_referencing_support: Option<bool>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
    #[serde(rename = "ServiceInsertionActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_insertion_actions: Option<Vec<ServiceInsertionAction>>,
    #[serde(rename = "SharedSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_segments: Option<Vec<String>>,
    #[serde(rename = "VpnEcmpSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_ecmp_support: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceInsertionAction {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Via")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via: Option<Via>,
    #[serde(rename = "WhenSentTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_sent_to: Option<WhenSentTo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Via {
    #[serde(rename = "NetworkFunctionGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_groups: Option<Vec<NetworkFunctionGroup>>,
    #[serde(rename = "WithEdgeOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_edge_overrides: Option<Vec<EdgeOverride>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkFunctionGroup {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EdgeOverride {
    #[serde(rename = "EdgeSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_sets: Option<Vec<Vec<String>>>,
    #[serde(rename = "UseEdge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_edge: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WhenSentTo {
    #[serde(rename = "WhenSentToSegmentsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_sent_to_segments_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkPolicyRequest {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkPolicyResponse {
    #[serde(rename = "CoreNetworkPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_policy: Option<CoreNetworkPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCoreNetworkResponse {
    #[serde(rename = "CoreNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network: Option<CoreNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomerGatewayAssociationsRequest {
    #[serde(rename = "CustomerGatewayArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_arns: Option<Vec<String>>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomerGatewayAssociationsResponse {
    #[serde(rename = "CustomerGatewayAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_associations: Option<Vec<CustomerGatewayAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDevicesRequest {
    #[serde(rename = "DeviceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDevicesResponse {
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectConnectGatewayAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectConnectGatewayAttachmentResponse {
    #[serde(rename = "DirectConnectGatewayAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_attachment: Option<DirectConnectGatewayAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLinkAssociationsRequest {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLinkAssociationsResponse {
    #[serde(rename = "LinkAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_associations: Option<Vec<LinkAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLinksRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_ids: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLinksResponse {
    #[serde(rename = "Links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkResourceCountsRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkResourceCountsResponse {
    #[serde(rename = "NetworkResourceCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_resource_counts: Option<Vec<NetworkResourceCount>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkResourceCount {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkResourceRelationshipsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegisteredGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_gateway_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkResourceRelationshipsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Relationships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Relationship {
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkResourcesRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegisteredGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_gateway_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkResourcesResponse {
    #[serde(rename = "NetworkResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_resources: Option<Vec<NetworkResource>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkResource {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(rename = "DefinitionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_timestamp: Option<f64>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RegisteredGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_gateway_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkRoutesRequest {
    #[serde(rename = "DestinationFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ExactCidrMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_cidr_matches: Option<Vec<String>>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LongestPrefixMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longest_prefix_matches: Option<Vec<String>>,
    #[serde(rename = "PrefixListIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_ids: Option<Vec<String>>,
    #[serde(rename = "RouteTableIdentifier")]
    #[serde(default)]
    pub route_table_identifier: RouteTableIdentifier,
    #[serde(rename = "States")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    #[serde(rename = "SubnetOfMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_of_matches: Option<Vec<String>>,
    #[serde(rename = "SupernetOfMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supernet_of_matches: Option<Vec<String>>,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteTableIdentifier {
    #[serde(rename = "CoreNetworkNetworkFunctionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_network_function_group: Option<CoreNetworkNetworkFunctionGroupIdentifier>,
    #[serde(rename = "CoreNetworkSegmentEdge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_segment_edge: Option<CoreNetworkSegmentEdgeIdentifier>,
    #[serde(rename = "TransitGatewayRouteTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_route_table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkNetworkFunctionGroupIdentifier {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "NetworkFunctionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkSegmentEdgeIdentifier {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkRoutesResponse {
    #[serde(rename = "CoreNetworkSegmentEdge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_segment_edge: Option<CoreNetworkSegmentEdgeIdentifier>,
    #[serde(rename = "NetworkRoutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_routes: Option<Vec<NetworkRoute>>,
    #[serde(rename = "RouteTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_arn: Option<String>,
    #[serde(rename = "RouteTableTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_timestamp: Option<f64>,
    #[serde(rename = "RouteTableType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkRoute {
    #[serde(rename = "DestinationCidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr_block: Option<String>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<NetworkRouteDestination>>,
    #[serde(rename = "PrefixListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkRouteDestination {
    #[serde(rename = "CoreNetworkAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_attachment_id: Option<String>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "NetworkFunctionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_function_group_name: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkTelemetryRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegisteredGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_gateway_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkTelemetryResponse {
    #[serde(rename = "NetworkTelemetry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_telemetry: Option<Vec<NetworkTelemetry>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkTelemetry {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "Health")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<ConnectionHealth>,
    #[serde(rename = "RegisteredGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_gateway_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionHealth {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteAnalysisRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "RouteAnalysisId")]
    #[serde(default)]
    pub route_analysis_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteAnalysisResponse {
    #[serde(rename = "RouteAnalysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_analysis: Option<RouteAnalysis>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteAnalysis {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<RouteAnalysisEndpointOptions>,
    #[serde(rename = "ForwardPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_path: Option<RouteAnalysisPath>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "IncludeReturnPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_return_path: Option<bool>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "ReturnPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path: Option<RouteAnalysisPath>,
    #[serde(rename = "RouteAnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_analysis_id: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RouteAnalysisEndpointOptions>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UseMiddleboxes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_middleboxes: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteAnalysisEndpointOptions {
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "TransitGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_arn: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteAnalysisPath {
    #[serde(rename = "CompletionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_status: Option<RouteAnalysisCompletion>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<PathComponent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteAnalysisCompletion {
    #[serde(rename = "ReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(rename = "ReasonContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResultCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathComponent {
    #[serde(rename = "DestinationCidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr_block: Option<String>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<NetworkResourceSummary>,
    #[serde(rename = "Sequence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkResourceSummary {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(rename = "IsMiddlebox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_middlebox: Option<bool>,
    #[serde(rename = "NameTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_tag: Option<String>,
    #[serde(rename = "RegisteredGatewayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_gateway_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSiteToSiteVpnAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSiteToSiteVpnAttachmentResponse {
    #[serde(rename = "SiteToSiteVpnAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_to_site_vpn_attachment: Option<SiteToSiteVpnAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSitesRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SiteIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSitesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<Site>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayConnectPeerAssociationsRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TransitGatewayConnectPeerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayConnectPeerAssociationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TransitGatewayConnectPeerAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_connect_peer_associations:
        Option<Vec<TransitGatewayConnectPeerAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayPeeringRequest {
    #[serde(rename = "PeeringId")]
    #[serde(default)]
    pub peering_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayPeeringResponse {
    #[serde(rename = "TransitGatewayPeering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_peering: Option<TransitGatewayPeering>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayRegistrationsRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TransitGatewayArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayRegistrationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TransitGatewayRegistrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_registrations: Option<Vec<TransitGatewayRegistration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayRouteTableAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTransitGatewayRouteTableAttachmentResponse {
    #[serde(rename = "TransitGatewayRouteTableAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_route_table_attachment: Option<TransitGatewayRouteTableAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcAttachmentResponse {
    #[serde(rename = "VpcAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_attachment: Option<VpcAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachmentRoutingPolicyAssociationsRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachmentRoutingPolicyAssociationsResponse {
    #[serde(rename = "AttachmentRoutingPolicyAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_routing_policy_associations:
        Option<Vec<AttachmentRoutingPolicyAssociationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentRoutingPolicyAssociationSummary {
    #[serde(rename = "AssociatedRoutingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_routing_policies: Option<Vec<String>>,
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "PendingRoutingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_routing_policies: Option<Vec<String>>,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachmentsRequest {
    #[serde(rename = "AttachmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachmentsResponse {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectPeersRequest {
    #[serde(rename = "ConnectAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_attachment_id: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectPeersResponse {
    #[serde(rename = "ConnectPeers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peers: Option<Vec<ConnectPeerSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectPeerSummary {
    #[serde(rename = "ConnectAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_attachment_id: Option<String>,
    #[serde(rename = "ConnectPeerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_id: Option<String>,
    #[serde(rename = "ConnectPeerState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_peer_state: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "SubnetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworkPolicyVersionsRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworkPolicyVersionsResponse {
    #[serde(rename = "CoreNetworkPolicyVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_policy_versions: Option<Vec<CoreNetworkPolicyVersion>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkPolicyVersion {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "ChangeSetState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_state: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworkPrefixListAssociationsRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrefixListArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworkPrefixListAssociationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrefixListAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_associations: Option<Vec<PrefixListAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrefixListAssociation {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "PrefixListAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_alias: Option<String>,
    #[serde(rename = "PrefixListArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworkRoutingInformationRequest {
    #[serde(rename = "CommunityMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_matches: Option<Vec<String>>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    pub edge_location: String,
    #[serde(rename = "ExactAsPathMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_as_path_matches: Option<Vec<String>>,
    #[serde(rename = "LocalPreferenceMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_preference_matches: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MedMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub med_matches: Option<Vec<String>>,
    #[serde(rename = "NextHopFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_hop_filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    pub segment_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworkRoutingInformationResponse {
    #[serde(rename = "CoreNetworkRoutingInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_routing_information: Option<Vec<CoreNetworkRoutingInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkRoutingInformation {
    #[serde(rename = "AsPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_path: Option<Vec<String>>,
    #[serde(rename = "Communities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communities: Option<Vec<String>>,
    #[serde(rename = "LocalPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_preference: Option<String>,
    #[serde(rename = "Med")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub med: Option<String>,
    #[serde(rename = "NextHop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_hop: Option<RoutingInformationNextHop>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingInformationNextHop {
    #[serde(rename = "CoreNetworkAttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_attachment_id: Option<String>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworksRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoreNetworksResponse {
    #[serde(rename = "CoreNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_networks: Option<Vec<CoreNetworkSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoreNetworkSummary {
    #[serde(rename = "CoreNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_arn: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network_id: Option<String>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationServiceAccessStatusRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationServiceAccessStatusResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_status: Option<OrganizationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationStatus {
    #[serde(rename = "AccountStatusList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status_list: Option<Vec<AccountStatus>>,
    #[serde(rename = "OrganizationAwsServiceAccessStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_aws_service_access_status: Option<String>,
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "SLRDeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_l_r_deployment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountStatus {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "SLRDeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_l_r_deployment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPeeringsRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "EdgeLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PeeringType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_type: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPeeringsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Peerings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peerings: Option<Vec<Peering>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAttachmentRoutingPolicyLabelRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    pub routing_policy_label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAttachmentRoutingPolicyLabelResponse {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCoreNetworkPolicyRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LatestVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_id: Option<i32>,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCoreNetworkPolicyResponse {
    #[serde(rename = "CoreNetworkPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_policy: Option<CoreNetworkPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTransitGatewayRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "TransitGatewayArn")]
    #[serde(default)]
    pub transit_gateway_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTransitGatewayResponse {
    #[serde(rename = "TransitGatewayRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_registration: Option<TransitGatewayRegistration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectAttachmentResponse {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAttachmentRoutingPolicyLabelRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAttachmentRoutingPolicyLabelResponse {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_id: Option<String>,
    #[serde(rename = "RoutingPolicyLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreCoreNetworkPolicyVersionRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "PolicyVersionId")]
    #[serde(default)]
    pub policy_version_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreCoreNetworkPolicyVersionResponse {
    #[serde(rename = "CoreNetworkPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_policy: Option<CoreNetworkPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOrganizationServiceAccessUpdateRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOrganizationServiceAccessUpdateResponse {
    #[serde(rename = "OrganizationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_status: Option<OrganizationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRouteAnalysisRequest {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: RouteAnalysisEndpointOptionsSpecification,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "IncludeReturnPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_return_path: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: RouteAnalysisEndpointOptionsSpecification,
    #[serde(rename = "UseMiddleboxes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_middleboxes: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteAnalysisEndpointOptionsSpecification {
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "TransitGatewayAttachmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRouteAnalysisResponse {
    #[serde(rename = "RouteAnalysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_analysis: Option<RouteAnalysis>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionRequest {
    #[serde(rename = "ConnectedLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_link_id: Option<String>,
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCoreNetworkRequest {
    #[serde(rename = "CoreNetworkId")]
    #[serde(default)]
    pub core_network_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCoreNetworkResponse {
    #[serde(rename = "CoreNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network: Option<CoreNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceRequest {
    #[serde(rename = "AWSLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_location: Option<AWSLocation>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "Model")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Vendor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceResponse {
    #[serde(rename = "Device")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectConnectGatewayAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
    #[serde(rename = "EdgeLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_locations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectConnectGatewayAttachmentResponse {
    #[serde(rename = "DirectConnectGatewayAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_attachment: Option<DirectConnectGatewayAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalNetworkRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGlobalNetworkResponse {
    #[serde(rename = "GlobalNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_network: Option<GlobalNetwork>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLinkRequest {
    #[serde(rename = "Bandwidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Bandwidth>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "LinkId")]
    #[serde(default)]
    pub link_id: String,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLinkResponse {
    #[serde(rename = "Link")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNetworkResourceMetadataRequest {
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNetworkResourceMetadataResponse {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalNetworkId")]
    #[serde(default)]
    pub global_network_id: String,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteResponse {
    #[serde(rename = "Site")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcAttachmentRequest {
    #[serde(rename = "AddSubnetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_subnet_arns: Option<Vec<String>>,
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<VpcOptions>,
    #[serde(rename = "RemoveSubnetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_subnet_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcAttachmentResponse {
    #[serde(rename = "VpcAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_attachment: Option<VpcAttachment>,
}
