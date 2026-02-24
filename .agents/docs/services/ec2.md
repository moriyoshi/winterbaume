# Amazon Elastic Compute Cloud

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elastic Compute Cloud You can access the features of Amazon Elastic Compute Cloud (Amazon EC2) programmatically. For more information, see the Amazon EC2 Developer Guide.

## Possible Usage Scenarios

- Backported from `crates/winterbaume-ec2/tests/scenario_test.rs`: bootstrap a three-tier VPC with subnets, routing, security groups, and supporting network resources.
- Backported from `scenario_test.rs`: launch a web fleet with Elastic IP, EBS volume/snapshot, AMI creation, and cleanup.
- Backported from `scenario_test.rs`: perform blue/green deployment through launch-template versioning.
- Backported from `scenario_test.rs`: model private-subnet egress through a NAT gateway, VPC peering handshakes, cross-AZ disaster recovery through snapshot restore, custom DHCP options, and hybrid VPN connectivity.
- Backported from `scenario_test.rs`: re-associate an Elastic IP during canary failover, preserving the allocation while minting a fresh association ID and updating `DescribeAddresses` to the replacement instance.
- Backported from `scenario_test.rs`: enforce account-wide EBS encryption-by-default so new implicit volumes become encrypted after enablement and return to the unencrypted default after disablement.
- Backported from `scenario_test.rs`: lock down a sensitive subnet by replacing its default network ACL association with a custom deny-by-default ACL that exposes only the intended bastion allow rule.
- Backported from `scenario_test.rs`: walk an instance through running, stopped, restarted, rebooted, stopped, and terminated states while checking state-change responses against `DescribeInstances`.
- From the AWS documentation and model: support compute, storage, image, network, security, peering, VPN, NAT, launch template, tagging, and state-transition workflows across the broad EC2 control-plane surface.

## Service Identity and Protocol

- AWS model slug: `ec2`
- AWS SDK for Rust slug: `ec2`
- Model version: `2016-11-15`
- Model file: `vendor/api-models-aws/models/ec2/service/2016-11-15/ec2-2016-11-15.json`
- SDK ID: `EC2`
- Endpoint prefix: `ec2`
- ARN namespace: `ec2`
- CloudFormation name: `EC2`
- CloudTrail event source: `ec2.amazonaws.com`
- Protocols: `ec2Query`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (185), `Create` (104), `Delete` (92), `Modify` (77), `Get` (68), `Enable` (23), `Disable` (21), `Associate` (19).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptAddressTransfer`, `AcceptCapacityReservationBillingOwnership`, `AcceptReservedInstancesExchangeQuote`, `AcceptTransitGatewayMulticastDomainAssociations`, `AcceptTransitGatewayPeeringAttachment`, `AcceptTransitGatewayVpcAttachment`, `AcceptVpcEndpointConnections`, `AcceptVpcPeeringConnection`, `AssociateAddress`, `AssociateCapacityReservationBillingOwner`, `AssociateClientVpnTargetNetwork`, `AssociateDhcpOptions`, `AssociateEnclaveCertificateIamRole`, `AssociateIamInstanceProfile`, `AssociateInstanceEventWindow`, `AssociateIpamByoasn`, `AssociateIpamResourceDiscovery`, `AssociateNatGatewayAddress`, `AssociateRouteServer`, `AssociateRouteTable`, ... (+406).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountAttributes`, `DescribeAddressTransfers`, `DescribeAddresses`, `DescribeAddressesAttribute`, `DescribeAggregateIdFormat`, `DescribeAvailabilityZones`, `DescribeAwsNetworkPerformanceMetricSubscriptions`, `DescribeBundleTasks`, `DescribeByoipCidrs`, `DescribeCapacityBlockExtensionHistory`, `DescribeCapacityBlockExtensionOfferings`, `DescribeCapacityBlockOfferings`, `DescribeCapacityBlockStatus`, `DescribeCapacityBlocks`, `DescribeCapacityManagerDataExports`, `DescribeCapacityReservationBillingRequests`, `DescribeCapacityReservationFleets`, `DescribeCapacityReservationTopology`, `DescribeCapacityReservations`, `DescribeCarrierGateways`, ... (+239).
- Pagination is modelled for 167 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 79 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelBundleTask`, `CancelCapacityReservation`, `CancelCapacityReservationFleets`, `CancelConversionTask`, `CancelDeclarativePoliciesReport`, `CancelExportTask`, `CancelImageLaunchPermission`, `CancelImportTask`, `CancelReservedInstancesListing`, `CancelSpotFleetRequests`, `CancelSpotInstanceRequests`, `CreateCapacityManagerDataExport`, `CreateDelegateMacVolumeOwnershipTask`, `CreateImageUsageReport`, `CreateInstanceExportTask`, `CreateMacSystemIntegrityProtectionModificationTask`, `CreateReplaceRootVolumeTask`, `CreateRestoreImageTask`, `CreateStoreImageTask`, `DeleteCapacityManagerDataExport`, ... (+36).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SNS`, `SQS`, `Lambda`, `EC2/VPC`, `ECS`, `RDS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html
- https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html
- https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DisassociateSecurityGroupVpc.html

Research outcomes:
- EC2 resources are strongly regional, and many APIs expose Availability Zone, Local Zone, Wavelength Zone, and Outposts placement concepts.
- Instance networking includes private IPv4, public IPv4, IPv6, DNS hostnames, multiple IP addresses, and multiple network interfaces.
- Security groups are associated with VPC/network-interface semantics; some association and disassociation operations are asynchronous and constrained by existing ENI usage.
- Many EC2 operations return state transitions rather than immediate terminal state, especially for instances, volumes, snapshots, VPC attachments, and managed networking resources.
- Describe APIs typically support rich filters and pagination, and many resource families use IDs that encode resource type.

Parity implications:
- EC2 parity depends on modelling relationships among VPCs, subnets, route tables, gateways, ENIs, instances, addresses, security groups, volumes, and snapshots.
- Asynchronous state transitions should be explicit, even if the emulator advances them deterministically.
- Describe filters, pagination, and dependency violations are as important as create/delete happy paths.

## Current Network Resource Stub Semantics

EC2 is the only crate that currently owns in-service VPC/network maps, but that state is not shared as a network oracle for other services.

- EC2 state contains local maps for VPCs, subnets, route tables, internet gateways, NAT gateways, network ACLs, security groups, network interfaces, elastic IPs, VPC endpoints, transit gateways, VPN resources, client VPN, verified access, route servers, and related associations.
- Many EC2 operations validate against those EC2-local maps and return EC2-shaped not-found or invalid-state errors, while other advanced families still synthesise IDs or use simplified association records.
- Other network-aware service crates currently store EC2 identifiers independently. Creating a subnet, security group, VPC endpoint, or ENI in EC2 does not make non-EC2 services validate against it, and non-EC2 services do not create back-references in EC2.

## Operation Groups

### Describe

- Operations: `DescribeAccountAttributes`, `DescribeAddressTransfers`, `DescribeAddresses`, `DescribeAddressesAttribute`, `DescribeAggregateIdFormat`, `DescribeAvailabilityZones`, `DescribeAwsNetworkPerformanceMetricSubscriptions`, `DescribeBundleTasks`, `DescribeByoipCidrs`, `DescribeCapacityBlockExtensionHistory`, `DescribeCapacityBlockExtensionOfferings`, `DescribeCapacityBlockOfferings`, `DescribeCapacityBlockStatus`, `DescribeCapacityBlocks`, `DescribeCapacityManagerDataExports`, `DescribeCapacityReservationBillingRequests`, `DescribeCapacityReservationFleets`, `DescribeCapacityReservationTopology`, `DescribeCapacityReservations`, `DescribeCarrierGateways`, `DescribeClassicLinkInstances`, `DescribeClientVpnAuthorizationRules`, `DescribeClientVpnConnections`, `DescribeClientVpnEndpoints`, `DescribeClientVpnRoutes`, `DescribeClientVpnTargetNetworks`, `DescribeCoipPools`, `DescribeConversionTasks`, `DescribeCustomerGateways`, `DescribeDeclarativePoliciesReports`, `DescribeDhcpOptions`, `DescribeEgressOnlyInternetGateways`, `DescribeElasticGpus`, `DescribeExportImageTasks`, `DescribeExportTasks`, `DescribeFastLaunchImages`, `DescribeFastSnapshotRestores`, `DescribeFleetHistory`, `DescribeFleetInstances`, `DescribeFleets`, ... (+145)
- Traits: `paginated` (135)
- Common required input members in this group: `Attribute`, `CapacityBlockExtensionDurationHours`, `CapacityDurationHours`, `CapacityReservationId`, `ClientVpnEndpointId`, `FirstSlotStartTimeRange`, `FleetId`, `FpgaImageId`, `GroupId`, `ImageId`, `ImageIds`, `InstanceId`, `MaxResults`, `NetworkInterfaceId`, `PrincipalArn`, `Recurrence`, `Role`, `ServiceId`, `SnapshotId`, `SpotFleetRequestId`, `StartTime`, `VolumeId`, `VpcId`

### Create

- Operations: `CreateCapacityManagerDataExport`, `CreateCapacityReservation`, `CreateCapacityReservationBySplitting`, `CreateCapacityReservationFleet`, `CreateCarrierGateway`, `CreateClientVpnEndpoint`, `CreateClientVpnRoute`, `CreateCoipCidr`, `CreateCoipPool`, `CreateCustomerGateway`, `CreateDefaultSubnet`, `CreateDefaultVpc`, `CreateDelegateMacVolumeOwnershipTask`, `CreateDhcpOptions`, `CreateEgressOnlyInternetGateway`, `CreateFleet`, `CreateFlowLogs`, `CreateFpgaImage`, `CreateImage`, `CreateImageUsageReport`, `CreateInstanceConnectEndpoint`, `CreateInstanceEventWindow`, `CreateInstanceExportTask`, `CreateInternetGateway`, `CreateInterruptibleCapacityReservationAllocation`, `CreateIpam`, `CreateIpamExternalResourceVerificationToken`, `CreateIpamPolicy`, `CreateIpamPool`, `CreateIpamPrefixListResolver`, `CreateIpamPrefixListResolverTarget`, `CreateIpamResourceDiscovery`, `CreateIpamScope`, `CreateKeyPair`, `CreateLaunchTemplate`, `CreateLaunchTemplateVersion`, `CreateLocalGatewayRoute`, `CreateLocalGatewayRouteTable`, `CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation`, `CreateLocalGatewayRouteTableVpcAssociation`, ... (+64)
- Traits: `idempotency-token` (43)
- Common required input members in this group: `AddressFamily`, `AmazonSideAsn`, `AttachmentType`, `AuthenticationOptions`, `BgpOptions`, `Bucket`, `CapacityReservationId`, `Cidr`, `ClientToken`, `ClientVpnEndpointId`, `CoipPoolId`, `ConnectionEvents`, `ConnectionLogOptions`, `ConnectionNotificationArn`, `CustomerGatewayId`, `Description`, `DestinationCidrBlock`, `DhcpConfigurations`, `Egress`, `EndpointType`, `ExportToS3Task`, `GroupName`, `ImageId`, `InputStorageLocation`, ... (+88)

### Delete

- Operations: `DeleteCapacityManagerDataExport`, `DeleteCarrierGateway`, `DeleteClientVpnEndpoint`, `DeleteClientVpnRoute`, `DeleteCoipCidr`, `DeleteCoipPool`, `DeleteCustomerGateway`, `DeleteDhcpOptions`, `DeleteEgressOnlyInternetGateway`, `DeleteFleets`, `DeleteFlowLogs`, `DeleteFpgaImage`, `DeleteImageUsageReport`, `DeleteInstanceConnectEndpoint`, `DeleteInstanceEventWindow`, `DeleteInternetGateway`, `DeleteIpam`, `DeleteIpamExternalResourceVerificationToken`, `DeleteIpamPolicy`, `DeleteIpamPool`, `DeleteIpamPrefixListResolver`, `DeleteIpamPrefixListResolverTarget`, `DeleteIpamResourceDiscovery`, `DeleteIpamScope`, `DeleteKeyPair`, `DeleteLaunchTemplate`, `DeleteLaunchTemplateVersions`, `DeleteLocalGatewayRoute`, `DeleteLocalGatewayRouteTable`, `DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation`, `DeleteLocalGatewayRouteTableVpcAssociation`, `DeleteLocalGatewayVirtualInterface`, `DeleteLocalGatewayVirtualInterfaceGroup`, `DeleteManagedPrefixList`, `DeleteNatGateway`, `DeleteNetworkAcl`, `DeleteNetworkAclEntry`, `DeleteNetworkInsightsAccessScope`, `DeleteNetworkInsightsAccessScopeAnalysis`, `DeleteNetworkInsightsAnalysis`, ... (+52)
- Traits: `idempotency-token` (6)
- Common required input members in this group: `CapacityManagerDataExportId`, `CarrierGatewayId`, `Cidr`, `ClientVpnEndpointId`, `CoipPoolId`, `ConnectionNotificationIds`, `CustomerGatewayId`, `DestinationCidrBlock`, `DhcpOptionsId`, `Egress`, `EgressOnlyInternetGatewayId`, `ExclusionId`, `FleetIds`, `FlowLogIds`, `FpgaImageId`, `GroupName`, `InstanceConnectEndpointId`, `InstanceEventWindowId`, `InternetGatewayId`, `IpamExternalResourceVerificationTokenId`, `IpamId`, `IpamPolicyId`, `IpamPoolId`, `IpamPrefixListResolverId`, ... (+59)

### Modify

- Operations: `ModifyAddressAttribute`, `ModifyAvailabilityZoneGroup`, `ModifyCapacityReservation`, `ModifyCapacityReservationFleet`, `ModifyClientVpnEndpoint`, `ModifyDefaultCreditSpecification`, `ModifyEbsDefaultKmsKeyId`, `ModifyFleet`, `ModifyFpgaImageAttribute`, `ModifyHosts`, `ModifyIdFormat`, `ModifyIdentityIdFormat`, `ModifyImageAttribute`, `ModifyInstanceAttribute`, `ModifyInstanceCapacityReservationAttributes`, `ModifyInstanceConnectEndpoint`, `ModifyInstanceCpuOptions`, `ModifyInstanceCreditSpecification`, `ModifyInstanceEventStartTime`, `ModifyInstanceEventWindow`, `ModifyInstanceMaintenanceOptions`, `ModifyInstanceMetadataDefaults`, `ModifyInstanceMetadataOptions`, `ModifyInstanceNetworkPerformanceOptions`, `ModifyInstancePlacement`, `ModifyIpam`, `ModifyIpamPolicyAllocationRules`, `ModifyIpamPool`, `ModifyIpamPrefixListResolver`, `ModifyIpamPrefixListResolverTarget`, `ModifyIpamResourceCidr`, `ModifyIpamResourceDiscovery`, `ModifyIpamScope`, `ModifyLaunchTemplate`, `ModifyLocalGatewayRoute`, `ModifyManagedPrefixList`, `ModifyNetworkInterfaceAttribute`, `ModifyPrivateDnsNameOptions`, `ModifyPublicIpDnsNameOptions`, `ModifyReservedInstances`, ... (+37)
- Traits: `idempotency-token` (9)
- Common required input members in this group: `AccessLogs`, `AllocationId`, `BandwidthWeighting`, `CapacityReservationFleetId`, `CapacityReservationId`, `CapacityReservationSpecification`, `ClientVpnEndpointId`, `ConnectionNotificationId`, `CpuCredits`, `CurrentIpamScopeId`, `ExclusionId`, `FleetId`, `FpgaImageId`, `GroupId`, `GroupName`, `HostIds`, `HostnameType`, `ImageId`, `InstanceConnectEndpointId`, `InstanceCreditSpecifications`, `InstanceEventId`, `InstanceEventWindowId`, `InstanceFamily`, `InstanceId`, ... (+53)

### Get

- Operations: `GetActiveVpnTunnelStatus`, `GetAllowedImagesSettings`, `GetAssociatedEnclaveCertificateIamRoles`, `GetAssociatedIpv6PoolCidrs`, `GetAwsNetworkPerformanceData`, `GetCapacityManagerAttributes`, `GetCapacityManagerMetricData`, `GetCapacityManagerMetricDimensions`, `GetCapacityReservationUsage`, `GetCoipPoolUsage`, `GetConsoleOutput`, `GetConsoleScreenshot`, `GetDeclarativePoliciesReportSummary`, `GetDefaultCreditSpecification`, `GetEbsDefaultKmsKeyId`, `GetEbsEncryptionByDefault`, `GetEnabledIpamPolicy`, `GetFlowLogsIntegrationTemplate`, `GetGroupsForCapacityReservation`, `GetHostReservationPurchasePreview`, `GetImageAncestry`, `GetImageBlockPublicAccessState`, `GetInstanceMetadataDefaults`, `GetInstanceTpmEkPub`, `GetInstanceTypesFromInstanceRequirements`, `GetInstanceUefiData`, `GetIpamAddressHistory`, `GetIpamDiscoveredAccounts`, `GetIpamDiscoveredPublicAddresses`, `GetIpamDiscoveredResourceCidrs`, `GetIpamPolicyAllocationRules`, `GetIpamPolicyOrganizationTargets`, `GetIpamPoolAllocations`, `GetIpamPoolCidrs`, `GetIpamPrefixListResolverRules`, `GetIpamPrefixListResolverVersionEntries`, `GetIpamPrefixListResolverVersions`, `GetIpamResourceCidrs`, `GetLaunchTemplateData`, `GetManagedPrefixListAssociations`, ... (+28)
- Traits: `paginated` (27)
- Common required input members in this group: `AddressRegion`, `ArchitectureTypes`, `CapacityReservationId`, `CertificateArn`, `Cidr`, `ConfigDeliveryS3DestinationArn`, `DiscoveryRegion`, `EndTime`, `FlowLogId`, `GroupBy`, `HostIdSet`, `ImageId`, `InstanceFamily`, `InstanceId`, `InstanceRequirements`, `IntegrateServices`, `IpamPolicyId`, `IpamPoolId`, `IpamPrefixListResolverId`, `IpamPrefixListResolverVersion`, `IpamResourceDiscoveryId`, `IpamScopeId`, `KeyFormat`, `KeyType`, ... (+26)

### Enable

- Operations: `EnableAddressTransfer`, `EnableAllowedImagesSettings`, `EnableAwsNetworkPerformanceMetricSubscription`, `EnableCapacityManager`, `EnableEbsEncryptionByDefault`, `EnableFastLaunch`, `EnableFastSnapshotRestores`, `EnableImage`, `EnableImageBlockPublicAccess`, `EnableImageDeprecation`, `EnableImageDeregistrationProtection`, `EnableInstanceSqlHaStandbyDetections`, `EnableIpamOrganizationAdminAccount`, `EnableIpamPolicy`, `EnableReachabilityAnalyzerOrganizationSharing`, `EnableRouteServerPropagation`, `EnableSerialConsoleAccess`, `EnableSnapshotBlockPublicAccess`, `EnableTransitGatewayRouteTablePropagation`, `EnableVgwRoutePropagation`, `EnableVolumeIO`, `EnableVpcClassicLink`, `EnableVpcClassicLinkDnsSupport`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AllocationId`, `AllowedImagesSettingsState`, `DelegatedAdminAccountId`, `DeprecateAt`, `GatewayId`, `ImageBlockPublicAccessState`, `ImageId`, `InstanceIds`, `IpamPolicyId`, `RouteServerId`, `RouteTableId`, `SourceSnapshotIds`, `State`, `TransferAccountId`, `TransitGatewayRouteTableId`, `VolumeId`, `VpcId`

### Disable

- Operations: `DisableAddressTransfer`, `DisableAllowedImagesSettings`, `DisableAwsNetworkPerformanceMetricSubscription`, `DisableCapacityManager`, `DisableEbsEncryptionByDefault`, `DisableFastLaunch`, `DisableFastSnapshotRestores`, `DisableImage`, `DisableImageBlockPublicAccess`, `DisableImageDeprecation`, `DisableImageDeregistrationProtection`, `DisableInstanceSqlHaStandbyDetections`, `DisableIpamOrganizationAdminAccount`, `DisableIpamPolicy`, `DisableRouteServerPropagation`, `DisableSerialConsoleAccess`, `DisableSnapshotBlockPublicAccess`, `DisableTransitGatewayRouteTablePropagation`, `DisableVgwRoutePropagation`, `DisableVpcClassicLink`, `DisableVpcClassicLinkDnsSupport`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AllocationId`, `DelegatedAdminAccountId`, `GatewayId`, `ImageId`, `InstanceIds`, `IpamPolicyId`, `RouteServerId`, `RouteTableId`, `SourceSnapshotIds`, `TransitGatewayRouteTableId`, `VpcId`

### Associate

- Operations: `AssociateAddress`, `AssociateCapacityReservationBillingOwner`, `AssociateClientVpnTargetNetwork`, `AssociateDhcpOptions`, `AssociateEnclaveCertificateIamRole`, `AssociateIamInstanceProfile`, `AssociateInstanceEventWindow`, `AssociateIpamByoasn`, `AssociateIpamResourceDiscovery`, `AssociateNatGatewayAddress`, `AssociateRouteServer`, `AssociateRouteTable`, `AssociateSecurityGroupVpc`, `AssociateSubnetCidrBlock`, `AssociateTransitGatewayMulticastDomain`, `AssociateTransitGatewayPolicyTable`, `AssociateTransitGatewayRouteTable`, `AssociateTrunkInterface`, `AssociateVpcCidrBlock`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `AllocationIds`, `Asn`, `AssociationTarget`, `BranchInterfaceId`, `CapacityReservationId`, `CertificateArn`, `Cidr`, `ClientVpnEndpointId`, `DhcpOptionsId`, `GroupId`, `IamInstanceProfile`, `InstanceEventWindowId`, `InstanceId`, `IpamId`, `IpamResourceDiscoveryId`, `NatGatewayId`, `RoleArn`, `RouteServerId`, `RouteTableId`, `SubnetId`, `SubnetIds`, `TransitGatewayAttachmentId`, `TransitGatewayMulticastDomainId`, `TransitGatewayPolicyTableId`, ... (+4)

### Disassociate

- Operations: `DisassociateAddress`, `DisassociateCapacityReservationBillingOwner`, `DisassociateClientVpnTargetNetwork`, `DisassociateEnclaveCertificateIamRole`, `DisassociateIamInstanceProfile`, `DisassociateInstanceEventWindow`, `DisassociateIpamByoasn`, `DisassociateIpamResourceDiscovery`, `DisassociateNatGatewayAddress`, `DisassociateRouteServer`, `DisassociateRouteTable`, `DisassociateSecurityGroupVpc`, `DisassociateSubnetCidrBlock`, `DisassociateTransitGatewayMulticastDomain`, `DisassociateTransitGatewayPolicyTable`, `DisassociateTransitGatewayRouteTable`, `DisassociateTrunkInterface`, `DisassociateVpcCidrBlock`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Asn`, `AssociationId`, `AssociationIds`, `AssociationTarget`, `CapacityReservationId`, `CertificateArn`, `Cidr`, `ClientVpnEndpointId`, `GroupId`, `InstanceEventWindowId`, `IpamResourceDiscoveryAssociationId`, `NatGatewayId`, `RoleArn`, `RouteServerId`, `SubnetIds`, `TransitGatewayAttachmentId`, `TransitGatewayMulticastDomainId`, `TransitGatewayPolicyTableId`, `TransitGatewayRouteTableId`, `UnusedReservationBillingOwnerId`, `VpcId`

### Cancel

- Operations: `CancelBundleTask`, `CancelCapacityReservation`, `CancelCapacityReservationFleets`, `CancelConversionTask`, `CancelDeclarativePoliciesReport`, `CancelExportTask`, `CancelImageLaunchPermission`, `CancelImportTask`, `CancelReservedInstancesListing`, `CancelSpotFleetRequests`, `CancelSpotInstanceRequests`
- Common required input members in this group: `BundleId`, `CapacityReservationFleetIds`, `CapacityReservationId`, `ConversionTaskId`, `ExportTaskId`, `ImageId`, `ReportId`, `ReservedInstancesListingId`, `SpotFleetRequestIds`, `SpotInstanceRequestIds`, `TerminateInstances`

### Accept

- Operations: `AcceptAddressTransfer`, `AcceptCapacityReservationBillingOwnership`, `AcceptReservedInstancesExchangeQuote`, `AcceptTransitGatewayMulticastDomainAssociations`, `AcceptTransitGatewayPeeringAttachment`, `AcceptTransitGatewayVpcAttachment`, `AcceptVpcEndpointConnections`, `AcceptVpcPeeringConnection`
- Common required input members in this group: `Address`, `CapacityReservationId`, `ReservedInstanceIds`, `ServiceId`, `TransitGatewayAttachmentId`, `VpcEndpointIds`, `VpcPeeringConnectionId`

### Replace

- Operations: `ReplaceIamInstanceProfileAssociation`, `ReplaceImageCriteriaInAllowedImagesSettings`, `ReplaceNetworkAclAssociation`, `ReplaceNetworkAclEntry`, `ReplaceRoute`, `ReplaceRouteTableAssociation`, `ReplaceTransitGatewayRoute`, `ReplaceVpnTunnel`
- Common required input members in this group: `AssociationId`, `DestinationCidrBlock`, `Egress`, `IamInstanceProfile`, `NetworkAclId`, `Protocol`, `RouteTableId`, `RuleAction`, `RuleNumber`, `TransitGatewayRouteTableId`, `VpnConnectionId`, `VpnTunnelOutsideIpAddress`

### Reset

- Operations: `ResetAddressAttribute`, `ResetEbsDefaultKmsKeyId`, `ResetFpgaImageAttribute`, `ResetImageAttribute`, `ResetInstanceAttribute`, `ResetNetworkInterfaceAttribute`, `ResetSnapshotAttribute`
- Common required input members in this group: `AllocationId`, `Attribute`, `FpgaImageId`, `ImageId`, `InstanceId`, `NetworkInterfaceId`, `SnapshotId`

### Attach

- Operations: `AttachClassicLinkVpc`, `AttachInternetGateway`, `AttachNetworkInterface`, `AttachVerifiedAccessTrustProvider`, `AttachVolume`, `AttachVpnGateway`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Device`, `DeviceIndex`, `Groups`, `InstanceId`, `InternetGatewayId`, `NetworkInterfaceId`, `VerifiedAccessInstanceId`, `VerifiedAccessTrustProviderId`, `VolumeId`, `VpcId`, `VpnGatewayId`

### Detach

- Operations: `DetachClassicLinkVpc`, `DetachInternetGateway`, `DetachNetworkInterface`, `DetachVerifiedAccessTrustProvider`, `DetachVolume`, `DetachVpnGateway`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AttachmentId`, `InstanceId`, `InternetGatewayId`, `VerifiedAccessInstanceId`, `VerifiedAccessTrustProviderId`, `VolumeId`, `VpcId`, `VpnGatewayId`

### Import

- Operations: `ImportClientVpnClientCertificateRevocationList`, `ImportImage`, `ImportInstance`, `ImportKeyPair`, `ImportSnapshot`, `ImportVolume`
- Common required input members in this group: `CertificateRevocationList`, `ClientVpnEndpointId`, `Image`, `KeyName`, `Platform`, `PublicKeyMaterial`, `Volume`

### Reject

- Operations: `RejectCapacityReservationBillingOwnership`, `RejectTransitGatewayMulticastDomainAssociations`, `RejectTransitGatewayPeeringAttachment`, `RejectTransitGatewayVpcAttachment`, `RejectVpcEndpointConnections`, `RejectVpcPeeringConnection`
- Common required input members in this group: `CapacityReservationId`, `ServiceId`, `TransitGatewayAttachmentId`, `VpcEndpointIds`, `VpcPeeringConnectionId`

### Restore

- Operations: `RestoreAddressToClassic`, `RestoreImageFromRecycleBin`, `RestoreManagedPrefixListVersion`, `RestoreSnapshotFromRecycleBin`, `RestoreSnapshotTier`, `RestoreVolumeFromRecycleBin`
- Common required input members in this group: `CurrentVersion`, `ImageId`, `PrefixListId`, `PreviousVersion`, `PublicIp`, `SnapshotId`, `VolumeId`

### Export

