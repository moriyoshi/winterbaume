# AWS Network Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services enables you to centrally manage your Amazon Web Services Cloud WAN core network and your Transit Gateway network across Amazon Web Services accounts, Regions, and on-premises locations.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-networkmanager/tests/scenario_test.rs`: create a full SD-WAN topology with global network, sites, devices, links, and associations.
- Backported from `scenario_test.rs`: manage a core network and VPC attachment lifecycle.
- Backported from `scenario_test.rs`: register and deregister a transit gateway with Network Manager.
- Scenario insight from EC2: include mutable binding failover for AWS Network Manager where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Network Manager by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: support Cloud WAN and transit gateway network administration across accounts, Regions, and on-premises locations, including topology inventory, attachments, policies, route analysis, telemetry, and tag management.

## Service Identity and Protocol

- AWS model slug: `networkmanager`
- AWS SDK for Rust slug: `networkmanager`
- Model version: `2019-07-05`
- Model file: `vendor/api-models-aws/models/networkmanager/service/2019-07-05/networkmanager-2019-07-05.json`
- SDK ID: `NetworkManager`
- Endpoint prefix: `networkmanager`
- ARN namespace: `networkmanager`
- CloudFormation name: `NetworkManager`
- CloudTrail event source: `networkmanager.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (27), `Create` (14), `Delete` (12), `List` (10), `Update` (9), `Associate` (4), `Disassociate` (4), `Put` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptAttachment`, `AssociateConnectPeer`, `AssociateCustomerGateway`, `AssociateLink`, `AssociateTransitGatewayConnectPeer`, `CreateConnectAttachment`, `CreateConnectPeer`, `CreateConnection`, `CreateCoreNetwork`, `CreateCoreNetworkPrefixListAssociation`, `CreateDevice`, `CreateDirectConnectGatewayAttachment`, `CreateGlobalNetwork`, `CreateLink`, `CreateSite`, `CreateSiteToSiteVpnAttachment`, `CreateTransitGatewayPeering`, `CreateTransitGatewayRouteTableAttachment`, `CreateVpcAttachment`, `DeleteAttachment`, ... (+36).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeGlobalNetworks`, `GetConnectAttachment`, `GetConnectPeer`, `GetConnectPeerAssociations`, `GetConnections`, `GetCoreNetwork`, `GetCoreNetworkChangeEvents`, `GetCoreNetworkChangeSet`, `GetCoreNetworkPolicy`, `GetCustomerGatewayAssociations`, `GetDevices`, `GetDirectConnectGatewayAttachment`, `GetLinkAssociations`, `GetLinks`, `GetNetworkResourceCounts`, `GetNetworkResourceRelationships`, `GetNetworkResources`, `GetNetworkRoutes`, `GetNetworkTelemetry`, `GetResourcePolicy`, ... (+18).
- Pagination is modelled for 24 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetRouteAnalysis`, `StartOrganizationServiceAccessUpdate`, `StartRouteAnalysis`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 94 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Current Network Resource Stub Semantics

Network Manager currently stores VPC attachments by ARN and subnet ARN metadata.

