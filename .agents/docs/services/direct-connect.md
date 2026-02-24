# AWS Direct Connect

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Direct Connect links your internal network to an Direct Connect location over a standard Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an Direct Connect router. With this connection in place, you can create virtual interfaces directly to the Amazon Web Services Cloud (for example, to Amazon EC2 and Amazon S3) and to Amazon VPC, bypassing Internet service providers in your network path. A connection provides access to all Amazon Web Services Regions except the China (Beijing) and (China) Ningxia Regions. Amazon Web Services resources in the China Regions can only be accessed through locations associated with those Regions.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-directconnect/tests/scenario_test.rs`: create, describe, and delete a Direct Connect connection.
- Backported from `scenario_test.rs`: manage a portfolio of independent connections and verify list/describe isolation.
- Backported from `scenario_test.rs`: discover locations before provisioning a connection.
- Scenario insight from EC2: include mutable binding failover for AWS Direct Connect where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Direct Connect by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: model dedicated connectivity planning, locations, connections, link aggregation groups, hosted/private/public virtual interfaces, gateways, confirmations, and tag-based inventory.

## Service Identity and Protocol

- AWS model slug: `direct-connect`
- AWS SDK for Rust slug: `directconnect`
- Model version: `2012-10-25`
- Model file: `vendor/api-models-aws/models/direct-connect/service/2012-10-25/direct-connect-2012-10-25.json`
- SDK ID: `Direct Connect`
- Endpoint prefix: `directconnect`
- ARN namespace: `directconnect`
- CloudFormation name: `DirectConnect`
- CloudTrail event source: `directconnect.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (18), `Create` (10), `Delete` (8), `Allocate` (5), `Confirm` (5), `Update` (5), `Associate` (4), `Disassociate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptDirectConnectGatewayAssociationProposal`, `AssociateConnectionWithLag`, `AssociateHostedConnection`, `AssociateMacSecKey`, `AssociateVirtualInterface`, `CreateBGPPeer`, `CreateConnection`, `CreateDirectConnectGateway`, `CreateDirectConnectGatewayAssociation`, `CreateDirectConnectGatewayAssociationProposal`, `CreateInterconnect`, `CreateLag`, `CreatePrivateVirtualInterface`, `CreatePublicVirtualInterface`, `CreateTransitVirtualInterface`, `DeleteBGPPeer`, `DeleteConnection`, `DeleteDirectConnectGateway`, `DeleteDirectConnectGatewayAssociation`, `DeleteDirectConnectGatewayAssociationProposal`, ... (+14).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeConnectionLoa`, `DescribeConnections`, `DescribeConnectionsOnInterconnect`, `DescribeCustomerMetadata`, `DescribeDirectConnectGatewayAssociationProposals`, `DescribeDirectConnectGatewayAssociations`, `DescribeDirectConnectGatewayAttachments`, `DescribeDirectConnectGateways`, `DescribeHostedConnections`, `DescribeInterconnectLoa`, `DescribeInterconnects`, `DescribeLags`, `DescribeLoa`, `DescribeLocations`, `DescribeRouterConfiguration`, `DescribeTags`, `DescribeVirtualGateways`, `DescribeVirtualInterfaces`, `ListVirtualInterfaceTestHistory`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartBgpFailoverTest`, `StopBgpFailoverTest`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 63 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/directconnect/latest/UserGuide/Welcome.html
- https://docs.aws.amazon.com/directconnect/latest/UserGuide/WorkingWithConnections.html
- https://docs.aws.amazon.com/directconnect/latest/UserGuide/WorkingWithVirtualInterfaces.html
- https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-gateways-intro.html

Research outcomes:
- Direct Connect provides dedicated or hosted network connections between customer networks and AWS.
- Connections are physical or partner-provided Ethernet links at Direct Connect locations.
- Virtual interfaces provide private, public, or transit connectivity over a connection using VLAN and BGP configuration.
- Hosted virtual interfaces and hosted connections involve cross-account acceptance workflows.
- Direct Connect gateways connect private/transit virtual interfaces to virtual private gateways or transit gateways across Regions.
- Link aggregation groups bundle multiple connections for aggregate capacity and redundancy.

