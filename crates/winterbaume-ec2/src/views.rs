//! Serde-compatible view types for EC2 state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Ec2Service;
use crate::state::Ec2State;
use crate::types::{
    AccessScopePathSpec, AddressTransfer, AsnAssociation, BillingOwnershipOffer, ByoipCidr,
    CapacityBlock, CapacityBlockExtension, CapacityManagerDataExport,
    CapacityManagerOrganizationsAccess, CapacityReservation, CapacityReservationCommitmentInfo,
    CapacityReservationFleet, CapacityReservationFleetInstanceSpec,
    CapacityReservationSpecificationResponse, CarrierGateway, ClientVpnAssociationStatus,
    ClientVpnAuthorizationRule, ClientVpnAuthorizationRuleStatus, ClientVpnConnection,
    ClientVpnConnectionStatus, ClientVpnEndpoint, ClientVpnEndpointStatus, ClientVpnRoute,
    ClientVpnRouteStatus, ClientVpnTargetNetworkAssociation, CoipCidr, CoipPool, ConversionTask,
    CustomerGateway, DeclarativePoliciesReport, DedicatedHost, DhcpConfiguration, DhcpOptions,
    Ec2Fleet, EgressOnlyInternetGateway, ElasticIp, EnclaveCertificateIamRoleAssociation,
    EoigwAttachment, ExportTask, FlowLog, IamInstanceProfileAssociation, IcmpTypeCode,
    IgwAttachment, Image, ImportInstanceVolumeDetail, Instance, InstanceConnectEndpoint,
    InstanceState, InternetGateway, InterruptibleCapacityReservationAllocation, IpPermission,
    IpRange, Ipam, IpamByoasn, IpamExternalResourceVerificationToken, IpamOperatingRegion,
    IpamPolicy, IpamPolicyAllocationRule, IpamPool, IpamPoolAllocation, IpamPoolCidr,
    IpamPrefixListResolver, IpamPrefixListResolverTarget, IpamResourceDiscovery,
    IpamResourceDiscoveryAssociation, IpamScope, Ipv6Range, KeyPair, LaunchTemplate,
    LaunchTemplateVersion, LocalGateway, LocalGatewayRoute, LocalGatewayRouteTable,
    LocalGatewayRouteTableVirtualInterfaceGroupAssociation, LocalGatewayRouteTableVpcAssociation,
    LocalGatewayVirtualInterface, LocalGatewayVirtualInterfaceGroup, MacSipModificationTask,
    MacVolumeOwnershipTask, ManagedPrefixList, MulticastSubnetAssociation, NatGateway,
    NatGatewayAddressAssociation, NetworkAcl, NetworkAclAssociation, NetworkAclEntry,
    NetworkInsightsAccessScope, NetworkInsightsAccessScopeAnalysis, NetworkInsightsAnalysis,
    NetworkInsightsPath, NetworkInsightsPathFilter, NetworkInsightsPathFilterPortRange,
    NetworkInterface, NetworkInterfacePermission, PacketHeaderStatementSpec, PathStatementSpec,
    PlacementGroup, PortRange, PublicIpv4Pool, PublicIpv4PoolRange, ReplaceRootVolumeTask,
    ResourceStatementSpec, Route, RouteServer, RouteServerAssociation, RouteServerBgpOptions,
    RouteServerEndpoint, RouteServerPeer, RouteServerPeerOptions, RouteTable,
    RouteTableAssociation, S3DestinationOptions, SecondaryNetwork, SecondarySubnet, SecurityGroup,
    SecurityGroupVpcAssociation, Snapshot, SnapshotImportTask, SpotFleetRequest,
    SpotInstanceRequest, Subnet, SubnetCidrReservationEntry, SubnetIpv6CidrAssoc,
    TrafficMirrorFilter, TrafficMirrorFilterRule, TrafficMirrorPortRange, TrafficMirrorSession,
    TrafficMirrorTarget, TransitGateway, TransitGatewayAttachmentBgpConfiguration,
    TransitGatewayConnect, TransitGatewayConnectPeer, TransitGatewayMeteringPolicy,
    TransitGatewayMeteringPolicyEntry, TransitGatewayMulticastDomain,
    TransitGatewayMulticastDomainAssociation, TransitGatewayMulticastGroupMember,
    TransitGatewayMulticastGroupSource, TransitGatewayPeeringAttachment, TransitGatewayPolicyTable,
    TransitGatewayPolicyTableAssociation, TransitGatewayPrefixListReference, TransitGatewayRoute,
    TransitGatewayRouteTable, TransitGatewayRouteTableAnnouncement, TransitGatewayVpcAttachment,
    TrunkInterfaceAssociation, TypesPrefixListEntry, UserIdGroupPair, VerifiedAccessDeviceOptions,
    VerifiedAccessEndpoint, VerifiedAccessEndpointCidrOptions, VerifiedAccessEndpointEniOptions,
    VerifiedAccessEndpointLoadBalancerOptions, VerifiedAccessEndpointPortRange,
    VerifiedAccessEndpointRdsOptions, VerifiedAccessGroup, VerifiedAccessInstance,
    VerifiedAccessLogs, VerifiedAccessNativeApplicationOidcOptions, VerifiedAccessOidcOptions,
    VerifiedAccessSseSpecification, VerifiedAccessTrustProvider,
    VerifiedAccessTrustProviderAttachment, VgwVpcAttachment, Volume, VolumeAttachment, Vpc,
    VpcBlockPublicAccessExclusion, VpcBlockPublicAccessOptions, VpcEncryptionControl, VpcEndpoint,
    VpcEndpointConnection, VpcEndpointConnectionNotification, VpcEndpointServiceConfiguration,
    VpcPeeringConnection, VpnConcentrator, VpnConnection, VpnConnectionOptions, VpnGateway,
    VpnStaticRoute, VpnTunnelOptions,
};

/// Serializable view of the entire EC2 state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ec2StateView {
    #[serde(default)]
    pub vpcs: HashMap<String, VpcView>,
    #[serde(default)]
    pub subnets: HashMap<String, SubnetView>,
    #[serde(default)]
    pub igws: HashMap<String, InternetGatewayView>,
    #[serde(default)]
    pub security_groups: HashMap<String, SecurityGroupView>,
    #[serde(default)]
    pub route_tables: HashMap<String, RouteTableView>,
    #[serde(default)]
    pub key_pairs: HashMap<String, KeyPairView>,
    #[serde(default)]
    pub network_acls: HashMap<String, NetworkAclView>,
    #[serde(default)]
    pub elastic_ips: HashMap<String, ElasticIpView>,
    #[serde(default)]
    pub nat_gateways: HashMap<String, NatGatewayView>,
    #[serde(default)]
    pub dhcp_options: HashMap<String, DhcpOptionsView>,
    #[serde(default)]
    pub egress_only_igws: HashMap<String, EgressOnlyIgwView>,
    #[serde(default)]
    pub flow_logs: HashMap<String, FlowLogView>,
    #[serde(default)]
    pub vpc_peering_connections: HashMap<String, VpcPeeringConnectionView>,
    #[serde(default)]
    pub vpc_endpoints: HashMap<String, VpcEndpointView>,
    #[serde(default)]
    pub managed_prefix_lists: HashMap<String, ManagedPrefixListView>,
    #[serde(default)]
    pub customer_gateways: HashMap<String, CustomerGatewayView>,
    #[serde(default)]
    pub vpn_gateways: HashMap<String, VpnGatewayView>,
    #[serde(default)]
    pub vpn_connections: HashMap<String, VpnConnectionView>,
    #[serde(default)]
    pub carrier_gateways: HashMap<String, CarrierGatewayView>,
    #[serde(default)]
    pub network_interfaces: HashMap<String, NetworkInterfaceView>,
    #[serde(default)]
    pub vpc_cidr_associations: HashMap<String, String>,
    #[serde(default)]
    pub ebs_encryption_by_default: bool,
    #[serde(default)]
    pub transit_gateways: HashMap<String, TransitGatewayView>,
    #[serde(default)]
    pub tgw_vpc_attachments: HashMap<String, TgwVpcAttachmentView>,
    #[serde(default)]
    pub tgw_peering_attachments: HashMap<String, TgwPeeringAttachmentView>,
    #[serde(default)]
    pub tgw_route_tables: HashMap<String, TgwRouteTableView>,
    #[serde(default)]
    pub tgw_routes: HashMap<String, Vec<TgwRouteView>>,
    #[serde(default)]
    pub instances: HashMap<String, InstanceView>,
    #[serde(default)]
    pub volumes: HashMap<String, VolumeView>,
    #[serde(default)]
    pub snapshots: HashMap<String, SnapshotView>,
    #[serde(default)]
    pub images: HashMap<String, ImageView>,
    #[serde(default)]
    pub launch_templates: HashMap<String, LaunchTemplateView>,
    #[serde(default)]
    pub launch_template_versions: HashMap<String, Vec<LaunchTemplateVersionView>>,
    #[serde(default)]
    pub spot_requests: HashMap<String, SpotInstanceRequestView>,
    #[serde(default)]
    pub iam_instance_profile_associations: HashMap<String, IamInstanceProfileAssociationView>,
    #[serde(default)]
    pub dedicated_hosts: HashMap<String, DedicatedHostView>,
    #[serde(default)]
    pub ec2_fleets: HashMap<String, Ec2FleetView>,
    #[serde(default)]
    pub vpc_endpoint_service_configs: HashMap<String, VpcEndpointServiceConfigView>,
    #[serde(default)]
    pub spot_fleet_requests: HashMap<String, SpotFleetRequestView>,
    #[serde(default)]
    pub subnet_cidr_reservations: HashMap<String, SubnetCidrReservationView>,
    #[serde(default)]
    pub placement_groups: HashMap<String, PlacementGroupView>,
    #[serde(default)]
    pub network_interface_permissions: HashMap<String, NetworkInterfacePermissionView>,
    #[serde(default)]
    pub instance_connect_endpoints: HashMap<String, InstanceConnectEndpointView>,
    #[serde(default)]
    pub capacity_reservations: HashMap<String, CapacityReservationView>,
    #[serde(default)]
    pub capacity_reservation_fleets: HashMap<String, CapacityReservationFleetView>,
    #[serde(default)]
    pub coip_pools: HashMap<String, CoipPoolView>,
    #[serde(default)]
    pub byoip_cidrs: HashMap<String, ByoipCidrView>,
    #[serde(default)]
    pub public_ipv4_pools: HashMap<String, PublicIpv4PoolView>,
    #[serde(default)]
    pub coip_cidrs: Vec<CoipCidrView>,
    #[serde(default)]
    pub address_transfers: HashMap<String, AddressTransferView>,
    #[serde(default)]
    pub security_group_vpc_associations: Vec<SecurityGroupVpcAssociationView>,
    #[serde(default)]
    pub enclave_certificate_iam_role_associations: Vec<EnclaveCertificateIamRoleAssociationView>,
    #[serde(default)]
    pub mac_sip_modification_tasks: HashMap<String, MacSipModificationTaskView>,
    #[serde(default)]
    pub declarative_policies_reports: HashMap<String, DeclarativePoliciesReportView>,
    #[serde(default)]
    pub vpn_concentrators: HashMap<String, VpnConcentratorView>,
    #[serde(default)]
    pub vpc_endpoint_connections: Vec<VpcEndpointConnectionView>,
    #[serde(default)]
    pub vpc_endpoint_connection_notifications:
        HashMap<String, VpcEndpointConnectionNotificationView>,
    #[serde(default)]
    pub vpc_block_public_access_exclusions: HashMap<String, VpcBlockPublicAccessExclusionView>,
    #[serde(default)]
    pub vpc_block_public_access_options: Option<VpcBlockPublicAccessOptionsView>,
    #[serde(default)]
    pub vpc_encryption_controls: HashMap<String, VpcEncryptionControlView>,
    #[serde(default)]
    pub mac_volume_ownership_tasks: HashMap<String, MacVolumeOwnershipTaskView>,
    #[serde(default)]
    pub replace_root_volume_tasks: HashMap<String, ReplaceRootVolumeTaskView>,
    #[serde(default)]
    pub snapshot_import_tasks: HashMap<String, SnapshotImportTaskView>,
    #[serde(default)]
    pub conversion_tasks: HashMap<String, ConversionTaskView>,
    #[serde(default)]
    pub export_tasks: HashMap<String, ExportTaskView>,
    #[serde(default)]
    pub import_tasks: HashMap<String, (String, Option<String>)>,
    #[serde(default)]
    pub trunk_interface_associations: HashMap<String, TrunkInterfaceAssociationView>,
    #[serde(default)]
    pub secondary_networks: HashMap<String, SecondaryNetworkView>,
    #[serde(default)]
    pub secondary_subnets: HashMap<String, SecondarySubnetView>,
    #[serde(default)]
    pub deleted_volumes_recycle_bin: HashMap<String, VolumeView>,
    #[serde(default)]
    pub deleted_snapshots_recycle_bin: HashMap<String, SnapshotView>,
    // --- Group 5 additions ---
    #[serde(default)]
    pub reserved_instances_exchanges: HashMap<String, ReservedInstancesExchangeView>,
    #[serde(default)]
    pub reserved_instances_listings: HashMap<String, ReservedInstancesListingView>,
    #[serde(default)]
    pub queued_reserved_instances_purchases: HashMap<String, ReservedInstancesPurchaseView>,
    #[serde(default)]
    pub reserved_instances_modifications: HashMap<String, ReservedInstancesModificationView>,
    #[serde(default)]
    pub reserved_instances_purchases: HashMap<String, ReservedInstancesPurchaseView>,
    #[serde(default)]
    pub reserved_instances: HashMap<String, ReservedInstancesRecordView>,
    #[serde(default)]
    pub fpga_images: HashMap<String, FpgaImageView>,
    #[serde(default)]
    pub image_usage_reports: HashMap<String, ImageUsageReportView>,
    #[serde(default)]
    pub restore_image_tasks: HashMap<String, RestoreImageTaskView>,
    #[serde(default)]
    pub store_image_tasks: HashMap<String, StoreImageTaskView>,
    #[serde(default)]
    pub import_image_tasks: HashMap<String, ImportImageTaskView>,
    #[serde(default)]
    pub allowed_image_criteria: Vec<AllowedImageCriterionView>,
    #[serde(default)]
    pub default_credit_specifications: HashMap<String, String>,
    #[serde(default)]
    pub instance_metadata_defaults: Option<InstanceMetadataDefaultsView>,
    #[serde(default)]
    pub instance_event_windows: HashMap<String, InstanceEventWindowView>,
    #[serde(default)]
    pub instance_event_notification_attributes: Option<InstanceTagNotificationAttributesView>,
    #[serde(default)]
    pub instance_events: HashMap<String, InstanceEventView>,
    #[serde(default)]
    pub host_reservations: HashMap<String, HostReservationView>,
    #[serde(default)]
    pub scheduled_instances: HashMap<String, ScheduledInstanceView>,
    #[serde(default)]
    pub az_group_opt_in: HashMap<String, String>,
    #[serde(default)]
    pub instance_status_reports: Vec<InstanceStatusReportView>,
    // --- Group 6 additions ---
    #[serde(default)]
    pub network_insights_access_scopes: HashMap<String, NetworkInsightsAccessScopeView>,
    #[serde(default)]
    pub network_insights_access_scope_analyses:
        HashMap<String, NetworkInsightsAccessScopeAnalysisView>,
    #[serde(default)]
    pub network_insights_paths: HashMap<String, NetworkInsightsPathView>,
    #[serde(default)]
    pub network_insights_analyses: HashMap<String, NetworkInsightsAnalysisView>,
    #[serde(default)]
    pub traffic_mirror_filters: HashMap<String, TrafficMirrorFilterView>,
    #[serde(default)]
    pub traffic_mirror_sessions: HashMap<String, TrafficMirrorSessionView>,
    #[serde(default)]
    pub traffic_mirror_targets: HashMap<String, TrafficMirrorTargetView>,
    // --- Group 7 additions ---
    #[serde(default)]
    pub client_vpn_endpoints: HashMap<String, ClientVpnEndpointView>,
    #[serde(default)]
    pub client_vpn_target_network_associations:
        HashMap<String, ClientVpnTargetNetworkAssociationView>,
    #[serde(default)]
    pub client_vpn_authorization_rules: Vec<ClientVpnAuthorizationRuleView>,
    #[serde(default)]
    pub client_vpn_routes: Vec<ClientVpnRouteView>,
    #[serde(default)]
    pub client_vpn_connections: HashMap<String, ClientVpnConnectionView>,
    #[serde(default)]
    pub local_gateways: HashMap<String, LocalGatewayView>,
    #[serde(default)]
    pub local_gateway_route_tables: HashMap<String, LocalGatewayRouteTableView>,
    #[serde(default)]
    pub local_gateway_routes: Vec<LocalGatewayRouteView>,
    #[serde(default)]
    pub local_gateway_route_table_virtual_interface_group_associations:
        HashMap<String, LocalGatewayRouteTableVirtualInterfaceGroupAssociationView>,
    #[serde(default)]
    pub local_gateway_route_table_vpc_associations:
        HashMap<String, LocalGatewayRouteTableVpcAssociationView>,
    #[serde(default)]
    pub local_gateway_virtual_interfaces: HashMap<String, LocalGatewayVirtualInterfaceView>,
    #[serde(default)]
    pub local_gateway_virtual_interface_groups:
        HashMap<String, LocalGatewayVirtualInterfaceGroupView>,
    // --- Group 8 additions ---
    #[serde(default)]
    pub route_servers: HashMap<String, RouteServerView>,
    #[serde(default)]
    pub route_server_endpoints: HashMap<String, RouteServerEndpointView>,
    #[serde(default)]
    pub route_server_peers: HashMap<String, RouteServerPeerView>,
    #[serde(default)]
    pub route_server_associations: Vec<RouteServerAssociationView>,
    // --- Group 9 additions ---
    #[serde(default)]
    pub verified_access_instances: HashMap<String, VerifiedAccessInstanceView>,
    #[serde(default)]
    pub verified_access_trust_providers: HashMap<String, VerifiedAccessTrustProviderView>,
    #[serde(default)]
    pub verified_access_groups: HashMap<String, VerifiedAccessGroupView>,
    #[serde(default)]
    pub verified_access_endpoints: HashMap<String, VerifiedAccessEndpointView>,
    #[serde(default)]
    pub verified_access_trust_provider_attachments: Vec<VerifiedAccessTrustProviderAttachmentView>,
    #[serde(default)]
    pub verified_access_instance_logging_configurations: HashMap<String, VerifiedAccessLogsView>,
    // --- Group 10 additions ---
    #[serde(default)]
    pub billing_ownership_offers: Vec<BillingOwnershipOfferView>,
    #[serde(default)]
    pub capacity_manager_data_exports: HashMap<String, CapacityManagerDataExportView>,
    #[serde(default)]
    pub interruptible_capacity_reservation_allocations:
        HashMap<String, InterruptibleCapacityReservationAllocationView>,
    #[serde(default)]
    pub capacity_blocks: HashMap<String, CapacityBlockView>,
    #[serde(default)]
    pub capacity_block_extensions: HashMap<String, CapacityBlockExtensionView>,
    #[serde(default)]
    pub capacity_manager_organizations_access: Option<CapacityManagerOrganizationsAccessView>,
    // --- Group 11 additions ---
    #[serde(default)]
    pub tgw_multicast_domains: HashMap<String, TransitGatewayMulticastDomainView>,
    #[serde(default)]
    pub tgw_multicast_domain_associations: Vec<TransitGatewayMulticastDomainAssociationView>,
    #[serde(default)]
    pub tgw_multicast_group_members: Vec<TransitGatewayMulticastGroupMemberView>,
    #[serde(default)]
    pub tgw_multicast_group_sources: Vec<TransitGatewayMulticastGroupSourceView>,
    #[serde(default)]
    pub tgw_connects: HashMap<String, TransitGatewayConnectView>,
    #[serde(default)]
    pub tgw_connect_peers: HashMap<String, TransitGatewayConnectPeerView>,
    #[serde(default)]
    pub tgw_metering_policies: HashMap<String, TransitGatewayMeteringPolicyView>,
    #[serde(default)]
    pub tgw_metering_policy_entries: Vec<TransitGatewayMeteringPolicyEntryView>,
    #[serde(default)]
    pub tgw_policy_tables: HashMap<String, TransitGatewayPolicyTableView>,
    #[serde(default)]
    pub tgw_policy_table_associations: Vec<TransitGatewayPolicyTableAssociationView>,
    #[serde(default)]
    pub tgw_prefix_list_references: Vec<TransitGatewayPrefixListReferenceView>,
    #[serde(default)]
    pub tgw_route_table_announcements: HashMap<String, TransitGatewayRouteTableAnnouncementView>,
    // --- Group 12 additions ---
    #[serde(default)]
    pub ipams: HashMap<String, IpamView>,
    #[serde(default)]
    pub ipam_scopes: HashMap<String, IpamScopeView>,
    #[serde(default)]
    pub ipam_pools: HashMap<String, IpamPoolView>,
    #[serde(default)]
    pub ipam_pool_cidrs: Vec<IpamPoolCidrView>,
    #[serde(default)]
    pub ipam_pool_allocations: Vec<IpamPoolAllocationView>,
    #[serde(default)]
    pub ipam_resource_discoveries: HashMap<String, IpamResourceDiscoveryView>,
    #[serde(default)]
    pub ipam_resource_discovery_associations: HashMap<String, IpamResourceDiscoveryAssociationView>,
    #[serde(default)]
    pub ipam_byoasns: Vec<IpamByoasnView>,
    #[serde(default)]
    pub ipam_external_resource_verification_tokens:
        HashMap<String, IpamExternalResourceVerificationTokenView>,
    #[serde(default)]
    pub ipam_policies: HashMap<String, IpamPolicyView>,
    #[serde(default)]
    pub ipam_prefix_list_resolvers: HashMap<String, IpamPrefixListResolverView>,
    #[serde(default)]
    pub ipam_prefix_list_resolver_targets: Vec<IpamPrefixListResolverTargetView>,
    // --- Batch B additions ---
    #[serde(default)]
    pub volume_modifications: HashMap<String, VolumeModificationView>,
    #[serde(default)]
    pub import_volume_tasks: HashMap<String, ImportVolumeTaskView>,
    #[serde(default)]
    pub bundle_tasks: HashMap<String, BundleTaskView>,
    #[serde(default)]
    pub id_format: HashMap<String, IdFormatEntryView>,
    #[serde(default)]
    pub outpost_lags: HashMap<String, OutpostLagView>,
    #[serde(default)]
    pub export_image_tasks: HashMap<String, ExportImageTaskView>,
    // --- Wave 4 additions ---
    #[serde(default)]
    pub ebs_default_kms_key_id: Option<String>,
    #[serde(default)]
    pub serial_console_access_enabled: bool,
    #[serde(default)]
    pub allowed_images_settings_state: Option<String>,
    #[serde(default)]
    pub image_block_public_access_state: Option<String>,
    #[serde(default)]
    pub aws_network_performance_subscriptions: Vec<AwsNetworkPerformanceSubscriptionView>,
    #[serde(default)]
    pub counters: CountersView,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwsNetworkPerformanceSubscriptionView {
    pub source: String,
    pub destination: String,
    pub metric: String,
    pub statistic: String,
    pub period: String,
}

// ---------------------------------------------------------------------------
// Existing view structs (unchanged)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcView {
    pub vpc_id: String,
    pub cidr_block: String,
    pub state: String,
    pub dhcp_options_id: String,
    pub instance_tenancy: String,
    pub is_default: bool,
    pub enable_dns_hostnames: bool,
    pub enable_dns_support: bool,
    pub secondary_cidr_blocks: Vec<(String, String)>,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub classic_link_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetView {
    pub subnet_id: String,
    pub vpc_id: String,
    pub cidr_block: String,
    pub availability_zone: String,
    pub state: String,
    pub available_ip_address_count: i64,
    pub map_public_ip_on_launch: bool,
    pub ipv6_cidr_blocks: Vec<SubnetIpv6CidrAssocView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetIpv6CidrAssocView {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetGatewayView {
    pub igw_id: String,
    pub attachments: Vec<IgwAttachmentView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgwAttachmentView {
    pub vpc_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGroupView {
    pub group_id: String,
    pub group_name: String,
    pub description: String,
    pub vpc_id: String,
    pub owner_id: String,
    pub ingress_rules: Vec<IpPermissionView>,
    pub egress_rules: Vec<IpPermissionView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpPermissionView {
    pub from_port: Option<i64>,
    pub to_port: Option<i64>,
    pub ip_protocol: String,
    pub ip_ranges: Vec<IpRangeView>,
    pub ipv6_ranges: Vec<Ipv6RangeView>,
    pub user_id_group_pairs: Vec<UserIdGroupPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRangeView {
    pub cidr_ip: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv6RangeView {
    pub cidr_ipv6: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdGroupPairView {
    pub group_id: String,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTableView {
    pub route_table_id: String,
    pub vpc_id: String,
    pub routes: Vec<RouteView>,
    pub associations: Vec<RouteTableAssociationView>,
    #[serde(default)]
    pub propagating_vgws: Vec<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteView {
    pub destination_cidr_block: Option<String>,
    pub destination_ipv6_cidr_block: Option<String>,
    pub gateway_id: Option<String>,
    pub state: String,
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTableAssociationView {
    pub association_id: String,
    pub subnet_id: Option<String>,
    #[serde(default)]
    pub gateway_id: Option<String>,
    pub main: bool,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPairView {
    pub key_pair_id: String,
    pub key_name: String,
    pub fingerprint: String,
    pub tags: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// New view structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAclView {
    pub network_acl_id: String,
    pub vpc_id: String,
    pub is_default: bool,
    pub entries: Vec<NetworkAclEntryView>,
    pub associations: Vec<NetworkAclAssociationView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAclEntryView {
    pub rule_number: i32,
    pub protocol: String,
    pub rule_action: String,
    pub egress: bool,
    pub cidr_block: Option<String>,
    pub ipv6_cidr_block: Option<String>,
    pub port_range: Option<PortRangeView>,
    pub icmp_type_code: Option<IcmpTypeCodeView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRangeView {
    pub from: i32,
    pub to: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcmpTypeCodeView {
    pub type_num: i32,
    pub code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAclAssociationView {
    pub network_acl_association_id: String,
    pub network_acl_id: String,
    pub subnet_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticIpView {
    pub allocation_id: String,
    pub public_ip: String,
    pub association_id: Option<String>,
    pub instance_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub private_ip_address: Option<String>,
    #[serde(default)]
    pub address_attribute_ptr_record: Option<String>,
    #[serde(default = "default_eip_domain")]
    pub domain: String,
    #[serde(default)]
    pub pending_transfer: Option<String>,
    pub tags: HashMap<String, String>,
}

fn default_eip_domain() -> String {
    "vpc".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatGatewayView {
    pub nat_gateway_id: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub state: String,
    pub connectivity_type: String,
    pub allocation_id: Option<String>,
    pub public_ip: Option<String>,
    #[serde(default)]
    pub secondary_addresses: Vec<NatGatewayAddressAssociationView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatGatewayAddressAssociationView {
    #[serde(default)]
    pub allocation_id: Option<String>,
    #[serde(default)]
    pub association_id: Option<String>,
    #[serde(default)]
    pub network_interface_id: Option<String>,
    #[serde(default)]
    pub private_ip: Option<String>,
    #[serde(default)]
    pub public_ip: Option<String>,
    pub status: String,
    #[serde(default)]
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByoipCidrView {
    pub cidr: String,
    #[serde(default)]
    pub description: Option<String>,
    pub state: String,
    #[serde(default)]
    pub asn_association: Option<AsnAssociationView>,
    #[serde(default)]
    pub ipam_pool_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsnAssociationView {
    pub asn: String,
    pub cidr: String,
    pub state: String,
    #[serde(default)]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIpv4PoolView {
    pub pool_id: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub network_border_group: Option<String>,
    pub total_address_count: i32,
    pub total_available_address_count: i32,
    #[serde(default)]
    pub pool_address_ranges: Vec<PublicIpv4PoolRangeView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIpv4PoolRangeView {
    pub first_address: String,
    pub last_address: String,
    pub address_count: i32,
    pub available_address_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoipCidrView {
    pub cidr: String,
    pub coip_pool_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressTransferView {
    pub allocation_id: String,
    pub public_ip: String,
    pub transfer_account_id: String,
    pub transfer_offer_expiration_timestamp: String,
    #[serde(default)]
    pub transfer_offer_accepted_timestamp: Option<String>,
    pub address_transfer_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpOptionsView {
    pub dhcp_options_id: String,
    pub configurations: Vec<DhcpConfigurationView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpConfigurationView {
    pub key: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgressOnlyIgwView {
    pub eigw_id: String,
    pub state: String,
    pub attachments: Vec<EoigwAttachmentView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EoigwAttachmentView {
    pub vpc_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowLogView {
    pub flow_log_id: String,
    pub resource_id: String,
    pub traffic_type: String,
    pub log_destination_type: String,
    pub log_destination: Option<String>,
    pub log_group_name: Option<String>,
    pub deliver_logs_status: String,
    pub flow_log_status: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcPeeringConnectionView {
    pub peering_id: String,
    pub requester_vpc_id: String,
    pub accepter_vpc_id: Option<String>,
    pub status: String,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub requester_peering_options: Option<VpcPeeringConnectionOptionsView>,
    #[serde(default)]
    pub accepter_peering_options: Option<VpcPeeringConnectionOptionsView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpcPeeringConnectionOptionsView {
    #[serde(default)]
    pub allow_dns_resolution_from_remote_vpc: bool,
    #[serde(default)]
    pub allow_egress_from_local_classic_link_to_remote_vpc: bool,
    #[serde(default)]
    pub allow_egress_from_local_vpc_to_remote_classic_link: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEndpointView {
    pub endpoint_id: String,
    pub vpc_id: String,
    pub service_name: String,
    pub endpoint_type: String,
    pub state: String,
    pub policy_document: Option<String>,
    pub route_table_ids: Vec<String>,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    #[serde(default)]
    pub private_dns_enabled: Option<bool>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedPrefixListView {
    pub prefix_list_id: String,
    pub prefix_list_name: String,
    pub state: String,
    pub address_family: String,
    pub max_entries: i32,
    pub entries: Vec<PrefixListEntryView>,
    pub tags: HashMap<String, String>,
    pub version: i64,
    #[serde(default)]
    pub version_history: Vec<ManagedPrefixListVersionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedPrefixListVersionView {
    pub version: i64,
    pub entries: Vec<PrefixListEntryView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixListEntryView {
    pub cidr: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGatewayView {
    pub customer_gateway_id: String,
    pub bgp_asn: String,
    pub ip_address: String,
    pub gateway_type: String,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnGatewayView {
    pub vpn_gateway_id: String,
    pub gateway_type: String,
    pub state: String,
    pub amazon_side_asn: Option<i64>,
    pub vpc_attachments: Vec<VgwVpcAttachmentView>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VgwVpcAttachmentView {
    pub vpc_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConnectionView {
    pub vpn_connection_id: String,
    pub vpn_gateway_id: String,
    pub customer_gateway_id: String,
    #[serde(default)]
    pub transit_gateway_id: Option<String>,
    pub connection_type: String,
    pub state: String,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub routes: Vec<VpnStaticRouteView>,
    #[serde(default)]
    pub options: Option<VpnConnectionOptionsView>,
    #[serde(default)]
    pub tunnel_replacement_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnStaticRouteView {
    pub destination_cidr_block: String,
    pub source: String,
    pub state: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpnConnectionOptionsView {
    #[serde(default)]
    pub local_ipv4_network_cidr: Option<String>,
    #[serde(default)]
    pub local_ipv6_network_cidr: Option<String>,
    #[serde(default)]
    pub remote_ipv4_network_cidr: Option<String>,
    #[serde(default)]
    pub remote_ipv6_network_cidr: Option<String>,
    #[serde(default)]
    pub tunnel_inside_ip_version: Option<String>,
    #[serde(default)]
    pub static_routes_only: Option<bool>,
    #[serde(default)]
    pub tunnel_options: Vec<VpnTunnelOptionsView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpnTunnelOptionsView {
    #[serde(default)]
    pub tunnel_inside_cidr: Option<String>,
    #[serde(default)]
    pub tunnel_inside_ipv6_cidr: Option<String>,
    #[serde(default)]
    pub pre_shared_key: Option<String>,
    #[serde(default)]
    pub outside_ip_address: Option<String>,
    #[serde(default)]
    pub certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConcentratorView {
    pub vpn_concentrator_id: String,
    pub concentrator_type: String,
    pub state: String,
    #[serde(default)]
    pub transit_gateway_id: Option<String>,
    #[serde(default)]
    pub transit_gateway_attachment_id: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEndpointConnectionView {
    pub service_id: String,
    pub vpc_endpoint_id: String,
    pub vpc_endpoint_owner: String,
    pub vpc_endpoint_state: String,
    pub creation_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEndpointConnectionNotificationView {
    pub connection_notification_id: String,
    pub connection_notification_arn: String,
    #[serde(default)]
    pub connection_events: Vec<String>,
    pub connection_notification_state: String,
    pub connection_notification_type: String,
    #[serde(default)]
    pub service_id: Option<String>,
    #[serde(default)]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcBlockPublicAccessExclusionView {
    pub exclusion_id: String,
    pub internet_gateway_exclusion_mode: String,
    pub resource_arn: String,
    pub state: String,
    pub creation_timestamp: String,
    pub last_update_timestamp: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcBlockPublicAccessOptionsView {
    pub aws_account_id: String,
    pub aws_region: String,
    pub internet_gateway_block_mode: String,
    pub state: String,
    pub last_update_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEncryptionControlView {
    pub vpc_encryption_control_id: String,
    pub vpc_id: String,
    pub mode: String,
    pub state: String,
    #[serde(default)]
    pub mode_history: Vec<(String, String)>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarrierGatewayView {
    pub carrier_gateway_id: String,
    pub vpc_id: String,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceView {
    pub network_interface_id: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub description: String,
    pub private_ip_address: String,
    pub status: String,
    pub attachment_id: Option<String>,
    pub instance_id: Option<String>,
    pub device_index: Option<i32>,
    pub security_groups: Vec<String>,
    #[serde(default = "default_true")]
    pub source_dest_check: bool,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub public_ip_dns_hostname_type: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayView {
    pub transit_gateway_id: String,
    pub state: String,
    pub amazon_side_asn: i64,
    pub description: String,
    pub dns_support: String,
    pub vpn_ecmp_support: String,
    pub multicast_support: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TgwVpcAttachmentView {
    pub attachment_id: String,
    pub transit_gateway_id: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TgwPeeringAttachmentView {
    pub attachment_id: String,
    pub transit_gateway_id: String,
    pub peer_transit_gateway_id: String,
    pub peer_account_id: String,
    pub peer_region: String,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TgwRouteTableView {
    pub route_table_id: String,
    pub transit_gateway_id: String,
    pub state: String,
    pub default_association_route_table: bool,
    pub default_propagation_route_table: bool,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TgwRouteView {
    pub destination_cidr_block: String,
    pub route_type: String,
    pub state: String,
    pub attachment_id: Option<String>,
}

// --- Group 11: Transit Gateway extension view types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayMulticastDomainView {
    pub transit_gateway_multicast_domain_id: String,
    pub transit_gateway_id: String,
    pub transit_gateway_multicast_domain_arn: String,
    pub owner_id: String,
    pub igmpv2_support: String,
    pub static_sources_support: String,
    pub auto_accept_shared_associations: String,
    pub state: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MulticastSubnetAssociationView {
    pub subnet_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayMulticastDomainAssociationView {
    pub transit_gateway_multicast_domain_id: String,
    pub transit_gateway_attachment_id: String,
    pub resource_id: String,
    pub resource_type: String,
    #[serde(default)]
    pub subnets: Vec<MulticastSubnetAssociationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayMulticastGroupMemberView {
    pub transit_gateway_multicast_domain_id: String,
    pub group_ip_address: String,
    pub transit_gateway_attachment_id: Option<String>,
    pub subnet_id: Option<String>,
    pub resource_id: Option<String>,
    pub resource_type: String,
    pub network_interface_id: String,
    pub member_type: String,
    pub source_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayMulticastGroupSourceView {
    pub transit_gateway_multicast_domain_id: String,
    pub group_ip_address: String,
    pub transit_gateway_attachment_id: Option<String>,
    pub subnet_id: Option<String>,
    pub resource_id: Option<String>,
    pub resource_type: String,
    pub network_interface_id: String,
    pub member_type: String,
    pub source_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayConnectView {
    pub transit_gateway_attachment_id: String,
    pub transport_transit_gateway_attachment_id: String,
    pub transit_gateway_id: String,
    pub state: String,
    pub creation_time: String,
    pub protocol: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayAttachmentBgpConfigurationView {
    pub transit_gateway_asn: i64,
    pub peer_asn: i64,
    pub transit_gateway_address: String,
    pub peer_address: String,
    pub bgp_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayConnectPeerView {
    pub transit_gateway_attachment_id: String,
    pub transit_gateway_connect_peer_id: String,
    pub state: String,
    pub creation_time: String,
    pub transit_gateway_address: String,
    pub peer_address: String,
    #[serde(default)]
    pub inside_cidr_blocks: Vec<String>,
    pub protocol: String,
    #[serde(default)]
    pub bgp_configurations: Vec<TransitGatewayAttachmentBgpConfigurationView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayMeteringPolicyView {
    pub transit_gateway_metering_policy_id: String,
    pub transit_gateway_metering_policy_arn: String,
    pub transit_gateway_id: String,
    pub name: String,
    pub description: Option<String>,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub last_updated_time: String,
    pub version: i32,
    #[serde(default)]
    pub middlebox_attachment_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayMeteringPolicyEntryView {
    pub transit_gateway_metering_policy_entry_id: String,
    pub transit_gateway_metering_policy_id: String,
    pub sequence_number: i32,
    pub action: String,
    pub source_cidr_block: Option<String>,
    pub destination_cidr_block: Option<String>,
    pub protocol: Option<String>,
    pub source_port: Option<String>,
    pub destination_port: Option<String>,
    #[serde(default)]
    pub dimensions: Vec<String>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayPolicyTableView {
    pub transit_gateway_policy_table_id: String,
    pub transit_gateway_id: String,
    pub state: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayPolicyTableAssociationView {
    pub transit_gateway_policy_table_id: String,
    pub transit_gateway_attachment_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayPrefixListReferenceView {
    pub transit_gateway_route_table_id: String,
    pub prefix_list_id: String,
    pub prefix_list_owner_id: String,
    pub state: String,
    pub blackhole: bool,
    pub transit_gateway_attachment_id: Option<String>,
    pub resource_id: Option<String>,
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayRouteTableAnnouncementView {
    pub transit_gateway_route_table_announcement_id: String,
    pub transit_gateway_id: String,
    pub core_network_id: String,
    pub peer_transit_gateway_id: String,
    pub peer_core_network_id: Option<String>,
    pub peering_attachment_id: String,
    pub announcement_direction: String,
    pub transit_gateway_route_table_id: String,
    pub state: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStateView {
    pub code: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceView {
    pub instance_id: String,
    pub image_id: String,
    pub instance_type: String,
    pub state: InstanceStateView,
    pub private_ip_address: Option<String>,
    pub public_ip_address: Option<String>,
    pub subnet_id: Option<String>,
    pub vpc_id: Option<String>,
    pub key_name: Option<String>,
    pub security_groups: Vec<String>,
    pub launch_time: String,
    pub tags: HashMap<String, String>,
    pub iam_instance_profile_arn: Option<String>,
    pub monitoring_state: String,
    pub placement_az: String,
    #[serde(default)]
    pub placement_group_name: Option<String>,
    #[serde(default)]
    pub placement_tenancy: Option<String>,
    #[serde(default)]
    pub placement_host_id: Option<String>,
    #[serde(default)]
    pub placement_affinity: Option<String>,
    #[serde(default)]
    pub placement_partition_number: Option<i32>,
    pub owner_id: String,
    #[serde(default)]
    pub classic_link_vpc: Option<(String, Vec<String>)>,
    #[serde(default)]
    pub private_dns_hostname_type: Option<String>,
    #[serde(default)]
    pub enable_resource_name_dns_a_record: Option<bool>,
    #[serde(default)]
    pub enable_resource_name_dns_aaaa_record: Option<bool>,
    #[serde(default)]
    pub credit_specification: Option<String>,
    #[serde(default)]
    pub cpu_options: Option<InstanceCpuOptionsView>,
    #[serde(default)]
    pub maintenance_options: Option<InstanceMaintenanceOptionsView>,
    #[serde(default)]
    pub network_bandwidth_weighting: Option<String>,
    #[serde(default)]
    pub lifecycle: Option<String>,
    #[serde(default)]
    pub product_codes: Vec<(String, String)>,
    #[serde(default)]
    pub capacity_reservation_specification: Option<CapacityReservationSpecificationResponseView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceCpuOptionsView {
    #[serde(default)]
    pub core_count: Option<i32>,
    #[serde(default)]
    pub threads_per_core: Option<i32>,
    #[serde(default)]
    pub amd_sev_snp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceMaintenanceOptionsView {
    #[serde(default)]
    pub auto_recovery: Option<String>,
    #[serde(default)]
    pub reboot_migration: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceMetadataDefaultsView {
    #[serde(default)]
    pub http_tokens: Option<String>,
    #[serde(default)]
    pub http_put_response_hop_limit: Option<i32>,
    #[serde(default)]
    pub http_endpoint: Option<String>,
    #[serde(default)]
    pub instance_metadata_tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeAttachmentView {
    pub volume_id: String,
    pub instance_id: String,
    pub device: String,
    pub state: String,
    pub attach_time: String,
    pub delete_on_termination: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeView {
    pub volume_id: String,
    pub size: i32,
    pub snapshot_id: Option<String>,
    pub availability_zone: String,
    pub state: String,
    pub volume_type: String,
    pub iops: Option<i32>,
    pub throughput: Option<i32>,
    pub encrypted: bool,
    pub create_time: String,
    pub attachments: Vec<VolumeAttachmentView>,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub recycle_bin_state: Option<String>,
    #[serde(default)]
    pub source_volume_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotView {
    pub snapshot_id: String,
    pub volume_id: String,
    pub volume_size: i32,
    pub state: String,
    pub description: String,
    pub start_time: String,
    pub progress: String,
    pub owner_id: String,
    pub encrypted: bool,
    pub tags: HashMap<String, String>,
    #[serde(default = "default_lock_state")]
    pub lock_state: String,
    #[serde(default)]
    pub lock_duration: Option<i32>,
    #[serde(default)]
    pub lock_created_on: Option<String>,
    #[serde(default)]
    pub lock_expires_on: Option<String>,
    #[serde(default)]
    pub lock_duration_start_time: Option<String>,
    #[serde(default)]
    pub cool_off_period: Option<i32>,
    #[serde(default)]
    pub cool_off_period_expires_on: Option<String>,
    #[serde(default = "default_storage_tier")]
    pub storage_tier: String,
    #[serde(default)]
    pub last_tiering_operation_status: Option<String>,
    #[serde(default)]
    pub fast_snapshot_restore_states: Vec<FastSnapshotRestoreStateView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FastSnapshotRestoreStateView {
    pub availability_zone: String,
    pub state: String,
}

fn default_lock_state() -> String {
    "none".to_string()
}

fn default_storage_tier() -> String {
    "standard".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageView {
    pub image_id: String,
    pub name: String,
    pub description: String,
    pub state: String,
    pub owner_id: String,
    pub architecture: String,
    pub image_type: String,
    pub platform: Option<String>,
    pub virtualization_type: String,
    pub root_device_type: String,
    pub root_device_name: String,
    pub public: bool,
    pub tags: HashMap<String, String>,
    pub source_instance_id: Option<String>,
    pub source_instance_type: String,
    #[serde(default)]
    pub launch_permissions: Vec<LaunchPermissionView>,
    #[serde(default)]
    pub recycle_bin_state: Option<String>,
    #[serde(default)]
    pub deprecation_time: Option<String>,
    #[serde(default)]
    pub recycle_bin_enter_time: Option<String>,
    #[serde(default)]
    pub product_codes: Vec<(String, String)>,
    #[serde(default)]
    pub fast_launch_state: Option<FastLaunchStateView>,
    #[serde(default)]
    pub deregistration_protection: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FastLaunchStateView {
    pub state: String,
    pub image_id: String,
    pub resource_type: String,
    pub snapshot_configuration: SnapshotConfigurationRequestView,
    pub launch_template: FastLaunchLaunchTemplateSpecificationView,
    pub max_parallel_launches: i32,
    pub owner_id: String,
    pub state_transition_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnapshotConfigurationRequestView {
    #[serde(default)]
    pub target_resource_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FastLaunchLaunchTemplateSpecificationView {
    #[serde(default)]
    pub launch_template_id: Option<String>,
    #[serde(default)]
    pub launch_template_name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchPermissionView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchTemplateView {
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub default_version_number: i64,
    pub latest_version_number: i64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchTemplateVersionView {
    pub version_number: i64,
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub version_description: String,
    pub data: HashMap<String, String>,
    pub default_version: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotInstanceRequestView {
    pub spot_instance_request_id: String,
    pub spot_price: String,
    pub instance_type: String,
    pub image_id: String,
    pub state: String,
    pub status_code: String,
    pub instance_id: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IamInstanceProfileAssociationView {
    pub association_id: String,
    pub instance_id: String,
    pub iam_instance_profile_arn: String,
    pub iam_instance_profile_name: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedicatedHostView {
    pub host_id: String,
    pub availability_zone: String,
    pub instance_type: Option<String>,
    pub auto_placement: String,
    pub host_recovery: String,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ec2FleetView {
    pub fleet_id: String,
    pub state: String,
    pub fleet_type: String,
    pub create_time: String,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub total_target_capacity: Option<i32>,
    #[serde(default)]
    pub on_demand_target_capacity: Option<i32>,
    #[serde(default)]
    pub spot_target_capacity: Option<i32>,
    #[serde(default)]
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEndpointServiceConfigView {
    pub service_id: String,
    pub service_name: String,
    pub service_type: String,
    pub acceptance_required: bool,
    pub state: String,
    pub network_load_balancer_arns: Vec<String>,
    pub gateway_load_balancer_arns: Vec<String>,
    pub allowed_principals: Vec<String>,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub payer_responsibility: Option<String>,
    #[serde(default)]
    pub private_dns_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotFleetRequestView {
    pub spot_fleet_request_id: String,
    pub spot_fleet_request_state: String,
    pub target_capacity: i32,
    pub iam_fleet_role: String,
    pub create_time: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetCidrReservationView {
    pub reservation_id: String,
    pub subnet_id: String,
    pub cidr: String,
    pub reservation_type: String,
    pub description: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementGroupView {
    pub group_id: String,
    pub group_name: String,
    pub group_arn: String,
    pub strategy: String,
    pub state: String,
    #[serde(default)]
    pub partition_count: Option<i32>,
    #[serde(default)]
    pub spread_level: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfacePermissionView {
    pub network_interface_permission_id: String,
    pub network_interface_id: String,
    #[serde(default)]
    pub aws_account_id: Option<String>,
    #[serde(default)]
    pub aws_service: Option<String>,
    pub permission: String,
    pub permission_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReservationView {
    pub capacity_reservation_id: String,
    pub capacity_reservation_arn: String,
    pub owner_id: String,
    pub instance_type: String,
    pub instance_platform: String,
    pub availability_zone: String,
    pub tenancy: String,
    pub total_instance_count: i32,
    pub available_instance_count: i32,
    pub ebs_optimized: bool,
    pub ephemeral_storage: bool,
    pub state: String,
    pub start_date: String,
    #[serde(default)]
    pub end_date: Option<String>,
    pub end_date_type: String,
    pub instance_match_criteria: String,
    pub create_date: String,
    #[serde(default)]
    pub outpost_arn: Option<String>,
    #[serde(default)]
    pub placement_group_arn: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub pending_billing_owner_account_id: Option<String>,
    #[serde(default)]
    pub billing_owner_account_id: Option<String>,
    #[serde(default)]
    pub target_capacity_reservation_id: Option<String>,
    #[serde(default)]
    pub reservation_type: Option<String>,
    #[serde(default)]
    pub commitment_info: Option<CapacityReservationCommitmentInfoView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapacityReservationCommitmentInfoView {
    #[serde(default)]
    pub commitment_end_date: Option<String>,
    #[serde(default)]
    pub committed_instance_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapacityReservationSpecificationResponseView {
    #[serde(default)]
    pub capacity_reservation_preference: Option<String>,
    #[serde(default)]
    pub capacity_reservation_id: Option<String>,
    #[serde(default)]
    pub capacity_reservation_resource_group_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingOwnershipOfferView {
    pub capacity_reservation_id: String,
    pub unused_reservation_billing_owner_id: String,
    pub requested_by: String,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    pub last_update_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct S3DestinationOptionsView {
    pub bucket: String,
    #[serde(default)]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityManagerDataExportView {
    pub data_export_id: String,
    pub schedule: String,
    #[serde(default)]
    pub organization_account_ids: Vec<String>,
    pub output_format: String,
    pub s3_destination: S3DestinationOptionsView,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub last_export_time: Option<String>,
    #[serde(default)]
    pub next_export_time: Option<String>,
    pub create_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityManagerOrganizationsAccessView {
    pub state: String,
    pub last_updated_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptibleCapacityReservationAllocationView {
    pub allocation_id: String,
    pub capacity_reservation_id: String,
    pub instance_count: i32,
    pub start_date_time: String,
    pub end_date_time: String,
    pub state: String,
    pub allocation_type: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityBlockView {
    pub capacity_block_id: String,
    pub capacity_reservation_id: String,
    pub capacity_block_offering_id: String,
    pub instance_type: String,
    pub instance_count: i32,
    pub availability_zone: String,
    pub start_date: String,
    pub end_date: String,
    pub tenancy: String,
    pub currency_code: String,
    pub upfront_fee: String,
    pub commitment_duration_in_seconds: i64,
    pub capacity_reservation_arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityBlockExtensionView {
    pub capacity_block_extension_id: String,
    pub capacity_reservation_id: String,
    pub instance_type: String,
    pub availability_zone: String,
    pub instance_count: i32,
    #[serde(default)]
    pub availability_zone_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub capacity_block_extension_offering_id: String,
    pub capacity_block_extension_status: String,
    pub capacity_block_extension_purchase_date: String,
    pub capacity_block_extension_duration_hours: i32,
    pub currency_code: String,
    pub upfront_fee: String,
    #[serde(default)]
    pub capacity_reservation_arn: Option<String>,
    #[serde(default)]
    pub capacity_block_extension_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReservationFleetInstanceSpecView {
    pub instance_type: String,
    pub instance_platform: String,
    #[serde(default)]
    pub availability_zone: Option<String>,
    #[serde(default)]
    pub ebs_optimized: Option<bool>,
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReservationFleetView {
    pub capacity_reservation_fleet_id: String,
    pub capacity_reservation_fleet_arn: String,
    pub state: String,
    pub tenancy: String,
    pub allocation_strategy: String,
    pub instance_match_criteria: String,
    pub total_target_capacity: i32,
    pub total_fulfilled_capacity: f64,
    pub create_time: String,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub instance_type_specifications: Vec<CapacityReservationFleetInstanceSpecView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConnectEndpointView {
    pub instance_connect_endpoint_id: String,
    pub instance_connect_endpoint_arn: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub availability_zone: String,
    pub state: String,
    pub created_at: String,
    pub preserve_client_ip: bool,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(default)]
    pub network_interface_ids: Vec<String>,
    pub dns_name: String,
    pub fips_dns_name: String,
    pub ip_address_type: String,
    pub owner_id: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoipPoolView {
    pub pool_id: String,
    pub pool_arn: String,
    pub local_gateway_route_table_id: String,
    #[serde(default)]
    pub pool_cidrs: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGroupVpcAssociationView {
    pub group_id: String,
    pub vpc_id: String,
    pub vpc_owner_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnclaveCertificateIamRoleAssociationView {
    pub certificate_arn: String,
    pub role_arn: String,
    pub certificate_s3_bucket_name: String,
    pub certificate_s3_object_key: String,
    pub encryption_kms_key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacSipModificationTaskView {
    pub task_id: String,
    pub instance_id: String,
    pub task_type: String,
    pub task_state: String,
    pub start_time: String,
    #[serde(default)]
    pub apple_internal: Option<String>,
    #[serde(default)]
    pub base_system: Option<String>,
    #[serde(default)]
    pub debugging_restrictions: Option<String>,
    #[serde(default)]
    pub dtrace_restrictions: Option<String>,
    #[serde(default)]
    pub filesystem_protections: Option<String>,
    #[serde(default)]
    pub kext_signing: Option<String>,
    #[serde(default)]
    pub nvram_protections: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclarativePoliciesReportView {
    pub report_id: String,
    pub s3_bucket: String,
    #[serde(default)]
    pub s3_prefix: Option<String>,
    pub target_id: String,
    pub status: String,
    pub start_time: String,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// ===== Group 6: Network Insights view types =====

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PathStatementSpecView {
    #[serde(default)]
    pub packet_header_statement: Option<PacketHeaderStatementSpecView>,
    #[serde(default)]
    pub resource_statement: Option<ResourceStatementSpecView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PacketHeaderStatementSpecView {
    #[serde(default)]
    pub destination_addresses: Vec<String>,
    #[serde(default)]
    pub destination_ports: Vec<String>,
    #[serde(default)]
    pub destination_prefix_lists: Vec<String>,
    #[serde(default)]
    pub protocols: Vec<String>,
    #[serde(default)]
    pub source_addresses: Vec<String>,
    #[serde(default)]
    pub source_ports: Vec<String>,
    #[serde(default)]
    pub source_prefix_lists: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceStatementSpecView {
    #[serde(default)]
    pub resource_types: Vec<String>,
    #[serde(default)]
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessScopePathSpecView {
    #[serde(default)]
    pub source: Option<PathStatementSpecView>,
    #[serde(default)]
    pub destination: Option<PathStatementSpecView>,
    #[serde(default)]
    pub through_resources: Vec<ResourceStatementSpecView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInsightsAccessScopeView {
    pub network_insights_access_scope_id: String,
    pub network_insights_access_scope_arn: String,
    pub created_date: String,
    pub updated_date: String,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub match_paths: Vec<AccessScopePathSpecView>,
    #[serde(default)]
    pub exclude_paths: Vec<AccessScopePathSpecView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInsightsAccessScopeAnalysisView {
    pub network_insights_access_scope_analysis_id: String,
    pub network_insights_access_scope_analysis_arn: String,
    pub network_insights_access_scope_id: String,
    pub status: String,
    pub status_message: Option<String>,
    pub warning_message: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub findings_found: String,
    pub analyzed_eni_count: i32,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkInsightsPathFilterPortRangeView {
    pub from_port: Option<i32>,
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkInsightsPathFilterView {
    pub destination_address: Option<String>,
    pub destination_port_range: Option<NetworkInsightsPathFilterPortRangeView>,
    pub source_address: Option<String>,
    pub source_port_range: Option<NetworkInsightsPathFilterPortRangeView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInsightsPathView {
    pub network_insights_path_id: String,
    pub network_insights_path_arn: String,
    pub created_date: String,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub source_arn: Option<String>,
    pub destination_arn: Option<String>,
    pub source_ip: Option<String>,
    pub destination_ip: Option<String>,
    pub protocol: String,
    pub destination_port: Option<i32>,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub filter_at_source: NetworkInsightsPathFilterView,
    #[serde(default)]
    pub filter_at_destination: NetworkInsightsPathFilterView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInsightsAnalysisView {
    pub network_insights_analysis_id: String,
    pub network_insights_analysis_arn: String,
    pub network_insights_path_id: String,
    #[serde(default)]
    pub additional_accounts: Vec<String>,
    #[serde(default)]
    pub filter_in_arns: Vec<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub warning_message: Option<String>,
    pub network_path_found: bool,
    pub tags: HashMap<String, String>,
}

// ===== Group 6: Traffic Mirror view types =====

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrafficMirrorPortRangeView {
    pub from_port: Option<i32>,
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficMirrorFilterRuleView {
    pub traffic_mirror_filter_rule_id: String,
    pub traffic_mirror_filter_id: String,
    pub traffic_direction: String,
    pub rule_number: i32,
    pub rule_action: String,
    pub protocol: Option<i32>,
    pub destination_port_range: Option<TrafficMirrorPortRangeView>,
    pub source_port_range: Option<TrafficMirrorPortRangeView>,
    pub destination_cidr_block: String,
    pub source_cidr_block: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficMirrorFilterView {
    pub traffic_mirror_filter_id: String,
    pub description: Option<String>,
    #[serde(default)]
    pub ingress_filter_rules: Vec<TrafficMirrorFilterRuleView>,
    #[serde(default)]
    pub egress_filter_rules: Vec<TrafficMirrorFilterRuleView>,
    #[serde(default)]
    pub network_services: Vec<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficMirrorSessionView {
    pub traffic_mirror_session_id: String,
    pub traffic_mirror_target_id: String,
    pub traffic_mirror_filter_id: String,
    pub network_interface_id: String,
    pub owner_id: String,
    pub packet_length: Option<i32>,
    pub session_number: i32,
    pub virtual_network_id: Option<i32>,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficMirrorTargetView {
    pub traffic_mirror_target_id: String,
    pub network_interface_id: Option<String>,
    pub network_load_balancer_arn: Option<String>,
    pub gateway_load_balancer_endpoint_id: Option<String>,
    pub r#type: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub tags: HashMap<String, String>,
}

// ===== Group 7: Client VPN view types =====

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientVpnEndpointStatusView {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientVpnAuthorizationRuleStatusView {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientVpnRouteStatusView {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientVpnAssociationStatusView {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientVpnConnectionStatusView {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientVpnEndpointView {
    pub client_vpn_endpoint_id: String,
    pub description: Option<String>,
    pub status: ClientVpnEndpointStatusView,
    pub creation_time: String,
    pub deletion_time: Option<String>,
    pub dns_name: String,
    pub client_cidr_block: String,
    #[serde(default)]
    pub dns_servers: Vec<String>,
    pub split_tunnel: bool,
    pub vpn_protocol: String,
    pub transport_protocol: String,
    pub vpn_port: i32,
    pub server_certificate_arn: String,
    #[serde(default)]
    pub authentication_options: Vec<String>,
    pub connection_log_options_enabled: bool,
    pub connection_log_options_cloudwatch_log_group: Option<String>,
    pub connection_log_options_cloudwatch_log_stream: Option<String>,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    pub vpc_id: Option<String>,
    pub self_service_portal_url: Option<String>,
    pub self_service_portal: String,
    pub session_timeout_hours: i32,
    pub client_login_banner_enabled: bool,
    pub client_login_banner_text: Option<String>,
    pub disconnect_on_session_timeout: bool,
    pub client_route_enforcement_enforced: bool,
    pub client_certificate_revocation_list: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientVpnTargetNetworkAssociationView {
    pub association_id: String,
    pub vpc_id: String,
    pub target_network_id: String,
    pub client_vpn_endpoint_id: String,
    #[serde(default)]
    pub security_groups: Vec<String>,
    pub status: ClientVpnAssociationStatusView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientVpnAuthorizationRuleView {
    pub client_vpn_endpoint_id: String,
    pub group_id: Option<String>,
    pub access_all: bool,
    pub destination_cidr: String,
    pub description: Option<String>,
    pub status: ClientVpnAuthorizationRuleStatusView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientVpnRouteView {
    pub client_vpn_endpoint_id: String,
    pub destination_cidr: String,
    pub target_subnet: String,
    pub r#type: String,
    pub origin: String,
    pub status: ClientVpnRouteStatusView,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientVpnConnectionView {
    pub connection_id: String,
    pub client_vpn_endpoint_id: String,
    pub username: Option<String>,
    pub status: ClientVpnConnectionStatusView,
    #[serde(default)]
    pub posture_compliance_statuses: Vec<String>,
    pub common_name: Option<String>,
    pub connection_established_time: String,
    pub connection_end_time: Option<String>,
    pub ingress_bytes: String,
    pub egress_bytes: String,
    pub ingress_packets: String,
    pub egress_packets: String,
    pub client_ip: Option<String>,
    pub client_port: Option<String>,
    pub timestamp: String,
}

// ===== Group 7: Local Gateway view types =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGatewayView {
    pub local_gateway_id: String,
    pub outpost_arn: String,
    pub owner_id: String,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGatewayRouteView {
    pub destination_cidr_block: String,
    pub local_gateway_route_table_id: String,
    pub r#type: String,
    pub state: String,
    pub local_gateway_route_table_arn: Option<String>,
    pub owner_id: String,
    pub subnet_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub destination_prefix_list_id: Option<String>,
    pub coip_pool_id: Option<String>,
    pub local_gateway_virtual_interface_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGatewayRouteTableView {
    pub local_gateway_route_table_id: String,
    pub local_gateway_route_table_arn: String,
    pub local_gateway_id: String,
    pub owner_id: String,
    pub state: String,
    pub mode: String,
    pub tags: HashMap<String, String>,
    pub state_reason_code: Option<String>,
    pub state_reason_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGatewayRouteTableVirtualInterfaceGroupAssociationView {
    pub local_gateway_route_table_virtual_interface_group_association_id: String,
    pub local_gateway_virtual_interface_group_id: String,
    pub local_gateway_route_table_id: String,
    pub local_gateway_route_table_arn: String,
    pub local_gateway_id: String,
    pub owner_id: String,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGatewayRouteTableVpcAssociationView {
    pub local_gateway_route_table_vpc_association_id: String,
    pub local_gateway_route_table_id: String,
    pub local_gateway_route_table_arn: String,
    pub local_gateway_id: String,
    pub vpc_id: String,
    pub owner_id: String,
    pub state: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGatewayVirtualInterfaceView {
    pub local_gateway_virtual_interface_id: String,
    pub local_gateway_id: String,
    pub vlan: i32,
    pub local_address: String,
    pub peer_address: String,
    pub local_bgp_asn: i32,
    pub peer_bgp_asn: i32,
    pub owner_id: String,
    pub tags: HashMap<String, String>,
    pub configuration_state: String,
    pub peer_bgp_asn_extended: Option<i64>,
    pub local_gateway_virtual_interface_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGatewayVirtualInterfaceGroupView {
    pub local_gateway_virtual_interface_group_id: String,
    #[serde(default)]
    pub local_gateway_virtual_interface_ids: Vec<String>,
    pub local_gateway_id: String,
    pub owner_id: String,
    pub tags: HashMap<String, String>,
    pub configuration_state: Option<String>,
    pub local_bgp_asn: i32,
    pub local_bgp_asn_extended: Option<i64>,
    pub local_gateway_virtual_interface_group_arn: Option<String>,
}

// ===== Group 8: Route Server view types =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteServerView {
    pub route_server_id: String,
    pub route_server_arn: String,
    pub amazon_side_asn: i64,
    pub state: String,
    pub persist_routes: String,
    pub persist_routes_duration: Option<i64>,
    pub sns_notifications_enabled: bool,
    pub sns_topic_arn: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteServerEndpointView {
    pub route_server_endpoint_id: String,
    pub route_server_id: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub eni_id: String,
    pub eni_address: Option<String>,
    pub state: String,
    pub failure_reason: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RouteServerBgpOptionsView {
    pub peer_asn: Option<i64>,
    pub peer_liveness_detection: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RouteServerPeerOptionsView {
    pub peer_asn: i64,
    pub peer_liveness_detection: String,
    pub bgp_options: Option<RouteServerBgpOptionsView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteServerPeerView {
    pub route_server_peer_id: String,
    pub route_server_endpoint_id: String,
    pub route_server_id: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub peer_address: String,
    pub state: String,
    pub failure_reason: Option<String>,
    pub options: RouteServerPeerOptionsView,
    pub endpoint_eni_id: Option<String>,
    pub endpoint_eni_address: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteServerAssociationView {
    pub route_server_id: String,
    pub vpc_id: String,
    pub state: String,
    #[serde(default)]
    pub propagations: Vec<String>,
}

// ===== Group 9: Verified Access view types =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedAccessInstanceView {
    pub verified_access_instance_id: String,
    #[serde(default)]
    pub description: Option<String>,
    pub creation_time: String,
    pub last_updated_time: String,
    #[serde(default)]
    pub fips_enabled: bool,
    #[serde(default)]
    pub cidr_endpoints_custom_subdomain: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub trust_provider_ids: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessOidcOptionsView {
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(default)]
    pub authorization_endpoint: Option<String>,
    #[serde(default)]
    pub token_endpoint: Option<String>,
    #[serde(default)]
    pub user_info_endpoint: Option<String>,
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub client_secret: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessDeviceOptionsView {
    #[serde(default)]
    pub tenant_id: Option<String>,
    #[serde(default)]
    pub public_signing_key_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessNativeApplicationOidcOptionsView {
    #[serde(default)]
    pub public_signing_key_endpoint: Option<String>,
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(default)]
    pub authorization_endpoint: Option<String>,
    #[serde(default)]
    pub token_endpoint: Option<String>,
    #[serde(default)]
    pub user_info_endpoint: Option<String>,
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub client_secret: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessSseSpecificationView {
    #[serde(default)]
    pub customer_managed_key_enabled: Option<bool>,
    #[serde(default)]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedAccessTrustProviderView {
    pub verified_access_trust_provider_id: String,
    #[serde(default)]
    pub description: Option<String>,
    pub trust_provider_type: String,
    #[serde(default)]
    pub user_trust_provider_type: Option<String>,
    #[serde(default)]
    pub device_trust_provider_type: Option<String>,
    #[serde(default)]
    pub oidc_options: Option<VerifiedAccessOidcOptionsView>,
    #[serde(default)]
    pub device_options: Option<VerifiedAccessDeviceOptionsView>,
    #[serde(default)]
    pub native_application_oidc_options: Option<VerifiedAccessNativeApplicationOidcOptionsView>,
    pub policy_reference_name: String,
    pub creation_time: String,
    pub last_updated_time: String,
    #[serde(default)]
    pub sse_specification: VerifiedAccessSseSpecificationView,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedAccessGroupView {
    pub verified_access_group_id: String,
    pub verified_access_group_arn: String,
    pub verified_access_instance_id: String,
    pub owner: String,
    #[serde(default)]
    pub description: Option<String>,
    pub creation_time: String,
    pub last_updated_time: String,
    #[serde(default)]
    pub deletion_time: Option<String>,
    #[serde(default)]
    pub sse_specification: VerifiedAccessSseSpecificationView,
    #[serde(default)]
    pub policy_document: Option<String>,
    #[serde(default)]
    pub policy_enabled: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessEndpointPortRangeView {
    #[serde(default)]
    pub from_port: Option<i32>,
    #[serde(default)]
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessEndpointLoadBalancerOptionsView {
    #[serde(default)]
    pub load_balancer_arn: Option<String>,
    #[serde(default)]
    pub port: Option<i32>,
    #[serde(default)]
    pub port_ranges: Vec<VerifiedAccessEndpointPortRangeView>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessEndpointEniOptionsView {
    #[serde(default)]
    pub network_interface_id: Option<String>,
    #[serde(default)]
    pub port: Option<i32>,
    #[serde(default)]
    pub port_ranges: Vec<VerifiedAccessEndpointPortRangeView>,
    #[serde(default)]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessEndpointCidrOptionsView {
    #[serde(default)]
    pub cidr: Option<String>,
    #[serde(default)]
    pub port_ranges: Vec<VerifiedAccessEndpointPortRangeView>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessEndpointRdsOptionsView {
    #[serde(default)]
    pub port: Option<i32>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub rds_db_cluster_arn: Option<String>,
    #[serde(default)]
    pub rds_db_instance_arn: Option<String>,
    #[serde(default)]
    pub rds_db_proxy_arn: Option<String>,
    #[serde(default)]
    pub rds_endpoint: Option<String>,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedAccessEndpointView {
    pub verified_access_endpoint_id: String,
    pub verified_access_instance_id: String,
    pub verified_access_group_id: String,
    #[serde(default)]
    pub application_domain: Option<String>,
    pub endpoint_type: String,
    pub attachment_type: String,
    #[serde(default)]
    pub domain_certificate_arn: Option<String>,
    #[serde(default)]
    pub endpoint_domain: Option<String>,
    #[serde(default)]
    pub device_validation_domain: Option<String>,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(default)]
    pub load_balancer_options: Option<VerifiedAccessEndpointLoadBalancerOptionsView>,
    #[serde(default)]
    pub network_interface_options: Option<VerifiedAccessEndpointEniOptionsView>,
    #[serde(default)]
    pub cidr_options: Option<VerifiedAccessEndpointCidrOptionsView>,
    #[serde(default)]
    pub rds_options: Option<VerifiedAccessEndpointRdsOptionsView>,
    pub status_code: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub creation_time: String,
    pub last_updated_time: String,
    #[serde(default)]
    pub deletion_time: Option<String>,
    #[serde(default)]
    pub sse_specification: VerifiedAccessSseSpecificationView,
    #[serde(default)]
    pub policy_document: Option<String>,
    #[serde(default)]
    pub policy_enabled: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedAccessTrustProviderAttachmentView {
    pub instance_id: String,
    pub trust_provider_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifiedAccessLogsView {
    #[serde(default)]
    pub cloud_watch_logs_enabled: bool,
    #[serde(default)]
    pub cloud_watch_logs_log_group: Option<String>,
    #[serde(default)]
    pub kinesis_data_firehose_enabled: bool,
    #[serde(default)]
    pub kinesis_data_firehose_delivery_stream: Option<String>,
    #[serde(default)]
    pub s3_enabled: bool,
    #[serde(default)]
    pub s3_bucket_name: Option<String>,
    #[serde(default)]
    pub s3_bucket_owner: Option<String>,
    #[serde(default)]
    pub s3_prefix: Option<String>,
    #[serde(default)]
    pub log_version: Option<String>,
    #[serde(default)]
    pub include_trust_context: Option<bool>,
}

/// Serializable snapshot of all counters so IDs remain stable across restore.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountersView {
    pub vpc: u32,
    pub subnet: u32,
    pub igw: u32,
    pub sg: u32,
    pub sgr: u32,
    pub rtb: u32,
    pub rtbassoc: u32,
    pub keypair: u32,
    pub nacl: u32,
    pub nacl_assoc: u32,
    pub eip: u32,
    #[serde(default)]
    pub eip_assoc: u32,
    pub nat: u32,
    pub dopt: u32,
    pub eigw: u32,
    pub flow_log: u32,
    pub vpc_peering: u32,
    pub vpc_endpoint: u32,
    pub prefix_list: u32,
    pub cgw: u32,
    pub vgw: u32,
    pub vpn: u32,
    pub cgw_carrier: u32,
    pub eni: u32,
    #[serde(default)]
    pub eni_attach: u32,
    pub vpc_cidr_assoc: u32,
    pub tgw: u32,
    pub tgw_attach: u32,
    pub tgw_rtb: u32,
    pub instance: u32,
    pub vol: u32,
    pub snapshot: u32,
    pub ami: u32,
    pub lt: u32,
    pub spot: u32,
    pub iam_assoc: u32,
    pub host: u32,
    pub fleet: u32,
    pub vpce_svc: u32,
    pub spot_fleet: u32,
    pub subnet_cidr_res: u32,
    pub subnet_ipv6_assoc: u32,
    #[serde(default)]
    pub placement_group: u32,
    #[serde(default)]
    pub eni_permission: u32,
    #[serde(default)]
    pub instance_connect_endpoint: u32,
    #[serde(default)]
    pub capacity_reservation: u32,
    #[serde(default)]
    pub capacity_reservation_fleet: u32,
    #[serde(default)]
    pub coip_pool: u32,
    #[serde(default)]
    pub mac_sip_task: u32,
    #[serde(default)]
    pub declarative_policies_report: u32,
    #[serde(default)]
    pub public_ipv4_pool: u32,
    #[serde(default)]
    pub address_transfer: u32,
    #[serde(default)]
    pub nat_gateway_address_assoc: u32,
    #[serde(default)]
    pub vpn_concentrator: u32,
    #[serde(default)]
    pub vpc_endpoint_connection_notification: u32,
    #[serde(default)]
    pub vpc_block_public_access_exclusion: u32,
    #[serde(default)]
    pub vpc_encryption_control: u32,
    #[serde(default)]
    pub mac_volume_ownership_task: u32,
    #[serde(default)]
    pub replace_root_volume_task: u32,
    #[serde(default)]
    pub snapshot_import_task: u32,
    #[serde(default)]
    pub conversion_task: u32,
    #[serde(default)]
    pub export_task: u32,
    #[serde(default)]
    pub import_task: u32,
    #[serde(default)]
    pub trunk_interface_assoc: u32,
    #[serde(default)]
    pub secondary_network: u32,
    #[serde(default)]
    pub secondary_subnet: u32,
    #[serde(default)]
    pub reserved_instances_exchange: u32,
    #[serde(default)]
    pub reserved_instances_listing: u32,
    #[serde(default)]
    pub reserved_instances_purchase: u32,
    #[serde(default)]
    pub reserved_instances: u32,
    #[serde(default)]
    pub reserved_instances_modification: u32,
    #[serde(default)]
    pub fpga_image: u32,
    #[serde(default)]
    pub image_usage_report: u32,
    #[serde(default)]
    pub import_image_task: u32,
    #[serde(default)]
    pub instance_event_window: u32,
    #[serde(default)]
    pub instance_event: u32,
    #[serde(default)]
    pub host_reservation: u32,
    #[serde(default)]
    pub scheduled_instance: u32,
    #[serde(default)]
    pub network_insights_access_scope: u32,
    #[serde(default)]
    pub network_insights_access_scope_analysis: u32,
    #[serde(default)]
    pub network_insights_path: u32,
    #[serde(default)]
    pub network_insights_analysis: u32,
    #[serde(default)]
    pub traffic_mirror_filter: u32,
    #[serde(default)]
    pub traffic_mirror_filter_rule: u32,
    #[serde(default)]
    pub traffic_mirror_session: u32,
    #[serde(default)]
    pub traffic_mirror_target: u32,
    // --- Group 7 additions ---
    #[serde(default)]
    pub client_vpn_endpoint: u32,
    #[serde(default)]
    pub client_vpn_target_network_association: u32,
    #[serde(default)]
    pub client_vpn_connection: u32,
    #[serde(default)]
    pub local_gateway: u32,
    #[serde(default)]
    pub local_gateway_route_table: u32,
    #[serde(default)]
    pub local_gateway_route_table_virtual_interface_group_association: u32,
    #[serde(default)]
    pub local_gateway_route_table_vpc_association: u32,
    #[serde(default)]
    pub local_gateway_virtual_interface: u32,
    #[serde(default)]
    pub local_gateway_virtual_interface_group: u32,
    // --- Group 8 additions ---
    #[serde(default)]
    pub route_server: u32,
    #[serde(default)]
    pub route_server_endpoint: u32,
    #[serde(default)]
    pub route_server_peer: u32,
    // --- Group 9 additions ---
    #[serde(default)]
    pub verified_access_instance: u32,
    #[serde(default)]
    pub verified_access_trust_provider: u32,
    #[serde(default)]
    pub verified_access_group: u32,
    #[serde(default)]
    pub verified_access_endpoint: u32,
    // --- Group 10 additions ---
    #[serde(default)]
    pub capacity_manager_data_export: u32,
    #[serde(default)]
    pub interruptible_capacity_reservation_allocation: u32,
    #[serde(default)]
    pub capacity_block: u32,
    #[serde(default)]
    pub capacity_block_extension: u32,
    // --- Group 11 additions ---
    #[serde(default)]
    pub tgw_multicast_domain: u32,
    #[serde(default)]
    pub tgw_connect: u32,
    #[serde(default)]
    pub tgw_connect_peer: u32,
    #[serde(default)]
    pub tgw_metering_policy: u32,
    #[serde(default)]
    pub tgw_metering_policy_entry: u32,
    #[serde(default)]
    pub tgw_policy_table: u32,
    #[serde(default)]
    pub tgw_route_table_announcement: u32,
    // --- Group 12 additions ---
    #[serde(default)]
    pub ipam: u32,
    #[serde(default)]
    pub ipam_scope: u32,
    #[serde(default)]
    pub ipam_pool: u32,
    #[serde(default)]
    pub ipam_pool_cidr: u32,
    #[serde(default)]
    pub ipam_pool_allocation: u32,
    #[serde(default)]
    pub ipam_resource_discovery: u32,
    #[serde(default)]
    pub ipam_resource_discovery_association: u32,
    #[serde(default)]
    pub ipam_external_resource_verification_token: u32,
    #[serde(default)]
    pub ipam_policy: u32,
    #[serde(default)]
    pub ipam_prefix_list_resolver: u32,
    #[serde(default)]
    pub ipam_prefix_list_resolver_target: u32,
    // --- Batch B additions ---
    #[serde(default)]
    pub bundle_task: u32,
    #[serde(default)]
    pub volume_modification: u32,
    #[serde(default)]
    pub import_volume_task: u32,
    #[serde(default)]
    pub export_image_task: u32,
    #[serde(default)]
    pub outpost_lag: u32,
}

// ---------------------------------------------------------------------------
// From internal types to view types
// ---------------------------------------------------------------------------

impl From<&Ec2State> for Ec2StateView {
    fn from(state: &Ec2State) -> Self {
        Ec2StateView {
            vpcs: state
                .vpcs
                .iter()
                .map(|(k, v)| (k.clone(), VpcView::from(v)))
                .collect(),
            subnets: state
                .subnets
                .iter()
                .map(|(k, v)| (k.clone(), SubnetView::from(v)))
                .collect(),
            igws: state
                .igws
                .iter()
                .map(|(k, v)| (k.clone(), InternetGatewayView::from(v)))
                .collect(),
            security_groups: state
                .security_groups
                .iter()
                .map(|(k, v)| (k.clone(), SecurityGroupView::from(v)))
                .collect(),
            route_tables: state
                .route_tables
                .iter()
                .map(|(k, v)| (k.clone(), RouteTableView::from(v)))
                .collect(),
            key_pairs: state
                .key_pairs
                .iter()
                .map(|(k, v)| (k.clone(), KeyPairView::from(v)))
                .collect(),
            network_acls: state
                .network_acls
                .iter()
                .map(|(k, v)| (k.clone(), NetworkAclView::from(v)))
                .collect(),
            elastic_ips: state
                .elastic_ips
                .iter()
                .map(|(k, v)| (k.clone(), ElasticIpView::from(v)))
                .collect(),
            nat_gateways: state
                .nat_gateways
                .iter()
                .map(|(k, v)| (k.clone(), NatGatewayView::from(v)))
                .collect(),
            dhcp_options: state
                .dhcp_options
                .iter()
                .map(|(k, v)| (k.clone(), DhcpOptionsView::from(v)))
                .collect(),
            egress_only_igws: state
                .egress_only_igws
                .iter()
                .map(|(k, v)| (k.clone(), EgressOnlyIgwView::from(v)))
                .collect(),
            flow_logs: state
                .flow_logs
                .iter()
                .map(|(k, v)| (k.clone(), FlowLogView::from(v)))
                .collect(),
            vpc_peering_connections: state
                .vpc_peering_connections
                .iter()
                .map(|(k, v)| (k.clone(), VpcPeeringConnectionView::from(v)))
                .collect(),
            vpc_endpoints: state
                .vpc_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), VpcEndpointView::from(v)))
                .collect(),
            managed_prefix_lists: state
                .managed_prefix_lists
                .iter()
                .map(|(k, v)| (k.clone(), ManagedPrefixListView::from(v)))
                .collect(),
            customer_gateways: state
                .customer_gateways
                .iter()
                .map(|(k, v)| (k.clone(), CustomerGatewayView::from(v)))
                .collect(),
            vpn_gateways: state
                .vpn_gateways
                .iter()
                .map(|(k, v)| (k.clone(), VpnGatewayView::from(v)))
                .collect(),
            vpn_connections: state
                .vpn_connections
                .iter()
                .map(|(k, v)| (k.clone(), VpnConnectionView::from(v)))
                .collect(),
            carrier_gateways: state
                .carrier_gateways
                .iter()
                .map(|(k, v)| (k.clone(), CarrierGatewayView::from(v)))
                .collect(),
            network_interfaces: state
                .network_interfaces
                .iter()
                .map(|(k, v)| (k.clone(), NetworkInterfaceView::from(v)))
                .collect(),
            vpc_cidr_associations: state.vpc_cidr_associations.clone(),
            ebs_encryption_by_default: state.ebs_encryption_by_default,
            transit_gateways: state
                .transit_gateways
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayView::from(v)))
                .collect(),
            tgw_vpc_attachments: state
                .tgw_vpc_attachments
                .iter()
                .map(|(k, v)| (k.clone(), TgwVpcAttachmentView::from(v)))
                .collect(),
            tgw_peering_attachments: state
                .tgw_peering_attachments
                .iter()
                .map(|(k, v)| (k.clone(), TgwPeeringAttachmentView::from(v)))
                .collect(),
            tgw_route_tables: state
                .tgw_route_tables
                .iter()
                .map(|(k, v)| (k.clone(), TgwRouteTableView::from(v)))
                .collect(),
            tgw_routes: state
                .tgw_routes
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(TgwRouteView::from).collect()))
                .collect(),
            instances: state
                .instances
                .iter()
                .map(|(k, v)| (k.clone(), InstanceView::from(v)))
                .collect(),
            volumes: state
                .volumes
                .iter()
                .map(|(k, v)| (k.clone(), VolumeView::from(v)))
                .collect(),
            snapshots: state
                .snapshots
                .iter()
                .map(|(k, v)| (k.clone(), SnapshotView::from(v)))
                .collect(),
            images: state
                .images
                .iter()
                .map(|(k, v)| (k.clone(), ImageView::from(v)))
                .collect(),
            launch_templates: state
                .launch_templates
                .iter()
                .map(|(k, v)| (k.clone(), LaunchTemplateView::from(v)))
                .collect(),
            launch_template_versions: state
                .launch_template_versions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(LaunchTemplateVersionView::from).collect(),
                    )
                })
                .collect(),
            spot_requests: state
                .spot_requests
                .iter()
                .map(|(k, v)| (k.clone(), SpotInstanceRequestView::from(v)))
                .collect(),
            iam_instance_profile_associations: state
                .iam_instance_profile_associations
                .iter()
                .map(|(k, v)| (k.clone(), IamInstanceProfileAssociationView::from(v)))
                .collect(),
            dedicated_hosts: state
                .dedicated_hosts
                .iter()
                .map(|(k, v)| (k.clone(), DedicatedHostView::from(v)))
                .collect(),
            ec2_fleets: state
                .ec2_fleets
                .iter()
                .map(|(k, v)| (k.clone(), Ec2FleetView::from(v)))
                .collect(),
            vpc_endpoint_service_configs: state
                .vpc_endpoint_service_configs
                .iter()
                .map(|(k, v)| (k.clone(), VpcEndpointServiceConfigView::from(v)))
                .collect(),
            spot_fleet_requests: state
                .spot_fleet_requests
                .iter()
                .map(|(k, v)| (k.clone(), SpotFleetRequestView::from(v)))
                .collect(),
            subnet_cidr_reservations: state
                .subnet_cidr_reservations
                .iter()
                .map(|(k, v)| (k.clone(), SubnetCidrReservationView::from(v)))
                .collect(),
            placement_groups: state
                .placement_groups
                .iter()
                .map(|(k, v)| (k.clone(), PlacementGroupView::from(v)))
                .collect(),
            network_interface_permissions: state
                .network_interface_permissions
                .iter()
                .map(|(k, v)| (k.clone(), NetworkInterfacePermissionView::from(v)))
                .collect(),
            instance_connect_endpoints: state
                .instance_connect_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), InstanceConnectEndpointView::from(v)))
                .collect(),
            capacity_reservations: state
                .capacity_reservations
                .iter()
                .map(|(k, v)| (k.clone(), CapacityReservationView::from(v)))
                .collect(),
            capacity_reservation_fleets: state
                .capacity_reservation_fleets
                .iter()
                .map(|(k, v)| (k.clone(), CapacityReservationFleetView::from(v)))
                .collect(),
            coip_pools: state
                .coip_pools
                .iter()
                .map(|(k, v)| (k.clone(), CoipPoolView::from(v)))
                .collect(),
            byoip_cidrs: state
                .byoip_cidrs
                .iter()
                .map(|(k, v)| (k.clone(), ByoipCidrView::from(v)))
                .collect(),
            public_ipv4_pools: state
                .public_ipv4_pools
                .iter()
                .map(|(k, v)| (k.clone(), PublicIpv4PoolView::from(v)))
                .collect(),
            coip_cidrs: state.coip_cidrs.values().map(CoipCidrView::from).collect(),
            address_transfers: state
                .address_transfers
                .iter()
                .map(|(k, v)| (k.clone(), AddressTransferView::from(v)))
                .collect(),
            security_group_vpc_associations: state
                .security_group_vpc_associations
                .values()
                .map(SecurityGroupVpcAssociationView::from)
                .collect(),
            enclave_certificate_iam_role_associations: state
                .enclave_certificate_iam_role_associations
                .values()
                .map(EnclaveCertificateIamRoleAssociationView::from)
                .collect(),
            mac_sip_modification_tasks: state
                .mac_sip_modification_tasks
                .iter()
                .map(|(k, v)| (k.clone(), MacSipModificationTaskView::from(v)))
                .collect(),
            declarative_policies_reports: state
                .declarative_policies_reports
                .iter()
                .map(|(k, v)| (k.clone(), DeclarativePoliciesReportView::from(v)))
                .collect(),
            vpn_concentrators: state
                .vpn_concentrators
                .iter()
                .map(|(k, v)| (k.clone(), VpnConcentratorView::from(v)))
                .collect(),
            vpc_endpoint_connections: state
                .vpc_endpoint_connections
                .values()
                .map(VpcEndpointConnectionView::from)
                .collect(),
            vpc_endpoint_connection_notifications: state
                .vpc_endpoint_connection_notifications
                .iter()
                .map(|(k, v)| (k.clone(), VpcEndpointConnectionNotificationView::from(v)))
                .collect(),
            vpc_block_public_access_exclusions: state
                .vpc_block_public_access_exclusions
                .iter()
                .map(|(k, v)| (k.clone(), VpcBlockPublicAccessExclusionView::from(v)))
                .collect(),
            vpc_block_public_access_options: state
                .vpc_block_public_access_options
                .as_ref()
                .map(VpcBlockPublicAccessOptionsView::from),
            vpc_encryption_controls: state
                .vpc_encryption_controls
                .iter()
                .map(|(k, v)| (k.clone(), VpcEncryptionControlView::from(v)))
                .collect(),
            mac_volume_ownership_tasks: state
                .mac_volume_ownership_tasks
                .iter()
                .map(|(k, v)| (k.clone(), MacVolumeOwnershipTaskView::from(v)))
                .collect(),
            replace_root_volume_tasks: state
                .replace_root_volume_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ReplaceRootVolumeTaskView::from(v)))
                .collect(),
            snapshot_import_tasks: state
                .snapshot_import_tasks
                .iter()
                .map(|(k, v)| (k.clone(), SnapshotImportTaskView::from(v)))
                .collect(),
            conversion_tasks: state
                .conversion_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ConversionTaskView::from(v)))
                .collect(),
            export_tasks: state
                .export_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ExportTaskView::from(v)))
                .collect(),
            import_tasks: state.import_tasks.clone(),
            trunk_interface_associations: state
                .trunk_interface_associations
                .iter()
                .map(|(k, v)| (k.clone(), TrunkInterfaceAssociationView::from(v)))
                .collect(),
            secondary_networks: state
                .secondary_networks
                .iter()
                .map(|(k, v)| (k.clone(), SecondaryNetworkView::from(v)))
                .collect(),
            secondary_subnets: state
                .secondary_subnets
                .iter()
                .map(|(k, v)| (k.clone(), SecondarySubnetView::from(v)))
                .collect(),
            deleted_volumes_recycle_bin: state
                .deleted_volumes_recycle_bin
                .iter()
                .map(|(k, v)| (k.clone(), VolumeView::from(v)))
                .collect(),
            deleted_snapshots_recycle_bin: state
                .deleted_snapshots_recycle_bin
                .iter()
                .map(|(k, v)| (k.clone(), SnapshotView::from(v)))
                .collect(),
            reserved_instances_exchanges: state
                .reserved_instances_exchanges
                .iter()
                .map(|(k, v)| (k.clone(), ReservedInstancesExchangeView::from(v)))
                .collect(),
            reserved_instances_listings: state
                .reserved_instances_listings
                .iter()
                .map(|(k, v)| (k.clone(), ReservedInstancesListingView::from(v)))
                .collect(),
            queued_reserved_instances_purchases: state
                .queued_reserved_instances_purchases
                .iter()
                .map(|(k, v)| (k.clone(), ReservedInstancesPurchaseView::from(v)))
                .collect(),
            reserved_instances_modifications: state
                .reserved_instances_modifications
                .iter()
                .map(|(k, v)| (k.clone(), ReservedInstancesModificationView::from(v)))
                .collect(),
            reserved_instances_purchases: state
                .reserved_instances_purchases
                .iter()
                .map(|(k, v)| (k.clone(), ReservedInstancesPurchaseView::from(v)))
                .collect(),
            reserved_instances: state
                .reserved_instances
                .iter()
                .map(|(k, v)| (k.clone(), ReservedInstancesRecordView::from(v)))
                .collect(),
            fpga_images: state
                .fpga_images
                .iter()
                .map(|(k, v)| (k.clone(), FpgaImageView::from(v)))
                .collect(),
            image_usage_reports: state
                .image_usage_reports
                .iter()
                .map(|(k, v)| (k.clone(), ImageUsageReportView::from(v)))
                .collect(),
            restore_image_tasks: state
                .restore_image_tasks
                .iter()
                .map(|(k, v)| (k.clone(), RestoreImageTaskView::from(v)))
                .collect(),
            store_image_tasks: state
                .store_image_tasks
                .iter()
                .map(|(k, v)| (k.clone(), StoreImageTaskView::from(v)))
                .collect(),
            import_image_tasks: state
                .import_image_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ImportImageTaskView::from(v)))
                .collect(),
            allowed_image_criteria: state
                .allowed_image_criteria
                .iter()
                .map(AllowedImageCriterionView::from)
                .collect(),
            default_credit_specifications: state.default_credit_specifications.clone(),
            instance_metadata_defaults: state
                .instance_metadata_defaults
                .as_ref()
                .map(InstanceMetadataDefaultsView::from),
            instance_event_windows: state
                .instance_event_windows
                .iter()
                .map(|(k, v)| (k.clone(), InstanceEventWindowView::from(v)))
                .collect(),
            instance_event_notification_attributes: state
                .instance_event_notification_attributes
                .as_ref()
                .map(InstanceTagNotificationAttributesView::from),
            instance_events: state
                .instance_events
                .iter()
                .map(|(k, v)| (k.clone(), InstanceEventView::from(v)))
                .collect(),
            host_reservations: state
                .host_reservations
                .iter()
                .map(|(k, v)| (k.clone(), HostReservationView::from(v)))
                .collect(),
            scheduled_instances: state
                .scheduled_instances
                .iter()
                .map(|(k, v)| (k.clone(), ScheduledInstanceView::from(v)))
                .collect(),
            az_group_opt_in: state.az_group_opt_in.clone(),
            instance_status_reports: state
                .instance_status_reports
                .iter()
                .map(InstanceStatusReportView::from)
                .collect(),
            network_insights_access_scopes: state
                .network_insights_access_scopes
                .iter()
                .map(|(k, v)| (k.clone(), NetworkInsightsAccessScopeView::from(v)))
                .collect(),
            network_insights_access_scope_analyses: state
                .network_insights_access_scope_analyses
                .iter()
                .map(|(k, v)| (k.clone(), NetworkInsightsAccessScopeAnalysisView::from(v)))
                .collect(),
            network_insights_paths: state
                .network_insights_paths
                .iter()
                .map(|(k, v)| (k.clone(), NetworkInsightsPathView::from(v)))
                .collect(),
            network_insights_analyses: state
                .network_insights_analyses
                .iter()
                .map(|(k, v)| (k.clone(), NetworkInsightsAnalysisView::from(v)))
                .collect(),
            traffic_mirror_filters: state
                .traffic_mirror_filters
                .iter()
                .map(|(k, v)| (k.clone(), TrafficMirrorFilterView::from(v)))
                .collect(),
            traffic_mirror_sessions: state
                .traffic_mirror_sessions
                .iter()
                .map(|(k, v)| (k.clone(), TrafficMirrorSessionView::from(v)))
                .collect(),
            traffic_mirror_targets: state
                .traffic_mirror_targets
                .iter()
                .map(|(k, v)| (k.clone(), TrafficMirrorTargetView::from(v)))
                .collect(),
            client_vpn_endpoints: state
                .client_vpn_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), ClientVpnEndpointView::from(v)))
                .collect(),
            client_vpn_target_network_associations: state
                .client_vpn_target_network_associations
                .iter()
                .map(|(k, v)| (k.clone(), ClientVpnTargetNetworkAssociationView::from(v)))
                .collect(),
            client_vpn_authorization_rules: state
                .client_vpn_authorization_rules
                .values()
                .map(ClientVpnAuthorizationRuleView::from)
                .collect(),
            client_vpn_routes: state
                .client_vpn_routes
                .values()
                .map(ClientVpnRouteView::from)
                .collect(),
            client_vpn_connections: state
                .client_vpn_connections
                .iter()
                .map(|(k, v)| (k.clone(), ClientVpnConnectionView::from(v)))
                .collect(),
            local_gateways: state
                .local_gateways
                .iter()
                .map(|(k, v)| (k.clone(), LocalGatewayView::from(v)))
                .collect(),
            local_gateway_route_tables: state
                .local_gateway_route_tables
                .iter()
                .map(|(k, v)| (k.clone(), LocalGatewayRouteTableView::from(v)))
                .collect(),
            local_gateway_routes: state
                .local_gateway_routes
                .values()
                .map(LocalGatewayRouteView::from)
                .collect(),
            local_gateway_route_table_virtual_interface_group_associations: state
                .local_gateway_route_table_virtual_interface_group_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        LocalGatewayRouteTableVirtualInterfaceGroupAssociationView::from(v),
                    )
                })
                .collect(),
            local_gateway_route_table_vpc_associations: state
                .local_gateway_route_table_vpc_associations
                .iter()
                .map(|(k, v)| (k.clone(), LocalGatewayRouteTableVpcAssociationView::from(v)))
                .collect(),
            local_gateway_virtual_interfaces: state
                .local_gateway_virtual_interfaces
                .iter()
                .map(|(k, v)| (k.clone(), LocalGatewayVirtualInterfaceView::from(v)))
                .collect(),
            local_gateway_virtual_interface_groups: state
                .local_gateway_virtual_interface_groups
                .iter()
                .map(|(k, v)| (k.clone(), LocalGatewayVirtualInterfaceGroupView::from(v)))
                .collect(),
            route_servers: state
                .route_servers
                .iter()
                .map(|(k, v)| (k.clone(), RouteServerView::from(v)))
                .collect(),
            route_server_endpoints: state
                .route_server_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), RouteServerEndpointView::from(v)))
                .collect(),
            route_server_peers: state
                .route_server_peers
                .iter()
                .map(|(k, v)| (k.clone(), RouteServerPeerView::from(v)))
                .collect(),
            route_server_associations: state
                .route_server_associations
                .values()
                .map(RouteServerAssociationView::from)
                .collect(),
            verified_access_instances: state
                .verified_access_instances
                .iter()
                .map(|(k, v)| (k.clone(), VerifiedAccessInstanceView::from(v)))
                .collect(),
            verified_access_trust_providers: state
                .verified_access_trust_providers
                .iter()
                .map(|(k, v)| (k.clone(), VerifiedAccessTrustProviderView::from(v)))
                .collect(),
            verified_access_groups: state
                .verified_access_groups
                .iter()
                .map(|(k, v)| (k.clone(), VerifiedAccessGroupView::from(v)))
                .collect(),
            verified_access_endpoints: state
                .verified_access_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), VerifiedAccessEndpointView::from(v)))
                .collect(),
            verified_access_trust_provider_attachments: state
                .verified_access_trust_provider_attachments
                .values()
                .map(VerifiedAccessTrustProviderAttachmentView::from)
                .collect(),
            verified_access_instance_logging_configurations: state
                .verified_access_instance_logging_configurations
                .iter()
                .map(|(k, v)| (k.clone(), VerifiedAccessLogsView::from(v)))
                .collect(),
            billing_ownership_offers: state
                .billing_ownership_offers
                .values()
                .map(BillingOwnershipOfferView::from)
                .collect(),
            capacity_manager_data_exports: state
                .capacity_manager_data_exports
                .iter()
                .map(|(k, v)| (k.clone(), CapacityManagerDataExportView::from(v)))
                .collect(),
            interruptible_capacity_reservation_allocations: state
                .interruptible_capacity_reservation_allocations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        InterruptibleCapacityReservationAllocationView::from(v),
                    )
                })
                .collect(),
            capacity_blocks: state
                .capacity_blocks
                .iter()
                .map(|(k, v)| (k.clone(), CapacityBlockView::from(v)))
                .collect(),
            capacity_block_extensions: state
                .capacity_block_extensions
                .iter()
                .map(|(k, v)| (k.clone(), CapacityBlockExtensionView::from(v)))
                .collect(),
            capacity_manager_organizations_access: state
                .capacity_manager_organizations_access
                .as_ref()
                .map(CapacityManagerOrganizationsAccessView::from),
            tgw_multicast_domains: state
                .tgw_multicast_domains
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayMulticastDomainView::from(v)))
                .collect(),
            tgw_multicast_domain_associations: state
                .tgw_multicast_domain_associations
                .values()
                .map(TransitGatewayMulticastDomainAssociationView::from)
                .collect(),
            tgw_multicast_group_members: state
                .tgw_multicast_group_members
                .values()
                .map(TransitGatewayMulticastGroupMemberView::from)
                .collect(),
            tgw_multicast_group_sources: state
                .tgw_multicast_group_sources
                .values()
                .map(TransitGatewayMulticastGroupSourceView::from)
                .collect(),
            tgw_connects: state
                .tgw_connects
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayConnectView::from(v)))
                .collect(),
            tgw_connect_peers: state
                .tgw_connect_peers
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayConnectPeerView::from(v)))
                .collect(),
            tgw_metering_policies: state
                .tgw_metering_policies
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayMeteringPolicyView::from(v)))
                .collect(),
            tgw_metering_policy_entries: state
                .tgw_metering_policy_entries
                .values()
                .map(TransitGatewayMeteringPolicyEntryView::from)
                .collect(),
            tgw_policy_tables: state
                .tgw_policy_tables
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayPolicyTableView::from(v)))
                .collect(),
            tgw_policy_table_associations: state
                .tgw_policy_table_associations
                .values()
                .map(TransitGatewayPolicyTableAssociationView::from)
                .collect(),
            tgw_prefix_list_references: state
                .tgw_prefix_list_references
                .values()
                .map(TransitGatewayPrefixListReferenceView::from)
                .collect(),
            tgw_route_table_announcements: state
                .tgw_route_table_announcements
                .iter()
                .map(|(k, v)| (k.clone(), TransitGatewayRouteTableAnnouncementView::from(v)))
                .collect(),
            ipams: state
                .ipams
                .iter()
                .map(|(k, v)| (k.clone(), IpamView::from(v)))
                .collect(),
            ipam_scopes: state
                .ipam_scopes
                .iter()
                .map(|(k, v)| (k.clone(), IpamScopeView::from(v)))
                .collect(),
            ipam_pools: state
                .ipam_pools
                .iter()
                .map(|(k, v)| (k.clone(), IpamPoolView::from(v)))
                .collect(),
            ipam_pool_cidrs: state
                .ipam_pool_cidrs
                .iter()
                .map(|(k, v)| IpamPoolCidrView::from_kv(k, v))
                .collect(),
            ipam_pool_allocations: state
                .ipam_pool_allocations
                .values()
                .map(IpamPoolAllocationView::from)
                .collect(),
            ipam_resource_discoveries: state
                .ipam_resource_discoveries
                .iter()
                .map(|(k, v)| (k.clone(), IpamResourceDiscoveryView::from(v)))
                .collect(),
            ipam_resource_discovery_associations: state
                .ipam_resource_discovery_associations
                .iter()
                .map(|(k, v)| (k.clone(), IpamResourceDiscoveryAssociationView::from(v)))
                .collect(),
            ipam_byoasns: state
                .ipam_byoasns
                .values()
                .map(IpamByoasnView::from)
                .collect(),
            ipam_external_resource_verification_tokens: state
                .ipam_external_resource_verification_tokens
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IpamExternalResourceVerificationTokenView::from(v),
                    )
                })
                .collect(),
            ipam_policies: state
                .ipam_policies
                .iter()
                .map(|(k, v)| (k.clone(), IpamPolicyView::from(v)))
                .collect(),
            ipam_prefix_list_resolvers: state
                .ipam_prefix_list_resolvers
                .iter()
                .map(|(k, v)| (k.clone(), IpamPrefixListResolverView::from(v)))
                .collect(),
            ipam_prefix_list_resolver_targets: state
                .ipam_prefix_list_resolver_targets
                .values()
                .map(IpamPrefixListResolverTargetView::from)
                .collect(),
            volume_modifications: state
                .volume_modifications
                .iter()
                .map(|(k, v)| (k.clone(), VolumeModificationView::from(v)))
                .collect(),
            import_volume_tasks: state
                .import_volume_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ImportVolumeTaskView::from(v)))
                .collect(),
            bundle_tasks: state
                .bundle_tasks
                .iter()
                .map(|(k, v)| (k.clone(), BundleTaskView::from(v)))
                .collect(),
            id_format: state
                .id_format
                .iter()
                .map(|(k, v)| (k.clone(), IdFormatEntryView::from(v)))
                .collect(),
            outpost_lags: state
                .outpost_lags
                .iter()
                .map(|(k, v)| (k.clone(), OutpostLagView::from(v)))
                .collect(),
            export_image_tasks: state
                .export_image_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ExportImageTaskView::from(v)))
                .collect(),
            ebs_default_kms_key_id: state.ebs_default_kms_key_id.clone(),
            serial_console_access_enabled: state.serial_console_access_enabled,
            allowed_images_settings_state: state.allowed_images_settings_state.clone(),
            image_block_public_access_state: state.image_block_public_access_state.clone(),
            aws_network_performance_subscriptions: state
                .aws_network_performance_subscriptions
                .values()
                .map(|s| AwsNetworkPerformanceSubscriptionView {
                    source: s.source.clone(),
                    destination: s.destination.clone(),
                    metric: s.metric.clone(),
                    statistic: s.statistic.clone(),
                    period: s.period.clone(),
                })
                .collect(),
            counters: CountersView::from(&state.counters),
        }
    }
}

impl From<&crate::state::Ec2Counters> for CountersView {
    fn from(c: &crate::state::Ec2Counters) -> Self {
        CountersView {
            vpc: c.vpc,
            subnet: c.subnet,
            igw: c.igw,
            sg: c.sg,
            sgr: c.sgr,
            rtb: c.rtb,
            rtbassoc: c.rtbassoc,
            keypair: c.keypair,
            nacl: c.nacl,
            nacl_assoc: c.nacl_assoc,
            eip: c.eip,
            eip_assoc: c.eip_assoc,
            nat: c.nat,
            dopt: c.dopt,
            eigw: c.eigw,
            flow_log: c.flow_log,
            vpc_peering: c.vpc_peering,
            vpc_endpoint: c.vpc_endpoint,
            prefix_list: c.prefix_list,
            cgw: c.cgw,
            vgw: c.vgw,
            vpn: c.vpn,
            cgw_carrier: c.cgw_carrier,
            eni: c.eni,
            eni_attach: c.eni_attach,
            vpc_cidr_assoc: c.vpc_cidr_assoc,
            tgw: c.tgw,
            tgw_attach: c.tgw_attach,
            tgw_rtb: c.tgw_rtb,
            instance: c.instance,
            vol: c.vol,
            snapshot: c.snapshot,
            ami: c.ami,
            lt: c.lt,
            spot: c.spot,
            iam_assoc: c.iam_assoc,
            host: c.host,
            fleet: c.fleet,
            vpce_svc: c.vpce_svc,
            spot_fleet: c.spot_fleet,
            subnet_cidr_res: c.subnet_cidr_res,
            subnet_ipv6_assoc: c.subnet_ipv6_assoc,
            placement_group: c.placement_group,
            eni_permission: c.eni_permission,
            instance_connect_endpoint: c.instance_connect_endpoint,
            capacity_reservation: c.capacity_reservation,
            capacity_reservation_fleet: c.capacity_reservation_fleet,
            coip_pool: c.coip_pool,
            mac_sip_task: c.mac_sip_task,
            declarative_policies_report: c.declarative_policies_report,
            public_ipv4_pool: c.public_ipv4_pool,
            address_transfer: c.address_transfer,
            nat_gateway_address_assoc: c.nat_gateway_address_assoc,
            vpn_concentrator: c.vpn_concentrator,
            vpc_endpoint_connection_notification: c.vpc_endpoint_connection_notification,
            vpc_block_public_access_exclusion: c.vpc_block_public_access_exclusion,
            vpc_encryption_control: c.vpc_encryption_control,
            mac_volume_ownership_task: c.mac_volume_ownership_task,
            replace_root_volume_task: c.replace_root_volume_task,
            snapshot_import_task: c.snapshot_import_task,
            conversion_task: c.conversion_task,
            export_task: c.export_task,
            import_task: c.import_task,
            trunk_interface_assoc: c.trunk_interface_assoc,
            secondary_network: c.secondary_network,
            secondary_subnet: c.secondary_subnet,
            reserved_instances_exchange: c.reserved_instances_exchange,
            reserved_instances_listing: c.reserved_instances_listing,
            reserved_instances_purchase: c.reserved_instances_purchase,
            reserved_instances: c.reserved_instances,
            reserved_instances_modification: c.reserved_instances_modification,
            fpga_image: c.fpga_image,
            image_usage_report: c.image_usage_report,
            import_image_task: c.import_image_task,
            instance_event_window: c.instance_event_window,
            instance_event: c.instance_event,
            host_reservation: c.host_reservation,
            scheduled_instance: c.scheduled_instance,
            network_insights_access_scope: c.network_insights_access_scope,
            network_insights_access_scope_analysis: c.network_insights_access_scope_analysis,
            network_insights_path: c.network_insights_path,
            network_insights_analysis: c.network_insights_analysis,
            traffic_mirror_filter: c.traffic_mirror_filter,
            traffic_mirror_filter_rule: c.traffic_mirror_filter_rule,
            traffic_mirror_session: c.traffic_mirror_session,
            traffic_mirror_target: c.traffic_mirror_target,
            client_vpn_endpoint: c.client_vpn_endpoint,
            client_vpn_target_network_association: c.client_vpn_target_network_association,
            client_vpn_connection: c.client_vpn_connection,
            local_gateway: c.local_gateway,
            local_gateway_route_table: c.local_gateway_route_table,
            local_gateway_route_table_virtual_interface_group_association: c
                .local_gateway_route_table_virtual_interface_group_association,
            local_gateway_route_table_vpc_association: c.local_gateway_route_table_vpc_association,
            local_gateway_virtual_interface: c.local_gateway_virtual_interface,
            local_gateway_virtual_interface_group: c.local_gateway_virtual_interface_group,
            route_server: c.route_server,
            route_server_endpoint: c.route_server_endpoint,
            route_server_peer: c.route_server_peer,
            verified_access_instance: c.verified_access_instance,
            verified_access_trust_provider: c.verified_access_trust_provider,
            verified_access_group: c.verified_access_group,
            verified_access_endpoint: c.verified_access_endpoint,
            capacity_manager_data_export: c.capacity_manager_data_export,
            interruptible_capacity_reservation_allocation: c
                .interruptible_capacity_reservation_allocation,
            capacity_block: c.capacity_block,
            capacity_block_extension: c.capacity_block_extension,
            tgw_multicast_domain: c.tgw_multicast_domain,
            tgw_connect: c.tgw_connect,
            tgw_connect_peer: c.tgw_connect_peer,
            tgw_metering_policy: c.tgw_metering_policy,
            tgw_metering_policy_entry: c.tgw_metering_policy_entry,
            tgw_policy_table: c.tgw_policy_table,
            tgw_route_table_announcement: c.tgw_route_table_announcement,
            ipam: c.ipam,
            ipam_scope: c.ipam_scope,
            ipam_pool: c.ipam_pool,
            ipam_pool_cidr: c.ipam_pool_cidr,
            ipam_pool_allocation: c.ipam_pool_allocation,
            ipam_resource_discovery: c.ipam_resource_discovery,
            ipam_resource_discovery_association: c.ipam_resource_discovery_association,
            ipam_external_resource_verification_token: c.ipam_external_resource_verification_token,
            ipam_policy: c.ipam_policy,
            ipam_prefix_list_resolver: c.ipam_prefix_list_resolver,
            ipam_prefix_list_resolver_target: c.ipam_prefix_list_resolver_target,
            bundle_task: c.bundle_task,
            volume_modification: c.volume_modification,
            import_volume_task: c.import_volume_task,
            export_image_task: c.export_image_task,
            outpost_lag: c.outpost_lag,
        }
    }
}

impl From<CountersView> for crate::state::Ec2Counters {
    fn from(c: CountersView) -> Self {
        crate::state::Ec2Counters {
            vpc: c.vpc,
            subnet: c.subnet,
            igw: c.igw,
            sg: c.sg,
            sgr: c.sgr,
            rtb: c.rtb,
            rtbassoc: c.rtbassoc,
            keypair: c.keypair,
            nacl: c.nacl,
            nacl_assoc: c.nacl_assoc,
            eip: c.eip,
            eip_assoc: c.eip_assoc,
            nat: c.nat,
            dopt: c.dopt,
            eigw: c.eigw,
            flow_log: c.flow_log,
            vpc_peering: c.vpc_peering,
            vpc_endpoint: c.vpc_endpoint,
            prefix_list: c.prefix_list,
            cgw: c.cgw,
            vgw: c.vgw,
            vpn: c.vpn,
            cgw_carrier: c.cgw_carrier,
            eni: c.eni,
            eni_attach: c.eni_attach,
            vpc_cidr_assoc: c.vpc_cidr_assoc,
            tgw: c.tgw,
            tgw_attach: c.tgw_attach,
            tgw_rtb: c.tgw_rtb,
            instance: c.instance,
            vol: c.vol,
            snapshot: c.snapshot,
            ami: c.ami,
            lt: c.lt,
            spot: c.spot,
            iam_assoc: c.iam_assoc,
            host: c.host,
            fleet: c.fleet,
            vpce_svc: c.vpce_svc,
            spot_fleet: c.spot_fleet,
            subnet_cidr_res: c.subnet_cidr_res,
            subnet_ipv6_assoc: c.subnet_ipv6_assoc,
            placement_group: c.placement_group,
            eni_permission: c.eni_permission,
            instance_connect_endpoint: c.instance_connect_endpoint,
            capacity_reservation: c.capacity_reservation,
            capacity_reservation_fleet: c.capacity_reservation_fleet,
            coip_pool: c.coip_pool,
            mac_sip_task: c.mac_sip_task,
            declarative_policies_report: c.declarative_policies_report,
            public_ipv4_pool: c.public_ipv4_pool,
            address_transfer: c.address_transfer,
            nat_gateway_address_assoc: c.nat_gateway_address_assoc,
            vpn_concentrator: c.vpn_concentrator,
            vpc_endpoint_connection_notification: c.vpc_endpoint_connection_notification,
            vpc_block_public_access_exclusion: c.vpc_block_public_access_exclusion,
            vpc_encryption_control: c.vpc_encryption_control,
            mac_volume_ownership_task: c.mac_volume_ownership_task,
            replace_root_volume_task: c.replace_root_volume_task,
            snapshot_import_task: c.snapshot_import_task,
            conversion_task: c.conversion_task,
            export_task: c.export_task,
            import_task: c.import_task,
            trunk_interface_assoc: c.trunk_interface_assoc,
            secondary_network: c.secondary_network,
            secondary_subnet: c.secondary_subnet,
            reserved_instances_exchange: c.reserved_instances_exchange,
            reserved_instances_listing: c.reserved_instances_listing,
            reserved_instances_purchase: c.reserved_instances_purchase,
            reserved_instances: c.reserved_instances,
            reserved_instances_modification: c.reserved_instances_modification,
            fpga_image: c.fpga_image,
            image_usage_report: c.image_usage_report,
            import_image_task: c.import_image_task,
            instance_event_window: c.instance_event_window,
            instance_event: c.instance_event,
            host_reservation: c.host_reservation,
            scheduled_instance: c.scheduled_instance,
            network_insights_access_scope: c.network_insights_access_scope,
            network_insights_access_scope_analysis: c.network_insights_access_scope_analysis,
            network_insights_path: c.network_insights_path,
            network_insights_analysis: c.network_insights_analysis,
            traffic_mirror_filter: c.traffic_mirror_filter,
            traffic_mirror_filter_rule: c.traffic_mirror_filter_rule,
            traffic_mirror_session: c.traffic_mirror_session,
            traffic_mirror_target: c.traffic_mirror_target,
            client_vpn_endpoint: c.client_vpn_endpoint,
            client_vpn_target_network_association: c.client_vpn_target_network_association,
            client_vpn_connection: c.client_vpn_connection,
            local_gateway: c.local_gateway,
            local_gateway_route_table: c.local_gateway_route_table,
            local_gateway_route_table_virtual_interface_group_association: c
                .local_gateway_route_table_virtual_interface_group_association,
            local_gateway_route_table_vpc_association: c.local_gateway_route_table_vpc_association,
            local_gateway_virtual_interface: c.local_gateway_virtual_interface,
            local_gateway_virtual_interface_group: c.local_gateway_virtual_interface_group,
            route_server: c.route_server,
            route_server_endpoint: c.route_server_endpoint,
            route_server_peer: c.route_server_peer,
            verified_access_instance: c.verified_access_instance,
            verified_access_trust_provider: c.verified_access_trust_provider,
            verified_access_group: c.verified_access_group,
            verified_access_endpoint: c.verified_access_endpoint,
            capacity_manager_data_export: c.capacity_manager_data_export,
            interruptible_capacity_reservation_allocation: c
                .interruptible_capacity_reservation_allocation,
            capacity_block: c.capacity_block,
            capacity_block_extension: c.capacity_block_extension,
            tgw_multicast_domain: c.tgw_multicast_domain,
            tgw_connect: c.tgw_connect,
            tgw_connect_peer: c.tgw_connect_peer,
            tgw_metering_policy: c.tgw_metering_policy,
            tgw_metering_policy_entry: c.tgw_metering_policy_entry,
            tgw_policy_table: c.tgw_policy_table,
            tgw_route_table_announcement: c.tgw_route_table_announcement,
            ipam: c.ipam,
            ipam_scope: c.ipam_scope,
            ipam_pool: c.ipam_pool,
            ipam_pool_cidr: c.ipam_pool_cidr,
            ipam_pool_allocation: c.ipam_pool_allocation,
            ipam_resource_discovery: c.ipam_resource_discovery,
            ipam_resource_discovery_association: c.ipam_resource_discovery_association,
            ipam_external_resource_verification_token: c.ipam_external_resource_verification_token,
            ipam_policy: c.ipam_policy,
            ipam_prefix_list_resolver: c.ipam_prefix_list_resolver,
            ipam_prefix_list_resolver_target: c.ipam_prefix_list_resolver_target,
            bundle_task: c.bundle_task,
            volume_modification: c.volume_modification,
            import_volume_task: c.import_volume_task,
            export_image_task: c.export_image_task,
            outpost_lag: c.outpost_lag,
        }
    }
}

impl From<&Vpc> for VpcView {
    fn from(v: &Vpc) -> Self {
        VpcView {
            vpc_id: v.vpc_id.clone(),
            cidr_block: v.cidr_block.clone(),
            state: v.state.clone(),
            dhcp_options_id: v.dhcp_options_id.clone(),
            instance_tenancy: v.instance_tenancy.clone(),
            is_default: v.is_default,
            enable_dns_hostnames: v.enable_dns_hostnames,
            enable_dns_support: v.enable_dns_support,
            secondary_cidr_blocks: v.secondary_cidr_blocks.clone(),
            tags: v.tags.clone(),
            classic_link_enabled: v.classic_link_enabled,
        }
    }
}

impl From<&Subnet> for SubnetView {
    fn from(s: &Subnet) -> Self {
        SubnetView {
            subnet_id: s.subnet_id.clone(),
            vpc_id: s.vpc_id.clone(),
            cidr_block: s.cidr_block.clone(),
            availability_zone: s.availability_zone.clone(),
            state: s.state.clone(),
            available_ip_address_count: s.available_ip_address_count,
            map_public_ip_on_launch: s.map_public_ip_on_launch,
            ipv6_cidr_blocks: s
                .ipv6_cidr_blocks
                .iter()
                .map(SubnetIpv6CidrAssocView::from)
                .collect(),
            tags: s.tags.clone(),
        }
    }
}

impl From<&SubnetIpv6CidrAssoc> for SubnetIpv6CidrAssocView {
    fn from(a: &SubnetIpv6CidrAssoc) -> Self {
        SubnetIpv6CidrAssocView {
            association_id: a.association_id.clone(),
            ipv6_cidr_block: a.ipv6_cidr_block.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&InternetGateway> for InternetGatewayView {
    fn from(igw: &InternetGateway) -> Self {
        InternetGatewayView {
            igw_id: igw.igw_id.clone(),
            attachments: igw
                .attachments
                .iter()
                .map(IgwAttachmentView::from)
                .collect(),
            tags: igw.tags.clone(),
        }
    }
}

impl From<&IgwAttachment> for IgwAttachmentView {
    fn from(a: &IgwAttachment) -> Self {
        IgwAttachmentView {
            vpc_id: a.vpc_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&SecurityGroup> for SecurityGroupView {
    fn from(sg: &SecurityGroup) -> Self {
        SecurityGroupView {
            group_id: sg.group_id.clone(),
            group_name: sg.group_name.clone(),
            description: sg.description.clone(),
            vpc_id: sg.vpc_id.clone(),
            owner_id: sg.owner_id.clone(),
            ingress_rules: sg
                .ingress_rules
                .iter()
                .map(IpPermissionView::from)
                .collect(),
            egress_rules: sg.egress_rules.iter().map(IpPermissionView::from).collect(),
            tags: sg.tags.clone(),
        }
    }
}

impl From<&IpPermission> for IpPermissionView {
    fn from(p: &IpPermission) -> Self {
        IpPermissionView {
            from_port: p.from_port,
            to_port: p.to_port,
            ip_protocol: p.ip_protocol.clone(),
            ip_ranges: p.ip_ranges.iter().map(IpRangeView::from).collect(),
            ipv6_ranges: p.ipv6_ranges.iter().map(Ipv6RangeView::from).collect(),
            user_id_group_pairs: p
                .user_id_group_pairs
                .iter()
                .map(UserIdGroupPairView::from)
                .collect(),
        }
    }
}

impl From<&IpRange> for IpRangeView {
    fn from(r: &IpRange) -> Self {
        IpRangeView {
            cidr_ip: r.cidr_ip.clone(),
            description: r.description.clone(),
        }
    }
}

impl From<&Ipv6Range> for Ipv6RangeView {
    fn from(r: &Ipv6Range) -> Self {
        Ipv6RangeView {
            cidr_ipv6: r.cidr_ipv6.clone(),
            description: r.description.clone(),
        }
    }
}

impl From<&UserIdGroupPair> for UserIdGroupPairView {
    fn from(p: &UserIdGroupPair) -> Self {
        UserIdGroupPairView {
            group_id: p.group_id.clone(),
            user_id: p.user_id.clone(),
        }
    }
}

impl From<&RouteTable> for RouteTableView {
    fn from(rtb: &RouteTable) -> Self {
        RouteTableView {
            route_table_id: rtb.route_table_id.clone(),
            vpc_id: rtb.vpc_id.clone(),
            routes: rtb.routes.iter().map(RouteView::from).collect(),
            associations: rtb
                .associations
                .iter()
                .map(RouteTableAssociationView::from)
                .collect(),
            propagating_vgws: rtb.propagating_vgws.clone(),
            tags: rtb.tags.clone(),
        }
    }
}

impl From<&Route> for RouteView {
    fn from(r: &Route) -> Self {
        RouteView {
            destination_cidr_block: r.destination_cidr_block.clone(),
            destination_ipv6_cidr_block: r.destination_ipv6_cidr_block.clone(),
            gateway_id: r.gateway_id.clone(),
            state: r.state.clone(),
            origin: r.origin.clone(),
        }
    }
}

impl From<&RouteTableAssociation> for RouteTableAssociationView {
    fn from(a: &RouteTableAssociation) -> Self {
        RouteTableAssociationView {
            association_id: a.association_id.clone(),
            subnet_id: a.subnet_id.clone(),
            gateway_id: a.gateway_id.clone(),
            main: a.main,
            state: a.state.clone(),
        }
    }
}

impl From<&KeyPair> for KeyPairView {
    fn from(kp: &KeyPair) -> Self {
        KeyPairView {
            key_pair_id: kp.key_pair_id.clone(),
            key_name: kp.key_name.clone(),
            fingerprint: kp.fingerprint.clone(),
            tags: kp.tags.clone(),
        }
    }
}

impl From<&NetworkAcl> for NetworkAclView {
    fn from(n: &NetworkAcl) -> Self {
        NetworkAclView {
            network_acl_id: n.network_acl_id.clone(),
            vpc_id: n.vpc_id.clone(),
            is_default: n.is_default,
            entries: n.entries.iter().map(NetworkAclEntryView::from).collect(),
            associations: n
                .associations
                .iter()
                .map(NetworkAclAssociationView::from)
                .collect(),
            tags: n.tags.clone(),
        }
    }
}

impl From<&NetworkAclEntry> for NetworkAclEntryView {
    fn from(e: &NetworkAclEntry) -> Self {
        NetworkAclEntryView {
            rule_number: e.rule_number,
            protocol: e.protocol.clone(),
            rule_action: e.rule_action.clone(),
            egress: e.egress,
            cidr_block: e.cidr_block.clone(),
            ipv6_cidr_block: e.ipv6_cidr_block.clone(),
            port_range: e.port_range.as_ref().map(PortRangeView::from),
            icmp_type_code: e.icmp_type_code.as_ref().map(IcmpTypeCodeView::from),
        }
    }
}

impl From<&PortRange> for PortRangeView {
    fn from(p: &PortRange) -> Self {
        PortRangeView {
            from: p.from,
            to: p.to,
        }
    }
}

impl From<&IcmpTypeCode> for IcmpTypeCodeView {
    fn from(i: &IcmpTypeCode) -> Self {
        IcmpTypeCodeView {
            type_num: i.type_num,
            code: i.code,
        }
    }
}

impl From<&NetworkAclAssociation> for NetworkAclAssociationView {
    fn from(a: &NetworkAclAssociation) -> Self {
        NetworkAclAssociationView {
            network_acl_association_id: a.network_acl_association_id.clone(),
            network_acl_id: a.network_acl_id.clone(),
            subnet_id: a.subnet_id.clone(),
        }
    }
}

impl From<&ElasticIp> for ElasticIpView {
    fn from(e: &ElasticIp) -> Self {
        ElasticIpView {
            allocation_id: e.allocation_id.clone(),
            public_ip: e.public_ip.clone(),
            association_id: e.association_id.clone(),
            instance_id: e.instance_id.clone(),
            network_interface_id: e.network_interface_id.clone(),
            private_ip_address: e.private_ip_address.clone(),
            address_attribute_ptr_record: e.address_attribute_ptr_record.clone(),
            domain: e.domain.clone(),
            pending_transfer: e.pending_transfer.clone(),
            tags: e.tags.clone(),
        }
    }
}

impl From<&NatGateway> for NatGatewayView {
    fn from(n: &NatGateway) -> Self {
        NatGatewayView {
            nat_gateway_id: n.nat_gateway_id.clone(),
            vpc_id: n.vpc_id.clone(),
            subnet_id: n.subnet_id.clone(),
            state: n.state.clone(),
            connectivity_type: n.connectivity_type.clone(),
            allocation_id: n.allocation_id.clone(),
            public_ip: n.public_ip.clone(),
            secondary_addresses: n
                .secondary_addresses
                .iter()
                .map(NatGatewayAddressAssociationView::from)
                .collect(),
            tags: n.tags.clone(),
        }
    }
}

impl From<&NatGatewayAddressAssociation> for NatGatewayAddressAssociationView {
    fn from(a: &NatGatewayAddressAssociation) -> Self {
        NatGatewayAddressAssociationView {
            allocation_id: a.allocation_id.clone(),
            association_id: a.association_id.clone(),
            network_interface_id: a.network_interface_id.clone(),
            private_ip: a.private_ip.clone(),
            public_ip: a.public_ip.clone(),
            status: a.status.clone(),
            is_primary: a.is_primary,
        }
    }
}

impl From<NatGatewayAddressAssociationView> for NatGatewayAddressAssociation {
    fn from(v: NatGatewayAddressAssociationView) -> Self {
        NatGatewayAddressAssociation {
            allocation_id: v.allocation_id,
            association_id: v.association_id,
            network_interface_id: v.network_interface_id,
            private_ip: v.private_ip,
            public_ip: v.public_ip,
            status: v.status,
            is_primary: v.is_primary,
        }
    }
}

impl From<&ByoipCidr> for ByoipCidrView {
    fn from(b: &ByoipCidr) -> Self {
        ByoipCidrView {
            cidr: b.cidr.clone(),
            description: b.description.clone(),
            state: b.state.clone(),
            asn_association: b.asn_association.as_ref().map(AsnAssociationView::from),
            ipam_pool_id: b.ipam_pool_id.clone(),
        }
    }
}

impl From<ByoipCidrView> for ByoipCidr {
    fn from(v: ByoipCidrView) -> Self {
        ByoipCidr {
            cidr: v.cidr,
            description: v.description,
            state: v.state,
            asn_association: v.asn_association.map(AsnAssociation::from),
            ipam_pool_id: v.ipam_pool_id,
        }
    }
}

impl From<&AsnAssociation> for AsnAssociationView {
    fn from(a: &AsnAssociation) -> Self {
        AsnAssociationView {
            asn: a.asn.clone(),
            cidr: a.cidr.clone(),
            state: a.state.clone(),
            status_message: a.status_message.clone(),
        }
    }
}

impl From<AsnAssociationView> for AsnAssociation {
    fn from(v: AsnAssociationView) -> Self {
        AsnAssociation {
            asn: v.asn,
            cidr: v.cidr,
            state: v.state,
            status_message: v.status_message,
        }
    }
}

impl From<&PublicIpv4Pool> for PublicIpv4PoolView {
    fn from(p: &PublicIpv4Pool) -> Self {
        PublicIpv4PoolView {
            pool_id: p.pool_id.clone(),
            description: p.description.clone(),
            network_border_group: p.network_border_group.clone(),
            total_address_count: p.total_address_count,
            total_available_address_count: p.total_available_address_count,
            pool_address_ranges: p
                .pool_address_ranges
                .iter()
                .map(PublicIpv4PoolRangeView::from)
                .collect(),
            tags: p.tags.clone(),
        }
    }
}

impl From<PublicIpv4PoolView> for PublicIpv4Pool {
    fn from(v: PublicIpv4PoolView) -> Self {
        PublicIpv4Pool {
            pool_id: v.pool_id,
            description: v.description,
            network_border_group: v.network_border_group,
            total_address_count: v.total_address_count,
            total_available_address_count: v.total_available_address_count,
            pool_address_ranges: v
                .pool_address_ranges
                .into_iter()
                .map(PublicIpv4PoolRange::from)
                .collect(),
            tags: v.tags,
        }
    }
}

impl From<&PublicIpv4PoolRange> for PublicIpv4PoolRangeView {
    fn from(r: &PublicIpv4PoolRange) -> Self {
        PublicIpv4PoolRangeView {
            first_address: r.first_address.clone(),
            last_address: r.last_address.clone(),
            address_count: r.address_count,
            available_address_count: r.available_address_count,
        }
    }
}

impl From<PublicIpv4PoolRangeView> for PublicIpv4PoolRange {
    fn from(v: PublicIpv4PoolRangeView) -> Self {
        PublicIpv4PoolRange {
            first_address: v.first_address,
            last_address: v.last_address,
            address_count: v.address_count,
            available_address_count: v.available_address_count,
        }
    }
}

impl From<&CoipCidr> for CoipCidrView {
    fn from(c: &CoipCidr) -> Self {
        CoipCidrView {
            cidr: c.cidr.clone(),
            coip_pool_id: c.coip_pool_id.clone(),
        }
    }
}

impl From<CoipCidrView> for CoipCidr {
    fn from(v: CoipCidrView) -> Self {
        CoipCidr {
            cidr: v.cidr,
            coip_pool_id: v.coip_pool_id,
        }
    }
}

impl From<&AddressTransfer> for AddressTransferView {
    fn from(a: &AddressTransfer) -> Self {
        AddressTransferView {
            allocation_id: a.allocation_id.clone(),
            public_ip: a.public_ip.clone(),
            transfer_account_id: a.transfer_account_id.clone(),
            transfer_offer_expiration_timestamp: a.transfer_offer_expiration_timestamp.clone(),
            transfer_offer_accepted_timestamp: a.transfer_offer_accepted_timestamp.clone(),
            address_transfer_status: a.address_transfer_status.clone(),
        }
    }
}

impl From<AddressTransferView> for AddressTransfer {
    fn from(v: AddressTransferView) -> Self {
        AddressTransfer {
            allocation_id: v.allocation_id,
            public_ip: v.public_ip,
            transfer_account_id: v.transfer_account_id,
            transfer_offer_expiration_timestamp: v.transfer_offer_expiration_timestamp,
            transfer_offer_accepted_timestamp: v.transfer_offer_accepted_timestamp,
            address_transfer_status: v.address_transfer_status,
        }
    }
}

impl From<&DhcpOptions> for DhcpOptionsView {
    fn from(d: &DhcpOptions) -> Self {
        DhcpOptionsView {
            dhcp_options_id: d.dhcp_options_id.clone(),
            configurations: d
                .configurations
                .iter()
                .map(DhcpConfigurationView::from)
                .collect(),
            tags: d.tags.clone(),
        }
    }
}

impl From<&DhcpConfiguration> for DhcpConfigurationView {
    fn from(c: &DhcpConfiguration) -> Self {
        DhcpConfigurationView {
            key: c.key.clone(),
            values: c.values.clone(),
        }
    }
}

impl From<&EgressOnlyInternetGateway> for EgressOnlyIgwView {
    fn from(e: &EgressOnlyInternetGateway) -> Self {
        EgressOnlyIgwView {
            eigw_id: e.eigw_id.clone(),
            state: e.state.clone(),
            attachments: e
                .attachments
                .iter()
                .map(EoigwAttachmentView::from)
                .collect(),
            tags: e.tags.clone(),
        }
    }
}

impl From<&EoigwAttachment> for EoigwAttachmentView {
    fn from(a: &EoigwAttachment) -> Self {
        EoigwAttachmentView {
            vpc_id: a.vpc_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&FlowLog> for FlowLogView {
    fn from(f: &FlowLog) -> Self {
        FlowLogView {
            flow_log_id: f.flow_log_id.clone(),
            resource_id: f.resource_id.clone(),
            traffic_type: f.traffic_type.clone(),
            log_destination_type: f.log_destination_type.clone(),
            log_destination: f.log_destination.clone(),
            log_group_name: f.log_group_name.clone(),
            deliver_logs_status: f.deliver_logs_status.clone(),
            flow_log_status: f.flow_log_status.clone(),
            tags: f.tags.clone(),
        }
    }
}

impl From<&VpcPeeringConnection> for VpcPeeringConnectionView {
    fn from(p: &VpcPeeringConnection) -> Self {
        VpcPeeringConnectionView {
            peering_id: p.peering_id.clone(),
            requester_vpc_id: p.requester_vpc_id.clone(),
            accepter_vpc_id: p.accepter_vpc_id.clone(),
            status: p.status.clone(),
            tags: p.tags.clone(),
            requester_peering_options: p.requester_peering_options.as_ref().map(|o| {
                VpcPeeringConnectionOptionsView {
                    allow_dns_resolution_from_remote_vpc: o.allow_dns_resolution_from_remote_vpc,
                    allow_egress_from_local_classic_link_to_remote_vpc: o
                        .allow_egress_from_local_classic_link_to_remote_vpc,
                    allow_egress_from_local_vpc_to_remote_classic_link: o
                        .allow_egress_from_local_vpc_to_remote_classic_link,
                }
            }),
            accepter_peering_options: p.accepter_peering_options.as_ref().map(|o| {
                VpcPeeringConnectionOptionsView {
                    allow_dns_resolution_from_remote_vpc: o.allow_dns_resolution_from_remote_vpc,
                    allow_egress_from_local_classic_link_to_remote_vpc: o
                        .allow_egress_from_local_classic_link_to_remote_vpc,
                    allow_egress_from_local_vpc_to_remote_classic_link: o
                        .allow_egress_from_local_vpc_to_remote_classic_link,
                }
            }),
        }
    }
}

impl From<&VpcEndpoint> for VpcEndpointView {
    fn from(e: &VpcEndpoint) -> Self {
        VpcEndpointView {
            endpoint_id: e.endpoint_id.clone(),
            vpc_id: e.vpc_id.clone(),
            service_name: e.service_name.clone(),
            endpoint_type: e.endpoint_type.clone(),
            state: e.state.clone(),
            policy_document: e.policy_document.clone(),
            route_table_ids: e.route_table_ids.clone(),
            subnet_ids: e.subnet_ids.clone(),
            security_group_ids: e.security_group_ids.clone(),
            private_dns_enabled: e.private_dns_enabled,
            tags: e.tags.clone(),
        }
    }
}

impl From<&ManagedPrefixList> for ManagedPrefixListView {
    fn from(m: &ManagedPrefixList) -> Self {
        ManagedPrefixListView {
            prefix_list_id: m.prefix_list_id.clone(),
            prefix_list_name: m.prefix_list_name.clone(),
            state: m.state.clone(),
            address_family: m.address_family.clone(),
            max_entries: m.max_entries,
            entries: m.entries.iter().map(PrefixListEntryView::from).collect(),
            tags: m.tags.clone(),
            version: m.version,
            version_history: m
                .version_history
                .iter()
                .map(|v| ManagedPrefixListVersionView {
                    version: v.version,
                    entries: v.entries.iter().map(PrefixListEntryView::from).collect(),
                })
                .collect(),
        }
    }
}

impl From<&TypesPrefixListEntry> for PrefixListEntryView {
    fn from(e: &TypesPrefixListEntry) -> Self {
        PrefixListEntryView {
            cidr: e.cidr.clone(),
            description: e.description.clone(),
        }
    }
}

impl From<&CustomerGateway> for CustomerGatewayView {
    fn from(c: &CustomerGateway) -> Self {
        CustomerGatewayView {
            customer_gateway_id: c.customer_gateway_id.clone(),
            bgp_asn: c.bgp_asn.clone(),
            ip_address: c.ip_address.clone(),
            gateway_type: c.gateway_type.clone(),
            state: c.state.clone(),
            tags: c.tags.clone(),
        }
    }
}

impl From<&VpnGateway> for VpnGatewayView {
    fn from(v: &VpnGateway) -> Self {
        VpnGatewayView {
            vpn_gateway_id: v.vpn_gateway_id.clone(),
            gateway_type: v.gateway_type.clone(),
            state: v.state.clone(),
            amazon_side_asn: v.amazon_side_asn,
            vpc_attachments: v
                .vpc_attachments
                .iter()
                .map(VgwVpcAttachmentView::from)
                .collect(),
            tags: v.tags.clone(),
        }
    }
}

impl From<&VgwVpcAttachment> for VgwVpcAttachmentView {
    fn from(a: &VgwVpcAttachment) -> Self {
        VgwVpcAttachmentView {
            vpc_id: a.vpc_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&VpnConnection> for VpnConnectionView {
    fn from(c: &VpnConnection) -> Self {
        VpnConnectionView {
            vpn_connection_id: c.vpn_connection_id.clone(),
            vpn_gateway_id: c.vpn_gateway_id.clone(),
            customer_gateway_id: c.customer_gateway_id.clone(),
            transit_gateway_id: c.transit_gateway_id.clone(),
            connection_type: c.connection_type.clone(),
            state: c.state.clone(),
            tags: c.tags.clone(),
            routes: c.routes.iter().map(VpnStaticRouteView::from).collect(),
            options: c.options.as_ref().map(VpnConnectionOptionsView::from),
            tunnel_replacement_status: c.tunnel_replacement_status.clone(),
        }
    }
}

impl From<&VpnStaticRoute> for VpnStaticRouteView {
    fn from(r: &VpnStaticRoute) -> Self {
        VpnStaticRouteView {
            destination_cidr_block: r.destination_cidr_block.clone(),
            source: r.source.clone(),
            state: r.state.clone(),
        }
    }
}

impl From<&VpnConnectionOptions> for VpnConnectionOptionsView {
    fn from(o: &VpnConnectionOptions) -> Self {
        VpnConnectionOptionsView {
            local_ipv4_network_cidr: o.local_ipv4_network_cidr.clone(),
            local_ipv6_network_cidr: o.local_ipv6_network_cidr.clone(),
            remote_ipv4_network_cidr: o.remote_ipv4_network_cidr.clone(),
            remote_ipv6_network_cidr: o.remote_ipv6_network_cidr.clone(),
            tunnel_inside_ip_version: o.tunnel_inside_ip_version.clone(),
            static_routes_only: o.static_routes_only,
            tunnel_options: o
                .tunnel_options
                .iter()
                .map(VpnTunnelOptionsView::from)
                .collect(),
        }
    }
}

impl From<&VpnTunnelOptions> for VpnTunnelOptionsView {
    fn from(t: &VpnTunnelOptions) -> Self {
        VpnTunnelOptionsView {
            tunnel_inside_cidr: t.tunnel_inside_cidr.clone(),
            tunnel_inside_ipv6_cidr: t.tunnel_inside_ipv6_cidr.clone(),
            pre_shared_key: t.pre_shared_key.clone(),
            outside_ip_address: t.outside_ip_address.clone(),
            certificate_arn: t.certificate_arn.clone(),
        }
    }
}

impl From<&VpnConcentrator> for VpnConcentratorView {
    fn from(v: &VpnConcentrator) -> Self {
        VpnConcentratorView {
            vpn_concentrator_id: v.vpn_concentrator_id.clone(),
            concentrator_type: v.concentrator_type.clone(),
            state: v.state.clone(),
            transit_gateway_id: v.transit_gateway_id.clone(),
            transit_gateway_attachment_id: v.transit_gateway_attachment_id.clone(),
            tags: v.tags.clone(),
        }
    }
}

impl From<&VpcEndpointConnection> for VpcEndpointConnectionView {
    fn from(c: &VpcEndpointConnection) -> Self {
        VpcEndpointConnectionView {
            service_id: c.service_id.clone(),
            vpc_endpoint_id: c.vpc_endpoint_id.clone(),
            vpc_endpoint_owner: c.vpc_endpoint_owner.clone(),
            vpc_endpoint_state: c.vpc_endpoint_state.clone(),
            creation_timestamp: c.creation_timestamp.clone(),
        }
    }
}

impl From<&VpcEndpointConnectionNotification> for VpcEndpointConnectionNotificationView {
    fn from(n: &VpcEndpointConnectionNotification) -> Self {
        VpcEndpointConnectionNotificationView {
            connection_notification_id: n.connection_notification_id.clone(),
            connection_notification_arn: n.connection_notification_arn.clone(),
            connection_events: n.connection_events.clone(),
            connection_notification_state: n.connection_notification_state.clone(),
            connection_notification_type: n.connection_notification_type.clone(),
            service_id: n.service_id.clone(),
            vpc_endpoint_id: n.vpc_endpoint_id.clone(),
        }
    }
}

impl From<&VpcBlockPublicAccessExclusion> for VpcBlockPublicAccessExclusionView {
    fn from(e: &VpcBlockPublicAccessExclusion) -> Self {
        VpcBlockPublicAccessExclusionView {
            exclusion_id: e.exclusion_id.clone(),
            internet_gateway_exclusion_mode: e.internet_gateway_exclusion_mode.clone(),
            resource_arn: e.resource_arn.clone(),
            state: e.state.clone(),
            creation_timestamp: e.creation_timestamp.clone(),
            last_update_timestamp: e.last_update_timestamp.clone(),
            tags: e.tags.clone(),
        }
    }
}

impl From<&VpcBlockPublicAccessOptions> for VpcBlockPublicAccessOptionsView {
    fn from(o: &VpcBlockPublicAccessOptions) -> Self {
        VpcBlockPublicAccessOptionsView {
            aws_account_id: o.aws_account_id.clone(),
            aws_region: o.aws_region.clone(),
            internet_gateway_block_mode: o.internet_gateway_block_mode.clone(),
            state: o.state.clone(),
            last_update_timestamp: o.last_update_timestamp.clone(),
        }
    }
}

impl From<&VpcEncryptionControl> for VpcEncryptionControlView {
    fn from(v: &VpcEncryptionControl) -> Self {
        VpcEncryptionControlView {
            vpc_encryption_control_id: v.vpc_encryption_control_id.clone(),
            vpc_id: v.vpc_id.clone(),
            mode: v.mode.clone(),
            state: v.state.clone(),
            mode_history: v.mode_history.clone(),
            tags: v.tags.clone(),
        }
    }
}

impl From<&CarrierGateway> for CarrierGatewayView {
    fn from(c: &CarrierGateway) -> Self {
        CarrierGatewayView {
            carrier_gateway_id: c.carrier_gateway_id.clone(),
            vpc_id: c.vpc_id.clone(),
            state: c.state.clone(),
            tags: c.tags.clone(),
        }
    }
}

impl From<&NetworkInterface> for NetworkInterfaceView {
    fn from(n: &NetworkInterface) -> Self {
        NetworkInterfaceView {
            network_interface_id: n.network_interface_id.clone(),
            subnet_id: n.subnet_id.clone(),
            vpc_id: n.vpc_id.clone(),
            description: n.description.clone(),
            private_ip_address: n.private_ip_address.clone(),
            status: n.status.clone(),
            attachment_id: n.attachment_id.clone(),
            instance_id: n.instance_id.clone(),
            device_index: n.device_index,
            security_groups: n.security_groups.clone(),
            source_dest_check: n.source_dest_check,
            tags: n.tags.clone(),
            public_ip_dns_hostname_type: n.public_ip_dns_hostname_type.clone(),
        }
    }
}

impl From<&TransitGateway> for TransitGatewayView {
    fn from(t: &TransitGateway) -> Self {
        TransitGatewayView {
            transit_gateway_id: t.transit_gateway_id.clone(),
            state: t.state.clone(),
            amazon_side_asn: t.amazon_side_asn,
            description: t.description.clone(),
            dns_support: t.dns_support.clone(),
            vpn_ecmp_support: t.vpn_ecmp_support.clone(),
            multicast_support: t.multicast_support.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<&TransitGatewayVpcAttachment> for TgwVpcAttachmentView {
    fn from(a: &TransitGatewayVpcAttachment) -> Self {
        TgwVpcAttachmentView {
            attachment_id: a.attachment_id.clone(),
            transit_gateway_id: a.transit_gateway_id.clone(),
            vpc_id: a.vpc_id.clone(),
            subnet_ids: a.subnet_ids.clone(),
            state: a.state.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<&TransitGatewayPeeringAttachment> for TgwPeeringAttachmentView {
    fn from(a: &TransitGatewayPeeringAttachment) -> Self {
        TgwPeeringAttachmentView {
            attachment_id: a.attachment_id.clone(),
            transit_gateway_id: a.transit_gateway_id.clone(),
            peer_transit_gateway_id: a.peer_transit_gateway_id.clone(),
            peer_account_id: a.peer_account_id.clone(),
            peer_region: a.peer_region.clone(),
            state: a.state.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<&TransitGatewayRouteTable> for TgwRouteTableView {
    fn from(t: &TransitGatewayRouteTable) -> Self {
        TgwRouteTableView {
            route_table_id: t.route_table_id.clone(),
            transit_gateway_id: t.transit_gateway_id.clone(),
            state: t.state.clone(),
            default_association_route_table: t.default_association_route_table,
            default_propagation_route_table: t.default_propagation_route_table,
            tags: t.tags.clone(),
        }
    }
}

impl From<&TransitGatewayRoute> for TgwRouteView {
    fn from(r: &TransitGatewayRoute) -> Self {
        TgwRouteView {
            destination_cidr_block: r.destination_cidr_block.clone(),
            route_type: r.route_type.clone(),
            state: r.state.clone(),
            attachment_id: r.attachment_id.clone(),
        }
    }
}

// --- Group 11: Transit Gateway extension From impls ---

impl From<&TransitGatewayMulticastDomain> for TransitGatewayMulticastDomainView {
    fn from(d: &TransitGatewayMulticastDomain) -> Self {
        TransitGatewayMulticastDomainView {
            transit_gateway_multicast_domain_id: d.transit_gateway_multicast_domain_id.clone(),
            transit_gateway_id: d.transit_gateway_id.clone(),
            transit_gateway_multicast_domain_arn: d.transit_gateway_multicast_domain_arn.clone(),
            owner_id: d.owner_id.clone(),
            igmpv2_support: d.igmpv2_support.clone(),
            static_sources_support: d.static_sources_support.clone(),
            auto_accept_shared_associations: d.auto_accept_shared_associations.clone(),
            state: d.state.clone(),
            creation_time: d.creation_time.clone(),
            tags: d.tags.clone(),
        }
    }
}

impl From<&MulticastSubnetAssociation> for MulticastSubnetAssociationView {
    fn from(s: &MulticastSubnetAssociation) -> Self {
        MulticastSubnetAssociationView {
            subnet_id: s.subnet_id.clone(),
            state: s.state.clone(),
        }
    }
}

impl From<&TransitGatewayMulticastDomainAssociation>
    for TransitGatewayMulticastDomainAssociationView
{
    fn from(a: &TransitGatewayMulticastDomainAssociation) -> Self {
        TransitGatewayMulticastDomainAssociationView {
            transit_gateway_multicast_domain_id: a.transit_gateway_multicast_domain_id.clone(),
            transit_gateway_attachment_id: a.transit_gateway_attachment_id.clone(),
            resource_id: a.resource_id.clone(),
            resource_type: a.resource_type.clone(),
            subnets: a
                .subnets
                .iter()
                .map(MulticastSubnetAssociationView::from)
                .collect(),
        }
    }
}

impl From<&TransitGatewayMulticastGroupMember> for TransitGatewayMulticastGroupMemberView {
    fn from(m: &TransitGatewayMulticastGroupMember) -> Self {
        TransitGatewayMulticastGroupMemberView {
            transit_gateway_multicast_domain_id: m.transit_gateway_multicast_domain_id.clone(),
            group_ip_address: m.group_ip_address.clone(),
            transit_gateway_attachment_id: m.transit_gateway_attachment_id.clone(),
            subnet_id: m.subnet_id.clone(),
            resource_id: m.resource_id.clone(),
            resource_type: m.resource_type.clone(),
            network_interface_id: m.network_interface_id.clone(),
            member_type: m.member_type.clone(),
            source_type: m.source_type.clone(),
        }
    }
}

impl From<&TransitGatewayMulticastGroupSource> for TransitGatewayMulticastGroupSourceView {
    fn from(s: &TransitGatewayMulticastGroupSource) -> Self {
        TransitGatewayMulticastGroupSourceView {
            transit_gateway_multicast_domain_id: s.transit_gateway_multicast_domain_id.clone(),
            group_ip_address: s.group_ip_address.clone(),
            transit_gateway_attachment_id: s.transit_gateway_attachment_id.clone(),
            subnet_id: s.subnet_id.clone(),
            resource_id: s.resource_id.clone(),
            resource_type: s.resource_type.clone(),
            network_interface_id: s.network_interface_id.clone(),
            member_type: s.member_type.clone(),
            source_type: s.source_type.clone(),
        }
    }
}

impl From<&TransitGatewayConnect> for TransitGatewayConnectView {
    fn from(c: &TransitGatewayConnect) -> Self {
        TransitGatewayConnectView {
            transit_gateway_attachment_id: c.transit_gateway_attachment_id.clone(),
            transport_transit_gateway_attachment_id: c
                .transport_transit_gateway_attachment_id
                .clone(),
            transit_gateway_id: c.transit_gateway_id.clone(),
            state: c.state.clone(),
            creation_time: c.creation_time.clone(),
            protocol: c.protocol.clone(),
            tags: c.tags.clone(),
        }
    }
}

impl From<&TransitGatewayAttachmentBgpConfiguration>
    for TransitGatewayAttachmentBgpConfigurationView
{
    fn from(b: &TransitGatewayAttachmentBgpConfiguration) -> Self {
        TransitGatewayAttachmentBgpConfigurationView {
            transit_gateway_asn: b.transit_gateway_asn,
            peer_asn: b.peer_asn,
            transit_gateway_address: b.transit_gateway_address.clone(),
            peer_address: b.peer_address.clone(),
            bgp_status: b.bgp_status.clone(),
        }
    }
}

impl From<&TransitGatewayConnectPeer> for TransitGatewayConnectPeerView {
    fn from(p: &TransitGatewayConnectPeer) -> Self {
        TransitGatewayConnectPeerView {
            transit_gateway_attachment_id: p.transit_gateway_attachment_id.clone(),
            transit_gateway_connect_peer_id: p.transit_gateway_connect_peer_id.clone(),
            state: p.state.clone(),
            creation_time: p.creation_time.clone(),
            transit_gateway_address: p.transit_gateway_address.clone(),
            peer_address: p.peer_address.clone(),
            inside_cidr_blocks: p.inside_cidr_blocks.clone(),
            protocol: p.protocol.clone(),
            bgp_configurations: p
                .bgp_configurations
                .iter()
                .map(TransitGatewayAttachmentBgpConfigurationView::from)
                .collect(),
            tags: p.tags.clone(),
        }
    }
}

impl From<&TransitGatewayMeteringPolicy> for TransitGatewayMeteringPolicyView {
    fn from(p: &TransitGatewayMeteringPolicy) -> Self {
        TransitGatewayMeteringPolicyView {
            transit_gateway_metering_policy_id: p.transit_gateway_metering_policy_id.clone(),
            transit_gateway_metering_policy_arn: p.transit_gateway_metering_policy_arn.clone(),
            transit_gateway_id: p.transit_gateway_id.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
            state: p.state.clone(),
            tags: p.tags.clone(),
            last_updated_time: p.last_updated_time.clone(),
            version: p.version,
            middlebox_attachment_ids: p.middlebox_attachment_ids.clone(),
        }
    }
}

impl From<&TransitGatewayMeteringPolicyEntry> for TransitGatewayMeteringPolicyEntryView {
    fn from(e: &TransitGatewayMeteringPolicyEntry) -> Self {
        TransitGatewayMeteringPolicyEntryView {
            transit_gateway_metering_policy_entry_id: e
                .transit_gateway_metering_policy_entry_id
                .clone(),
            transit_gateway_metering_policy_id: e.transit_gateway_metering_policy_id.clone(),
            sequence_number: e.sequence_number,
            action: e.action.clone(),
            source_cidr_block: e.source_cidr_block.clone(),
            destination_cidr_block: e.destination_cidr_block.clone(),
            protocol: e.protocol.clone(),
            source_port: e.source_port.clone(),
            destination_port: e.destination_port.clone(),
            dimensions: e.dimensions.clone(),
            state: e.state.clone(),
        }
    }
}

impl From<&TransitGatewayPolicyTable> for TransitGatewayPolicyTableView {
    fn from(p: &TransitGatewayPolicyTable) -> Self {
        TransitGatewayPolicyTableView {
            transit_gateway_policy_table_id: p.transit_gateway_policy_table_id.clone(),
            transit_gateway_id: p.transit_gateway_id.clone(),
            state: p.state.clone(),
            creation_time: p.creation_time.clone(),
            tags: p.tags.clone(),
        }
    }
}

impl From<&TransitGatewayPolicyTableAssociation> for TransitGatewayPolicyTableAssociationView {
    fn from(a: &TransitGatewayPolicyTableAssociation) -> Self {
        TransitGatewayPolicyTableAssociationView {
            transit_gateway_policy_table_id: a.transit_gateway_policy_table_id.clone(),
            transit_gateway_attachment_id: a.transit_gateway_attachment_id.clone(),
            resource_type: a.resource_type.clone(),
            resource_id: a.resource_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&TransitGatewayPrefixListReference> for TransitGatewayPrefixListReferenceView {
    fn from(r: &TransitGatewayPrefixListReference) -> Self {
        TransitGatewayPrefixListReferenceView {
            transit_gateway_route_table_id: r.transit_gateway_route_table_id.clone(),
            prefix_list_id: r.prefix_list_id.clone(),
            prefix_list_owner_id: r.prefix_list_owner_id.clone(),
            state: r.state.clone(),
            blackhole: r.blackhole,
            transit_gateway_attachment_id: r.transit_gateway_attachment_id.clone(),
            resource_id: r.resource_id.clone(),
            resource_type: r.resource_type.clone(),
        }
    }
}

impl From<&TransitGatewayRouteTableAnnouncement> for TransitGatewayRouteTableAnnouncementView {
    fn from(a: &TransitGatewayRouteTableAnnouncement) -> Self {
        TransitGatewayRouteTableAnnouncementView {
            transit_gateway_route_table_announcement_id: a
                .transit_gateway_route_table_announcement_id
                .clone(),
            transit_gateway_id: a.transit_gateway_id.clone(),
            core_network_id: a.core_network_id.clone(),
            peer_transit_gateway_id: a.peer_transit_gateway_id.clone(),
            peer_core_network_id: a.peer_core_network_id.clone(),
            peering_attachment_id: a.peering_attachment_id.clone(),
            announcement_direction: a.announcement_direction.clone(),
            transit_gateway_route_table_id: a.transit_gateway_route_table_id.clone(),
            state: a.state.clone(),
            creation_time: a.creation_time.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<&Instance> for InstanceView {
    fn from(i: &Instance) -> Self {
        InstanceView {
            instance_id: i.instance_id.clone(),
            image_id: i.image_id.clone(),
            instance_type: i.instance_type.clone(),
            state: InstanceStateView {
                code: i.state.code,
                name: i.state.name.clone(),
            },
            private_ip_address: i.private_ip_address.clone(),
            public_ip_address: i.public_ip_address.clone(),
            subnet_id: i.subnet_id.clone(),
            vpc_id: i.vpc_id.clone(),
            key_name: i.key_name.clone(),
            security_groups: i.security_groups.clone(),
            launch_time: i.launch_time.clone(),
            tags: i.tags.clone(),
            iam_instance_profile_arn: i.iam_instance_profile_arn.clone(),
            monitoring_state: i.monitoring_state.clone(),
            placement_az: i.placement_az.clone(),
            placement_group_name: i.placement_group_name.clone(),
            placement_tenancy: i.placement_tenancy.clone(),
            placement_host_id: i.placement_host_id.clone(),
            placement_affinity: i.placement_affinity.clone(),
            placement_partition_number: i.placement_partition_number,
            owner_id: i.owner_id.clone(),
            classic_link_vpc: i.classic_link_vpc.clone(),
            private_dns_hostname_type: i.private_dns_hostname_type.clone(),
            enable_resource_name_dns_a_record: i.enable_resource_name_dns_a_record,
            enable_resource_name_dns_aaaa_record: i.enable_resource_name_dns_aaaa_record,
            credit_specification: i.credit_specification.clone(),
            cpu_options: i.cpu_options.as_ref().map(|c| InstanceCpuOptionsView {
                core_count: c.core_count,
                threads_per_core: c.threads_per_core,
                amd_sev_snp: c.amd_sev_snp.clone(),
            }),
            maintenance_options: i.maintenance_options.as_ref().map(|m| {
                InstanceMaintenanceOptionsView {
                    auto_recovery: m.auto_recovery.clone(),
                    reboot_migration: m.reboot_migration.clone(),
                }
            }),
            network_bandwidth_weighting: i.network_bandwidth_weighting.clone(),
            lifecycle: i.lifecycle.clone(),
            product_codes: i.product_codes.clone(),
            capacity_reservation_specification: i
                .capacity_reservation_specification
                .as_ref()
                .map(CapacityReservationSpecificationResponseView::from),
        }
    }
}

impl From<&VolumeAttachment> for VolumeAttachmentView {
    fn from(a: &VolumeAttachment) -> Self {
        VolumeAttachmentView {
            volume_id: a.volume_id.clone(),
            instance_id: a.instance_id.clone(),
            device: a.device.clone(),
            state: a.state.clone(),
            attach_time: a.attach_time.clone(),
            delete_on_termination: a.delete_on_termination,
        }
    }
}

impl From<&Volume> for VolumeView {
    fn from(v: &Volume) -> Self {
        VolumeView {
            volume_id: v.volume_id.clone(),
            size: v.size,
            snapshot_id: v.snapshot_id.clone(),
            availability_zone: v.availability_zone.clone(),
            state: v.state.clone(),
            volume_type: v.volume_type.clone(),
            iops: v.iops,
            throughput: v.throughput,
            encrypted: v.encrypted,
            create_time: v.create_time.clone(),
            attachments: v
                .attachments
                .iter()
                .map(VolumeAttachmentView::from)
                .collect(),
            tags: v.tags.clone(),
            recycle_bin_state: v.recycle_bin_state.clone(),
            source_volume_id: v.source_volume_id.clone(),
        }
    }
}

impl From<&Snapshot> for SnapshotView {
    fn from(s: &Snapshot) -> Self {
        SnapshotView {
            snapshot_id: s.snapshot_id.clone(),
            volume_id: s.volume_id.clone(),
            volume_size: s.volume_size,
            state: s.state.clone(),
            description: s.description.clone(),
            start_time: s.start_time.clone(),
            progress: s.progress.clone(),
            owner_id: s.owner_id.clone(),
            encrypted: s.encrypted,
            tags: s.tags.clone(),
            lock_state: s.lock_state.clone(),
            lock_duration: s.lock_duration,
            lock_created_on: s.lock_created_on.clone(),
            lock_expires_on: s.lock_expires_on.clone(),
            lock_duration_start_time: s.lock_duration_start_time.clone(),
            cool_off_period: s.cool_off_period,
            cool_off_period_expires_on: s.cool_off_period_expires_on.clone(),
            storage_tier: s.storage_tier.clone(),
            last_tiering_operation_status: s.last_tiering_operation_status.clone(),
            fast_snapshot_restore_states: s
                .fast_snapshot_restore_states
                .iter()
                .map(|f| FastSnapshotRestoreStateView {
                    availability_zone: f.availability_zone.clone(),
                    state: f.state.clone(),
                })
                .collect(),
        }
    }
}

impl From<&Image> for ImageView {
    fn from(i: &Image) -> Self {
        ImageView {
            image_id: i.image_id.clone(),
            name: i.name.clone(),
            description: i.description.clone(),
            state: i.state.clone(),
            owner_id: i.owner_id.clone(),
            architecture: i.architecture.clone(),
            image_type: i.image_type.clone(),
            platform: i.platform.clone(),
            virtualization_type: i.virtualization_type.clone(),
            root_device_type: i.root_device_type.clone(),
            root_device_name: i.root_device_name.clone(),
            public: i.public,
            tags: i.tags.clone(),
            source_instance_id: i.source_instance_id.clone(),
            source_instance_type: i.source_instance_type.clone(),
            launch_permissions: i
                .launch_permissions
                .iter()
                .map(LaunchPermissionView::from)
                .collect(),
            recycle_bin_state: i.recycle_bin_state.clone(),
            deprecation_time: i.deprecation_time.clone(),
            recycle_bin_enter_time: i.recycle_bin_enter_time.clone(),
            product_codes: i.product_codes.clone(),
            fast_launch_state: i.fast_launch_state.as_ref().map(FastLaunchStateView::from),
            deregistration_protection: i.deregistration_protection.clone(),
        }
    }
}

impl From<&crate::types::FastLaunchState> for FastLaunchStateView {
    fn from(f: &crate::types::FastLaunchState) -> Self {
        FastLaunchStateView {
            state: f.state.clone(),
            image_id: f.image_id.clone(),
            resource_type: f.resource_type.clone(),
            snapshot_configuration: SnapshotConfigurationRequestView {
                target_resource_count: f.snapshot_configuration.target_resource_count,
            },
            launch_template: FastLaunchLaunchTemplateSpecificationView {
                launch_template_id: f.launch_template.launch_template_id.clone(),
                launch_template_name: f.launch_template.launch_template_name.clone(),
                version: f.launch_template.version.clone(),
            },
            max_parallel_launches: f.max_parallel_launches,
            owner_id: f.owner_id.clone(),
            state_transition_time: f.state_transition_time.clone(),
        }
    }
}

impl From<FastLaunchStateView> for crate::types::FastLaunchState {
    fn from(v: FastLaunchStateView) -> Self {
        crate::types::FastLaunchState {
            state: v.state,
            image_id: v.image_id,
            resource_type: v.resource_type,
            snapshot_configuration: crate::types::SnapshotConfigurationRequest {
                target_resource_count: v.snapshot_configuration.target_resource_count,
            },
            launch_template: crate::types::FastLaunchLaunchTemplateSpecification {
                launch_template_id: v.launch_template.launch_template_id,
                launch_template_name: v.launch_template.launch_template_name,
                version: v.launch_template.version,
            },
            max_parallel_launches: v.max_parallel_launches,
            owner_id: v.owner_id,
            state_transition_time: v.state_transition_time,
        }
    }
}

impl From<&crate::types::LaunchPermission> for LaunchPermissionView {
    fn from(p: &crate::types::LaunchPermission) -> Self {
        LaunchPermissionView {
            user_id: p.user_id.clone(),
            group: p.group.clone(),
        }
    }
}

impl From<&LaunchTemplate> for LaunchTemplateView {
    fn from(t: &LaunchTemplate) -> Self {
        LaunchTemplateView {
            launch_template_id: t.launch_template_id.clone(),
            launch_template_name: t.launch_template_name.clone(),
            default_version_number: t.default_version_number,
            latest_version_number: t.latest_version_number,
            tags: t.tags.clone(),
        }
    }
}

impl From<&LaunchTemplateVersion> for LaunchTemplateVersionView {
    fn from(v: &LaunchTemplateVersion) -> Self {
        LaunchTemplateVersionView {
            version_number: v.version_number,
            launch_template_id: v.launch_template_id.clone(),
            launch_template_name: v.launch_template_name.clone(),
            version_description: v.version_description.clone(),
            data: v.data.clone(),
            default_version: v.default_version,
        }
    }
}

impl From<&SpotInstanceRequest> for SpotInstanceRequestView {
    fn from(s: &SpotInstanceRequest) -> Self {
        SpotInstanceRequestView {
            spot_instance_request_id: s.spot_instance_request_id.clone(),
            spot_price: s.spot_price.clone(),
            instance_type: s.instance_type.clone(),
            image_id: s.image_id.clone(),
            state: s.state.clone(),
            status_code: s.status_code.clone(),
            instance_id: s.instance_id.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<&IamInstanceProfileAssociation> for IamInstanceProfileAssociationView {
    fn from(a: &IamInstanceProfileAssociation) -> Self {
        IamInstanceProfileAssociationView {
            association_id: a.association_id.clone(),
            instance_id: a.instance_id.clone(),
            iam_instance_profile_arn: a.iam_instance_profile_arn.clone(),
            iam_instance_profile_name: a.iam_instance_profile_name.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<&DedicatedHost> for DedicatedHostView {
    fn from(h: &DedicatedHost) -> Self {
        DedicatedHostView {
            host_id: h.host_id.clone(),
            availability_zone: h.availability_zone.clone(),
            instance_type: h.instance_type.clone(),
            auto_placement: h.auto_placement.clone(),
            host_recovery: h.host_recovery.clone(),
            state: h.state.clone(),
            tags: h.tags.clone(),
        }
    }
}

impl From<&Ec2Fleet> for Ec2FleetView {
    fn from(f: &Ec2Fleet) -> Self {
        Ec2FleetView {
            fleet_id: f.fleet_id.clone(),
            state: f.state.clone(),
            fleet_type: f.fleet_type.clone(),
            create_time: f.create_time.clone(),
            tags: f.tags.clone(),
            total_target_capacity: f.total_target_capacity,
            on_demand_target_capacity: f.on_demand_target_capacity,
            spot_target_capacity: f.spot_target_capacity,
            context: f.context.clone(),
        }
    }
}

impl From<&VpcEndpointServiceConfiguration> for VpcEndpointServiceConfigView {
    fn from(s: &VpcEndpointServiceConfiguration) -> Self {
        VpcEndpointServiceConfigView {
            service_id: s.service_id.clone(),
            service_name: s.service_name.clone(),
            service_type: s.service_type.clone(),
            acceptance_required: s.acceptance_required,
            state: s.state.clone(),
            network_load_balancer_arns: s.network_load_balancer_arns.clone(),
            gateway_load_balancer_arns: s.gateway_load_balancer_arns.clone(),
            allowed_principals: s.allowed_principals.clone(),
            tags: s.tags.clone(),
            payer_responsibility: s.payer_responsibility.clone(),
            private_dns_state: s.private_dns_state.clone(),
        }
    }
}

impl From<&SpotFleetRequest> for SpotFleetRequestView {
    fn from(s: &SpotFleetRequest) -> Self {
        SpotFleetRequestView {
            spot_fleet_request_id: s.spot_fleet_request_id.clone(),
            spot_fleet_request_state: s.spot_fleet_request_state.clone(),
            target_capacity: s.target_capacity,
            iam_fleet_role: s.iam_fleet_role.clone(),
            create_time: s.create_time.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<&SubnetCidrReservationEntry> for SubnetCidrReservationView {
    fn from(r: &SubnetCidrReservationEntry) -> Self {
        SubnetCidrReservationView {
            reservation_id: r.reservation_id.clone(),
            subnet_id: r.subnet_id.clone(),
            cidr: r.cidr.clone(),
            reservation_type: r.reservation_type.clone(),
            description: r.description.clone(),
            owner_id: r.owner_id.clone(),
        }
    }
}

impl From<&PlacementGroup> for PlacementGroupView {
    fn from(p: &PlacementGroup) -> Self {
        PlacementGroupView {
            group_id: p.group_id.clone(),
            group_name: p.group_name.clone(),
            group_arn: p.group_arn.clone(),
            strategy: p.strategy.clone(),
            state: p.state.clone(),
            partition_count: p.partition_count,
            spread_level: p.spread_level.clone(),
            tags: p.tags.clone(),
        }
    }
}

impl From<&NetworkInterfacePermission> for NetworkInterfacePermissionView {
    fn from(p: &NetworkInterfacePermission) -> Self {
        NetworkInterfacePermissionView {
            network_interface_permission_id: p.network_interface_permission_id.clone(),
            network_interface_id: p.network_interface_id.clone(),
            aws_account_id: p.aws_account_id.clone(),
            aws_service: p.aws_service.clone(),
            permission: p.permission.clone(),
            permission_state: p.permission_state.clone(),
        }
    }
}

impl From<&CapacityReservation> for CapacityReservationView {
    fn from(c: &CapacityReservation) -> Self {
        CapacityReservationView {
            capacity_reservation_id: c.capacity_reservation_id.clone(),
            capacity_reservation_arn: c.capacity_reservation_arn.clone(),
            owner_id: c.owner_id.clone(),
            instance_type: c.instance_type.clone(),
            instance_platform: c.instance_platform.clone(),
            availability_zone: c.availability_zone.clone(),
            tenancy: c.tenancy.clone(),
            total_instance_count: c.total_instance_count,
            available_instance_count: c.available_instance_count,
            ebs_optimized: c.ebs_optimized,
            ephemeral_storage: c.ephemeral_storage,
            state: c.state.clone(),
            start_date: c.start_date.clone(),
            end_date: c.end_date.clone(),
            end_date_type: c.end_date_type.clone(),
            instance_match_criteria: c.instance_match_criteria.clone(),
            create_date: c.create_date.clone(),
            outpost_arn: c.outpost_arn.clone(),
            placement_group_arn: c.placement_group_arn.clone(),
            tags: c.tags.clone(),
            pending_billing_owner_account_id: c.pending_billing_owner_account_id.clone(),
            billing_owner_account_id: c.billing_owner_account_id.clone(),
            target_capacity_reservation_id: c.target_capacity_reservation_id.clone(),
            reservation_type: c.reservation_type.clone(),
            commitment_info: c
                .commitment_info
                .as_ref()
                .map(CapacityReservationCommitmentInfoView::from),
        }
    }
}

impl From<&CapacityReservationCommitmentInfo> for CapacityReservationCommitmentInfoView {
    fn from(c: &CapacityReservationCommitmentInfo) -> Self {
        CapacityReservationCommitmentInfoView {
            commitment_end_date: c.commitment_end_date.clone(),
            committed_instance_count: c.committed_instance_count,
        }
    }
}

impl From<CapacityReservationCommitmentInfoView> for CapacityReservationCommitmentInfo {
    fn from(c: CapacityReservationCommitmentInfoView) -> Self {
        CapacityReservationCommitmentInfo {
            commitment_end_date: c.commitment_end_date,
            committed_instance_count: c.committed_instance_count,
        }
    }
}

impl From<&CapacityReservationSpecificationResponse>
    for CapacityReservationSpecificationResponseView
{
    fn from(s: &CapacityReservationSpecificationResponse) -> Self {
        CapacityReservationSpecificationResponseView {
            capacity_reservation_preference: s.capacity_reservation_preference.clone(),
            capacity_reservation_id: s.capacity_reservation_id.clone(),
            capacity_reservation_resource_group_arn: s
                .capacity_reservation_resource_group_arn
                .clone(),
        }
    }
}

impl From<CapacityReservationSpecificationResponseView>
    for CapacityReservationSpecificationResponse
{
    fn from(s: CapacityReservationSpecificationResponseView) -> Self {
        CapacityReservationSpecificationResponse {
            capacity_reservation_preference: s.capacity_reservation_preference,
            capacity_reservation_id: s.capacity_reservation_id,
            capacity_reservation_resource_group_arn: s.capacity_reservation_resource_group_arn,
        }
    }
}

impl From<&BillingOwnershipOffer> for BillingOwnershipOfferView {
    fn from(o: &BillingOwnershipOffer) -> Self {
        BillingOwnershipOfferView {
            capacity_reservation_id: o.capacity_reservation_id.clone(),
            unused_reservation_billing_owner_id: o.unused_reservation_billing_owner_id.clone(),
            requested_by: o.requested_by.clone(),
            status: o.status.clone(),
            status_message: o.status_message.clone(),
            last_update_time: o.last_update_time.clone(),
        }
    }
}

impl From<BillingOwnershipOfferView> for BillingOwnershipOffer {
    fn from(o: BillingOwnershipOfferView) -> Self {
        BillingOwnershipOffer {
            capacity_reservation_id: o.capacity_reservation_id,
            unused_reservation_billing_owner_id: o.unused_reservation_billing_owner_id,
            requested_by: o.requested_by,
            status: o.status,
            status_message: o.status_message,
            last_update_time: o.last_update_time,
        }
    }
}

impl From<&S3DestinationOptions> for S3DestinationOptionsView {
    fn from(s: &S3DestinationOptions) -> Self {
        S3DestinationOptionsView {
            bucket: s.bucket.clone(),
            prefix: s.prefix.clone(),
        }
    }
}

impl From<S3DestinationOptionsView> for S3DestinationOptions {
    fn from(s: S3DestinationOptionsView) -> Self {
        S3DestinationOptions {
            bucket: s.bucket,
            prefix: s.prefix,
        }
    }
}

impl From<&CapacityManagerDataExport> for CapacityManagerDataExportView {
    fn from(e: &CapacityManagerDataExport) -> Self {
        CapacityManagerDataExportView {
            data_export_id: e.data_export_id.clone(),
            schedule: e.schedule.clone(),
            organization_account_ids: e.organization_account_ids.clone(),
            output_format: e.output_format.clone(),
            s3_destination: S3DestinationOptionsView::from(&e.s3_destination),
            status: e.status.clone(),
            status_message: e.status_message.clone(),
            last_export_time: e.last_export_time.clone(),
            next_export_time: e.next_export_time.clone(),
            create_time: e.create_time.clone(),
            tags: e.tags.clone(),
        }
    }
}

impl From<CapacityManagerDataExportView> for CapacityManagerDataExport {
    fn from(e: CapacityManagerDataExportView) -> Self {
        CapacityManagerDataExport {
            data_export_id: e.data_export_id,
            schedule: e.schedule,
            organization_account_ids: e.organization_account_ids,
            output_format: e.output_format,
            s3_destination: S3DestinationOptions::from(e.s3_destination),
            status: e.status,
            status_message: e.status_message,
            last_export_time: e.last_export_time,
            next_export_time: e.next_export_time,
            create_time: e.create_time,
            tags: e.tags,
        }
    }
}

impl From<&CapacityManagerOrganizationsAccess> for CapacityManagerOrganizationsAccessView {
    fn from(a: &CapacityManagerOrganizationsAccess) -> Self {
        CapacityManagerOrganizationsAccessView {
            state: a.state.clone(),
            last_updated_time: a.last_updated_time.clone(),
        }
    }
}

impl From<CapacityManagerOrganizationsAccessView> for CapacityManagerOrganizationsAccess {
    fn from(a: CapacityManagerOrganizationsAccessView) -> Self {
        CapacityManagerOrganizationsAccess {
            state: a.state,
            last_updated_time: a.last_updated_time,
        }
    }
}

impl From<&InterruptibleCapacityReservationAllocation>
    for InterruptibleCapacityReservationAllocationView
{
    fn from(a: &InterruptibleCapacityReservationAllocation) -> Self {
        InterruptibleCapacityReservationAllocationView {
            allocation_id: a.allocation_id.clone(),
            capacity_reservation_id: a.capacity_reservation_id.clone(),
            instance_count: a.instance_count,
            start_date_time: a.start_date_time.clone(),
            end_date_time: a.end_date_time.clone(),
            state: a.state.clone(),
            allocation_type: a.allocation_type.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<InterruptibleCapacityReservationAllocationView>
    for InterruptibleCapacityReservationAllocation
{
    fn from(a: InterruptibleCapacityReservationAllocationView) -> Self {
        InterruptibleCapacityReservationAllocation {
            allocation_id: a.allocation_id,
            capacity_reservation_id: a.capacity_reservation_id,
            instance_count: a.instance_count,
            start_date_time: a.start_date_time,
            end_date_time: a.end_date_time,
            state: a.state,
            allocation_type: a.allocation_type,
            tags: a.tags,
        }
    }
}

impl From<&CapacityBlock> for CapacityBlockView {
    fn from(b: &CapacityBlock) -> Self {
        CapacityBlockView {
            capacity_block_id: b.capacity_block_id.clone(),
            capacity_reservation_id: b.capacity_reservation_id.clone(),
            capacity_block_offering_id: b.capacity_block_offering_id.clone(),
            instance_type: b.instance_type.clone(),
            instance_count: b.instance_count,
            availability_zone: b.availability_zone.clone(),
            start_date: b.start_date.clone(),
            end_date: b.end_date.clone(),
            tenancy: b.tenancy.clone(),
            currency_code: b.currency_code.clone(),
            upfront_fee: b.upfront_fee.clone(),
            commitment_duration_in_seconds: b.commitment_duration_in_seconds,
            capacity_reservation_arn: b.capacity_reservation_arn.clone(),
            tags: b.tags.clone(),
        }
    }
}

impl From<CapacityBlockView> for CapacityBlock {
    fn from(b: CapacityBlockView) -> Self {
        CapacityBlock {
            capacity_block_id: b.capacity_block_id,
            capacity_reservation_id: b.capacity_reservation_id,
            capacity_block_offering_id: b.capacity_block_offering_id,
            instance_type: b.instance_type,
            instance_count: b.instance_count,
            availability_zone: b.availability_zone,
            start_date: b.start_date,
            end_date: b.end_date,
            tenancy: b.tenancy,
            currency_code: b.currency_code,
            upfront_fee: b.upfront_fee,
            commitment_duration_in_seconds: b.commitment_duration_in_seconds,
            capacity_reservation_arn: b.capacity_reservation_arn,
            tags: b.tags,
        }
    }
}

impl From<&CapacityBlockExtension> for CapacityBlockExtensionView {
    fn from(e: &CapacityBlockExtension) -> Self {
        CapacityBlockExtensionView {
            capacity_block_extension_id: e.capacity_block_extension_id.clone(),
            capacity_reservation_id: e.capacity_reservation_id.clone(),
            instance_type: e.instance_type.clone(),
            availability_zone: e.availability_zone.clone(),
            instance_count: e.instance_count,
            availability_zone_id: e.availability_zone_id.clone(),
            start_date: e.start_date.clone(),
            end_date: e.end_date.clone(),
            capacity_block_extension_offering_id: e.capacity_block_extension_offering_id.clone(),
            capacity_block_extension_status: e.capacity_block_extension_status.clone(),
            capacity_block_extension_purchase_date: e
                .capacity_block_extension_purchase_date
                .clone(),
            capacity_block_extension_duration_hours: e.capacity_block_extension_duration_hours,
            currency_code: e.currency_code.clone(),
            upfront_fee: e.upfront_fee.clone(),
            capacity_reservation_arn: e.capacity_reservation_arn.clone(),
            capacity_block_extension_arn: e.capacity_block_extension_arn.clone(),
        }
    }
}

impl From<CapacityBlockExtensionView> for CapacityBlockExtension {
    fn from(e: CapacityBlockExtensionView) -> Self {
        CapacityBlockExtension {
            capacity_block_extension_id: e.capacity_block_extension_id,
            capacity_reservation_id: e.capacity_reservation_id,
            instance_type: e.instance_type,
            availability_zone: e.availability_zone,
            instance_count: e.instance_count,
            availability_zone_id: e.availability_zone_id,
            start_date: e.start_date,
            end_date: e.end_date,
            capacity_block_extension_offering_id: e.capacity_block_extension_offering_id,
            capacity_block_extension_status: e.capacity_block_extension_status,
            capacity_block_extension_purchase_date: e.capacity_block_extension_purchase_date,
            capacity_block_extension_duration_hours: e.capacity_block_extension_duration_hours,
            currency_code: e.currency_code,
            upfront_fee: e.upfront_fee,
            capacity_reservation_arn: e.capacity_reservation_arn,
            capacity_block_extension_arn: e.capacity_block_extension_arn,
        }
    }
}

impl From<&CapacityReservationFleet> for CapacityReservationFleetView {
    fn from(f: &CapacityReservationFleet) -> Self {
        CapacityReservationFleetView {
            capacity_reservation_fleet_id: f.capacity_reservation_fleet_id.clone(),
            capacity_reservation_fleet_arn: f.capacity_reservation_fleet_arn.clone(),
            state: f.state.clone(),
            tenancy: f.tenancy.clone(),
            allocation_strategy: f.allocation_strategy.clone(),
            instance_match_criteria: f.instance_match_criteria.clone(),
            total_target_capacity: f.total_target_capacity,
            total_fulfilled_capacity: f.total_fulfilled_capacity,
            create_time: f.create_time.clone(),
            end_date: f.end_date.clone(),
            instance_type_specifications: f
                .instance_type_specifications
                .iter()
                .map(CapacityReservationFleetInstanceSpecView::from)
                .collect(),
            tags: f.tags.clone(),
        }
    }
}

impl From<&CapacityReservationFleetInstanceSpec> for CapacityReservationFleetInstanceSpecView {
    fn from(s: &CapacityReservationFleetInstanceSpec) -> Self {
        CapacityReservationFleetInstanceSpecView {
            instance_type: s.instance_type.clone(),
            instance_platform: s.instance_platform.clone(),
            availability_zone: s.availability_zone.clone(),
            ebs_optimized: s.ebs_optimized,
            priority: s.priority,
            weight: s.weight,
        }
    }
}

impl From<&InstanceConnectEndpoint> for InstanceConnectEndpointView {
    fn from(i: &InstanceConnectEndpoint) -> Self {
        InstanceConnectEndpointView {
            instance_connect_endpoint_id: i.instance_connect_endpoint_id.clone(),
            instance_connect_endpoint_arn: i.instance_connect_endpoint_arn.clone(),
            subnet_id: i.subnet_id.clone(),
            vpc_id: i.vpc_id.clone(),
            availability_zone: i.availability_zone.clone(),
            state: i.state.clone(),
            created_at: i.created_at.clone(),
            preserve_client_ip: i.preserve_client_ip,
            security_group_ids: i.security_group_ids.clone(),
            network_interface_ids: i.network_interface_ids.clone(),
            dns_name: i.dns_name.clone(),
            fips_dns_name: i.fips_dns_name.clone(),
            ip_address_type: i.ip_address_type.clone(),
            owner_id: i.owner_id.clone(),
            tags: i.tags.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// From view types to internal types
// ---------------------------------------------------------------------------

impl From<Ec2StateView> for Ec2State {
    fn from(view: Ec2StateView) -> Self {
        Ec2State {
            vpcs: view
                .vpcs
                .into_iter()
                .map(|(k, v)| (k, Vpc::from(v)))
                .collect(),
            subnets: view
                .subnets
                .into_iter()
                .map(|(k, v)| (k, Subnet::from(v)))
                .collect(),
            igws: view
                .igws
                .into_iter()
                .map(|(k, v)| (k, InternetGateway::from(v)))
                .collect(),
            security_groups: view
                .security_groups
                .into_iter()
                .map(|(k, v)| (k, SecurityGroup::from(v)))
                .collect(),
            route_tables: view
                .route_tables
                .into_iter()
                .map(|(k, v)| (k, RouteTable::from(v)))
                .collect(),
            key_pairs: view
                .key_pairs
                .into_iter()
                .map(|(k, v)| (k, KeyPair::from(v)))
                .collect(),
            network_acls: view
                .network_acls
                .into_iter()
                .map(|(k, v)| (k, NetworkAcl::from(v)))
                .collect(),
            elastic_ips: view
                .elastic_ips
                .into_iter()
                .map(|(k, v)| (k, ElasticIp::from(v)))
                .collect(),
            nat_gateways: view
                .nat_gateways
                .into_iter()
                .map(|(k, v)| (k, NatGateway::from(v)))
                .collect(),
            dhcp_options: view
                .dhcp_options
                .into_iter()
                .map(|(k, v)| (k, DhcpOptions::from(v)))
                .collect(),
            egress_only_igws: view
                .egress_only_igws
                .into_iter()
                .map(|(k, v)| (k, EgressOnlyInternetGateway::from(v)))
                .collect(),
            flow_logs: view
                .flow_logs
                .into_iter()
                .map(|(k, v)| (k, FlowLog::from(v)))
                .collect(),
            vpc_peering_connections: view
                .vpc_peering_connections
                .into_iter()
                .map(|(k, v)| (k, VpcPeeringConnection::from(v)))
                .collect(),
            vpc_endpoints: view
                .vpc_endpoints
                .into_iter()
                .map(|(k, v)| (k, VpcEndpoint::from(v)))
                .collect(),
            managed_prefix_lists: view
                .managed_prefix_lists
                .into_iter()
                .map(|(k, v)| (k, ManagedPrefixList::from(v)))
                .collect(),
            customer_gateways: view
                .customer_gateways
                .into_iter()
                .map(|(k, v)| (k, CustomerGateway::from(v)))
                .collect(),
            vpn_gateways: view
                .vpn_gateways
                .into_iter()
                .map(|(k, v)| (k, VpnGateway::from(v)))
                .collect(),
            vpn_connections: view
                .vpn_connections
                .into_iter()
                .map(|(k, v)| (k, VpnConnection::from(v)))
                .collect(),
            carrier_gateways: view
                .carrier_gateways
                .into_iter()
                .map(|(k, v)| (k, CarrierGateway::from(v)))
                .collect(),
            network_interfaces: view
                .network_interfaces
                .into_iter()
                .map(|(k, v)| (k, NetworkInterface::from(v)))
                .collect(),
            vpc_cidr_associations: view.vpc_cidr_associations,
            ebs_encryption_by_default: view.ebs_encryption_by_default,
            transit_gateways: view
                .transit_gateways
                .into_iter()
                .map(|(k, v)| (k, TransitGateway::from(v)))
                .collect(),
            tgw_vpc_attachments: view
                .tgw_vpc_attachments
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayVpcAttachment::from(v)))
                .collect(),
            tgw_peering_attachments: view
                .tgw_peering_attachments
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayPeeringAttachment::from(v)))
                .collect(),
            tgw_route_tables: view
                .tgw_route_tables
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayRouteTable::from(v)))
                .collect(),
            tgw_routes: view
                .tgw_routes
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(TransitGatewayRoute::from).collect()))
                .collect(),
            instances: view
                .instances
                .into_iter()
                .map(|(k, v)| (k, Instance::from(v)))
                .collect(),
            volumes: view
                .volumes
                .into_iter()
                .map(|(k, v)| (k, Volume::from(v)))
                .collect(),
            snapshots: view
                .snapshots
                .into_iter()
                .map(|(k, v)| (k, Snapshot::from(v)))
                .collect(),
            images: view
                .images
                .into_iter()
                .map(|(k, v)| (k, Image::from(v)))
                .collect(),
            launch_templates: view
                .launch_templates
                .into_iter()
                .map(|(k, v)| (k, LaunchTemplate::from(v)))
                .collect(),
            launch_template_versions: view
                .launch_template_versions
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(LaunchTemplateVersion::from).collect()))
                .collect(),
            spot_requests: view
                .spot_requests
                .into_iter()
                .map(|(k, v)| (k, SpotInstanceRequest::from(v)))
                .collect(),
            iam_instance_profile_associations: view
                .iam_instance_profile_associations
                .into_iter()
                .map(|(k, v)| (k, IamInstanceProfileAssociation::from(v)))
                .collect(),
            dedicated_hosts: view
                .dedicated_hosts
                .into_iter()
                .map(|(k, v)| (k, DedicatedHost::from(v)))
                .collect(),
            ec2_fleets: view
                .ec2_fleets
                .into_iter()
                .map(|(k, v)| (k, Ec2Fleet::from(v)))
                .collect(),
            vpc_endpoint_service_configs: view
                .vpc_endpoint_service_configs
                .into_iter()
                .map(|(k, v)| (k, VpcEndpointServiceConfiguration::from(v)))
                .collect(),
            spot_fleet_requests: view
                .spot_fleet_requests
                .into_iter()
                .map(|(k, v)| (k, SpotFleetRequest::from(v)))
                .collect(),
            subnet_cidr_reservations: view
                .subnet_cidr_reservations
                .into_iter()
                .map(|(k, v)| (k, SubnetCidrReservationEntry::from(v)))
                .collect(),
            placement_groups: view
                .placement_groups
                .into_iter()
                .map(|(k, v)| (k, PlacementGroup::from(v)))
                .collect(),
            network_interface_permissions: view
                .network_interface_permissions
                .into_iter()
                .map(|(k, v)| (k, NetworkInterfacePermission::from(v)))
                .collect(),
            instance_connect_endpoints: view
                .instance_connect_endpoints
                .into_iter()
                .map(|(k, v)| (k, InstanceConnectEndpoint::from(v)))
                .collect(),
            capacity_reservations: view
                .capacity_reservations
                .into_iter()
                .map(|(k, v)| (k, CapacityReservation::from(v)))
                .collect(),
            capacity_reservation_fleets: view
                .capacity_reservation_fleets
                .into_iter()
                .map(|(k, v)| (k, CapacityReservationFleet::from(v)))
                .collect(),
            coip_pools: view
                .coip_pools
                .into_iter()
                .map(|(k, v)| (k, CoipPool::from(v)))
                .collect(),
            byoip_cidrs: view
                .byoip_cidrs
                .into_iter()
                .map(|(k, v)| (k, ByoipCidr::from(v)))
                .collect(),
            public_ipv4_pools: view
                .public_ipv4_pools
                .into_iter()
                .map(|(k, v)| (k, PublicIpv4Pool::from(v)))
                .collect(),
            coip_cidrs: view
                .coip_cidrs
                .into_iter()
                .map(|v| ((v.cidr.clone(), v.coip_pool_id.clone()), CoipCidr::from(v)))
                .collect(),
            address_transfers: view
                .address_transfers
                .into_iter()
                .map(|(k, v)| (k, AddressTransfer::from(v)))
                .collect(),
            security_group_vpc_associations: view
                .security_group_vpc_associations
                .into_iter()
                .map(|v| {
                    (
                        (v.group_id.clone(), v.vpc_id.clone()),
                        SecurityGroupVpcAssociation::from(v),
                    )
                })
                .collect(),
            enclave_certificate_iam_role_associations: view
                .enclave_certificate_iam_role_associations
                .into_iter()
                .map(|v| {
                    (
                        (v.certificate_arn.clone(), v.role_arn.clone()),
                        EnclaveCertificateIamRoleAssociation::from(v),
                    )
                })
                .collect(),
            mac_sip_modification_tasks: view
                .mac_sip_modification_tasks
                .into_iter()
                .map(|(k, v)| (k, MacSipModificationTask::from(v)))
                .collect(),
            declarative_policies_reports: view
                .declarative_policies_reports
                .into_iter()
                .map(|(k, v)| (k, DeclarativePoliciesReport::from(v)))
                .collect(),
            vpn_concentrators: view
                .vpn_concentrators
                .into_iter()
                .map(|(k, v)| (k, VpnConcentrator::from(v)))
                .collect(),
            vpc_endpoint_connections: view
                .vpc_endpoint_connections
                .into_iter()
                .map(|v| {
                    let key = (v.service_id.clone(), v.vpc_endpoint_id.clone());
                    (key, VpcEndpointConnection::from(v))
                })
                .collect(),
            vpc_endpoint_connection_notifications: view
                .vpc_endpoint_connection_notifications
                .into_iter()
                .map(|(k, v)| (k, VpcEndpointConnectionNotification::from(v)))
                .collect(),
            vpc_block_public_access_exclusions: view
                .vpc_block_public_access_exclusions
                .into_iter()
                .map(|(k, v)| (k, VpcBlockPublicAccessExclusion::from(v)))
                .collect(),
            vpc_block_public_access_options: view
                .vpc_block_public_access_options
                .map(VpcBlockPublicAccessOptions::from),
            vpc_encryption_controls: view
                .vpc_encryption_controls
                .into_iter()
                .map(|(k, v)| (k, VpcEncryptionControl::from(v)))
                .collect(),
            mac_volume_ownership_tasks: view
                .mac_volume_ownership_tasks
                .into_iter()
                .map(|(k, v)| (k, MacVolumeOwnershipTask::from(v)))
                .collect(),
            replace_root_volume_tasks: view
                .replace_root_volume_tasks
                .into_iter()
                .map(|(k, v)| (k, ReplaceRootVolumeTask::from(v)))
                .collect(),
            snapshot_import_tasks: view
                .snapshot_import_tasks
                .into_iter()
                .map(|(k, v)| (k, SnapshotImportTask::from(v)))
                .collect(),
            conversion_tasks: view
                .conversion_tasks
                .into_iter()
                .map(|(k, v)| (k, ConversionTask::from(v)))
                .collect(),
            export_tasks: view
                .export_tasks
                .into_iter()
                .map(|(k, v)| (k, ExportTask::from(v)))
                .collect(),
            import_tasks: view.import_tasks,
            trunk_interface_associations: view
                .trunk_interface_associations
                .into_iter()
                .map(|(k, v)| (k, TrunkInterfaceAssociation::from(v)))
                .collect(),
            secondary_networks: view
                .secondary_networks
                .into_iter()
                .map(|(k, v)| (k, SecondaryNetwork::from(v)))
                .collect(),
            secondary_subnets: view
                .secondary_subnets
                .into_iter()
                .map(|(k, v)| (k, SecondarySubnet::from(v)))
                .collect(),
            deleted_volumes_recycle_bin: view
                .deleted_volumes_recycle_bin
                .into_iter()
                .map(|(k, v)| (k, Volume::from(v)))
                .collect(),
            deleted_snapshots_recycle_bin: view
                .deleted_snapshots_recycle_bin
                .into_iter()
                .map(|(k, v)| (k, Snapshot::from(v)))
                .collect(),
            reserved_instances_exchanges: view
                .reserved_instances_exchanges
                .into_iter()
                .map(|(k, v)| (k, crate::types::ReservedInstancesExchange::from(v)))
                .collect(),
            reserved_instances_listings: view
                .reserved_instances_listings
                .into_iter()
                .map(|(k, v)| (k, crate::types::ReservedInstancesListing::from(v)))
                .collect(),
            queued_reserved_instances_purchases: view
                .queued_reserved_instances_purchases
                .into_iter()
                .map(|(k, v)| (k, crate::types::ReservedInstancesPurchase::from(v)))
                .collect(),
            reserved_instances_modifications: view
                .reserved_instances_modifications
                .into_iter()
                .map(|(k, v)| (k, crate::types::ReservedInstancesModification::from(v)))
                .collect(),
            reserved_instances_purchases: view
                .reserved_instances_purchases
                .into_iter()
                .map(|(k, v)| (k, crate::types::ReservedInstancesPurchase::from(v)))
                .collect(),
            reserved_instances: view
                .reserved_instances
                .into_iter()
                .map(|(k, v)| (k, crate::types::ReservedInstances::from(v)))
                .collect(),
            fpga_images: view
                .fpga_images
                .into_iter()
                .map(|(k, v)| (k, crate::types::FpgaImage::from(v)))
                .collect(),
            image_usage_reports: view
                .image_usage_reports
                .into_iter()
                .map(|(k, v)| (k, crate::types::ImageUsageReport::from(v)))
                .collect(),
            restore_image_tasks: view
                .restore_image_tasks
                .into_iter()
                .map(|(k, v)| (k, crate::types::RestoreImageTask::from(v)))
                .collect(),
            store_image_tasks: view
                .store_image_tasks
                .into_iter()
                .map(|(k, v)| (k, crate::types::StoreImageTask::from(v)))
                .collect(),
            import_image_tasks: view
                .import_image_tasks
                .into_iter()
                .map(|(k, v)| (k, crate::types::ImportImageTask::from(v)))
                .collect(),
            allowed_image_criteria: view
                .allowed_image_criteria
                .into_iter()
                .map(crate::types::AllowedImageCriterion::from)
                .collect(),
            default_credit_specifications: view.default_credit_specifications,
            instance_metadata_defaults: view
                .instance_metadata_defaults
                .map(crate::types::InstanceMetadataDefaults::from),
            instance_event_windows: view
                .instance_event_windows
                .into_iter()
                .map(|(k, v)| (k, crate::types::InstanceEventWindow::from(v)))
                .collect(),
            instance_event_notification_attributes: view
                .instance_event_notification_attributes
                .map(crate::types::InstanceTagNotificationAttributes::from),
            instance_events: view
                .instance_events
                .into_iter()
                .map(|(k, v)| (k, crate::types::InstanceEvent::from(v)))
                .collect(),
            host_reservations: view
                .host_reservations
                .into_iter()
                .map(|(k, v)| (k, crate::types::HostReservation::from(v)))
                .collect(),
            scheduled_instances: view
                .scheduled_instances
                .into_iter()
                .map(|(k, v)| (k, crate::types::ScheduledInstance::from(v)))
                .collect(),
            az_group_opt_in: view.az_group_opt_in,
            instance_status_reports: view
                .instance_status_reports
                .into_iter()
                .map(crate::types::InstanceStatusReport::from)
                .collect(),
            network_insights_access_scopes: view
                .network_insights_access_scopes
                .into_iter()
                .map(|(k, v)| (k, NetworkInsightsAccessScope::from(v)))
                .collect(),
            network_insights_access_scope_analyses: view
                .network_insights_access_scope_analyses
                .into_iter()
                .map(|(k, v)| (k, NetworkInsightsAccessScopeAnalysis::from(v)))
                .collect(),
            network_insights_paths: view
                .network_insights_paths
                .into_iter()
                .map(|(k, v)| (k, NetworkInsightsPath::from(v)))
                .collect(),
            network_insights_analyses: view
                .network_insights_analyses
                .into_iter()
                .map(|(k, v)| (k, NetworkInsightsAnalysis::from(v)))
                .collect(),
            traffic_mirror_filters: view
                .traffic_mirror_filters
                .into_iter()
                .map(|(k, v)| (k, TrafficMirrorFilter::from(v)))
                .collect(),
            traffic_mirror_sessions: view
                .traffic_mirror_sessions
                .into_iter()
                .map(|(k, v)| (k, TrafficMirrorSession::from(v)))
                .collect(),
            traffic_mirror_targets: view
                .traffic_mirror_targets
                .into_iter()
                .map(|(k, v)| (k, TrafficMirrorTarget::from(v)))
                .collect(),
            client_vpn_endpoints: view
                .client_vpn_endpoints
                .into_iter()
                .map(|(k, v)| (k, ClientVpnEndpoint::from(v)))
                .collect(),
            client_vpn_target_network_associations: view
                .client_vpn_target_network_associations
                .into_iter()
                .map(|(k, v)| (k, ClientVpnTargetNetworkAssociation::from(v)))
                .collect(),
            client_vpn_authorization_rules: view
                .client_vpn_authorization_rules
                .into_iter()
                .map(|v| {
                    let key = (
                        v.client_vpn_endpoint_id.clone(),
                        v.destination_cidr.clone(),
                        v.group_id.clone().unwrap_or_default(),
                    );
                    (key, ClientVpnAuthorizationRule::from(v))
                })
                .collect(),
            client_vpn_routes: view
                .client_vpn_routes
                .into_iter()
                .map(|v| {
                    let key = (
                        v.client_vpn_endpoint_id.clone(),
                        v.destination_cidr.clone(),
                        v.target_subnet.clone(),
                    );
                    (key, ClientVpnRoute::from(v))
                })
                .collect(),
            client_vpn_connections: view
                .client_vpn_connections
                .into_iter()
                .map(|(k, v)| (k, ClientVpnConnection::from(v)))
                .collect(),
            local_gateways: view
                .local_gateways
                .into_iter()
                .map(|(k, v)| (k, LocalGateway::from(v)))
                .collect(),
            local_gateway_route_tables: view
                .local_gateway_route_tables
                .into_iter()
                .map(|(k, v)| (k, LocalGatewayRouteTable::from(v)))
                .collect(),
            local_gateway_routes: view
                .local_gateway_routes
                .into_iter()
                .map(|v| {
                    let key = (
                        v.local_gateway_route_table_id.clone(),
                        v.destination_cidr_block.clone(),
                    );
                    (key, LocalGatewayRoute::from(v))
                })
                .collect(),
            local_gateway_route_table_virtual_interface_group_associations: view
                .local_gateway_route_table_virtual_interface_group_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        LocalGatewayRouteTableVirtualInterfaceGroupAssociation::from(v),
                    )
                })
                .collect(),
            local_gateway_route_table_vpc_associations: view
                .local_gateway_route_table_vpc_associations
                .into_iter()
                .map(|(k, v)| (k, LocalGatewayRouteTableVpcAssociation::from(v)))
                .collect(),
            local_gateway_virtual_interfaces: view
                .local_gateway_virtual_interfaces
                .into_iter()
                .map(|(k, v)| (k, LocalGatewayVirtualInterface::from(v)))
                .collect(),
            local_gateway_virtual_interface_groups: view
                .local_gateway_virtual_interface_groups
                .into_iter()
                .map(|(k, v)| (k, LocalGatewayVirtualInterfaceGroup::from(v)))
                .collect(),
            route_servers: view
                .route_servers
                .into_iter()
                .map(|(k, v)| (k, RouteServer::from(v)))
                .collect(),
            route_server_endpoints: view
                .route_server_endpoints
                .into_iter()
                .map(|(k, v)| (k, RouteServerEndpoint::from(v)))
                .collect(),
            route_server_peers: view
                .route_server_peers
                .into_iter()
                .map(|(k, v)| (k, RouteServerPeer::from(v)))
                .collect(),
            route_server_associations: view
                .route_server_associations
                .into_iter()
                .map(|v| {
                    let key = (v.route_server_id.clone(), v.vpc_id.clone());
                    (key, RouteServerAssociation::from(v))
                })
                .collect(),
            verified_access_instances: view
                .verified_access_instances
                .into_iter()
                .map(|(k, v)| (k, VerifiedAccessInstance::from(v)))
                .collect(),
            verified_access_trust_providers: view
                .verified_access_trust_providers
                .into_iter()
                .map(|(k, v)| (k, VerifiedAccessTrustProvider::from(v)))
                .collect(),
            verified_access_groups: view
                .verified_access_groups
                .into_iter()
                .map(|(k, v)| (k, VerifiedAccessGroup::from(v)))
                .collect(),
            verified_access_endpoints: view
                .verified_access_endpoints
                .into_iter()
                .map(|(k, v)| (k, VerifiedAccessEndpoint::from(v)))
                .collect(),
            verified_access_trust_provider_attachments: view
                .verified_access_trust_provider_attachments
                .into_iter()
                .map(|v| {
                    let key = (v.instance_id.clone(), v.trust_provider_id.clone());
                    (key, VerifiedAccessTrustProviderAttachment::from(v))
                })
                .collect(),
            verified_access_instance_logging_configurations: view
                .verified_access_instance_logging_configurations
                .into_iter()
                .map(|(k, v)| (k, VerifiedAccessLogs::from(v)))
                .collect(),
            billing_ownership_offers: view
                .billing_ownership_offers
                .into_iter()
                .map(|o| {
                    let key = (
                        o.capacity_reservation_id.clone(),
                        o.unused_reservation_billing_owner_id.clone(),
                    );
                    (key, BillingOwnershipOffer::from(o))
                })
                .collect(),
            capacity_manager_data_exports: view
                .capacity_manager_data_exports
                .into_iter()
                .map(|(k, v)| (k, CapacityManagerDataExport::from(v)))
                .collect(),
            interruptible_capacity_reservation_allocations: view
                .interruptible_capacity_reservation_allocations
                .into_iter()
                .map(|(k, v)| (k, InterruptibleCapacityReservationAllocation::from(v)))
                .collect(),
            capacity_blocks: view
                .capacity_blocks
                .into_iter()
                .map(|(k, v)| (k, CapacityBlock::from(v)))
                .collect(),
            capacity_block_extensions: view
                .capacity_block_extensions
                .into_iter()
                .map(|(k, v)| (k, CapacityBlockExtension::from(v)))
                .collect(),
            capacity_manager_organizations_access: view
                .capacity_manager_organizations_access
                .map(CapacityManagerOrganizationsAccess::from),
            tgw_multicast_domains: view
                .tgw_multicast_domains
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayMulticastDomain::from(v)))
                .collect(),
            tgw_multicast_domain_associations: view
                .tgw_multicast_domain_associations
                .into_iter()
                .map(|v| {
                    let key = (
                        v.transit_gateway_multicast_domain_id.clone(),
                        v.transit_gateway_attachment_id.clone(),
                    );
                    (key, TransitGatewayMulticastDomainAssociation::from(v))
                })
                .collect(),
            tgw_multicast_group_members: view
                .tgw_multicast_group_members
                .into_iter()
                .map(|v| {
                    let key = (
                        v.transit_gateway_multicast_domain_id.clone(),
                        v.group_ip_address.clone(),
                        v.network_interface_id.clone(),
                    );
                    (key, TransitGatewayMulticastGroupMember::from(v))
                })
                .collect(),
            tgw_multicast_group_sources: view
                .tgw_multicast_group_sources
                .into_iter()
                .map(|v| {
                    let key = (
                        v.transit_gateway_multicast_domain_id.clone(),
                        v.group_ip_address.clone(),
                        v.network_interface_id.clone(),
                    );
                    (key, TransitGatewayMulticastGroupSource::from(v))
                })
                .collect(),
            tgw_connects: view
                .tgw_connects
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayConnect::from(v)))
                .collect(),
            tgw_connect_peers: view
                .tgw_connect_peers
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayConnectPeer::from(v)))
                .collect(),
            tgw_metering_policies: view
                .tgw_metering_policies
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayMeteringPolicy::from(v)))
                .collect(),
            tgw_metering_policy_entries: view
                .tgw_metering_policy_entries
                .into_iter()
                .map(|v| {
                    let key = (
                        v.transit_gateway_metering_policy_id.clone(),
                        v.transit_gateway_metering_policy_entry_id.clone(),
                    );
                    (key, TransitGatewayMeteringPolicyEntry::from(v))
                })
                .collect(),
            tgw_policy_tables: view
                .tgw_policy_tables
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayPolicyTable::from(v)))
                .collect(),
            tgw_policy_table_associations: view
                .tgw_policy_table_associations
                .into_iter()
                .map(|v| {
                    let key = (
                        v.transit_gateway_policy_table_id.clone(),
                        v.transit_gateway_attachment_id.clone(),
                    );
                    (key, TransitGatewayPolicyTableAssociation::from(v))
                })
                .collect(),
            tgw_prefix_list_references: view
                .tgw_prefix_list_references
                .into_iter()
                .map(|v| {
                    let key = (
                        v.transit_gateway_route_table_id.clone(),
                        v.prefix_list_id.clone(),
                    );
                    (key, TransitGatewayPrefixListReference::from(v))
                })
                .collect(),
            tgw_route_table_announcements: view
                .tgw_route_table_announcements
                .into_iter()
                .map(|(k, v)| (k, TransitGatewayRouteTableAnnouncement::from(v)))
                .collect(),
            ipams: view
                .ipams
                .into_iter()
                .map(|(k, v)| (k, Ipam::from(v)))
                .collect(),
            ipam_scopes: view
                .ipam_scopes
                .into_iter()
                .map(|(k, v)| (k, IpamScope::from(v)))
                .collect(),
            ipam_pools: view
                .ipam_pools
                .into_iter()
                .map(|(k, v)| (k, IpamPool::from(v)))
                .collect(),
            ipam_pool_cidrs: view
                .ipam_pool_cidrs
                .into_iter()
                .map(|v| {
                    let key = (v.ipam_pool_id.clone(), v.cidr.clone());
                    (key, IpamPoolCidr::from(v))
                })
                .collect(),
            ipam_pool_allocations: view
                .ipam_pool_allocations
                .into_iter()
                .map(|v| {
                    let key = (v.ipam_pool_id.clone(), v.ipam_pool_allocation_id.clone());
                    (key, IpamPoolAllocation::from(v))
                })
                .collect(),
            ipam_resource_discoveries: view
                .ipam_resource_discoveries
                .into_iter()
                .map(|(k, v)| (k, IpamResourceDiscovery::from(v)))
                .collect(),
            ipam_resource_discovery_associations: view
                .ipam_resource_discovery_associations
                .into_iter()
                .map(|(k, v)| (k, IpamResourceDiscoveryAssociation::from(v)))
                .collect(),
            ipam_byoasns: view
                .ipam_byoasns
                .into_iter()
                .map(|v| {
                    let key = (v.ipam_id.clone(), v.asn.clone());
                    (key, IpamByoasn::from(v))
                })
                .collect(),
            ipam_external_resource_verification_tokens: view
                .ipam_external_resource_verification_tokens
                .into_iter()
                .map(|(k, v)| (k, IpamExternalResourceVerificationToken::from(v)))
                .collect(),
            ipam_policies: view
                .ipam_policies
                .into_iter()
                .map(|(k, v)| (k, IpamPolicy::from(v)))
                .collect(),
            ipam_prefix_list_resolvers: view
                .ipam_prefix_list_resolvers
                .into_iter()
                .map(|(k, v)| (k, IpamPrefixListResolver::from(v)))
                .collect(),
            ipam_prefix_list_resolver_targets: view
                .ipam_prefix_list_resolver_targets
                .into_iter()
                .map(|v| {
                    let key = (
                        v.ipam_prefix_list_resolver_id.clone(),
                        v.ipam_prefix_list_resolver_target_id.clone(),
                    );
                    (key, IpamPrefixListResolverTarget::from(v))
                })
                .collect(),
            volume_modifications: view
                .volume_modifications
                .into_iter()
                .map(|(k, v)| (k, crate::types::VolumeModification::from(v)))
                .collect(),
            import_volume_tasks: view
                .import_volume_tasks
                .into_iter()
                .map(|(k, v)| (k, crate::types::ImportVolumeTask::from(v)))
                .collect(),
            bundle_tasks: view
                .bundle_tasks
                .into_iter()
                .map(|(k, v)| (k, crate::types::BundleTask::from(v)))
                .collect(),
            id_format: view
                .id_format
                .into_iter()
                .map(|(k, v)| (k, crate::types::IdFormatEntry::from(v)))
                .collect(),
            outpost_lags: view
                .outpost_lags
                .into_iter()
                .map(|(k, v)| (k, crate::types::OutpostLag::from(v)))
                .collect(),
            export_image_tasks: view
                .export_image_tasks
                .into_iter()
                .map(|(k, v)| (k, crate::types::ExportImageTask::from(v)))
                .collect(),
            ebs_default_kms_key_id: view.ebs_default_kms_key_id,
            serial_console_access_enabled: view.serial_console_access_enabled,
            allowed_images_settings_state: view.allowed_images_settings_state,
            image_block_public_access_state: view.image_block_public_access_state,
            aws_network_performance_subscriptions: view
                .aws_network_performance_subscriptions
                .into_iter()
                .map(|s| {
                    let key = (
                        s.source.clone(),
                        s.destination.clone(),
                        s.metric.clone(),
                        s.statistic.clone(),
                    );
                    (
                        key,
                        crate::types::AwsNetworkPerformanceSubscription {
                            source: s.source,
                            destination: s.destination,
                            metric: s.metric,
                            statistic: s.statistic,
                            period: s.period,
                        },
                    )
                })
                .collect(),
            counters: crate::state::Ec2Counters::from(view.counters),
            deregistered_images: Default::default(),
        }
    }
}

impl From<VpcView> for Vpc {
    fn from(v: VpcView) -> Self {
        Vpc {
            vpc_id: v.vpc_id,
            cidr_block: v.cidr_block,
            state: v.state,
            dhcp_options_id: v.dhcp_options_id,
            instance_tenancy: v.instance_tenancy,
            is_default: v.is_default,
            enable_dns_hostnames: v.enable_dns_hostnames,
            enable_dns_support: v.enable_dns_support,
            secondary_cidr_blocks: v.secondary_cidr_blocks,
            tags: v.tags,
            classic_link_enabled: v.classic_link_enabled,
        }
    }
}

impl From<SubnetView> for Subnet {
    fn from(s: SubnetView) -> Self {
        Subnet {
            subnet_id: s.subnet_id,
            vpc_id: s.vpc_id,
            cidr_block: s.cidr_block,
            availability_zone: s.availability_zone,
            state: s.state,
            available_ip_address_count: s.available_ip_address_count,
            map_public_ip_on_launch: s.map_public_ip_on_launch,
            ipv6_cidr_blocks: s
                .ipv6_cidr_blocks
                .into_iter()
                .map(SubnetIpv6CidrAssoc::from)
                .collect(),
            tags: s.tags,
        }
    }
}

impl From<SubnetIpv6CidrAssocView> for SubnetIpv6CidrAssoc {
    fn from(a: SubnetIpv6CidrAssocView) -> Self {
        SubnetIpv6CidrAssoc {
            association_id: a.association_id,
            ipv6_cidr_block: a.ipv6_cidr_block,
            state: a.state,
        }
    }
}

impl From<InternetGatewayView> for InternetGateway {
    fn from(igw: InternetGatewayView) -> Self {
        InternetGateway {
            igw_id: igw.igw_id,
            attachments: igw
                .attachments
                .into_iter()
                .map(IgwAttachment::from)
                .collect(),
            tags: igw.tags,
        }
    }
}

impl From<IgwAttachmentView> for IgwAttachment {
    fn from(a: IgwAttachmentView) -> Self {
        IgwAttachment {
            vpc_id: a.vpc_id,
            state: a.state,
        }
    }
}

impl From<SecurityGroupView> for SecurityGroup {
    fn from(sg: SecurityGroupView) -> Self {
        SecurityGroup {
            group_id: sg.group_id,
            group_name: sg.group_name,
            description: sg.description,
            vpc_id: sg.vpc_id,
            owner_id: sg.owner_id,
            ingress_rules: sg
                .ingress_rules
                .into_iter()
                .map(IpPermission::from)
                .collect(),
            egress_rules: sg
                .egress_rules
                .into_iter()
                .map(IpPermission::from)
                .collect(),
            tags: sg.tags,
        }
    }
}

impl From<IpPermissionView> for IpPermission {
    fn from(p: IpPermissionView) -> Self {
        IpPermission {
            rule_id: String::new(),
            from_port: p.from_port,
            to_port: p.to_port,
            ip_protocol: p.ip_protocol,
            ip_ranges: p.ip_ranges.into_iter().map(IpRange::from).collect(),
            ipv6_ranges: p.ipv6_ranges.into_iter().map(Ipv6Range::from).collect(),
            user_id_group_pairs: p
                .user_id_group_pairs
                .into_iter()
                .map(UserIdGroupPair::from)
                .collect(),
        }
    }
}

impl From<IpRangeView> for IpRange {
    fn from(r: IpRangeView) -> Self {
        IpRange {
            cidr_ip: r.cidr_ip,
            description: r.description,
        }
    }
}

impl From<Ipv6RangeView> for Ipv6Range {
    fn from(r: Ipv6RangeView) -> Self {
        Ipv6Range {
            cidr_ipv6: r.cidr_ipv6,
            description: r.description,
        }
    }
}

impl From<UserIdGroupPairView> for UserIdGroupPair {
    fn from(p: UserIdGroupPairView) -> Self {
        UserIdGroupPair {
            group_id: p.group_id,
            user_id: p.user_id,
        }
    }
}

impl From<RouteTableView> for RouteTable {
    fn from(rtb: RouteTableView) -> Self {
        RouteTable {
            route_table_id: rtb.route_table_id,
            vpc_id: rtb.vpc_id,
            routes: rtb.routes.into_iter().map(Route::from).collect(),
            associations: rtb
                .associations
                .into_iter()
                .map(RouteTableAssociation::from)
                .collect(),
            propagating_vgws: rtb.propagating_vgws,
            tags: rtb.tags,
        }
    }
}

impl From<RouteView> for Route {
    fn from(r: RouteView) -> Self {
        Route {
            destination_cidr_block: r.destination_cidr_block,
            destination_ipv6_cidr_block: r.destination_ipv6_cidr_block,
            gateway_id: r.gateway_id,
            state: r.state,
            origin: r.origin,
        }
    }
}

impl From<RouteTableAssociationView> for RouteTableAssociation {
    fn from(a: RouteTableAssociationView) -> Self {
        RouteTableAssociation {
            association_id: a.association_id,
            subnet_id: a.subnet_id,
            gateway_id: a.gateway_id,
            main: a.main,
            state: a.state,
        }
    }
}

impl From<KeyPairView> for KeyPair {
    fn from(kp: KeyPairView) -> Self {
        KeyPair {
            key_pair_id: kp.key_pair_id,
            key_name: kp.key_name,
            fingerprint: kp.fingerprint,
            tags: kp.tags,
        }
    }
}

impl From<NetworkAclView> for NetworkAcl {
    fn from(n: NetworkAclView) -> Self {
        NetworkAcl {
            network_acl_id: n.network_acl_id,
            vpc_id: n.vpc_id,
            is_default: n.is_default,
            entries: n.entries.into_iter().map(NetworkAclEntry::from).collect(),
            associations: n
                .associations
                .into_iter()
                .map(NetworkAclAssociation::from)
                .collect(),
            tags: n.tags,
        }
    }
}

impl From<NetworkAclEntryView> for NetworkAclEntry {
    fn from(e: NetworkAclEntryView) -> Self {
        NetworkAclEntry {
            rule_number: e.rule_number,
            protocol: e.protocol,
            rule_action: e.rule_action,
            egress: e.egress,
            cidr_block: e.cidr_block,
            ipv6_cidr_block: e.ipv6_cidr_block,
            port_range: e.port_range.map(PortRange::from),
            icmp_type_code: e.icmp_type_code.map(IcmpTypeCode::from),
        }
    }
}

impl From<PortRangeView> for PortRange {
    fn from(p: PortRangeView) -> Self {
        PortRange {
            from: p.from,
            to: p.to,
        }
    }
}

impl From<IcmpTypeCodeView> for IcmpTypeCode {
    fn from(i: IcmpTypeCodeView) -> Self {
        IcmpTypeCode {
            type_num: i.type_num,
            code: i.code,
        }
    }
}

impl From<NetworkAclAssociationView> for NetworkAclAssociation {
    fn from(a: NetworkAclAssociationView) -> Self {
        NetworkAclAssociation {
            network_acl_association_id: a.network_acl_association_id,
            network_acl_id: a.network_acl_id,
            subnet_id: a.subnet_id,
        }
    }
}

impl From<ElasticIpView> for ElasticIp {
    fn from(e: ElasticIpView) -> Self {
        ElasticIp {
            allocation_id: e.allocation_id,
            public_ip: e.public_ip,
            association_id: e.association_id,
            instance_id: e.instance_id,
            network_interface_id: e.network_interface_id,
            private_ip_address: e.private_ip_address,
            address_attribute_ptr_record: e.address_attribute_ptr_record,
            domain: e.domain,
            pending_transfer: e.pending_transfer,
            tags: e.tags,
        }
    }
}

impl From<NatGatewayView> for NatGateway {
    fn from(n: NatGatewayView) -> Self {
        NatGateway {
            nat_gateway_id: n.nat_gateway_id,
            vpc_id: n.vpc_id,
            subnet_id: n.subnet_id,
            state: n.state,
            connectivity_type: n.connectivity_type,
            allocation_id: n.allocation_id,
            public_ip: n.public_ip,
            secondary_addresses: n
                .secondary_addresses
                .into_iter()
                .map(NatGatewayAddressAssociation::from)
                .collect(),
            tags: n.tags,
        }
    }
}

impl From<DhcpOptionsView> for DhcpOptions {
    fn from(d: DhcpOptionsView) -> Self {
        DhcpOptions {
            dhcp_options_id: d.dhcp_options_id,
            configurations: d
                .configurations
                .into_iter()
                .map(DhcpConfiguration::from)
                .collect(),
            tags: d.tags,
        }
    }
}

impl From<DhcpConfigurationView> for DhcpConfiguration {
    fn from(c: DhcpConfigurationView) -> Self {
        DhcpConfiguration {
            key: c.key,
            values: c.values,
        }
    }
}

impl From<EgressOnlyIgwView> for EgressOnlyInternetGateway {
    fn from(e: EgressOnlyIgwView) -> Self {
        EgressOnlyInternetGateway {
            eigw_id: e.eigw_id,
            state: e.state,
            attachments: e
                .attachments
                .into_iter()
                .map(EoigwAttachment::from)
                .collect(),
            tags: e.tags,
        }
    }
}

impl From<EoigwAttachmentView> for EoigwAttachment {
    fn from(a: EoigwAttachmentView) -> Self {
        EoigwAttachment {
            vpc_id: a.vpc_id,
            state: a.state,
        }
    }
}

impl From<FlowLogView> for FlowLog {
    fn from(f: FlowLogView) -> Self {
        FlowLog {
            flow_log_id: f.flow_log_id,
            resource_id: f.resource_id,
            traffic_type: f.traffic_type,
            log_destination_type: f.log_destination_type,
            log_destination: f.log_destination,
            log_group_name: f.log_group_name,
            deliver_logs_status: f.deliver_logs_status,
            flow_log_status: f.flow_log_status,
            tags: f.tags,
        }
    }
}

impl From<VpcPeeringConnectionView> for VpcPeeringConnection {
    fn from(p: VpcPeeringConnectionView) -> Self {
        use crate::types::VpcPeeringConnectionOptions;
        VpcPeeringConnection {
            peering_id: p.peering_id,
            requester_vpc_id: p.requester_vpc_id,
            accepter_vpc_id: p.accepter_vpc_id,
            status: p.status,
            tags: p.tags,
            requester_peering_options: p.requester_peering_options.map(|o| {
                VpcPeeringConnectionOptions {
                    allow_dns_resolution_from_remote_vpc: o.allow_dns_resolution_from_remote_vpc,
                    allow_egress_from_local_classic_link_to_remote_vpc: o
                        .allow_egress_from_local_classic_link_to_remote_vpc,
                    allow_egress_from_local_vpc_to_remote_classic_link: o
                        .allow_egress_from_local_vpc_to_remote_classic_link,
                }
            }),
            accepter_peering_options: p.accepter_peering_options.map(|o| {
                VpcPeeringConnectionOptions {
                    allow_dns_resolution_from_remote_vpc: o.allow_dns_resolution_from_remote_vpc,
                    allow_egress_from_local_classic_link_to_remote_vpc: o
                        .allow_egress_from_local_classic_link_to_remote_vpc,
                    allow_egress_from_local_vpc_to_remote_classic_link: o
                        .allow_egress_from_local_vpc_to_remote_classic_link,
                }
            }),
        }
    }
}

impl From<VpcEndpointView> for VpcEndpoint {
    fn from(e: VpcEndpointView) -> Self {
        VpcEndpoint {
            endpoint_id: e.endpoint_id,
            vpc_id: e.vpc_id,
            service_name: e.service_name,
            endpoint_type: e.endpoint_type,
            state: e.state,
            policy_document: e.policy_document,
            route_table_ids: e.route_table_ids,
            subnet_ids: e.subnet_ids,
            security_group_ids: e.security_group_ids,
            private_dns_enabled: e.private_dns_enabled,
            tags: e.tags,
        }
    }
}

impl From<ManagedPrefixListView> for ManagedPrefixList {
    fn from(m: ManagedPrefixListView) -> Self {
        ManagedPrefixList {
            prefix_list_id: m.prefix_list_id,
            prefix_list_name: m.prefix_list_name,
            state: m.state,
            address_family: m.address_family,
            max_entries: m.max_entries,
            entries: m
                .entries
                .into_iter()
                .map(TypesPrefixListEntry::from)
                .collect(),
            tags: m.tags,
            version: m.version,
            version_history: m
                .version_history
                .into_iter()
                .map(|v| crate::types::ManagedPrefixListVersion {
                    version: v.version,
                    entries: v
                        .entries
                        .into_iter()
                        .map(TypesPrefixListEntry::from)
                        .collect(),
                })
                .collect(),
        }
    }
}

impl From<PrefixListEntryView> for TypesPrefixListEntry {
    fn from(e: PrefixListEntryView) -> Self {
        TypesPrefixListEntry {
            cidr: e.cidr,
            description: e.description,
        }
    }
}

impl From<CustomerGatewayView> for CustomerGateway {
    fn from(c: CustomerGatewayView) -> Self {
        CustomerGateway {
            customer_gateway_id: c.customer_gateway_id,
            bgp_asn: c.bgp_asn,
            ip_address: c.ip_address,
            gateway_type: c.gateway_type,
            state: c.state,
            tags: c.tags,
        }
    }
}

impl From<VpnGatewayView> for VpnGateway {
    fn from(v: VpnGatewayView) -> Self {
        VpnGateway {
            vpn_gateway_id: v.vpn_gateway_id,
            gateway_type: v.gateway_type,
            state: v.state,
            amazon_side_asn: v.amazon_side_asn,
            vpc_attachments: v
                .vpc_attachments
                .into_iter()
                .map(VgwVpcAttachment::from)
                .collect(),
            tags: v.tags,
        }
    }
}

impl From<VgwVpcAttachmentView> for VgwVpcAttachment {
    fn from(a: VgwVpcAttachmentView) -> Self {
        VgwVpcAttachment {
            vpc_id: a.vpc_id,
            state: a.state,
        }
    }
}

impl From<VpnConnectionView> for VpnConnection {
    fn from(c: VpnConnectionView) -> Self {
        VpnConnection {
            vpn_connection_id: c.vpn_connection_id,
            vpn_gateway_id: c.vpn_gateway_id,
            customer_gateway_id: c.customer_gateway_id,
            transit_gateway_id: c.transit_gateway_id,
            connection_type: c.connection_type,
            state: c.state,
            tags: c.tags,
            routes: c.routes.into_iter().map(VpnStaticRoute::from).collect(),
            options: c.options.map(VpnConnectionOptions::from),
            tunnel_replacement_status: c.tunnel_replacement_status,
        }
    }
}

impl From<VpnStaticRouteView> for VpnStaticRoute {
    fn from(r: VpnStaticRouteView) -> Self {
        VpnStaticRoute {
            destination_cidr_block: r.destination_cidr_block,
            source: r.source,
            state: r.state,
        }
    }
}

impl From<VpnConnectionOptionsView> for VpnConnectionOptions {
    fn from(o: VpnConnectionOptionsView) -> Self {
        VpnConnectionOptions {
            local_ipv4_network_cidr: o.local_ipv4_network_cidr,
            local_ipv6_network_cidr: o.local_ipv6_network_cidr,
            remote_ipv4_network_cidr: o.remote_ipv4_network_cidr,
            remote_ipv6_network_cidr: o.remote_ipv6_network_cidr,
            tunnel_inside_ip_version: o.tunnel_inside_ip_version,
            static_routes_only: o.static_routes_only,
            tunnel_options: o
                .tunnel_options
                .into_iter()
                .map(VpnTunnelOptions::from)
                .collect(),
        }
    }
}

impl From<VpnTunnelOptionsView> for VpnTunnelOptions {
    fn from(t: VpnTunnelOptionsView) -> Self {
        VpnTunnelOptions {
            tunnel_inside_cidr: t.tunnel_inside_cidr,
            tunnel_inside_ipv6_cidr: t.tunnel_inside_ipv6_cidr,
            pre_shared_key: t.pre_shared_key,
            outside_ip_address: t.outside_ip_address,
            certificate_arn: t.certificate_arn,
        }
    }
}

impl From<VpnConcentratorView> for VpnConcentrator {
    fn from(v: VpnConcentratorView) -> Self {
        VpnConcentrator {
            vpn_concentrator_id: v.vpn_concentrator_id,
            concentrator_type: v.concentrator_type,
            state: v.state,
            transit_gateway_id: v.transit_gateway_id,
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            tags: v.tags,
        }
    }
}

impl From<VpcEndpointConnectionView> for VpcEndpointConnection {
    fn from(c: VpcEndpointConnectionView) -> Self {
        VpcEndpointConnection {
            service_id: c.service_id,
            vpc_endpoint_id: c.vpc_endpoint_id,
            vpc_endpoint_owner: c.vpc_endpoint_owner,
            vpc_endpoint_state: c.vpc_endpoint_state,
            creation_timestamp: c.creation_timestamp,
        }
    }
}

impl From<VpcEndpointConnectionNotificationView> for VpcEndpointConnectionNotification {
    fn from(n: VpcEndpointConnectionNotificationView) -> Self {
        VpcEndpointConnectionNotification {
            connection_notification_id: n.connection_notification_id,
            connection_notification_arn: n.connection_notification_arn,
            connection_events: n.connection_events,
            connection_notification_state: n.connection_notification_state,
            connection_notification_type: n.connection_notification_type,
            service_id: n.service_id,
            vpc_endpoint_id: n.vpc_endpoint_id,
        }
    }
}

impl From<VpcBlockPublicAccessExclusionView> for VpcBlockPublicAccessExclusion {
    fn from(e: VpcBlockPublicAccessExclusionView) -> Self {
        VpcBlockPublicAccessExclusion {
            exclusion_id: e.exclusion_id,
            internet_gateway_exclusion_mode: e.internet_gateway_exclusion_mode,
            resource_arn: e.resource_arn,
            state: e.state,
            creation_timestamp: e.creation_timestamp,
            last_update_timestamp: e.last_update_timestamp,
            tags: e.tags,
        }
    }
}

impl From<VpcBlockPublicAccessOptionsView> for VpcBlockPublicAccessOptions {
    fn from(o: VpcBlockPublicAccessOptionsView) -> Self {
        VpcBlockPublicAccessOptions {
            aws_account_id: o.aws_account_id,
            aws_region: o.aws_region,
            internet_gateway_block_mode: o.internet_gateway_block_mode,
            state: o.state,
            last_update_timestamp: o.last_update_timestamp,
        }
    }
}

impl From<VpcEncryptionControlView> for VpcEncryptionControl {
    fn from(v: VpcEncryptionControlView) -> Self {
        VpcEncryptionControl {
            vpc_encryption_control_id: v.vpc_encryption_control_id,
            vpc_id: v.vpc_id,
            mode: v.mode,
            state: v.state,
            mode_history: v.mode_history,
            tags: v.tags,
        }
    }
}

impl From<CarrierGatewayView> for CarrierGateway {
    fn from(c: CarrierGatewayView) -> Self {
        CarrierGateway {
            carrier_gateway_id: c.carrier_gateway_id,
            vpc_id: c.vpc_id,
            state: c.state,
            tags: c.tags,
        }
    }
}

impl From<NetworkInterfaceView> for NetworkInterface {
    fn from(n: NetworkInterfaceView) -> Self {
        NetworkInterface {
            network_interface_id: n.network_interface_id,
            subnet_id: n.subnet_id,
            vpc_id: n.vpc_id,
            description: n.description,
            private_ip_address: n.private_ip_address,
            status: n.status,
            attachment_id: n.attachment_id,
            instance_id: n.instance_id,
            device_index: n.device_index,
            security_groups: n.security_groups,
            source_dest_check: n.source_dest_check,
            tags: n.tags,
            public_ip_dns_hostname_type: n.public_ip_dns_hostname_type,
        }
    }
}

impl From<TransitGatewayView> for TransitGateway {
    fn from(t: TransitGatewayView) -> Self {
        TransitGateway {
            transit_gateway_id: t.transit_gateway_id,
            state: t.state,
            amazon_side_asn: t.amazon_side_asn,
            description: t.description,
            dns_support: t.dns_support,
            vpn_ecmp_support: t.vpn_ecmp_support,
            multicast_support: t.multicast_support,
            tags: t.tags,
        }
    }
}

impl From<TgwVpcAttachmentView> for TransitGatewayVpcAttachment {
    fn from(a: TgwVpcAttachmentView) -> Self {
        TransitGatewayVpcAttachment {
            attachment_id: a.attachment_id,
            transit_gateway_id: a.transit_gateway_id,
            vpc_id: a.vpc_id,
            subnet_ids: a.subnet_ids,
            state: a.state,
            tags: a.tags,
        }
    }
}

impl From<TgwPeeringAttachmentView> for TransitGatewayPeeringAttachment {
    fn from(a: TgwPeeringAttachmentView) -> Self {
        TransitGatewayPeeringAttachment {
            attachment_id: a.attachment_id,
            transit_gateway_id: a.transit_gateway_id,
            peer_transit_gateway_id: a.peer_transit_gateway_id,
            peer_account_id: a.peer_account_id,
            peer_region: a.peer_region,
            state: a.state,
            tags: a.tags,
        }
    }
}

impl From<TgwRouteTableView> for TransitGatewayRouteTable {
    fn from(t: TgwRouteTableView) -> Self {
        TransitGatewayRouteTable {
            route_table_id: t.route_table_id,
            transit_gateway_id: t.transit_gateway_id,
            state: t.state,
            default_association_route_table: t.default_association_route_table,
            default_propagation_route_table: t.default_propagation_route_table,
            tags: t.tags,
        }
    }
}

impl From<TgwRouteView> for TransitGatewayRoute {
    fn from(r: TgwRouteView) -> Self {
        TransitGatewayRoute {
            destination_cidr_block: r.destination_cidr_block,
            route_type: r.route_type,
            state: r.state,
            attachment_id: r.attachment_id,
        }
    }
}

// --- Group 11: View → internal `From` impls ---

impl From<TransitGatewayMulticastDomainView> for TransitGatewayMulticastDomain {
    fn from(v: TransitGatewayMulticastDomainView) -> Self {
        TransitGatewayMulticastDomain {
            transit_gateway_multicast_domain_id: v.transit_gateway_multicast_domain_id,
            transit_gateway_id: v.transit_gateway_id,
            transit_gateway_multicast_domain_arn: v.transit_gateway_multicast_domain_arn,
            owner_id: v.owner_id,
            igmpv2_support: v.igmpv2_support,
            static_sources_support: v.static_sources_support,
            auto_accept_shared_associations: v.auto_accept_shared_associations,
            state: v.state,
            creation_time: v.creation_time,
            tags: v.tags,
        }
    }
}

impl From<MulticastSubnetAssociationView> for MulticastSubnetAssociation {
    fn from(v: MulticastSubnetAssociationView) -> Self {
        MulticastSubnetAssociation {
            subnet_id: v.subnet_id,
            state: v.state,
        }
    }
}

impl From<TransitGatewayMulticastDomainAssociationView>
    for TransitGatewayMulticastDomainAssociation
{
    fn from(v: TransitGatewayMulticastDomainAssociationView) -> Self {
        TransitGatewayMulticastDomainAssociation {
            transit_gateway_multicast_domain_id: v.transit_gateway_multicast_domain_id,
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            resource_id: v.resource_id,
            resource_type: v.resource_type,
            subnets: v
                .subnets
                .into_iter()
                .map(MulticastSubnetAssociation::from)
                .collect(),
        }
    }
}

impl From<TransitGatewayMulticastGroupMemberView> for TransitGatewayMulticastGroupMember {
    fn from(v: TransitGatewayMulticastGroupMemberView) -> Self {
        TransitGatewayMulticastGroupMember {
            transit_gateway_multicast_domain_id: v.transit_gateway_multicast_domain_id,
            group_ip_address: v.group_ip_address,
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            subnet_id: v.subnet_id,
            resource_id: v.resource_id,
            resource_type: v.resource_type,
            network_interface_id: v.network_interface_id,
            member_type: v.member_type,
            source_type: v.source_type,
        }
    }
}

impl From<TransitGatewayMulticastGroupSourceView> for TransitGatewayMulticastGroupSource {
    fn from(v: TransitGatewayMulticastGroupSourceView) -> Self {
        TransitGatewayMulticastGroupSource {
            transit_gateway_multicast_domain_id: v.transit_gateway_multicast_domain_id,
            group_ip_address: v.group_ip_address,
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            subnet_id: v.subnet_id,
            resource_id: v.resource_id,
            resource_type: v.resource_type,
            network_interface_id: v.network_interface_id,
            member_type: v.member_type,
            source_type: v.source_type,
        }
    }
}

impl From<TransitGatewayConnectView> for TransitGatewayConnect {
    fn from(v: TransitGatewayConnectView) -> Self {
        TransitGatewayConnect {
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            transport_transit_gateway_attachment_id: v.transport_transit_gateway_attachment_id,
            transit_gateway_id: v.transit_gateway_id,
            state: v.state,
            creation_time: v.creation_time,
            protocol: v.protocol,
            tags: v.tags,
        }
    }
}

impl From<TransitGatewayAttachmentBgpConfigurationView>
    for TransitGatewayAttachmentBgpConfiguration
{
    fn from(v: TransitGatewayAttachmentBgpConfigurationView) -> Self {
        TransitGatewayAttachmentBgpConfiguration {
            transit_gateway_asn: v.transit_gateway_asn,
            peer_asn: v.peer_asn,
            transit_gateway_address: v.transit_gateway_address,
            peer_address: v.peer_address,
            bgp_status: v.bgp_status,
        }
    }
}

impl From<TransitGatewayConnectPeerView> for TransitGatewayConnectPeer {
    fn from(v: TransitGatewayConnectPeerView) -> Self {
        TransitGatewayConnectPeer {
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            transit_gateway_connect_peer_id: v.transit_gateway_connect_peer_id,
            state: v.state,
            creation_time: v.creation_time,
            transit_gateway_address: v.transit_gateway_address,
            peer_address: v.peer_address,
            inside_cidr_blocks: v.inside_cidr_blocks,
            protocol: v.protocol,
            bgp_configurations: v
                .bgp_configurations
                .into_iter()
                .map(TransitGatewayAttachmentBgpConfiguration::from)
                .collect(),
            tags: v.tags,
        }
    }
}

impl From<TransitGatewayMeteringPolicyView> for TransitGatewayMeteringPolicy {
    fn from(v: TransitGatewayMeteringPolicyView) -> Self {
        TransitGatewayMeteringPolicy {
            transit_gateway_metering_policy_id: v.transit_gateway_metering_policy_id,
            transit_gateway_metering_policy_arn: v.transit_gateway_metering_policy_arn,
            transit_gateway_id: v.transit_gateway_id,
            name: v.name,
            description: v.description,
            state: v.state,
            tags: v.tags,
            last_updated_time: v.last_updated_time,
            version: v.version,
            middlebox_attachment_ids: v.middlebox_attachment_ids,
        }
    }
}

impl From<TransitGatewayMeteringPolicyEntryView> for TransitGatewayMeteringPolicyEntry {
    fn from(v: TransitGatewayMeteringPolicyEntryView) -> Self {
        TransitGatewayMeteringPolicyEntry {
            transit_gateway_metering_policy_entry_id: v.transit_gateway_metering_policy_entry_id,
            transit_gateway_metering_policy_id: v.transit_gateway_metering_policy_id,
            sequence_number: v.sequence_number,
            action: v.action,
            source_cidr_block: v.source_cidr_block,
            destination_cidr_block: v.destination_cidr_block,
            protocol: v.protocol,
            source_port: v.source_port,
            destination_port: v.destination_port,
            dimensions: v.dimensions,
            state: v.state,
        }
    }
}

impl From<TransitGatewayPolicyTableView> for TransitGatewayPolicyTable {
    fn from(v: TransitGatewayPolicyTableView) -> Self {
        TransitGatewayPolicyTable {
            transit_gateway_policy_table_id: v.transit_gateway_policy_table_id,
            transit_gateway_id: v.transit_gateway_id,
            state: v.state,
            creation_time: v.creation_time,
            tags: v.tags,
        }
    }
}

impl From<TransitGatewayPolicyTableAssociationView> for TransitGatewayPolicyTableAssociation {
    fn from(v: TransitGatewayPolicyTableAssociationView) -> Self {
        TransitGatewayPolicyTableAssociation {
            transit_gateway_policy_table_id: v.transit_gateway_policy_table_id,
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            resource_type: v.resource_type,
            resource_id: v.resource_id,
            state: v.state,
        }
    }
}

impl From<TransitGatewayPrefixListReferenceView> for TransitGatewayPrefixListReference {
    fn from(v: TransitGatewayPrefixListReferenceView) -> Self {
        TransitGatewayPrefixListReference {
            transit_gateway_route_table_id: v.transit_gateway_route_table_id,
            prefix_list_id: v.prefix_list_id,
            prefix_list_owner_id: v.prefix_list_owner_id,
            state: v.state,
            blackhole: v.blackhole,
            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
            resource_id: v.resource_id,
            resource_type: v.resource_type,
        }
    }
}

impl From<TransitGatewayRouteTableAnnouncementView> for TransitGatewayRouteTableAnnouncement {
    fn from(v: TransitGatewayRouteTableAnnouncementView) -> Self {
        TransitGatewayRouteTableAnnouncement {
            transit_gateway_route_table_announcement_id: v
                .transit_gateway_route_table_announcement_id,
            transit_gateway_id: v.transit_gateway_id,
            core_network_id: v.core_network_id,
            peer_transit_gateway_id: v.peer_transit_gateway_id,
            peer_core_network_id: v.peer_core_network_id,
            peering_attachment_id: v.peering_attachment_id,
            announcement_direction: v.announcement_direction,
            transit_gateway_route_table_id: v.transit_gateway_route_table_id,
            state: v.state,
            creation_time: v.creation_time,
            tags: v.tags,
        }
    }
}

impl From<InstanceView> for Instance {
    fn from(i: InstanceView) -> Self {
        Instance {
            instance_id: i.instance_id,
            image_id: i.image_id,
            instance_type: i.instance_type,
            state: InstanceState {
                code: i.state.code,
                name: i.state.name,
            },
            private_ip_address: i.private_ip_address,
            public_ip_address: i.public_ip_address,
            subnet_id: i.subnet_id,
            vpc_id: i.vpc_id,
            key_name: i.key_name,
            security_groups: i.security_groups,
            launch_time: i.launch_time,
            tags: i.tags,
            iam_instance_profile_arn: i.iam_instance_profile_arn,
            monitoring_state: i.monitoring_state,
            placement_az: i.placement_az,
            placement_group_name: i.placement_group_name,
            placement_tenancy: i.placement_tenancy,
            placement_host_id: i.placement_host_id,
            placement_affinity: i.placement_affinity,
            placement_partition_number: i.placement_partition_number,
            owner_id: i.owner_id,
            classic_link_vpc: i.classic_link_vpc,
            private_dns_hostname_type: i.private_dns_hostname_type,
            enable_resource_name_dns_a_record: i.enable_resource_name_dns_a_record,
            enable_resource_name_dns_aaaa_record: i.enable_resource_name_dns_aaaa_record,
            credit_specification: i.credit_specification,
            cpu_options: i.cpu_options.map(|c| crate::types::InstanceCpuOptions {
                core_count: c.core_count,
                threads_per_core: c.threads_per_core,
                amd_sev_snp: c.amd_sev_snp,
            }),
            maintenance_options: i.maintenance_options.map(|m| {
                crate::types::InstanceMaintenanceOptions {
                    auto_recovery: m.auto_recovery,
                    reboot_migration: m.reboot_migration,
                }
            }),
            network_bandwidth_weighting: i.network_bandwidth_weighting,
            lifecycle: i.lifecycle,
            product_codes: i.product_codes,
            capacity_reservation_specification: i
                .capacity_reservation_specification
                .map(CapacityReservationSpecificationResponse::from),
        }
    }
}

impl From<CapacityReservationFleetView> for CapacityReservationFleet {
    fn from(v: CapacityReservationFleetView) -> Self {
        CapacityReservationFleet {
            capacity_reservation_fleet_id: v.capacity_reservation_fleet_id,
            capacity_reservation_fleet_arn: v.capacity_reservation_fleet_arn,
            state: v.state,
            tenancy: v.tenancy,
            allocation_strategy: v.allocation_strategy,
            instance_match_criteria: v.instance_match_criteria,
            total_target_capacity: v.total_target_capacity,
            total_fulfilled_capacity: v.total_fulfilled_capacity,
            create_time: v.create_time,
            end_date: v.end_date,
            instance_type_specifications: v
                .instance_type_specifications
                .into_iter()
                .map(CapacityReservationFleetInstanceSpec::from)
                .collect(),
            tags: v.tags,
        }
    }
}

impl From<CapacityReservationFleetInstanceSpecView> for CapacityReservationFleetInstanceSpec {
    fn from(v: CapacityReservationFleetInstanceSpecView) -> Self {
        CapacityReservationFleetInstanceSpec {
            instance_type: v.instance_type,
            instance_platform: v.instance_platform,
            availability_zone: v.availability_zone,
            ebs_optimized: v.ebs_optimized,
            priority: v.priority,
            weight: v.weight,
        }
    }
}

impl From<VolumeAttachmentView> for VolumeAttachment {
    fn from(a: VolumeAttachmentView) -> Self {
        VolumeAttachment {
            volume_id: a.volume_id,
            instance_id: a.instance_id,
            device: a.device,
            state: a.state,
            attach_time: a.attach_time,
            delete_on_termination: a.delete_on_termination,
        }
    }
}

impl From<VolumeView> for Volume {
    fn from(v: VolumeView) -> Self {
        Volume {
            volume_id: v.volume_id,
            size: v.size,
            snapshot_id: v.snapshot_id,
            availability_zone: v.availability_zone,
            state: v.state,
            volume_type: v.volume_type,
            iops: v.iops,
            throughput: v.throughput,
            encrypted: v.encrypted,
            create_time: v.create_time,
            attachments: v
                .attachments
                .into_iter()
                .map(VolumeAttachment::from)
                .collect(),
            tags: v.tags,
            recycle_bin_state: v.recycle_bin_state,
            source_volume_id: v.source_volume_id,
        }
    }
}

impl From<SnapshotView> for Snapshot {
    fn from(s: SnapshotView) -> Self {
        use crate::types::FastSnapshotRestoreState;
        Snapshot {
            snapshot_id: s.snapshot_id,
            volume_id: s.volume_id,
            volume_size: s.volume_size,
            state: s.state,
            description: s.description,
            start_time: s.start_time,
            progress: s.progress,
            owner_id: s.owner_id,
            encrypted: s.encrypted,
            tags: s.tags,
            lock_state: s.lock_state,
            lock_duration: s.lock_duration,
            lock_created_on: s.lock_created_on,
            lock_expires_on: s.lock_expires_on,
            lock_duration_start_time: s.lock_duration_start_time,
            cool_off_period: s.cool_off_period,
            cool_off_period_expires_on: s.cool_off_period_expires_on,
            storage_tier: s.storage_tier,
            last_tiering_operation_status: s.last_tiering_operation_status,
            fast_snapshot_restore_states: s
                .fast_snapshot_restore_states
                .into_iter()
                .map(|f| FastSnapshotRestoreState {
                    availability_zone: f.availability_zone,
                    state: f.state,
                })
                .collect(),
        }
    }
}

impl From<ImageView> for Image {
    fn from(i: ImageView) -> Self {
        Image {
            image_id: i.image_id,
            name: i.name,
            description: i.description,
            state: i.state,
            owner_id: i.owner_id,
            architecture: i.architecture,
            image_type: i.image_type,
            platform: i.platform,
            virtualization_type: i.virtualization_type,
            root_device_type: i.root_device_type,
            root_device_name: i.root_device_name,
            public: i.public,
            tags: i.tags,
            source_instance_id: i.source_instance_id,
            source_instance_type: i.source_instance_type,
            launch_permissions: i
                .launch_permissions
                .into_iter()
                .map(crate::types::LaunchPermission::from)
                .collect(),
            recycle_bin_state: i.recycle_bin_state,
            deprecation_time: i.deprecation_time,
            recycle_bin_enter_time: i.recycle_bin_enter_time,
            product_codes: i.product_codes,
            fast_launch_state: i.fast_launch_state.map(crate::types::FastLaunchState::from),
            deregistration_protection: i.deregistration_protection,
        }
    }
}

impl From<LaunchPermissionView> for crate::types::LaunchPermission {
    fn from(p: LaunchPermissionView) -> Self {
        crate::types::LaunchPermission {
            user_id: p.user_id,
            group: p.group,
        }
    }
}

impl From<LaunchTemplateView> for LaunchTemplate {
    fn from(t: LaunchTemplateView) -> Self {
        LaunchTemplate {
            launch_template_id: t.launch_template_id,
            launch_template_name: t.launch_template_name,
            default_version_number: t.default_version_number,
            latest_version_number: t.latest_version_number,
            tags: t.tags,
        }
    }
}

impl From<LaunchTemplateVersionView> for LaunchTemplateVersion {
    fn from(v: LaunchTemplateVersionView) -> Self {
        LaunchTemplateVersion {
            version_number: v.version_number,
            launch_template_id: v.launch_template_id,
            launch_template_name: v.launch_template_name,
            version_description: v.version_description,
            data: v.data,
            default_version: v.default_version,
        }
    }
}

impl From<SpotInstanceRequestView> for SpotInstanceRequest {
    fn from(s: SpotInstanceRequestView) -> Self {
        SpotInstanceRequest {
            spot_instance_request_id: s.spot_instance_request_id,
            spot_price: s.spot_price,
            instance_type: s.instance_type,
            image_id: s.image_id,
            state: s.state,
            status_code: s.status_code,
            instance_id: s.instance_id,
            tags: s.tags,
        }
    }
}

impl From<IamInstanceProfileAssociationView> for IamInstanceProfileAssociation {
    fn from(a: IamInstanceProfileAssociationView) -> Self {
        IamInstanceProfileAssociation {
            association_id: a.association_id,
            instance_id: a.instance_id,
            iam_instance_profile_arn: a.iam_instance_profile_arn,
            iam_instance_profile_name: a.iam_instance_profile_name,
            state: a.state,
        }
    }
}

impl From<DedicatedHostView> for DedicatedHost {
    fn from(h: DedicatedHostView) -> Self {
        DedicatedHost {
            host_id: h.host_id,
            availability_zone: h.availability_zone,
            instance_type: h.instance_type,
            auto_placement: h.auto_placement,
            host_recovery: h.host_recovery,
            state: h.state,
            tags: h.tags,
        }
    }
}

impl From<Ec2FleetView> for Ec2Fleet {
    fn from(f: Ec2FleetView) -> Self {
        Ec2Fleet {
            fleet_id: f.fleet_id,
            state: f.state,
            fleet_type: f.fleet_type,
            create_time: f.create_time,
            tags: f.tags,
            total_target_capacity: f.total_target_capacity,
            on_demand_target_capacity: f.on_demand_target_capacity,
            spot_target_capacity: f.spot_target_capacity,
            context: f.context,
        }
    }
}

impl From<VpcEndpointServiceConfigView> for VpcEndpointServiceConfiguration {
    fn from(s: VpcEndpointServiceConfigView) -> Self {
        VpcEndpointServiceConfiguration {
            service_id: s.service_id,
            service_name: s.service_name,
            service_type: s.service_type,
            acceptance_required: s.acceptance_required,
            state: s.state,
            network_load_balancer_arns: s.network_load_balancer_arns,
            gateway_load_balancer_arns: s.gateway_load_balancer_arns,
            allowed_principals: s.allowed_principals,
            tags: s.tags,
            payer_responsibility: s.payer_responsibility,
            private_dns_state: s.private_dns_state,
        }
    }
}

impl From<SpotFleetRequestView> for SpotFleetRequest {
    fn from(s: SpotFleetRequestView) -> Self {
        SpotFleetRequest {
            spot_fleet_request_id: s.spot_fleet_request_id,
            spot_fleet_request_state: s.spot_fleet_request_state,
            target_capacity: s.target_capacity,
            iam_fleet_role: s.iam_fleet_role,
            create_time: s.create_time,
            tags: s.tags,
        }
    }
}

impl From<SubnetCidrReservationView> for SubnetCidrReservationEntry {
    fn from(r: SubnetCidrReservationView) -> Self {
        SubnetCidrReservationEntry {
            reservation_id: r.reservation_id,
            subnet_id: r.subnet_id,
            cidr: r.cidr,
            reservation_type: r.reservation_type,
            description: r.description,
            owner_id: r.owner_id,
        }
    }
}

impl From<PlacementGroupView> for PlacementGroup {
    fn from(p: PlacementGroupView) -> Self {
        PlacementGroup {
            group_id: p.group_id,
            group_name: p.group_name,
            group_arn: p.group_arn,
            strategy: p.strategy,
            state: p.state,
            partition_count: p.partition_count,
            spread_level: p.spread_level,
            tags: p.tags,
        }
    }
}

impl From<NetworkInterfacePermissionView> for NetworkInterfacePermission {
    fn from(p: NetworkInterfacePermissionView) -> Self {
        NetworkInterfacePermission {
            network_interface_permission_id: p.network_interface_permission_id,
            network_interface_id: p.network_interface_id,
            aws_account_id: p.aws_account_id,
            aws_service: p.aws_service,
            permission: p.permission,
            permission_state: p.permission_state,
        }
    }
}

impl From<CapacityReservationView> for CapacityReservation {
    fn from(c: CapacityReservationView) -> Self {
        CapacityReservation {
            capacity_reservation_id: c.capacity_reservation_id,
            capacity_reservation_arn: c.capacity_reservation_arn,
            owner_id: c.owner_id,
            instance_type: c.instance_type,
            instance_platform: c.instance_platform,
            availability_zone: c.availability_zone,
            tenancy: c.tenancy,
            total_instance_count: c.total_instance_count,
            available_instance_count: c.available_instance_count,
            ebs_optimized: c.ebs_optimized,
            ephemeral_storage: c.ephemeral_storage,
            state: c.state,
            start_date: c.start_date,
            end_date: c.end_date,
            end_date_type: c.end_date_type,
            instance_match_criteria: c.instance_match_criteria,
            create_date: c.create_date,
            outpost_arn: c.outpost_arn,
            placement_group_arn: c.placement_group_arn,
            tags: c.tags,
            pending_billing_owner_account_id: c.pending_billing_owner_account_id,
            billing_owner_account_id: c.billing_owner_account_id,
            target_capacity_reservation_id: c.target_capacity_reservation_id,
            reservation_type: c.reservation_type,
            commitment_info: c
                .commitment_info
                .map(CapacityReservationCommitmentInfo::from),
        }
    }
}

impl From<&CoipPool> for CoipPoolView {
    fn from(p: &CoipPool) -> Self {
        CoipPoolView {
            pool_id: p.pool_id.clone(),
            pool_arn: p.pool_arn.clone(),
            local_gateway_route_table_id: p.local_gateway_route_table_id.clone(),
            pool_cidrs: p.pool_cidrs.clone(),
            tags: p.tags.clone(),
        }
    }
}

impl From<CoipPoolView> for CoipPool {
    fn from(v: CoipPoolView) -> Self {
        CoipPool {
            pool_id: v.pool_id,
            pool_arn: v.pool_arn,
            local_gateway_route_table_id: v.local_gateway_route_table_id,
            pool_cidrs: v.pool_cidrs,
            tags: v.tags,
        }
    }
}

impl From<&SecurityGroupVpcAssociation> for SecurityGroupVpcAssociationView {
    fn from(a: &SecurityGroupVpcAssociation) -> Self {
        SecurityGroupVpcAssociationView {
            group_id: a.group_id.clone(),
            vpc_id: a.vpc_id.clone(),
            vpc_owner_id: a.vpc_owner_id.clone(),
            state: a.state.clone(),
        }
    }
}

impl From<SecurityGroupVpcAssociationView> for SecurityGroupVpcAssociation {
    fn from(v: SecurityGroupVpcAssociationView) -> Self {
        SecurityGroupVpcAssociation {
            group_id: v.group_id,
            vpc_id: v.vpc_id,
            vpc_owner_id: v.vpc_owner_id,
            state: v.state,
        }
    }
}

impl From<&EnclaveCertificateIamRoleAssociation> for EnclaveCertificateIamRoleAssociationView {
    fn from(a: &EnclaveCertificateIamRoleAssociation) -> Self {
        EnclaveCertificateIamRoleAssociationView {
            certificate_arn: a.certificate_arn.clone(),
            role_arn: a.role_arn.clone(),
            certificate_s3_bucket_name: a.certificate_s3_bucket_name.clone(),
            certificate_s3_object_key: a.certificate_s3_object_key.clone(),
            encryption_kms_key_id: a.encryption_kms_key_id.clone(),
        }
    }
}

impl From<EnclaveCertificateIamRoleAssociationView> for EnclaveCertificateIamRoleAssociation {
    fn from(v: EnclaveCertificateIamRoleAssociationView) -> Self {
        EnclaveCertificateIamRoleAssociation {
            certificate_arn: v.certificate_arn,
            role_arn: v.role_arn,
            certificate_s3_bucket_name: v.certificate_s3_bucket_name,
            certificate_s3_object_key: v.certificate_s3_object_key,
            encryption_kms_key_id: v.encryption_kms_key_id,
        }
    }
}

impl From<&MacSipModificationTask> for MacSipModificationTaskView {
    fn from(t: &MacSipModificationTask) -> Self {
        MacSipModificationTaskView {
            task_id: t.task_id.clone(),
            instance_id: t.instance_id.clone(),
            task_type: t.task_type.clone(),
            task_state: t.task_state.clone(),
            start_time: t.start_time.clone(),
            apple_internal: t.apple_internal.clone(),
            base_system: t.base_system.clone(),
            debugging_restrictions: t.debugging_restrictions.clone(),
            dtrace_restrictions: t.dtrace_restrictions.clone(),
            filesystem_protections: t.filesystem_protections.clone(),
            kext_signing: t.kext_signing.clone(),
            nvram_protections: t.nvram_protections.clone(),
            status: t.status.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<MacSipModificationTaskView> for MacSipModificationTask {
    fn from(v: MacSipModificationTaskView) -> Self {
        MacSipModificationTask {
            task_id: v.task_id,
            instance_id: v.instance_id,
            task_type: v.task_type,
            task_state: v.task_state,
            start_time: v.start_time,
            apple_internal: v.apple_internal,
            base_system: v.base_system,
            debugging_restrictions: v.debugging_restrictions,
            dtrace_restrictions: v.dtrace_restrictions,
            filesystem_protections: v.filesystem_protections,
            kext_signing: v.kext_signing,
            nvram_protections: v.nvram_protections,
            status: v.status,
            tags: v.tags,
        }
    }
}

impl From<&DeclarativePoliciesReport> for DeclarativePoliciesReportView {
    fn from(r: &DeclarativePoliciesReport) -> Self {
        DeclarativePoliciesReportView {
            report_id: r.report_id.clone(),
            s3_bucket: r.s3_bucket.clone(),
            s3_prefix: r.s3_prefix.clone(),
            target_id: r.target_id.clone(),
            status: r.status.clone(),
            start_time: r.start_time.clone(),
            end_time: r.end_time.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<DeclarativePoliciesReportView> for DeclarativePoliciesReport {
    fn from(v: DeclarativePoliciesReportView) -> Self {
        DeclarativePoliciesReport {
            report_id: v.report_id,
            s3_bucket: v.s3_bucket,
            s3_prefix: v.s3_prefix,
            target_id: v.target_id,
            status: v.status,
            start_time: v.start_time,
            end_time: v.end_time,
            tags: v.tags,
        }
    }
}

impl From<InstanceConnectEndpointView> for InstanceConnectEndpoint {
    fn from(i: InstanceConnectEndpointView) -> Self {
        InstanceConnectEndpoint {
            instance_connect_endpoint_id: i.instance_connect_endpoint_id,
            instance_connect_endpoint_arn: i.instance_connect_endpoint_arn,
            subnet_id: i.subnet_id,
            vpc_id: i.vpc_id,
            availability_zone: i.availability_zone,
            state: i.state,
            created_at: i.created_at,
            preserve_client_ip: i.preserve_client_ip,
            security_group_ids: i.security_group_ids,
            network_interface_ids: i.network_interface_ids,
            dns_name: i.dns_name,
            fips_dns_name: i.fips_dns_name,
            ip_address_type: i.ip_address_type,
            owner_id: i.owner_id,
            tags: i.tags,
        }
    }
}

// ---------------------------------------------------------------------------
// Group 4: View structs + round-trip From impls
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacVolumeOwnershipTaskView {
    pub task_id: String,
    pub mac_volume_ownership_task_state: String,
    pub volume_id: String,
    pub source_volume_owner_account_id: String,
    pub target_volume_owner_account_id: String,
    pub creation_time: String,
    #[serde(default)]
    pub completion_time: Option<String>,
}

impl From<&MacVolumeOwnershipTask> for MacVolumeOwnershipTaskView {
    fn from(t: &MacVolumeOwnershipTask) -> Self {
        Self {
            task_id: t.task_id.clone(),
            mac_volume_ownership_task_state: t.mac_volume_ownership_task_state.clone(),
            volume_id: t.volume_id.clone(),
            source_volume_owner_account_id: t.source_volume_owner_account_id.clone(),
            target_volume_owner_account_id: t.target_volume_owner_account_id.clone(),
            creation_time: t.creation_time.clone(),
            completion_time: t.completion_time.clone(),
        }
    }
}

impl From<MacVolumeOwnershipTaskView> for MacVolumeOwnershipTask {
    fn from(v: MacVolumeOwnershipTaskView) -> Self {
        Self {
            task_id: v.task_id,
            mac_volume_ownership_task_state: v.mac_volume_ownership_task_state,
            volume_id: v.volume_id,
            source_volume_owner_account_id: v.source_volume_owner_account_id,
            target_volume_owner_account_id: v.target_volume_owner_account_id,
            creation_time: v.creation_time,
            completion_time: v.completion_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceRootVolumeTaskView {
    pub task_id: String,
    pub instance_id: String,
    pub task_state: String,
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub snapshot_id: Option<String>,
    pub delete_replaced_root_volume: bool,
    pub start_time: String,
    #[serde(default)]
    pub complete_time: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&ReplaceRootVolumeTask> for ReplaceRootVolumeTaskView {
    fn from(t: &ReplaceRootVolumeTask) -> Self {
        Self {
            task_id: t.task_id.clone(),
            instance_id: t.instance_id.clone(),
            task_state: t.task_state.clone(),
            image_id: t.image_id.clone(),
            snapshot_id: t.snapshot_id.clone(),
            delete_replaced_root_volume: t.delete_replaced_root_volume,
            start_time: t.start_time.clone(),
            complete_time: t.complete_time.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<ReplaceRootVolumeTaskView> for ReplaceRootVolumeTask {
    fn from(v: ReplaceRootVolumeTaskView) -> Self {
        Self {
            task_id: v.task_id,
            instance_id: v.instance_id,
            task_state: v.task_state,
            image_id: v.image_id,
            snapshot_id: v.snapshot_id,
            delete_replaced_root_volume: v.delete_replaced_root_volume,
            start_time: v.start_time,
            complete_time: v.complete_time,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotImportTaskView {
    pub import_task_id: String,
    pub status: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub disk_image_size: Option<f64>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub user_bucket_s3_bucket: Option<String>,
    #[serde(default)]
    pub user_bucket_s3_key: Option<String>,
    pub owner_id: String,
    pub encrypted: bool,
    #[serde(default)]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub snapshot_id: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&SnapshotImportTask> for SnapshotImportTaskView {
    fn from(t: &SnapshotImportTask) -> Self {
        Self {
            import_task_id: t.import_task_id.clone(),
            status: t.status.clone(),
            description: t.description.clone(),
            disk_image_size: t.disk_image_size,
            format: t.format.clone(),
            url: t.url.clone(),
            user_bucket_s3_bucket: t.user_bucket_s3_bucket.clone(),
            user_bucket_s3_key: t.user_bucket_s3_key.clone(),
            owner_id: t.owner_id.clone(),
            encrypted: t.encrypted,
            kms_key_id: t.kms_key_id.clone(),
            snapshot_id: t.snapshot_id.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<SnapshotImportTaskView> for SnapshotImportTask {
    fn from(v: SnapshotImportTaskView) -> Self {
        Self {
            import_task_id: v.import_task_id,
            status: v.status,
            description: v.description,
            disk_image_size: v.disk_image_size,
            format: v.format,
            url: v.url,
            user_bucket_s3_bucket: v.user_bucket_s3_bucket,
            user_bucket_s3_key: v.user_bucket_s3_key,
            owner_id: v.owner_id,
            encrypted: v.encrypted,
            kms_key_id: v.kms_key_id,
            snapshot_id: v.snapshot_id,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInstanceVolumeDetailView {
    #[serde(default)]
    pub availability_zone: Option<String>,
    #[serde(default)]
    pub bytes_converted: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    pub status: String,
}

impl From<&ImportInstanceVolumeDetail> for ImportInstanceVolumeDetailView {
    fn from(v: &ImportInstanceVolumeDetail) -> Self {
        Self {
            availability_zone: v.availability_zone.clone(),
            bytes_converted: v.bytes_converted,
            description: v.description.clone(),
            status: v.status.clone(),
        }
    }
}

impl From<ImportInstanceVolumeDetailView> for ImportInstanceVolumeDetail {
    fn from(v: ImportInstanceVolumeDetailView) -> Self {
        Self {
            availability_zone: v.availability_zone,
            bytes_converted: v.bytes_converted,
            description: v.description,
            status: v.status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionTaskView {
    pub conversion_task_id: String,
    pub expiration_time: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub instance_id: Option<String>,
    pub platform: String,
    #[serde(default)]
    pub volumes: Vec<ImportInstanceVolumeDetailView>,
    pub state: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&ConversionTask> for ConversionTaskView {
    fn from(t: &ConversionTask) -> Self {
        Self {
            conversion_task_id: t.conversion_task_id.clone(),
            expiration_time: t.expiration_time.clone(),
            description: t.description.clone(),
            instance_id: t.instance_id.clone(),
            platform: t.platform.clone(),
            volumes: t
                .volumes
                .iter()
                .map(ImportInstanceVolumeDetailView::from)
                .collect(),
            state: t.state.clone(),
            status_message: t.status_message.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<ConversionTaskView> for ConversionTask {
    fn from(v: ConversionTaskView) -> Self {
        Self {
            conversion_task_id: v.conversion_task_id,
            expiration_time: v.expiration_time,
            description: v.description,
            instance_id: v.instance_id,
            platform: v.platform,
            volumes: v
                .volumes
                .into_iter()
                .map(ImportInstanceVolumeDetail::from)
                .collect(),
            state: v.state,
            status_message: v.status_message,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaskView {
    pub export_task_id: String,
    pub description: String,
    pub instance_id: String,
    pub target_environment: String,
    pub disk_image_format: String,
    #[serde(default)]
    pub container_format: Option<String>,
    pub s3_bucket: String,
    #[serde(default)]
    pub s3_prefix: Option<String>,
    pub s3_key: String,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&ExportTask> for ExportTaskView {
    fn from(t: &ExportTask) -> Self {
        Self {
            export_task_id: t.export_task_id.clone(),
            description: t.description.clone(),
            instance_id: t.instance_id.clone(),
            target_environment: t.target_environment.clone(),
            disk_image_format: t.disk_image_format.clone(),
            container_format: t.container_format.clone(),
            s3_bucket: t.s3_bucket.clone(),
            s3_prefix: t.s3_prefix.clone(),
            s3_key: t.s3_key.clone(),
            status: t.status.clone(),
            status_message: t.status_message.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<ExportTaskView> for ExportTask {
    fn from(v: ExportTaskView) -> Self {
        Self {
            export_task_id: v.export_task_id,
            description: v.description,
            instance_id: v.instance_id,
            target_environment: v.target_environment,
            disk_image_format: v.disk_image_format,
            container_format: v.container_format,
            s3_bucket: v.s3_bucket,
            s3_prefix: v.s3_prefix,
            s3_key: v.s3_key,
            status: v.status,
            status_message: v.status_message,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrunkInterfaceAssociationView {
    pub association_id: String,
    pub branch_interface_id: String,
    pub trunk_interface_id: String,
    pub interface_protocol: String,
    #[serde(default)]
    pub vlan_id: Option<i32>,
    #[serde(default)]
    pub gre_key: Option<i32>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&TrunkInterfaceAssociation> for TrunkInterfaceAssociationView {
    fn from(a: &TrunkInterfaceAssociation) -> Self {
        Self {
            association_id: a.association_id.clone(),
            branch_interface_id: a.branch_interface_id.clone(),
            trunk_interface_id: a.trunk_interface_id.clone(),
            interface_protocol: a.interface_protocol.clone(),
            vlan_id: a.vlan_id,
            gre_key: a.gre_key,
            tags: a.tags.clone(),
        }
    }
}

impl From<TrunkInterfaceAssociationView> for TrunkInterfaceAssociation {
    fn from(v: TrunkInterfaceAssociationView) -> Self {
        Self {
            association_id: v.association_id,
            branch_interface_id: v.branch_interface_id,
            trunk_interface_id: v.trunk_interface_id,
            interface_protocol: v.interface_protocol,
            vlan_id: v.vlan_id,
            gre_key: v.gre_key,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryNetworkView {
    pub network_id: String,
    pub vpc_id: String,
    pub primary_cidr_block: String,
    #[serde(default)]
    pub secondary_cidr_blocks: Vec<String>,
    pub state: String,
    #[serde(default)]
    pub network_border_group: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&SecondaryNetwork> for SecondaryNetworkView {
    fn from(n: &SecondaryNetwork) -> Self {
        Self {
            network_id: n.network_id.clone(),
            vpc_id: n.vpc_id.clone(),
            primary_cidr_block: n.primary_cidr_block.clone(),
            secondary_cidr_blocks: n.secondary_cidr_blocks.clone(),
            state: n.state.clone(),
            network_border_group: n.network_border_group.clone(),
            tags: n.tags.clone(),
        }
    }
}

impl From<SecondaryNetworkView> for SecondaryNetwork {
    fn from(v: SecondaryNetworkView) -> Self {
        Self {
            network_id: v.network_id,
            vpc_id: v.vpc_id,
            primary_cidr_block: v.primary_cidr_block,
            secondary_cidr_blocks: v.secondary_cidr_blocks,
            state: v.state,
            network_border_group: v.network_border_group,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondarySubnetView {
    pub subnet_id: String,
    pub vpc_id: String,
    pub secondary_network_id: String,
    pub cidr_block: String,
    pub availability_zone: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&SecondarySubnet> for SecondarySubnetView {
    fn from(s: &SecondarySubnet) -> Self {
        Self {
            subnet_id: s.subnet_id.clone(),
            vpc_id: s.vpc_id.clone(),
            secondary_network_id: s.secondary_network_id.clone(),
            cidr_block: s.cidr_block.clone(),
            availability_zone: s.availability_zone.clone(),
            state: s.state.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<SecondarySubnetView> for SecondarySubnet {
    fn from(v: SecondarySubnetView) -> Self {
        Self {
            subnet_id: v.subnet_id,
            vpc_id: v.vpc_id,
            secondary_network_id: v.secondary_network_id,
            cidr_block: v.cidr_block,
            availability_zone: v.availability_zone,
            state: v.state,
            tags: v.tags,
        }
    }
}

// ---------------------------------------------------------------------------
// Group 5: View structs + From impls
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedInstancesExchangeView {
    pub exchange_id: String,
    #[serde(default)]
    pub target_reserved_instances_ids: Vec<String>,
    #[serde(default)]
    pub source_reserved_instances_ids: Vec<String>,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    pub time: String,
}

impl From<&crate::types::ReservedInstancesExchange> for ReservedInstancesExchangeView {
    fn from(t: &crate::types::ReservedInstancesExchange) -> Self {
        Self {
            exchange_id: t.exchange_id.clone(),
            target_reserved_instances_ids: t.target_reserved_instances_ids.clone(),
            source_reserved_instances_ids: t.source_reserved_instances_ids.clone(),
            status: t.status.clone(),
            status_message: t.status_message.clone(),
            time: t.time.clone(),
        }
    }
}

impl From<ReservedInstancesExchangeView> for crate::types::ReservedInstancesExchange {
    fn from(v: ReservedInstancesExchangeView) -> Self {
        Self {
            exchange_id: v.exchange_id,
            target_reserved_instances_ids: v.target_reserved_instances_ids,
            source_reserved_instances_ids: v.source_reserved_instances_ids,
            status: v.status,
            status_message: v.status_message,
            time: v.time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceScheduleView {
    pub term: i64,
    pub price: f64,
    pub currency_code: String,
    pub active: bool,
}

impl From<&crate::types::PriceSchedule> for PriceScheduleView {
    fn from(p: &crate::types::PriceSchedule) -> Self {
        Self {
            term: p.term,
            price: p.price,
            currency_code: p.currency_code.clone(),
            active: p.active,
        }
    }
}

impl From<PriceScheduleView> for crate::types::PriceSchedule {
    fn from(v: PriceScheduleView) -> Self {
        Self {
            term: v.term,
            price: v.price,
            currency_code: v.currency_code,
            active: v.active,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedInstancesListingView {
    pub listing_id: String,
    pub reserved_instances_id: String,
    pub instance_count: i32,
    #[serde(default)]
    pub price_schedules: Vec<PriceScheduleView>,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    pub create_date: String,
    pub update_date: String,
    #[serde(default)]
    pub client_token: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&crate::types::ReservedInstancesListing> for ReservedInstancesListingView {
    fn from(t: &crate::types::ReservedInstancesListing) -> Self {
        Self {
            listing_id: t.listing_id.clone(),
            reserved_instances_id: t.reserved_instances_id.clone(),
            instance_count: t.instance_count,
            price_schedules: t
                .price_schedules
                .iter()
                .map(PriceScheduleView::from)
                .collect(),
            status: t.status.clone(),
            status_message: t.status_message.clone(),
            create_date: t.create_date.clone(),
            update_date: t.update_date.clone(),
            client_token: t.client_token.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<ReservedInstancesListingView> for crate::types::ReservedInstancesListing {
    fn from(v: ReservedInstancesListingView) -> Self {
        Self {
            listing_id: v.listing_id,
            reserved_instances_id: v.reserved_instances_id,
            instance_count: v.instance_count,
            price_schedules: v
                .price_schedules
                .into_iter()
                .map(crate::types::PriceSchedule::from)
                .collect(),
            status: v.status,
            status_message: v.status_message,
            create_date: v.create_date,
            update_date: v.update_date,
            client_token: v.client_token,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedInstancesPurchaseView {
    pub purchase_id: String,
    pub reserved_instances_offering_id: String,
    pub instance_count: i32,
    #[serde(default)]
    pub limit_price: Option<String>,
    pub purchase_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub queued: bool,
    #[serde(default)]
    pub reserved_instances_id: Option<String>,
}

impl From<&crate::types::ReservedInstancesPurchase> for ReservedInstancesPurchaseView {
    fn from(t: &crate::types::ReservedInstancesPurchase) -> Self {
        Self {
            purchase_id: t.purchase_id.clone(),
            reserved_instances_offering_id: t.reserved_instances_offering_id.clone(),
            instance_count: t.instance_count,
            limit_price: t.limit_price.clone(),
            purchase_time: t.purchase_time.clone(),
            tags: t.tags.clone(),
            queued: t.queued,
            reserved_instances_id: t.reserved_instances_id.clone(),
        }
    }
}

impl From<ReservedInstancesPurchaseView> for crate::types::ReservedInstancesPurchase {
    fn from(v: ReservedInstancesPurchaseView) -> Self {
        Self {
            purchase_id: v.purchase_id,
            reserved_instances_offering_id: v.reserved_instances_offering_id,
            instance_count: v.instance_count,
            limit_price: v.limit_price,
            purchase_time: v.purchase_time,
            tags: v.tags,
            queued: v.queued,
            reserved_instances_id: v.reserved_instances_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedInstancesRecordView {
    pub reserved_instances_id: String,
    pub instance_type: String,
    pub instance_count: i32,
    pub product_description: String,
    pub scope: String,
    pub currency_code: String,
    pub duration: i64,
    pub fixed_price: f64,
    pub usage_price: f64,
    pub offering_class: String,
    pub offering_type: String,
    pub instance_tenancy: String,
    pub start: String,
    pub end: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&crate::types::ReservedInstances> for ReservedInstancesRecordView {
    fn from(t: &crate::types::ReservedInstances) -> Self {
        Self {
            reserved_instances_id: t.reserved_instances_id.clone(),
            instance_type: t.instance_type.clone(),
            instance_count: t.instance_count,
            product_description: t.product_description.clone(),
            scope: t.scope.clone(),
            currency_code: t.currency_code.clone(),
            duration: t.duration,
            fixed_price: t.fixed_price,
            usage_price: t.usage_price,
            offering_class: t.offering_class.clone(),
            offering_type: t.offering_type.clone(),
            instance_tenancy: t.instance_tenancy.clone(),
            start: t.start.clone(),
            end: t.end.clone(),
            state: t.state.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<ReservedInstancesRecordView> for crate::types::ReservedInstances {
    fn from(v: ReservedInstancesRecordView) -> Self {
        Self {
            reserved_instances_id: v.reserved_instances_id,
            instance_type: v.instance_type,
            instance_count: v.instance_count,
            product_description: v.product_description,
            scope: v.scope,
            currency_code: v.currency_code,
            duration: v.duration,
            fixed_price: v.fixed_price,
            usage_price: v.usage_price,
            offering_class: v.offering_class,
            offering_type: v.offering_type,
            instance_tenancy: v.instance_tenancy,
            start: v.start,
            end: v.end,
            state: v.state,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedInstancesConfigurationView {
    #[serde(default)]
    pub availability_zone: Option<String>,
    #[serde(default)]
    pub instance_count: Option<i32>,
    #[serde(default)]
    pub instance_type: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub scope: Option<String>,
}

impl From<&crate::types::ReservedInstancesConfiguration> for ReservedInstancesConfigurationView {
    fn from(c: &crate::types::ReservedInstancesConfiguration) -> Self {
        Self {
            availability_zone: c.availability_zone.clone(),
            instance_count: c.instance_count,
            instance_type: c.instance_type.clone(),
            platform: c.platform.clone(),
            scope: c.scope.clone(),
        }
    }
}

impl From<ReservedInstancesConfigurationView> for crate::types::ReservedInstancesConfiguration {
    fn from(v: ReservedInstancesConfigurationView) -> Self {
        Self {
            availability_zone: v.availability_zone,
            instance_count: v.instance_count,
            instance_type: v.instance_type,
            platform: v.platform,
            scope: v.scope,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedInstancesModificationView {
    pub modification_id: String,
    #[serde(default)]
    pub reserved_instances_ids: Vec<String>,
    #[serde(default)]
    pub target_configurations: Vec<ReservedInstancesConfigurationView>,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    pub create_date: String,
    pub update_date: String,
    pub effective_date: String,
    #[serde(default)]
    pub client_token: Option<String>,
}

impl From<&crate::types::ReservedInstancesModification> for ReservedInstancesModificationView {
    fn from(m: &crate::types::ReservedInstancesModification) -> Self {
        Self {
            modification_id: m.modification_id.clone(),
            reserved_instances_ids: m.reserved_instances_ids.clone(),
            target_configurations: m
                .target_configurations
                .iter()
                .map(ReservedInstancesConfigurationView::from)
                .collect(),
            status: m.status.clone(),
            status_message: m.status_message.clone(),
            create_date: m.create_date.clone(),
            update_date: m.update_date.clone(),
            effective_date: m.effective_date.clone(),
            client_token: m.client_token.clone(),
        }
    }
}

impl From<ReservedInstancesModificationView> for crate::types::ReservedInstancesModification {
    fn from(v: ReservedInstancesModificationView) -> Self {
        Self {
            modification_id: v.modification_id,
            reserved_instances_ids: v.reserved_instances_ids,
            target_configurations: v
                .target_configurations
                .into_iter()
                .map(crate::types::ReservedInstancesConfiguration::from)
                .collect(),
            status: v.status,
            status_message: v.status_message,
            create_date: v.create_date,
            update_date: v.update_date,
            effective_date: v.effective_date,
            client_token: v.client_token,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpgaImageView {
    pub fpga_image_id: String,
    pub fpga_image_global_id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub shell_version: Option<String>,
    #[serde(default)]
    pub pci_id_vendor: Option<String>,
    #[serde(default)]
    pub pci_id_device: Option<String>,
    pub state: String,
    pub create_time: String,
    pub update_time: String,
    pub owner_id: String,
    #[serde(default)]
    pub owner_alias: Option<String>,
    #[serde(default)]
    pub product_codes: Vec<(String, String)>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub public: bool,
    pub data_retention_support: bool,
    #[serde(default)]
    pub instance_types: Vec<String>,
    #[serde(default)]
    pub load_permissions: Vec<(String, String)>,
}

impl From<&crate::types::FpgaImage> for FpgaImageView {
    fn from(t: &crate::types::FpgaImage) -> Self {
        Self {
            fpga_image_id: t.fpga_image_id.clone(),
            fpga_image_global_id: t.fpga_image_global_id.clone(),
            name: t.name.clone(),
            description: t.description.clone(),
            shell_version: t.shell_version.clone(),
            pci_id_vendor: t.pci_id_vendor.clone(),
            pci_id_device: t.pci_id_device.clone(),
            state: t.state.clone(),
            create_time: t.create_time.clone(),
            update_time: t.update_time.clone(),
            owner_id: t.owner_id.clone(),
            owner_alias: t.owner_alias.clone(),
            product_codes: t.product_codes.clone(),
            tags: t.tags.clone(),
            public: t.public,
            data_retention_support: t.data_retention_support,
            instance_types: t.instance_types.clone(),
            load_permissions: t.load_permissions.clone(),
        }
    }
}

impl From<FpgaImageView> for crate::types::FpgaImage {
    fn from(v: FpgaImageView) -> Self {
        Self {
            fpga_image_id: v.fpga_image_id,
            fpga_image_global_id: v.fpga_image_global_id,
            name: v.name,
            description: v.description,
            shell_version: v.shell_version,
            pci_id_vendor: v.pci_id_vendor,
            pci_id_device: v.pci_id_device,
            state: v.state,
            create_time: v.create_time,
            update_time: v.update_time,
            owner_id: v.owner_id,
            owner_alias: v.owner_alias,
            product_codes: v.product_codes,
            tags: v.tags,
            public: v.public,
            data_retention_support: v.data_retention_support,
            instance_types: v.instance_types,
            load_permissions: v.load_permissions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUsageReportView {
    pub report_id: String,
    pub image_id: String,
    #[serde(default)]
    pub account_filters: Vec<String>,
    #[serde(default)]
    pub resource_types: Vec<String>,
    pub status: String,
    pub creation_time: String,
    #[serde(default)]
    pub completion_time: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&crate::types::ImageUsageReport> for ImageUsageReportView {
    fn from(t: &crate::types::ImageUsageReport) -> Self {
        Self {
            report_id: t.report_id.clone(),
            image_id: t.image_id.clone(),
            account_filters: t.account_filters.clone(),
            resource_types: t.resource_types.clone(),
            status: t.status.clone(),
            creation_time: t.creation_time.clone(),
            completion_time: t.completion_time.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<ImageUsageReportView> for crate::types::ImageUsageReport {
    fn from(v: ImageUsageReportView) -> Self {
        Self {
            report_id: v.report_id,
            image_id: v.image_id,
            account_filters: v.account_filters,
            resource_types: v.resource_types,
            status: v.status,
            creation_time: v.creation_time,
            completion_time: v.completion_time,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreImageTaskView {
    pub image_id: String,
    pub name: String,
    pub s3_object_url: String,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    pub creation_time: String,
}

impl From<&crate::types::RestoreImageTask> for RestoreImageTaskView {
    fn from(t: &crate::types::RestoreImageTask) -> Self {
        Self {
            image_id: t.image_id.clone(),
            name: t.name.clone(),
            s3_object_url: t.s3_object_url.clone(),
            status: t.status.clone(),
            status_message: t.status_message.clone(),
            creation_time: t.creation_time.clone(),
        }
    }
}

impl From<RestoreImageTaskView> for crate::types::RestoreImageTask {
    fn from(v: RestoreImageTaskView) -> Self {
        Self {
            image_id: v.image_id,
            name: v.name,
            s3_object_url: v.s3_object_url,
            status: v.status,
            status_message: v.status_message,
            creation_time: v.creation_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreImageTaskView {
    pub image_id: String,
    pub ami_id: String,
    pub bucket: String,
    pub s3_object_key: String,
    pub store_task_state: String,
    #[serde(default)]
    pub store_task_failure_reason: Option<String>,
    pub progress_percentage: i32,
    pub task_start_time: String,
}

impl From<&crate::types::StoreImageTask> for StoreImageTaskView {
    fn from(t: &crate::types::StoreImageTask) -> Self {
        Self {
            image_id: t.image_id.clone(),
            ami_id: t.ami_id.clone(),
            bucket: t.bucket.clone(),
            s3_object_key: t.s3_object_key.clone(),
            store_task_state: t.store_task_state.clone(),
            store_task_failure_reason: t.store_task_failure_reason.clone(),
            progress_percentage: t.progress_percentage,
            task_start_time: t.task_start_time.clone(),
        }
    }
}

impl From<StoreImageTaskView> for crate::types::StoreImageTask {
    fn from(v: StoreImageTaskView) -> Self {
        Self {
            image_id: v.image_id,
            ami_id: v.ami_id,
            bucket: v.bucket,
            s3_object_key: v.s3_object_key,
            store_task_state: v.store_task_state,
            store_task_failure_reason: v.store_task_failure_reason,
            progress_percentage: v.progress_percentage,
            task_start_time: v.task_start_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportImageSnapshotDetailView {
    #[serde(default)]
    pub disk_image_size: Option<f64>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub progress: Option<String>,
    #[serde(default)]
    pub snapshot_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub user_bucket_s3_bucket: Option<String>,
    #[serde(default)]
    pub user_bucket_s3_key: Option<String>,
}

impl From<&crate::types::ImportImageSnapshotDetail> for ImportImageSnapshotDetailView {
    fn from(s: &crate::types::ImportImageSnapshotDetail) -> Self {
        Self {
            disk_image_size: s.disk_image_size,
            format: s.format.clone(),
            progress: s.progress.clone(),
            snapshot_id: s.snapshot_id.clone(),
            status: s.status.clone(),
            url: s.url.clone(),
            user_bucket_s3_bucket: s.user_bucket_s3_bucket.clone(),
            user_bucket_s3_key: s.user_bucket_s3_key.clone(),
        }
    }
}

impl From<ImportImageSnapshotDetailView> for crate::types::ImportImageSnapshotDetail {
    fn from(v: ImportImageSnapshotDetailView) -> Self {
        Self {
            disk_image_size: v.disk_image_size,
            format: v.format,
            progress: v.progress,
            snapshot_id: v.snapshot_id,
            status: v.status,
            url: v.url,
            user_bucket_s3_bucket: v.user_bucket_s3_bucket,
            user_bucket_s3_key: v.user_bucket_s3_key,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportImageTaskView {
    pub import_task_id: String,
    #[serde(default)]
    pub architecture: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub encrypted: bool,
    #[serde(default)]
    pub hypervisor: Option<String>,
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub license_type: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub progress: Option<String>,
    #[serde(default)]
    pub snapshot_details: Vec<ImportImageSnapshotDetailView>,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub usage_operation: Option<String>,
    #[serde(default)]
    pub boot_mode: Option<String>,
}

impl From<&crate::types::ImportImageTask> for ImportImageTaskView {
    fn from(t: &crate::types::ImportImageTask) -> Self {
        Self {
            import_task_id: t.import_task_id.clone(),
            architecture: t.architecture.clone(),
            description: t.description.clone(),
            encrypted: t.encrypted,
            hypervisor: t.hypervisor.clone(),
            image_id: t.image_id.clone(),
            license_type: t.license_type.clone(),
            platform: t.platform.clone(),
            progress: t.progress.clone(),
            snapshot_details: t
                .snapshot_details
                .iter()
                .map(ImportImageSnapshotDetailView::from)
                .collect(),
            status: t.status.clone(),
            status_message: t.status_message.clone(),
            tags: t.tags.clone(),
            usage_operation: t.usage_operation.clone(),
            boot_mode: t.boot_mode.clone(),
        }
    }
}

impl From<ImportImageTaskView> for crate::types::ImportImageTask {
    fn from(v: ImportImageTaskView) -> Self {
        Self {
            import_task_id: v.import_task_id,
            architecture: v.architecture,
            description: v.description,
            encrypted: v.encrypted,
            hypervisor: v.hypervisor,
            image_id: v.image_id,
            license_type: v.license_type,
            platform: v.platform,
            progress: v.progress,
            snapshot_details: v
                .snapshot_details
                .into_iter()
                .map(crate::types::ImportImageSnapshotDetail::from)
                .collect(),
            status: v.status,
            status_message: v.status_message,
            tags: v.tags,
            usage_operation: v.usage_operation,
            boot_mode: v.boot_mode,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllowedImageCriterionView {
    #[serde(default)]
    pub image_providers: Vec<String>,
    #[serde(default)]
    pub marketplace_product_codes: Vec<String>,
    #[serde(default)]
    pub deprecation_time_condition: Option<String>,
}

impl From<&crate::types::AllowedImageCriterion> for AllowedImageCriterionView {
    fn from(c: &crate::types::AllowedImageCriterion) -> Self {
        Self {
            image_providers: c.image_providers.clone(),
            marketplace_product_codes: c.marketplace_product_codes.clone(),
            deprecation_time_condition: c.deprecation_time_condition.clone(),
        }
    }
}

impl From<AllowedImageCriterionView> for crate::types::AllowedImageCriterion {
    fn from(v: AllowedImageCriterionView) -> Self {
        Self {
            image_providers: v.image_providers,
            marketplace_product_codes: v.marketplace_product_codes,
            deprecation_time_condition: v.deprecation_time_condition,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceEventWindowTimeRangeView {
    #[serde(default)]
    pub start_week_day: Option<String>,
    #[serde(default)]
    pub start_hour: Option<i32>,
    #[serde(default)]
    pub end_week_day: Option<String>,
    #[serde(default)]
    pub end_hour: Option<i32>,
}

impl From<&crate::types::InstanceEventWindowTimeRange> for InstanceEventWindowTimeRangeView {
    fn from(t: &crate::types::InstanceEventWindowTimeRange) -> Self {
        Self {
            start_week_day: t.start_week_day.clone(),
            start_hour: t.start_hour,
            end_week_day: t.end_week_day.clone(),
            end_hour: t.end_hour,
        }
    }
}

impl From<InstanceEventWindowTimeRangeView> for crate::types::InstanceEventWindowTimeRange {
    fn from(v: InstanceEventWindowTimeRangeView) -> Self {
        Self {
            start_week_day: v.start_week_day,
            start_hour: v.start_hour,
            end_week_day: v.end_week_day,
            end_hour: v.end_hour,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceEventWindowAssociationView {
    #[serde(default)]
    pub instance_ids: Vec<String>,
    #[serde(default)]
    pub dedicated_host_ids: Vec<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

impl From<&crate::types::InstanceEventWindowAssociation> for InstanceEventWindowAssociationView {
    fn from(t: &crate::types::InstanceEventWindowAssociation) -> Self {
        Self {
            instance_ids: t.instance_ids.clone(),
            dedicated_host_ids: t.dedicated_host_ids.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<InstanceEventWindowAssociationView> for crate::types::InstanceEventWindowAssociation {
    fn from(v: InstanceEventWindowAssociationView) -> Self {
        Self {
            instance_ids: v.instance_ids,
            dedicated_host_ids: v.dedicated_host_ids,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceEventWindowView {
    pub instance_event_window_id: String,
    pub name: String,
    #[serde(default)]
    pub time_ranges: Vec<InstanceEventWindowTimeRangeView>,
    #[serde(default)]
    pub cron_expression: Option<String>,
    #[serde(default)]
    pub association_target: Option<InstanceEventWindowAssociationView>,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&crate::types::InstanceEventWindow> for InstanceEventWindowView {
    fn from(t: &crate::types::InstanceEventWindow) -> Self {
        Self {
            instance_event_window_id: t.instance_event_window_id.clone(),
            name: t.name.clone(),
            time_ranges: t
                .time_ranges
                .iter()
                .map(InstanceEventWindowTimeRangeView::from)
                .collect(),
            cron_expression: t.cron_expression.clone(),
            association_target: t
                .association_target
                .as_ref()
                .map(InstanceEventWindowAssociationView::from),
            state: t.state.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<InstanceEventWindowView> for crate::types::InstanceEventWindow {
    fn from(v: InstanceEventWindowView) -> Self {
        Self {
            instance_event_window_id: v.instance_event_window_id,
            name: v.name,
            time_ranges: v
                .time_ranges
                .into_iter()
                .map(crate::types::InstanceEventWindowTimeRange::from)
                .collect(),
            cron_expression: v.cron_expression,
            association_target: v
                .association_target
                .map(crate::types::InstanceEventWindowAssociation::from),
            state: v.state,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceEventView {
    pub event_id: String,
    pub instance_id: String,
    pub code: String,
    pub description: String,
    pub not_before: String,
    pub not_after: String,
    #[serde(default)]
    pub not_before_deadline: Option<String>,
}

impl From<&crate::types::InstanceEvent> for InstanceEventView {
    fn from(t: &crate::types::InstanceEvent) -> Self {
        Self {
            event_id: t.event_id.clone(),
            instance_id: t.instance_id.clone(),
            code: t.code.clone(),
            description: t.description.clone(),
            not_before: t.not_before.clone(),
            not_after: t.not_after.clone(),
            not_before_deadline: t.not_before_deadline.clone(),
        }
    }
}

impl From<InstanceEventView> for crate::types::InstanceEvent {
    fn from(v: InstanceEventView) -> Self {
        Self {
            event_id: v.event_id,
            instance_id: v.instance_id,
            code: v.code,
            description: v.description,
            not_before: v.not_before,
            not_after: v.not_after,
            not_before_deadline: v.not_before_deadline,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstanceTagNotificationAttributesView {
    #[serde(default)]
    pub include_all_tags_of_instance: bool,
    #[serde(default)]
    pub instance_tag_keys: Vec<String>,
}

impl From<&crate::types::InstanceTagNotificationAttributes>
    for InstanceTagNotificationAttributesView
{
    fn from(t: &crate::types::InstanceTagNotificationAttributes) -> Self {
        Self {
            include_all_tags_of_instance: t.include_all_tags_of_instance,
            instance_tag_keys: t.instance_tag_keys.clone(),
        }
    }
}

impl From<InstanceTagNotificationAttributesView>
    for crate::types::InstanceTagNotificationAttributes
{
    fn from(v: InstanceTagNotificationAttributesView) -> Self {
        Self {
            include_all_tags_of_instance: v.include_all_tags_of_instance,
            instance_tag_keys: v.instance_tag_keys,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostReservationView {
    pub host_reservation_id: String,
    #[serde(default)]
    pub host_id_set: Vec<String>,
    pub currency_code: String,
    pub duration: i32,
    #[serde(default)]
    pub end: Option<String>,
    pub hourly_price: String,
    pub instance_family: String,
    pub offering_id: String,
    pub payment_option: String,
    pub start: String,
    pub state: String,
    pub upfront_price: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&crate::types::HostReservation> for HostReservationView {
    fn from(t: &crate::types::HostReservation) -> Self {
        Self {
            host_reservation_id: t.host_reservation_id.clone(),
            host_id_set: t.host_id_set.clone(),
            currency_code: t.currency_code.clone(),
            duration: t.duration,
            end: t.end.clone(),
            hourly_price: t.hourly_price.clone(),
            instance_family: t.instance_family.clone(),
            offering_id: t.offering_id.clone(),
            payment_option: t.payment_option.clone(),
            start: t.start.clone(),
            state: t.state.clone(),
            upfront_price: t.upfront_price.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<HostReservationView> for crate::types::HostReservation {
    fn from(v: HostReservationView) -> Self {
        Self {
            host_reservation_id: v.host_reservation_id,
            host_id_set: v.host_id_set,
            currency_code: v.currency_code,
            duration: v.duration,
            end: v.end,
            hourly_price: v.hourly_price,
            instance_family: v.instance_family,
            offering_id: v.offering_id,
            payment_option: v.payment_option,
            start: v.start,
            state: v.state,
            upfront_price: v.upfront_price,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScheduledInstanceRecurrenceView {
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub interval: Option<i32>,
    #[serde(default)]
    pub occurrence_day_set: Vec<i32>,
    #[serde(default)]
    pub occurrence_relative_to_end: Option<bool>,
    #[serde(default)]
    pub occurrence_unit: Option<String>,
}

impl From<&crate::types::ScheduledInstanceRecurrence> for ScheduledInstanceRecurrenceView {
    fn from(t: &crate::types::ScheduledInstanceRecurrence) -> Self {
        Self {
            frequency: t.frequency.clone(),
            interval: t.interval,
            occurrence_day_set: t.occurrence_day_set.clone(),
            occurrence_relative_to_end: t.occurrence_relative_to_end,
            occurrence_unit: t.occurrence_unit.clone(),
        }
    }
}

impl From<ScheduledInstanceRecurrenceView> for crate::types::ScheduledInstanceRecurrence {
    fn from(v: ScheduledInstanceRecurrenceView) -> Self {
        Self {
            frequency: v.frequency,
            interval: v.interval,
            occurrence_day_set: v.occurrence_day_set,
            occurrence_relative_to_end: v.occurrence_relative_to_end,
            occurrence_unit: v.occurrence_unit,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledInstanceView {
    pub scheduled_instance_id: String,
    pub instance_type: String,
    pub platform: String,
    pub network_platform: String,
    pub availability_zone: String,
    pub instance_count: i32,
    pub hourly_price: String,
    pub total_scheduled_instance_hours: i32,
    pub term_start_date: String,
    pub term_end_date: String,
    pub recurrence: ScheduledInstanceRecurrenceView,
    pub slot_duration_in_hours: i32,
    #[serde(default)]
    pub previous_slot_end_time: Option<String>,
    #[serde(default)]
    pub next_slot_start_time: Option<String>,
    pub create_date: String,
}

impl From<&crate::types::ScheduledInstance> for ScheduledInstanceView {
    fn from(t: &crate::types::ScheduledInstance) -> Self {
        Self {
            scheduled_instance_id: t.scheduled_instance_id.clone(),
            instance_type: t.instance_type.clone(),
            platform: t.platform.clone(),
            network_platform: t.network_platform.clone(),
            availability_zone: t.availability_zone.clone(),
            instance_count: t.instance_count,
            hourly_price: t.hourly_price.clone(),
            total_scheduled_instance_hours: t.total_scheduled_instance_hours,
            term_start_date: t.term_start_date.clone(),
            term_end_date: t.term_end_date.clone(),
            recurrence: ScheduledInstanceRecurrenceView::from(&t.recurrence),
            slot_duration_in_hours: t.slot_duration_in_hours,
            previous_slot_end_time: t.previous_slot_end_time.clone(),
            next_slot_start_time: t.next_slot_start_time.clone(),
            create_date: t.create_date.clone(),
        }
    }
}

impl From<ScheduledInstanceView> for crate::types::ScheduledInstance {
    fn from(v: ScheduledInstanceView) -> Self {
        Self {
            scheduled_instance_id: v.scheduled_instance_id,
            instance_type: v.instance_type,
            platform: v.platform,
            network_platform: v.network_platform,
            availability_zone: v.availability_zone,
            instance_count: v.instance_count,
            hourly_price: v.hourly_price,
            total_scheduled_instance_hours: v.total_scheduled_instance_hours,
            term_start_date: v.term_start_date,
            term_end_date: v.term_end_date,
            recurrence: crate::types::ScheduledInstanceRecurrence::from(v.recurrence),
            slot_duration_in_hours: v.slot_duration_in_hours,
            previous_slot_end_time: v.previous_slot_end_time,
            next_slot_start_time: v.next_slot_start_time,
            create_date: v.create_date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStatusReportView {
    pub instance_id: String,
    pub status: String,
    #[serde(default)]
    pub reason_codes: Vec<String>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl From<&crate::types::InstanceStatusReport> for InstanceStatusReportView {
    fn from(t: &crate::types::InstanceStatusReport) -> Self {
        Self {
            instance_id: t.instance_id.clone(),
            status: t.status.clone(),
            reason_codes: t.reason_codes.clone(),
            start_time: t.start_time.clone(),
            end_time: t.end_time.clone(),
            description: t.description.clone(),
        }
    }
}

impl From<InstanceStatusReportView> for crate::types::InstanceStatusReport {
    fn from(v: InstanceStatusReportView) -> Self {
        Self {
            instance_id: v.instance_id,
            status: v.status,
            reason_codes: v.reason_codes,
            start_time: v.start_time,
            end_time: v.end_time,
            description: v.description,
        }
    }
}

impl From<&crate::types::InstanceMetadataDefaults> for InstanceMetadataDefaultsView {
    fn from(t: &crate::types::InstanceMetadataDefaults) -> Self {
        Self {
            http_tokens: t.http_tokens.clone(),
            http_put_response_hop_limit: t.http_put_response_hop_limit,
            http_endpoint: t.http_endpoint.clone(),
            instance_metadata_tags: t.instance_metadata_tags.clone(),
        }
    }
}

impl From<InstanceMetadataDefaultsView> for crate::types::InstanceMetadataDefaults {
    fn from(v: InstanceMetadataDefaultsView) -> Self {
        Self {
            http_tokens: v.http_tokens,
            http_put_response_hop_limit: v.http_put_response_hop_limit,
            http_endpoint: v.http_endpoint,
            instance_metadata_tags: v.instance_metadata_tags,
        }
    }
}

// ---------------------------------------------------------------------------
// Group 6: Network Insights & Traffic Mirror From impls
// ---------------------------------------------------------------------------

impl From<&PacketHeaderStatementSpec> for PacketHeaderStatementSpecView {
    fn from(s: &PacketHeaderStatementSpec) -> Self {
        Self {
            destination_addresses: s.destination_addresses.clone(),
            destination_ports: s.destination_ports.clone(),
            destination_prefix_lists: s.destination_prefix_lists.clone(),
            protocols: s.protocols.clone(),
            source_addresses: s.source_addresses.clone(),
            source_ports: s.source_ports.clone(),
            source_prefix_lists: s.source_prefix_lists.clone(),
        }
    }
}

impl From<PacketHeaderStatementSpecView> for PacketHeaderStatementSpec {
    fn from(v: PacketHeaderStatementSpecView) -> Self {
        Self {
            destination_addresses: v.destination_addresses,
            destination_ports: v.destination_ports,
            destination_prefix_lists: v.destination_prefix_lists,
            protocols: v.protocols,
            source_addresses: v.source_addresses,
            source_ports: v.source_ports,
            source_prefix_lists: v.source_prefix_lists,
        }
    }
}

impl From<&ResourceStatementSpec> for ResourceStatementSpecView {
    fn from(s: &ResourceStatementSpec) -> Self {
        Self {
            resource_types: s.resource_types.clone(),
            resources: s.resources.clone(),
        }
    }
}

impl From<ResourceStatementSpecView> for ResourceStatementSpec {
    fn from(v: ResourceStatementSpecView) -> Self {
        Self {
            resource_types: v.resource_types,
            resources: v.resources,
        }
    }
}

impl From<&PathStatementSpec> for PathStatementSpecView {
    fn from(s: &PathStatementSpec) -> Self {
        Self {
            packet_header_statement: s
                .packet_header_statement
                .as_ref()
                .map(PacketHeaderStatementSpecView::from),
            resource_statement: s
                .resource_statement
                .as_ref()
                .map(ResourceStatementSpecView::from),
        }
    }
}

impl From<PathStatementSpecView> for PathStatementSpec {
    fn from(v: PathStatementSpecView) -> Self {
        Self {
            packet_header_statement: v
                .packet_header_statement
                .map(PacketHeaderStatementSpec::from),
            resource_statement: v.resource_statement.map(ResourceStatementSpec::from),
        }
    }
}

impl From<&AccessScopePathSpec> for AccessScopePathSpecView {
    fn from(s: &AccessScopePathSpec) -> Self {
        Self {
            source: s.source.as_ref().map(PathStatementSpecView::from),
            destination: s.destination.as_ref().map(PathStatementSpecView::from),
            through_resources: s
                .through_resources
                .iter()
                .map(ResourceStatementSpecView::from)
                .collect(),
        }
    }
}

impl From<AccessScopePathSpecView> for AccessScopePathSpec {
    fn from(v: AccessScopePathSpecView) -> Self {
        Self {
            source: v.source.map(PathStatementSpec::from),
            destination: v.destination.map(PathStatementSpec::from),
            through_resources: v
                .through_resources
                .into_iter()
                .map(ResourceStatementSpec::from)
                .collect(),
        }
    }
}

impl From<&NetworkInsightsAccessScope> for NetworkInsightsAccessScopeView {
    fn from(s: &NetworkInsightsAccessScope) -> Self {
        Self {
            network_insights_access_scope_id: s.network_insights_access_scope_id.clone(),
            network_insights_access_scope_arn: s.network_insights_access_scope_arn.clone(),
            created_date: s.created_date.clone(),
            updated_date: s.updated_date.clone(),
            tags: s.tags.clone(),
            match_paths: s
                .match_paths
                .iter()
                .map(AccessScopePathSpecView::from)
                .collect(),
            exclude_paths: s
                .exclude_paths
                .iter()
                .map(AccessScopePathSpecView::from)
                .collect(),
        }
    }
}

impl From<NetworkInsightsAccessScopeView> for NetworkInsightsAccessScope {
    fn from(v: NetworkInsightsAccessScopeView) -> Self {
        Self {
            network_insights_access_scope_id: v.network_insights_access_scope_id,
            network_insights_access_scope_arn: v.network_insights_access_scope_arn,
            created_date: v.created_date,
            updated_date: v.updated_date,
            tags: v.tags,
            match_paths: v
                .match_paths
                .into_iter()
                .map(AccessScopePathSpec::from)
                .collect(),
            exclude_paths: v
                .exclude_paths
                .into_iter()
                .map(AccessScopePathSpec::from)
                .collect(),
        }
    }
}

impl From<&NetworkInsightsAccessScopeAnalysis> for NetworkInsightsAccessScopeAnalysisView {
    fn from(s: &NetworkInsightsAccessScopeAnalysis) -> Self {
        Self {
            network_insights_access_scope_analysis_id: s
                .network_insights_access_scope_analysis_id
                .clone(),
            network_insights_access_scope_analysis_arn: s
                .network_insights_access_scope_analysis_arn
                .clone(),
            network_insights_access_scope_id: s.network_insights_access_scope_id.clone(),
            status: s.status.clone(),
            status_message: s.status_message.clone(),
            warning_message: s.warning_message.clone(),
            start_date: s.start_date.clone(),
            end_date: s.end_date.clone(),
            findings_found: s.findings_found.clone(),
            analyzed_eni_count: s.analyzed_eni_count,
            tags: s.tags.clone(),
        }
    }
}

impl From<NetworkInsightsAccessScopeAnalysisView> for NetworkInsightsAccessScopeAnalysis {
    fn from(v: NetworkInsightsAccessScopeAnalysisView) -> Self {
        Self {
            network_insights_access_scope_analysis_id: v.network_insights_access_scope_analysis_id,
            network_insights_access_scope_analysis_arn: v
                .network_insights_access_scope_analysis_arn,
            network_insights_access_scope_id: v.network_insights_access_scope_id,
            status: v.status,
            status_message: v.status_message,
            warning_message: v.warning_message,
            start_date: v.start_date,
            end_date: v.end_date,
            findings_found: v.findings_found,
            analyzed_eni_count: v.analyzed_eni_count,
            tags: v.tags,
        }
    }
}

impl From<&NetworkInsightsPathFilterPortRange> for NetworkInsightsPathFilterPortRangeView {
    fn from(p: &NetworkInsightsPathFilterPortRange) -> Self {
        Self {
            from_port: p.from_port,
            to_port: p.to_port,
        }
    }
}

impl From<NetworkInsightsPathFilterPortRangeView> for NetworkInsightsPathFilterPortRange {
    fn from(v: NetworkInsightsPathFilterPortRangeView) -> Self {
        Self {
            from_port: v.from_port,
            to_port: v.to_port,
        }
    }
}

impl From<&NetworkInsightsPathFilter> for NetworkInsightsPathFilterView {
    fn from(p: &NetworkInsightsPathFilter) -> Self {
        Self {
            destination_address: p.destination_address.clone(),
            destination_port_range: p
                .destination_port_range
                .as_ref()
                .map(NetworkInsightsPathFilterPortRangeView::from),
            source_address: p.source_address.clone(),
            source_port_range: p
                .source_port_range
                .as_ref()
                .map(NetworkInsightsPathFilterPortRangeView::from),
        }
    }
}

impl From<NetworkInsightsPathFilterView> for NetworkInsightsPathFilter {
    fn from(v: NetworkInsightsPathFilterView) -> Self {
        Self {
            destination_address: v.destination_address,
            destination_port_range: v
                .destination_port_range
                .map(NetworkInsightsPathFilterPortRange::from),
            source_address: v.source_address,
            source_port_range: v
                .source_port_range
                .map(NetworkInsightsPathFilterPortRange::from),
        }
    }
}

impl From<&NetworkInsightsPath> for NetworkInsightsPathView {
    fn from(s: &NetworkInsightsPath) -> Self {
        Self {
            network_insights_path_id: s.network_insights_path_id.clone(),
            network_insights_path_arn: s.network_insights_path_arn.clone(),
            created_date: s.created_date.clone(),
            source: s.source.clone(),
            destination: s.destination.clone(),
            source_arn: s.source_arn.clone(),
            destination_arn: s.destination_arn.clone(),
            source_ip: s.source_ip.clone(),
            destination_ip: s.destination_ip.clone(),
            protocol: s.protocol.clone(),
            destination_port: s.destination_port,
            tags: s.tags.clone(),
            filter_at_source: NetworkInsightsPathFilterView::from(&s.filter_at_source),
            filter_at_destination: NetworkInsightsPathFilterView::from(&s.filter_at_destination),
        }
    }
}

impl From<NetworkInsightsPathView> for NetworkInsightsPath {
    fn from(v: NetworkInsightsPathView) -> Self {
        Self {
            network_insights_path_id: v.network_insights_path_id,
            network_insights_path_arn: v.network_insights_path_arn,
            created_date: v.created_date,
            source: v.source,
            destination: v.destination,
            source_arn: v.source_arn,
            destination_arn: v.destination_arn,
            source_ip: v.source_ip,
            destination_ip: v.destination_ip,
            protocol: v.protocol,
            destination_port: v.destination_port,
            tags: v.tags,
            filter_at_source: NetworkInsightsPathFilter::from(v.filter_at_source),
            filter_at_destination: NetworkInsightsPathFilter::from(v.filter_at_destination),
        }
    }
}

impl From<&NetworkInsightsAnalysis> for NetworkInsightsAnalysisView {
    fn from(s: &NetworkInsightsAnalysis) -> Self {
        Self {
            network_insights_analysis_id: s.network_insights_analysis_id.clone(),
            network_insights_analysis_arn: s.network_insights_analysis_arn.clone(),
            network_insights_path_id: s.network_insights_path_id.clone(),
            additional_accounts: s.additional_accounts.clone(),
            filter_in_arns: s.filter_in_arns.clone(),
            start_date: s.start_date.clone(),
            end_date: s.end_date.clone(),
            status: s.status.clone(),
            status_message: s.status_message.clone(),
            warning_message: s.warning_message.clone(),
            network_path_found: s.network_path_found,
            tags: s.tags.clone(),
        }
    }
}

impl From<NetworkInsightsAnalysisView> for NetworkInsightsAnalysis {
    fn from(v: NetworkInsightsAnalysisView) -> Self {
        Self {
            network_insights_analysis_id: v.network_insights_analysis_id,
            network_insights_analysis_arn: v.network_insights_analysis_arn,
            network_insights_path_id: v.network_insights_path_id,
            additional_accounts: v.additional_accounts,
            filter_in_arns: v.filter_in_arns,
            start_date: v.start_date,
            end_date: v.end_date,
            status: v.status,
            status_message: v.status_message,
            warning_message: v.warning_message,
            network_path_found: v.network_path_found,
            tags: v.tags,
        }
    }
}

impl From<&TrafficMirrorPortRange> for TrafficMirrorPortRangeView {
    fn from(p: &TrafficMirrorPortRange) -> Self {
        Self {
            from_port: p.from_port,
            to_port: p.to_port,
        }
    }
}

impl From<TrafficMirrorPortRangeView> for TrafficMirrorPortRange {
    fn from(v: TrafficMirrorPortRangeView) -> Self {
        Self {
            from_port: v.from_port,
            to_port: v.to_port,
        }
    }
}

impl From<&TrafficMirrorFilterRule> for TrafficMirrorFilterRuleView {
    fn from(r: &TrafficMirrorFilterRule) -> Self {
        Self {
            traffic_mirror_filter_rule_id: r.traffic_mirror_filter_rule_id.clone(),
            traffic_mirror_filter_id: r.traffic_mirror_filter_id.clone(),
            traffic_direction: r.traffic_direction.clone(),
            rule_number: r.rule_number,
            rule_action: r.rule_action.clone(),
            protocol: r.protocol,
            destination_port_range: r
                .destination_port_range
                .as_ref()
                .map(TrafficMirrorPortRangeView::from),
            source_port_range: r
                .source_port_range
                .as_ref()
                .map(TrafficMirrorPortRangeView::from),
            destination_cidr_block: r.destination_cidr_block.clone(),
            source_cidr_block: r.source_cidr_block.clone(),
            description: r.description.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<TrafficMirrorFilterRuleView> for TrafficMirrorFilterRule {
    fn from(v: TrafficMirrorFilterRuleView) -> Self {
        Self {
            traffic_mirror_filter_rule_id: v.traffic_mirror_filter_rule_id,
            traffic_mirror_filter_id: v.traffic_mirror_filter_id,
            traffic_direction: v.traffic_direction,
            rule_number: v.rule_number,
            rule_action: v.rule_action,
            protocol: v.protocol,
            destination_port_range: v.destination_port_range.map(TrafficMirrorPortRange::from),
            source_port_range: v.source_port_range.map(TrafficMirrorPortRange::from),
            destination_cidr_block: v.destination_cidr_block,
            source_cidr_block: v.source_cidr_block,
            description: v.description,
            tags: v.tags,
        }
    }
}

impl From<&TrafficMirrorFilter> for TrafficMirrorFilterView {
    fn from(f: &TrafficMirrorFilter) -> Self {
        Self {
            traffic_mirror_filter_id: f.traffic_mirror_filter_id.clone(),
            description: f.description.clone(),
            ingress_filter_rules: f
                .ingress_filter_rules
                .iter()
                .map(TrafficMirrorFilterRuleView::from)
                .collect(),
            egress_filter_rules: f
                .egress_filter_rules
                .iter()
                .map(TrafficMirrorFilterRuleView::from)
                .collect(),
            network_services: f.network_services.clone(),
            tags: f.tags.clone(),
        }
    }
}

impl From<TrafficMirrorFilterView> for TrafficMirrorFilter {
    fn from(v: TrafficMirrorFilterView) -> Self {
        Self {
            traffic_mirror_filter_id: v.traffic_mirror_filter_id,
            description: v.description,
            ingress_filter_rules: v
                .ingress_filter_rules
                .into_iter()
                .map(TrafficMirrorFilterRule::from)
                .collect(),
            egress_filter_rules: v
                .egress_filter_rules
                .into_iter()
                .map(TrafficMirrorFilterRule::from)
                .collect(),
            network_services: v.network_services,
            tags: v.tags,
        }
    }
}

impl From<&TrafficMirrorSession> for TrafficMirrorSessionView {
    fn from(s: &TrafficMirrorSession) -> Self {
        Self {
            traffic_mirror_session_id: s.traffic_mirror_session_id.clone(),
            traffic_mirror_target_id: s.traffic_mirror_target_id.clone(),
            traffic_mirror_filter_id: s.traffic_mirror_filter_id.clone(),
            network_interface_id: s.network_interface_id.clone(),
            owner_id: s.owner_id.clone(),
            packet_length: s.packet_length,
            session_number: s.session_number,
            virtual_network_id: s.virtual_network_id,
            description: s.description.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<TrafficMirrorSessionView> for TrafficMirrorSession {
    fn from(v: TrafficMirrorSessionView) -> Self {
        Self {
            traffic_mirror_session_id: v.traffic_mirror_session_id,
            traffic_mirror_target_id: v.traffic_mirror_target_id,
            traffic_mirror_filter_id: v.traffic_mirror_filter_id,
            network_interface_id: v.network_interface_id,
            owner_id: v.owner_id,
            packet_length: v.packet_length,
            session_number: v.session_number,
            virtual_network_id: v.virtual_network_id,
            description: v.description,
            tags: v.tags,
        }
    }
}

impl From<&TrafficMirrorTarget> for TrafficMirrorTargetView {
    fn from(t: &TrafficMirrorTarget) -> Self {
        Self {
            traffic_mirror_target_id: t.traffic_mirror_target_id.clone(),
            network_interface_id: t.network_interface_id.clone(),
            network_load_balancer_arn: t.network_load_balancer_arn.clone(),
            gateway_load_balancer_endpoint_id: t.gateway_load_balancer_endpoint_id.clone(),
            r#type: t.r#type.clone(),
            description: t.description.clone(),
            owner_id: t.owner_id.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<TrafficMirrorTargetView> for TrafficMirrorTarget {
    fn from(v: TrafficMirrorTargetView) -> Self {
        Self {
            traffic_mirror_target_id: v.traffic_mirror_target_id,
            network_interface_id: v.network_interface_id,
            network_load_balancer_arn: v.network_load_balancer_arn,
            gateway_load_balancer_endpoint_id: v.gateway_load_balancer_endpoint_id,
            r#type: v.r#type,
            description: v.description,
            owner_id: v.owner_id,
            tags: v.tags,
        }
    }
}

// ===== Group 7 conversions =====

impl From<&ClientVpnEndpoint> for ClientVpnEndpointView {
    fn from(e: &ClientVpnEndpoint) -> Self {
        ClientVpnEndpointView {
            client_vpn_endpoint_id: e.client_vpn_endpoint_id.clone(),
            description: e.description.clone(),
            status: ClientVpnEndpointStatusView {
                code: e.status.code.clone(),
                message: e.status.message.clone(),
            },
            creation_time: e.creation_time.clone(),
            deletion_time: e.deletion_time.clone(),
            dns_name: e.dns_name.clone(),
            client_cidr_block: e.client_cidr_block.clone(),
            dns_servers: e.dns_servers.clone(),
            split_tunnel: e.split_tunnel,
            vpn_protocol: e.vpn_protocol.clone(),
            transport_protocol: e.transport_protocol.clone(),
            vpn_port: e.vpn_port,
            server_certificate_arn: e.server_certificate_arn.clone(),
            authentication_options: e.authentication_options.clone(),
            connection_log_options_enabled: e.connection_log_options_enabled,
            connection_log_options_cloudwatch_log_group: e
                .connection_log_options_cloudwatch_log_group
                .clone(),
            connection_log_options_cloudwatch_log_stream: e
                .connection_log_options_cloudwatch_log_stream
                .clone(),
            tags: e.tags.clone(),
            security_group_ids: e.security_group_ids.clone(),
            vpc_id: e.vpc_id.clone(),
            self_service_portal_url: e.self_service_portal_url.clone(),
            self_service_portal: e.self_service_portal.clone(),
            session_timeout_hours: e.session_timeout_hours,
            client_login_banner_enabled: e.client_login_banner_enabled,
            client_login_banner_text: e.client_login_banner_text.clone(),
            disconnect_on_session_timeout: e.disconnect_on_session_timeout,
            client_route_enforcement_enforced: e.client_route_enforcement_enforced,
            client_certificate_revocation_list: e.client_certificate_revocation_list.clone(),
        }
    }
}

impl From<ClientVpnEndpointView> for ClientVpnEndpoint {
    fn from(v: ClientVpnEndpointView) -> Self {
        ClientVpnEndpoint {
            client_vpn_endpoint_id: v.client_vpn_endpoint_id,
            description: v.description,
            status: ClientVpnEndpointStatus {
                code: v.status.code,
                message: v.status.message,
            },
            creation_time: v.creation_time,
            deletion_time: v.deletion_time,
            dns_name: v.dns_name,
            client_cidr_block: v.client_cidr_block,
            dns_servers: v.dns_servers,
            split_tunnel: v.split_tunnel,
            vpn_protocol: v.vpn_protocol,
            transport_protocol: v.transport_protocol,
            vpn_port: v.vpn_port,
            server_certificate_arn: v.server_certificate_arn,
            authentication_options: v.authentication_options,
            connection_log_options_enabled: v.connection_log_options_enabled,
            connection_log_options_cloudwatch_log_group: v
                .connection_log_options_cloudwatch_log_group,
            connection_log_options_cloudwatch_log_stream: v
                .connection_log_options_cloudwatch_log_stream,
            tags: v.tags,
            security_group_ids: v.security_group_ids,
            vpc_id: v.vpc_id,
            self_service_portal_url: v.self_service_portal_url,
            self_service_portal: v.self_service_portal,
            session_timeout_hours: v.session_timeout_hours,
            client_login_banner_enabled: v.client_login_banner_enabled,
            client_login_banner_text: v.client_login_banner_text,
            disconnect_on_session_timeout: v.disconnect_on_session_timeout,
            client_route_enforcement_enforced: v.client_route_enforcement_enforced,
            client_certificate_revocation_list: v.client_certificate_revocation_list,
        }
    }
}

impl From<&ClientVpnTargetNetworkAssociation> for ClientVpnTargetNetworkAssociationView {
    fn from(a: &ClientVpnTargetNetworkAssociation) -> Self {
        ClientVpnTargetNetworkAssociationView {
            association_id: a.association_id.clone(),
            vpc_id: a.vpc_id.clone(),
            target_network_id: a.target_network_id.clone(),
            client_vpn_endpoint_id: a.client_vpn_endpoint_id.clone(),
            security_groups: a.security_groups.clone(),
            status: ClientVpnAssociationStatusView {
                code: a.status.code.clone(),
                message: a.status.message.clone(),
            },
        }
    }
}

impl From<ClientVpnTargetNetworkAssociationView> for ClientVpnTargetNetworkAssociation {
    fn from(v: ClientVpnTargetNetworkAssociationView) -> Self {
        ClientVpnTargetNetworkAssociation {
            association_id: v.association_id,
            vpc_id: v.vpc_id,
            target_network_id: v.target_network_id,
            client_vpn_endpoint_id: v.client_vpn_endpoint_id,
            security_groups: v.security_groups,
            status: ClientVpnAssociationStatus {
                code: v.status.code,
                message: v.status.message,
            },
        }
    }
}

impl From<&ClientVpnAuthorizationRule> for ClientVpnAuthorizationRuleView {
    fn from(r: &ClientVpnAuthorizationRule) -> Self {
        ClientVpnAuthorizationRuleView {
            client_vpn_endpoint_id: r.client_vpn_endpoint_id.clone(),
            group_id: r.group_id.clone(),
            access_all: r.access_all,
            destination_cidr: r.destination_cidr.clone(),
            description: r.description.clone(),
            status: ClientVpnAuthorizationRuleStatusView {
                code: r.status.code.clone(),
                message: r.status.message.clone(),
            },
        }
    }
}

impl From<ClientVpnAuthorizationRuleView> for ClientVpnAuthorizationRule {
    fn from(v: ClientVpnAuthorizationRuleView) -> Self {
        ClientVpnAuthorizationRule {
            client_vpn_endpoint_id: v.client_vpn_endpoint_id,
            group_id: v.group_id,
            access_all: v.access_all,
            destination_cidr: v.destination_cidr,
            description: v.description,
            status: ClientVpnAuthorizationRuleStatus {
                code: v.status.code,
                message: v.status.message,
            },
        }
    }
}

impl From<&ClientVpnRoute> for ClientVpnRouteView {
    fn from(r: &ClientVpnRoute) -> Self {
        ClientVpnRouteView {
            client_vpn_endpoint_id: r.client_vpn_endpoint_id.clone(),
            destination_cidr: r.destination_cidr.clone(),
            target_subnet: r.target_subnet.clone(),
            r#type: r.r#type.clone(),
            origin: r.origin.clone(),
            status: ClientVpnRouteStatusView {
                code: r.status.code.clone(),
                message: r.status.message.clone(),
            },
            description: r.description.clone(),
        }
    }
}

impl From<ClientVpnRouteView> for ClientVpnRoute {
    fn from(v: ClientVpnRouteView) -> Self {
        ClientVpnRoute {
            client_vpn_endpoint_id: v.client_vpn_endpoint_id,
            destination_cidr: v.destination_cidr,
            target_subnet: v.target_subnet,
            r#type: v.r#type,
            origin: v.origin,
            status: ClientVpnRouteStatus {
                code: v.status.code,
                message: v.status.message,
            },
            description: v.description,
        }
    }
}

impl From<&ClientVpnConnection> for ClientVpnConnectionView {
    fn from(c: &ClientVpnConnection) -> Self {
        ClientVpnConnectionView {
            connection_id: c.connection_id.clone(),
            client_vpn_endpoint_id: c.client_vpn_endpoint_id.clone(),
            username: c.username.clone(),
            status: ClientVpnConnectionStatusView {
                code: c.status.code.clone(),
                message: c.status.message.clone(),
            },
            posture_compliance_statuses: c.posture_compliance_statuses.clone(),
            common_name: c.common_name.clone(),
            connection_established_time: c.connection_established_time.clone(),
            connection_end_time: c.connection_end_time.clone(),
            ingress_bytes: c.ingress_bytes.clone(),
            egress_bytes: c.egress_bytes.clone(),
            ingress_packets: c.ingress_packets.clone(),
            egress_packets: c.egress_packets.clone(),
            client_ip: c.client_ip.clone(),
            client_port: c.client_port.clone(),
            timestamp: c.timestamp.clone(),
        }
    }
}

impl From<ClientVpnConnectionView> for ClientVpnConnection {
    fn from(v: ClientVpnConnectionView) -> Self {
        ClientVpnConnection {
            connection_id: v.connection_id,
            client_vpn_endpoint_id: v.client_vpn_endpoint_id,
            username: v.username,
            status: ClientVpnConnectionStatus {
                code: v.status.code,
                message: v.status.message,
            },
            posture_compliance_statuses: v.posture_compliance_statuses,
            common_name: v.common_name,
            connection_established_time: v.connection_established_time,
            connection_end_time: v.connection_end_time,
            ingress_bytes: v.ingress_bytes,
            egress_bytes: v.egress_bytes,
            ingress_packets: v.ingress_packets,
            egress_packets: v.egress_packets,
            client_ip: v.client_ip,
            client_port: v.client_port,
            timestamp: v.timestamp,
        }
    }
}

impl From<&LocalGateway> for LocalGatewayView {
    fn from(g: &LocalGateway) -> Self {
        LocalGatewayView {
            local_gateway_id: g.local_gateway_id.clone(),
            outpost_arn: g.outpost_arn.clone(),
            owner_id: g.owner_id.clone(),
            state: g.state.clone(),
            tags: g.tags.clone(),
        }
    }
}

impl From<LocalGatewayView> for LocalGateway {
    fn from(v: LocalGatewayView) -> Self {
        LocalGateway {
            local_gateway_id: v.local_gateway_id,
            outpost_arn: v.outpost_arn,
            owner_id: v.owner_id,
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<&LocalGatewayRoute> for LocalGatewayRouteView {
    fn from(r: &LocalGatewayRoute) -> Self {
        LocalGatewayRouteView {
            destination_cidr_block: r.destination_cidr_block.clone(),
            local_gateway_route_table_id: r.local_gateway_route_table_id.clone(),
            r#type: r.r#type.clone(),
            state: r.state.clone(),
            local_gateway_route_table_arn: r.local_gateway_route_table_arn.clone(),
            owner_id: r.owner_id.clone(),
            subnet_id: r.subnet_id.clone(),
            network_interface_id: r.network_interface_id.clone(),
            destination_prefix_list_id: r.destination_prefix_list_id.clone(),
            coip_pool_id: r.coip_pool_id.clone(),
            local_gateway_virtual_interface_group_id: r
                .local_gateway_virtual_interface_group_id
                .clone(),
        }
    }
}

impl From<LocalGatewayRouteView> for LocalGatewayRoute {
    fn from(v: LocalGatewayRouteView) -> Self {
        LocalGatewayRoute {
            destination_cidr_block: v.destination_cidr_block,
            local_gateway_route_table_id: v.local_gateway_route_table_id,
            r#type: v.r#type,
            state: v.state,
            local_gateway_route_table_arn: v.local_gateway_route_table_arn,
            owner_id: v.owner_id,
            subnet_id: v.subnet_id,
            network_interface_id: v.network_interface_id,
            destination_prefix_list_id: v.destination_prefix_list_id,
            coip_pool_id: v.coip_pool_id,
            local_gateway_virtual_interface_group_id: v.local_gateway_virtual_interface_group_id,
        }
    }
}

impl From<&LocalGatewayRouteTable> for LocalGatewayRouteTableView {
    fn from(t: &LocalGatewayRouteTable) -> Self {
        LocalGatewayRouteTableView {
            local_gateway_route_table_id: t.local_gateway_route_table_id.clone(),
            local_gateway_route_table_arn: t.local_gateway_route_table_arn.clone(),
            local_gateway_id: t.local_gateway_id.clone(),
            owner_id: t.owner_id.clone(),
            state: t.state.clone(),
            mode: t.mode.clone(),
            tags: t.tags.clone(),
            state_reason_code: t.state_reason_code.clone(),
            state_reason_message: t.state_reason_message.clone(),
        }
    }
}

impl From<LocalGatewayRouteTableView> for LocalGatewayRouteTable {
    fn from(v: LocalGatewayRouteTableView) -> Self {
        LocalGatewayRouteTable {
            local_gateway_route_table_id: v.local_gateway_route_table_id,
            local_gateway_route_table_arn: v.local_gateway_route_table_arn,
            local_gateway_id: v.local_gateway_id,
            owner_id: v.owner_id,
            state: v.state,
            mode: v.mode,
            tags: v.tags,
            state_reason_code: v.state_reason_code,
            state_reason_message: v.state_reason_message,
        }
    }
}

impl From<&LocalGatewayRouteTableVirtualInterfaceGroupAssociation>
    for LocalGatewayRouteTableVirtualInterfaceGroupAssociationView
{
    fn from(a: &LocalGatewayRouteTableVirtualInterfaceGroupAssociation) -> Self {
        LocalGatewayRouteTableVirtualInterfaceGroupAssociationView {
            local_gateway_route_table_virtual_interface_group_association_id: a
                .local_gateway_route_table_virtual_interface_group_association_id
                .clone(),
            local_gateway_virtual_interface_group_id: a
                .local_gateway_virtual_interface_group_id
                .clone(),
            local_gateway_route_table_id: a.local_gateway_route_table_id.clone(),
            local_gateway_route_table_arn: a.local_gateway_route_table_arn.clone(),
            local_gateway_id: a.local_gateway_id.clone(),
            owner_id: a.owner_id.clone(),
            state: a.state.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<LocalGatewayRouteTableVirtualInterfaceGroupAssociationView>
    for LocalGatewayRouteTableVirtualInterfaceGroupAssociation
{
    fn from(v: LocalGatewayRouteTableVirtualInterfaceGroupAssociationView) -> Self {
        LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
            local_gateway_route_table_virtual_interface_group_association_id: v
                .local_gateway_route_table_virtual_interface_group_association_id,
            local_gateway_virtual_interface_group_id: v.local_gateway_virtual_interface_group_id,
            local_gateway_route_table_id: v.local_gateway_route_table_id,
            local_gateway_route_table_arn: v.local_gateway_route_table_arn,
            local_gateway_id: v.local_gateway_id,
            owner_id: v.owner_id,
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<&LocalGatewayRouteTableVpcAssociation> for LocalGatewayRouteTableVpcAssociationView {
    fn from(a: &LocalGatewayRouteTableVpcAssociation) -> Self {
        LocalGatewayRouteTableVpcAssociationView {
            local_gateway_route_table_vpc_association_id: a
                .local_gateway_route_table_vpc_association_id
                .clone(),
            local_gateway_route_table_id: a.local_gateway_route_table_id.clone(),
            local_gateway_route_table_arn: a.local_gateway_route_table_arn.clone(),
            local_gateway_id: a.local_gateway_id.clone(),
            vpc_id: a.vpc_id.clone(),
            owner_id: a.owner_id.clone(),
            state: a.state.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<LocalGatewayRouteTableVpcAssociationView> for LocalGatewayRouteTableVpcAssociation {
    fn from(v: LocalGatewayRouteTableVpcAssociationView) -> Self {
        LocalGatewayRouteTableVpcAssociation {
            local_gateway_route_table_vpc_association_id: v
                .local_gateway_route_table_vpc_association_id,
            local_gateway_route_table_id: v.local_gateway_route_table_id,
            local_gateway_route_table_arn: v.local_gateway_route_table_arn,
            local_gateway_id: v.local_gateway_id,
            vpc_id: v.vpc_id,
            owner_id: v.owner_id,
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<&LocalGatewayVirtualInterface> for LocalGatewayVirtualInterfaceView {
    fn from(i: &LocalGatewayVirtualInterface) -> Self {
        LocalGatewayVirtualInterfaceView {
            local_gateway_virtual_interface_id: i.local_gateway_virtual_interface_id.clone(),
            local_gateway_id: i.local_gateway_id.clone(),
            vlan: i.vlan,
            local_address: i.local_address.clone(),
            peer_address: i.peer_address.clone(),
            local_bgp_asn: i.local_bgp_asn,
            peer_bgp_asn: i.peer_bgp_asn,
            owner_id: i.owner_id.clone(),
            tags: i.tags.clone(),
            configuration_state: i.configuration_state.clone(),
            peer_bgp_asn_extended: i.peer_bgp_asn_extended,
            local_gateway_virtual_interface_arn: i.local_gateway_virtual_interface_arn.clone(),
        }
    }
}

impl From<LocalGatewayVirtualInterfaceView> for LocalGatewayVirtualInterface {
    fn from(v: LocalGatewayVirtualInterfaceView) -> Self {
        LocalGatewayVirtualInterface {
            local_gateway_virtual_interface_id: v.local_gateway_virtual_interface_id,
            local_gateway_id: v.local_gateway_id,
            vlan: v.vlan,
            local_address: v.local_address,
            peer_address: v.peer_address,
            local_bgp_asn: v.local_bgp_asn,
            peer_bgp_asn: v.peer_bgp_asn,
            owner_id: v.owner_id,
            tags: v.tags,
            configuration_state: v.configuration_state,
            peer_bgp_asn_extended: v.peer_bgp_asn_extended,
            local_gateway_virtual_interface_arn: v.local_gateway_virtual_interface_arn,
        }
    }
}

impl From<&LocalGatewayVirtualInterfaceGroup> for LocalGatewayVirtualInterfaceGroupView {
    fn from(g: &LocalGatewayVirtualInterfaceGroup) -> Self {
        LocalGatewayVirtualInterfaceGroupView {
            local_gateway_virtual_interface_group_id: g
                .local_gateway_virtual_interface_group_id
                .clone(),
            local_gateway_virtual_interface_ids: g.local_gateway_virtual_interface_ids.clone(),
            local_gateway_id: g.local_gateway_id.clone(),
            owner_id: g.owner_id.clone(),
            tags: g.tags.clone(),
            configuration_state: g.configuration_state.clone(),
            local_bgp_asn: g.local_bgp_asn,
            local_bgp_asn_extended: g.local_bgp_asn_extended,
            local_gateway_virtual_interface_group_arn: g
                .local_gateway_virtual_interface_group_arn
                .clone(),
        }
    }
}

impl From<LocalGatewayVirtualInterfaceGroupView> for LocalGatewayVirtualInterfaceGroup {
    fn from(v: LocalGatewayVirtualInterfaceGroupView) -> Self {
        LocalGatewayVirtualInterfaceGroup {
            local_gateway_virtual_interface_group_id: v.local_gateway_virtual_interface_group_id,
            local_gateway_virtual_interface_ids: v.local_gateway_virtual_interface_ids,
            local_gateway_id: v.local_gateway_id,
            owner_id: v.owner_id,
            tags: v.tags,
            configuration_state: v.configuration_state,
            local_bgp_asn: v.local_bgp_asn,
            local_bgp_asn_extended: v.local_bgp_asn_extended,
            local_gateway_virtual_interface_group_arn: v.local_gateway_virtual_interface_group_arn,
        }
    }
}

// ===== Group 8: Route Server From impls =====

impl From<&RouteServer> for RouteServerView {
    fn from(r: &RouteServer) -> Self {
        RouteServerView {
            route_server_id: r.route_server_id.clone(),
            route_server_arn: r.route_server_arn.clone(),
            amazon_side_asn: r.amazon_side_asn,
            state: r.state.clone(),
            persist_routes: r.persist_routes.clone(),
            persist_routes_duration: r.persist_routes_duration,
            sns_notifications_enabled: r.sns_notifications_enabled,
            sns_topic_arn: r.sns_topic_arn.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<RouteServerView> for RouteServer {
    fn from(v: RouteServerView) -> Self {
        RouteServer {
            route_server_id: v.route_server_id,
            route_server_arn: v.route_server_arn,
            amazon_side_asn: v.amazon_side_asn,
            state: v.state,
            persist_routes: v.persist_routes,
            persist_routes_duration: v.persist_routes_duration,
            sns_notifications_enabled: v.sns_notifications_enabled,
            sns_topic_arn: v.sns_topic_arn,
            tags: v.tags,
        }
    }
}

impl From<&RouteServerEndpoint> for RouteServerEndpointView {
    fn from(e: &RouteServerEndpoint) -> Self {
        RouteServerEndpointView {
            route_server_endpoint_id: e.route_server_endpoint_id.clone(),
            route_server_id: e.route_server_id.clone(),
            vpc_id: e.vpc_id.clone(),
            subnet_id: e.subnet_id.clone(),
            eni_id: e.eni_id.clone(),
            eni_address: e.eni_address.clone(),
            state: e.state.clone(),
            failure_reason: e.failure_reason.clone(),
            tags: e.tags.clone(),
        }
    }
}

impl From<RouteServerEndpointView> for RouteServerEndpoint {
    fn from(v: RouteServerEndpointView) -> Self {
        RouteServerEndpoint {
            route_server_endpoint_id: v.route_server_endpoint_id,
            route_server_id: v.route_server_id,
            vpc_id: v.vpc_id,
            subnet_id: v.subnet_id,
            eni_id: v.eni_id,
            eni_address: v.eni_address,
            state: v.state,
            failure_reason: v.failure_reason,
            tags: v.tags,
        }
    }
}

impl From<&RouteServerBgpOptions> for RouteServerBgpOptionsView {
    fn from(o: &RouteServerBgpOptions) -> Self {
        RouteServerBgpOptionsView {
            peer_asn: o.peer_asn,
            peer_liveness_detection: o.peer_liveness_detection.clone(),
        }
    }
}

impl From<RouteServerBgpOptionsView> for RouteServerBgpOptions {
    fn from(v: RouteServerBgpOptionsView) -> Self {
        RouteServerBgpOptions {
            peer_asn: v.peer_asn,
            peer_liveness_detection: v.peer_liveness_detection,
        }
    }
}

impl From<&RouteServerPeerOptions> for RouteServerPeerOptionsView {
    fn from(o: &RouteServerPeerOptions) -> Self {
        RouteServerPeerOptionsView {
            peer_asn: o.peer_asn,
            peer_liveness_detection: o.peer_liveness_detection.clone(),
            bgp_options: o.bgp_options.as_ref().map(RouteServerBgpOptionsView::from),
        }
    }
}

impl From<RouteServerPeerOptionsView> for RouteServerPeerOptions {
    fn from(v: RouteServerPeerOptionsView) -> Self {
        RouteServerPeerOptions {
            peer_asn: v.peer_asn,
            peer_liveness_detection: v.peer_liveness_detection,
            bgp_options: v.bgp_options.map(RouteServerBgpOptions::from),
        }
    }
}

impl From<&RouteServerPeer> for RouteServerPeerView {
    fn from(p: &RouteServerPeer) -> Self {
        RouteServerPeerView {
            route_server_peer_id: p.route_server_peer_id.clone(),
            route_server_endpoint_id: p.route_server_endpoint_id.clone(),
            route_server_id: p.route_server_id.clone(),
            vpc_id: p.vpc_id.clone(),
            subnet_id: p.subnet_id.clone(),
            peer_address: p.peer_address.clone(),
            state: p.state.clone(),
            failure_reason: p.failure_reason.clone(),
            options: RouteServerPeerOptionsView::from(&p.options),
            endpoint_eni_id: p.endpoint_eni_id.clone(),
            endpoint_eni_address: p.endpoint_eni_address.clone(),
            tags: p.tags.clone(),
        }
    }
}

impl From<RouteServerPeerView> for RouteServerPeer {
    fn from(v: RouteServerPeerView) -> Self {
        RouteServerPeer {
            route_server_peer_id: v.route_server_peer_id,
            route_server_endpoint_id: v.route_server_endpoint_id,
            route_server_id: v.route_server_id,
            vpc_id: v.vpc_id,
            subnet_id: v.subnet_id,
            peer_address: v.peer_address,
            state: v.state,
            failure_reason: v.failure_reason,
            options: RouteServerPeerOptions::from(v.options),
            endpoint_eni_id: v.endpoint_eni_id,
            endpoint_eni_address: v.endpoint_eni_address,
            tags: v.tags,
        }
    }
}

impl From<&RouteServerAssociation> for RouteServerAssociationView {
    fn from(a: &RouteServerAssociation) -> Self {
        RouteServerAssociationView {
            route_server_id: a.route_server_id.clone(),
            vpc_id: a.vpc_id.clone(),
            state: a.state.clone(),
            propagations: a.propagations.clone(),
        }
    }
}

impl From<RouteServerAssociationView> for RouteServerAssociation {
    fn from(v: RouteServerAssociationView) -> Self {
        RouteServerAssociation {
            route_server_id: v.route_server_id,
            vpc_id: v.vpc_id,
            state: v.state,
            propagations: v.propagations,
        }
    }
}

// ===== Group 9: Verified Access view conversions =====

impl From<&VerifiedAccessInstance> for VerifiedAccessInstanceView {
    fn from(i: &VerifiedAccessInstance) -> Self {
        VerifiedAccessInstanceView {
            verified_access_instance_id: i.verified_access_instance_id.clone(),
            description: i.description.clone(),
            creation_time: i.creation_time.clone(),
            last_updated_time: i.last_updated_time.clone(),
            fips_enabled: i.fips_enabled,
            cidr_endpoints_custom_subdomain: i.cidr_endpoints_custom_subdomain.clone(),
            name: i.name.clone(),
            trust_provider_ids: i.trust_provider_ids.clone(),
            tags: i.tags.clone(),
        }
    }
}

impl From<VerifiedAccessInstanceView> for VerifiedAccessInstance {
    fn from(v: VerifiedAccessInstanceView) -> Self {
        VerifiedAccessInstance {
            verified_access_instance_id: v.verified_access_instance_id,
            description: v.description,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            fips_enabled: v.fips_enabled,
            cidr_endpoints_custom_subdomain: v.cidr_endpoints_custom_subdomain,
            name: v.name,
            trust_provider_ids: v.trust_provider_ids,
            tags: v.tags,
        }
    }
}

impl From<&VerifiedAccessOidcOptions> for VerifiedAccessOidcOptionsView {
    fn from(o: &VerifiedAccessOidcOptions) -> Self {
        VerifiedAccessOidcOptionsView {
            issuer: o.issuer.clone(),
            authorization_endpoint: o.authorization_endpoint.clone(),
            token_endpoint: o.token_endpoint.clone(),
            user_info_endpoint: o.user_info_endpoint.clone(),
            client_id: o.client_id.clone(),
            client_secret: o.client_secret.clone(),
            scope: o.scope.clone(),
        }
    }
}

impl From<VerifiedAccessOidcOptionsView> for VerifiedAccessOidcOptions {
    fn from(v: VerifiedAccessOidcOptionsView) -> Self {
        VerifiedAccessOidcOptions {
            issuer: v.issuer,
            authorization_endpoint: v.authorization_endpoint,
            token_endpoint: v.token_endpoint,
            user_info_endpoint: v.user_info_endpoint,
            client_id: v.client_id,
            client_secret: v.client_secret,
            scope: v.scope,
        }
    }
}

impl From<&VerifiedAccessDeviceOptions> for VerifiedAccessDeviceOptionsView {
    fn from(o: &VerifiedAccessDeviceOptions) -> Self {
        VerifiedAccessDeviceOptionsView {
            tenant_id: o.tenant_id.clone(),
            public_signing_key_url: o.public_signing_key_url.clone(),
        }
    }
}

impl From<VerifiedAccessDeviceOptionsView> for VerifiedAccessDeviceOptions {
    fn from(v: VerifiedAccessDeviceOptionsView) -> Self {
        VerifiedAccessDeviceOptions {
            tenant_id: v.tenant_id,
            public_signing_key_url: v.public_signing_key_url,
        }
    }
}

impl From<&VerifiedAccessNativeApplicationOidcOptions>
    for VerifiedAccessNativeApplicationOidcOptionsView
{
    fn from(o: &VerifiedAccessNativeApplicationOidcOptions) -> Self {
        VerifiedAccessNativeApplicationOidcOptionsView {
            public_signing_key_endpoint: o.public_signing_key_endpoint.clone(),
            issuer: o.issuer.clone(),
            authorization_endpoint: o.authorization_endpoint.clone(),
            token_endpoint: o.token_endpoint.clone(),
            user_info_endpoint: o.user_info_endpoint.clone(),
            client_id: o.client_id.clone(),
            client_secret: o.client_secret.clone(),
            scope: o.scope.clone(),
        }
    }
}

impl From<VerifiedAccessNativeApplicationOidcOptionsView>
    for VerifiedAccessNativeApplicationOidcOptions
{
    fn from(v: VerifiedAccessNativeApplicationOidcOptionsView) -> Self {
        VerifiedAccessNativeApplicationOidcOptions {
            public_signing_key_endpoint: v.public_signing_key_endpoint,
            issuer: v.issuer,
            authorization_endpoint: v.authorization_endpoint,
            token_endpoint: v.token_endpoint,
            user_info_endpoint: v.user_info_endpoint,
            client_id: v.client_id,
            client_secret: v.client_secret,
            scope: v.scope,
        }
    }
}

impl From<&VerifiedAccessSseSpecification> for VerifiedAccessSseSpecificationView {
    fn from(s: &VerifiedAccessSseSpecification) -> Self {
        VerifiedAccessSseSpecificationView {
            customer_managed_key_enabled: s.customer_managed_key_enabled,
            kms_key_arn: s.kms_key_arn.clone(),
        }
    }
}

impl From<VerifiedAccessSseSpecificationView> for VerifiedAccessSseSpecification {
    fn from(v: VerifiedAccessSseSpecificationView) -> Self {
        VerifiedAccessSseSpecification {
            customer_managed_key_enabled: v.customer_managed_key_enabled,
            kms_key_arn: v.kms_key_arn,
        }
    }
}

impl From<&VerifiedAccessTrustProvider> for VerifiedAccessTrustProviderView {
    fn from(t: &VerifiedAccessTrustProvider) -> Self {
        VerifiedAccessTrustProviderView {
            verified_access_trust_provider_id: t.verified_access_trust_provider_id.clone(),
            description: t.description.clone(),
            trust_provider_type: t.trust_provider_type.clone(),
            user_trust_provider_type: t.user_trust_provider_type.clone(),
            device_trust_provider_type: t.device_trust_provider_type.clone(),
            oidc_options: t
                .oidc_options
                .as_ref()
                .map(VerifiedAccessOidcOptionsView::from),
            device_options: t
                .device_options
                .as_ref()
                .map(VerifiedAccessDeviceOptionsView::from),
            native_application_oidc_options: t
                .native_application_oidc_options
                .as_ref()
                .map(VerifiedAccessNativeApplicationOidcOptionsView::from),
            policy_reference_name: t.policy_reference_name.clone(),
            creation_time: t.creation_time.clone(),
            last_updated_time: t.last_updated_time.clone(),
            sse_specification: VerifiedAccessSseSpecificationView::from(&t.sse_specification),
            tags: t.tags.clone(),
        }
    }
}

impl From<VerifiedAccessTrustProviderView> for VerifiedAccessTrustProvider {
    fn from(v: VerifiedAccessTrustProviderView) -> Self {
        VerifiedAccessTrustProvider {
            verified_access_trust_provider_id: v.verified_access_trust_provider_id,
            description: v.description,
            trust_provider_type: v.trust_provider_type,
            user_trust_provider_type: v.user_trust_provider_type,
            device_trust_provider_type: v.device_trust_provider_type,
            oidc_options: v.oidc_options.map(VerifiedAccessOidcOptions::from),
            device_options: v.device_options.map(VerifiedAccessDeviceOptions::from),
            native_application_oidc_options: v
                .native_application_oidc_options
                .map(VerifiedAccessNativeApplicationOidcOptions::from),
            policy_reference_name: v.policy_reference_name,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            sse_specification: VerifiedAccessSseSpecification::from(v.sse_specification),
            tags: v.tags,
        }
    }
}

impl From<&VerifiedAccessGroup> for VerifiedAccessGroupView {
    fn from(g: &VerifiedAccessGroup) -> Self {
        VerifiedAccessGroupView {
            verified_access_group_id: g.verified_access_group_id.clone(),
            verified_access_group_arn: g.verified_access_group_arn.clone(),
            verified_access_instance_id: g.verified_access_instance_id.clone(),
            owner: g.owner.clone(),
            description: g.description.clone(),
            creation_time: g.creation_time.clone(),
            last_updated_time: g.last_updated_time.clone(),
            deletion_time: g.deletion_time.clone(),
            sse_specification: VerifiedAccessSseSpecificationView::from(&g.sse_specification),
            policy_document: g.policy_document.clone(),
            policy_enabled: g.policy_enabled,
            tags: g.tags.clone(),
        }
    }
}

impl From<VerifiedAccessGroupView> for VerifiedAccessGroup {
    fn from(v: VerifiedAccessGroupView) -> Self {
        VerifiedAccessGroup {
            verified_access_group_id: v.verified_access_group_id,
            verified_access_group_arn: v.verified_access_group_arn,
            verified_access_instance_id: v.verified_access_instance_id,
            owner: v.owner,
            description: v.description,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            deletion_time: v.deletion_time,
            sse_specification: VerifiedAccessSseSpecification::from(v.sse_specification),
            policy_document: v.policy_document,
            policy_enabled: v.policy_enabled,
            tags: v.tags,
        }
    }
}

impl From<&VerifiedAccessEndpointPortRange> for VerifiedAccessEndpointPortRangeView {
    fn from(p: &VerifiedAccessEndpointPortRange) -> Self {
        VerifiedAccessEndpointPortRangeView {
            from_port: p.from_port,
            to_port: p.to_port,
        }
    }
}

impl From<VerifiedAccessEndpointPortRangeView> for VerifiedAccessEndpointPortRange {
    fn from(v: VerifiedAccessEndpointPortRangeView) -> Self {
        VerifiedAccessEndpointPortRange {
            from_port: v.from_port,
            to_port: v.to_port,
        }
    }
}

impl From<&VerifiedAccessEndpointLoadBalancerOptions>
    for VerifiedAccessEndpointLoadBalancerOptionsView
{
    fn from(o: &VerifiedAccessEndpointLoadBalancerOptions) -> Self {
        VerifiedAccessEndpointLoadBalancerOptionsView {
            load_balancer_arn: o.load_balancer_arn.clone(),
            port: o.port,
            port_ranges: o
                .port_ranges
                .iter()
                .map(VerifiedAccessEndpointPortRangeView::from)
                .collect(),
            protocol: o.protocol.clone(),
            subnet_ids: o.subnet_ids.clone(),
        }
    }
}

impl From<VerifiedAccessEndpointLoadBalancerOptionsView>
    for VerifiedAccessEndpointLoadBalancerOptions
{
    fn from(v: VerifiedAccessEndpointLoadBalancerOptionsView) -> Self {
        VerifiedAccessEndpointLoadBalancerOptions {
            load_balancer_arn: v.load_balancer_arn,
            port: v.port,
            port_ranges: v
                .port_ranges
                .into_iter()
                .map(VerifiedAccessEndpointPortRange::from)
                .collect(),
            protocol: v.protocol,
            subnet_ids: v.subnet_ids,
        }
    }
}

impl From<&VerifiedAccessEndpointEniOptions> for VerifiedAccessEndpointEniOptionsView {
    fn from(o: &VerifiedAccessEndpointEniOptions) -> Self {
        VerifiedAccessEndpointEniOptionsView {
            network_interface_id: o.network_interface_id.clone(),
            port: o.port,
            port_ranges: o
                .port_ranges
                .iter()
                .map(VerifiedAccessEndpointPortRangeView::from)
                .collect(),
            protocol: o.protocol.clone(),
        }
    }
}

impl From<VerifiedAccessEndpointEniOptionsView> for VerifiedAccessEndpointEniOptions {
    fn from(v: VerifiedAccessEndpointEniOptionsView) -> Self {
        VerifiedAccessEndpointEniOptions {
            network_interface_id: v.network_interface_id,
            port: v.port,
            port_ranges: v
                .port_ranges
                .into_iter()
                .map(VerifiedAccessEndpointPortRange::from)
                .collect(),
            protocol: v.protocol,
        }
    }
}

impl From<&VerifiedAccessEndpointCidrOptions> for VerifiedAccessEndpointCidrOptionsView {
    fn from(o: &VerifiedAccessEndpointCidrOptions) -> Self {
        VerifiedAccessEndpointCidrOptionsView {
            cidr: o.cidr.clone(),
            port_ranges: o
                .port_ranges
                .iter()
                .map(VerifiedAccessEndpointPortRangeView::from)
                .collect(),
            protocol: o.protocol.clone(),
            subnet_ids: o.subnet_ids.clone(),
        }
    }
}

impl From<VerifiedAccessEndpointCidrOptionsView> for VerifiedAccessEndpointCidrOptions {
    fn from(v: VerifiedAccessEndpointCidrOptionsView) -> Self {
        VerifiedAccessEndpointCidrOptions {
            cidr: v.cidr,
            port_ranges: v
                .port_ranges
                .into_iter()
                .map(VerifiedAccessEndpointPortRange::from)
                .collect(),
            protocol: v.protocol,
            subnet_ids: v.subnet_ids,
        }
    }
}

impl From<&VerifiedAccessEndpointRdsOptions> for VerifiedAccessEndpointRdsOptionsView {
    fn from(o: &VerifiedAccessEndpointRdsOptions) -> Self {
        VerifiedAccessEndpointRdsOptionsView {
            port: o.port,
            protocol: o.protocol.clone(),
            rds_db_cluster_arn: o.rds_db_cluster_arn.clone(),
            rds_db_instance_arn: o.rds_db_instance_arn.clone(),
            rds_db_proxy_arn: o.rds_db_proxy_arn.clone(),
            rds_endpoint: o.rds_endpoint.clone(),
            subnet_ids: o.subnet_ids.clone(),
        }
    }
}

impl From<VerifiedAccessEndpointRdsOptionsView> for VerifiedAccessEndpointRdsOptions {
    fn from(v: VerifiedAccessEndpointRdsOptionsView) -> Self {
        VerifiedAccessEndpointRdsOptions {
            port: v.port,
            protocol: v.protocol,
            rds_db_cluster_arn: v.rds_db_cluster_arn,
            rds_db_instance_arn: v.rds_db_instance_arn,
            rds_db_proxy_arn: v.rds_db_proxy_arn,
            rds_endpoint: v.rds_endpoint,
            subnet_ids: v.subnet_ids,
        }
    }
}

impl From<&VerifiedAccessEndpoint> for VerifiedAccessEndpointView {
    fn from(e: &VerifiedAccessEndpoint) -> Self {
        VerifiedAccessEndpointView {
            verified_access_endpoint_id: e.verified_access_endpoint_id.clone(),
            verified_access_instance_id: e.verified_access_instance_id.clone(),
            verified_access_group_id: e.verified_access_group_id.clone(),
            application_domain: e.application_domain.clone(),
            endpoint_type: e.endpoint_type.clone(),
            attachment_type: e.attachment_type.clone(),
            domain_certificate_arn: e.domain_certificate_arn.clone(),
            endpoint_domain: e.endpoint_domain.clone(),
            device_validation_domain: e.device_validation_domain.clone(),
            security_group_ids: e.security_group_ids.clone(),
            load_balancer_options: e
                .load_balancer_options
                .as_ref()
                .map(VerifiedAccessEndpointLoadBalancerOptionsView::from),
            network_interface_options: e
                .network_interface_options
                .as_ref()
                .map(VerifiedAccessEndpointEniOptionsView::from),
            cidr_options: e
                .cidr_options
                .as_ref()
                .map(VerifiedAccessEndpointCidrOptionsView::from),
            rds_options: e
                .rds_options
                .as_ref()
                .map(VerifiedAccessEndpointRdsOptionsView::from),
            status_code: e.status_code.clone(),
            status_message: e.status_message.clone(),
            description: e.description.clone(),
            creation_time: e.creation_time.clone(),
            last_updated_time: e.last_updated_time.clone(),
            deletion_time: e.deletion_time.clone(),
            sse_specification: VerifiedAccessSseSpecificationView::from(&e.sse_specification),
            policy_document: e.policy_document.clone(),
            policy_enabled: e.policy_enabled,
            tags: e.tags.clone(),
        }
    }
}

impl From<VerifiedAccessEndpointView> for VerifiedAccessEndpoint {
    fn from(v: VerifiedAccessEndpointView) -> Self {
        VerifiedAccessEndpoint {
            verified_access_endpoint_id: v.verified_access_endpoint_id,
            verified_access_instance_id: v.verified_access_instance_id,
            verified_access_group_id: v.verified_access_group_id,
            application_domain: v.application_domain,
            endpoint_type: v.endpoint_type,
            attachment_type: v.attachment_type,
            domain_certificate_arn: v.domain_certificate_arn,
            endpoint_domain: v.endpoint_domain,
            device_validation_domain: v.device_validation_domain,
            security_group_ids: v.security_group_ids,
            load_balancer_options: v
                .load_balancer_options
                .map(VerifiedAccessEndpointLoadBalancerOptions::from),
            network_interface_options: v
                .network_interface_options
                .map(VerifiedAccessEndpointEniOptions::from),
            cidr_options: v.cidr_options.map(VerifiedAccessEndpointCidrOptions::from),
            rds_options: v.rds_options.map(VerifiedAccessEndpointRdsOptions::from),
            status_code: v.status_code,
            status_message: v.status_message,
            description: v.description,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            deletion_time: v.deletion_time,
            sse_specification: VerifiedAccessSseSpecification::from(v.sse_specification),
            policy_document: v.policy_document,
            policy_enabled: v.policy_enabled,
            tags: v.tags,
        }
    }
}

impl From<&VerifiedAccessTrustProviderAttachment> for VerifiedAccessTrustProviderAttachmentView {
    fn from(a: &VerifiedAccessTrustProviderAttachment) -> Self {
        VerifiedAccessTrustProviderAttachmentView {
            instance_id: a.instance_id.clone(),
            trust_provider_id: a.trust_provider_id.clone(),
        }
    }
}

impl From<VerifiedAccessTrustProviderAttachmentView> for VerifiedAccessTrustProviderAttachment {
    fn from(v: VerifiedAccessTrustProviderAttachmentView) -> Self {
        VerifiedAccessTrustProviderAttachment {
            instance_id: v.instance_id,
            trust_provider_id: v.trust_provider_id,
        }
    }
}

impl From<&VerifiedAccessLogs> for VerifiedAccessLogsView {
    fn from(l: &VerifiedAccessLogs) -> Self {
        VerifiedAccessLogsView {
            cloud_watch_logs_enabled: l.cloud_watch_logs_enabled,
            cloud_watch_logs_log_group: l.cloud_watch_logs_log_group.clone(),
            kinesis_data_firehose_enabled: l.kinesis_data_firehose_enabled,
            kinesis_data_firehose_delivery_stream: l.kinesis_data_firehose_delivery_stream.clone(),
            s3_enabled: l.s3_enabled,
            s3_bucket_name: l.s3_bucket_name.clone(),
            s3_bucket_owner: l.s3_bucket_owner.clone(),
            s3_prefix: l.s3_prefix.clone(),
            log_version: l.log_version.clone(),
            include_trust_context: l.include_trust_context,
        }
    }
}

impl From<VerifiedAccessLogsView> for VerifiedAccessLogs {
    fn from(v: VerifiedAccessLogsView) -> Self {
        VerifiedAccessLogs {
            cloud_watch_logs_enabled: v.cloud_watch_logs_enabled,
            cloud_watch_logs_log_group: v.cloud_watch_logs_log_group,
            kinesis_data_firehose_enabled: v.kinesis_data_firehose_enabled,
            kinesis_data_firehose_delivery_stream: v.kinesis_data_firehose_delivery_stream,
            s3_enabled: v.s3_enabled,
            s3_bucket_name: v.s3_bucket_name,
            s3_bucket_owner: v.s3_bucket_owner,
            s3_prefix: v.s3_prefix,
            log_version: v.log_version,
            include_trust_context: v.include_trust_context,
        }
    }
}

// ---------------------------------------------------------------------------
// Group 12: IPAM view types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamOperatingRegionView {
    pub region_name: String,
}

impl From<&IpamOperatingRegion> for IpamOperatingRegionView {
    fn from(r: &IpamOperatingRegion) -> Self {
        Self {
            region_name: r.region_name.clone(),
        }
    }
}

impl From<IpamOperatingRegionView> for IpamOperatingRegion {
    fn from(v: IpamOperatingRegionView) -> Self {
        Self {
            region_name: v.region_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamView {
    pub ipam_id: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub public_default_scope_id: String,
    pub private_default_scope_id: String,
    pub scope_count: i32,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub operating_regions: Vec<IpamOperatingRegionView>,
    pub state: String,
    pub owner_id: String,
    #[serde(default)]
    pub default_resource_discovery_id: Option<String>,
    #[serde(default)]
    pub default_resource_discovery_association_id: Option<String>,
    pub resource_discovery_association_count: i32,
    pub tier: String,
    pub enable_private_gua: bool,
    pub metered_account: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&Ipam> for IpamView {
    fn from(i: &Ipam) -> Self {
        IpamView {
            ipam_id: i.ipam_id.clone(),
            ipam_arn: i.ipam_arn.clone(),
            ipam_region: i.ipam_region.clone(),
            public_default_scope_id: i.public_default_scope_id.clone(),
            private_default_scope_id: i.private_default_scope_id.clone(),
            scope_count: i.scope_count,
            description: i.description.clone(),
            operating_regions: i
                .operating_regions
                .iter()
                .map(IpamOperatingRegionView::from)
                .collect(),
            state: i.state.clone(),
            owner_id: i.owner_id.clone(),
            default_resource_discovery_id: i.default_resource_discovery_id.clone(),
            default_resource_discovery_association_id: i
                .default_resource_discovery_association_id
                .clone(),
            resource_discovery_association_count: i.resource_discovery_association_count,
            tier: i.tier.clone(),
            enable_private_gua: i.enable_private_gua,
            metered_account: i.metered_account.clone(),
            tags: i.tags.clone(),
        }
    }
}

impl From<IpamView> for Ipam {
    fn from(v: IpamView) -> Self {
        Ipam {
            ipam_id: v.ipam_id,
            ipam_arn: v.ipam_arn,
            ipam_region: v.ipam_region,
            public_default_scope_id: v.public_default_scope_id,
            private_default_scope_id: v.private_default_scope_id,
            scope_count: v.scope_count,
            description: v.description,
            operating_regions: v
                .operating_regions
                .into_iter()
                .map(IpamOperatingRegion::from)
                .collect(),
            state: v.state,
            owner_id: v.owner_id,
            default_resource_discovery_id: v.default_resource_discovery_id,
            default_resource_discovery_association_id: v.default_resource_discovery_association_id,
            resource_discovery_association_count: v.resource_discovery_association_count,
            tier: v.tier,
            enable_private_gua: v.enable_private_gua,
            metered_account: v.metered_account,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamScopeView {
    pub ipam_scope_id: String,
    pub ipam_scope_arn: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub ipam_scope_type: String,
    pub is_default: bool,
    #[serde(default)]
    pub description: Option<String>,
    pub pool_count: i32,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub owner_id: String,
}

impl From<&IpamScope> for IpamScopeView {
    fn from(s: &IpamScope) -> Self {
        IpamScopeView {
            ipam_scope_id: s.ipam_scope_id.clone(),
            ipam_scope_arn: s.ipam_scope_arn.clone(),
            ipam_arn: s.ipam_arn.clone(),
            ipam_region: s.ipam_region.clone(),
            ipam_scope_type: s.ipam_scope_type.clone(),
            is_default: s.is_default,
            description: s.description.clone(),
            pool_count: s.pool_count,
            state: s.state.clone(),
            tags: s.tags.clone(),
            owner_id: s.owner_id.clone(),
        }
    }
}

impl From<IpamScopeView> for IpamScope {
    fn from(v: IpamScopeView) -> Self {
        IpamScope {
            ipam_scope_id: v.ipam_scope_id,
            ipam_scope_arn: v.ipam_scope_arn,
            ipam_arn: v.ipam_arn,
            ipam_region: v.ipam_region,
            ipam_scope_type: v.ipam_scope_type,
            is_default: v.is_default,
            description: v.description,
            pool_count: v.pool_count,
            state: v.state,
            tags: v.tags,
            owner_id: v.owner_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPoolView {
    pub ipam_pool_id: String,
    #[serde(default)]
    pub source_ipam_pool_id: Option<String>,
    pub ipam_pool_arn: String,
    pub ipam_scope_arn: String,
    pub ipam_scope_type: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub locale: String,
    pub pool_depth: i32,
    pub state: String,
    #[serde(default)]
    pub state_message: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub auto_import: bool,
    pub publicly_advertisable: bool,
    pub address_family: String,
    #[serde(default)]
    pub allocation_min_netmask_length: Option<i32>,
    #[serde(default)]
    pub allocation_max_netmask_length: Option<i32>,
    #[serde(default)]
    pub allocation_default_netmask_length: Option<i32>,
    #[serde(default)]
    pub allocation_resource_tags: Vec<(String, String)>,
    #[serde(default)]
    pub aws_service: Option<String>,
    #[serde(default)]
    pub public_ip_source: Option<String>,
    #[serde(default)]
    pub source_resource_id: Option<String>,
    #[serde(default)]
    pub source_resource_type: Option<String>,
    #[serde(default)]
    pub source_resource_region: Option<String>,
    #[serde(default)]
    pub source_resource_owner: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub owner_id: String,
    pub allocation_count: i32,
}

impl From<&IpamPool> for IpamPoolView {
    fn from(p: &IpamPool) -> Self {
        IpamPoolView {
            ipam_pool_id: p.ipam_pool_id.clone(),
            source_ipam_pool_id: p.source_ipam_pool_id.clone(),
            ipam_pool_arn: p.ipam_pool_arn.clone(),
            ipam_scope_arn: p.ipam_scope_arn.clone(),
            ipam_scope_type: p.ipam_scope_type.clone(),
            ipam_arn: p.ipam_arn.clone(),
            ipam_region: p.ipam_region.clone(),
            locale: p.locale.clone(),
            pool_depth: p.pool_depth,
            state: p.state.clone(),
            state_message: p.state_message.clone(),
            description: p.description.clone(),
            auto_import: p.auto_import,
            publicly_advertisable: p.publicly_advertisable,
            address_family: p.address_family.clone(),
            allocation_min_netmask_length: p.allocation_min_netmask_length,
            allocation_max_netmask_length: p.allocation_max_netmask_length,
            allocation_default_netmask_length: p.allocation_default_netmask_length,
            allocation_resource_tags: p.allocation_resource_tags.clone(),
            aws_service: p.aws_service.clone(),
            public_ip_source: p.public_ip_source.clone(),
            source_resource_id: p.source_resource_id.clone(),
            source_resource_type: p.source_resource_type.clone(),
            source_resource_region: p.source_resource_region.clone(),
            source_resource_owner: p.source_resource_owner.clone(),
            tags: p.tags.clone(),
            owner_id: p.owner_id.clone(),
            allocation_count: p.allocation_count,
        }
    }
}

impl From<IpamPoolView> for IpamPool {
    fn from(v: IpamPoolView) -> Self {
        IpamPool {
            ipam_pool_id: v.ipam_pool_id,
            source_ipam_pool_id: v.source_ipam_pool_id,
            ipam_pool_arn: v.ipam_pool_arn,
            ipam_scope_arn: v.ipam_scope_arn,
            ipam_scope_type: v.ipam_scope_type,
            ipam_arn: v.ipam_arn,
            ipam_region: v.ipam_region,
            locale: v.locale,
            pool_depth: v.pool_depth,
            state: v.state,
            state_message: v.state_message,
            description: v.description,
            auto_import: v.auto_import,
            publicly_advertisable: v.publicly_advertisable,
            address_family: v.address_family,
            allocation_min_netmask_length: v.allocation_min_netmask_length,
            allocation_max_netmask_length: v.allocation_max_netmask_length,
            allocation_default_netmask_length: v.allocation_default_netmask_length,
            allocation_resource_tags: v.allocation_resource_tags,
            aws_service: v.aws_service,
            public_ip_source: v.public_ip_source,
            source_resource_id: v.source_resource_id,
            source_resource_type: v.source_resource_type,
            source_resource_region: v.source_resource_region,
            source_resource_owner: v.source_resource_owner,
            tags: v.tags,
            owner_id: v.owner_id,
            allocation_count: v.allocation_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPoolCidrView {
    pub ipam_pool_id: String,
    pub cidr: String,
    pub state: String,
    #[serde(default)]
    pub failure_reason: Option<String>,
    pub ipam_pool_cidr_id: String,
    #[serde(default)]
    pub netmask_length: Option<i32>,
}

impl IpamPoolCidrView {
    pub fn from_kv(key: &(String, String), c: &IpamPoolCidr) -> Self {
        IpamPoolCidrView {
            ipam_pool_id: key.0.clone(),
            cidr: c.cidr.clone(),
            state: c.state.clone(),
            failure_reason: c.failure_reason.clone(),
            ipam_pool_cidr_id: c.ipam_pool_cidr_id.clone(),
            netmask_length: c.netmask_length,
        }
    }
}

impl From<IpamPoolCidrView> for IpamPoolCidr {
    fn from(v: IpamPoolCidrView) -> Self {
        IpamPoolCidr {
            cidr: v.cidr,
            state: v.state,
            failure_reason: v.failure_reason,
            ipam_pool_cidr_id: v.ipam_pool_cidr_id,
            netmask_length: v.netmask_length,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPoolAllocationView {
    pub ipam_pool_allocation_id: String,
    pub cidr: String,
    pub ipam_pool_id: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    pub resource_type: String,
    #[serde(default)]
    pub resource_region: Option<String>,
    #[serde(default)]
    pub resource_owner: Option<String>,
}

impl From<&IpamPoolAllocation> for IpamPoolAllocationView {
    fn from(a: &IpamPoolAllocation) -> Self {
        IpamPoolAllocationView {
            ipam_pool_allocation_id: a.ipam_pool_allocation_id.clone(),
            cidr: a.cidr.clone(),
            ipam_pool_id: a.ipam_pool_id.clone(),
            description: a.description.clone(),
            resource_id: a.resource_id.clone(),
            resource_type: a.resource_type.clone(),
            resource_region: a.resource_region.clone(),
            resource_owner: a.resource_owner.clone(),
        }
    }
}

impl From<IpamPoolAllocationView> for IpamPoolAllocation {
    fn from(v: IpamPoolAllocationView) -> Self {
        IpamPoolAllocation {
            ipam_pool_allocation_id: v.ipam_pool_allocation_id,
            cidr: v.cidr,
            ipam_pool_id: v.ipam_pool_id,
            description: v.description,
            resource_id: v.resource_id,
            resource_type: v.resource_type,
            resource_region: v.resource_region,
            resource_owner: v.resource_owner,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamResourceDiscoveryView {
    pub ipam_resource_discovery_id: String,
    pub ipam_resource_discovery_arn: String,
    pub ipam_resource_discovery_region: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub operating_regions: Vec<IpamOperatingRegionView>,
    pub is_default: bool,
    pub state: String,
    pub owner_id: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&IpamResourceDiscovery> for IpamResourceDiscoveryView {
    fn from(r: &IpamResourceDiscovery) -> Self {
        IpamResourceDiscoveryView {
            ipam_resource_discovery_id: r.ipam_resource_discovery_id.clone(),
            ipam_resource_discovery_arn: r.ipam_resource_discovery_arn.clone(),
            ipam_resource_discovery_region: r.ipam_resource_discovery_region.clone(),
            description: r.description.clone(),
            operating_regions: r
                .operating_regions
                .iter()
                .map(IpamOperatingRegionView::from)
                .collect(),
            is_default: r.is_default,
            state: r.state.clone(),
            owner_id: r.owner_id.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<IpamResourceDiscoveryView> for IpamResourceDiscovery {
    fn from(v: IpamResourceDiscoveryView) -> Self {
        IpamResourceDiscovery {
            ipam_resource_discovery_id: v.ipam_resource_discovery_id,
            ipam_resource_discovery_arn: v.ipam_resource_discovery_arn,
            ipam_resource_discovery_region: v.ipam_resource_discovery_region,
            description: v.description,
            operating_regions: v
                .operating_regions
                .into_iter()
                .map(IpamOperatingRegion::from)
                .collect(),
            is_default: v.is_default,
            state: v.state,
            owner_id: v.owner_id,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamResourceDiscoveryAssociationView {
    pub ipam_resource_discovery_association_id: String,
    pub ipam_resource_discovery_association_arn: String,
    pub ipam_arn: String,
    pub ipam_id: String,
    pub ipam_region: String,
    pub ipam_resource_discovery_id: String,
    pub owner_id: String,
    pub is_default: bool,
    pub resource_discovery_status: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&IpamResourceDiscoveryAssociation> for IpamResourceDiscoveryAssociationView {
    fn from(a: &IpamResourceDiscoveryAssociation) -> Self {
        IpamResourceDiscoveryAssociationView {
            ipam_resource_discovery_association_id: a
                .ipam_resource_discovery_association_id
                .clone(),
            ipam_resource_discovery_association_arn: a
                .ipam_resource_discovery_association_arn
                .clone(),
            ipam_arn: a.ipam_arn.clone(),
            ipam_id: a.ipam_id.clone(),
            ipam_region: a.ipam_region.clone(),
            ipam_resource_discovery_id: a.ipam_resource_discovery_id.clone(),
            owner_id: a.owner_id.clone(),
            is_default: a.is_default,
            resource_discovery_status: a.resource_discovery_status.clone(),
            state: a.state.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<IpamResourceDiscoveryAssociationView> for IpamResourceDiscoveryAssociation {
    fn from(v: IpamResourceDiscoveryAssociationView) -> Self {
        IpamResourceDiscoveryAssociation {
            ipam_resource_discovery_association_id: v.ipam_resource_discovery_association_id,
            ipam_resource_discovery_association_arn: v.ipam_resource_discovery_association_arn,
            ipam_arn: v.ipam_arn,
            ipam_id: v.ipam_id,
            ipam_region: v.ipam_region,
            ipam_resource_discovery_id: v.ipam_resource_discovery_id,
            owner_id: v.owner_id,
            is_default: v.is_default,
            resource_discovery_status: v.resource_discovery_status,
            state: v.state,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamByoasnView {
    pub asn: String,
    pub ipam_id: String,
    #[serde(default)]
    pub description: Option<String>,
    pub state: String,
    #[serde(default)]
    pub status_message: Option<String>,
}

impl From<&IpamByoasn> for IpamByoasnView {
    fn from(b: &IpamByoasn) -> Self {
        IpamByoasnView {
            asn: b.asn.clone(),
            ipam_id: b.ipam_id.clone(),
            description: b.description.clone(),
            state: b.state.clone(),
            status_message: b.status_message.clone(),
        }
    }
}

impl From<IpamByoasnView> for IpamByoasn {
    fn from(v: IpamByoasnView) -> Self {
        IpamByoasn {
            asn: v.asn,
            ipam_id: v.ipam_id,
            description: v.description,
            state: v.state,
            status_message: v.status_message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamExternalResourceVerificationTokenView {
    pub ipam_external_resource_verification_token_id: String,
    pub ipam_external_resource_verification_token_arn: String,
    pub ipam_id: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub token_value: String,
    pub token_name: String,
    pub not_after: String,
    pub status: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&IpamExternalResourceVerificationToken> for IpamExternalResourceVerificationTokenView {
    fn from(t: &IpamExternalResourceVerificationToken) -> Self {
        IpamExternalResourceVerificationTokenView {
            ipam_external_resource_verification_token_id: t
                .ipam_external_resource_verification_token_id
                .clone(),
            ipam_external_resource_verification_token_arn: t
                .ipam_external_resource_verification_token_arn
                .clone(),
            ipam_id: t.ipam_id.clone(),
            ipam_arn: t.ipam_arn.clone(),
            ipam_region: t.ipam_region.clone(),
            token_value: t.token_value.clone(),
            token_name: t.token_name.clone(),
            not_after: t.not_after.clone(),
            status: t.status.clone(),
            state: t.state.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<IpamExternalResourceVerificationTokenView> for IpamExternalResourceVerificationToken {
    fn from(v: IpamExternalResourceVerificationTokenView) -> Self {
        IpamExternalResourceVerificationToken {
            ipam_external_resource_verification_token_id: v
                .ipam_external_resource_verification_token_id,
            ipam_external_resource_verification_token_arn: v
                .ipam_external_resource_verification_token_arn,
            ipam_id: v.ipam_id,
            ipam_arn: v.ipam_arn,
            ipam_region: v.ipam_region,
            token_value: v.token_value,
            token_name: v.token_name,
            not_after: v.not_after,
            status: v.status,
            state: v.state,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPolicyAllocationRuleView {
    #[serde(default)]
    pub source_ipam_pool_id: Option<String>,
}

impl From<&IpamPolicyAllocationRule> for IpamPolicyAllocationRuleView {
    fn from(r: &IpamPolicyAllocationRule) -> Self {
        Self {
            source_ipam_pool_id: r.source_ipam_pool_id.clone(),
        }
    }
}

impl From<IpamPolicyAllocationRuleView> for IpamPolicyAllocationRule {
    fn from(v: IpamPolicyAllocationRuleView) -> Self {
        Self {
            source_ipam_pool_id: v.source_ipam_pool_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPolicyView {
    pub ipam_policy_id: String,
    pub ipam_policy_arn: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub policy_name: String,
    pub policy_type: String,
    #[serde(default)]
    pub description: Option<String>,
    pub state: String,
    #[serde(default)]
    pub allocation_rules: Vec<IpamPolicyAllocationRuleView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub owner_id: String,
}

impl From<&IpamPolicy> for IpamPolicyView {
    fn from(p: &IpamPolicy) -> Self {
        IpamPolicyView {
            ipam_policy_id: p.ipam_policy_id.clone(),
            ipam_policy_arn: p.ipam_policy_arn.clone(),
            ipam_arn: p.ipam_arn.clone(),
            ipam_region: p.ipam_region.clone(),
            policy_name: p.policy_name.clone(),
            policy_type: p.policy_type.clone(),
            description: p.description.clone(),
            state: p.state.clone(),
            allocation_rules: p
                .allocation_rules
                .iter()
                .map(IpamPolicyAllocationRuleView::from)
                .collect(),
            tags: p.tags.clone(),
            owner_id: p.owner_id.clone(),
        }
    }
}

impl From<IpamPolicyView> for IpamPolicy {
    fn from(v: IpamPolicyView) -> Self {
        IpamPolicy {
            ipam_policy_id: v.ipam_policy_id,
            ipam_policy_arn: v.ipam_policy_arn,
            ipam_arn: v.ipam_arn,
            ipam_region: v.ipam_region,
            policy_name: v.policy_name,
            policy_type: v.policy_type,
            description: v.description,
            state: v.state,
            allocation_rules: v
                .allocation_rules
                .into_iter()
                .map(IpamPolicyAllocationRule::from)
                .collect(),
            tags: v.tags,
            owner_id: v.owner_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPrefixListResolverView {
    pub ipam_prefix_list_resolver_id: String,
    pub ipam_prefix_list_resolver_arn: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub state: String,
    pub owner_id: String,
    pub target_count: i32,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&IpamPrefixListResolver> for IpamPrefixListResolverView {
    fn from(r: &IpamPrefixListResolver) -> Self {
        IpamPrefixListResolverView {
            ipam_prefix_list_resolver_id: r.ipam_prefix_list_resolver_id.clone(),
            ipam_prefix_list_resolver_arn: r.ipam_prefix_list_resolver_arn.clone(),
            ipam_arn: r.ipam_arn.clone(),
            ipam_region: r.ipam_region.clone(),
            name: r.name.clone(),
            description: r.description.clone(),
            state: r.state.clone(),
            owner_id: r.owner_id.clone(),
            target_count: r.target_count,
            tags: r.tags.clone(),
        }
    }
}

impl From<IpamPrefixListResolverView> for IpamPrefixListResolver {
    fn from(v: IpamPrefixListResolverView) -> Self {
        IpamPrefixListResolver {
            ipam_prefix_list_resolver_id: v.ipam_prefix_list_resolver_id,
            ipam_prefix_list_resolver_arn: v.ipam_prefix_list_resolver_arn,
            ipam_arn: v.ipam_arn,
            ipam_region: v.ipam_region,
            name: v.name,
            description: v.description,
            state: v.state,
            owner_id: v.owner_id,
            target_count: v.target_count,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPrefixListResolverTargetView {
    pub ipam_prefix_list_resolver_target_id: String,
    pub ipam_prefix_list_resolver_id: String,
    pub target_resource_arn: String,
    pub target_resource_type: String,
    pub target_resource_region: String,
    pub owner_id: String,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&IpamPrefixListResolverTarget> for IpamPrefixListResolverTargetView {
    fn from(t: &IpamPrefixListResolverTarget) -> Self {
        IpamPrefixListResolverTargetView {
            ipam_prefix_list_resolver_target_id: t.ipam_prefix_list_resolver_target_id.clone(),
            ipam_prefix_list_resolver_id: t.ipam_prefix_list_resolver_id.clone(),
            target_resource_arn: t.target_resource_arn.clone(),
            target_resource_type: t.target_resource_type.clone(),
            target_resource_region: t.target_resource_region.clone(),
            owner_id: t.owner_id.clone(),
            state: t.state.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<IpamPrefixListResolverTargetView> for IpamPrefixListResolverTarget {
    fn from(v: IpamPrefixListResolverTargetView) -> Self {
        IpamPrefixListResolverTarget {
            ipam_prefix_list_resolver_target_id: v.ipam_prefix_list_resolver_target_id,
            ipam_prefix_list_resolver_id: v.ipam_prefix_list_resolver_id,
            target_resource_arn: v.target_resource_arn,
            target_resource_type: v.target_resource_type,
            target_resource_region: v.target_resource_region,
            owner_id: v.owner_id,
            state: v.state,
            tags: v.tags,
        }
    }
}

// ---------------------------------------------------------------------------
// Batch B view types and round-trip From impls.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VolumeModificationView {
    pub volume_id: String,
    pub modification_state: String,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub target_size: Option<i32>,
    #[serde(default)]
    pub target_iops: Option<i32>,
    #[serde(default)]
    pub target_throughput: Option<i32>,
    #[serde(default)]
    pub target_volume_type: Option<String>,
    #[serde(default)]
    pub target_multi_attach_enabled: Option<bool>,
    #[serde(default)]
    pub original_size: Option<i32>,
    #[serde(default)]
    pub original_iops: Option<i32>,
    #[serde(default)]
    pub original_throughput: Option<i32>,
    #[serde(default)]
    pub original_volume_type: Option<String>,
    #[serde(default)]
    pub original_multi_attach_enabled: Option<bool>,
    pub progress: i64,
    pub start_time: String,
    #[serde(default)]
    pub end_time: Option<String>,
}

impl From<&crate::types::VolumeModification> for VolumeModificationView {
    fn from(m: &crate::types::VolumeModification) -> Self {
        VolumeModificationView {
            volume_id: m.volume_id.clone(),
            modification_state: m.modification_state.clone(),
            status_message: m.status_message.clone(),
            target_size: m.target_size,
            target_iops: m.target_iops,
            target_throughput: m.target_throughput,
            target_volume_type: m.target_volume_type.clone(),
            target_multi_attach_enabled: m.target_multi_attach_enabled,
            original_size: m.original_size,
            original_iops: m.original_iops,
            original_throughput: m.original_throughput,
            original_volume_type: m.original_volume_type.clone(),
            original_multi_attach_enabled: m.original_multi_attach_enabled,
            progress: m.progress,
            start_time: m.start_time.clone(),
            end_time: m.end_time.clone(),
        }
    }
}

impl From<VolumeModificationView> for crate::types::VolumeModification {
    fn from(v: VolumeModificationView) -> Self {
        crate::types::VolumeModification {
            volume_id: v.volume_id,
            modification_state: v.modification_state,
            status_message: v.status_message,
            target_size: v.target_size,
            target_iops: v.target_iops,
            target_throughput: v.target_throughput,
            target_volume_type: v.target_volume_type,
            target_multi_attach_enabled: v.target_multi_attach_enabled,
            original_size: v.original_size,
            original_iops: v.original_iops,
            original_throughput: v.original_throughput,
            original_volume_type: v.original_volume_type,
            original_multi_attach_enabled: v.original_multi_attach_enabled,
            progress: v.progress,
            start_time: v.start_time,
            end_time: v.end_time,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportVolumeTaskView {
    pub conversion_task_id: String,
    pub expiration_time: String,
    pub image: DiskImageDescriptionView,
    pub volume: DiskImageVolumeDescriptionView,
    pub availability_zone: String,
    pub bytes_converted: i64,
    #[serde(default)]
    pub description: Option<String>,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiskImageDescriptionView {
    pub format: String,
    pub size: i64,
    pub import_manifest_url: String,
    #[serde(default)]
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiskImageVolumeDescriptionView {
    pub size: i64,
    pub id: String,
}

impl From<&crate::types::ImportVolumeTask> for ImportVolumeTaskView {
    fn from(t: &crate::types::ImportVolumeTask) -> Self {
        ImportVolumeTaskView {
            conversion_task_id: t.conversion_task_id.clone(),
            expiration_time: t.expiration_time.clone(),
            image: DiskImageDescriptionView {
                format: t.image.format.clone(),
                size: t.image.size,
                import_manifest_url: t.image.import_manifest_url.clone(),
                checksum: t.image.checksum.clone(),
            },
            volume: DiskImageVolumeDescriptionView {
                size: t.volume.size,
                id: t.volume.id.clone(),
            },
            availability_zone: t.availability_zone.clone(),
            bytes_converted: t.bytes_converted,
            description: t.description.clone(),
            status: t.status.clone(),
            status_message: t.status_message.clone(),
        }
    }
}

impl From<ImportVolumeTaskView> for crate::types::ImportVolumeTask {
    fn from(v: ImportVolumeTaskView) -> Self {
        crate::types::ImportVolumeTask {
            conversion_task_id: v.conversion_task_id,
            expiration_time: v.expiration_time,
            image: crate::types::DiskImageDescription {
                format: v.image.format,
                size: v.image.size,
                import_manifest_url: v.image.import_manifest_url,
                checksum: v.image.checksum,
            },
            volume: crate::types::DiskImageVolumeDescription {
                size: v.volume.size,
                id: v.volume.id,
            },
            availability_zone: v.availability_zone,
            bytes_converted: v.bytes_converted,
            description: v.description,
            status: v.status,
            status_message: v.status_message,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BundleTaskView {
    pub bundle_id: String,
    pub instance_id: String,
    pub bucket: String,
    pub prefix: String,
    pub start_time: String,
    pub update_time: String,
    pub state: String,
    pub progress: String,
    #[serde(default)]
    pub error_code: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
}

impl From<&crate::types::BundleTask> for BundleTaskView {
    fn from(t: &crate::types::BundleTask) -> Self {
        BundleTaskView {
            bundle_id: t.bundle_id.clone(),
            instance_id: t.instance_id.clone(),
            bucket: t.bucket.clone(),
            prefix: t.prefix.clone(),
            start_time: t.start_time.clone(),
            update_time: t.update_time.clone(),
            state: t.state.clone(),
            progress: t.progress.clone(),
            error_code: t.error_code.clone(),
            error_message: t.error_message.clone(),
        }
    }
}

impl From<BundleTaskView> for crate::types::BundleTask {
    fn from(v: BundleTaskView) -> Self {
        crate::types::BundleTask {
            bundle_id: v.bundle_id,
            instance_id: v.instance_id,
            bucket: v.bucket,
            prefix: v.prefix,
            start_time: v.start_time,
            update_time: v.update_time,
            state: v.state,
            progress: v.progress,
            error_code: v.error_code,
            error_message: v.error_message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdFormatEntryView {
    pub use_long_ids: bool,
    #[serde(default)]
    pub deadline: Option<String>,
}

impl From<&crate::types::IdFormatEntry> for IdFormatEntryView {
    fn from(e: &crate::types::IdFormatEntry) -> Self {
        IdFormatEntryView {
            use_long_ids: e.use_long_ids,
            deadline: e.deadline.clone(),
        }
    }
}

impl From<IdFormatEntryView> for crate::types::IdFormatEntry {
    fn from(v: IdFormatEntryView) -> Self {
        crate::types::IdFormatEntry {
            use_long_ids: v.use_long_ids,
            deadline: v.deadline,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutpostLagView {
    pub outpost_lag_id: String,
    pub outpost_arn: String,
    pub owner_id: String,
    pub state: String,
    #[serde(default)]
    pub local_gateway_virtual_interface_ids: Vec<String>,
    #[serde(default)]
    pub service_link_virtual_interface_ids: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&crate::types::OutpostLag> for OutpostLagView {
    fn from(l: &crate::types::OutpostLag) -> Self {
        OutpostLagView {
            outpost_lag_id: l.outpost_lag_id.clone(),
            outpost_arn: l.outpost_arn.clone(),
            owner_id: l.owner_id.clone(),
            state: l.state.clone(),
            local_gateway_virtual_interface_ids: l.local_gateway_virtual_interface_ids.clone(),
            service_link_virtual_interface_ids: l.service_link_virtual_interface_ids.clone(),
            tags: l.tags.clone(),
        }
    }
}

impl From<OutpostLagView> for crate::types::OutpostLag {
    fn from(v: OutpostLagView) -> Self {
        crate::types::OutpostLag {
            outpost_lag_id: v.outpost_lag_id,
            outpost_arn: v.outpost_arn,
            owner_id: v.owner_id,
            state: v.state,
            local_gateway_virtual_interface_ids: v.local_gateway_virtual_interface_ids,
            service_link_virtual_interface_ids: v.service_link_virtual_interface_ids,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportImageTaskView {
    pub export_image_task_id: String,
    #[serde(default)]
    pub description: Option<String>,
    pub image_id: String,
    pub role_name: String,
    pub status: String,
    #[serde(default)]
    pub status_message: Option<String>,
    pub progress: String,
    pub s3_export_location: ExportTaskS3LocationView,
    pub disk_image_format: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportTaskS3LocationView {
    pub s3_bucket: String,
    #[serde(default)]
    pub s3_prefix: Option<String>,
}

impl From<&crate::types::ExportImageTask> for ExportImageTaskView {
    fn from(t: &crate::types::ExportImageTask) -> Self {
        ExportImageTaskView {
            export_image_task_id: t.export_image_task_id.clone(),
            description: t.description.clone(),
            image_id: t.image_id.clone(),
            role_name: t.role_name.clone(),
            status: t.status.clone(),
            status_message: t.status_message.clone(),
            progress: t.progress.clone(),
            s3_export_location: ExportTaskS3LocationView {
                s3_bucket: t.s3_export_location.s3_bucket.clone(),
                s3_prefix: t.s3_export_location.s3_prefix.clone(),
            },
            disk_image_format: t.disk_image_format.clone(),
            tags: t.tags.clone(),
        }
    }
}

impl From<ExportImageTaskView> for crate::types::ExportImageTask {
    fn from(v: ExportImageTaskView) -> Self {
        crate::types::ExportImageTask {
            export_image_task_id: v.export_image_task_id,
            description: v.description,
            image_id: v.image_id,
            role_name: v.role_name,
            status: v.status,
            status_message: v.status_message,
            progress: v.progress,
            s3_export_location: crate::types::ExportTaskS3Location {
                s3_bucket: v.s3_export_location.s3_bucket,
                s3_prefix: v.s3_export_location.s3_prefix,
            },
            disk_image_format: v.disk_image_format,
            tags: v.tags,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for Ec2Service {
    type StateView = Ec2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Ec2StateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = Ec2State::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.vpcs {
                guard.vpcs.insert(k, Vpc::from(v));
            }
            for (k, v) in view.subnets {
                guard.subnets.insert(k, Subnet::from(v));
            }
            for (k, v) in view.igws {
                guard.igws.insert(k, InternetGateway::from(v));
            }
            for (k, v) in view.security_groups {
                guard.security_groups.insert(k, SecurityGroup::from(v));
            }
            for (k, v) in view.route_tables {
                guard.route_tables.insert(k, RouteTable::from(v));
            }
            for (k, v) in view.key_pairs {
                guard.key_pairs.insert(k, KeyPair::from(v));
            }
            for (k, v) in view.network_acls {
                guard.network_acls.insert(k, NetworkAcl::from(v));
            }
            for (k, v) in view.elastic_ips {
                guard.elastic_ips.insert(k, ElasticIp::from(v));
            }
            for (k, v) in view.nat_gateways {
                guard.nat_gateways.insert(k, NatGateway::from(v));
            }
            for (k, v) in view.dhcp_options {
                guard.dhcp_options.insert(k, DhcpOptions::from(v));
            }
            for (k, v) in view.egress_only_igws {
                guard
                    .egress_only_igws
                    .insert(k, EgressOnlyInternetGateway::from(v));
            }
            for (k, v) in view.flow_logs {
                guard.flow_logs.insert(k, FlowLog::from(v));
            }
            for (k, v) in view.vpc_peering_connections {
                guard
                    .vpc_peering_connections
                    .insert(k, VpcPeeringConnection::from(v));
            }
            for (k, v) in view.vpc_endpoints {
                guard.vpc_endpoints.insert(k, VpcEndpoint::from(v));
            }
            for (k, v) in view.managed_prefix_lists {
                guard
                    .managed_prefix_lists
                    .insert(k, ManagedPrefixList::from(v));
            }
            for (k, v) in view.customer_gateways {
                guard.customer_gateways.insert(k, CustomerGateway::from(v));
            }
            for (k, v) in view.vpn_gateways {
                guard.vpn_gateways.insert(k, VpnGateway::from(v));
            }
            for (k, v) in view.vpn_connections {
                guard.vpn_connections.insert(k, VpnConnection::from(v));
            }
            for (k, v) in view.carrier_gateways {
                guard.carrier_gateways.insert(k, CarrierGateway::from(v));
            }
            for (k, v) in view.network_interfaces {
                guard
                    .network_interfaces
                    .insert(k, NetworkInterface::from(v));
            }
            for (k, v) in view.vpc_cidr_associations {
                guard.vpc_cidr_associations.insert(k, v);
            }
            if view.ebs_encryption_by_default {
                guard.ebs_encryption_by_default = true;
            }
            for (k, v) in view.transit_gateways {
                guard.transit_gateways.insert(k, TransitGateway::from(v));
            }
            for (k, v) in view.tgw_vpc_attachments {
                guard
                    .tgw_vpc_attachments
                    .insert(k, TransitGatewayVpcAttachment::from(v));
            }
            for (k, v) in view.tgw_peering_attachments {
                guard
                    .tgw_peering_attachments
                    .insert(k, TransitGatewayPeeringAttachment::from(v));
            }
            for (k, v) in view.tgw_route_tables {
                guard
                    .tgw_route_tables
                    .insert(k, TransitGatewayRouteTable::from(v));
            }
            for (k, v) in view.tgw_routes {
                guard
                    .tgw_routes
                    .insert(k, v.into_iter().map(TransitGatewayRoute::from).collect());
            }
            for (k, v) in view.instances {
                guard.instances.insert(k, Instance::from(v));
            }
            for (k, v) in view.volumes {
                guard.volumes.insert(k, Volume::from(v));
            }
            for (k, v) in view.snapshots {
                guard.snapshots.insert(k, Snapshot::from(v));
            }
            for (k, v) in view.images {
                guard.images.insert(k, Image::from(v));
            }
            for (k, v) in view.launch_templates {
                guard.launch_templates.insert(k, LaunchTemplate::from(v));
            }
            for (k, v) in view.launch_template_versions {
                guard
                    .launch_template_versions
                    .insert(k, v.into_iter().map(LaunchTemplateVersion::from).collect());
            }
            for (k, v) in view.spot_requests {
                guard.spot_requests.insert(k, SpotInstanceRequest::from(v));
            }
            for (k, v) in view.iam_instance_profile_associations {
                guard
                    .iam_instance_profile_associations
                    .insert(k, IamInstanceProfileAssociation::from(v));
            }
            for (k, v) in view.dedicated_hosts {
                guard.dedicated_hosts.insert(k, DedicatedHost::from(v));
            }
            for (k, v) in view.ec2_fleets {
                guard.ec2_fleets.insert(k, Ec2Fleet::from(v));
            }
            for (k, v) in view.vpc_endpoint_service_configs {
                guard
                    .vpc_endpoint_service_configs
                    .insert(k, VpcEndpointServiceConfiguration::from(v));
            }
            for (k, v) in view.spot_fleet_requests {
                guard
                    .spot_fleet_requests
                    .insert(k, SpotFleetRequest::from(v));
            }
            for (k, v) in view.subnet_cidr_reservations {
                guard
                    .subnet_cidr_reservations
                    .insert(k, SubnetCidrReservationEntry::from(v));
            }
            for (k, v) in view.placement_groups {
                guard.placement_groups.insert(k, PlacementGroup::from(v));
            }
            for (k, v) in view.network_interface_permissions {
                guard
                    .network_interface_permissions
                    .insert(k, NetworkInterfacePermission::from(v));
            }
            for (k, v) in view.instance_connect_endpoints {
                guard
                    .instance_connect_endpoints
                    .insert(k, InstanceConnectEndpoint::from(v));
            }
            for (k, v) in view.capacity_reservations {
                guard
                    .capacity_reservations
                    .insert(k, CapacityReservation::from(v));
            }
            for (k, v) in view.capacity_reservation_fleets {
                guard
                    .capacity_reservation_fleets
                    .insert(k, CapacityReservationFleet::from(v));
            }
            for (k, v) in view.coip_pools {
                guard.coip_pools.insert(k, CoipPool::from(v));
            }
            for (k, v) in view.byoip_cidrs {
                guard.byoip_cidrs.insert(k, ByoipCidr::from(v));
            }
            for (k, v) in view.public_ipv4_pools {
                guard.public_ipv4_pools.insert(k, PublicIpv4Pool::from(v));
            }
            for v in view.coip_cidrs {
                let key = (v.cidr.clone(), v.coip_pool_id.clone());
                guard.coip_cidrs.insert(key, CoipCidr::from(v));
            }
            for (k, v) in view.address_transfers {
                guard.address_transfers.insert(k, AddressTransfer::from(v));
            }
            for v in view.security_group_vpc_associations {
                let key = (v.group_id.clone(), v.vpc_id.clone());
                guard
                    .security_group_vpc_associations
                    .insert(key, SecurityGroupVpcAssociation::from(v));
            }
            for v in view.enclave_certificate_iam_role_associations {
                let key = (v.certificate_arn.clone(), v.role_arn.clone());
                guard
                    .enclave_certificate_iam_role_associations
                    .insert(key, EnclaveCertificateIamRoleAssociation::from(v));
            }
            for (k, v) in view.mac_sip_modification_tasks {
                guard
                    .mac_sip_modification_tasks
                    .insert(k, MacSipModificationTask::from(v));
            }
            for (k, v) in view.declarative_policies_reports {
                guard
                    .declarative_policies_reports
                    .insert(k, DeclarativePoliciesReport::from(v));
            }
            for (k, v) in view.vpn_concentrators {
                guard.vpn_concentrators.insert(k, VpnConcentrator::from(v));
            }
            for v in view.vpc_endpoint_connections {
                let key = (v.service_id.clone(), v.vpc_endpoint_id.clone());
                guard
                    .vpc_endpoint_connections
                    .insert(key, VpcEndpointConnection::from(v));
            }
            for (k, v) in view.vpc_endpoint_connection_notifications {
                guard
                    .vpc_endpoint_connection_notifications
                    .insert(k, VpcEndpointConnectionNotification::from(v));
            }
            for (k, v) in view.vpc_block_public_access_exclusions {
                guard
                    .vpc_block_public_access_exclusions
                    .insert(k, VpcBlockPublicAccessExclusion::from(v));
            }
            if let Some(o) = view.vpc_block_public_access_options {
                guard.vpc_block_public_access_options = Some(VpcBlockPublicAccessOptions::from(o));
            }
            for (k, v) in view.vpc_encryption_controls {
                guard
                    .vpc_encryption_controls
                    .insert(k, VpcEncryptionControl::from(v));
            }
            for (k, v) in view.reserved_instances_exchanges {
                guard
                    .reserved_instances_exchanges
                    .insert(k, crate::types::ReservedInstancesExchange::from(v));
            }
            for (k, v) in view.reserved_instances_listings {
                guard
                    .reserved_instances_listings
                    .insert(k, crate::types::ReservedInstancesListing::from(v));
            }
            for (k, v) in view.queued_reserved_instances_purchases {
                guard
                    .queued_reserved_instances_purchases
                    .insert(k, crate::types::ReservedInstancesPurchase::from(v));
            }
            for (k, v) in view.reserved_instances_modifications {
                guard
                    .reserved_instances_modifications
                    .insert(k, crate::types::ReservedInstancesModification::from(v));
            }
            for (k, v) in view.reserved_instances_purchases {
                guard
                    .reserved_instances_purchases
                    .insert(k, crate::types::ReservedInstancesPurchase::from(v));
            }
            for (k, v) in view.reserved_instances {
                guard
                    .reserved_instances
                    .insert(k, crate::types::ReservedInstances::from(v));
            }
            for (k, v) in view.fpga_images {
                guard
                    .fpga_images
                    .insert(k, crate::types::FpgaImage::from(v));
            }
            for (k, v) in view.image_usage_reports {
                guard
                    .image_usage_reports
                    .insert(k, crate::types::ImageUsageReport::from(v));
            }
            for (k, v) in view.restore_image_tasks {
                guard
                    .restore_image_tasks
                    .insert(k, crate::types::RestoreImageTask::from(v));
            }
            for (k, v) in view.store_image_tasks {
                guard
                    .store_image_tasks
                    .insert(k, crate::types::StoreImageTask::from(v));
            }
            for (k, v) in view.import_image_tasks {
                guard
                    .import_image_tasks
                    .insert(k, crate::types::ImportImageTask::from(v));
            }
            if !view.allowed_image_criteria.is_empty() {
                guard.allowed_image_criteria = view
                    .allowed_image_criteria
                    .into_iter()
                    .map(crate::types::AllowedImageCriterion::from)
                    .collect();
            }
            for (k, v) in view.default_credit_specifications {
                guard.default_credit_specifications.insert(k, v);
            }
            if let Some(o) = view.instance_metadata_defaults {
                guard.instance_metadata_defaults =
                    Some(crate::types::InstanceMetadataDefaults::from(o));
            }
            for (k, v) in view.instance_event_windows {
                guard
                    .instance_event_windows
                    .insert(k, crate::types::InstanceEventWindow::from(v));
            }
            if let Some(a) = view.instance_event_notification_attributes {
                guard.instance_event_notification_attributes =
                    Some(crate::types::InstanceTagNotificationAttributes::from(a));
            }
            for (k, v) in view.instance_events {
                guard
                    .instance_events
                    .insert(k, crate::types::InstanceEvent::from(v));
            }
            for (k, v) in view.host_reservations {
                guard
                    .host_reservations
                    .insert(k, crate::types::HostReservation::from(v));
            }
            for (k, v) in view.scheduled_instances {
                guard
                    .scheduled_instances
                    .insert(k, crate::types::ScheduledInstance::from(v));
            }
            for (k, v) in view.az_group_opt_in {
                guard.az_group_opt_in.insert(k, v);
            }
            guard.instance_status_reports.extend(
                view.instance_status_reports
                    .into_iter()
                    .map(crate::types::InstanceStatusReport::from),
            );
            for (k, v) in view.network_insights_access_scopes {
                guard
                    .network_insights_access_scopes
                    .insert(k, NetworkInsightsAccessScope::from(v));
            }
            for (k, v) in view.network_insights_access_scope_analyses {
                guard
                    .network_insights_access_scope_analyses
                    .insert(k, NetworkInsightsAccessScopeAnalysis::from(v));
            }
            for (k, v) in view.network_insights_paths {
                guard
                    .network_insights_paths
                    .insert(k, NetworkInsightsPath::from(v));
            }
            for (k, v) in view.network_insights_analyses {
                guard
                    .network_insights_analyses
                    .insert(k, NetworkInsightsAnalysis::from(v));
            }
            for (k, v) in view.traffic_mirror_filters {
                guard
                    .traffic_mirror_filters
                    .insert(k, TrafficMirrorFilter::from(v));
            }
            for (k, v) in view.traffic_mirror_sessions {
                guard
                    .traffic_mirror_sessions
                    .insert(k, TrafficMirrorSession::from(v));
            }
            for (k, v) in view.traffic_mirror_targets {
                guard
                    .traffic_mirror_targets
                    .insert(k, TrafficMirrorTarget::from(v));
            }
            for (k, v) in view.client_vpn_endpoints {
                guard
                    .client_vpn_endpoints
                    .insert(k, ClientVpnEndpoint::from(v));
            }
            for (k, v) in view.client_vpn_target_network_associations {
                guard
                    .client_vpn_target_network_associations
                    .insert(k, ClientVpnTargetNetworkAssociation::from(v));
            }
            for v in view.client_vpn_authorization_rules {
                let key = (
                    v.client_vpn_endpoint_id.clone(),
                    v.destination_cidr.clone(),
                    v.group_id.clone().unwrap_or_default(),
                );
                guard
                    .client_vpn_authorization_rules
                    .insert(key, ClientVpnAuthorizationRule::from(v));
            }
            for v in view.client_vpn_routes {
                let key = (
                    v.client_vpn_endpoint_id.clone(),
                    v.destination_cidr.clone(),
                    v.target_subnet.clone(),
                );
                guard.client_vpn_routes.insert(key, ClientVpnRoute::from(v));
            }
            for (k, v) in view.client_vpn_connections {
                guard
                    .client_vpn_connections
                    .insert(k, ClientVpnConnection::from(v));
            }
            for (k, v) in view.local_gateways {
                guard.local_gateways.insert(k, LocalGateway::from(v));
            }
            for (k, v) in view.local_gateway_route_tables {
                guard
                    .local_gateway_route_tables
                    .insert(k, LocalGatewayRouteTable::from(v));
            }
            for v in view.local_gateway_routes {
                let key = (
                    v.local_gateway_route_table_id.clone(),
                    v.destination_cidr_block.clone(),
                );
                guard
                    .local_gateway_routes
                    .insert(key, LocalGatewayRoute::from(v));
            }
            for (k, v) in view.local_gateway_route_table_virtual_interface_group_associations {
                guard
                    .local_gateway_route_table_virtual_interface_group_associations
                    .insert(
                        k,
                        LocalGatewayRouteTableVirtualInterfaceGroupAssociation::from(v),
                    );
            }
            for (k, v) in view.local_gateway_route_table_vpc_associations {
                guard
                    .local_gateway_route_table_vpc_associations
                    .insert(k, LocalGatewayRouteTableVpcAssociation::from(v));
            }
            for (k, v) in view.local_gateway_virtual_interfaces {
                guard
                    .local_gateway_virtual_interfaces
                    .insert(k, LocalGatewayVirtualInterface::from(v));
            }
            for (k, v) in view.local_gateway_virtual_interface_groups {
                guard
                    .local_gateway_virtual_interface_groups
                    .insert(k, LocalGatewayVirtualInterfaceGroup::from(v));
            }
            for (k, v) in view.route_servers {
                guard.route_servers.insert(k, RouteServer::from(v));
            }
            for (k, v) in view.route_server_endpoints {
                guard
                    .route_server_endpoints
                    .insert(k, RouteServerEndpoint::from(v));
            }
            for (k, v) in view.route_server_peers {
                guard.route_server_peers.insert(k, RouteServerPeer::from(v));
            }
            for v in view.route_server_associations {
                let key = (v.route_server_id.clone(), v.vpc_id.clone());
                guard
                    .route_server_associations
                    .insert(key, RouteServerAssociation::from(v));
            }
            for (k, v) in view.verified_access_instances {
                guard
                    .verified_access_instances
                    .insert(k, VerifiedAccessInstance::from(v));
            }
            for (k, v) in view.verified_access_trust_providers {
                guard
                    .verified_access_trust_providers
                    .insert(k, VerifiedAccessTrustProvider::from(v));
            }
            for (k, v) in view.verified_access_groups {
                guard
                    .verified_access_groups
                    .insert(k, VerifiedAccessGroup::from(v));
            }
            for (k, v) in view.verified_access_endpoints {
                guard
                    .verified_access_endpoints
                    .insert(k, VerifiedAccessEndpoint::from(v));
            }
            for v in view.verified_access_trust_provider_attachments {
                let key = (v.instance_id.clone(), v.trust_provider_id.clone());
                guard
                    .verified_access_trust_provider_attachments
                    .insert(key, VerifiedAccessTrustProviderAttachment::from(v));
            }
            for (k, v) in view.verified_access_instance_logging_configurations {
                guard
                    .verified_access_instance_logging_configurations
                    .insert(k, VerifiedAccessLogs::from(v));
            }
            for o in view.billing_ownership_offers {
                let key = (
                    o.capacity_reservation_id.clone(),
                    o.unused_reservation_billing_owner_id.clone(),
                );
                guard
                    .billing_ownership_offers
                    .insert(key, BillingOwnershipOffer::from(o));
            }
            for (k, v) in view.capacity_manager_data_exports {
                guard
                    .capacity_manager_data_exports
                    .insert(k, CapacityManagerDataExport::from(v));
            }
            for (k, v) in view.interruptible_capacity_reservation_allocations {
                guard
                    .interruptible_capacity_reservation_allocations
                    .insert(k, InterruptibleCapacityReservationAllocation::from(v));
            }
            for (k, v) in view.capacity_blocks {
                guard.capacity_blocks.insert(k, CapacityBlock::from(v));
            }
            for (k, v) in view.capacity_block_extensions {
                guard
                    .capacity_block_extensions
                    .insert(k, CapacityBlockExtension::from(v));
            }
            if let Some(a) = view.capacity_manager_organizations_access {
                guard.capacity_manager_organizations_access =
                    Some(CapacityManagerOrganizationsAccess::from(a));
            }
            for (k, v) in view.tgw_multicast_domains {
                guard
                    .tgw_multicast_domains
                    .insert(k, TransitGatewayMulticastDomain::from(v));
            }
            for v in view.tgw_multicast_domain_associations {
                let key = (
                    v.transit_gateway_multicast_domain_id.clone(),
                    v.transit_gateway_attachment_id.clone(),
                );
                guard
                    .tgw_multicast_domain_associations
                    .insert(key, TransitGatewayMulticastDomainAssociation::from(v));
            }
            for v in view.tgw_multicast_group_members {
                let key = (
                    v.transit_gateway_multicast_domain_id.clone(),
                    v.group_ip_address.clone(),
                    v.network_interface_id.clone(),
                );
                guard
                    .tgw_multicast_group_members
                    .insert(key, TransitGatewayMulticastGroupMember::from(v));
            }
            for v in view.tgw_multicast_group_sources {
                let key = (
                    v.transit_gateway_multicast_domain_id.clone(),
                    v.group_ip_address.clone(),
                    v.network_interface_id.clone(),
                );
                guard
                    .tgw_multicast_group_sources
                    .insert(key, TransitGatewayMulticastGroupSource::from(v));
            }
            for (k, v) in view.tgw_connects {
                guard.tgw_connects.insert(k, TransitGatewayConnect::from(v));
            }
            for (k, v) in view.tgw_connect_peers {
                guard
                    .tgw_connect_peers
                    .insert(k, TransitGatewayConnectPeer::from(v));
            }
            for (k, v) in view.tgw_metering_policies {
                guard
                    .tgw_metering_policies
                    .insert(k, TransitGatewayMeteringPolicy::from(v));
            }
            for v in view.tgw_metering_policy_entries {
                let key = (
                    v.transit_gateway_metering_policy_id.clone(),
                    v.transit_gateway_metering_policy_entry_id.clone(),
                );
                guard
                    .tgw_metering_policy_entries
                    .insert(key, TransitGatewayMeteringPolicyEntry::from(v));
            }
            for (k, v) in view.tgw_policy_tables {
                guard
                    .tgw_policy_tables
                    .insert(k, TransitGatewayPolicyTable::from(v));
            }
            for v in view.tgw_policy_table_associations {
                let key = (
                    v.transit_gateway_policy_table_id.clone(),
                    v.transit_gateway_attachment_id.clone(),
                );
                guard
                    .tgw_policy_table_associations
                    .insert(key, TransitGatewayPolicyTableAssociation::from(v));
            }
            for v in view.tgw_prefix_list_references {
                let key = (
                    v.transit_gateway_route_table_id.clone(),
                    v.prefix_list_id.clone(),
                );
                guard
                    .tgw_prefix_list_references
                    .insert(key, TransitGatewayPrefixListReference::from(v));
            }
            for (k, v) in view.tgw_route_table_announcements {
                guard
                    .tgw_route_table_announcements
                    .insert(k, TransitGatewayRouteTableAnnouncement::from(v));
            }
            for (k, v) in view.ipams {
                guard.ipams.insert(k, Ipam::from(v));
            }
            for (k, v) in view.ipam_scopes {
                guard.ipam_scopes.insert(k, IpamScope::from(v));
            }
            for (k, v) in view.ipam_pools {
                guard.ipam_pools.insert(k, IpamPool::from(v));
            }
            for v in view.ipam_pool_cidrs {
                let key = (v.ipam_pool_id.clone(), v.cidr.clone());
                guard.ipam_pool_cidrs.insert(key, IpamPoolCidr::from(v));
            }
            for v in view.ipam_pool_allocations {
                let key = (v.ipam_pool_id.clone(), v.ipam_pool_allocation_id.clone());
                guard
                    .ipam_pool_allocations
                    .insert(key, IpamPoolAllocation::from(v));
            }
            for (k, v) in view.ipam_resource_discoveries {
                guard
                    .ipam_resource_discoveries
                    .insert(k, IpamResourceDiscovery::from(v));
            }
            for (k, v) in view.ipam_resource_discovery_associations {
                guard
                    .ipam_resource_discovery_associations
                    .insert(k, IpamResourceDiscoveryAssociation::from(v));
            }
            for v in view.ipam_byoasns {
                let key = (v.ipam_id.clone(), v.asn.clone());
                guard.ipam_byoasns.insert(key, IpamByoasn::from(v));
            }
            for (k, v) in view.ipam_external_resource_verification_tokens {
                guard
                    .ipam_external_resource_verification_tokens
                    .insert(k, IpamExternalResourceVerificationToken::from(v));
            }
            for (k, v) in view.ipam_policies {
                guard.ipam_policies.insert(k, IpamPolicy::from(v));
            }
            for (k, v) in view.ipam_prefix_list_resolvers {
                guard
                    .ipam_prefix_list_resolvers
                    .insert(k, IpamPrefixListResolver::from(v));
            }
            for v in view.ipam_prefix_list_resolver_targets {
                let key = (
                    v.ipam_prefix_list_resolver_id.clone(),
                    v.ipam_prefix_list_resolver_target_id.clone(),
                );
                guard
                    .ipam_prefix_list_resolver_targets
                    .insert(key, IpamPrefixListResolverTarget::from(v));
            }
            for (k, v) in view.volume_modifications {
                guard
                    .volume_modifications
                    .insert(k, crate::types::VolumeModification::from(v));
            }
            for (k, v) in view.import_volume_tasks {
                guard
                    .import_volume_tasks
                    .insert(k, crate::types::ImportVolumeTask::from(v));
            }
            for (k, v) in view.bundle_tasks {
                guard
                    .bundle_tasks
                    .insert(k, crate::types::BundleTask::from(v));
            }
            for (k, v) in view.id_format {
                guard
                    .id_format
                    .insert(k, crate::types::IdFormatEntry::from(v));
            }
            for (k, v) in view.outpost_lags {
                guard
                    .outpost_lags
                    .insert(k, crate::types::OutpostLag::from(v));
            }
            for (k, v) in view.export_image_tasks {
                guard
                    .export_image_tasks
                    .insert(k, crate::types::ExportImageTask::from(v));
            }
            // Wave 4 additions: only overwrite when the incoming view supplies
            // a non-default value. This keeps `merge` additive: an empty incoming
            // view does not clear existing toggles.
            if view.ebs_default_kms_key_id.is_some() {
                guard.ebs_default_kms_key_id = view.ebs_default_kms_key_id;
            }
            if view.serial_console_access_enabled {
                guard.serial_console_access_enabled = true;
            }
            if view.allowed_images_settings_state.is_some() {
                guard.allowed_images_settings_state = view.allowed_images_settings_state;
            }
            if view.image_block_public_access_state.is_some() {
                guard.image_block_public_access_state = view.image_block_public_access_state;
            }
            for s in view.aws_network_performance_subscriptions {
                let key = (
                    s.source.clone(),
                    s.destination.clone(),
                    s.metric.clone(),
                    s.statistic.clone(),
                );
                guard.aws_network_performance_subscriptions.insert(
                    key,
                    crate::types::AwsNetworkPerformanceSubscription {
                        source: s.source,
                        destination: s.destination,
                        metric: s.metric,
                        statistic: s.statistic,
                        period: s.period,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