- Operations: `ExportClientVpnClientCertificateRevocationList`, `ExportClientVpnClientConfiguration`, `ExportImage`, `ExportTransitGatewayRoutes`, `ExportVerifiedAccessInstanceClientConfiguration`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientVpnEndpointId`, `DiskImageFormat`, `ImageId`, `S3Bucket`, `S3ExportLocation`, `TransitGatewayRouteTableId`, `VerifiedAccessInstanceId`

### Purchase

- Operations: `PurchaseCapacityBlock`, `PurchaseCapacityBlockExtension`, `PurchaseHostReservation`, `PurchaseReservedInstancesOffering`, `PurchaseScheduledInstances`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `CapacityBlockExtensionOfferingId`, `CapacityBlockOfferingId`, `CapacityReservationId`, `HostIdSet`, `InstanceCount`, `InstancePlatform`, `OfferingId`, `PurchaseRequests`, `ReservedInstancesOfferingId`

### Start

- Operations: `StartDeclarativePoliciesReport`, `StartInstances`, `StartNetworkInsightsAccessScopeAnalysis`, `StartNetworkInsightsAnalysis`, `StartVpcEndpointServicePrivateDnsVerification`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `ClientToken`, `InstanceIds`, `NetworkInsightsAccessScopeId`, `NetworkInsightsPathId`, `S3Bucket`, `ServiceId`, `TargetId`

### Copy

- Operations: `CopyFpgaImage`, `CopyImage`, `CopySnapshot`, `CopyVolumes`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `Name`, `SourceFpgaImageId`, `SourceImageId`, `SourceRegion`, `SourceSnapshotId`, `SourceVolumeId`

### Deprovision

- Operations: `DeprovisionByoipCidr`, `DeprovisionIpamByoasn`, `DeprovisionIpamPoolCidr`, `DeprovisionPublicIpv4PoolCidr`
- Common required input members in this group: `Asn`, `Cidr`, `IpamId`, `IpamPoolId`, `PoolId`

### Deregister

- Operations: `DeregisterImage`, `DeregisterInstanceEventNotificationAttributes`, `DeregisterTransitGatewayMulticastGroupMembers`, `DeregisterTransitGatewayMulticastGroupSources`
- Common required input members in this group: `ImageId`, `InstanceTagAttribute`

### Provision

- Operations: `ProvisionByoipCidr`, `ProvisionIpamByoasn`, `ProvisionIpamPoolCidr`, `ProvisionPublicIpv4PoolCidr`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Asn`, `AsnAuthorizationContext`, `Cidr`, `IpamId`, `IpamPoolId`, `NetmaskLength`, `PoolId`

### Register

- Operations: `RegisterImage`, `RegisterInstanceEventNotificationAttributes`, `RegisterTransitGatewayMulticastGroupMembers`, `RegisterTransitGatewayMulticastGroupSources`
- Common required input members in this group: `InstanceTagAttribute`, `Name`, `NetworkInterfaceIds`, `TransitGatewayMulticastDomainId`

### Update

- Operations: `UpdateCapacityManagerOrganizationsAccess`, `UpdateInterruptibleCapacityReservationAllocation`, `UpdateSecurityGroupRuleDescriptionsEgress`, `UpdateSecurityGroupRuleDescriptionsIngress`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `CapacityReservationId`, `OrganizationsAccess`, `TargetInstanceCount`

### Allocate

- Operations: `AllocateAddress`, `AllocateHosts`, `AllocateIpamPoolCidr`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `IpamPoolId`

### Assign

- Operations: `AssignIpv6Addresses`, `AssignPrivateIpAddresses`, `AssignPrivateNatGatewayAddress`
- Common required input members in this group: `NatGatewayId`, `NetworkInterfaceId`

### Authorize

- Operations: `AuthorizeClientVpnIngress`, `AuthorizeSecurityGroupEgress`, `AuthorizeSecurityGroupIngress`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientVpnEndpointId`, `GroupId`, `TargetNetworkCidr`

### List

- Operations: `ListImagesInRecycleBin`, `ListSnapshotsInRecycleBin`, `ListVolumesInRecycleBin`
- Traits: `paginated` (2)

### Move

- Operations: `MoveAddressToVpc`, `MoveByoipCidrToIpam`, `MoveCapacityReservationInstances`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Cidr`, `DestinationCapacityReservationId`, `InstanceCount`, `IpamPoolId`, `IpamPoolOwner`, `PublicIp`, `SourceCapacityReservationId`

### Release

- Operations: `ReleaseAddress`, `ReleaseHosts`, `ReleaseIpamPoolAllocation`
- Common required input members in this group: `Cidr`, `HostIds`, `IpamPoolAllocationId`, `IpamPoolId`

### Revoke

- Operations: `RevokeClientVpnIngress`, `RevokeSecurityGroupEgress`, `RevokeSecurityGroupIngress`
- Common required input members in this group: `ClientVpnEndpointId`, `GroupId`, `TargetNetworkCidr`

### Search

- Operations: `SearchLocalGatewayRoutes`, `SearchTransitGatewayMulticastGroups`, `SearchTransitGatewayRoutes`
- Traits: `paginated` (3)
- Common required input members in this group: `Filters`, `LocalGatewayRouteTableId`, `TransitGatewayMulticastDomainId`, `TransitGatewayRouteTableId`

### Unassign

- Operations: `UnassignIpv6Addresses`, `UnassignPrivateIpAddresses`, `UnassignPrivateNatGatewayAddress`
- Common required input members in this group: `NatGatewayId`, `NetworkInterfaceId`, `PrivateIpAddresses`

### Request

- Operations: `RequestSpotFleet`, `RequestSpotInstances`
- Common required input members in this group: `SpotFleetRequestConfig`

### Run

- Operations: `RunInstances`, `RunScheduledInstances`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `LaunchSpecification`, `MaxCount`, `MinCount`, `ScheduledInstanceId`

### Terminate

- Operations: `TerminateClientVpnConnections`, `TerminateInstances`
- Common required input members in this group: `ClientVpnEndpointId`, `InstanceIds`

### Advertise

- Operations: `AdvertiseByoipCidr`
- Common required input members in this group: `Cidr`

### Apply

- Operations: `ApplySecurityGroupsToClientVpnTargetNetwork`
- Common required input members in this group: `ClientVpnEndpointId`, `SecurityGroupIds`, `VpcId`

### Bundle

- Operations: `BundleInstance`
- Common required input members in this group: `InstanceId`, `Storage`

### Confirm

- Operations: `ConfirmProductInstance`
- Common required input members in this group: `InstanceId`, `ProductCode`

### Lock

- Operations: `LockSnapshot`
- Common required input members in this group: `LockMode`, `SnapshotId`

### Monitor

- Operations: `MonitorInstances`
- Common required input members in this group: `InstanceIds`

### Reboot

- Operations: `RebootInstances`
- Common required input members in this group: `InstanceIds`

### Report

- Operations: `ReportInstanceStatus`
- Common required input members in this group: `Instances`, `ReasonCodes`, `Status`

### Send

- Operations: `SendDiagnosticInterrupt`
- Common required input members in this group: `InstanceId`

### Stop

- Operations: `StopInstances`
- Common required input members in this group: `InstanceIds`

### Unlock

- Operations: `UnlockSnapshot`
- Common required input members in this group: `SnapshotId`

### Unmonitor

- Operations: `UnmonitorInstances`
- Common required input members in this group: `InstanceIds`

### Withdraw

