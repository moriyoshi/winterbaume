pub type Tags = std::collections::HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Vpc {
    pub vpc_id: String,
    pub cidr_block: String,
    pub state: String,
    pub dhcp_options_id: String,
    pub instance_tenancy: String,
    pub is_default: bool,
    pub enable_dns_hostnames: bool,
    pub enable_dns_support: bool,
    pub secondary_cidr_blocks: Vec<(String, String)>, // (assoc_id, cidr_block)
    pub tags: Tags,
    /// EC2-Classic ClassicLink toggle. AWS retired EC2-Classic but still
    /// honours `DescribeVpcClassicLink`; new VPCs always report `false`.
    pub classic_link_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Subnet {
    pub subnet_id: String,
    pub vpc_id: String,
    pub cidr_block: String,
    pub availability_zone: String,
    pub state: String,
    pub available_ip_address_count: i64,
    pub map_public_ip_on_launch: bool,
    pub ipv6_cidr_blocks: Vec<SubnetIpv6CidrAssoc>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct SubnetIpv6CidrAssoc {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct InternetGateway {
    pub igw_id: String,
    pub attachments: Vec<IgwAttachment>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct IgwAttachment {
    pub vpc_id: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct SecurityGroup {
    pub group_id: String,
    pub group_name: String,
    pub description: String,
    pub vpc_id: String,
    pub owner_id: String,
    pub ingress_rules: Vec<IpPermission>,
    pub egress_rules: Vec<IpPermission>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct IpPermission {
    pub rule_id: String,
    pub from_port: Option<i64>,
    pub to_port: Option<i64>,
    pub ip_protocol: String,
    pub ip_ranges: Vec<IpRange>,
    pub ipv6_ranges: Vec<Ipv6Range>,
    pub user_id_group_pairs: Vec<UserIdGroupPair>,
}

#[derive(Debug, Clone)]
pub struct IpRange {
    pub cidr_ip: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Ipv6Range {
    pub cidr_ipv6: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserIdGroupPair {
    pub group_id: String,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RouteTable {
    pub route_table_id: String,
    pub vpc_id: String,
    pub routes: Vec<Route>,
    pub associations: Vec<RouteTableAssociation>,
    /// Virtual private gateway IDs ( `vgw-*` ) propagating routes into this
    /// route table. Populated by `EnableVgwRoutePropagation` and consulted
    /// by `DescribeRouteTables`. Empty when no VGW is propagating.
    pub propagating_vgws: Vec<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub destination_cidr_block: Option<String>,
    pub destination_ipv6_cidr_block: Option<String>,
    pub gateway_id: Option<String>,
    pub state: String,
    pub origin: String,
}

#[derive(Debug, Clone)]
pub struct RouteTableAssociation {
    pub association_id: String,
    pub subnet_id: Option<String>,
    /// Gateway ID this route table is associated with, set by
    /// `AssociateRouteTable` with the `GatewayId` parameter ( edge
    /// associations -- typically `igw-*` for ingress routing ).
    /// Mutually exclusive with `subnet_id`.
    pub gateway_id: Option<String>,
    pub main: bool,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct KeyPair {
    pub key_pair_id: String,
    pub key_name: String,
    pub fingerprint: String,
    pub tags: Tags,
}

// Network ACL types

#[derive(Debug, Clone)]
pub struct NetworkAcl {
    pub network_acl_id: String,
    pub vpc_id: String,
    pub is_default: bool,
    pub entries: Vec<NetworkAclEntry>,
    pub associations: Vec<NetworkAclAssociation>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct NetworkAclEntry {
    pub rule_number: i32,
    pub protocol: String,
    pub rule_action: String,
    pub egress: bool,
    pub cidr_block: Option<String>,
    pub ipv6_cidr_block: Option<String>,
    pub port_range: Option<PortRange>,
    pub icmp_type_code: Option<IcmpTypeCode>,
}

#[derive(Debug, Clone)]
pub struct PortRange {
    pub from: i32,
    pub to: i32,
}

#[derive(Debug, Clone)]
pub struct IcmpTypeCode {
    pub type_num: i32,
    pub code: i32,
}

#[derive(Debug, Clone)]
pub struct NetworkAclAssociation {
    pub network_acl_association_id: String,
    pub network_acl_id: String,
    pub subnet_id: String,
}

// Elastic IP types

#[derive(Debug, Clone)]
pub struct ElasticIp {
    pub allocation_id: String,
    pub public_ip: String,
    pub association_id: Option<String>,
    pub instance_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub private_ip_address: Option<String>,
    /// Reverse-DNS PTR record domain name. Set via `ModifyAddressAttribute`,
    /// cleared via `ResetAddressAttribute`.
    pub address_attribute_ptr_record: Option<String>,
    /// Address scope: "vpc" (default) or "standard" (Classic), used by
    /// `MoveAddressToVpc` / `RestoreAddressToClassic` legacy operations.
    pub domain: String,
    /// Account ID an `EnableAddressTransfer` offer was issued to. Cleared by
    /// `DisableAddressTransfer` (and on completion of the transfer).
    pub pending_transfer: Option<String>,
    pub tags: Tags,
}

// NAT Gateway types

#[derive(Debug, Clone)]
pub struct NatGateway {
    pub nat_gateway_id: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub state: String,
    pub connectivity_type: String,
    pub allocation_id: Option<String>,
    pub public_ip: Option<String>,
    /// Secondary addresses (assigned via `AssignPrivateNatGatewayAddress` /
    /// `AssociateNatGatewayAddress`).
    pub secondary_addresses: Vec<NatGatewayAddressAssociation>,
    pub tags: Tags,
}

/// One element of a NAT gateway's address set.
#[derive(Debug, Clone)]
pub struct NatGatewayAddressAssociation {
    pub allocation_id: Option<String>,
    pub association_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub private_ip: Option<String>,
    pub public_ip: Option<String>,
    /// One of "assigning", "unassigning", "associating", "disassociating",
    /// "succeeded", "failed", "failed-disassociating".
    pub status: String,
    pub is_primary: bool,
}

// BYOIP CIDR types

#[derive(Debug, Clone)]
pub struct ByoipCidr {
    pub cidr: String,
    pub description: Option<String>,
    /// One of "pending-provision", "provisioned", "pending-advertisement",
    /// "advertised", "pending-deprovision", "deprovisioned".
    pub state: String,
    /// ASN association for this CIDR, if a BYOASN has been associated.
    pub asn_association: Option<AsnAssociation>,
    /// IPAM pool ID this CIDR has been moved into via `MoveByoipCidrToIpam`.
    pub ipam_pool_id: Option<String>,
}

/// Association between a BYO-ASN and a BYOIP CIDR.
#[derive(Debug, Clone)]
pub struct AsnAssociation {
    pub asn: String,
    pub cidr: String,
    /// `pending-association` / `associated` / `pending-disassociation` /
    /// `disassociated` / `failed-association` / `failed-disassociation`.
    pub state: String,
    pub status_message: Option<String>,
}

// Public IPv4 pool types

#[derive(Debug, Clone)]
pub struct PublicIpv4Pool {
    pub pool_id: String,
    pub description: Option<String>,
    pub network_border_group: Option<String>,
    pub total_address_count: i32,
    pub total_available_address_count: i32,
    pub pool_address_ranges: Vec<PublicIpv4PoolRange>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct PublicIpv4PoolRange {
    pub first_address: String,
    pub last_address: String,
    pub address_count: i32,
    pub available_address_count: i32,
}

// COIP CIDR types

#[derive(Debug, Clone)]
pub struct CoipCidr {
    pub cidr: String,
    pub coip_pool_id: String,
}

// Address transfer types

#[derive(Debug, Clone)]
pub struct AddressTransfer {
    pub allocation_id: String,
    pub public_ip: String,
    pub transfer_account_id: String,
    pub transfer_offer_expiration_timestamp: String,
    pub transfer_offer_accepted_timestamp: Option<String>,
    /// One of "pending", "accepted", "disabled".
    pub address_transfer_status: String,
}

// DHCP Options types

#[derive(Debug, Clone)]
pub struct DhcpOptions {
    pub dhcp_options_id: String,
    pub configurations: Vec<DhcpConfiguration>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct DhcpConfiguration {
    pub key: String,
    pub values: Vec<String>,
}

// Egress-only internet gateway types

#[derive(Debug, Clone)]
pub struct EgressOnlyInternetGateway {
    pub eigw_id: String,
    pub state: String,
    pub attachments: Vec<EoigwAttachment>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct EoigwAttachment {
    pub vpc_id: String,
    pub state: String,
}

// Flow log types

#[derive(Debug, Clone)]
pub struct FlowLog {
    pub flow_log_id: String,
    pub resource_id: String,
    pub traffic_type: String,
    pub log_destination_type: String,
    pub log_destination: Option<String>,
    pub log_group_name: Option<String>,
    pub deliver_logs_status: String,
    pub flow_log_status: String,
    pub tags: Tags,
}

// VPC peering types

#[derive(Debug, Clone)]
pub struct VpcPeeringConnection {
    pub peering_id: String,
    pub requester_vpc_id: String,
    pub accepter_vpc_id: Option<String>,
    pub status: String,
    pub tags: Tags,
    /// Peering options on the requester side, set by
    /// `ModifyVpcPeeringConnectionOptions`.
    pub requester_peering_options: Option<VpcPeeringConnectionOptions>,
    /// Peering options on the accepter side.
    pub accepter_peering_options: Option<VpcPeeringConnectionOptions>,
}

/// Peering options for either side of a VPC peering connection.
#[derive(Debug, Clone, Default)]
pub struct VpcPeeringConnectionOptions {
    pub allow_dns_resolution_from_remote_vpc: bool,
    pub allow_egress_from_local_classic_link_to_remote_vpc: bool,
    pub allow_egress_from_local_vpc_to_remote_classic_link: bool,
}

// VPC endpoint types

#[derive(Debug, Clone)]
pub struct VpcEndpoint {
    pub endpoint_id: String,
    pub vpc_id: String,
    pub service_name: String,
    pub endpoint_type: String,
    pub state: String,
    pub policy_document: Option<String>,
    pub route_table_ids: Vec<String>,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    /// `private_dns_enabled` flag for Interface endpoints ( ignored for
    /// Gateway endpoints ). `None` preserves the legacy unset case so the
    /// terraform converter can distinguish "not specified" from
    /// "explicitly false".
    pub private_dns_enabled: Option<bool>,
    pub tags: Tags,
}

// Managed prefix list types

#[derive(Debug, Clone)]
pub struct ManagedPrefixList {
    pub prefix_list_id: String,
    pub prefix_list_name: String,
    pub state: String,
    pub address_family: String,
    pub max_entries: i32,
    pub entries: Vec<TypesPrefixListEntry>,
    pub tags: Tags,
    pub version: i64,
    /// Snapshot of (version, entries) for each historical version. Allows
    /// `RestoreManagedPrefixListVersion` to revert to a prior state.
    pub version_history: Vec<ManagedPrefixListVersion>,
}

#[derive(Debug, Clone)]
pub struct ManagedPrefixListVersion {
    pub version: i64,
    pub entries: Vec<TypesPrefixListEntry>,
}

#[derive(Debug, Clone)]
pub struct TypesPrefixListEntry {
    pub cidr: String,
    pub description: Option<String>,
}

// Customer gateway types

#[derive(Debug, Clone)]
pub struct CustomerGateway {
    pub customer_gateway_id: String,
    pub bgp_asn: String,
    pub ip_address: String,
    pub gateway_type: String,
    pub state: String,
    pub tags: Tags,
}

// VPN gateway types

#[derive(Debug, Clone)]
pub struct VpnGateway {
    pub vpn_gateway_id: String,
    pub gateway_type: String,
    pub state: String,
    pub amazon_side_asn: Option<i64>,
    pub vpc_attachments: Vec<VgwVpcAttachment>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct VgwVpcAttachment {
    pub vpc_id: String,
    pub state: String,
}

// VPN connection types

#[derive(Debug, Clone, Default)]
pub struct VpnConnection {
    pub vpn_connection_id: String,
    pub vpn_gateway_id: String,
    pub customer_gateway_id: String,
    /// Optional alternative target endpoint -- a transit gateway. AWS allows
    /// `ModifyVpnConnection` to switch the target between VGW / TGW.
    pub transit_gateway_id: Option<String>,
    pub connection_type: String,
    pub state: String,
    pub tags: Tags,
    /// Static routes added via `CreateVpnConnectionRoute`.
    pub routes: Vec<VpnStaticRoute>,
    /// Tunnel and traffic options. Modified by `ModifyVpnConnectionOptions`
    /// and `ModifyVpnTunnelOptions`.
    pub options: Option<VpnConnectionOptions>,
    /// State of any in-flight tunnel replacement triggered by
    /// `ReplaceVpnTunnel`. One of "available", "pending".
    pub tunnel_replacement_status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VpnStaticRoute {
    pub destination_cidr_block: String,
    /// Always "Static" for routes added via `CreateVpnConnectionRoute`.
    pub source: String,
    pub state: String,
}

#[derive(Debug, Clone, Default)]
pub struct VpnConnectionOptions {
    pub local_ipv4_network_cidr: Option<String>,
    pub local_ipv6_network_cidr: Option<String>,
    pub remote_ipv4_network_cidr: Option<String>,
    pub remote_ipv6_network_cidr: Option<String>,
    pub tunnel_inside_ip_version: Option<String>,
    pub static_routes_only: Option<bool>,
    pub tunnel_options: Vec<VpnTunnelOptions>,
}

#[derive(Debug, Clone, Default)]
pub struct VpnTunnelOptions {
    /// Inside-tunnel IPv4 CIDR (e.g. "169.254.10.0/30").
    pub tunnel_inside_cidr: Option<String>,
    pub tunnel_inside_ipv6_cidr: Option<String>,
    pub pre_shared_key: Option<String>,
    /// Outside IP address that identifies this tunnel.
    pub outside_ip_address: Option<String>,
    /// Certificate ARN, set by `ModifyVpnTunnelCertificate`.
    pub certificate_arn: Option<String>,
}

// VPN Concentrator types -- a less-documented internal AWS resource. Modelled
// as a thin record so `Create`/`Describe`/`Delete` round-trip cleanly.

#[derive(Debug, Clone)]
pub struct VpnConcentrator {
    pub vpn_concentrator_id: String,
    pub concentrator_type: String,
    pub state: String,
    pub transit_gateway_id: Option<String>,
    pub transit_gateway_attachment_id: Option<String>,
    pub tags: Tags,
}

// Carrier gateway types

#[derive(Debug, Clone)]
pub struct CarrierGateway {
    pub carrier_gateway_id: String,
    pub vpc_id: String,
    pub state: String,
    pub tags: Tags,
}

// Network interface types

#[derive(Debug, Clone)]
pub struct NetworkInterface {
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
    pub source_dest_check: bool,
    pub tags: Tags,
    /// Public IP DNS name options hostname type.
    pub public_ip_dns_hostname_type: Option<String>,
}

// Transit Gateway types

#[derive(Debug, Clone)]
pub struct TransitGateway {
    pub transit_gateway_id: String,
    pub state: String,
    pub amazon_side_asn: i64,
    pub description: String,
    pub dns_support: String,
    pub vpn_ecmp_support: String,
    pub multicast_support: String,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct TransitGatewayVpcAttachment {
    pub attachment_id: String,
    pub transit_gateway_id: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub state: String,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct TransitGatewayPeeringAttachment {
    pub attachment_id: String,
    pub transit_gateway_id: String,
    pub peer_transit_gateway_id: String,
    pub peer_account_id: String,
    pub peer_region: String,
    pub state: String,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct TransitGatewayRouteTable {
    pub route_table_id: String,
    pub transit_gateway_id: String,
    pub state: String,
    pub default_association_route_table: bool,
    pub default_propagation_route_table: bool,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct TransitGatewayRoute {
    pub destination_cidr_block: String,
    pub route_type: String,
    pub state: String,
    pub attachment_id: Option<String>,
}

// Instance types

#[derive(Debug, Clone)]
pub struct InstanceState {
    pub code: i32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub instance_id: String,
    pub image_id: String,
    pub instance_type: String,
    pub state: InstanceState,
    pub private_ip_address: Option<String>,
    pub public_ip_address: Option<String>,
    pub subnet_id: Option<String>,
    pub vpc_id: Option<String>,
    pub key_name: Option<String>,
    pub security_groups: Vec<String>,
    pub launch_time: String,
    pub tags: Tags,
    pub iam_instance_profile_arn: Option<String>,
    pub monitoring_state: String,
    pub placement_az: String,
    pub placement_group_name: Option<String>,
    pub placement_tenancy: Option<String>,
    pub placement_host_id: Option<String>,
    pub placement_affinity: Option<String>,
    pub placement_partition_number: Option<i32>,
    pub owner_id: String,
    /// ClassicLink VPC association: (vpc_id, [security_group_ids])
    pub classic_link_vpc: Option<(String, Vec<String>)>,
    /// Private DNS hostname type (e.g. "ip-name", "resource-name").
    pub private_dns_hostname_type: Option<String>,
    /// Whether to enable A record IP-based hostnames.
    pub enable_resource_name_dns_a_record: Option<bool>,
    /// Whether to enable AAAA record IP-based hostnames.
    pub enable_resource_name_dns_aaaa_record: Option<bool>,
    /// Per-instance burstable credit specification: "standard" | "unlimited".
    pub credit_specification: Option<String>,
    /// CPU options: (core_count, threads_per_core, amd_sev_snp).
    pub cpu_options: Option<InstanceCpuOptions>,
    /// Maintenance options.
    pub maintenance_options: Option<InstanceMaintenanceOptions>,
    /// Network performance bandwidth weighting: "default" | "vpc-1" | "ebs-1".
    pub network_bandwidth_weighting: Option<String>,
    /// Lifecycle code: e.g. "spot", "scheduled".
    pub lifecycle: Option<String>,
    /// Product codes attached to this instance (each `(code, code_type)`).
    pub product_codes: Vec<(String, String)>,
    /// Group 10: per-instance capacity-reservation specification.
    pub capacity_reservation_specification: Option<CapacityReservationSpecificationResponse>,
}

#[derive(Debug, Clone, Default)]
pub struct InstanceCpuOptions {
    pub core_count: Option<i32>,
    pub threads_per_core: Option<i32>,
    pub amd_sev_snp: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct InstanceMaintenanceOptions {
    pub auto_recovery: Option<String>,
    pub reboot_migration: Option<String>,
}

/// Account-level / per-region instance metadata defaults.
#[derive(Debug, Clone, Default)]
pub struct InstanceMetadataDefaults {
    pub http_tokens: Option<String>,
    pub http_put_response_hop_limit: Option<i32>,
    pub http_endpoint: Option<String>,
    pub instance_metadata_tags: Option<String>,
}

// EBS Volume types

#[derive(Debug, Clone)]
pub struct VolumeAttachment {
    pub volume_id: String,
    pub instance_id: String,
    pub device: String,
    pub state: String,
    pub attach_time: String,
    pub delete_on_termination: bool,
}

#[derive(Debug, Clone)]
pub struct Volume {
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
    pub attachments: Vec<VolumeAttachment>,
    pub tags: Tags,
    /// One of "in-recycle-bin" or empty. Set when the volume is moved to
    /// the Recycle Bin (mock policy treats every deletion as recoverable).
    pub recycle_bin_state: Option<String>,
    /// Set when the volume was created via `CopyVolumes`.
    pub source_volume_id: Option<String>,
}

// EBS Snapshot types

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub snapshot_id: String,
    pub volume_id: String,
    pub volume_size: i32,
    pub state: String,
    pub description: String,
    pub start_time: String,
    pub progress: String,
    pub owner_id: String,
    pub encrypted: bool,
    pub tags: Tags,
    /// Lock state: "governance" | "compliance" | "none".
    pub lock_state: String,
    pub lock_duration: Option<i32>,
    pub lock_created_on: Option<String>,
    pub lock_expires_on: Option<String>,
    pub lock_duration_start_time: Option<String>,
    pub cool_off_period: Option<i32>,
    pub cool_off_period_expires_on: Option<String>,
    /// Storage tier: "standard" | "archive".
    pub storage_tier: String,
    /// Status of the most recent tier-change operation.
    pub last_tiering_operation_status: Option<String>,
    /// Per-AZ fast-snapshot-restore states. Mutated by
    /// `EnableFastSnapshotRestores` / `DisableFastSnapshotRestores`.
    pub fast_snapshot_restore_states: Vec<FastSnapshotRestoreState>,
}

#[derive(Debug, Clone, Default)]
pub struct FastSnapshotRestoreState {
    pub availability_zone: String,
    /// One of `enabling`, `optimizing`, `enabled`, `disabling`, `disabled`.
    pub state: String,
}

// AMI / Image types

#[derive(Debug, Clone)]
pub struct Image {
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
    pub tags: Tags,
    pub source_instance_id: Option<String>,
    pub source_instance_type: String,
    pub launch_permissions: Vec<LaunchPermission>,
    /// Set to "recycled" while the AMI is in the Recycle Bin (after
    /// `DeregisterImage`); cleared on `RestoreImageFromRecycleBin`.
    pub recycle_bin_state: Option<String>,
    /// RFC 3339 timestamp set by `EnableImageDeprecation`; cleared by
    /// `DisableImageDeprecation`.
    pub deprecation_time: Option<String>,
    /// RFC 3339 timestamp at which the AMI entered the Recycle Bin. Set when
    /// `recycle_bin_state` becomes `recycled`.
    pub recycle_bin_enter_time: Option<String>,
    /// Account product-code list (optional).
    pub product_codes: Vec<(String, String)>,
    /// Fast-launch (`Windows AMI`) configuration set by `EnableFastLaunch` /
    /// `DisableFastLaunch`. `None` when fast-launch has never been configured.
    pub fast_launch_state: Option<FastLaunchState>,
    /// Image deregistration protection state. One of `enabled`,
    /// `enabled-with-cooldown`, `disabled`. `None` when never configured.
    pub deregistration_protection: Option<String>,
    /// Kernel AMI ID ( for older paravirtual / HVM-with-kernel AMIs ).
    pub kernel_id: Option<String>,
    /// Ramdisk AMI ID.
    pub ramdisk_id: Option<String>,
    /// `EnaSupport`: whether the AMI is configured for ENA networking.
    pub ena_support: Option<bool>,
    /// `SriovNetSupport`: typically `"simple"` when SR-IOV is enabled.
    pub sriov_net_support: Option<String>,
    /// `TpmSupport`: e.g. `"v2.0"`.
    pub tpm_support: Option<String>,
    /// `BootMode`: `"legacy-bios"`, `"uefi"`, `"uefi-preferred"`.
    pub boot_mode: Option<String>,
    /// `ImdsSupport`: e.g. `"v2.0"` to require IMDSv2.
    pub imds_support: Option<String>,
    /// `ImageLocation`: S3-style path, set by `RegisterImage` flows that
    /// reference an existing manifest.
    pub image_location: Option<String>,
    /// `SourceImageId`: for AMIs created via `CopyImage`, the source AMI's
    /// ID. `None` for natively created AMIs.
    pub source_image_id: Option<String>,
    /// `SourceRegion`: for AMIs created via `CopyImage`, the region the
    /// source AMI lived in. `None` for natively created AMIs.
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FastLaunchState {
    /// One of "enabling", "enabled", "disabling", "disabled".
    pub state: String,
    pub image_id: String,
    /// Always "snapshot" today (the only supported `ResourceType`).
    pub resource_type: String,
    pub snapshot_configuration: SnapshotConfigurationRequest,
    pub launch_template: FastLaunchLaunchTemplateSpecification,
    pub max_parallel_launches: i32,
    pub owner_id: String,
    pub state_transition_time: String,
}

#[derive(Debug, Clone, Default)]
pub struct SnapshotConfigurationRequest {
    pub target_resource_count: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct FastLaunchLaunchTemplateSpecification {
    pub launch_template_id: Option<String>,
    pub launch_template_name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LaunchPermission {
    pub user_id: Option<String>,
    pub group: Option<String>,
}

// Launch Template types

#[derive(Debug, Clone)]
pub struct LaunchTemplate {
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub default_version_number: i64,
    pub latest_version_number: i64,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct LaunchTemplateVersion {
    pub version_number: i64,
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub version_description: String,
    pub data: std::collections::HashMap<String, String>,
    pub default_version: bool,
}

// Spot Instance types

#[derive(Debug, Clone)]
pub struct SpotInstanceRequest {
    pub spot_instance_request_id: String,
    pub spot_price: String,
    pub instance_type: String,
    pub image_id: String,
    pub state: String,
    pub status_code: String,
    pub instance_id: Option<String>,
    pub tags: Tags,
}

/// Singleton per-account spot-instance datafeed subscription. AWS allows
/// at most one subscription per account; `CreateSpotDatafeedSubscription`
/// fails with `AlreadyExists` if one is already present, and
/// `DeleteSpotDatafeedSubscription` clears it.
#[derive(Debug, Clone)]
pub struct SpotDatafeedSubscription {
    pub bucket: String,
    pub prefix: Option<String>,
    pub owner_id: String,
    /// `"Active"` after `Create`, no other states are reachable in the
    /// emulator. Real AWS also exposes `Inactive` while propagation to
    /// S3 fails, which we don't model.
    pub state: String,
}

// IAM Instance Profile Association

#[derive(Debug, Clone)]
pub struct IamInstanceProfileAssociation {
    pub association_id: String,
    pub instance_id: String,
    pub iam_instance_profile_arn: String,
    pub iam_instance_profile_name: String,
    pub state: String,
}

// Dedicated Host types

#[derive(Debug, Clone)]
pub struct DedicatedHost {
    pub host_id: String,
    pub availability_zone: String,
    pub instance_type: Option<String>,
    pub auto_placement: String,
    pub host_recovery: String,
    pub state: String,
    pub tags: Tags,
}

// EC2 Fleet types

#[derive(Debug, Clone)]
pub struct Ec2Fleet {
    pub fleet_id: String,
    pub state: String,
    pub fleet_type: String,
    pub create_time: String,
    pub tags: Tags,
    /// Total target capacity (modifiable via `ModifyFleet`).
    pub total_target_capacity: Option<i32>,
    /// On-demand portion of target capacity.
    pub on_demand_target_capacity: Option<i32>,
    /// Spot portion of target capacity.
    pub spot_target_capacity: Option<i32>,
    /// Free-form context string (modifiable via `ModifyFleet`).
    pub context: Option<String>,
}

// VPC Endpoint Service Configuration types

#[derive(Debug, Clone, Default)]
pub struct VpcEndpointServiceConfiguration {
    pub service_id: String,
    pub service_name: String,
    pub service_type: String,
    pub acceptance_required: bool,
    pub state: String,
    pub network_load_balancer_arns: Vec<String>,
    pub gateway_load_balancer_arns: Vec<String>,
    pub allowed_principals: Vec<String>,
    pub tags: Tags,
    /// One of "ServiceOwner", "Endpoint". Toggled by
    /// `ModifyVpcEndpointServicePayerResponsibility`.
    pub payer_responsibility: Option<String>,
    /// State of the service's private DNS verification. One of
    /// "pendingVerification", "verifying", "verified", "failed".
    pub private_dns_state: Option<String>,
}

// VPC Endpoint Connection types -- represents an endpoint that has connected
// to a service configuration. Created implicitly by `CreateVpcEndpoint` against
// a service ID, transitioned by `Accept`/`RejectVpcEndpointConnections`.

#[derive(Debug, Clone)]
pub struct VpcEndpointConnection {
    pub service_id: String,
    pub vpc_endpoint_id: String,
    pub vpc_endpoint_owner: String,
    /// One of "pendingAcceptance", "available", "rejected", "deleted".
    pub vpc_endpoint_state: String,
    pub creation_timestamp: String,
}

// VPC Endpoint Connection Notification (SNS topic for endpoint events).

#[derive(Debug, Clone)]
pub struct VpcEndpointConnectionNotification {
    pub connection_notification_id: String,
    pub connection_notification_arn: String,
    pub connection_events: Vec<String>,
    pub connection_notification_state: String,
    pub connection_notification_type: String,
    pub service_id: Option<String>,
    pub vpc_endpoint_id: Option<String>,
}

// VPC Block Public Access exclusion -- an opt-out scoped to a single VPC or
// subnet, allowing internet egress (or both directions) even when the
// account-level block is on.

#[derive(Debug, Clone)]
pub struct VpcBlockPublicAccessExclusion {
    pub exclusion_id: String,
    /// One of "allow-bidirectional", "allow-egress", "block-bidirectional".
    pub internet_gateway_exclusion_mode: String,
    pub resource_arn: String,
    /// One of "create-in-progress", "create-complete", "update-in-progress",
    /// "update-complete", "delete-in-progress", "delete-complete".
    pub state: String,
    pub creation_timestamp: String,
    pub last_update_timestamp: String,
    pub tags: Tags,
}

// VPC Block Public Access account-level options -- single-value per
// account/region, modified by `ModifyVpcBlockPublicAccessOptions`.

#[derive(Debug, Clone)]
pub struct VpcBlockPublicAccessOptions {
    pub aws_account_id: String,
    pub aws_region: String,
    /// One of "off", "block-bidirectional", "block-ingress".
    pub internet_gateway_block_mode: String,
    /// One of "default", "update-in-progress", "update-complete".
    pub state: String,
    pub last_update_timestamp: String,
}

// Per-VPC encryption control resource.

#[derive(Debug, Clone)]
pub struct VpcEncryptionControl {
    pub vpc_encryption_control_id: String,
    pub vpc_id: String,
    /// One of "enforce", "monitor".
    pub mode: String,
    /// One of "enforce-in-progress", "enforce-complete", "monitor-in-progress",
    /// "monitor-complete", "delete-in-progress", "delete-complete".
    pub state: String,
    /// History of mode changes ((mode, timestamp)).
    pub mode_history: Vec<(String, String)>,
    pub tags: Tags,
}

// Spot Fleet types

#[derive(Debug, Clone)]
pub struct SpotFleetRequest {
    pub spot_fleet_request_id: String,
    pub spot_fleet_request_state: String,
    pub target_capacity: i32,
    pub iam_fleet_role: String,
    pub create_time: String,
    pub tags: Tags,
}

// Subnet CIDR Reservation types

#[derive(Debug, Clone)]
pub struct SubnetCidrReservationEntry {
    pub reservation_id: String,
    pub subnet_id: String,
    pub cidr: String,
    pub reservation_type: String,
    pub description: String,
    pub owner_id: String,
}

// Subnet IPv6 CIDR association

#[derive(Debug, Clone)]
pub struct SubnetIpv6Cidr {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub state: String,
}

// Placement Group

#[derive(Debug, Clone)]
pub struct PlacementGroup {
    pub group_id: String,
    pub group_name: String,
    pub group_arn: String,
    pub strategy: String,
    pub state: String,
    pub partition_count: Option<i32>,
    pub spread_level: Option<String>,
    pub tags: Tags,
}

// Network Interface Permission

#[derive(Debug, Clone)]
pub struct NetworkInterfacePermission {
    pub network_interface_permission_id: String,
    pub network_interface_id: String,
    pub aws_account_id: Option<String>,
    pub aws_service: Option<String>,
    pub permission: String,
    pub permission_state: String,
}

// Capacity Reservation

#[derive(Debug, Clone)]
pub struct CapacityReservation {
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
    pub end_date: Option<String>,
    pub end_date_type: String,
    pub instance_match_criteria: String,
    pub create_date: String,
    pub outpost_arn: Option<String>,
    pub placement_group_arn: Option<String>,
    pub tags: Tags,
    /// Group 10: pending billing-ownership transfer target account ID.
    pub pending_billing_owner_account_id: Option<String>,
    /// Group 10: account ID that currently owns the unused-reservation billing.
    pub billing_owner_account_id: Option<String>,
    /// Group 10: when this reservation was split off, the parent reservation ID.
    pub target_capacity_reservation_id: Option<String>,
    /// Group 10: reservation type (e.g. "default", "capacity-block").
    pub reservation_type: Option<String>,
    /// Group 10: optional commitment details.
    pub commitment_info: Option<CapacityReservationCommitmentInfo>,
}

// Instance Connect Endpoint

#[derive(Debug, Clone)]
pub struct InstanceConnectEndpoint {
    pub instance_connect_endpoint_id: String,
    pub instance_connect_endpoint_arn: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub availability_zone: String,
    pub state: String,
    pub created_at: String,
    pub preserve_client_ip: bool,
    pub security_group_ids: Vec<String>,
    pub network_interface_ids: Vec<String>,
    pub dns_name: String,
    pub fips_dns_name: String,
    pub ip_address_type: String,
    pub owner_id: String,
    pub tags: Tags,
}

// COIP (Customer-Owned IP) Pool

#[derive(Debug, Clone)]
pub struct CoipPool {
    pub pool_id: String,
    pub pool_arn: String,
    pub local_gateway_route_table_id: String,
    pub pool_cidrs: Vec<String>,
    pub tags: Tags,
}

// Security Group VPC Association

#[derive(Debug, Clone)]
pub struct SecurityGroupVpcAssociation {
    pub group_id: String,
    pub vpc_id: String,
    pub vpc_owner_id: String,
    pub state: String,
}

// Enclave Certificate IAM Role Association

#[derive(Debug, Clone)]
pub struct EnclaveCertificateIamRoleAssociation {
    pub certificate_arn: String,
    pub role_arn: String,
    pub certificate_s3_bucket_name: String,
    pub certificate_s3_object_key: String,
    pub encryption_kms_key_id: String,
}

// Mac System Integrity Protection Modification Task

#[derive(Debug, Clone)]
pub struct MacSipModificationTask {
    pub task_id: String,
    pub instance_id: String,
    pub task_type: String,
    pub task_state: String,
    pub start_time: String,
    pub apple_internal: Option<String>,
    pub base_system: Option<String>,
    pub debugging_restrictions: Option<String>,
    pub dtrace_restrictions: Option<String>,
    pub filesystem_protections: Option<String>,
    pub kext_signing: Option<String>,
    pub nvram_protections: Option<String>,
    pub status: Option<String>,
    pub tags: Tags,
}

// Declarative Policies Report

#[derive(Debug, Clone)]
pub struct DeclarativePoliciesReport {
    pub report_id: String,
    pub s3_bucket: String,
    pub s3_prefix: Option<String>,
    pub target_id: String,
    pub status: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub tags: Tags,
}

// Capacity Reservation Fleet

#[derive(Debug, Clone)]
pub struct CapacityReservationFleetInstanceSpec {
    pub instance_type: String,
    pub instance_platform: String,
    pub availability_zone: Option<String>,
    pub ebs_optimized: Option<bool>,
    pub priority: Option<i32>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct CapacityReservationFleet {
    pub capacity_reservation_fleet_id: String,
    pub capacity_reservation_fleet_arn: String,
    pub state: String,
    pub tenancy: String,
    pub allocation_strategy: String,
    pub instance_match_criteria: String,
    pub total_target_capacity: i32,
    pub total_fulfilled_capacity: f64,
    pub create_time: String,
    pub end_date: Option<String>,
    pub instance_type_specifications: Vec<CapacityReservationFleetInstanceSpec>,
    pub tags: Tags,
}

// Mac Volume Ownership Delegation Task

#[derive(Debug, Clone)]
pub struct MacVolumeOwnershipTask {
    pub task_id: String,
    /// One of "pending", "in-progress", "completed", "failed".
    pub mac_volume_ownership_task_state: String,
    pub volume_id: String,
    pub source_volume_owner_account_id: String,
    pub target_volume_owner_account_id: String,
    pub creation_time: String,
    pub completion_time: Option<String>,
}

// Replace Root Volume Task

#[derive(Debug, Clone)]
pub struct ReplaceRootVolumeTask {
    pub task_id: String,
    pub instance_id: String,
    /// One of "pending", "in-progress", "failing", "succeeding", "failed",
    /// "succeeded".
    pub task_state: String,
    pub image_id: Option<String>,
    pub snapshot_id: Option<String>,
    pub delete_replaced_root_volume: bool,
    pub start_time: String,
    pub complete_time: Option<String>,
    pub tags: Tags,
}

// Snapshot Import Task

#[derive(Debug, Clone)]
pub struct SnapshotImportTask {
    pub import_task_id: String,
    /// One of "active", "cancelling", "cancelled", "deleting", "deleted",
    /// "completed".
    pub status: String,
    pub description: Option<String>,
    pub disk_image_size: Option<f64>,
    pub format: Option<String>,
    pub url: Option<String>,
    pub user_bucket_s3_bucket: Option<String>,
    pub user_bucket_s3_key: Option<String>,
    pub owner_id: String,
    pub encrypted: bool,
    pub kms_key_id: Option<String>,
    /// Set after the task completes and a snapshot is materialised.
    pub snapshot_id: Option<String>,
    pub tags: Tags,
}

// Conversion Task (ImportInstance / ImportVolume)

#[derive(Debug, Clone)]
pub struct ImportInstanceVolumeDetail {
    pub availability_zone: Option<String>,
    pub bytes_converted: Option<i64>,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ConversionTask {
    pub conversion_task_id: String,
    pub expiration_time: String,
    pub description: Option<String>,
    pub instance_id: Option<String>,
    /// One of "Windows", "Linux".
    pub platform: String,
    pub volumes: Vec<ImportInstanceVolumeDetail>,
    /// One of "active", "cancelling", "cancelled", "completed".
    pub state: String,
    pub status_message: Option<String>,
    pub tags: Tags,
}

// Export Task (CreateInstanceExportTask)

#[derive(Debug, Clone)]
pub struct ExportTask {
    pub export_task_id: String,
    pub description: String,
    pub instance_id: String,
    /// One of "citrix", "vmware", "microsoft".
    pub target_environment: String,
    /// One of "vmdk", "raw", "vhd".
    pub disk_image_format: String,
    pub container_format: Option<String>,
    pub s3_bucket: String,
    pub s3_prefix: Option<String>,
    pub s3_key: String,
    /// One of "active", "cancelling", "cancelled", "completed".
    pub status: String,
    pub status_message: Option<String>,
    pub tags: Tags,
}

// Trunk Interface Association

#[derive(Debug, Clone)]
pub struct TrunkInterfaceAssociation {
    pub association_id: String,
    pub branch_interface_id: String,
    pub trunk_interface_id: String,
    /// One of "vlan", "gre".
    pub interface_protocol: String,
    pub vlan_id: Option<i32>,
    pub gre_key: Option<i32>,
    pub tags: Tags,
}

// Secondary Network and Subnet

#[derive(Debug, Clone)]
pub struct SecondaryNetwork {
    pub network_id: String,
    pub vpc_id: String,
    pub primary_cidr_block: String,
    pub secondary_cidr_blocks: Vec<String>,
    /// One of "pending", "available", "deleting", "deleted".
    pub state: String,
    pub network_border_group: Option<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct SecondarySubnet {
    pub subnet_id: String,
    pub vpc_id: String,
    pub secondary_network_id: String,
    pub cidr_block: String,
    pub availability_zone: String,
    pub state: String,
    pub tags: Tags,
}

// ---------------------------------------------------------------------------
// Group 5 additions: Reserved Instances, FPGA Image, Image tasks, Instance
// Event windows / events, Host reservations, Scheduled instances.
// ---------------------------------------------------------------------------

/// Quote acceptance record from `AcceptReservedInstancesExchangeQuote`.
#[derive(Debug, Clone)]
pub struct ReservedInstancesExchange {
    pub exchange_id: String,
    pub target_reserved_instances_ids: Vec<String>,
    pub source_reserved_instances_ids: Vec<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub time: String,
}

#[derive(Debug, Clone)]
pub struct ReservedInstancesListing {
    pub listing_id: String,
    pub reserved_instances_id: String,
    pub instance_count: i32,
    pub price_schedules: Vec<PriceSchedule>,
    pub status: String,
    pub status_message: Option<String>,
    pub create_date: String,
    pub update_date: String,
    pub client_token: Option<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct PriceSchedule {
    pub term: i64,
    pub price: f64,
    pub currency_code: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct ReservedInstancesPurchase {
    pub purchase_id: String,
    pub reserved_instances_offering_id: String,
    pub instance_count: i32,
    pub limit_price: Option<String>,
    pub purchase_time: String,
    pub tags: Tags,
    /// Whether this purchase is queued (not yet active). Set to `true` when
    /// the request specifies `PurchaseTime` in the future.
    pub queued: bool,
    /// The reserved instances created by this purchase.
    pub reserved_instances_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReservedInstances {
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
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct ReservedInstancesModification {
    pub modification_id: String,
    pub reserved_instances_ids: Vec<String>,
    pub target_configurations: Vec<ReservedInstancesConfiguration>,
    pub status: String,
    pub status_message: Option<String>,
    pub create_date: String,
    pub update_date: String,
    pub effective_date: String,
    pub client_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReservedInstancesConfiguration {
    pub availability_zone: Option<String>,
    pub instance_count: Option<i32>,
    pub instance_type: Option<String>,
    pub platform: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FpgaImage {
    pub fpga_image_id: String,
    pub fpga_image_global_id: String,
    pub name: String,
    pub description: Option<String>,
    pub shell_version: Option<String>,
    pub pci_id_vendor: Option<String>,
    pub pci_id_device: Option<String>,
    pub state: String,
    pub create_time: String,
    pub update_time: String,
    pub owner_id: String,
    pub owner_alias: Option<String>,
    pub product_codes: Vec<(String, String)>,
    pub tags: Tags,
    pub public: bool,
    pub data_retention_support: bool,
    pub instance_types: Vec<String>,
    /// Per-account load permissions: each is a `(scope, value)` pair where
    /// `scope` is "user_id" or "group".
    pub load_permissions: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ImageUsageReport {
    pub report_id: String,
    pub image_id: String,
    pub account_filters: Vec<String>,
    pub resource_types: Vec<String>,
    pub status: String,
    pub creation_time: String,
    pub completion_time: Option<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct RestoreImageTask {
    pub image_id: String,
    pub name: String,
    pub s3_object_url: String,
    pub status: String,
    pub status_message: Option<String>,
    pub creation_time: String,
}

#[derive(Debug, Clone)]
pub struct StoreImageTask {
    pub image_id: String,
    pub ami_id: String,
    pub bucket: String,
    pub s3_object_key: String,
    pub store_task_state: String,
    pub store_task_failure_reason: Option<String>,
    pub progress_percentage: i32,
    pub task_start_time: String,
}

#[derive(Debug, Clone)]
pub struct ImportImageTask {
    pub import_task_id: String,
    pub architecture: Option<String>,
    pub description: Option<String>,
    pub encrypted: bool,
    pub hypervisor: Option<String>,
    pub image_id: Option<String>,
    pub license_type: Option<String>,
    pub platform: Option<String>,
    pub progress: Option<String>,
    pub snapshot_details: Vec<ImportImageSnapshotDetail>,
    pub status: String,
    pub status_message: Option<String>,
    pub tags: Tags,
    pub usage_operation: Option<String>,
    pub boot_mode: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImportImageSnapshotDetail {
    pub disk_image_size: Option<f64>,
    pub format: Option<String>,
    pub progress: Option<String>,
    pub snapshot_id: Option<String>,
    pub status: Option<String>,
    pub url: Option<String>,
    pub user_bucket_s3_bucket: Option<String>,
    pub user_bucket_s3_key: Option<String>,
}

/// Per-region allowed AMI criteria entry (image owners, names, etc.).
#[derive(Debug, Clone, Default)]
pub struct AllowedImageCriterion {
    pub image_providers: Vec<String>,
    pub marketplace_product_codes: Vec<String>,
    pub deprecation_time_condition: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InstanceEventWindow {
    pub instance_event_window_id: String,
    pub name: String,
    pub time_ranges: Vec<InstanceEventWindowTimeRange>,
    pub cron_expression: Option<String>,
    pub association_target: Option<InstanceEventWindowAssociation>,
    pub state: String,
    pub tags: Tags,
}

#[derive(Debug, Clone, Default)]
pub struct InstanceEventWindowTimeRange {
    pub start_week_day: Option<String>,
    pub start_hour: Option<i32>,
    pub end_week_day: Option<String>,
    pub end_hour: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct InstanceEventWindowAssociation {
    pub instance_ids: Vec<String>,
    pub dedicated_host_ids: Vec<String>,
    pub tags: Vec<(String, String)>,
}

/// One scheduled instance status event (e.g. retirement, system reboot).
#[derive(Debug, Clone)]
pub struct InstanceEvent {
    pub event_id: String,
    pub instance_id: String,
    pub code: String,
    pub description: String,
    pub not_before: String,
    pub not_after: String,
    pub not_before_deadline: Option<String>,
}

/// Subscribe-by-tag instance event notification attribute.
#[derive(Debug, Clone, Default)]
pub struct InstanceTagNotificationAttributes {
    pub include_all_tags_of_instance: bool,
    pub instance_tag_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HostReservation {
    pub host_reservation_id: String,
    pub host_id_set: Vec<String>,
    pub currency_code: String,
    pub duration: i32,
    pub end: Option<String>,
    pub hourly_price: String,
    pub instance_family: String,
    pub offering_id: String,
    pub payment_option: String,
    pub start: String,
    pub state: String,
    pub upfront_price: String,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct ScheduledInstance {
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
    pub recurrence: ScheduledInstanceRecurrence,
    pub slot_duration_in_hours: i32,
    pub previous_slot_end_time: Option<String>,
    pub next_slot_start_time: Option<String>,
    pub create_date: String,
}

#[derive(Debug, Clone, Default)]
pub struct ScheduledInstanceRecurrence {
    pub frequency: Option<String>,
    pub interval: Option<i32>,
    pub occurrence_day_set: Vec<i32>,
    pub occurrence_relative_to_end: Option<bool>,
    pub occurrence_unit: Option<String>,
}

/// Per-instance status report submitted via `ReportInstanceStatus`.
#[derive(Debug, Clone)]
pub struct InstanceStatusReport {
    pub instance_id: String,
    pub status: String,
    pub reason_codes: Vec<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub description: Option<String>,
}

// ===== Group 6: Network Insights =====

#[derive(Debug, Clone, Default)]
pub struct PathStatementSpec {
    pub packet_header_statement: Option<PacketHeaderStatementSpec>,
    pub resource_statement: Option<ResourceStatementSpec>,
}

#[derive(Debug, Clone, Default)]
pub struct PacketHeaderStatementSpec {
    pub destination_addresses: Vec<String>,
    pub destination_ports: Vec<String>,
    pub destination_prefix_lists: Vec<String>,
    pub protocols: Vec<String>,
    pub source_addresses: Vec<String>,
    pub source_ports: Vec<String>,
    pub source_prefix_lists: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ResourceStatementSpec {
    pub resource_types: Vec<String>,
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AccessScopePathSpec {
    pub source: Option<PathStatementSpec>,
    pub destination: Option<PathStatementSpec>,
    pub through_resources: Vec<ResourceStatementSpec>,
}

#[derive(Debug, Clone)]
pub struct NetworkInsightsAccessScope {
    pub network_insights_access_scope_id: String,
    pub network_insights_access_scope_arn: String,
    pub created_date: String,
    pub updated_date: String,
    pub tags: Tags,
    pub match_paths: Vec<AccessScopePathSpec>,
    pub exclude_paths: Vec<AccessScopePathSpec>,
}

#[derive(Debug, Clone)]
pub struct NetworkInsightsAccessScopeAnalysis {
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
    pub tags: Tags,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkInsightsPathFilterPortRange {
    pub from_port: Option<i32>,
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkInsightsPathFilter {
    pub destination_address: Option<String>,
    pub destination_port_range: Option<NetworkInsightsPathFilterPortRange>,
    pub source_address: Option<String>,
    pub source_port_range: Option<NetworkInsightsPathFilterPortRange>,
}

#[derive(Debug, Clone)]
pub struct NetworkInsightsPath {
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
    pub tags: Tags,
    pub filter_at_source: NetworkInsightsPathFilter,
    pub filter_at_destination: NetworkInsightsPathFilter,
}

#[derive(Debug, Clone)]
pub struct NetworkInsightsAnalysis {
    pub network_insights_analysis_id: String,
    pub network_insights_analysis_arn: String,
    pub network_insights_path_id: String,
    pub additional_accounts: Vec<String>,
    pub filter_in_arns: Vec<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub warning_message: Option<String>,
    pub network_path_found: bool,
    pub tags: Tags,
}

// ===== Group 6: Traffic Mirror =====

#[derive(Debug, Clone, Default)]
pub struct TrafficMirrorPortRange {
    pub from_port: Option<i32>,
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct TrafficMirrorFilterRule {
    pub traffic_mirror_filter_rule_id: String,
    pub traffic_mirror_filter_id: String,
    pub traffic_direction: String,
    pub rule_number: i32,
    pub rule_action: String,
    pub protocol: Option<i32>,
    pub destination_port_range: Option<TrafficMirrorPortRange>,
    pub source_port_range: Option<TrafficMirrorPortRange>,
    pub destination_cidr_block: String,
    pub source_cidr_block: String,
    pub description: Option<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct TrafficMirrorFilter {
    pub traffic_mirror_filter_id: String,
    pub description: Option<String>,
    pub ingress_filter_rules: Vec<TrafficMirrorFilterRule>,
    pub egress_filter_rules: Vec<TrafficMirrorFilterRule>,
    pub network_services: Vec<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct TrafficMirrorSession {
    pub traffic_mirror_session_id: String,
    pub traffic_mirror_target_id: String,
    pub traffic_mirror_filter_id: String,
    pub network_interface_id: String,
    pub owner_id: String,
    pub packet_length: Option<i32>,
    pub session_number: i32,
    pub virtual_network_id: Option<i32>,
    pub description: Option<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct TrafficMirrorTarget {
    pub traffic_mirror_target_id: String,
    pub network_interface_id: Option<String>,
    pub network_load_balancer_arn: Option<String>,
    pub gateway_load_balancer_endpoint_id: Option<String>,
    pub r#type: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub tags: Tags,
}

// ===== Group 7: Client VPN =====

#[derive(Debug, Clone, Default)]
pub struct ClientVpnEndpointStatus {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClientVpnAuthorizationRuleStatus {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClientVpnRouteStatus {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClientVpnAssociationStatus {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClientVpnConnectionStatus {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClientVpnEndpoint {
    pub client_vpn_endpoint_id: String,
    pub description: Option<String>,
    pub status: ClientVpnEndpointStatus,
    pub creation_time: String,
    pub deletion_time: Option<String>,
    pub dns_name: String,
    pub client_cidr_block: String,
    pub dns_servers: Vec<String>,
    pub split_tunnel: bool,
    pub vpn_protocol: String,
    pub transport_protocol: String,
    pub vpn_port: i32,
    pub server_certificate_arn: String,
    pub authentication_options: Vec<String>,
    pub connection_log_options_enabled: bool,
    pub connection_log_options_cloudwatch_log_group: Option<String>,
    pub connection_log_options_cloudwatch_log_stream: Option<String>,
    pub tags: Tags,
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

#[derive(Debug, Clone)]
pub struct ClientVpnTargetNetworkAssociation {
    pub association_id: String,
    pub vpc_id: String,
    pub target_network_id: String, // subnet_id
    pub client_vpn_endpoint_id: String,
    pub security_groups: Vec<String>,
    pub status: ClientVpnAssociationStatus,
}

#[derive(Debug, Clone)]
pub struct ClientVpnAuthorizationRule {
    pub client_vpn_endpoint_id: String,
    pub group_id: Option<String>,
    pub access_all: bool,
    pub destination_cidr: String,
    pub description: Option<String>,
    pub status: ClientVpnAuthorizationRuleStatus,
}

#[derive(Debug, Clone)]
pub struct ClientVpnRoute {
    pub client_vpn_endpoint_id: String,
    pub destination_cidr: String,
    pub target_subnet: String,
    pub r#type: String,
    pub origin: String,
    pub status: ClientVpnRouteStatus,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClientVpnConnection {
    pub connection_id: String,
    pub client_vpn_endpoint_id: String,
    pub username: Option<String>,
    pub status: ClientVpnConnectionStatus,
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

// ===== Group 7: Local Gateway =====

#[derive(Debug, Clone)]
pub struct LocalGateway {
    pub local_gateway_id: String,
    pub outpost_arn: String,
    pub owner_id: String,
    pub state: String,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct LocalGatewayRoute {
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

#[derive(Debug, Clone)]
pub struct LocalGatewayRouteTable {
    pub local_gateway_route_table_id: String,
    pub local_gateway_route_table_arn: String,
    pub local_gateway_id: String,
    pub owner_id: String,
    pub state: String,
    pub mode: String,
    pub tags: Tags,
    pub state_reason_code: Option<String>,
    pub state_reason_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
    pub local_gateway_route_table_virtual_interface_group_association_id: String,
    pub local_gateway_virtual_interface_group_id: String,
    pub local_gateway_route_table_id: String,
    pub local_gateway_route_table_arn: String,
    pub local_gateway_id: String,
    pub owner_id: String,
    pub state: String,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct LocalGatewayRouteTableVpcAssociation {
    pub local_gateway_route_table_vpc_association_id: String,
    pub local_gateway_route_table_id: String,
    pub local_gateway_route_table_arn: String,
    pub local_gateway_id: String,
    pub vpc_id: String,
    pub owner_id: String,
    pub state: String,
    pub tags: Tags,
}

#[derive(Debug, Clone)]
pub struct LocalGatewayVirtualInterface {
    pub local_gateway_virtual_interface_id: String,
    pub local_gateway_id: String,
    pub vlan: i32,
    pub local_address: String,
    pub peer_address: String,
    pub local_bgp_asn: i32,
    pub peer_bgp_asn: i32,
    pub owner_id: String,
    pub tags: Tags,
    pub configuration_state: String,
    pub peer_bgp_asn_extended: Option<i64>,
    pub local_gateway_virtual_interface_arn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalGatewayVirtualInterfaceGroup {
    pub local_gateway_virtual_interface_group_id: String,
    pub local_gateway_virtual_interface_ids: Vec<String>,
    pub local_gateway_id: String,
    pub owner_id: String,
    pub tags: Tags,
    pub configuration_state: Option<String>,
    pub local_bgp_asn: i32,
    pub local_bgp_asn_extended: Option<i64>,
    pub local_gateway_virtual_interface_group_arn: Option<String>,
}

// ===== Group 8: Route Server =====

/// A Route Server resource. Tracks BGP route exchange between AWS-managed
/// route servers and customer route reflectors deployed in a VPC.
#[derive(Debug, Clone)]
pub struct RouteServer {
    pub route_server_id: String,
    pub route_server_arn: String,
    pub amazon_side_asn: i64,
    /// One of "pending", "available", "modifying", "deleting", "deleted".
    pub state: String,
    /// One of "enable", "disable", "reset".
    pub persist_routes: String,
    pub persist_routes_duration: Option<i64>,
    pub sns_notifications_enabled: bool,
    pub sns_topic_arn: Option<String>,
    pub tags: Tags,
}

/// A Route Server endpoint -- a route server's foothold in a single subnet of a
/// VPC, materialised as an ENI.
#[derive(Debug, Clone)]
pub struct RouteServerEndpoint {
    pub route_server_endpoint_id: String,
    pub route_server_id: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub eni_id: String,
    pub eni_address: Option<String>,
    /// One of "pending", "available", "deleting", "deleted", "failed".
    pub state: String,
    pub failure_reason: Option<String>,
    pub tags: Tags,
}

/// A BGP peer attached to a route server endpoint.
#[derive(Debug, Clone)]
pub struct RouteServerPeer {
    pub route_server_peer_id: String,
    pub route_server_endpoint_id: String,
    pub route_server_id: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub peer_address: String,
    /// One of "pending", "available", "deleting", "deleted", "failed".
    pub state: String,
    pub failure_reason: Option<String>,
    pub options: RouteServerPeerOptions,
    pub endpoint_eni_id: Option<String>,
    pub endpoint_eni_address: Option<String>,
    pub tags: Tags,
}

#[derive(Debug, Clone, Default)]
pub struct RouteServerPeerOptions {
    pub peer_asn: i64,
    /// One of "bgp-keepalive", "bfd".
    pub peer_liveness_detection: String,
    pub bgp_options: Option<RouteServerBgpOptions>,
}

#[derive(Debug, Clone, Default)]
pub struct RouteServerBgpOptions {
    pub peer_asn: Option<i64>,
    /// One of "bgp-keepalive", "bfd".
    pub peer_liveness_detection: Option<String>,
}

/// A Route Server VPC association.
#[derive(Debug, Clone)]
pub struct RouteServerAssociation {
    pub route_server_id: String,
    pub vpc_id: String,
    /// One of "associating", "associated", "disassociating", "disassociated".
    pub state: String,
    /// Route tables that have route-server propagation enabled, in addition
    /// to whatever the VPC's main table is. Mutated by
    /// `EnableRouteServerPropagation` / `DisableRouteServerPropagation`.
    pub propagations: Vec<String>,
}

// ===== Group 9: Verified Access =====

/// A Verified Access instance.
#[derive(Debug, Clone)]
pub struct VerifiedAccessInstance {
    pub verified_access_instance_id: String,
    pub description: Option<String>,
    pub creation_time: String,
    pub last_updated_time: String,
    pub fips_enabled: bool,
    pub cidr_endpoints_custom_subdomain: Option<String>,
    pub name: Option<String>,
    /// IDs of attached trust providers (resolved into condensed views in
    /// responses).
    pub trust_provider_ids: Vec<String>,
    pub tags: Tags,
}

/// A Verified Access trust provider.
#[derive(Debug, Clone)]
pub struct VerifiedAccessTrustProvider {
    pub verified_access_trust_provider_id: String,
    pub description: Option<String>,
    /// One of "user", "device".
    pub trust_provider_type: String,
    pub user_trust_provider_type: Option<String>,
    pub device_trust_provider_type: Option<String>,
    pub oidc_options: Option<VerifiedAccessOidcOptions>,
    pub device_options: Option<VerifiedAccessDeviceOptions>,
    pub native_application_oidc_options: Option<VerifiedAccessNativeApplicationOidcOptions>,
    pub policy_reference_name: String,
    pub creation_time: String,
    pub last_updated_time: String,
    pub sse_specification: VerifiedAccessSseSpecification,
    pub tags: Tags,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessOidcOptions {
    pub issuer: Option<String>,
    pub authorization_endpoint: Option<String>,
    pub token_endpoint: Option<String>,
    pub user_info_endpoint: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessDeviceOptions {
    pub tenant_id: Option<String>,
    pub public_signing_key_url: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessNativeApplicationOidcOptions {
    pub public_signing_key_endpoint: Option<String>,
    pub issuer: Option<String>,
    pub authorization_endpoint: Option<String>,
    pub token_endpoint: Option<String>,
    pub user_info_endpoint: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessSseSpecification {
    pub customer_managed_key_enabled: Option<bool>,
    pub kms_key_arn: Option<String>,
}

/// A Verified Access group within an instance.
#[derive(Debug, Clone)]
pub struct VerifiedAccessGroup {
    pub verified_access_group_id: String,
    pub verified_access_group_arn: String,
    pub verified_access_instance_id: String,
    pub owner: String,
    pub description: Option<String>,
    pub creation_time: String,
    pub last_updated_time: String,
    pub deletion_time: Option<String>,
    pub sse_specification: VerifiedAccessSseSpecification,
    pub policy_document: Option<String>,
    pub policy_enabled: bool,
    pub tags: Tags,
}

/// A Verified Access endpoint within a group.
#[derive(Debug, Clone)]
pub struct VerifiedAccessEndpoint {
    pub verified_access_endpoint_id: String,
    pub verified_access_instance_id: String,
    pub verified_access_group_id: String,
    pub application_domain: Option<String>,
    /// One of "load-balancer", "network-interface", "cidr", "rds".
    pub endpoint_type: String,
    pub attachment_type: String,
    pub domain_certificate_arn: Option<String>,
    pub endpoint_domain: Option<String>,
    pub device_validation_domain: Option<String>,
    pub security_group_ids: Vec<String>,
    pub load_balancer_options: Option<VerifiedAccessEndpointLoadBalancerOptions>,
    pub network_interface_options: Option<VerifiedAccessEndpointEniOptions>,
    pub cidr_options: Option<VerifiedAccessEndpointCidrOptions>,
    pub rds_options: Option<VerifiedAccessEndpointRdsOptions>,
    pub status_code: String,
    pub status_message: Option<String>,
    pub description: Option<String>,
    pub creation_time: String,
    pub last_updated_time: String,
    pub deletion_time: Option<String>,
    pub sse_specification: VerifiedAccessSseSpecification,
    pub policy_document: Option<String>,
    pub policy_enabled: bool,
    pub tags: Tags,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessEndpointPortRange {
    pub from_port: Option<i32>,
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessEndpointLoadBalancerOptions {
    pub load_balancer_arn: Option<String>,
    pub port: Option<i32>,
    pub port_ranges: Vec<VerifiedAccessEndpointPortRange>,
    pub protocol: Option<String>,
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessEndpointEniOptions {
    pub network_interface_id: Option<String>,
    pub port: Option<i32>,
    pub port_ranges: Vec<VerifiedAccessEndpointPortRange>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessEndpointCidrOptions {
    pub cidr: Option<String>,
    pub port_ranges: Vec<VerifiedAccessEndpointPortRange>,
    pub protocol: Option<String>,
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessEndpointRdsOptions {
    pub port: Option<i32>,
    pub protocol: Option<String>,
    pub rds_db_cluster_arn: Option<String>,
    pub rds_db_instance_arn: Option<String>,
    pub rds_db_proxy_arn: Option<String>,
    pub rds_endpoint: Option<String>,
    pub subnet_ids: Vec<String>,
}

/// A trust-provider <-> instance attachment record.
#[derive(Debug, Clone)]
pub struct VerifiedAccessTrustProviderAttachment {
    pub instance_id: String,
    pub trust_provider_id: String,
}

/// Logging configuration for a Verified Access instance.
#[derive(Debug, Clone, Default)]
pub struct VerifiedAccessLogs {
    pub cloud_watch_logs_enabled: bool,
    pub cloud_watch_logs_log_group: Option<String>,
    pub kinesis_data_firehose_enabled: bool,
    pub kinesis_data_firehose_delivery_stream: Option<String>,
    pub s3_enabled: bool,
    pub s3_bucket_name: Option<String>,
    pub s3_bucket_owner: Option<String>,
    pub s3_prefix: Option<String>,
    pub log_version: Option<String>,
    pub include_trust_context: Option<bool>,
}

// --- Group 10 additions: Capacity Reservation extensions ---

/// Pending / accepted billing-ownership offer for a capacity reservation.
#[derive(Debug, Clone)]
pub struct BillingOwnershipOffer {
    pub capacity_reservation_id: String,
    pub unused_reservation_billing_owner_id: String,
    pub requested_by: String,
    /// One of "pending", "accepted", "cancelled", "rejected", "deleted", "expired".
    pub status: String,
    pub status_message: Option<String>,
    pub last_update_time: String,
}

/// S3 destination for a Capacity Manager data export.
#[derive(Debug, Clone, Default)]
pub struct S3DestinationOptions {
    pub bucket: String,
    pub prefix: Option<String>,
}

/// Capacity Manager data export.
#[derive(Debug, Clone)]
pub struct CapacityManagerDataExport {
    pub data_export_id: String,
    /// One of "ondemand", "daily", "weekly", "monthly".
    pub schedule: String,
    pub organization_account_ids: Vec<String>,
    /// One of "parquet", "csv".
    pub output_format: String,
    pub s3_destination: S3DestinationOptions,
    /// One of "pending", "active", "paused", "failed", "deleted".
    pub status: String,
    pub status_message: Option<String>,
    pub last_export_time: Option<String>,
    pub next_export_time: Option<String>,
    pub create_time: String,
    pub tags: Tags,
}

/// Account-level Capacity Manager Organizations access.
#[derive(Debug, Clone)]
pub struct CapacityManagerOrganizationsAccess {
    /// One of "enabled", "disabled", "enabling", "disabling".
    pub state: String,
    pub last_updated_time: String,
}

/// Interruptible capacity-reservation allocation record.
#[derive(Debug, Clone)]
pub struct InterruptibleCapacityReservationAllocation {
    pub allocation_id: String,
    pub capacity_reservation_id: String,
    pub instance_count: i32,
    pub start_date_time: String,
    pub end_date_time: String,
    /// One of "scheduled", "active", "completed", "failed", "cancelled".
    pub state: String,
    /// One of "scheduled", "on-demand".
    pub allocation_type: String,
    pub tags: Tags,
}

/// A Capacity Block purchase record (fixed-duration capacity reservation).
#[derive(Debug, Clone)]
pub struct CapacityBlock {
    pub capacity_block_id: String,
    pub capacity_reservation_id: String,
    pub capacity_block_offering_id: String,
    pub instance_type: String,
    pub instance_count: i32,
    pub availability_zone: String,
    pub start_date: String,
    pub end_date: String,
    /// One of "default", "dedicated".
    pub tenancy: String,
    pub currency_code: String,
    pub upfront_fee: String,
    pub commitment_duration_in_seconds: i64,
    pub capacity_reservation_arn: String,
    pub tags: Tags,
}

/// Extension of an existing Capacity Block.
#[derive(Debug, Clone)]
pub struct CapacityBlockExtension {
    pub capacity_block_extension_id: String,
    pub capacity_reservation_id: String,
    pub instance_type: String,
    pub availability_zone: String,
    pub instance_count: i32,
    pub availability_zone_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub capacity_block_extension_offering_id: String,
    /// One of "payment-pending", "payment-failed", "payment-succeeded".
    pub capacity_block_extension_status: String,
    pub capacity_block_extension_purchase_date: String,
    pub capacity_block_extension_duration_hours: i32,
    pub currency_code: String,
    pub upfront_fee: String,
    pub capacity_reservation_arn: Option<String>,
    pub capacity_block_extension_arn: Option<String>,
}

/// Optional commitment metadata attached to a capacity reservation.
#[derive(Debug, Clone, Default)]
pub struct CapacityReservationCommitmentInfo {
    pub commitment_end_date: Option<String>,
    pub committed_instance_count: Option<i32>,
}

/// Per-instance capacity-reservation specification (response shape).
#[derive(Debug, Clone, Default)]
pub struct CapacityReservationSpecificationResponse {
    /// One of "open", "none", "capacity-reservations-only".
    pub capacity_reservation_preference: Option<String>,
    pub capacity_reservation_id: Option<String>,
    pub capacity_reservation_resource_group_arn: Option<String>,
}

// ---------------------------------------------------------------------------
// Group 11: Transit Gateway extension types
// ---------------------------------------------------------------------------

/// TGW multicast domain.
#[derive(Debug, Clone)]
pub struct TransitGatewayMulticastDomain {
    pub transit_gateway_multicast_domain_id: String,
    pub transit_gateway_id: String,
    pub transit_gateway_multicast_domain_arn: String,
    pub owner_id: String,
    /// One of `enable` / `disable`.
    pub igmpv2_support: String,
    pub static_sources_support: String,
    pub auto_accept_shared_associations: String,
    /// One of `pending` / `available` / `deleting` / `deleted`.
    pub state: String,
    pub creation_time: String,
    pub tags: Tags,
}

/// One subnet within a `TransitGatewayMulticastDomainAssociation`.
#[derive(Debug, Clone)]
pub struct MulticastSubnetAssociation {
    pub subnet_id: String,
    /// One of `associating` / `associated` / `disassociating` / `disassociated`
    /// / `rejected` / `failed` / `pendingAcceptance`.
    pub state: String,
}

/// VPC↔domain association keyed by `(domain_id, attachment_id)`.
#[derive(Debug, Clone)]
pub struct TransitGatewayMulticastDomainAssociation {
    pub transit_gateway_multicast_domain_id: String,
    pub transit_gateway_attachment_id: String,
    pub resource_id: String,
    /// Always `vpc` in this mock.
    pub resource_type: String,
    pub subnets: Vec<MulticastSubnetAssociation>,
}

/// Multicast group member (keyed by `(domain_id, group_ip, network_interface_id)`).
#[derive(Debug, Clone)]
pub struct TransitGatewayMulticastGroupMember {
    pub transit_gateway_multicast_domain_id: String,
    pub group_ip_address: String,
    pub transit_gateway_attachment_id: Option<String>,
    pub subnet_id: Option<String>,
    pub resource_id: Option<String>,
    /// Always `vpc` in this mock.
    pub resource_type: String,
    pub network_interface_id: String,
    /// Always `igmp` for members registered through the API.
    pub member_type: String,
    pub source_type: String,
}

/// Multicast group source (keyed by `(domain_id, group_ip, network_interface_id)`).
#[derive(Debug, Clone)]
pub struct TransitGatewayMulticastGroupSource {
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

/// TGW connect attachment (GRE-tunnelled connect attachment over a transport VPC attachment).
#[derive(Debug, Clone)]
pub struct TransitGatewayConnect {
    pub transit_gateway_attachment_id: String,
    pub transport_transit_gateway_attachment_id: String,
    pub transit_gateway_id: String,
    /// One of `pendingAcceptance` / `rollingBack` / `pending` / `available` /
    /// `modifying` / `deleting` / `deleted` / `failed` / `rejected` / `rejecting`
    /// / `failing`.
    pub state: String,
    pub creation_time: String,
    pub protocol: String,
    pub tags: Tags,
}

/// TGW connect peer attached to a `TransitGatewayConnect`.
#[derive(Debug, Clone)]
pub struct TransitGatewayConnectPeer {
    pub transit_gateway_attachment_id: String,
    pub transit_gateway_connect_peer_id: String,
    /// One of `pending` / `available` / `deleting` / `deleted`.
    pub state: String,
    pub creation_time: String,
    pub transit_gateway_address: String,
    pub peer_address: String,
    pub inside_cidr_blocks: Vec<String>,
    pub protocol: String,
    pub bgp_configurations: Vec<TransitGatewayAttachmentBgpConfiguration>,
    pub tags: Tags,
}

#[derive(Debug, Clone, Default)]
pub struct TransitGatewayAttachmentBgpConfiguration {
    pub transit_gateway_asn: i64,
    pub peer_asn: i64,
    pub transit_gateway_address: String,
    pub peer_address: String,
    /// `up` / `down`.
    pub bgp_status: String,
}

/// TGW metering policy (one record per policy ID).
#[derive(Debug, Clone)]
pub struct TransitGatewayMeteringPolicy {
    pub transit_gateway_metering_policy_id: String,
    pub transit_gateway_metering_policy_arn: String,
    pub transit_gateway_id: String,
    pub name: String,
    pub description: Option<String>,
    /// `available` / `deleted`.
    pub state: String,
    pub tags: Tags,
    pub last_updated_time: String,
    pub version: i32,
    pub middlebox_attachment_ids: Vec<String>,
}

/// One entry within a metering policy. Stored separately so `Modify*` /
/// `Delete*Entry` can mutate independently.
#[derive(Debug, Clone)]
pub struct TransitGatewayMeteringPolicyEntry {
    pub transit_gateway_metering_policy_entry_id: String,
    pub transit_gateway_metering_policy_id: String,
    pub sequence_number: i32,
    /// `meter` / `exclude`.
    pub action: String,
    pub source_cidr_block: Option<String>,
    pub destination_cidr_block: Option<String>,
    pub protocol: Option<String>,
    pub source_port: Option<String>,
    pub destination_port: Option<String>,
    pub dimensions: Vec<String>,
    pub state: String,
}

/// TGW policy table.
#[derive(Debug, Clone)]
pub struct TransitGatewayPolicyTable {
    pub transit_gateway_policy_table_id: String,
    pub transit_gateway_id: String,
    /// `pending` / `available` / `deleting` / `deleted`.
    pub state: String,
    pub creation_time: String,
    pub tags: Tags,
}

/// Policy-table-to-attachment association (keyed by `(policy_table_id, attachment_id)`).
#[derive(Debug, Clone)]
pub struct TransitGatewayPolicyTableAssociation {
    pub transit_gateway_policy_table_id: String,
    pub transit_gateway_attachment_id: String,
    /// Always `vpc` for now.
    pub resource_type: String,
    pub resource_id: String,
    /// `associating` / `associated` / `disassociating` / `disassociated`.
    pub state: String,
}

/// Prefix-list reference attached to a TGW route table (keyed by
/// `(route_table_id, prefix_list_id)`).
#[derive(Debug, Clone)]
pub struct TransitGatewayPrefixListReference {
    pub transit_gateway_route_table_id: String,
    pub prefix_list_id: String,
    pub prefix_list_owner_id: String,
    /// `pending` / `available` / `modifying` / `deleting`.
    pub state: String,
    pub blackhole: bool,
    pub transit_gateway_attachment_id: Option<String>,
    pub resource_id: Option<String>,
    pub resource_type: Option<String>,
}

/// TGW route-table announcement (per-table cross-network announcement).
#[derive(Debug, Clone)]
pub struct TransitGatewayRouteTableAnnouncement {
    pub transit_gateway_route_table_announcement_id: String,
    pub transit_gateway_id: String,
    pub core_network_id: String,
    pub peer_transit_gateway_id: String,
    pub peer_core_network_id: Option<String>,
    pub peering_attachment_id: String,
    /// `outgoing` / `incoming`.
    pub announcement_direction: String,
    pub transit_gateway_route_table_id: String,
    /// `available` / `pending` / `failing` / `failed` / `deleting` / `deleted`.
    pub state: String,
    pub creation_time: String,
    pub tags: Tags,
}

// ---------------------------------------------------------------------------
// Group 12: IPAM (IP Address Manager) types
// ---------------------------------------------------------------------------

/// One operating region for an IPAM or resource discovery.
#[derive(Debug, Clone)]
pub struct IpamOperatingRegion {
    pub region_name: String,
}

/// Top-level IPAM record.
#[derive(Debug, Clone)]
pub struct Ipam {
    pub ipam_id: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub public_default_scope_id: String,
    pub private_default_scope_id: String,
    pub scope_count: i32,
    pub description: Option<String>,
    pub operating_regions: Vec<IpamOperatingRegion>,
    /// `create-in-progress` / `create-complete` / `modify-in-progress` /
    /// `modify-complete` / `delete-in-progress` / `delete-complete` / etc.
    pub state: String,
    pub owner_id: String,
    pub default_resource_discovery_id: Option<String>,
    pub default_resource_discovery_association_id: Option<String>,
    pub resource_discovery_association_count: i32,
    /// `free` / `advanced`.
    pub tier: String,
    pub enable_private_gua: bool,
    /// `ipam-owner` / `resource-owner`.
    pub metered_account: String,
    pub tags: Tags,
}

/// One IPAM scope (private or public). Created automatically when an IPAM
/// is created (one default `private` and one default `public`); additional
/// scopes can be created with `CreateIpamScope`.
#[derive(Debug, Clone)]
pub struct IpamScope {
    pub ipam_scope_id: String,
    pub ipam_scope_arn: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    /// `public` / `private`.
    pub ipam_scope_type: String,
    pub is_default: bool,
    pub description: Option<String>,
    pub pool_count: i32,
    pub state: String,
    pub tags: Tags,
    pub owner_id: String,
}

/// One IPAM pool inside a scope.
#[derive(Debug, Clone)]
pub struct IpamPool {
    pub ipam_pool_id: String,
    pub source_ipam_pool_id: Option<String>,
    pub ipam_pool_arn: String,
    pub ipam_scope_arn: String,
    pub ipam_scope_type: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub locale: String,
    pub pool_depth: i32,
    pub state: String,
    pub state_message: Option<String>,
    pub description: Option<String>,
    pub auto_import: bool,
    pub publicly_advertisable: bool,
    /// `ipv4` / `ipv6`.
    pub address_family: String,
    pub allocation_min_netmask_length: Option<i32>,
    pub allocation_max_netmask_length: Option<i32>,
    pub allocation_default_netmask_length: Option<i32>,
    pub allocation_resource_tags: Vec<(String, String)>,
    pub aws_service: Option<String>,
    pub public_ip_source: Option<String>,
    pub source_resource_id: Option<String>,
    pub source_resource_type: Option<String>,
    pub source_resource_region: Option<String>,
    pub source_resource_owner: Option<String>,
    pub tags: Tags,
    pub owner_id: String,
    pub allocation_count: i32,
}

/// One CIDR provisioned into an IPAM pool.
#[derive(Debug, Clone)]
pub struct IpamPoolCidr {
    pub cidr: String,
    /// `pending-provision` / `provisioned` / `failed-provision` /
    /// `pending-deprovision` / `deprovisioned` / `failed-deprovision`.
    pub state: String,
    pub failure_reason: Option<String>,
    pub ipam_pool_cidr_id: String,
    pub netmask_length: Option<i32>,
}

/// One allocation against an IPAM pool. Keyed in state by
/// `(ipam_pool_id, ipam_pool_allocation_id)`.
#[derive(Debug, Clone)]
pub struct IpamPoolAllocation {
    pub ipam_pool_allocation_id: String,
    pub cidr: String,
    pub ipam_pool_id: String,
    pub description: Option<String>,
    pub resource_id: Option<String>,
    /// `ipam-pool` / `vpc` / `subnet` / `ec2-public-ipv4-pool` / `custom`.
    pub resource_type: String,
    pub resource_region: Option<String>,
    pub resource_owner: Option<String>,
}

/// IPAM resource discovery record (for cross-account discovery).
#[derive(Debug, Clone)]
pub struct IpamResourceDiscovery {
    pub ipam_resource_discovery_id: String,
    pub ipam_resource_discovery_arn: String,
    pub ipam_resource_discovery_region: String,
    pub description: Option<String>,
    pub operating_regions: Vec<IpamOperatingRegion>,
    pub is_default: bool,
    pub state: String,
    pub owner_id: String,
    pub tags: Tags,
}

/// Association between an IPAM and a resource discovery.
#[derive(Debug, Clone)]
pub struct IpamResourceDiscoveryAssociation {
    pub ipam_resource_discovery_association_id: String,
    pub ipam_resource_discovery_association_arn: String,
    pub ipam_arn: String,
    pub ipam_id: String,
    pub ipam_region: String,
    pub ipam_resource_discovery_id: String,
    pub owner_id: String,
    pub is_default: bool,
    /// `active` / `not-found`.
    pub resource_discovery_status: String,
    /// `associate-in-progress` / `associate-complete` / `associate-failed` /
    /// `disassociate-in-progress` / `disassociate-complete` / `disassociate-failed` /
    /// `isolate-in-progress` / `isolate-complete` / `restore-in-progress`.
    pub state: String,
    pub tags: Tags,
}

/// BYO-ASN provisioned into an IPAM. Keyed by `(ipam_id, asn)`.
#[derive(Debug, Clone)]
pub struct IpamByoasn {
    pub asn: String,
    pub ipam_id: String,
    pub description: Option<String>,
    /// `deprovisioned` / `failed-deprovision` / `failed-provision` /
    /// `pending-deprovision` / `pending-provision` / `provisioned`.
    pub state: String,
    pub status_message: Option<String>,
}

/// IPAM external resource verification token.
#[derive(Debug, Clone)]
pub struct IpamExternalResourceVerificationToken {
    pub ipam_external_resource_verification_token_id: String,
    pub ipam_external_resource_verification_token_arn: String,
    pub ipam_id: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub token_value: String,
    pub token_name: String,
    pub not_after: String,
    /// `valid` / `expired`.
    pub status: String,
    pub state: String,
    pub tags: Tags,
}

/// IPAM allocation policy. Stores its current allocation rules (modifiable
/// via `ModifyIpamPolicyAllocationRules`).
#[derive(Debug, Clone)]
pub struct IpamPolicy {
    pub ipam_policy_id: String,
    pub ipam_policy_arn: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub policy_name: String,
    /// Always `allocation` for now.
    pub policy_type: String,
    pub description: Option<String>,
    pub state: String,
    pub allocation_rules: Vec<IpamPolicyAllocationRule>,
    pub tags: Tags,
    pub owner_id: String,
}

/// One allocation rule in an IPAM policy.
#[derive(Debug, Clone)]
pub struct IpamPolicyAllocationRule {
    pub source_ipam_pool_id: Option<String>,
}

/// IPAM Prefix List Resolver.
#[derive(Debug, Clone)]
pub struct IpamPrefixListResolver {
    pub ipam_prefix_list_resolver_id: String,
    pub ipam_prefix_list_resolver_arn: String,
    pub ipam_arn: String,
    pub ipam_region: String,
    pub name: String,
    pub description: Option<String>,
    /// `pending` / `available` / `modifying` / `deleting` / `deleted`.
    pub state: String,
    pub owner_id: String,
    pub target_count: i32,
    pub tags: Tags,
}

/// One target attached to a prefix-list resolver. Keyed by
/// `(resolver_id, target_id)`.
#[derive(Debug, Clone)]
pub struct IpamPrefixListResolverTarget {
    pub ipam_prefix_list_resolver_target_id: String,
    pub ipam_prefix_list_resolver_id: String,
    pub target_resource_arn: String,
    pub target_resource_type: String,
    pub target_resource_region: String,
    pub owner_id: String,
    pub state: String,
    pub tags: Tags,
}

// ---------------------------------------------------------------------------
// Batch B additions: VolumeModification, ImportVolumeTask, BundleTask,
// IdFormat, OutpostLag, ExportImageTask.
// ---------------------------------------------------------------------------

/// Pending or completed `ModifyVolume` operation, keyed by volume_id.
#[derive(Debug, Clone)]
pub struct VolumeModification {
    pub volume_id: String,
    /// One of "modifying", "optimizing", "completed", "failed".
    pub modification_state: String,
    pub status_message: Option<String>,
    pub target_size: Option<i32>,
    pub target_iops: Option<i32>,
    pub target_throughput: Option<i32>,
    pub target_volume_type: Option<String>,
    pub target_multi_attach_enabled: Option<bool>,
    pub original_size: Option<i32>,
    pub original_iops: Option<i32>,
    pub original_throughput: Option<i32>,
    pub original_volume_type: Option<String>,
    pub original_multi_attach_enabled: Option<bool>,
    /// Progress percent, 0-100.
    pub progress: i64,
    pub start_time: String,
    pub end_time: Option<String>,
}

/// `ImportVolume` conversion-task record (legacy, distinct from
/// `ConversionTask` for VM-import). Keyed by conversion_task_id.
#[derive(Debug, Clone)]
pub struct ImportVolumeTask {
    pub conversion_task_id: String,
    pub expiration_time: String,
    pub image: DiskImageDescription,
    pub volume: DiskImageVolumeDescription,
    pub availability_zone: String,
    pub bytes_converted: i64,
    pub description: Option<String>,
    /// One of "active", "cancelling", "cancelled", "completed".
    pub status: String,
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DiskImageDescription {
    pub format: String,
    pub size: i64,
    pub import_manifest_url: String,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DiskImageVolumeDescription {
    pub size: i64,
    pub id: String,
}

/// `BundleInstance` task tracked by ID. AWS retired the operation but
/// continues to honour the API.
#[derive(Debug, Clone)]
pub struct BundleTask {
    pub bundle_id: String,
    pub instance_id: String,
    pub bucket: String,
    pub prefix: String,
    pub start_time: String,
    pub update_time: String,
    /// One of "pending", "waiting-for-shutdown", "bundling", "storing",
    /// "cancelling", "complete", "failed".
    pub state: String,
    pub progress: String,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// Per-resource-type long/short ID format toggle.
#[derive(Debug, Clone)]
pub struct IdFormatEntry {
    pub use_long_ids: bool,
    pub deadline: Option<String>,
}

/// Outpost link aggregation group, keyed by outpost-lag ID.
#[derive(Debug, Clone)]
pub struct OutpostLag {
    pub outpost_lag_id: String,
    pub outpost_arn: String,
    pub owner_id: String,
    pub state: String,
    pub local_gateway_virtual_interface_ids: Vec<String>,
    pub service_link_virtual_interface_ids: Vec<String>,
    pub tags: Tags,
}

/// `ExportImage` task. Keyed by export_image_task_id.
#[derive(Debug, Clone)]
pub struct ExportImageTask {
    pub export_image_task_id: String,
    pub description: Option<String>,
    pub image_id: String,
    pub role_name: String,
    /// One of "active", "completed", "deleting", "deleted".
    pub status: String,
    pub status_message: Option<String>,
    pub progress: String,
    pub s3_export_location: ExportTaskS3Location,
    /// One of "vmdk", "raw", "vhd".
    pub disk_image_format: String,
    pub tags: Tags,
}

#[derive(Debug, Clone, Default)]
pub struct ExportTaskS3Location {
    pub s3_bucket: String,
    pub s3_prefix: Option<String>,
}

/// AWS Network Performance metric subscription, keyed in state by
/// `(source, destination, metric, statistic)`.
#[derive(Debug, Clone, Default)]
pub struct AwsNetworkPerformanceSubscription {
    pub source: String,
    pub destination: String,
    /// e.g. `aggregate-latency`.
    pub metric: String,
    /// e.g. `p50`.
    pub statistic: String,
    /// e.g. `five-minutes`.
    pub period: String,
}