Parity implications:
- Model connections, hosted connections, LAGs, virtual interfaces, hosted VIFs, Direct Connect gateways, associations, BGP peers, and acceptance state separately.
- Virtual interface state should depend on connection state and BGP configuration.
- Cross-account proposals/hosted resources should require acceptance before becoming active.

## Operation Groups

### Describe

- Operations: `DescribeConnectionLoa`, `DescribeConnections`, `DescribeConnectionsOnInterconnect`, `DescribeCustomerMetadata`, `DescribeDirectConnectGatewayAssociationProposals`, `DescribeDirectConnectGatewayAssociations`, `DescribeDirectConnectGatewayAttachments`, `DescribeDirectConnectGateways`, `DescribeHostedConnections`, `DescribeInterconnectLoa`, `DescribeInterconnects`, `DescribeLags`, `DescribeLoa`, `DescribeLocations`, `DescribeRouterConfiguration`, `DescribeTags`, `DescribeVirtualGateways`, `DescribeVirtualInterfaces`
- Common required input members in this group: `connectionId`, `interconnectId`, `resourceArns`, `virtualInterfaceId`

### Create

- Operations: `CreateBGPPeer`, `CreateConnection`, `CreateDirectConnectGateway`, `CreateDirectConnectGatewayAssociation`, `CreateDirectConnectGatewayAssociationProposal`, `CreateInterconnect`, `CreateLag`, `CreatePrivateVirtualInterface`, `CreatePublicVirtualInterface`, `CreateTransitVirtualInterface`
- Common required input members in this group: `bandwidth`, `connectionId`, `connectionName`, `connectionsBandwidth`, `directConnectGatewayId`, `directConnectGatewayName`, `directConnectGatewayOwnerAccount`, `gatewayId`, `interconnectName`, `lagName`, `location`, `newPrivateVirtualInterface`, `newPublicVirtualInterface`, `newTransitVirtualInterface`, `numberOfConnections`

### Delete

- Operations: `DeleteBGPPeer`, `DeleteConnection`, `DeleteDirectConnectGateway`, `DeleteDirectConnectGatewayAssociation`, `DeleteDirectConnectGatewayAssociationProposal`, `DeleteInterconnect`, `DeleteLag`, `DeleteVirtualInterface`
- Common required input members in this group: `connectionId`, `directConnectGatewayId`, `interconnectId`, `lagId`, `proposalId`, `virtualInterfaceId`

### Allocate

- Operations: `AllocateConnectionOnInterconnect`, `AllocateHostedConnection`, `AllocatePrivateVirtualInterface`, `AllocatePublicVirtualInterface`, `AllocateTransitVirtualInterface`
- Common required input members in this group: `bandwidth`, `connectionId`, `connectionName`, `interconnectId`, `newPrivateVirtualInterfaceAllocation`, `newPublicVirtualInterfaceAllocation`, `newTransitVirtualInterfaceAllocation`, `ownerAccount`, `vlan`

### Confirm

- Operations: `ConfirmConnection`, `ConfirmCustomerAgreement`, `ConfirmPrivateVirtualInterface`, `ConfirmPublicVirtualInterface`, `ConfirmTransitVirtualInterface`
- Common required input members in this group: `connectionId`, `directConnectGatewayId`, `virtualInterfaceId`

### Update

- Operations: `UpdateConnection`, `UpdateDirectConnectGateway`, `UpdateDirectConnectGatewayAssociation`, `UpdateLag`, `UpdateVirtualInterfaceAttributes`
- Common required input members in this group: `connectionId`, `directConnectGatewayId`, `lagId`, `newDirectConnectGatewayName`, `virtualInterfaceId`

### Associate

- Operations: `AssociateConnectionWithLag`, `AssociateHostedConnection`, `AssociateMacSecKey`, `AssociateVirtualInterface`
- Common required input members in this group: `connectionId`, `lagId`, `parentConnectionId`, `virtualInterfaceId`