- Operations: `WithdrawByoipCidr`
- Common required input members in this group: `Cidr`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptAddressTransfer` | - | - | `Address` | - | `AcceptAddressTransferResult` | - | Accepts an Elastic IP address transfer. For more information, see Accept a transferred Elastic IP address in the Amazon VPC User Guide . |
| `AcceptCapacityReservationBillingOwnership` | - | - | `CapacityReservationId` | - | `AcceptCapacityReservationBillingOwnershipResult` | - | Accepts a request to assign billing of the available capacity of a shared Capacity Reservation to your account. For more information, see Billing assignment for shared Amazon EC2 Capacity Reservations. |
| `AcceptReservedInstancesExchangeQuote` | - | - | `ReservedInstanceIds` | - | `AcceptReservedInstancesExchangeQuoteResult` | - | Accepts the Convertible Reserved Instance exchange quote described in the GetReservedInstancesExchangeQuote call. |
| `AcceptTransitGatewayMulticastDomainAssociations` | - | - | - | - | `AcceptTransitGatewayMulticastDomainAssociationsResult` | - | Accepts a request to associate subnets with a transit gateway multicast domain. |
| `AcceptTransitGatewayPeeringAttachment` | - | - | `TransitGatewayAttachmentId` | - | `AcceptTransitGatewayPeeringAttachmentResult` | - | Accepts a transit gateway peering attachment request. The peering attachment must be in the `pendingAcceptance` state. |
| `AcceptTransitGatewayVpcAttachment` | - | - | `TransitGatewayAttachmentId` | - | `AcceptTransitGatewayVpcAttachmentResult` | - | Accepts a request to attach a VPC to a transit gateway. The VPC attachment must be in the `pendingAcceptance` state. |
| `AcceptVpcEndpointConnections` | - | - | `ServiceId`, `VpcEndpointIds` | - | `AcceptVpcEndpointConnectionsResult` | - | Accepts connection requests to your VPC endpoint service. |
| `AcceptVpcPeeringConnection` | - | - | `VpcPeeringConnectionId` | - | `AcceptVpcPeeringConnectionResult` | - | Accept a VPC peering connection request. To accept a request, the VPC peering connection must be in the `pending-acceptance` state, and you must be the owner of the peer VPC. |
| `AdvertiseByoipCidr` | - | - | `Cidr` | - | `AdvertiseByoipCidrResult` | - | Advertises an IPv4 or IPv6 address range that is provisioned for use with your Amazon Web Services resources through bring your own IP addresses (BYOIP). You can perform this operation at most once every 10 seconds, even if you specify different address... |
| `AllocateAddress` | - | - | - | - | `AllocateAddressResult` | - | Allocates an Elastic IP address to your Amazon Web Services account. After you allocate the Elastic IP address you can associate it with an instance or network interface. |
| `AllocateHosts` | - | - | - | - | `AllocateHostsResult` | - | Allocates a Dedicated Host to your account. At a minimum, specify the supported instance type or instance family, the Availability Zone in which to allocate the host, and the number of hosts to allocate. |
| `AllocateIpamPoolCidr` | - | `idempotency-token` | `IpamPoolId` | `ClientToken` | `AllocateIpamPoolCidrResult` | - | Allocate a CIDR from an IPAM pool. The Region you use should be the IPAM pool locale. |
| `ApplySecurityGroupsToClientVpnTargetNetwork` | - | - | `ClientVpnEndpointId`, `SecurityGroupIds`, `VpcId` | - | `ApplySecurityGroupsToClientVpnTargetNetworkResult` | - | Applies a security group to the association between the target network and the Client VPN endpoint. This action replaces the existing security groups with the specified security groups. |
| `AssignIpv6Addresses` | - | - | `NetworkInterfaceId` | - | `AssignIpv6AddressesResult` | - | Assigns the specified IPv6 addresses to the specified network interface. You can specify specific IPv6 addresses, or you can specify the number of IPv6 addresses to be automatically assigned from the subnet's IPv6 CIDR block range. |
| `AssignPrivateIpAddresses` | - | - | `NetworkInterfaceId` | - | `AssignPrivateIpAddressesResult` | - | Assigns the specified secondary private IP addresses to the specified network interface. You can specify specific secondary IP addresses, or you can specify the number of secondary IP addresses to be automatically assigned from the subnet's CIDR block range. |
| `AssignPrivateNatGatewayAddress` | - | - | `NatGatewayId` | - | `AssignPrivateNatGatewayAddressResult` | - | Assigns private IPv4 addresses to a private NAT gateway. For more information, see Work with NAT gateways in the Amazon VPC User Guide . |
| `AssociateAddress` | - | - | - | - | `AssociateAddressResult` | - | Associates an Elastic IP address, or carrier IP address (for instances that are in subnets in Wavelength Zones) with an instance or a network interface. Before you can use an Elastic IP address, you must allocate it to your account. |
| `AssociateCapacityReservationBillingOwner` | - | - | `CapacityReservationId`, `UnusedReservationBillingOwnerId` | - | `AssociateCapacityReservationBillingOwnerResult` | - | Initiates a request to assign billing of the unused capacity of a shared Capacity Reservation to a consumer account that is consolidated under the same Amazon Web Services organizations payer account. For more information, see Billing assignment for shared... |
| `AssociateClientVpnTargetNetwork` | - | `idempotency-token` | `ClientVpnEndpointId`, `SubnetId` | `ClientToken` | `AssociateClientVpnTargetNetworkResult` | - | Associates a target network with a Client VPN endpoint. A target network is a subnet in a VPC. |
| `AssociateDhcpOptions` | - | - | `DhcpOptionsId`, `VpcId` | - | `Unit` | - | Associates a set of DHCP options (that you've previously created) with the specified VPC, or associates no DHCP options with the VPC. After you associate the options with the VPC, any existing instances and all new instances that you launch in that VPC use... |
| `AssociateEnclaveCertificateIamRole` | - | - | `CertificateArn`, `RoleArn` | - | `AssociateEnclaveCertificateIamRoleResult` | - | Associates an Identity and Access Management (IAM) role with an Certificate Manager (ACM) certificate. This enables the certificate to be used by the ACM for Nitro Enclaves application inside an enclave. |
| `AssociateIamInstanceProfile` | - | - | `IamInstanceProfile`, `InstanceId` | - | `AssociateIamInstanceProfileResult` | - | Associates an IAM instance profile with a running or stopped instance. You cannot associate more than one IAM instance profile with an instance. |
| `AssociateInstanceEventWindow` | - | - | `AssociationTarget`, `InstanceEventWindowId` | - | `AssociateInstanceEventWindowResult` | - | Associates one or more targets with an event window. Only one type of target (instance IDs, Dedicated Host IDs, or tags) can be specified with an event window. |
| `AssociateIpamByoasn` | - | - | `Asn`, `Cidr` | - | `AssociateIpamByoasnResult` | - | Associates your Autonomous System Number (ASN) with a BYOIP CIDR that you own in the same Amazon Web Services Region. For more information, see Tutorial: Bring your ASN to IPAM in the Amazon VPC IPAM guide . |
| `AssociateIpamResourceDiscovery` | - | `idempotency-token` | `IpamId`, `IpamResourceDiscoveryId` | `ClientToken` | `AssociateIpamResourceDiscoveryResult` | - | Associates an IPAM resource discovery with an Amazon VPC IPAM. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account. |
| `AssociateNatGatewayAddress` | - | - | `AllocationIds`, `NatGatewayId` | - | `AssociateNatGatewayAddressResult` | - | Associates Elastic IP addresses (EIPs) and private IPv4 addresses with a public NAT gateway. For more information, see Work with NAT gateways in the Amazon VPC User Guide . |
| `AssociateRouteServer` | - | - | `RouteServerId`, `VpcId` | - | `AssociateRouteServerResult` | - | Associates a route server with a VPC to enable dynamic route updates. A route server association is the connection established between a route server and a VPC. |
| `AssociateRouteTable` | - | - | `RouteTableId` | - | `AssociateRouteTableResult` | - | Associates a subnet in your VPC or an internet gateway or virtual private gateway attached to your VPC with a route table in your VPC. This association causes traffic from the subnet or gateway to be routed according to the routes in the route table. |
| `AssociateSecurityGroupVpc` | - | - | `GroupId`, `VpcId` | - | `AssociateSecurityGroupVpcResult` | - | Associates a security group with another VPC in the same Region. This enables you to use the same security group with network interfaces and instances in the specified VPC. |
| `AssociateSubnetCidrBlock` | - | - | `SubnetId` | - | `AssociateSubnetCidrBlockResult` | - | Associates a CIDR block with your subnet. You can only associate a single IPv6 CIDR block with your subnet. |
| `AssociateTransitGatewayMulticastDomain` | - | - | `SubnetIds`, `TransitGatewayAttachmentId`, `TransitGatewayMulticastDomainId` | - | `AssociateTransitGatewayMulticastDomainResult` | - | Associates the specified subnets and transit gateway attachments with the specified transit gateway multicast domain. The transit gateway attachment must be in the available state before you can add a resource. |
| `AssociateTransitGatewayPolicyTable` | - | - | `TransitGatewayAttachmentId`, `TransitGatewayPolicyTableId` | - | `AssociateTransitGatewayPolicyTableResult` | - | Associates the specified transit gateway attachment with a transit gateway policy table. |
| `AssociateTransitGatewayRouteTable` | - | - | `TransitGatewayAttachmentId`, `TransitGatewayRouteTableId` | - | `AssociateTransitGatewayRouteTableResult` | - | Associates the specified attachment with the specified transit gateway route table. You can associate only one route table with an attachment. |
| `AssociateTrunkInterface` | - | `idempotency-token` | `BranchInterfaceId`, `TrunkInterfaceId` | `ClientToken` | `AssociateTrunkInterfaceResult` | - | Associates a branch network interface with a trunk network interface. Before you create the association, use CreateNetworkInterface command and set the interface type to `trunk`. |
| `AssociateVpcCidrBlock` | - | - | `VpcId` | - | `AssociateVpcCidrBlockResult` | - | Associates a CIDR block with your VPC. You can associate a secondary IPv4 CIDR block, an Amazon-provided IPv6 CIDR block, or an IPv6 CIDR block from an IPv6 address pool that you provisioned through bring your own IP addresses (BYOIP). |
| `AttachClassicLinkVpc` | - | - | `Groups`, `InstanceId`, `VpcId` | - | `AttachClassicLinkVpcResult` | - | This action is deprecated. Links an EC2-Classic instance to a ClassicLink-enabled VPC through one or more of the VPC security groups. |
| `AttachInternetGateway` | - | - | `InternetGatewayId`, `VpcId` | - | `Unit` | - | Attaches an internet gateway or a virtual private gateway to a VPC, enabling connectivity between the internet and the VPC. For more information, see Internet gateways in the Amazon VPC User Guide . |
| `AttachNetworkInterface` | - | - | `DeviceIndex`, `InstanceId`, `NetworkInterfaceId` | - | `AttachNetworkInterfaceResult` | - | Attaches a network interface to an instance. |
| `AttachVerifiedAccessTrustProvider` | - | `idempotency-token` | `VerifiedAccessInstanceId`, `VerifiedAccessTrustProviderId` | `ClientToken` | `AttachVerifiedAccessTrustProviderResult` | - | Attaches the specified Amazon Web Services Verified Access trust provider to the specified Amazon Web Services Verified Access instance. |
| `AttachVolume` | - | - | `Device`, `InstanceId`, `VolumeId` | - | `VolumeAttachment` | - | Attaches an Amazon EBS volume to a `running` or `stopped` instance, and exposes it to the instance with the specified device name. The maximum number of Amazon EBS volumes that you can attach to an instance depends on the instance type. |
| `AttachVpnGateway` | - | - | `VpcId`, `VpnGatewayId` | - | `AttachVpnGatewayResult` | - | Attaches an available virtual private gateway to a VPC. You can attach one virtual private gateway to one VPC at a time. |
| `AuthorizeClientVpnIngress` | - | `idempotency-token` | `ClientVpnEndpointId`, `TargetNetworkCidr` | `ClientToken` | `AuthorizeClientVpnIngressResult` | - | Adds an ingress authorization rule to a Client VPN endpoint. Ingress authorization rules act as firewall rules that grant access to networks. |
| `AuthorizeSecurityGroupEgress` | - | - | `GroupId` | - | `AuthorizeSecurityGroupEgressResult` | - | Adds the specified outbound (egress) rules to a security group. An outbound rule permits instances to send traffic to the specified IPv4 or IPv6 address ranges, the IP address ranges specified by a prefix list, or the instances that are associated with a... |
| `AuthorizeSecurityGroupIngress` | - | - | - | - | `AuthorizeSecurityGroupIngressResult` | - | Adds the specified inbound (ingress) rules to a security group. An inbound rule permits instances to receive traffic from the specified IPv4 or IPv6 address range, the IP address ranges that are specified by a prefix list, or the instances that are associated... |
| `BundleInstance` | - | - | `InstanceId`, `Storage` | - | `BundleInstanceResult` | - | Bundles an Amazon instance store-backed Windows instance. During bundling, only the root device volume (C:\) is bundled. |
| `CancelBundleTask` | - | - | `BundleId` | - | `CancelBundleTaskResult` | - | Cancels a bundling operation for an instance store-backed Windows instance. |
| `CancelCapacityReservation` | - | - | `CapacityReservationId` | - | `CancelCapacityReservationResult` | - | Cancels the specified Capacity Reservation, releases the reserved capacity, and changes the Capacity Reservation's state to `cancelled`. You can cancel a Capacity Reservation that is in the following states: `assessing` `active` and there is no commitment... |
| `CancelCapacityReservationFleets` | - | - | `CapacityReservationFleetIds` | - | `CancelCapacityReservationFleetsResult` | - | Cancels one or more Capacity Reservation Fleets. When you cancel a Capacity Reservation Fleet, the following happens: The Capacity Reservation Fleet's status changes to `cancelled`. |
| `CancelConversionTask` | - | - | `ConversionTaskId` | - | `Unit` | - | Cancels an active conversion task. The task can be the import of an instance or volume. |
| `CancelDeclarativePoliciesReport` | - | - | `ReportId` | - | `CancelDeclarativePoliciesReportResult` | - | Cancels the generation of an account status report. You can only cancel a report while it has the `running` status. |
| `CancelExportTask` | - | - | `ExportTaskId` | - | `Unit` | - | Cancels an active export task. The request removes all artifacts of the export, including any partially-created Amazon S3 objects. |
| `CancelImageLaunchPermission` | - | - | `ImageId` | - | `CancelImageLaunchPermissionResult` | - | Removes your Amazon Web Services account from the launch permissions for the specified AMI. For more information, see Cancel having an AMI shared with your Amazon Web Services account in the Amazon EC2 User Guide . |
| `CancelImportTask` | - | - | - | - | `CancelImportTaskResult` | - | Cancels an in-process import virtual machine or import snapshot task. |
| `CancelReservedInstancesListing` | - | - | `ReservedInstancesListingId` | - | `CancelReservedInstancesListingResult` | - | Cancels the specified Reserved Instance listing in the Reserved Instance Marketplace. For more information, see Sell in the Reserved Instance Marketplace in the Amazon EC2 User Guide . |
| `CancelSpotFleetRequests` | - | - | `SpotFleetRequestIds`, `TerminateInstances` | - | `CancelSpotFleetRequestsResponse` | - | Cancels the specified Spot Fleet requests. After you cancel a Spot Fleet request, the Spot Fleet launches no new instances. |
| `CancelSpotInstanceRequests` | - | - | `SpotInstanceRequestIds` | - | `CancelSpotInstanceRequestsResult` | - | Cancels one or more Spot Instance requests. Canceling a Spot Instance request does not terminate running Spot Instances associated with the request. |
| `ConfirmProductInstance` | - | - | `InstanceId`, `ProductCode` | - | `ConfirmProductInstanceResult` | - | Determines whether a product code is associated with an instance. This action can only be used by the owner of the product code. |
| `CopyFpgaImage` | - | - | `SourceFpgaImageId`, `SourceRegion` | - | `CopyFpgaImageResult` | - | Copies the specified Amazon FPGA Image (AFI) to the current Region. |
| `CopyImage` | - | `idempotency-token` | `Name`, `SourceImageId`, `SourceRegion` | `ClientToken` | `CopyImageResult` | - | Initiates an AMI copy operation. You must specify the source AMI ID and both the source and destination locations. |
| `CopySnapshot` | - | - | `SourceRegion`, `SourceSnapshotId` | - | `CopySnapshotResult` | - | Creates an exact copy of an Amazon EBS snapshot. The location of the source snapshot determines whether you can copy it or not, and the allowed destinations for the snapshot copy. |
| `CopyVolumes` | - | `idempotency-token` | `SourceVolumeId` | `ClientToken` | `CopyVolumesResult` | - | Creates a crash-consistent, point-in-time copy of an existing Amazon EBS volume within the same Availability Zone. The volume copy can be attached to an Amazon EC2 instance once it reaches the `available` state. |
| `CreateCapacityManagerDataExport` | - | `idempotency-token` | `OutputFormat`, `S3BucketName`, `Schedule` | `ClientToken` | `CreateCapacityManagerDataExportResult` | - | Creates a new data export configuration for EC2 Capacity Manager. This allows you to automatically export capacity usage data to an S3 bucket on a scheduled basis. |
| `CreateCapacityReservation` | - | - | `InstanceCount`, `InstancePlatform`, `InstanceType` | - | `CreateCapacityReservationResult` | - | Creates a new Capacity Reservation with the specified attributes. Capacity Reservations enable you to reserve capacity for your Amazon EC2 instances in a specific Availability Zone for any duration. |
| `CreateCapacityReservationBySplitting` | - | `idempotency-token` | `InstanceCount`, `SourceCapacityReservationId` | `ClientToken` | `CreateCapacityReservationBySplittingResult` | - | Create a new Capacity Reservation by splitting the capacity of the source Capacity Reservation. The new Capacity Reservation will have the same attributes as the source Capacity Reservation except for tags. |
| `CreateCapacityReservationFleet` | - | `idempotency-token` | `InstanceTypeSpecifications`, `TotalTargetCapacity` | `ClientToken` | `CreateCapacityReservationFleetResult` | - | Creates a Capacity Reservation Fleet. For more information, see Create a Capacity Reservation Fleet in the Amazon EC2 User Guide . |
| `CreateCarrierGateway` | - | `idempotency-token` | `VpcId` | `ClientToken` | `CreateCarrierGatewayResult` | - | Creates a carrier gateway. For more information about carrier gateways, see Carrier gateways in the Amazon Web Services Wavelength Developer Guide . |
| `CreateClientVpnEndpoint` | - | `idempotency-token` | `AuthenticationOptions`, `ConnectionLogOptions`, `ServerCertificateArn` | `ClientToken` | `CreateClientVpnEndpointResult` | - | Creates a Client VPN endpoint. A Client VPN endpoint is the resource you create and configure to enable and manage client VPN sessions. |
| `CreateClientVpnRoute` | - | `idempotency-token` | `ClientVpnEndpointId`, `DestinationCidrBlock`, `TargetVpcSubnetId` | `ClientToken` | `CreateClientVpnRouteResult` | - | Adds a route to a network to a Client VPN endpoint. Each Client VPN endpoint has a route table that describes the available destination network routes. |
| `CreateCoipCidr` | - | - | `Cidr`, `CoipPoolId` | - | `CreateCoipCidrResult` | - | Creates a range of customer-owned IP addresses. |
| `CreateCoipPool` | - | - | `LocalGatewayRouteTableId` | - | `CreateCoipPoolResult` | - | Creates a pool of customer-owned IP (CoIP) addresses. |
| `CreateCustomerGateway` | - | - | `Type` | - | `CreateCustomerGatewayResult` | - | Provides information to Amazon Web Services about your customer gateway device. The customer gateway device is the appliance at your end of the VPN connection. |
| `CreateDefaultSubnet` | - | - | - | - | `CreateDefaultSubnetResult` | - | Creates a default subnet with a size `/20` IPv4 CIDR block in the specified Availability Zone in your default VPC. You can have only one default subnet per Availability Zone. |
| `CreateDefaultVpc` | - | - | - | - | `CreateDefaultVpcResult` | - | Creates a default VPC with a size `/16` IPv4 CIDR block and a default subnet in each Availability Zone. For more information about the components of a default VPC, see Default VPCs in the Amazon VPC User Guide . |
| `CreateDelegateMacVolumeOwnershipTask` | - | `idempotency-token` | `InstanceId`, `MacCredentials` | `ClientToken` | `CreateDelegateMacVolumeOwnershipTaskResult` | - | Delegates ownership of the Amazon EBS root volume for an Apple silicon Mac instance to an administrative user. |
| `CreateDhcpOptions` | - | - | `DhcpConfigurations` | - | `CreateDhcpOptionsResult` | - | Creates a custom set of DHCP options. After you create a DHCP option set, you associate it with a VPC. |
| `CreateEgressOnlyInternetGateway` | - | - | `VpcId` | - | `CreateEgressOnlyInternetGatewayResult` | - | [IPv6 only] Creates an egress-only internet gateway for your VPC. An egress-only internet gateway is used to enable outbound communication over IPv6 from instances in your VPC to the internet, and prevents hosts outside of your VPC from initiating an IPv6... |
| `CreateFleet` | - | `idempotency-token` | `LaunchTemplateConfigs`, `TargetCapacitySpecification` | `ClientToken` | `CreateFleetResult` | - | Creates an EC2 Fleet that contains the configuration information for On-Demand Instances and Spot Instances. Instances are launched immediately if there is available capacity. |
| `CreateFlowLogs` | - | - | `ResourceIds`, `ResourceType` | - | `CreateFlowLogsResult` | - | Creates one or more flow logs to capture information about IP traffic for a specific network interface, subnet, or VPC. Flow log data for a monitored network interface is recorded as flow log records, which are log events consisting of fields that describe... |
| `CreateFpgaImage` | - | - | `InputStorageLocation` | - | `CreateFpgaImageResult` | - | Creates an Amazon FPGA Image (AFI) from the specified design checkpoint (DCP). The create operation is asynchronous. |
| `CreateImage` | - | - | `InstanceId`, `Name` | - | `CreateImageResult` | - | Creates an Amazon EBS-backed AMI from an Amazon EBS-backed instance that is either running or stopped. If you customized your instance with instance store volumes or Amazon EBS volumes in addition to the root device volume, the new AMI contains block device... |
| `CreateImageUsageReport` | - | `idempotency-token` | `ImageId`, `ResourceTypes` | `ClientToken` | `CreateImageUsageReportResult` | - | Creates a report that shows how your image is used across other Amazon Web Services accounts. The report provides visibility into which accounts are using the specified image, and how many resources (EC2 instances or launch templates) are referencing it. |
| `CreateInstanceConnectEndpoint` | - | `idempotency-token` | `SubnetId` | `ClientToken` | `CreateInstanceConnectEndpointResult` | - | Creates an EC2 Instance Connect Endpoint. An EC2 Instance Connect Endpoint allows you to connect to an instance, without requiring the instance to have a public IPv4 or public IPv6 address. |
| `CreateInstanceEventWindow` | - | - | - | - | `CreateInstanceEventWindowResult` | - | Creates an event window in which scheduled events for the associated Amazon EC2 instances can run. You can define either a set of time ranges or a cron expression when creating the event window, but not both. |
| `CreateInstanceExportTask` | - | - | `ExportToS3Task`, `InstanceId`, `TargetEnvironment` | - | `CreateInstanceExportTaskResult` | - | Exports a running or stopped instance to an Amazon S3 bucket. For information about the prerequisites for your Amazon S3 bucket, supported operating systems, image formats, and known limitations for the types of instances you can export, see Exporting an... |
| `CreateInternetGateway` | - | - | - | - | `CreateInternetGatewayResult` | - | Creates an internet gateway for use with a VPC. After creating the internet gateway, you attach it to a VPC using AttachInternetGateway. |
| `CreateInterruptibleCapacityReservationAllocation` | - | `idempotency-token` | `CapacityReservationId`, `InstanceCount` | `ClientToken` | `CreateInterruptibleCapacityReservationAllocationResult` | - | Creates an interruptible Capacity Reservation by specifying the number of unused instances you want to allocate from your source reservation. This helps you make unused capacity available for other workloads within your account while maintaining control to... |
| `CreateIpam` | - | `idempotency-token` | - | `ClientToken` | `CreateIpamResult` | - | Create an IPAM. Amazon VPC IP Address Manager (IPAM) is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across Amazon Web Services Regions and accounts... |
| `CreateIpamExternalResourceVerificationToken` | - | `idempotency-token` | `IpamId` | `ClientToken` | `CreateIpamExternalResourceVerificationTokenResult` | - | Create a verification token. A verification token is an Amazon Web Services-generated random value that you can use to prove ownership of an external resource. |
| `CreateIpamPolicy` | - | `idempotency-token` | `IpamId` | `ClientToken` | `CreateIpamPolicyResult` | - | Creates an IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `CreateIpamPool` | - | `idempotency-token` | `AddressFamily`, `IpamScopeId` | `ClientToken` | `CreateIpamPoolResult` | - | Create an IP address pool for Amazon VPC IP Address Manager (IPAM). In IPAM, a pool is a collection of contiguous IP addresses CIDRs. |
| `CreateIpamPrefixListResolver` | - | `idempotency-token` | `AddressFamily`, `IpamId` | `ClientToken` | `CreateIpamPrefixListResolverResult` | - | Creates an IPAM prefix list resolver. An IPAM prefix list resolver is a component that manages the synchronization between IPAM's CIDR selection rules and customer-managed prefix lists. |
| `CreateIpamPrefixListResolverTarget` | - | `idempotency-token` | `IpamPrefixListResolverId`, `PrefixListId`, `PrefixListRegion`, `TrackLatestVersion` | `ClientToken` | `CreateIpamPrefixListResolverTargetResult` | - | Creates an IPAM prefix list resolver target. An IPAM prefix list resolver target is an association between a specific customer-managed prefix list and an IPAM prefix list resolver. |
| `CreateIpamResourceDiscovery` | - | `idempotency-token` | - | `ClientToken` | `CreateIpamResourceDiscoveryResult` | - | Creates an IPAM resource discovery. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account. |
| `CreateIpamScope` | - | `idempotency-token` | `IpamId` | `ClientToken` | `CreateIpamScopeResult` | - | Create an IPAM scope. In IPAM, a scope is the highest-level container within IPAM. |
| `CreateKeyPair` | - | - | `KeyName` | - | `KeyPair` | - | Creates an ED25519 or 2048-bit RSA key pair with the specified name and in the specified format. Amazon EC2 stores the public key and displays the private key for you to save to a file. |
| `CreateLaunchTemplate` | - | `idempotency-token` | `LaunchTemplateData`, `LaunchTemplateName` | `ClientToken` | `CreateLaunchTemplateResult` | - | Creates a launch template. A launch template contains the parameters to launch an instance. |
| `CreateLaunchTemplateVersion` | - | `idempotency-token` | `LaunchTemplateData` | `ClientToken` | `CreateLaunchTemplateVersionResult` | - | Creates a new version of a launch template. You must specify an existing launch template, either by name or ID. |
| `CreateLocalGatewayRoute` | - | - | `LocalGatewayRouteTableId` | - | `CreateLocalGatewayRouteResult` | - | Creates a static route for the specified local gateway route table. You must specify one of the following targets: `LocalGatewayVirtualInterfaceGroupId` `NetworkInterfaceId` |
| `CreateLocalGatewayRouteTable` | - | - | `LocalGatewayId` | - | `CreateLocalGatewayRouteTableResult` | - | Creates a local gateway route table. |
| `CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation` | - | - | `LocalGatewayRouteTableId`, `LocalGatewayVirtualInterfaceGroupId` | - | `CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationResult` | - | Creates a local gateway route table virtual interface group association. |
| `CreateLocalGatewayRouteTableVpcAssociation` | - | - | `LocalGatewayRouteTableId`, `VpcId` | - | `CreateLocalGatewayRouteTableVpcAssociationResult` | - | Associates the specified VPC with the specified local gateway route table. |
| `CreateLocalGatewayVirtualInterface` | - | - | `LocalAddress`, `LocalGatewayVirtualInterfaceGroupId`, `OutpostLagId`, `PeerAddress`, `Vlan` | - | `CreateLocalGatewayVirtualInterfaceResult` | - | Create a virtual interface for a local gateway. |
| `CreateLocalGatewayVirtualInterfaceGroup` | - | - | `LocalGatewayId` | - | `CreateLocalGatewayVirtualInterfaceGroupResult` | - | Create a local gateway virtual interface group. |
| `CreateMacSystemIntegrityProtectionModificationTask` | - | `idempotency-token` | `InstanceId`, `MacSystemIntegrityProtectionStatus` | `ClientToken` | `CreateMacSystemIntegrityProtectionModificationTaskResult` | - | Creates a System Integrity Protection (SIP) modification task to configure the SIP settings for an x86 Mac instance or Apple silicon Mac instance. For more information, see Configure SIP for Amazon EC2 instances in the Amazon EC2 User Guide . |
| `CreateManagedPrefixList` | - | `idempotency-token` | `AddressFamily`, `MaxEntries`, `PrefixListName` | `ClientToken` | `CreateManagedPrefixListResult` | - | Creates a managed prefix list. You can specify entries for the prefix list. |
| `CreateNatGateway` | - | `idempotency-token` | - | `ClientToken` | `CreateNatGatewayResult` | - | Creates a NAT gateway in the specified subnet. This action creates a network interface in the specified subnet with a private IP address from the IP address range of the subnet. |
| `CreateNetworkAcl` | - | `idempotency-token` | `VpcId` | `ClientToken` | `CreateNetworkAclResult` | - | Creates a network ACL in a VPC. Network ACLs provide an optional layer of security (in addition to security groups) for the instances in your VPC. |
| `CreateNetworkAclEntry` | - | - | `Egress`, `NetworkAclId`, `Protocol`, `RuleAction`, `RuleNumber` | - | `Unit` | - | Creates an entry (a rule) in a network ACL with the specified rule number. Each network ACL has a set of numbered ingress rules and a separate set of numbered egress rules. |
| `CreateNetworkInsightsAccessScope` | - | `idempotency-token` | `ClientToken` | `ClientToken` | `CreateNetworkInsightsAccessScopeResult` | - | Creates a Network Access Scope. Amazon Web Services Network Access Analyzer enables cloud networking and cloud operations teams to verify that their networks on Amazon Web Services conform to their network security and governance objectives. |
| `CreateNetworkInsightsPath` | - | `idempotency-token` | `ClientToken`, `Protocol`, `Source` | `ClientToken` | `CreateNetworkInsightsPathResult` | - | Creates a path to analyze for reachability. Reachability Analyzer enables you to analyze and debug network reachability between two resources in your virtual private cloud (VPC). |
| `CreateNetworkInterface` | - | `idempotency-token` | `SubnetId` | `ClientToken` | `CreateNetworkInterfaceResult` | - | Creates a network interface in the specified subnet. The number of IP addresses you can assign to a network interface varies by instance type. |
| `CreateNetworkInterfacePermission` | - | - | `NetworkInterfaceId`, `Permission` | - | `CreateNetworkInterfacePermissionResult` | - | Grants an Amazon Web Services-authorized account permission to attach the specified network interface to an instance in their account. You can grant permission to a single Amazon Web Services account only, and only one account at a time. |
| `CreatePlacementGroup` | - | - | - | - | `CreatePlacementGroupResult` | - | Creates a placement group in which to launch instances. The strategy of the placement group determines how the instances are organized within the group. |
| `CreatePublicIpv4Pool` | - | - | - | - | `CreatePublicIpv4PoolResult` | - | Creates a public IPv4 address pool. A public IPv4 pool is an EC2 IP address pool required for the public IPv4 CIDRs that you own and bring to Amazon Web Services to manage with IPAM. |
| `CreateReplaceRootVolumeTask` | - | `idempotency-token` | `InstanceId` | `ClientToken` | `CreateReplaceRootVolumeTaskResult` | - | Replaces the EBS-backed root volume for a `running` instance with a new volume that is restored to the original root volume's launch state, that is restored to a specific snapshot taken from the original root volume, or that is restored from an AMI that has... |
| `CreateReservedInstancesListing` | - | - | `ClientToken`, `InstanceCount`, `PriceSchedules`, `ReservedInstancesId` | - | `CreateReservedInstancesListingResult` | - | Creates a listing for Amazon EC2 Standard Reserved Instances to be sold in the Reserved Instance Marketplace. You can submit one Standard Reserved Instance listing at a time. |
| `CreateRestoreImageTask` | - | - | `Bucket`, `ObjectKey` | - | `CreateRestoreImageTaskResult` | - | Starts a task that restores an AMI from an Amazon S3 object that was previously created by using CreateStoreImageTask. To use this API, you must have the required permissions. |
| `CreateRoute` | - | - | `RouteTableId` | - | `CreateRouteResult` | - | Creates a route in a route table within a VPC. You must specify either a destination CIDR block or a prefix list ID. |
| `CreateRouteServer` | - | `idempotency-token` | `AmazonSideAsn` | `ClientToken` | `CreateRouteServerResult` | - | Creates a new route server to manage dynamic routing in a VPC. Amazon VPC Route Server simplifies routing for traffic between workloads that are deployed within a VPC and its internet gateways. |
| `CreateRouteServerEndpoint` | - | `idempotency-token` | `RouteServerId`, `SubnetId` | `ClientToken` | `CreateRouteServerEndpointResult` | - | Creates a new endpoint for a route server in a specified subnet. A route server endpoint is an Amazon Web Services-managed component inside a subnet that facilitates BGP (Border Gateway Protocol) connections between your route server and your BGP peers. |
| `CreateRouteServerPeer` | - | - | `BgpOptions`, `PeerAddress`, `RouteServerEndpointId` | - | `CreateRouteServerPeerResult` | - | Creates a new BGP peer for a specified route server endpoint. A route server peer is a session between a route server endpoint and the device deployed in Amazon Web Services (such as a firewall appliance or other network security function running on an EC2... |
| `CreateRouteTable` | - | `idempotency-token` | `VpcId` | `ClientToken` | `CreateRouteTableResult` | - | Creates a route table for the specified VPC. After you create a route table, you can add routes and associate the table with a subnet. |
| `CreateSecondaryNetwork` | - | `idempotency-token` | `Ipv4CidrBlock`, `NetworkType` | `ClientToken` | `CreateSecondaryNetworkResult` | - | Creates a secondary network. The allowed size for a secondary network CIDR block is between /28 netmask (16 IP addresses) and /12 netmask (1,048,576 IP addresses). |
| `CreateSecondarySubnet` | - | `idempotency-token` | `Ipv4CidrBlock`, `SecondaryNetworkId` | `ClientToken` | `CreateSecondarySubnetResult` | - | Creates a secondary subnet in a secondary network. A secondary subnet CIDR block must not overlap with the CIDR block of an existing secondary subnet in the secondary network. |
| `CreateSecurityGroup` | - | - | `Description`, `GroupName` | - | `CreateSecurityGroupResult` | - | Creates a security group. A security group acts as a virtual firewall for your instance to control inbound and outbound traffic. |
| `CreateSnapshot` | - | - | `VolumeId` | - | `Snapshot` | - | Creates a snapshot of an EBS volume and stores it in Amazon S3. You can use snapshots for backups, to make copies of EBS volumes, and to save data before shutting down an instance. |
| `CreateSnapshots` | - | - | `InstanceSpecification` | - | `CreateSnapshotsResult` | - | Creates crash-consistent snapshots of multiple EBS volumes attached to an Amazon EC2 instance. Volumes are chosen by specifying an instance. |
| `CreateSpotDatafeedSubscription` | - | - | `Bucket` | - | `CreateSpotDatafeedSubscriptionResult` | - | Creates a data feed for Spot Instances, enabling you to view Spot Instance usage logs. You can create one data feed per Amazon Web Services account. |
| `CreateStoreImageTask` | - | - | `Bucket`, `ImageId` | - | `CreateStoreImageTaskResult` | - | Stores an AMI as a single object in an Amazon S3 bucket. To use this API, you must have the required permissions. |
| `CreateSubnet` | - | - | `VpcId` | - | `CreateSubnetResult` | - | Creates a subnet in the specified VPC. For an IPv4 only subnet, specify an IPv4 CIDR block. |
| `CreateSubnetCidrReservation` | - | - | `Cidr`, `ReservationType`, `SubnetId` | - | `CreateSubnetCidrReservationResult` | - | Creates a subnet CIDR reservation. For more information, see Subnet CIDR reservations in the Amazon VPC User Guide and Manage prefixes for your network interfaces in the Amazon EC2 User Guide . |
| `CreateTags` | - | - | `Resources`, `Tags` | - | `Unit` | - | Adds or overwrites only the specified tags for the specified Amazon EC2 resource or resources. When you specify an existing tag key, the value is overwritten with the new value. |
| `CreateTrafficMirrorFilter` | - | `idempotency-token` | - | `ClientToken` | `CreateTrafficMirrorFilterResult` | - | Creates a Traffic Mirror filter. A Traffic Mirror filter is a set of rules that defines the traffic to mirror. |
| `CreateTrafficMirrorFilterRule` | - | `idempotency-token` | `DestinationCidrBlock`, `RuleAction`, `RuleNumber`, `SourceCidrBlock`, `TrafficDirection`, `TrafficMirrorFilterId` | `ClientToken` | `CreateTrafficMirrorFilterRuleResult` | - | Creates a Traffic Mirror filter rule. A Traffic Mirror rule defines the Traffic Mirror source traffic to mirror. |
| `CreateTrafficMirrorSession` | - | `idempotency-token` | `NetworkInterfaceId`, `SessionNumber`, `TrafficMirrorFilterId`, `TrafficMirrorTargetId` | `ClientToken` | `CreateTrafficMirrorSessionResult` | - | Creates a Traffic Mirror session. A Traffic Mirror session actively copies packets from a Traffic Mirror source to a Traffic Mirror target. |
| `CreateTrafficMirrorTarget` | - | `idempotency-token` | - | `ClientToken` | `CreateTrafficMirrorTargetResult` | - | Creates a target for your Traffic Mirror session. A Traffic Mirror target is the destination for mirrored traffic. |
| `CreateTransitGateway` | - | - | - | - | `CreateTransitGatewayResult` | - | Creates a transit gateway. You can use a transit gateway to interconnect your virtual private clouds (VPC) and on-premises networks. |
| `CreateTransitGatewayConnect` | - | - | `Options`, `TransportTransitGatewayAttachmentId` | - | `CreateTransitGatewayConnectResult` | - | Creates a Connect attachment from a specified transit gateway attachment. A Connect attachment is a GRE-based tunnel attachment that you can use to establish a connection between a transit gateway and an appliance. |
| `CreateTransitGatewayConnectPeer` | - | - | `InsideCidrBlocks`, `PeerAddress`, `TransitGatewayAttachmentId` | - | `CreateTransitGatewayConnectPeerResult` | - | Creates a Connect peer for a specified transit gateway Connect attachment between a transit gateway and an appliance. The peer address and transit gateway address must be the same IP address family (IPv4 or IPv6). |
| `CreateTransitGatewayMeteringPolicy` | - | - | `TransitGatewayId` | - | `CreateTransitGatewayMeteringPolicyResult` | - | Creates a metering policy for a transit gateway to track and measure network traffic. |
| `CreateTransitGatewayMeteringPolicyEntry` | - | - | `MeteredAccount`, `PolicyRuleNumber`, `TransitGatewayMeteringPolicyId` | - | `CreateTransitGatewayMeteringPolicyEntryResult` | - | Creates an entry in a transit gateway metering policy to define traffic measurement rules. |
| `CreateTransitGatewayMulticastDomain` | - | - | `TransitGatewayId` | - | `CreateTransitGatewayMulticastDomainResult` | - | Creates a multicast domain using the specified transit gateway. The transit gateway must be in the available state before you create a domain. |
| `CreateTransitGatewayPeeringAttachment` | - | - | `PeerAccountId`, `PeerRegion`, `PeerTransitGatewayId`, `TransitGatewayId` | - | `CreateTransitGatewayPeeringAttachmentResult` | - | Requests a transit gateway peering attachment between the specified transit gateway (requester) and a peer transit gateway (accepter). The peer transit gateway can be in your account or a different Amazon Web Services account. |
| `CreateTransitGatewayPolicyTable` | - | - | `TransitGatewayId` | - | `CreateTransitGatewayPolicyTableResult` | - | Creates a transit gateway policy table. |
| `CreateTransitGatewayPrefixListReference` | - | - | `PrefixListId`, `TransitGatewayRouteTableId` | - | `CreateTransitGatewayPrefixListReferenceResult` | - | Creates a reference (route) to a prefix list in a specified transit gateway route table. |
| `CreateTransitGatewayRoute` | - | - | `DestinationCidrBlock`, `TransitGatewayRouteTableId` | - | `CreateTransitGatewayRouteResult` | - | Creates a static route for the specified transit gateway route table. |
| `CreateTransitGatewayRouteTable` | - | - | `TransitGatewayId` | - | `CreateTransitGatewayRouteTableResult` | - | Creates a route table for the specified transit gateway. |
| `CreateTransitGatewayRouteTableAnnouncement` | - | - | `PeeringAttachmentId`, `TransitGatewayRouteTableId` | - | `CreateTransitGatewayRouteTableAnnouncementResult` | - | Advertises a new transit gateway route table. |
| `CreateTransitGatewayVpcAttachment` | - | - | `SubnetIds`, `TransitGatewayId`, `VpcId` | - | `CreateTransitGatewayVpcAttachmentResult` | - | Attaches the specified VPC to the specified transit gateway. If you attach a VPC with a CIDR range that overlaps the CIDR range of a VPC that is already attached, the new VPC CIDR range is not propagated to the default propagation route table. |
| `CreateVerifiedAccessEndpoint` | - | `idempotency-token` | `AttachmentType`, `EndpointType`, `VerifiedAccessGroupId` | `ClientToken` | `CreateVerifiedAccessEndpointResult` | - | An Amazon Web Services Verified Access endpoint is where you define your application along with an optional endpoint-level access policy. |
| `CreateVerifiedAccessGroup` | - | `idempotency-token` | `VerifiedAccessInstanceId` | `ClientToken` | `CreateVerifiedAccessGroupResult` | - | An Amazon Web Services Verified Access group is a collection of Amazon Web Services Verified Access endpoints who's associated applications have similar security requirements. Each instance within a Verified Access group shares an Verified Access policy. |
| `CreateVerifiedAccessInstance` | - | `idempotency-token` | - | `ClientToken` | `CreateVerifiedAccessInstanceResult` | - | An Amazon Web Services Verified Access instance is a regional entity that evaluates application requests and grants access only when your security requirements are met. |
| `CreateVerifiedAccessTrustProvider` | - | `idempotency-token` | `PolicyReferenceName`, `TrustProviderType` | `ClientToken` | `CreateVerifiedAccessTrustProviderResult` | - | A trust provider is a third-party entity that creates, maintains, and manages identity information for users and devices. When an application request is made, the identity information sent by the trust provider is evaluated by Verified Access before allowing... |
| `CreateVolume` | - | `idempotency-token` | - | `ClientToken` | `Volume` | - | Creates an EBS volume that can be attached to an instance in the same Availability Zone. You can create a new empty volume or restore a volume from an EBS snapshot. |
| `CreateVpc` | - | - | - | - | `CreateVpcResult` | - | Creates a VPC with the specified CIDR blocks. A VPC must have an associated IPv4 CIDR block. |
| `CreateVpcBlockPublicAccessExclusion` | - | - | `InternetGatewayExclusionMode` | - | `CreateVpcBlockPublicAccessExclusionResult` | - | Create a VPC Block Public Access (BPA) exclusion. A VPC BPA exclusion is a mode that can be applied to a single VPC or subnet that exempts it from the account’s BPA mode and will allow bidirectional or egress-only access. |
| `CreateVpcEncryptionControl` | - | - | `VpcId` | - | `CreateVpcEncryptionControlResult` | - | Creates a VPC Encryption Control configuration for a specified VPC. VPC Encryption Control enables you to enforce encryption for all data in transit within and between VPCs to meet compliance requirements for standards like HIPAA, FedRAMP, and PCI DSS. |
| `CreateVpcEndpoint` | - | - | `VpcId` | - | `CreateVpcEndpointResult` | - | Creates a VPC endpoint. A VPC endpoint provides a private connection between the specified VPC and the specified endpoint service. |
| `CreateVpcEndpointConnectionNotification` | - | - | `ConnectionEvents`, `ConnectionNotificationArn` | - | `CreateVpcEndpointConnectionNotificationResult` | - | Creates a connection notification for a specified VPC endpoint or VPC endpoint service. A connection notification notifies you of specific endpoint events. |
| `CreateVpcEndpointServiceConfiguration` | - | - | - | - | `CreateVpcEndpointServiceConfigurationResult` | - | Creates a VPC endpoint service to which service consumers (Amazon Web Services accounts, users, and IAM roles) can connect. Before you create an endpoint service, you must create one of the following for your service: A Network Load Balancer. |
| `CreateVpcPeeringConnection` | - | - | `VpcId` | - | `CreateVpcPeeringConnectionResult` | - | Requests a VPC peering connection between two VPCs: a requester VPC that you own and an accepter VPC with which to create the connection. The accepter VPC can belong to another Amazon Web Services account and can be in a different Region to the requester VPC. |
| `CreateVpnConcentrator` | - | - | `Type` | - | `CreateVpnConcentratorResult` | - | Creates a VPN concentrator that aggregates multiple VPN connections to a transit gateway. |
| `CreateVpnConnection` | - | - | `CustomerGatewayId`, `Type` | - | `CreateVpnConnectionResult` | - | Creates a VPN connection between an existing virtual private gateway or transit gateway and a customer gateway. The supported connection type is `ipsec.1`. |
| `CreateVpnConnectionRoute` | - | - | `DestinationCidrBlock`, `VpnConnectionId` | - | `Unit` | - | Creates a static route associated with a VPN connection between an existing virtual private gateway and a VPN customer gateway. The static route allows traffic to be routed from the virtual private gateway to the VPN customer gateway. |
| `CreateVpnGateway` | - | - | `Type` | - | `CreateVpnGatewayResult` | - | Creates a virtual private gateway. A virtual private gateway is the endpoint on the VPC side of your VPN connection. |
| `DeleteCapacityManagerDataExport` | - | - | `CapacityManagerDataExportId` | - | `DeleteCapacityManagerDataExportResult` | - | Deletes an existing Capacity Manager data export configuration. This stops future scheduled exports but does not delete previously exported files from S3. |
| `DeleteCarrierGateway` | - | - | `CarrierGatewayId` | - | `DeleteCarrierGatewayResult` | - | Deletes a carrier gateway. If you do not delete the route that contains the carrier gateway as the Target, the route is a blackhole route. |
| `DeleteClientVpnEndpoint` | - | - | `ClientVpnEndpointId` | - | `DeleteClientVpnEndpointResult` | - | Deletes the specified Client VPN endpoint. You must disassociate all target networks before you can delete a Client VPN endpoint. |
| `DeleteClientVpnRoute` | - | - | `ClientVpnEndpointId`, `DestinationCidrBlock` | - | `DeleteClientVpnRouteResult` | - | Deletes a route from a Client VPN endpoint. You can only delete routes that you manually added using the CreateClientVpnRoute action. |
| `DeleteCoipCidr` | - | - | `Cidr`, `CoipPoolId` | - | `DeleteCoipCidrResult` | - | Deletes a range of customer-owned IP addresses. |
| `DeleteCoipPool` | - | - | `CoipPoolId` | - | `DeleteCoipPoolResult` | - | Deletes a pool of customer-owned IP (CoIP) addresses. |
| `DeleteCustomerGateway` | - | - | `CustomerGatewayId` | - | `Unit` | - | Deletes the specified customer gateway. You must delete the VPN connection before you can delete the customer gateway. |
| `DeleteDhcpOptions` | - | - | `DhcpOptionsId` | - | `Unit` | - | Deletes the specified set of DHCP options. You must disassociate the set of DHCP options before you can delete it. |
| `DeleteEgressOnlyInternetGateway` | - | - | `EgressOnlyInternetGatewayId` | - | `DeleteEgressOnlyInternetGatewayResult` | - | Deletes an egress-only internet gateway. |
| `DeleteFleets` | - | - | `FleetIds`, `TerminateInstances` | - | `DeleteFleetsResult` | - | Deletes the specified EC2 Fleet request. After you delete an EC2 Fleet request, it launches no new instances. |
| `DeleteFlowLogs` | - | - | `FlowLogIds` | - | `DeleteFlowLogsResult` | - | Deletes one or more flow logs. |
| `DeleteFpgaImage` | - | - | `FpgaImageId` | - | `DeleteFpgaImageResult` | - | Deletes the specified Amazon FPGA Image (AFI). |
| `DeleteImageUsageReport` | - | - | `ReportId` | - | `DeleteImageUsageReportResult` | - | Deletes the specified image usage report. For more information, see View your AMI usage in the Amazon EC2 User Guide . |
| `DeleteInstanceConnectEndpoint` | - | - | `InstanceConnectEndpointId` | - | `DeleteInstanceConnectEndpointResult` | - | Deletes the specified EC2 Instance Connect Endpoint. |
| `DeleteInstanceEventWindow` | - | - | `InstanceEventWindowId` | - | `DeleteInstanceEventWindowResult` | - | Deletes the specified event window. For more information, see Define event windows for scheduled events in the Amazon EC2 User Guide . |
| `DeleteInternetGateway` | - | - | `InternetGatewayId` | - | `Unit` | - | Deletes the specified internet gateway. You must detach the internet gateway from the VPC before you can delete it. |
| `DeleteIpam` | - | - | `IpamId` | - | `DeleteIpamResult` | - | Delete an IPAM. Deleting an IPAM removes all monitored data associated with the IPAM including the historical data for CIDRs. |
| `DeleteIpamExternalResourceVerificationToken` | - | - | `IpamExternalResourceVerificationTokenId` | - | `DeleteIpamExternalResourceVerificationTokenResult` | - | Delete a verification token. A verification token is an Amazon Web Services-generated random value that you can use to prove ownership of an external resource. |
| `DeleteIpamPolicy` | - | - | `IpamPolicyId` | - | `DeleteIpamPolicyResult` | - | Deletes an IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `DeleteIpamPool` | - | - | `IpamPoolId` | - | `DeleteIpamPoolResult` | - | Delete an IPAM pool. You cannot delete an IPAM pool if there are allocations in it or CIDRs provisioned to it. |
| `DeleteIpamPrefixListResolver` | - | - | `IpamPrefixListResolverId` | - | `DeleteIpamPrefixListResolverResult` | - | Deletes an IPAM prefix list resolver. Before deleting a resolver, you must first delete all resolver targets associated with it. |
| `DeleteIpamPrefixListResolverTarget` | - | - | `IpamPrefixListResolverTargetId` | - | `DeleteIpamPrefixListResolverTargetResult` | - | Deletes an IPAM prefix list resolver target. This removes the association between the resolver and the managed prefix list, stopping automatic CIDR synchronization. |
| `DeleteIpamResourceDiscovery` | - | - | `IpamResourceDiscoveryId` | - | `DeleteIpamResourceDiscoveryResult` | - | Deletes an IPAM resource discovery. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account. |
| `DeleteIpamScope` | - | - | `IpamScopeId` | - | `DeleteIpamScopeResult` | - | Delete the scope for an IPAM. You cannot delete the default scopes. |
| `DeleteKeyPair` | - | - | - | - | `DeleteKeyPairResult` | - | Deletes the specified key pair, by removing the public key from Amazon EC2. |
| `DeleteLaunchTemplate` | - | - | - | - | `DeleteLaunchTemplateResult` | - | Deletes a launch template. Deleting a launch template deletes all of its versions. |
| `DeleteLaunchTemplateVersions` | - | - | `Versions` | - | `DeleteLaunchTemplateVersionsResult` | - | Deletes one or more versions of a launch template. You can't delete the default version of a launch template; you must first assign a different version as the default. |
| `DeleteLocalGatewayRoute` | - | - | `LocalGatewayRouteTableId` | - | `DeleteLocalGatewayRouteResult` | - | Deletes the specified route from the specified local gateway route table. |
| `DeleteLocalGatewayRouteTable` | - | - | `LocalGatewayRouteTableId` | - | `DeleteLocalGatewayRouteTableResult` | - | Deletes a local gateway route table. |
| `DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation` | - | - | `LocalGatewayRouteTableVirtualInterfaceGroupAssociationId` | - | `DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationResult` | - | Deletes a local gateway route table virtual interface group association. |
| `DeleteLocalGatewayRouteTableVpcAssociation` | - | - | `LocalGatewayRouteTableVpcAssociationId` | - | `DeleteLocalGatewayRouteTableVpcAssociationResult` | - | Deletes the specified association between a VPC and local gateway route table. |
| `DeleteLocalGatewayVirtualInterface` | - | - | `LocalGatewayVirtualInterfaceId` | - | `DeleteLocalGatewayVirtualInterfaceResult` | - | Deletes the specified local gateway virtual interface. |
| `DeleteLocalGatewayVirtualInterfaceGroup` | - | - | `LocalGatewayVirtualInterfaceGroupId` | - | `DeleteLocalGatewayVirtualInterfaceGroupResult` | - | Delete the specified local gateway interface group. |
| `DeleteManagedPrefixList` | - | - | `PrefixListId` | - | `DeleteManagedPrefixListResult` | - | Deletes the specified managed prefix list. You must first remove all references to the prefix list in your resources. |
| `DeleteNatGateway` | - | - | `NatGatewayId` | - | `DeleteNatGatewayResult` | - | Deletes the specified NAT gateway. Deleting a public NAT gateway disassociates its Elastic IP address, but does not release the address from your account. |
| `DeleteNetworkAcl` | - | - | `NetworkAclId` | - | `Unit` | - | Deletes the specified network ACL. You can't delete the ACL if it's associated with any subnets. |
| `DeleteNetworkAclEntry` | - | - | `Egress`, `NetworkAclId`, `RuleNumber` | - | `Unit` | - | Deletes the specified ingress or egress entry (rule) from the specified network ACL. |
| `DeleteNetworkInsightsAccessScope` | - | - | `NetworkInsightsAccessScopeId` | - | `DeleteNetworkInsightsAccessScopeResult` | - | Deletes the specified Network Access Scope. |
| `DeleteNetworkInsightsAccessScopeAnalysis` | - | - | `NetworkInsightsAccessScopeAnalysisId` | - | `DeleteNetworkInsightsAccessScopeAnalysisResult` | - | Deletes the specified Network Access Scope analysis. |
| `DeleteNetworkInsightsAnalysis` | - | - | `NetworkInsightsAnalysisId` | - | `DeleteNetworkInsightsAnalysisResult` | - | Deletes the specified network insights analysis. |
| `DeleteNetworkInsightsPath` | - | - | `NetworkInsightsPathId` | - | `DeleteNetworkInsightsPathResult` | - | Deletes the specified path. |
| `DeleteNetworkInterface` | - | - | `NetworkInterfaceId` | - | `Unit` | - | Deletes the specified network interface. You must detach the network interface before you can delete it. |
| `DeleteNetworkInterfacePermission` | - | - | `NetworkInterfacePermissionId` | - | `DeleteNetworkInterfacePermissionResult` | - | Deletes a permission for a network interface. By default, you cannot delete the permission if the account for which you're removing the permission has attached the network interface to an instance. |
| `DeletePlacementGroup` | - | - | `GroupName` | - | `Unit` | - | Deletes the specified placement group. You must terminate all instances in the placement group before you can delete the placement group. |
| `DeletePublicIpv4Pool` | - | - | `PoolId` | - | `DeletePublicIpv4PoolResult` | - | Delete a public IPv4 pool. A public IPv4 pool is an EC2 IP address pool required for the public IPv4 CIDRs that you own and bring to Amazon Web Services to manage with IPAM. |
| `DeleteQueuedReservedInstances` | - | - | `ReservedInstancesIds` | - | `DeleteQueuedReservedInstancesResult` | - | Deletes the queued purchases for the specified Reserved Instances. |
| `DeleteRoute` | - | - | `RouteTableId` | - | `Unit` | - | Deletes the specified route from the specified route table. |
| `DeleteRouteServer` | - | - | `RouteServerId` | - | `DeleteRouteServerResult` | - | Deletes the specified route server. Amazon VPC Route Server simplifies routing for traffic between workloads that are deployed within a VPC and its internet gateways. |
| `DeleteRouteServerEndpoint` | - | - | `RouteServerEndpointId` | - | `DeleteRouteServerEndpointResult` | - | Deletes the specified route server endpoint. A route server endpoint is an Amazon Web Services-managed component inside a subnet that facilitates BGP (Border Gateway Protocol) connections between your route server and your BGP peers. |
| `DeleteRouteServerPeer` | - | - | `RouteServerPeerId` | - | `DeleteRouteServerPeerResult` | - | Deletes the specified BGP peer from a route server. A route server peer is a session between a route server endpoint and the device deployed in Amazon Web Services (such as a firewall appliance or other network security function running on an EC2 instance). |
| `DeleteRouteTable` | - | - | `RouteTableId` | - | `Unit` | - | Deletes the specified route table. You must disassociate the route table from any subnets before you can delete it. |
| `DeleteSecondaryNetwork` | - | `idempotency-token` | `SecondaryNetworkId` | `ClientToken` | `DeleteSecondaryNetworkResult` | - | Deletes a secondary network. You must delete all secondary subnets in the secondary network before you can delete the secondary network. |
| `DeleteSecondarySubnet` | - | `idempotency-token` | `SecondarySubnetId` | `ClientToken` | `DeleteSecondarySubnetResult` | - | Deletes a secondary subnet. A secondary subnet must not contain any secondary interfaces prior to deletion. |
| `DeleteSecurityGroup` | - | - | - | - | `DeleteSecurityGroupResult` | - | Deletes a security group. If you attempt to delete a security group that is associated with an instance or network interface, is referenced by another security group in the same VPC, or has a VPC association, the operation fails with `DependencyViolation`. |
| `DeleteSnapshot` | - | - | `SnapshotId` | - | `Unit` | - | Deletes the specified snapshot. When you make periodic snapshots of a volume, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. |
| `DeleteSpotDatafeedSubscription` | - | - | - | - | `Unit` | - | Deletes the data feed for Spot Instances. |
| `DeleteSubnet` | - | - | `SubnetId` | - | `Unit` | - | Deletes the specified subnet. You must terminate all running instances in the subnet before you can delete the subnet. |
| `DeleteSubnetCidrReservation` | - | - | `SubnetCidrReservationId` | - | `DeleteSubnetCidrReservationResult` | - | Deletes a subnet CIDR reservation. |
| `DeleteTags` | - | - | `Resources` | - | `Unit` | - | Deletes the specified set of tags from the specified set of resources. To list the current tags, use DescribeTags. |
| `DeleteTrafficMirrorFilter` | - | - | `TrafficMirrorFilterId` | - | `DeleteTrafficMirrorFilterResult` | - | Deletes the specified Traffic Mirror filter. You cannot delete a Traffic Mirror filter that is in use by a Traffic Mirror session. |
| `DeleteTrafficMirrorFilterRule` | - | - | `TrafficMirrorFilterRuleId` | - | `DeleteTrafficMirrorFilterRuleResult` | - | Deletes the specified Traffic Mirror rule. |
| `DeleteTrafficMirrorSession` | - | - | `TrafficMirrorSessionId` | - | `DeleteTrafficMirrorSessionResult` | - | Deletes the specified Traffic Mirror session. |
| `DeleteTrafficMirrorTarget` | - | - | `TrafficMirrorTargetId` | - | `DeleteTrafficMirrorTargetResult` | - | Deletes the specified Traffic Mirror target. You cannot delete a Traffic Mirror target that is in use by a Traffic Mirror session. |
| `DeleteTransitGateway` | - | - | `TransitGatewayId` | - | `DeleteTransitGatewayResult` | - | Deletes the specified transit gateway. |
| `DeleteTransitGatewayConnect` | - | - | `TransitGatewayAttachmentId` | - | `DeleteTransitGatewayConnectResult` | - | Deletes the specified Connect attachment. You must first delete any Connect peers for the attachment. |
| `DeleteTransitGatewayConnectPeer` | - | - | `TransitGatewayConnectPeerId` | - | `DeleteTransitGatewayConnectPeerResult` | - | Deletes the specified Connect peer. |
| `DeleteTransitGatewayMeteringPolicy` | - | - | `TransitGatewayMeteringPolicyId` | - | `DeleteTransitGatewayMeteringPolicyResult` | - | Deletes a transit gateway metering policy. |
| `DeleteTransitGatewayMeteringPolicyEntry` | - | - | `PolicyRuleNumber`, `TransitGatewayMeteringPolicyId` | - | `DeleteTransitGatewayMeteringPolicyEntryResult` | - | Deletes an entry from a transit gateway metering policy. |
| `DeleteTransitGatewayMulticastDomain` | - | - | `TransitGatewayMulticastDomainId` | - | `DeleteTransitGatewayMulticastDomainResult` | - | Deletes the specified transit gateway multicast domain. |
| `DeleteTransitGatewayPeeringAttachment` | - | - | `TransitGatewayAttachmentId` | - | `DeleteTransitGatewayPeeringAttachmentResult` | - | Deletes a transit gateway peering attachment. |
| `DeleteTransitGatewayPolicyTable` | - | - | `TransitGatewayPolicyTableId` | - | `DeleteTransitGatewayPolicyTableResult` | - | Deletes the specified transit gateway policy table. |
| `DeleteTransitGatewayPrefixListReference` | - | - | `PrefixListId`, `TransitGatewayRouteTableId` | - | `DeleteTransitGatewayPrefixListReferenceResult` | - | Deletes a reference (route) to a prefix list in a specified transit gateway route table. |
| `DeleteTransitGatewayRoute` | - | - | `DestinationCidrBlock`, `TransitGatewayRouteTableId` | - | `DeleteTransitGatewayRouteResult` | - | Deletes the specified route from the specified transit gateway route table. |
| `DeleteTransitGatewayRouteTable` | - | - | `TransitGatewayRouteTableId` | - | `DeleteTransitGatewayRouteTableResult` | - | Deletes the specified transit gateway route table. If there are any route tables associated with the transit gateway route table, you must first run DisassociateRouteTable before you can delete the transit gateway route table. |
| `DeleteTransitGatewayRouteTableAnnouncement` | - | - | `TransitGatewayRouteTableAnnouncementId` | - | `DeleteTransitGatewayRouteTableAnnouncementResult` | - | Advertises to the transit gateway that a transit gateway route table is deleted. |
| `DeleteTransitGatewayVpcAttachment` | - | - | `TransitGatewayAttachmentId` | - | `DeleteTransitGatewayVpcAttachmentResult` | - | Deletes the specified VPC attachment. |
| `DeleteVerifiedAccessEndpoint` | - | `idempotency-token` | `VerifiedAccessEndpointId` | `ClientToken` | `DeleteVerifiedAccessEndpointResult` | - | Delete an Amazon Web Services Verified Access endpoint. |
| `DeleteVerifiedAccessGroup` | - | `idempotency-token` | `VerifiedAccessGroupId` | `ClientToken` | `DeleteVerifiedAccessGroupResult` | - | Delete an Amazon Web Services Verified Access group. |
| `DeleteVerifiedAccessInstance` | - | `idempotency-token` | `VerifiedAccessInstanceId` | `ClientToken` | `DeleteVerifiedAccessInstanceResult` | - | Delete an Amazon Web Services Verified Access instance. |
| `DeleteVerifiedAccessTrustProvider` | - | `idempotency-token` | `VerifiedAccessTrustProviderId` | `ClientToken` | `DeleteVerifiedAccessTrustProviderResult` | - | Delete an Amazon Web Services Verified Access trust provider. |
| `DeleteVolume` | - | - | `VolumeId` | - | `Unit` | - | Deletes the specified EBS volume. The volume must be in the `available` state (not attached to an instance). |
| `DeleteVpc` | - | - | `VpcId` | - | `Unit` | - | Deletes the specified VPC. You must detach or delete all gateways and resources that are associated with the VPC before you can delete it. |
| `DeleteVpcBlockPublicAccessExclusion` | - | - | `ExclusionId` | - | `DeleteVpcBlockPublicAccessExclusionResult` | - | Delete a VPC Block Public Access (BPA) exclusion. A VPC BPA exclusion is a mode that can be applied to a single VPC or subnet that exempts it from the account’s BPA mode and will allow bidirectional or egress-only access. |
| `DeleteVpcEncryptionControl` | - | - | `VpcEncryptionControlId` | - | `DeleteVpcEncryptionControlResult` | - | Deletes a VPC Encryption Control configuration. This removes the encryption policy enforcement from the specified VPC. |
| `DeleteVpcEndpointConnectionNotifications` | - | - | `ConnectionNotificationIds` | - | `DeleteVpcEndpointConnectionNotificationsResult` | - | Deletes the specified VPC endpoint connection notifications. |
| `DeleteVpcEndpointServiceConfigurations` | - | - | `ServiceIds` | - | `DeleteVpcEndpointServiceConfigurationsResult` | - | Deletes the specified VPC endpoint service configurations. Before you can delete an endpoint service configuration, you must reject any `Available` or `PendingAcceptance` interface endpoint connections that are attached to the service. |
| `DeleteVpcEndpoints` | - | - | `VpcEndpointIds` | - | `DeleteVpcEndpointsResult` | - | Deletes the specified VPC endpoints. When you delete a gateway endpoint, we delete the endpoint routes in the route tables for the endpoint. |
| `DeleteVpcPeeringConnection` | - | - | `VpcPeeringConnectionId` | - | `DeleteVpcPeeringConnectionResult` | - | Deletes a VPC peering connection. Either the owner of the requester VPC or the owner of the accepter VPC can delete the VPC peering connection if it's in the `active` state. |
| `DeleteVpnConcentrator` | - | - | `VpnConcentratorId` | - | `DeleteVpnConcentratorResult` | - | Deletes the specified VPN concentrator. |
| `DeleteVpnConnection` | - | - | `VpnConnectionId` | - | `Unit` | - | Deletes the specified VPN connection. If you're deleting the VPC and its associated components, we recommend that you detach the virtual private gateway from the VPC and delete the VPC before deleting the VPN connection. |
| `DeleteVpnConnectionRoute` | - | - | `DestinationCidrBlock`, `VpnConnectionId` | - | `Unit` | - | Deletes the specified static route associated with a VPN connection between an existing virtual private gateway and a VPN customer gateway. The static route allows traffic to be routed from the virtual private gateway to the VPN customer gateway. |
| `DeleteVpnGateway` | - | - | `VpnGatewayId` | - | `Unit` | - | Deletes the specified virtual private gateway. You must first detach the virtual private gateway from the VPC. |
| `DeprovisionByoipCidr` | - | - | `Cidr` | - | `DeprovisionByoipCidrResult` | - | Releases the specified address range that you provisioned for use with your Amazon Web Services resources through bring your own IP addresses (BYOIP) and deletes the corresponding address pool. Before you can release an address range, you must stop... |
| `DeprovisionIpamByoasn` | - | - | `Asn`, `IpamId` | - | `DeprovisionIpamByoasnResult` | - | Deprovisions your Autonomous System Number (ASN) from your Amazon Web Services account. This action can only be called after any BYOIP CIDR associations are removed from your Amazon Web Services account with DisassociateIpamByoasn. |
| `DeprovisionIpamPoolCidr` | - | - | `IpamPoolId` | - | `DeprovisionIpamPoolCidrResult` | - | Deprovision a CIDR provisioned from an IPAM pool. If you deprovision a CIDR from a pool that has a source pool, the CIDR is recycled back into the source pool. |
| `DeprovisionPublicIpv4PoolCidr` | - | - | `Cidr`, `PoolId` | - | `DeprovisionPublicIpv4PoolCidrResult` | - | Deprovision a CIDR from a public IPv4 pool. |
| `DeregisterImage` | - | - | `ImageId` | - | `DeregisterImageResult` | - | Deregisters the specified AMI. A deregistered AMI can't be used to launch new instances. |
| `DeregisterInstanceEventNotificationAttributes` | - | - | `InstanceTagAttribute` | - | `DeregisterInstanceEventNotificationAttributesResult` | - | Deregisters tag keys to prevent tags that have the specified tag keys from being included in scheduled event notifications for resources in the Region. |
| `DeregisterTransitGatewayMulticastGroupMembers` | - | - | - | - | `DeregisterTransitGatewayMulticastGroupMembersResult` | - | Deregisters the specified members (network interfaces) from the transit gateway multicast group. |
| `DeregisterTransitGatewayMulticastGroupSources` | - | - | - | - | `DeregisterTransitGatewayMulticastGroupSourcesResult` | - | Deregisters the specified sources (network interfaces) from the transit gateway multicast group. |
| `DescribeAccountAttributes` | - | - | - | - | `DescribeAccountAttributesResult` | - | Describes attributes of your Amazon Web Services account. The following are the supported account attributes: `default-vpc`: The ID of the default VPC for your account, or `none`. |
| `DescribeAddressTransfers` | - | `paginated` | - | - | `DescribeAddressTransfersResult` | - | Describes an Elastic IP address transfer. For more information, see Transfer Elastic IP addresses in the Amazon VPC User Guide . |
| `DescribeAddresses` | - | - | - | - | `DescribeAddressesResult` | - | Describes the specified Elastic IP addresses or all of your Elastic IP addresses. |
| `DescribeAddressesAttribute` | - | `paginated` | - | - | `DescribeAddressesAttributeResult` | - | Describes the attributes of the specified Elastic IP addresses. For requirements, see Using reverse DNS for email applications. |
| `DescribeAggregateIdFormat` | - | - | - | - | `DescribeAggregateIdFormatResult` | - | Describes the longer ID format settings for all resource types in a specific Region. This request is useful for performing a quick audit to determine whether a specific Region is fully opted in for longer IDs (17-character IDs). |
| `DescribeAvailabilityZones` | - | - | - | - | `DescribeAvailabilityZonesResult` | - | Describes the Availability Zones, Local Zones, and Wavelength Zones that are available to you. For more information about Availability Zones, Local Zones, and Wavelength Zones, see Regions and zones in the Amazon EC2 User Guide . |
| `DescribeAwsNetworkPerformanceMetricSubscriptions` | - | `paginated` | - | - | `DescribeAwsNetworkPerformanceMetricSubscriptionsResult` | - | Describes the current Infrastructure Performance metric subscriptions. |
| `DescribeBundleTasks` | - | - | - | - | `DescribeBundleTasksResult` | - | Describes the specified bundle tasks or all of your bundle tasks. Completed bundle tasks are listed for only a limited time. |
| `DescribeByoipCidrs` | - | `paginated` | `MaxResults` | - | `DescribeByoipCidrsResult` | - | Describes the IP address ranges that were provisioned for use with Amazon Web Services resources through through bring your own IP addresses (BYOIP). |
| `DescribeCapacityBlockExtensionHistory` | - | `paginated` | - | - | `DescribeCapacityBlockExtensionHistoryResult` | - | Describes the events for the specified Capacity Block extension during the specified time. |
| `DescribeCapacityBlockExtensionOfferings` | - | `paginated` | `CapacityBlockExtensionDurationHours`, `CapacityReservationId` | - | `DescribeCapacityBlockExtensionOfferingsResult` | - | Describes Capacity Block extension offerings available for purchase in the Amazon Web Services Region that you're currently using. |
| `DescribeCapacityBlockOfferings` | - | `paginated` | `CapacityDurationHours` | - | `DescribeCapacityBlockOfferingsResult` | - | Describes Capacity Block offerings available for purchase in the Amazon Web Services Region that you're currently using. With Capacity Blocks, you can purchase a specific GPU instance type or EC2 UltraServer for a period of time. |
| `DescribeCapacityBlockStatus` | - | `paginated` | - | - | `DescribeCapacityBlockStatusResult` | - | Describes the availability of capacity for the specified Capacity blocks, or all of your Capacity Blocks. |
| `DescribeCapacityBlocks` | - | `paginated` | - | - | `DescribeCapacityBlocksResult` | - | Describes details about Capacity Blocks in the Amazon Web Services Region that you're currently using. |
| `DescribeCapacityManagerDataExports` | - | `paginated` | - | - | `DescribeCapacityManagerDataExportsResult` | - | Describes one or more Capacity Manager data export configurations. Returns information about export settings, delivery status, and recent export activity. |
| `DescribeCapacityReservationBillingRequests` | - | `paginated` | `Role` | - | `DescribeCapacityReservationBillingRequestsResult` | - | Describes a request to assign the billing of the unused capacity of a Capacity Reservation. For more information, see Billing assignment for shared Amazon EC2 Capacity Reservations. |
| `DescribeCapacityReservationFleets` | - | `paginated` | - | - | `DescribeCapacityReservationFleetsResult` | - | Describes one or more Capacity Reservation Fleets. |
| `DescribeCapacityReservationTopology` | - | - | - | - | `DescribeCapacityReservationTopologyResult` | - | Describes a tree-based hierarchy that represents the physical host placement of your pending or active Capacity Reservations within an Availability Zone or Local Zone. You can use this information to determine the relative proximity of your capacity within... |
| `DescribeCapacityReservations` | - | `paginated` | - | - | `DescribeCapacityReservationsResult` | - | Describes one or more of your Capacity Reservations. The results describe only the Capacity Reservations in the Amazon Web Services Region that you're currently using. |
| `DescribeCarrierGateways` | - | `paginated` | - | - | `DescribeCarrierGatewaysResult` | - | Describes one or more of your carrier gateways. |
| `DescribeClassicLinkInstances` | - | `paginated` | - | - | `DescribeClassicLinkInstancesResult` | - | This action is deprecated. Describes your linked EC2-Classic instances. |
| `DescribeClientVpnAuthorizationRules` | - | `paginated` | `ClientVpnEndpointId` | - | `DescribeClientVpnAuthorizationRulesResult` | - | Describes the authorization rules for a specified Client VPN endpoint. |
| `DescribeClientVpnConnections` | - | `paginated` | `ClientVpnEndpointId` | - | `DescribeClientVpnConnectionsResult` | - | Describes active client connections and connections that have been terminated within the last 60 minutes for the specified Client VPN endpoint. |
| `DescribeClientVpnEndpoints` | - | `paginated` | - | - | `DescribeClientVpnEndpointsResult` | - | Describes one or more Client VPN endpoints in the account. |
| `DescribeClientVpnRoutes` | - | `paginated` | `ClientVpnEndpointId` | - | `DescribeClientVpnRoutesResult` | - | Describes the routes for the specified Client VPN endpoint. |
| `DescribeClientVpnTargetNetworks` | - | `paginated` | `ClientVpnEndpointId` | - | `DescribeClientVpnTargetNetworksResult` | - | Describes the target networks associated with the specified Client VPN endpoint. |
| `DescribeCoipPools` | - | `paginated` | - | - | `DescribeCoipPoolsResult` | - | Describes the specified customer-owned address pools or all of your customer-owned address pools. |
| `DescribeConversionTasks` | - | - | - | - | `DescribeConversionTasksResult` | - | Describes the specified conversion tasks or all your conversion tasks. For more information, see the VM Import/Export User Guide. |
| `DescribeCustomerGateways` | - | - | - | - | `DescribeCustomerGatewaysResult` | - | Describes one or more of your VPN customer gateways. For more information, see Amazon Web Services Site-to-Site VPN in the Amazon Web Services Site-to-Site VPN User Guide . |
| `DescribeDeclarativePoliciesReports` | - | - | - | - | `DescribeDeclarativePoliciesReportsResult` | - | Describes the metadata of an account status report, including the status of the report. To view the full report, download it from the Amazon S3 bucket where it was saved. |
| `DescribeDhcpOptions` | - | `paginated` | - | - | `DescribeDhcpOptionsResult` | - | Describes your DHCP option sets. The default is to describe all your DHCP option sets. |
| `DescribeEgressOnlyInternetGateways` | - | `paginated` | - | - | `DescribeEgressOnlyInternetGatewaysResult` | - | Describes your egress-only internet gateways. The default is to describe all your egress-only internet gateways. |
| `DescribeElasticGpus` | - | - | - | - | `DescribeElasticGpusResult` | - | Amazon Elastic Graphics reached end of life on January 8, 2024. Describes the Elastic Graphics accelerator associated with your instances. |
| `DescribeExportImageTasks` | - | `paginated` | - | - | `DescribeExportImageTasksResult` | - | Describes the specified export image tasks or all of your export image tasks. |
| `DescribeExportTasks` | - | - | - | - | `DescribeExportTasksResult` | - | Describes the specified export instance tasks or all of your export instance tasks. |
| `DescribeFastLaunchImages` | - | `paginated` | - | - | `DescribeFastLaunchImagesResult` | - | Describe details for Windows AMIs that are configured for Windows fast launch. |
| `DescribeFastSnapshotRestores` | - | `paginated` | - | - | `DescribeFastSnapshotRestoresResult` | - | Describes the state of fast snapshot restores for your snapshots. |
| `DescribeFleetHistory` | - | - | `FleetId`, `StartTime` | - | `DescribeFleetHistoryResult` | - | Describes the events for the specified EC2 Fleet during the specified time. EC2 Fleet events are delayed by up to 30 seconds before they can be described. |
| `DescribeFleetInstances` | - | - | `FleetId` | - | `DescribeFleetInstancesResult` | - | Describes the running instances for the specified EC2 Fleet. Currently, `DescribeFleetInstances` does not support fleets of type `instant`. |
| `DescribeFleets` | - | `paginated` | - | - | `DescribeFleetsResult` | - | Describes the specified EC2 Fleet or all of your EC2 Fleets. If a fleet is of type `instant`, you must specify the fleet ID in the request, otherwise the fleet does not appear in the response. |
| `DescribeFlowLogs` | - | `paginated` | - | - | `DescribeFlowLogsResult` | - | Describes one or more flow logs. To view the published flow log records, you must view the log destination. |
| `DescribeFpgaImageAttribute` | - | - | `Attribute`, `FpgaImageId` | - | `DescribeFpgaImageAttributeResult` | - | Describes the specified attribute of the specified Amazon FPGA Image (AFI). |
| `DescribeFpgaImages` | - | `paginated` | - | - | `DescribeFpgaImagesResult` | - | Describes the Amazon FPGA Images (AFIs) available to you. These include public AFIs, private AFIs that you own, and AFIs owned by other Amazon Web Services accounts for which you have load permissions. |
| `DescribeHostReservationOfferings` | - | `paginated` | - | - | `DescribeHostReservationOfferingsResult` | - | Describes the Dedicated Host reservations that are available to purchase. The results describe all of the Dedicated Host reservation offerings, including offerings that might not match the instance family and Region of your Dedicated Hosts. |
| `DescribeHostReservations` | - | `paginated` | - | - | `DescribeHostReservationsResult` | - | Describes reservations that are associated with Dedicated Hosts in your account. |
| `DescribeHosts` | - | `paginated` | - | - | `DescribeHostsResult` | - | Describes the specified Dedicated Hosts or all your Dedicated Hosts. The results describe only the Dedicated Hosts in the Region you're currently using. |
| `DescribeIamInstanceProfileAssociations` | - | `paginated` | - | - | `DescribeIamInstanceProfileAssociationsResult` | - | Describes your IAM instance profile associations. |
| `DescribeIdFormat` | - | - | - | - | `DescribeIdFormatResult` | - | Describes the ID format settings for your resources on a per-Region basis, for example, to view which resource types are enabled for longer IDs. This request only returns information about resource types whose ID formats can be modified; it does not return... |
| `DescribeIdentityIdFormat` | - | - | `PrincipalArn` | - | `DescribeIdentityIdFormatResult` | - | Describes the ID format settings for resources for the specified IAM user, IAM role, or root user. For example, you can view the resource types that are enabled for longer IDs. |
| `DescribeImageAttribute` | - | - | `Attribute`, `ImageId` | - | `ImageAttribute` | - | Describes the specified attribute of the specified AMI. You can specify only one attribute at a time. |
| `DescribeImageReferences` | - | `paginated` | `ImageIds` | - | `DescribeImageReferencesResult` | - | Describes your Amazon Web Services resources that are referencing the specified images. For more information, see Identify your resources referencing specified AMIs in the Amazon EC2 User Guide . |
| `DescribeImageUsageReportEntries` | - | `paginated` | - | - | `DescribeImageUsageReportEntriesResult` | - | Describes the entries in image usage reports, showing how your images are used across other Amazon Web Services accounts. For more information, see View your AMI usage in the Amazon EC2 User Guide . |
| `DescribeImageUsageReports` | - | `paginated` | - | - | `DescribeImageUsageReportsResult` | - | Describes the configuration and status of image usage reports, filtered by report IDs or image IDs. For more information, see View your AMI usage in the Amazon EC2 User Guide . |
| `DescribeImages` | - | `paginated` | - | - | `DescribeImagesResult` | - | Describes the specified images (AMIs, AKIs, and ARIs) available to you or all of the images available to you. The images available to you include public images, private images that you own, and private images owned by other Amazon Web Services accounts for... |
| `DescribeImportImageTasks` | - | `paginated` | - | - | `DescribeImportImageTasksResult` | - | Displays details about an import virtual machine or import snapshot tasks that are already created. |
| `DescribeImportSnapshotTasks` | - | `paginated` | - | - | `DescribeImportSnapshotTasksResult` | - | Describes your import snapshot tasks. |
| `DescribeInstanceAttribute` | - | - | `Attribute`, `InstanceId` | - | `InstanceAttribute` | - | Describes the specified attribute of the specified instance. You can specify only one attribute at a time. |
| `DescribeInstanceConnectEndpoints` | - | `paginated` | - | - | `DescribeInstanceConnectEndpointsResult` | - | Describes the specified EC2 Instance Connect Endpoints or all EC2 Instance Connect Endpoints. |
| `DescribeInstanceCreditSpecifications` | - | `paginated` | - | - | `DescribeInstanceCreditSpecificationsResult` | - | Describes the credit option for CPU usage of the specified burstable performance instances. The credit options are `standard` and `unlimited`. |
| `DescribeInstanceEventNotificationAttributes` | - | - | - | - | `DescribeInstanceEventNotificationAttributesResult` | - | Describes the tag keys that are registered to appear in scheduled event notifications for resources in the current Region. |
| `DescribeInstanceEventWindows` | - | `paginated` | - | - | `DescribeInstanceEventWindowsResult` | - | Describes the specified event windows or all event windows. If you specify event window IDs, the output includes information for only the specified event windows. |
| `DescribeInstanceImageMetadata` | - | `paginated` | - | - | `DescribeInstanceImageMetadataResult` | - | Describes the AMI that was used to launch an instance, even if the AMI is deprecated, deregistered, made private (no longer public or shared with your account), or not allowed. If you specify instance IDs, the output includes information for only the... |
| `DescribeInstanceSqlHaHistoryStates` | - | - | - | - | `DescribeInstanceSqlHaHistoryStatesResult` | - | Describes the historical SQL Server High Availability states for Amazon EC2 instances that are enabled for Amazon EC2 High Availability for SQL Server monitoring. |
| `DescribeInstanceSqlHaStates` | - | - | - | - | `DescribeInstanceSqlHaStatesResult` | - | Describes the SQL Server High Availability states for Amazon EC2 instances that are enabled for Amazon EC2 High Availability for SQL Server monitoring. |
| `DescribeInstanceStatus` | - | `paginated` | - | - | `DescribeInstanceStatusResult` | - | Describes the status of the specified instances or all of your instances. By default, only running instances are described, unless you specifically indicate to return the status of all instances. |
| `DescribeInstanceTopology` | - | `paginated` | - | - | `DescribeInstanceTopologyResult` | - | Describes a tree-based hierarchy that represents the physical host placement of your EC2 instances within an Availability Zone or Local Zone. You can use this information to determine the relative proximity of your EC2 instances within the Amazon Web Services... |
| `DescribeInstanceTypeOfferings` | - | `paginated` | - | - | `DescribeInstanceTypeOfferingsResult` | - | Lists the instance types that are offered for the specified location. If no location is specified, the default is to list the instance types that are offered in the current Region. |
| `DescribeInstanceTypes` | - | `paginated` | - | - | `DescribeInstanceTypesResult` | - | Describes the specified instance types. By default, all instance types for the current Region are described. |
| `DescribeInstances` | - | `paginated` | - | - | `DescribeInstancesResult` | - | Describes the specified instances or all instances. If you specify instance IDs, the output includes information for only the specified instances. |
| `DescribeInternetGateways` | - | `paginated` | - | - | `DescribeInternetGatewaysResult` | - | Describes your internet gateways. The default is to describe all your internet gateways. |
| `DescribeIpamByoasn` | - | - | - | - | `DescribeIpamByoasnResult` | - | Describes your Autonomous System Numbers (ASNs), their provisioning statuses, and the BYOIP CIDRs with which they are associated. For more information, see Tutorial: Bring your ASN to IPAM in the Amazon VPC IPAM guide . |
| `DescribeIpamExternalResourceVerificationTokens` | - | - | - | - | `DescribeIpamExternalResourceVerificationTokensResult` | - | Describe verification tokens. A verification token is an Amazon Web Services-generated random value that you can use to prove ownership of an external resource. |
| `DescribeIpamPolicies` | - | - | - | - | `DescribeIpamPoliciesResult` | - | Describes one or more IPAM policies. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `DescribeIpamPools` | - | `paginated` | - | - | `DescribeIpamPoolsResult` | - | Get information about your IPAM pools. |
| `DescribeIpamPrefixListResolverTargets` | - | `paginated` | - | - | `DescribeIpamPrefixListResolverTargetsResult` | - | Describes one or more IPAM prefix list resolver Targets. Use this operation to view the configuration and status of resolver targets. |
| `DescribeIpamPrefixListResolvers` | - | `paginated` | - | - | `DescribeIpamPrefixListResolversResult` | - | Describes one or more IPAM prefix list resolvers. Use this operation to view the configuration, status, and properties of your resolvers. |
| `DescribeIpamResourceDiscoveries` | - | `paginated` | - | - | `DescribeIpamResourceDiscoveriesResult` | - | Describes IPAM resource discoveries. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account. |
| `DescribeIpamResourceDiscoveryAssociations` | - | `paginated` | - | - | `DescribeIpamResourceDiscoveryAssociationsResult` | - | Describes resource discovery association with an Amazon VPC IPAM. An associated resource discovery is a resource discovery that has been associated with an IPAM.. |
| `DescribeIpamScopes` | - | `paginated` | - | - | `DescribeIpamScopesResult` | - | Get information about your IPAM scopes. |
| `DescribeIpams` | - | `paginated` | - | - | `DescribeIpamsResult` | - | Get information about your IPAM pools. For more information, see What is IPAM? |
| `DescribeIpv6Pools` | - | `paginated` | - | - | `DescribeIpv6PoolsResult` | - | Describes your IPv6 address pools. |
| `DescribeKeyPairs` | - | - | - | - | `DescribeKeyPairsResult` | - | Describes the specified key pairs or all of your key pairs. For more information about key pairs, see Amazon EC2 key pairs in the Amazon EC2 User Guide . |
| `DescribeLaunchTemplateVersions` | - | `paginated` | - | - | `DescribeLaunchTemplateVersionsResult` | - | Describes one or more versions of a specified launch template. You can describe all versions, individual versions, or a range of versions. |
| `DescribeLaunchTemplates` | - | `paginated` | - | - | `DescribeLaunchTemplatesResult` | - | Describes one or more launch templates. |
| `DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociations` | - | `paginated` | - | - | `DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsResult` | - | Describes the associations between virtual interface groups and local gateway route tables. |
| `DescribeLocalGatewayRouteTableVpcAssociations` | - | `paginated` | - | - | `DescribeLocalGatewayRouteTableVpcAssociationsResult` | - | Describes the specified associations between VPCs and local gateway route tables. |
| `DescribeLocalGatewayRouteTables` | - | `paginated` | - | - | `DescribeLocalGatewayRouteTablesResult` | - | Describes one or more local gateway route tables. By default, all local gateway route tables are described. |
| `DescribeLocalGatewayVirtualInterfaceGroups` | - | `paginated` | - | - | `DescribeLocalGatewayVirtualInterfaceGroupsResult` | - | Describes the specified local gateway virtual interface groups. |
| `DescribeLocalGatewayVirtualInterfaces` | - | `paginated` | - | - | `DescribeLocalGatewayVirtualInterfacesResult` | - | Describes the specified local gateway virtual interfaces. |
| `DescribeLocalGateways` | - | `paginated` | - | - | `DescribeLocalGatewaysResult` | - | Describes one or more local gateways. By default, all local gateways are described. |
| `DescribeLockedSnapshots` | - | - | - | - | `DescribeLockedSnapshotsResult` | - | Describes the lock status for a snapshot. |
| `DescribeMacHosts` | - | `paginated` | - | - | `DescribeMacHostsResult` | - | Describes the specified EC2 Mac Dedicated Host or all of your EC2 Mac Dedicated Hosts. |
| `DescribeMacModificationTasks` | - | `paginated` | - | - | `DescribeMacModificationTasksResult` | - | Describes a System Integrity Protection (SIP) modification task or volume ownership delegation task for an Amazon EC2 Mac instance. For more information, see Configure SIP for Amazon EC2 instances in the Amazon EC2 User Guide . |
| `DescribeManagedPrefixLists` | - | `paginated` | - | - | `DescribeManagedPrefixListsResult` | - | Describes your managed prefix lists and any Amazon Web Services-managed prefix lists. |
| `DescribeMovingAddresses` | - | `paginated` | - | - | `DescribeMovingAddressesResult` | - | This action is deprecated. Describes your Elastic IP addresses that are being moved from or being restored to the EC2-Classic platform. |
| `DescribeNatGateways` | - | `paginated` | - | - | `DescribeNatGatewaysResult` | - | Describes your NAT gateways. The default is to describe all your NAT gateways. |
| `DescribeNetworkAcls` | - | `paginated` | - | - | `DescribeNetworkAclsResult` | - | Describes your network ACLs. The default is to describe all your network ACLs. |
| `DescribeNetworkInsightsAccessScopeAnalyses` | - | `paginated` | - | - | `DescribeNetworkInsightsAccessScopeAnalysesResult` | - | Describes the specified Network Access Scope analyses. |
| `DescribeNetworkInsightsAccessScopes` | - | `paginated` | - | - | `DescribeNetworkInsightsAccessScopesResult` | - | Describes the specified Network Access Scopes. |
| `DescribeNetworkInsightsAnalyses` | - | `paginated` | - | - | `DescribeNetworkInsightsAnalysesResult` | - | Describes one or more of your network insights analyses. |
| `DescribeNetworkInsightsPaths` | - | `paginated` | - | - | `DescribeNetworkInsightsPathsResult` | - | Describes one or more of your paths. |
| `DescribeNetworkInterfaceAttribute` | - | - | `NetworkInterfaceId` | - | `DescribeNetworkInterfaceAttributeResult` | - | Describes a network interface attribute. You can specify only one attribute at a time. |
| `DescribeNetworkInterfacePermissions` | - | `paginated` | - | - | `DescribeNetworkInterfacePermissionsResult` | - | Describes the permissions for your network interfaces. |
| `DescribeNetworkInterfaces` | - | `paginated` | - | - | `DescribeNetworkInterfacesResult` | - | Describes the specified network interfaces or all your network interfaces. If you have a large number of network interfaces, the operation fails unless you use pagination or one of the following filters: `group-id`, `mac-address`, `private-dns-name`... |
| `DescribeOutpostLags` | - | - | - | - | `DescribeOutpostLagsResult` | - | Describes the Outposts link aggregation groups (LAGs). LAGs are only available for second-generation Outposts racks at this time. |
| `DescribePlacementGroups` | - | - | - | - | `DescribePlacementGroupsResult` | - | Describes the specified placement groups or all of your placement groups. To describe a specific placement group that is shared with your account, you must specify the ID of the placement group using the `GroupId` parameter. |
| `DescribePrefixLists` | - | `paginated` | - | - | `DescribePrefixListsResult` | - | Describes available Amazon Web Services services in a prefix list format, which includes the prefix list name and prefix list ID of the service and the IP address range for the service. |
| `DescribePrincipalIdFormat` | - | `paginated` | - | - | `DescribePrincipalIdFormatResult` | - | Describes the ID format settings for the root user and all IAM roles and IAM users that have explicitly specified a longer ID (17-character ID) preference. By default, all IAM roles and IAM users default to the same ID settings as the root user, unless they... |
| `DescribePublicIpv4Pools` | - | `paginated` | - | - | `DescribePublicIpv4PoolsResult` | - | Describes the specified IPv4 address pools. |
| `DescribeRegions` | - | - | - | - | `DescribeRegionsResult` | - | Describes the Regions that are enabled for your account, or all Regions. For a list of the Regions supported by Amazon EC2, see Amazon EC2 service endpoints. |
| `DescribeReplaceRootVolumeTasks` | - | `paginated` | - | - | `DescribeReplaceRootVolumeTasksResult` | - | Describes a root volume replacement task. For more information, see Replace a root volume in the Amazon EC2 User Guide . |
| `DescribeReservedInstances` | - | - | - | - | `DescribeReservedInstancesResult` | - | Describes one or more of the Reserved Instances that you purchased. For more information about Reserved Instances, see Reserved Instances in the Amazon EC2 User Guide . |
| `DescribeReservedInstancesListings` | - | - | - | - | `DescribeReservedInstancesListingsResult` | - | Describes your account's Reserved Instance listings in the Reserved Instance Marketplace. The Reserved Instance Marketplace matches sellers who want to resell Reserved Instance capacity that they no longer need with buyers who want to purchase additional... |
| `DescribeReservedInstancesModifications` | - | `paginated` | - | - | `DescribeReservedInstancesModificationsResult` | - | Describes the modifications made to your Reserved Instances. If no parameter is specified, information about all your Reserved Instances modification requests is returned. |
| `DescribeReservedInstancesOfferings` | - | `paginated` | - | - | `DescribeReservedInstancesOfferingsResult` | - | Describes Reserved Instance offerings that are available for purchase. With Reserved Instances, you purchase the right to launch instances for a period of time. |
| `DescribeRouteServerEndpoints` | - | `paginated` | - | - | `DescribeRouteServerEndpointsResult` | - | Describes one or more route server endpoints. A route server endpoint is an Amazon Web Services-managed component inside a subnet that facilitates BGP (Border Gateway Protocol) connections between your route server and your BGP peers. |
| `DescribeRouteServerPeers` | - | `paginated` | - | - | `DescribeRouteServerPeersResult` | - | Describes one or more route server peers. A route server peer is a session between a route server endpoint and the device deployed in Amazon Web Services (such as a firewall appliance or other network security function running on an EC2 instance). |
| `DescribeRouteServers` | - | `paginated` | - | - | `DescribeRouteServersResult` | - | Describes one or more route servers. Amazon VPC Route Server simplifies routing for traffic between workloads that are deployed within a VPC and its internet gateways. |
| `DescribeRouteTables` | - | `paginated` | - | - | `DescribeRouteTablesResult` | - | Describes your route tables. The default is to describe all your route tables. |
| `DescribeScheduledInstanceAvailability` | - | `paginated` | `FirstSlotStartTimeRange`, `Recurrence` | - | `DescribeScheduledInstanceAvailabilityResult` | - | Finds available schedules that meet the specified criteria. You can search for an available schedule no more than 3 months in advance. |
| `DescribeScheduledInstances` | - | `paginated` | - | - | `DescribeScheduledInstancesResult` | - | Describes the specified Scheduled Instances or all your Scheduled Instances. |
| `DescribeSecondaryInterfaces` | - | `paginated` | - | - | `DescribeSecondaryInterfacesResult` | - | Describes one or more of your secondary interfaces. |
| `DescribeSecondaryNetworks` | - | `paginated` | - | - | `DescribeSecondaryNetworksResult` | - | Describes one or more secondary networks. |
| `DescribeSecondarySubnets` | - | `paginated` | - | - | `DescribeSecondarySubnetsResult` | - | Describes one or more of your secondary subnets. |
| `DescribeSecurityGroupReferences` | - | - | `GroupId` | - | `DescribeSecurityGroupReferencesResult` | - | Describes the VPCs on the other side of a VPC peering or Transit Gateway connection that are referencing the security groups you've specified in this request. |
| `DescribeSecurityGroupRules` | - | `paginated` | - | - | `DescribeSecurityGroupRulesResult` | - | Describes one or more of your security group rules. |
| `DescribeSecurityGroupVpcAssociations` | - | `paginated` | - | - | `DescribeSecurityGroupVpcAssociationsResult` | - | Describes security group VPC associations made with AssociateSecurityGroupVpc. |
| `DescribeSecurityGroups` | - | `paginated` | - | - | `DescribeSecurityGroupsResult` | - | Describes the specified security groups or all of your security groups. |
| `DescribeServiceLinkVirtualInterfaces` | - | - | - | - | `DescribeServiceLinkVirtualInterfacesResult` | - | Describes the Outpost service link virtual interfaces. |
| `DescribeSnapshotAttribute` | - | - | `Attribute`, `SnapshotId` | - | `DescribeSnapshotAttributeResult` | - | Describes the specified attribute of the specified snapshot. You can specify only one attribute at a time. |
| `DescribeSnapshotTierStatus` | - | `paginated` | - | - | `DescribeSnapshotTierStatusResult` | - | Describes the storage tier status of one or more Amazon EBS snapshots. |
| `DescribeSnapshots` | - | `paginated` | - | - | `DescribeSnapshotsResult` | - | Describes the specified EBS snapshots available to you or all of the EBS snapshots available to you. The snapshots available to you include public snapshots, private snapshots that you own, and private snapshots owned by other Amazon Web Services accounts for... |
| `DescribeSpotDatafeedSubscription` | - | - | - | - | `DescribeSpotDatafeedSubscriptionResult` | - | Describes the data feed for Spot Instances. For more information, see Spot Instance data feed in the Amazon EC2 User Guide . |
| `DescribeSpotFleetInstances` | - | - | `SpotFleetRequestId` | - | `DescribeSpotFleetInstancesResponse` | - | Describes the running instances for the specified Spot Fleet. |
| `DescribeSpotFleetRequestHistory` | - | - | `SpotFleetRequestId`, `StartTime` | - | `DescribeSpotFleetRequestHistoryResponse` | - | Describes the events for the specified Spot Fleet request during the specified time. Spot Fleet events are delayed by up to 30 seconds before they can be described. |
| `DescribeSpotFleetRequests` | - | `paginated` | - | - | `DescribeSpotFleetRequestsResponse` | - | Describes your Spot Fleet requests. Spot Fleet requests are deleted 48 hours after they are canceled and their instances are terminated. |
| `DescribeSpotInstanceRequests` | - | `paginated` | - | - | `DescribeSpotInstanceRequestsResult` | - | Describes the specified Spot Instance requests. You can use `DescribeSpotInstanceRequests` to find a running Spot Instance by examining the response. |
| `DescribeSpotPriceHistory` | - | `paginated` | - | - | `DescribeSpotPriceHistoryResult` | - | Describes the Spot price history. For more information, see Spot Instance pricing history in the Amazon EC2 User Guide . |
| `DescribeStaleSecurityGroups` | - | `paginated` | `VpcId` | - | `DescribeStaleSecurityGroupsResult` | - | Describes the stale security group rules for security groups referenced across a VPC peering connection, transit gateway connection, or with a security group VPC association. Rules are stale when they reference a deleted security group. |
| `DescribeStoreImageTasks` | - | `paginated` | - | - | `DescribeStoreImageTasksResult` | - | Describes the progress of the AMI store tasks. You can describe the store tasks for specified AMIs. |
| `DescribeSubnets` | - | `paginated` | - | - | `DescribeSubnetsResult` | - | Describes your subnets. The default is to describe all your subnets. |
| `DescribeTags` | - | `paginated` | - | - | `DescribeTagsResult` | - | Describes the specified tags for your EC2 resources. For more information about tags, see Tag your Amazon EC2 resources in the Amazon Elastic Compute Cloud User Guide . |
| `DescribeTrafficMirrorFilterRules` | - | - | - | - | `DescribeTrafficMirrorFilterRulesResult` | - | Describe traffic mirror filters that determine the traffic that is mirrored. |
| `DescribeTrafficMirrorFilters` | - | `paginated` | - | - | `DescribeTrafficMirrorFiltersResult` | - | Describes one or more Traffic Mirror filters. |
| `DescribeTrafficMirrorSessions` | - | `paginated` | - | - | `DescribeTrafficMirrorSessionsResult` | - | Describes one or more Traffic Mirror sessions. By default, all Traffic Mirror sessions are described. |
| `DescribeTrafficMirrorTargets` | - | `paginated` | - | - | `DescribeTrafficMirrorTargetsResult` | - | Information about one or more Traffic Mirror targets. |
| `DescribeTransitGatewayAttachments` | - | `paginated` | - | - | `DescribeTransitGatewayAttachmentsResult` | - | Describes one or more attachments between resources and transit gateways. By default, all attachments are described. |
| `DescribeTransitGatewayConnectPeers` | - | `paginated` | - | - | `DescribeTransitGatewayConnectPeersResult` | - | Describes one or more Connect peers. |
| `DescribeTransitGatewayConnects` | - | `paginated` | - | - | `DescribeTransitGatewayConnectsResult` | - | Describes one or more Connect attachments. |
| `DescribeTransitGatewayMeteringPolicies` | - | - | - | - | `DescribeTransitGatewayMeteringPoliciesResult` | - | Describes one or more transit gateway metering policies. |
| `DescribeTransitGatewayMulticastDomains` | - | `paginated` | - | - | `DescribeTransitGatewayMulticastDomainsResult` | - | Describes one or more transit gateway multicast domains. |
| `DescribeTransitGatewayPeeringAttachments` | - | `paginated` | - | - | `DescribeTransitGatewayPeeringAttachmentsResult` | - | Describes your transit gateway peering attachments. |
| `DescribeTransitGatewayPolicyTables` | - | `paginated` | - | - | `DescribeTransitGatewayPolicyTablesResult` | - | Describes one or more transit gateway route policy tables. |
| `DescribeTransitGatewayRouteTableAnnouncements` | - | `paginated` | - | - | `DescribeTransitGatewayRouteTableAnnouncementsResult` | - | Describes one or more transit gateway route table advertisements. |
| `DescribeTransitGatewayRouteTables` | - | `paginated` | - | - | `DescribeTransitGatewayRouteTablesResult` | - | Describes one or more transit gateway route tables. By default, all transit gateway route tables are described. |
| `DescribeTransitGatewayVpcAttachments` | - | `paginated` | - | - | `DescribeTransitGatewayVpcAttachmentsResult` | - | Describes one or more VPC attachments. By default, all VPC attachments are described. |
| `DescribeTransitGateways` | - | `paginated` | - | - | `DescribeTransitGatewaysResult` | - | Describes one or more transit gateways. By default, all transit gateways are described. |
| `DescribeTrunkInterfaceAssociations` | - | `paginated` | - | - | `DescribeTrunkInterfaceAssociationsResult` | - | Describes one or more network interface trunk associations. |
| `DescribeVerifiedAccessEndpoints` | - | `paginated` | - | - | `DescribeVerifiedAccessEndpointsResult` | - | Describes the specified Amazon Web Services Verified Access endpoints. |
| `DescribeVerifiedAccessGroups` | - | `paginated` | - | - | `DescribeVerifiedAccessGroupsResult` | - | Describes the specified Verified Access groups. |
| `DescribeVerifiedAccessInstanceLoggingConfigurations` | - | `paginated` | - | - | `DescribeVerifiedAccessInstanceLoggingConfigurationsResult` | - | Describes the specified Amazon Web Services Verified Access instances. |
| `DescribeVerifiedAccessInstances` | - | `paginated` | - | - | `DescribeVerifiedAccessInstancesResult` | - | Describes the specified Amazon Web Services Verified Access instances. |
| `DescribeVerifiedAccessTrustProviders` | - | `paginated` | - | - | `DescribeVerifiedAccessTrustProvidersResult` | - | Describes the specified Amazon Web Services Verified Access trust providers. |
| `DescribeVolumeAttribute` | - | - | `Attribute`, `VolumeId` | - | `DescribeVolumeAttributeResult` | - | Describes the specified attribute of the specified volume. You can specify only one attribute at a time. |
| `DescribeVolumeStatus` | - | `paginated` | - | - | `DescribeVolumeStatusResult` | - | Describes the status of the specified volumes. Volume status provides the result of the checks performed on your volumes to determine events that can impair the performance of your volumes. |
| `DescribeVolumes` | - | `paginated` | - | - | `DescribeVolumesResult` | - | Describes the specified EBS volumes or all of your EBS volumes. If you are describing a long list of volumes, we recommend that you paginate the output to make the list more manageable. |
| `DescribeVolumesModifications` | - | `paginated` | - | - | `DescribeVolumesModificationsResult` | - | Describes the most recent volume modification request for the specified EBS volumes. For more information, see Monitor the progress of volume modifications in the Amazon EBS User Guide . |
| `DescribeVpcAttribute` | - | - | `Attribute`, `VpcId` | - | `DescribeVpcAttributeResult` | - | Describes the specified attribute of the specified VPC. You can specify only one attribute at a time. |
| `DescribeVpcBlockPublicAccessExclusions` | - | - | - | - | `DescribeVpcBlockPublicAccessExclusionsResult` | - | Describe VPC Block Public Access (BPA) exclusions. A VPC BPA exclusion is a mode that can be applied to a single VPC or subnet that exempts it from the account’s BPA mode and will allow bidirectional or egress-only access. |
| `DescribeVpcBlockPublicAccessOptions` | - | - | - | - | `DescribeVpcBlockPublicAccessOptionsResult` | - | Describe VPC Block Public Access (BPA) options. VPC Block Public Access (BPA) enables you to block resources in VPCs and subnets that you own in a Region from reaching or being reached from the internet through internet gateways and egress-only internet... |
| `DescribeVpcClassicLink` | - | - | - | - | `DescribeVpcClassicLinkResult` | - | This action is deprecated. Describes the ClassicLink status of the specified VPCs. |
| `DescribeVpcClassicLinkDnsSupport` | - | `paginated` | - | - | `DescribeVpcClassicLinkDnsSupportResult` | - | This action is deprecated. Describes the ClassicLink DNS support status of one or more VPCs. |
| `DescribeVpcEncryptionControls` | - | - | - | - | `DescribeVpcEncryptionControlsResult` | - | Describes one or more VPC Encryption Control configurations. VPC Encryption Control enables you to enforce encryption for all data in transit within and between VPCs to meet compliance requirements You can filter the results to return information about... |
| `DescribeVpcEndpointAssociations` | - | - | - | - | `DescribeVpcEndpointAssociationsResult` | - | Describes the VPC resources, VPC endpoint services, Amazon Lattice services, or service networks associated with the VPC endpoint. |
| `DescribeVpcEndpointConnectionNotifications` | - | `paginated` | - | - | `DescribeVpcEndpointConnectionNotificationsResult` | - | Describes the connection notifications for VPC endpoints and VPC endpoint services. |
| `DescribeVpcEndpointConnections` | - | `paginated` | - | - | `DescribeVpcEndpointConnectionsResult` | - | Describes the VPC endpoint connections to your VPC endpoint services, including any endpoints that are pending your acceptance. |
| `DescribeVpcEndpointServiceConfigurations` | - | `paginated` | - | - | `DescribeVpcEndpointServiceConfigurationsResult` | - | Describes the VPC endpoint service configurations in your account (your services). |
| `DescribeVpcEndpointServicePermissions` | - | `paginated` | `ServiceId` | - | `DescribeVpcEndpointServicePermissionsResult` | - | Describes the principals (service consumers) that are permitted to discover your VPC endpoint service. Principal ARNs with path components aren't supported. |
| `DescribeVpcEndpointServices` | - | - | - | - | `DescribeVpcEndpointServicesResult` | - | Describes available services to which you can create a VPC endpoint. When the service provider and the consumer have different accounts in multiple Availability Zones, and the consumer views the VPC endpoint service information, the response only includes the... |
| `DescribeVpcEndpoints` | - | `paginated` | - | - | `DescribeVpcEndpointsResult` | - | Describes your VPC endpoints. The default is to describe all your VPC endpoints. |
| `DescribeVpcPeeringConnections` | - | `paginated` | - | - | `DescribeVpcPeeringConnectionsResult` | - | Describes your VPC peering connections. The default is to describe all your VPC peering connections. |
| `DescribeVpcs` | - | `paginated` | - | - | `DescribeVpcsResult` | - | Describes your VPCs. The default is to describe all your VPCs. |
| `DescribeVpnConcentrators` | - | `paginated` | - | - | `DescribeVpnConcentratorsResult` | - | Describes one or more of your VPN concentrators. |
| `DescribeVpnConnections` | - | - | - | - | `DescribeVpnConnectionsResult` | - | Describes one or more of your VPN connections. For more information, see Amazon Web Services Site-to-Site VPN in the Amazon Web Services Site-to-Site VPN User Guide . |
| `DescribeVpnGateways` | - | - | - | - | `DescribeVpnGatewaysResult` | - | Describes one or more of your virtual private gateways. For more information, see Amazon Web Services Site-to-Site VPN in the Amazon Web Services Site-to-Site VPN User Guide . |
| `DetachClassicLinkVpc` | - | - | `InstanceId`, `VpcId` | - | `DetachClassicLinkVpcResult` | - | This action is deprecated. Unlinks (detaches) a linked EC2-Classic instance from a VPC. |
| `DetachInternetGateway` | - | - | `InternetGatewayId`, `VpcId` | - | `Unit` | - | Detaches an internet gateway from a VPC, disabling connectivity between the internet and the VPC. The VPC must not contain any running instances with Elastic IP addresses or public IPv4 addresses. |
| `DetachNetworkInterface` | - | - | `AttachmentId` | - | `Unit` | - | Detaches a network interface from an instance. |
| `DetachVerifiedAccessTrustProvider` | - | `idempotency-token` | `VerifiedAccessInstanceId`, `VerifiedAccessTrustProviderId` | `ClientToken` | `DetachVerifiedAccessTrustProviderResult` | - | Detaches the specified Amazon Web Services Verified Access trust provider from the specified Amazon Web Services Verified Access instance. |
| `DetachVolume` | - | - | `VolumeId` | - | `VolumeAttachment` | - | Detaches an EBS volume from an instance. Make sure to unmount any file systems on the device within your operating system before detaching the volume. |
| `DetachVpnGateway` | - | - | `VpcId`, `VpnGatewayId` | - | `Unit` | - | Detaches a virtual private gateway from a VPC. You do this if you're planning to turn off the VPC and not use it anymore. |
| `DisableAddressTransfer` | - | - | `AllocationId` | - | `DisableAddressTransferResult` | - | Disables Elastic IP address transfer. For more information, see Transfer Elastic IP addresses in the Amazon VPC User Guide . |
| `DisableAllowedImagesSettings` | - | - | - | - | `DisableAllowedImagesSettingsResult` | - | Disables Allowed AMIs for your account in the specified Amazon Web Services Region. When set to `disabled`, the image criteria in your Allowed AMIs settings do not apply, and no restrictions are placed on AMI discoverability or usage. |
| `DisableAwsNetworkPerformanceMetricSubscription` | - | - | - | - | `DisableAwsNetworkPerformanceMetricSubscriptionResult` | - | Disables Infrastructure Performance metric subscriptions. |
| `DisableCapacityManager` | - | `idempotency-token` | - | `ClientToken` | `DisableCapacityManagerResult` | - | Disables EC2 Capacity Manager for your account. This stops data ingestion and removes access to capacity analytics and optimization recommendations. |
| `DisableEbsEncryptionByDefault` | - | - | - | - | `DisableEbsEncryptionByDefaultResult` | - | Disables EBS encryption by default for your account in the current Region. After you disable encryption by default, you can still create encrypted volumes by enabling encryption when you create each volume. |
| `DisableFastLaunch` | - | - | `ImageId` | - | `DisableFastLaunchResult` | - | Discontinue Windows fast launch for a Windows AMI, and clean up existing pre-provisioned snapshots. After you disable Windows fast launch, the AMI uses the standard launch process for each new instance. |
| `DisableFastSnapshotRestores` | - | - | `SourceSnapshotIds` | - | `DisableFastSnapshotRestoresResult` | - | Disables fast snapshot restores for the specified snapshots in the specified Availability Zones. |
| `DisableImage` | - | - | `ImageId` | - | `DisableImageResult` | - | Sets the AMI state to `disabled` and removes all launch permissions from the AMI. A disabled AMI can't be used for instance launches. |
| `DisableImageBlockPublicAccess` | - | - | - | - | `DisableImageBlockPublicAccessResult` | - | Disables block public access for AMIs at the account level in the specified Amazon Web Services Region. This removes the block public access restriction from your account. |
| `DisableImageDeprecation` | - | - | `ImageId` | - | `DisableImageDeprecationResult` | - | Cancels the deprecation of the specified AMI. For more information, see Deprecate an Amazon EC2 AMI in the Amazon EC2 User Guide . |
| `DisableImageDeregistrationProtection` | - | - | `ImageId` | - | `DisableImageDeregistrationProtectionResult` | - | Disables deregistration protection for an AMI. When deregistration protection is disabled, the AMI can be deregistered. |
| `DisableInstanceSqlHaStandbyDetections` | - | - | `InstanceIds` | - | `DisableInstanceSqlHaStandbyDetectionsResult` | - | Disable Amazon EC2 instances running in an SQL Server High Availability cluster from SQL Server High Availability instance standby detection monitoring. Once disabled, Amazon Web Services no longer monitors the metadata for the instances to determine whether... |
| `DisableIpamOrganizationAdminAccount` | - | - | `DelegatedAdminAccountId` | - | `DisableIpamOrganizationAdminAccountResult` | - | Disable the IPAM account. For more information, see Enable integration with Organizations in the Amazon VPC IPAM User Guide . |
| `DisableIpamPolicy` | - | - | `IpamPolicyId` | - | `DisableIpamPolicyResult` | - | Disables an IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `DisableRouteServerPropagation` | - | - | `RouteServerId`, `RouteTableId` | - | `DisableRouteServerPropagationResult` | - | Disables route propagation from a route server to a specified route table. When enabled, route server propagation installs the routes in the FIB on the route table you've specified. |
| `DisableSerialConsoleAccess` | - | - | - | - | `DisableSerialConsoleAccessResult` | - | Disables access to the EC2 serial console of all instances for your account. By default, access to the EC2 serial console is disabled for your account. |
| `DisableSnapshotBlockPublicAccess` | - | - | - | - | `DisableSnapshotBlockPublicAccessResult` | - | Disables the block public access for snapshots setting at the account level for the specified Amazon Web Services Region. After you disable block public access for snapshots in a Region, users can publicly share snapshots in that Region. |
| `DisableTransitGatewayRouteTablePropagation` | - | - | `TransitGatewayRouteTableId` | - | `DisableTransitGatewayRouteTablePropagationResult` | - | Disables the specified resource attachment from propagating routes to the specified propagation route table. |
| `DisableVgwRoutePropagation` | - | - | `GatewayId`, `RouteTableId` | - | `Unit` | - | Disables a virtual private gateway (VGW) from propagating routes to a specified route table of a VPC. |
| `DisableVpcClassicLink` | - | - | `VpcId` | - | `DisableVpcClassicLinkResult` | - | This action is deprecated. Disables ClassicLink for a VPC. |
| `DisableVpcClassicLinkDnsSupport` | - | - | - | - | `DisableVpcClassicLinkDnsSupportResult` | - | This action is deprecated. Disables ClassicLink DNS support for a VPC. |
| `DisassociateAddress` | - | - | - | - | `Unit` | - | Disassociates an Elastic IP address from the instance or network interface it's associated with. This is an idempotent operation. |
| `DisassociateCapacityReservationBillingOwner` | - | - | `CapacityReservationId`, `UnusedReservationBillingOwnerId` | - | `DisassociateCapacityReservationBillingOwnerResult` | - | Cancels a pending request to assign billing of the unused capacity of a Capacity Reservation to a consumer account, or revokes a request that has already been accepted. For more information, see Billing assignment for shared Amazon EC2 Capacity Reservations. |
| `DisassociateClientVpnTargetNetwork` | - | - | `AssociationId`, `ClientVpnEndpointId` | - | `DisassociateClientVpnTargetNetworkResult` | - | Disassociates a target network from the specified Client VPN endpoint. When you disassociate the last target network from a Client VPN, the following happens: The route that was automatically added for the VPC is deleted All active client connections are... |
| `DisassociateEnclaveCertificateIamRole` | - | - | `CertificateArn`, `RoleArn` | - | `DisassociateEnclaveCertificateIamRoleResult` | - | Disassociates an IAM role from an Certificate Manager (ACM) certificate. Disassociating an IAM role from an ACM certificate removes the Amazon S3 object that contains the certificate, certificate chain, and encrypted private key from the Amazon S3 bucket. |
| `DisassociateIamInstanceProfile` | - | - | `AssociationId` | - | `DisassociateIamInstanceProfileResult` | - | Disassociates an IAM instance profile from a running or stopped instance. Use DescribeIamInstanceProfileAssociations to get the association ID. |
| `DisassociateInstanceEventWindow` | - | - | `AssociationTarget`, `InstanceEventWindowId` | - | `DisassociateInstanceEventWindowResult` | - | Disassociates one or more targets from an event window. For more information, see Define event windows for scheduled events in the Amazon EC2 User Guide . |
| `DisassociateIpamByoasn` | - | - | `Asn`, `Cidr` | - | `DisassociateIpamByoasnResult` | - | Remove the association between your Autonomous System Number (ASN) and your BYOIP CIDR. You may want to use this action to disassociate an ASN from a CIDR or if you want to swap ASNs. |
| `DisassociateIpamResourceDiscovery` | - | - | `IpamResourceDiscoveryAssociationId` | - | `DisassociateIpamResourceDiscoveryResult` | - | Disassociates a resource discovery from an Amazon VPC IPAM. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account. |
| `DisassociateNatGatewayAddress` | - | - | `AssociationIds`, `NatGatewayId` | - | `DisassociateNatGatewayAddressResult` | - | Disassociates secondary Elastic IP addresses (EIPs) from a public NAT gateway. You cannot disassociate your primary EIP. |
| `DisassociateRouteServer` | - | - | `RouteServerId`, `VpcId` | - | `DisassociateRouteServerResult` | - | Disassociates a route server from a VPC. A route server association is the connection established between a route server and a VPC. |
| `DisassociateRouteTable` | - | - | `AssociationId` | - | `Unit` | - | Disassociates a subnet or gateway from a route table. After you perform this action, the subnet no longer uses the routes in the route table. |
| `DisassociateSecurityGroupVpc` | - | - | `GroupId`, `VpcId` | - | `DisassociateSecurityGroupVpcResult` | - | Disassociates a security group from a VPC. You cannot disassociate the security group if any Elastic network interfaces in the associated VPC are still associated with the security group. |
| `DisassociateSubnetCidrBlock` | - | - | `AssociationId` | - | `DisassociateSubnetCidrBlockResult` | - | Disassociates a CIDR block from a subnet. Currently, you can disassociate an IPv6 CIDR block only. |
| `DisassociateTransitGatewayMulticastDomain` | - | - | `SubnetIds`, `TransitGatewayAttachmentId`, `TransitGatewayMulticastDomainId` | - | `DisassociateTransitGatewayMulticastDomainResult` | - | Disassociates the specified subnets from the transit gateway multicast domain. |
| `DisassociateTransitGatewayPolicyTable` | - | - | `TransitGatewayAttachmentId`, `TransitGatewayPolicyTableId` | - | `DisassociateTransitGatewayPolicyTableResult` | - | Removes the association between an an attachment and a policy table. |
| `DisassociateTransitGatewayRouteTable` | - | - | `TransitGatewayAttachmentId`, `TransitGatewayRouteTableId` | - | `DisassociateTransitGatewayRouteTableResult` | - | Disassociates a resource attachment from a transit gateway route table. |
| `DisassociateTrunkInterface` | - | `idempotency-token` | `AssociationId` | `ClientToken` | `DisassociateTrunkInterfaceResult` | - | Removes an association between a branch network interface with a trunk network interface. |
| `DisassociateVpcCidrBlock` | - | - | `AssociationId` | - | `DisassociateVpcCidrBlockResult` | - | Disassociates a CIDR block from a VPC. To disassociate the CIDR block, you must specify its association ID. |
| `EnableAddressTransfer` | - | - | `AllocationId`, `TransferAccountId` | - | `EnableAddressTransferResult` | - | Enables Elastic IP address transfer. For more information, see Transfer Elastic IP addresses in the Amazon VPC User Guide . |
| `EnableAllowedImagesSettings` | - | - | `AllowedImagesSettingsState` | - | `EnableAllowedImagesSettingsResult` | - | Enables Allowed AMIs for your account in the specified Amazon Web Services Region. Two values are accepted: `enabled`: The image criteria in your Allowed AMIs settings are applied. |
| `EnableAwsNetworkPerformanceMetricSubscription` | - | - | - | - | `EnableAwsNetworkPerformanceMetricSubscriptionResult` | - | Enables Infrastructure Performance subscriptions. |
| `EnableCapacityManager` | - | `idempotency-token` | - | `ClientToken` | `EnableCapacityManagerResult` | - | Enables EC2 Capacity Manager for your account. This starts data ingestion for your EC2 capacity usage across On-Demand, Spot, and Capacity Reservations. |
| `EnableEbsEncryptionByDefault` | - | - | - | - | `EnableEbsEncryptionByDefaultResult` | - | Enables EBS encryption by default for your account in the current Region. After you enable encryption by default, the EBS volumes that you create are always encrypted, either using the default KMS key or the KMS key that you specified when you created each... |
| `EnableFastLaunch` | - | - | `ImageId` | - | `EnableFastLaunchResult` | - | When you enable Windows fast launch for a Windows AMI, images are pre-provisioned, using snapshots to launch instances up to 65% faster. To create the optimized Windows image, Amazon EC2 launches an instance and runs through Sysprep steps, rebooting as... |
| `EnableFastSnapshotRestores` | - | - | `SourceSnapshotIds` | - | `EnableFastSnapshotRestoresResult` | - | Enables fast snapshot restores for the specified snapshots in the specified Availability Zones. You get the full benefit of fast snapshot restores after they enter the `enabled` state. |
| `EnableImage` | - | - | `ImageId` | - | `EnableImageResult` | - | Re-enables a disabled AMI. The re-enabled AMI is marked as `available` and can be used for instance launches, appears in describe operations, and can be shared. |
| `EnableImageBlockPublicAccess` | - | - | `ImageBlockPublicAccessState` | - | `EnableImageBlockPublicAccessResult` | - | Enables block public access for AMIs at the account level in the specified Amazon Web Services Region. This prevents the public sharing of your AMIs. |
| `EnableImageDeprecation` | - | - | `DeprecateAt`, `ImageId` | - | `EnableImageDeprecationResult` | - | Enables deprecation of the specified AMI at the specified date and time. For more information, see Deprecate an AMI in the Amazon EC2 User Guide . |
| `EnableImageDeregistrationProtection` | - | - | `ImageId` | - | `EnableImageDeregistrationProtectionResult` | - | Enables deregistration protection for an AMI. When deregistration protection is enabled, the AMI can't be deregistered. |
| `EnableInstanceSqlHaStandbyDetections` | - | - | `InstanceIds` | - | `EnableInstanceSqlHaStandbyDetectionsResult` | - | Enable Amazon EC2 instances running in an SQL Server High Availability cluster for SQL Server High Availability instance standby detection monitoring. Once enabled, Amazon Web Services monitors the metadata for the instances to determine whether they are... |
| `EnableIpamOrganizationAdminAccount` | - | - | `DelegatedAdminAccountId` | - | `EnableIpamOrganizationAdminAccountResult` | - | Enable an Organizations member account as the IPAM admin account. You cannot select the Organizations management account as the IPAM admin account. |
| `EnableIpamPolicy` | - | - | `IpamPolicyId` | - | `EnableIpamPolicyResult` | - | Enables an IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `EnableReachabilityAnalyzerOrganizationSharing` | - | - | - | - | `EnableReachabilityAnalyzerOrganizationSharingResult` | - | Establishes a trust relationship between Reachability Analyzer and Organizations. This operation must be performed by the management account for the organization. |
| `EnableRouteServerPropagation` | - | - | `RouteServerId`, `RouteTableId` | - | `EnableRouteServerPropagationResult` | - | Defines which route tables the route server can update with routes. When enabled, route server propagation installs the routes in the FIB on the route table you've specified. |
| `EnableSerialConsoleAccess` | - | - | - | - | `EnableSerialConsoleAccessResult` | - | Enables access to the EC2 serial console of all instances for your account. By default, access to the EC2 serial console is disabled for your account. |
| `EnableSnapshotBlockPublicAccess` | - | - | `State` | - | `EnableSnapshotBlockPublicAccessResult` | - | Enables or modifies the block public access for snapshots setting at the account level for the specified Amazon Web Services Region. After you enable block public access for snapshots in a Region, users can no longer request public sharing for snapshots in... |
| `EnableTransitGatewayRouteTablePropagation` | - | - | `TransitGatewayRouteTableId` | - | `EnableTransitGatewayRouteTablePropagationResult` | - | Enables the specified attachment to propagate routes to the specified propagation route table. |
| `EnableVgwRoutePropagation` | - | - | `GatewayId`, `RouteTableId` | - | `Unit` | - | Enables a virtual private gateway (VGW) to propagate routes to the specified route table of a VPC. |
| `EnableVolumeIO` | - | - | `VolumeId` | - | `Unit` | - | Enables I/O operations for a volume that had I/O operations disabled because the data on the volume was potentially inconsistent. |
| `EnableVpcClassicLink` | - | - | `VpcId` | - | `EnableVpcClassicLinkResult` | - | This action is deprecated. Enables a VPC for ClassicLink. |
| `EnableVpcClassicLinkDnsSupport` | - | - | - | - | `EnableVpcClassicLinkDnsSupportResult` | - | This action is deprecated. Enables a VPC to support DNS hostname resolution for ClassicLink. |
| `ExportClientVpnClientCertificateRevocationList` | - | - | `ClientVpnEndpointId` | - | `ExportClientVpnClientCertificateRevocationListResult` | - | Downloads the client certificate revocation list for the specified Client VPN endpoint. |
| `ExportClientVpnClientConfiguration` | - | - | `ClientVpnEndpointId` | - | `ExportClientVpnClientConfigurationResult` | - | Downloads the contents of the Client VPN endpoint configuration file for the specified Client VPN endpoint. The Client VPN endpoint configuration file includes the Client VPN endpoint and certificate information clients need to establish a connection with the... |
| `ExportImage` | - | `idempotency-token` | `DiskImageFormat`, `ImageId`, `S3ExportLocation` | `ClientToken` | `ExportImageResult` | - | Exports an Amazon Machine Image (AMI) to a VM file. For more information, see Exporting a VM directly from an Amazon Machine Image (AMI) in the VM Import/Export User Guide . |
| `ExportTransitGatewayRoutes` | - | - | `S3Bucket`, `TransitGatewayRouteTableId` | - | `ExportTransitGatewayRoutesResult` | - | Exports routes from the specified transit gateway route table to the specified S3 bucket. By default, all routes are exported. |
| `ExportVerifiedAccessInstanceClientConfiguration` | - | - | `VerifiedAccessInstanceId` | - | `ExportVerifiedAccessInstanceClientConfigurationResult` | - | Exports the client configuration for a Verified Access instance. |
| `GetActiveVpnTunnelStatus` | - | - | `VpnConnectionId`, `VpnTunnelOutsideIpAddress` | - | `GetActiveVpnTunnelStatusResult` | - | Returns the currently negotiated security parameters for an active VPN tunnel, including IKE version, DH groups, encryption algorithms, and integrity algorithms. |
| `GetAllowedImagesSettings` | - | - | - | - | `GetAllowedImagesSettingsResult` | - | Gets the current state of the Allowed AMIs setting and the list of Allowed AMIs criteria at the account level in the specified Region. The Allowed AMIs feature does not restrict the AMIs owned by your account. |
| `GetAssociatedEnclaveCertificateIamRoles` | - | - | `CertificateArn` | - | `GetAssociatedEnclaveCertificateIamRolesResult` | - | Returns the IAM roles that are associated with the specified ACM (ACM) certificate. It also returns the name of the Amazon S3 bucket and the Amazon S3 object key where the certificate, certificate chain, and encrypted private key bundle are stored, and the... |
| `GetAssociatedIpv6PoolCidrs` | - | `paginated` | `PoolId` | - | `GetAssociatedIpv6PoolCidrsResult` | - | Gets information about the IPv6 CIDR block associations for a specified IPv6 address pool. |
| `GetAwsNetworkPerformanceData` | - | `paginated` | - | - | `GetAwsNetworkPerformanceDataResult` | - | Gets network performance data. |
| `GetCapacityManagerAttributes` | - | - | - | - | `GetCapacityManagerAttributesResult` | - | Retrieves the current configuration and status of EC2 Capacity Manager for your account, including enablement status, Organizations access settings, and data ingestion status. |
| `GetCapacityManagerMetricData` | - | `paginated` | `EndTime`, `MetricNames`, `Period`, `StartTime` | - | `GetCapacityManagerMetricDataResult` | - | Retrieves capacity usage metrics for your EC2 resources. Returns time-series data for metrics like unused capacity, utilization rates, and costs across On-Demand, Spot, and Capacity Reservations. |
| `GetCapacityManagerMetricDimensions` | - | `paginated` | `EndTime`, `GroupBy`, `MetricNames`, `StartTime` | - | `GetCapacityManagerMetricDimensionsResult` | - | Retrieves the available dimension values for capacity metrics within a specified time range. This is useful for discovering what accounts, regions, instance families, and other dimensions have data available for filtering and grouping. |
| `GetCapacityReservationUsage` | - | - | `CapacityReservationId` | - | `GetCapacityReservationUsageResult` | - | Gets usage information about a Capacity Reservation. If the Capacity Reservation is shared, it shows usage information for the Capacity Reservation owner and each Amazon Web Services account that is currently using the shared capacity. |
| `GetCoipPoolUsage` | - | - | `PoolId` | - | `GetCoipPoolUsageResult` | - | Describes the allocations from the specified customer-owned address pool. |
| `GetConsoleOutput` | - | - | `InstanceId` | - | `GetConsoleOutputResult` | - | Gets the console output for the specified instance. For Linux instances, the instance console output displays the exact console output that would normally be displayed on a physical monitor attached to a computer. |
| `GetConsoleScreenshot` | - | - | `InstanceId` | - | `GetConsoleScreenshotResult` | - | Retrieve a JPG-format screenshot of a running instance to help with troubleshooting. The returned content is Base64-encoded. |
| `GetDeclarativePoliciesReportSummary` | - | - | `ReportId` | - | `GetDeclarativePoliciesReportSummaryResult` | - | Retrieves a summary of the account status report. To view the full report, download it from the Amazon S3 bucket where it was saved. |
| `GetDefaultCreditSpecification` | - | - | `InstanceFamily` | - | `GetDefaultCreditSpecificationResult` | - | Describes the default credit option for CPU usage of a burstable performance instance family. For more information, see Burstable performance instances in the Amazon EC2 User Guide . |
| `GetEbsDefaultKmsKeyId` | - | - | - | - | `GetEbsDefaultKmsKeyIdResult` | - | Describes the default KMS key for EBS encryption by default for your account in this Region. For more information, see Amazon EBS encryption in the Amazon EBS User Guide . |
| `GetEbsEncryptionByDefault` | - | - | - | - | `GetEbsEncryptionByDefaultResult` | - | Describes whether EBS encryption by default is enabled for your account in the current Region. For more information, see Amazon EBS encryption in the Amazon EBS User Guide . |
| `GetEnabledIpamPolicy` | - | - | - | - | `GetEnabledIpamPolicyResult` | - | Gets the enabled IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `GetFlowLogsIntegrationTemplate` | - | - | `ConfigDeliveryS3DestinationArn`, `FlowLogId`, `IntegrateServices` | - | `GetFlowLogsIntegrationTemplateResult` | - | Generates a CloudFormation template that streamlines and automates the integration of VPC flow logs with Amazon Athena. This make it easier for you to query and gain insights from VPC flow logs data. |
| `GetGroupsForCapacityReservation` | - | `paginated` | `CapacityReservationId` | - | `GetGroupsForCapacityReservationResult` | - | Lists the resource groups to which a Capacity Reservation has been added. |
| `GetHostReservationPurchasePreview` | - | - | `HostIdSet`, `OfferingId` | - | `GetHostReservationPurchasePreviewResult` | - | Preview a reservation purchase with configurations that match those of your Dedicated Host. You must have active Dedicated Hosts in your account before you purchase a reservation. |
| `GetImageAncestry` | - | - | `ImageId` | - | `GetImageAncestryResult` | - | Retrieves the ancestry chain of the specified AMI, tracing its lineage back to the root AMI. For more information, see AMI ancestry in Amazon EC2 User Guide . |
| `GetImageBlockPublicAccessState` | - | - | - | - | `GetImageBlockPublicAccessStateResult` | - | Gets the current state of block public access for AMIs at the account level in the specified Amazon Web Services Region. For more information, see Block public access to your AMIs in the Amazon EC2 User Guide . |
| `GetInstanceMetadataDefaults` | - | - | - | - | `GetInstanceMetadataDefaultsResult` | - | Gets the default instance metadata service (IMDS) settings that are set at the account level in the specified Amazon Web Services Region. For more information, see Order of precedence for instance metadata options in the Amazon EC2 User Guide . |
| `GetInstanceTpmEkPub` | - | - | `InstanceId`, `KeyFormat`, `KeyType` | - | `GetInstanceTpmEkPubResult` | - | Gets the public endorsement key associated with the Nitro Trusted Platform Module (NitroTPM) for the specified instance. |
| `GetInstanceTypesFromInstanceRequirements` | - | `paginated` | `ArchitectureTypes`, `InstanceRequirements`, `VirtualizationTypes` | - | `GetInstanceTypesFromInstanceRequirementsResult` | - | Returns a list of instance types with the specified instance attributes. You can use the response to preview the instance types without launching instances. |
| `GetInstanceUefiData` | - | - | `InstanceId` | - | `GetInstanceUefiDataResult` | - | A binary representation of the UEFI variable store. Only non-volatile variables are stored. |
| `GetIpamAddressHistory` | - | `paginated` | `Cidr`, `IpamScopeId` | - | `GetIpamAddressHistoryResult` | - | Retrieve historical information about a CIDR within an IPAM scope. For more information, see View the history of IP addresses in the Amazon VPC IPAM User Guide . |
| `GetIpamDiscoveredAccounts` | - | `paginated` | `DiscoveryRegion`, `IpamResourceDiscoveryId` | - | `GetIpamDiscoveredAccountsResult` | - | Gets IPAM discovered accounts. A discovered account is an Amazon Web Services account that is monitored under a resource discovery. |
| `GetIpamDiscoveredPublicAddresses` | - | - | `AddressRegion`, `IpamResourceDiscoveryId` | - | `GetIpamDiscoveredPublicAddressesResult` | - | Gets the public IP addresses that have been discovered by IPAM. |
| `GetIpamDiscoveredResourceCidrs` | - | `paginated` | `IpamResourceDiscoveryId`, `ResourceRegion` | - | `GetIpamDiscoveredResourceCidrsResult` | - | Returns the resource CIDRs that are monitored as part of a resource discovery. A discovered resource is a resource CIDR monitored under a resource discovery. |
| `GetIpamPolicyAllocationRules` | - | - | `IpamPolicyId` | - | `GetIpamPolicyAllocationRulesResult` | - | Gets the allocation rules for an IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `GetIpamPolicyOrganizationTargets` | - | - | `IpamPolicyId` | - | `GetIpamPolicyOrganizationTargetsResult` | - | Gets the Amazon Web Services Organizations targets for an IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `GetIpamPoolAllocations` | - | `paginated` | `IpamPoolId` | - | `GetIpamPoolAllocationsResult` | - | Get a list of all the CIDR allocations in an IPAM pool. The Region you use should be the IPAM pool locale. |
| `GetIpamPoolCidrs` | - | `paginated` | `IpamPoolId` | - | `GetIpamPoolCidrsResult` | - | Get the CIDRs provisioned to an IPAM pool. |
| `GetIpamPrefixListResolverRules` | - | `paginated` | `IpamPrefixListResolverId` | - | `GetIpamPrefixListResolverRulesResult` | - | Retrieves the CIDR selection rules for an IPAM prefix list resolver. Use this operation to view the business logic that determines which CIDRs are selected for synchronization with prefix lists. |
| `GetIpamPrefixListResolverVersionEntries` | - | `paginated` | `IpamPrefixListResolverId`, `IpamPrefixListResolverVersion` | - | `GetIpamPrefixListResolverVersionEntriesResult` | - | Retrieves the CIDR entries for a specific version of an IPAM prefix list resolver. This shows the actual CIDRs that were selected and synchronized at a particular point in time. |
| `GetIpamPrefixListResolverVersions` | - | `paginated` | `IpamPrefixListResolverId` | - | `GetIpamPrefixListResolverVersionsResult` | - | Retrieves version information for an IPAM prefix list resolver. Each version is a snapshot of what CIDRs matched your rules at that moment in time. |
| `GetIpamResourceCidrs` | - | `paginated` | `IpamScopeId` | - | `GetIpamResourceCidrsResult` | - | Returns resource CIDRs managed by IPAM in a given scope. If an IPAM is associated with more than one resource discovery, the resource CIDRs across all of the resource discoveries is returned. |
| `GetLaunchTemplateData` | - | - | `InstanceId` | - | `GetLaunchTemplateDataResult` | - | Retrieves the configuration data of the specified instance. You can use this data to create a launch template. |
| `GetManagedPrefixListAssociations` | - | `paginated` | `PrefixListId` | - | `GetManagedPrefixListAssociationsResult` | - | Gets information about the resources that are associated with the specified managed prefix list. |
| `GetManagedPrefixListEntries` | - | `paginated` | `PrefixListId` | - | `GetManagedPrefixListEntriesResult` | - | Gets information about the entries for a specified managed prefix list. |
| `GetNetworkInsightsAccessScopeAnalysisFindings` | - | `paginated` | `NetworkInsightsAccessScopeAnalysisId` | - | `GetNetworkInsightsAccessScopeAnalysisFindingsResult` | - | Gets the findings for the specified Network Access Scope analysis. |
| `GetNetworkInsightsAccessScopeContent` | - | - | `NetworkInsightsAccessScopeId` | - | `GetNetworkInsightsAccessScopeContentResult` | - | Gets the content for the specified Network Access Scope. |
| `GetPasswordData` | - | - | `InstanceId` | - | `GetPasswordDataResult` | - | Retrieves the encrypted administrator password for a running Windows instance. The Windows password is generated at boot by the `EC2Config` service or `EC2Launch` scripts (Windows Server 2016 and later). |
| `GetReservedInstancesExchangeQuote` | - | - | `ReservedInstanceIds` | - | `GetReservedInstancesExchangeQuoteResult` | - | Returns a quote and exchange information for exchanging one or more specified Convertible Reserved Instances for a new Convertible Reserved Instance. If the exchange cannot be performed, the reason is returned in the response. |
| `GetRouteServerAssociations` | - | - | `RouteServerId` | - | `GetRouteServerAssociationsResult` | - | Gets information about the associations for the specified route server. A route server association is the connection established between a route server and a VPC. |
| `GetRouteServerPropagations` | - | - | `RouteServerId` | - | `GetRouteServerPropagationsResult` | - | Gets information about the route propagations for the specified route server. When enabled, route server propagation installs the routes in the FIB on the route table you've specified. |
| `GetRouteServerRoutingDatabase` | - | - | `RouteServerId` | - | `GetRouteServerRoutingDatabaseResult` | - | Gets the routing database for the specified route server. The Routing Information Base (RIB) serves as a database that stores all the routing information and network topology data collected by a router or routing system, such as routes learned from BGP peers. |
| `GetSecurityGroupsForVpc` | - | `paginated` | `VpcId` | - | `GetSecurityGroupsForVpcResult` | - | Gets security groups that can be associated by the Amazon Web Services account making the request with network interfaces in the specified VPC. |
| `GetSerialConsoleAccessStatus` | - | - | - | - | `GetSerialConsoleAccessStatusResult` | - | Retrieves the access status of your account to the EC2 serial console of all instances. By default, access to the EC2 serial console is disabled for your account. |
| `GetSnapshotBlockPublicAccessState` | - | - | - | - | `GetSnapshotBlockPublicAccessStateResult` | - | Gets the current state of block public access for snapshots setting for the account and Region. For more information, see Block public access for snapshots in the Amazon EBS User Guide . |
| `GetSpotPlacementScores` | - | `paginated` | `TargetCapacity` | - | `GetSpotPlacementScoresResult` | - | Calculates the Spot placement score for a Region or Availability Zone based on the specified target capacity and compute requirements. You can specify your compute requirements either by using `InstanceRequirementsWithMetadata` and letting Amazon EC2 choose... |
| `GetSubnetCidrReservations` | - | - | `SubnetId` | - | `GetSubnetCidrReservationsResult` | - | Gets information about the subnet CIDR reservations. |
| `GetTransitGatewayAttachmentPropagations` | - | `paginated` | `TransitGatewayAttachmentId` | - | `GetTransitGatewayAttachmentPropagationsResult` | - | Lists the route tables to which the specified resource attachment propagates routes. |
| `GetTransitGatewayMeteringPolicyEntries` | - | - | `TransitGatewayMeteringPolicyId` | - | `GetTransitGatewayMeteringPolicyEntriesResult` | - | Retrieves the entries for a transit gateway metering policy. |
| `GetTransitGatewayMulticastDomainAssociations` | - | `paginated` | `TransitGatewayMulticastDomainId` | - | `GetTransitGatewayMulticastDomainAssociationsResult` | - | Gets information about the associations for the transit gateway multicast domain. |
| `GetTransitGatewayPolicyTableAssociations` | - | `paginated` | `TransitGatewayPolicyTableId` | - | `GetTransitGatewayPolicyTableAssociationsResult` | - | Gets a list of the transit gateway policy table associations. |
| `GetTransitGatewayPolicyTableEntries` | - | - | `TransitGatewayPolicyTableId` | - | `GetTransitGatewayPolicyTableEntriesResult` | - | Returns a list of transit gateway policy table entries. |
| `GetTransitGatewayPrefixListReferences` | - | `paginated` | `TransitGatewayRouteTableId` | - | `GetTransitGatewayPrefixListReferencesResult` | - | Gets information about the prefix list references in a specified transit gateway route table. |
| `GetTransitGatewayRouteTableAssociations` | - | `paginated` | `TransitGatewayRouteTableId` | - | `GetTransitGatewayRouteTableAssociationsResult` | - | Gets information about the associations for the specified transit gateway route table. |
| `GetTransitGatewayRouteTablePropagations` | - | `paginated` | `TransitGatewayRouteTableId` | - | `GetTransitGatewayRouteTablePropagationsResult` | - | Gets information about the route table propagations for the specified transit gateway route table. |
| `GetVerifiedAccessEndpointPolicy` | - | - | `VerifiedAccessEndpointId` | - | `GetVerifiedAccessEndpointPolicyResult` | - | Get the Verified Access policy associated with the endpoint. |
| `GetVerifiedAccessEndpointTargets` | - | - | `VerifiedAccessEndpointId` | - | `GetVerifiedAccessEndpointTargetsResult` | - | Gets the targets for the specified network CIDR endpoint for Verified Access. |
| `GetVerifiedAccessGroupPolicy` | - | - | `VerifiedAccessGroupId` | - | `GetVerifiedAccessGroupPolicyResult` | - | Shows the contents of the Verified Access policy associated with the group. |
| `GetVpcResourcesBlockingEncryptionEnforcement` | - | - | `VpcId` | - | `GetVpcResourcesBlockingEncryptionEnforcementResult` | - | Gets information about resources in a VPC that are blocking encryption enforcement. For more information, see Enforce VPC encryption in transit in the Amazon VPC User Guide . |
| `GetVpnConnectionDeviceSampleConfiguration` | - | - | `VpnConnectionDeviceTypeId`, `VpnConnectionId` | - | `GetVpnConnectionDeviceSampleConfigurationResult` | - | Download an Amazon Web Services-provided sample configuration file to be used with the customer gateway device specified for your Site-to-Site VPN connection. |
| `GetVpnConnectionDeviceTypes` | - | `paginated` | - | - | `GetVpnConnectionDeviceTypesResult` | - | Obtain a list of customer gateway devices for which sample configuration files can be provided. The request has no additional parameters. |
| `GetVpnTunnelReplacementStatus` | - | - | `VpnConnectionId`, `VpnTunnelOutsideIpAddress` | - | `GetVpnTunnelReplacementStatusResult` | - | Get details of available tunnel endpoint maintenance. |
| `ImportClientVpnClientCertificateRevocationList` | - | - | `CertificateRevocationList`, `ClientVpnEndpointId` | - | `ImportClientVpnClientCertificateRevocationListResult` | - | Uploads a client certificate revocation list to the specified Client VPN endpoint. Uploading a client certificate revocation list overwrites the existing client certificate revocation list. |
| `ImportImage` | - | - | - | - | `ImportImageResult` | - | To import your virtual machines (VMs) with a console-based experience, you can use the Import virtual machine images to Amazon Web Services template in the Migration Hub Orchestrator console. For more information, see the Migration Hub Orchestrator User Guide... |
| `ImportInstance` | - | - | `Platform` | - | `ImportInstanceResult` | - | We recommend that you use the `ImportImage` API instead. For more information, see Importing a VM as an image using VM Import/Export in the VM Import/Export User Guide . |
| `ImportKeyPair` | - | - | `KeyName`, `PublicKeyMaterial` | - | `ImportKeyPairResult` | - | Imports the public key from an RSA or ED25519 key pair that you created using a third-party tool. You give Amazon Web Services only the public key. |
| `ImportSnapshot` | - | - | - | - | `ImportSnapshotResult` | - | Imports a disk into an EBS snapshot. For more information, see Importing a disk as a snapshot using VM Import/Export in the VM Import/Export User Guide . |
| `ImportVolume` | - | - | `Image`, `Volume` | - | `ImportVolumeResult` | - | This API action supports only single-volume VMs. To import multi-volume VMs, use ImportImage instead. |
| `ListImagesInRecycleBin` | - | `paginated` | - | - | `ListImagesInRecycleBinResult` | - | Lists one or more AMIs that are currently in the Recycle Bin. For more information, see Recycle Bin in the Amazon EC2 User Guide . |
| `ListSnapshotsInRecycleBin` | - | `paginated` | - | - | `ListSnapshotsInRecycleBinResult` | - | Lists one or more snapshots that are currently in the Recycle Bin. |
| `ListVolumesInRecycleBin` | - | - | - | - | `ListVolumesInRecycleBinResult` | - | Lists one or more volumes that are currently in the Recycle Bin. |
| `LockSnapshot` | - | - | `LockMode`, `SnapshotId` | - | `LockSnapshotResult` | - | Locks an Amazon EBS snapshot in either governance or compliance mode to protect it against accidental or malicious deletions for a specific duration. A locked snapshot can't be deleted. |
| `ModifyAddressAttribute` | - | - | `AllocationId` | - | `ModifyAddressAttributeResult` | - | Modifies an attribute of the specified Elastic IP address. For requirements, see Using reverse DNS for email applications. |
| `ModifyAvailabilityZoneGroup` | - | - | `GroupName`, `OptInStatus` | - | `ModifyAvailabilityZoneGroupResult` | - | Changes the opt-in status of the specified zone group for your account. |
| `ModifyCapacityReservation` | - | - | `CapacityReservationId` | - | `ModifyCapacityReservationResult` | - | Modifies a Capacity Reservation's capacity, instance eligibility, and the conditions under which it is to be released. You can't modify a Capacity Reservation's instance type, EBS optimization, platform, instance store settings, Availability Zone, or tenancy. |
| `ModifyCapacityReservationFleet` | - | - | `CapacityReservationFleetId` | - | `ModifyCapacityReservationFleetResult` | - | Modifies a Capacity Reservation Fleet. When you modify the total target capacity of a Capacity Reservation Fleet, the Fleet automatically creates new Capacity Reservations, or modifies or cancels existing Capacity Reservations in the Fleet to meet the new... |
| `ModifyClientVpnEndpoint` | - | - | `ClientVpnEndpointId` | - | `ModifyClientVpnEndpointResult` | - | Modifies the specified Client VPN endpoint. Modifying the DNS server resets existing client connections. |
| `ModifyDefaultCreditSpecification` | - | - | `CpuCredits`, `InstanceFamily` | - | `ModifyDefaultCreditSpecificationResult` | - | Modifies the default credit option for CPU usage of burstable performance instances. The default credit option is set at the account level per Amazon Web Services Region, and is specified per instance family. |
| `ModifyEbsDefaultKmsKeyId` | - | - | `KmsKeyId` | - | `ModifyEbsDefaultKmsKeyIdResult` | - | Changes the default KMS key for EBS encryption by default for your account in this Region. Amazon Web Services creates a unique Amazon Web Services managed KMS key in each Region for use with encryption by default. |
| `ModifyFleet` | - | - | `FleetId` | - | `ModifyFleetResult` | - | Modifies the specified EC2 Fleet. You can only modify an EC2 Fleet request of type `maintain`. |
| `ModifyFpgaImageAttribute` | - | - | `FpgaImageId` | - | `ModifyFpgaImageAttributeResult` | - | Modifies the specified attribute of the specified Amazon FPGA Image (AFI). |
| `ModifyHosts` | - | - | `HostIds` | - | `ModifyHostsResult` | - | Modify the auto-placement setting of a Dedicated Host. When auto-placement is enabled, any instances that you launch with a tenancy of `host` but without a specific host ID are placed onto any available Dedicated Host in your account that has auto-placement... |
| `ModifyIdFormat` | - | - | `Resource`, `UseLongIds` | - | `Unit` | - | Modifies the ID format for the specified resource on a per-Region basis. You can specify that resources should receive longer IDs (17-character IDs) when they are created. |
| `ModifyIdentityIdFormat` | - | - | `PrincipalArn`, `Resource`, `UseLongIds` | - | `Unit` | - | Modifies the ID format of a resource for a specified IAM user, IAM role, or the root user for an account; or all IAM users, IAM roles, and the root user for an account. You can specify that resources should receive longer IDs (17-character IDs) when they are... |
| `ModifyImageAttribute` | - | - | `ImageId` | - | `Unit` | - | Modifies the specified attribute of the specified AMI. You can specify only one attribute at a time. |
| `ModifyInstanceAttribute` | - | - | `InstanceId` | - | `Unit` | - | Modifies the specified attribute of the specified instance. You can specify only one attribute at a time. |
| `ModifyInstanceCapacityReservationAttributes` | - | - | `CapacityReservationSpecification`, `InstanceId` | - | `ModifyInstanceCapacityReservationAttributesResult` | - | Modifies the Capacity Reservation settings for a stopped instance. Use this action to configure an instance to target a specific Capacity Reservation, run in any `open` Capacity Reservation with matching attributes, run in On-Demand Instance capacity, or only... |
| `ModifyInstanceConnectEndpoint` | - | - | `InstanceConnectEndpointId` | - | `ModifyInstanceConnectEndpointResult` | - | Modifies the specified EC2 Instance Connect Endpoint. For more information, see Modify an EC2 Instance Connect Endpoint in the Amazon EC2 User Guide . |
| `ModifyInstanceCpuOptions` | - | - | `InstanceId` | - | `ModifyInstanceCpuOptionsResult` | - | By default, all vCPUs for the instance type are active when you launch an instance. When you configure the number of active vCPUs for the instance, it can help you save on licensing costs and optimize performance. |
| `ModifyInstanceCreditSpecification` | - | - | `InstanceCreditSpecifications` | - | `ModifyInstanceCreditSpecificationResult` | - | Modifies the credit option for CPU usage on a running or stopped burstable performance instance. The credit options are `standard` and `unlimited`. |
| `ModifyInstanceEventStartTime` | - | - | `InstanceEventId`, `InstanceId`, `NotBefore` | - | `ModifyInstanceEventStartTimeResult` | - | Modifies the start time for a scheduled Amazon EC2 instance event. |
| `ModifyInstanceEventWindow` | - | - | `InstanceEventWindowId` | - | `ModifyInstanceEventWindowResult` | - | Modifies the specified event window. You can define either a set of time ranges or a cron expression when modifying the event window, but not both. |
| `ModifyInstanceMaintenanceOptions` | - | - | `InstanceId` | - | `ModifyInstanceMaintenanceOptionsResult` | - | Modifies the recovery behavior of your instance to disable simplified automatic recovery or set the recovery behavior to default. The default configuration will not enable simplified automatic recovery for an unsupported instance type. |
| `ModifyInstanceMetadataDefaults` | - | - | - | - | `ModifyInstanceMetadataDefaultsResult` | - | Modifies the default instance metadata service (IMDS) settings at the account level in the specified Amazon Web Services Region. To remove a parameter's account-level default setting, specify `no-preference`. |
| `ModifyInstanceMetadataOptions` | - | - | `InstanceId` | - | `ModifyInstanceMetadataOptionsResult` | - | Modify the instance metadata parameters on a running or stopped instance. When you modify the parameters on a stopped instance, they are applied when the instance is started. |
| `ModifyInstanceNetworkPerformanceOptions` | - | - | `BandwidthWeighting`, `InstanceId` | - | `ModifyInstanceNetworkPerformanceResult` | - | Change the configuration of the network performance options for an existing instance. |
| `ModifyInstancePlacement` | - | - | `InstanceId` | - | `ModifyInstancePlacementResult` | - | Modifies the placement attributes for a specified instance. You can do the following: Modify the affinity between an instance and a Dedicated Host. |
| `ModifyIpam` | - | - | `IpamId` | - | `ModifyIpamResult` | - | Modify the configurations of an IPAM. |
| `ModifyIpamPolicyAllocationRules` | - | - | `IpamPolicyId`, `Locale`, `ResourceType` | - | `ModifyIpamPolicyAllocationRulesResult` | - | Modifies the allocation rules in an IPAM policy. An IPAM policy is a set of rules that define how public IPv4 addresses from IPAM pools are allocated to Amazon Web Services resources. |
| `ModifyIpamPool` | - | - | `IpamPoolId` | - | `ModifyIpamPoolResult` | - | Modify the configurations of an IPAM pool. For more information, see Modify a pool in the Amazon VPC IPAM User Guide . |
| `ModifyIpamPrefixListResolver` | - | - | `IpamPrefixListResolverId` | - | `ModifyIpamPrefixListResolverResult` | - | Modifies an IPAM prefix list resolver. You can update the description and CIDR selection rules. |
| `ModifyIpamPrefixListResolverTarget` | - | `idempotency-token` | `IpamPrefixListResolverTargetId` | `ClientToken` | `ModifyIpamPrefixListResolverTargetResult` | - | Modifies an IPAM prefix list resolver target. You can update version tracking settings and the desired version of the target prefix list. |
| `ModifyIpamResourceCidr` | - | - | `CurrentIpamScopeId`, `Monitored`, `ResourceCidr`, `ResourceId`, `ResourceRegion` | - | `ModifyIpamResourceCidrResult` | - | Modify a resource CIDR. You can use this action to transfer resource CIDRs between scopes and ignore resource CIDRs that you do not want to manage. |
| `ModifyIpamResourceDiscovery` | - | - | `IpamResourceDiscoveryId` | - | `ModifyIpamResourceDiscoveryResult` | - | Modifies a resource discovery. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account. |
| `ModifyIpamScope` | - | - | `IpamScopeId` | - | `ModifyIpamScopeResult` | - | Modify an IPAM scope. |
| `ModifyLaunchTemplate` | - | `idempotency-token` | - | `ClientToken` | `ModifyLaunchTemplateResult` | - | Modifies a launch template. You can specify which version of the launch template to set as the default version. |
| `ModifyLocalGatewayRoute` | - | - | `LocalGatewayRouteTableId` | - | `ModifyLocalGatewayRouteResult` | - | Modifies the specified local gateway route. |
| `ModifyManagedPrefixList` | - | - | `PrefixListId` | - | `ModifyManagedPrefixListResult` | - | Modifies the specified managed prefix list. Adding or removing entries in a prefix list creates a new version of the prefix list. |
| `ModifyNetworkInterfaceAttribute` | - | - | `NetworkInterfaceId` | - | `Unit` | - | Modifies the specified network interface attribute. You can specify only one attribute at a time. |
| `ModifyPrivateDnsNameOptions` | - | - | `InstanceId` | - | `ModifyPrivateDnsNameOptionsResult` | - | Modifies the options for instance hostnames for the specified instance. |
| `ModifyPublicIpDnsNameOptions` | - | - | `HostnameType`, `NetworkInterfaceId` | - | `ModifyPublicIpDnsNameOptionsResult` | - | Modify public hostname options for a network interface. For more information, see EC2 instance hostnames, DNS names, and domains in the Amazon EC2 User Guide . |
| `ModifyReservedInstances` | - | - | `ReservedInstancesIds`, `TargetConfigurations` | - | `ModifyReservedInstancesResult` | - | Modifies the configuration of your Reserved Instances, such as the Availability Zone, instance count, or instance type. The Reserved Instances to be modified must be identical, except for Availability Zone, network platform, and instance type. |
| `ModifyRouteServer` | - | - | `RouteServerId` | - | `ModifyRouteServerResult` | - | Modifies the configuration of an existing route server. Amazon VPC Route Server simplifies routing for traffic between workloads that are deployed within a VPC and its internet gateways. |
| `ModifySecurityGroupRules` | - | - | `GroupId`, `SecurityGroupRules` | - | `ModifySecurityGroupRulesResult` | - | Modifies the rules of a security group. |
| `ModifySnapshotAttribute` | - | - | `SnapshotId` | - | `Unit` | - | Adds or removes permission settings for the specified snapshot. You may add or remove specified Amazon Web Services account IDs from a snapshot's list of create volume permissions, but you cannot do both in a single operation. |
| `ModifySnapshotTier` | - | - | `SnapshotId` | - | `ModifySnapshotTierResult` | - | Archives an Amazon EBS snapshot. When you archive a snapshot, it is converted to a full snapshot that includes all of the blocks of data that were written to the volume at the time the snapshot was created, and moved from the standard tier to the archive tier. |
| `ModifySpotFleetRequest` | - | - | `SpotFleetRequestId` | - | `ModifySpotFleetRequestResponse` | - | Modifies the specified Spot Fleet request. You can only modify a Spot Fleet request of type `maintain`. |
| `ModifySubnetAttribute` | - | - | `SubnetId` | - | `Unit` | - | Modifies a subnet attribute. You can only modify one attribute at a time. |
| `ModifyTrafficMirrorFilterNetworkServices` | - | - | `TrafficMirrorFilterId` | - | `ModifyTrafficMirrorFilterNetworkServicesResult` | - | Allows or restricts mirroring network services. By default, Amazon DNS network services are not eligible for Traffic Mirror. |
| `ModifyTrafficMirrorFilterRule` | - | - | `TrafficMirrorFilterRuleId` | - | `ModifyTrafficMirrorFilterRuleResult` | - | Modifies the specified Traffic Mirror rule. `DestinationCidrBlock` and `SourceCidrBlock` must both be an IPv4 range or an IPv6 range. |
| `ModifyTrafficMirrorSession` | - | - | `TrafficMirrorSessionId` | - | `ModifyTrafficMirrorSessionResult` | - | Modifies a Traffic Mirror session. |
| `ModifyTransitGateway` | - | - | `TransitGatewayId` | - | `ModifyTransitGatewayResult` | - | Modifies the specified transit gateway. When you modify a transit gateway, the modified options are applied to new transit gateway attachments only. |
| `ModifyTransitGatewayMeteringPolicy` | - | - | `TransitGatewayMeteringPolicyId` | - | `ModifyTransitGatewayMeteringPolicyResult` | - | Modifies a transit gateway metering policy. |
| `ModifyTransitGatewayPrefixListReference` | - | - | `PrefixListId`, `TransitGatewayRouteTableId` | - | `ModifyTransitGatewayPrefixListReferenceResult` | - | Modifies a reference (route) to a prefix list in a specified transit gateway route table. |
| `ModifyTransitGatewayVpcAttachment` | - | - | `TransitGatewayAttachmentId` | - | `ModifyTransitGatewayVpcAttachmentResult` | - | Modifies the specified VPC attachment. |
| `ModifyVerifiedAccessEndpoint` | - | `idempotency-token` | `VerifiedAccessEndpointId` | `ClientToken` | `ModifyVerifiedAccessEndpointResult` | - | Modifies the configuration of the specified Amazon Web Services Verified Access endpoint. |
| `ModifyVerifiedAccessEndpointPolicy` | - | `idempotency-token` | `VerifiedAccessEndpointId` | `ClientToken` | `ModifyVerifiedAccessEndpointPolicyResult` | - | Modifies the specified Amazon Web Services Verified Access endpoint policy. |
| `ModifyVerifiedAccessGroup` | - | `idempotency-token` | `VerifiedAccessGroupId` | `ClientToken` | `ModifyVerifiedAccessGroupResult` | - | Modifies the specified Amazon Web Services Verified Access group configuration. |
| `ModifyVerifiedAccessGroupPolicy` | - | `idempotency-token` | `VerifiedAccessGroupId` | `ClientToken` | `ModifyVerifiedAccessGroupPolicyResult` | - | Modifies the specified Amazon Web Services Verified Access group policy. |
| `ModifyVerifiedAccessInstance` | - | `idempotency-token` | `VerifiedAccessInstanceId` | `ClientToken` | `ModifyVerifiedAccessInstanceResult` | - | Modifies the configuration of the specified Amazon Web Services Verified Access instance. |
| `ModifyVerifiedAccessInstanceLoggingConfiguration` | - | `idempotency-token` | `AccessLogs`, `VerifiedAccessInstanceId` | `ClientToken` | `ModifyVerifiedAccessInstanceLoggingConfigurationResult` | - | Modifies the logging configuration for the specified Amazon Web Services Verified Access instance. |
| `ModifyVerifiedAccessTrustProvider` | - | `idempotency-token` | `VerifiedAccessTrustProviderId` | `ClientToken` | `ModifyVerifiedAccessTrustProviderResult` | - | Modifies the configuration of the specified Amazon Web Services Verified Access trust provider. |
| `ModifyVolume` | - | - | `VolumeId` | - | `ModifyVolumeResult` | - | You can modify several parameters of an existing EBS volume, including volume size, volume type, and IOPS capacity. If your EBS volume is attached to a current-generation EC2 instance type, you might be able to apply these changes without stopping the... |
| `ModifyVolumeAttribute` | - | - | `VolumeId` | - | `Unit` | - | Modifies a volume attribute. By default, all I/O operations for the volume are suspended when the data on the volume is determined to be potentially inconsistent, to prevent undetectable, latent data corruption. |
| `ModifyVpcAttribute` | - | - | `VpcId` | - | `Unit` | - | Modifies the specified attribute of the specified VPC. |
| `ModifyVpcBlockPublicAccessExclusion` | - | - | `ExclusionId`, `InternetGatewayExclusionMode` | - | `ModifyVpcBlockPublicAccessExclusionResult` | - | Modify VPC Block Public Access (BPA) exclusions. A VPC BPA exclusion is a mode that can be applied to a single VPC or subnet that exempts it from the account’s BPA mode and will allow bidirectional or egress-only access. |
| `ModifyVpcBlockPublicAccessOptions` | - | - | `InternetGatewayBlockMode` | - | `ModifyVpcBlockPublicAccessOptionsResult` | - | Modify VPC Block Public Access (BPA) options. VPC Block Public Access (BPA) enables you to block resources in VPCs and subnets that you own in a Region from reaching or being reached from the internet through internet gateways and egress-only internet... |
| `ModifyVpcEncryptionControl` | - | - | `VpcEncryptionControlId` | - | `ModifyVpcEncryptionControlResult` | - | Modifies the encryption control configuration for a VPC. You can update the encryption mode and exclusion settings for various gateway types and peering connections. |
| `ModifyVpcEndpoint` | - | - | `VpcEndpointId` | - | `ModifyVpcEndpointResult` | - | Modifies attributes of a specified VPC endpoint. The attributes that you can modify depend on the type of VPC endpoint (interface, gateway, or Gateway Load Balancer). |
| `ModifyVpcEndpointConnectionNotification` | - | - | `ConnectionNotificationId` | - | `ModifyVpcEndpointConnectionNotificationResult` | - | Modifies a connection notification for VPC endpoint or VPC endpoint service. You can change the SNS topic for the notification, or the events for which to be notified. |
| `ModifyVpcEndpointServiceConfiguration` | - | - | `ServiceId` | - | `ModifyVpcEndpointServiceConfigurationResult` | - | Modifies the attributes of the specified VPC endpoint service configuration. If you set or modify the private DNS name, you must prove that you own the private DNS domain name. |
| `ModifyVpcEndpointServicePayerResponsibility` | - | - | `PayerResponsibility`, `ServiceId` | - | `ModifyVpcEndpointServicePayerResponsibilityResult` | - | Modifies the payer responsibility for your VPC endpoint service. |
| `ModifyVpcEndpointServicePermissions` | - | - | `ServiceId` | - | `ModifyVpcEndpointServicePermissionsResult` | - | Modifies the permissions for your VPC endpoint service. You can add or remove permissions for service consumers (Amazon Web Services accounts, users, and IAM roles) to connect to your endpoint service. |
| `ModifyVpcPeeringConnectionOptions` | - | - | `VpcPeeringConnectionId` | - | `ModifyVpcPeeringConnectionOptionsResult` | - | Modifies the VPC peering connection options on one side of a VPC peering connection. If the peered VPCs are in the same Amazon Web Services account, you can enable DNS resolution for queries from the local VPC. |
| `ModifyVpcTenancy` | - | - | `InstanceTenancy`, `VpcId` | - | `ModifyVpcTenancyResult` | - | Modifies the instance tenancy attribute of the specified VPC. You can change the instance tenancy attribute of a VPC to `default` only. |
| `ModifyVpnConnection` | - | - | `VpnConnectionId` | - | `ModifyVpnConnectionResult` | - | Modifies the customer gateway or the target gateway of an Amazon Web Services Site-to-Site VPN connection. To modify the target gateway, the following migration options are available: An existing virtual private gateway to a new virtual private gateway An... |
| `ModifyVpnConnectionOptions` | - | - | `VpnConnectionId` | - | `ModifyVpnConnectionOptionsResult` | - | Modifies the connection options for your Site-to-Site VPN connection. When you modify the VPN connection options, the VPN endpoint IP addresses on the Amazon Web Services side do not change, and the tunnel options do not change. |
| `ModifyVpnTunnelCertificate` | - | - | `VpnConnectionId`, `VpnTunnelOutsideIpAddress` | - | `ModifyVpnTunnelCertificateResult` | - | Modifies the VPN tunnel endpoint certificate. |
| `ModifyVpnTunnelOptions` | - | - | `TunnelOptions`, `VpnConnectionId`, `VpnTunnelOutsideIpAddress` | - | `ModifyVpnTunnelOptionsResult` | - | Modifies the options for a VPN tunnel in an Amazon Web Services Site-to-Site VPN connection. You can modify multiple options for a tunnel in a single request, but you can only modify one tunnel at a time. |
| `MonitorInstances` | - | - | `InstanceIds` | - | `MonitorInstancesResult` | - | Enables detailed monitoring for a running instance. Otherwise, basic monitoring is enabled. |
| `MoveAddressToVpc` | - | - | `PublicIp` | - | `MoveAddressToVpcResult` | - | This action is deprecated. Moves an Elastic IP address from the EC2-Classic platform to the EC2-VPC platform. |
| `MoveByoipCidrToIpam` | - | - | `Cidr`, `IpamPoolId`, `IpamPoolOwner` | - | `MoveByoipCidrToIpamResult` | - | Move a BYOIPv4 CIDR to IPAM from a public IPv4 pool. If you already have a BYOIPv4 CIDR with Amazon Web Services, you can move the CIDR to IPAM from a public IPv4 pool. |
| `MoveCapacityReservationInstances` | - | `idempotency-token` | `DestinationCapacityReservationId`, `InstanceCount`, `SourceCapacityReservationId` | `ClientToken` | `MoveCapacityReservationInstancesResult` | - | Move available capacity from a source Capacity Reservation to a destination Capacity Reservation. The source Capacity Reservation and the destination Capacity Reservation must be `active`, owned by your Amazon Web Services account, and share the following... |
| `ProvisionByoipCidr` | - | - | `Cidr` | - | `ProvisionByoipCidrResult` | - | Provisions an IPv4 or IPv6 address range for use with your Amazon Web Services resources through bring your own IP addresses (BYOIP) and creates a corresponding address pool. After the address range is provisioned, it is ready to be advertised. |
| `ProvisionIpamByoasn` | - | - | `Asn`, `AsnAuthorizationContext`, `IpamId` | - | `ProvisionIpamByoasnResult` | - | Provisions your Autonomous System Number (ASN) for use in your Amazon Web Services account. This action requires authorization context for Amazon to bring the ASN to an Amazon Web Services account. |
| `ProvisionIpamPoolCidr` | - | `idempotency-token` | `IpamPoolId` | `ClientToken` | `ProvisionIpamPoolCidrResult` | - | Provision a CIDR to an IPAM pool. You can use this action to provision new CIDRs to a top-level pool or to transfer a CIDR from a top-level pool to a pool within it. |
| `ProvisionPublicIpv4PoolCidr` | - | - | `IpamPoolId`, `NetmaskLength`, `PoolId` | - | `ProvisionPublicIpv4PoolCidrResult` | - | Provision a CIDR to a public IPv4 pool. For more information about IPAM, see What is IPAM? |
| `PurchaseCapacityBlock` | - | - | `CapacityBlockOfferingId`, `InstancePlatform` | - | `PurchaseCapacityBlockResult` | - | Purchase the Capacity Block for use with your account. With Capacity Blocks you ensure GPU capacity is available for machine learning (ML) workloads. |
| `PurchaseCapacityBlockExtension` | - | - | `CapacityBlockExtensionOfferingId`, `CapacityReservationId` | - | `PurchaseCapacityBlockExtensionResult` | - | Purchase the Capacity Block extension for use with your account. You must specify the ID of the Capacity Block extension offering you are purchasing. |
| `PurchaseHostReservation` | - | - | `HostIdSet`, `OfferingId` | - | `PurchaseHostReservationResult` | - | Purchase a reservation with configurations that match those of your Dedicated Host. You must have active Dedicated Hosts in your account before you purchase a reservation. |
| `PurchaseReservedInstancesOffering` | - | - | `InstanceCount`, `ReservedInstancesOfferingId` | - | `PurchaseReservedInstancesOfferingResult` | - | Purchases a Reserved Instance for use with your account. With Reserved Instances, you pay a lower hourly rate compared to On-Demand instance pricing. |
| `PurchaseScheduledInstances` | - | `idempotency-token` | `PurchaseRequests` | `ClientToken` | `PurchaseScheduledInstancesResult` | - | You can no longer purchase Scheduled Instances. Purchases the Scheduled Instances with the specified schedule. |
| `RebootInstances` | - | - | `InstanceIds` | - | `Unit` | - | Requests a reboot of the specified instances. This operation is asynchronous; it only queues a request to reboot the specified instances. |
| `RegisterImage` | - | - | `Name` | - | `RegisterImageResult` | - | Registers an AMI. When you're creating an instance-store backed AMI, registering the AMI is the final step in the creation process. |
| `RegisterInstanceEventNotificationAttributes` | - | - | `InstanceTagAttribute` | - | `RegisterInstanceEventNotificationAttributesResult` | - | Registers a set of tag keys to include in scheduled event notifications for your resources. To remove tags, use DeregisterInstanceEventNotificationAttributes. |
| `RegisterTransitGatewayMulticastGroupMembers` | - | - | `NetworkInterfaceIds`, `TransitGatewayMulticastDomainId` | - | `RegisterTransitGatewayMulticastGroupMembersResult` | - | Registers members (network interfaces) with the transit gateway multicast group. A member is a network interface associated with a supported EC2 instance that receives multicast traffic. |
| `RegisterTransitGatewayMulticastGroupSources` | - | - | `NetworkInterfaceIds`, `TransitGatewayMulticastDomainId` | - | `RegisterTransitGatewayMulticastGroupSourcesResult` | - | Registers sources (network interfaces) with the specified transit gateway multicast group. A multicast source is a network interface attached to a supported instance that sends multicast traffic. |
| `RejectCapacityReservationBillingOwnership` | - | - | `CapacityReservationId` | - | `RejectCapacityReservationBillingOwnershipResult` | - | Rejects a request to assign billing of the available capacity of a shared Capacity Reservation to your account. For more information, see Billing assignment for shared Amazon EC2 Capacity Reservations. |
| `RejectTransitGatewayMulticastDomainAssociations` | - | - | - | - | `RejectTransitGatewayMulticastDomainAssociationsResult` | - | Rejects a request to associate cross-account subnets with a transit gateway multicast domain. |
| `RejectTransitGatewayPeeringAttachment` | - | - | `TransitGatewayAttachmentId` | - | `RejectTransitGatewayPeeringAttachmentResult` | - | Rejects a transit gateway peering attachment request. |
| `RejectTransitGatewayVpcAttachment` | - | - | `TransitGatewayAttachmentId` | - | `RejectTransitGatewayVpcAttachmentResult` | - | Rejects a request to attach a VPC to a transit gateway. The VPC attachment must be in the `pendingAcceptance` state. |
| `RejectVpcEndpointConnections` | - | - | `ServiceId`, `VpcEndpointIds` | - | `RejectVpcEndpointConnectionsResult` | - | Rejects VPC endpoint connection requests to your VPC endpoint service. |
| `RejectVpcPeeringConnection` | - | - | `VpcPeeringConnectionId` | - | `RejectVpcPeeringConnectionResult` | - | Rejects a VPC peering connection request. The VPC peering connection must be in the `pending-acceptance` state. |
| `ReleaseAddress` | - | - | - | - | `Unit` | - | Releases the specified Elastic IP address. [Default VPC] Releasing an Elastic IP address automatically disassociates it from any instance that it's associated with. |
| `ReleaseHosts` | - | - | `HostIds` | - | `ReleaseHostsResult` | - | When you no longer want to use an On-Demand Dedicated Host it can be released. On-Demand billing is stopped and the host goes into `released` state. |
| `ReleaseIpamPoolAllocation` | - | - | `Cidr`, `IpamPoolAllocationId`, `IpamPoolId` | - | `ReleaseIpamPoolAllocationResult` | - | Release an allocation within an IPAM pool. The Region you use should be the IPAM pool locale. |
| `ReplaceIamInstanceProfileAssociation` | - | - | `AssociationId`, `IamInstanceProfile` | - | `ReplaceIamInstanceProfileAssociationResult` | - | Replaces an IAM instance profile for the specified running instance. You can use this action to change the IAM instance profile that's associated with an instance without having to disassociate the existing IAM instance profile first. |
| `ReplaceImageCriteriaInAllowedImagesSettings` | - | - | - | - | `ReplaceImageCriteriaInAllowedImagesSettingsResult` | - | Sets or replaces the criteria for Allowed AMIs. The Allowed AMIs feature does not restrict the AMIs owned by your account. |
| `ReplaceNetworkAclAssociation` | - | - | `AssociationId`, `NetworkAclId` | - | `ReplaceNetworkAclAssociationResult` | - | Changes which network ACL a subnet is associated with. By default when you create a subnet, it's automatically associated with the default network ACL. |
| `ReplaceNetworkAclEntry` | - | - | `Egress`, `NetworkAclId`, `Protocol`, `RuleAction`, `RuleNumber` | - | `Unit` | - | Replaces an entry (rule) in a network ACL. For more information, see Network ACLs in the Amazon VPC User Guide . |
| `ReplaceRoute` | - | - | `RouteTableId` | - | `Unit` | - | Replaces an existing route within a route table in a VPC. You must specify either a destination CIDR block or a prefix list ID. |
| `ReplaceRouteTableAssociation` | - | - | `AssociationId`, `RouteTableId` | - | `ReplaceRouteTableAssociationResult` | - | Changes the route table associated with a given subnet, internet gateway, or virtual private gateway in a VPC. After the operation completes, the subnet or gateway uses the routes in the new route table. |
| `ReplaceTransitGatewayRoute` | - | - | `DestinationCidrBlock`, `TransitGatewayRouteTableId` | - | `ReplaceTransitGatewayRouteResult` | - | Replaces the specified route in the specified transit gateway route table. |
| `ReplaceVpnTunnel` | - | - | `VpnConnectionId`, `VpnTunnelOutsideIpAddress` | - | `ReplaceVpnTunnelResult` | - | Trigger replacement of specified VPN tunnel. |
| `ReportInstanceStatus` | - | - | `Instances`, `ReasonCodes`, `Status` | - | `Unit` | - | Submits feedback about the status of an instance. The instance must be in the `running` state. |
| `RequestSpotFleet` | - | - | `SpotFleetRequestConfig` | - | `RequestSpotFleetResponse` | - | Creates a Spot Fleet request. The Spot Fleet request specifies the total target capacity and the On-Demand target capacity. |
| `RequestSpotInstances` | - | - | - | - | `RequestSpotInstancesResult` | - | Creates a Spot Instance request. For more information, see Work with Spot Instance in the Amazon EC2 User Guide . |
| `ResetAddressAttribute` | - | - | `AllocationId`, `Attribute` | - | `ResetAddressAttributeResult` | - | Resets the attribute of the specified IP address. For requirements, see Using reverse DNS for email applications. |
| `ResetEbsDefaultKmsKeyId` | - | - | - | - | `ResetEbsDefaultKmsKeyIdResult` | - | Resets the default KMS key for EBS encryption for your account in this Region to the Amazon Web Services managed KMS key for EBS. After resetting the default KMS key to the Amazon Web Services managed KMS key, you can continue to encrypt by a customer managed... |
| `ResetFpgaImageAttribute` | - | - | `FpgaImageId` | - | `ResetFpgaImageAttributeResult` | - | Resets the specified attribute of the specified Amazon FPGA Image (AFI) to its default value. You can only reset the load permission attribute. |
| `ResetImageAttribute` | - | - | `Attribute`, `ImageId` | - | `Unit` | - | Resets an attribute of an AMI to its default value. |
| `ResetInstanceAttribute` | - | - | `Attribute`, `InstanceId` | - | `Unit` | - | Resets an attribute of an instance to its default value. To reset the `kernel` or `ramdisk`, the instance must be in a stopped state. |
| `ResetNetworkInterfaceAttribute` | - | - | `NetworkInterfaceId` | - | `Unit` | - | Resets a network interface attribute. You can specify only one attribute at a time. |
| `ResetSnapshotAttribute` | - | - | `Attribute`, `SnapshotId` | - | `Unit` | - | Resets permission settings for the specified snapshot. For more information about modifying snapshot permissions, see Share a snapshot in the Amazon EBS User Guide . |
| `RestoreAddressToClassic` | - | - | `PublicIp` | - | `RestoreAddressToClassicResult` | - | This action is deprecated. Restores an Elastic IP address that was previously moved to the EC2-VPC platform back to the EC2-Classic platform. |
| `RestoreImageFromRecycleBin` | - | - | `ImageId` | - | `RestoreImageFromRecycleBinResult` | - | Restores an AMI from the Recycle Bin. For more information, see Recover deleted Amazon EBS snapshots and EBS-back AMIs with Recycle Bin in the Amazon EC2 User Guide . |
| `RestoreManagedPrefixListVersion` | - | - | `CurrentVersion`, `PrefixListId`, `PreviousVersion` | - | `RestoreManagedPrefixListVersionResult` | - | Restores the entries from a previous version of a managed prefix list to a new version of the prefix list. |
| `RestoreSnapshotFromRecycleBin` | - | - | `SnapshotId` | - | `RestoreSnapshotFromRecycleBinResult` | - | Restores a snapshot from the Recycle Bin. For more information, see Restore snapshots from the Recycle Bin in the Amazon EBS User Guide . |
| `RestoreSnapshotTier` | - | - | `SnapshotId` | - | `RestoreSnapshotTierResult` | - | Restores an archived Amazon EBS snapshot for use temporarily or permanently, or modifies the restore period or restore type for a snapshot that was previously temporarily restored. For more information see Restore an archived snapshot and modify the restore... |
| `RestoreVolumeFromRecycleBin` | - | - | `VolumeId` | - | `RestoreVolumeFromRecycleBinResult` | - | Restores a volume from the Recycle Bin. For more information, see Restore volumes from the Recycle Bin in the Amazon EBS User Guide . |
| `RevokeClientVpnIngress` | - | - | `ClientVpnEndpointId`, `TargetNetworkCidr` | - | `RevokeClientVpnIngressResult` | - | Removes an ingress authorization rule from a Client VPN endpoint. |
| `RevokeSecurityGroupEgress` | - | - | `GroupId` | - | `RevokeSecurityGroupEgressResult` | - | Removes the specified outbound (egress) rules from the specified security group. You can specify rules using either rule IDs or security group rule properties. |
| `RevokeSecurityGroupIngress` | - | - | - | - | `RevokeSecurityGroupIngressResult` | - | Removes the specified inbound (ingress) rules from a security group. You can specify rules using either rule IDs or security group rule properties. |
| `RunInstances` | - | `idempotency-token` | `MaxCount`, `MinCount` | `ClientToken` | `Reservation` | - | Launches the specified number of instances using an AMI for which you have permissions. You can specify a number of options, or leave the default options. |
| `RunScheduledInstances` | - | `idempotency-token` | `LaunchSpecification`, `ScheduledInstanceId` | `ClientToken` | `RunScheduledInstancesResult` | - | Launches the specified Scheduled Instances. Before you can launch a Scheduled Instance, you must purchase it and obtain an identifier using PurchaseScheduledInstances. |
| `SearchLocalGatewayRoutes` | - | `paginated` | `LocalGatewayRouteTableId` | - | `SearchLocalGatewayRoutesResult` | - | Searches for routes in the specified local gateway route table. |
| `SearchTransitGatewayMulticastGroups` | - | `paginated` | `TransitGatewayMulticastDomainId` | - | `SearchTransitGatewayMulticastGroupsResult` | - | Searches one or more transit gateway multicast groups and returns the group membership information. |
| `SearchTransitGatewayRoutes` | - | `paginated` | `Filters`, `TransitGatewayRouteTableId` | - | `SearchTransitGatewayRoutesResult` | - | Searches for routes in the specified transit gateway route table. |
| `SendDiagnosticInterrupt` | - | - | `InstanceId` | - | `Unit` | - | Sends a diagnostic interrupt to the specified Amazon EC2 instance to trigger a kernel panic (on Linux instances), or a blue screen / stop error (on Windows instances). For instances based on Intel and AMD processors, the interrupt is received as a... |
| `StartDeclarativePoliciesReport` | - | - | `S3Bucket`, `TargetId` | - | `StartDeclarativePoliciesReportResult` | - | Generates an account status report. The report is generated asynchronously, and can take several hours to complete. |
| `StartInstances` | - | - | `InstanceIds` | - | `StartInstancesResult` | - | Starts an Amazon EBS-backed instance that you've previously stopped. Instances that use Amazon EBS volumes as their root devices can be quickly stopped and started. |
| `StartNetworkInsightsAccessScopeAnalysis` | - | `idempotency-token` | `ClientToken`, `NetworkInsightsAccessScopeId` | `ClientToken` | `StartNetworkInsightsAccessScopeAnalysisResult` | - | Starts analyzing the specified Network Access Scope. |
| `StartNetworkInsightsAnalysis` | - | `idempotency-token` | `ClientToken`, `NetworkInsightsPathId` | `ClientToken` | `StartNetworkInsightsAnalysisResult` | - | Starts analyzing the specified path. If the path is reachable, the operation returns the shortest feasible path. |
| `StartVpcEndpointServicePrivateDnsVerification` | - | - | `ServiceId` | - | `StartVpcEndpointServicePrivateDnsVerificationResult` | - | Initiates the verification process to prove that the service provider owns the private DNS name domain for the endpoint service. The service provider must successfully perform the verification before the consumer can use the name to access the service. |
| `StopInstances` | - | - | `InstanceIds` | - | `StopInstancesResult` | - | Stops an Amazon EBS-backed instance. You can restart your instance at any time using the StartInstances API. |
| `TerminateClientVpnConnections` | - | - | `ClientVpnEndpointId` | - | `TerminateClientVpnConnectionsResult` | - | Terminates active Client VPN endpoint connections. This action can be used to terminate a specific client connection, or up to five connections established by a specific user. |
| `TerminateInstances` | - | - | `InstanceIds` | - | `TerminateInstancesResult` | - | Terminates (deletes) the specified instances. This operation is idempotent; if you terminate an instance more than once, each call succeeds. |
| `UnassignIpv6Addresses` | - | - | `NetworkInterfaceId` | - | `UnassignIpv6AddressesResult` | - | Unassigns the specified IPv6 addresses or Prefix Delegation prefixes from a network interface. |
| `UnassignPrivateIpAddresses` | - | - | `NetworkInterfaceId` | - | `Unit` | - | Unassigns the specified secondary private IP addresses or IPv4 Prefix Delegation prefixes from a network interface. |
| `UnassignPrivateNatGatewayAddress` | - | - | `NatGatewayId`, `PrivateIpAddresses` | - | `UnassignPrivateNatGatewayAddressResult` | - | Unassigns secondary private IPv4 addresses from a private NAT gateway. You cannot unassign your primary private IP. |
| `UnlockSnapshot` | - | - | `SnapshotId` | - | `UnlockSnapshotResult` | - | Unlocks a snapshot that is locked in governance mode or that is locked in compliance mode but still in the cooling-off period. You can't unlock a snapshot that is locked in compliance mode after the cooling-off period has expired. |
| `UnmonitorInstances` | - | - | `InstanceIds` | - | `UnmonitorInstancesResult` | - | Disables detailed monitoring for a running instance. For more information, see Monitoring your instances and volumes in the Amazon EC2 User Guide . |
| `UpdateCapacityManagerOrganizationsAccess` | - | `idempotency-token` | `OrganizationsAccess` | `ClientToken` | `UpdateCapacityManagerOrganizationsAccessResult` | - | Updates the Organizations access setting for EC2 Capacity Manager. This controls whether Capacity Manager can aggregate data from all accounts in your Amazon Web Services Organization or only from the current account. |
| `UpdateInterruptibleCapacityReservationAllocation` | - | - | `CapacityReservationId`, `TargetInstanceCount` | - | `UpdateInterruptibleCapacityReservationAllocationResult` | - | Modifies the number of instances allocated to an interruptible reservation, allowing you to add more capacity or reclaim capacity to your source Capacity Reservation. |
| `UpdateSecurityGroupRuleDescriptionsEgress` | - | - | - | - | `UpdateSecurityGroupRuleDescriptionsEgressResult` | - | Updates the description of an egress (outbound) security group rule. You can replace an existing description, or add a description to a rule that did not have one previously. |
| `UpdateSecurityGroupRuleDescriptionsIngress` | - | - | - | - | `UpdateSecurityGroupRuleDescriptionsIngressResult` | - | Updates the description of an ingress (inbound) security group rule. You can replace an existing description, or add a description to a rule that did not have one previously. |
| `WithdrawByoipCidr` | - | - | `Cidr` | - | `WithdrawByoipCidrResult` | - | Stops advertising an address range that is provisioned as an address pool. You can perform this operation at most once every 10 seconds, even if you specify different address ranges each time. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `VolumeAttachment` | `structure` | `AssociatedResource`, `AttachTime`, `DeleteOnTermination`, `Device`, `EbsCardIndex`, `InstanceId`, `InstanceOwningService`, `State`, `VolumeId` | Describes volume attachment details. |
| `AcceptAddressTransferRequest` | `structure` | `Address`, `DryRun`, `TagSpecifications` | - |
| `AcceptAddressTransferResult` | `structure` | `AddressTransfer` | - |
| `AcceptCapacityReservationBillingOwnershipRequest` | `structure` | `CapacityReservationId`, `DryRun` | - |
| `AcceptCapacityReservationBillingOwnershipResult` | `structure` | `Return` | - |
| `AcceptReservedInstancesExchangeQuoteRequest` | `structure` | `DryRun`, `ReservedInstanceIds`, `TargetConfigurations` | Contains the parameters for accepting the quote. |
| `AcceptReservedInstancesExchangeQuoteResult` | `structure` | `ExchangeId` | The result of the exchange and whether it was `successful`. |
| `AcceptTransitGatewayMulticastDomainAssociationsRequest` | `structure` | `DryRun`, `SubnetIds`, `TransitGatewayAttachmentId`, `TransitGatewayMulticastDomainId` | - |
| `AcceptTransitGatewayMulticastDomainAssociationsResult` | `structure` | `Associations` | - |
| `AcceptTransitGatewayPeeringAttachmentRequest` | `structure` | `DryRun`, `TransitGatewayAttachmentId` | - |
| `AcceptTransitGatewayPeeringAttachmentResult` | `structure` | `TransitGatewayPeeringAttachment` | - |
| `AcceptTransitGatewayVpcAttachmentRequest` | `structure` | `DryRun`, `TransitGatewayAttachmentId` | - |
| `AcceptTransitGatewayVpcAttachmentResult` | `structure` | `TransitGatewayVpcAttachment` | - |
| `AcceptVpcEndpointConnectionsRequest` | `structure` | `DryRun`, `ServiceId`, `VpcEndpointIds` | - |
| `AcceptVpcEndpointConnectionsResult` | `structure` | `Unsuccessful` | - |
| `AcceptVpcPeeringConnectionRequest` | `structure` | `DryRun`, `VpcPeeringConnectionId` | - |
| `AcceptVpcPeeringConnectionResult` | `structure` | `VpcPeeringConnection` | - |
| `AdvertiseByoipCidrRequest` | `structure` | `Asn`, `Cidr`, `DryRun`, `NetworkBorderGroup` | - |
| `AdvertiseByoipCidrResult` | `structure` | `ByoipCidr` | - |
| `AllocateAddressRequest` | `structure` | `Address`, `CustomerOwnedIpv4Pool`, `Domain`, `DryRun`, `IpamPoolId`, `NetworkBorderGroup`, `PublicIpv4Pool`, `TagSpecifications` | - |
| `AllocateAddressResult` | `structure` | `AllocationId`, `CarrierIp`, `CustomerOwnedIp`, `CustomerOwnedIpv4Pool`, `Domain`, `NetworkBorderGroup`, `PublicIp`, `PublicIpv4Pool` | - |
| `AllocateHostsRequest` | `structure` | `AssetIds`, `AutoPlacement`, `AvailabilityZone`, `AvailabilityZoneId`, `ClientToken`, `HostMaintenance`, `HostRecovery`, `InstanceFamily`, `InstanceType`, `OutpostArn`, `Quantity`, `TagSpecifications` | - |
| `AllocateHostsResult` | `structure` | `HostIds` | Contains the output of AllocateHosts. |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/ec2-crate-split-and-feature-gating.md, .agents/docs/LTM/ec2-operation-expansion-and-invariants.md, .agents/docs/LTM/terraform-e2e-harness-and-fix-coverage.md, .agents/docs/LTM/smithy-codegen-and-wire-serialization.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: full distillation.

### Crate Split

- EC2 is split into `winterbaume-ec2-generated` for generated `model.rs` and `wire.rs` plus `winterbaume-ec2` for hand-written handlers, state, views, and re-exports.
- `winterbaume-ec2/Cargo.toml` intentionally uses a direct path dependency on `winterbaume-ec2-generated` with `default-features = false`; workspace dependency inheritance cannot override default features there.
- Coverage reporting reads `winterbaume-ec2`, not the generated crate, so the split should be invisible to API coverage machinery and to users of the public crate.

### Feature Gates

- The authoritative operation-to-feature mapping is `tools/smithy-codegen/ec2-features.toml`. Current buckets are `network`, `compute`, `storage`, `advanced-network`, and `extras`; `default = ["full"]` preserves published-crate behaviour.
- Adding an EC2 operation requires feature-bucket selection, regeneration with `--features-map`, cfg gates on the handler and dispatch arm, helper cfg gates where needed, and verification of both full and relevant partial-feature builds.
- Common failure modes are missing cfg gates on list/set wrapper types, assuming workspace dependency inheritance can disable generated-crate defaults, and checking only the full feature set.
- EC2 service work should update hand-written service logic in `winterbaume-ec2`; generated model or wire bugs belong in the generator and then in regenerated generated-crate output.
- Regenerate `winterbaume-ec2-generated` with `--features-map tools/smithy-codegen/ec2-features.toml`. Omitting the flag strips generated `#[cfg(feature = "...")]` attributes from `model.rs` and `wire.rs`, undoing the split-crate build-time benefit.