- `CreateVpcAttachment` requires a VPC ARN and records it as the attachment resource ARN.
- Supplied subnet ARNs are stored as raw strings on the attachment.
- Attachment state is not linked to EC2 VPC, subnet, or transit gateway attachment state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetConnectAttachment`, `GetConnectPeer`, `GetConnectPeerAssociations`, `GetConnections`, `GetCoreNetwork`, `GetCoreNetworkChangeEvents`, `GetCoreNetworkChangeSet`, `GetCoreNetworkPolicy`, `GetCustomerGatewayAssociations`, `GetDevices`, `GetDirectConnectGatewayAttachment`, `GetLinkAssociations`, `GetLinks`, `GetNetworkResourceCounts`, `GetNetworkResourceRelationships`, `GetNetworkResources`, `GetNetworkRoutes`, `GetNetworkTelemetry`, `GetResourcePolicy`, `GetRouteAnalysis`, `GetSiteToSiteVpnAttachment`, `GetSites`, `GetTransitGatewayConnectPeerAssociations`, `GetTransitGatewayPeering`, `GetTransitGatewayRegistrations`, `GetTransitGatewayRouteTableAttachment`, `GetVpcAttachment`
- Traits: `paginated` (15)
- Common required input members in this group: `AttachmentId`, `ConnectPeerId`, `CoreNetworkId`, `GlobalNetworkId`, `PeeringId`, `PolicyVersionId`, `ResourceArn`, `RouteAnalysisId`, `RouteTableIdentifier`

### Create

- Operations: `CreateConnectAttachment`, `CreateConnectPeer`, `CreateConnection`, `CreateCoreNetwork`, `CreateCoreNetworkPrefixListAssociation`, `CreateDevice`, `CreateDirectConnectGatewayAttachment`, `CreateGlobalNetwork`, `CreateLink`, `CreateSite`, `CreateSiteToSiteVpnAttachment`, `CreateTransitGatewayPeering`, `CreateTransitGatewayRouteTableAttachment`, `CreateVpcAttachment`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `Bandwidth`, `ConnectAttachmentId`, `ConnectedDeviceId`, `CoreNetworkId`, `DeviceId`, `DirectConnectGatewayArn`, `EdgeLocation`, `EdgeLocations`, `GlobalNetworkId`, `Options`, `PeerAddress`, `PeeringId`, `PrefixListAlias`, `PrefixListArn`, `SiteId`, `SubnetArns`, `TransitGatewayArn`, `TransitGatewayRouteTableArn`, `TransportAttachmentId`, `VpcArn`, `VpnConnectionArn`

### Delete

- Operations: `DeleteAttachment`, `DeleteConnectPeer`, `DeleteConnection`, `DeleteCoreNetwork`, `DeleteCoreNetworkPolicyVersion`, `DeleteCoreNetworkPrefixListAssociation`, `DeleteDevice`, `DeleteGlobalNetwork`, `DeleteLink`, `DeletePeering`, `DeleteResourcePolicy`, `DeleteSite`
- Common required input members in this group: `AttachmentId`, `ConnectPeerId`, `ConnectionId`, `CoreNetworkId`, `DeviceId`, `GlobalNetworkId`, `LinkId`, `PeeringId`, `PolicyVersionId`, `PrefixListArn`, `ResourceArn`, `SiteId`

### List

- Operations: `ListAttachmentRoutingPolicyAssociations`, `ListAttachments`, `ListConnectPeers`, `ListCoreNetworkPolicyVersions`, `ListCoreNetworkPrefixListAssociations`, `ListCoreNetworkRoutingInformation`, `ListCoreNetworks`, `ListOrganizationServiceAccessStatus`, `ListPeerings`, `ListTagsForResource`
- Traits: `paginated` (8)
- Common required input members in this group: `CoreNetworkId`, `EdgeLocation`, `ResourceArn`, `SegmentName`

### Update

- Operations: `UpdateConnection`, `UpdateCoreNetwork`, `UpdateDevice`, `UpdateDirectConnectGatewayAttachment`, `UpdateGlobalNetwork`, `UpdateLink`, `UpdateNetworkResourceMetadata`, `UpdateSite`, `UpdateVpcAttachment`
- Common required input members in this group: `AttachmentId`, `ConnectionId`, `CoreNetworkId`, `DeviceId`, `GlobalNetworkId`, `LinkId`, `Metadata`, `ResourceArn`, `SiteId`

### Associate

- Operations: `AssociateConnectPeer`, `AssociateCustomerGateway`, `AssociateLink`, `AssociateTransitGatewayConnectPeer`
- Common required input members in this group: `ConnectPeerId`, `CustomerGatewayArn`, `DeviceId`, `GlobalNetworkId`, `LinkId`, `TransitGatewayConnectPeerArn`

### Disassociate

- Operations: `DisassociateConnectPeer`, `DisassociateCustomerGateway`, `DisassociateLink`, `DisassociateTransitGatewayConnectPeer`
- Common required input members in this group: `ConnectPeerId`, `CustomerGatewayArn`, `DeviceId`, `GlobalNetworkId`, `LinkId`, `TransitGatewayConnectPeerArn`

### Put

- Operations: `PutAttachmentRoutingPolicyLabel`, `PutCoreNetworkPolicy`, `PutResourcePolicy`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `AttachmentId`, `CoreNetworkId`, `PolicyDocument`, `ResourceArn`, `RoutingPolicyLabel`

### Start

- Operations: `StartOrganizationServiceAccessUpdate`, `StartRouteAnalysis`
- Common required input members in this group: `Action`, `Destination`, `GlobalNetworkId`, `Source`

### Accept

- Operations: `AcceptAttachment`
- Common required input members in this group: `AttachmentId`

### Deregister

- Operations: `DeregisterTransitGateway`
- Common required input members in this group: `GlobalNetworkId`, `TransitGatewayArn`

### Describe

- Operations: `DescribeGlobalNetworks`
- Traits: `paginated` (1)

### Execute

- Operations: `ExecuteCoreNetworkChangeSet`
- Common required input members in this group: `CoreNetworkId`, `PolicyVersionId`

### Register

- Operations: `RegisterTransitGateway`
- Common required input members in this group: `GlobalNetworkId`, `TransitGatewayArn`

### Reject

- Operations: `RejectAttachment`
- Common required input members in this group: `AttachmentId`

### Remove

- Operations: `RemoveAttachmentRoutingPolicyLabel`
- Common required input members in this group: `AttachmentId`, `CoreNetworkId`

### Restore

- Operations: `RestoreCoreNetworkPolicyVersion`
- Common required input members in this group: `CoreNetworkId`, `PolicyVersionId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptAttachment` | `POST /attachments/{AttachmentId}/accept` | - | `AttachmentId` | - | `AcceptAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Accepts a core network attachment request. Once the attachment request is accepted by a core network owner, the attachment is created and connected to a core network. |
| `AssociateConnectPeer` | `POST /global-networks/{GlobalNetworkId}/connect-peer-associations` | - | `ConnectPeerId`, `DeviceId`, `GlobalNetworkId` | - | `AssociateConnectPeerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a core network Connect peer with a device and optionally, with a link. If you specify a link, it must be associated with the specified device. |
| `AssociateCustomerGateway` | `POST /global-networks/{GlobalNetworkId}/customer-gateway-associations` | - | `CustomerGatewayArn`, `DeviceId`, `GlobalNetworkId` | - | `AssociateCustomerGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a customer gateway with a device and optionally, with a link. If you specify a link, it must be associated with the specified device. |
| `AssociateLink` | `POST /global-networks/{GlobalNetworkId}/link-associations` | - | `DeviceId`, `GlobalNetworkId`, `LinkId` | - | `AssociateLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a link to a device. A device can be associated to multiple links and a link can be associated to multiple devices. |
| `AssociateTransitGatewayConnectPeer` | `POST /global-networks/{GlobalNetworkId}/transit-gateway-connect-peer-associations` | - | `DeviceId`, `GlobalNetworkId`, `TransitGatewayConnectPeerArn` | - | `AssociateTransitGatewayConnectPeerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a transit gateway Connect peer with a device, and optionally, with a link. If you specify a link, it must be associated with the specified device. |
| `CreateConnectAttachment` | `POST /connect-attachments` | `idempotency-token` | `CoreNetworkId`, `EdgeLocation`, `Options`, `TransportAttachmentId` | `ClientToken` | `CreateConnectAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a core network Connect attachment from a specified core network attachment. A core network Connect attachment is a GRE-based tunnel attachment that you can use to establish a connection between a core network and an appliance. |
| `CreateConnectPeer` | `POST /connect-peers` | `idempotency-token` | `ConnectAttachmentId`, `PeerAddress` | `ClientToken` | `CreateConnectPeerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a core network Connect peer for a specified core network connect attachment between a core network and an appliance. The peer address and transit gateway address must be the same IP address family (IPv4 or IPv6). |
| `CreateConnection` | `POST /global-networks/{GlobalNetworkId}/connections` | - | `ConnectedDeviceId`, `DeviceId`, `GlobalNetworkId` | - | `CreateConnectionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a connection between two devices. The devices can be a physical or virtual appliance that connects to a third-party appliance in a VPC, or a physical appliance that connects to another physical appliance in an on-premises network. |
| `CreateCoreNetwork` | `POST /core-networks` | `idempotency-token` | `GlobalNetworkId` | `ClientToken` | `CreateCoreNetworkResponse` | `AccessDeniedException`, `ConflictException`, `CoreNetworkPolicyException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a core network as part of your global network, and optionally, with a core network policy. |
| `CreateCoreNetworkPrefixListAssociation` | `POST /prefix-list` | `idempotency-token` | `CoreNetworkId`, `PrefixListAlias`, `PrefixListArn` | `ClientToken` | `CreateCoreNetworkPrefixListAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an association between a core network and a prefix list for routing control. |
| `CreateDevice` | `POST /global-networks/{GlobalNetworkId}/devices` | - | `GlobalNetworkId` | - | `CreateDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new device in a global network. If you specify both a site ID and a location, the location of the site is used for visualization in the Network Manager console. |
| `CreateDirectConnectGatewayAttachment` | `POST /direct-connect-gateway-attachments` | `idempotency-token` | `CoreNetworkId`, `DirectConnectGatewayArn`, `EdgeLocations` | `ClientToken` | `CreateDirectConnectGatewayAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services Direct Connect gateway attachment |
| `CreateGlobalNetwork` | `POST /global-networks` | - | - | - | `CreateGlobalNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new, empty global network. |
| `CreateLink` | `POST /global-networks/{GlobalNetworkId}/links` | - | `Bandwidth`, `GlobalNetworkId`, `SiteId` | - | `CreateLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new link for a specified site. |
| `CreateSite` | `POST /global-networks/{GlobalNetworkId}/sites` | - | `GlobalNetworkId` | - | `CreateSiteResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new site in a global network. |
| `CreateSiteToSiteVpnAttachment` | `POST /site-to-site-vpn-attachments` | `idempotency-token` | `CoreNetworkId`, `VpnConnectionArn` | `ClientToken` | `CreateSiteToSiteVpnAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Web Services site-to-site VPN attachment on an edge location of a core network. |
| `CreateTransitGatewayPeering` | `POST /transit-gateway-peerings` | `idempotency-token` | `CoreNetworkId`, `TransitGatewayArn` | `ClientToken` | `CreateTransitGatewayPeeringResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a transit gateway peering connection. |
| `CreateTransitGatewayRouteTableAttachment` | `POST /transit-gateway-route-table-attachments` | `idempotency-token` | `PeeringId`, `TransitGatewayRouteTableArn` | `ClientToken` | `CreateTransitGatewayRouteTableAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a transit gateway route table attachment. |
| `CreateVpcAttachment` | `POST /vpc-attachments` | `idempotency-token` | `CoreNetworkId`, `SubnetArns`, `VpcArn` | `ClientToken` | `CreateVpcAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a VPC attachment on an edge location of a core network. |
| `DeleteAttachment` | `DELETE /attachments/{AttachmentId}` | - | `AttachmentId` | - | `DeleteAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an attachment. Supports all attachment types. |
| `DeleteConnectPeer` | `DELETE /connect-peers/{ConnectPeerId}` | - | `ConnectPeerId` | - | `DeleteConnectPeerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Connect peer. |
| `DeleteConnection` | `DELETE /global-networks/{GlobalNetworkId}/connections/{ConnectionId}` | - | `ConnectionId`, `GlobalNetworkId` | - | `DeleteConnectionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified connection in your global network. |
| `DeleteCoreNetwork` | `DELETE /core-networks/{CoreNetworkId}` | - | `CoreNetworkId` | - | `DeleteCoreNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a core network along with all core network policies. This can only be done if there are no attachments on a core network. |
| `DeleteCoreNetworkPolicyVersion` | `DELETE /core-networks/{CoreNetworkId}/core-network-policy-versions/{PolicyVersionId}` | - | `CoreNetworkId`, `PolicyVersionId` | - | `DeleteCoreNetworkPolicyVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a policy version from a core network. You can't delete the current LIVE policy. |
| `DeleteCoreNetworkPrefixListAssociation` | `DELETE /prefix-list/{PrefixListArn}/core-network/{CoreNetworkId}` | - | `CoreNetworkId`, `PrefixListArn` | - | `DeleteCoreNetworkPrefixListAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes an association between a core network and a prefix list. |
| `DeleteDevice` | `DELETE /global-networks/{GlobalNetworkId}/devices/{DeviceId}` | - | `DeviceId`, `GlobalNetworkId` | - | `DeleteDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing device. You must first disassociate the device from any links and customer gateways. |
| `DeleteGlobalNetwork` | `DELETE /global-networks/{GlobalNetworkId}` | - | `GlobalNetworkId` | - | `DeleteGlobalNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing global network. You must first delete all global network objects (devices, links, and sites), deregister all transit gateways, and delete any core networks. |
| `DeleteLink` | `DELETE /global-networks/{GlobalNetworkId}/links/{LinkId}` | - | `GlobalNetworkId`, `LinkId` | - | `DeleteLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing link. You must first disassociate the link from any devices and customer gateways. |
| `DeletePeering` | `DELETE /peerings/{PeeringId}` | - | `PeeringId` | - | `DeletePeeringResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing peering connection. |
| `DeleteResourcePolicy` | `DELETE /resource-policy/{ResourceArn}` | - | `ResourceArn` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a resource policy for the specified resource. This revokes the access of the principals specified in the resource policy. |
| `DeleteSite` | `DELETE /global-networks/{GlobalNetworkId}/sites/{SiteId}` | - | `GlobalNetworkId`, `SiteId` | - | `DeleteSiteResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing site. The site cannot be associated with any device or link. |
| `DeregisterTransitGateway` | `DELETE /global-networks/{GlobalNetworkId}/transit-gateway-registrations/{TransitGatewayArn}` | - | `GlobalNetworkId`, `TransitGatewayArn` | - | `DeregisterTransitGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deregisters a transit gateway from your global network. This action does not delete your transit gateway, or modify any of its attachments. |
| `DescribeGlobalNetworks` | `GET /global-networks` | `paginated` | - | - | `DescribeGlobalNetworksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes one or more global networks. By default, all global networks are described. |
| `DisassociateConnectPeer` | `DELETE /global-networks/{GlobalNetworkId}/connect-peer-associations/{ConnectPeerId}` | - | `ConnectPeerId`, `GlobalNetworkId` | - | `DisassociateConnectPeerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a core network Connect peer from a device and a link. |
| `DisassociateCustomerGateway` | `DELETE /global-networks/{GlobalNetworkId}/customer-gateway-associations/{CustomerGatewayArn}` | - | `CustomerGatewayArn`, `GlobalNetworkId` | - | `DisassociateCustomerGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a customer gateway from a device and a link. |
| `DisassociateLink` | `DELETE /global-networks/{GlobalNetworkId}/link-associations` | - | `DeviceId`, `GlobalNetworkId`, `LinkId` | - | `DisassociateLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates an existing device from a link. You must first disassociate any customer gateways that are associated with the link. |
| `DisassociateTransitGatewayConnectPeer` | `DELETE /global-networks/{GlobalNetworkId}/transit-gateway-connect-peer-associations/{TransitGatewayConnectPeerArn}` | - | `GlobalNetworkId`, `TransitGatewayConnectPeerArn` | - | `DisassociateTransitGatewayConnectPeerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a transit gateway Connect peer from a device and link. |
| `ExecuteCoreNetworkChangeSet` | `POST /core-networks/{CoreNetworkId}/core-network-change-sets/{PolicyVersionId}/execute` | - | `CoreNetworkId`, `PolicyVersionId` | - | `ExecuteCoreNetworkChangeSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Executes a change set on your core network. Deploys changes globally based on the policy submitted.. |
| `GetConnectAttachment` | `GET /connect-attachments/{AttachmentId}` | - | `AttachmentId` | - | `GetConnectAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a core network Connect attachment. |
| `GetConnectPeer` | `GET /connect-peers/{ConnectPeerId}` | - | `ConnectPeerId` | - | `GetConnectPeerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a core network Connect peer. |
| `GetConnectPeerAssociations` | `GET /global-networks/{GlobalNetworkId}/connect-peer-associations` | `paginated` | `GlobalNetworkId` | - | `GetConnectPeerAssociationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a core network Connect peer associations. |
| `GetConnections` | `GET /global-networks/{GlobalNetworkId}/connections` | `paginated` | `GlobalNetworkId` | - | `GetConnectionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about one or more of your connections in a global network. |
| `GetCoreNetwork` | `GET /core-networks/{CoreNetworkId}` | - | `CoreNetworkId` | - | `GetCoreNetworkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the LIVE policy for a core network. |
| `GetCoreNetworkChangeEvents` | `GET /core-networks/{CoreNetworkId}/core-network-change-events/{PolicyVersionId}` | `paginated` | `CoreNetworkId`, `PolicyVersionId` | - | `GetCoreNetworkChangeEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a core network change event. |
| `GetCoreNetworkChangeSet` | `GET /core-networks/{CoreNetworkId}/core-network-change-sets/{PolicyVersionId}` | `paginated` | `CoreNetworkId`, `PolicyVersionId` | - | `GetCoreNetworkChangeSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a change set between the LIVE core network policy and a submitted policy. |
| `GetCoreNetworkPolicy` | `GET /core-networks/{CoreNetworkId}/core-network-policy` | - | `CoreNetworkId` | - | `GetCoreNetworkPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about a core network policy. You can get details about your current live policy or any previous policy version. |
| `GetCustomerGatewayAssociations` | `GET /global-networks/{GlobalNetworkId}/customer-gateway-associations` | `paginated` | `GlobalNetworkId` | - | `GetCustomerGatewayAssociationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the association information for customer gateways that are associated with devices and links in your global network. |
| `GetDevices` | `GET /global-networks/{GlobalNetworkId}/devices` | `paginated` | `GlobalNetworkId` | - | `GetDevicesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about one or more of your devices in a global network. |
| `GetDirectConnectGatewayAttachment` | `GET /direct-connect-gateway-attachments/{AttachmentId}` | - | `AttachmentId` | - | `GetDirectConnectGatewayAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific Amazon Web Services Direct Connect gateway attachment. |
| `GetLinkAssociations` | `GET /global-networks/{GlobalNetworkId}/link-associations` | `paginated` | `GlobalNetworkId` | - | `GetLinkAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the link associations for a device or a link. Either the device ID or the link ID must be specified. |
| `GetLinks` | `GET /global-networks/{GlobalNetworkId}/links` | `paginated` | `GlobalNetworkId` | - | `GetLinksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about one or more links in a specified global network. If you specify the site ID, you cannot specify the type or provider in the same request. |
| `GetNetworkResourceCounts` | `GET /global-networks/{GlobalNetworkId}/network-resource-count` | `paginated` | `GlobalNetworkId` | - | `GetNetworkResourceCountsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets the count of network resources, by resource type, for the specified global network. |
| `GetNetworkResourceRelationships` | `GET /global-networks/{GlobalNetworkId}/network-resource-relationships` | `paginated` | `GlobalNetworkId` | - | `GetNetworkResourceRelationshipsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the network resource relationships for the specified global network. |
| `GetNetworkResources` | `GET /global-networks/{GlobalNetworkId}/network-resources` | `paginated` | `GlobalNetworkId` | - | `GetNetworkResourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the network resources for the specified global network. The results include information from the corresponding Describe call for the resource, minus any sensitive information such as pre-shared keys. |
| `GetNetworkRoutes` | `POST /global-networks/{GlobalNetworkId}/network-routes` | - | `GlobalNetworkId`, `RouteTableIdentifier` | - | `GetNetworkRoutesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the network routes of the specified global network. |
| `GetNetworkTelemetry` | `GET /global-networks/{GlobalNetworkId}/network-telemetry` | `paginated` | `GlobalNetworkId` | - | `GetNetworkTelemetryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the network telemetry of the specified global network. |
| `GetResourcePolicy` | `GET /resource-policy/{ResourceArn}` | - | `ResourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about a resource policy. |
| `GetRouteAnalysis` | `GET /global-networks/{GlobalNetworkId}/route-analyses/{RouteAnalysisId}` | - | `GlobalNetworkId`, `RouteAnalysisId` | - | `GetRouteAnalysisResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about the specified route analysis. |
| `GetSiteToSiteVpnAttachment` | `GET /site-to-site-vpn-attachments/{AttachmentId}` | - | `AttachmentId` | - | `GetSiteToSiteVpnAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a site-to-site VPN attachment. |
| `GetSites` | `GET /global-networks/{GlobalNetworkId}/sites` | `paginated` | `GlobalNetworkId` | - | `GetSitesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about one or more of your sites in a global network. |
| `GetTransitGatewayConnectPeerAssociations` | `GET /global-networks/{GlobalNetworkId}/transit-gateway-connect-peer-associations` | `paginated` | `GlobalNetworkId` | - | `GetTransitGatewayConnectPeerAssociationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about one or more of your transit gateway Connect peer associations in a global network. |
| `GetTransitGatewayPeering` | `GET /transit-gateway-peerings/{PeeringId}` | - | `PeeringId` | - | `GetTransitGatewayPeeringResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a transit gateway peer. |
| `GetTransitGatewayRegistrations` | `GET /global-networks/{GlobalNetworkId}/transit-gateway-registrations` | `paginated` | `GlobalNetworkId` | - | `GetTransitGatewayRegistrationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about the transit gateway registrations in a specified global network. |
| `GetTransitGatewayRouteTableAttachment` | `GET /transit-gateway-route-table-attachments/{AttachmentId}` | - | `AttachmentId` | - | `GetTransitGatewayRouteTableAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a transit gateway route table attachment. |
| `GetVpcAttachment` | `GET /vpc-attachments/{AttachmentId}` | - | `AttachmentId` | - | `GetVpcAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a VPC attachment. |
| `ListAttachmentRoutingPolicyAssociations` | `GET /routing-policy-label/core-network/{CoreNetworkId}` | `paginated` | `CoreNetworkId` | - | `ListAttachmentRoutingPolicyAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the routing policy associations for attachments in a core network. |
| `ListAttachments` | `GET /attachments` | `paginated` | - | - | `ListAttachmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of core network attachments. |
| `ListConnectPeers` | `GET /connect-peers` | `paginated` | - | - | `ListConnectPeersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of core network Connect peers. |
| `ListCoreNetworkPolicyVersions` | `GET /core-networks/{CoreNetworkId}/core-network-policy-versions` | `paginated` | `CoreNetworkId` | - | `ListCoreNetworkPolicyVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of core network policy versions. |
| `ListCoreNetworkPrefixListAssociations` | `GET /prefix-list/core-network/{CoreNetworkId}` | `paginated` | `CoreNetworkId` | - | `ListCoreNetworkPrefixListAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the prefix list associations for a core network. |
| `ListCoreNetworkRoutingInformation` | `POST /core-networks/{CoreNetworkId}/core-network-routing-information` | `paginated` | `CoreNetworkId`, `EdgeLocation`, `SegmentName` | - | `ListCoreNetworkRoutingInformationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists routing information for a core network, including routes and their attributes. |
| `ListCoreNetworks` | `GET /core-networks` | `paginated` | - | - | `ListCoreNetworksResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of owned and shared core networks. |
| `ListOrganizationServiceAccessStatus` | `GET /organizations/service-access` | - | - | - | `ListOrganizationServiceAccessStatusResponse` | - | Gets the status of the Service Linked Role (SLR) deployment for the accounts in a given Amazon Web Services Organization. |
| `ListPeerings` | `GET /peerings` | `paginated` | - | - | `ListPeeringsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the peerings for a core network. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags for a specified resource. |
| `PutAttachmentRoutingPolicyLabel` | `POST /routing-policy-label` | `idempotency-token` | `AttachmentId`, `CoreNetworkId`, `RoutingPolicyLabel` | `ClientToken` | `PutAttachmentRoutingPolicyLabelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Applies a routing policy label to an attachment for traffic routing decisions. |
| `PutCoreNetworkPolicy` | `POST /core-networks/{CoreNetworkId}/core-network-policy` | `idempotency-token` | `CoreNetworkId`, `PolicyDocument` | `ClientToken` | `PutCoreNetworkPolicyResponse` | `AccessDeniedException`, `ConflictException`, `CoreNetworkPolicyException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new, immutable version of a core network policy. A subsequent change set is created showing the differences between the LIVE policy and the submitted policy. |
| `PutResourcePolicy` | `POST /resource-policy/{ResourceArn}` | - | `PolicyDocument`, `ResourceArn` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates or updates a resource policy. |
| `RegisterTransitGateway` | `POST /global-networks/{GlobalNetworkId}/transit-gateway-registrations` | - | `GlobalNetworkId`, `TransitGatewayArn` | - | `RegisterTransitGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Registers a transit gateway in your global network. Not all Regions support transit gateways for global networks. |
| `RejectAttachment` | `POST /attachments/{AttachmentId}/reject` | - | `AttachmentId` | - | `RejectAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rejects a core network attachment request. |
| `RemoveAttachmentRoutingPolicyLabel` | `DELETE /routing-policy-label/core-network/{CoreNetworkId}/attachment/{AttachmentId}` | - | `AttachmentId`, `CoreNetworkId` | - | `RemoveAttachmentRoutingPolicyLabelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes a routing policy label from an attachment. |
| `RestoreCoreNetworkPolicyVersion` | `POST /core-networks/{CoreNetworkId}/core-network-policy-versions/{PolicyVersionId}/restore` | - | `CoreNetworkId`, `PolicyVersionId` | - | `RestoreCoreNetworkPolicyVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Restores a previous policy version as a new, immutable version of a core network policy. A subsequent change set is created showing the differences between the LIVE policy and restored policy. |
| `StartOrganizationServiceAccessUpdate` | `POST /organizations/service-access` | - | `Action` | - | `StartOrganizationServiceAccessUpdateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables the Network Manager service for an Amazon Web Services Organization. This can only be called by a management account within the organization. |
| `StartRouteAnalysis` | `POST /global-networks/{GlobalNetworkId}/route-analyses` | - | `Destination`, `GlobalNetworkId`, `Source` | - | `StartRouteAnalysisResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts analyzing the routing path between the specified source and destination. For more information, see Route Analyzer. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Tags a specified resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a specified resource. |
| `UpdateConnection` | `PATCH /global-networks/{GlobalNetworkId}/connections/{ConnectionId}` | - | `ConnectionId`, `GlobalNetworkId` | - | `UpdateConnectionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the information for an existing connection. To remove information for any of the parameters, specify an empty string. |
| `UpdateCoreNetwork` | `PATCH /core-networks/{CoreNetworkId}` | - | `CoreNetworkId` | - | `UpdateCoreNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the description of a core network. |
| `UpdateDevice` | `PATCH /global-networks/{GlobalNetworkId}/devices/{DeviceId}` | - | `DeviceId`, `GlobalNetworkId` | - | `UpdateDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the details for an existing device. To remove information for any of the parameters, specify an empty string. |
| `UpdateDirectConnectGatewayAttachment` | `PATCH /direct-connect-gateway-attachments/{AttachmentId}` | - | `AttachmentId` | - | `UpdateDirectConnectGatewayAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the edge locations associated with an Amazon Web Services Direct Connect gateway attachment. |
| `UpdateGlobalNetwork` | `PATCH /global-networks/{GlobalNetworkId}` | - | `GlobalNetworkId` | - | `UpdateGlobalNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing global network. To remove information for any of the parameters, specify an empty string. |
| `UpdateLink` | `PATCH /global-networks/{GlobalNetworkId}/links/{LinkId}` | - | `GlobalNetworkId`, `LinkId` | - | `UpdateLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the details for an existing link. To remove information for any of the parameters, specify an empty string. |
| `UpdateNetworkResourceMetadata` | `PATCH /global-networks/{GlobalNetworkId}/network-resources/{ResourceArn}/metadata` | - | `GlobalNetworkId`, `Metadata`, `ResourceArn` | - | `UpdateNetworkResourceMetadataResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the resource metadata for the specified global network. |
| `UpdateSite` | `PATCH /global-networks/{GlobalNetworkId}/sites/{SiteId}` | - | `GlobalNetworkId`, `SiteId` | - | `UpdateSiteResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the information for an existing site. To remove information for any of the parameters, specify an empty string. |
| `UpdateVpcAttachment` | `PATCH /vpc-attachments/{AttachmentId}` | - | `AttachmentId` | - | `UpdateVpcAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a VPC attachment. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message`, `RetryAfterSeconds` | The request has failed due to an internal error. |
| `ThrottlingException` | `structure` | `Message`, `RetryAfterSeconds` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Fields`, `Message`, `Reason` | The input fails to satisfy the constraints. |
| `ResourceNotFoundException` | `structure` | `Context`, `Message`, `ResourceId`, `ResourceType` | The specified resource could not be found. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | There was a conflict processing the request. |
| `ServiceQuotaExceededException` | `structure` | `LimitCode`, `Message`, `ResourceId`, `ResourceType`, `ServiceCode` | A service limit was exceeded. |
| `CoreNetworkPolicyException` | `structure` | `Errors`, `Message` | Describes a core network policy exception. |
| `AcceptAttachmentRequest` | `structure` | `AttachmentId` | - |
| `AcceptAttachmentResponse` | `structure` | `Attachment` | - |
| `AssociateConnectPeerRequest` | `structure` | `ConnectPeerId`, `DeviceId`, `GlobalNetworkId`, `LinkId` | - |
| `AssociateConnectPeerResponse` | `structure` | `ConnectPeerAssociation` | - |
| `AssociateCustomerGatewayRequest` | `structure` | `CustomerGatewayArn`, `DeviceId`, `GlobalNetworkId`, `LinkId` | - |
| `AssociateCustomerGatewayResponse` | `structure` | `CustomerGatewayAssociation` | - |
| `AssociateLinkRequest` | `structure` | `DeviceId`, `GlobalNetworkId`, `LinkId` | - |
| `AssociateLinkResponse` | `structure` | `LinkAssociation` | - |
| `AssociateTransitGatewayConnectPeerRequest` | `structure` | `DeviceId`, `GlobalNetworkId`, `LinkId`, `TransitGatewayConnectPeerArn` | - |
| `AssociateTransitGatewayConnectPeerResponse` | `structure` | `TransitGatewayConnectPeerAssociation` | - |
| `CreateConnectAttachmentRequest` | `structure` | `ClientToken`, `CoreNetworkId`, `EdgeLocation`, `Options`, `RoutingPolicyLabel`, `Tags`, `TransportAttachmentId` | - |
| `CreateConnectAttachmentResponse` | `structure` | `ConnectAttachment` | - |
| `CreateConnectPeerRequest` | `structure` | `BgpOptions`, `ClientToken`, `ConnectAttachmentId`, `CoreNetworkAddress`, `InsideCidrBlocks`, `PeerAddress`, `SubnetArn`, `Tags` | - |
| `CreateConnectPeerResponse` | `structure` | `ConnectPeer` | - |
| `CreateConnectionRequest` | `structure` | `ConnectedDeviceId`, `ConnectedLinkId`, `Description`, `DeviceId`, `GlobalNetworkId`, `LinkId`, `Tags` | - |
| `CreateConnectionResponse` | `structure` | `Connection` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