### Disassociate

- Operations: `DisassociateConnectionFromLag`, `DisassociateMacSecKey`
- Common required input members in this group: `connectionId`, `lagId`, `secretARN`

### Accept

- Operations: `AcceptDirectConnectGatewayAssociationProposal`
- Common required input members in this group: `associatedGatewayOwnerAccount`, `directConnectGatewayId`, `proposalId`

### List

- Operations: `ListVirtualInterfaceTestHistory`

### Start

- Operations: `StartBgpFailoverTest`
- Common required input members in this group: `virtualInterfaceId`

### Stop

- Operations: `StopBgpFailoverTest`
- Common required input members in this group: `virtualInterfaceId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptDirectConnectGatewayAssociationProposal` | - | - | `associatedGatewayOwnerAccount`, `directConnectGatewayId`, `proposalId` | - | `AcceptDirectConnectGatewayAssociationProposalResult` | `DirectConnectClientException`, `DirectConnectServerException` | Accepts a proposal request to attach a virtual private gateway or transit gateway to a Direct Connect gateway. |
| `AllocateConnectionOnInterconnect` | - | - | `bandwidth`, `connectionName`, `interconnectId`, `ownerAccount`, `vlan` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException` | Deprecated. Use AllocateHostedConnection instead. |
| `AllocateHostedConnection` | - | - | `bandwidth`, `connectionId`, `connectionName`, `ownerAccount`, `vlan` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Creates a hosted connection on the specified interconnect or a link aggregation group (LAG) of interconnects. Allocates a VLAN number and a specified amount of capacity (bandwidth) for use by a hosted connection on the specified interconnect or LAG of... |
| `AllocatePrivateVirtualInterface` | - | - | `connectionId`, `newPrivateVirtualInterfaceAllocation`, `ownerAccount` | - | `VirtualInterface` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Provisions a private virtual interface to be owned by the specified Amazon Web Services account. Virtual interfaces created using this action must be confirmed by the owner using ConfirmPrivateVirtualInterface. |
| `AllocatePublicVirtualInterface` | - | - | `connectionId`, `newPublicVirtualInterfaceAllocation`, `ownerAccount` | - | `VirtualInterface` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Provisions a public virtual interface to be owned by the specified Amazon Web Services account. The owner of a connection calls this function to provision a public virtual interface to be owned by the specified Amazon Web Services account. |
| `AllocateTransitVirtualInterface` | - | - | `connectionId`, `newTransitVirtualInterfaceAllocation`, `ownerAccount` | - | `AllocateTransitVirtualInterfaceResult` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Provisions a transit virtual interface to be owned by the specified Amazon Web Services account. Use this type of interface to connect a transit gateway to your Direct Connect gateway. |
| `AssociateConnectionWithLag` | - | - | `connectionId`, `lagId` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException` | Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to Amazon Web Services is interrupted). |
| `AssociateHostedConnection` | - | - | `connectionId`, `parentConnectionId` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException` | Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. |
| `AssociateMacSecKey` | - | - | `connectionId` | - | `AssociateMacSecKeyResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Associates a MAC Security (MACsec) Connection Key Name (CKN)/ Connectivity Association Key (CAK) pair with a Direct Connect connection. You must supply either the `secretARN,` or the CKN/CAK (`ckn` and `cak`) pair in the request. |
| `AssociateVirtualInterface` | - | - | `connectionId`, `virtualInterfaceId` | - | `VirtualInterface` | `DirectConnectClientException`, `DirectConnectServerException` | Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to Amazon Web Services is temporarily interrupted as the virtual interface is being migrated. |
| `ConfirmConnection` | - | - | `connectionId` | - | `ConfirmConnectionResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Confirms the creation of the specified hosted connection on an interconnect. Upon creation, the hosted connection is initially in the `Ordering` state, and remains in this state until the owner confirms creation of the hosted connection. |
| `ConfirmCustomerAgreement` | - | - | - | - | `ConfirmCustomerAgreementResponse` | `DirectConnectClientException`, `DirectConnectServerException` | The confirmation of the terms of agreement when creating the connection/link aggregation group (LAG). |
| `ConfirmPrivateVirtualInterface` | - | - | `virtualInterfaceId` | - | `ConfirmPrivateVirtualInterfaceResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Accepts ownership of a private virtual interface created by another Amazon Web Services account. After the virtual interface owner makes this call, the virtual interface is created and attached to the specified virtual private gateway or Direct Connect... |
| `ConfirmPublicVirtualInterface` | - | - | `virtualInterfaceId` | - | `ConfirmPublicVirtualInterfaceResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Accepts ownership of a public virtual interface created by another Amazon Web Services account. After the virtual interface owner makes this call, the specified virtual interface is created and made available to handle traffic. |
| `ConfirmTransitVirtualInterface` | - | - | `directConnectGatewayId`, `virtualInterfaceId` | - | `ConfirmTransitVirtualInterfaceResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Accepts ownership of a transit virtual interface created by another Amazon Web Services account. After the owner of the transit virtual interface makes this call, the specified transit virtual interface is created and made available to handle traffic. |
| `CreateBGPPeer` | - | - | - | - | `CreateBGPPeerResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Creates a BGP peer on the specified virtual interface. You must create a BGP peer for the corresponding address family (IPv4/IPv6) in order to access Amazon Web Services resources that also use that address family. |
| `CreateConnection` | - | - | `bandwidth`, `connectionName`, `location` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Creates a connection between a customer network and a specific Direct Connect location. A connection links your internal network to an Direct Connect location over a standard Ethernet fiber-optic cable. |
| `CreateDirectConnectGateway` | - | - | `directConnectGatewayName` | - | `CreateDirectConnectGatewayResult` | `DirectConnectClientException`, `DirectConnectServerException` | Creates a Direct Connect gateway, which is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. A Direct Connect gateway is global and visible in any Amazon Web Services Region after it is created. |
| `CreateDirectConnectGatewayAssociation` | - | - | `directConnectGatewayId` | - | `CreateDirectConnectGatewayAssociationResult` | `DirectConnectClientException`, `DirectConnectServerException` | Creates an association between a Direct Connect gateway and a virtual private gateway. The virtual private gateway must be attached to a VPC and must not be associated with another Direct Connect gateway. |
| `CreateDirectConnectGatewayAssociationProposal` | - | - | `directConnectGatewayId`, `directConnectGatewayOwnerAccount`, `gatewayId` | - | `CreateDirectConnectGatewayAssociationProposalResult` | `DirectConnectClientException`, `DirectConnectServerException` | Creates a proposal to associate the specified virtual private gateway or transit gateway with the specified Direct Connect gateway. You can associate a Direct Connect gateway and virtual private gateway or transit gateway that is owned by any Amazon Web... |
| `CreateInterconnect` | - | - | `bandwidth`, `interconnectName`, `location` | - | `Interconnect` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Creates an interconnect between an Direct Connect Partner's network and a specific Direct Connect location. An interconnect is a connection that is capable of hosting other connections. |
| `CreateLag` | - | - | `connectionsBandwidth`, `lagName`, `location`, `numberOfConnections` | - | `Lag` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Creates a link aggregation group (LAG) with the specified number of bundled physical dedicated connections between the customer network and a specific Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP)... |
| `CreatePrivateVirtualInterface` | - | - | `connectionId`, `newPrivateVirtualInterface` | - | `VirtualInterface` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Creates a private virtual interface. A virtual interface is the VLAN that transports Direct Connect traffic. |
| `CreatePublicVirtualInterface` | - | - | `connectionId`, `newPublicVirtualInterface` | - | `VirtualInterface` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Creates a public virtual interface. A virtual interface is the VLAN that transports Direct Connect traffic. |
| `CreateTransitVirtualInterface` | - | - | `connectionId`, `newTransitVirtualInterface` | - | `CreateTransitVirtualInterfaceResult` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Creates a transit virtual interface. A transit virtual interface should be used to access one or more transit gateways associated with Direct Connect gateways. |
| `DeleteBGPPeer` | - | - | - | - | `DeleteBGPPeerResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes the specified BGP peer on the specified virtual interface with the specified customer address and ASN. You cannot delete the last BGP peer from a virtual interface. |
| `DeleteConnection` | - | - | `connectionId` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes the specified connection. Deleting a connection only stops the Direct Connect port hour and data transfer charges. |
| `DeleteDirectConnectGateway` | - | - | `directConnectGatewayId` | - | `DeleteDirectConnectGatewayResult` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes the specified Direct Connect gateway. You must first delete all virtual interfaces that are attached to the Direct Connect gateway and disassociate all virtual private gateways associated with the Direct Connect gateway. |
| `DeleteDirectConnectGatewayAssociation` | - | - | - | - | `DeleteDirectConnectGatewayAssociationResult` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes the association between the specified Direct Connect gateway and virtual private gateway. We recommend that you specify the `associationID` to delete the association. |
| `DeleteDirectConnectGatewayAssociationProposal` | - | - | `proposalId` | - | `DeleteDirectConnectGatewayAssociationProposalResult` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes the association proposal request between the specified Direct Connect gateway and virtual private gateway or transit gateway. |
| `DeleteInterconnect` | - | - | `interconnectId` | - | `DeleteInterconnectResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes the specified interconnect. Intended for use by Direct Connect Partners only. |
| `DeleteLag` | - | - | `lagId` | - | `Lag` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes the specified link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections. |
| `DeleteVirtualInterface` | - | - | `virtualInterfaceId` | - | `DeleteVirtualInterfaceResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Deletes a virtual interface. |
| `DescribeConnectionLoa` | - | - | `connectionId` | - | `DescribeConnectionLoaResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Deprecated. Use DescribeLoa instead. |
| `DescribeConnections` | - | - | - | - | `Connections` | `DirectConnectClientException`, `DirectConnectServerException` | Displays the specified connection or all connections in this Region. |
| `DescribeConnectionsOnInterconnect` | - | - | `interconnectId` | - | `Connections` | `DirectConnectClientException`, `DirectConnectServerException` | Deprecated. Use DescribeHostedConnections instead. |
| `DescribeCustomerMetadata` | - | - | - | - | `DescribeCustomerMetadataResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Get and view a list of customer agreements, along with their signed status and whether the customer is an NNIPartner, NNIPartnerV2, or a nonPartner. |
| `DescribeDirectConnectGatewayAssociationProposals` | - | - | - | - | `DescribeDirectConnectGatewayAssociationProposalsResult` | `DirectConnectClientException`, `DirectConnectServerException` | Describes one or more association proposals for connection between a virtual private gateway or transit gateway and a Direct Connect gateway. |
| `DescribeDirectConnectGatewayAssociations` | - | - | - | - | `DescribeDirectConnectGatewayAssociationsResult` | `DirectConnectClientException`, `DirectConnectServerException` | Lists the associations between your Direct Connect gateways and virtual private gateways and transit gateways. You must specify one of the following: A Direct Connect gateway The response contains all virtual private gateways and transit gateways associated... |
| `DescribeDirectConnectGatewayAttachments` | - | - | - | - | `DescribeDirectConnectGatewayAttachmentsResult` | `DirectConnectClientException`, `DirectConnectServerException` | Lists the attachments between your Direct Connect gateways and virtual interfaces. You must specify a Direct Connect gateway, a virtual interface, or both. |
| `DescribeDirectConnectGateways` | - | - | - | - | `DescribeDirectConnectGatewaysResult` | `DirectConnectClientException`, `DirectConnectServerException` | Lists all your Direct Connect gateways or only the specified Direct Connect gateway. Deleted Direct Connect gateways are not returned. |
| `DescribeHostedConnections` | - | - | `connectionId` | - | `Connections` | `DirectConnectClientException`, `DirectConnectServerException` | Lists the hosted connections that have been provisioned on the specified interconnect or link aggregation group (LAG). Intended for use by Direct Connect Partners only. |
| `DescribeInterconnectLoa` | - | - | `interconnectId` | - | `DescribeInterconnectLoaResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Deprecated. Use DescribeLoa instead. |
| `DescribeInterconnects` | - | - | - | - | `Interconnects` | `DirectConnectClientException`, `DirectConnectServerException` | Lists the interconnects owned by the Amazon Web Services account or only the specified interconnect. |
| `DescribeLags` | - | - | - | - | `Lags` | `DirectConnectClientException`, `DirectConnectServerException` | Describes all your link aggregation groups (LAG) or the specified LAG. |
| `DescribeLoa` | - | - | `connectionId` | - | `Loa` | `DirectConnectClientException`, `DirectConnectServerException` | Gets the LOA-CFA for a connection, interconnect, or link aggregation group (LAG). The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to Amazon Web Services at the colocation... |
| `DescribeLocations` | - | - | - | - | `Locations` | `DirectConnectClientException`, `DirectConnectServerException` | Lists the Direct Connect locations in the current Amazon Web Services Region. These are the locations that can be selected when calling CreateConnection or CreateInterconnect. |
| `DescribeRouterConfiguration` | - | - | `virtualInterfaceId` | - | `DescribeRouterConfigurationResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Details about the router. |
| `DescribeTags` | - | - | `resourceArns` | - | `DescribeTagsResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Describes the tags associated with the specified Direct Connect resources. |
| `DescribeVirtualGateways` | - | - | - | - | `VirtualGateways` | `DirectConnectClientException`, `DirectConnectServerException` | Deprecated. Use `DescribeVpnGateways` instead. |
| `DescribeVirtualInterfaces` | - | - | - | - | `VirtualInterfaces` | `DirectConnectClientException`, `DirectConnectServerException` | Displays all virtual interfaces for an Amazon Web Services account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. |
| `DisassociateConnectionFromLag` | - | - | `connectionId`, `lagId` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException` | Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the DeleteConnection request). |
| `DisassociateMacSecKey` | - | - | `connectionId`, `secretARN` | - | `DisassociateMacSecKeyResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Removes the association between a MAC Security (MACsec) security key and a Direct Connect connection. |
| `ListVirtualInterfaceTestHistory` | - | - | - | - | `ListVirtualInterfaceTestHistoryResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Lists the virtual interface failover test history. |
| `StartBgpFailoverTest` | - | - | `virtualInterfaceId` | - | `StartBgpFailoverTestResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Starts the virtual interface failover test that verifies your configuration meets your resiliency requirements by placing the BGP peering session in the DOWN state. You can then send traffic to verify that there are no outages. |
| `StopBgpFailoverTest` | - | - | `virtualInterfaceId` | - | `StopBgpFailoverTestResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Stops the virtual interface failover test. |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `DirectConnectClientException`, `DirectConnectServerException`, `DuplicateTagKeysException`, `TooManyTagsException` | Adds the specified tags to the specified Direct Connect resource. Each resource can have a maximum of 50 tags. |
| `UntagResource` | - | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Removes one or more tags from the specified Direct Connect resource. |
| `UpdateConnection` | - | - | `connectionId` | - | `Connection` | `DirectConnectClientException`, `DirectConnectServerException` | Updates the Direct Connect connection configuration. You can update the following parameters for a connection: The connection name The connection's MAC Security (MACsec) encryption mode. |
| `UpdateDirectConnectGateway` | - | - | `directConnectGatewayId`, `newDirectConnectGatewayName` | - | `UpdateDirectConnectGatewayResponse` | `DirectConnectClientException`, `DirectConnectServerException` | Updates the name of a current Direct Connect gateway. |
| `UpdateDirectConnectGatewayAssociation` | - | - | - | - | `UpdateDirectConnectGatewayAssociationResult` | `DirectConnectClientException`, `DirectConnectServerException` | Updates the specified attributes of the Direct Connect gateway association. Add or remove prefixes from the association. |
| `UpdateLag` | - | - | `lagId` | - | `Lag` | `DirectConnectClientException`, `DirectConnectServerException` | Updates the attributes of the specified link aggregation group (LAG). You can update the following LAG attributes: The name of the LAG. |
| `UpdateVirtualInterfaceAttributes` | - | - | `virtualInterfaceId` | - | `VirtualInterface` | `DirectConnectClientException`, `DirectConnectServerException` | Updates the specified attributes of the specified virtual private interface. Setting the MTU of a virtual interface to 8500 (jumbo frames) can cause an update to the underlying physical connection if it wasn't updated to support jumbo frames. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DirectConnectClientException` | `structure` | `message` | One or more parameters are not valid. |
| `DirectConnectServerException` | `structure` | `message` | A server-side error occurred. |
| `DuplicateTagKeysException` | `structure` | `message` | A tag key was specified more than once. |
| `TooManyTagsException` | `structure` | `message` | You have reached the limit on the number of tags that can be assigned. |
| `Connection` | `structure` | `awsDevice`, `awsDeviceV2`, `awsLogicalDeviceId`, `bandwidth`, `connectionId`, `connectionName`, `connectionState`, `encryptionMode`, `hasLogicalRedundancy`, `jumboFrameCapable`, `lagId`, `loaIssueTime`, ... (+11) | Information about an Direct Connect connection. |
| `VirtualInterface` | `structure` | `addressFamily`, `amazonAddress`, `amazonSideAsn`, `asn`, `asnLong`, `authKey`, `awsDeviceV2`, `awsLogicalDeviceId`, `bgpPeers`, `connectionId`, `customerAddress`, `customerRouterConfig`, ... (+15) | Information about a virtual interface. |
| `Lag` | `structure` | `allowsHostedConnections`, `awsDevice`, `awsDeviceV2`, `awsLogicalDeviceId`, `connections`, `connectionsBandwidth`, `encryptionMode`, `hasLogicalRedundancy`, `jumboFrameCapable`, `lagId`, `lagName`, `lagState`, ... (+9) | Information about a link aggregation group (LAG). |
| `Connections` | `structure` | `connections`, `nextToken` | - |
| `AcceptDirectConnectGatewayAssociationProposalRequest` | `structure` | `associatedGatewayOwnerAccount`, `directConnectGatewayId`, `overrideAllowedPrefixesToDirectConnectGateway`, `proposalId` | - |
| `AcceptDirectConnectGatewayAssociationProposalResult` | `structure` | `directConnectGatewayAssociation` | - |
| `AllocateConnectionOnInterconnectRequest` | `structure` | `bandwidth`, `connectionName`, `interconnectId`, `ownerAccount`, `vlan` | - |
| `AllocateHostedConnectionRequest` | `structure` | `bandwidth`, `connectionId`, `connectionName`, `ownerAccount`, `tags`, `vlan` | - |
| `AllocatePrivateVirtualInterfaceRequest` | `structure` | `connectionId`, `newPrivateVirtualInterfaceAllocation`, `ownerAccount` | - |
| `AllocatePublicVirtualInterfaceRequest` | `structure` | `connectionId`, `newPublicVirtualInterfaceAllocation`, `ownerAccount` | - |
| `AllocateTransitVirtualInterfaceRequest` | `structure` | `connectionId`, `newTransitVirtualInterfaceAllocation`, `ownerAccount` | - |
| `AllocateTransitVirtualInterfaceResult` | `structure` | `virtualInterface` | - |
| `AssociateConnectionWithLagRequest` | `structure` | `connectionId`, `lagId` | - |
| `AssociateHostedConnectionRequest` | `structure` | `connectionId`, `parentConnectionId` | - |
| `AssociateMacSecKeyRequest` | `structure` | `cak`, `ckn`, `connectionId`, `secretARN` | - |
| `AssociateMacSecKeyResponse` | `structure` | `connectionId`, `macSecKeys` | - |
| `AssociateVirtualInterfaceRequest` | `structure` | `connectionId`, `virtualInterfaceId` | - |
| `ConfirmConnectionRequest` | `structure` | `connectionId` | - |
| `ConfirmConnectionResponse` | `structure` | `connectionState` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