### Operation Expansion and Invariant Testing

- EC2 handler work is safest in sequential resource-family batches because most meaningful changes touch the same central files: `crates/winterbaume-ec2/src/handlers.rs`, `state.rs`, `types.rs`, and `views.rs`.
- Reduced-semantics implementations are acceptable where real EC2 depends on AWS-managed internals, but the simplification must be explicit. Examples include immediate Network Insights completion, synchronous snapshot import materialisation, unverified BYOIP authorisation signatures, empty BGP telemetry, seeded cross-account pending-acceptance state, and deterministic placeholders for catalogue-like purchase APIs.
- Scenario coverage should exercise more than one operation and then read the resulting state. EC2 has durable bug classes around launch-template default-version propagation, snapshot-derived volume defaults, fresh association IDs for repeated `AssociateAddress` calls, EBS encryption-by-default propagation, and ENI attachment ID uniqueness.
- Counter helpers need ID-family review. Reusing an EC2 resource counter for a distinct association or attachment namespace can mint duplicate AWS-visible IDs; `AssociateAddress` and `AttachNetworkInterface` are the reference cases.
- Terraform converter work for EC2 should follow view support. High-impact converter families include IPAM, Verified Access, Traffic Mirror, transit-gateway extensions, and Network Insights; Client VPN, Local Gateway, Route Server, and some Capacity Reservation extensions are known deferred areas.

