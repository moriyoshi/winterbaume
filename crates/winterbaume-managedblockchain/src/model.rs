//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-managedblockchain

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessorInput {
    #[serde(rename = "AccessorType")]
    #[serde(default)]
    pub accessor_type: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessorOutput {
    #[serde(rename = "AccessorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessor_id: Option<String>,
    #[serde(rename = "BillingToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_token: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMemberInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "InvitationId")]
    #[serde(default)]
    pub invitation_id: String,
    #[serde(rename = "MemberConfiguration")]
    #[serde(default)]
    pub member_configuration: MemberConfiguration,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberConfiguration {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FrameworkConfiguration")]
    #[serde(default)]
    pub framework_configuration: MemberFrameworkConfiguration,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<MemberLogPublishingConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberFrameworkConfiguration {
    #[serde(rename = "Fabric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<MemberFabricConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberFabricConfiguration {
    #[serde(rename = "AdminPassword")]
    #[serde(default)]
    pub admin_password: String,
    #[serde(rename = "AdminUsername")]
    #[serde(default)]
    pub admin_username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberLogPublishingConfiguration {
    #[serde(rename = "Fabric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<MemberFabricLogPublishingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberFabricLogPublishingConfiguration {
    #[serde(rename = "CaLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_logs: Option<LogConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfigurations {
    #[serde(rename = "Cloudwatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch: Option<LogConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMemberOutput {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNetworkInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Framework")]
    #[serde(default)]
    pub framework: String,
    #[serde(rename = "FrameworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_configuration: Option<NetworkFrameworkConfiguration>,
    #[serde(rename = "FrameworkVersion")]
    #[serde(default)]
    pub framework_version: String,
    #[serde(rename = "MemberConfiguration")]
    #[serde(default)]
    pub member_configuration: MemberConfiguration,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VotingPolicy")]
    #[serde(default)]
    pub voting_policy: VotingPolicy,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkFrameworkConfiguration {
    #[serde(rename = "Fabric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NetworkFabricConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkFabricConfiguration {
    #[serde(rename = "Edition")]
    #[serde(default)]
    pub edition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VotingPolicy {
    #[serde(rename = "ApprovalThresholdPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_threshold_policy: Option<ApprovalThresholdPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApprovalThresholdPolicy {
    #[serde(rename = "ProposalDurationInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_duration_in_hours: Option<i32>,
    #[serde(rename = "ThresholdComparator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_comparator: Option<String>,
    #[serde(rename = "ThresholdPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNetworkOutput {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NodeConfiguration")]
    #[serde(default)]
    pub node_configuration: NodeConfiguration,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeConfiguration {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    pub instance_type: String,
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<NodeLogPublishingConfiguration>,
    #[serde(rename = "StateDB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_d_b: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeLogPublishingConfiguration {
    #[serde(rename = "Fabric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NodeFabricLogPublishingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeFabricLogPublishingConfiguration {
    #[serde(rename = "ChaincodeLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chaincode_logs: Option<LogConfigurations>,
    #[serde(rename = "PeerLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_logs: Option<LogConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeOutput {
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProposalInput {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: ProposalActions,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: String,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProposalActions {
    #[serde(rename = "Invitations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<InviteAction>>,
    #[serde(rename = "Removals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removals: Option<Vec<RemoveAction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteAction {
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAction {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProposalOutput {
    #[serde(rename = "ProposalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessorInput {
    #[serde(rename = "AccessorId")]
    #[serde(default)]
    pub accessor_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessorOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMemberInput {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: String,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMemberOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNodeInput {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    pub node_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNodeOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessorInput {
    #[serde(rename = "AccessorId")]
    #[serde(default)]
    pub accessor_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessorOutput {
    #[serde(rename = "Accessor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessor: Option<Accessor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Accessor {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "BillingToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_token: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberInput {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: String,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberOutput {
    #[serde(rename = "Member")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Member {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FrameworkAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_attributes: Option<MemberFrameworkAttributes>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<MemberLogPublishingConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberFrameworkAttributes {
    #[serde(rename = "Fabric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<MemberFabricAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberFabricAttributes {
    #[serde(rename = "AdminUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[serde(rename = "CaEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkInput {
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNetworkOutput {
    #[serde(rename = "Network")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Network {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Framework")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(rename = "FrameworkAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_attributes: Option<NetworkFrameworkAttributes>,
    #[serde(rename = "FrameworkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VotingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_policy: Option<VotingPolicy>,
    #[serde(rename = "VpcEndpointServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkFrameworkAttributes {
    #[serde(rename = "Ethereum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethereum: Option<NetworkEthereumAttributes>,
    #[serde(rename = "Fabric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NetworkFabricAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkEthereumAttributes {
    #[serde(rename = "ChainId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkFabricAttributes {
    #[serde(rename = "Edition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "OrderingServiceEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering_service_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNodeInput {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    pub node_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNodeOutput {
    #[serde(rename = "Node")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Node>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "FrameworkAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_attributes: Option<NodeFrameworkAttributes>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<NodeLogPublishingConfiguration>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "StateDB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_d_b: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeFrameworkAttributes {
    #[serde(rename = "Ethereum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethereum: Option<NodeEthereumAttributes>,
    #[serde(rename = "Fabric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric: Option<NodeFabricAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeEthereumAttributes {
    #[serde(rename = "HttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
    #[serde(rename = "WebSocketEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_socket_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeFabricAttributes {
    #[serde(rename = "PeerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_endpoint: Option<String>,
    #[serde(rename = "PeerEventEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_event_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProposalInput {
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "ProposalId")]
    #[serde(default)]
    pub proposal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProposalOutput {
    #[serde(rename = "Proposal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal: Option<Proposal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Proposal {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<ProposalActions>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "NoVoteCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_vote_count: Option<i32>,
    #[serde(rename = "OutstandingVoteCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding_vote_count: Option<i32>,
    #[serde(rename = "ProposalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    #[serde(rename = "ProposedByMemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_id: Option<String>,
    #[serde(rename = "ProposedByMemberName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "YesVoteCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yes_vote_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessorsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessorsOutput {
    #[serde(rename = "Accessors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<AccessorSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessorSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInvitationsInput {
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
pub struct ListInvitationsOutput {
    #[serde(rename = "Invitations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Invitation {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "InvitationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "NetworkSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_summary: Option<NetworkSummary>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Framework")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(rename = "FrameworkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersInput {
    #[serde(rename = "IsOwned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersOutput {
    #[serde(rename = "Members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<MemberSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsOwned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNetworksInput {
    #[serde(rename = "Framework")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNetworksOutput {
    #[serde(rename = "Networks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Nodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<NodeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProposalVotesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProposalId")]
    #[serde(default)]
    pub proposal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProposalVotesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProposalVotes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_votes: Option<Vec<VoteSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoteSummary {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "MemberName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
    #[serde(rename = "Vote")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProposalsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProposalsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Proposals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<ProposalSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProposalSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "ProposalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    #[serde(rename = "ProposedByMemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_id: Option<String>,
    #[serde(rename = "ProposedByMemberName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_by_member_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectInvitationInput {
    #[serde(rename = "InvitationId")]
    #[serde(default)]
    pub invitation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectInvitationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
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
pub struct UpdateMemberInput {
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<MemberLogPublishingConfiguration>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: String,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMemberOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodeInput {
    #[serde(rename = "LogPublishingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_configuration: Option<NodeLogPublishingConfiguration>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    pub node_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodeOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoteOnProposalInput {
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(rename = "ProposalId")]
    #[serde(default)]
    pub proposal_id: String,
    #[serde(rename = "Vote")]
    #[serde(default)]
    pub vote: String,
    #[serde(rename = "VoterMemberId")]
    #[serde(default)]
    pub voter_member_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoteOnProposalOutput {}
