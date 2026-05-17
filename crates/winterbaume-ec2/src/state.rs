use std::collections::{HashMap, HashSet};

use crate::types::*;

#[derive(Debug, Default)]
pub struct Ec2State {
    pub vpcs: HashMap<String, Vpc>,
    pub subnets: HashMap<String, Subnet>,
    pub igws: HashMap<String, InternetGateway>,
    pub security_groups: HashMap<String, SecurityGroup>,
    pub route_tables: HashMap<String, RouteTable>,
    pub key_pairs: HashMap<String, KeyPair>,
    pub network_acls: HashMap<String, NetworkAcl>,
    pub elastic_ips: HashMap<String, ElasticIp>,
    pub nat_gateways: HashMap<String, NatGateway>,
    pub dhcp_options: HashMap<String, DhcpOptions>,
    pub egress_only_igws: HashMap<String, EgressOnlyInternetGateway>,
    pub flow_logs: HashMap<String, FlowLog>,
    pub vpc_peering_connections: HashMap<String, VpcPeeringConnection>,
    pub vpc_endpoints: HashMap<String, VpcEndpoint>,
    pub managed_prefix_lists: HashMap<String, ManagedPrefixList>,
    pub customer_gateways: HashMap<String, CustomerGateway>,
    pub vpn_gateways: HashMap<String, VpnGateway>,
    pub vpn_connections: HashMap<String, VpnConnection>,
    pub carrier_gateways: HashMap<String, CarrierGateway>,
    pub network_interfaces: HashMap<String, NetworkInterface>,
    pub vpc_cidr_associations: HashMap<String, String>, // assoc_id -> cidr_block
    pub ebs_encryption_by_default: bool,
    /// Account-level EBS default KMS key ID, set by `ModifyEbsDefaultKmsKeyId`
    /// and reset by `ResetEbsDefaultKmsKeyId`. `None` means use the AWS-managed
    /// default `alias/aws/ebs`.
    pub ebs_default_kms_key_id: Option<String>,
    /// Toggle for `EnableSerialConsoleAccess` / `DisableSerialConsoleAccess`.
    pub serial_console_access_enabled: bool,
    /// State for `EnableAllowedImagesSettings` / `DisableAllowedImagesSettings`,
    /// independent of the criteria list. `None` means never configured (treated
    /// as `disabled`).
    pub allowed_images_settings_state: Option<String>,
    /// Account-level state for `EnableImageBlockPublicAccess`. One of
    /// `block-new-sharing`, `unblocked`. `None` means never configured.
    pub image_block_public_access_state: Option<String>,
    /// Account-level AWS Network Performance metric subscriptions, keyed by
    /// `(source, destination, metric, statistic)`.
    pub aws_network_performance_subscriptions:
        HashMap<(String, String, String, String), AwsNetworkPerformanceSubscription>,
    pub transit_gateways: HashMap<String, TransitGateway>,
    pub tgw_vpc_attachments: HashMap<String, TransitGatewayVpcAttachment>,
    pub tgw_peering_attachments: HashMap<String, TransitGatewayPeeringAttachment>,
    pub tgw_route_tables: HashMap<String, TransitGatewayRouteTable>,
    pub tgw_routes: HashMap<String, Vec<TransitGatewayRoute>>,
    pub instances: HashMap<String, Instance>,
    pub volumes: HashMap<String, Volume>,
    pub snapshots: HashMap<String, Snapshot>,
    pub images: HashMap<String, Image>,
    /// IDs of images that were explicitly deregistered. Used to suppress
    /// stub-synthesis in `describe_images` for IDs that once existed.
    pub deregistered_images: HashSet<String>,
    pub launch_templates: HashMap<String, LaunchTemplate>,
    pub launch_template_versions: HashMap<String, Vec<LaunchTemplateVersion>>,
    pub spot_requests: HashMap<String, SpotInstanceRequest>,
    /// Singleton per-account spot-instance datafeed subscription. `None`
    /// after `DeleteSpotDatafeedSubscription` or before any
    /// `CreateSpotDatafeedSubscription`; `Describe` should return an
    /// `InvalidSpotDatafeed.NotFound` error when this is `None`.
    pub spot_datafeed_subscription: Option<SpotDatafeedSubscription>,
    pub iam_instance_profile_associations: HashMap<String, IamInstanceProfileAssociation>,
    pub dedicated_hosts: HashMap<String, DedicatedHost>,
    pub ec2_fleets: HashMap<String, Ec2Fleet>,
    pub vpc_endpoint_service_configs: HashMap<String, VpcEndpointServiceConfiguration>,
    pub spot_fleet_requests: HashMap<String, SpotFleetRequest>,
    pub subnet_cidr_reservations: HashMap<String, SubnetCidrReservationEntry>,
    pub placement_groups: HashMap<String, PlacementGroup>,
    pub network_interface_permissions: HashMap<String, NetworkInterfacePermission>,
    pub instance_connect_endpoints: HashMap<String, InstanceConnectEndpoint>,
    pub capacity_reservations: HashMap<String, CapacityReservation>,
    pub capacity_reservation_fleets: HashMap<String, CapacityReservationFleet>,
    pub coip_pools: HashMap<String, CoipPool>,
    /// Bring-your-own-IP CIDRs, keyed by CIDR.
    pub byoip_cidrs: HashMap<String, ByoipCidr>,
    /// Public IPv4 pools, keyed by pool ID.
    pub public_ipv4_pools: HashMap<String, PublicIpv4Pool>,
    /// Customer-owned IP CIDRs, keyed by `(cidr, coip_pool_id)`.
    pub coip_cidrs: HashMap<(String, String), CoipCidr>,
    /// Pending / accepted EIP transfer offers, keyed by allocation ID.
    pub address_transfers: HashMap<String, AddressTransfer>,
    /// Keyed by `(group_id, vpc_id)`.
    pub security_group_vpc_associations: HashMap<(String, String), SecurityGroupVpcAssociation>,
    /// Keyed by `(certificate_arn, role_arn)`.
    pub enclave_certificate_iam_role_associations:
        HashMap<(String, String), EnclaveCertificateIamRoleAssociation>,
    pub mac_sip_modification_tasks: HashMap<String, MacSipModificationTask>,
    pub declarative_policies_reports: HashMap<String, DeclarativePoliciesReport>,
    /// Mock VPN concentrator records, keyed by `vpn-concentrator-...` ID.
    pub vpn_concentrators: HashMap<String, VpnConcentrator>,
    /// Pending / accepted endpoint connections to a service configuration,
    /// keyed by `(service_id, vpc_endpoint_id)`.
    pub vpc_endpoint_connections: HashMap<(String, String), VpcEndpointConnection>,
    /// VPC endpoint connection notifications, keyed by notification ID.
    pub vpc_endpoint_connection_notifications: HashMap<String, VpcEndpointConnectionNotification>,
    /// VPC block public access exclusions, keyed by exclusion ID.
    pub vpc_block_public_access_exclusions: HashMap<String, VpcBlockPublicAccessExclusion>,
    /// Account/region-level block public access options. There is at most one
    /// of these per account/region; this is the live value (or `None` when
    /// `ModifyVpcBlockPublicAccessOptions` has never been called).
    pub vpc_block_public_access_options: Option<VpcBlockPublicAccessOptions>,
    /// VPC encryption controls, keyed by `vpc-encryption-control-...` ID.
    pub vpc_encryption_controls: HashMap<String, VpcEncryptionControl>,
    // --- Group 4 additions ---
    /// Mac volume ownership delegation tasks, keyed by task ID.
    pub mac_volume_ownership_tasks: HashMap<String, MacVolumeOwnershipTask>,
    /// Replace-root-volume tasks, keyed by task ID.
    pub replace_root_volume_tasks: HashMap<String, ReplaceRootVolumeTask>,
    /// Snapshot import tasks, keyed by `import-snap-...` ID.
    pub snapshot_import_tasks: HashMap<String, SnapshotImportTask>,
    /// Conversion tasks (ImportInstance / ImportVolume), keyed by task ID.
    pub conversion_tasks: HashMap<String, ConversionTask>,
    /// Export tasks (CreateInstanceExportTask), keyed by export task ID.
    pub export_tasks: HashMap<String, ExportTask>,
    /// Generic import tasks (e.g. ImportImage), keyed by import task ID.
    /// Stores `(state, previous_state)` so `CancelImportTask` can report
    /// the transition.
    pub import_tasks: HashMap<String, (String, Option<String>)>,
    /// Branch / trunk interface associations, keyed by association ID.
    pub trunk_interface_associations: HashMap<String, TrunkInterfaceAssociation>,
    /// Secondary networks, keyed by network ID.
    pub secondary_networks: HashMap<String, SecondaryNetwork>,
    /// Secondary subnets, keyed by subnet ID.
    pub secondary_subnets: HashMap<String, SecondarySubnet>,
    /// Volumes that have been logically deleted but are recoverable via
    /// `RestoreVolumeFromRecycleBin`. Mock policy treats every volume
    /// deletion as recoverable.
    pub deleted_volumes_recycle_bin: HashMap<String, Volume>,
    /// Snapshots that have been logically deleted but are recoverable via
    /// `RestoreSnapshotFromRecycleBin`.
    pub deleted_snapshots_recycle_bin: HashMap<String, Snapshot>,
    // --- Group 5 additions ---
    /// Reserved Instance exchange-quote acceptance records, keyed by exchange ID.
    pub reserved_instances_exchanges: HashMap<String, ReservedInstancesExchange>,
    /// Reserved Instance marketplace listings, keyed by listing ID.
    pub reserved_instances_listings: HashMap<String, ReservedInstancesListing>,
    /// Queued (not-yet-active) Reserved Instance purchase records.
    pub queued_reserved_instances_purchases: HashMap<String, ReservedInstancesPurchase>,
    /// Reserved Instance modification requests, keyed by modification ID.
    pub reserved_instances_modifications: HashMap<String, ReservedInstancesModification>,
    /// Active Reserved Instance purchases, keyed by purchase ID.
    pub reserved_instances_purchases: HashMap<String, ReservedInstancesPurchase>,
    /// Active Reserved Instances, keyed by ReservedInstancesId.
    pub reserved_instances: HashMap<String, ReservedInstances>,
    /// FPGA images, keyed by FpgaImageId.
    pub fpga_images: HashMap<String, FpgaImage>,
    /// Image-usage reports, keyed by ReportId.
    pub image_usage_reports: HashMap<String, ImageUsageReport>,
    /// In-progress and completed `CreateRestoreImageTask` records, keyed by image_id.
    pub restore_image_tasks: HashMap<String, RestoreImageTask>,
    /// In-progress and completed `CreateStoreImageTask` records, keyed by ami_id.
    pub store_image_tasks: HashMap<String, StoreImageTask>,
    /// `ImportImage` task records, keyed by import-task-id.
    pub import_image_tasks: HashMap<String, ImportImageTask>,
    /// Per-region allowed AMI criteria.
    pub allowed_image_criteria: Vec<AllowedImageCriterion>,
    /// Per-account default credit specification, keyed by instance family.
    pub default_credit_specifications: HashMap<String, String>,
    /// Account-level / per-region instance metadata defaults.
    pub instance_metadata_defaults: Option<InstanceMetadataDefaults>,
    /// Instance event windows, keyed by InstanceEventWindowId.
    pub instance_event_windows: HashMap<String, InstanceEventWindow>,
    /// Instance event notification attributes (single-record-per-account).
    pub instance_event_notification_attributes: Option<InstanceTagNotificationAttributes>,
    /// Scheduled instance status events, keyed by event_id.
    pub instance_events: HashMap<String, InstanceEvent>,
    /// Host reservations, keyed by host-reservation ID.
    pub host_reservations: HashMap<String, HostReservation>,
    /// Scheduled instances, keyed by scheduled_instance_id.
    pub scheduled_instances: HashMap<String, ScheduledInstance>,
    /// Per-AZ-group opt-in status.
    pub az_group_opt_in: HashMap<String, String>,
    /// User-submitted instance status reports.
    pub instance_status_reports: Vec<InstanceStatusReport>,
    // --- Group 6 additions ---
    /// Network Insights access scopes, keyed by scope ID.
    pub network_insights_access_scopes: HashMap<String, NetworkInsightsAccessScope>,
    /// Network Insights access scope analyses, keyed by analysis ID.
    pub network_insights_access_scope_analyses: HashMap<String, NetworkInsightsAccessScopeAnalysis>,
    /// Network Insights paths, keyed by path ID.
    pub network_insights_paths: HashMap<String, NetworkInsightsPath>,
    /// Network Insights analyses, keyed by analysis ID.
    pub network_insights_analyses: HashMap<String, NetworkInsightsAnalysis>,
    /// Traffic Mirror filters, keyed by filter ID.
    pub traffic_mirror_filters: HashMap<String, TrafficMirrorFilter>,
    /// Traffic Mirror sessions, keyed by session ID.
    pub traffic_mirror_sessions: HashMap<String, TrafficMirrorSession>,
    /// Traffic Mirror targets, keyed by target ID.
    pub traffic_mirror_targets: HashMap<String, TrafficMirrorTarget>,
    // --- Group 7 additions ---
    /// Client VPN endpoints, keyed by endpoint ID.
    pub client_vpn_endpoints: HashMap<String, ClientVpnEndpoint>,
    /// Client VPN target-network associations, keyed by association ID.
    pub client_vpn_target_network_associations: HashMap<String, ClientVpnTargetNetworkAssociation>,
    /// Client VPN authorization rules, keyed by `(endpoint_id, destination_cidr, group_id)`.
    pub client_vpn_authorization_rules:
        HashMap<(String, String, String), ClientVpnAuthorizationRule>,
    /// Client VPN routes, keyed by `(endpoint_id, destination_cidr, target_subnet)`.
    pub client_vpn_routes: HashMap<(String, String, String), ClientVpnRoute>,
    /// Client VPN connections, keyed by connection ID.
    pub client_vpn_connections: HashMap<String, ClientVpnConnection>,
    /// Local Gateways, keyed by `lgw-...` ID. Seeded on demand by tests/handlers.
    pub local_gateways: HashMap<String, LocalGateway>,
    /// Local Gateway route tables, keyed by `lgw-rtb-...` ID.
    pub local_gateway_route_tables: HashMap<String, LocalGatewayRouteTable>,
    /// Local Gateway routes, keyed by `(local_gateway_route_table_id, destination_cidr_block)`.
    pub local_gateway_routes: HashMap<(String, String), LocalGatewayRoute>,
    /// Virtual-interface group associations, keyed by association ID.
    pub local_gateway_route_table_virtual_interface_group_associations:
        HashMap<String, LocalGatewayRouteTableVirtualInterfaceGroupAssociation>,
    /// VPC associations, keyed by association ID.
    pub local_gateway_route_table_vpc_associations:
        HashMap<String, LocalGatewayRouteTableVpcAssociation>,
    /// Local Gateway virtual interfaces, keyed by virtual interface ID.
    pub local_gateway_virtual_interfaces: HashMap<String, LocalGatewayVirtualInterface>,
    /// Local Gateway virtual interface groups, keyed by group ID.
    pub local_gateway_virtual_interface_groups: HashMap<String, LocalGatewayVirtualInterfaceGroup>,
    // --- Group 8 additions ---
    /// Route Servers, keyed by `rs-...` ID.
    pub route_servers: HashMap<String, RouteServer>,
    /// Route Server endpoints, keyed by endpoint ID.
    pub route_server_endpoints: HashMap<String, RouteServerEndpoint>,
    /// Route Server peers, keyed by peer ID.
    pub route_server_peers: HashMap<String, RouteServerPeer>,
    /// Route Server <-> VPC associations, keyed by `(route_server_id, vpc_id)`.
    pub route_server_associations: HashMap<(String, String), RouteServerAssociation>,
    // --- Group 9 additions ---
    /// Verified Access instances, keyed by `vai-...` ID.
    pub verified_access_instances: HashMap<String, VerifiedAccessInstance>,
    /// Verified Access trust providers, keyed by `vatp-...` ID.
    pub verified_access_trust_providers: HashMap<String, VerifiedAccessTrustProvider>,
    /// Verified Access groups, keyed by `vagr-...` ID.
    pub verified_access_groups: HashMap<String, VerifiedAccessGroup>,
    /// Verified Access endpoints, keyed by `vae-...` ID.
    pub verified_access_endpoints: HashMap<String, VerifiedAccessEndpoint>,
    /// Trust-provider <-> instance attachments, keyed by `(instance_id, trust_provider_id)`.
    pub verified_access_trust_provider_attachments:
        HashMap<(String, String), VerifiedAccessTrustProviderAttachment>,
    /// Per-instance logging configuration, keyed by instance ID.
    pub verified_access_instance_logging_configurations: HashMap<String, VerifiedAccessLogs>,
    // --- Group 10 additions ---
    /// Pending / accepted billing-ownership offers, keyed by `(capacity_reservation_id, account_id)`.
    pub billing_ownership_offers: HashMap<(String, String), BillingOwnershipOffer>,
    /// Capacity Manager data exports, keyed by export ID.
    pub capacity_manager_data_exports: HashMap<String, CapacityManagerDataExport>,
    /// Interruptible capacity-reservation allocations, keyed by allocation ID.
    pub interruptible_capacity_reservation_allocations:
        HashMap<String, InterruptibleCapacityReservationAllocation>,
    /// Capacity Block purchases, keyed by capacity block ID.
    pub capacity_blocks: HashMap<String, CapacityBlock>,
    /// Capacity Block extensions, keyed by extension ID.
    pub capacity_block_extensions: HashMap<String, CapacityBlockExtension>,
    /// Account-level Capacity Manager Organizations access state.
    pub capacity_manager_organizations_access: Option<CapacityManagerOrganizationsAccess>,
    // --- Group 11 additions: Transit Gateway extension families ---
    /// TGW multicast domains, keyed by `tgw-mcast-domain-...` ID.
    pub tgw_multicast_domains: HashMap<String, TransitGatewayMulticastDomain>,
    /// TGW multicast domain↔VPC associations, keyed by `(domain_id, attachment_id)`.
    pub tgw_multicast_domain_associations:
        HashMap<(String, String), TransitGatewayMulticastDomainAssociation>,
    /// TGW multicast group members, keyed by `(domain_id, group_ip, network_interface_id)`.
    pub tgw_multicast_group_members:
        HashMap<(String, String, String), TransitGatewayMulticastGroupMember>,
    /// TGW multicast group sources, keyed by `(domain_id, group_ip, network_interface_id)`.
    pub tgw_multicast_group_sources:
        HashMap<(String, String, String), TransitGatewayMulticastGroupSource>,
    /// TGW connect attachments, keyed by attachment ID.
    pub tgw_connects: HashMap<String, TransitGatewayConnect>,
    /// TGW connect peers, keyed by `tgw-connect-peer-...` ID.
    pub tgw_connect_peers: HashMap<String, TransitGatewayConnectPeer>,
    /// TGW metering policies, keyed by `tgw-mp-...` ID.
    pub tgw_metering_policies: HashMap<String, TransitGatewayMeteringPolicy>,
    /// TGW metering policy entries, keyed by `(policy_id, entry_id)`.
    pub tgw_metering_policy_entries: HashMap<(String, String), TransitGatewayMeteringPolicyEntry>,
    /// TGW policy tables, keyed by `tgw-rtb-policy-...` ID.
    pub tgw_policy_tables: HashMap<String, TransitGatewayPolicyTable>,
    /// TGW policy table associations, keyed by `(policy_table_id, attachment_id)`.
    pub tgw_policy_table_associations:
        HashMap<(String, String), TransitGatewayPolicyTableAssociation>,
    /// TGW prefix-list references, keyed by `(route_table_id, prefix_list_id)`.
    pub tgw_prefix_list_references: HashMap<(String, String), TransitGatewayPrefixListReference>,
    /// TGW route-table announcements, keyed by announcement ID.
    pub tgw_route_table_announcements: HashMap<String, TransitGatewayRouteTableAnnouncement>,
    // --- Group 12 additions: IPAM (IP Address Manager) families ---
    /// IPAMs, keyed by `ipam-...` ID.
    pub ipams: HashMap<String, Ipam>,
    /// IPAM scopes, keyed by `ipam-scope-...` ID.
    pub ipam_scopes: HashMap<String, IpamScope>,
    /// IPAM pools, keyed by `ipam-pool-...` ID.
    pub ipam_pools: HashMap<String, IpamPool>,
    /// IPAM pool CIDRs, keyed by `(pool_id, cidr)`.
    pub ipam_pool_cidrs: HashMap<(String, String), IpamPoolCidr>,
    /// IPAM pool allocations, keyed by `(pool_id, allocation_id)`.
    pub ipam_pool_allocations: HashMap<(String, String), IpamPoolAllocation>,
    /// IPAM resource discoveries, keyed by `ipam-res-disco-...` ID.
    pub ipam_resource_discoveries: HashMap<String, IpamResourceDiscovery>,
    /// IPAM resource discovery associations, keyed by association ID.
    pub ipam_resource_discovery_associations: HashMap<String, IpamResourceDiscoveryAssociation>,
    /// IPAM BYO-ASNs, keyed by `(ipam_id, asn)`.
    pub ipam_byoasns: HashMap<(String, String), IpamByoasn>,
    /// IPAM external resource verification tokens, keyed by token ID.
    pub ipam_external_resource_verification_tokens:
        HashMap<String, IpamExternalResourceVerificationToken>,
    /// IPAM policies, keyed by `ipam-policy-...` ID.
    pub ipam_policies: HashMap<String, IpamPolicy>,
    /// IPAM prefix list resolvers, keyed by resolver ID.
    pub ipam_prefix_list_resolvers: HashMap<String, IpamPrefixListResolver>,
    /// IPAM prefix list resolver targets, keyed by `(resolver_id, target_id)`.
    pub ipam_prefix_list_resolver_targets: HashMap<(String, String), IpamPrefixListResolverTarget>,
    // --- Batch B additions ---
    /// `ModifyVolume` records, keyed by volume_id.
    pub volume_modifications: HashMap<String, VolumeModification>,
    /// `ImportVolume` conversion tasks, keyed by conversion_task_id.
    pub import_volume_tasks: HashMap<String, ImportVolumeTask>,
    /// `BundleInstance` tasks, keyed by bundle_id.
    pub bundle_tasks: HashMap<String, BundleTask>,
    /// Per-resource-type long/short ID format toggles. Defaults to long IDs
    /// for any resource not explicitly modified.
    pub id_format: HashMap<String, IdFormatEntry>,
    /// Outpost LAGs, keyed by outpost-lag ID.
    pub outpost_lags: HashMap<String, OutpostLag>,
    /// `ExportImage` tasks, keyed by export_image_task_id.
    pub export_image_tasks: HashMap<String, ExportImageTask>,
    pub counters: Ec2Counters,
}

#[derive(Debug, Default)]
pub struct Ec2Counters {
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
    pub placement_group: u32,
    pub eni_permission: u32,
    pub instance_connect_endpoint: u32,
    pub capacity_reservation: u32,
    pub capacity_reservation_fleet: u32,
    pub coip_pool: u32,
    pub mac_sip_task: u32,
    pub declarative_policies_report: u32,
    pub public_ipv4_pool: u32,
    pub address_transfer: u32,
    pub nat_gateway_address_assoc: u32,
    pub vpn_concentrator: u32,
    pub vpc_endpoint_connection_notification: u32,
    pub vpc_block_public_access_exclusion: u32,
    pub vpc_encryption_control: u32,
    // --- Group 4 additions ---
    pub mac_volume_ownership_task: u32,
    pub replace_root_volume_task: u32,
    pub snapshot_import_task: u32,
    pub conversion_task: u32,
    pub export_task: u32,
    pub import_task: u32,
    pub trunk_interface_assoc: u32,
    pub secondary_network: u32,
    pub secondary_subnet: u32,
    // --- Group 5 additions ---
    pub reserved_instances_exchange: u32,
    pub reserved_instances_listing: u32,
    pub reserved_instances_purchase: u32,
    pub reserved_instances: u32,
    pub reserved_instances_modification: u32,
    pub fpga_image: u32,
    pub image_usage_report: u32,
    pub import_image_task: u32,
    pub instance_event_window: u32,
    pub instance_event: u32,
    pub host_reservation: u32,
    pub scheduled_instance: u32,
    // --- Group 6 additions ---
    pub network_insights_access_scope: u32,
    pub network_insights_access_scope_analysis: u32,
    pub network_insights_path: u32,
    pub network_insights_analysis: u32,
    pub traffic_mirror_filter: u32,
    pub traffic_mirror_filter_rule: u32,
    pub traffic_mirror_session: u32,
    pub traffic_mirror_target: u32,
    // --- Group 7 additions ---
    pub client_vpn_endpoint: u32,
    pub client_vpn_target_network_association: u32,
    pub client_vpn_connection: u32,
    pub local_gateway: u32,
    pub local_gateway_route_table: u32,
    pub local_gateway_route_table_virtual_interface_group_association: u32,
    pub local_gateway_route_table_vpc_association: u32,
    pub local_gateway_virtual_interface: u32,
    pub local_gateway_virtual_interface_group: u32,
    // --- Group 8 additions ---
    pub route_server: u32,
    pub route_server_endpoint: u32,
    pub route_server_peer: u32,
    // --- Group 9 additions ---
    pub verified_access_instance: u32,
    pub verified_access_trust_provider: u32,
    pub verified_access_group: u32,
    pub verified_access_endpoint: u32,
    // --- Group 10 additions ---
    pub capacity_manager_data_export: u32,
    pub interruptible_capacity_reservation_allocation: u32,
    pub capacity_block: u32,
    pub capacity_block_extension: u32,
    // --- Group 11 additions ---
    pub tgw_multicast_domain: u32,
    pub tgw_connect: u32,
    pub tgw_connect_peer: u32,
    pub tgw_metering_policy: u32,
    pub tgw_metering_policy_entry: u32,
    pub tgw_policy_table: u32,
    pub tgw_route_table_announcement: u32,
    // --- Group 12 additions ---
    pub ipam: u32,
    pub ipam_scope: u32,
    pub ipam_pool: u32,
    pub ipam_pool_cidr: u32,
    pub ipam_pool_allocation: u32,
    pub ipam_resource_discovery: u32,
    pub ipam_resource_discovery_association: u32,
    pub ipam_external_resource_verification_token: u32,
    pub ipam_policy: u32,
    pub ipam_prefix_list_resolver: u32,
    pub ipam_prefix_list_resolver_target: u32,
    // --- Batch B additions ---
    pub bundle_task: u32,
    pub volume_modification: u32,
    pub import_volume_task: u32,
    pub export_image_task: u32,
    pub outpost_lag: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum Ec2Error {
    #[error("The vpc ID '{0}' does not exist")]
    VpcNotFound(String),

    #[error("The network ACL '{0}' does not exist")]
    NetworkAclNotFound(String),

    #[error("Cannot delete the default network ACL")]
    CannotDeleteDefaultNetworkAcl,

    #[error("DHCP options set is associated with a VPC")]
    DhcpOptionsAssociatedWithVpc,

    #[error("The association '{0}' does not exist")]
    AssociationNotFound(String),

    #[error("The association ID '{0}' does not exist")]
    AssociationIdNotFound(String),

    #[error("The allocation ID '{0}' does not exist")]
    AllocationNotFound(String),

    #[error("The subnet ID '{0}' does not exist")]
    SubnetNotFound(String),

    #[error("The NAT gateway '{0}' does not exist")]
    NatGatewayNotFound(String),

    #[error("The DHCP options ID '{0}' does not exist")]
    DhcpOptionsNotFound(String),

    #[error("The internetGateway ID '{0}' does not exist")]
    InternetGatewayNotFound(String),

    #[error("The security group '{0}' does not exist")]
    SecurityGroupNotFound(String),

    #[error("The routeTable ID '{0}' does not exist")]
    RouteTableNotFound(String),

    #[error("The TGW route table '{0}' does not exist")]
    TgwRouteTableNotFound(String),

    #[error("The route with destination '{0}' does not exist")]
    RouteNotFound(String),

    #[error("The egress-only internet gateway '{0}' does not exist")]
    EgressOnlyIgwNotFound(String),

    #[error("The VPC peering connection '{0}' does not exist")]
    VpcPeeringConnectionNotFound(String),

    #[error("The prefix list '{0}' does not exist")]
    PrefixListNotFound(String),

    #[error("The customer gateway '{0}' does not exist")]
    CustomerGatewayNotFound(String),

    #[error("The VPN gateway '{0}' does not exist")]
    VpnGatewayNotFound(String),

    #[error("The VPN connection '{0}' does not exist")]
    VpnConnectionNotFound(String),

    #[error("The carrier gateway '{0}' does not exist")]
    CarrierGatewayNotFound(String),

    #[error("The network interface '{0}' does not exist")]
    NetworkInterfaceNotFound(String),

    #[error("The attachment '{0}' does not exist")]
    AttachmentNotFound(String),

    #[error("The CIDR block association '{0}' does not exist")]
    VpcCidrBlockAssociationNotFound(String),

    #[error("The volume '{0}' does not exist")]
    VolumeNotFound(String),

    #[error("The instance '{0}' does not exist")]
    InstanceNotFound(String),

    #[error("Volume is not attached to the specified instance/device")]
    VolumeNotAttached,

    #[error("The snapshot '{0}' does not exist")]
    SnapshotNotFound(String),

    #[error("The AMI '{0}' does not exist")]
    AmiNotFound(String),

    #[error("A spot datafeed subscription already exists for this account")]
    SpotDatafeedAlreadyExists,

    #[error("The spot datafeed subscription for this account was not found")]
    SpotDatafeedNotFound,

    #[error("A launch template with name '{0}' already exists")]
    LaunchTemplateAlreadyExists(String),

    #[error("Launch template '{0}' does not exist")]
    LaunchTemplateNotFound(String),

    #[error("The transit gateway '{0}' does not exist")]
    TransitGatewayNotFound(String),

    #[error("The TGW attachment '{0}' does not exist")]
    TgwVpcAttachmentNotFound(String),

    #[error("The peering attachment '{0}' does not exist")]
    TgwPeeringAttachmentNotFound(String),

    #[error("The endpoint '{0}' does not exist")]
    VpcEndpointNotFound(String),

    #[error("The VPC endpoint service ID '{0}' does not exist")]
    VpcEndpointServiceNotFound(String),

    #[error("The CIDR reservation '{0}' does not exist")]
    SubnetCidrReservationNotFound(String),

    #[error("A Default VPC already exists for this user in this region.")]
    DefaultVpcAlreadyExists,

    #[error("No default VPC found")]
    DefaultVpcNotFound,

    #[error("The placement group '{0}' does not exist")]
    PlacementGroupNotFound(String),

    #[error("The placement group '{0}' already exists")]
    PlacementGroupAlreadyExists(String),

    #[error("{0}")]
    InvalidParameterValue(String),

    #[error("The network interface permission '{0}' does not exist")]
    NetworkInterfacePermissionNotFound(String),

    #[error("The instance connect endpoint '{0}' does not exist")]
    InstanceConnectEndpointNotFound(String),

    #[error("The capacity reservation '{0}' does not exist")]
    CapacityReservationNotFound(String),

    #[error("The capacity reservation fleet '{0}' does not exist")]
    CapacityReservationFleetNotFound(String),

    #[error("The COIP pool '{0}' does not exist")]
    CoipPoolNotFound(String),

    #[error("The security group VPC association for group '{0}' and VPC '{1}' does not exist")]
    SecurityGroupVpcAssociationNotFound(String, String),

    #[error(
        "The enclave certificate IAM role association for certificate '{0}' and role '{1}' does not exist"
    )]
    EnclaveCertificateIamRoleAssociationNotFound(String, String),

    #[error("The Mac SIP modification task '{0}' does not exist")]
    MacSipModificationTaskNotFound(String),

    #[error("The declarative policies report '{0}' does not exist")]
    DeclarativePoliciesReportNotFound(String),

    #[error("The declarative policies report '{0}' is not in a cancellable state")]
    DeclarativePoliciesReportNotCancellable(String),

    #[error("The BYOIP CIDR '{0}' does not exist")]
    InvalidByoipCidrNotFound(String),

    #[error("The BYOIP CIDR '{0}' already exists")]
    ByoipCidrAlreadyExists(String),

    #[error("The BYOIP CIDR '{0}' is not in a state that allows this operation")]
    ByoipCidrInvalidState(String),

    #[error("The public IPv4 pool '{0}' does not exist")]
    InvalidPublicIpv4PoolNotFound(String),

    #[error("The public IPv4 pool '{0}' is not empty")]
    PublicIpv4PoolNotEmpty(String),

    #[error("The CIDR '{0}' does not belong to the public IPv4 pool '{1}'")]
    PublicIpv4PoolCidrNotFound(String, String),

    #[error("The COIP CIDR '{0}' in pool '{1}' does not exist")]
    CoipCidrNotFound(String, String),

    #[error("The COIP CIDR '{0}' in pool '{1}' already exists")]
    CoipCidrAlreadyExists(String, String),

    #[error("The address transfer for allocation '{0}' does not exist")]
    InvalidAddressTransferNotFound(String),

    #[error(
        "The NAT gateway secondary address (allocation '{0}' / private IP '{1}') does not exist"
    )]
    InvalidNatGatewaySecondaryAddressNotFound(String, String),

    #[error("The VPN concentrator '{0}' does not exist")]
    InvalidVpnConcentratorNotFound(String),

    #[error("The VPC endpoint connection (service '{0}', endpoint '{1}') does not exist")]
    InvalidVpcEndpointConnectionNotFound(String, String),

    #[error("The VPC endpoint connection notification '{0}' does not exist")]
    InvalidVpcEndpointConnectionNotificationNotFound(String),

    #[error("The VPC block public access exclusion '{0}' does not exist")]
    InvalidVpcBlockPublicAccessExclusionNotFound(String),

    #[error("VPC block public access options have not been configured for this account/region")]
    InvalidVpcBlockPublicAccessOptionsNotFound,

    #[error("The VPC encryption control '{0}' does not exist")]
    InvalidVpcEncryptionControlNotFound(String),

    #[error(
        "The VPN connection static route ({vpn_connection_id} -> {destination_cidr}) does not exist"
    )]
    InvalidVpnConnectionRouteNotFound {
        vpn_connection_id: String,
        destination_cidr: String,
    },

    #[error(
        "The VPN tunnel ({vpn_connection_id} / {outside_ip_address}) does not exist on this VPN connection"
    )]
    InvalidVpnTunnelNotFound {
        vpn_connection_id: String,
        outside_ip_address: String,
    },

    #[error("The conversion task '{0}' does not exist")]
    InvalidConversionTaskNotFound(String),

    #[error("The export task '{0}' does not exist")]
    InvalidExportTaskNotFound(String),

    #[error("The import task '{0}' does not exist")]
    InvalidImportTaskNotFound(String),

    #[error("The snapshot import task '{0}' does not exist")]
    InvalidSnapshotImportTaskNotFound(String),

    #[error("The Mac volume ownership delegation task '{0}' does not exist")]
    InvalidMacVolumeOwnershipTaskNotFound(String),

    #[error("The replace-root-volume task '{0}' does not exist")]
    InvalidReplaceRootVolumeTaskNotFound(String),

    #[error("The secondary network '{0}' does not exist")]
    InvalidSecondaryNetworkNotFound(String),

    #[error("The secondary subnet '{0}' does not exist")]
    InvalidSecondarySubnetNotFound(String),

    #[error("The trunk interface association '{0}' does not exist")]
    InvalidTrunkInterfaceAssociationNotFound(String),

    #[error("The secondary network '{0}' has dependent secondary subnets")]
    SecondaryNetworkHasSubnets(String),

    #[error("The snapshot '{0}' is locked and cannot be modified or deleted")]
    SnapshotIsLocked(String),

    #[error("The snapshot '{0}' is not in the recycle bin")]
    SnapshotNotInRecycleBin(String),

    #[error("The volume '{0}' is not in the recycle bin")]
    VolumeNotInRecycleBin(String),

    #[error("The FPGA image '{0}' does not exist")]
    InvalidFpgaImageNotFound(String),

    #[error("The import image task '{0}' does not exist")]
    InvalidImportImageTaskNotFound(String),

    #[error("The store image task for AMI '{0}' does not exist")]
    InvalidStoreImageTaskNotFound(String),

    #[error("The restore image task for image '{0}' does not exist")]
    InvalidRestoreImageTaskNotFound(String),

    #[error("The image usage report '{0}' does not exist")]
    InvalidImageUsageReportNotFound(String),

    #[error("The instance event window '{0}' does not exist")]
    InvalidInstanceEventWindowNotFound(String),

    #[error("The instance event '{0}' does not exist")]
    InvalidInstanceEventNotFound(String),

    #[error("The host reservation '{0}' does not exist")]
    InvalidHostReservationNotFound(String),

    #[error("The scheduled instance '{0}' does not exist")]
    InvalidScheduledInstanceNotFound(String),

    #[error("The reserved instances listing '{0}' does not exist")]
    InvalidReservedInstancesListingNotFound(String),

    #[error("The reserved instances exchange '{0}' does not exist")]
    InvalidReservedInstancesExchangeNotFound(String),

    #[error("The reserved instances modification '{0}' does not exist")]
    InvalidReservedInstancesModificationNotFound(String),

    #[error("The queued reserved instances purchase '{0}' does not exist")]
    InvalidQueuedReservedInstancesNotFound(String),

    #[error("The launch template version '{0}' does not exist")]
    InvalidLaunchTemplateVersionNotFound(i64),

    #[error("The image '{0}' is not in the recycle bin")]
    ImageNotInRecycleBin(String),

    #[error("The network insights access scope '{0}' does not exist")]
    InvalidNetworkInsightsAccessScopeNotFound(String),

    #[error("The network insights access scope analysis '{0}' does not exist")]
    InvalidNetworkInsightsAccessScopeAnalysisNotFound(String),

    #[error("The network insights path '{0}' does not exist")]
    InvalidNetworkInsightsPathNotFound(String),

    #[error("The network insights analysis '{0}' does not exist")]
    InvalidNetworkInsightsAnalysisNotFound(String),

    #[error("The network insights path '{0}' has active analyses and cannot be deleted")]
    NetworkInsightsPathHasAnalyses(String),

    #[error("The traffic mirror filter '{0}' does not exist")]
    InvalidTrafficMirrorFilterNotFound(String),

    #[error("The traffic mirror filter '{0}' is in use by one or more sessions")]
    TrafficMirrorFilterInUse(String),

    #[error("The traffic mirror filter rule '{0}' does not exist")]
    InvalidTrafficMirrorFilterRuleNotFound(String),

    #[error("The traffic mirror session '{0}' does not exist")]
    InvalidTrafficMirrorSessionNotFound(String),

    #[error("The traffic mirror target '{0}' does not exist")]
    InvalidTrafficMirrorTargetNotFound(String),

    #[error("The traffic mirror target '{0}' is in use by one or more sessions")]
    TrafficMirrorTargetInUse(String),

    // --- Group 7 errors ---
    #[error("The Client VPN endpoint '{0}' does not exist")]
    InvalidClientVpnEndpointNotFound(String),

    #[error("The Client VPN endpoint '{0}' is in use and cannot be deleted")]
    ClientVpnEndpointInUse(String),

    #[error("The Client VPN target-network association '{0}' does not exist")]
    InvalidClientVpnTargetNetworkAssociationNotFound(String),

    #[error("The Client VPN authorization rule for endpoint '{0}' / cidr '{1}' does not exist")]
    InvalidClientVpnAuthorizationRuleNotFound(String, String),

    #[error("The Client VPN route for endpoint '{0}' / cidr '{1}' / subnet '{2}' does not exist")]
    InvalidClientVpnRouteNotFound(String, String, String),

    #[error("The local gateway '{0}' does not exist")]
    InvalidLocalGatewayNotFound(String),

    #[error("The local gateway route table '{0}' does not exist")]
    InvalidLocalGatewayRouteTableNotFound(String),

    #[error("The local gateway route table '{0}' is in use and has dependent routes")]
    LocalGatewayRouteTableInUse(String),

    #[error("The local gateway route ({0}, {1}) does not exist")]
    InvalidLocalGatewayRouteNotFound(String, String),

    #[error("The local gateway virtual interface '{0}' does not exist")]
    InvalidLocalGatewayVirtualInterfaceNotFound(String),

    #[error("The local gateway virtual interface '{0}' is in use by a virtual interface group")]
    LocalGatewayVirtualInterfaceInUse(String),

    #[error("The local gateway virtual interface group '{0}' does not exist")]
    InvalidLocalGatewayVirtualInterfaceGroupNotFound(String),

    #[error(
        "The local gateway virtual interface group '{0}' is in use and has dependent interfaces or associations"
    )]
    LocalGatewayVirtualInterfaceGroupInUse(String),

    #[error(
        "The local gateway route table virtual interface group association '{0}' does not exist"
    )]
    InvalidLocalGatewayRouteTableVirtualInterfaceGroupAssociationNotFound(String),

    #[error("The local gateway route table VPC association '{0}' does not exist")]
    InvalidLocalGatewayRouteTableVpcAssociationNotFound(String),

    // --- Group 8 errors ---
    #[error("The route server '{0}' does not exist")]
    InvalidRouteServerNotFound(String),

    #[error("The route server '{0}' is in use and cannot be deleted")]
    RouteServerInUse(String),

    #[error("The route server endpoint '{0}' does not exist")]
    InvalidRouteServerEndpointNotFound(String),

    #[error("The route server endpoint '{0}' is in use and cannot be deleted")]
    RouteServerEndpointInUse(String),

    #[error("The route server peer '{0}' does not exist")]
    InvalidRouteServerPeerNotFound(String),

    #[error("The route server association for route server '{0}' and VPC '{1}' does not exist")]
    InvalidRouteServerAssociationNotFound(String, String),

    // --- Group 9 errors ---
    #[error("The Verified Access instance '{0}' does not exist")]
    InvalidVerifiedAccessInstanceNotFound(String),

    #[error("The Verified Access instance '{0}' is in use and cannot be deleted")]
    VerifiedAccessInstanceInUse(String),

    #[error("The Verified Access trust provider '{0}' does not exist")]
    InvalidVerifiedAccessTrustProviderNotFound(String),

    #[error("The Verified Access trust provider '{0}' is in use and cannot be deleted")]
    VerifiedAccessTrustProviderInUse(String),

    #[error("The Verified Access group '{0}' does not exist")]
    InvalidVerifiedAccessGroupNotFound(String),

    #[error("The Verified Access group '{0}' is in use and cannot be deleted")]
    VerifiedAccessGroupInUse(String),

    #[error("The Verified Access endpoint '{0}' does not exist")]
    InvalidVerifiedAccessEndpointNotFound(String),

    #[error(
        "The Verified Access trust provider attachment (instance '{0}', trust provider '{1}') does not exist"
    )]
    InvalidVerifiedAccessTrustProviderAttachmentNotFound(String, String),

    // --- Group 10 errors ---
    #[error(
        "The billing ownership offer for capacity reservation '{0}' and account '{1}' does not exist"
    )]
    InvalidBillingOwnershipOfferNotFound(String, String),

    #[error(
        "A billing ownership offer for capacity reservation '{0}' and account '{1}' already exists"
    )]
    BillingOwnershipOfferAlreadyExists(String, String),

    #[error("The capacity manager data export '{0}' does not exist")]
    InvalidCapacityManagerDataExportNotFound(String),

    #[error("The interruptible capacity reservation allocation '{0}' does not exist")]
    InvalidInterruptibleCapacityReservationAllocationNotFound(String),

    #[error("The capacity block '{0}' does not exist")]
    InvalidCapacityBlockNotFound(String),

    #[error("The capacity block extension '{0}' does not exist")]
    InvalidCapacityBlockExtensionNotFound(String),

    #[error("Insufficient capacity in capacity reservation '{0}': have {1}, requested {2}")]
    InsufficientCapacityReservationCapacity(String, i32, i32),

    // --- Group 11 errors ---
    #[error("The TGW multicast domain '{0}' does not exist")]
    InvalidTgwMulticastDomainNotFound(String),

    #[error("The TGW multicast domain '{0}' is in use and cannot be deleted")]
    TgwMulticastDomainInUse(String),

    #[error("The TGW multicast domain association (domain '{0}', attachment '{1}') does not exist")]
    InvalidTgwMulticastDomainAssociationNotFound(String, String),

    #[error("The TGW multicast group member '{0}' does not exist")]
    InvalidTgwMulticastGroupMemberNotFound(String),

    #[error("The TGW multicast group source '{0}' does not exist")]
    InvalidTgwMulticastGroupSourceNotFound(String),

    #[error("The TGW connect '{0}' does not exist")]
    InvalidTgwConnectNotFound(String),

    #[error("The TGW connect '{0}' has dependent connect peers")]
    TgwConnectInUse(String),

    #[error("The TGW connect peer '{0}' does not exist")]
    InvalidTgwConnectPeerNotFound(String),

    #[error("The TGW metering policy '{0}' does not exist")]
    InvalidTgwMeteringPolicyNotFound(String),

    #[error("The TGW metering policy entry '{0}' does not exist")]
    InvalidTgwMeteringPolicyEntryNotFound(String),

    #[error("The TGW policy table '{0}' does not exist")]
    InvalidTgwPolicyTableNotFound(String),

    #[error(
        "The TGW policy table association (policy table '{0}', attachment '{1}') does not exist"
    )]
    InvalidTgwPolicyTableAssociationNotFound(String, String),

    #[error("The TGW prefix list reference (route table '{0}', prefix list '{1}') does not exist")]
    InvalidTgwPrefixListReferenceNotFound(String, String),

    #[error("The TGW route table announcement '{0}' does not exist")]
    InvalidTgwRouteTableAnnouncementNotFound(String),

    #[error("The TGW attachment '{0}' is not in the pendingAcceptance state")]
    TgwAttachmentNotPendingAcceptance(String),

    #[error("No TGW route in route table '{0}' for destination CIDR '{1}'")]
    TgwRouteNotFound(String, String),

    // --- Group 12 errors ---
    #[error("The IPAM '{0}' does not exist")]
    InvalidIpamNotFound(String),

    #[error("The IPAM '{0}' has dependent scopes or pools and cannot be deleted")]
    IpamInUse(String),

    #[error("The IPAM scope '{0}' does not exist")]
    InvalidIpamScopeNotFound(String),

    #[error("The IPAM scope '{0}' has dependent pools and cannot be deleted")]
    IpamScopeInUse(String),

    #[error("The IPAM scope '{0}' is the default scope and cannot be deleted")]
    IpamScopeIsDefault(String),

    #[error("The IPAM pool '{0}' does not exist")]
    InvalidIpamPoolNotFound(String),

    #[error("The IPAM pool '{0}' has dependent allocations and cannot be deleted")]
    IpamPoolInUse(String),

    #[error("The IPAM pool CIDR ({0}, {1}) does not exist")]
    InvalidIpamPoolCidrNotFound(String, String),

    #[error("The IPAM pool allocation ({0}, {1}) does not exist")]
    InvalidIpamPoolAllocationNotFound(String, String),

    #[error("The IPAM resource discovery '{0}' does not exist")]
    InvalidIpamResourceDiscoveryNotFound(String),

    #[error("The IPAM resource discovery '{0}' is in use and cannot be deleted")]
    IpamResourceDiscoveryInUse(String),

    #[error("The IPAM resource discovery association '{0}' does not exist")]
    InvalidIpamResourceDiscoveryAssociationNotFound(String),

    #[error("The IPAM BYO-ASN ({0}, {1}) does not exist")]
    InvalidIpamByoasnNotFound(String, String),

    #[error("The IPAM BYO-ASN '{0}' is already associated with a CIDR")]
    IpamByoasnAlreadyAssociated(String),

    #[error("The IPAM external resource verification token '{0}' does not exist")]
    InvalidIpamExternalResourceVerificationTokenNotFound(String),

    #[error("The IPAM policy '{0}' does not exist")]
    InvalidIpamPolicyNotFound(String),

    #[error("The IPAM prefix list resolver '{0}' does not exist")]
    InvalidIpamPrefixListResolverNotFound(String),

    #[error("The IPAM prefix list resolver '{0}' has dependent targets and cannot be deleted")]
    IpamPrefixListResolverInUse(String),

    #[error("The IPAM prefix list resolver target ({0}, {1}) does not exist")]
    InvalidIpamPrefixListResolverTargetNotFound(String, String),

    // --- Batch B errors ---
    #[error("The volume '{0}' has no modifications")]
    InvalidVolumeNotFound(String),

    #[error("The bundle task '{0}' does not exist")]
    InvalidBundleTaskNotFound(String),

    #[error("The import volume task '{0}' does not exist")]
    InvalidImportVolumeTaskNotFound(String),

    #[error("The export image task '{0}' does not exist")]
    InvalidExportImageTaskNotFound(String),

    #[error("The outpost LAG '{0}' does not exist")]
    InvalidOutpostLagNotFound(String),
}

impl Ec2State {
    fn next_vpc_id(&mut self) -> String {
        self.counters.vpc += 1;
        format!("vpc-{:08x}", self.counters.vpc)
    }

    fn next_subnet_id(&mut self) -> String {
        self.counters.subnet += 1;
        format!("subnet-{:08x}", self.counters.subnet)
    }

    fn next_igw_id(&mut self) -> String {
        self.counters.igw += 1;
        format!("igw-{:08x}", self.counters.igw)
    }

    fn next_sg_id(&mut self) -> String {
        self.counters.sg += 1;
        format!("sg-{:08x}", self.counters.sg)
    }

    pub fn next_sgr_id(&mut self) -> String {
        self.counters.sgr += 1;
        format!("sgr-{:08x}", self.counters.sgr)
    }

    fn next_rtb_id(&mut self) -> String {
        self.counters.rtb += 1;
        format!("rtb-{:08x}", self.counters.rtb)
    }

    fn next_rtbassoc_id(&mut self) -> String {
        self.counters.rtbassoc += 1;
        format!("rtbassoc-{:08x}", self.counters.rtbassoc)
    }

    fn next_keypair_id(&mut self) -> String {
        self.counters.keypair += 1;
        format!("key-{:08x}", self.counters.keypair)
    }

    fn next_nacl_id(&mut self) -> String {
        self.counters.nacl += 1;
        format!("acl-{:08x}", self.counters.nacl)
    }

    fn next_nacl_assoc_id(&mut self) -> String {
        self.counters.nacl_assoc += 1;
        format!("aclassoc-{:08x}", self.counters.nacl_assoc)
    }

    fn next_eip_id(&mut self) -> String {
        self.counters.eip += 1;
        format!("eipalloc-{:08x}", self.counters.eip)
    }

    fn next_nat_id(&mut self) -> String {
        self.counters.nat += 1;
        format!("nat-{:08x}", self.counters.nat)
    }

    fn next_dopt_id(&mut self) -> String {
        self.counters.dopt += 1;
        format!("dopt-{:08x}", self.counters.dopt)
    }

    fn next_tgw_id(&mut self) -> String {
        self.counters.tgw += 1;
        format!("tgw-{:08x}", self.counters.tgw)
    }

    fn next_tgw_attach_id(&mut self) -> String {
        self.counters.tgw_attach += 1;
        format!("tgw-attach-{:08x}", self.counters.tgw_attach)
    }

    fn next_tgw_rtb_id(&mut self) -> String {
        self.counters.tgw_rtb += 1;
        format!("tgw-rtb-{:08x}", self.counters.tgw_rtb)
    }

    pub fn next_tgw_multicast_domain_id(&mut self) -> String {
        self.counters.tgw_multicast_domain += 1;
        format!(
            "tgw-mcast-domain-{:08x}",
            self.counters.tgw_multicast_domain
        )
    }

    pub fn next_tgw_connect_id(&mut self) -> String {
        // Connect attachments share the tgw-attach- ID prefix in real AWS;
        // we mirror that and bump only the tgw_attach counter.
        self.counters.tgw_connect = self.counters.tgw_connect.wrapping_add(1);
        self.next_tgw_attach_id()
    }

    pub fn next_tgw_connect_peer_id(&mut self) -> String {
        self.counters.tgw_connect_peer += 1;
        format!("tgw-connect-peer-{:08x}", self.counters.tgw_connect_peer)
    }

    pub fn next_tgw_metering_policy_id(&mut self) -> String {
        self.counters.tgw_metering_policy += 1;
        format!("tgw-mp-{:08x}", self.counters.tgw_metering_policy)
    }

    pub fn next_tgw_metering_policy_entry_id(&mut self) -> String {
        self.counters.tgw_metering_policy_entry += 1;
        format!(
            "tgw-mp-entry-{:08x}",
            self.counters.tgw_metering_policy_entry
        )
    }

    pub fn next_tgw_policy_table_id(&mut self) -> String {
        self.counters.tgw_policy_table += 1;
        format!("tgw-rtb-policy-{:08x}", self.counters.tgw_policy_table)
    }

    pub fn next_tgw_route_table_announcement_id(&mut self) -> String {
        self.counters.tgw_route_table_announcement += 1;
        format!(
            "tgw-rtb-ann-{:08x}",
            self.counters.tgw_route_table_announcement
        )
    }

    fn next_instance_id(&mut self) -> String {
        self.counters.instance += 1;
        format!("i-{:08x}", self.counters.instance)
    }

    fn next_vol_id(&mut self) -> String {
        self.counters.vol += 1;
        format!("vol-{:08x}", self.counters.vol)
    }

    fn next_snapshot_id(&mut self) -> String {
        self.counters.snapshot += 1;
        format!("snap-{:08x}", self.counters.snapshot)
    }

    fn next_ami_id(&mut self) -> String {
        self.counters.ami += 1;
        format!("ami-{:08x}", self.counters.ami)
    }

    fn next_lt_id(&mut self) -> String {
        self.counters.lt += 1;
        format!("lt-{:08x}", self.counters.lt)
    }

    fn next_spot_id(&mut self) -> String {
        self.counters.spot += 1;
        format!("sir-{:08x}", self.counters.spot)
    }

    fn next_iam_assoc_id(&mut self) -> String {
        self.counters.iam_assoc += 1;
        format!("iip-assoc-{:08x}", self.counters.iam_assoc)
    }

    fn next_host_id(&mut self) -> String {
        self.counters.host += 1;
        format!("h-{:08x}", self.counters.host)
    }

    fn next_fleet_id(&mut self) -> String {
        self.counters.fleet += 1;
        format!("fleet-{:08x}", self.counters.fleet)
    }

    fn next_vpce_svc_id(&mut self) -> String {
        self.counters.vpce_svc += 1;
        format!("vpce-svc-{:08x}", self.counters.vpce_svc)
    }

    fn next_spot_fleet_id(&mut self) -> String {
        self.counters.spot_fleet += 1;
        format!("sfr-{:08x}", self.counters.spot_fleet)
    }

    fn next_subnet_cidr_res_id(&mut self) -> String {
        self.counters.subnet_cidr_res += 1;
        format!("scr-{:08x}", self.counters.subnet_cidr_res)
    }

    fn next_subnet_ipv6_assoc_id(&mut self) -> String {
        self.counters.subnet_ipv6_assoc += 1;
        format!("subnet-cidr-assoc-{:08x}", self.counters.subnet_ipv6_assoc)
    }

    // --- Network ACL operations ---

    pub fn create_network_acl(
        &mut self,
        vpc_id: &str,
        tags: Tags,
    ) -> Result<&NetworkAcl, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let nacl_id = self.next_nacl_id();
        let nacl = NetworkAcl {
            network_acl_id: nacl_id.clone(),
            vpc_id: vpc_id.to_string(),
            is_default: false,
            entries: vec![
                // Default deny-all inbound
                NetworkAclEntry {
                    rule_number: 32767,
                    protocol: "-1".to_string(),
                    rule_action: "deny".to_string(),
                    egress: false,
                    cidr_block: Some("0.0.0.0/0".to_string()),
                    ipv6_cidr_block: None,
                    port_range: None,
                    icmp_type_code: None,
                },
                // Default deny-all outbound
                NetworkAclEntry {
                    rule_number: 32767,
                    protocol: "-1".to_string(),
                    rule_action: "deny".to_string(),
                    egress: true,
                    cidr_block: Some("0.0.0.0/0".to_string()),
                    ipv6_cidr_block: None,
                    port_range: None,
                    icmp_type_code: None,
                },
            ],
            associations: Vec::new(),
            tags,
        };
        self.network_acls.insert(nacl_id.clone(), nacl);
        Ok(self.network_acls.get(&nacl_id).unwrap())
    }

    pub fn delete_network_acl(&mut self, nacl_id: &str) -> Result<(), Ec2Error> {
        let nacl = self
            .network_acls
            .get(nacl_id)
            .ok_or_else(|| Ec2Error::NetworkAclNotFound(nacl_id.to_string()))?;
        if nacl.is_default {
            return Err(Ec2Error::CannotDeleteDefaultNetworkAcl);
        }
        self.network_acls.remove(nacl_id);
        Ok(())
    }

    pub fn create_network_acl_entry(
        &mut self,
        nacl_id: &str,
        entry: NetworkAclEntry,
    ) -> Result<(), Ec2Error> {
        let nacl = self
            .network_acls
            .get_mut(nacl_id)
            .ok_or_else(|| Ec2Error::NetworkAclNotFound(nacl_id.to_string()))?;
        nacl.entries.push(entry);
        Ok(())
    }

    pub fn delete_network_acl_entry(
        &mut self,
        nacl_id: &str,
        rule_number: i32,
        egress: bool,
    ) -> Result<(), Ec2Error> {
        let nacl = self
            .network_acls
            .get_mut(nacl_id)
            .ok_or_else(|| Ec2Error::NetworkAclNotFound(nacl_id.to_string()))?;
        nacl.entries
            .retain(|e| !(e.rule_number == rule_number && e.egress == egress));
        Ok(())
    }

    pub fn replace_network_acl_entry(
        &mut self,
        nacl_id: &str,
        entry: NetworkAclEntry,
    ) -> Result<(), Ec2Error> {
        let nacl = self
            .network_acls
            .get_mut(nacl_id)
            .ok_or_else(|| Ec2Error::NetworkAclNotFound(nacl_id.to_string()))?;
        nacl.entries
            .retain(|e| !(e.rule_number == entry.rule_number && e.egress == entry.egress));
        nacl.entries.push(entry);
        Ok(())
    }

    pub fn replace_network_acl_association(
        &mut self,
        assoc_id: &str,
        new_nacl_id: &str,
    ) -> Result<String, Ec2Error> {
        if !self.network_acls.contains_key(new_nacl_id) {
            return Err(Ec2Error::NetworkAclNotFound(new_nacl_id.to_string()));
        }
        // Find and remove the association from its current nacl
        let mut found_subnet_id: Option<String> = None;
        for nacl in self.network_acls.values_mut() {
            if let Some(pos) = nacl
                .associations
                .iter()
                .position(|a| a.network_acl_association_id == assoc_id)
            {
                found_subnet_id = Some(nacl.associations[pos].subnet_id.clone());
                nacl.associations.remove(pos);
                break;
            }
        }
        let subnet_id =
            found_subnet_id.ok_or_else(|| Ec2Error::AssociationNotFound(assoc_id.to_string()))?;
        let new_assoc_id = self.next_nacl_assoc_id();
        let new_nacl = self.network_acls.get_mut(new_nacl_id).unwrap();
        new_nacl.associations.push(NetworkAclAssociation {
            network_acl_association_id: new_assoc_id.clone(),
            network_acl_id: new_nacl_id.to_string(),
            subnet_id,
        });
        Ok(new_assoc_id)
    }

    // --- Elastic IP operations ---

    pub fn allocate_address(&mut self, tags: Tags) -> &ElasticIp {
        let alloc_id = self.next_eip_id();
        let count = self.counters.eip;
        let eip = ElasticIp {
            allocation_id: alloc_id.clone(),
            public_ip: format!(
                "54.{}.{}.{}",
                (count >> 16) & 0xff,
                (count >> 8) & 0xff,
                count & 0xff
            ),
            association_id: None,
            instance_id: None,
            network_interface_id: None,
            private_ip_address: None,
            address_attribute_ptr_record: None,
            domain: "vpc".to_string(),
            pending_transfer: None,
            tags,
        };
        self.elastic_ips.insert(alloc_id.clone(), eip);
        self.elastic_ips.get(&alloc_id).unwrap()
    }

    pub fn release_address(&mut self, allocation_id: &str) -> Result<(), Ec2Error> {
        if self.elastic_ips.remove(allocation_id).is_none() {
            return Err(Ec2Error::AllocationNotFound(allocation_id.to_string()));
        }
        Ok(())
    }

    pub fn associate_address(
        &mut self,
        allocation_id: &str,
        instance_id: Option<String>,
        network_interface_id: Option<String>,
        private_ip: Option<String>,
    ) -> Result<String, Ec2Error> {
        if !self.elastic_ips.contains_key(allocation_id) {
            return Err(Ec2Error::AllocationNotFound(allocation_id.to_string()));
        }
        self.counters.eip_assoc += 1;
        let assoc_id = format!("eipassoc-{:08x}", self.counters.eip_assoc);
        let eip = self.elastic_ips.get_mut(allocation_id).unwrap();
        eip.association_id = Some(assoc_id.clone());
        eip.instance_id = instance_id;
        eip.network_interface_id = network_interface_id;
        eip.private_ip_address = private_ip;
        Ok(assoc_id)
    }

    pub fn disassociate_address(&mut self, association_id: &str) -> Result<(), Ec2Error> {
        for eip in self.elastic_ips.values_mut() {
            if eip.association_id.as_deref() == Some(association_id) {
                eip.association_id = None;
                eip.instance_id = None;
                eip.network_interface_id = None;
                eip.private_ip_address = None;
                return Ok(());
            }
        }
        Err(Ec2Error::AssociationIdNotFound(association_id.to_string()))
    }

    // --- NAT Gateway operations ---

    pub fn create_nat_gateway(
        &mut self,
        subnet_id: &str,
        connectivity_type: &str,
        allocation_id: Option<String>,
        tags: Tags,
    ) -> Result<&NatGateway, Ec2Error> {
        let subnet = self
            .subnets
            .get(subnet_id)
            .ok_or_else(|| Ec2Error::SubnetNotFound(subnet_id.to_string()))?;
        let vpc_id = subnet.vpc_id.clone();
        let nat_id = self.next_nat_id();
        let public_ip = if connectivity_type == "public" {
            allocation_id
                .as_ref()
                .and_then(|id| self.elastic_ips.get(id).map(|e| e.public_ip.clone()))
        } else {
            None
        };
        let nat = NatGateway {
            nat_gateway_id: nat_id.clone(),
            vpc_id,
            subnet_id: subnet_id.to_string(),
            state: "available".to_string(),
            connectivity_type: connectivity_type.to_string(),
            allocation_id,
            public_ip,
            secondary_addresses: Vec::new(),
            tags,
        };
        self.nat_gateways.insert(nat_id.clone(), nat);
        Ok(self.nat_gateways.get(&nat_id).unwrap())
    }

    pub fn delete_nat_gateway(&mut self, nat_id: &str) -> Result<(), Ec2Error> {
        let nat = self
            .nat_gateways
            .get_mut(nat_id)
            .ok_or_else(|| Ec2Error::NatGatewayNotFound(nat_id.to_string()))?;
        nat.state = "deleted".to_string();
        Ok(())
    }

    // --- DHCP Options operations ---

    pub fn create_dhcp_options(
        &mut self,
        configurations: Vec<DhcpConfiguration>,
        tags: Tags,
    ) -> &DhcpOptions {
        let dopt_id = self.next_dopt_id();
        let dopt = DhcpOptions {
            dhcp_options_id: dopt_id.clone(),
            configurations,
            tags,
        };
        self.dhcp_options.insert(dopt_id.clone(), dopt);
        self.dhcp_options.get(&dopt_id).unwrap()
    }

    pub fn delete_dhcp_options(&mut self, dopt_id: &str) -> Result<(), Ec2Error> {
        // Check it's not associated with any VPC
        for vpc in self.vpcs.values() {
            if vpc.dhcp_options_id == dopt_id {
                return Err(Ec2Error::DhcpOptionsAssociatedWithVpc);
            }
        }
        if self.dhcp_options.remove(dopt_id).is_none() {
            return Err(Ec2Error::DhcpOptionsNotFound(dopt_id.to_string()));
        }
        Ok(())
    }

    pub fn associate_dhcp_options(&mut self, vpc_id: &str, dopt_id: &str) -> Result<(), Ec2Error> {
        if dopt_id != "default" && !self.dhcp_options.contains_key(dopt_id) {
            return Err(Ec2Error::DhcpOptionsNotFound(dopt_id.to_string()));
        }
        let vpc = self
            .vpcs
            .get_mut(vpc_id)
            .ok_or_else(|| Ec2Error::VpcNotFound(vpc_id.to_string()))?;
        vpc.dhcp_options_id = dopt_id.to_string();
        Ok(())
    }

    pub fn create_vpc(
        &mut self,
        cidr_block: &str,
        instance_tenancy: &str,
        tags: Tags,
    ) -> Result<&Vpc, Ec2Error> {
        let vpc_id = self.next_vpc_id();
        let dhcp_options_id = format!("dopt-{:08x}", self.counters.vpc);
        let vpc = Vpc {
            vpc_id: vpc_id.clone(),
            cidr_block: cidr_block.to_string(),
            state: "available".to_string(),
            dhcp_options_id,
            instance_tenancy: instance_tenancy.to_string(),
            is_default: false,
            enable_dns_hostnames: false,
            enable_dns_support: true,
            secondary_cidr_blocks: Vec::new(),
            tags,
            classic_link_enabled: false,
        };
        self.vpcs.insert(vpc_id.clone(), vpc);
        // Create default NetworkAcl for this VPC
        let nacl_id = self.next_nacl_id();
        let default_nacl = NetworkAcl {
            network_acl_id: nacl_id.clone(),
            vpc_id: vpc_id.clone(),
            is_default: true,
            entries: vec![
                // Allow-all inbound (rule 100)
                NetworkAclEntry {
                    rule_number: 100,
                    protocol: "-1".to_string(),
                    rule_action: "allow".to_string(),
                    egress: false,
                    cidr_block: Some("0.0.0.0/0".to_string()),
                    ipv6_cidr_block: None,
                    port_range: None,
                    icmp_type_code: None,
                },
                // Default deny-all inbound (32767)
                NetworkAclEntry {
                    rule_number: 32767,
                    protocol: "-1".to_string(),
                    rule_action: "deny".to_string(),
                    egress: false,
                    cidr_block: Some("0.0.0.0/0".to_string()),
                    ipv6_cidr_block: None,
                    port_range: None,
                    icmp_type_code: None,
                },
                // Allow-all outbound (rule 100)
                NetworkAclEntry {
                    rule_number: 100,
                    protocol: "-1".to_string(),
                    rule_action: "allow".to_string(),
                    egress: true,
                    cidr_block: Some("0.0.0.0/0".to_string()),
                    ipv6_cidr_block: None,
                    port_range: None,
                    icmp_type_code: None,
                },
                // Default deny-all outbound (32767)
                NetworkAclEntry {
                    rule_number: 32767,
                    protocol: "-1".to_string(),
                    rule_action: "deny".to_string(),
                    egress: true,
                    cidr_block: Some("0.0.0.0/0".to_string()),
                    ipv6_cidr_block: None,
                    port_range: None,
                    icmp_type_code: None,
                },
            ],
            associations: Vec::new(),
            tags: Tags::new(),
        };
        self.network_acls.insert(nacl_id, default_nacl);
        Ok(self.vpcs.get(&vpc_id).unwrap())
    }

    pub fn delete_vpc(&mut self, vpc_id: &str) -> Result<(), Ec2Error> {
        if self.vpcs.remove(vpc_id).is_none() {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        Ok(())
    }

    pub fn modify_vpc_attribute(
        &mut self,
        vpc_id: &str,
        attribute: &str,
        value: bool,
    ) -> Result<(), Ec2Error> {
        let vpc = self
            .vpcs
            .get_mut(vpc_id)
            .ok_or_else(|| Ec2Error::VpcNotFound(vpc_id.to_string()))?;
        match attribute {
            "enableDnsHostnames" => vpc.enable_dns_hostnames = value,
            "enableDnsSupport" => vpc.enable_dns_support = value,
            _ => {}
        }
        Ok(())
    }

    pub fn create_subnet(
        &mut self,
        vpc_id: &str,
        cidr_block: &str,
        availability_zone: &str,
        tags: Tags,
    ) -> Result<&Subnet, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let subnet_id = self.next_subnet_id();
        let subnet = Subnet {
            subnet_id: subnet_id.clone(),
            vpc_id: vpc_id.to_string(),
            cidr_block: cidr_block.to_string(),
            availability_zone: availability_zone.to_string(),
            state: "available".to_string(),
            available_ip_address_count: 251,
            map_public_ip_on_launch: false,
            ipv6_cidr_blocks: Vec::new(),
            tags,
        };
        self.subnets.insert(subnet_id.clone(), subnet);
        // Auto-associate with VPC's default NetworkAcl
        let nacl_id_opt = self
            .network_acls
            .values()
            .find(|nacl| nacl.vpc_id == vpc_id && nacl.is_default)
            .map(|nacl| nacl.network_acl_id.clone());
        if let Some(nacl_id) = nacl_id_opt {
            let assoc_id = self.next_nacl_assoc_id();
            if let Some(nacl) = self.network_acls.get_mut(&nacl_id) {
                nacl.associations.push(NetworkAclAssociation {
                    network_acl_association_id: assoc_id,
                    network_acl_id: nacl_id.clone(),
                    subnet_id: subnet_id.clone(),
                });
            }
        }
        Ok(self.subnets.get(&subnet_id).unwrap())
    }

    pub fn modify_subnet_attribute(
        &mut self,
        subnet_id: &str,
        map_public_ip_on_launch: bool,
    ) -> Result<(), Ec2Error> {
        let subnet = self
            .subnets
            .get_mut(subnet_id)
            .ok_or_else(|| Ec2Error::SubnetNotFound(subnet_id.to_string()))?;
        subnet.map_public_ip_on_launch = map_public_ip_on_launch;
        Ok(())
    }

    pub fn delete_subnet(&mut self, subnet_id: &str) -> Result<(), Ec2Error> {
        if self.subnets.remove(subnet_id).is_none() {
            return Err(Ec2Error::SubnetNotFound(subnet_id.to_string()));
        }
        Ok(())
    }

    pub fn create_internet_gateway(&mut self, tags: Tags) -> &InternetGateway {
        let igw_id = self.next_igw_id();
        let igw = InternetGateway {
            igw_id: igw_id.clone(),
            attachments: Vec::new(),
            tags,
        };
        self.igws.insert(igw_id.clone(), igw);
        self.igws.get(&igw_id).unwrap()
    }

    pub fn attach_internet_gateway(&mut self, igw_id: &str, vpc_id: &str) -> Result<(), Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let igw = self
            .igws
            .get_mut(igw_id)
            .ok_or_else(|| Ec2Error::InternetGatewayNotFound(igw_id.to_string()))?;
        igw.attachments.push(IgwAttachment {
            vpc_id: vpc_id.to_string(),
            state: "available".to_string(),
        });
        Ok(())
    }

    pub fn detach_internet_gateway(&mut self, igw_id: &str, vpc_id: &str) -> Result<(), Ec2Error> {
        let igw = self
            .igws
            .get_mut(igw_id)
            .ok_or_else(|| Ec2Error::InternetGatewayNotFound(igw_id.to_string()))?;
        igw.attachments.retain(|a| a.vpc_id != vpc_id);
        Ok(())
    }

    pub fn delete_internet_gateway(&mut self, igw_id: &str) -> Result<(), Ec2Error> {
        if self.igws.remove(igw_id).is_none() {
            return Err(Ec2Error::InternetGatewayNotFound(igw_id.to_string()));
        }
        Ok(())
    }

    pub fn create_security_group(
        &mut self,
        group_name: &str,
        description: &str,
        vpc_id: &str,
        owner_id: &str,
        tags: Tags,
    ) -> Result<&SecurityGroup, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let group_id = self.next_sg_id();
        let sg = SecurityGroup {
            group_id: group_id.clone(),
            group_name: group_name.to_string(),
            description: description.to_string(),
            vpc_id: vpc_id.to_string(),
            owner_id: owner_id.to_string(),
            ingress_rules: Vec::new(),
            egress_rules: vec![IpPermission {
                rule_id: self.next_sgr_id(),
                from_port: None,
                to_port: None,
                ip_protocol: "-1".to_string(),
                ip_ranges: vec![IpRange {
                    cidr_ip: "0.0.0.0/0".to_string(),
                    description: None,
                }],
                ipv6_ranges: Vec::new(),
                user_id_group_pairs: Vec::new(),
            }],
            tags,
        };
        self.security_groups.insert(group_id.clone(), sg);
        Ok(self.security_groups.get(&group_id).unwrap())
    }

    pub fn authorize_security_group_ingress(
        &mut self,
        group_id: &str,
        rules: Vec<IpPermission>,
    ) -> Result<(), Ec2Error> {
        let rules_with_ids: Vec<IpPermission> = rules
            .into_iter()
            .map(|mut r| {
                if r.rule_id.is_empty() {
                    r.rule_id = self.next_sgr_id();
                }
                r
            })
            .collect();
        let sg = self
            .security_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::SecurityGroupNotFound(group_id.to_string()))?;
        sg.ingress_rules.extend(rules_with_ids);
        Ok(())
    }

    pub fn authorize_security_group_egress(
        &mut self,
        group_id: &str,
        rules: Vec<IpPermission>,
    ) -> Result<(), Ec2Error> {
        let rules_with_ids: Vec<IpPermission> = rules
            .into_iter()
            .map(|mut r| {
                if r.rule_id.is_empty() {
                    r.rule_id = self.next_sgr_id();
                }
                r
            })
            .collect();
        let sg = self
            .security_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::SecurityGroupNotFound(group_id.to_string()))?;
        sg.egress_rules.extend(rules_with_ids);
        Ok(())
    }

    pub fn revoke_security_group_ingress(
        &mut self,
        group_id: &str,
        rules: &[IpPermission],
    ) -> Result<(), Ec2Error> {
        let sg = self
            .security_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::SecurityGroupNotFound(group_id.to_string()))?;
        for rule in rules {
            sg.ingress_rules.retain(|r| {
                r.ip_protocol != rule.ip_protocol
                    || r.from_port != rule.from_port
                    || r.to_port != rule.to_port
            });
        }
        Ok(())
    }

    pub fn revoke_security_group_egress(
        &mut self,
        group_id: &str,
        rules: &[IpPermission],
    ) -> Result<(), Ec2Error> {
        let sg = self
            .security_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::SecurityGroupNotFound(group_id.to_string()))?;
        for rule in rules {
            sg.egress_rules.retain(|r| {
                r.ip_protocol != rule.ip_protocol
                    || r.from_port != rule.from_port
                    || r.to_port != rule.to_port
            });
        }
        Ok(())
    }

    pub fn delete_security_group(&mut self, group_id: &str) -> Result<(), Ec2Error> {
        if self.security_groups.remove(group_id).is_none() {
            return Err(Ec2Error::SecurityGroupNotFound(group_id.to_string()));
        }
        Ok(())
    }

    pub fn create_route_table(
        &mut self,
        vpc_id: &str,
        tags: Tags,
    ) -> Result<&RouteTable, Ec2Error> {
        let vpc = self
            .vpcs
            .get(vpc_id)
            .ok_or_else(|| Ec2Error::VpcNotFound(vpc_id.to_string()))?;
        let local_cidr = vpc.cidr_block.clone();
        let rtb_id = self.next_rtb_id();
        let rtb = RouteTable {
            route_table_id: rtb_id.clone(),
            vpc_id: vpc_id.to_string(),
            routes: vec![Route {
                destination_cidr_block: Some(local_cidr),
                destination_ipv6_cidr_block: None,
                gateway_id: Some("local".to_string()),
                state: "active".to_string(),
                origin: "CreateRouteTable".to_string(),
            }],
            associations: Vec::new(),
            propagating_vgws: Vec::new(),
            tags,
        };
        self.route_tables.insert(rtb_id.clone(), rtb);
        Ok(self.route_tables.get(&rtb_id).unwrap())
    }

    pub fn associate_route_table(
        &mut self,
        rtb_id: &str,
        subnet_id: &str,
    ) -> Result<String, Ec2Error> {
        if !self.subnets.contains_key(subnet_id) {
            return Err(Ec2Error::SubnetNotFound(subnet_id.to_string()));
        }
        let assoc_id = self.next_rtbassoc_id();
        let rtb = self
            .route_tables
            .get_mut(rtb_id)
            .ok_or_else(|| Ec2Error::RouteTableNotFound(rtb_id.to_string()))?;
        rtb.associations.push(RouteTableAssociation {
            association_id: assoc_id.clone(),
            subnet_id: Some(subnet_id.to_string()),
            gateway_id: None,
            main: false,
            state: "associated".to_string(),
        });
        Ok(assoc_id)
    }

    pub fn disassociate_route_table(&mut self, assoc_id: &str) -> Result<(), Ec2Error> {
        for rtb in self.route_tables.values_mut() {
            let before = rtb.associations.len();
            rtb.associations.retain(|a| a.association_id != assoc_id);
            if rtb.associations.len() < before {
                return Ok(());
            }
        }
        Err(Ec2Error::AssociationIdNotFound(assoc_id.to_string()))
    }

    pub fn create_route(
        &mut self,
        rtb_id: &str,
        destination_cidr: Option<String>,
        destination_ipv6_cidr: Option<String>,
        gateway_id: Option<String>,
    ) -> Result<(), Ec2Error> {
        let rtb = self
            .route_tables
            .get_mut(rtb_id)
            .ok_or_else(|| Ec2Error::RouteTableNotFound(rtb_id.to_string()))?;
        rtb.routes.push(Route {
            destination_cidr_block: destination_cidr,
            destination_ipv6_cidr_block: destination_ipv6_cidr,
            gateway_id,
            state: "active".to_string(),
            origin: "CreateRoute".to_string(),
        });
        Ok(())
    }

    pub fn delete_route(&mut self, rtb_id: &str, destination_cidr: &str) -> Result<(), Ec2Error> {
        let rtb = self
            .route_tables
            .get_mut(rtb_id)
            .ok_or_else(|| Ec2Error::RouteTableNotFound(rtb_id.to_string()))?;
        rtb.routes.retain(|r| {
            r.destination_cidr_block.as_deref() != Some(destination_cidr)
                && r.destination_ipv6_cidr_block.as_deref() != Some(destination_cidr)
        });
        Ok(())
    }

    pub fn delete_route_table(&mut self, rtb_id: &str) -> Result<(), Ec2Error> {
        if self.route_tables.remove(rtb_id).is_none() {
            return Err(Ec2Error::RouteTableNotFound(rtb_id.to_string()));
        }
        Ok(())
    }

    pub fn import_key_pair(
        &mut self,
        key_name: &str,
        _public_key_material: &str,
        tags: Tags,
    ) -> &KeyPair {
        let key_pair_id = self.next_keypair_id();
        let kp = KeyPair {
            key_pair_id: key_pair_id.clone(),
            key_name: key_name.to_string(),
            fingerprint: "aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99".to_string(),
            tags,
        };
        self.key_pairs.insert(key_name.to_string(), kp);
        self.key_pairs.get(key_name).unwrap()
    }

    pub fn delete_key_pair(
        &mut self,
        key_name: Option<&str>,
        key_pair_id: Option<&str>,
    ) -> Result<(), Ec2Error> {
        if let Some(name) = key_name {
            self.key_pairs.remove(name);
            return Ok(());
        }
        if let Some(id) = key_pair_id {
            let name = self
                .key_pairs
                .iter()
                .find(|(_, kp)| kp.key_pair_id == id)
                .map(|(k, _)| k.clone());
            if let Some(n) = name {
                self.key_pairs.remove(&n);
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn create_tags(&mut self, resource_ids: &[String], tags: &Tags) {
        for id in resource_ids {
            if let Some(r) = self.vpcs.get_mut(id.as_str()) {
                r.tags.extend(tags.clone());
            } else if let Some(r) = self.subnets.get_mut(id.as_str()) {
                r.tags.extend(tags.clone());
            } else if let Some(r) = self.igws.get_mut(id.as_str()) {
                r.tags.extend(tags.clone());
            } else if let Some(r) = self.security_groups.get_mut(id.as_str()) {
                r.tags.extend(tags.clone());
            } else if let Some(r) = self.route_tables.get_mut(id.as_str()) {
                r.tags.extend(tags.clone());
            } else {
                // Try key pairs by key_pair_id
                let name = self
                    .key_pairs
                    .iter()
                    .find(|(_, kp)| kp.key_pair_id == *id)
                    .map(|(k, _)| k.clone());
                if let Some(n) = name {
                    if let Some(kp) = self.key_pairs.get_mut(&n) {
                        kp.tags.extend(tags.clone());
                    }
                }
            }
        }
    }

    pub fn delete_tags(&mut self, resource_ids: &[String], tag_keys: &[String]) {
        for id in resource_ids {
            let remove = |tags: &mut Tags| {
                for k in tag_keys {
                    tags.remove(k);
                }
            };
            if let Some(r) = self.vpcs.get_mut(id.as_str()) {
                remove(&mut r.tags);
            } else if let Some(r) = self.subnets.get_mut(id.as_str()) {
                remove(&mut r.tags);
            } else if let Some(r) = self.igws.get_mut(id.as_str()) {
                remove(&mut r.tags);
            } else if let Some(r) = self.security_groups.get_mut(id.as_str()) {
                remove(&mut r.tags);
            } else if let Some(r) = self.route_tables.get_mut(id.as_str()) {
                remove(&mut r.tags);
            }
        }
    }

    // --- Route table extended operations ---

    pub fn replace_route(
        &mut self,
        rtb_id: &str,
        destination_cidr: &str,
        gateway_id: Option<String>,
    ) -> Result<(), Ec2Error> {
        let rtb = self
            .route_tables
            .get_mut(rtb_id)
            .ok_or_else(|| Ec2Error::RouteTableNotFound(rtb_id.to_string()))?;
        for route in rtb.routes.iter_mut() {
            if route.destination_cidr_block.as_deref() == Some(destination_cidr)
                || route.destination_ipv6_cidr_block.as_deref() == Some(destination_cidr)
            {
                route.gateway_id = gateway_id;
                return Ok(());
            }
        }
        Err(Ec2Error::RouteNotFound(destination_cidr.to_string()))
    }

    pub fn replace_route_table_association(
        &mut self,
        assoc_id: &str,
        new_rtb_id: &str,
    ) -> Result<String, Ec2Error> {
        if !self.route_tables.contains_key(new_rtb_id) {
            return Err(Ec2Error::RouteTableNotFound(new_rtb_id.to_string()));
        }
        let mut found_assoc: Option<RouteTableAssociation> = None;
        for rtb in self.route_tables.values_mut() {
            if let Some(pos) = rtb
                .associations
                .iter()
                .position(|a| a.association_id == assoc_id)
            {
                found_assoc = Some(rtb.associations.remove(pos));
                break;
            }
        }
        let old_assoc =
            found_assoc.ok_or_else(|| Ec2Error::AssociationIdNotFound(assoc_id.to_string()))?;
        let new_assoc_id = self.next_rtbassoc_id();
        let rtb = self.route_tables.get_mut(new_rtb_id).unwrap();
        rtb.associations.push(RouteTableAssociation {
            association_id: new_assoc_id.clone(),
            subnet_id: old_assoc.subnet_id,
            gateway_id: old_assoc.gateway_id,
            main: old_assoc.main,
            state: "associated".to_string(),
        });
        Ok(new_assoc_id)
    }

    // --- CreateKeyPair ---

    pub fn create_key_pair(&mut self, key_name: &str, tags: Tags) -> &KeyPair {
        let key_pair_id = self.next_keypair_id();
        let kp = KeyPair {
            key_pair_id: key_pair_id.clone(),
            key_name: key_name.to_string(),
            fingerprint: "aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99".to_string(),
            tags,
        };
        self.key_pairs.insert(key_name.to_string(), kp);
        self.key_pairs.get(key_name).unwrap()
    }

    // --- Egress-only internet gateway operations ---

    fn next_eigw_id(&mut self) -> String {
        self.counters.eigw += 1;
        format!("eigw-{:08x}", self.counters.eigw)
    }

    pub fn create_egress_only_igw(
        &mut self,
        vpc_id: &str,
        tags: Tags,
    ) -> Result<&EgressOnlyInternetGateway, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let eigw_id = self.next_eigw_id();
        let eigw = EgressOnlyInternetGateway {
            eigw_id: eigw_id.clone(),
            state: "available".to_string(),
            attachments: vec![EoigwAttachment {
                vpc_id: vpc_id.to_string(),
                state: "attached".to_string(),
            }],
            tags,
        };
        self.egress_only_igws.insert(eigw_id.clone(), eigw);
        Ok(self.egress_only_igws.get(&eigw_id).unwrap())
    }

    pub fn delete_egress_only_igw(&mut self, eigw_id: &str) -> Result<(), Ec2Error> {
        if self.egress_only_igws.remove(eigw_id).is_none() {
            return Err(Ec2Error::EgressOnlyIgwNotFound(eigw_id.to_string()));
        }
        Ok(())
    }

    // --- Flow log operations ---

    fn next_flow_log_id(&mut self) -> String {
        self.counters.flow_log += 1;
        format!("fl-{:08x}", self.counters.flow_log)
    }

    pub fn create_flow_log(
        &mut self,
        resource_id: &str,
        traffic_type: &str,
        log_destination_type: &str,
        log_destination: Option<String>,
        log_group_name: Option<String>,
        tags: Tags,
    ) -> String {
        let flow_log_id = self.next_flow_log_id();
        let fl = FlowLog {
            flow_log_id: flow_log_id.clone(),
            resource_id: resource_id.to_string(),
            traffic_type: traffic_type.to_string(),
            log_destination_type: log_destination_type.to_string(),
            log_destination,
            log_group_name,
            deliver_logs_status: "SUCCESS".to_string(),
            flow_log_status: "ACTIVE".to_string(),
            tags,
        };
        self.flow_logs.insert(flow_log_id.clone(), fl);
        flow_log_id
    }

    pub fn delete_flow_logs(&mut self, flow_log_ids: &[String]) {
        for id in flow_log_ids {
            self.flow_logs.remove(id);
        }
    }

    // --- VPC peering operations ---

    fn next_peering_id(&mut self) -> String {
        self.counters.vpc_peering += 1;
        format!("pcx-{:08x}", self.counters.vpc_peering)
    }

    pub fn create_vpc_peering_connection(
        &mut self,
        requester_vpc_id: &str,
        accepter_vpc_id: &str,
        tags: Tags,
    ) -> &VpcPeeringConnection {
        let peering_id = self.next_peering_id();
        let conn = VpcPeeringConnection {
            peering_id: peering_id.clone(),
            requester_vpc_id: requester_vpc_id.to_string(),
            accepter_vpc_id: Some(accepter_vpc_id.to_string()),
            status: "pending-acceptance".to_string(),
            tags,
            requester_peering_options: None,
            accepter_peering_options: None,
        };
        self.vpc_peering_connections
            .insert(peering_id.clone(), conn);
        self.vpc_peering_connections.get(&peering_id).unwrap()
    }

    pub fn accept_vpc_peering_connection(&mut self, peering_id: &str) -> Result<(), Ec2Error> {
        let conn = self
            .vpc_peering_connections
            .get_mut(peering_id)
            .ok_or_else(|| Ec2Error::VpcPeeringConnectionNotFound(peering_id.to_string()))?;
        conn.status = "active".to_string();
        Ok(())
    }

    pub fn reject_vpc_peering_connection(&mut self, peering_id: &str) -> Result<(), Ec2Error> {
        let conn = self
            .vpc_peering_connections
            .get_mut(peering_id)
            .ok_or_else(|| Ec2Error::VpcPeeringConnectionNotFound(peering_id.to_string()))?;
        conn.status = "rejected".to_string();
        Ok(())
    }

    pub fn delete_vpc_peering_connection(&mut self, peering_id: &str) -> Result<(), Ec2Error> {
        let conn = self
            .vpc_peering_connections
            .get_mut(peering_id)
            .ok_or_else(|| Ec2Error::VpcPeeringConnectionNotFound(peering_id.to_string()))?;
        conn.status = "deleted".to_string();
        Ok(())
    }

    // --- VPC endpoint operations ---

    fn next_endpoint_id(&mut self) -> String {
        self.counters.vpc_endpoint += 1;
        format!("vpce-{:08x}", self.counters.vpc_endpoint)
    }

    pub fn create_vpc_endpoint(
        &mut self,
        vpc_id: &str,
        service_name: &str,
        endpoint_type: &str,
        policy_document: Option<String>,
        route_table_ids: Vec<String>,
        subnet_ids: Vec<String>,
        security_group_ids: Vec<String>,
        tags: Tags,
    ) -> Result<&VpcEndpoint, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let endpoint_id = self.next_endpoint_id();
        // If the endpoint targets a known endpoint-service configuration,
        // seed a connection record so Accept/Reject can transition it. The
        // initial state depends on whether the service requires acceptance.
        let matched_service: Option<(String, bool)> = self
            .vpc_endpoint_service_configs
            .values()
            .find(|s| s.service_name == service_name)
            .map(|s| (s.service_id.clone(), s.acceptance_required));
        let ep = VpcEndpoint {
            endpoint_id: endpoint_id.clone(),
            vpc_id: vpc_id.to_string(),
            service_name: service_name.to_string(),
            endpoint_type: endpoint_type.to_string(),
            state: "available".to_string(),
            policy_document,
            route_table_ids,
            subnet_ids,
            security_group_ids,
            private_dns_enabled: None,
            tags,
        };
        self.vpc_endpoints.insert(endpoint_id.clone(), ep);
        if let Some((svc_id, acceptance_required)) = matched_service {
            let conn_state = if acceptance_required {
                "pendingAcceptance"
            } else {
                "available"
            };
            self.upsert_vpc_endpoint_connection(&svc_id, &endpoint_id, "000000000000", conn_state);
        }
        Ok(self.vpc_endpoints.get(&endpoint_id).unwrap())
    }

    pub fn delete_vpc_endpoints(&mut self, endpoint_ids: &[String]) {
        for id in endpoint_ids {
            self.vpc_endpoints.remove(id);
        }
    }

    // --- Managed prefix list operations ---

    fn next_prefix_list_id(&mut self) -> String {
        self.counters.prefix_list += 1;
        format!("pl-{:08x}", self.counters.prefix_list)
    }

    pub fn create_managed_prefix_list(
        &mut self,
        name: &str,
        max_entries: i32,
        address_family: &str,
        entries: Vec<TypesPrefixListEntry>,
        tags: Tags,
    ) -> &ManagedPrefixList {
        let id = self.next_prefix_list_id();
        let pl = ManagedPrefixList {
            prefix_list_id: id.clone(),
            prefix_list_name: name.to_string(),
            state: "create-complete".to_string(),
            address_family: address_family.to_string(),
            max_entries,
            entries: entries.clone(),
            tags,
            version: 1,
            version_history: vec![ManagedPrefixListVersion {
                version: 1,
                entries,
            }],
        };
        self.managed_prefix_lists.insert(id.clone(), pl);
        self.managed_prefix_lists.get(&id).unwrap()
    }

    pub fn delete_managed_prefix_list(&mut self, id: &str) -> Result<&ManagedPrefixList, Ec2Error> {
        let pl = self
            .managed_prefix_lists
            .get_mut(id)
            .ok_or_else(|| Ec2Error::PrefixListNotFound(id.to_string()))?;
        pl.state = "delete-complete".to_string();
        Ok(self.managed_prefix_lists.get(id).unwrap())
    }

    pub fn modify_managed_prefix_list(
        &mut self,
        id: &str,
        add_entries: Vec<TypesPrefixListEntry>,
        remove_cidrs: Vec<String>,
    ) -> Result<(), Ec2Error> {
        let pl = self
            .managed_prefix_lists
            .get_mut(id)
            .ok_or_else(|| Ec2Error::PrefixListNotFound(id.to_string()))?;
        pl.entries.retain(|e| !remove_cidrs.contains(&e.cidr));
        pl.entries.extend(add_entries);
        pl.version += 1;
        pl.version_history.push(ManagedPrefixListVersion {
            version: pl.version,
            entries: pl.entries.clone(),
        });
        Ok(())
    }

    // --- Customer gateway operations ---

    fn next_cgw_id(&mut self) -> String {
        self.counters.cgw += 1;
        format!("cgw-{:08x}", self.counters.cgw)
    }

    pub fn create_customer_gateway(
        &mut self,
        bgp_asn: &str,
        ip_address: &str,
        gateway_type: &str,
        tags: Tags,
    ) -> &CustomerGateway {
        let id = self.next_cgw_id();
        let cgw = CustomerGateway {
            customer_gateway_id: id.clone(),
            bgp_asn: bgp_asn.to_string(),
            ip_address: ip_address.to_string(),
            gateway_type: gateway_type.to_string(),
            state: "available".to_string(),
            tags,
        };
        self.customer_gateways.insert(id.clone(), cgw);
        self.customer_gateways.get(&id).unwrap()
    }

    pub fn delete_customer_gateway(&mut self, id: &str) -> Result<(), Ec2Error> {
        if self.customer_gateways.remove(id).is_none() {
            return Err(Ec2Error::CustomerGatewayNotFound(id.to_string()));
        }
        Ok(())
    }

    // --- VPN gateway operations ---

    fn next_vgw_id(&mut self) -> String {
        self.counters.vgw += 1;
        format!("vgw-{:08x}", self.counters.vgw)
    }

    pub fn create_vpn_gateway(
        &mut self,
        gateway_type: &str,
        amazon_side_asn: Option<i64>,
        tags: Tags,
    ) -> &VpnGateway {
        let id = self.next_vgw_id();
        let vgw = VpnGateway {
            vpn_gateway_id: id.clone(),
            gateway_type: gateway_type.to_string(),
            state: "available".to_string(),
            amazon_side_asn,
            vpc_attachments: Vec::new(),
            tags,
        };
        self.vpn_gateways.insert(id.clone(), vgw);
        self.vpn_gateways.get(&id).unwrap()
    }

    pub fn delete_vpn_gateway(&mut self, id: &str) -> Result<(), Ec2Error> {
        let vgw = self
            .vpn_gateways
            .get_mut(id)
            .ok_or_else(|| Ec2Error::VpnGatewayNotFound(id.to_string()))?;
        vgw.state = "deleted".to_string();
        Ok(())
    }

    pub fn attach_vpn_gateway(&mut self, vgw_id: &str, vpc_id: &str) -> Result<(), Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let vgw = self
            .vpn_gateways
            .get_mut(vgw_id)
            .ok_or_else(|| Ec2Error::VpnGatewayNotFound(vgw_id.to_string()))?;
        vgw.vpc_attachments.push(VgwVpcAttachment {
            vpc_id: vpc_id.to_string(),
            state: "attached".to_string(),
        });
        Ok(())
    }

    pub fn detach_vpn_gateway(&mut self, vgw_id: &str, vpc_id: &str) -> Result<(), Ec2Error> {
        let vgw = self
            .vpn_gateways
            .get_mut(vgw_id)
            .ok_or_else(|| Ec2Error::VpnGatewayNotFound(vgw_id.to_string()))?;
        vgw.vpc_attachments.retain(|a| a.vpc_id != vpc_id);
        Ok(())
    }

    // --- VPN connection operations ---

    fn next_vpn_id(&mut self) -> String {
        self.counters.vpn += 1;
        format!("vpn-{:08x}", self.counters.vpn)
    }

    pub fn create_vpn_connection(
        &mut self,
        vgw_id: &str,
        cgw_id: &str,
        connection_type: &str,
        tags: Tags,
    ) -> &VpnConnection {
        let id = self.next_vpn_id();
        let vpn = VpnConnection {
            vpn_connection_id: id.clone(),
            vpn_gateway_id: vgw_id.to_string(),
            customer_gateway_id: cgw_id.to_string(),
            transit_gateway_id: None,
            connection_type: connection_type.to_string(),
            state: "available".to_string(),
            tags,
            routes: Vec::new(),
            // Two pre-allocated tunnels per real VPN connection. We seed them
            // with deterministic outside IPs so `ModifyVpnTunnelOptions` and
            // `ModifyVpnTunnelCertificate` have something to address.
            options: Some(VpnConnectionOptions {
                tunnel_options: vec![
                    VpnTunnelOptions {
                        outside_ip_address: Some("203.0.113.1".to_string()),
                        ..Default::default()
                    },
                    VpnTunnelOptions {
                        outside_ip_address: Some("203.0.113.2".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            tunnel_replacement_status: None,
        };
        self.vpn_connections.insert(id.clone(), vpn);
        self.vpn_connections.get(&id).unwrap()
    }

    pub fn delete_vpn_connection(&mut self, id: &str) -> Result<(), Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(id.to_string()))?;
        vpn.state = "deleted".to_string();
        Ok(())
    }

    pub fn create_vpn_connection_route(
        &mut self,
        vpn_id: &str,
        destination_cidr: &str,
    ) -> Result<(), Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(vpn_id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(vpn_id.to_string()))?;
        // Idempotent: if the same CIDR is added twice, just keep one entry.
        if !vpn
            .routes
            .iter()
            .any(|r| r.destination_cidr_block == destination_cidr)
        {
            vpn.routes.push(VpnStaticRoute {
                destination_cidr_block: destination_cidr.to_string(),
                source: "Static".to_string(),
                state: "available".to_string(),
            });
        }
        Ok(())
    }

    pub fn delete_vpn_connection_route(
        &mut self,
        vpn_id: &str,
        destination_cidr: &str,
    ) -> Result<(), Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(vpn_id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(vpn_id.to_string()))?;
        let before = vpn.routes.len();
        vpn.routes
            .retain(|r| r.destination_cidr_block != destination_cidr);
        if vpn.routes.len() == before {
            return Err(Ec2Error::InvalidVpnConnectionRouteNotFound {
                vpn_connection_id: vpn_id.to_string(),
                destination_cidr: destination_cidr.to_string(),
            });
        }
        Ok(())
    }

    pub fn modify_vpn_connection(
        &mut self,
        vpn_id: &str,
        new_cgw: Option<String>,
        new_vgw: Option<String>,
        new_tgw: Option<String>,
    ) -> Result<&VpnConnection, Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(vpn_id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(vpn_id.to_string()))?;
        if let Some(c) = new_cgw {
            vpn.customer_gateway_id = c;
        }
        if let Some(v) = new_vgw {
            vpn.vpn_gateway_id = v;
        }
        if let Some(t) = new_tgw {
            vpn.transit_gateway_id = Some(t);
        }
        Ok(self.vpn_connections.get(vpn_id).unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_vpn_connection_options(
        &mut self,
        vpn_id: &str,
        local_ipv4: Option<String>,
        local_ipv6: Option<String>,
        remote_ipv4: Option<String>,
        remote_ipv6: Option<String>,
        tunnel_inside_ip_version: Option<String>,
    ) -> Result<&VpnConnection, Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(vpn_id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(vpn_id.to_string()))?;
        let opts = vpn
            .options
            .get_or_insert_with(VpnConnectionOptions::default);
        if let Some(v) = local_ipv4 {
            opts.local_ipv4_network_cidr = Some(v);
        }
        if let Some(v) = local_ipv6 {
            opts.local_ipv6_network_cidr = Some(v);
        }
        if let Some(v) = remote_ipv4 {
            opts.remote_ipv4_network_cidr = Some(v);
        }
        if let Some(v) = remote_ipv6 {
            opts.remote_ipv6_network_cidr = Some(v);
        }
        if let Some(v) = tunnel_inside_ip_version {
            opts.tunnel_inside_ip_version = Some(v);
        }
        Ok(self.vpn_connections.get(vpn_id).unwrap())
    }

    pub fn modify_vpn_tunnel_certificate(
        &mut self,
        vpn_id: &str,
        outside_ip: &str,
    ) -> Result<&VpnConnection, Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(vpn_id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(vpn_id.to_string()))?;
        let opts = vpn
            .options
            .get_or_insert_with(VpnConnectionOptions::default);
        let tunnel = opts
            .tunnel_options
            .iter_mut()
            .find(|t| t.outside_ip_address.as_deref() == Some(outside_ip))
            .ok_or_else(|| Ec2Error::InvalidVpnTunnelNotFound {
                vpn_connection_id: vpn_id.to_string(),
                outside_ip_address: outside_ip.to_string(),
            })?;
        tunnel.certificate_arn = Some(format!(
            "arn:aws:acm:us-east-1:000000000000:certificate/{}-{}",
            vpn_id, outside_ip
        ));
        Ok(self.vpn_connections.get(vpn_id).unwrap())
    }

    pub fn modify_vpn_tunnel_options(
        &mut self,
        vpn_id: &str,
        outside_ip: &str,
        tunnel_inside_cidr: Option<String>,
        tunnel_inside_ipv6_cidr: Option<String>,
        pre_shared_key: Option<String>,
    ) -> Result<&VpnConnection, Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(vpn_id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(vpn_id.to_string()))?;
        let opts = vpn
            .options
            .get_or_insert_with(VpnConnectionOptions::default);
        let tunnel = opts
            .tunnel_options
            .iter_mut()
            .find(|t| t.outside_ip_address.as_deref() == Some(outside_ip))
            .ok_or_else(|| Ec2Error::InvalidVpnTunnelNotFound {
                vpn_connection_id: vpn_id.to_string(),
                outside_ip_address: outside_ip.to_string(),
            })?;
        if let Some(v) = tunnel_inside_cidr {
            tunnel.tunnel_inside_cidr = Some(v);
        }
        if let Some(v) = tunnel_inside_ipv6_cidr {
            tunnel.tunnel_inside_ipv6_cidr = Some(v);
        }
        if let Some(v) = pre_shared_key {
            tunnel.pre_shared_key = Some(v);
        }
        Ok(self.vpn_connections.get(vpn_id).unwrap())
    }

    pub fn replace_vpn_tunnel(&mut self, vpn_id: &str, outside_ip: &str) -> Result<(), Ec2Error> {
        let vpn = self
            .vpn_connections
            .get_mut(vpn_id)
            .ok_or_else(|| Ec2Error::VpnConnectionNotFound(vpn_id.to_string()))?;
        // Verify the tunnel exists; we don't need to mutate it here.
        let exists = vpn
            .options
            .as_ref()
            .map(|o| {
                o.tunnel_options
                    .iter()
                    .any(|t| t.outside_ip_address.as_deref() == Some(outside_ip))
            })
            .unwrap_or(false);
        if !exists {
            return Err(Ec2Error::InvalidVpnTunnelNotFound {
                vpn_connection_id: vpn_id.to_string(),
                outside_ip_address: outside_ip.to_string(),
            });
        }
        vpn.tunnel_replacement_status = Some("pending".to_string());
        Ok(())
    }

    // --- VPN concentrator operations ---

    fn next_vpn_concentrator_id(&mut self) -> String {
        self.counters.vpn_concentrator += 1;
        format!("vpn-concentrator-{:08x}", self.counters.vpn_concentrator)
    }

    pub fn create_vpn_concentrator(
        &mut self,
        concentrator_type: &str,
        transit_gateway_id: Option<String>,
        tags: Tags,
    ) -> &VpnConcentrator {
        let id = self.next_vpn_concentrator_id();
        // For TGW-attached concentrators, fabricate a deterministic attachment
        // id so describe round-trips. Pure mock; real AWS would create the
        // attachment as a side-effect of CreateVpnConcentrator.
        let attach_id = transit_gateway_id
            .as_ref()
            .map(|tgw| format!("tgw-attach-vpnc-{}-{tgw}", &id[16..]));
        let vc = VpnConcentrator {
            vpn_concentrator_id: id.clone(),
            concentrator_type: concentrator_type.to_string(),
            state: "available".to_string(),
            transit_gateway_id,
            transit_gateway_attachment_id: attach_id,
            tags,
        };
        self.vpn_concentrators.insert(id.clone(), vc);
        self.vpn_concentrators.get(&id).unwrap()
    }

    pub fn delete_vpn_concentrator(&mut self, id: &str) -> Result<(), Ec2Error> {
        if self.vpn_concentrators.remove(id).is_none() {
            return Err(Ec2Error::InvalidVpnConcentratorNotFound(id.to_string()));
        }
        Ok(())
    }

    // --- Carrier gateway operations ---

    fn next_carrier_gw_id(&mut self) -> String {
        self.counters.cgw_carrier += 1;
        format!("cagw-{:08x}", self.counters.cgw_carrier)
    }

    pub fn create_carrier_gateway(
        &mut self,
        vpc_id: &str,
        tags: Tags,
    ) -> Result<&CarrierGateway, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let id = self.next_carrier_gw_id();
        let cgw = CarrierGateway {
            carrier_gateway_id: id.clone(),
            vpc_id: vpc_id.to_string(),
            state: "available".to_string(),
            tags,
        };
        self.carrier_gateways.insert(id.clone(), cgw);
        Ok(self.carrier_gateways.get(&id).unwrap())
    }

    pub fn delete_carrier_gateway(&mut self, id: &str) -> Result<(), Ec2Error> {
        let cgw = self
            .carrier_gateways
            .get_mut(id)
            .ok_or_else(|| Ec2Error::CarrierGatewayNotFound(id.to_string()))?;
        cgw.state = "deleted".to_string();
        Ok(())
    }

    // --- Network interface operations ---

    fn next_eni_id(&mut self) -> String {
        self.counters.eni += 1;
        format!("eni-{:08x}", self.counters.eni)
    }

    fn next_eni_attach_id(&mut self) -> String {
        self.counters.eni_attach += 1;
        format!("eni-attach-{:08x}", self.counters.eni_attach)
    }

    pub fn create_network_interface(
        &mut self,
        subnet_id: &str,
        description: &str,
        private_ip_address: &str,
        security_groups: Vec<String>,
        tags: Tags,
    ) -> Result<&NetworkInterface, Ec2Error> {
        let vpc_id = {
            let subnet = self
                .subnets
                .get(subnet_id)
                .ok_or_else(|| Ec2Error::SubnetNotFound(subnet_id.to_string()))?;
            subnet.vpc_id.clone()
        };
        let eni_id = self.next_eni_id();
        let eni = NetworkInterface {
            network_interface_id: eni_id.clone(),
            subnet_id: subnet_id.to_string(),
            vpc_id,
            description: description.to_string(),
            private_ip_address: if private_ip_address.is_empty() {
                "10.0.0.100".to_string()
            } else {
                private_ip_address.to_string()
            },
            status: "available".to_string(),
            attachment_id: None,
            instance_id: None,
            device_index: None,
            security_groups,
            source_dest_check: true,
            tags,
            public_ip_dns_hostname_type: None,
        };
        self.network_interfaces.insert(eni_id.clone(), eni);
        Ok(self.network_interfaces.get(&eni_id).unwrap())
    }

    pub fn delete_network_interface(&mut self, eni_id: &str) -> Result<(), Ec2Error> {
        if self.network_interfaces.remove(eni_id).is_none() {
            return Err(Ec2Error::NetworkInterfaceNotFound(eni_id.to_string()));
        }
        Ok(())
    }

    pub fn attach_network_interface(
        &mut self,
        eni_id: &str,
        instance_id: &str,
        device_index: i32,
    ) -> Result<String, Ec2Error> {
        let attach_id = self.next_eni_attach_id();
        let eni = self
            .network_interfaces
            .get_mut(eni_id)
            .ok_or_else(|| Ec2Error::NetworkInterfaceNotFound(eni_id.to_string()))?;
        eni.status = "in-use".to_string();
        eni.attachment_id = Some(attach_id.clone());
        eni.instance_id = Some(instance_id.to_string());
        eni.device_index = Some(device_index);
        Ok(attach_id)
    }

    pub fn detach_network_interface(&mut self, attachment_id: &str) -> Result<(), Ec2Error> {
        for eni in self.network_interfaces.values_mut() {
            if eni.attachment_id.as_deref() == Some(attachment_id) {
                eni.status = "available".to_string();
                eni.attachment_id = None;
                eni.instance_id = None;
                eni.device_index = None;
                return Ok(());
            }
        }
        Err(Ec2Error::AttachmentNotFound(attachment_id.to_string()))
    }

    // --- VPC CIDR block operations ---

    fn next_cidr_assoc_id(&mut self) -> String {
        self.counters.vpc_cidr_assoc += 1;
        format!("vpc-cidr-assoc-{:08x}", self.counters.vpc_cidr_assoc)
    }

    pub fn associate_vpc_cidr_block(
        &mut self,
        vpc_id: &str,
        cidr_block: &str,
    ) -> Result<String, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let assoc_id = self.next_cidr_assoc_id();
        let vpc = self.vpcs.get_mut(vpc_id).unwrap();
        vpc.secondary_cidr_blocks
            .push((assoc_id.clone(), cidr_block.to_string()));
        self.vpc_cidr_associations
            .insert(assoc_id.clone(), vpc_id.to_string());
        Ok(assoc_id)
    }

    pub fn disassociate_vpc_cidr_block(&mut self, assoc_id: &str) -> Result<String, Ec2Error> {
        let vpc_id = self
            .vpc_cidr_associations
            .remove(assoc_id)
            .ok_or_else(|| Ec2Error::VpcCidrBlockAssociationNotFound(assoc_id.to_string()))?;
        if let Some(vpc) = self.vpcs.get_mut(&vpc_id) {
            vpc.secondary_cidr_blocks.retain(|(id, _)| id != assoc_id);
        }
        Ok(vpc_id)
    }

    // --- Security group rule operations ---

    pub fn describe_security_group_rules(
        &self,
        group_id: Option<&str>,
        rule_ids: &[String],
    ) -> Vec<(&str, bool, &IpPermission)> {
        // Returns (group_id, is_egress, rule)
        let mut results = Vec::new();
        for sg in self.security_groups.values() {
            let matches_group = group_id.is_none_or(|gid| sg.group_id == gid);
            if !matches_group {
                continue;
            }
            for rule in &sg.ingress_rules {
                if rule_ids.is_empty() || rule_ids.contains(&rule.rule_id) {
                    results.push((sg.group_id.as_str(), false, rule));
                }
            }
            for rule in &sg.egress_rules {
                if rule_ids.is_empty() || rule_ids.contains(&rule.rule_id) {
                    results.push((sg.group_id.as_str(), true, rule));
                }
            }
        }
        results
    }

    pub fn update_sgr_description_ingress(
        &mut self,
        group_id: &str,
        rule_id: &str,
        description: Option<String>,
    ) -> Result<(), Ec2Error> {
        let sg = self
            .security_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::SecurityGroupNotFound(group_id.to_string()))?;
        for rule in sg.ingress_rules.iter_mut() {
            if rule.rule_id == rule_id {
                for range in rule.ip_ranges.iter_mut() {
                    range.description = description.clone();
                }
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn update_sgr_description_egress(
        &mut self,
        group_id: &str,
        rule_id: &str,
        description: Option<String>,
    ) -> Result<(), Ec2Error> {
        let sg = self
            .security_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::SecurityGroupNotFound(group_id.to_string()))?;
        for rule in sg.egress_rules.iter_mut() {
            if rule.rule_id == rule_id {
                for range in rule.ip_ranges.iter_mut() {
                    range.description = description.clone();
                }
                return Ok(());
            }
        }
        Ok(())
    }

    // --- Instance operations ---

    pub fn run_instances(
        &mut self,
        image_id: &str,
        instance_type: &str,
        min_count: i32,
        max_count: i32,
        key_name: Option<String>,
        subnet_id: Option<String>,
        security_group_ids: Vec<String>,
        tags: Tags,
        iam_instance_profile_arn: Option<String>,
        placement_az: &str,
        owner_id: &str,
    ) -> Vec<Instance> {
        let count = max_count.max(min_count).max(1) as usize;
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let vpc_id = subnet_id
            .as_deref()
            .and_then(|sid| self.subnets.get(sid).map(|s| s.vpc_id.clone()));
        let mut instances = Vec::new();
        for _ in 0..count {
            let instance_id = self.next_instance_id();
            let private_ip = Some(format!(
                "10.0.{}.{}",
                (self.counters.instance >> 8) & 0xff,
                self.counters.instance & 0xff
            ));
            let instance = Instance {
                instance_id: instance_id.clone(),
                image_id: image_id.to_string(),
                instance_type: instance_type.to_string(),
                state: InstanceState {
                    code: 16,
                    name: "running".to_string(),
                },
                private_ip_address: private_ip,
                public_ip_address: None,
                subnet_id: subnet_id.clone(),
                vpc_id: vpc_id.clone(),
                key_name: key_name.clone(),
                security_groups: security_group_ids.clone(),
                launch_time: now.clone(),
                tags: tags.clone(),
                iam_instance_profile_arn: iam_instance_profile_arn.clone(),
                monitoring_state: "disabled".to_string(),
                placement_az: placement_az.to_string(),
                placement_group_name: None,
                placement_tenancy: None,
                placement_host_id: None,
                placement_affinity: None,
                placement_partition_number: None,
                owner_id: owner_id.to_string(),
                classic_link_vpc: None,
                private_dns_hostname_type: None,
                enable_resource_name_dns_a_record: None,
                enable_resource_name_dns_aaaa_record: None,
                credit_specification: None,
                cpu_options: None,
                maintenance_options: None,
                network_bandwidth_weighting: None,
                lifecycle: None,
                product_codes: Vec::new(),
                capacity_reservation_specification: None,
            };
            self.instances.insert(instance_id, instance.clone());
            instances.push(instance);
        }
        instances
    }

    pub fn start_instances(
        &mut self,
        instance_ids: &[String],
    ) -> Vec<(String, i32, String, i32, String)> {
        // Returns (id, prev_code, prev_name, curr_code, curr_name)
        let mut changes = Vec::new();
        for id in instance_ids {
            if let Some(inst) = self.instances.get_mut(id) {
                let prev_code = inst.state.code;
                let prev_name = inst.state.name.clone();
                inst.state = InstanceState {
                    code: 16,
                    name: "running".to_string(),
                };
                changes.push((id.clone(), prev_code, prev_name, 16, "running".to_string()));
            }
        }
        changes
    }

    pub fn stop_instances(
        &mut self,
        instance_ids: &[String],
    ) -> Vec<(String, i32, String, i32, String)> {
        let mut changes = Vec::new();
        for id in instance_ids {
            if let Some(inst) = self.instances.get_mut(id) {
                let prev_code = inst.state.code;
                let prev_name = inst.state.name.clone();
                inst.state = InstanceState {
                    code: 80,
                    name: "stopped".to_string(),
                };
                changes.push((id.clone(), prev_code, prev_name, 80, "stopped".to_string()));
            }
        }
        changes
    }

    pub fn terminate_instances(
        &mut self,
        instance_ids: &[String],
    ) -> Vec<(String, i32, String, i32, String)> {
        let mut changes = Vec::new();
        for id in instance_ids {
            if let Some(inst) = self.instances.get_mut(id) {
                let prev_code = inst.state.code;
                let prev_name = inst.state.name.clone();
                inst.state = InstanceState {
                    code: 48,
                    name: "terminated".to_string(),
                };
                changes.push((
                    id.clone(),
                    prev_code,
                    prev_name,
                    48,
                    "terminated".to_string(),
                ));
            }
        }
        changes
    }

    // --- EBS Volume operations ---

    pub fn create_volume(
        &mut self,
        size: i32,
        availability_zone: &str,
        snapshot_id: Option<String>,
        volume_type: &str,
        iops: Option<i32>,
        throughput: Option<i32>,
        encrypted: bool,
        tags: Tags,
    ) -> &Volume {
        let vol_id = self.next_vol_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let vol = Volume {
            volume_id: vol_id.clone(),
            size,
            snapshot_id,
            availability_zone: availability_zone.to_string(),
            state: "available".to_string(),
            volume_type: volume_type.to_string(),
            iops,
            throughput,
            encrypted,
            create_time: now,
            attachments: Vec::new(),
            tags,
            recycle_bin_state: None,
            source_volume_id: None,
        };
        self.volumes.insert(vol_id.clone(), vol);
        self.volumes.get(&vol_id).unwrap()
    }

    pub fn delete_volume(&mut self, vol_id: &str) -> Result<(), Ec2Error> {
        // Mock policy: every deleted volume is moved into the Recycle Bin so
        // it can be restored via `RestoreVolumeFromRecycleBin`. The volume is
        // removed from `self.volumes` (so `DescribeVolumes` does not surface
        // it) and parked in `self.deleted_volumes_recycle_bin` keyed by ID.
        let mut vol = self
            .volumes
            .remove(vol_id)
            .ok_or_else(|| Ec2Error::VolumeNotFound(vol_id.to_string()))?;
        vol.state = "deleted-pending-recycle".to_string();
        vol.recycle_bin_state = Some("in-recycle-bin".to_string());
        self.deleted_volumes_recycle_bin
            .insert(vol_id.to_string(), vol);
        Ok(())
    }

    pub fn restore_volume_from_recycle_bin(&mut self, vol_id: &str) -> Result<(), Ec2Error> {
        let mut vol = self
            .deleted_volumes_recycle_bin
            .remove(vol_id)
            .ok_or_else(|| Ec2Error::VolumeNotInRecycleBin(vol_id.to_string()))?;
        vol.state = "available".to_string();
        vol.recycle_bin_state = None;
        self.volumes.insert(vol_id.to_string(), vol);
        Ok(())
    }

    pub fn copy_volumes(
        &mut self,
        source_volume_ids: &[String],
        tags: Tags,
    ) -> Result<Vec<String>, Ec2Error> {
        // Verify all sources exist first.
        for src in source_volume_ids {
            if !self.volumes.contains_key(src) {
                return Err(Ec2Error::VolumeNotFound(src.clone()));
            }
        }
        let mut new_ids = Vec::new();
        for src in source_volume_ids {
            // Snapshot the source's properties first so we can release the
            // immutable borrow before mutating the map.
            let (size, az, vol_type, iops, throughput, encrypted, snapshot_id) = {
                let v = self.volumes.get(src).unwrap();
                (
                    v.size,
                    v.availability_zone.clone(),
                    v.volume_type.clone(),
                    v.iops,
                    v.throughput,
                    v.encrypted,
                    v.snapshot_id.clone(),
                )
            };
            let new_id = self.next_vol_id();
            let now = chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S.000Z")
                .to_string();
            let new_vol = Volume {
                volume_id: new_id.clone(),
                size,
                snapshot_id,
                availability_zone: az,
                state: "available".to_string(),
                volume_type: vol_type,
                iops,
                throughput,
                encrypted,
                create_time: now,
                attachments: Vec::new(),
                tags: tags.clone(),
                recycle_bin_state: None,
                source_volume_id: Some(src.clone()),
            };
            self.volumes.insert(new_id.clone(), new_vol);
            new_ids.push(new_id);
        }
        Ok(new_ids)
    }

    pub fn attach_volume(
        &mut self,
        vol_id: &str,
        instance_id: &str,
        device: &str,
    ) -> Result<&VolumeAttachment, Ec2Error> {
        if !self.instances.contains_key(instance_id) {
            return Err(Ec2Error::InstanceNotFound(instance_id.to_string()));
        }
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let vol = self
            .volumes
            .get_mut(vol_id)
            .ok_or_else(|| Ec2Error::VolumeNotFound(vol_id.to_string()))?;
        vol.state = "in-use".to_string();
        vol.attachments.push(VolumeAttachment {
            volume_id: vol_id.to_string(),
            instance_id: instance_id.to_string(),
            device: device.to_string(),
            state: "attached".to_string(),
            attach_time: now,
            delete_on_termination: false,
        });
        Ok(vol.attachments.last().unwrap())
    }

    pub fn detach_volume(
        &mut self,
        vol_id: &str,
        instance_id: Option<&str>,
        device: Option<&str>,
    ) -> Result<VolumeAttachment, Ec2Error> {
        let vol = self
            .volumes
            .get_mut(vol_id)
            .ok_or_else(|| Ec2Error::VolumeNotFound(vol_id.to_string()))?;
        let pos = vol
            .attachments
            .iter()
            .position(|a| {
                instance_id.is_none_or(|id| a.instance_id == id)
                    && device.is_none_or(|d| a.device == d)
            })
            .ok_or(Ec2Error::VolumeNotAttached)?;
        let att = vol.attachments.remove(pos);
        if vol.attachments.is_empty() {
            vol.state = "available".to_string();
        }
        Ok(att)
    }

    // --- Snapshot operations ---

    pub fn create_snapshot(
        &mut self,
        vol_id: &str,
        description: &str,
        tags: Tags,
        owner_id: &str,
    ) -> Result<&Snapshot, Ec2Error> {
        let vol = self
            .volumes
            .get(vol_id)
            .ok_or_else(|| Ec2Error::VolumeNotFound(vol_id.to_string()))?;
        let vol_size = vol.size;
        let encrypted = vol.encrypted;
        let snap_id = self.next_snapshot_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let snap = Snapshot {
            snapshot_id: snap_id.clone(),
            volume_id: vol_id.to_string(),
            volume_size: vol_size,
            state: "completed".to_string(),
            description: description.to_string(),
            start_time: now,
            progress: "100%".to_string(),
            owner_id: owner_id.to_string(),
            encrypted,
            tags,
            lock_state: "none".to_string(),
            lock_duration: None,
            lock_created_on: None,
            lock_expires_on: None,
            lock_duration_start_time: None,
            cool_off_period: None,
            cool_off_period_expires_on: None,
            storage_tier: "standard".to_string(),
            last_tiering_operation_status: None,
            fast_snapshot_restore_states: Vec::new(),
        };
        self.snapshots.insert(snap_id.clone(), snap);
        Ok(self.snapshots.get(&snap_id).unwrap())
    }

    pub fn delete_snapshot(&mut self, snap_id: &str) -> Result<(), Ec2Error> {
        // Mock policy: deletion sends the snapshot to the recycle bin where it
        // can be restored via `RestoreSnapshotFromRecycleBin`.
        let snap = self
            .snapshots
            .get(snap_id)
            .ok_or_else(|| Ec2Error::SnapshotNotFound(snap_id.to_string()))?;
        if snap.lock_state == "compliance" || snap.lock_state == "governance" {
            return Err(Ec2Error::SnapshotIsLocked(snap_id.to_string()));
        }
        let mut snap = self.snapshots.remove(snap_id).unwrap();
        snap.state = "recycled".to_string();
        self.deleted_snapshots_recycle_bin
            .insert(snap_id.to_string(), snap);
        Ok(())
    }

    pub fn restore_snapshot_from_recycle_bin(
        &mut self,
        snap_id: &str,
    ) -> Result<&Snapshot, Ec2Error> {
        let mut snap = self
            .deleted_snapshots_recycle_bin
            .remove(snap_id)
            .ok_or_else(|| Ec2Error::SnapshotNotInRecycleBin(snap_id.to_string()))?;
        snap.state = "completed".to_string();
        self.snapshots.insert(snap_id.to_string(), snap);
        Ok(self.snapshots.get(snap_id).unwrap())
    }

    pub fn lock_snapshot(
        &mut self,
        snap_id: &str,
        lock_mode: &str,
        lock_duration: Option<i32>,
        cool_off_period: Option<i32>,
    ) -> Result<&Snapshot, Ec2Error> {
        let snap = self
            .snapshots
            .get_mut(snap_id)
            .ok_or_else(|| Ec2Error::SnapshotNotFound(snap_id.to_string()))?;
        // Compliance-locked snapshots cannot be re-locked or modified.
        if snap.lock_state == "compliance" {
            return Err(Ec2Error::SnapshotIsLocked(snap_id.to_string()));
        }
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        snap.lock_state = lock_mode.to_string();
        snap.lock_created_on = Some(now.clone());
        snap.lock_duration_start_time = Some(now.clone());
        snap.lock_duration = lock_duration;
        snap.lock_expires_on = lock_duration.map(|d| {
            let expires = chrono::Utc::now() + chrono::Duration::days(d as i64);
            expires.format("%Y-%m-%dT%H:%M:%S.000Z").to_string()
        });
        snap.cool_off_period = cool_off_period;
        snap.cool_off_period_expires_on = cool_off_period.map(|cp| {
            let expires = chrono::Utc::now() + chrono::Duration::hours(cp as i64);
            expires.format("%Y-%m-%dT%H:%M:%S.000Z").to_string()
        });
        Ok(self.snapshots.get(snap_id).unwrap())
    }

    pub fn unlock_snapshot(&mut self, snap_id: &str) -> Result<(), Ec2Error> {
        let snap = self
            .snapshots
            .get_mut(snap_id)
            .ok_or_else(|| Ec2Error::SnapshotNotFound(snap_id.to_string()))?;
        if snap.lock_state == "compliance" {
            return Err(Ec2Error::SnapshotIsLocked(snap_id.to_string()));
        }
        snap.lock_state = "none".to_string();
        snap.lock_duration = None;
        snap.lock_created_on = None;
        snap.lock_expires_on = None;
        snap.lock_duration_start_time = None;
        snap.cool_off_period = None;
        snap.cool_off_period_expires_on = None;
        Ok(())
    }

    pub fn modify_snapshot_tier(
        &mut self,
        snap_id: &str,
        storage_tier: &str,
    ) -> Result<String, Ec2Error> {
        let snap = self
            .snapshots
            .get_mut(snap_id)
            .ok_or_else(|| Ec2Error::SnapshotNotFound(snap_id.to_string()))?;
        snap.storage_tier = storage_tier.to_string();
        snap.last_tiering_operation_status = Some(format!("tiering-in-progress-to-{storage_tier}"));
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        Ok(now)
    }

    pub fn restore_snapshot_tier(&mut self, snap_id: &str) -> Result<String, Ec2Error> {
        let snap = self
            .snapshots
            .get_mut(snap_id)
            .ok_or_else(|| Ec2Error::SnapshotNotFound(snap_id.to_string()))?;
        snap.storage_tier = "standard".to_string();
        snap.last_tiering_operation_status = Some("tiering-finished".to_string());
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        Ok(now)
    }

    // --- AMI / Image operations ---

    pub fn create_image(
        &mut self,
        instance_id: &str,
        name: &str,
        description: &str,
        tags: Tags,
        owner_id: &str,
    ) -> Result<String, Ec2Error> {
        if !self.instances.contains_key(instance_id) {
            return Err(Ec2Error::InstanceNotFound(instance_id.to_string()));
        }
        let source_instance_type = self
            .instances
            .get(instance_id)
            .unwrap()
            .instance_type
            .clone();
        let image_id = self.next_ami_id();
        let img = Image {
            image_id: image_id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            state: "available".to_string(),
            owner_id: owner_id.to_string(),
            architecture: "x86_64".to_string(),
            image_type: "machine".to_string(),
            platform: None,
            virtualization_type: "hvm".to_string(),
            root_device_type: "ebs".to_string(),
            root_device_name: "/dev/xvda".to_string(),
            public: false,
            tags,
            source_instance_id: Some(instance_id.to_string()),
            source_instance_type,
            launch_permissions: Vec::new(),
            recycle_bin_state: None,
            deprecation_time: None,
            recycle_bin_enter_time: None,
            product_codes: Vec::new(),
            fast_launch_state: None,
            deregistration_protection: None,
            kernel_id: None,
            ramdisk_id: None,
            ena_support: None,
            sriov_net_support: None,
            tpm_support: None,
            boot_mode: None,
            imds_support: None,
            image_location: None,
            source_image_id: None,
            source_region: None,
        };
        self.images.insert(image_id.clone(), img);
        Ok(image_id)
    }

    pub fn modify_image_launch_permissions(
        &mut self,
        image_id: &str,
        add: &[crate::types::LaunchPermission],
        remove: &[crate::types::LaunchPermission],
    ) -> Result<(), Ec2Error> {
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        for perm in remove {
            img.launch_permissions.retain(|p| p != perm);
        }
        for perm in add {
            if !img.launch_permissions.contains(perm) {
                img.launch_permissions.push(perm.clone());
            }
        }
        Ok(())
    }

    pub fn modify_image_description(
        &mut self,
        image_id: &str,
        description: &str,
    ) -> Result<(), Ec2Error> {
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        img.description = description.to_string();
        Ok(())
    }

    pub fn get_image(&self, image_id: &str) -> Result<&crate::types::Image, Ec2Error> {
        self.images
            .get(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))
    }

    pub fn deregister_image(&mut self, image_id: &str) -> Result<(), Ec2Error> {
        // Move the image to the recycle bin instead of fully deleting it,
        // so RestoreImageFromRecycleBin can recover it.
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        img.recycle_bin_state = Some("recycled".to_string());
        img.state = "deregistered".to_string();
        self.deregistered_images.insert(image_id.to_string());
        Ok(())
    }

    // --- Launch Template operations ---

    pub fn create_launch_template(
        &mut self,
        name: &str,
        version_description: &str,
        data: std::collections::HashMap<String, String>,
        tags: Tags,
    ) -> Result<&LaunchTemplate, Ec2Error> {
        // Check for duplicate name
        if self
            .launch_templates
            .values()
            .any(|lt| lt.launch_template_name == name)
        {
            return Err(Ec2Error::LaunchTemplateAlreadyExists(name.to_string()));
        }
        let lt_id = self.next_lt_id();
        let lt = LaunchTemplate {
            launch_template_id: lt_id.clone(),
            launch_template_name: name.to_string(),
            default_version_number: 1,
            latest_version_number: 1,
            tags: tags.clone(),
        };
        self.launch_templates.insert(lt_id.clone(), lt);
        let version = LaunchTemplateVersion {
            version_number: 1,
            launch_template_id: lt_id.clone(),
            launch_template_name: name.to_string(),
            version_description: version_description.to_string(),
            data,
            default_version: true,
        };
        self.launch_template_versions
            .entry(lt_id.clone())
            .or_default()
            .push(version);
        Ok(self.launch_templates.get(&lt_id).unwrap())
    }

    pub fn delete_launch_template(
        &mut self,
        lt_id_or_name: &str,
    ) -> Result<LaunchTemplate, Ec2Error> {
        // Try by ID first, then by name
        let lt_id = if self.launch_templates.contains_key(lt_id_or_name) {
            lt_id_or_name.to_string()
        } else {
            self.launch_templates
                .values()
                .find(|lt| lt.launch_template_name == lt_id_or_name)
                .map(|lt| lt.launch_template_id.clone())
                .ok_or_else(|| Ec2Error::LaunchTemplateNotFound(lt_id_or_name.to_string()))?
        };
        self.launch_template_versions.remove(&lt_id);
        Ok(self.launch_templates.remove(&lt_id).unwrap())
    }

    // --- Launch Template Version operations ---

    pub fn create_launch_template_version(
        &mut self,
        lt_id: &str,
        description: &str,
        data: HashMap<String, String>,
    ) -> Result<i64, Ec2Error> {
        // Also accept by name
        let lt_id_resolved = if self.launch_templates.contains_key(lt_id) {
            lt_id.to_string()
        } else {
            self.launch_templates
                .values()
                .find(|lt| lt.launch_template_name == lt_id)
                .map(|lt| lt.launch_template_id.clone())
                .ok_or_else(|| Ec2Error::LaunchTemplateNotFound(lt_id.to_string()))?
        };
        let lt = self.launch_templates.get_mut(&lt_id_resolved).unwrap();
        lt.latest_version_number += 1;
        let version_number = lt.latest_version_number;
        let lt_name = lt.launch_template_name.clone();
        let version = LaunchTemplateVersion {
            version_number,
            launch_template_id: lt_id_resolved.clone(),
            launch_template_name: lt_name,
            version_description: description.to_string(),
            data,
            default_version: false,
        };
        self.launch_template_versions
            .entry(lt_id_resolved)
            .or_default()
            .push(version);
        Ok(version_number)
    }

    pub fn modify_launch_template(
        &mut self,
        lt_id: &str,
        default_version: Option<i64>,
    ) -> Result<(), Ec2Error> {
        let lt = self
            .launch_templates
            .get_mut(lt_id)
            .ok_or_else(|| Ec2Error::LaunchTemplateNotFound(lt_id.to_string()))?;
        if let Some(v) = default_version {
            lt.default_version_number = v;
            if let Some(versions) = self.launch_template_versions.get_mut(lt_id) {
                for ver in versions.iter_mut() {
                    ver.default_version = ver.version_number == v;
                }
            }
        }
        Ok(())
    }

    pub fn get_launch_template_data_from_instance(
        &self,
        instance_id: &str,
    ) -> HashMap<String, String> {
        let mut data = HashMap::new();
        if let Some(inst) = self.instances.get(instance_id) {
            data.insert(
                "LaunchTemplateData.ImageId".to_string(),
                inst.image_id.clone(),
            );
            data.insert(
                "LaunchTemplateData.InstanceType".to_string(),
                inst.instance_type.clone(),
            );
            if let Some(key) = &inst.key_name {
                data.insert("LaunchTemplateData.KeyName".to_string(), key.clone());
            }
        }
        data
    }

    // --- Transit Gateway operations ---

    pub fn create_transit_gateway(
        &mut self,
        description: &str,
        amazon_side_asn: i64,
        dns_support: &str,
        vpn_ecmp_support: &str,
        tags: Tags,
    ) -> &TransitGateway {
        let tgw_id = self.next_tgw_id();
        let tgw = TransitGateway {
            transit_gateway_id: tgw_id.clone(),
            state: "available".to_string(),
            amazon_side_asn,
            description: description.to_string(),
            dns_support: dns_support.to_string(),
            vpn_ecmp_support: vpn_ecmp_support.to_string(),
            multicast_support: "disable".to_string(),
            tags,
        };
        self.transit_gateways.insert(tgw_id.clone(), tgw);
        self.transit_gateways.get(&tgw_id).unwrap()
    }

    pub fn delete_transit_gateway(&mut self, tgw_id: &str) -> Result<(), Ec2Error> {
        let tgw = self
            .transit_gateways
            .get_mut(tgw_id)
            .ok_or_else(|| Ec2Error::TransitGatewayNotFound(tgw_id.to_string()))?;
        tgw.state = "deleted".to_string();
        Ok(())
    }

    pub fn create_transit_gateway_vpc_attachment(
        &mut self,
        tgw_id: &str,
        vpc_id: &str,
        subnet_ids: Vec<String>,
        tags: Tags,
    ) -> Result<&TransitGatewayVpcAttachment, Ec2Error> {
        if !self.transit_gateways.contains_key(tgw_id) {
            return Err(Ec2Error::TransitGatewayNotFound(tgw_id.to_string()));
        }
        let attach_id = self.next_tgw_attach_id();
        let att = TransitGatewayVpcAttachment {
            attachment_id: attach_id.clone(),
            transit_gateway_id: tgw_id.to_string(),
            vpc_id: vpc_id.to_string(),
            subnet_ids,
            state: "available".to_string(),
            tags,
        };
        self.tgw_vpc_attachments.insert(attach_id.clone(), att);
        Ok(self.tgw_vpc_attachments.get(&attach_id).unwrap())
    }

    pub fn delete_transit_gateway_vpc_attachment(
        &mut self,
        attach_id: &str,
    ) -> Result<(), Ec2Error> {
        let att = self
            .tgw_vpc_attachments
            .get_mut(attach_id)
            .ok_or_else(|| Ec2Error::TgwVpcAttachmentNotFound(attach_id.to_string()))?;
        att.state = "deleted".to_string();
        Ok(())
    }

    pub fn create_transit_gateway_route_table(
        &mut self,
        tgw_id: &str,
        tags: Tags,
    ) -> Result<&TransitGatewayRouteTable, Ec2Error> {
        if !self.transit_gateways.contains_key(tgw_id) {
            return Err(Ec2Error::TransitGatewayNotFound(tgw_id.to_string()));
        }
        let rtb_id = self.next_tgw_rtb_id();
        let rtb = TransitGatewayRouteTable {
            route_table_id: rtb_id.clone(),
            transit_gateway_id: tgw_id.to_string(),
            state: "available".to_string(),
            default_association_route_table: false,
            default_propagation_route_table: false,
            tags,
        };
        self.tgw_route_tables.insert(rtb_id.clone(), rtb);
        Ok(self.tgw_route_tables.get(&rtb_id).unwrap())
    }

    pub fn delete_transit_gateway_route_table(&mut self, rtb_id: &str) -> Result<(), Ec2Error> {
        let rtb = self
            .tgw_route_tables
            .get_mut(rtb_id)
            .ok_or_else(|| Ec2Error::TgwRouteTableNotFound(rtb_id.to_string()))?;
        rtb.state = "deleted".to_string();
        Ok(())
    }

    pub fn create_transit_gateway_route(
        &mut self,
        rtb_id: &str,
        cidr: &str,
        attachment_id: Option<String>,
    ) -> Result<(), Ec2Error> {
        if !self.tgw_route_tables.contains_key(rtb_id) {
            return Err(Ec2Error::TgwRouteTableNotFound(rtb_id.to_string()));
        }
        let route = TransitGatewayRoute {
            destination_cidr_block: cidr.to_string(),
            route_type: "static".to_string(),
            state: "active".to_string(),
            attachment_id,
        };
        self.tgw_routes
            .entry(rtb_id.to_string())
            .or_default()
            .push(route);
        Ok(())
    }

    pub fn delete_transit_gateway_route(
        &mut self,
        rtb_id: &str,
        cidr: &str,
    ) -> Result<(), Ec2Error> {
        let routes = self
            .tgw_routes
            .get_mut(rtb_id)
            .ok_or_else(|| Ec2Error::TgwRouteTableNotFound(rtb_id.to_string()))?;
        routes.retain(|r| r.destination_cidr_block != cidr);
        Ok(())
    }

    pub fn create_transit_gateway_peering_attachment(
        &mut self,
        tgw_id: &str,
        peer_tgw_id: &str,
    ) -> &TransitGatewayPeeringAttachment {
        let attach_id = self.next_tgw_attach_id();
        let att = TransitGatewayPeeringAttachment {
            attachment_id: attach_id.clone(),
            transit_gateway_id: tgw_id.to_string(),
            peer_transit_gateway_id: peer_tgw_id.to_string(),
            peer_account_id: "123456789012".to_string(),
            peer_region: "us-east-1".to_string(),
            state: "pendingAcceptance".to_string(),
            tags: HashMap::new(),
        };
        self.tgw_peering_attachments.insert(attach_id.clone(), att);
        self.tgw_peering_attachments.get(&attach_id).unwrap()
    }

    pub fn accept_transit_gateway_peering_attachment(
        &mut self,
        attach_id: &str,
    ) -> Result<(), Ec2Error> {
        let att = self
            .tgw_peering_attachments
            .get_mut(attach_id)
            .ok_or_else(|| Ec2Error::TgwPeeringAttachmentNotFound(attach_id.to_string()))?;
        att.state = "available".to_string();
        Ok(())
    }

    pub fn reject_transit_gateway_peering_attachment(
        &mut self,
        attach_id: &str,
    ) -> Result<(), Ec2Error> {
        let att = self
            .tgw_peering_attachments
            .get_mut(attach_id)
            .ok_or_else(|| Ec2Error::TgwPeeringAttachmentNotFound(attach_id.to_string()))?;
        att.state = "rejected".to_string();
        Ok(())
    }

    pub fn delete_transit_gateway_peering_attachment(
        &mut self,
        attach_id: &str,
    ) -> Result<(), Ec2Error> {
        let att = self
            .tgw_peering_attachments
            .get_mut(attach_id)
            .ok_or_else(|| Ec2Error::TgwPeeringAttachmentNotFound(attach_id.to_string()))?;
        att.state = "deleted".to_string();
        Ok(())
    }

    // --- Spot Instance operations ---

    pub fn request_spot_instances(
        &mut self,
        spot_price: &str,
        count: i32,
        image_id: &str,
        instance_type: &str,
        owner_id: &str,
    ) -> Vec<String> {
        let count = count.max(1) as usize;
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let mut request_ids = Vec::new();
        for _ in 0..count {
            let spot_id = self.next_spot_id();
            let instance_id = self.next_instance_id();
            let instance = Instance {
                instance_id: instance_id.clone(),
                image_id: image_id.to_string(),
                instance_type: instance_type.to_string(),
                state: InstanceState {
                    code: 16,
                    name: "running".to_string(),
                },
                private_ip_address: Some(format!(
                    "10.0.{}.{}",
                    (self.counters.instance >> 8) & 0xff,
                    self.counters.instance & 0xff
                )),
                public_ip_address: None,
                subnet_id: None,
                vpc_id: None,
                key_name: None,
                security_groups: Vec::new(),
                launch_time: now.clone(),
                tags: HashMap::new(),
                iam_instance_profile_arn: None,
                monitoring_state: "disabled".to_string(),
                placement_az: "us-east-1a".to_string(),
                placement_group_name: None,
                placement_tenancy: None,
                placement_host_id: None,
                placement_affinity: None,
                placement_partition_number: None,
                owner_id: owner_id.to_string(),
                classic_link_vpc: None,
                private_dns_hostname_type: None,
                enable_resource_name_dns_a_record: None,
                enable_resource_name_dns_aaaa_record: None,
                credit_specification: None,
                cpu_options: None,
                maintenance_options: None,
                network_bandwidth_weighting: None,
                lifecycle: Some("spot".to_string()),
                product_codes: Vec::new(),
                capacity_reservation_specification: None,
            };
            self.instances.insert(instance_id.clone(), instance);
            let req = SpotInstanceRequest {
                spot_instance_request_id: spot_id.clone(),
                spot_price: spot_price.to_string(),
                instance_type: instance_type.to_string(),
                image_id: image_id.to_string(),
                state: "active".to_string(),
                status_code: "fulfilled".to_string(),
                instance_id: Some(instance_id),
                tags: HashMap::new(),
            };
            self.spot_requests.insert(spot_id.clone(), req);
            request_ids.push(spot_id);
        }
        request_ids
    }

    pub fn cancel_spot_instance_requests(&mut self, ids: &[String]) {
        for id in ids {
            if let Some(req) = self.spot_requests.get_mut(id) {
                req.state = "cancelled".to_string();
            }
        }
    }

    /// Create the singleton spot datafeed subscription. Returns
    /// `AlreadyExists` if one is already present.
    pub fn create_spot_datafeed_subscription(
        &mut self,
        bucket: String,
        prefix: Option<String>,
        owner_id: String,
    ) -> Result<&SpotDatafeedSubscription, Ec2Error> {
        if self.spot_datafeed_subscription.is_some() {
            return Err(Ec2Error::SpotDatafeedAlreadyExists);
        }
        self.spot_datafeed_subscription = Some(SpotDatafeedSubscription {
            bucket,
            prefix,
            owner_id,
            state: "Active".to_string(),
        });
        Ok(self.spot_datafeed_subscription.as_ref().unwrap())
    }

    /// Delete the singleton spot datafeed subscription. No-op if none
    /// exists ( real AWS also returns success in that case ).
    pub fn delete_spot_datafeed_subscription(&mut self) {
        self.spot_datafeed_subscription = None;
    }

    /// Describe the singleton spot datafeed subscription. Returns
    /// `InvalidSpotDatafeedNotFound` when no subscription is set.
    pub fn describe_spot_datafeed_subscription(
        &self,
    ) -> Result<&SpotDatafeedSubscription, Ec2Error> {
        self.spot_datafeed_subscription
            .as_ref()
            .ok_or(Ec2Error::SpotDatafeedNotFound)
    }

    // --- IAM Instance Profile Association operations ---

    pub fn associate_iam_instance_profile(
        &mut self,
        instance_id: &str,
        arn: &str,
        name: &str,
    ) -> &IamInstanceProfileAssociation {
        let assoc_id = self.next_iam_assoc_id();
        let assoc = IamInstanceProfileAssociation {
            association_id: assoc_id.clone(),
            instance_id: instance_id.to_string(),
            iam_instance_profile_arn: arn.to_string(),
            iam_instance_profile_name: name.to_string(),
            state: "associated".to_string(),
        };
        // Update the instance's IAM profile
        if let Some(inst) = self.instances.get_mut(instance_id) {
            inst.iam_instance_profile_arn = Some(arn.to_string());
        }
        self.iam_instance_profile_associations
            .insert(assoc_id.clone(), assoc);
        self.iam_instance_profile_associations
            .get(&assoc_id)
            .unwrap()
    }

    pub fn disassociate_iam_instance_profile(&mut self, assoc_id: &str) -> Result<(), Ec2Error> {
        let assoc = self
            .iam_instance_profile_associations
            .remove(assoc_id)
            .ok_or_else(|| Ec2Error::AssociationNotFound(assoc_id.to_string()))?;
        if let Some(inst) = self.instances.get_mut(&assoc.instance_id) {
            inst.iam_instance_profile_arn = None;
        }
        Ok(())
    }

    pub fn replace_iam_instance_profile_association(
        &mut self,
        assoc_id: &str,
        arn: &str,
        name: &str,
    ) -> Result<(), Ec2Error> {
        let assoc = self
            .iam_instance_profile_associations
            .get_mut(assoc_id)
            .ok_or_else(|| Ec2Error::AssociationNotFound(assoc_id.to_string()))?;
        assoc.iam_instance_profile_arn = arn.to_string();
        assoc.iam_instance_profile_name = name.to_string();
        let instance_id = assoc.instance_id.clone();
        if let Some(inst) = self.instances.get_mut(&instance_id) {
            inst.iam_instance_profile_arn = Some(arn.to_string());
        }
        Ok(())
    }

    // --- Additional AMI operations ---

    pub fn register_image(
        &mut self,
        name: &str,
        description: &str,
        architecture: &str,
        virtualization_type: &str,
        root_device_name: &str,
        tags: Tags,
        owner_id: &str,
    ) -> String {
        let image_id = self.next_ami_id();
        let img = Image {
            image_id: image_id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            state: "available".to_string(),
            owner_id: owner_id.to_string(),
            architecture: architecture.to_string(),
            image_type: "machine".to_string(),
            platform: None,
            virtualization_type: virtualization_type.to_string(),
            root_device_type: "ebs".to_string(),
            root_device_name: root_device_name.to_string(),
            public: false,
            tags,
            source_instance_id: None,
            source_instance_type: String::new(),
            launch_permissions: Vec::new(),
            recycle_bin_state: None,
            deprecation_time: None,
            recycle_bin_enter_time: None,
            product_codes: Vec::new(),
            fast_launch_state: None,
            deregistration_protection: None,
            kernel_id: None,
            ramdisk_id: None,
            ena_support: None,
            sriov_net_support: None,
            tpm_support: None,
            boot_mode: None,
            imds_support: None,
            image_location: None,
            source_image_id: None,
            source_region: None,
        };
        self.images.insert(image_id.clone(), img);
        image_id
    }

    pub fn copy_image(
        &mut self,
        source_image_id: &str,
        name: &str,
        owner_id: &str,
    ) -> Result<String, Ec2Error> {
        let mut new_img = self
            .images
            .get(source_image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(source_image_id.to_string()))?
            .clone();
        let new_id = self.next_ami_id();
        new_img.image_id = new_id.clone();
        new_img.name = name.to_string();
        new_img.owner_id = owner_id.to_string();
        new_img.source_image_id = Some(source_image_id.to_string());
        self.images.insert(new_id.clone(), new_img);
        Ok(new_id)
    }

    // --- Additional Snapshot operations ---

    pub fn copy_snapshot(
        &mut self,
        source_snapshot_id: &str,
        owner_id: &str,
    ) -> Result<String, Ec2Error> {
        let mut new_snap = self
            .snapshots
            .get(source_snapshot_id)
            .ok_or_else(|| Ec2Error::SnapshotNotFound(source_snapshot_id.to_string()))?
            .clone();
        let new_id = self.next_snapshot_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        new_snap.snapshot_id = new_id.clone();
        new_snap.owner_id = owner_id.to_string();
        new_snap.start_time = now;
        self.snapshots.insert(new_id.clone(), new_snap);
        Ok(new_id)
    }

    pub fn create_snapshots_from_instance(
        &mut self,
        instance_id: &str,
        owner_id: &str,
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.instances.contains_key(instance_id) {
            return Err(Ec2Error::InstanceNotFound(instance_id.to_string()));
        }
        let vol_ids: Vec<String> = self
            .volumes
            .values()
            .filter(|v| v.attachments.iter().any(|a| a.instance_id == instance_id))
            .map(|v| v.volume_id.clone())
            .collect();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let mut snap_ids = Vec::new();
        for vol_id in vol_ids {
            let (vol_size, encrypted) = {
                let v = self.volumes.get(&vol_id).unwrap();
                (v.size, v.encrypted)
            };
            let snap_id = self.next_snapshot_id();
            let snap = Snapshot {
                snapshot_id: snap_id.clone(),
                volume_id: vol_id,
                volume_size: vol_size,
                state: "completed".to_string(),
                description: format!("Created by CreateSnapshots for instance {instance_id}"),
                start_time: now.clone(),
                progress: "100%".to_string(),
                owner_id: owner_id.to_string(),
                encrypted,
                tags: HashMap::new(),
                lock_state: "none".to_string(),
                lock_duration: None,
                lock_created_on: None,
                lock_expires_on: None,
                lock_duration_start_time: None,
                cool_off_period: None,
                cool_off_period_expires_on: None,
                storage_tier: "standard".to_string(),
                last_tiering_operation_status: None,
                fast_snapshot_restore_states: Vec::new(),
            };
            self.snapshots.insert(snap_id.clone(), snap);
            snap_ids.push(snap_id);
        }
        Ok(snap_ids)
    }

    // --- VPC endpoint modify ---

    pub fn modify_vpc_endpoint(&mut self, endpoint_id: &str) -> Result<(), Ec2Error> {
        if !self.vpc_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::VpcEndpointNotFound(endpoint_id.to_string()));
        }
        Ok(())
    }

    // --- Dedicated Host operations ---

    pub fn allocate_hosts(
        &mut self,
        availability_zone: &str,
        instance_type: Option<String>,
        quantity: usize,
        auto_placement: &str,
        host_recovery: &str,
        tags: Tags,
    ) -> Vec<String> {
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let _ = now;
        let mut ids = Vec::new();
        for _ in 0..quantity {
            let host_id = self.next_host_id();
            let host = DedicatedHost {
                host_id: host_id.clone(),
                availability_zone: availability_zone.to_string(),
                instance_type: instance_type.clone(),
                auto_placement: auto_placement.to_string(),
                host_recovery: host_recovery.to_string(),
                state: "available".to_string(),
                tags: tags.clone(),
            };
            self.dedicated_hosts.insert(host_id.clone(), host);
            ids.push(host_id);
        }
        ids
    }

    pub fn release_hosts(&mut self, host_ids: &[String]) -> Vec<String> {
        let mut successful = Vec::new();
        for id in host_ids {
            if let Some(h) = self.dedicated_hosts.get_mut(id) {
                h.state = "released".to_string();
                successful.push(id.clone());
            }
        }
        successful
    }

    pub fn modify_hosts(&mut self, host_ids: &[String]) -> Vec<String> {
        let mut successful = Vec::new();
        for id in host_ids {
            if self.dedicated_hosts.contains_key(id) {
                successful.push(id.clone());
            }
        }
        successful
    }

    // --- EC2 Fleet operations ---

    pub fn create_fleet(&mut self, fleet_type: &str, tags: Tags) -> String {
        let fleet_id = self.next_fleet_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let fleet = Ec2Fleet {
            fleet_id: fleet_id.clone(),
            state: "active".to_string(),
            fleet_type: fleet_type.to_string(),
            create_time: now,
            tags,
            total_target_capacity: None,
            on_demand_target_capacity: None,
            spot_target_capacity: None,
            context: None,
        };
        self.ec2_fleets.insert(fleet_id.clone(), fleet);
        fleet_id
    }

    pub fn delete_fleets(&mut self, fleet_ids: &[String]) -> Vec<String> {
        let mut successful = Vec::new();
        for id in fleet_ids {
            if let Some(f) = self.ec2_fleets.get_mut(id) {
                f.state = "deleted".to_string();
                successful.push(id.clone());
            }
        }
        successful
    }

    // --- VPC Endpoint Service Configuration operations ---

    pub fn create_vpc_endpoint_service_configuration(
        &mut self,
        acceptance_required: bool,
        network_load_balancer_arns: Vec<String>,
        gateway_load_balancer_arns: Vec<String>,
        tags: Tags,
    ) -> &VpcEndpointServiceConfiguration {
        let svc_id = self.next_vpce_svc_id();
        let svc_name = format!("com.amazonaws.vpce.us-east-1.{svc_id}");
        let svc_type = if !gateway_load_balancer_arns.is_empty() {
            "GatewayLoadBalancer"
        } else {
            "Interface"
        };
        let config = VpcEndpointServiceConfiguration {
            service_id: svc_id.clone(),
            service_name: svc_name,
            service_type: svc_type.to_string(),
            acceptance_required,
            state: "available".to_string(),
            network_load_balancer_arns,
            gateway_load_balancer_arns,
            allowed_principals: Vec::new(),
            tags,
            payer_responsibility: Some("ServiceOwner".to_string()),
            private_dns_state: None,
        };
        self.vpc_endpoint_service_configs
            .insert(svc_id.clone(), config);
        self.vpc_endpoint_service_configs.get(&svc_id).unwrap()
    }

    /// Seed a pending endpoint connection -- used by tests, by the
    /// `CreateVpcEndpoint` handler when the endpoint targets a service that
    /// requires acceptance, and by Terraform converters.
    pub fn upsert_vpc_endpoint_connection(
        &mut self,
        service_id: &str,
        endpoint_id: &str,
        owner: &str,
        state: &str,
    ) {
        self.vpc_endpoint_connections.insert(
            (service_id.to_string(), endpoint_id.to_string()),
            VpcEndpointConnection {
                service_id: service_id.to_string(),
                vpc_endpoint_id: endpoint_id.to_string(),
                vpc_endpoint_owner: owner.to_string(),
                vpc_endpoint_state: state.to_string(),
                creation_timestamp: chrono::Utc::now()
                    .format("%Y-%m-%dT%H:%M:%S.000Z")
                    .to_string(),
            },
        );
    }

    pub fn accept_vpc_endpoint_connections(
        &mut self,
        service_id: &str,
        endpoint_ids: &[String],
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.vpc_endpoint_service_configs.contains_key(service_id) {
            return Err(Ec2Error::VpcEndpointServiceNotFound(service_id.to_string()));
        }
        let mut accepted = Vec::new();
        for ep in endpoint_ids {
            let key = (service_id.to_string(), ep.clone());
            if let Some(conn) = self.vpc_endpoint_connections.get_mut(&key) {
                if conn.vpc_endpoint_state == "pendingAcceptance" {
                    conn.vpc_endpoint_state = "available".to_string();
                    accepted.push(ep.clone());
                }
            }
        }
        Ok(accepted)
    }

    pub fn reject_vpc_endpoint_connections(
        &mut self,
        service_id: &str,
        endpoint_ids: &[String],
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.vpc_endpoint_service_configs.contains_key(service_id) {
            return Err(Ec2Error::VpcEndpointServiceNotFound(service_id.to_string()));
        }
        let mut rejected = Vec::new();
        for ep in endpoint_ids {
            let key = (service_id.to_string(), ep.clone());
            if let Some(conn) = self.vpc_endpoint_connections.get_mut(&key) {
                if conn.vpc_endpoint_state == "pendingAcceptance" {
                    conn.vpc_endpoint_state = "rejected".to_string();
                    rejected.push(ep.clone());
                }
            }
        }
        Ok(rejected)
    }

    pub fn modify_vpc_endpoint_service_payer_responsibility(
        &mut self,
        service_id: &str,
        payer_responsibility: &str,
    ) -> Result<(), Ec2Error> {
        let cfg = self
            .vpc_endpoint_service_configs
            .get_mut(service_id)
            .ok_or_else(|| Ec2Error::VpcEndpointServiceNotFound(service_id.to_string()))?;
        cfg.payer_responsibility = Some(payer_responsibility.to_string());
        Ok(())
    }

    pub fn start_vpc_endpoint_service_private_dns_verification(
        &mut self,
        service_id: &str,
    ) -> Result<(), Ec2Error> {
        let cfg = self
            .vpc_endpoint_service_configs
            .get_mut(service_id)
            .ok_or_else(|| Ec2Error::VpcEndpointServiceNotFound(service_id.to_string()))?;
        // Mock: AWS asynchronously transitions verifying -> verified.
        // We collapse to "verified" immediately so callers can observe the
        // terminal state via DescribeVpcEndpointServiceConfigurations.
        cfg.private_dns_state = Some("verified".to_string());
        Ok(())
    }

    // --- VPC Endpoint Connection Notification operations ---

    fn next_vpc_endpoint_connection_notification_id(&mut self) -> String {
        self.counters.vpc_endpoint_connection_notification += 1;
        format!(
            "vpce-notif-{:08x}",
            self.counters.vpc_endpoint_connection_notification
        )
    }

    pub fn create_vpc_endpoint_connection_notification(
        &mut self,
        sns_topic_arn: &str,
        connection_events: Vec<String>,
        service_id: Option<String>,
        vpc_endpoint_id: Option<String>,
    ) -> &VpcEndpointConnectionNotification {
        let id = self.next_vpc_endpoint_connection_notification_id();
        let notification_type = if vpc_endpoint_id.is_some() {
            "Endpoint".to_string()
        } else {
            "Topic".to_string()
        };
        let notification = VpcEndpointConnectionNotification {
            connection_notification_id: id.clone(),
            connection_notification_arn: sns_topic_arn.to_string(),
            connection_events,
            connection_notification_state: "Enabled".to_string(),
            connection_notification_type: notification_type,
            service_id,
            vpc_endpoint_id,
        };
        self.vpc_endpoint_connection_notifications
            .insert(id.clone(), notification);
        self.vpc_endpoint_connection_notifications.get(&id).unwrap()
    }

    pub fn delete_vpc_endpoint_connection_notifications(&mut self, ids: &[String]) -> Vec<String> {
        let mut not_found = Vec::new();
        for id in ids {
            if self
                .vpc_endpoint_connection_notifications
                .remove(id)
                .is_none()
            {
                not_found.push(id.clone());
            }
        }
        not_found
    }

    pub fn modify_vpc_endpoint_connection_notification(
        &mut self,
        id: &str,
        connection_notification_arn: Option<String>,
        connection_events: Option<Vec<String>>,
    ) -> Result<(), Ec2Error> {
        let n = self
            .vpc_endpoint_connection_notifications
            .get_mut(id)
            .ok_or_else(|| {
                Ec2Error::InvalidVpcEndpointConnectionNotificationNotFound(id.to_string())
            })?;
        if let Some(arn) = connection_notification_arn {
            n.connection_notification_arn = arn;
        }
        if let Some(evs) = connection_events {
            n.connection_events = evs;
        }
        Ok(())
    }

    // --- VPC Block Public Access operations ---

    fn next_vpc_block_public_access_exclusion_id(&mut self) -> String {
        self.counters.vpc_block_public_access_exclusion += 1;
        format!(
            "vpcbpa-exclusion-{:08x}",
            self.counters.vpc_block_public_access_exclusion
        )
    }

    pub fn create_vpc_block_public_access_exclusion(
        &mut self,
        resource_arn: &str,
        internet_gateway_exclusion_mode: &str,
        tags: Tags,
    ) -> &VpcBlockPublicAccessExclusion {
        let id = self.next_vpc_block_public_access_exclusion_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let ex = VpcBlockPublicAccessExclusion {
            exclusion_id: id.clone(),
            internet_gateway_exclusion_mode: internet_gateway_exclusion_mode.to_string(),
            resource_arn: resource_arn.to_string(),
            state: "create-complete".to_string(),
            creation_timestamp: now.clone(),
            last_update_timestamp: now,
            tags,
        };
        self.vpc_block_public_access_exclusions
            .insert(id.clone(), ex);
        self.vpc_block_public_access_exclusions.get(&id).unwrap()
    }

    pub fn delete_vpc_block_public_access_exclusion(
        &mut self,
        id: &str,
    ) -> Result<VpcBlockPublicAccessExclusion, Ec2Error> {
        let mut ex = self
            .vpc_block_public_access_exclusions
            .remove(id)
            .ok_or_else(|| {
                Ec2Error::InvalidVpcBlockPublicAccessExclusionNotFound(id.to_string())
            })?;
        ex.state = "delete-complete".to_string();
        ex.last_update_timestamp = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        Ok(ex)
    }

    pub fn modify_vpc_block_public_access_exclusion(
        &mut self,
        id: &str,
        new_mode: &str,
    ) -> Result<&VpcBlockPublicAccessExclusion, Ec2Error> {
        let ex = self
            .vpc_block_public_access_exclusions
            .get_mut(id)
            .ok_or_else(|| {
                Ec2Error::InvalidVpcBlockPublicAccessExclusionNotFound(id.to_string())
            })?;
        ex.internet_gateway_exclusion_mode = new_mode.to_string();
        ex.state = "update-complete".to_string();
        ex.last_update_timestamp = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        Ok(self.vpc_block_public_access_exclusions.get(id).unwrap())
    }

    pub fn modify_vpc_block_public_access_options(
        &mut self,
        account_id: &str,
        region: &str,
        block_mode: &str,
    ) -> &VpcBlockPublicAccessOptions {
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        self.vpc_block_public_access_options = Some(VpcBlockPublicAccessOptions {
            aws_account_id: account_id.to_string(),
            aws_region: region.to_string(),
            internet_gateway_block_mode: block_mode.to_string(),
            state: "update-complete".to_string(),
            last_update_timestamp: now,
        });
        self.vpc_block_public_access_options.as_ref().unwrap()
    }

    // --- VPC Encryption Control operations ---

    fn next_vpc_encryption_control_id(&mut self) -> String {
        self.counters.vpc_encryption_control += 1;
        format!(
            "vpc-encryption-control-{:08x}",
            self.counters.vpc_encryption_control
        )
    }

    pub fn create_vpc_encryption_control(
        &mut self,
        vpc_id: &str,
        tags: Tags,
    ) -> Result<&VpcEncryptionControl, Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let id = self.next_vpc_encryption_control_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let mode = "monitor".to_string();
        let vec = VpcEncryptionControl {
            vpc_encryption_control_id: id.clone(),
            vpc_id: vpc_id.to_string(),
            mode: mode.clone(),
            state: "monitor-complete".to_string(),
            mode_history: vec![(mode, now)],
            tags,
        };
        self.vpc_encryption_controls.insert(id.clone(), vec);
        Ok(self.vpc_encryption_controls.get(&id).unwrap())
    }

    pub fn delete_vpc_encryption_control(
        &mut self,
        id: &str,
    ) -> Result<VpcEncryptionControl, Ec2Error> {
        let mut vec = self
            .vpc_encryption_controls
            .remove(id)
            .ok_or_else(|| Ec2Error::InvalidVpcEncryptionControlNotFound(id.to_string()))?;
        vec.state = "delete-complete".to_string();
        Ok(vec)
    }

    pub fn modify_vpc_encryption_control(
        &mut self,
        id: &str,
        new_mode: Option<&str>,
    ) -> Result<&VpcEncryptionControl, Ec2Error> {
        let vec = self
            .vpc_encryption_controls
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidVpcEncryptionControlNotFound(id.to_string()))?;
        if let Some(m) = new_mode {
            let now = chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S.000Z")
                .to_string();
            vec.mode = m.to_string();
            vec.state = match m {
                "enforce" => "enforce-complete".to_string(),
                _ => "monitor-complete".to_string(),
            };
            vec.mode_history.push((m.to_string(), now));
        }
        Ok(self.vpc_encryption_controls.get(id).unwrap())
    }

    pub fn delete_vpc_endpoint_service_configurations(
        &mut self,
        svc_ids: &[String],
    ) -> Vec<String> {
        let mut removed = Vec::new();
        for id in svc_ids {
            if self.vpc_endpoint_service_configs.remove(id).is_some() {
                removed.push(id.clone());
            }
        }
        removed
    }

    pub fn modify_vpc_endpoint_service_permissions(
        &mut self,
        svc_id: &str,
        add_arns: Vec<String>,
        remove_arns: Vec<String>,
    ) -> Result<(), Ec2Error> {
        let config = self
            .vpc_endpoint_service_configs
            .get_mut(svc_id)
            .ok_or_else(|| Ec2Error::VpcEndpointServiceNotFound(svc_id.to_string()))?;
        for arn in add_arns {
            if !config.allowed_principals.contains(&arn) {
                config.allowed_principals.push(arn);
            }
        }
        config
            .allowed_principals
            .retain(|p| !remove_arns.contains(p));
        Ok(())
    }

    // --- Spot Fleet operations ---

    pub fn request_spot_fleet(
        &mut self,
        target_capacity: i32,
        iam_fleet_role: &str,
        tags: Tags,
    ) -> String {
        let id = self.next_spot_fleet_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let req = SpotFleetRequest {
            spot_fleet_request_id: id.clone(),
            spot_fleet_request_state: "active".to_string(),
            target_capacity,
            iam_fleet_role: iam_fleet_role.to_string(),
            create_time: now,
            tags,
        };
        self.spot_fleet_requests.insert(id.clone(), req);
        id
    }

    pub fn cancel_spot_fleet_requests(&mut self, ids: &[String]) -> Vec<String> {
        let mut successful = Vec::new();
        for id in ids {
            if let Some(r) = self.spot_fleet_requests.get_mut(id) {
                r.spot_fleet_request_state = "cancelled".to_string();
                successful.push(id.clone());
            }
        }
        successful
    }

    // --- Subnet CIDR Reservation operations ---

    pub fn create_subnet_cidr_reservation(
        &mut self,
        subnet_id: &str,
        cidr: &str,
        reservation_type: &str,
        description: &str,
        owner_id: &str,
    ) -> Result<SubnetCidrReservationEntry, Ec2Error> {
        if !self.subnets.contains_key(subnet_id) {
            return Err(Ec2Error::SubnetNotFound(subnet_id.to_string()));
        }
        let id = self.next_subnet_cidr_res_id();
        let entry = SubnetCidrReservationEntry {
            reservation_id: id.clone(),
            subnet_id: subnet_id.to_string(),
            cidr: cidr.to_string(),
            reservation_type: reservation_type.to_string(),
            description: description.to_string(),
            owner_id: owner_id.to_string(),
        };
        self.subnet_cidr_reservations.insert(id, entry.clone());
        Ok(entry)
    }

    pub fn delete_subnet_cidr_reservation(
        &mut self,
        reservation_id: &str,
    ) -> Result<SubnetCidrReservationEntry, Ec2Error> {
        self.subnet_cidr_reservations
            .remove(reservation_id)
            .ok_or_else(|| Ec2Error::SubnetCidrReservationNotFound(reservation_id.to_string()))
    }

    // --- Subnet IPv6 CIDR operations ---

    pub fn associate_subnet_cidr_block(
        &mut self,
        subnet_id: &str,
        ipv6_cidr_block: &str,
    ) -> Result<(String, String), Ec2Error> {
        let subnet = self
            .subnets
            .get_mut(subnet_id)
            .ok_or_else(|| Ec2Error::SubnetNotFound(subnet_id.to_string()))?;
        let assoc_id = {
            self.counters.subnet_ipv6_assoc += 1;
            format!("subnet-cidr-assoc-{:08x}", self.counters.subnet_ipv6_assoc)
        };
        subnet.ipv6_cidr_blocks.push(SubnetIpv6CidrAssoc {
            association_id: assoc_id.clone(),
            ipv6_cidr_block: ipv6_cidr_block.to_string(),
            state: "associated".to_string(),
        });
        Ok((assoc_id, ipv6_cidr_block.to_string()))
    }

    pub fn disassociate_subnet_cidr_block(
        &mut self,
        association_id: &str,
    ) -> Result<(String, String), Ec2Error> {
        for subnet in self.subnets.values_mut() {
            if let Some(pos) = subnet
                .ipv6_cidr_blocks
                .iter()
                .position(|a| a.association_id == association_id)
            {
                let cidr = subnet.ipv6_cidr_blocks[pos].ipv6_cidr_block.clone();
                let subnet_id = subnet.subnet_id.clone();
                subnet.ipv6_cidr_blocks.remove(pos);
                return Ok((subnet_id, cidr));
            }
        }
        Err(Ec2Error::AssociationNotFound(association_id.to_string()))
    }

    // --- Default VPC / Subnet operations ---

    pub fn create_default_vpc(&mut self) -> Result<&Vpc, Ec2Error> {
        // Check if a default VPC already exists
        if self.vpcs.values().any(|v| v.is_default) {
            return Err(Ec2Error::DefaultVpcAlreadyExists);
        }
        let vpc_id = self.next_vpc_id();
        let dhcp_options_id = format!("dopt-{:08x}", self.counters.vpc);
        let vpc = Vpc {
            vpc_id: vpc_id.clone(),
            cidr_block: "172.31.0.0/16".to_string(),
            state: "available".to_string(),
            dhcp_options_id,
            instance_tenancy: "default".to_string(),
            is_default: true,
            enable_dns_hostnames: true,
            enable_dns_support: true,
            secondary_cidr_blocks: Vec::new(),
            tags: HashMap::new(),
            classic_link_enabled: false,
        };
        self.vpcs.insert(vpc_id.clone(), vpc);
        Ok(self.vpcs.get(&vpc_id).unwrap())
    }

    pub fn create_default_subnet(&mut self, availability_zone: &str) -> Result<&Subnet, Ec2Error> {
        let default_vpc_id = self
            .vpcs
            .values()
            .find(|v| v.is_default)
            .map(|v| v.vpc_id.clone())
            .ok_or(Ec2Error::DefaultVpcNotFound)?;
        let subnet_id = self.next_subnet_id();
        let subnet = Subnet {
            subnet_id: subnet_id.clone(),
            vpc_id: default_vpc_id,
            cidr_block: "172.31.0.0/20".to_string(),
            availability_zone: availability_zone.to_string(),
            state: "available".to_string(),
            available_ip_address_count: 4091,
            map_public_ip_on_launch: true,
            ipv6_cidr_blocks: Vec::new(),
            tags: HashMap::new(),
        };
        self.subnets.insert(subnet_id.clone(), subnet);
        Ok(self.subnets.get(&subnet_id).unwrap())
    }

    fn next_pg_id(&mut self) -> String {
        self.counters.placement_group += 1;
        format!("pg-{:08x}", self.counters.placement_group)
    }

    pub fn create_placement_group(
        &mut self,
        group_name: &str,
        strategy: &str,
        partition_count: Option<i32>,
        spread_level: Option<String>,
        account_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<&PlacementGroup, Ec2Error> {
        if self
            .placement_groups
            .values()
            .any(|g| g.group_name == group_name)
        {
            return Err(Ec2Error::PlacementGroupAlreadyExists(
                group_name.to_string(),
            ));
        }
        match strategy {
            "cluster" | "spread" | "partition" => {}
            other => {
                return Err(Ec2Error::InvalidParameterValue(format!(
                    "Invalid placement strategy: {other}"
                )));
            }
        }
        if strategy == "partition" {
            let pc = partition_count.unwrap_or(2);
            if !(2..=7).contains(&pc) {
                return Err(Ec2Error::InvalidParameterValue(
                    "Partition count must be between 2 and 7".to_string(),
                ));
            }
        }
        let resolved_partition_count = if strategy == "partition" {
            Some(partition_count.unwrap_or(2))
        } else {
            None
        };
        let group_id = self.next_pg_id();
        let group_arn = format!("arn:aws:ec2:{region}:{account_id}:placement-group/{group_name}");
        let pg = PlacementGroup {
            group_id: group_id.clone(),
            group_name: group_name.to_string(),
            group_arn,
            strategy: strategy.to_string(),
            state: "available".to_string(),
            partition_count: resolved_partition_count,
            spread_level,
            tags,
        };
        self.placement_groups.insert(group_id.clone(), pg);
        Ok(self.placement_groups.get(&group_id).unwrap())
    }

    pub fn delete_placement_group(&mut self, group_name: &str) -> Result<(), Ec2Error> {
        let id = self
            .placement_groups
            .values()
            .find(|g| g.group_name == group_name)
            .map(|g| g.group_id.clone())
            .ok_or_else(|| Ec2Error::PlacementGroupNotFound(group_name.to_string()))?;
        self.placement_groups.remove(&id);
        Ok(())
    }

    fn next_eni_permission_id(&mut self) -> String {
        self.counters.eni_permission += 1;
        format!("eni-perm-{:08x}", self.counters.eni_permission)
    }

    pub fn create_network_interface_permission(
        &mut self,
        network_interface_id: &str,
        aws_account_id: Option<String>,
        aws_service: Option<String>,
        permission: &str,
    ) -> Result<&NetworkInterfacePermission, Ec2Error> {
        if !self.network_interfaces.contains_key(network_interface_id) {
            return Err(Ec2Error::NetworkInterfaceNotFound(
                network_interface_id.to_string(),
            ));
        }
        match permission {
            "INSTANCE-ATTACH" | "EIP-ASSOCIATE" => {}
            other => {
                return Err(Ec2Error::InvalidParameterValue(format!(
                    "Invalid permission: {other}"
                )));
            }
        }
        let id = self.next_eni_permission_id();
        let perm = NetworkInterfacePermission {
            network_interface_permission_id: id.clone(),
            network_interface_id: network_interface_id.to_string(),
            aws_account_id,
            aws_service,
            permission: permission.to_string(),
            permission_state: "GRANTED".to_string(),
        };
        self.network_interface_permissions.insert(id.clone(), perm);
        Ok(self.network_interface_permissions.get(&id).unwrap())
    }

    pub fn delete_network_interface_permission(
        &mut self,
        network_interface_permission_id: &str,
    ) -> Result<(), Ec2Error> {
        if self
            .network_interface_permissions
            .remove(network_interface_permission_id)
            .is_none()
        {
            return Err(Ec2Error::NetworkInterfacePermissionNotFound(
                network_interface_permission_id.to_string(),
            ));
        }
        Ok(())
    }

    fn next_instance_connect_endpoint_id(&mut self) -> String {
        self.counters.instance_connect_endpoint += 1;
        format!("eice-{:08x}", self.counters.instance_connect_endpoint)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_instance_connect_endpoint(
        &mut self,
        subnet_id: &str,
        security_group_ids: Vec<String>,
        preserve_client_ip: bool,
        ip_address_type: Option<String>,
        account_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<&InstanceConnectEndpoint, Ec2Error> {
        let subnet = self
            .subnets
            .get(subnet_id)
            .ok_or_else(|| Ec2Error::SubnetNotFound(subnet_id.to_string()))?;
        let vpc_id = subnet.vpc_id.clone();
        let availability_zone = subnet.availability_zone.clone();
        let id = self.next_instance_connect_endpoint_id();
        let arn = format!("arn:aws:ec2:{region}:{account_id}:instance-connect-endpoint/{id}");
        let dns_name = format!("{id}.{region}.ec2-instance-connect-endpoint.amazonaws.com");
        let fips_dns_name =
            format!("{id}.{region}.fips.ec2-instance-connect-endpoint.amazonaws.com");
        let ice = InstanceConnectEndpoint {
            instance_connect_endpoint_id: id.clone(),
            instance_connect_endpoint_arn: arn,
            subnet_id: subnet_id.to_string(),
            vpc_id,
            availability_zone,
            state: "create-complete".to_string(),
            created_at: "1970-01-01T00:00:00Z".to_string(),
            preserve_client_ip,
            security_group_ids,
            network_interface_ids: vec![],
            dns_name,
            fips_dns_name,
            ip_address_type: ip_address_type.unwrap_or_else(|| "ipv4".to_string()),
            owner_id: account_id.to_string(),
            tags,
        };
        self.instance_connect_endpoints.insert(id.clone(), ice);
        Ok(self.instance_connect_endpoints.get(&id).unwrap())
    }

    pub fn delete_instance_connect_endpoint(&mut self, endpoint_id: &str) -> Result<(), Ec2Error> {
        let ice = self
            .instance_connect_endpoints
            .get_mut(endpoint_id)
            .ok_or_else(|| Ec2Error::InstanceConnectEndpointNotFound(endpoint_id.to_string()))?;
        ice.state = "delete-complete".to_string();
        // Keep entry briefly so describe can find it; in real AWS they linger.
        // For simplicity, remove from map.
        self.instance_connect_endpoints.remove(endpoint_id);
        Ok(())
    }

    fn next_capacity_reservation_id(&mut self) -> String {
        self.counters.capacity_reservation += 1;
        format!("cr-{:08x}", self.counters.capacity_reservation)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_capacity_reservation(
        &mut self,
        instance_type: &str,
        instance_platform: &str,
        availability_zone: String,
        instance_count: i32,
        ebs_optimized: bool,
        ephemeral_storage: bool,
        tenancy: String,
        instance_match_criteria: String,
        end_date: Option<String>,
        end_date_type: String,
        outpost_arn: Option<String>,
        placement_group_arn: Option<String>,
        account_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<&CapacityReservation, Ec2Error> {
        if instance_count <= 0 {
            return Err(Ec2Error::InvalidParameterValue(
                "InstanceCount must be > 0".to_string(),
            ));
        }
        match end_date_type.as_str() {
            "unlimited" | "limited" => {}
            _ => {
                return Err(Ec2Error::InvalidParameterValue(format!(
                    "Invalid EndDateType: {end_date_type}"
                )));
            }
        }
        let id = self.next_capacity_reservation_id();
        let arn = format!("arn:aws:ec2:{region}:{account_id}:capacity-reservation/{id}");
        let cr = CapacityReservation {
            capacity_reservation_id: id.clone(),
            capacity_reservation_arn: arn,
            owner_id: account_id.to_string(),
            instance_type: instance_type.to_string(),
            instance_platform: instance_platform.to_string(),
            availability_zone,
            tenancy,
            total_instance_count: instance_count,
            available_instance_count: instance_count,
            ebs_optimized,
            ephemeral_storage,
            state: "active".to_string(),
            start_date: "1970-01-01T00:00:00Z".to_string(),
            end_date,
            end_date_type,
            instance_match_criteria,
            create_date: "1970-01-01T00:00:00Z".to_string(),
            outpost_arn,
            placement_group_arn,
            tags,
            pending_billing_owner_account_id: None,
            billing_owner_account_id: Some(account_id.to_string()),
            target_capacity_reservation_id: None,
            reservation_type: Some("default".to_string()),
            commitment_info: None,
        };
        self.capacity_reservations.insert(id.clone(), cr);
        Ok(self.capacity_reservations.get(&id).unwrap())
    }

    pub fn cancel_capacity_reservation(&mut self, id: &str) -> Result<(), Ec2Error> {
        let cr = self
            .capacity_reservations
            .get_mut(id)
            .ok_or_else(|| Ec2Error::CapacityReservationNotFound(id.to_string()))?;
        cr.state = "cancelled".to_string();
        Ok(())
    }

    pub fn modify_capacity_reservation(
        &mut self,
        id: &str,
        instance_count: Option<i32>,
        end_date: Option<String>,
        end_date_type: Option<String>,
        instance_match_criteria: Option<String>,
    ) -> Result<&CapacityReservation, Ec2Error> {
        let cr = self
            .capacity_reservations
            .get_mut(id)
            .ok_or_else(|| Ec2Error::CapacityReservationNotFound(id.to_string()))?;
        if let Some(c) = instance_count {
            if c <= 0 {
                return Err(Ec2Error::InvalidParameterValue(
                    "InstanceCount must be > 0".to_string(),
                ));
            }
            cr.total_instance_count = c;
            cr.available_instance_count = c;
        }
        if let Some(d) = end_date {
            cr.end_date = Some(d);
        }
        if let Some(t) = end_date_type {
            cr.end_date_type = t;
        }
        if let Some(m) = instance_match_criteria {
            cr.instance_match_criteria = m;
        }
        Ok(self.capacity_reservations.get(id).unwrap())
    }

    // --- Instance placement modification ---

    #[allow(clippy::too_many_arguments)]
    pub fn modify_instance_placement(
        &mut self,
        instance_id: &str,
        group_name: Option<String>,
        partition_number: Option<i32>,
        host_id: Option<String>,
        affinity: Option<String>,
        tenancy: Option<String>,
    ) -> Result<(), Ec2Error> {
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        if let Some(g) = group_name {
            // An empty string clears the placement group; this matches AWS
            // behaviour where ModifyInstancePlacement with GroupName="" removes
            // the instance from any placement group.
            if g.is_empty() {
                inst.placement_group_name = None;
            } else {
                inst.placement_group_name = Some(g);
            }
        }
        if let Some(p) = partition_number {
            inst.placement_partition_number = Some(p);
        }
        if let Some(h) = host_id {
            if h.is_empty() {
                inst.placement_host_id = None;
            } else {
                inst.placement_host_id = Some(h);
            }
        }
        if let Some(a) = affinity {
            if a.is_empty() {
                inst.placement_affinity = None;
            } else {
                inst.placement_affinity = Some(a);
            }
        }
        if let Some(t) = tenancy {
            if t.is_empty() {
                inst.placement_tenancy = None;
            } else {
                inst.placement_tenancy = Some(t);
            }
        }
        Ok(())
    }

    // --- Capacity Reservation Fleet operations ---

    fn next_capacity_reservation_fleet_id(&mut self) -> String {
        self.counters.capacity_reservation_fleet += 1;
        format!("crf-{:08x}", self.counters.capacity_reservation_fleet)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_capacity_reservation_fleet(
        &mut self,
        allocation_strategy: Option<String>,
        tenancy: Option<String>,
        instance_match_criteria: Option<String>,
        total_target_capacity: i32,
        end_date: Option<String>,
        instance_specs: Vec<CapacityReservationFleetInstanceSpec>,
        account_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<&CapacityReservationFleet, Ec2Error> {
        if total_target_capacity <= 0 {
            return Err(Ec2Error::InvalidParameterValue(
                "TotalTargetCapacity must be > 0".to_string(),
            ));
        }
        if instance_specs.is_empty() {
            return Err(Ec2Error::InvalidParameterValue(
                "At least one InstanceTypeSpecification is required".to_string(),
            ));
        }
        let id = self.next_capacity_reservation_fleet_id();
        let arn = format!("arn:aws:ec2:{region}:{account_id}:capacity-reservation-fleet/{id}");
        // Sum of specification weights serves as a proxy for fulfilled capacity.
        let fulfilled: f64 = instance_specs.iter().map(|s| s.weight.unwrap_or(1.0)).sum();
        let fleet = CapacityReservationFleet {
            capacity_reservation_fleet_id: id.clone(),
            capacity_reservation_fleet_arn: arn,
            state: "active".to_string(),
            tenancy: tenancy.unwrap_or_else(|| "default".to_string()),
            allocation_strategy: allocation_strategy.unwrap_or_else(|| "prioritized".to_string()),
            instance_match_criteria: instance_match_criteria.unwrap_or_else(|| "open".to_string()),
            total_target_capacity,
            total_fulfilled_capacity: fulfilled,
            create_time: "1970-01-01T00:00:00Z".to_string(),
            end_date,
            instance_type_specifications: instance_specs,
            tags,
        };
        self.capacity_reservation_fleets.insert(id.clone(), fleet);
        Ok(self.capacity_reservation_fleets.get(&id).unwrap())
    }

    pub fn cancel_capacity_reservation_fleet(&mut self, id: &str) -> Result<(), Ec2Error> {
        let fleet = self
            .capacity_reservation_fleets
            .get_mut(id)
            .ok_or_else(|| Ec2Error::CapacityReservationFleetNotFound(id.to_string()))?;
        fleet.state = "cancelled".to_string();
        Ok(())
    }

    pub fn modify_capacity_reservation_fleet(
        &mut self,
        id: &str,
        total_target_capacity: Option<i32>,
        end_date: Option<String>,
        remove_end_date: Option<bool>,
    ) -> Result<&CapacityReservationFleet, Ec2Error> {
        let fleet = self
            .capacity_reservation_fleets
            .get_mut(id)
            .ok_or_else(|| Ec2Error::CapacityReservationFleetNotFound(id.to_string()))?;
        if let Some(c) = total_target_capacity {
            if c <= 0 {
                return Err(Ec2Error::InvalidParameterValue(
                    "TotalTargetCapacity must be > 0".to_string(),
                ));
            }
            fleet.total_target_capacity = c;
        }
        if remove_end_date == Some(true) {
            fleet.end_date = None;
        } else if let Some(d) = end_date {
            fleet.end_date = Some(d);
        }
        Ok(self.capacity_reservation_fleets.get(id).unwrap())
    }

    // --- COIP Pool operations ---

    fn next_coip_pool_id(&mut self) -> String {
        self.counters.coip_pool += 1;
        format!("ipv4pool-coip-{:08x}", self.counters.coip_pool)
    }

    pub fn create_coip_pool(
        &mut self,
        local_gateway_route_table_id: &str,
        pool_cidrs: Vec<String>,
        account_id: &str,
        region: &str,
        tags: Tags,
    ) -> &CoipPool {
        let pool_id = self.next_coip_pool_id();
        let arn = format!("arn:aws:ec2:{region}:{account_id}:coip-pool/{pool_id}");
        let pool = CoipPool {
            pool_id: pool_id.clone(),
            pool_arn: arn,
            local_gateway_route_table_id: local_gateway_route_table_id.to_string(),
            pool_cidrs,
            tags,
        };
        self.coip_pools.insert(pool_id.clone(), pool);
        self.coip_pools.get(&pool_id).unwrap()
    }

    pub fn delete_coip_pool(&mut self, pool_id: &str) -> Result<CoipPool, Ec2Error> {
        self.coip_pools
            .remove(pool_id)
            .ok_or_else(|| Ec2Error::CoipPoolNotFound(pool_id.to_string()))
    }

    // --- ClassicLink VPC operations ---

    pub fn attach_classic_link_vpc(
        &mut self,
        instance_id: &str,
        vpc_id: &str,
        groups: Vec<String>,
    ) -> Result<(), Ec2Error> {
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        inst.classic_link_vpc = Some((vpc_id.to_string(), groups));
        Ok(())
    }

    pub fn detach_classic_link_vpc(
        &mut self,
        instance_id: &str,
        vpc_id: &str,
    ) -> Result<(), Ec2Error> {
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        match &inst.classic_link_vpc {
            Some((linked_vpc, _)) if linked_vpc == vpc_id => {
                inst.classic_link_vpc = None;
                Ok(())
            }
            _ => Err(Ec2Error::InvalidParameterValue(format!(
                "Instance '{instance_id}' is not linked to VPC '{vpc_id}'"
            ))),
        }
    }

    // --- Security Group VPC association operations ---

    pub fn associate_security_group_vpc(
        &mut self,
        group_id: &str,
        vpc_id: &str,
        vpc_owner_id: &str,
    ) -> Result<&SecurityGroupVpcAssociation, Ec2Error> {
        if !self.security_groups.contains_key(group_id) {
            return Err(Ec2Error::SecurityGroupNotFound(group_id.to_string()));
        }
        let key = (group_id.to_string(), vpc_id.to_string());
        let assoc = SecurityGroupVpcAssociation {
            group_id: group_id.to_string(),
            vpc_id: vpc_id.to_string(),
            vpc_owner_id: vpc_owner_id.to_string(),
            state: "associated".to_string(),
        };
        self.security_group_vpc_associations
            .insert(key.clone(), assoc);
        Ok(self.security_group_vpc_associations.get(&key).unwrap())
    }

    pub fn disassociate_security_group_vpc(
        &mut self,
        group_id: &str,
        vpc_id: &str,
    ) -> Result<SecurityGroupVpcAssociation, Ec2Error> {
        let key = (group_id.to_string(), vpc_id.to_string());
        self.security_group_vpc_associations
            .remove(&key)
            .ok_or_else(|| {
                Ec2Error::SecurityGroupVpcAssociationNotFound(
                    group_id.to_string(),
                    vpc_id.to_string(),
                )
            })
    }

    // --- Enclave Certificate IAM Role association operations ---

    pub fn associate_enclave_certificate_iam_role(
        &mut self,
        certificate_arn: &str,
        role_arn: &str,
    ) -> &EnclaveCertificateIamRoleAssociation {
        let key = (certificate_arn.to_string(), role_arn.to_string());
        // Generate predictable bucket/key/kms ID values for the mock.
        let bucket = format!(
            "aws-ec2-enclave-certificate-{:016x}",
            self.enclave_certificate_iam_role_associations.len() as u64 + 1
        );
        let object_key = format!("{certificate_arn}/{role_arn}.pem");
        let kms_key_id = format!(
            "arn:aws:kms:us-east-1:000000000000:key/{:08x}-mock-mock-mock-{:012x}",
            self.enclave_certificate_iam_role_associations.len() as u64 + 1,
            self.enclave_certificate_iam_role_associations.len() as u64 + 1
        );
        let assoc = EnclaveCertificateIamRoleAssociation {
            certificate_arn: certificate_arn.to_string(),
            role_arn: role_arn.to_string(),
            certificate_s3_bucket_name: bucket,
            certificate_s3_object_key: object_key,
            encryption_kms_key_id: kms_key_id,
        };
        self.enclave_certificate_iam_role_associations
            .insert(key.clone(), assoc);
        self.enclave_certificate_iam_role_associations
            .get(&key)
            .unwrap()
    }

    pub fn disassociate_enclave_certificate_iam_role(
        &mut self,
        certificate_arn: &str,
        role_arn: &str,
    ) -> Result<(), Ec2Error> {
        let key = (certificate_arn.to_string(), role_arn.to_string());
        if self
            .enclave_certificate_iam_role_associations
            .remove(&key)
            .is_none()
        {
            return Err(Ec2Error::EnclaveCertificateIamRoleAssociationNotFound(
                certificate_arn.to_string(),
                role_arn.to_string(),
            ));
        }
        Ok(())
    }

    // --- DNS name option operations ---

    pub fn modify_private_dns_name_options(
        &mut self,
        instance_id: &str,
        private_dns_hostname_type: Option<String>,
        enable_resource_name_dns_a_record: Option<bool>,
        enable_resource_name_dns_aaaa_record: Option<bool>,
    ) -> Result<(), Ec2Error> {
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        if let Some(t) = private_dns_hostname_type {
            inst.private_dns_hostname_type = Some(t);
        }
        if let Some(v) = enable_resource_name_dns_a_record {
            inst.enable_resource_name_dns_a_record = Some(v);
        }
        if let Some(v) = enable_resource_name_dns_aaaa_record {
            inst.enable_resource_name_dns_aaaa_record = Some(v);
        }
        Ok(())
    }

    pub fn modify_public_ip_dns_name_options(
        &mut self,
        network_interface_id: &str,
        hostname_type: &str,
    ) -> Result<(), Ec2Error> {
        let eni = self
            .network_interfaces
            .get_mut(network_interface_id)
            .ok_or_else(|| Ec2Error::NetworkInterfaceNotFound(network_interface_id.to_string()))?;
        eni.public_ip_dns_hostname_type = Some(hostname_type.to_string());
        Ok(())
    }

    // --- Mac SIP modification task operations ---

    fn next_mac_sip_task_id(&mut self) -> String {
        self.counters.mac_sip_task += 1;
        format!("macmodification-{:08x}", self.counters.mac_sip_task)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_mac_sip_modification_task(
        &mut self,
        instance_id: &str,
        apple_internal: Option<String>,
        base_system: Option<String>,
        debugging_restrictions: Option<String>,
        dtrace_restrictions: Option<String>,
        filesystem_protections: Option<String>,
        kext_signing: Option<String>,
        nvram_protections: Option<String>,
        tags: Tags,
    ) -> Result<&MacSipModificationTask, Ec2Error> {
        if !self.instances.contains_key(instance_id) {
            return Err(Ec2Error::InstanceNotFound(instance_id.to_string()));
        }
        let task_id = self.next_mac_sip_task_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let task = MacSipModificationTask {
            task_id: task_id.clone(),
            instance_id: instance_id.to_string(),
            task_type: "sip-modification".to_string(),
            task_state: "in-progress".to_string(),
            start_time: now,
            apple_internal,
            base_system,
            debugging_restrictions,
            dtrace_restrictions,
            filesystem_protections,
            kext_signing,
            nvram_protections,
            status: Some("pending".to_string()),
            tags,
        };
        self.mac_sip_modification_tasks
            .insert(task_id.clone(), task);
        Ok(self.mac_sip_modification_tasks.get(&task_id).unwrap())
    }

    // --- Declarative Policies Report operations ---

    fn next_declarative_policies_report_id(&mut self) -> String {
        self.counters.declarative_policies_report += 1;
        format!("report-{:016x}", self.counters.declarative_policies_report)
    }

    pub fn start_declarative_policies_report(
        &mut self,
        target_id: &str,
        s3_bucket: &str,
        s3_prefix: Option<String>,
        tags: Tags,
    ) -> &DeclarativePoliciesReport {
        let report_id = self.next_declarative_policies_report_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let report = DeclarativePoliciesReport {
            report_id: report_id.clone(),
            s3_bucket: s3_bucket.to_string(),
            s3_prefix,
            target_id: target_id.to_string(),
            status: "running".to_string(),
            start_time: now,
            end_time: None,
            tags,
        };
        self.declarative_policies_reports
            .insert(report_id.clone(), report);
        self.declarative_policies_reports.get(&report_id).unwrap()
    }

    pub fn cancel_declarative_policies_report(&mut self, report_id: &str) -> Result<(), Ec2Error> {
        let report = self
            .declarative_policies_reports
            .get_mut(report_id)
            .ok_or_else(|| Ec2Error::DeclarativePoliciesReportNotFound(report_id.to_string()))?;
        if report.status != "running" {
            return Err(Ec2Error::DeclarativePoliciesReportNotCancellable(
                report_id.to_string(),
            ));
        }
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        report.status = "cancelled".to_string();
        report.end_time = Some(now);
        Ok(())
    }

    // --- BYOIP CIDR operations ---

    pub fn provision_byoip_cidr(
        &mut self,
        cidr: &str,
        description: Option<String>,
    ) -> Result<&ByoipCidr, Ec2Error> {
        if self.byoip_cidrs.contains_key(cidr) {
            return Err(Ec2Error::ByoipCidrAlreadyExists(cidr.to_string()));
        }
        let entry = ByoipCidr {
            cidr: cidr.to_string(),
            description,
            state: "provisioned".to_string(),
            asn_association: None,
            ipam_pool_id: None,
        };
        self.byoip_cidrs.insert(cidr.to_string(), entry);
        Ok(self.byoip_cidrs.get(cidr).unwrap())
    }

    pub fn advertise_byoip_cidr(&mut self, cidr: &str) -> Result<&ByoipCidr, Ec2Error> {
        let entry = self
            .byoip_cidrs
            .get_mut(cidr)
            .ok_or_else(|| Ec2Error::InvalidByoipCidrNotFound(cidr.to_string()))?;
        if entry.state != "provisioned" && entry.state != "deprovisioned" {
            return Err(Ec2Error::ByoipCidrInvalidState(cidr.to_string()));
        }
        entry.state = "advertised".to_string();
        Ok(entry)
    }

    pub fn withdraw_byoip_cidr(&mut self, cidr: &str) -> Result<&ByoipCidr, Ec2Error> {
        let entry = self
            .byoip_cidrs
            .get_mut(cidr)
            .ok_or_else(|| Ec2Error::InvalidByoipCidrNotFound(cidr.to_string()))?;
        if entry.state != "advertised" {
            return Err(Ec2Error::ByoipCidrInvalidState(cidr.to_string()));
        }
        entry.state = "provisioned".to_string();
        Ok(entry)
    }

    pub fn deprovision_byoip_cidr(&mut self, cidr: &str) -> Result<&ByoipCidr, Ec2Error> {
        let entry = self
            .byoip_cidrs
            .get_mut(cidr)
            .ok_or_else(|| Ec2Error::InvalidByoipCidrNotFound(cidr.to_string()))?;
        if entry.state == "advertised" {
            return Err(Ec2Error::ByoipCidrInvalidState(cidr.to_string()));
        }
        entry.state = "deprovisioned".to_string();
        Ok(entry)
    }

    // --- Public IPv4 pool operations ---

    fn next_public_ipv4_pool_id(&mut self) -> String {
        self.counters.public_ipv4_pool += 1;
        format!("ipv4pool-ec2-{:08x}", self.counters.public_ipv4_pool)
    }

    pub fn create_public_ipv4_pool(
        &mut self,
        description: Option<String>,
        network_border_group: Option<String>,
        tags: Tags,
    ) -> &PublicIpv4Pool {
        let pool_id = self.next_public_ipv4_pool_id();
        let pool = PublicIpv4Pool {
            pool_id: pool_id.clone(),
            description,
            network_border_group,
            total_address_count: 0,
            total_available_address_count: 0,
            pool_address_ranges: Vec::new(),
            tags,
        };
        self.public_ipv4_pools.insert(pool_id.clone(), pool);
        self.public_ipv4_pools.get(&pool_id).unwrap()
    }

    pub fn delete_public_ipv4_pool(&mut self, pool_id: &str) -> Result<(), Ec2Error> {
        let pool = self
            .public_ipv4_pools
            .get(pool_id)
            .ok_or_else(|| Ec2Error::InvalidPublicIpv4PoolNotFound(pool_id.to_string()))?;
        if !pool.pool_address_ranges.is_empty() {
            return Err(Ec2Error::PublicIpv4PoolNotEmpty(pool_id.to_string()));
        }
        self.public_ipv4_pools.remove(pool_id);
        Ok(())
    }

    pub fn provision_public_ipv4_pool_cidr(
        &mut self,
        pool_id: &str,
        cidr: &str,
        netmask_length: i32,
    ) -> Result<(String, PublicIpv4PoolRange), Ec2Error> {
        let pool = self
            .public_ipv4_pools
            .get_mut(pool_id)
            .ok_or_else(|| Ec2Error::InvalidPublicIpv4PoolNotFound(pool_id.to_string()))?;
        // For the mock we simply derive a synthetic first/last address pair
        // from the CIDR string and the requested netmask length. The real AWS
        // API allocates the addresses out of a previously-provisioned BYOIP
        // CIDR; here we just record what was asked for.
        let count = if (1..=32).contains(&netmask_length) {
            1i32 << ((32 - netmask_length).max(0) as u32)
        } else {
            1
        };
        let first = cidr.split('/').next().unwrap_or("0.0.0.0").to_string();
        // Synthesize a "last" address from the integer offset.
        let last_octet = count.saturating_sub(1).min(255) as u8;
        let mut octets: Vec<u8> = first
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect::<Vec<u8>>();
        if octets.len() == 4 {
            octets[3] = octets[3].saturating_add(last_octet);
        }
        let last = if octets.len() == 4 {
            format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
        } else {
            first.clone()
        };
        let range = PublicIpv4PoolRange {
            first_address: first.clone(),
            last_address: last,
            address_count: count,
            available_address_count: count,
        };
        pool.pool_address_ranges.push(range.clone());
        pool.total_address_count += count;
        pool.total_available_address_count += count;
        Ok((first, range))
    }

    pub fn deprovision_public_ipv4_pool_cidr(
        &mut self,
        pool_id: &str,
        cidr: &str,
    ) -> Result<Vec<String>, Ec2Error> {
        let pool = self
            .public_ipv4_pools
            .get_mut(pool_id)
            .ok_or_else(|| Ec2Error::InvalidPublicIpv4PoolNotFound(pool_id.to_string()))?;
        let first_addr = cidr.split('/').next().unwrap_or("").to_string();
        let pos = pool
            .pool_address_ranges
            .iter()
            .position(|r| r.first_address == first_addr)
            .ok_or_else(|| {
                Ec2Error::PublicIpv4PoolCidrNotFound(cidr.to_string(), pool_id.to_string())
            })?;
        let range = pool.pool_address_ranges.remove(pos);
        pool.total_address_count -= range.address_count;
        pool.total_available_address_count -= range.available_address_count;
        let mut deprovisioned = Vec::new();
        for i in 0..range.address_count {
            let octets: Vec<u8> = range
                .first_address
                .split('.')
                .filter_map(|p| p.parse().ok())
                .collect();
            if octets.len() == 4 {
                let new_last = octets[3].saturating_add(i as u8);
                deprovisioned.push(format!(
                    "{}.{}.{}.{}",
                    octets[0], octets[1], octets[2], new_last
                ));
            }
        }
        Ok(deprovisioned)
    }

    // --- COIP CIDR operations ---

    pub fn create_coip_cidr(
        &mut self,
        cidr: &str,
        coip_pool_id: &str,
    ) -> Result<&CoipCidr, Ec2Error> {
        if !self.coip_pools.contains_key(coip_pool_id) {
            return Err(Ec2Error::CoipPoolNotFound(coip_pool_id.to_string()));
        }
        let key = (cidr.to_string(), coip_pool_id.to_string());
        if self.coip_cidrs.contains_key(&key) {
            return Err(Ec2Error::CoipCidrAlreadyExists(
                cidr.to_string(),
                coip_pool_id.to_string(),
            ));
        }
        // Also reflect into the pool's `pool_cidrs` Vec so the existing
        // describe path can see the CIDR if it learns to render `coip_cidrs`.
        if let Some(pool) = self.coip_pools.get_mut(coip_pool_id)
            && !pool.pool_cidrs.iter().any(|c| c == cidr)
        {
            pool.pool_cidrs.push(cidr.to_string());
        }
        let cidr_entry = CoipCidr {
            cidr: cidr.to_string(),
            coip_pool_id: coip_pool_id.to_string(),
        };
        self.coip_cidrs.insert(key.clone(), cidr_entry);
        Ok(self.coip_cidrs.get(&key).unwrap())
    }

    pub fn delete_coip_cidr(
        &mut self,
        cidr: &str,
        coip_pool_id: &str,
    ) -> Result<CoipCidr, Ec2Error> {
        let key = (cidr.to_string(), coip_pool_id.to_string());
        let entry = self.coip_cidrs.remove(&key).ok_or_else(|| {
            Ec2Error::CoipCidrNotFound(cidr.to_string(), coip_pool_id.to_string())
        })?;
        if let Some(pool) = self.coip_pools.get_mut(coip_pool_id) {
            pool.pool_cidrs.retain(|c| c != cidr);
        }
        Ok(entry)
    }

    // --- Address transfer operations ---

    pub fn accept_address_transfer(
        &mut self,
        allocation_id: &str,
    ) -> Result<&AddressTransfer, Ec2Error> {
        let entry = self
            .address_transfers
            .get_mut(allocation_id)
            .ok_or_else(|| Ec2Error::InvalidAddressTransferNotFound(allocation_id.to_string()))?;
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        entry.address_transfer_status = "accepted".to_string();
        entry.transfer_offer_accepted_timestamp = Some(now);
        Ok(entry)
    }

    // --- Address attribute operations ---

    pub fn modify_address_attribute(
        &mut self,
        allocation_id: &str,
        domain_name: Option<String>,
    ) -> Result<&ElasticIp, Ec2Error> {
        let eip = self
            .elastic_ips
            .get_mut(allocation_id)
            .ok_or_else(|| Ec2Error::AllocationNotFound(allocation_id.to_string()))?;
        eip.address_attribute_ptr_record = domain_name;
        Ok(eip)
    }

    pub fn reset_address_attribute(&mut self, allocation_id: &str) -> Result<&ElasticIp, Ec2Error> {
        let eip = self
            .elastic_ips
            .get_mut(allocation_id)
            .ok_or_else(|| Ec2Error::AllocationNotFound(allocation_id.to_string()))?;
        eip.address_attribute_ptr_record = None;
        Ok(eip)
    }

    pub fn move_address_to_vpc(&mut self, public_ip: &str) -> Result<&ElasticIp, Ec2Error> {
        let eip = self
            .elastic_ips
            .values_mut()
            .find(|e| e.public_ip == public_ip)
            .ok_or_else(|| Ec2Error::AllocationNotFound(public_ip.to_string()))?;
        eip.domain = "vpc".to_string();
        Ok(eip)
    }

    pub fn restore_address_to_classic(&mut self, public_ip: &str) -> Result<&ElasticIp, Ec2Error> {
        let eip = self
            .elastic_ips
            .values_mut()
            .find(|e| e.public_ip == public_ip)
            .ok_or_else(|| Ec2Error::AllocationNotFound(public_ip.to_string()))?;
        eip.domain = "standard".to_string();
        Ok(eip)
    }

    // --- NAT gateway secondary address operations ---

    fn next_nat_gw_assoc_id(&mut self) -> String {
        self.counters.nat_gateway_address_assoc += 1;
        format!(
            "eipassoc-natgw-{:08x}",
            self.counters.nat_gateway_address_assoc
        )
    }

    pub fn assign_private_nat_gateway_address(
        &mut self,
        nat_gateway_id: &str,
        private_ips: Vec<String>,
    ) -> Result<&NatGateway, Ec2Error> {
        // Assign synthetic private IPs if none were specified — generate
        // sequential addresses inside a private range. Match AWS where the
        // operation accepts either explicit addresses or a count.
        let nat = self
            .nat_gateways
            .get_mut(nat_gateway_id)
            .ok_or_else(|| Ec2Error::NatGatewayNotFound(nat_gateway_id.to_string()))?;
        let mut next_octet = nat.secondary_addresses.len() as u8 + 10;
        let resolved: Vec<String> = if private_ips.is_empty() {
            // default: allocate one address
            let ip = format!("10.0.0.{next_octet}");
            next_octet = next_octet.saturating_add(1);
            vec![ip]
        } else {
            private_ips
        };
        let _ = next_octet; // silence unused
        for ip in resolved {
            nat.secondary_addresses.push(NatGatewayAddressAssociation {
                allocation_id: None,
                association_id: None,
                network_interface_id: Some(format!("eni-{nat_gateway_id}")),
                private_ip: Some(ip),
                public_ip: None,
                status: "succeeded".to_string(),
                is_primary: false,
            });
        }
        Ok(self.nat_gateways.get(nat_gateway_id).unwrap())
    }

    pub fn unassign_private_nat_gateway_address(
        &mut self,
        nat_gateway_id: &str,
        private_ips: Vec<String>,
    ) -> Result<&NatGateway, Ec2Error> {
        let nat = self
            .nat_gateways
            .get_mut(nat_gateway_id)
            .ok_or_else(|| Ec2Error::NatGatewayNotFound(nat_gateway_id.to_string()))?;
        // Mark targeted addresses as unassigning, then drop them.
        let before = nat.secondary_addresses.len();
        nat.secondary_addresses.retain(|a| {
            !private_ips
                .iter()
                .any(|ip| a.private_ip.as_deref() == Some(ip))
        });
        if nat.secondary_addresses.len() == before && !private_ips.is_empty() {
            return Err(Ec2Error::InvalidNatGatewaySecondaryAddressNotFound(
                String::new(),
                private_ips.first().cloned().unwrap_or_default(),
            ));
        }
        Ok(self.nat_gateways.get(nat_gateway_id).unwrap())
    }

    pub fn associate_nat_gateway_address(
        &mut self,
        nat_gateway_id: &str,
        allocation_ids: Vec<String>,
        private_ips: Vec<String>,
    ) -> Result<&NatGateway, Ec2Error> {
        // Resolve each allocation_id to its public IP up-front so we don't
        // borrow `self.elastic_ips` and `self.nat_gateways` simultaneously.
        let mut resolved: Vec<(String, Option<String>, Option<String>, String)> = Vec::new();
        for (idx, alloc) in allocation_ids.iter().enumerate() {
            let public_ip = self
                .elastic_ips
                .get(alloc)
                .map(|e| e.public_ip.clone())
                .ok_or_else(|| Ec2Error::AllocationNotFound(alloc.clone()))?;
            let private_ip = private_ips.get(idx).cloned();
            let assoc_id = self.next_nat_gw_assoc_id();
            resolved.push((alloc.clone(), Some(public_ip), private_ip, assoc_id));
        }
        let nat = self
            .nat_gateways
            .get_mut(nat_gateway_id)
            .ok_or_else(|| Ec2Error::NatGatewayNotFound(nat_gateway_id.to_string()))?;
        for (alloc, public_ip, private_ip, assoc_id) in resolved {
            nat.secondary_addresses.push(NatGatewayAddressAssociation {
                allocation_id: Some(alloc),
                association_id: Some(assoc_id),
                network_interface_id: Some(format!("eni-{nat_gateway_id}")),
                private_ip,
                public_ip,
                status: "succeeded".to_string(),
                is_primary: false,
            });
        }
        Ok(self.nat_gateways.get(nat_gateway_id).unwrap())
    }

    pub fn disassociate_nat_gateway_address(
        &mut self,
        nat_gateway_id: &str,
        association_ids: Vec<String>,
    ) -> Result<&NatGateway, Ec2Error> {
        let nat = self
            .nat_gateways
            .get_mut(nat_gateway_id)
            .ok_or_else(|| Ec2Error::NatGatewayNotFound(nat_gateway_id.to_string()))?;
        let before = nat.secondary_addresses.len();
        nat.secondary_addresses.retain(|a| {
            !association_ids
                .iter()
                .any(|id| a.association_id.as_deref() == Some(id))
        });
        if nat.secondary_addresses.len() == before && !association_ids.is_empty() {
            return Err(Ec2Error::InvalidNatGatewaySecondaryAddressNotFound(
                association_ids.first().cloned().unwrap_or_default(),
                String::new(),
            ));
        }
        Ok(self.nat_gateways.get(nat_gateway_id).unwrap())
    }

    // --- Group 4: ID generators ---

    fn next_mac_volume_ownership_task_id(&mut self) -> String {
        self.counters.mac_volume_ownership_task += 1;
        format!(
            "macmodification-{:08x}",
            self.counters.mac_volume_ownership_task
        )
    }

    fn next_replace_root_volume_task_id(&mut self) -> String {
        self.counters.replace_root_volume_task += 1;
        format!("replacevol-{:08x}", self.counters.replace_root_volume_task)
    }

    fn next_snapshot_import_task_id(&mut self) -> String {
        self.counters.snapshot_import_task += 1;
        format!("import-snap-{:08x}", self.counters.snapshot_import_task)
    }

    fn next_conversion_task_id(&mut self) -> String {
        self.counters.conversion_task += 1;
        format!("import-i-{:08x}", self.counters.conversion_task)
    }

    fn next_export_task_id(&mut self) -> String {
        self.counters.export_task += 1;
        format!("export-i-{:08x}", self.counters.export_task)
    }

    fn next_import_task_id(&mut self) -> String {
        self.counters.import_task += 1;
        format!("import-ami-{:08x}", self.counters.import_task)
    }

    fn next_trunk_assoc_id(&mut self) -> String {
        self.counters.trunk_interface_assoc += 1;
        format!("trunk-assoc-{:08x}", self.counters.trunk_interface_assoc)
    }

    fn next_secondary_network_id(&mut self) -> String {
        self.counters.secondary_network += 1;
        format!("snet-{:08x}", self.counters.secondary_network)
    }

    fn next_secondary_subnet_id(&mut self) -> String {
        self.counters.secondary_subnet += 1;
        format!("ssubnet-{:08x}", self.counters.secondary_subnet)
    }

    // --- Group 4: Mac Volume Ownership / Replace Root Volume ---

    pub fn create_delegate_mac_volume_ownership_task(
        &mut self,
        instance_id: &str,
        source_account: &str,
        target_account: &str,
    ) -> &MacVolumeOwnershipTask {
        let task_id = self.next_mac_volume_ownership_task_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let task = MacVolumeOwnershipTask {
            task_id: task_id.clone(),
            mac_volume_ownership_task_state: "pending".to_string(),
            volume_id: instance_id.to_string(),
            source_volume_owner_account_id: source_account.to_string(),
            target_volume_owner_account_id: target_account.to_string(),
            creation_time: now,
            completion_time: None,
        };
        self.mac_volume_ownership_tasks
            .insert(task_id.clone(), task);
        self.mac_volume_ownership_tasks.get(&task_id).unwrap()
    }

    pub fn create_replace_root_volume_task(
        &mut self,
        instance_id: &str,
        image_id: Option<String>,
        snapshot_id: Option<String>,
        delete_replaced_root_volume: bool,
        tags: Tags,
    ) -> Result<&ReplaceRootVolumeTask, Ec2Error> {
        if !self.instances.contains_key(instance_id) {
            return Err(Ec2Error::InstanceNotFound(instance_id.to_string()));
        }
        let task_id = self.next_replace_root_volume_task_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let task = ReplaceRootVolumeTask {
            task_id: task_id.clone(),
            instance_id: instance_id.to_string(),
            task_state: "pending".to_string(),
            image_id,
            snapshot_id,
            delete_replaced_root_volume,
            start_time: now,
            complete_time: None,
            tags,
        };
        self.replace_root_volume_tasks.insert(task_id.clone(), task);
        Ok(self.replace_root_volume_tasks.get(&task_id).unwrap())
    }

    // --- Group 4: Snapshot Import ---

    #[allow(clippy::too_many_arguments)]
    pub fn import_snapshot(
        &mut self,
        description: Option<String>,
        format: Option<String>,
        url: Option<String>,
        s3_bucket: Option<String>,
        s3_key: Option<String>,
        encrypted: bool,
        kms_key_id: Option<String>,
        owner_id: &str,
        tags: Tags,
    ) -> &SnapshotImportTask {
        let task_id = self.next_snapshot_import_task_id();
        // The mock "completes" the import immediately and materialises a
        // snapshot keyed off this import task.
        let snap_id = self.next_snapshot_id();
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let snap = Snapshot {
            snapshot_id: snap_id.clone(),
            volume_id: String::new(),
            volume_size: 8,
            state: "completed".to_string(),
            description: description.clone().unwrap_or_default(),
            start_time: now.clone(),
            progress: "100%".to_string(),
            owner_id: owner_id.to_string(),
            encrypted,
            tags: tags.clone(),
            lock_state: "none".to_string(),
            lock_duration: None,
            lock_created_on: None,
            lock_expires_on: None,
            lock_duration_start_time: None,
            cool_off_period: None,
            cool_off_period_expires_on: None,
            storage_tier: "standard".to_string(),
            last_tiering_operation_status: None,
            fast_snapshot_restore_states: Vec::new(),
        };
        self.snapshots.insert(snap_id.clone(), snap);
        let task = SnapshotImportTask {
            import_task_id: task_id.clone(),
            status: "completed".to_string(),
            description,
            disk_image_size: Some(8.0 * 1024.0 * 1024.0 * 1024.0),
            format,
            url,
            user_bucket_s3_bucket: s3_bucket,
            user_bucket_s3_key: s3_key,
            owner_id: owner_id.to_string(),
            encrypted,
            kms_key_id,
            snapshot_id: Some(snap_id),
            tags,
        };
        self.snapshot_import_tasks.insert(task_id.clone(), task);
        // Also register a generic import-task entry so CancelImportTask /
        // describe lookups can find it by import task ID.
        self.import_tasks
            .insert(task_id.clone(), ("completed".to_string(), None));
        self.snapshot_import_tasks.get(&task_id).unwrap()
    }

    pub fn cancel_import_task(&mut self, task_id: &str) -> Result<(String, String), Ec2Error> {
        let entry = self
            .import_tasks
            .get_mut(task_id)
            .ok_or_else(|| Ec2Error::InvalidImportTaskNotFound(task_id.to_string()))?;
        let previous = entry.0.clone();
        entry.0 = "cancelled".to_string();
        entry.1 = Some(previous.clone());
        // Mirror the state into the snapshot import task if present so
        // describe operations stay consistent.
        if let Some(t) = self.snapshot_import_tasks.get_mut(task_id) {
            t.status = "cancelled".to_string();
        }
        Ok((previous, "cancelled".to_string()))
    }

    // --- Group 4: Conversion / Export tasks ---

    pub fn import_instance(
        &mut self,
        description: Option<String>,
        platform: &str,
        tags: Tags,
    ) -> &ConversionTask {
        let task_id = self.next_conversion_task_id();
        let now = chrono::Utc::now();
        let expires = (now + chrono::Duration::days(7))
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let task = ConversionTask {
            conversion_task_id: task_id.clone(),
            expiration_time: expires,
            description,
            instance_id: None,
            platform: platform.to_string(),
            volumes: Vec::new(),
            state: "active".to_string(),
            status_message: None,
            tags,
        };
        self.conversion_tasks.insert(task_id.clone(), task);
        self.conversion_tasks.get(&task_id).unwrap()
    }

    pub fn cancel_conversion_task(&mut self, task_id: &str) -> Result<(), Ec2Error> {
        // Try the regular conversion-task table first; fall back to the
        // ImportVolume task table.
        if let Some(task) = self.conversion_tasks.get_mut(task_id) {
            task.state = "cancelled".to_string();
            return Ok(());
        }
        if let Some(task) = self.import_volume_tasks.get_mut(task_id) {
            task.status = "cancelled".to_string();
            return Ok(());
        }
        Err(Ec2Error::InvalidConversionTaskNotFound(task_id.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_instance_export_task(
        &mut self,
        description: String,
        instance_id: String,
        target_environment: String,
        disk_image_format: String,
        container_format: Option<String>,
        s3_bucket: String,
        s3_prefix: Option<String>,
        tags: Tags,
    ) -> &ExportTask {
        let task_id = self.next_export_task_id();
        let s3_key = format!(
            "{}{}/{}.{}",
            s3_prefix.clone().unwrap_or_default(),
            instance_id,
            task_id,
            match disk_image_format.as_str() {
                "vmdk" => "vmdk",
                "raw" => "raw",
                "vhd" => "vhd",
                _ => "vmdk",
            }
        );
        let task = ExportTask {
            export_task_id: task_id.clone(),
            description,
            instance_id,
            target_environment,
            disk_image_format,
            container_format,
            s3_bucket,
            s3_prefix,
            s3_key,
            status: "active".to_string(),
            status_message: None,
            tags,
        };
        self.export_tasks.insert(task_id.clone(), task);
        self.export_tasks.get(&task_id).unwrap()
    }

    pub fn cancel_export_task(&mut self, task_id: &str) -> Result<(), Ec2Error> {
        let task = self
            .export_tasks
            .get_mut(task_id)
            .ok_or_else(|| Ec2Error::InvalidExportTaskNotFound(task_id.to_string()))?;
        task.status = "cancelled".to_string();
        Ok(())
    }

    // --- Group 4: Trunk interface ---

    #[allow(clippy::too_many_arguments)]
    pub fn associate_trunk_interface(
        &mut self,
        branch_interface_id: String,
        trunk_interface_id: String,
        interface_protocol: String,
        vlan_id: Option<i32>,
        gre_key: Option<i32>,
        tags: Tags,
    ) -> &TrunkInterfaceAssociation {
        let assoc_id = self.next_trunk_assoc_id();
        let assoc = TrunkInterfaceAssociation {
            association_id: assoc_id.clone(),
            branch_interface_id,
            trunk_interface_id,
            interface_protocol,
            vlan_id,
            gre_key,
            tags,
        };
        self.trunk_interface_associations
            .insert(assoc_id.clone(), assoc);
        self.trunk_interface_associations.get(&assoc_id).unwrap()
    }

    pub fn disassociate_trunk_interface(&mut self, assoc_id: &str) -> Result<(), Ec2Error> {
        if self.trunk_interface_associations.remove(assoc_id).is_none() {
            return Err(Ec2Error::InvalidTrunkInterfaceAssociationNotFound(
                assoc_id.to_string(),
            ));
        }
        Ok(())
    }

    // --- Group 4: Secondary network / subnet ---

    pub fn create_secondary_network(
        &mut self,
        vpc_id: String,
        primary_cidr_block: String,
        network_border_group: Option<String>,
        tags: Tags,
    ) -> &SecondaryNetwork {
        let id = self.next_secondary_network_id();
        let network = SecondaryNetwork {
            network_id: id.clone(),
            vpc_id,
            primary_cidr_block,
            secondary_cidr_blocks: Vec::new(),
            state: "available".to_string(),
            network_border_group,
            tags,
        };
        self.secondary_networks.insert(id.clone(), network);
        self.secondary_networks.get(&id).unwrap()
    }

    pub fn delete_secondary_network(&mut self, id: &str) -> Result<SecondaryNetwork, Ec2Error> {
        if !self.secondary_networks.contains_key(id) {
            return Err(Ec2Error::InvalidSecondaryNetworkNotFound(id.to_string()));
        }
        let has_subnets = self
            .secondary_subnets
            .values()
            .any(|s| s.secondary_network_id == id);
        if has_subnets {
            return Err(Ec2Error::SecondaryNetworkHasSubnets(id.to_string()));
        }
        let mut removed = self.secondary_networks.remove(id).unwrap();
        removed.state = "deleted".to_string();
        Ok(removed)
    }

    pub fn create_secondary_subnet(
        &mut self,
        secondary_network_id: String,
        cidr_block: String,
        availability_zone: String,
        tags: Tags,
    ) -> Result<&SecondarySubnet, Ec2Error> {
        let net = self
            .secondary_networks
            .get(&secondary_network_id)
            .ok_or_else(|| {
                Ec2Error::InvalidSecondaryNetworkNotFound(secondary_network_id.clone())
            })?;
        let vpc_id = net.vpc_id.clone();
        let id = self.next_secondary_subnet_id();
        let subnet = SecondarySubnet {
            subnet_id: id.clone(),
            vpc_id,
            secondary_network_id,
            cidr_block,
            availability_zone,
            state: "available".to_string(),
            tags,
        };
        self.secondary_subnets.insert(id.clone(), subnet);
        Ok(self.secondary_subnets.get(&id).unwrap())
    }

    pub fn delete_secondary_subnet(&mut self, id: &str) -> Result<SecondarySubnet, Ec2Error> {
        let mut removed = self
            .secondary_subnets
            .remove(id)
            .ok_or_else(|| Ec2Error::InvalidSecondarySubnetNotFound(id.to_string()))?;
        removed.state = "deleted".to_string();
        Ok(removed)
    }

    // ===== Group 5 helpers =====

    fn now_iso(&self) -> String {
        chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string()
    }

    fn next_reserved_instances_exchange_id(&mut self) -> String {
        self.counters.reserved_instances_exchange += 1;
        format!("riex-{:08x}", self.counters.reserved_instances_exchange)
    }

    fn next_reserved_instances_listing_id(&mut self) -> String {
        self.counters.reserved_instances_listing += 1;
        format!("ril-{:08x}", self.counters.reserved_instances_listing)
    }

    fn next_reserved_instances_purchase_id(&mut self) -> String {
        self.counters.reserved_instances_purchase += 1;
        format!("rip-{:08x}", self.counters.reserved_instances_purchase)
    }

    fn next_reserved_instances_id(&mut self) -> String {
        self.counters.reserved_instances += 1;
        format!("ri-{:08x}", self.counters.reserved_instances)
    }

    fn next_reserved_instances_modification_id(&mut self) -> String {
        self.counters.reserved_instances_modification += 1;
        format!("rim-{:08x}", self.counters.reserved_instances_modification)
    }

    fn next_fpga_image_id(&mut self) -> String {
        self.counters.fpga_image += 1;
        format!("afi-{:08x}", self.counters.fpga_image)
    }

    fn next_image_usage_report_id(&mut self) -> String {
        self.counters.image_usage_report += 1;
        format!("iur-{:08x}", self.counters.image_usage_report)
    }

    fn next_import_image_task_id(&mut self) -> String {
        self.counters.import_image_task += 1;
        format!("import-ami-{:08x}", self.counters.import_image_task)
    }

    fn next_instance_event_window_id(&mut self) -> String {
        self.counters.instance_event_window += 1;
        format!("iew-{:08x}", self.counters.instance_event_window)
    }

    fn next_instance_event_id(&mut self) -> String {
        self.counters.instance_event += 1;
        format!("event-{:08x}", self.counters.instance_event)
    }

    fn next_host_reservation_id(&mut self) -> String {
        self.counters.host_reservation += 1;
        format!("hr-{:08x}", self.counters.host_reservation)
    }

    fn next_scheduled_instance_id(&mut self) -> String {
        self.counters.scheduled_instance += 1;
        format!("sci-{:08x}", self.counters.scheduled_instance)
    }

    pub fn next_instance_id_pub(&mut self) -> String {
        self.counters.instance += 1;
        format!("i-{:08x}", self.counters.instance)
    }

    // ----- Reserved Instances family -----

    pub fn accept_reserved_instances_exchange_quote(
        &mut self,
        target_reserved_instances_ids: Vec<String>,
        source_reserved_instances_ids: Vec<String>,
    ) -> String {
        let id = self.next_reserved_instances_exchange_id();
        let now = self.now_iso();
        self.reserved_instances_exchanges.insert(
            id.clone(),
            ReservedInstancesExchange {
                exchange_id: id.clone(),
                target_reserved_instances_ids,
                source_reserved_instances_ids,
                status: "completed".to_string(),
                status_message: None,
                time: now,
            },
        );
        id
    }

    pub fn cancel_reserved_instances_listing(
        &mut self,
        listing_id: &str,
    ) -> Result<&ReservedInstancesListing, Ec2Error> {
        let listing = self
            .reserved_instances_listings
            .get_mut(listing_id)
            .ok_or_else(|| {
                Ec2Error::InvalidReservedInstancesListingNotFound(listing_id.to_string())
            })?;
        listing.status = "cancelled".to_string();
        listing.update_date = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        Ok(self.reserved_instances_listings.get(listing_id).unwrap())
    }

    pub fn create_reserved_instances_listing(
        &mut self,
        reserved_instances_id: &str,
        instance_count: i32,
        price_schedules: Vec<PriceSchedule>,
        client_token: Option<String>,
        tags: Tags,
    ) -> &ReservedInstancesListing {
        let id = self.next_reserved_instances_listing_id();
        let now = self.now_iso();
        let listing = ReservedInstancesListing {
            listing_id: id.clone(),
            reserved_instances_id: reserved_instances_id.to_string(),
            instance_count,
            price_schedules,
            status: "active".to_string(),
            status_message: None,
            create_date: now.clone(),
            update_date: now,
            client_token,
            tags,
        };
        self.reserved_instances_listings.insert(id.clone(), listing);
        self.reserved_instances_listings.get(&id).unwrap()
    }

    pub fn delete_queued_reserved_instances(
        &mut self,
        ids: &[String],
    ) -> (Vec<String>, Vec<String>) {
        let mut succ = Vec::new();
        let mut fail = Vec::new();
        for id in ids {
            if self
                .queued_reserved_instances_purchases
                .remove(id)
                .is_some()
            {
                succ.push(id.clone());
            } else {
                fail.push(id.clone());
            }
        }
        (succ, fail)
    }

    pub fn modify_fleet(
        &mut self,
        fleet_id: &str,
        total_target_capacity: Option<i32>,
        on_demand_target_capacity: Option<i32>,
        spot_target_capacity: Option<i32>,
        context: Option<String>,
    ) -> Result<(), Ec2Error> {
        let fleet = self.ec2_fleets.get_mut(fleet_id).ok_or_else(|| {
            Ec2Error::InvalidParameterValue(format!("InvalidFleetId: {fleet_id}"))
        })?;
        if let Some(v) = total_target_capacity {
            fleet.total_target_capacity = Some(v);
        }
        if let Some(v) = on_demand_target_capacity {
            fleet.on_demand_target_capacity = Some(v);
        }
        if let Some(v) = spot_target_capacity {
            fleet.spot_target_capacity = Some(v);
        }
        if let Some(v) = context {
            fleet.context = Some(v);
        }
        Ok(())
    }

    pub fn modify_reserved_instances(
        &mut self,
        reserved_instances_ids: Vec<String>,
        target_configurations: Vec<ReservedInstancesConfiguration>,
        client_token: Option<String>,
    ) -> String {
        let id = self.next_reserved_instances_modification_id();
        let now = self.now_iso();
        let modification = ReservedInstancesModification {
            modification_id: id.clone(),
            reserved_instances_ids,
            target_configurations,
            status: "fulfilled".to_string(),
            status_message: None,
            create_date: now.clone(),
            update_date: now.clone(),
            effective_date: now,
            client_token,
        };
        self.reserved_instances_modifications
            .insert(id.clone(), modification);
        id
    }

    pub fn purchase_reserved_instances_offering(
        &mut self,
        offering_id: &str,
        instance_count: i32,
        limit_price: Option<String>,
        tags: Tags,
    ) -> String {
        let now = self.now_iso();
        let ri_id = self.next_reserved_instances_id();
        let purchase_id = self.next_reserved_instances_purchase_id();
        let ri = ReservedInstances {
            reserved_instances_id: ri_id.clone(),
            instance_type: "t3.micro".to_string(),
            instance_count,
            product_description: "Linux/UNIX".to_string(),
            scope: "Region".to_string(),
            currency_code: "USD".to_string(),
            duration: 31536000,
            fixed_price: 0.0,
            usage_price: 0.0,
            offering_class: "standard".to_string(),
            offering_type: "No Upfront".to_string(),
            instance_tenancy: "default".to_string(),
            start: now.clone(),
            end: now.clone(),
            state: "active".to_string(),
            tags: tags.clone(),
        };
        self.reserved_instances.insert(ri_id.clone(), ri);
        let purchase = ReservedInstancesPurchase {
            purchase_id: purchase_id.clone(),
            reserved_instances_offering_id: offering_id.to_string(),
            instance_count,
            limit_price,
            purchase_time: now,
            tags,
            queued: false,
            reserved_instances_id: Some(ri_id.clone()),
        };
        self.reserved_instances_purchases
            .insert(purchase_id, purchase);
        ri_id
    }

    // ----- Image / FPGA Image family -----

    pub fn cancel_image_launch_permission(&mut self, image_id: &str) -> Result<(), Ec2Error> {
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        img.launch_permissions.clear();
        Ok(())
    }

    pub fn create_fpga_image(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        owner_id: &str,
        tags: Tags,
    ) -> &FpgaImage {
        let id = self.next_fpga_image_id();
        let global_id = format!("agfi-{:08x}", self.counters.fpga_image);
        let now = self.now_iso();
        let img = FpgaImage {
            fpga_image_id: id.clone(),
            fpga_image_global_id: global_id,
            name: name.unwrap_or_else(|| format!("fpga-{}", self.counters.fpga_image)),
            description,
            shell_version: Some("0x04261818".to_string()),
            pci_id_vendor: None,
            pci_id_device: None,
            state: "available".to_string(),
            create_time: now.clone(),
            update_time: now,
            owner_id: owner_id.to_string(),
            owner_alias: None,
            product_codes: Vec::new(),
            tags,
            public: false,
            data_retention_support: false,
            instance_types: Vec::new(),
            load_permissions: Vec::new(),
        };
        self.fpga_images.insert(id.clone(), img);
        self.fpga_images.get(&id).unwrap()
    }

    pub fn copy_fpga_image(
        &mut self,
        source_fpga_image_id: &str,
        name: Option<String>,
        description: Option<String>,
        owner_id: &str,
    ) -> Result<String, Ec2Error> {
        let src = self
            .fpga_images
            .get(source_fpga_image_id)
            .ok_or_else(|| Ec2Error::InvalidFpgaImageNotFound(source_fpga_image_id.to_string()))?
            .clone();
        let id = self.next_fpga_image_id();
        let global_id = format!("agfi-{:08x}", self.counters.fpga_image);
        let now = self.now_iso();
        let img = FpgaImage {
            fpga_image_id: id.clone(),
            fpga_image_global_id: global_id,
            name: name.unwrap_or(src.name),
            description: description.or(src.description),
            shell_version: src.shell_version,
            pci_id_vendor: src.pci_id_vendor,
            pci_id_device: src.pci_id_device,
            state: "available".to_string(),
            create_time: now.clone(),
            update_time: now,
            owner_id: owner_id.to_string(),
            owner_alias: None,
            product_codes: Vec::new(),
            tags: Tags::new(),
            public: false,
            data_retention_support: src.data_retention_support,
            instance_types: src.instance_types,
            load_permissions: Vec::new(),
        };
        self.fpga_images.insert(id.clone(), img);
        Ok(id)
    }

    pub fn delete_fpga_image(&mut self, id: &str) -> Result<(), Ec2Error> {
        self.fpga_images
            .remove(id)
            .ok_or_else(|| Ec2Error::InvalidFpgaImageNotFound(id.to_string()))?;
        Ok(())
    }

    pub fn modify_fpga_image_attribute(
        &mut self,
        id: &str,
        name: Option<String>,
        description: Option<String>,
        add_load: Vec<(String, String)>,
        remove_load: Vec<(String, String)>,
    ) -> Result<&FpgaImage, Ec2Error> {
        let img = self
            .fpga_images
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidFpgaImageNotFound(id.to_string()))?;
        if let Some(n) = name {
            img.name = n;
        }
        if let Some(d) = description {
            img.description = Some(d);
        }
        for r in &remove_load {
            img.load_permissions.retain(|p| p != r);
        }
        for a in add_load {
            if !img.load_permissions.contains(&a) {
                img.load_permissions.push(a);
            }
        }
        Ok(self.fpga_images.get(id).unwrap())
    }

    pub fn reset_fpga_image_attribute(&mut self, id: &str, attr: &str) -> Result<(), Ec2Error> {
        let img = self
            .fpga_images
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidFpgaImageNotFound(id.to_string()))?;
        match attr {
            "loadPermission" => img.load_permissions.clear(),
            "description" => img.description = None,
            _ => {}
        }
        Ok(())
    }

    pub fn create_image_usage_report(
        &mut self,
        image_id: &str,
        account_filters: Vec<String>,
        resource_types: Vec<String>,
        tags: Tags,
    ) -> Result<String, Ec2Error> {
        if !self.images.contains_key(image_id) {
            return Err(Ec2Error::AmiNotFound(image_id.to_string()));
        }
        let id = self.next_image_usage_report_id();
        let now = self.now_iso();
        self.image_usage_reports.insert(
            id.clone(),
            ImageUsageReport {
                report_id: id.clone(),
                image_id: image_id.to_string(),
                account_filters,
                resource_types,
                status: "completed".to_string(),
                creation_time: now.clone(),
                completion_time: Some(now),
                tags,
            },
        );
        Ok(id)
    }

    pub fn delete_image_usage_report(&mut self, id: &str) -> Result<(), Ec2Error> {
        self.image_usage_reports
            .remove(id)
            .ok_or_else(|| Ec2Error::InvalidImageUsageReportNotFound(id.to_string()))?;
        Ok(())
    }

    pub fn create_restore_image_task(
        &mut self,
        bucket: &str,
        object_key: &str,
        name: Option<String>,
        owner_id: &str,
    ) -> String {
        let image_id = self.next_ami_id();
        let now = self.now_iso();
        let img_name = name.clone().unwrap_or_else(|| image_id.clone());
        // Materialise an Image so subsequent DescribeImages calls succeed.
        self.images.insert(
            image_id.clone(),
            Image {
                image_id: image_id.clone(),
                name: img_name.clone(),
                description: format!("Restored from s3://{bucket}/{object_key}"),
                state: "available".to_string(),
                owner_id: owner_id.to_string(),
                architecture: "x86_64".to_string(),
                image_type: "machine".to_string(),
                platform: None,
                virtualization_type: "hvm".to_string(),
                root_device_type: "ebs".to_string(),
                root_device_name: "/dev/xvda".to_string(),
                public: false,
                tags: Tags::new(),
                source_instance_id: None,
                source_instance_type: String::new(),
                launch_permissions: Vec::new(),
                recycle_bin_state: None,
                deprecation_time: None,
                recycle_bin_enter_time: None,
                product_codes: Vec::new(),
                fast_launch_state: None,
                deregistration_protection: None,
                kernel_id: None,
                ramdisk_id: None,
                ena_support: None,
                sriov_net_support: None,
                tpm_support: None,
                boot_mode: None,
                imds_support: None,
                image_location: Some(format!("s3://{bucket}/{object_key}")),
                source_image_id: None,
                source_region: None,
            },
        );
        self.restore_image_tasks.insert(
            image_id.clone(),
            RestoreImageTask {
                image_id: image_id.clone(),
                name: img_name,
                s3_object_url: format!("s3://{bucket}/{object_key}"),
                status: "completed".to_string(),
                status_message: None,
                creation_time: now,
            },
        );
        image_id
    }

    pub fn create_store_image_task(
        &mut self,
        ami_id: &str,
        bucket: &str,
    ) -> Result<String, Ec2Error> {
        if !self.images.contains_key(ami_id) {
            return Err(Ec2Error::AmiNotFound(ami_id.to_string()));
        }
        let object_key = format!("{ami_id}.bin");
        let now = self.now_iso();
        self.store_image_tasks.insert(
            ami_id.to_string(),
            StoreImageTask {
                image_id: ami_id.to_string(),
                ami_id: ami_id.to_string(),
                bucket: bucket.to_string(),
                s3_object_key: object_key.clone(),
                store_task_state: "completed".to_string(),
                store_task_failure_reason: None,
                progress_percentage: 100,
                task_start_time: now,
            },
        );
        Ok(object_key)
    }

    pub fn import_image(
        &mut self,
        description: Option<String>,
        license_type: Option<String>,
        platform: Option<String>,
        architecture: Option<String>,
        encrypted: bool,
        tags: Tags,
    ) -> &ImportImageTask {
        let id = self.next_import_image_task_id();
        let task = ImportImageTask {
            import_task_id: id.clone(),
            architecture,
            description,
            encrypted,
            hypervisor: Some("xen".to_string()),
            image_id: None,
            license_type,
            platform,
            progress: Some("0".to_string()),
            snapshot_details: Vec::new(),
            status: "active".to_string(),
            status_message: Some("pending".to_string()),
            tags,
            usage_operation: None,
            boot_mode: Some("uefi".to_string()),
        };
        self.import_image_tasks.insert(id.clone(), task);
        self.import_image_tasks.get(&id).unwrap()
    }

    pub fn restore_image_from_recycle_bin(&mut self, image_id: &str) -> Result<(), Ec2Error> {
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        if img.recycle_bin_state.as_deref() != Some("recycled") {
            return Err(Ec2Error::ImageNotInRecycleBin(image_id.to_string()));
        }
        img.recycle_bin_state = None;
        img.state = "available".to_string();
        self.deregistered_images.remove(image_id);
        Ok(())
    }

    pub fn replace_allowed_image_criteria(&mut self, criteria: Vec<AllowedImageCriterion>) {
        self.allowed_image_criteria = criteria;
    }

    // ----- Instance modify family -----

    pub fn modify_default_credit_specification(
        &mut self,
        instance_family: &str,
        cpu_credits: &str,
    ) -> InstanceFamilyCreditPair {
        self.default_credit_specifications
            .insert(instance_family.to_string(), cpu_credits.to_string());
        InstanceFamilyCreditPair {
            instance_family: instance_family.to_string(),
            cpu_credits: cpu_credits.to_string(),
        }
    }

    pub fn modify_instance_connect_endpoint(
        &mut self,
        endpoint_id: &str,
        preserve_client_ip: Option<bool>,
        security_group_ids: Option<Vec<String>>,
    ) -> Result<(), Ec2Error> {
        let ep = self
            .instance_connect_endpoints
            .get_mut(endpoint_id)
            .ok_or_else(|| Ec2Error::InstanceConnectEndpointNotFound(endpoint_id.to_string()))?;
        if let Some(v) = preserve_client_ip {
            ep.preserve_client_ip = v;
        }
        if let Some(v) = security_group_ids {
            ep.security_group_ids = v;
        }
        Ok(())
    }

    pub fn modify_instance_cpu_options(
        &mut self,
        instance_id: &str,
        core_count: Option<i32>,
        threads_per_core: Option<i32>,
    ) -> Result<&Instance, Ec2Error> {
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        let mut opts = inst.cpu_options.clone().unwrap_or_default();
        if let Some(v) = core_count {
            opts.core_count = Some(v);
        }
        if let Some(v) = threads_per_core {
            opts.threads_per_core = Some(v);
        }
        inst.cpu_options = Some(opts);
        Ok(self.instances.get(instance_id).unwrap())
    }

    pub fn modify_instance_credit_specification(
        &mut self,
        specs: &[(String, String)],
    ) -> (Vec<String>, Vec<String>) {
        let mut succ = Vec::new();
        let mut fail = Vec::new();
        for (instance_id, cpu_credits) in specs {
            if let Some(inst) = self.instances.get_mut(instance_id) {
                inst.credit_specification = Some(cpu_credits.clone());
                succ.push(instance_id.clone());
            } else {
                fail.push(instance_id.clone());
            }
        }
        (succ, fail)
    }

    pub fn modify_instance_maintenance_options(
        &mut self,
        instance_id: &str,
        auto_recovery: Option<String>,
        reboot_migration: Option<String>,
    ) -> Result<&Instance, Ec2Error> {
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        let mut opts = inst.maintenance_options.clone().unwrap_or_default();
        if let Some(v) = auto_recovery {
            opts.auto_recovery = Some(v);
        }
        if let Some(v) = reboot_migration {
            opts.reboot_migration = Some(v);
        }
        inst.maintenance_options = Some(opts);
        Ok(self.instances.get(instance_id).unwrap())
    }

    pub fn modify_instance_metadata_defaults(
        &mut self,
        http_tokens: Option<String>,
        http_put_response_hop_limit: Option<i32>,
        http_endpoint: Option<String>,
        instance_metadata_tags: Option<String>,
    ) {
        let mut defaults = self.instance_metadata_defaults.clone().unwrap_or_default();
        if let Some(v) = http_tokens {
            defaults.http_tokens = Some(v);
        }
        if let Some(v) = http_put_response_hop_limit {
            defaults.http_put_response_hop_limit = Some(v);
        }
        if let Some(v) = http_endpoint {
            defaults.http_endpoint = Some(v);
        }
        if let Some(v) = instance_metadata_tags {
            defaults.instance_metadata_tags = Some(v);
        }
        self.instance_metadata_defaults = Some(defaults);
    }

    pub fn modify_instance_network_performance_options(
        &mut self,
        instance_id: &str,
        bandwidth_weighting: &str,
    ) -> Result<(), Ec2Error> {
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        inst.network_bandwidth_weighting = Some(bandwidth_weighting.to_string());
        Ok(())
    }

    pub fn reset_instance_attribute(&mut self, instance_id: &str) -> Result<(), Ec2Error> {
        if !self.instances.contains_key(instance_id) {
            return Err(Ec2Error::InstanceNotFound(instance_id.to_string()));
        }
        Ok(())
    }

    // ----- Instance event window family -----

    pub fn create_instance_event_window(
        &mut self,
        name: Option<String>,
        time_ranges: Vec<InstanceEventWindowTimeRange>,
        cron_expression: Option<String>,
        tags: Tags,
    ) -> &InstanceEventWindow {
        let id = self.next_instance_event_window_id();
        let win = InstanceEventWindow {
            instance_event_window_id: id.clone(),
            name: name
                .unwrap_or_else(|| format!("event-window-{}", self.counters.instance_event_window)),
            time_ranges,
            cron_expression,
            association_target: None,
            state: "active".to_string(),
            tags,
        };
        self.instance_event_windows.insert(id.clone(), win);
        self.instance_event_windows.get(&id).unwrap()
    }

    pub fn modify_instance_event_window(
        &mut self,
        id: &str,
        name: Option<String>,
        time_ranges: Option<Vec<InstanceEventWindowTimeRange>>,
        cron_expression: Option<String>,
    ) -> Result<&InstanceEventWindow, Ec2Error> {
        let win = self
            .instance_event_windows
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidInstanceEventWindowNotFound(id.to_string()))?;
        if let Some(n) = name {
            win.name = n;
        }
        if let Some(t) = time_ranges {
            win.time_ranges = t;
        }
        if let Some(c) = cron_expression {
            win.cron_expression = Some(c);
        }
        Ok(self.instance_event_windows.get(id).unwrap())
    }

    pub fn delete_instance_event_window(&mut self, id: &str) -> Result<(), Ec2Error> {
        self.instance_event_windows
            .remove(id)
            .ok_or_else(|| Ec2Error::InvalidInstanceEventWindowNotFound(id.to_string()))?;
        Ok(())
    }

    pub fn associate_instance_event_window(
        &mut self,
        id: &str,
        instance_ids: Vec<String>,
        dedicated_host_ids: Vec<String>,
        tags: Vec<(String, String)>,
    ) -> Result<&InstanceEventWindow, Ec2Error> {
        let win = self
            .instance_event_windows
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidInstanceEventWindowNotFound(id.to_string()))?;
        let mut assoc = win.association_target.clone().unwrap_or_default();
        assoc.instance_ids.extend(instance_ids);
        assoc.dedicated_host_ids.extend(dedicated_host_ids);
        assoc.tags.extend(tags);
        win.association_target = Some(assoc);
        Ok(self.instance_event_windows.get(id).unwrap())
    }

    pub fn disassociate_instance_event_window(
        &mut self,
        id: &str,
        instance_ids: Vec<String>,
        dedicated_host_ids: Vec<String>,
    ) -> Result<&InstanceEventWindow, Ec2Error> {
        let win = self
            .instance_event_windows
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidInstanceEventWindowNotFound(id.to_string()))?;
        if let Some(assoc) = win.association_target.as_mut() {
            assoc.instance_ids.retain(|i| !instance_ids.contains(i));
            assoc
                .dedicated_host_ids
                .retain(|h| !dedicated_host_ids.contains(h));
        }
        Ok(self.instance_event_windows.get(id).unwrap())
    }

    pub fn modify_instance_event_start_time(
        &mut self,
        instance_id: &str,
        instance_event_id: &str,
        not_before: &str,
    ) -> Result<&InstanceEvent, Ec2Error> {
        // Synthesize the event on first lookup so callers can model upcoming
        // retirement windows without seeding state externally.
        if !self.instance_events.contains_key(instance_event_id) {
            if !self.instances.contains_key(instance_id) {
                return Err(Ec2Error::InstanceNotFound(instance_id.to_string()));
            }
            let event = InstanceEvent {
                event_id: instance_event_id.to_string(),
                instance_id: instance_id.to_string(),
                code: "instance-retirement".to_string(),
                description: "Scheduled instance retirement".to_string(),
                not_before: not_before.to_string(),
                not_after: not_before.to_string(),
                not_before_deadline: None,
            };
            self.instance_events
                .insert(instance_event_id.to_string(), event);
        }
        let event = self.instance_events.get_mut(instance_event_id).unwrap();
        event.not_before = not_before.to_string();
        Ok(self.instance_events.get(instance_event_id).unwrap())
    }

    pub fn register_instance_event_notification_attributes(
        &mut self,
        include_all_tags_of_instance: Option<bool>,
        instance_tag_keys: Vec<String>,
    ) -> &InstanceTagNotificationAttributes {
        let mut attrs = self
            .instance_event_notification_attributes
            .clone()
            .unwrap_or_default();
        if let Some(v) = include_all_tags_of_instance {
            attrs.include_all_tags_of_instance = v;
        }
        for k in instance_tag_keys {
            if !attrs.instance_tag_keys.contains(&k) {
                attrs.instance_tag_keys.push(k);
            }
        }
        self.instance_event_notification_attributes = Some(attrs);
        self.instance_event_notification_attributes
            .as_ref()
            .unwrap()
    }

    pub fn deregister_instance_event_notification_attributes(
        &mut self,
        include_all_tags_of_instance: Option<bool>,
        instance_tag_keys: &[String],
    ) -> InstanceTagNotificationAttributes {
        let mut attrs = self
            .instance_event_notification_attributes
            .clone()
            .unwrap_or_default();
        if include_all_tags_of_instance == Some(true) {
            attrs.include_all_tags_of_instance = false;
        }
        attrs
            .instance_tag_keys
            .retain(|k| !instance_tag_keys.contains(k));
        self.instance_event_notification_attributes = Some(attrs.clone());
        attrs
    }

    // ----- Misc family -----

    pub fn confirm_product_instance(
        &self,
        instance_id: &str,
        product_code: &str,
    ) -> Result<(String, bool), Ec2Error> {
        let inst = self
            .instances
            .get(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        let matched = inst.product_codes.iter().any(|(c, _)| c == product_code);
        Ok((
            inst.owner_id.clone(),
            matched || inst.product_codes.is_empty(),
        ))
    }

    pub fn delete_launch_template_versions(
        &mut self,
        lt_id: &str,
        versions: &[i64],
    ) -> Result<(Vec<i64>, Vec<i64>), Ec2Error> {
        let lt_id_resolved = if self.launch_templates.contains_key(lt_id) {
            lt_id.to_string()
        } else {
            self.launch_templates
                .values()
                .find(|lt| lt.launch_template_name == lt_id)
                .map(|lt| lt.launch_template_id.clone())
                .ok_or_else(|| Ec2Error::LaunchTemplateNotFound(lt_id.to_string()))?
        };
        let entries = self
            .launch_template_versions
            .get_mut(&lt_id_resolved)
            .ok_or_else(|| Ec2Error::LaunchTemplateNotFound(lt_id.to_string()))?;
        let mut succ = Vec::new();
        let mut fail = Vec::new();
        for v in versions {
            let before = entries.len();
            entries.retain(|e| e.version_number != *v);
            if entries.len() < before {
                succ.push(*v);
            } else {
                fail.push(*v);
            }
        }
        Ok((succ, fail))
    }

    pub fn modify_availability_zone_group(&mut self, group_name: &str, opt_in_status: &str) {
        self.az_group_opt_in
            .insert(group_name.to_string(), opt_in_status.to_string());
    }

    pub fn purchase_host_reservation(
        &mut self,
        host_id_set: Vec<String>,
        offering_id: &str,
        currency_code: Option<String>,
        instance_family: &str,
        tags: Tags,
    ) -> &HostReservation {
        let id = self.next_host_reservation_id();
        let now = self.now_iso();
        let res = HostReservation {
            host_reservation_id: id.clone(),
            host_id_set,
            currency_code: currency_code.unwrap_or_else(|| "USD".to_string()),
            duration: 31536000,
            end: None,
            hourly_price: "0.0".to_string(),
            instance_family: instance_family.to_string(),
            offering_id: offering_id.to_string(),
            payment_option: "NoUpfront".to_string(),
            start: now,
            state: "active".to_string(),
            upfront_price: "0.0".to_string(),
            tags,
        };
        self.host_reservations.insert(id.clone(), res);
        self.host_reservations.get(&id).unwrap()
    }

    pub fn purchase_scheduled_instances(
        &mut self,
        requests: Vec<ScheduledInstancePurchaseRequest>,
    ) -> Vec<ScheduledInstance> {
        let mut out = Vec::new();
        for req in requests {
            let id = self.next_scheduled_instance_id();
            let now = self.now_iso();
            let si = ScheduledInstance {
                scheduled_instance_id: id.clone(),
                instance_type: req.instance_type,
                platform: req.platform.unwrap_or_else(|| "Linux/UNIX".to_string()),
                network_platform: req
                    .network_platform
                    .unwrap_or_else(|| "EC2-VPC".to_string()),
                availability_zone: req.availability_zone,
                instance_count: req.instance_count,
                hourly_price: req.hourly_price.unwrap_or_else(|| "0.0".to_string()),
                total_scheduled_instance_hours: req.total_scheduled_instance_hours,
                term_start_date: now.clone(),
                term_end_date: now.clone(),
                recurrence: req.recurrence,
                slot_duration_in_hours: req.slot_duration_in_hours,
                previous_slot_end_time: None,
                next_slot_start_time: Some(now.clone()),
                create_date: now,
            };
            self.scheduled_instances.insert(id, si.clone());
            out.push(si);
        }
        out
    }

    pub fn report_instance_status(&mut self, report: InstanceStatusReport) {
        self.instance_status_reports.push(report);
    }

    pub fn restore_managed_prefix_list_version(
        &mut self,
        prefix_list_id: &str,
        previous_version: i64,
    ) -> Result<&ManagedPrefixList, Ec2Error> {
        let pl = self
            .managed_prefix_lists
            .get_mut(prefix_list_id)
            .ok_or_else(|| Ec2Error::PrefixListNotFound(prefix_list_id.to_string()))?;
        let snap = pl
            .version_history
            .iter()
            .find(|v| v.version == previous_version)
            .cloned()
            .ok_or_else(|| {
                Ec2Error::InvalidParameterValue(format!(
                    "PreviousVersion {previous_version} does not exist"
                ))
            })?;
        pl.entries = snap.entries;
        pl.version += 1;
        pl.version_history.push(ManagedPrefixListVersion {
            version: pl.version,
            entries: pl.entries.clone(),
        });
        Ok(self.managed_prefix_lists.get(prefix_list_id).unwrap())
    }

    pub fn run_scheduled_instances(
        &mut self,
        scheduled_instance_id: &str,
        instance_count: i32,
        owner_id: &str,
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.scheduled_instances.contains_key(scheduled_instance_id) {
            return Err(Ec2Error::InvalidScheduledInstanceNotFound(
                scheduled_instance_id.to_string(),
            ));
        }
        let now = self.now_iso();
        let mut ids = Vec::new();
        for _ in 0..instance_count.max(1) {
            let instance_id = self.next_instance_id_pub();
            let inst = Instance {
                instance_id: instance_id.clone(),
                image_id: "ami-scheduled".to_string(),
                instance_type: "t3.micro".to_string(),
                state: InstanceState {
                    code: 16,
                    name: "running".to_string(),
                },
                private_ip_address: None,
                public_ip_address: None,
                subnet_id: None,
                vpc_id: None,
                key_name: None,
                security_groups: Vec::new(),
                launch_time: now.clone(),
                tags: Tags::new(),
                iam_instance_profile_arn: None,
                monitoring_state: "disabled".to_string(),
                placement_az: "us-east-1a".to_string(),
                placement_group_name: None,
                placement_tenancy: None,
                placement_host_id: None,
                placement_affinity: None,
                placement_partition_number: None,
                owner_id: owner_id.to_string(),
                classic_link_vpc: None,
                private_dns_hostname_type: None,
                enable_resource_name_dns_a_record: None,
                enable_resource_name_dns_aaaa_record: None,
                credit_specification: None,
                cpu_options: None,
                maintenance_options: None,
                network_bandwidth_weighting: None,
                lifecycle: Some("scheduled".to_string()),
                product_codes: Vec::new(),
                capacity_reservation_specification: None,
            };
            self.instances.insert(instance_id.clone(), inst);
            ids.push(instance_id);
        }
        Ok(ids)
    }

    pub fn deregister_image_to_recycle_bin(&mut self, image_id: &str) -> Result<(), Ec2Error> {
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        img.recycle_bin_state = Some("recycled".to_string());
        img.state = "deregistered".to_string();
        self.deregistered_images.insert(image_id.to_string());
        Ok(())
    }

    // ===== Group 6 helpers and operations =====

    fn next_network_insights_access_scope_id(&mut self) -> String {
        self.counters.network_insights_access_scope += 1;
        format!("nis-{:08x}", self.counters.network_insights_access_scope)
    }

    fn next_network_insights_access_scope_analysis_id(&mut self) -> String {
        self.counters.network_insights_access_scope_analysis += 1;
        format!(
            "nisa-{:08x}",
            self.counters.network_insights_access_scope_analysis
        )
    }

    fn next_network_insights_path_id(&mut self) -> String {
        self.counters.network_insights_path += 1;
        format!("nip-{:08x}", self.counters.network_insights_path)
    }

    fn next_network_insights_analysis_id(&mut self) -> String {
        self.counters.network_insights_analysis += 1;
        format!("nia-{:08x}", self.counters.network_insights_analysis)
    }

    fn next_traffic_mirror_filter_id(&mut self) -> String {
        self.counters.traffic_mirror_filter += 1;
        format!("tmf-{:08x}", self.counters.traffic_mirror_filter)
    }

    fn next_traffic_mirror_filter_rule_id(&mut self) -> String {
        self.counters.traffic_mirror_filter_rule += 1;
        format!("tmfr-{:08x}", self.counters.traffic_mirror_filter_rule)
    }

    fn next_traffic_mirror_session_id(&mut self) -> String {
        self.counters.traffic_mirror_session += 1;
        format!("tms-{:08x}", self.counters.traffic_mirror_session)
    }

    fn next_traffic_mirror_target_id(&mut self) -> String {
        self.counters.traffic_mirror_target += 1;
        format!("tmt-{:08x}", self.counters.traffic_mirror_target)
    }

    // ----- Network Insights access scopes -----

    pub fn create_network_insights_access_scope(
        &mut self,
        match_paths: Vec<AccessScopePathSpec>,
        exclude_paths: Vec<AccessScopePathSpec>,
        tags: Tags,
    ) -> &NetworkInsightsAccessScope {
        let id = self.next_network_insights_access_scope_id();
        let now = self.now_iso();
        let arn = format!("arn:aws:ec2:us-east-1:123456789012:network-insights-access-scope/{id}");
        let scope = NetworkInsightsAccessScope {
            network_insights_access_scope_id: id.clone(),
            network_insights_access_scope_arn: arn,
            created_date: now.clone(),
            updated_date: now,
            tags,
            match_paths,
            exclude_paths,
        };
        self.network_insights_access_scopes
            .insert(id.clone(), scope);
        self.network_insights_access_scopes.get(&id).unwrap()
    }

    pub fn delete_network_insights_access_scope(&mut self, scope_id: &str) -> Result<(), Ec2Error> {
        if self
            .network_insights_access_scopes
            .remove(scope_id)
            .is_none()
        {
            return Err(Ec2Error::InvalidNetworkInsightsAccessScopeNotFound(
                scope_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn delete_network_insights_access_scope_analysis(
        &mut self,
        analysis_id: &str,
    ) -> Result<(), Ec2Error> {
        if self
            .network_insights_access_scope_analyses
            .remove(analysis_id)
            .is_none()
        {
            return Err(Ec2Error::InvalidNetworkInsightsAccessScopeAnalysisNotFound(
                analysis_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn start_network_insights_access_scope_analysis(
        &mut self,
        scope_id: &str,
        tags: Tags,
    ) -> Result<&NetworkInsightsAccessScopeAnalysis, Ec2Error> {
        if !self.network_insights_access_scopes.contains_key(scope_id) {
            return Err(Ec2Error::InvalidNetworkInsightsAccessScopeNotFound(
                scope_id.to_string(),
            ));
        }
        let id = self.next_network_insights_access_scope_analysis_id();
        let now = self.now_iso();
        let arn = format!(
            "arn:aws:ec2:us-east-1:123456789012:network-insights-access-scope-analysis/{id}"
        );
        let analysis = NetworkInsightsAccessScopeAnalysis {
            network_insights_access_scope_analysis_id: id.clone(),
            network_insights_access_scope_analysis_arn: arn,
            network_insights_access_scope_id: scope_id.to_string(),
            status: "succeeded".to_string(),
            status_message: None,
            warning_message: None,
            start_date: now.clone(),
            end_date: Some(now),
            findings_found: "false".to_string(),
            analyzed_eni_count: 0,
            tags,
        };
        self.network_insights_access_scope_analyses
            .insert(id.clone(), analysis);
        Ok(self
            .network_insights_access_scope_analyses
            .get(&id)
            .unwrap())
    }

    // ----- Network Insights paths -----

    #[allow(clippy::too_many_arguments)]
    pub fn create_network_insights_path(
        &mut self,
        source: Option<String>,
        destination: Option<String>,
        source_arn: Option<String>,
        destination_arn: Option<String>,
        source_ip: Option<String>,
        destination_ip: Option<String>,
        protocol: String,
        destination_port: Option<i32>,
        filter_at_source: NetworkInsightsPathFilter,
        filter_at_destination: NetworkInsightsPathFilter,
        tags: Tags,
    ) -> &NetworkInsightsPath {
        let id = self.next_network_insights_path_id();
        let now = self.now_iso();
        let arn = format!("arn:aws:ec2:us-east-1:123456789012:network-insights-path/{id}");
        let path = NetworkInsightsPath {
            network_insights_path_id: id.clone(),
            network_insights_path_arn: arn,
            created_date: now,
            source,
            destination,
            source_arn,
            destination_arn,
            source_ip,
            destination_ip,
            protocol,
            destination_port,
            tags,
            filter_at_source,
            filter_at_destination,
        };
        self.network_insights_paths.insert(id.clone(), path);
        self.network_insights_paths.get(&id).unwrap()
    }

    pub fn delete_network_insights_path(&mut self, path_id: &str) -> Result<(), Ec2Error> {
        if !self.network_insights_paths.contains_key(path_id) {
            return Err(Ec2Error::InvalidNetworkInsightsPathNotFound(
                path_id.to_string(),
            ));
        }
        let has_analyses = self
            .network_insights_analyses
            .values()
            .any(|a| a.network_insights_path_id == path_id);
        if has_analyses {
            return Err(Ec2Error::NetworkInsightsPathHasAnalyses(
                path_id.to_string(),
            ));
        }
        self.network_insights_paths.remove(path_id);
        Ok(())
    }

    pub fn delete_network_insights_analysis(&mut self, analysis_id: &str) -> Result<(), Ec2Error> {
        if self.network_insights_analyses.remove(analysis_id).is_none() {
            return Err(Ec2Error::InvalidNetworkInsightsAnalysisNotFound(
                analysis_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn start_network_insights_analysis(
        &mut self,
        path_id: &str,
        additional_accounts: Vec<String>,
        filter_in_arns: Vec<String>,
        tags: Tags,
    ) -> Result<&NetworkInsightsAnalysis, Ec2Error> {
        if !self.network_insights_paths.contains_key(path_id) {
            return Err(Ec2Error::InvalidNetworkInsightsPathNotFound(
                path_id.to_string(),
            ));
        }
        let id = self.next_network_insights_analysis_id();
        let now = self.now_iso();
        let arn = format!("arn:aws:ec2:us-east-1:123456789012:network-insights-analysis/{id}");
        let analysis = NetworkInsightsAnalysis {
            network_insights_analysis_id: id.clone(),
            network_insights_analysis_arn: arn,
            network_insights_path_id: path_id.to_string(),
            additional_accounts,
            filter_in_arns,
            start_date: now.clone(),
            end_date: Some(now),
            status: "succeeded".to_string(),
            status_message: None,
            warning_message: None,
            network_path_found: true,
            tags,
        };
        self.network_insights_analyses.insert(id.clone(), analysis);
        Ok(self.network_insights_analyses.get(&id).unwrap())
    }

    // ----- Traffic Mirror filters -----

    pub fn create_traffic_mirror_filter(
        &mut self,
        description: Option<String>,
        tags: Tags,
    ) -> &TrafficMirrorFilter {
        let id = self.next_traffic_mirror_filter_id();
        let filter = TrafficMirrorFilter {
            traffic_mirror_filter_id: id.clone(),
            description,
            ingress_filter_rules: Vec::new(),
            egress_filter_rules: Vec::new(),
            network_services: Vec::new(),
            tags,
        };
        self.traffic_mirror_filters.insert(id.clone(), filter);
        self.traffic_mirror_filters.get(&id).unwrap()
    }

    pub fn delete_traffic_mirror_filter(&mut self, filter_id: &str) -> Result<(), Ec2Error> {
        if !self.traffic_mirror_filters.contains_key(filter_id) {
            return Err(Ec2Error::InvalidTrafficMirrorFilterNotFound(
                filter_id.to_string(),
            ));
        }
        if self
            .traffic_mirror_sessions
            .values()
            .any(|s| s.traffic_mirror_filter_id == filter_id)
        {
            return Err(Ec2Error::TrafficMirrorFilterInUse(filter_id.to_string()));
        }
        self.traffic_mirror_filters.remove(filter_id);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_traffic_mirror_filter_rule(
        &mut self,
        filter_id: &str,
        traffic_direction: String,
        rule_number: i32,
        rule_action: String,
        protocol: Option<i32>,
        destination_port_range: Option<TrafficMirrorPortRange>,
        source_port_range: Option<TrafficMirrorPortRange>,
        destination_cidr_block: String,
        source_cidr_block: String,
        description: Option<String>,
        tags: Tags,
    ) -> Result<TrafficMirrorFilterRule, Ec2Error> {
        if !self.traffic_mirror_filters.contains_key(filter_id) {
            return Err(Ec2Error::InvalidTrafficMirrorFilterNotFound(
                filter_id.to_string(),
            ));
        }
        let rule_id = self.next_traffic_mirror_filter_rule_id();
        let rule = TrafficMirrorFilterRule {
            traffic_mirror_filter_rule_id: rule_id,
            traffic_mirror_filter_id: filter_id.to_string(),
            traffic_direction: traffic_direction.clone(),
            rule_number,
            rule_action,
            protocol,
            destination_port_range,
            source_port_range,
            destination_cidr_block,
            source_cidr_block,
            description,
            tags,
        };
        let filter = self.traffic_mirror_filters.get_mut(filter_id).unwrap();
        if traffic_direction == "egress" {
            filter.egress_filter_rules.push(rule.clone());
        } else {
            filter.ingress_filter_rules.push(rule.clone());
        }
        Ok(rule)
    }

    pub fn delete_traffic_mirror_filter_rule(&mut self, rule_id: &str) -> Result<(), Ec2Error> {
        for filter in self.traffic_mirror_filters.values_mut() {
            let before_in = filter.ingress_filter_rules.len();
            filter
                .ingress_filter_rules
                .retain(|r| r.traffic_mirror_filter_rule_id != rule_id);
            let before_out = filter.egress_filter_rules.len();
            filter
                .egress_filter_rules
                .retain(|r| r.traffic_mirror_filter_rule_id != rule_id);
            if filter.ingress_filter_rules.len() != before_in
                || filter.egress_filter_rules.len() != before_out
            {
                return Ok(());
            }
        }
        Err(Ec2Error::InvalidTrafficMirrorFilterRuleNotFound(
            rule_id.to_string(),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_traffic_mirror_filter_rule(
        &mut self,
        rule_id: &str,
        rule_number: Option<i32>,
        rule_action: Option<String>,
        protocol: Option<i32>,
        destination_port_range: Option<TrafficMirrorPortRange>,
        source_port_range: Option<TrafficMirrorPortRange>,
        destination_cidr_block: Option<String>,
        source_cidr_block: Option<String>,
        description: Option<String>,
        traffic_direction: Option<String>,
        remove_fields: &[String],
    ) -> Result<TrafficMirrorFilterRule, Ec2Error> {
        for filter in self.traffic_mirror_filters.values_mut() {
            for rule in filter
                .ingress_filter_rules
                .iter_mut()
                .chain(filter.egress_filter_rules.iter_mut())
            {
                if rule.traffic_mirror_filter_rule_id == rule_id {
                    if let Some(n) = rule_number {
                        rule.rule_number = n;
                    }
                    if let Some(a) = rule_action {
                        rule.rule_action = a;
                    }
                    if protocol.is_some() {
                        rule.protocol = protocol;
                    }
                    if destination_port_range.is_some() {
                        rule.destination_port_range = destination_port_range;
                    }
                    if source_port_range.is_some() {
                        rule.source_port_range = source_port_range;
                    }
                    if let Some(c) = destination_cidr_block {
                        rule.destination_cidr_block = c;
                    }
                    if let Some(c) = source_cidr_block {
                        rule.source_cidr_block = c;
                    }
                    if description.is_some() {
                        rule.description = description;
                    }
                    if let Some(td) = traffic_direction {
                        rule.traffic_direction = td;
                    }
                    for f in remove_fields {
                        match f.as_str() {
                            "protocol" => rule.protocol = None,
                            "description" => rule.description = None,
                            "destination-port-range" => rule.destination_port_range = None,
                            "source-port-range" => rule.source_port_range = None,
                            _ => {}
                        }
                    }
                    return Ok(rule.clone());
                }
            }
        }
        Err(Ec2Error::InvalidTrafficMirrorFilterRuleNotFound(
            rule_id.to_string(),
        ))
    }

    pub fn modify_traffic_mirror_filter_network_services(
        &mut self,
        filter_id: &str,
        add: &[String],
        remove: &[String],
    ) -> Result<TrafficMirrorFilter, Ec2Error> {
        let filter = self
            .traffic_mirror_filters
            .get_mut(filter_id)
            .ok_or_else(|| Ec2Error::InvalidTrafficMirrorFilterNotFound(filter_id.to_string()))?;
        for s in add {
            if !filter.network_services.contains(s) {
                filter.network_services.push(s.clone());
            }
        }
        filter.network_services.retain(|s| !remove.contains(s));
        Ok(filter.clone())
    }

    // ----- Traffic Mirror sessions -----

    #[allow(clippy::too_many_arguments)]
    pub fn create_traffic_mirror_session(
        &mut self,
        traffic_mirror_target_id: String,
        traffic_mirror_filter_id: String,
        network_interface_id: String,
        owner_id: String,
        packet_length: Option<i32>,
        session_number: i32,
        virtual_network_id: Option<i32>,
        description: Option<String>,
        tags: Tags,
    ) -> Result<&TrafficMirrorSession, Ec2Error> {
        if !self
            .traffic_mirror_filters
            .contains_key(&traffic_mirror_filter_id)
        {
            return Err(Ec2Error::InvalidTrafficMirrorFilterNotFound(
                traffic_mirror_filter_id,
            ));
        }
        if !self
            .traffic_mirror_targets
            .contains_key(&traffic_mirror_target_id)
        {
            return Err(Ec2Error::InvalidTrafficMirrorTargetNotFound(
                traffic_mirror_target_id,
            ));
        }
        let id = self.next_traffic_mirror_session_id();
        let session = TrafficMirrorSession {
            traffic_mirror_session_id: id.clone(),
            traffic_mirror_target_id,
            traffic_mirror_filter_id,
            network_interface_id,
            owner_id,
            packet_length,
            session_number,
            virtual_network_id,
            description,
            tags,
        };
        self.traffic_mirror_sessions.insert(id.clone(), session);
        Ok(self.traffic_mirror_sessions.get(&id).unwrap())
    }

    pub fn delete_traffic_mirror_session(&mut self, session_id: &str) -> Result<(), Ec2Error> {
        if self.traffic_mirror_sessions.remove(session_id).is_none() {
            return Err(Ec2Error::InvalidTrafficMirrorSessionNotFound(
                session_id.to_string(),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_traffic_mirror_session(
        &mut self,
        session_id: &str,
        traffic_mirror_target_id: Option<String>,
        traffic_mirror_filter_id: Option<String>,
        packet_length: Option<i32>,
        session_number: Option<i32>,
        virtual_network_id: Option<i32>,
        description: Option<String>,
        remove_fields: &[String],
    ) -> Result<TrafficMirrorSession, Ec2Error> {
        let session = self
            .traffic_mirror_sessions
            .get_mut(session_id)
            .ok_or_else(|| Ec2Error::InvalidTrafficMirrorSessionNotFound(session_id.to_string()))?;
        if let Some(t) = traffic_mirror_target_id {
            session.traffic_mirror_target_id = t;
        }
        if let Some(f) = traffic_mirror_filter_id {
            session.traffic_mirror_filter_id = f;
        }
        if packet_length.is_some() {
            session.packet_length = packet_length;
        }
        if let Some(n) = session_number {
            session.session_number = n;
        }
        if virtual_network_id.is_some() {
            session.virtual_network_id = virtual_network_id;
        }
        if description.is_some() {
            session.description = description;
        }
        for f in remove_fields {
            match f.as_str() {
                "packet-length" => session.packet_length = None,
                "description" => session.description = None,
                "virtual-network-id" => session.virtual_network_id = None,
                _ => {}
            }
        }
        Ok(session.clone())
    }

    // ----- Traffic Mirror targets -----

    #[allow(clippy::too_many_arguments)]
    pub fn create_traffic_mirror_target(
        &mut self,
        network_interface_id: Option<String>,
        network_load_balancer_arn: Option<String>,
        gateway_load_balancer_endpoint_id: Option<String>,
        description: Option<String>,
        owner_id: String,
        tags: Tags,
    ) -> &TrafficMirrorTarget {
        let id = self.next_traffic_mirror_target_id();
        let r#type = if network_load_balancer_arn.is_some() {
            "network-load-balancer".to_string()
        } else if gateway_load_balancer_endpoint_id.is_some() {
            "gateway-load-balancer-endpoint".to_string()
        } else {
            "network-interface".to_string()
        };
        let target = TrafficMirrorTarget {
            traffic_mirror_target_id: id.clone(),
            network_interface_id,
            network_load_balancer_arn,
            gateway_load_balancer_endpoint_id,
            r#type,
            description,
            owner_id,
            tags,
        };
        self.traffic_mirror_targets.insert(id.clone(), target);
        self.traffic_mirror_targets.get(&id).unwrap()
    }

    pub fn delete_traffic_mirror_target(&mut self, target_id: &str) -> Result<(), Ec2Error> {
        if !self.traffic_mirror_targets.contains_key(target_id) {
            return Err(Ec2Error::InvalidTrafficMirrorTargetNotFound(
                target_id.to_string(),
            ));
        }
        if self
            .traffic_mirror_sessions
            .values()
            .any(|s| s.traffic_mirror_target_id == target_id)
        {
            return Err(Ec2Error::TrafficMirrorTargetInUse(target_id.to_string()));
        }
        self.traffic_mirror_targets.remove(target_id);
        Ok(())
    }

    // ===== Group 7: Client VPN =====

    fn next_client_vpn_endpoint_id(&mut self) -> String {
        self.counters.client_vpn_endpoint += 1;
        format!("cvpn-endpoint-{:08x}", self.counters.client_vpn_endpoint)
    }

    fn next_client_vpn_target_network_association_id(&mut self) -> String {
        self.counters.client_vpn_target_network_association += 1;
        format!(
            "cvpn-assoc-{:08x}",
            self.counters.client_vpn_target_network_association
        )
    }

    fn next_client_vpn_connection_id(&mut self) -> String {
        self.counters.client_vpn_connection += 1;
        format!(
            "cvpn-connection-{:08x}",
            self.counters.client_vpn_connection
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_client_vpn_endpoint(
        &mut self,
        description: Option<String>,
        client_cidr_block: String,
        server_certificate_arn: String,
        dns_servers: Vec<String>,
        transport_protocol: String,
        vpn_port: i32,
        split_tunnel: bool,
        authentication_options: Vec<String>,
        connection_log_options_enabled: bool,
        connection_log_options_cloudwatch_log_group: Option<String>,
        connection_log_options_cloudwatch_log_stream: Option<String>,
        security_group_ids: Vec<String>,
        vpc_id: Option<String>,
        self_service_portal: String,
        session_timeout_hours: i32,
        client_login_banner_enabled: bool,
        client_login_banner_text: Option<String>,
        disconnect_on_session_timeout: bool,
        client_route_enforcement_enforced: bool,
        tags: Tags,
    ) -> &ClientVpnEndpoint {
        let id = self.next_client_vpn_endpoint_id();
        let dns_name = format!("{id}.cvpn-endpoint.amazonaws.com");
        let endpoint = ClientVpnEndpoint {
            client_vpn_endpoint_id: id.clone(),
            description,
            status: ClientVpnEndpointStatus {
                code: "pending-associate".to_string(),
                message: None,
            },
            creation_time: self.now_iso(),
            deletion_time: None,
            dns_name,
            client_cidr_block,
            dns_servers,
            split_tunnel,
            vpn_protocol: "openvpn".to_string(),
            transport_protocol,
            vpn_port,
            server_certificate_arn,
            authentication_options,
            connection_log_options_enabled,
            connection_log_options_cloudwatch_log_group,
            connection_log_options_cloudwatch_log_stream,
            tags,
            security_group_ids,
            vpc_id,
            self_service_portal_url: None,
            self_service_portal,
            session_timeout_hours,
            client_login_banner_enabled,
            client_login_banner_text,
            disconnect_on_session_timeout,
            client_route_enforcement_enforced,
            client_certificate_revocation_list: None,
        };
        self.client_vpn_endpoints.insert(id.clone(), endpoint);
        self.client_vpn_endpoints.get(&id).unwrap()
    }

    pub fn delete_client_vpn_endpoint(
        &mut self,
        endpoint_id: &str,
    ) -> Result<ClientVpnEndpointStatus, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let has_assoc = self
            .client_vpn_target_network_associations
            .values()
            .any(|a| a.client_vpn_endpoint_id == endpoint_id);
        let has_routes = self
            .client_vpn_routes
            .keys()
            .any(|(eid, _, _)| eid == endpoint_id);
        if has_assoc || has_routes {
            return Err(Ec2Error::ClientVpnEndpointInUse(endpoint_id.to_string()));
        }
        self.client_vpn_endpoints.remove(endpoint_id);
        Ok(ClientVpnEndpointStatus {
            code: "deleting".to_string(),
            message: None,
        })
    }

    pub fn associate_client_vpn_target_network(
        &mut self,
        endpoint_id: &str,
        subnet_id: &str,
    ) -> Result<&ClientVpnTargetNetworkAssociation, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let vpc_id = self
            .subnets
            .get(subnet_id)
            .map(|s| s.vpc_id.clone())
            .unwrap_or_default();
        let assoc_id = self.next_client_vpn_target_network_association_id();
        let assoc = ClientVpnTargetNetworkAssociation {
            association_id: assoc_id.clone(),
            vpc_id: vpc_id.clone(),
            target_network_id: subnet_id.to_string(),
            client_vpn_endpoint_id: endpoint_id.to_string(),
            security_groups: Vec::new(),
            status: ClientVpnAssociationStatus {
                code: "associating".to_string(),
                message: None,
            },
        };
        self.client_vpn_target_network_associations
            .insert(assoc_id.clone(), assoc);
        // Bind the endpoint to the VPC if not already.
        if let Some(ep) = self.client_vpn_endpoints.get_mut(endpoint_id)
            && ep.vpc_id.is_none()
            && !vpc_id.is_empty()
        {
            ep.vpc_id = Some(vpc_id);
        }
        Ok(self
            .client_vpn_target_network_associations
            .get(&assoc_id)
            .unwrap())
    }

    pub fn disassociate_client_vpn_target_network(
        &mut self,
        endpoint_id: &str,
        association_id: &str,
    ) -> Result<ClientVpnAssociationStatus, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let assoc = self
            .client_vpn_target_network_associations
            .remove(association_id)
            .ok_or_else(|| {
                Ec2Error::InvalidClientVpnTargetNetworkAssociationNotFound(
                    association_id.to_string(),
                )
            })?;
        if assoc.client_vpn_endpoint_id != endpoint_id {
            // Restore and refuse.
            self.client_vpn_target_network_associations
                .insert(association_id.to_string(), assoc);
            return Err(Ec2Error::InvalidClientVpnTargetNetworkAssociationNotFound(
                association_id.to_string(),
            ));
        }
        Ok(ClientVpnAssociationStatus {
            code: "disassociating".to_string(),
            message: None,
        })
    }

    pub fn apply_security_groups_to_client_vpn_target_network(
        &mut self,
        endpoint_id: &str,
        vpc_id: &str,
        security_group_ids: Vec<String>,
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let mut applied = false;
        for assoc in self.client_vpn_target_network_associations.values_mut() {
            if assoc.client_vpn_endpoint_id == endpoint_id && assoc.vpc_id == vpc_id {
                assoc.security_groups = security_group_ids.clone();
                applied = true;
            }
        }
        // Also remember on the endpoint itself.
        if let Some(ep) = self.client_vpn_endpoints.get_mut(endpoint_id) {
            ep.security_group_ids = security_group_ids.clone();
        }
        if !applied {
            // No matching association is fine; AWS still records the change at endpoint scope.
        }
        Ok(security_group_ids)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn authorize_client_vpn_ingress(
        &mut self,
        endpoint_id: &str,
        destination_cidr: String,
        access_all: bool,
        group_id: Option<String>,
        description: Option<String>,
    ) -> Result<ClientVpnAuthorizationRuleStatus, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let key = (
            endpoint_id.to_string(),
            destination_cidr.clone(),
            group_id.clone().unwrap_or_default(),
        );
        let status = ClientVpnAuthorizationRuleStatus {
            code: "active".to_string(),
            message: None,
        };
        let rule = ClientVpnAuthorizationRule {
            client_vpn_endpoint_id: endpoint_id.to_string(),
            group_id,
            access_all,
            destination_cidr,
            description,
            status: status.clone(),
        };
        self.client_vpn_authorization_rules.insert(key, rule);
        Ok(status)
    }

    pub fn revoke_client_vpn_ingress(
        &mut self,
        endpoint_id: &str,
        destination_cidr: &str,
        group_id: Option<String>,
    ) -> Result<ClientVpnAuthorizationRuleStatus, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let key = (
            endpoint_id.to_string(),
            destination_cidr.to_string(),
            group_id.unwrap_or_default(),
        );
        if self.client_vpn_authorization_rules.remove(&key).is_some() {
            Ok(ClientVpnAuthorizationRuleStatus {
                code: "revoking".to_string(),
                message: None,
            })
        } else {
            Err(Ec2Error::InvalidClientVpnAuthorizationRuleNotFound(
                endpoint_id.to_string(),
                destination_cidr.to_string(),
            ))
        }
    }

    pub fn create_client_vpn_route(
        &mut self,
        endpoint_id: &str,
        destination_cidr: String,
        target_subnet: String,
        description: Option<String>,
    ) -> Result<ClientVpnRouteStatus, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let key = (
            endpoint_id.to_string(),
            destination_cidr.clone(),
            target_subnet.clone(),
        );
        let status = ClientVpnRouteStatus {
            code: "creating".to_string(),
            message: None,
        };
        let route = ClientVpnRoute {
            client_vpn_endpoint_id: endpoint_id.to_string(),
            destination_cidr,
            target_subnet,
            r#type: "Nat".to_string(),
            origin: "add-route".to_string(),
            status: status.clone(),
            description,
        };
        self.client_vpn_routes.insert(key, route);
        Ok(status)
    }

    pub fn delete_client_vpn_route(
        &mut self,
        endpoint_id: &str,
        destination_cidr: &str,
        target_subnet: &str,
    ) -> Result<ClientVpnRouteStatus, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let key = (
            endpoint_id.to_string(),
            destination_cidr.to_string(),
            target_subnet.to_string(),
        );
        if self.client_vpn_routes.remove(&key).is_some() {
            Ok(ClientVpnRouteStatus {
                code: "deleting".to_string(),
                message: None,
            })
        } else {
            Err(Ec2Error::InvalidClientVpnRouteNotFound(
                endpoint_id.to_string(),
                destination_cidr.to_string(),
                target_subnet.to_string(),
            ))
        }
    }

    pub fn import_client_vpn_revocation_list(
        &mut self,
        endpoint_id: &str,
        crl: String,
    ) -> Result<(), Ec2Error> {
        let ep = self
            .client_vpn_endpoints
            .get_mut(endpoint_id)
            .ok_or_else(|| Ec2Error::InvalidClientVpnEndpointNotFound(endpoint_id.to_string()))?;
        ep.client_certificate_revocation_list = Some(crl);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_client_vpn_endpoint(
        &mut self,
        endpoint_id: &str,
        server_certificate_arn: Option<String>,
        connection_log_options_enabled: Option<bool>,
        connection_log_options_cloudwatch_log_group: Option<String>,
        connection_log_options_cloudwatch_log_stream: Option<String>,
        dns_servers_custom: Option<Vec<String>>,
        dns_servers_enabled: Option<bool>,
        vpn_port: Option<i32>,
        description: Option<String>,
        split_tunnel: Option<bool>,
        security_group_ids: Option<Vec<String>>,
        vpc_id: Option<String>,
        self_service_portal: Option<String>,
        session_timeout_hours: Option<i32>,
        client_login_banner_enabled: Option<bool>,
        client_login_banner_text: Option<String>,
        disconnect_on_session_timeout: Option<bool>,
        client_route_enforcement_enforced: Option<bool>,
    ) -> Result<(), Ec2Error> {
        let ep = self
            .client_vpn_endpoints
            .get_mut(endpoint_id)
            .ok_or_else(|| Ec2Error::InvalidClientVpnEndpointNotFound(endpoint_id.to_string()))?;
        if let Some(s) = server_certificate_arn {
            ep.server_certificate_arn = s;
        }
        if let Some(b) = connection_log_options_enabled {
            ep.connection_log_options_enabled = b;
        }
        if connection_log_options_cloudwatch_log_group.is_some() {
            ep.connection_log_options_cloudwatch_log_group =
                connection_log_options_cloudwatch_log_group;
        }
        if connection_log_options_cloudwatch_log_stream.is_some() {
            ep.connection_log_options_cloudwatch_log_stream =
                connection_log_options_cloudwatch_log_stream;
        }
        if let Some(enabled) = dns_servers_enabled {
            if !enabled {
                ep.dns_servers.clear();
            } else if let Some(custom) = dns_servers_custom {
                ep.dns_servers = custom;
            }
        } else if let Some(custom) = dns_servers_custom {
            ep.dns_servers = custom;
        }
        if let Some(p) = vpn_port {
            ep.vpn_port = p;
        }
        if description.is_some() {
            ep.description = description;
        }
        if let Some(b) = split_tunnel {
            ep.split_tunnel = b;
        }
        if let Some(s) = security_group_ids {
            ep.security_group_ids = s;
        }
        if vpc_id.is_some() {
            ep.vpc_id = vpc_id;
        }
        if let Some(s) = self_service_portal {
            ep.self_service_portal = s;
        }
        if let Some(h) = session_timeout_hours {
            ep.session_timeout_hours = h;
        }
        if let Some(b) = client_login_banner_enabled {
            ep.client_login_banner_enabled = b;
        }
        if client_login_banner_text.is_some() {
            ep.client_login_banner_text = client_login_banner_text;
        }
        if let Some(b) = disconnect_on_session_timeout {
            ep.disconnect_on_session_timeout = b;
        }
        if let Some(b) = client_route_enforcement_enforced {
            ep.client_route_enforcement_enforced = b;
        }
        Ok(())
    }

    /// Seeds a client VPN connection record. Used by tests.
    #[allow(dead_code)]
    pub fn seed_client_vpn_connection(
        &mut self,
        endpoint_id: &str,
        username: Option<String>,
    ) -> String {
        let id = self.next_client_vpn_connection_id();
        let now = self.now_iso();
        let conn = ClientVpnConnection {
            connection_id: id.clone(),
            client_vpn_endpoint_id: endpoint_id.to_string(),
            username,
            status: ClientVpnConnectionStatus {
                code: "active".to_string(),
                message: None,
            },
            posture_compliance_statuses: Vec::new(),
            common_name: None,
            connection_established_time: now.clone(),
            connection_end_time: None,
            ingress_bytes: "0".to_string(),
            egress_bytes: "0".to_string(),
            ingress_packets: "0".to_string(),
            egress_packets: "0".to_string(),
            client_ip: None,
            client_port: None,
            timestamp: now,
        };
        self.client_vpn_connections.insert(id.clone(), conn);
        id
    }

    /// Terminate connections by id list (fallback by username if list empty).
    /// Returns the (current, previous) status per connection.
    pub fn terminate_client_vpn_connections(
        &mut self,
        endpoint_id: &str,
        connection_ids: Vec<String>,
        username: Option<String>,
    ) -> Result<Vec<(String, ClientVpnConnectionStatus, ClientVpnConnectionStatus)>, Ec2Error> {
        if !self.client_vpn_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidClientVpnEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let mut out = Vec::new();
        let to_terminate: Vec<String> = if !connection_ids.is_empty() {
            connection_ids
        } else if let Some(u) = username.as_ref() {
            self.client_vpn_connections
                .values()
                .filter(|c| {
                    c.client_vpn_endpoint_id == endpoint_id
                        && c.username.as_deref() == Some(u.as_str())
                })
                .map(|c| c.connection_id.clone())
                .collect()
        } else {
            Vec::new()
        };
        for id in to_terminate {
            if let Some(conn) = self.client_vpn_connections.get_mut(&id) {
                let prev = conn.status.clone();
                conn.status = ClientVpnConnectionStatus {
                    code: "terminating".to_string(),
                    message: None,
                };
                out.push((id, conn.status.clone(), prev));
            }
        }
        Ok(out)
    }

    // ===== Group 7: Local Gateway =====

    fn next_local_gateway_id(&mut self) -> String {
        self.counters.local_gateway += 1;
        format!("lgw-{:08x}", self.counters.local_gateway)
    }

    fn next_local_gateway_route_table_id(&mut self) -> String {
        self.counters.local_gateway_route_table += 1;
        format!("lgw-rtb-{:08x}", self.counters.local_gateway_route_table)
    }

    fn next_local_gateway_route_table_virtual_interface_group_association_id(&mut self) -> String {
        self.counters
            .local_gateway_route_table_virtual_interface_group_association += 1;
        format!(
            "lgw-vif-grp-assoc-{:08x}",
            self.counters
                .local_gateway_route_table_virtual_interface_group_association
        )
    }

    fn next_local_gateway_route_table_vpc_association_id(&mut self) -> String {
        self.counters.local_gateway_route_table_vpc_association += 1;
        format!(
            "lgw-vpc-assoc-{:08x}",
            self.counters.local_gateway_route_table_vpc_association
        )
    }

    fn next_local_gateway_virtual_interface_id(&mut self) -> String {
        self.counters.local_gateway_virtual_interface += 1;
        format!(
            "lgw-vif-{:08x}",
            self.counters.local_gateway_virtual_interface
        )
    }

    fn next_local_gateway_virtual_interface_group_id(&mut self) -> String {
        self.counters.local_gateway_virtual_interface_group += 1;
        format!(
            "lgw-vif-grp-{:08x}",
            self.counters.local_gateway_virtual_interface_group
        )
    }

    /// Seeds a local gateway record (used by tests / handlers when AWS does not
    /// expose a `Create*` API for the parent gateway).
    pub fn seed_local_gateway(&mut self, owner_id: &str) -> String {
        let id = self.next_local_gateway_id();
        let lgw = LocalGateway {
            local_gateway_id: id.clone(),
            outpost_arn: format!(
                "arn:aws:outposts:us-east-1:{owner_id}:outpost/op-{:08x}",
                self.counters.local_gateway
            ),
            owner_id: owner_id.to_string(),
            state: "available".to_string(),
            tags: Tags::new(),
        };
        self.local_gateways.insert(id.clone(), lgw);
        id
    }

    /// Seed a `LocalGatewayRouteTable` with the caller-supplied ID, also seeding a paired
    /// `LocalGateway` if needed. No-op when the route table already exists. Used to keep
    /// `CreateLocalGatewayRoute` and `CreateLocalGatewayRouteTableVpcAssociation` working when
    /// the caller passes a pre-existing route-table ID (real AWS pre-creates these for Outposts).
    pub fn seed_local_gateway_route_table_with_id(&mut self, table_id: &str, owner_id: &str) {
        if self.local_gateway_route_tables.contains_key(table_id) {
            return;
        }
        let lgw_id = table_id
            .strip_prefix("lgw-rtb-")
            .map(|suffix| format!("lgw-{suffix}"))
            .unwrap_or_else(|| format!("lgw-injected-{table_id}"));
        if !self.local_gateways.contains_key(&lgw_id) {
            self.local_gateways.insert(
                lgw_id.clone(),
                LocalGateway {
                    local_gateway_id: lgw_id.clone(),
                    outpost_arn: format!(
                        "arn:aws:outposts:us-east-1:{owner_id}:outpost/op-injected"
                    ),
                    owner_id: owner_id.to_string(),
                    state: "available".to_string(),
                    tags: Tags::new(),
                },
            );
        }
        let arn = format!("arn:aws:ec2:us-east-1:{owner_id}:local-gateway-route-table/{table_id}");
        self.local_gateway_route_tables.insert(
            table_id.to_string(),
            LocalGatewayRouteTable {
                local_gateway_route_table_id: table_id.to_string(),
                local_gateway_route_table_arn: arn,
                local_gateway_id: lgw_id,
                owner_id: owner_id.to_string(),
                state: "available".to_string(),
                mode: "direct-vpc-routing".to_string(),
                tags: Tags::new(),
                state_reason_code: None,
                state_reason_message: None,
            },
        );
    }

    pub fn create_local_gateway_route_table(
        &mut self,
        local_gateway_id: &str,
        mode: Option<String>,
        owner_id: &str,
        tags: Tags,
    ) -> Result<&LocalGatewayRouteTable, Ec2Error> {
        if !self.local_gateways.contains_key(local_gateway_id) {
            return Err(Ec2Error::InvalidLocalGatewayNotFound(
                local_gateway_id.to_string(),
            ));
        }
        let id = self.next_local_gateway_route_table_id();
        let arn = format!("arn:aws:ec2:us-east-1:{owner_id}:local-gateway-route-table/{id}");
        let table = LocalGatewayRouteTable {
            local_gateway_route_table_id: id.clone(),
            local_gateway_route_table_arn: arn,
            local_gateway_id: local_gateway_id.to_string(),
            owner_id: owner_id.to_string(),
            state: "available".to_string(),
            mode: mode.unwrap_or_else(|| "direct-vpc-routing".to_string()),
            tags,
            state_reason_code: None,
            state_reason_message: None,
        };
        self.local_gateway_route_tables.insert(id.clone(), table);
        Ok(self.local_gateway_route_tables.get(&id).unwrap())
    }

    pub fn delete_local_gateway_route_table(
        &mut self,
        table_id: &str,
    ) -> Result<LocalGatewayRouteTable, Ec2Error> {
        if !self.local_gateway_route_tables.contains_key(table_id) {
            return Err(Ec2Error::InvalidLocalGatewayRouteTableNotFound(
                table_id.to_string(),
            ));
        }
        let has_routes = self
            .local_gateway_routes
            .keys()
            .any(|(rid, _)| rid == table_id);
        if has_routes {
            return Err(Ec2Error::LocalGatewayRouteTableInUse(table_id.to_string()));
        }
        let mut t = self.local_gateway_route_tables.remove(table_id).unwrap();
        t.state = "deleting".to_string();
        Ok(t)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_local_gateway_route(
        &mut self,
        local_gateway_route_table_id: &str,
        destination_cidr_block: String,
        owner_id: &str,
        local_gateway_virtual_interface_group_id: Option<String>,
        network_interface_id: Option<String>,
        subnet_id: Option<String>,
        destination_prefix_list_id: Option<String>,
        coip_pool_id: Option<String>,
    ) -> Result<&LocalGatewayRoute, Ec2Error> {
        let table_arn = match self
            .local_gateway_route_tables
            .get(local_gateway_route_table_id)
        {
            Some(t) => Some(t.local_gateway_route_table_arn.clone()),
            None => {
                return Err(Ec2Error::InvalidLocalGatewayRouteTableNotFound(
                    local_gateway_route_table_id.to_string(),
                ));
            }
        };
        let r#type = "static".to_string();
        let route = LocalGatewayRoute {
            destination_cidr_block: destination_cidr_block.clone(),
            local_gateway_route_table_id: local_gateway_route_table_id.to_string(),
            r#type,
            state: "active".to_string(),
            local_gateway_route_table_arn: table_arn,
            owner_id: owner_id.to_string(),
            subnet_id,
            network_interface_id,
            destination_prefix_list_id,
            coip_pool_id,
            local_gateway_virtual_interface_group_id,
        };
        let key = (
            local_gateway_route_table_id.to_string(),
            destination_cidr_block,
        );
        self.local_gateway_routes.insert(key.clone(), route);
        Ok(self.local_gateway_routes.get(&key).unwrap())
    }

    pub fn delete_local_gateway_route(
        &mut self,
        table_id: &str,
        destination_cidr_block: &str,
    ) -> Result<LocalGatewayRoute, Ec2Error> {
        let key = (table_id.to_string(), destination_cidr_block.to_string());
        let mut r = self.local_gateway_routes.remove(&key).ok_or_else(|| {
            Ec2Error::InvalidLocalGatewayRouteNotFound(
                table_id.to_string(),
                destination_cidr_block.to_string(),
            )
        })?;
        r.state = "deleted".to_string();
        Ok(r)
    }

    pub fn modify_local_gateway_route(
        &mut self,
        table_id: &str,
        destination_cidr_block: &str,
        new_subnet_id: Option<String>,
        new_network_interface_id: Option<String>,
    ) -> Result<&LocalGatewayRoute, Ec2Error> {
        let key = (table_id.to_string(), destination_cidr_block.to_string());
        let route = self.local_gateway_routes.get_mut(&key).ok_or_else(|| {
            Ec2Error::InvalidLocalGatewayRouteNotFound(
                table_id.to_string(),
                destination_cidr_block.to_string(),
            )
        })?;
        if new_subnet_id.is_some() {
            route.subnet_id = new_subnet_id;
        }
        if new_network_interface_id.is_some() {
            route.network_interface_id = new_network_interface_id;
        }
        Ok(self.local_gateway_routes.get(&key).unwrap())
    }

    pub fn create_local_gateway_virtual_interface(
        &mut self,
        local_gateway_virtual_interface_group_id: &str,
        vlan: i32,
        local_address: String,
        peer_address: String,
        peer_bgp_asn: Option<i32>,
        peer_bgp_asn_extended: Option<i64>,
        owner_id: &str,
        tags: Tags,
    ) -> Result<&LocalGatewayVirtualInterface, Ec2Error> {
        // Resolve LG via group.
        let group = self
            .local_gateway_virtual_interface_groups
            .get(local_gateway_virtual_interface_group_id)
            .ok_or_else(|| {
                Ec2Error::InvalidLocalGatewayVirtualInterfaceGroupNotFound(
                    local_gateway_virtual_interface_group_id.to_string(),
                )
            })?;
        let local_gateway_id = group.local_gateway_id.clone();
        let local_bgp_asn = group.local_bgp_asn;
        let id = self.next_local_gateway_virtual_interface_id();
        let arn = format!("arn:aws:ec2:us-east-1:{owner_id}:local-gateway-virtual-interface/{id}");
        let vif = LocalGatewayVirtualInterface {
            local_gateway_virtual_interface_id: id.clone(),
            local_gateway_id,
            vlan,
            local_address,
            peer_address,
            local_bgp_asn,
            peer_bgp_asn: peer_bgp_asn.unwrap_or(0),
            owner_id: owner_id.to_string(),
            tags,
            configuration_state: "available".to_string(),
            peer_bgp_asn_extended,
            local_gateway_virtual_interface_arn: Some(arn),
        };
        self.local_gateway_virtual_interfaces
            .insert(id.clone(), vif);
        // Link into the group.
        let group = self
            .local_gateway_virtual_interface_groups
            .get_mut(local_gateway_virtual_interface_group_id)
            .unwrap();
        group.local_gateway_virtual_interface_ids.push(id.clone());
        Ok(self.local_gateway_virtual_interfaces.get(&id).unwrap())
    }

    pub fn delete_local_gateway_virtual_interface(
        &mut self,
        vif_id: &str,
    ) -> Result<LocalGatewayVirtualInterface, Ec2Error> {
        if !self.local_gateway_virtual_interfaces.contains_key(vif_id) {
            return Err(Ec2Error::InvalidLocalGatewayVirtualInterfaceNotFound(
                vif_id.to_string(),
            ));
        }
        // Refuse if still part of a group.
        if self
            .local_gateway_virtual_interface_groups
            .values()
            .any(|g| {
                g.local_gateway_virtual_interface_ids
                    .iter()
                    .any(|i| i == vif_id)
            })
        {
            return Err(Ec2Error::LocalGatewayVirtualInterfaceInUse(
                vif_id.to_string(),
            ));
        }
        let v = self
            .local_gateway_virtual_interfaces
            .remove(vif_id)
            .unwrap();
        Ok(v)
    }

    pub fn create_local_gateway_virtual_interface_group(
        &mut self,
        local_gateway_id: &str,
        local_bgp_asn: i32,
        local_bgp_asn_extended: Option<i64>,
        owner_id: &str,
        tags: Tags,
    ) -> Result<&LocalGatewayVirtualInterfaceGroup, Ec2Error> {
        if !self.local_gateways.contains_key(local_gateway_id) {
            return Err(Ec2Error::InvalidLocalGatewayNotFound(
                local_gateway_id.to_string(),
            ));
        }
        let id = self.next_local_gateway_virtual_interface_group_id();
        let arn =
            format!("arn:aws:ec2:us-east-1:{owner_id}:local-gateway-virtual-interface-group/{id}");
        let group = LocalGatewayVirtualInterfaceGroup {
            local_gateway_virtual_interface_group_id: id.clone(),
            local_gateway_virtual_interface_ids: Vec::new(),
            local_gateway_id: local_gateway_id.to_string(),
            owner_id: owner_id.to_string(),
            tags,
            configuration_state: Some("available".to_string()),
            local_bgp_asn,
            local_bgp_asn_extended,
            local_gateway_virtual_interface_group_arn: Some(arn),
        };
        self.local_gateway_virtual_interface_groups
            .insert(id.clone(), group);
        Ok(self
            .local_gateway_virtual_interface_groups
            .get(&id)
            .unwrap())
    }

    pub fn delete_local_gateway_virtual_interface_group(
        &mut self,
        group_id: &str,
    ) -> Result<LocalGatewayVirtualInterfaceGroup, Ec2Error> {
        let g = self
            .local_gateway_virtual_interface_groups
            .get(group_id)
            .ok_or_else(|| {
                Ec2Error::InvalidLocalGatewayVirtualInterfaceGroupNotFound(group_id.to_string())
            })?;
        if !g.local_gateway_virtual_interface_ids.is_empty() {
            return Err(Ec2Error::LocalGatewayVirtualInterfaceGroupInUse(
                group_id.to_string(),
            ));
        }
        // Refuse if a route table association references this group.
        if self
            .local_gateway_route_table_virtual_interface_group_associations
            .values()
            .any(|a| a.local_gateway_virtual_interface_group_id == group_id)
        {
            return Err(Ec2Error::LocalGatewayVirtualInterfaceGroupInUse(
                group_id.to_string(),
            ));
        }
        let g = self
            .local_gateway_virtual_interface_groups
            .remove(group_id)
            .unwrap();
        Ok(g)
    }

    pub fn create_local_gateway_route_table_virtual_interface_group_association(
        &mut self,
        local_gateway_route_table_id: &str,
        local_gateway_virtual_interface_group_id: &str,
        owner_id: &str,
        tags: Tags,
    ) -> Result<&LocalGatewayRouteTableVirtualInterfaceGroupAssociation, Ec2Error> {
        let table = self
            .local_gateway_route_tables
            .get(local_gateway_route_table_id)
            .ok_or_else(|| {
                Ec2Error::InvalidLocalGatewayRouteTableNotFound(
                    local_gateway_route_table_id.to_string(),
                )
            })?;
        let local_gateway_id = table.local_gateway_id.clone();
        let local_gateway_route_table_arn = table.local_gateway_route_table_arn.clone();
        if !self
            .local_gateway_virtual_interface_groups
            .contains_key(local_gateway_virtual_interface_group_id)
        {
            return Err(Ec2Error::InvalidLocalGatewayVirtualInterfaceGroupNotFound(
                local_gateway_virtual_interface_group_id.to_string(),
            ));
        }
        let id = self.next_local_gateway_route_table_virtual_interface_group_association_id();
        let assoc = LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
            local_gateway_route_table_virtual_interface_group_association_id: id.clone(),
            local_gateway_virtual_interface_group_id: local_gateway_virtual_interface_group_id
                .to_string(),
            local_gateway_route_table_id: local_gateway_route_table_id.to_string(),
            local_gateway_route_table_arn,
            local_gateway_id,
            owner_id: owner_id.to_string(),
            state: "associated".to_string(),
            tags,
        };
        self.local_gateway_route_table_virtual_interface_group_associations
            .insert(id.clone(), assoc);
        Ok(self
            .local_gateway_route_table_virtual_interface_group_associations
            .get(&id)
            .unwrap())
    }

    pub fn delete_local_gateway_route_table_virtual_interface_group_association(
        &mut self,
        association_id: &str,
    ) -> Result<LocalGatewayRouteTableVirtualInterfaceGroupAssociation, Ec2Error> {
        let mut a = self
            .local_gateway_route_table_virtual_interface_group_associations
            .remove(association_id)
            .ok_or_else(|| {
                Ec2Error::InvalidLocalGatewayRouteTableVirtualInterfaceGroupAssociationNotFound(
                    association_id.to_string(),
                )
            })?;
        a.state = "disassociated".to_string();
        Ok(a)
    }

    pub fn create_local_gateway_route_table_vpc_association(
        &mut self,
        local_gateway_route_table_id: &str,
        vpc_id: &str,
        owner_id: &str,
        tags: Tags,
    ) -> Result<&LocalGatewayRouteTableVpcAssociation, Ec2Error> {
        let table = self
            .local_gateway_route_tables
            .get(local_gateway_route_table_id)
            .ok_or_else(|| {
                Ec2Error::InvalidLocalGatewayRouteTableNotFound(
                    local_gateway_route_table_id.to_string(),
                )
            })?;
        let local_gateway_id = table.local_gateway_id.clone();
        let local_gateway_route_table_arn = table.local_gateway_route_table_arn.clone();
        let id = self.next_local_gateway_route_table_vpc_association_id();
        let assoc = LocalGatewayRouteTableVpcAssociation {
            local_gateway_route_table_vpc_association_id: id.clone(),
            local_gateway_route_table_id: local_gateway_route_table_id.to_string(),
            local_gateway_route_table_arn,
            local_gateway_id,
            vpc_id: vpc_id.to_string(),
            owner_id: owner_id.to_string(),
            state: "associated".to_string(),
            tags,
        };
        self.local_gateway_route_table_vpc_associations
            .insert(id.clone(), assoc);
        Ok(self
            .local_gateway_route_table_vpc_associations
            .get(&id)
            .unwrap())
    }

    pub fn delete_local_gateway_route_table_vpc_association(
        &mut self,
        association_id: &str,
    ) -> Result<LocalGatewayRouteTableVpcAssociation, Ec2Error> {
        let mut a = self
            .local_gateway_route_table_vpc_associations
            .remove(association_id)
            .ok_or_else(|| {
                Ec2Error::InvalidLocalGatewayRouteTableVpcAssociationNotFound(
                    association_id.to_string(),
                )
            })?;
        a.state = "disassociated".to_string();
        Ok(a)
    }

    // ===== Group 8: Route Server =====

    fn next_route_server_id(&mut self) -> String {
        self.counters.route_server += 1;
        format!("rs-{:08x}", self.counters.route_server)
    }

    fn next_route_server_endpoint_id(&mut self) -> String {
        self.counters.route_server_endpoint += 1;
        format!("rse-{:08x}", self.counters.route_server_endpoint)
    }

    fn next_route_server_peer_id(&mut self) -> String {
        self.counters.route_server_peer += 1;
        format!("rsp-{:08x}", self.counters.route_server_peer)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_route_server(
        &mut self,
        amazon_side_asn: i64,
        persist_routes: Option<String>,
        persist_routes_duration: Option<i64>,
        sns_notifications_enabled: Option<bool>,
        owner_id: &str,
        tags: Tags,
    ) -> &RouteServer {
        let id = self.next_route_server_id();
        let arn = format!("arn:aws:ec2:us-east-1:{owner_id}:route-server/{id}");
        let rs = RouteServer {
            route_server_id: id.clone(),
            route_server_arn: arn,
            amazon_side_asn,
            state: "available".to_string(),
            // Action ("enable" / "disable" / "reset") -> state ("enabled" /
            // "disabled" / "resetting") per AWS semantics.
            persist_routes: action_to_persist_routes_state(persist_routes.as_deref()),
            persist_routes_duration,
            sns_notifications_enabled: sns_notifications_enabled.unwrap_or(false),
            sns_topic_arn: None,
            tags,
        };
        self.route_servers.insert(id.clone(), rs);
        self.route_servers.get(&id).unwrap()
    }

    pub fn delete_route_server(&mut self, route_server_id: &str) -> Result<RouteServer, Ec2Error> {
        if !self.route_servers.contains_key(route_server_id) {
            return Err(Ec2Error::InvalidRouteServerNotFound(
                route_server_id.to_string(),
            ));
        }
        let has_endpoints = self
            .route_server_endpoints
            .values()
            .any(|e| e.route_server_id == route_server_id);
        let has_associations = self
            .route_server_associations
            .keys()
            .any(|(rsid, _)| rsid == route_server_id);
        if has_endpoints || has_associations {
            return Err(Ec2Error::RouteServerInUse(route_server_id.to_string()));
        }
        let mut rs = self.route_servers.remove(route_server_id).unwrap();
        rs.state = "deleting".to_string();
        Ok(rs)
    }

    pub fn modify_route_server(
        &mut self,
        route_server_id: &str,
        persist_routes: Option<String>,
        persist_routes_duration: Option<i64>,
        sns_notifications_enabled: Option<bool>,
    ) -> Result<&RouteServer, Ec2Error> {
        let rs = self
            .route_servers
            .get_mut(route_server_id)
            .ok_or_else(|| Ec2Error::InvalidRouteServerNotFound(route_server_id.to_string()))?;
        if let Some(p) = persist_routes {
            rs.persist_routes = action_to_persist_routes_state(Some(p.as_str()));
        }
        if let Some(d) = persist_routes_duration {
            rs.persist_routes_duration = Some(d);
        }
        if let Some(e) = sns_notifications_enabled {
            rs.sns_notifications_enabled = e;
        }
        Ok(self.route_servers.get(route_server_id).unwrap())
    }

    pub fn associate_route_server(
        &mut self,
        route_server_id: &str,
        vpc_id: &str,
    ) -> Result<&RouteServerAssociation, Ec2Error> {
        if !self.route_servers.contains_key(route_server_id) {
            return Err(Ec2Error::InvalidRouteServerNotFound(
                route_server_id.to_string(),
            ));
        }
        if !self.vpcs.contains_key(vpc_id) {
            return Err(Ec2Error::VpcNotFound(vpc_id.to_string()));
        }
        let key = (route_server_id.to_string(), vpc_id.to_string());
        let assoc = RouteServerAssociation {
            route_server_id: route_server_id.to_string(),
            vpc_id: vpc_id.to_string(),
            state: "associated".to_string(),
            propagations: Vec::new(),
        };
        self.route_server_associations.insert(key.clone(), assoc);
        Ok(self.route_server_associations.get(&key).unwrap())
    }

    pub fn disassociate_route_server(
        &mut self,
        route_server_id: &str,
        vpc_id: &str,
    ) -> Result<RouteServerAssociation, Ec2Error> {
        if !self.route_servers.contains_key(route_server_id) {
            return Err(Ec2Error::InvalidRouteServerNotFound(
                route_server_id.to_string(),
            ));
        }
        let key = (route_server_id.to_string(), vpc_id.to_string());
        let mut a = self.route_server_associations.remove(&key).ok_or_else(|| {
            Ec2Error::InvalidRouteServerAssociationNotFound(
                route_server_id.to_string(),
                vpc_id.to_string(),
            )
        })?;
        a.state = "disassociated".to_string();
        Ok(a)
    }

    pub fn create_route_server_endpoint(
        &mut self,
        route_server_id: &str,
        subnet_id: &str,
        tags: Tags,
    ) -> Result<&RouteServerEndpoint, Ec2Error> {
        if !self.route_servers.contains_key(route_server_id) {
            return Err(Ec2Error::InvalidRouteServerNotFound(
                route_server_id.to_string(),
            ));
        }
        let vpc_id = self
            .subnets
            .get(subnet_id)
            .map(|s| s.vpc_id.clone())
            .unwrap_or_default();
        let id = self.next_route_server_endpoint_id();
        // Synthesise an ENI ID (we don't materialise an actual NetworkInterface;
        // that would require copying the existing eni_to_model machinery and
        // is not exercised by any of the Route-Server-only ops in this batch).
        let eni_id = format!("eni-rse{:08x}", self.counters.route_server_endpoint);
        let endpoint = RouteServerEndpoint {
            route_server_endpoint_id: id.clone(),
            route_server_id: route_server_id.to_string(),
            vpc_id,
            subnet_id: subnet_id.to_string(),
            eni_id,
            eni_address: None,
            state: "available".to_string(),
            failure_reason: None,
            tags,
        };
        self.route_server_endpoints.insert(id.clone(), endpoint);
        Ok(self.route_server_endpoints.get(&id).unwrap())
    }

    pub fn delete_route_server_endpoint(
        &mut self,
        endpoint_id: &str,
    ) -> Result<RouteServerEndpoint, Ec2Error> {
        if !self.route_server_endpoints.contains_key(endpoint_id) {
            return Err(Ec2Error::InvalidRouteServerEndpointNotFound(
                endpoint_id.to_string(),
            ));
        }
        let has_peers = self
            .route_server_peers
            .values()
            .any(|p| p.route_server_endpoint_id == endpoint_id);
        if has_peers {
            return Err(Ec2Error::RouteServerEndpointInUse(endpoint_id.to_string()));
        }
        let mut ep = self.route_server_endpoints.remove(endpoint_id).unwrap();
        ep.state = "deleting".to_string();
        Ok(ep)
    }

    pub fn create_route_server_peer(
        &mut self,
        endpoint_id: &str,
        peer_address: String,
        peer_asn: i64,
        peer_liveness_detection: Option<String>,
        tags: Tags,
    ) -> Result<&RouteServerPeer, Ec2Error> {
        let endpoint = self
            .route_server_endpoints
            .get(endpoint_id)
            .ok_or_else(|| Ec2Error::InvalidRouteServerEndpointNotFound(endpoint_id.to_string()))?;
        let route_server_id = endpoint.route_server_id.clone();
        let vpc_id = endpoint.vpc_id.clone();
        let subnet_id = endpoint.subnet_id.clone();
        let endpoint_eni_id = Some(endpoint.eni_id.clone());
        let endpoint_eni_address = endpoint.eni_address.clone();
        let id = self.next_route_server_peer_id();
        let liveness = peer_liveness_detection.unwrap_or_else(|| "bgp-keepalive".to_string());
        let bgp_options = RouteServerBgpOptions {
            peer_asn: Some(peer_asn),
            peer_liveness_detection: Some(liveness.clone()),
        };
        let options = RouteServerPeerOptions {
            peer_asn,
            peer_liveness_detection: liveness,
            bgp_options: Some(bgp_options),
        };
        let peer = RouteServerPeer {
            route_server_peer_id: id.clone(),
            route_server_endpoint_id: endpoint_id.to_string(),
            route_server_id,
            vpc_id,
            subnet_id,
            peer_address,
            state: "available".to_string(),
            failure_reason: None,
            options,
            endpoint_eni_id,
            endpoint_eni_address,
            tags,
        };
        self.route_server_peers.insert(id.clone(), peer);
        Ok(self.route_server_peers.get(&id).unwrap())
    }

    pub fn delete_route_server_peer(&mut self, peer_id: &str) -> Result<RouteServerPeer, Ec2Error> {
        let mut p = self
            .route_server_peers
            .remove(peer_id)
            .ok_or_else(|| Ec2Error::InvalidRouteServerPeerNotFound(peer_id.to_string()))?;
        p.state = "deleting".to_string();
        Ok(p)
    }

    // ===== Group 9: Verified Access =====

    fn next_verified_access_instance_id(&mut self) -> String {
        self.counters.verified_access_instance += 1;
        format!("vai-{:08x}", self.counters.verified_access_instance)
    }

    fn next_verified_access_trust_provider_id(&mut self) -> String {
        self.counters.verified_access_trust_provider += 1;
        format!("vatp-{:08x}", self.counters.verified_access_trust_provider)
    }

    fn next_verified_access_group_id(&mut self) -> String {
        self.counters.verified_access_group += 1;
        format!("vagr-{:08x}", self.counters.verified_access_group)
    }

    fn next_verified_access_endpoint_id(&mut self) -> String {
        self.counters.verified_access_endpoint += 1;
        format!("vae-{:08x}", self.counters.verified_access_endpoint)
    }

    pub fn create_verified_access_instance(
        &mut self,
        description: Option<String>,
        fips_enabled: bool,
        cidr_endpoints_custom_subdomain: Option<String>,
        name: Option<String>,
        tags: Tags,
    ) -> &VerifiedAccessInstance {
        let id = self.next_verified_access_instance_id();
        let now = self.now_iso();
        let inst = VerifiedAccessInstance {
            verified_access_instance_id: id.clone(),
            description,
            creation_time: now.clone(),
            last_updated_time: now,
            fips_enabled,
            cidr_endpoints_custom_subdomain,
            name,
            trust_provider_ids: Vec::new(),
            tags,
        };
        self.verified_access_instances.insert(id.clone(), inst);
        self.verified_access_instances.get(&id).unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_verified_access_trust_provider(
        &mut self,
        description: Option<String>,
        trust_provider_type: String,
        user_trust_provider_type: Option<String>,
        device_trust_provider_type: Option<String>,
        oidc_options: Option<VerifiedAccessOidcOptions>,
        device_options: Option<VerifiedAccessDeviceOptions>,
        native_application_oidc_options: Option<VerifiedAccessNativeApplicationOidcOptions>,
        policy_reference_name: String,
        sse_specification: VerifiedAccessSseSpecification,
        tags: Tags,
    ) -> &VerifiedAccessTrustProvider {
        let id = self.next_verified_access_trust_provider_id();
        let now = self.now_iso();
        let tp = VerifiedAccessTrustProvider {
            verified_access_trust_provider_id: id.clone(),
            description,
            trust_provider_type,
            user_trust_provider_type,
            device_trust_provider_type,
            oidc_options,
            device_options,
            native_application_oidc_options,
            policy_reference_name,
            creation_time: now.clone(),
            last_updated_time: now,
            sse_specification,
            tags,
        };
        self.verified_access_trust_providers.insert(id.clone(), tp);
        self.verified_access_trust_providers.get(&id).unwrap()
    }

    pub fn create_verified_access_group(
        &mut self,
        instance_id: String,
        owner: String,
        description: Option<String>,
        sse_specification: VerifiedAccessSseSpecification,
        policy_document: Option<String>,
        tags: Tags,
    ) -> Result<&VerifiedAccessGroup, Ec2Error> {
        if !self.verified_access_instances.contains_key(&instance_id) {
            return Err(Ec2Error::InvalidVerifiedAccessInstanceNotFound(
                instance_id.clone(),
            ));
        }
        let id = self.next_verified_access_group_id();
        let arn = format!(
            "arn:aws:ec2:us-east-1:{owner}:verified-access-group/{id}",
            owner = owner,
            id = id,
        );
        let now = self.now_iso();
        let policy_enabled = policy_document.is_some();
        let group = VerifiedAccessGroup {
            verified_access_group_id: id.clone(),
            verified_access_group_arn: arn,
            verified_access_instance_id: instance_id,
            owner,
            description,
            creation_time: now.clone(),
            last_updated_time: now,
            deletion_time: None,
            sse_specification,
            policy_document,
            policy_enabled,
            tags,
        };
        self.verified_access_groups.insert(id.clone(), group);
        Ok(self.verified_access_groups.get(&id).unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_verified_access_endpoint(
        &mut self,
        verified_access_group_id: String,
        application_domain: Option<String>,
        endpoint_type: String,
        attachment_type: String,
        domain_certificate_arn: Option<String>,
        endpoint_domain_prefix: Option<String>,
        security_group_ids: Vec<String>,
        load_balancer_options: Option<VerifiedAccessEndpointLoadBalancerOptions>,
        network_interface_options: Option<VerifiedAccessEndpointEniOptions>,
        cidr_options: Option<VerifiedAccessEndpointCidrOptions>,
        rds_options: Option<VerifiedAccessEndpointRdsOptions>,
        description: Option<String>,
        policy_document: Option<String>,
        sse_specification: VerifiedAccessSseSpecification,
        tags: Tags,
    ) -> Result<&VerifiedAccessEndpoint, Ec2Error> {
        let group = self
            .verified_access_groups
            .get(&verified_access_group_id)
            .ok_or_else(|| {
                Ec2Error::InvalidVerifiedAccessGroupNotFound(verified_access_group_id.clone())
            })?;
        let instance_id = group.verified_access_instance_id.clone();
        let id = self.next_verified_access_endpoint_id();
        let now = self.now_iso();
        let endpoint_domain = match (
            endpoint_domain_prefix.as_deref(),
            application_domain.as_deref(),
        ) {
            (Some(p), Some(d)) => Some(format!("{p}.{d}")),
            (Some(p), None) => Some(p.to_string()),
            (None, Some(d)) => Some(d.to_string()),
            (None, None) => None,
        };
        let policy_enabled = policy_document.is_some();
        let ep = VerifiedAccessEndpoint {
            verified_access_endpoint_id: id.clone(),
            verified_access_instance_id: instance_id,
            verified_access_group_id,
            application_domain,
            endpoint_type,
            attachment_type,
            domain_certificate_arn,
            endpoint_domain,
            device_validation_domain: None,
            security_group_ids,
            load_balancer_options,
            network_interface_options,
            cidr_options,
            rds_options,
            status_code: "active".to_string(),
            status_message: None,
            description,
            creation_time: now.clone(),
            last_updated_time: now,
            deletion_time: None,
            sse_specification,
            policy_document,
            policy_enabled,
            tags,
        };
        self.verified_access_endpoints.insert(id.clone(), ep);
        Ok(self.verified_access_endpoints.get(&id).unwrap())
    }

    pub fn attach_verified_access_trust_provider(
        &mut self,
        instance_id: &str,
        trust_provider_id: &str,
    ) -> Result<(VerifiedAccessTrustProvider, VerifiedAccessInstance), Ec2Error> {
        if !self.verified_access_instances.contains_key(instance_id) {
            return Err(Ec2Error::InvalidVerifiedAccessInstanceNotFound(
                instance_id.to_string(),
            ));
        }
        if !self
            .verified_access_trust_providers
            .contains_key(trust_provider_id)
        {
            return Err(Ec2Error::InvalidVerifiedAccessTrustProviderNotFound(
                trust_provider_id.to_string(),
            ));
        }
        let key = (instance_id.to_string(), trust_provider_id.to_string());
        self.verified_access_trust_provider_attachments.insert(
            key,
            VerifiedAccessTrustProviderAttachment {
                instance_id: instance_id.to_string(),
                trust_provider_id: trust_provider_id.to_string(),
            },
        );
        // Update instance to include the trust provider id (deduped) and bump
        // last_updated_time.
        let now = self.now_iso();
        if let Some(inst) = self.verified_access_instances.get_mut(instance_id) {
            if !inst
                .trust_provider_ids
                .iter()
                .any(|x| x == trust_provider_id)
            {
                inst.trust_provider_ids.push(trust_provider_id.to_string());
            }
            inst.last_updated_time = now.clone();
        }
        if let Some(tp) = self
            .verified_access_trust_providers
            .get_mut(trust_provider_id)
        {
            tp.last_updated_time = now;
        }
        let inst = self
            .verified_access_instances
            .get(instance_id)
            .unwrap()
            .clone();
        let tp = self
            .verified_access_trust_providers
            .get(trust_provider_id)
            .unwrap()
            .clone();
        Ok((tp, inst))
    }

    pub fn detach_verified_access_trust_provider(
        &mut self,
        instance_id: &str,
        trust_provider_id: &str,
    ) -> Result<(VerifiedAccessTrustProvider, VerifiedAccessInstance), Ec2Error> {
        if !self.verified_access_instances.contains_key(instance_id) {
            return Err(Ec2Error::InvalidVerifiedAccessInstanceNotFound(
                instance_id.to_string(),
            ));
        }
        if !self
            .verified_access_trust_providers
            .contains_key(trust_provider_id)
        {
            return Err(Ec2Error::InvalidVerifiedAccessTrustProviderNotFound(
                trust_provider_id.to_string(),
            ));
        }
        let key = (instance_id.to_string(), trust_provider_id.to_string());
        if self
            .verified_access_trust_provider_attachments
            .remove(&key)
            .is_none()
        {
            return Err(
                Ec2Error::InvalidVerifiedAccessTrustProviderAttachmentNotFound(
                    instance_id.to_string(),
                    trust_provider_id.to_string(),
                ),
            );
        }
        let now = self.now_iso();
        if let Some(inst) = self.verified_access_instances.get_mut(instance_id) {
            inst.trust_provider_ids.retain(|x| x != trust_provider_id);
            inst.last_updated_time = now.clone();
        }
        if let Some(tp) = self
            .verified_access_trust_providers
            .get_mut(trust_provider_id)
        {
            tp.last_updated_time = now;
        }
        let inst = self
            .verified_access_instances
            .get(instance_id)
            .unwrap()
            .clone();
        let tp = self
            .verified_access_trust_providers
            .get(trust_provider_id)
            .unwrap()
            .clone();
        Ok((tp, inst))
    }

    pub fn delete_verified_access_endpoint(
        &mut self,
        endpoint_id: &str,
    ) -> Result<VerifiedAccessEndpoint, Ec2Error> {
        let mut ep = self
            .verified_access_endpoints
            .remove(endpoint_id)
            .ok_or_else(|| {
                Ec2Error::InvalidVerifiedAccessEndpointNotFound(endpoint_id.to_string())
            })?;
        let now = self.now_iso();
        ep.status_code = "deleted".to_string();
        ep.deletion_time = Some(now.clone());
        ep.last_updated_time = now;
        Ok(ep)
    }

    pub fn delete_verified_access_group(
        &mut self,
        group_id: &str,
    ) -> Result<VerifiedAccessGroup, Ec2Error> {
        if !self.verified_access_groups.contains_key(group_id) {
            return Err(Ec2Error::InvalidVerifiedAccessGroupNotFound(
                group_id.to_string(),
            ));
        }
        let in_use = self
            .verified_access_endpoints
            .values()
            .any(|e| e.verified_access_group_id == group_id);
        if in_use {
            return Err(Ec2Error::VerifiedAccessGroupInUse(group_id.to_string()));
        }
        let mut group = self.verified_access_groups.remove(group_id).unwrap();
        let now = self.now_iso();
        group.deletion_time = Some(now.clone());
        group.last_updated_time = now;
        Ok(group)
    }

    pub fn delete_verified_access_instance(
        &mut self,
        instance_id: &str,
    ) -> Result<VerifiedAccessInstance, Ec2Error> {
        if !self.verified_access_instances.contains_key(instance_id) {
            return Err(Ec2Error::InvalidVerifiedAccessInstanceNotFound(
                instance_id.to_string(),
            ));
        }
        let has_attachments = self
            .verified_access_trust_provider_attachments
            .keys()
            .any(|(iid, _)| iid == instance_id);
        let has_groups = self
            .verified_access_groups
            .values()
            .any(|g| g.verified_access_instance_id == instance_id);
        if has_attachments || has_groups {
            return Err(Ec2Error::VerifiedAccessInstanceInUse(
                instance_id.to_string(),
            ));
        }
        // Remove logging configuration alongside the instance.
        self.verified_access_instance_logging_configurations
            .remove(instance_id);
        Ok(self.verified_access_instances.remove(instance_id).unwrap())
    }

    pub fn delete_verified_access_trust_provider(
        &mut self,
        trust_provider_id: &str,
    ) -> Result<VerifiedAccessTrustProvider, Ec2Error> {
        if !self
            .verified_access_trust_providers
            .contains_key(trust_provider_id)
        {
            return Err(Ec2Error::InvalidVerifiedAccessTrustProviderNotFound(
                trust_provider_id.to_string(),
            ));
        }
        let attached = self
            .verified_access_trust_provider_attachments
            .keys()
            .any(|(_, tp)| tp == trust_provider_id);
        if attached {
            return Err(Ec2Error::VerifiedAccessTrustProviderInUse(
                trust_provider_id.to_string(),
            ));
        }
        Ok(self
            .verified_access_trust_providers
            .remove(trust_provider_id)
            .unwrap())
    }

    pub fn modify_verified_access_instance(
        &mut self,
        instance_id: &str,
        description: Option<String>,
        cidr_endpoints_custom_subdomain: Option<String>,
    ) -> Result<&VerifiedAccessInstance, Ec2Error> {
        let now = self.now_iso();
        let inst = self
            .verified_access_instances
            .get_mut(instance_id)
            .ok_or_else(|| {
                Ec2Error::InvalidVerifiedAccessInstanceNotFound(instance_id.to_string())
            })?;
        if let Some(d) = description {
            inst.description = Some(d);
        }
        if let Some(s) = cidr_endpoints_custom_subdomain {
            inst.cidr_endpoints_custom_subdomain = Some(s);
        }
        inst.last_updated_time = now;
        Ok(self.verified_access_instances.get(instance_id).unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_verified_access_trust_provider(
        &mut self,
        trust_provider_id: &str,
        description: Option<String>,
        oidc_options: Option<VerifiedAccessOidcOptions>,
        device_options: Option<VerifiedAccessDeviceOptions>,
        sse_specification: Option<VerifiedAccessSseSpecification>,
    ) -> Result<&VerifiedAccessTrustProvider, Ec2Error> {
        let now = self.now_iso();
        let tp = self
            .verified_access_trust_providers
            .get_mut(trust_provider_id)
            .ok_or_else(|| {
                Ec2Error::InvalidVerifiedAccessTrustProviderNotFound(trust_provider_id.to_string())
            })?;
        if let Some(d) = description {
            tp.description = Some(d);
        }
        if let Some(o) = oidc_options {
            // Merge non-None fields into existing oidc_options.
            let mut existing = tp.oidc_options.clone().unwrap_or_default();
            if o.issuer.is_some() {
                existing.issuer = o.issuer;
            }
            if o.authorization_endpoint.is_some() {
                existing.authorization_endpoint = o.authorization_endpoint;
            }
            if o.token_endpoint.is_some() {
                existing.token_endpoint = o.token_endpoint;
            }
            if o.user_info_endpoint.is_some() {
                existing.user_info_endpoint = o.user_info_endpoint;
            }
            if o.client_id.is_some() {
                existing.client_id = o.client_id;
            }
            if o.client_secret.is_some() {
                existing.client_secret = o.client_secret;
            }
            if o.scope.is_some() {
                existing.scope = o.scope;
            }
            tp.oidc_options = Some(existing);
        }
        if let Some(d) = device_options {
            let mut existing = tp.device_options.clone().unwrap_or_default();
            if d.public_signing_key_url.is_some() {
                existing.public_signing_key_url = d.public_signing_key_url;
            }
            tp.device_options = Some(existing);
        }
        if let Some(s) = sse_specification {
            tp.sse_specification = s;
        }
        tp.last_updated_time = now;
        Ok(self
            .verified_access_trust_providers
            .get(trust_provider_id)
            .unwrap())
    }

    pub fn modify_verified_access_group(
        &mut self,
        group_id: &str,
        description: Option<String>,
        verified_access_instance_id: Option<String>,
    ) -> Result<&VerifiedAccessGroup, Ec2Error> {
        if let Some(ref iid) = verified_access_instance_id {
            if !self.verified_access_instances.contains_key(iid) {
                return Err(Ec2Error::InvalidVerifiedAccessInstanceNotFound(iid.clone()));
            }
        }
        let now = self.now_iso();
        let group = self
            .verified_access_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::InvalidVerifiedAccessGroupNotFound(group_id.to_string()))?;
        if let Some(d) = description {
            group.description = Some(d);
        }
        if let Some(iid) = verified_access_instance_id {
            group.verified_access_instance_id = iid;
        }
        group.last_updated_time = now;
        Ok(self.verified_access_groups.get(group_id).unwrap())
    }

    pub fn modify_verified_access_group_policy(
        &mut self,
        group_id: &str,
        policy_document: Option<String>,
        policy_enabled: Option<bool>,
        sse_specification: Option<VerifiedAccessSseSpecification>,
    ) -> Result<&VerifiedAccessGroup, Ec2Error> {
        let now = self.now_iso();
        let group = self
            .verified_access_groups
            .get_mut(group_id)
            .ok_or_else(|| Ec2Error::InvalidVerifiedAccessGroupNotFound(group_id.to_string()))?;
        if let Some(d) = policy_document {
            group.policy_document = Some(d);
        }
        if let Some(b) = policy_enabled {
            group.policy_enabled = b;
        }
        if let Some(s) = sse_specification {
            group.sse_specification = s;
        }
        group.last_updated_time = now;
        Ok(self.verified_access_groups.get(group_id).unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_verified_access_endpoint(
        &mut self,
        endpoint_id: &str,
        description: Option<String>,
        verified_access_group_id: Option<String>,
        load_balancer_options: Option<VerifiedAccessEndpointLoadBalancerOptions>,
        network_interface_options: Option<VerifiedAccessEndpointEniOptions>,
        cidr_options: Option<VerifiedAccessEndpointCidrOptions>,
        rds_options: Option<VerifiedAccessEndpointRdsOptions>,
    ) -> Result<&VerifiedAccessEndpoint, Ec2Error> {
        if let Some(ref gid) = verified_access_group_id {
            if !self.verified_access_groups.contains_key(gid) {
                return Err(Ec2Error::InvalidVerifiedAccessGroupNotFound(gid.clone()));
            }
        }
        let now = self.now_iso();
        let ep = self
            .verified_access_endpoints
            .get_mut(endpoint_id)
            .ok_or_else(|| {
                Ec2Error::InvalidVerifiedAccessEndpointNotFound(endpoint_id.to_string())
            })?;
        if let Some(d) = description {
            ep.description = Some(d);
        }
        if let Some(gid) = verified_access_group_id {
            ep.verified_access_group_id = gid;
        }
        if let Some(o) = load_balancer_options {
            ep.load_balancer_options = Some(o);
        }
        if let Some(o) = network_interface_options {
            ep.network_interface_options = Some(o);
        }
        if let Some(o) = cidr_options {
            ep.cidr_options = Some(o);
        }
        if let Some(o) = rds_options {
            ep.rds_options = Some(o);
        }
        ep.last_updated_time = now;
        Ok(self.verified_access_endpoints.get(endpoint_id).unwrap())
    }

    pub fn modify_verified_access_endpoint_policy(
        &mut self,
        endpoint_id: &str,
        policy_document: Option<String>,
        policy_enabled: Option<bool>,
        sse_specification: Option<VerifiedAccessSseSpecification>,
    ) -> Result<&VerifiedAccessEndpoint, Ec2Error> {
        let now = self.now_iso();
        let ep = self
            .verified_access_endpoints
            .get_mut(endpoint_id)
            .ok_or_else(|| {
                Ec2Error::InvalidVerifiedAccessEndpointNotFound(endpoint_id.to_string())
            })?;
        if let Some(d) = policy_document {
            ep.policy_document = Some(d);
        }
        if let Some(b) = policy_enabled {
            ep.policy_enabled = b;
        }
        if let Some(s) = sse_specification {
            ep.sse_specification = s;
        }
        ep.last_updated_time = now;
        Ok(self.verified_access_endpoints.get(endpoint_id).unwrap())
    }

    pub fn modify_verified_access_instance_logging_configuration(
        &mut self,
        instance_id: &str,
        logs: VerifiedAccessLogs,
    ) -> Result<&VerifiedAccessLogs, Ec2Error> {
        if !self.verified_access_instances.contains_key(instance_id) {
            return Err(Ec2Error::InvalidVerifiedAccessInstanceNotFound(
                instance_id.to_string(),
            ));
        }
        self.verified_access_instance_logging_configurations
            .insert(instance_id.to_string(), logs);
        // Bump instance last_updated_time.
        let now = self.now_iso();
        if let Some(inst) = self.verified_access_instances.get_mut(instance_id) {
            inst.last_updated_time = now;
        }
        Ok(self
            .verified_access_instance_logging_configurations
            .get(instance_id)
            .unwrap())
    }

    // ===== Group 10 helpers and operations =====

    fn next_capacity_manager_data_export_id(&mut self) -> String {
        self.counters.capacity_manager_data_export += 1;
        format!("cmde-{:08x}", self.counters.capacity_manager_data_export)
    }

    fn next_interruptible_capacity_reservation_allocation_id(&mut self) -> String {
        self.counters.interruptible_capacity_reservation_allocation += 1;
        format!(
            "icra-{:08x}",
            self.counters.interruptible_capacity_reservation_allocation
        )
    }

    fn next_capacity_block_id(&mut self) -> String {
        self.counters.capacity_block += 1;
        format!("cb-{:08x}", self.counters.capacity_block)
    }

    fn next_capacity_block_extension_id(&mut self) -> String {
        self.counters.capacity_block_extension += 1;
        format!("cbe-{:08x}", self.counters.capacity_block_extension)
    }

    /// Create a pending billing-ownership transfer offer.
    pub fn associate_capacity_reservation_billing_owner(
        &mut self,
        capacity_reservation_id: &str,
        unused_reservation_billing_owner_id: &str,
        requester_account_id: &str,
    ) -> Result<&BillingOwnershipOffer, Ec2Error> {
        if !self
            .capacity_reservations
            .contains_key(capacity_reservation_id)
        {
            return Err(Ec2Error::CapacityReservationNotFound(
                capacity_reservation_id.to_string(),
            ));
        }
        let key = (
            capacity_reservation_id.to_string(),
            unused_reservation_billing_owner_id.to_string(),
        );
        if let Some(existing) = self.billing_ownership_offers.get(&key)
            && existing.status == "pending"
        {
            return Err(Ec2Error::BillingOwnershipOfferAlreadyExists(
                capacity_reservation_id.to_string(),
                unused_reservation_billing_owner_id.to_string(),
            ));
        }
        let now = self.now_iso();
        let offer = BillingOwnershipOffer {
            capacity_reservation_id: capacity_reservation_id.to_string(),
            unused_reservation_billing_owner_id: unused_reservation_billing_owner_id.to_string(),
            requested_by: requester_account_id.to_string(),
            status: "pending".to_string(),
            status_message: None,
            last_update_time: now,
        };
        if let Some(cr) = self.capacity_reservations.get_mut(capacity_reservation_id) {
            cr.pending_billing_owner_account_id =
                Some(unused_reservation_billing_owner_id.to_string());
        }
        self.billing_ownership_offers.insert(key.clone(), offer);
        Ok(self.billing_ownership_offers.get(&key).unwrap())
    }

    /// Accept a pending billing-ownership offer.
    pub fn accept_capacity_reservation_billing_ownership(
        &mut self,
        capacity_reservation_id: &str,
        accepter_account_id: &str,
    ) -> Result<&BillingOwnershipOffer, Ec2Error> {
        let key = (
            capacity_reservation_id.to_string(),
            accepter_account_id.to_string(),
        );
        let now = self.now_iso();
        let offer = self.billing_ownership_offers.get_mut(&key).ok_or_else(|| {
            Ec2Error::InvalidBillingOwnershipOfferNotFound(
                capacity_reservation_id.to_string(),
                accepter_account_id.to_string(),
            )
        })?;
        offer.status = "accepted".to_string();
        offer.last_update_time = now;
        if let Some(cr) = self.capacity_reservations.get_mut(capacity_reservation_id) {
            cr.billing_owner_account_id = Some(accepter_account_id.to_string());
            cr.pending_billing_owner_account_id = None;
        }
        Ok(self.billing_ownership_offers.get(&key).unwrap())
    }

    /// Cancel a pending billing-ownership offer (initiator side).
    pub fn disassociate_capacity_reservation_billing_owner(
        &mut self,
        capacity_reservation_id: &str,
        unused_reservation_billing_owner_id: &str,
    ) -> Result<&BillingOwnershipOffer, Ec2Error> {
        let key = (
            capacity_reservation_id.to_string(),
            unused_reservation_billing_owner_id.to_string(),
        );
        let now = self.now_iso();
        let offer = self.billing_ownership_offers.get_mut(&key).ok_or_else(|| {
            Ec2Error::InvalidBillingOwnershipOfferNotFound(
                capacity_reservation_id.to_string(),
                unused_reservation_billing_owner_id.to_string(),
            )
        })?;
        offer.status = "cancelled".to_string();
        offer.last_update_time = now;
        if let Some(cr) = self.capacity_reservations.get_mut(capacity_reservation_id) {
            cr.pending_billing_owner_account_id = None;
        }
        Ok(self.billing_ownership_offers.get(&key).unwrap())
    }

    /// Reject a pending billing-ownership offer (recipient side).
    pub fn reject_capacity_reservation_billing_ownership(
        &mut self,
        capacity_reservation_id: &str,
        rejecter_account_id: &str,
    ) -> Result<&BillingOwnershipOffer, Ec2Error> {
        let key = (
            capacity_reservation_id.to_string(),
            rejecter_account_id.to_string(),
        );
        let now = self.now_iso();
        let offer = self.billing_ownership_offers.get_mut(&key).ok_or_else(|| {
            Ec2Error::InvalidBillingOwnershipOfferNotFound(
                capacity_reservation_id.to_string(),
                rejecter_account_id.to_string(),
            )
        })?;
        offer.status = "rejected".to_string();
        offer.last_update_time = now;
        if let Some(cr) = self.capacity_reservations.get_mut(capacity_reservation_id) {
            cr.pending_billing_owner_account_id = None;
        }
        Ok(self.billing_ownership_offers.get(&key).unwrap())
    }

    /// Create a Capacity Manager data export.
    #[allow(clippy::too_many_arguments)]
    pub fn create_capacity_manager_data_export(
        &mut self,
        schedule: String,
        organization_account_ids: Vec<String>,
        output_format: String,
        s3_destination: S3DestinationOptions,
        tags: Tags,
    ) -> Result<&CapacityManagerDataExport, Ec2Error> {
        if !matches!(
            schedule.as_str(),
            "ondemand" | "daily" | "weekly" | "monthly" | "hourly"
        ) {
            return Err(Ec2Error::InvalidParameterValue(format!(
                "Invalid Schedule: {schedule}"
            )));
        }
        if !matches!(output_format.as_str(), "parquet" | "csv") {
            return Err(Ec2Error::InvalidParameterValue(format!(
                "Invalid OutputFormat: {output_format}"
            )));
        }
        let id = self.next_capacity_manager_data_export_id();
        let now = self.now_iso();
        let export = CapacityManagerDataExport {
            data_export_id: id.clone(),
            schedule,
            organization_account_ids,
            output_format,
            s3_destination,
            status: "active".to_string(),
            status_message: None,
            last_export_time: None,
            next_export_time: None,
            create_time: now,
            tags,
        };
        self.capacity_manager_data_exports
            .insert(id.clone(), export);
        Ok(self.capacity_manager_data_exports.get(&id).unwrap())
    }

    /// Delete a Capacity Manager data export by id.
    pub fn delete_capacity_manager_data_export(
        &mut self,
        data_export_id: &str,
    ) -> Result<CapacityManagerDataExport, Ec2Error> {
        self.capacity_manager_data_exports
            .remove(data_export_id)
            .ok_or_else(|| {
                Ec2Error::InvalidCapacityManagerDataExportNotFound(data_export_id.to_string())
            })
    }

    /// Update account-level Capacity Manager Organizations access.
    pub fn update_capacity_manager_organizations_access(
        &mut self,
        organizations_access: bool,
    ) -> &CapacityManagerOrganizationsAccess {
        let now = self.now_iso();
        let new_state = if organizations_access {
            "enabled"
        } else {
            "disabled"
        };
        self.capacity_manager_organizations_access = Some(CapacityManagerOrganizationsAccess {
            state: new_state.to_string(),
            last_updated_time: now,
        });
        self.capacity_manager_organizations_access.as_ref().unwrap()
    }

    /// Split an existing capacity reservation into two (the source loses
    /// `instance_count` instances, the new one gets them).
    #[allow(clippy::too_many_arguments)]
    pub fn create_capacity_reservation_by_splitting(
        &mut self,
        source_capacity_reservation_id: &str,
        instance_count: i32,
        account_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<(CapacityReservation, CapacityReservation), Ec2Error> {
        if instance_count <= 0 {
            return Err(Ec2Error::InvalidParameterValue(
                "InstanceCount must be > 0".to_string(),
            ));
        }
        let source = self
            .capacity_reservations
            .get(source_capacity_reservation_id)
            .ok_or_else(|| {
                Ec2Error::CapacityReservationNotFound(source_capacity_reservation_id.to_string())
            })?
            .clone();
        if source.available_instance_count < instance_count {
            return Err(Ec2Error::InsufficientCapacityReservationCapacity(
                source_capacity_reservation_id.to_string(),
                source.available_instance_count,
                instance_count,
            ));
        }
        // Decrement the source.
        let src_mut = self
            .capacity_reservations
            .get_mut(source_capacity_reservation_id)
            .unwrap();
        src_mut.total_instance_count -= instance_count;
        src_mut.available_instance_count -= instance_count;
        let updated_source = src_mut.clone();
        // Create the new split reservation, inheriting most properties of
        // the source.
        let new_id = self.next_capacity_reservation_id();
        let arn = format!("arn:aws:ec2:{region}:{account_id}:capacity-reservation/{new_id}");
        let new_cr = CapacityReservation {
            capacity_reservation_id: new_id.clone(),
            capacity_reservation_arn: arn,
            owner_id: account_id.to_string(),
            instance_type: source.instance_type.clone(),
            instance_platform: source.instance_platform.clone(),
            availability_zone: source.availability_zone.clone(),
            tenancy: source.tenancy.clone(),
            total_instance_count: instance_count,
            available_instance_count: instance_count,
            ebs_optimized: source.ebs_optimized,
            ephemeral_storage: source.ephemeral_storage,
            state: "active".to_string(),
            start_date: self.now_iso(),
            end_date: source.end_date.clone(),
            end_date_type: source.end_date_type.clone(),
            instance_match_criteria: source.instance_match_criteria.clone(),
            create_date: self.now_iso(),
            outpost_arn: source.outpost_arn.clone(),
            placement_group_arn: source.placement_group_arn.clone(),
            tags,
            pending_billing_owner_account_id: None,
            billing_owner_account_id: Some(account_id.to_string()),
            target_capacity_reservation_id: Some(source_capacity_reservation_id.to_string()),
            reservation_type: source.reservation_type.clone(),
            commitment_info: None,
        };
        self.capacity_reservations
            .insert(new_id.clone(), new_cr.clone());
        Ok((updated_source, new_cr))
    }

    /// Move some instances from a source capacity reservation to a destination.
    pub fn move_capacity_reservation_instances(
        &mut self,
        source_capacity_reservation_id: &str,
        destination_capacity_reservation_id: &str,
        instance_count: i32,
    ) -> Result<(CapacityReservation, CapacityReservation), Ec2Error> {
        if instance_count <= 0 {
            return Err(Ec2Error::InvalidParameterValue(
                "InstanceCount must be > 0".to_string(),
            ));
        }
        if !self
            .capacity_reservations
            .contains_key(destination_capacity_reservation_id)
        {
            return Err(Ec2Error::CapacityReservationNotFound(
                destination_capacity_reservation_id.to_string(),
            ));
        }
        let src = self
            .capacity_reservations
            .get(source_capacity_reservation_id)
            .ok_or_else(|| {
                Ec2Error::CapacityReservationNotFound(source_capacity_reservation_id.to_string())
            })?
            .clone();
        if src.available_instance_count < instance_count {
            return Err(Ec2Error::InsufficientCapacityReservationCapacity(
                source_capacity_reservation_id.to_string(),
                src.available_instance_count,
                instance_count,
            ));
        }
        let src_mut = self
            .capacity_reservations
            .get_mut(source_capacity_reservation_id)
            .unwrap();
        src_mut.total_instance_count -= instance_count;
        src_mut.available_instance_count -= instance_count;
        let new_src = src_mut.clone();
        let dest_mut = self
            .capacity_reservations
            .get_mut(destination_capacity_reservation_id)
            .unwrap();
        dest_mut.total_instance_count += instance_count;
        dest_mut.available_instance_count += instance_count;
        let new_dest = dest_mut.clone();
        Ok((new_src, new_dest))
    }

    /// Modify the per-instance capacity-reservation specification.
    pub fn modify_instance_capacity_reservation_attributes(
        &mut self,
        instance_id: &str,
        spec: CapacityReservationSpecificationResponse,
    ) -> Result<(), Ec2Error> {
        let inst = self
            .instances
            .get_mut(instance_id)
            .ok_or_else(|| Ec2Error::InstanceNotFound(instance_id.to_string()))?;
        inst.capacity_reservation_specification = Some(spec);
        Ok(())
    }

    /// Create an interruptible capacity-reservation allocation.
    #[allow(clippy::too_many_arguments)]
    pub fn create_interruptible_capacity_reservation_allocation(
        &mut self,
        capacity_reservation_id: &str,
        instance_count: i32,
        start_date_time: String,
        end_date_time: String,
        allocation_type: String,
        tags: Tags,
    ) -> Result<&InterruptibleCapacityReservationAllocation, Ec2Error> {
        if !self
            .capacity_reservations
            .contains_key(capacity_reservation_id)
        {
            return Err(Ec2Error::CapacityReservationNotFound(
                capacity_reservation_id.to_string(),
            ));
        }
        if instance_count <= 0 {
            return Err(Ec2Error::InvalidParameterValue(
                "InstanceCount must be > 0".to_string(),
            ));
        }
        if !matches!(allocation_type.as_str(), "scheduled" | "on-demand") {
            return Err(Ec2Error::InvalidParameterValue(format!(
                "Invalid AllocationType: {allocation_type}"
            )));
        }
        let id = self.next_interruptible_capacity_reservation_allocation_id();
        let alloc = InterruptibleCapacityReservationAllocation {
            allocation_id: id.clone(),
            capacity_reservation_id: capacity_reservation_id.to_string(),
            instance_count,
            start_date_time,
            end_date_time,
            state: "scheduled".to_string(),
            allocation_type,
            tags,
        };
        self.interruptible_capacity_reservation_allocations
            .insert(id.clone(), alloc);
        Ok(self
            .interruptible_capacity_reservation_allocations
            .get(&id)
            .unwrap())
    }

    /// Update an interruptible capacity-reservation allocation.
    pub fn update_interruptible_capacity_reservation_allocation(
        &mut self,
        allocation_id: &str,
        instance_count: Option<i32>,
        start_date_time: Option<String>,
        end_date_time: Option<String>,
        allocation_type: Option<String>,
    ) -> Result<&InterruptibleCapacityReservationAllocation, Ec2Error> {
        let alloc = self
            .interruptible_capacity_reservation_allocations
            .get_mut(allocation_id)
            .ok_or_else(|| {
                Ec2Error::InvalidInterruptibleCapacityReservationAllocationNotFound(
                    allocation_id.to_string(),
                )
            })?;
        if let Some(c) = instance_count {
            if c <= 0 {
                return Err(Ec2Error::InvalidParameterValue(
                    "InstanceCount must be > 0".to_string(),
                ));
            }
            alloc.instance_count = c;
        }
        if let Some(s) = start_date_time {
            alloc.start_date_time = s;
        }
        if let Some(e) = end_date_time {
            alloc.end_date_time = e;
        }
        if let Some(t) = allocation_type {
            if !matches!(t.as_str(), "scheduled" | "on-demand") {
                return Err(Ec2Error::InvalidParameterValue(format!(
                    "Invalid AllocationType: {t}"
                )));
            }
            alloc.allocation_type = t;
        }
        Ok(self
            .interruptible_capacity_reservation_allocations
            .get(allocation_id)
            .unwrap())
    }

    /// Purchase a capacity block. This also synthesises a fixed-duration
    /// capacity reservation backed by the block.
    #[allow(clippy::too_many_arguments)]
    pub fn purchase_capacity_block(
        &mut self,
        capacity_block_offering_id: &str,
        instance_platform: &str,
        instance_type: &str,
        instance_count: i32,
        availability_zone: &str,
        start_date: String,
        end_date: String,
        tenancy: String,
        upfront_fee: String,
        commitment_duration_in_seconds: i64,
        account_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<(CapacityBlock, CapacityReservation), Ec2Error> {
        if instance_count <= 0 {
            return Err(Ec2Error::InvalidParameterValue(
                "InstanceCount must be > 0".to_string(),
            ));
        }
        // Synthesise a backing capacity reservation.
        let cr_id = self.next_capacity_reservation_id();
        let cr_arn = format!("arn:aws:ec2:{region}:{account_id}:capacity-reservation/{cr_id}");
        let cr = CapacityReservation {
            capacity_reservation_id: cr_id.clone(),
            capacity_reservation_arn: cr_arn.clone(),
            owner_id: account_id.to_string(),
            instance_type: instance_type.to_string(),
            instance_platform: instance_platform.to_string(),
            availability_zone: availability_zone.to_string(),
            tenancy: tenancy.clone(),
            total_instance_count: instance_count,
            available_instance_count: instance_count,
            ebs_optimized: false,
            ephemeral_storage: false,
            state: "active".to_string(),
            start_date: start_date.clone(),
            end_date: Some(end_date.clone()),
            end_date_type: "limited".to_string(),
            instance_match_criteria: "targeted".to_string(),
            create_date: self.now_iso(),
            outpost_arn: None,
            placement_group_arn: None,
            tags: tags.clone(),
            pending_billing_owner_account_id: None,
            billing_owner_account_id: Some(account_id.to_string()),
            target_capacity_reservation_id: None,
            reservation_type: Some("capacity-block".to_string()),
            commitment_info: Some(CapacityReservationCommitmentInfo {
                commitment_end_date: Some(end_date.clone()),
                committed_instance_count: Some(instance_count),
            }),
        };
        self.capacity_reservations.insert(cr_id.clone(), cr.clone());

        let block_id = self.next_capacity_block_id();
        let block = CapacityBlock {
            capacity_block_id: block_id.clone(),
            capacity_reservation_id: cr_id,
            capacity_block_offering_id: capacity_block_offering_id.to_string(),
            instance_type: instance_type.to_string(),
            instance_count,
            availability_zone: availability_zone.to_string(),
            start_date,
            end_date,
            tenancy,
            currency_code: "USD".to_string(),
            upfront_fee,
            commitment_duration_in_seconds,
            capacity_reservation_arn: cr_arn,
            tags,
        };
        self.capacity_blocks.insert(block_id, block.clone());
        Ok((block, cr))
    }

    /// Purchase a capacity-block extension for an existing block-backed
    /// capacity reservation.
    #[allow(clippy::too_many_arguments)]
    pub fn purchase_capacity_block_extension(
        &mut self,
        capacity_block_extension_offering_id: &str,
        capacity_reservation_id: &str,
        capacity_block_extension_duration_hours: i32,
        upfront_fee: String,
        account_id: &str,
        region: &str,
    ) -> Result<CapacityBlockExtension, Ec2Error> {
        let cr = self
            .capacity_reservations
            .get(capacity_reservation_id)
            .ok_or_else(|| {
                Ec2Error::CapacityReservationNotFound(capacity_reservation_id.to_string())
            })?
            .clone();
        if capacity_block_extension_duration_hours <= 0 {
            return Err(Ec2Error::InvalidParameterValue(
                "CapacityBlockExtensionDurationHours must be > 0".to_string(),
            ));
        }
        let id = self.next_capacity_block_extension_id();
        let now = self.now_iso();
        let start_date = cr.end_date.clone().unwrap_or_else(|| now.clone());
        // Compute end_date as start_date + duration hours. We attempt to
        // parse the start as RFC 3339; on failure we fall back to `now` so
        // the response still contains a valid timestamp.
        let end_date = match chrono::DateTime::parse_from_rfc3339(&start_date) {
            Ok(start) => {
                let new_end =
                    start + chrono::Duration::hours(capacity_block_extension_duration_hours as i64);
                new_end
                    .with_timezone(&chrono::Utc)
                    .format("%Y-%m-%dT%H:%M:%S.000Z")
                    .to_string()
            }
            Err(_) => self.now_iso(),
        };
        let arn = format!("arn:aws:ec2:{region}:{account_id}:capacity-block-extension/{id}");
        let ext = CapacityBlockExtension {
            capacity_block_extension_id: id.clone(),
            capacity_reservation_id: capacity_reservation_id.to_string(),
            instance_type: cr.instance_type.clone(),
            availability_zone: cr.availability_zone.clone(),
            instance_count: cr.total_instance_count,
            availability_zone_id: None,
            start_date: start_date.clone(),
            end_date: end_date.clone(),
            capacity_block_extension_offering_id: capacity_block_extension_offering_id.to_string(),
            capacity_block_extension_status: "payment-succeeded".to_string(),
            capacity_block_extension_purchase_date: now.clone(),
            capacity_block_extension_duration_hours,
            currency_code: "USD".to_string(),
            upfront_fee,
            capacity_reservation_arn: Some(cr.capacity_reservation_arn.clone()),
            capacity_block_extension_arn: Some(arn),
        };
        self.capacity_block_extensions
            .insert(id.clone(), ext.clone());
        // Bump the underlying reservation's end_date.
        if let Some(cr_mut) = self.capacity_reservations.get_mut(capacity_reservation_id) {
            cr_mut.end_date = Some(end_date);
        }
        Ok(ext)
    }

    // ----- Group 11: Transit Gateway extensions -----

    pub fn create_tgw_multicast_domain(
        &mut self,
        tgw_id: &str,
        igmpv2_support: &str,
        static_sources_support: &str,
        auto_accept_shared_associations: &str,
        owner_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<&TransitGatewayMulticastDomain, Ec2Error> {
        if !self.transit_gateways.contains_key(tgw_id) {
            return Err(Ec2Error::TransitGatewayNotFound(tgw_id.to_string()));
        }
        let domain_id = self.next_tgw_multicast_domain_id();
        let arn =
            format!("arn:aws:ec2:{region}:{owner_id}:transit-gateway-multicast-domain/{domain_id}");
        let domain = TransitGatewayMulticastDomain {
            transit_gateway_multicast_domain_id: domain_id.clone(),
            transit_gateway_id: tgw_id.to_string(),
            transit_gateway_multicast_domain_arn: arn,
            owner_id: owner_id.to_string(),
            igmpv2_support: igmpv2_support.to_string(),
            static_sources_support: static_sources_support.to_string(),
            auto_accept_shared_associations: auto_accept_shared_associations.to_string(),
            state: "available".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            tags,
        };
        self.tgw_multicast_domains.insert(domain_id.clone(), domain);
        Ok(self.tgw_multicast_domains.get(&domain_id).unwrap())
    }

    pub fn delete_tgw_multicast_domain(&mut self, domain_id: &str) -> Result<(), Ec2Error> {
        if !self.tgw_multicast_domains.contains_key(domain_id) {
            return Err(Ec2Error::InvalidTgwMulticastDomainNotFound(
                domain_id.to_string(),
            ));
        }
        let has_assoc = self
            .tgw_multicast_domain_associations
            .keys()
            .any(|(d, _)| d == domain_id);
        let has_member = self
            .tgw_multicast_group_members
            .keys()
            .any(|(d, _, _)| d == domain_id);
        let has_source = self
            .tgw_multicast_group_sources
            .keys()
            .any(|(d, _, _)| d == domain_id);
        if has_assoc || has_member || has_source {
            return Err(Ec2Error::TgwMulticastDomainInUse(domain_id.to_string()));
        }
        if let Some(d) = self.tgw_multicast_domains.get_mut(domain_id) {
            d.state = "deleted".to_string();
        }
        Ok(())
    }

    pub fn associate_tgw_multicast_domain(
        &mut self,
        domain_id: &str,
        attachment_id: &str,
        subnet_ids: Vec<String>,
    ) -> Result<&TransitGatewayMulticastDomainAssociation, Ec2Error> {
        let auto_accept = self
            .tgw_multicast_domains
            .get(domain_id)
            .ok_or_else(|| Ec2Error::InvalidTgwMulticastDomainNotFound(domain_id.to_string()))?
            .auto_accept_shared_associations
            .clone();
        let resource_id = self
            .tgw_vpc_attachments
            .get(attachment_id)
            .map(|a| a.vpc_id.clone())
            .unwrap_or_default();
        let subnet_state = if auto_accept == "enable" {
            "associated"
        } else {
            "pendingAcceptance"
        };
        let subnets: Vec<MulticastSubnetAssociation> = subnet_ids
            .into_iter()
            .map(|s| MulticastSubnetAssociation {
                subnet_id: s,
                state: subnet_state.to_string(),
            })
            .collect();
        let key = (domain_id.to_string(), attachment_id.to_string());
        let assoc = TransitGatewayMulticastDomainAssociation {
            transit_gateway_multicast_domain_id: domain_id.to_string(),
            transit_gateway_attachment_id: attachment_id.to_string(),
            resource_id,
            resource_type: "vpc".to_string(),
            subnets,
        };
        self.tgw_multicast_domain_associations
            .insert(key.clone(), assoc);
        Ok(self.tgw_multicast_domain_associations.get(&key).unwrap())
    }

    pub fn accept_tgw_multicast_domain_associations(
        &mut self,
        domain_id: &str,
        attachment_id: &str,
        subnet_ids: &[String],
    ) -> Result<&TransitGatewayMulticastDomainAssociation, Ec2Error> {
        let key = (domain_id.to_string(), attachment_id.to_string());
        let assoc = self
            .tgw_multicast_domain_associations
            .get_mut(&key)
            .ok_or_else(|| {
                Ec2Error::InvalidTgwMulticastDomainAssociationNotFound(
                    domain_id.to_string(),
                    attachment_id.to_string(),
                )
            })?;
        for s in &mut assoc.subnets {
            if subnet_ids.is_empty() || subnet_ids.contains(&s.subnet_id) {
                s.state = "associated".to_string();
            }
        }
        Ok(self.tgw_multicast_domain_associations.get(&key).unwrap())
    }

    pub fn reject_tgw_multicast_domain_associations(
        &mut self,
        domain_id: &str,
        attachment_id: &str,
        subnet_ids: &[String],
    ) -> Result<&TransitGatewayMulticastDomainAssociation, Ec2Error> {
        let key = (domain_id.to_string(), attachment_id.to_string());
        let assoc = self
            .tgw_multicast_domain_associations
            .get_mut(&key)
            .ok_or_else(|| {
                Ec2Error::InvalidTgwMulticastDomainAssociationNotFound(
                    domain_id.to_string(),
                    attachment_id.to_string(),
                )
            })?;
        for s in &mut assoc.subnets {
            if subnet_ids.is_empty() || subnet_ids.contains(&s.subnet_id) {
                s.state = "rejected".to_string();
            }
        }
        Ok(self.tgw_multicast_domain_associations.get(&key).unwrap())
    }

    pub fn disassociate_tgw_multicast_domain(
        &mut self,
        domain_id: &str,
        attachment_id: &str,
    ) -> Result<TransitGatewayMulticastDomainAssociation, Ec2Error> {
        let key = (domain_id.to_string(), attachment_id.to_string());
        let mut assoc = self
            .tgw_multicast_domain_associations
            .remove(&key)
            .ok_or_else(|| {
                Ec2Error::InvalidTgwMulticastDomainAssociationNotFound(
                    domain_id.to_string(),
                    attachment_id.to_string(),
                )
            })?;
        for s in &mut assoc.subnets {
            s.state = "disassociated".to_string();
        }
        Ok(assoc)
    }

    pub fn register_tgw_multicast_group_members(
        &mut self,
        domain_id: &str,
        group_ip: &str,
        nic_ids: Vec<String>,
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.tgw_multicast_domains.contains_key(domain_id) {
            return Err(Ec2Error::InvalidTgwMulticastDomainNotFound(
                domain_id.to_string(),
            ));
        }
        let registered = nic_ids.clone();
        for nic in nic_ids {
            let key = (domain_id.to_string(), group_ip.to_string(), nic.clone());
            self.tgw_multicast_group_members.insert(
                key,
                TransitGatewayMulticastGroupMember {
                    transit_gateway_multicast_domain_id: domain_id.to_string(),
                    group_ip_address: group_ip.to_string(),
                    transit_gateway_attachment_id: None,
                    subnet_id: None,
                    resource_id: None,
                    resource_type: "vpc".to_string(),
                    network_interface_id: nic,
                    member_type: "igmp".to_string(),
                    source_type: "igmp".to_string(),
                },
            );
        }
        Ok(registered)
    }

    pub fn deregister_tgw_multicast_group_members(
        &mut self,
        domain_id: &str,
        group_ip: &str,
        nic_ids: Vec<String>,
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.tgw_multicast_domains.contains_key(domain_id) {
            return Err(Ec2Error::InvalidTgwMulticastDomainNotFound(
                domain_id.to_string(),
            ));
        }
        let mut removed = Vec::new();
        for nic in nic_ids {
            let key = (domain_id.to_string(), group_ip.to_string(), nic.clone());
            if self.tgw_multicast_group_members.remove(&key).is_some() {
                removed.push(nic);
            }
        }
        Ok(removed)
    }

    pub fn register_tgw_multicast_group_sources(
        &mut self,
        domain_id: &str,
        group_ip: &str,
        nic_ids: Vec<String>,
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.tgw_multicast_domains.contains_key(domain_id) {
            return Err(Ec2Error::InvalidTgwMulticastDomainNotFound(
                domain_id.to_string(),
            ));
        }
        let registered = nic_ids.clone();
        for nic in nic_ids {
            let key = (domain_id.to_string(), group_ip.to_string(), nic.clone());
            self.tgw_multicast_group_sources.insert(
                key,
                TransitGatewayMulticastGroupSource {
                    transit_gateway_multicast_domain_id: domain_id.to_string(),
                    group_ip_address: group_ip.to_string(),
                    transit_gateway_attachment_id: None,
                    subnet_id: None,
                    resource_id: None,
                    resource_type: "vpc".to_string(),
                    network_interface_id: nic,
                    member_type: "static".to_string(),
                    source_type: "static".to_string(),
                },
            );
        }
        Ok(registered)
    }

    pub fn deregister_tgw_multicast_group_sources(
        &mut self,
        domain_id: &str,
        group_ip: &str,
        nic_ids: Vec<String>,
    ) -> Result<Vec<String>, Ec2Error> {
        if !self.tgw_multicast_domains.contains_key(domain_id) {
            return Err(Ec2Error::InvalidTgwMulticastDomainNotFound(
                domain_id.to_string(),
            ));
        }
        let mut removed = Vec::new();
        for nic in nic_ids {
            let key = (domain_id.to_string(), group_ip.to_string(), nic.clone());
            if self.tgw_multicast_group_sources.remove(&key).is_some() {
                removed.push(nic);
            }
        }
        Ok(removed)
    }

    pub fn create_tgw_connect(
        &mut self,
        transport_attachment_id: &str,
        protocol: &str,
        tags: Tags,
    ) -> Result<&TransitGatewayConnect, Ec2Error> {
        let tgw_id = self
            .tgw_vpc_attachments
            .get(transport_attachment_id)
            .map(|a| a.transit_gateway_id.clone())
            .ok_or_else(|| {
                Ec2Error::TgwVpcAttachmentNotFound(transport_attachment_id.to_string())
            })?;
        let attach_id = self.next_tgw_connect_id();
        let connect = TransitGatewayConnect {
            transit_gateway_attachment_id: attach_id.clone(),
            transport_transit_gateway_attachment_id: transport_attachment_id.to_string(),
            transit_gateway_id: tgw_id,
            state: "available".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            protocol: protocol.to_string(),
            tags,
        };
        self.tgw_connects.insert(attach_id.clone(), connect);
        Ok(self.tgw_connects.get(&attach_id).unwrap())
    }

    pub fn delete_tgw_connect(&mut self, attach_id: &str) -> Result<(), Ec2Error> {
        if !self.tgw_connects.contains_key(attach_id) {
            return Err(Ec2Error::InvalidTgwConnectNotFound(attach_id.to_string()));
        }
        let has_peers = self
            .tgw_connect_peers
            .values()
            .any(|p| p.transit_gateway_attachment_id == attach_id);
        if has_peers {
            return Err(Ec2Error::TgwConnectInUse(attach_id.to_string()));
        }
        if let Some(c) = self.tgw_connects.get_mut(attach_id) {
            c.state = "deleted".to_string();
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_tgw_connect_peer(
        &mut self,
        attachment_id: &str,
        peer_address: &str,
        transit_gateway_address: Option<&str>,
        inside_cidr_blocks: Vec<String>,
        peer_asn: i64,
        tags: Tags,
    ) -> Result<&TransitGatewayConnectPeer, Ec2Error> {
        if !self.tgw_connects.contains_key(attachment_id) {
            return Err(Ec2Error::InvalidTgwConnectNotFound(
                attachment_id.to_string(),
            ));
        }
        let peer_id = self.next_tgw_connect_peer_id();
        let tgw_addr = transit_gateway_address.unwrap_or("169.254.6.1").to_string();
        let bgp = TransitGatewayAttachmentBgpConfiguration {
            transit_gateway_asn: 64512,
            peer_asn,
            transit_gateway_address: tgw_addr.clone(),
            peer_address: peer_address.to_string(),
            bgp_status: "up".to_string(),
        };
        let peer = TransitGatewayConnectPeer {
            transit_gateway_attachment_id: attachment_id.to_string(),
            transit_gateway_connect_peer_id: peer_id.clone(),
            state: "available".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            transit_gateway_address: tgw_addr,
            peer_address: peer_address.to_string(),
            inside_cidr_blocks,
            protocol: "gre".to_string(),
            bgp_configurations: vec![bgp],
            tags,
        };
        self.tgw_connect_peers.insert(peer_id.clone(), peer);
        Ok(self.tgw_connect_peers.get(&peer_id).unwrap())
    }

    pub fn delete_tgw_connect_peer(&mut self, peer_id: &str) -> Result<(), Ec2Error> {
        // Mark deleted then drop so subsequent `delete_tgw_connect` does not see
        // it as a dependent peer.
        if !self.tgw_connect_peers.contains_key(peer_id) {
            return Err(Ec2Error::InvalidTgwConnectPeerNotFound(peer_id.to_string()));
        }
        self.tgw_connect_peers.remove(peer_id);
        Ok(())
    }

    pub fn create_tgw_metering_policy(
        &mut self,
        tgw_id: &str,
        name: &str,
        description: Option<String>,
        owner_id: &str,
        region: &str,
        tags: Tags,
    ) -> Result<&TransitGatewayMeteringPolicy, Ec2Error> {
        if !self.transit_gateways.contains_key(tgw_id) {
            return Err(Ec2Error::TransitGatewayNotFound(tgw_id.to_string()));
        }
        let policy_id = self.next_tgw_metering_policy_id();
        let arn =
            format!("arn:aws:ec2:{region}:{owner_id}:transit-gateway-metering-policy/{policy_id}");
        let policy = TransitGatewayMeteringPolicy {
            transit_gateway_metering_policy_id: policy_id.clone(),
            transit_gateway_metering_policy_arn: arn,
            transit_gateway_id: tgw_id.to_string(),
            name: name.to_string(),
            description,
            state: "available".to_string(),
            tags,
            last_updated_time: "2024-01-01T00:00:00Z".to_string(),
            version: 1,
            middlebox_attachment_ids: Vec::new(),
        };
        self.tgw_metering_policies.insert(policy_id.clone(), policy);
        Ok(self.tgw_metering_policies.get(&policy_id).unwrap())
    }

    pub fn delete_tgw_metering_policy(&mut self, policy_id: &str) -> Result<(), Ec2Error> {
        if !self.tgw_metering_policies.contains_key(policy_id) {
            return Err(Ec2Error::InvalidTgwMeteringPolicyNotFound(
                policy_id.to_string(),
            ));
        }
        // Cascade entries.
        self.tgw_metering_policy_entries
            .retain(|(p, _), _| p != policy_id);
        if let Some(p) = self.tgw_metering_policies.get_mut(policy_id) {
            p.state = "deleted".to_string();
        }
        Ok(())
    }

    pub fn modify_tgw_metering_policy(
        &mut self,
        policy_id: &str,
        new_name: Option<String>,
        new_description: Option<String>,
    ) -> Result<&TransitGatewayMeteringPolicy, Ec2Error> {
        let p = self
            .tgw_metering_policies
            .get_mut(policy_id)
            .ok_or_else(|| Ec2Error::InvalidTgwMeteringPolicyNotFound(policy_id.to_string()))?;
        if let Some(n) = new_name {
            p.name = n;
        }
        if let Some(d) = new_description {
            p.description = Some(d);
        }
        p.version += 1;
        p.last_updated_time = "2024-01-01T00:00:01Z".to_string();
        Ok(self.tgw_metering_policies.get(policy_id).unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_tgw_metering_policy_entry(
        &mut self,
        policy_id: &str,
        sequence_number: i32,
        action: &str,
        source_cidr_block: Option<String>,
        destination_cidr_block: Option<String>,
        protocol: Option<String>,
        source_port: Option<String>,
        destination_port: Option<String>,
        dimensions: Vec<String>,
    ) -> Result<&TransitGatewayMeteringPolicyEntry, Ec2Error> {
        if !self.tgw_metering_policies.contains_key(policy_id) {
            return Err(Ec2Error::InvalidTgwMeteringPolicyNotFound(
                policy_id.to_string(),
            ));
        }
        let entry_id = self.next_tgw_metering_policy_entry_id();
        let entry = TransitGatewayMeteringPolicyEntry {
            transit_gateway_metering_policy_entry_id: entry_id.clone(),
            transit_gateway_metering_policy_id: policy_id.to_string(),
            sequence_number,
            action: action.to_string(),
            source_cidr_block,
            destination_cidr_block,
            protocol,
            source_port,
            destination_port,
            dimensions,
            state: "available".to_string(),
        };
        let key = (policy_id.to_string(), entry_id.clone());
        self.tgw_metering_policy_entries.insert(key.clone(), entry);
        Ok(self.tgw_metering_policy_entries.get(&key).unwrap())
    }

    pub fn delete_tgw_metering_policy_entry(
        &mut self,
        policy_id: &str,
        entry_id: &str,
    ) -> Result<(), Ec2Error> {
        let key = (policy_id.to_string(), entry_id.to_string());
        if self.tgw_metering_policy_entries.remove(&key).is_none() {
            return Err(Ec2Error::InvalidTgwMeteringPolicyEntryNotFound(
                entry_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn create_tgw_policy_table(
        &mut self,
        tgw_id: &str,
        tags: Tags,
    ) -> Result<&TransitGatewayPolicyTable, Ec2Error> {
        if !self.transit_gateways.contains_key(tgw_id) {
            return Err(Ec2Error::TransitGatewayNotFound(tgw_id.to_string()));
        }
        let id = self.next_tgw_policy_table_id();
        let pt = TransitGatewayPolicyTable {
            transit_gateway_policy_table_id: id.clone(),
            transit_gateway_id: tgw_id.to_string(),
            state: "available".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            tags,
        };
        self.tgw_policy_tables.insert(id.clone(), pt);
        Ok(self.tgw_policy_tables.get(&id).unwrap())
    }

    pub fn delete_tgw_policy_table(&mut self, id: &str) -> Result<(), Ec2Error> {
        let pt = self
            .tgw_policy_tables
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidTgwPolicyTableNotFound(id.to_string()))?;
        pt.state = "deleted".to_string();
        // Drop associations cascading.
        self.tgw_policy_table_associations
            .retain(|(p, _), _| p != id);
        Ok(())
    }

    pub fn associate_tgw_policy_table(
        &mut self,
        policy_table_id: &str,
        attachment_id: &str,
    ) -> Result<&TransitGatewayPolicyTableAssociation, Ec2Error> {
        if !self.tgw_policy_tables.contains_key(policy_table_id) {
            return Err(Ec2Error::InvalidTgwPolicyTableNotFound(
                policy_table_id.to_string(),
            ));
        }
        let resource_id = self
            .tgw_vpc_attachments
            .get(attachment_id)
            .map(|a| a.vpc_id.clone())
            .unwrap_or_default();
        let key = (policy_table_id.to_string(), attachment_id.to_string());
        let assoc = TransitGatewayPolicyTableAssociation {
            transit_gateway_policy_table_id: policy_table_id.to_string(),
            transit_gateway_attachment_id: attachment_id.to_string(),
            resource_type: "vpc".to_string(),
            resource_id,
            state: "associated".to_string(),
        };
        self.tgw_policy_table_associations
            .insert(key.clone(), assoc);
        Ok(self.tgw_policy_table_associations.get(&key).unwrap())
    }

    pub fn disassociate_tgw_policy_table(
        &mut self,
        policy_table_id: &str,
        attachment_id: &str,
    ) -> Result<TransitGatewayPolicyTableAssociation, Ec2Error> {
        let key = (policy_table_id.to_string(), attachment_id.to_string());
        let mut assoc = self
            .tgw_policy_table_associations
            .remove(&key)
            .ok_or_else(|| {
                Ec2Error::InvalidTgwPolicyTableAssociationNotFound(
                    policy_table_id.to_string(),
                    attachment_id.to_string(),
                )
            })?;
        assoc.state = "disassociated".to_string();
        Ok(assoc)
    }

    pub fn create_tgw_prefix_list_reference(
        &mut self,
        route_table_id: &str,
        prefix_list_id: &str,
        owner_id: &str,
        blackhole: bool,
        attachment_id: Option<String>,
    ) -> Result<&TransitGatewayPrefixListReference, Ec2Error> {
        if !self.tgw_route_tables.contains_key(route_table_id) {
            return Err(Ec2Error::TgwRouteTableNotFound(route_table_id.to_string()));
        }
        let (resource_id, resource_type) = if let Some(att_id) = &attachment_id {
            let r = self
                .tgw_vpc_attachments
                .get(att_id)
                .map(|a| (a.vpc_id.clone(), "vpc".to_string()));
            r.unwrap_or((String::new(), String::new()))
        } else {
            (String::new(), String::new())
        };
        let key = (route_table_id.to_string(), prefix_list_id.to_string());
        let r = TransitGatewayPrefixListReference {
            transit_gateway_route_table_id: route_table_id.to_string(),
            prefix_list_id: prefix_list_id.to_string(),
            prefix_list_owner_id: owner_id.to_string(),
            state: "available".to_string(),
            blackhole,
            transit_gateway_attachment_id: attachment_id,
            resource_id: if resource_id.is_empty() {
                None
            } else {
                Some(resource_id)
            },
            resource_type: if resource_type.is_empty() {
                None
            } else {
                Some(resource_type)
            },
        };
        self.tgw_prefix_list_references.insert(key.clone(), r);
        Ok(self.tgw_prefix_list_references.get(&key).unwrap())
    }

    pub fn delete_tgw_prefix_list_reference(
        &mut self,
        route_table_id: &str,
        prefix_list_id: &str,
    ) -> Result<TransitGatewayPrefixListReference, Ec2Error> {
        let key = (route_table_id.to_string(), prefix_list_id.to_string());
        let mut r = self
            .tgw_prefix_list_references
            .remove(&key)
            .ok_or_else(|| {
                Ec2Error::InvalidTgwPrefixListReferenceNotFound(
                    route_table_id.to_string(),
                    prefix_list_id.to_string(),
                )
            })?;
        r.state = "deleted".to_string();
        Ok(r)
    }

    pub fn modify_tgw_prefix_list_reference(
        &mut self,
        route_table_id: &str,
        prefix_list_id: &str,
        blackhole: Option<bool>,
        attachment_id: Option<String>,
    ) -> Result<&TransitGatewayPrefixListReference, Ec2Error> {
        let key = (route_table_id.to_string(), prefix_list_id.to_string());
        if !self.tgw_prefix_list_references.contains_key(&key) {
            return Err(Ec2Error::InvalidTgwPrefixListReferenceNotFound(
                route_table_id.to_string(),
                prefix_list_id.to_string(),
            ));
        }
        // Look up attachment metadata (immutable borrow) before mutating.
        let new_resource = attachment_id.as_ref().and_then(|a| {
            self.tgw_vpc_attachments
                .get(a)
                .map(|att| (att.vpc_id.clone(), "vpc".to_string()))
        });
        let r = self.tgw_prefix_list_references.get_mut(&key).unwrap();
        if let Some(b) = blackhole {
            r.blackhole = b;
        }
        if let Some(a) = attachment_id {
            r.transit_gateway_attachment_id = Some(a);
            if let Some((rid, rt)) = new_resource {
                r.resource_id = Some(rid);
                r.resource_type = Some(rt);
            }
        }
        r.state = "available".to_string();
        Ok(self.tgw_prefix_list_references.get(&key).unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_tgw_route_table_announcement(
        &mut self,
        route_table_id: &str,
        peering_attachment_id: &str,
        announcement_direction: &str,
        peer_core_network_id: Option<String>,
    ) -> Result<&TransitGatewayRouteTableAnnouncement, Ec2Error> {
        let tgw_id = self
            .tgw_route_tables
            .get(route_table_id)
            .map(|r| r.transit_gateway_id.clone())
            .ok_or_else(|| Ec2Error::TgwRouteTableNotFound(route_table_id.to_string()))?;
        let peer_tgw_id = self
            .tgw_peering_attachments
            .get(peering_attachment_id)
            .map(|p| p.peer_transit_gateway_id.clone())
            .ok_or_else(|| {
                Ec2Error::TgwPeeringAttachmentNotFound(peering_attachment_id.to_string())
            })?;
        let id = self.next_tgw_route_table_announcement_id();
        let ann = TransitGatewayRouteTableAnnouncement {
            transit_gateway_route_table_announcement_id: id.clone(),
            transit_gateway_id: tgw_id,
            core_network_id: String::new(),
            peer_transit_gateway_id: peer_tgw_id,
            peer_core_network_id,
            peering_attachment_id: peering_attachment_id.to_string(),
            announcement_direction: announcement_direction.to_string(),
            transit_gateway_route_table_id: route_table_id.to_string(),
            state: "available".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            tags: Tags::new(),
        };
        self.tgw_route_table_announcements.insert(id.clone(), ann);
        Ok(self.tgw_route_table_announcements.get(&id).unwrap())
    }

    pub fn delete_tgw_route_table_announcement(&mut self, id: &str) -> Result<(), Ec2Error> {
        let ann = self
            .tgw_route_table_announcements
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidTgwRouteTableAnnouncementNotFound(id.to_string()))?;
        ann.state = "deleted".to_string();
        Ok(())
    }

    pub fn accept_tgw_vpc_attachment(
        &mut self,
        attach_id: &str,
    ) -> Result<&TransitGatewayVpcAttachment, Ec2Error> {
        let att = self
            .tgw_vpc_attachments
            .get_mut(attach_id)
            .ok_or_else(|| Ec2Error::TgwVpcAttachmentNotFound(attach_id.to_string()))?;
        if att.state != "pendingAcceptance" {
            return Err(Ec2Error::TgwAttachmentNotPendingAcceptance(
                attach_id.to_string(),
            ));
        }
        att.state = "available".to_string();
        Ok(self.tgw_vpc_attachments.get(attach_id).unwrap())
    }

    pub fn reject_tgw_vpc_attachment(
        &mut self,
        attach_id: &str,
    ) -> Result<&TransitGatewayVpcAttachment, Ec2Error> {
        let att = self
            .tgw_vpc_attachments
            .get_mut(attach_id)
            .ok_or_else(|| Ec2Error::TgwVpcAttachmentNotFound(attach_id.to_string()))?;
        if att.state != "pendingAcceptance" {
            return Err(Ec2Error::TgwAttachmentNotPendingAcceptance(
                attach_id.to_string(),
            ));
        }
        att.state = "rejected".to_string();
        Ok(self.tgw_vpc_attachments.get(attach_id).unwrap())
    }

    pub fn replace_tgw_route(
        &mut self,
        route_table_id: &str,
        cidr: &str,
        new_attachment_id: Option<String>,
        blackhole: bool,
    ) -> Result<TransitGatewayRoute, Ec2Error> {
        let routes = self
            .tgw_routes
            .get_mut(route_table_id)
            .ok_or_else(|| Ec2Error::TgwRouteTableNotFound(route_table_id.to_string()))?;
        let pos = routes
            .iter()
            .position(|r| r.destination_cidr_block == cidr)
            .ok_or_else(|| {
                Ec2Error::TgwRouteNotFound(route_table_id.to_string(), cidr.to_string())
            })?;
        let r = &mut routes[pos];
        r.attachment_id = new_attachment_id;
        r.state = if blackhole { "blackhole" } else { "active" }.to_string();
        Ok(r.clone())
    }

    // ----- Group 12: IPAM -----

    pub fn next_ipam_id(&mut self) -> String {
        self.counters.ipam += 1;
        format!("ipam-{:08x}", self.counters.ipam)
    }

    pub fn next_ipam_scope_id(&mut self) -> String {
        self.counters.ipam_scope += 1;
        format!("ipam-scope-{:08x}", self.counters.ipam_scope)
    }

    pub fn next_ipam_pool_id(&mut self) -> String {
        self.counters.ipam_pool += 1;
        format!("ipam-pool-{:08x}", self.counters.ipam_pool)
    }

    pub fn next_ipam_pool_cidr_id(&mut self) -> String {
        self.counters.ipam_pool_cidr += 1;
        format!("ipam-pool-cidr-{:08x}", self.counters.ipam_pool_cidr)
    }

    pub fn next_ipam_pool_allocation_id(&mut self) -> String {
        self.counters.ipam_pool_allocation += 1;
        format!("ipam-pool-alloc-{:08x}", self.counters.ipam_pool_allocation)
    }

    pub fn next_ipam_resource_discovery_id(&mut self) -> String {
        self.counters.ipam_resource_discovery += 1;
        format!(
            "ipam-res-disco-{:08x}",
            self.counters.ipam_resource_discovery
        )
    }

    pub fn next_ipam_resource_discovery_assoc_id(&mut self) -> String {
        self.counters.ipam_resource_discovery_association += 1;
        format!(
            "ipam-res-disco-assoc-{:08x}",
            self.counters.ipam_resource_discovery_association
        )
    }

    pub fn next_ipam_external_resource_verification_token_id(&mut self) -> String {
        self.counters.ipam_external_resource_verification_token += 1;
        format!(
            "ipam-ext-res-ver-token-{:08x}",
            self.counters.ipam_external_resource_verification_token
        )
    }

    pub fn next_ipam_policy_id(&mut self) -> String {
        self.counters.ipam_policy += 1;
        format!("ipam-policy-{:08x}", self.counters.ipam_policy)
    }

    pub fn next_ipam_prefix_list_resolver_id(&mut self) -> String {
        self.counters.ipam_prefix_list_resolver += 1;
        format!(
            "ipam-pl-resolver-{:08x}",
            self.counters.ipam_prefix_list_resolver
        )
    }

    pub fn next_ipam_prefix_list_resolver_target_id(&mut self) -> String {
        self.counters.ipam_prefix_list_resolver_target += 1;
        format!(
            "ipam-pl-resolver-tgt-{:08x}",
            self.counters.ipam_prefix_list_resolver_target
        )
    }

    /// Create an IPAM. Auto-creates a default `private` and a default `public` scope.
    #[allow(clippy::too_many_arguments)]
    pub fn create_ipam(
        &mut self,
        region: &str,
        owner_id: &str,
        description: Option<String>,
        operating_regions: Vec<IpamOperatingRegion>,
        tier: String,
        enable_private_gua: bool,
        metered_account: String,
        tags: Tags,
    ) -> &Ipam {
        let ipam_id = self.next_ipam_id();
        let ipam_arn = format!(
            "arn:aws:ec2::{owner_id}:ipam/{ipam_id}",
            owner_id = owner_id,
            ipam_id = ipam_id,
        );
        // Auto-create default scopes.
        let private_scope_id = self.next_ipam_scope_id();
        let public_scope_id = self.next_ipam_scope_id();
        let private_scope_arn = format!(
            "arn:aws:ec2::{owner_id}:ipam-scope/{id}",
            owner_id = owner_id,
            id = private_scope_id,
        );
        let public_scope_arn = format!(
            "arn:aws:ec2::{owner_id}:ipam-scope/{id}",
            owner_id = owner_id,
            id = public_scope_id,
        );
        for (sid, sarn, stype) in [
            (
                private_scope_id.clone(),
                private_scope_arn.clone(),
                "private",
            ),
            (public_scope_id.clone(), public_scope_arn.clone(), "public"),
        ] {
            let scope = IpamScope {
                ipam_scope_id: sid.clone(),
                ipam_scope_arn: sarn,
                ipam_arn: ipam_arn.clone(),
                ipam_region: region.to_string(),
                ipam_scope_type: stype.to_string(),
                is_default: true,
                description: None,
                pool_count: 0,
                state: "create-complete".to_string(),
                tags: Tags::new(),
                owner_id: owner_id.to_string(),
            };
            self.ipam_scopes.insert(sid, scope);
        }
        let ipam = Ipam {
            ipam_id: ipam_id.clone(),
            ipam_arn,
            ipam_region: region.to_string(),
            public_default_scope_id: public_scope_id,
            private_default_scope_id: private_scope_id,
            scope_count: 2,
            description,
            operating_regions,
            state: "create-complete".to_string(),
            owner_id: owner_id.to_string(),
            default_resource_discovery_id: None,
            default_resource_discovery_association_id: None,
            resource_discovery_association_count: 0,
            tier,
            enable_private_gua,
            metered_account,
            tags,
        };
        self.ipams.insert(ipam_id.clone(), ipam);
        self.ipams.get(&ipam_id).unwrap()
    }

    pub fn delete_ipam(&mut self, ipam_id: &str, cascade: bool) -> Result<Ipam, Ec2Error> {
        if !self.ipams.contains_key(ipam_id) {
            return Err(Ec2Error::InvalidIpamNotFound(ipam_id.to_string()));
        }
        // Find scopes belonging to this IPAM.
        let scope_ids: Vec<String> = self
            .ipam_scopes
            .values()
            .filter(|s| {
                self.ipams
                    .get(ipam_id)
                    .map(|i| s.ipam_arn == i.ipam_arn)
                    .unwrap_or(false)
            })
            .map(|s| s.ipam_scope_id.clone())
            .collect();
        // Find pools belonging to those scopes.
        let pool_ids: Vec<String> = self
            .ipam_pools
            .values()
            .filter(|p| {
                self.ipam_scopes.values().any(|s| {
                    scope_ids.contains(&s.ipam_scope_id) && s.ipam_scope_arn == p.ipam_scope_arn
                })
            })
            .map(|p| p.ipam_pool_id.clone())
            .collect();
        if !pool_ids.is_empty() && !cascade {
            return Err(Ec2Error::IpamInUse(ipam_id.to_string()));
        }
        if cascade {
            for pid in &pool_ids {
                self.ipam_pools.remove(pid);
                self.ipam_pool_cidrs.retain(|(pool, _), _| pool != pid);
                self.ipam_pool_allocations
                    .retain(|(pool, _), _| pool != pid);
            }
            for sid in &scope_ids {
                self.ipam_scopes.remove(sid);
            }
        } else {
            // Even without cascade we still wipe the auto-created default scopes.
            for sid in &scope_ids {
                self.ipam_scopes.remove(sid);
            }
        }
        let mut ipam = self.ipams.remove(ipam_id).unwrap();
        ipam.state = "delete-complete".to_string();
        Ok(ipam)
    }

    pub fn modify_ipam(
        &mut self,
        ipam_id: &str,
        description: Option<String>,
        add_operating_regions: Vec<IpamOperatingRegion>,
        remove_operating_regions: Vec<String>,
        tier: Option<String>,
        enable_private_gua: Option<bool>,
        metered_account: Option<String>,
    ) -> Result<&Ipam, Ec2Error> {
        let ipam = self
            .ipams
            .get_mut(ipam_id)
            .ok_or_else(|| Ec2Error::InvalidIpamNotFound(ipam_id.to_string()))?;
        if let Some(d) = description {
            ipam.description = Some(d);
        }
        if !remove_operating_regions.is_empty() {
            ipam.operating_regions
                .retain(|r| !remove_operating_regions.contains(&r.region_name));
        }
        for r in add_operating_regions {
            if !ipam
                .operating_regions
                .iter()
                .any(|x| x.region_name == r.region_name)
            {
                ipam.operating_regions.push(r);
            }
        }
        if let Some(t) = tier {
            ipam.tier = t;
        }
        if let Some(v) = enable_private_gua {
            ipam.enable_private_gua = v;
        }
        if let Some(m) = metered_account {
            ipam.metered_account = m;
        }
        ipam.state = "modify-complete".to_string();
        Ok(ipam)
    }

    pub fn create_ipam_scope(
        &mut self,
        ipam_id: &str,
        description: Option<String>,
        tags: Tags,
    ) -> Result<&IpamScope, Ec2Error> {
        let ipam = self
            .ipams
            .get(ipam_id)
            .ok_or_else(|| Ec2Error::InvalidIpamNotFound(ipam_id.to_string()))?;
        let owner = ipam.owner_id.clone();
        let region = ipam.ipam_region.clone();
        let ipam_arn = ipam.ipam_arn.clone();
        let scope_id = self.next_ipam_scope_id();
        let scope_arn = format!("arn:aws:ec2::{owner}:ipam-scope/{scope_id}",);
        let scope = IpamScope {
            ipam_scope_id: scope_id.clone(),
            ipam_scope_arn: scope_arn,
            ipam_arn,
            ipam_region: region,
            ipam_scope_type: "private".to_string(),
            is_default: false,
            description,
            pool_count: 0,
            state: "create-complete".to_string(),
            tags,
            owner_id: owner,
        };
        self.ipam_scopes.insert(scope_id.clone(), scope);
        // Bump IPAM scope_count.
        if let Some(i) = self.ipams.get_mut(ipam_id) {
            i.scope_count += 1;
        }
        Ok(self.ipam_scopes.get(&scope_id).unwrap())
    }

    pub fn delete_ipam_scope(&mut self, scope_id: &str) -> Result<IpamScope, Ec2Error> {
        let scope = self
            .ipam_scopes
            .get(scope_id)
            .ok_or_else(|| Ec2Error::InvalidIpamScopeNotFound(scope_id.to_string()))?;
        if scope.is_default {
            return Err(Ec2Error::IpamScopeIsDefault(scope_id.to_string()));
        }
        let scope_arn = scope.ipam_scope_arn.clone();
        let ipam_arn = scope.ipam_arn.clone();
        let has_pools = self
            .ipam_pools
            .values()
            .any(|p| p.ipam_scope_arn == scope_arn);
        if has_pools {
            return Err(Ec2Error::IpamScopeInUse(scope_id.to_string()));
        }
        let mut removed = self.ipam_scopes.remove(scope_id).unwrap();
        removed.state = "delete-complete".to_string();
        // Decrement IPAM scope_count.
        if let Some(ipam) = self.ipams.values_mut().find(|i| i.ipam_arn == ipam_arn) {
            if ipam.scope_count > 0 {
                ipam.scope_count -= 1;
            }
        }
        Ok(removed)
    }

    pub fn modify_ipam_scope(
        &mut self,
        scope_id: &str,
        description: Option<String>,
    ) -> Result<&IpamScope, Ec2Error> {
        let scope = self
            .ipam_scopes
            .get_mut(scope_id)
            .ok_or_else(|| Ec2Error::InvalidIpamScopeNotFound(scope_id.to_string()))?;
        if let Some(d) = description {
            scope.description = Some(d);
        }
        scope.state = "modify-complete".to_string();
        Ok(scope)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_ipam_pool(
        &mut self,
        scope_id: &str,
        address_family: String,
        description: Option<String>,
        locale: Option<String>,
        publicly_advertisable: bool,
        auto_import: bool,
        allocation_min_netmask_length: Option<i32>,
        allocation_max_netmask_length: Option<i32>,
        allocation_default_netmask_length: Option<i32>,
        allocation_resource_tags: Vec<(String, String)>,
        aws_service: Option<String>,
        public_ip_source: Option<String>,
        source_ipam_pool_id: Option<String>,
        tags: Tags,
    ) -> Result<&IpamPool, Ec2Error> {
        let scope = self
            .ipam_scopes
            .get(scope_id)
            .ok_or_else(|| Ec2Error::InvalidIpamScopeNotFound(scope_id.to_string()))?;
        let scope_arn = scope.ipam_scope_arn.clone();
        let scope_type = scope.ipam_scope_type.clone();
        let ipam_arn = scope.ipam_arn.clone();
        let region = scope.ipam_region.clone();
        let owner = scope.owner_id.clone();
        let pool_id = self.next_ipam_pool_id();
        let pool_arn = format!("arn:aws:ec2::{owner}:ipam-pool/{pool_id}",);
        let pool = IpamPool {
            ipam_pool_id: pool_id.clone(),
            source_ipam_pool_id,
            ipam_pool_arn: pool_arn,
            ipam_scope_arn: scope_arn,
            ipam_scope_type: scope_type,
            ipam_arn,
            ipam_region: region.clone(),
            locale: locale.unwrap_or_else(|| region.clone()),
            pool_depth: 1,
            state: "create-complete".to_string(),
            state_message: None,
            description,
            auto_import,
            publicly_advertisable,
            address_family,
            allocation_min_netmask_length,
            allocation_max_netmask_length,
            allocation_default_netmask_length,
            allocation_resource_tags,
            aws_service,
            public_ip_source,
            source_resource_id: None,
            source_resource_type: None,
            source_resource_region: None,
            source_resource_owner: None,
            tags,
            owner_id: owner,
            allocation_count: 0,
        };
        self.ipam_pools.insert(pool_id.clone(), pool);
        // Bump scope.pool_count.
        if let Some(s) = self.ipam_scopes.get_mut(scope_id) {
            s.pool_count += 1;
        }
        Ok(self.ipam_pools.get(&pool_id).unwrap())
    }

    pub fn delete_ipam_pool(&mut self, pool_id: &str) -> Result<IpamPool, Ec2Error> {
        let pool = self
            .ipam_pools
            .get(pool_id)
            .ok_or_else(|| Ec2Error::InvalidIpamPoolNotFound(pool_id.to_string()))?;
        let scope_arn = pool.ipam_scope_arn.clone();
        let has_allocations = self
            .ipam_pool_allocations
            .keys()
            .any(|(pid, _)| pid == pool_id);
        if has_allocations {
            return Err(Ec2Error::IpamPoolInUse(pool_id.to_string()));
        }
        // Remove dependent CIDRs (no allocations means we can clean them up).
        self.ipam_pool_cidrs.retain(|(pid, _), _| pid != pool_id);
        let mut removed = self.ipam_pools.remove(pool_id).unwrap();
        removed.state = "delete-complete".to_string();
        // Decrement scope.pool_count.
        if let Some(scope) = self
            .ipam_scopes
            .values_mut()
            .find(|s| s.ipam_scope_arn == scope_arn)
        {
            if scope.pool_count > 0 {
                scope.pool_count -= 1;
            }
        }
        Ok(removed)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_ipam_pool(
        &mut self,
        pool_id: &str,
        description: Option<String>,
        auto_import: Option<bool>,
        allocation_min_netmask_length: Option<i32>,
        allocation_max_netmask_length: Option<i32>,
        allocation_default_netmask_length: Option<i32>,
        add_allocation_resource_tags: Vec<(String, String)>,
        remove_allocation_resource_tags: Vec<(String, String)>,
    ) -> Result<&IpamPool, Ec2Error> {
        let pool = self
            .ipam_pools
            .get_mut(pool_id)
            .ok_or_else(|| Ec2Error::InvalidIpamPoolNotFound(pool_id.to_string()))?;
        if let Some(d) = description {
            pool.description = Some(d);
        }
        if let Some(v) = auto_import {
            pool.auto_import = v;
        }
        if let Some(v) = allocation_min_netmask_length {
            pool.allocation_min_netmask_length = Some(v);
        }
        if let Some(v) = allocation_max_netmask_length {
            pool.allocation_max_netmask_length = Some(v);
        }
        if let Some(v) = allocation_default_netmask_length {
            pool.allocation_default_netmask_length = Some(v);
        }
        for tag in add_allocation_resource_tags {
            if !pool
                .allocation_resource_tags
                .iter()
                .any(|(k, _)| k == &tag.0)
            {
                pool.allocation_resource_tags.push(tag);
            }
        }
        for tag in remove_allocation_resource_tags {
            pool.allocation_resource_tags.retain(|(k, _)| k != &tag.0);
        }
        pool.state = "modify-complete".to_string();
        Ok(pool)
    }

    pub fn provision_ipam_pool_cidr(
        &mut self,
        pool_id: &str,
        cidr: Option<String>,
        netmask_length: Option<i32>,
    ) -> Result<&IpamPoolCidr, Ec2Error> {
        if !self.ipam_pools.contains_key(pool_id) {
            return Err(Ec2Error::InvalidIpamPoolNotFound(pool_id.to_string()));
        }
        let cidr_str = cidr.unwrap_or_else(|| {
            // Synthesize a placeholder CIDR if only netmask given.
            format!(
                "10.{}.0.0/{}",
                self.counters.ipam_pool_cidr & 0xff,
                netmask_length.unwrap_or(16)
            )
        });
        let pool_cidr_id = self.next_ipam_pool_cidr_id();
        let entry = IpamPoolCidr {
            cidr: cidr_str.clone(),
            state: "provisioned".to_string(),
            failure_reason: None,
            ipam_pool_cidr_id: pool_cidr_id,
            netmask_length,
        };
        let key = (pool_id.to_string(), cidr_str.clone());
        self.ipam_pool_cidrs.insert(key.clone(), entry);
        Ok(self.ipam_pool_cidrs.get(&key).unwrap())
    }

    pub fn deprovision_ipam_pool_cidr(
        &mut self,
        pool_id: &str,
        cidr: &str,
    ) -> Result<IpamPoolCidr, Ec2Error> {
        let key = (pool_id.to_string(), cidr.to_string());
        let mut entry = self.ipam_pool_cidrs.remove(&key).ok_or_else(|| {
            Ec2Error::InvalidIpamPoolCidrNotFound(pool_id.to_string(), cidr.to_string())
        })?;
        entry.state = "deprovisioned".to_string();
        Ok(entry)
    }

    pub fn allocate_ipam_pool_cidr(
        &mut self,
        pool_id: &str,
        cidr: Option<String>,
        netmask_length: Option<i32>,
        description: Option<String>,
    ) -> Result<&IpamPoolAllocation, Ec2Error> {
        let pool = self
            .ipam_pools
            .get(pool_id)
            .ok_or_else(|| Ec2Error::InvalidIpamPoolNotFound(pool_id.to_string()))?;
        let region = pool.ipam_region.clone();
        let owner = pool.owner_id.clone();
        let allocation_id = self.next_ipam_pool_allocation_id();
        let cidr_str = cidr.unwrap_or_else(|| {
            format!(
                "10.{}.0.0/{}",
                self.counters.ipam_pool_allocation & 0xff,
                netmask_length.unwrap_or(28)
            )
        });
        let alloc = IpamPoolAllocation {
            ipam_pool_allocation_id: allocation_id.clone(),
            cidr: cidr_str,
            ipam_pool_id: pool_id.to_string(),
            description,
            resource_id: None,
            resource_type: "custom".to_string(),
            resource_region: Some(region),
            resource_owner: Some(owner),
        };
        let key = (pool_id.to_string(), allocation_id);
        self.ipam_pool_allocations.insert(key.clone(), alloc);
        if let Some(p) = self.ipam_pools.get_mut(pool_id) {
            p.allocation_count += 1;
        }
        Ok(self.ipam_pool_allocations.get(&key).unwrap())
    }

    pub fn release_ipam_pool_allocation(
        &mut self,
        pool_id: &str,
        allocation_id: &str,
    ) -> Result<(), Ec2Error> {
        let key = (pool_id.to_string(), allocation_id.to_string());
        if self.ipam_pool_allocations.remove(&key).is_none() {
            return Err(Ec2Error::InvalidIpamPoolAllocationNotFound(
                pool_id.to_string(),
                allocation_id.to_string(),
            ));
        }
        if let Some(p) = self.ipam_pools.get_mut(pool_id) {
            if p.allocation_count > 0 {
                p.allocation_count -= 1;
            }
        }
        Ok(())
    }

    pub fn create_ipam_resource_discovery(
        &mut self,
        region: &str,
        owner_id: &str,
        description: Option<String>,
        operating_regions: Vec<IpamOperatingRegion>,
        tags: Tags,
    ) -> &IpamResourceDiscovery {
        let id = self.next_ipam_resource_discovery_id();
        let arn = format!("arn:aws:ec2::{owner_id}:ipam-resource-discovery/{id}",);
        let rd = IpamResourceDiscovery {
            ipam_resource_discovery_id: id.clone(),
            ipam_resource_discovery_arn: arn,
            ipam_resource_discovery_region: region.to_string(),
            description,
            operating_regions,
            is_default: false,
            state: "create-complete".to_string(),
            owner_id: owner_id.to_string(),
            tags,
        };
        self.ipam_resource_discoveries.insert(id.clone(), rd);
        self.ipam_resource_discoveries.get(&id).unwrap()
    }

    pub fn delete_ipam_resource_discovery(
        &mut self,
        id: &str,
    ) -> Result<IpamResourceDiscovery, Ec2Error> {
        let rd = self
            .ipam_resource_discoveries
            .get(id)
            .ok_or_else(|| Ec2Error::InvalidIpamResourceDiscoveryNotFound(id.to_string()))?;
        if rd.is_default {
            return Err(Ec2Error::IpamResourceDiscoveryInUse(id.to_string()));
        }
        let has_assoc = self
            .ipam_resource_discovery_associations
            .values()
            .any(|a| a.ipam_resource_discovery_id == id);
        if has_assoc {
            return Err(Ec2Error::IpamResourceDiscoveryInUse(id.to_string()));
        }
        let mut removed = self.ipam_resource_discoveries.remove(id).unwrap();
        removed.state = "delete-complete".to_string();
        Ok(removed)
    }

    pub fn modify_ipam_resource_discovery(
        &mut self,
        id: &str,
        description: Option<String>,
        add_operating_regions: Vec<IpamOperatingRegion>,
        remove_operating_regions: Vec<String>,
    ) -> Result<&IpamResourceDiscovery, Ec2Error> {
        let rd = self
            .ipam_resource_discoveries
            .get_mut(id)
            .ok_or_else(|| Ec2Error::InvalidIpamResourceDiscoveryNotFound(id.to_string()))?;
        if let Some(d) = description {
            rd.description = Some(d);
        }
        if !remove_operating_regions.is_empty() {
            rd.operating_regions
                .retain(|r| !remove_operating_regions.contains(&r.region_name));
        }
        for r in add_operating_regions {
            if !rd
                .operating_regions
                .iter()
                .any(|x| x.region_name == r.region_name)
            {
                rd.operating_regions.push(r);
            }
        }
        rd.state = "modify-complete".to_string();
        Ok(rd)
    }

    pub fn associate_ipam_resource_discovery(
        &mut self,
        ipam_id: &str,
        resource_discovery_id: &str,
        tags: Tags,
    ) -> Result<&IpamResourceDiscoveryAssociation, Ec2Error> {
        let ipam = self
            .ipams
            .get(ipam_id)
            .ok_or_else(|| Ec2Error::InvalidIpamNotFound(ipam_id.to_string()))?;
        if !self
            .ipam_resource_discoveries
            .contains_key(resource_discovery_id)
        {
            return Err(Ec2Error::InvalidIpamResourceDiscoveryNotFound(
                resource_discovery_id.to_string(),
            ));
        }
        let owner = ipam.owner_id.clone();
        let region = ipam.ipam_region.clone();
        let ipam_arn = ipam.ipam_arn.clone();
        let assoc_id = self.next_ipam_resource_discovery_assoc_id();
        let assoc_arn =
            format!("arn:aws:ec2::{owner}:ipam-resource-discovery-association/{assoc_id}",);
        let assoc = IpamResourceDiscoveryAssociation {
            ipam_resource_discovery_association_id: assoc_id.clone(),
            ipam_resource_discovery_association_arn: assoc_arn,
            ipam_arn,
            ipam_id: ipam_id.to_string(),
            ipam_region: region,
            ipam_resource_discovery_id: resource_discovery_id.to_string(),
            owner_id: owner,
            is_default: false,
            resource_discovery_status: "active".to_string(),
            state: "associate-complete".to_string(),
            tags,
        };
        self.ipam_resource_discovery_associations
            .insert(assoc_id.clone(), assoc);
        if let Some(i) = self.ipams.get_mut(ipam_id) {
            i.resource_discovery_association_count += 1;
        }
        Ok(self
            .ipam_resource_discovery_associations
            .get(&assoc_id)
            .unwrap())
    }

    pub fn disassociate_ipam_resource_discovery(
        &mut self,
        assoc_id: &str,
    ) -> Result<IpamResourceDiscoveryAssociation, Ec2Error> {
        let assoc = self
            .ipam_resource_discovery_associations
            .get(assoc_id)
            .ok_or_else(|| {
                Ec2Error::InvalidIpamResourceDiscoveryAssociationNotFound(assoc_id.to_string())
            })?;
        let ipam_id = assoc.ipam_id.clone();
        let mut removed = self
            .ipam_resource_discovery_associations
            .remove(assoc_id)
            .unwrap();
        removed.state = "disassociate-complete".to_string();
        if let Some(i) = self.ipams.get_mut(&ipam_id) {
            if i.resource_discovery_association_count > 0 {
                i.resource_discovery_association_count -= 1;
            }
        }
        Ok(removed)
    }

    pub fn provision_ipam_byoasn(
        &mut self,
        ipam_id: &str,
        asn: &str,
        description: Option<String>,
    ) -> Result<&IpamByoasn, Ec2Error> {
        if !self.ipams.contains_key(ipam_id) {
            return Err(Ec2Error::InvalidIpamNotFound(ipam_id.to_string()));
        }
        let entry = IpamByoasn {
            asn: asn.to_string(),
            ipam_id: ipam_id.to_string(),
            description,
            state: "provisioned".to_string(),
            status_message: None,
        };
        let key = (ipam_id.to_string(), asn.to_string());
        self.ipam_byoasns.insert(key.clone(), entry);
        Ok(self.ipam_byoasns.get(&key).unwrap())
    }

    pub fn deprovision_ipam_byoasn(
        &mut self,
        ipam_id: &str,
        asn: &str,
    ) -> Result<IpamByoasn, Ec2Error> {
        let key = (ipam_id.to_string(), asn.to_string());
        let mut entry = self.ipam_byoasns.remove(&key).ok_or_else(|| {
            Ec2Error::InvalidIpamByoasnNotFound(ipam_id.to_string(), asn.to_string())
        })?;
        entry.state = "deprovisioned".to_string();
        Ok(entry)
    }

    pub fn associate_ipam_byoasn(
        &mut self,
        asn: &str,
        cidr: &str,
    ) -> Result<AsnAssociation, Ec2Error> {
        // Find a provisioned BYOASN with the matching ASN.
        let exists = self.ipam_byoasns.keys().any(|(_, a)| a == asn);
        if !exists {
            return Err(Ec2Error::InvalidIpamByoasnNotFound(
                String::new(),
                asn.to_string(),
            ));
        }
        let cidr_entry = self
            .byoip_cidrs
            .get_mut(cidr)
            .ok_or_else(|| Ec2Error::InvalidByoipCidrNotFound(cidr.to_string()))?;
        if cidr_entry.asn_association.is_some() {
            return Err(Ec2Error::IpamByoasnAlreadyAssociated(asn.to_string()));
        }
        let assoc = AsnAssociation {
            asn: asn.to_string(),
            cidr: cidr.to_string(),
            state: "associated".to_string(),
            status_message: None,
        };
        cidr_entry.asn_association = Some(assoc.clone());
        Ok(assoc)
    }

    pub fn disassociate_ipam_byoasn(
        &mut self,
        asn: &str,
        cidr: &str,
    ) -> Result<AsnAssociation, Ec2Error> {
        let cidr_entry = self
            .byoip_cidrs
            .get_mut(cidr)
            .ok_or_else(|| Ec2Error::InvalidByoipCidrNotFound(cidr.to_string()))?;
        let mut assoc = cidr_entry
            .asn_association
            .take()
            .ok_or_else(|| Ec2Error::InvalidIpamByoasnNotFound(String::new(), asn.to_string()))?;
        assoc.state = "disassociated".to_string();
        Ok(assoc)
    }

    pub fn create_ipam_external_resource_verification_token(
        &mut self,
        ipam_id: &str,
        tags: Tags,
    ) -> Result<&IpamExternalResourceVerificationToken, Ec2Error> {
        let ipam = self
            .ipams
            .get(ipam_id)
            .ok_or_else(|| Ec2Error::InvalidIpamNotFound(ipam_id.to_string()))?;
        let region = ipam.ipam_region.clone();
        let ipam_arn = ipam.ipam_arn.clone();
        let owner = ipam.owner_id.clone();
        let token_id = self.next_ipam_external_resource_verification_token_id();
        let token_arn =
            format!("arn:aws:ec2::{owner}:ipam-external-resource-verification-token/{token_id}",);
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let entry = IpamExternalResourceVerificationToken {
            ipam_external_resource_verification_token_id: token_id.clone(),
            ipam_external_resource_verification_token_arn: token_arn,
            ipam_id: ipam_id.to_string(),
            ipam_arn,
            ipam_region: region,
            token_value: format!("token-{token_id}"),
            token_name: format!("token-name-{token_id}"),
            not_after: now,
            status: "valid".to_string(),
            state: "create-complete".to_string(),
            tags,
        };
        self.ipam_external_resource_verification_tokens
            .insert(token_id.clone(), entry);
        Ok(self
            .ipam_external_resource_verification_tokens
            .get(&token_id)
            .unwrap())
    }

    pub fn delete_ipam_external_resource_verification_token(
        &mut self,
        token_id: &str,
    ) -> Result<IpamExternalResourceVerificationToken, Ec2Error> {
        let mut entry = self
            .ipam_external_resource_verification_tokens
            .remove(token_id)
            .ok_or_else(|| {
                Ec2Error::InvalidIpamExternalResourceVerificationTokenNotFound(token_id.to_string())
            })?;
        entry.state = "delete-complete".to_string();
        Ok(entry)
    }

    pub fn create_ipam_policy(
        &mut self,
        ipam_id: &str,
        policy_name: String,
        description: Option<String>,
        allocation_rules: Vec<IpamPolicyAllocationRule>,
        tags: Tags,
    ) -> Result<&IpamPolicy, Ec2Error> {
        let ipam = self
            .ipams
            .get(ipam_id)
            .ok_or_else(|| Ec2Error::InvalidIpamNotFound(ipam_id.to_string()))?;
        let region = ipam.ipam_region.clone();
        let ipam_arn = ipam.ipam_arn.clone();
        let owner = ipam.owner_id.clone();
        let policy_id = self.next_ipam_policy_id();
        let policy_arn = format!("arn:aws:ec2::{owner}:ipam-policy/{policy_id}",);
        let policy = IpamPolicy {
            ipam_policy_id: policy_id.clone(),
            ipam_policy_arn: policy_arn,
            ipam_arn,
            ipam_region: region,
            policy_name,
            policy_type: "allocation".to_string(),
            description,
            state: "create-complete".to_string(),
            allocation_rules,
            tags,
            owner_id: owner,
        };
        self.ipam_policies.insert(policy_id.clone(), policy);
        Ok(self.ipam_policies.get(&policy_id).unwrap())
    }

    pub fn delete_ipam_policy(&mut self, policy_id: &str) -> Result<IpamPolicy, Ec2Error> {
        let mut removed = self
            .ipam_policies
            .remove(policy_id)
            .ok_or_else(|| Ec2Error::InvalidIpamPolicyNotFound(policy_id.to_string()))?;
        removed.state = "delete-complete".to_string();
        Ok(removed)
    }

    pub fn modify_ipam_policy_allocation_rules(
        &mut self,
        policy_id: &str,
        add_rules: Vec<IpamPolicyAllocationRule>,
        remove_rules: Vec<IpamPolicyAllocationRule>,
    ) -> Result<&IpamPolicy, Ec2Error> {
        let policy = self
            .ipam_policies
            .get_mut(policy_id)
            .ok_or_else(|| Ec2Error::InvalidIpamPolicyNotFound(policy_id.to_string()))?;
        let remove_pool_ids: Vec<Option<String>> = remove_rules
            .into_iter()
            .map(|r| r.source_ipam_pool_id)
            .collect();
        if !remove_pool_ids.is_empty() {
            policy
                .allocation_rules
                .retain(|r| !remove_pool_ids.contains(&r.source_ipam_pool_id));
        }
        for r in add_rules {
            if !policy
                .allocation_rules
                .iter()
                .any(|x| x.source_ipam_pool_id == r.source_ipam_pool_id)
            {
                policy.allocation_rules.push(r);
            }
        }
        Ok(policy)
    }

    pub fn create_ipam_prefix_list_resolver(
        &mut self,
        ipam_id: &str,
        name: String,
        description: Option<String>,
        tags: Tags,
    ) -> Result<&IpamPrefixListResolver, Ec2Error> {
        let ipam = self
            .ipams
            .get(ipam_id)
            .ok_or_else(|| Ec2Error::InvalidIpamNotFound(ipam_id.to_string()))?;
        let region = ipam.ipam_region.clone();
        let ipam_arn = ipam.ipam_arn.clone();
        let owner = ipam.owner_id.clone();
        let resolver_id = self.next_ipam_prefix_list_resolver_id();
        let resolver_arn = format!("arn:aws:ec2::{owner}:ipam-prefix-list-resolver/{resolver_id}",);
        let resolver = IpamPrefixListResolver {
            ipam_prefix_list_resolver_id: resolver_id.clone(),
            ipam_prefix_list_resolver_arn: resolver_arn,
            ipam_arn,
            ipam_region: region,
            name,
            description,
            state: "available".to_string(),
            owner_id: owner,
            target_count: 0,
            tags,
        };
        self.ipam_prefix_list_resolvers
            .insert(resolver_id.clone(), resolver);
        Ok(self.ipam_prefix_list_resolvers.get(&resolver_id).unwrap())
    }

    pub fn delete_ipam_prefix_list_resolver(
        &mut self,
        resolver_id: &str,
    ) -> Result<IpamPrefixListResolver, Ec2Error> {
        if !self.ipam_prefix_list_resolvers.contains_key(resolver_id) {
            return Err(Ec2Error::InvalidIpamPrefixListResolverNotFound(
                resolver_id.to_string(),
            ));
        }
        let has_targets = self
            .ipam_prefix_list_resolver_targets
            .keys()
            .any(|(rid, _)| rid == resolver_id);
        if has_targets {
            return Err(Ec2Error::IpamPrefixListResolverInUse(
                resolver_id.to_string(),
            ));
        }
        let mut removed = self.ipam_prefix_list_resolvers.remove(resolver_id).unwrap();
        removed.state = "deleted".to_string();
        Ok(removed)
    }

    pub fn modify_ipam_prefix_list_resolver(
        &mut self,
        resolver_id: &str,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<&IpamPrefixListResolver, Ec2Error> {
        let resolver = self
            .ipam_prefix_list_resolvers
            .get_mut(resolver_id)
            .ok_or_else(|| {
                Ec2Error::InvalidIpamPrefixListResolverNotFound(resolver_id.to_string())
            })?;
        if let Some(n) = name {
            resolver.name = n;
        }
        if let Some(d) = description {
            resolver.description = Some(d);
        }
        resolver.state = "modifying".to_string();
        Ok(resolver)
    }

    pub fn create_ipam_prefix_list_resolver_target(
        &mut self,
        resolver_id: &str,
        target_resource_arn: String,
        target_resource_type: String,
        target_resource_region: String,
        tags: Tags,
    ) -> Result<&IpamPrefixListResolverTarget, Ec2Error> {
        let resolver = self
            .ipam_prefix_list_resolvers
            .get(resolver_id)
            .ok_or_else(|| {
                Ec2Error::InvalidIpamPrefixListResolverNotFound(resolver_id.to_string())
            })?;
        let owner = resolver.owner_id.clone();
        let target_id = self.next_ipam_prefix_list_resolver_target_id();
        let target = IpamPrefixListResolverTarget {
            ipam_prefix_list_resolver_target_id: target_id.clone(),
            ipam_prefix_list_resolver_id: resolver_id.to_string(),
            target_resource_arn,
            target_resource_type,
            target_resource_region,
            owner_id: owner,
            state: "available".to_string(),
            tags,
        };
        let key = (resolver_id.to_string(), target_id);
        self.ipam_prefix_list_resolver_targets
            .insert(key.clone(), target);
        if let Some(r) = self.ipam_prefix_list_resolvers.get_mut(resolver_id) {
            r.target_count += 1;
        }
        Ok(self.ipam_prefix_list_resolver_targets.get(&key).unwrap())
    }

    pub fn delete_ipam_prefix_list_resolver_target(
        &mut self,
        resolver_id: &str,
        target_id: &str,
    ) -> Result<IpamPrefixListResolverTarget, Ec2Error> {
        let key = (resolver_id.to_string(), target_id.to_string());
        let mut removed = self
            .ipam_prefix_list_resolver_targets
            .remove(&key)
            .ok_or_else(|| {
                Ec2Error::InvalidIpamPrefixListResolverTargetNotFound(
                    resolver_id.to_string(),
                    target_id.to_string(),
                )
            })?;
        removed.state = "deleted".to_string();
        if let Some(r) = self.ipam_prefix_list_resolvers.get_mut(resolver_id) {
            if r.target_count > 0 {
                r.target_count -= 1;
            }
        }
        Ok(removed)
    }

    pub fn modify_ipam_prefix_list_resolver_target(
        &mut self,
        resolver_id: &str,
        target_id: &str,
        target_resource_region: Option<String>,
    ) -> Result<&IpamPrefixListResolverTarget, Ec2Error> {
        let key = (resolver_id.to_string(), target_id.to_string());
        let target = self
            .ipam_prefix_list_resolver_targets
            .get_mut(&key)
            .ok_or_else(|| {
                Ec2Error::InvalidIpamPrefixListResolverTargetNotFound(
                    resolver_id.to_string(),
                    target_id.to_string(),
                )
            })?;
        if let Some(r) = target_resource_region {
            target.target_resource_region = r;
        }
        target.state = "modifying".to_string();
        Ok(target)
    }

    pub fn move_byoip_cidr_to_ipam(
        &mut self,
        cidr: &str,
        ipam_pool_id: &str,
    ) -> Result<&ByoipCidr, Ec2Error> {
        if !self.ipam_pools.contains_key(ipam_pool_id) {
            return Err(Ec2Error::InvalidIpamPoolNotFound(ipam_pool_id.to_string()));
        }
        let entry = self
            .byoip_cidrs
            .get_mut(cidr)
            .ok_or_else(|| Ec2Error::InvalidByoipCidrNotFound(cidr.to_string()))?;
        entry.ipam_pool_id = Some(ipam_pool_id.to_string());
        Ok(entry)
    }

    // -----------------------------------------------------------------------
    // Batch B: ID generators and state operations.
    // -----------------------------------------------------------------------

    pub fn next_bundle_task_id(&mut self) -> String {
        self.counters.bundle_task += 1;
        format!("bun-{:08x}", self.counters.bundle_task)
    }

    pub fn next_import_volume_task_id(&mut self) -> String {
        self.counters.import_volume_task += 1;
        format!("import-vol-{:08x}", self.counters.import_volume_task)
    }

    pub fn next_export_image_task_id(&mut self) -> String {
        self.counters.export_image_task += 1;
        format!("export-ami-{:08x}", self.counters.export_image_task)
    }

    pub fn next_outpost_lag_id(&mut self) -> String {
        self.counters.outpost_lag += 1;
        format!("outpost-lag-{:08x}", self.counters.outpost_lag)
    }

    /// `ModifyVolume`: mutate the volume in place and record a `VolumeModification`.
    #[allow(clippy::too_many_arguments)]
    pub fn modify_volume(
        &mut self,
        volume_id: &str,
        new_size: Option<i32>,
        new_volume_type: Option<String>,
        new_iops: Option<i32>,
        new_throughput: Option<i32>,
        new_multi_attach_enabled: Option<bool>,
    ) -> Result<&VolumeModification, Ec2Error> {
        let now = self.now_iso();
        let vol = self
            .volumes
            .get_mut(volume_id)
            .ok_or_else(|| Ec2Error::VolumeNotFound(volume_id.to_string()))?;
        let original_size = vol.size;
        let original_iops = vol.iops;
        let original_throughput = vol.throughput;
        let original_volume_type = vol.volume_type.clone();
        if let Some(s) = new_size {
            vol.size = s;
        }
        if let Some(ref vt) = new_volume_type {
            vol.volume_type = vt.clone();
        }
        if let Some(i) = new_iops {
            vol.iops = Some(i);
        }
        if let Some(t) = new_throughput {
            vol.throughput = Some(t);
        }
        let modification = VolumeModification {
            volume_id: volume_id.to_string(),
            modification_state: "completed".to_string(),
            status_message: None,
            target_size: new_size.or(Some(original_size)),
            target_iops: new_iops.or(original_iops),
            target_throughput: new_throughput.or(original_throughput),
            target_volume_type: new_volume_type.or(Some(original_volume_type.clone())),
            target_multi_attach_enabled: new_multi_attach_enabled,
            original_size: Some(original_size),
            original_iops,
            original_throughput,
            original_volume_type: Some(original_volume_type),
            original_multi_attach_enabled: None,
            progress: 100,
            start_time: now.clone(),
            end_time: Some(now),
        };
        self.counters.volume_modification += 1;
        self.volume_modifications
            .insert(volume_id.to_string(), modification);
        Ok(self.volume_modifications.get(volume_id).unwrap())
    }

    /// `ImportVolume`: synthesise a conversion task.
    #[allow(clippy::too_many_arguments)]
    pub fn import_volume(
        &mut self,
        availability_zone: String,
        image_format: String,
        image_size: i64,
        image_import_manifest_url: String,
        volume_size: i64,
        description: Option<String>,
    ) -> &ImportVolumeTask {
        let id = self.next_import_volume_task_id();
        let now = chrono::Utc::now();
        let expires = (now + chrono::Duration::days(7))
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let volume_id = format!("vol-{:08x}", self.counters.vol);
        let task = ImportVolumeTask {
            conversion_task_id: id.clone(),
            expiration_time: expires,
            image: DiskImageDescription {
                format: image_format,
                size: image_size,
                import_manifest_url: image_import_manifest_url,
                checksum: None,
            },
            volume: DiskImageVolumeDescription {
                size: volume_size,
                id: volume_id,
            },
            availability_zone,
            bytes_converted: 0,
            description,
            status: "active".to_string(),
            status_message: None,
        };
        self.import_volume_tasks.insert(id.clone(), task);
        self.import_volume_tasks.get(&id).unwrap()
    }

    pub fn cancel_import_volume_task(&mut self, task_id: &str) -> Result<(), Ec2Error> {
        let task = self
            .import_volume_tasks
            .get_mut(task_id)
            .ok_or_else(|| Ec2Error::InvalidImportVolumeTaskNotFound(task_id.to_string()))?;
        task.status = "cancelled".to_string();
        Ok(())
    }

    /// `BundleInstance`: create a bundle task.
    pub fn bundle_instance(
        &mut self,
        instance_id: String,
        bucket: String,
        prefix: String,
    ) -> &BundleTask {
        let id = self.next_bundle_task_id();
        let now = self.now_iso();
        let task = BundleTask {
            bundle_id: id.clone(),
            instance_id,
            bucket,
            prefix,
            start_time: now.clone(),
            update_time: now,
            state: "pending".to_string(),
            progress: "0%".to_string(),
            error_code: None,
            error_message: None,
        };
        self.bundle_tasks.insert(id.clone(), task);
        self.bundle_tasks.get(&id).unwrap()
    }

    pub fn cancel_bundle_task(&mut self, bundle_id: &str) -> Result<&BundleTask, Ec2Error> {
        let now = self.now_iso();
        let task = self
            .bundle_tasks
            .get_mut(bundle_id)
            .ok_or_else(|| Ec2Error::InvalidBundleTaskNotFound(bundle_id.to_string()))?;
        task.state = "cancelling".to_string();
        task.update_time = now;
        Ok(self.bundle_tasks.get(bundle_id).unwrap())
    }

    pub fn describe_bundle_tasks(&self, ids: &[String]) -> Vec<BundleTask> {
        self.bundle_tasks
            .values()
            .filter(|t| ids.is_empty() || ids.contains(&t.bundle_id))
            .cloned()
            .collect()
    }

    /// `ModifyIdFormat`: set per-resource long/short ID format toggle.
    pub fn set_id_format(&mut self, resource: &str, use_long_ids: bool) {
        self.id_format.insert(
            resource.to_string(),
            IdFormatEntry {
                use_long_ids,
                deadline: None,
            },
        );
    }

    /// `EnableFastLaunch`: store fast-launch state on the image.
    #[allow(clippy::too_many_arguments)]
    pub fn enable_fast_launch(
        &mut self,
        image_id: &str,
        resource_type: String,
        target_resource_count: Option<i32>,
        max_parallel_launches: i32,
        launch_template_id: Option<String>,
        launch_template_name: Option<String>,
        version: Option<String>,
        owner_id: String,
    ) -> Result<&FastLaunchState, Ec2Error> {
        let now = self.now_iso();
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        img.fast_launch_state = Some(FastLaunchState {
            state: "enabled".to_string(),
            image_id: image_id.to_string(),
            resource_type,
            snapshot_configuration: SnapshotConfigurationRequest {
                target_resource_count,
            },
            launch_template: FastLaunchLaunchTemplateSpecification {
                launch_template_id,
                launch_template_name,
                version,
            },
            max_parallel_launches,
            owner_id,
            state_transition_time: now,
        });
        Ok(img.fast_launch_state.as_ref().unwrap())
    }

    pub fn disable_fast_launch(&mut self, image_id: &str) -> Result<&FastLaunchState, Ec2Error> {
        let now = self.now_iso();
        let img = self
            .images
            .get_mut(image_id)
            .ok_or_else(|| Ec2Error::AmiNotFound(image_id.to_string()))?;
        match img.fast_launch_state.as_mut() {
            Some(fl) => {
                fl.state = "disabled".to_string();
                fl.state_transition_time = now;
                Ok(img.fast_launch_state.as_ref().unwrap())
            }
            None => {
                // AWS returns the disabled state even when fast-launch was
                // never enabled; synthesise a minimal record.
                img.fast_launch_state = Some(FastLaunchState {
                    state: "disabled".to_string(),
                    image_id: image_id.to_string(),
                    resource_type: "snapshot".to_string(),
                    snapshot_configuration: SnapshotConfigurationRequest::default(),
                    launch_template: FastLaunchLaunchTemplateSpecification::default(),
                    max_parallel_launches: 0,
                    owner_id: String::new(),
                    state_transition_time: now,
                });
                Ok(img.fast_launch_state.as_ref().unwrap())
            }
        }
    }

    /// `ExportImage`: synthesise an export task.
    #[allow(clippy::too_many_arguments)]
    pub fn export_image(
        &mut self,
        image_id: String,
        role_name: String,
        s3_bucket: String,
        s3_prefix: Option<String>,
        disk_image_format: String,
        description: Option<String>,
        tags: Tags,
    ) -> &ExportImageTask {
        let id = self.next_export_image_task_id();
        let task = ExportImageTask {
            export_image_task_id: id.clone(),
            description,
            image_id,
            role_name,
            status: "active".to_string(),
            status_message: None,
            progress: "0".to_string(),
            s3_export_location: ExportTaskS3Location {
                s3_bucket,
                s3_prefix,
            },
            disk_image_format,
            tags,
        };
        self.export_image_tasks.insert(id.clone(), task);
        self.export_image_tasks.get(&id).unwrap()
    }
}

/// Translate a Route Server `PersistRoutes` action ("enable" / "disable" /
/// "reset") to its corresponding state ("enabled" / "disabled" / "resetting").
/// Defaults to "disabled" when the action is `None` or unrecognised.
fn action_to_persist_routes_state(action: Option<&str>) -> String {
    match action.unwrap_or("disable") {
        "enable" => "enabled".to_string(),
        "disable" => "disabled".to_string(),
        "reset" => "resetting".to_string(),
        other => other.to_string(),
    }
}

/// Per-family default credit pair, returned by `ModifyDefaultCreditSpecification`.
#[derive(Debug, Clone)]
pub struct InstanceFamilyCreditPair {
    pub instance_family: String,
    pub cpu_credits: String,
}

/// Single purchase entry passed to `purchase_scheduled_instances`.
#[derive(Debug, Clone)]
pub struct ScheduledInstancePurchaseRequest {
    pub instance_type: String,
    pub platform: Option<String>,
    pub network_platform: Option<String>,
    pub availability_zone: String,
    pub instance_count: i32,
    pub hourly_price: Option<String>,
    pub total_scheduled_instance_hours: i32,
    pub recurrence: ScheduledInstanceRecurrence,
    pub slot_duration_in_hours: i32,
}