### Terraform Provider Compatibility

- Describe handlers must honour explicit `<Id>.N` request members and standard `Filter.N` entries. Terraform's `tfresource.AssertSinglePtrResult` treats both zero rows and more than one row as not-found-class failures, so returning all state only works while a test has exactly one resource in that family.
- Generic Describe operations must include every subtype sharing an AWS id namespace. `DescribeTransitGatewayAttachments` must consider VPC, peering, and connect attachments because they all use `tgw-attach-...` ids.
- Local Gateway route and route-table VPC association flows may reference pre-provisioned Outposts-style route table ids. The mock auto-seeds those Local Gateway route tables on first use rather than requiring a preceding `CreateLocalGatewayRouteTable`.
- `DescribeIpamScopes` needs filtering by explicit ids and by filters such as `ipam-scope-id`, `ipam-id`, and `is-default`, because IPAM creation seeds default-private and default-public scopes alongside user-created scopes.
- The Terraform AWS provider v6.43.0 `aws_ec2_capacity_block_reservation` resource has an upstream AutoFlex mapping gap: it does not bridge SDK fields `CapacityReservationArn`, `CreateDate`, and `TotalInstanceCount` to framework fields `ARN`, `CreatedDate`, and `InstanceCount`. Winterbaume should keep emitting the Smithy-modelled XML fields; adding non-model XML aliases would not be a durable fix.

## Cross-call invariants

Sources: .agents/docs/LTM/ec2-operation-expansion-and-invariants.md, .agents/docs/LTM/aws-doc-test-plan-catalog.md.

| Category | This service's invariant | Doc URL |
|----------|--------------------------|---------|
| Toggle propagation | Account or regional toggles such as EBS encryption-by-default must affect later create calls when the per-resource field is omitted. | https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EnableEbsEncryptionByDefault.html |
| Modify rewrites sibling state | A parent default such as `ModifyLaunchTemplate.SetDefaultVersion` must be reflected on per-version read state, not only on the parent object. | https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyLaunchTemplate.html |
| Per-call uniqueness | Repeated association or attachment calls that return AWS-visible IDs must mint fresh IDs even when the parent resource is reused. `AssociateAddress` and `AttachNetworkInterface` are reference cases. | https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_AssociateAddress.html |
| Default inheritance from parent | A child create call can inherit fields from a parent resource, such as `CreateVolume` inheriting size from `SnapshotId` when `Size` is omitted. | https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateVolume.html |
| Lifecycle state transitions | Instance state-machine calls must report previous/current state and leave `DescribeInstances` consistent across stop, start, reboot, and terminate workflows. | https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_StopInstances.html |
| Cross-resource references on read | Reads must preserve linked networking relationships such as subnet network ACL associations, routes, peering attachments, VPN components, and ENI attachments. | https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ReplaceNetworkAclAssociation.html |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
